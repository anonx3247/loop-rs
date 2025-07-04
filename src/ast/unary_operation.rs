use crate::ast::{ASTNode,value::Value};
use crate::environment::environment::{Environment, RuntimeError};
use crate::lexer::token::Operator;
use crate::Error;

#[derive(Debug)]
pub struct UnaryOperation {
    pub operand: Box<dyn ASTNode>,
    pub operator: Operator,
}

#[derive(Debug)]
pub enum UnaryOperationError {
    CannotPerform(Operator, Value),
}

impl ASTNode for UnaryOperation {
    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![self.operand.clone_to_node()]
    }

    fn element(&self) -> String {
        format!("{:?}", self.operator)
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        let operand = self.operand.clone_to_node();
        Box::new(UnaryOperation {
            operand,
            operator: self.operator.clone(),
        })
    }

    fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        let operand = self.operand.eval(env)?;
        Ok(match operand {
                Value::Int(l) => {
                    match self.operator {
                        Operator::Sub => Value::Int(-l),
                        Operator::BitNot => Value::Int(!l),
                        _ => return Err(Error::RuntimeError(RuntimeError::UnaryOperationError(UnaryOperationError::CannotPerform(self.operator.clone(), operand)))),
                    }
                }
                Value::Float(l) => {
                    match self.operator {
                        Operator::Sub => Value::Float(-l),
                        _ => return Err(Error::RuntimeError(RuntimeError::UnaryOperationError(UnaryOperationError::CannotPerform(self.operator.clone(), operand)))),
                    }
                }
                Value::Bool(l) => {
                    match self.operator {
                        Operator::Not => Value::Bool(!l),
                        _ => return Err(Error::RuntimeError(RuntimeError::UnaryOperationError(UnaryOperationError::CannotPerform(self.operator.clone(), operand)))),
                    }
                }
                _ => return Err(Error::RuntimeError(RuntimeError::UnaryOperationError(UnaryOperationError::CannotPerform(self.operator.clone(), operand)))),
            })
    }
}
