use crate::ast::{ASTNode};
use std::collections::HashMap;
use crate::lexer::token::Type;

pub struct FnDeclaration {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub body: Vec<Box<dyn ASTNode>>,
}