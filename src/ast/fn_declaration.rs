use crate::ast::{ASTNode, Value, Error};
use std::collections::HashMap;
use crate::lexer::token::Type;

pub struct FnDeclaration {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub body: Vec<Box<dyn ASTNode>>,
}

impl FnDeclaration {
    pub fn new(name: String, params: HashMap<String, Type>, body: Vec<Box<dyn ASTNode>>) -> Self {
        Self { name, params, body }
    }
}
