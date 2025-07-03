use crate::ast::{ASTNode, Value};
use crate::Error;
use crate::environment::environment::Environment;

#[derive(Debug)]
pub struct Scope(Vec<Box<dyn ASTNode>>);

impl Scope {
    pub fn new(children: Vec<Box<dyn ASTNode>>) -> Self {
        Self(children)
    }

    pub fn eval(&self, env: &mut Environment, by_reference: bool) -> Result<Value, Error> {
        if self.0.len() > 0 {
            let mut local_env = env.new_child(by_reference);
            for child in 0..self.0.len() - 1 {
                self.0[child].eval(&mut local_env)?;
            }
            let result = self.0[self.0.len() - 1].eval(&mut local_env)?;
            local_env.free();
            Ok(result)
        } else {
            Ok(Value::None)
        }
    }

    pub fn clone(&self) -> Self {
        Self(self.0.iter().map(|c| c.clone_to_node()).collect())
    }

    pub fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.0.iter().map(|c| c.as_ref().clone_to_node()).collect()
    }
}