use crate::lexer::token::*;
use crate::lexer::token::Literal;
use crate::lexer::utils::*;
use regex::Regex;
use std::collections::HashMap;


pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    NoMatchingSymbol(String),
    NoMatchingKeyword(String),
    NoMatchingBaseType(String),
    NoMatchingIdentifier(String),
    NoMatchingCustomType(String),
    NoMatchingLiteral(String),
    CouldNotTokenizeWhitespace,
    CouldNotTokenize(String),
    InvalidFloatLiteral(String),
    InvalidIntegerLiteral(String),
    InvalidStringLiteral(String),
    InvalidComment(String),
    NoMatchingBracket(String),
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

    pub fn tokenize(&mut self) -> Result<(), LexerError> {
        while self.source.len() > 0 {
            self.tokenize_next()?;
        }
        Ok(())
    }

    pub fn clean_tokens(&mut self) {
        let mut cursor = 0;
        while cursor < self.tokens.len() {
            match self.tokens[cursor] {
                Token::Whitespace(Whitespace::Space) => {
                    self.tokens.remove(cursor); // no need for spaces
                },
                Token::Whitespace(Whitespace::Newline) => {
                    if cursor == 0 {
                        self.tokens.remove(cursor); // remove leading whitespace
                    } else if cursor > 1 
                    && self.tokens[cursor-1] == Token::Whitespace(Whitespace::Newline) 
                    && self.tokens[cursor-2] == Token::Whitespace(Whitespace::Newline) {
                        self.tokens.remove(cursor); // remove 3+ newlines (allow max 2 newlines)
                    } else if cursor + 1 < self.tokens.len()
                    && match self.tokens[cursor + 1] {
                        Token::Bracket(Bracket::CloseBrace) | Token::Bracket(Bracket::CloseBracket) | Token::Bracket(Bracket::CloseParen) => true,
                        _ => false
                    }{
                        self.tokens.remove(cursor); // remove newline before closing brackets
                    } else if cursor - 1 > 0
                    && match self.tokens[cursor - 1] {
                        Token::Bracket(Bracket::OpenBrace) | Token::Bracket(Bracket::OpenBracket) | Token::Bracket(Bracket::OpenParen) => true,
                        _ => false
                    }{
                        self.tokens.remove(cursor); // remove newline after opening brackets
                    } else if cursor == self.tokens.len() - 1 {   
                        self.tokens.remove(cursor); // remove newline before EOF
                    } else {
                        cursor += 1; // keep the newline if it's not leading or consecutive
                    }
                },
                Token::Comment(_) => {
                    self.tokens.remove(cursor);
                },
                _ => {
                    cursor += 1;
                }
            }
        }
    }

    pub fn tokenize_next(&mut self) -> Result<Token, LexerError> {
        let (token, _) = self.tokenize_next_with_index()?;
        if token != Token::Whitespace(Whitespace::Space) {
            self.tokens.push(token.clone())
        } else {
            return self.tokenize_next()
        }
        Ok(token)
    }

    pub fn tokenize_next_with_index(&mut self) -> Result<(Token, usize), LexerError> {
        let result = {
            let whitespace_re = Regex::new(r"^[\s\t\n]+").unwrap();
            let string_re = Regex::new(r#"^("|'|r"|r')"#).unwrap();
            let bool_re = Regex::new(r"^(true|false)").unwrap();
            let none_re = Regex::new(r"^none").unwrap();
            let number_re = Regex::new(r"^-?[0-9]+([.][0-9]+)?([eE][+-]?[0-9]+)?").unwrap();
            let comment_re = Regex::new(r"^--").unwrap();
            let identifier_re = Regex::new(r"^[_a-z][a-zA-Z0-9_]*").unwrap();
            let custom_type_re = Regex::new(r"^[A-Z][a-zA-Z0-9_]*\??").unwrap();

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
                    Err(_) => match Self::tokenize_keyword(&self.source) {
                        Ok(r) => Ok(r),
                        Err(_) => match Self::tokenize_base_type(&self.source) {
                            Ok(r) => Ok(r),
                            Err(_) => if identifier_re.is_match(&self.source) {
                                Self::tokenize_identifier(&self.source)
                            } else if custom_type_re.is_match(&self.source) {
                                Self::tokenize_custom_type(&self.source)
                            } else {
                                Err(LexerError::CouldNotTokenize(preview(&self.source)))
                            }
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
        Ok((token, index))
    }

    pub fn tokenize_symbol(s: &String) -> Result<(Token, usize), LexerError> {
        match Self::tokenize_from_map(s, get_symbols_map()) {
            Ok(r) => Ok(r),
            Err(_) => Err(LexerError::NoMatchingSymbol(preview(s))),
        }
    }

    pub fn tokenize_keyword(s: &String) -> Result<(Token, usize), LexerError> {
        match Self::tokenize_from_map(s, get_keywords_map()) {
            Ok(r) => Ok(r),
            Err(_) => Err(LexerError::NoMatchingKeyword(preview(s))),
        }
    }

    pub fn tokenize_base_type(s: &String) -> Result<(Token, usize), LexerError> {
        match Self::tokenize_from_map(s, get_base_types_map()) {
            Ok(r) => Ok(r),
            Err(_) => Err(LexerError::NoMatchingBaseType(preview(s))),
        }
    }

    fn tokenize_from_map(s: &String, map: HashMap<&str, Token>) -> Result<(Token, usize), LexerError> {
        let mut sorted_map = map.iter().collect::<Vec<(&&str, &Token)>>();
        sorted_map.sort_by(|(k, _), (k2, _)| k.len().cmp(&k2.len()));
        sorted_map.reverse();
        for (base_type, token) in sorted_map {
            if s.starts_with(base_type) {
                return Ok((token.clone(), base_type.len()));
            }
        }
        Err(LexerError::NoMatchingBaseType(s.clone()))
    }

    pub fn tokenize_custom_type(custom_type: &String) -> Result<(Token, usize), LexerError> {
        let (custom_type, index) = index_until_boundary(custom_type.as_str());
        Ok((Token::custom_type(custom_type), index))
    }

    pub fn tokenize_literal(s: &String) -> Result<(Token, usize), LexerError> {
        let (literal, index) = match Literal::tokenize_literal(s) {
            Ok((literal, index)) => (literal, index),
            Err(e) => return Err(e),
        };
        Ok((Token::Literal(literal), index))
    }

    pub fn tokenize_comment(comment: &String) -> Result<(Token, usize), LexerError> {
        let (comment, index) = index_until_char(comment.as_str(), '\n');
        Ok((Token::Comment(Comment::SingleLine(String::from(comment))), index))
    }   

    pub fn tokenize_whitespace(whitespace: &String) -> Result<(Token, usize), LexerError> {
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

        Err(LexerError::CouldNotTokenizeWhitespace)
    }

    pub fn tokenize_identifier(identifier: &String) -> Result<(Token, usize), LexerError> {
        let (identifier, index) = index_until_boundary_excluding(identifier.as_str(), vec!['_']);
        Ok((Token::Identifier(String::from(identifier)), index))
    }
}