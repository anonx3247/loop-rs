use crate::ast::{ASTNode,Value, ASTError};
use crate::environment::environment::Environment;
use crate::lexer::token;
use crate::Error;

#[derive(Debug)]
pub struct Literal(pub Value);

impl Literal {

    pub fn from_token(token: token::Token) -> Result<Self, Error> {
        Ok(Self(match token {
            token::Token::Literal(token::Literal::Int(value)) => Value::Int(value),
            token::Token::Literal(token::Literal::Float(value)) => Value::Float(value),
            token::Token::Literal(token::Literal::String(value, raw)) => Value::String(value, raw),
            token::Token::Literal(token::Literal::Bool(value)) => Value::Bool(value),
            token::Token::Literal(token::Literal::None) => Value::None,
            _ => return Err(Error::ASTError(ASTError::InvalidLiteralToken(token))),
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

    fn eval(&self, _env: &mut Environment) -> Result<Value, Error> {
        _env.interpolate(self.0.clone())
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(Literal(self.0.clone()))
    }
}

