use std::collections::HashMap;
use crate::ast::type_node::Type;
use crate::ast::Value;
use crate::environment::environment::{Environment, RuntimeError, ReferenceOrValue};
use crate::environment::variable::check_type;
use crate::Error;

impl Environment {

    pub fn call(&mut self, name: &str, args: HashMap<String, ReferenceOrValue>) -> Result<Value, Error> {
        let function = self.get_variable(name)?;
        let mut env = self.new_child();
        for (param, reference_or_value) in args.clone() {
            if reference_or_value.is_reference() {
                env.add_reference(&param, reference_or_value.index)?;
            } else {
                env.declare_assign(param, reference_or_value.value.unwrap(), false, None)?;
            }
        }
        let signature = match function.type_ {
            Type::FnType(signature) => signature.clone(),
            _ => return Err(Error::RuntimeError(RuntimeError::FunctionNotFound(name.to_string()))),
        };
        self.params_match(signature.params, args)?;
        let body = match self.lookup(name)? {
            Value::Fn(body) => body,
            _ => return Err(Error::RuntimeError(RuntimeError::FunctionNotFound(name.to_string()))),
        };
        let result = body.eval(&mut env)?;
        Ok(result)
    }

    fn params_match(&mut self, params: HashMap<String, Type>, args: HashMap<String, ReferenceOrValue>) -> Result<(), Error> {
        for (param, type_) in params {
            let reference_or_value = match args.get(&param) {
                Some(reference_or_value) => reference_or_value,
                None => return Err(Error::RuntimeError(RuntimeError::VariableNotFound(param.to_string()))),
            };
            if reference_or_value.is_reference() {
                let value = reference_or_value.eval(self)?;
                check_type(type_, value.clone())?;
            } else {
                check_type(type_, reference_or_value.value.clone().unwrap())?;
            }
        }
        Ok(())
    }
}