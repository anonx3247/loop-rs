use crate::ast::{ASTNode, tuple::Clonable, scope::Scope, value::Value};
use crate::environment::environment::Environment;
use crate::Error;
use std::collections::HashMap;
use crate::ast::type_node::Type;

#[derive(Debug)]
pub struct FnDeclaration {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub return_type: Option<Type>,
    pub body: Scope,
}

impl FnDeclaration {
    pub fn signature(&self) -> FnSignature {
        FnSignature { params: self.params.clone(), return_type: self.return_type.clone() }
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
            name: self.name.clone(),
            params: self.params.clone(),
            return_type: self.return_type.clone(),
            body: self.body.clone(),
        }
    }
}

impl ASTNode for FnDeclaration {
    fn element(&self) -> String {
        format!("fn {} ({}) -> {:?}", self.name, self.params.keys().map(|k| 
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
        env.declare_assign(
            self.name.clone(), 
            Value::Fn(Box::new(self.body.clone())), 
            false, 
            Some(Type::FnType(Box::new(self.signature())))
        )?;
        Ok(Value::Bool(true))
    }
}
