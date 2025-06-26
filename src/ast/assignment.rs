use crate::ast::{ASTNode,Error};
use crate::ast::value::Value;
use crate::environment::environment::Environment;
use crate::environment::environment::Type;
use crate::ast::variable_declaration::VariableDeclaration;

pub struct VariableAssignment {
    pub is_also_decl: bool,
    pub mutable: bool,
    pub type_: Option<Type>,
    pub name: String,
    pub expr: Box<dyn ASTNode>,
}

impl VariableAssignment {

    pub fn from_variable_declaration(variable_declaration: VariableDeclaration, expr: Box<dyn ASTNode>) -> Self {
        Self {
            is_also_decl: true,
            mutable: variable_declaration.mutable,
            type_: Some(variable_declaration.type_),
            name: variable_declaration.name,
            expr,
        }
    }
}

impl ASTNode for VariableAssignment {
    fn element(&self) -> String {
        let mutable = if self.mutable { "mut" } else if self.is_also_decl { "let" } else { "const" };
        let type_ = match &self.type_ {
            Some(type_) => format!("{:?}", type_),
            _ => "[inferred]".to_string(),
        };
        format!("{} {} : {} =", mutable, self.name, type_)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.expr.clone()]
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        let new_assignment = VariableAssignment {
            is_also_decl: self.is_also_decl,
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
            expr: self.expr.clone(),
        };
        Box::new(new_assignment)
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let value = self.expr.eval(env)?;
        if let Ok(_) = env.lookup_mut(&self.name) {
            if self.is_also_decl {
                env.declare_assign(self.name.clone(), value.clone(), self.mutable, self.type_.clone(), true)?;
            } else {
                env.assign(&self.name, value.clone())?;
            }
        } else {
            env.declare_assign(self.name.clone(), value.clone(), self.mutable, self.type_.clone(), false)?;
        }
        Ok(Value::Bool(true))
    }

}