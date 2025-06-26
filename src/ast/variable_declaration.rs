use crate::ast::{ASTNode,Error};
use crate::ast::value::Value;
use crate::environment::environment::Environment;
use crate::environment::environment::Type;

pub struct VariableDeclaration {
    pub mutable: bool,
    pub type_: Type,
    pub name: String,
}

impl VariableDeclaration {
    pub fn new(name: String, type_: Type, mutable: bool) -> Self {
        Self {
            mutable,
            type_,
            name,
        }
    }
}

impl ASTNode for VariableDeclaration {
    fn element(&self) -> String {
        let mutable = if self.mutable { "mut" } else { "const" };
        format!("{} {} : {:?}", mutable, self.name, self.type_)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        Box::new(VariableDeclaration {
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
        })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        env.declare(self.name.clone(), self.mutable, self.type_.clone())?;
        Ok(Value::Bool(true))
    }

}