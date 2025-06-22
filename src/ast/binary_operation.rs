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
                            token::Operator::Add => Value::Int(r + l),
                            token::Operator::Sub => Value::Int(r - l),
                            token::Operator::Mul => Value::Int(r * l),
                            token::Operator::Div => Value::Int(r / l),
                            token::Operator::Mod => Value::Int(r % l),
                            token::Operator::Pow => Value::Int(r.pow(*l as u32)),
                            token::Operator::BitShiftLeft => Value::Int(r << l),
                            token::Operator::BitShiftRight => Value::Int(r >> l),
                            token::Operator::BitAnd => Value::Int(r & l),
                            token::Operator::BitOr => Value::Int(r | l),
                            token::Operator::BitXor => Value::Int(r ^ l),
                            token::Operator::Gt => Value::Bool(r > l),
                            token::Operator::Lt => Value::Bool(r < l),
                            token::Operator::Gte => Value::Bool(r >= l),
                            token::Operator::Lte => Value::Bool(r <= l),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::Float(l), Value::Float(r)) => {
                        match self.operator {
                            token::Operator::Add => Value::Float(r + l),
                            token::Operator::Sub => Value::Float(r - l),
                            token::Operator::Mul => Value::Float(r * l),
                            token::Operator::Div => Value::Float(r / l),
                            token::Operator::Mod => Value::Float(r % l),
                            token::Operator::Pow => Value::Float(r.powf(*l)),
                            token::Operator::Gt => Value::Bool(r > l),
                            token::Operator::Lt => Value::Bool(r < l),
                            token::Operator::Gte => Value::Bool(r >= l),
                            token::Operator::Lte => Value::Bool(r <= l),
                            _ => return Err(Error::RuntimeError(format!("Invalid operator: {:?}", self.operator))),
                        }
                    }
                    (Value::String(l), Value::String(r)) => {
                        match self.operator {
                            token::Operator::Add => Value::String(r.clone() + &l.clone()),
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
