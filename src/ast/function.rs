use crate::ast::{ASTNode, tuple::Clonable, scope::Scope, value::Value};
use crate::environment::environment::{Environment, ReferenceOrValue, RuntimeError};
use crate::Error;
use std::collections::HashMap;
use crate::ast::type_node::Type;

#[derive(Debug)]
pub struct FnDeclaration {
    pub name: Option<String>,
    pub params: HashMap<String, Type>,
    pub return_type: Option<Type>,
    pub body: Scope,
}

impl FnDeclaration {
    pub fn signature(&self) -> FnSignature {
        FnSignature { params: self.params.clone(), return_type: self.return_type.clone() }
    }

    pub fn from_signature(name: Option<String>, signature: FnSignature, body: Scope) -> Self {
        Self { name, params: signature.params, return_type: signature.return_type, body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnSignature {
    pub params: HashMap<String, Type>,
    pub return_type: Option<Type>,
}

impl Clonable for FnDeclaration {
    fn clone_element(&self) -> Self {
        Self {
            name: self.name.clone().map(|n| n.clone()),
            params: self.params.clone(),
            return_type: self.return_type.clone(),
            body: self.body.clone(),
        }
    }
}

impl ASTNode for FnDeclaration {
    fn element(&self) -> String {
        format!("fn {} ({}) -> {:?}", self.name.clone().unwrap_or("".to_string()), self.params.keys().map(|k| 
            format!("{}: {:?}", k, self.params.get(k).unwrap())
        ).collect::<Vec<String>>().join(", "), self.return_type)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.body.children()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(self.clone_element())
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        env.declare_function(self.clone_element())?;
        Ok(Value::Bool(true))
    }
}



#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub params: HashMap<Option<String>, Box<dyn ASTNode>>
}

impl Clonable for FnCall {
    fn clone_element(&self) -> Self {
        Self {
            name: self.name.clone(),
            params: self.params.iter().map(|(k, v)| (k.clone(), v.clone_to_node())).collect(),
        }
    }
}

impl ASTNode for FnCall {
    fn element(&self) -> String {
        format!("{} ({})", self.name, self.params.keys().map(|k| 
            format!("{}", if k.is_some() {k.clone().unwrap()} else {"<>".to_string()})
        ).collect::<Vec<String>>().join(", "))
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.params.values().map(|p| p.clone_to_node()).collect()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(self.clone_element())
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let mut references = HashMap::new();
        let func = match env.get_variable(&self.name)?.type_ {
            Type::FnType(sig) => *sig,
            _ => return Err(Error::RuntimeError(RuntimeError::FunctionNotFound(self.name.clone())))
        };
        for (param, value) in self.params.iter() {
            let value = value.get_reference(env)?;
            let param = match param {
                Some(p) => p,
                None => {
                    if func.params.keys().len() == 1 {
                        let params: Vec<&String> = func.params.keys().collect();
                        params[0]
                    } else {
                        let param = match &value {
                            ReferenceOrValue::Reference(_, name) => {
                                let mut param = None;
                                for p in func.params.keys() {
                                    if name.starts_with(p) {
                                        if param.is_none() {
                                            param = Some(p);
                                        } else {
                                            return Err(Error::RuntimeError(RuntimeError::InvalidFunctionCall));
                                        }
                                    }
                                }
                                if param.is_none() {
                                    return Err(Error::RuntimeError(RuntimeError::InvalidFunctionCall));
                                }
                                param.unwrap()
                            }
                            _ => {
                                return Err(Error::RuntimeError(RuntimeError::InvalidFunctionCall))
                            }
                        };
                        param
                    }
                }
            };
            references.insert(param.clone(), value);
        }
        let result = env.call(&self.name, references)?;
        Ok(result)
    }
}