use crate::ast::{ASTNode,Value, scope::Scope};
use crate::Error;
use crate::environment::environment::Environment;

#[derive(Debug)]
pub struct IfBlock {
    pub condition: Box<dyn ASTNode>,
    pub content: Scope,
    pub next_conditional: Option<Box<dyn Conditional>>,
}

impl IfBlock {
    pub fn new(condition: Box<dyn ASTNode>, content: Scope, next_conditional: Option<Box<dyn Conditional>>) -> Self {
        Self { condition, content, next_conditional }
    }
}

pub trait Conditional: ASTNode {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error>;
    fn next_conditional(&self) -> Option<Box<dyn Conditional>>;
    fn clone_conditional(&self) -> Box<dyn Conditional>;
    fn content(&self) -> Scope;

    fn evaluate_conditional(&self, env: &mut Environment) -> Result<Option<Value>, Error> {
        let mut current_conditional: Option<Box<dyn Conditional>> = Some(self.clone_conditional());
        while let Some(conditional) = current_conditional {
            match conditional.condition(env) {
                Ok(Value::Bool(true)) => {
                    let mut result = Value::None;
                    result = conditional.content().eval(env, true)?;
                    return Ok(Some(result));
                }
                Err(e) => {
                    return Err(e);
                }
                _ => {}
            }
            current_conditional = conditional.next_conditional();
        }
        Ok(None)
    }
}

impl Conditional for IfBlock {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error> {
        self.condition.eval(env)
    }

    fn next_conditional(&self) -> Option<Box<dyn Conditional>> {
        self.next_conditional.as_ref().map(|c| c.clone_conditional())
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.clone();
        Box::new(IfBlock::new(self.condition.clone_to_node(), content, self.next_conditional.as_ref().map(|c| c.clone_conditional())))
    }

    fn content(&self) -> Scope {
        self.content.clone()
    }
}

impl ASTNode for IfBlock {
    fn element(&self) -> String {
        "If".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self.evaluate_conditional(env) {
            Ok(Some(value)) => Ok(value),
            Ok(_) => Ok(Value::None),
            Err(e) => Err(e),
        }
    }
    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let content = self.content.clone();
        Box::new(IfBlock::new(self.condition.clone_to_node(), content, self.next_conditional.as_ref().map(|c| c.clone_conditional())))
    }

    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push('|');
        result.push_str(&"--".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("|-condition: \n");
        result.push_str(&self.condition.print_tree(indent+1));
        result.push_str(&"|   ".repeat(indent));
        result.push_str("|-content: \n");
        for child in self.content.children().iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        if let Some(cond) = self.next_conditional() {
            result.push_str(&cond.print_tree(indent));
        }
        result
    }
}

#[derive(Debug)]
pub struct ElseBlock {
    pub content: Scope,
}

impl ElseBlock {
    pub fn new(content: Scope) -> Self {
        Self { content }
    }
}

impl Conditional for ElseBlock {
    fn condition(&self, _env: &mut Environment) -> Result<Value, Error> {
        Ok(Value::Bool(true))
    }

    fn next_conditional(&self) -> Option<Box<dyn Conditional>> {
        None
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.clone();
        Box::new(ElseBlock::new(content))
    }

    fn content(&self) -> Scope {
        self.content.clone()
    }
}

impl ASTNode for ElseBlock {
    fn element(&self) -> String {
        "Else".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self.evaluate_conditional(env) {
            Ok(value) => match value {
                Some(value) => Ok(value),
                _ => self.content.eval(env, true),
            },
            Err(e) => Err(e),
        }
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let content = self.content.clone();
        Box::new(ElseBlock::new(content))
    }

    fn print_tree(&self, indent: usize) -> String {
        let mut result = String::new();
        result.push('|');
        result.push_str(&"--".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("|-content: \n");
        for child in self.content.children().iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        result
    }
}

#[derive(Debug)]
pub struct ElifBlock {
    pub condition: Box<dyn ASTNode>,
    pub content: Scope,
    pub next_conditional: Option<Box<dyn Conditional>>,
}

impl ElifBlock {
    pub fn new(condition: Box<dyn ASTNode>, content: Scope, next_conditional: Option<Box<dyn Conditional>>) -> Self {
        Self { condition, content, next_conditional }
    }
}

impl Conditional for ElifBlock {
    fn condition(&self, env: &mut Environment) -> Result<Value, Error> {
        self.condition.eval(env)
    }

    fn next_conditional(&self) -> Option<Box<dyn Conditional>> {
        self.next_conditional.as_ref().map(|c| c.clone_conditional())
    }

    fn clone_conditional(&self) -> Box<dyn Conditional> {
        let content = self.content.clone();
        Box::new(ElifBlock::new(self.condition.clone_to_node(), content, self.next_conditional.as_ref().map(|c| c.clone_conditional())))
    }

    fn content(&self) -> Scope {
        self.content.clone()
    }
}

impl ASTNode for ElifBlock {
    fn element(&self) -> String {
        "Elif".to_string()
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.content.children()
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
        result.push('|');
        result.push_str(&"--".repeat(indent));
        result.push_str(&self.element());
        result.push('\n');
        result.push_str(&"|   ".repeat(indent));
        result.push_str("|-condition: \n");
        result.push_str(&self.condition.print_tree(indent+1));
        result.push_str(&"|   ".repeat(indent));
        result.push_str("|-content: \n");
        for child in self.content.children().iter() {
            result.push_str(&child.print_tree(indent+1));
        }
        if let Some(cond) = self.next_conditional() {
            result.push_str(&cond.print_tree(indent));
        }
        result
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let content = self.content.clone();
        Box::new(ElifBlock::new(self.condition.clone_to_node(), content, self.next_conditional.as_ref().map(|c| c.clone_conditional())))
    }
}

