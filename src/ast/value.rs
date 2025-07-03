use crate::ast::tuple::{Clonable, Tuple, TupleLike};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String, bool),
    Bool(bool),
    Tuple(Vec<Value>),
    None,
    Error(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s, _) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::None => write!(f, "none"),
            Value::Tuple(values) => {
                let mut result = String::new();
                for value in values[..values.len() - 1].iter() {
                    result.push_str(&value.to_string());
                    result.push_str(", ");
                }
                result.push_str(&values[values.len() - 1].to_string());
                write!(f, "({})", result)
            }
            Value::Error(e) => write!(f, "error({})", e),
        }
    }
}

impl Clonable for Value {
    fn clone_element(&self) -> Self {
        self.clone()
    }
}

impl TupleLike<Value> for Value {
    fn to_tuple(&self) -> Tuple<Value> {
        match self {
            Value::Tuple(values) => {
                let mut tuple_values = Vec::new();
                for value in values {
                    tuple_values.push(value.to_tuple());
                }
                if tuple_values.len() == 0 {
                    Tuple::Empty
                } else if tuple_values.len() == 1 {
                    tuple_values[0].clone()
                } else {
                    Tuple::List(tuple_values)
                }
            },
            _ => Tuple::Element(self.clone()),
        }
    }
}