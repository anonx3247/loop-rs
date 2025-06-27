use crate::ast::value::Value;
use crate::environment::environment::Environment;

#[derive(Debug)]
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

    fn clone_to_node(&self) -> Box<dyn ASTNode>;

}

pub struct RootASTNode {
    pub children: Vec<Box<dyn ASTNode>>,
}

impl RootASTNode {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub fn push(&mut self, node: Box<dyn ASTNode>) {
        self.children.push(node);
    }
}

impl ASTNode for RootASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.children.iter().map(|c| c.as_ref().clone_to_node()).collect()
    }

    fn element(&self) -> String {
        "Root".to_string()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let children = self.children.iter().map(|c| c.as_ref().clone_to_node()).collect();
        Box::new(RootASTNode { children })
    }
}

pub struct MultiExpression {
    pub children: Vec<Box<dyn ASTNode>>,
}

impl MultiExpression {
    pub fn new(children: Vec<Box<dyn ASTNode>>) -> Self {
        Self { children }
    }
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