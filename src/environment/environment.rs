use crate::ast::ast::Error;
use crate::ast::value::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub value: Value,
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

    pub fn declare(&mut self, name: String, value: Value, mutable: bool) -> Result<(), Error> {
        if self.variables.contains_key(&name) {
            return Err(Error::RuntimeError(format!(
                "Variable '{}' already declared",
                name
            )));
        }
        self.variables.insert(name, Variable { value, mutable });
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<Value, Error> {
        match self.variables.get_mut(name) {
            Some(var) => {
                if !var.mutable {
                    return Err(Error::RuntimeError(format!(
                        "Cannot assign to immutable variable '{}'",
                        name
                    )));
                }
                var.value = value.clone();
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
} 