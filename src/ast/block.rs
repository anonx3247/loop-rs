use crate::ast::{ASTNode,Value, Error};
use crate::environment::environment::Environment;

pub struct IfBlock {
    pub condition: Box<dyn ASTNode>,
    pub content: Vec<Box<dyn ASTNode>>,
}

impl IfBlock {
    pub fn new(condition: Box<dyn ASTNode>, content: Vec<Box<dyn ASTNode>>) -> Self {
        Self { condition, content }
    }
}

pub trait Conditional: ASTNode {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error>;
    fn previous_conditional(&self) -> Option<Box<dyn Conditional>>;
    fn display(&self) -> String;
    fn clone_conditional(&self) -> Box<dyn Conditional>;
    fn content(&self) -> Vec<Box<dyn ASTNode>>;

    fn evaluate_conditional(&self, env: &mut Environment) -> Result<Option<Value>, Error> {
        let mut conditionals = Vec::new();
        let mut current_conditional: Box<dyn Conditional> = self.clone_conditional();
        conditionals.push(self.clone_conditional());
        while let Some(prev_conditional) = current_conditional.previous_conditional() {
            conditionals.push(prev_conditional.clone_conditional());
            current_conditional = prev_conditional.clone_conditional();
        }

        conditionals.reverse();

        
        for conditional in conditionals {
            match conditional.condition(env) {
                Ok(Value::Bool(true)) => {
                    return Ok(Some(conditional.content().last().unwrap().eval(env)?));
                }
                Err(e) => {
                    return Err(e);
                }
                _ => {}
            }
        }
        Ok(None)
    }
}


impl Conditional for IfBlock {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error> {
        self.condition.eval(env)
    }

    fn previous_conditional(&self) -> Option<Box<dyn Conditional>> {
        None
    }

    fn display(&self) -> String {
        format!("if {}", self.condition.element())
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(IfBlock::new(self.condition.clone(), content))
    }

    fn content(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }
}

impl ASTNode for IfBlock {
    fn element(&self) -> String {
        "if".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self.evaluate_conditional(env) {
            Ok(Some(value)) => Ok(value),
            Ok(_) => Ok(Value::None),
            Err(e) => Err(e),
        }
    }
    fn clone(&self) -> Box<dyn ASTNode> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(IfBlock::new(self.condition.clone(), content))
    }

    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push_str(&"|   ".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("condition: \n");
        result.push_str(&self.condition.print_tree(indent+1));
        result.push_str(&"|   ".repeat(indent));
        result.push_str("content: \n");
        for child in self.content.iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        result
    }
}

pub struct ElseBlock {
    pub content: Vec<Box<dyn ASTNode>>,
    pub previous_conditional: Box<dyn Conditional>,
}

impl ElseBlock {
    pub fn new(content: Vec<Box<dyn ASTNode>>, previous_conditional: Box<dyn Conditional>) -> Self {
        Self { content, previous_conditional }
    }
}

impl Conditional for ElseBlock {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error> {
        Ok(Value::Bool(false))
    }

    fn content(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }

    fn previous_conditional(&self) -> Option<Box<dyn Conditional>> {
        Some(self.previous_conditional.clone_conditional())
    }

    fn display(&self) -> String {
        format!("{} \n else", self.previous_conditional.display())
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(ElseBlock::new(content, self.previous_conditional.clone_conditional()))
    }
}

impl ASTNode for ElseBlock {
    fn element(&self) -> String {
        format!("{} \n else", self.previous_conditional.display())
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self.evaluate_conditional(env) {
            Ok(value) => match value {
                Some(value) => Ok(value),
                _ => self.content.last().unwrap().eval(env),
            },
            Err(e) => Err(e),
        }
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(ElseBlock::new(content, self.previous_conditional.clone_conditional()))
    }

    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push_str(&"|   ".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("content: \n");
        for child in self.content.iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        result
    }
}

pub struct ElifBlock {
    pub condition: Box<dyn ASTNode>,
    pub content: Vec<Box<dyn ASTNode>>,
    pub previous_conditional: Box<dyn Conditional>,
}

impl ElifBlock {
    pub fn new(condition: Box<dyn ASTNode>, content: Vec<Box<dyn ASTNode>>, previous_conditional: Box<dyn Conditional>) -> Self {
        Self { condition, content, previous_conditional }
    }
}

impl Conditional for ElifBlock {

    fn condition(&self, env: &mut Environment) -> Result<Value, Error> {
        self.condition.eval(env)
    }

    fn previous_conditional(&self) -> Option<Box<dyn Conditional>> {
        Some(self.previous_conditional.clone_conditional())
    }

    fn display(&self) -> String {
        format!("{} \n elif {}", self.previous_conditional.display(), self.condition.element())
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(ElifBlock::new(self.condition.clone(), content, self.previous_conditional.clone_conditional()))
    }

    fn content(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }
}

impl ASTNode for ElifBlock {
    fn element(&self) -> String {
        "elif".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.iter()
            .map(|c| c.as_ref().clone())
            .collect::<Vec<Box<dyn ASTNode>>>()
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self.evaluate_conditional(env) {
            Ok(Some(value)) => Ok(value),
            Ok(_) => Ok(Value::None),
            Err(e) => Err(e),
        }
    }

    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push_str(&"|   ".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("condition: \n");
        result.push_str(&self.condition.print_tree(indent+1));
        result.push_str(&"|   ".repeat(indent));
        result.push_str("content: \n");
        for child in self.content.iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        result
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        let content = self.content.iter().map(|c| c.as_ref().clone()).collect();
        Box::new(ElifBlock::new(self.condition.clone(), content, self.previous_conditional.clone_conditional()))
    }
}

