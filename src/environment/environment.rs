use crate::ast::assignment::AssignmentError;
use crate::ast::value::Value;
use crate::ast::type_node::Type;
use crate::ast::binary_operation::BinaryOperationError;
use crate::ast::unary_operation::UnaryOperationError;
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
    InvalidFunctionCall,
    UnaryOperationError(UnaryOperationError),
}   

#[derive(Clone, Debug, Default)]
pub struct Environment {
    pub local_variables: HashMap<String, Variable>,
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub heap: Rc<RefCell<VariableHeap>>,
}   

#[derive(Clone, Debug)]
pub enum ReferenceOrValue {
    Reference(usize, String),
    Value(Value),
}

impl ReferenceOrValue {

    pub fn from_value(value: Value) -> Self {
        Self::Value(value)
    }

    pub fn from_reference(env: &mut Environment, name: &str) -> Result<Self, Error> {
        let var = env.get_variable(name)?;
        let value = env.heap.borrow().get(var.index).unwrap().clone();
        if var.type_.is_basic() {
            Ok(Self::Value(value.clone()))
        } else {
            Ok(Self::Reference(var.index, name.to_string()))
        }
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_, _))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self {
            Self::Reference(index, _) => {
                let value = env.heap.borrow().get(*index).unwrap().clone();
                Ok(value)
            }
            Self::Value(value) => Ok(value.clone()),
        }
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