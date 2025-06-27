#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
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
            Value::String(s) => write!(f, "{}", s),
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