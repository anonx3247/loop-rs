use crate::ast::value::Value;

pub enum Error {
    SyntaxError(String),
    RuntimeError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyntaxError(message) => write!(f, "Syntax error: {}", message),
            Error::RuntimeError(message) => write!(f, "Runtime error: {}", message),
        }
    }
}

pub trait ASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>>;
    fn element(&self) -> String;
    fn to_string(&self, indent: usize) -> String {
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
            result.push_str(&child.to_string(indent + 1));
        }
        result
    }
    fn eval(&self) -> Result<Value, Error>;
    fn clone(&self) -> Box<dyn ASTNode>;
}