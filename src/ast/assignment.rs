use crate::ast::{ASTNode,Error};
use crate::ast::value::Value;
use crate::environment::environment::Environment;
use crate::lexer::token::Type;

pub struct VariableAssignment {
    pub mutable: bool,
    pub type_: Option<Type>,
    pub name: String,
    pub expr: Box<dyn ASTNode>,
}

impl ASTNode for VariableAssignment {
    fn element(&self) -> String {
        let mutable = if self.mutable { "mut" } else { "const" };
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
            mutable: self.mutable,
            type_: self.type_.clone(),
            name: self.name.clone(),
            expr: self.expr.clone(),
        };
        Box::new(new_assignment)
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let value = self.expr.eval(env)?;
        if let Ok(mutable) = env.lookup_mut(&self.name) {
            if !mutable {
                return Err(Error::RuntimeError(format!(
                    "Cannot assign to immutable variable '{}'",
                    self.name
                )));
            }
            env.assign(&self.name, value.clone())?;
        } else {
            env.declare(self.name.clone(), value.clone(), self.mutable)?;
        }
        Ok(Value::Bool(true))
    }

}