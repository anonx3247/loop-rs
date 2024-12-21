use crate::ast::{ASTNode,Error};
use crate::ast::value::Value;
use crate::lexer::token::Type;

pub struct VariableAssignment {
    pub mutable: bool,
    pub type_: Type,
    pub name: String,
    pub expr: Box<dyn ASTNode>,
}

impl ASTNode for VariableAssignment {
    fn element(&self) -> String {
        let mutable = if self.mutable { "mut" } else { "const" };
        let type_ = format!("{:?}", self.type_);
        format!("{} {} : {} =", mutable, self.name, type_)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        self.expr.children()
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        let new_assignment = VariableAssignment {
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
            expr: self.expr.clone(),
        };
        Box::new(new_assignment)
    }

    fn eval(&self) -> Result<Value, Error> {
        self.expr.eval()
    }

}