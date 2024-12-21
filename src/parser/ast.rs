use crate::lexer::token;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub enum Error {
    SyntaxError(String),
    RuntimeError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyntaxError(message) => write!(f, "Syntax error: {}", message),
            Error::RuntimeError(message) => write!(f, "Runtime error: {}", message),
        }
    }
}

pub trait ASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>>;
    fn element(&self) -> String;
    fn add_child(&mut self, child: Box<dyn ASTNode>);
    fn to_string(&self, indent: usize) -> String {
        let mut result = String::new();
        if indent == 1 {
            result.push_str(&"|--");
        } else if indent > 1 {
            result.push_str(&"|   ".repeat(indent - 1));
            result.push_str(&"|--");
        }
        result.push_str(&self.element());
        result.push('\n');
        for child in self.children() {
            result.push_str(&child.to_string(indent + 1));
        }
        result
    }
    fn eval(&self) -> Result<Value, Error>;
    fn clone(&self) -> Box<dyn ASTNode>;
}

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

    fn add_child(&mut self, child: Box<dyn ASTNode>) {
        panic!("BinaryOperation cannot add children");
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

    fn eval(&self) -> Result<Value, Error> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
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

pub struct Literal(Value);

impl Literal {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn from_token(token: token::Token) -> Result<Self, Error> {
        Ok(Self(match token {
            token::Token::Literal(token::Literal::Int(value)) => Value::Int(value),
            token::Token::Literal(token::Literal::Float(value)) => Value::Float(value),
            token::Token::Literal(token::Literal::String(value)) => Value::String(value),
            token::Token::Literal(token::Literal::Bool(value)) => Value::Bool(value),
            _ => return Err(Error::SyntaxError(format!("Invalid literal token: {:?}", token))),
        }))
    }
}

impl ASTNode for Literal {
    fn element(&self) -> String {
        format!("{:?}", self.0)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn add_child(&mut self, _child: Box<dyn ASTNode>) {
        panic!("Literal cannot have children");
    }

    fn eval(&self) -> Result<Value, Error> {
        Ok(self.0.clone())
    }

    fn clone(&self) -> Box<dyn ASTNode> {
        Box::new(Literal(self.0.clone()))
    }
}
