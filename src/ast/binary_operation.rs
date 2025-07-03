use crate::ast::{ASTNode,value::Value};
use crate::environment::environment::{Environment, RuntimeError};
use crate::lexer::token::Operator;
use crate::Error;

#[derive(Debug)]
pub struct BinaryOperation {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub operator: Operator,
}

#[derive(Debug)]
pub enum BinaryOperationError {
    CannotPerform(Operator, Value, Value),
}

impl ASTNode for BinaryOperation {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.left.clone_to_node(), self.right.clone_to_node()]
    }

    fn element(&self) -> String {
        format!("{:?}", self.operator)
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let left = self.left.clone_to_node();
        let right = self.right.clone_to_node();
        Box::new(BinaryOperation {
            left,
            right,
            operator: self.operator.clone(),
        })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;
        Ok(match (&left, &right) {
                (Value::Int(l), Value::Int(r)) => {
                    match self.operator {
                        Operator::Add => Value::Int(l + r),
                        Operator::Sub => Value::Int(l - r),
                        Operator::Mul => Value::Int(l * r),
                        Operator::Div => Value::Int(l / r),
                        Operator::Mod => Value::Int(l % r),
                        Operator::Pow => Value::Int(l.pow(*r as u32)),
                        Operator::BitShiftLeft => Value::Int(l << r),
                        Operator::BitShiftRight => Value::Int(l >> r),
                        Operator::BitAnd => Value::Int(l & r),
                        Operator::BitOr => Value::Int(l | r),
                        Operator::BitXor => Value::Int(l ^ r),
                        Operator::Gt => Value::Bool(l > r),
                        Operator::Lt => Value::Bool(l < r),
                        Operator::Gte => Value::Bool(l >= r),
                        Operator::Lte => Value::Bool(l <= r),
                        Operator::Eq => Value::Bool(*l == *r),
                        Operator::Neq => Value::Bool(*l != *r),
                        _ => return Err(Error::RuntimeError(RuntimeError::BinaryOperationError(BinaryOperationError::CannotPerform(self.operator.clone(), left, right)))),
                    }
                }
                (Value::Float(l), Value::Float(r)) => {
                    match self.operator {
                        Operator::Add => Value::Float(l + r),
                        Operator::Sub => Value::Float(l - r),
                        Operator::Mul => Value::Float(l * r),
                        Operator::Div => Value::Float(l / r),
                        Operator::Mod => Value::Float(l % r),
                        Operator::Pow => Value::Float(l.powf(*r)),
                        Operator::Gt => Value::Bool(l > r),
                        Operator::Lt => Value::Bool(l < r),
                        Operator::Gte => Value::Bool(l >= r),
                        Operator::Lte => Value::Bool(l <= r),
                        Operator::Eq => Value::Bool(*l == *r),
                        Operator::Neq => Value::Bool(*l != *r),
                        _ => return Err(Error::RuntimeError(RuntimeError::BinaryOperationError(BinaryOperationError::CannotPerform(self.operator.clone(), left, right)))),
                    }
                }
                (Value::String(l, _), Value::String(r, _)) => {
                    match self.operator {
                        Operator::Add => Value::String(l.clone() + &r.clone(), false),
                        _ => return Err(Error::RuntimeError(RuntimeError::BinaryOperationError(BinaryOperationError::CannotPerform(self.operator.clone(), left, right)))),
                    }
                }
                (Value::Bool(l), Value::Bool(r)) => {
                    match self.operator {
                        Operator::Eq => Value::Bool(*l == *r),
                        Operator::Neq => Value::Bool(*l != *r),
                        Operator::And => Value::Bool(*l && *r),
                        Operator::Or => Value::Bool(*l || *r),
                        _ => return Err(Error::RuntimeError(RuntimeError::BinaryOperationError(BinaryOperationError::CannotPerform(self.operator.clone(), left, right)))),
                    }
                }
                _ => return Err(Error::RuntimeError(RuntimeError::BinaryOperationError(BinaryOperationError::CannotPerform(self.operator.clone(), left, right)))),
            })
    }
}
