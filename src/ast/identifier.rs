use crate::ast::{ASTNode,Value, Error};
use crate::environment::environment::Environment;
use crate::lexer::token;

pub struct Identifier(String);

impl Identifier {

    pub fn from_token(token: token::Token) -> Result<Self, Error> {
        Ok(Self(match token {
            token::Token::Identifier(name) => name,
            _ => return Err(Error::SyntaxError(format!("Invalid identifier token: {:?}", token))),
        }))
    }
}

impl ASTNode for Identifier {
    fn element(&self) -> String {
        self.0.clone()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        env.lookup(&self.0)
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(Identifier(self.0.clone()))
    }
}