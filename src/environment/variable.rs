use std::collections::HashMap;
use crate::environment::heap::Heap;

use crate::ast::type_node::Type;
use crate::ast::value::Value;
use crate::environment::environment::{Environment, RuntimeError};
use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub initialized: bool,
    pub index: usize,
    pub type_: Type,
    pub mutable: bool,
}

impl Environment {
    pub fn all_variables(&self) -> HashMap<String, Variable> {
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

    pub fn get_variable(&self, name: &str) -> Result<Variable, Error> {
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
        Type::FnType(signature) => match value {
            Value::Fn(_) => Ok(()),
            _ => Err(Error::RuntimeError(RuntimeError::ValueNotOfType(value.to_string(), Type::FnType(signature)))),
        },
        Type::Any => Ok(()),
    }
}