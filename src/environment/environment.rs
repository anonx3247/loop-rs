use crate::ast::assignment::AssignmentError;
use crate::ast::value::Value;
use crate::ast::type_node::Type;
use crate::ast::binary_operation::BinaryOperationError;
use std::collections::HashMap;
use crate::Error;
use crate::lexer::{get_string_interpolations};
use crate::parser::parser::Parser;
use std::rc::Rc;
use crate::environment::heap::Heap;
use std::cell::RefCell;

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
}   

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub initialized: bool,
    pub index: usize,
    pub type_: Type,
    pub mutable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Environment {
    local_variables: HashMap<String, Variable>,
    parent: Option<Rc<RefCell<Environment>>>,
    heap: Rc<RefCell<Heap>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>, heap: Option<Rc<RefCell<Heap>>>) -> Self {
        let heap = match heap {
            Some(heap) => heap,
            None => match &parent {
                Some(p) => Rc::clone(&p.borrow().heap),
                None => Rc::new(RefCell::new(Heap::new())),
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

    pub fn new_child(&self, all_by_reference: bool) -> Self {
        let parent = Some(Rc::new(RefCell::new(self.clone())));
        let heap = Rc::clone(&self.heap);
        let variables = self.all_variables();
        let mut child = Self::new(parent, Some(heap));
        if !all_by_reference {
            for (name, variable) in variables {
                if variable.type_.is_basic() && variable.initialized {
                    let value = self.heap.borrow().get(variable.index).unwrap().clone();
                    child.declare_assign(name, value, variable.mutable, Some(variable.type_)).unwrap();
                }
            }
        }
        child
    }

    fn all_variables(&self) -> HashMap<String, Variable> {
        let mut variables = HashMap::new();
        for (name, variable) in &self.local_variables {
            variables.insert(name.clone(), variable.clone());
        }
        match &self.parent {
            Some(p) => {
                let parent_variables = p.borrow().all_variables();
                variables.extend(parent_variables);
                variables
            }
            None => variables,
        }
    }

    fn update_variable_value(&mut self, name: &str, value: Value) -> Result<(), Error> {
        let index = self.get_variable(name)?.index;
        let mut heap = self.heap.borrow_mut();
        let val = match heap.get_mut(index) {
            Some(val) => val,
            None => return Err(Error::RuntimeError(RuntimeError::NoVariableAtHeapIndex(index))),
        };
        *val = value;
        Ok(())
    }
    
    fn update_variable(&mut self, name: &str, variable: Variable) -> Result<(), Error> {
        match self.local_variables.get_mut(name) {
            Some(var) => {
                *var = variable;
                Ok(())
            }
            None => match &mut self.parent {
                Some(p) => p.borrow_mut().update_variable(name, variable),
                None => Err(Error::RuntimeError(RuntimeError::VariableNotFound(name.to_string()))),
            }
        }
    }

    fn get_variable(&self, name: &str) -> Result<Variable, Error> {
        match self.local_variables.get(name) {
            Some(var) => Ok(var.clone()),
            _ => match &self.parent {
                Some(p) => p.borrow().get_variable(name),
                None => Err(Error::RuntimeError(RuntimeError::VariableNotFound(name.to_string()))),
            }
        }
    }


    pub fn declare_assign(&mut self, name: String, value: Value, mutable: bool, type_: Option<Type>) -> Result<(), Error> {
        let type_ = if type_.is_some() {
            let type_ = type_.unwrap();
            check_type(type_.clone(), value.clone())?;
            type_
        } else {
            self.infer_type(value.clone())?
        };
        let index = self.heap.borrow_mut().allocate(value);
        self.local_variables.insert(name, Variable { initialized: true, index, mutable, type_ });
        Ok(())
    }

    pub fn declare(&mut self, name: String, mutable: bool, type_: Type) -> Result<(), Error> {
        let index = self.heap.borrow_mut().allocate(Value::None);
        self.local_variables.insert(name, Variable { initialized: false, index, mutable, type_ });
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), Error> {
        let mut var = self.get_variable(name)?;
        if !var.mutable && var.initialized {
            return Err(Error::RuntimeError(RuntimeError::CannotAssignToImmutableVariable(name.to_string())));
        }
        check_type(var.type_.clone(), value.clone())?;
        if !var.initialized {
            var.initialized = true;
            self.update_variable(name, var)?;
        }
        self.update_variable_value(name, value)?;
        Ok(())
    }

    pub fn lookup_mut(&mut self, name: &str) -> Result<bool, Error> {
        Ok(self.get_variable(name)?.mutable)
    }

    pub fn lookup(&mut self, name: &str) -> Result<Value, Error> {
        let var = self.get_variable(name)?;
        if !var.initialized {
            return Err(Error::RuntimeError(RuntimeError::VariableNotInitialized(name.to_string())));
        }
        let val = self.heap.borrow().get(var.index).unwrap().clone();
        Ok(val)
    }

    pub fn infer_type(&self, value: Value) -> Result<Type, Error> {
        match value {
            Value::Int(_) => Ok(Type::I32),
            Value::Float(_) => Ok(Type::F32),
            Value::String(_, _) => Ok(Type::String),
            Value::Bool(_) => Ok(Type::Bool),
            Value::Tuple(values) => {
                let mut types = Vec::new();
                for value in values {
                    types.push(self.infer_type(value)?);
                }
                Ok(Type::Tuple(types))
            },
            _ => Err(Error::RuntimeError(RuntimeError::CannotInferType(value.to_string()))),
        }
    } 

    pub fn get_type(&self, name: &str) -> Result<Type, Error> {
        Ok(self.get_variable(name)?.type_.clone())
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
}

pub fn check_bounds(value: Value, type_: Type) -> Result<(), Error> {
    match type_ {
        Type::I32 => match value {
            Value::Int(i) => if i > i32::MIN as i64 && i < i32::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::I64 => match value {
            Value::Int(i) => if i > i64::MIN as i64 && i < i64::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::I16 => match value {
            Value::Int(i) => if i > i16::MIN as i64 && i < i16::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::U8 => match value {
            Value::Int(i) => if i > 0 && i < u8::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::U16 => match value {
            Value::Int(i) => if i > 0 && i < u16::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::U32 => match value {
            Value::Int(i) => if i > 0 && i < u32::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::U64 => match value {
            Value::Int(i) => if i > 0 && i < u64::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::F32 => match value {
            Value::Float(f) => if f > f32::MIN as f64 && f < f32::MAX as f64 {
                return Ok(())
            }
            Value::Int(i) => if i > f32::MIN as i64 && i < f32::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        Type::F64 => match value {
            Value::Float(f) => if f > f64::MIN && f < f64::MAX {
                return Ok(())
            }
            Value::Int(i) => if i > f64::MIN as i64 && i < f64::MAX as i64 {
                return Ok(())
            }
            _ => {}
        },
        _ => {}
    }
    Err(Error::RuntimeError(RuntimeError::ValueOutOfBounds(value.to_string(), type_)))
}


pub fn check_type(type_: Type, value: Value) -> Result<(), Error> {
    match type_ {
        Type::I32 | Type::I64 | Type::I16 | Type::U8 | Type::U16 | Type::U32 | Type::U64 => match value {
            Value::Int(_) => check_bounds(value, type_),
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfType(value.to_string(), type_))),
        },
        Type::F32 | Type::F64 => match value {
            Value::Float(_) => check_bounds(value, type_),
            Value::Int(_) => check_bounds(value, type_),
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfType(value.to_string(), type_))),
        },
        Type::String => match value {
            Value::String(_, _) => Ok(()),
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfType(value.to_string(), type_))),
        },
        Type::Bool => match value {
            Value::Bool(_) => Ok(()),
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfType(value.to_string(), type_))),
        },
        Type::Option(inner) => match value {
            Value::None => Ok(()),
            _ => check_type(*inner, value),
        },
        Type::Generic(_) |  Type::UserDefined(_) => Err(Error::RuntimeError(RuntimeError::TypeNotImplemented(type_))),
        Type::Tuple(types) => match value {
            Value::Tuple(values) => {
                if values.len() != types.len() {
                    return Err(Error::RuntimeError(RuntimeError::TupleLengthMismatch(types.len(), values.len())));
                }
                for (value, type_) in values.iter().zip(types.iter()) {
                    check_type(type_.clone(), value.clone())?;
                }
                Ok(())
            },
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfTupleType(value.to_string(), types))),
        },
    }
}