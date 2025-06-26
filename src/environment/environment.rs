use crate::ast::ast::Error;
use crate::ast::value::Value;
use std::collections::HashMap;
use crate::lexer::token;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
    Generic(char),
    UserDefined(String),
    Option(Box<Type>),
}

impl Type {
    pub fn from_token_type(token: token::Type) -> Self {
        match token {
            token::Type::U8 => Type::U8,
            token::Type::U16 => Type::U16,
            token::Type::U32 => Type::U32,
            token::Type::U64 => Type::U64,
            token::Type::I16 => Type::I16,
            token::Type::I32 => Type::I32,
            token::Type::I64 => Type::I64,
            token::Type::F32 => Type::F32,
            token::Type::F64 => Type::F64,
            token::Type::String => Type::String,
            token::Type::Bool => Type::Bool,
            token::Type::U8Option => Type::Option(Box::new(Type::U8)),
            token::Type::U16Option => Type::Option(Box::new(Type::U16)),
            token::Type::U32Option => Type::Option(Box::new(Type::U32)),
            token::Type::U64Option => Type::Option(Box::new(Type::U64)),
            token::Type::I16Option => Type::Option(Box::new(Type::I16)),
            token::Type::I32Option => Type::Option(Box::new(Type::I32)),
            token::Type::I64Option => Type::Option(Box::new(Type::I64)),
            token::Type::F32Option => Type::Option(Box::new(Type::F32)),
            token::Type::F64Option => Type::Option(Box::new(Type::F64)),
            token::Type::BoolOption => Type::Option(Box::new(Type::Bool)),
            token::Type::StringOption => Type::Option(Box::new(Type::String)),
            token::Type::Generic(c) => Type::Generic(c),
            token::Type::UserDefined(s) => Type::UserDefined(s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub initialized: bool,
    pub value: Value,
    pub type_: Type,
    pub mutable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Environment {
    variables: HashMap<String, Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn declare_assign(&mut self, name: String, value: Value, mutable: bool, type_: Option<Type>, is_decl: bool) -> Result<(), Error> {
        if self.variables.contains_key(&name) && !is_decl {
            return Err(Error::RuntimeError(format!(
                "Variable '{}' already declared",
                name
            )));
        }
        let type_ = if type_.is_some() {
            let type_ = type_.unwrap();
            check_type(type_.clone(), value.clone())?;
            type_
        } else {
            self.infer_type(value.clone())?
        };
        self.variables.insert(name, Variable { initialized: true, value: value.clone(), mutable, type_ });
        Ok(())
    }

    pub fn declare(&mut self, name: String, mutable: bool, type_: Type) -> Result<(), Error> {
        self.variables.insert(name, Variable { initialized: false, value: Value::None, mutable, type_ });
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<Value, Error> {
        match self.variables.get_mut(name) {
            Some(var) => {
                if !var.mutable && var.initialized {
                    return Err(Error::RuntimeError(format!(
                        "Cannot assign to immutable variable '{}'",
                        name
                    )));
                }
                check_type(var.type_.clone(), value.clone())?;
                var.value = value.clone();
                var.initialized = true;
                Ok(value)
            }
            _ => Err(Error::RuntimeError(format!(
                "Variable '{}' not found for assignment",
                name
            ))),
        }
    }

    pub fn lookup_mut(&mut self, name: &str) -> Result<bool, Error> {
        match self.variables.get_mut(name) {
            Some(var) => Ok(var.mutable),
            _ => Err(Error::RuntimeError(format!(
                "Variable '{}' not found",
                name
            ))),
        }
    }

    pub fn lookup(&self, name: &str) -> Result<Value, Error> {
        match self.variables.get(name) {
            Some(var) => Ok(var.value.clone()),
            _ => Err(Error::RuntimeError(format!(
                "Variable '{}' not found",
                name
            ))),
        }
    }

    pub fn infer_type(&self, value: Value) -> Result<Type, Error> {
        match value {
            Value::Int(_) => Ok(Type::I32),
            Value::Float(_) => Ok(Type::F32),
            Value::String(_) => Ok(Type::String),
            Value::Bool(_) => Ok(Type::Bool),
            _ => Err(Error::RuntimeError(format!(
                "Cannot infer type of value '{}'",
                value
            ))),
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
    Err(Error::RuntimeError(format!(
        "Value '{}' is out of bounds for type '{:?}'",
        value, type_)))
}


pub fn check_type(type_: Type, value: Value) -> Result<(), Error> {
    match type_ {
        Type::I32 | Type::I64 | Type::I16 | Type::U8 | Type::U16 | Type::U32 | Type::U64 => match value {
            Value::Int(_) => check_bounds(value, type_),
            _ => Err(Error::RuntimeError(format!(
                "Value '{}' is not of type '{:?}'",
                value, type_
            ))),
        },
        Type::F32 | Type::F64 => match value {
            Value::Float(_) => check_bounds(value, type_),
            Value::Int(_) => check_bounds(value, type_),
            _ => Err(Error::RuntimeError(format!(
                "Value '{}' is not of type '{:?}'",
                value, type_
            ))),
        },
        Type::String => match value {
            Value::String(_) => Ok(()),
            _ => Err(Error::RuntimeError(format!(
                "Value '{}' is not of type '{:?}'",
                value, type_
            ))),
        },
        Type::Bool => match value {
            Value::Bool(_) => Ok(()),
            _ => Err(Error::RuntimeError(format!(
                "Value '{}' is not of type '{:?}'",
                value, type_
            ))),
        },
        Type::Option(inner) => match value {
            Value::None => Ok(()),
            _ => check_type(*inner, value),
        },
        Type::Generic(_) |  Type::UserDefined(_) => Err(Error::RuntimeError(format!(
            "Type '{:?}' is not implemented",
            type_
        ))),
    }
}