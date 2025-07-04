use std::collections::HashMap;
use crate::ast::function::FnDeclaration;
use crate::ast::type_node::Type;
use crate::ast::Value;
use crate::environment::environment::{Environment, RuntimeError, ReferenceOrValue};
use crate::environment::heap::Heap;
use crate::environment::variable::{check_type, Variable};
use crate::Error;

impl Environment {

    pub fn declare_function(&mut self, declaration: FnDeclaration) -> Result<(), Error> {
        let index = self.heap.borrow_mut().allocate(Value::Fn(Box::new(declaration.body.clone())));
        let type_ = Type::FnType(Box::new(declaration.signature()));
        self.local_variables.insert(declaration.name, Variable { initialized: true, index, mutable: false, type_ });
        Ok(())
    }

    pub fn call(&mut self, name: &str, args: HashMap<String, ReferenceOrValue>) -> Result<Value, Error> {
        let function = self.get_variable(name)?;
        let mut env = self.new_child();
        for (param, reference_or_value) in args.clone() {
            match reference_or_value {
                ReferenceOrValue::Reference(index, _) => {
                    env.add_reference(&param, index)?;
                }
                ReferenceOrValue::Value(value) => {
                    env.declare_assign(param, value.clone(), false, None)?;
                }
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
            match reference_or_value {
                ReferenceOrValue::Reference(_, _) => {
                    let value = reference_or_value.eval(self)?;
                    check_type(type_, value.clone())?;
                }
                ReferenceOrValue::Value(value) => {
                    check_type(type_, value.clone())?;
                }
            }
        }
        Ok(())
    }
}