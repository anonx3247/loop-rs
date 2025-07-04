use crate::ast::{ASTNode,Value, scope::Scope};
use crate::environment::environment::Environment;
use crate::Error;

#[derive(Debug)]
pub struct Loop {
    pub content: Scope
}


impl Loop {
    pub fn new(content: Scope) -> Self {
        Self { content }
    }
}

impl ASTNode for Loop {
    fn element(&self) -> String {
        "Loop".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(Loop::new(self.content.clone()))
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        loop {
            self.content.eval(env)?;
        }
    }
}

#[derive(Debug)]
pub struct For {
    pub range_expr: Box<dyn ASTNode>,
    pub content: Scope
}

#[derive(Debug)]
pub struct While {
    pub condition: Box<dyn ASTNode>,
    pub content: Scope
}

impl For {
    pub fn new(range_expr: Box<dyn ASTNode>, content: Scope) -> Self {
        Self { range_expr, content }
    }
}

impl ASTNode for For {
    fn element(&self) -> String {
        "For".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(For::new(self.range_expr.clone_to_node(), self.content.clone()))
    }
}

impl While {
    pub fn new(condition: Box<dyn ASTNode>, content: Scope) -> Self {
        Self { condition, content }
    }
}

impl ASTNode for While {
    fn element(&self) -> String {
        "While".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(While::new(self.condition.clone_to_node(), self.content.clone()))
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let mut current_value = self.condition.eval(env)?;
        let mut result = Value::None;
        while current_value == Value::Bool(true) {
            result = self.content.eval(env)?;
            current_value = self.condition.eval(env)?;
        }
        Ok(result)
    }
    
}