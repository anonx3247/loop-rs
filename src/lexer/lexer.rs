use crate::lexer::token::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            source: String::new(),
            tokens,
        }
    }

    pub fn tokenize(&mut self) -> Result<(), String> {
        while self.source.len() > 0 {
            self.tokenize_next()?;
        }
        Ok(())
    }

    pub fn tokenize_next(&mut self) -> Result<Token, String> {
        let result = {
            let whitespace_re = Regex::new(r"^[\s\t\n]+").unwrap();
            let string_re = Regex::new(r#"^["']"#).unwrap();
            let bool_re = Regex::new(r"^(true|false)").unwrap();
            let none_re = Regex::new(r"^none").unwrap();
            let number_re = Regex::new(r"^[0-9]+([.][0-9]+)?([eE][+-]?[0-9]+)?").unwrap();
            let comment_re = Regex::new(r"^--").unwrap();
            let identifier_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*").unwrap();

            if whitespace_re.is_match(&self.source) {
                Self::tokenize_whitespace(&self.source)
            } else if string_re.is_match(&self.source) 
                || bool_re.is_match(&self.source)
                || none_re.is_match(&self.source)
                || number_re.is_match(&self.source) {
                Self::tokenize_literal(&self.source)
            } else if comment_re.is_match(&self.source) {
                Self::tokenize_comment(&self.source)
            } else {
                match Self::tokenize_symbol(&self.source) {
                    Ok(r) => Ok(r),
                    Err(e1) => match Self::tokenize_keyword(&self.source) {
                        Ok(r) => Ok(r),
                        Err(e2) => if identifier_re.is_match(&self.source) {
                            Self::tokenize_identifier(&self.source)
                        } else {
                            Err(e1 + &e2)
                        }
                    }
                }
                
            }
        };

        let (token, index) = match result {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        
        self.source = self.source[index..].to_string();
        if token != Token::Whitespace(Whitespace::Space) {
            self.tokens.push(token.clone())
        } else {
            return self.tokenize_next()
        }
        Ok(token)
    }

    pub fn tokenize_symbol(s: &String) -> Result<(Token, usize), String> {
        Self::tokenize_from_map(s, get_symbols_map())
    }

    pub fn tokenize_keyword(s: &String) -> Result<(Token, usize), String> {
        Self::tokenize_from_map(s, get_keywords_map())
    }

    pub fn tokenize_base_type(s: &String) -> Result<(Token, usize), String> {
        Self::tokenize_from_map(s, get_base_types_map())
    }

    fn tokenize_from_map(s: &String, map: HashMap<&str, Token>) -> Result<(Token, usize), String> {
        let mut sorted_map = map.iter().collect::<Vec<(&&str, &Token)>>();
        sorted_map.sort_by(|(k, _), (k2, _)| k.len().cmp(&k2.len()));
        sorted_map.reverse();
        for (base_type, token) in sorted_map {
            if s.starts_with(base_type) {
                return Ok((token.clone(), base_type.len()));
            }
        }
        Err(s.clone())
    }

    pub fn tokenize_custom_type(custom_type: &String) -> Result<(Token, usize), String> {
        let (custom_type, index) = index_until_boundary(custom_type.as_str());
        Ok((Token::custom_type(custom_type), index))
    }

    pub fn tokenize_literal(s: &String) -> Result<(Token, usize), String> {
        let (literal, index) = match Literal::tokenize_literal(s) {
            Ok((literal, index)) => (literal, index),
            Err(e) => return Err(e),
        };
        Ok((Token::Literal(literal), index))
    }

    pub fn tokenize_comment(comment: &String) -> Result<(Token, usize), String> {
        let (comment, index) = index_until_char(comment.as_str(), '\n');
        Ok((Token::Comment(Comment::SingleLine(String::from(comment))), index))
    }   

    pub fn tokenize_whitespace(whitespace: &String) -> Result<(Token, usize), String> {
        let re = regex::Regex::new(r"^([\s\t]*[\n]+[\s\t]*)|([\s\t]+)").unwrap();
        if let Some(captures) = re.captures(whitespace) {
            let spaces = captures.get(2).map_or("", |m| m.as_str());
            let newlines = captures.get(1).map_or("", |m| m.as_str());
            
            if newlines != "" {
                return Ok((Token::Whitespace(Whitespace::Newline), spaces.len() + newlines.len()));
            } else if spaces != "" {
                return Ok((Token::Whitespace(Whitespace::Space), spaces.len()));
            }
        }

        Err(whitespace.clone())
    }

    pub fn tokenize_identifier(identifier: &String) -> Result<(Token, usize), String> {
        let (identifier, index) = index_until_boundary_excluding(identifier.as_str(), vec!['_']);
        Ok((Token::Identifier(String::from(identifier)), index))
    }
}


pub fn index_until_boundary_excluding(literal: &str, excluding: Vec<char>) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if (c.is_whitespace() || !c.is_alphanumeric()) && !excluding.contains(&c) {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index)

}

pub fn index_until_boundary(literal: &str) -> (&str, usize) {
    index_until_boundary_excluding(literal, Vec::new())
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
        let (literal, index) = index_until_boundary_excluding(literal.as_str(), vec!['.']);

        let float_regex = regex::Regex::new(r"^(-?\d+\.?\d*)[eE](-?\d+)$").unwrap();
        
        if let Some(captures) = float_regex.captures(literal) {
            let base = captures[1].parse::<f64>().map_err(|e| e.to_string())?;
            let exp = captures[2].parse::<i32>().map_err(|e| e.to_string())?;
            Ok((Literal::Float(base * 10_f64.powf(exp as f64)), index))
        } else {
            match literal.parse::<f64>() {
                Ok(f) => Ok((Literal::Float(f), index)),
                Err(e) => Err(e.to_string()),
            }
        }
    }

    fn tokenize_int(literal: &String) -> Result<(Literal, usize), String> {
        let (literal, index) = index_until_boundary(literal.as_str());
        let int_regex = regex::Regex::new(r"^-?\d+$").unwrap();
        
        if int_regex.is_match(literal) {
            match literal.parse::<i64>() {
                Ok(i) => Ok((Literal::Int(i), index)),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err(format!("Invalid integer literal: {}", literal))
        }
    }
    
    pub fn tokenize_string(literal: &String) -> Result<(Literal, usize), String> {
        let string_regex = regex::Regex::new(r#"^([^"']*?)(['"'])"#).unwrap();
        
        if literal.starts_with('"') || literal.starts_with('\'') {
            let content = &literal[1..];
            if let Some(captures) = string_regex.captures(content) {
                let string_content = captures.get(1).unwrap().as_str();
                Ok((Literal::String(string_content.to_string()), string_content.len() + 2))
            } else {
                Err(literal.clone())
            }
        } else {
            Err(literal.clone())
        }
    }
    
    pub fn tokenize_literal(literal: &String) -> Result<(Literal, usize), String> {
        let true_re = regex::Regex::new(r"^true").unwrap();
        let false_re = regex::Regex::new(r"^false").unwrap();
        let none_re = regex::Regex::new(r"^none").unwrap();
        let float_re = regex::Regex::new(r"(-?[0-9]+)\.[0-9]+([eE]-?[0-9]+)?").unwrap();

        if true_re.is_match(literal) {
            Ok((Literal::Bool(true), 4))
        } else if false_re.is_match(literal) {
            Ok((Literal::Bool(false), 5))
        } else if none_re.is_match(literal) {
            Ok((Literal::None, 4))
        } else if literal.starts_with('"') || literal.starts_with('\'') {
            Self::tokenize_string(literal)
        } else if float_re.is_match(literal) {
            Self::tokenize_float(literal)
        } else {
            Self::tokenize_int(literal)
        }
    }
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


/*
fn start_string(source: &String) -> String {
    let (source, index) = index_until_boundary(source.as_str());
    source[..index].to_string()
}
*/