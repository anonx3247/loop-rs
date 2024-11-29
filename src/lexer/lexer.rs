use crate::lexer::tokens::*;

pub struct Lexer {
    source: String,
    index: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            index: 0,
            tokens: Vec::new(),
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            source: String::new(),
            index: 0,
            tokens,
        }
    }

    pub fn tokenize_next(&mut self) -> Result<Token, String> {
        let result = match self.source {
            _ if self.source.starts_with(' ') 
            || self.source.starts_with('\t') 
            || self.source.starts_with('\n') => Self::tokenize_whitespace(&self.source),
            _ if self.source.starts_with('"') 
            || self.source.starts_with('\'')
            || self.source.starts_with("true") 
            || self.source.starts_with("false") 
            || self.source.starts_with("none") 
            || self.source.chars().next().unwrap().is_digit(10) && (self.source.contains('e') || self.source.contains('E') || self.source.contains('.'))
            || self.source.chars().next().map_or(false, |c| c.is_numeric()) => Self::tokenize_literal(&self.source),
            _ if self.source.starts_with("--") => Self::tokenize_comment(&self.source),
            _ if self.source.chars().next().unwrap().is_alphabetic() => match self.source.chars().next().unwrap().is_uppercase() {
                false => Self::tokenize_keyword(&self.source),
                true => Self::tokenize_custom_type(&self.source),
            },
            _ if self.source.chars().next().map_or(false, |c| c.is_alphanumeric()) => Self::tokenize_identifier(&self.source),
            _ => Self::tokenize_symbol(&self.source),
        };
        let (token, index) = match result {
            Ok(result) => result,
            Err(e) => return Err(e),
        };

        self.index += index;
        self.tokens.push(token.clone());
        Ok(token)
    }

    pub fn tokenize_symbol(symbol: &String) -> Result<(Token, usize), String> {
        for (symbol, token) in get_symbols_map() {
            if symbol.starts_with(symbol) {
                return Ok((Token::new(token), symbol.len()));
            }
        }
        Err(symbol.clone())
    }

    pub fn tokenize_keyword(keyword: &String) -> Result<(Token, usize), String> {
        for (keyword, token) in get_keywords_map() {
            if keyword.starts_with(keyword) {
                return Ok((Token::new(token), keyword.len()));
            }
        }
        Err(keyword.clone())
    }

    pub fn tokenize_base_type(base_type: &String) -> Result<(Token, usize), String> {
        for (base_type, token) in get_base_types_map() {
            if base_type.starts_with(base_type) {
                return Ok((Token::new(token), base_type.len()));
            }
        }
        Err(base_type.clone())
    }

    pub fn tokenize_custom_type(custom_type: &String) -> Result<(Token, usize), String> {
        let (custom_type, index) = index_until_boundary(custom_type.as_str());
        Ok((Token::custom_type(custom_type), index))
    }

    pub fn tokenize_literal(literal: &String) -> Result<(Token, usize), String> {
        let (literal, index) = match Literal::tokenize_literal(literal) {
            Ok((literal, index)) => (literal, index),
            Err(e) => return Err(e),
        };
        Ok((Token::new(TokenType::Literal(literal)), index))
    }

    pub fn tokenize_comment(comment: &String) -> Result<(Token, usize), String> {
        let (comment, index) = index_until_char(comment.as_str(), '\n');
        Ok((Token::new(TokenType::Comment(Comment::SingleLine(String::from(comment)))), index))
    }   

    pub fn tokenize_whitespace(whitespace: &String) -> Result<(Token, usize), String> {
        let (_, index) = index_until_whitespace(whitespace.as_str());
        Ok((Token::new(TokenType::Whitespace(Whitespace::Space)), index))
    }

    pub fn tokenize_identifier(identifier: &String) -> Result<(Token, usize), String> {
        let (identifier, index) = index_until_boundary(identifier.as_str());
        Ok((Token::new(TokenType::Identifier(String::from(identifier))), index))
    }
}



fn index_until_whitespace(literal: &str) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if c.is_whitespace() {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index)
}

fn index_until_boundary(literal: &str) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if c.is_whitespace() || !c.is_alphanumeric() {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index)
}

fn index_until_char(literal: &str, char: char) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if c == char {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index) 
}

impl Literal {
    fn tokenize_float(literal: &String) -> Result<(Literal, usize), String> {
        let (literal, index) = index_until_boundary(literal.as_str());

        let tokenize_float_with_split_char = |split_char: char| -> Result<(Literal, usize), String> {
            let parts = literal.split(split_char).collect::<Vec<&str>>();
            if parts.len() == 2 && parts[1].chars().all(|c| c.is_digit(10)) {
                if let (Ok(base), Ok(exp)) = (parts[0].parse::<f64>(), parts[1].parse::<i32>()) {
                    Ok((Literal::Float(base.powf(exp as f64)), index))
                } else {
                    Err(String::from(literal))
                }
            } else {
                Err(String::from(literal))
            }
        };

        match literal {
            _ if literal.contains('e') => tokenize_float_with_split_char('e'),
            _ if literal.contains('E') => tokenize_float_with_split_char('E'),
            _ => match literal.parse::<f64>() {
                Ok(f) => Ok((Literal::Float(f), index)),
                Err(e) => Err(e.to_string()),
            },
        }
    }

    fn tokenize_int(literal: &String) -> Result<(Literal, usize), String> {
        let (literal, index) = index_until_boundary(literal.as_str());
    
        match literal.parse::<i64>() {
            Ok(i) => Ok((Literal::Int(i), index)),
            Err(e) => Err(e.to_string()),
        }
    }
    
    pub fn tokenize_string(literal: &String) -> Result<(Literal, usize), String> {
        // parse until matching quote character
        fn parse_string_until_quote(literal: &str, quote: char) -> Result<(Literal, usize), String> {
            let (literal, index) = index_until_char(literal, quote);
            if index == literal.len() {
                Err(String::from(literal))
            } else {
                Ok((Literal::String(literal[..index].to_string()), index))
            }
        }
    
        match literal {
            _ if literal.starts_with('"') => parse_string_until_quote(&literal[1..], '"'),
            _ if literal.starts_with('\'') => parse_string_until_quote(&literal[1..], '\''),
            _ => Err(literal.clone()),
        }
    }
    
    pub fn tokenize_literal(literal: &String) -> Result<(Literal, usize), String> {
        match literal {
            _ if literal.starts_with("true") => Ok((Literal::Bool(true), 4)),
            _ if literal.starts_with("false") => Ok((Literal::Bool(false), 5)),
            _ if literal.starts_with("none") => Ok((Literal::None, 4)),
            _ if literal.starts_with('"') => Self::tokenize_string(literal),
            _ if literal.contains('e') || literal.contains('E') || literal.contains('.') => Self::tokenize_float(literal),
            _ => Self::tokenize_int(literal),
        }
    }
}

pub fn tokenize_identifier(identifier: &String) -> Result<(TokenType, usize), String> {
    let (identifier, index) = index_until_boundary(identifier.as_str());
    Ok((TokenType::Identifier(String::from(identifier)), index))
}

pub fn tokenize_comment(comment: &String) -> Result<(Comment, usize), String> {
    match comment {
        _ if comment.starts_with("--") => {
            let (comment, index) = index_until_char(comment.as_str(), '\n');
            Ok((Comment::SingleLine(String::from(comment)), index))
        }
        _ => Err(comment.clone()),
    }
}

pub fn tokenize_whitespace(whitespace: &String) -> Result<(Whitespace, usize), String> {
    match whitespace {
        _ if whitespace.starts_with(' ') || whitespace.starts_with('\t') => {
            // get the full whitespace
            let mut index = 1;
            let whitespace_chars: Vec<char> = whitespace.chars().collect();
            while index < whitespace_chars.len() && (whitespace_chars[index] == ' ' || whitespace_chars[index] == '\t') {
                index += 1;
            }
            Ok((Whitespace::Space, index))
        },
        _ if whitespace.starts_with('\n') => Ok((Whitespace::Newline, 1)),
        _ => Err(whitespace.clone()),
    }
}

/*
fn start_string(source: &String) -> String {
    let (source, index) = index_until_boundary(source.as_str());
    source[..index].to_string()
}
*/