use crate::ast::value::Value;
use crate::environment::environment::{Environment, ReferenceOrValue};
use crate::ast::tuple::Clonable;
use crate::Error;
use crate::lexer::token;

#[derive(Debug)]
pub enum ASTError {
    InvalidLiteralToken(token::Token),
    InvalidIdentifierToken(token::Token),
}

pub trait ASTNode : std::fmt::Debug {
    fn children(&self) -> Vec<Box<dyn ASTNode>>;
    fn element(&self) -> String;
    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        if indent == 1 {
            result.push_str(&"|--");
        } else if indent > 1 {
            result.push_str(&"|   ".repeat(indent - 1));
            result.push_str(&"|--");
        }
        result.push_str(&self.element());
        result.push('\n');
        for child in self.children() {
            result.push_str(&child.print_tree(indent + 1));
        }
        result
    }
    fn to_string(&self) -> String {
        self.print_tree(0)
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let children = self.children();
        if children.len() == 0 {
            Ok(Value::None)
        } else {
            let mut result = Value::None;
            for child in children {
                result = child.eval(env)?;
            }
            Ok(result)
        }
    }

    fn get_reference(&self, env: &mut Environment) -> Result<ReferenceOrValue, Error> {
        Ok(ReferenceOrValue::from_value(self.eval(env)?))
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode>;

}

impl Clonable for Box<dyn ASTNode> {
    fn clone_element(&self) -> Box<dyn ASTNode> {
        self.clone_to_node()
    }
}

#[derive(Debug)]
pub struct MultiExpression {
    pub children: Vec<Box<dyn ASTNode>>,
}

impl ASTNode for MultiExpression {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.children.iter().map(|c| c.as_ref().clone_to_node()).collect()
    }

    fn element(&self) -> String {
        "MultiExpression".to_string()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let children = self.children.iter().map(|c| c.as_ref().clone_to_node()).collect();
        Box::new(MultiExpression { children })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let mut result = Value::None;
        for child in self.children.iter() {
            result = child.eval(env)?;
        }
        Ok(result)
    }
}


#[derive(Debug)]
pub struct EmptyASTNode {}

impl EmptyASTNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl ASTNode for EmptyASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn element(&self) -> String {
        "Empty".to_string()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(EmptyASTNode {})
    }

    fn eval(&self, _env: &mut Environment) -> Result<Value, Error> {
        Ok(Value::None)
    }
}