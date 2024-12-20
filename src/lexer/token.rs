#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Literal(Literal),
    Operator(Operator),
    Type(Type),
    Function(Function),
    Module(Module),
    Loop(Loop),
    Conditional(Conditional),
    VariableDeclaration(VariableDeclaration),
    TypeDeclaration(TypeDeclaration),
    Bracket(Bracket),
    Punctuation(Punctuation),
    Identifier(String),
    Comment(Comment),
    Whitespace(Whitespace),
    Error(Error),
    EOF,
    Debug,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    None,
}


impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => a == b,
            (Literal::Float(a), Literal::Float(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::Char(a), Literal::Char(b)) => a == b,
            (Literal::None, Literal::None) => true,
            _ => false,
        }
    }
}

impl Eq for Literal {}
use std::hash::{Hash, Hasher};

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(val) => val.hash(state),
            Literal::Float(val) => val.to_bits().hash(state),
            Literal::String(val) => val.hash(state),
            Literal::Bool(val) => val.hash(state),
            Literal::Char(val) => val.hash(state),
            Literal::None => ().hash(state),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitShiftLeft,
    BitShiftRight,
    Assign,
    PlusAssign,
    MinusAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    Bool,
    String,
    Char,
    Int,
    UInt,
    Float,
    Generic(char),
    UserDefined(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Conditional {
    If,
    Else,
    Elif,
    Match,
    MatchArm,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Loop {
    For,
    While,
    Loop,
    Break,
    Continue,
    In,
    Range,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum VariableDeclaration {
    Let,
    Mut,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeDeclaration {
    Enum,
    Struct,
    Required,
    Class,
    Abstract,
    Implement,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Function {
    Async,
    Arrow,
    Return,
    Fn,
    Get,
    Set,
    Getter,
    Setter,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Module {
    Module,
    Import,
    From,
    As,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Punctuation {
    Dot,
    Comma,
    Colon,
    Arrow,
    Question,
    Exclamation,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Bracket {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Comment {
    SingleLine(String),
    MultiLine(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Whitespace {
    Newline,
    Space,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Error {
    Error,
    Except,
}

use std::collections::HashMap;

pub fn get_keywords_map() -> HashMap<&'static str, Token> {
    let mut keywords = HashMap::new();
    keywords.insert("async", Token::Function(Function::Async));
    keywords.insert("module", Token::Module(Module::Module));
    keywords.insert("import", Token::Module(Module::Import));
    keywords.insert("from", Token::Module(Module::From));
    keywords.insert("required", Token::TypeDeclaration(TypeDeclaration::Required));
    keywords.insert("struct", Token::TypeDeclaration(TypeDeclaration::Struct));
    keywords.insert("class", Token::TypeDeclaration(TypeDeclaration::Class));
    keywords.insert("abs", Token::TypeDeclaration(TypeDeclaration::Abstract));
    keywords.insert("impl", Token::TypeDeclaration(TypeDeclaration::Implement));
    keywords.insert("enum", Token::TypeDeclaration(TypeDeclaration::Enum));
    keywords.insert("fn", Token::Function(Function::Fn));
    keywords.insert("let", Token::VariableDeclaration(VariableDeclaration::Let));
    keywords.insert("mut", Token::VariableDeclaration(VariableDeclaration::Mut));
    keywords.insert("getter", Token::Function(Function::Getter));
    keywords.insert("setter", Token::Function(Function::Setter));
    keywords.insert("get", Token::Function(Function::Get));
    keywords.insert("set", Token::Function(Function::Set));
    keywords.insert("if", Token::Conditional(Conditional::If));
    keywords.insert("else", Token::Conditional(Conditional::Else));
    keywords.insert("match", Token::Conditional(Conditional::Match));
    keywords.insert("for", Token::Loop(Loop::For));
    keywords.insert("in", Token::Loop(Loop::In));
    keywords.insert("while", Token::Loop(Loop::While));
    keywords.insert("loop", Token::Loop(Loop::Loop));
    keywords.insert("ret", Token::Function(Function::Return));
    keywords.insert("break", Token::Loop(Loop::Break));
    keywords.insert("continue", Token::Loop(Loop::Continue));
    keywords.insert("dbg", Token::Debug);
    keywords
}

/* 
fn invert_map(map: HashMap<&'static str, TokenType>) -> HashMap<TokenType, &'static str> {
    map.into_iter().map(|(k, v)| (v, k)).collect()
}
*/
pub fn get_symbols_map() -> HashMap<&'static str, Token> {
    let mut symbols = HashMap::new();
    symbols.insert("+", Token::Operator(Operator::Add));
    symbols.insert("-", Token::Operator(Operator::Sub));
    symbols.insert("*", Token::Operator(Operator::Mul));
    symbols.insert("/", Token::Operator(Operator::Div));
    symbols.insert("%", Token::Operator(Operator::Mod));
    symbols.insert("**", Token::Operator(Operator::Pow));
    symbols.insert("==", Token::Operator(Operator::Eq));
    symbols.insert("!=", Token::Operator(Operator::Neq));
    symbols.insert(">", Token::Operator(Operator::Gt));
    symbols.insert("<", Token::Operator(Operator::Lt));
    symbols.insert(">=", Token::Operator(Operator::Gte));
    symbols.insert("<=", Token::Operator(Operator::Lte));
    symbols.insert("and", Token::Operator(Operator::And));
    symbols.insert("or", Token::Operator(Operator::Or));
    symbols.insert("not", Token::Operator(Operator::Not));
    symbols.insert("&", Token::Operator(Operator::BitAnd));
    symbols.insert("|", Token::Operator(Operator::BitOr));
    symbols.insert("<<", Token::Operator(Operator::BitShiftLeft));
    symbols.insert(">>", Token::Operator(Operator::BitShiftRight));
    symbols.insert("=>", Token::Conditional(Conditional::MatchArm));
    symbols.insert("->", Token::Function(Function::Arrow));
    symbols.insert("..", Token::Loop(Loop::Range));
    symbols.insert(":", Token::Punctuation(Punctuation::Colon));
    symbols.insert(",", Token::Punctuation(Punctuation::Comma));
    symbols.insert(".", Token::Punctuation(Punctuation::Dot));
    symbols.insert("(", Token::Bracket(Bracket::OpenParen));
    symbols.insert(")", Token::Bracket(Bracket::CloseParen));
    symbols.insert("{", Token::Bracket(Bracket::OpenBrace));
    symbols.insert("}", Token::Bracket(Bracket::CloseBrace));
    symbols.insert("[", Token::Bracket(Bracket::OpenBracket));
    symbols.insert("]", Token::Bracket(Bracket::CloseBracket));
    symbols.insert(":=", Token::Operator(Operator::Assign));
    symbols.insert("+=", Token::Operator(Operator::PlusAssign));
    symbols.insert("-=", Token::Operator(Operator::MinusAssign));
    symbols.insert("*=", Token::Operator(Operator::MulAssign));
    symbols.insert("/=", Token::Operator(Operator::DivAssign));
    symbols.insert("%=", Token::Operator(Operator::ModAssign));
    symbols
}

pub fn get_base_types_map() -> HashMap<&'static str, Token> {
    let mut base_types = HashMap::new();
    base_types.insert("u8", Token::Type(Type::U8));
    base_types.insert("u16", Token::Type(Type::U16));
    base_types.insert("u32", Token::Type(Type::U32));
    base_types.insert("u64", Token::Type(Type::U64));
    base_types.insert("i16", Token::Type(Type::I16));
    base_types.insert("i32", Token::Type(Type::I32));
    base_types.insert("i64", Token::Type(Type::I64));
    base_types.insert("f32", Token::Type(Type::F32));
    base_types.insert("f64", Token::Type(Type::F64));
    base_types.insert("bool", Token::Type(Type::Bool));
    base_types.insert("string", Token::Type(Type::String));
    base_types.insert("char", Token::Type(Type::Char));
    base_types.insert("int", Token::Type(Type::Int));
    base_types.insert("uint", Token::Type(Type::UInt));
    base_types.insert("float", Token::Type(Type::Float));
    base_types
}

impl Token {

    pub fn from_keyword(keyword: &str) -> Result<Self, String> {
        let keywords = get_keywords_map();
        match keywords.get(keyword) {
            Some(token) => Ok(token.clone()),
            _ => Err(format!("Unknown keyword: {}", keyword)),
        }
    }

    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let symbols = get_symbols_map();
        match symbols.get(symbol) {
            Some(token) => Ok(token.clone()),
            _ => Err(format!("Unknown symbol: {}", symbol)),
        }
    }

    pub fn from_base_type(base_type: &str) -> Result<Self, String> {
        let base_types = get_base_types_map();
        match base_types.get(base_type) {
            Some(token) => Ok(token.clone()),
            _ => Err(format!("Unknown base type: {}", base_type)),
        }
    }

    pub fn identifier(identifier: &str) -> Self {
        Token::Identifier(identifier.to_string())
    }

    pub fn comment(comment: &str) -> Self {
        Token::Comment(Comment::SingleLine(comment.to_string()))
    }

    pub fn literal(literal: Literal) -> Self {
        Token::Literal(literal)
    }

    pub fn custom_type(custom_type: &str) -> Self {
        match custom_type.len() {
            1 => Token::Type(Type::Generic(custom_type.chars().next().unwrap())),
            _ => Token::Type(Type::UserDefined(custom_type.to_string())),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Token::Operator(op) => format!("{:?}", &op),
            Token::Function(func) => format!("{:?}", &func),
            Token::Bracket(bracket) => format!("{:?}", &bracket),
            Token::Conditional(cond) => format!("{:?}", &cond),
            Token::Punctuation(punct) => format!("{:?}", &punct),
            Token::VariableDeclaration(var_decl) => format!("{:?}", &var_decl),
            Token::TypeDeclaration(type_decl) => format!("{:?}", &type_decl),
            Token::Loop(loop_) => format!("{:?}", &loop_),
            Token::Literal(literal) => match literal {
                Literal::String(s) => s.clone(),
                Literal::None => "None".to_string(),
                _ => format!("{:?}", &literal),
            },
            Token::Identifier(identifier) => identifier.clone(),
            Token::Comment(comment) => match comment {
                Comment::SingleLine(s) => s.clone(),
                Comment::MultiLine(s) => s.clone(),
            },
            Token::Whitespace(whitespace) => match whitespace {
                Whitespace::Newline => String::from("Newline"),
                Whitespace::Space => String::from("Space"),
            },
            Token::Error(error) => match error {
                Error::Error => String::from("Error"),
                Error::Except => String::from("Except"),
            },
            Token::EOF => String::from("EOF"),
            Token::Type(_type) => match _type {
                Type::Generic(c) => c.to_string(),
                Type::UserDefined(s) => s.clone(),
                Type::U8 => String::from("u8"),
                Type::U16 => String::from("u16"),
                Type::U32 => String::from("u32"),
                Type::U64 => String::from("u64"),
                Type::I16 => String::from("i16"),
                Type::I32 => String::from("i32"),
                Type::I64 => String::from("i64"),
                Type::F32 => String::from("f32"),
                Type::F64 => String::from("f64"),
                Type::Bool => String::from("bool"),
                Type::String => String::from("string"),
                Type::Char => String::from("char"),
                Type::Int => String::from("int"),
                Type::UInt => String::from("uint"),
                Type::Float => String::from("float"),
            },
            Token::Module(_module) => match _module {
                Module::Module => String::from("Module"),
                Module::Import => String::from("Import"),
                Module::From => String::from("From"),
                Module::As => String::from("As"),
            },
            Token::Debug => String::from("Debug"),
        }
    }
}
