#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    token_type: TokenType,
}

use std::collections::HashMap;

pub fn get_keywords_map() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("async", TokenType::Function(Function::Async));
    keywords.insert("module", TokenType::Module(Module::Module));
    keywords.insert("import", TokenType::Module(Module::Import));
    keywords.insert("from", TokenType::Module(Module::From));
    keywords.insert("required", TokenType::TypeDeclaration(TypeDeclaration::Required));
    keywords.insert("struct", TokenType::TypeDeclaration(TypeDeclaration::Struct));
    keywords.insert("class", TokenType::TypeDeclaration(TypeDeclaration::Class));
    keywords.insert("abs", TokenType::TypeDeclaration(TypeDeclaration::Abstract));
    keywords.insert("impl", TokenType::TypeDeclaration(TypeDeclaration::Implement));
    keywords.insert("enum", TokenType::TypeDeclaration(TypeDeclaration::Enum));
    keywords.insert("fn", TokenType::Function(Function::Fn));
    keywords.insert("let", TokenType::VariableDeclaration(VariableDeclaration::Let));
    keywords.insert("mut", TokenType::VariableDeclaration(VariableDeclaration::Mut));
    keywords.insert("getter", TokenType::Function(Function::Getter));
    keywords.insert("setter", TokenType::Function(Function::Setter));
    keywords.insert("get", TokenType::Function(Function::Get));
    keywords.insert("set", TokenType::Function(Function::Set));
    keywords.insert("if", TokenType::Conditional(Conditional::If));
    keywords.insert("else", TokenType::Conditional(Conditional::Else));
    keywords.insert("match", TokenType::Conditional(Conditional::Match));
    keywords.insert("for", TokenType::Loop(Loop::For));
    keywords.insert("while", TokenType::Loop(Loop::While));
    keywords.insert("loop", TokenType::Loop(Loop::Loop));
    keywords.insert("ret", TokenType::Function(Function::Return));
    keywords.insert("break", TokenType::Loop(Loop::Break));
    keywords.insert("continue", TokenType::Loop(Loop::Continue));
    keywords.insert("dbg", TokenType::Debug);
    keywords
}

/* 
fn invert_map(map: HashMap<&'static str, TokenType>) -> HashMap<TokenType, &'static str> {
    map.into_iter().map(|(k, v)| (v, k)).collect()
}
*/
pub fn get_symbols_map() -> HashMap<&'static str, TokenType> {
    let mut symbols = HashMap::new();
    symbols.insert("+", TokenType::Operator(Operator::Add));
    symbols.insert("-", TokenType::Operator(Operator::Sub));
    symbols.insert("*", TokenType::Operator(Operator::Mul));
    symbols.insert("/", TokenType::Operator(Operator::Div));
    symbols.insert("%", TokenType::Operator(Operator::Mod));
    symbols.insert("**", TokenType::Operator(Operator::Pow));
    symbols.insert("==", TokenType::Operator(Operator::Eq));
    symbols.insert("!=", TokenType::Operator(Operator::Neq));
    symbols.insert(">", TokenType::Operator(Operator::Gt));
    symbols.insert("<", TokenType::Operator(Operator::Lt));
    symbols.insert(">=", TokenType::Operator(Operator::Gte));
    symbols.insert("<=", TokenType::Operator(Operator::Lte));
    symbols.insert("and", TokenType::Operator(Operator::And));
    symbols.insert("or", TokenType::Operator(Operator::Or));
    symbols.insert("not", TokenType::Operator(Operator::Not));
    symbols.insert("&", TokenType::Operator(Operator::BitAnd));
    symbols.insert("|", TokenType::Operator(Operator::BitOr));
    symbols.insert("<<", TokenType::Operator(Operator::BitShiftLeft));
    symbols.insert(">>", TokenType::Operator(Operator::BitShiftRight));
    symbols.insert("=>", TokenType::Conditional(Conditional::MatchArm));
    symbols.insert("->", TokenType::Function(Function::Arrow));
    symbols.insert("..", TokenType::Loop(Loop::Range));
    symbols.insert(":", TokenType::Punctuation(Punctuation::Colon));
    symbols.insert(",", TokenType::Punctuation(Punctuation::Comma));
    symbols.insert(".", TokenType::Punctuation(Punctuation::Dot));
    symbols.insert("(", TokenType::Bracket(Bracket::OpenParen));
    symbols.insert(")", TokenType::Bracket(Bracket::CloseParen));
    symbols.insert("{", TokenType::Bracket(Bracket::OpenBrace));
    symbols.insert("}", TokenType::Bracket(Bracket::CloseBrace));
    symbols.insert("[", TokenType::Bracket(Bracket::OpenBracket));
    symbols.insert("]", TokenType::Bracket(Bracket::CloseBracket));
    symbols
}

pub fn get_base_types_map() -> HashMap<&'static str, TokenType> {
    let mut base_types = HashMap::new();
    base_types.insert("u8", TokenType::Type(Type::U8));
    base_types.insert("u16", TokenType::Type(Type::U16));
    base_types.insert("u32", TokenType::Type(Type::U32));
    base_types.insert("u64", TokenType::Type(Type::U64));
    base_types.insert("i16", TokenType::Type(Type::I16));
    base_types.insert("i32", TokenType::Type(Type::I32));
    base_types.insert("i64", TokenType::Type(Type::I64));
    base_types.insert("f32", TokenType::Type(Type::F32));
    base_types.insert("f64", TokenType::Type(Type::F64));
    base_types.insert("bool", TokenType::Type(Type::Bool));
    base_types.insert("string", TokenType::Type(Type::String));
    base_types.insert("char", TokenType::Type(Type::Char));
    base_types.insert("int", TokenType::Type(Type::Int));
    base_types.insert("uint", TokenType::Type(Type::UInt));
    base_types.insert("float", TokenType::Type(Type::Float));
    base_types
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }

    pub fn from_keyword(keyword: &str) -> Result<Self, String> {
        let keywords = get_keywords_map();
        match keywords.get(keyword) {
            Some(token_type) => Ok(Token::new(token_type.clone())),
            None => Err(format!("Unknown keyword: {}", keyword)),
        }
    }

    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let symbols = get_symbols_map();
        match symbols.get(symbol) {
            Some(token_type) => Ok(Token::new(token_type.clone())),
            None => Err(format!("Unknown symbol: {}", symbol)),
        }
    }

    pub fn from_base_type(base_type: &str) -> Result<Self, String> {
        let base_types = get_base_types_map();
        match base_types.get(base_type) {
            Some(token_type) => Ok(Token::new(token_type.clone())),
            None => Err(format!("Unknown base type: {}", base_type)),
        }
    }

    pub fn identifier(identifier: &str) -> Self {
        Token::new(TokenType::Identifier(identifier.to_string()))
    }

    pub fn comment(comment: &str) -> Self {
        Token::new(TokenType::Comment(Comment::SingleLine(comment.to_string())))
    }

    pub fn literal(literal: Literal) -> Self {
        Token::new(TokenType::Literal(literal))
    }

    pub fn custom_type(custom_type: &str) -> Self {
        match custom_type.len() {
            1 => Token::new(TokenType::Type(Type::Generic(custom_type.chars().next().unwrap()))),
            _ => Token::new(TokenType::Type(Type::UserDefined(custom_type.to_string()))),
        }
    }
    pub fn to_string(&self) -> String {
        match &self.token_type {
            TokenType::Operator(op) => format!("{:?}", &op),
            TokenType::Function(func) => format!("{:?}", &func),
            TokenType::Bracket(bracket) => format!("{:?}", &bracket),
            TokenType::Conditional(cond) => format!("{:?}", &cond),
            TokenType::Punctuation(punct) => format!("{:?}", &punct),
            TokenType::VariableDeclaration(var_decl) => format!("{:?}", &var_decl),
            TokenType::TypeDeclaration(type_decl) => format!("{:?}", &type_decl),
            TokenType::Loop(loop_) => format!("{:?}", &loop_),
            TokenType::Literal(literal) => match literal {
                Literal::String(s) => s.clone(),
                Literal::None => "None".to_string(),
                _ => format!("{:?}", &literal),
            },
            TokenType::Identifier(identifier) => identifier.clone(),
            TokenType::Comment(comment) => match comment {
                Comment::SingleLine(s) => s.clone(),
                Comment::MultiLine(s) => s.clone(),
            },
            TokenType::Whitespace(whitespace) => match whitespace {
                Whitespace::Newline => String::from("Newline"),
                Whitespace::Space => String::from("Space"),
            },
            TokenType::Error(error) => match error {
                Error::Error => String::from("Error"),
                Error::Except => String::from("Except"),
            },
            TokenType::EOF => String::from("EOF"),
            TokenType::Type(_type) => match _type {
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
            TokenType::Module(_module) => match _module {
                Module::Module => String::from("Module"),
                Module::Import => String::from("Import"),
                Module::From => String::from("From"),
                Module::As => String::from("As"),
            },
            TokenType::Debug => String::from("Debug"),
        }
    }
}
