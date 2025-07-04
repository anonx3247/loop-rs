use crate::ast::assignment::AssignmentError;
use crate::ast::value::Value;
use crate::ast::type_node::Type;
use crate::ast::binary_operation::BinaryOperationError;
use std::collections::HashMap;
use crate::Error;
use crate::lexer::{get_string_interpolations};
use crate::parser::parser::Parser;
use std::rc::Rc;
use crate::environment::heap::{Heap, VariableHeap};
use std::cell::RefCell;
use crate::environment::variable::Variable;

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotFound(String),
    VariableNotInitialized(String),
    ValueOutOfBounds(String, Type),
    ValueNotOfType(String, Type),
    TupleLengthMismatch(usize, usize),
    CannotAssignToImmutableVariable(String),
    CannotInferType(String),
    TypeNotImplemented(Type),
    ValueNotOfTupleType(String, Vec<Type>),
    BinaryOperationError(BinaryOperationError),
    AssignmentError(AssignmentError),
    NoVariableAtHeapIndex(usize),
    FunctionNotFound(String),
}   

#[derive(Clone, Debug, Default)]
pub struct Environment {
    pub local_variables: HashMap<String, Variable>,
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub heap: Rc<RefCell<VariableHeap>>,
}   

#[derive(Clone, Debug)]
pub struct ReferenceOrValue {
    pub index: usize,
    pub value: Option<Value>,
}

impl ReferenceOrValue {
    pub fn new(env: &mut Environment, name: &str) -> Result<Self, Error> {
        let var = env.get_variable(name)?;
        let value = env.heap.borrow().get(var.index).unwrap().clone();
        if var.type_.is_basic() {
            Ok(Self { index: var.index, value: Some(value.clone()) })
        } else {
            Ok(Self { index: var.index, value: None })
        }
    }

    pub fn is_reference(&self) -> bool {
        self.value.is_none()
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        if self.value.is_none() {
            let value = env.heap.borrow().get(self.index).unwrap().clone();
            return Ok(value);
        }
        Ok(self.value.clone().unwrap())
    }
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>, heap: Option<Rc<RefCell<VariableHeap>>>) -> Self {
        let heap = match heap {
            Some(heap) => heap,
            None => match &parent {
                Some(p) => Rc::clone(&p.borrow().heap),
                None => Rc::new(RefCell::new(VariableHeap::new())),
            }
        };
        Self { local_variables: HashMap::new(), parent, heap }
    }

    pub fn free(&self) {
        let mut heap = self.heap.borrow_mut();
        for (_, variable) in &self.local_variables {
            heap.deallocate(variable.index);
        }
    }

    pub fn new_child(&self) -> Self {
        let parent = Some(Rc::new(RefCell::new(self.clone())));
        let heap = Rc::clone(&self.heap);
        Self::new(parent, Some(heap))
    }

    pub fn interpolate(&mut self, value: Value) -> Result<Value, Error> {
        match value {
            Value::String(ref s, false) => {
                let interpolations = get_string_interpolations(s);
                let mut s = s.clone();
                for (interpolation, index) in interpolations {
                    let node = Parser::parse_string(&interpolation)?;
                    let value = node.eval(self)?;
                    s = s[..index].to_string() + &value.to_string() + &s[index+interpolation.len()+2..];
                }
                Ok(Value::String(s, false))
            }
            _ => Ok(value),
        }
    }

    pub fn add_reference(&mut self, name: &str, index: usize) -> Result<(), Error> {
        self.local_variables.insert(name.to_string(), Variable { index, ..self.get_variable(name)? });
        Ok(())
    }
}