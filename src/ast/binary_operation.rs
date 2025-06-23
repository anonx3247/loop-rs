use crate::ast::{ASTNode,value::Value,Error};
use crate::environment::environment::Environment;
use crate::lexer::token;

pub struct BinaryOperation {
    pub left: Box<dyn ASTNode>,
    pub right: Box<dyn ASTNode>,
    pub operator: token::Operator,
}


impl ASTNode for BinaryOperation {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.left.clone(), self.right.clone()]
    }

    fn element(&self) -> String {
        format!("{:?}", self.operator)
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        let left = self.left.clone();
        let right = self.right.clone();
        Box::new(BinaryOperation {
            left,
            right,
            operator: self.operator.clone(),
        })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;
        Ok(match self.operator {
            token::Operator::Add 
            | token::Operator::Sub 
            | token::Operator::Mul 
            | token::Operator::Div 
            | token::Operator::Mod 
            | token::Operator::Pow 
            | token::Operator::BitShiftLeft 
            | token::Operator::BitShiftRight 
            | token::Operator::BitAnd 
            | token::Operator::BitOr 
            | token::Operator::BitXor 
            | token::Operator::Gt 
            | token::Operator::Lt 
            | token::Operator::Gte 
            | token::Operator::Lte => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => {
                        match self.operator {
                            token::Operator::Add => Value::Int(l + r),
                            token::Operator::Sub => Value::Int(l - r),
                            token::Operator::Mul => Value::Int(l * r),
                            token::Operator::Div => Value::Int(l / r),
                            token::Operator::Mod => Value::Int(l % r),
                            token::Operator::Pow => Value::Int(l.pow(*r as u32)),
                            token::Operator::BitShiftLeft => Value::Int(l << r),
                            token::Operator::BitShiftRight => Value::Int(l >> r),
                            token::Operator::BitAnd => Value::Int(l & r),
                            token::Operator::BitOr => Value::Int(l | r),
                            token::Operator::BitXor => Value::Int(l ^ r),
                            token::Operator::Gt => Value::Bool(l > r),
                            token::Operator::Lt => Value::Bool(l < r),
                            token::Operator::Gte => Value::Bool(l >= r),
                            token::Operator::Lte => Value::Bool(l <= r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::Float(l), Value::Float(r)) => {
                        match self.operator {
                            token::Operator::Add => Value::Float(l + r),
                            token::Operator::Sub => Value::Float(l - r),
                            token::Operator::Mul => Value::Float(l * r),
                            token::Operator::Div => Value::Float(l / r),
                            token::Operator::Mod => Value::Float(l % r),
                            token::Operator::Pow => Value::Float(l.powf(*r)),
                            token::Operator::Gt => Value::Bool(l > r),
                            token::Operator::Lt => Value::Bool(l < r),
                            token::Operator::Gte => Value::Bool(l >= r),
                            token::Operator::Lte => Value::Bool(l <= r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::String(l), Value::String(r)) => {
                        match self.operator {
                            token::Operator::Add => Value::String(l.clone() + &r.clone()),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    _ => return Err(Error::RuntimeError(format!("Cannot perform {:?} on {:?} and {:?}", self.operator, left, right))),
                }
            }
            token::Operator::Eq | token::Operator::Neq => {
                match (&left, &right) {
                    (Value::Int(l), Value::Int(r)) => {
                        match self.operator {
                            token::Operator::Eq => Value::Bool(*l == *r),
                            token::Operator::Neq => Value::Bool(*l != *r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::Float(l), Value::Float(r)) => {
                        match self.operator {
                            token::Operator::Eq => Value::Bool(*l == *r),
                            token::Operator::Neq => Value::Bool(*l != *r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::String(l), Value::String(r)) => {
                        match self.operator {
                            token::Operator::Eq => Value::Bool(*l == *r),
                            token::Operator::Neq => Value::Bool(*l != *r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::Bool(l), Value::Bool(r)) => {
                        match self.operator {
                            token::Operator::Eq => Value::Bool(*l == *r),
                            token::Operator::Neq => Value::Bool(*l != *r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    _ => return Err(Error::RuntimeError(format!("Cannot perform {:?} on {:?} and {:?}", self.operator, left, right))),
                }
            }
            token::Operator::And | token::Operator::Or => {
                match (&left, &right) {
                    (Value::Bool(l), Value::Bool(r)) => {
                        match self.operator {
                            token::Operator::And => Value::Bool(*l && *r),
                            token::Operator::Or => Value::Bool(*l || *r),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    _ => return Err(Error::RuntimeError(format!("Cannot perform {:?} on {:?} and {:?}", self.operator, left, right))),
                }
            }
            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
        })
    }
}
