use crate::ast::{ASTNode,Value, Error};
use crate::lexer::token;
pub struct Literal(Value);

impl Literal {

    pub fn from_token(token: token::Token) -> Result<Self, Error> {
        Ok(Self(match token {
            token::Token::Literal(token::Literal::Int(value)) => Value::Int(value),
            token::Token::Literal(token::Literal::Float(value)) => Value::Float(value),
            token::Token::Literal(token::Literal::String(value)) => Value::String(value),
            token::Token::Literal(token::Literal::Bool(value)) => Value::Bool(value),
            _ => return Err(Error::SyntaxError(format!("Invalid literal token: {:?}", token))),
        }))
    }
}

impl ASTNode for Literal {
    fn element(&self) -> String {
        format!("{:?}", self.0)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn eval(&self) -> Result<Value, Error> {
        Ok(self.0.clone())
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        Box::new(Literal(self.0.clone()))
    }
}