use crate::ast::{ASTNode,Value, Error};
use crate::environment::environment::Environment;
pub struct Loop {
    pub content: Vec<Box<dyn ASTNode>>
}


impl Loop {
    pub fn new(content: Vec<Box<dyn ASTNode>>) -> Self {
        Self { content }
    }
}

impl ASTNode for Loop {
    fn element(&self) -> String {
        "Loop".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter().map(
            |c| c.as_ref().clone_to_node()
        ).collect()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(Loop::new(self.content.iter().map(
        |c| c.as_ref().clone_to_node()
        ).collect()))
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        loop {
            for child in self.content.iter() {
                child.eval(env)?;
            }
        }
    }
}

pub struct For {
    pub range_expr: Box<dyn ASTNode>,
    pub content: Vec<Box<dyn ASTNode>>
}

pub struct While {
    pub condition: Box<dyn ASTNode>,
    pub content: Vec<Box<dyn ASTNode>>
}

impl For {
    pub fn new(range_expr: Box<dyn ASTNode>, content: Vec<Box<dyn ASTNode>>) -> Self {
        Self { range_expr, content }
    }
}

impl ASTNode for For {
    fn element(&self) -> String {
        "For".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter().map(
            |c| c.as_ref().clone_to_node()
        ).collect()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(For::new(self.range_expr.clone_to_node(), self.content.iter().map(
            |c| c.as_ref().clone_to_node())
            .collect()))
    }
}

impl While {
    pub fn new(condition: Box<dyn ASTNode>, content: Vec<Box<dyn ASTNode>>) -> Self {
        Self { condition, content }
    }
}

impl ASTNode for While {
    fn element(&self) -> String {
        "While".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter().map(
            |c| c.as_ref().clone_to_node()
        ).collect()
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(While::new(self.condition.clone_to_node(), self.content.iter().map(
            |c| c.as_ref().clone_to_node())
            .collect()))
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let mut current_value = self.condition.eval(env)?;
        let mut result = Value::None;
        while current_value == Value::Bool(true) {
            for child in self.content.iter() {
                result = child.eval(env)?;
            }
            current_value = self.condition.eval(env)?;
        }
        Ok(result)
    }
    
}