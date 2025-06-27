use crate::ast::{ASTNode,Value, Error};
use crate::environment::environment::{Environment};
use crate::lexer::token;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
    Generic(char),
    UserDefined(String),
    Option(Box<Type>),
    Tuple(Vec<Type>),
}

impl Type {
    pub fn from_token_type(token: token::Type) -> Self {
        match token {
            token::Type::U8 => Type::U8,
            token::Type::U16 => Type::U16,
            token::Type::U32 => Type::U32,
            token::Type::U64 => Type::U64,
            token::Type::I16 => Type::I16,
            token::Type::I32 => Type::I32,
            token::Type::I64 => Type::I64,
            token::Type::F32 => Type::F32,
            token::Type::F64 => Type::F64,
            token::Type::String => Type::String,
            token::Type::Bool => Type::Bool,
            token::Type::U8Option => Type::Option(Box::new(Type::U8)),
            token::Type::U16Option => Type::Option(Box::new(Type::U16)),
            token::Type::U32Option => Type::Option(Box::new(Type::U32)),
            token::Type::U64Option => Type::Option(Box::new(Type::U64)),
            token::Type::I16Option => Type::Option(Box::new(Type::I16)),
            token::Type::I32Option => Type::Option(Box::new(Type::I32)),
            token::Type::I64Option => Type::Option(Box::new(Type::I64)),
            token::Type::F32Option => Type::Option(Box::new(Type::F32)),
            token::Type::F64Option => Type::Option(Box::new(Type::F64)),
            token::Type::BoolOption => Type::Option(Box::new(Type::Bool)),
            token::Type::StringOption => Type::Option(Box::new(Type::String)),
            token::Type::Generic(c) => Type::Generic(c),
            token::Type::UserDefined(s) => Type::UserDefined(s),
        }
    }
}

impl ASTNode for Type {
    fn element(&self) -> String {
        format!("{:?}", self)
    }

    fn children(&self) -> Vec<Box<dyn ASTNode>> {
        vec![]
    }

    fn eval(&self, _env: &mut Environment) -> Result<Value, Error> {
        Ok(Value::None)
    }

    fn clone_to_node(&self) -> Box<dyn ASTNode> {
        Box::new(self.clone())
    }
}