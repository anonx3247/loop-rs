use crate::lexer::token::Literal;
use crate::lexer::lexer::LexerError;
use crate::lexer::utils::*;

impl Literal {
    fn tokenize_float(literal: &String) -> Result<(Literal, usize), LexerError> {
        let (literal, index) = index_until_boundary_excluding(literal.as_str(), vec!['.']);

        // first determine if there is a period in the expression:
        let float_regex = if literal.contains('.') {
            regex::Regex::new(r"^(-?\d+\.?\d*)[eE]?(-?\d+)?$").unwrap()
        } else {
            regex::Regex::new(r"^(-?\d+)[eE](-?\d+)$").unwrap() // if without period, must have exponent
        };

        if let Some(captures) = float_regex.captures(literal) {
            let base = match captures.get(1) {
                Some(b) => b.as_str().parse::<f64>().map_err(|_| LexerError::InvalidFloatLiteral(literal.to_string())),
                _ => Err(LexerError::InvalidFloatLiteral(literal.to_string())),
            }?;
            let exp = match captures.get(2) {
                Some(e) => e.as_str().parse::<i32>().map_err(|_| LexerError::InvalidFloatLiteral(literal.to_string())),
                _ => Ok(0),
            }?;
            
            Ok((Literal::Float(base * 10_f64.powf(exp as f64)), index))

        } else {
            Err(LexerError::InvalidFloatLiteral(literal.to_string()))
        }
    }

    fn tokenize_int(literal: &String) -> Result<(Literal, usize), LexerError> {
        let (literal, index) = index_until_boundary(literal.as_str());
        let int_regex = regex::Regex::new(r"^-?\d+$").unwrap();
        
        if int_regex.is_match(literal) {
            match literal.parse::<i64>() {
                Ok(i) => Ok((Literal::Int(i), index)),
                Err(_) => Err(LexerError::InvalidIntegerLiteral(literal.to_string())),
            }
        } else {
            Err(LexerError::InvalidIntegerLiteral(literal.to_string()))
        }
    }
    
    pub fn tokenize_string(literal: &String) -> Result<(Literal, usize), LexerError> {
        let string_beginning_re = regex::Regex::new(r#"^("|'|r"|r')"#).unwrap();
        let opening_quote = match string_beginning_re.captures(literal) {
            Some(captures) => {
                captures.get(1).unwrap().as_str()
            }
            None => return Err(LexerError::InvalidStringLiteral(literal.to_string())),
        };

        let closing_quote = match opening_quote {
            "\"" => "\"",
            "'" => "'",
            "r\"" => "\"",
            "r'" => "'",
            _ => return Err(LexerError::InvalidStringLiteral(literal.to_string())),
        };

        let raw = opening_quote.starts_with('r');

        let mut cursor = opening_quote.len();
        let mut closed = false;
        while !closed {
            if cursor >= literal.len() {
                return Err(LexerError::InvalidStringLiteral(literal.to_string()));
            }
            if literal[cursor..].starts_with('\\') && cursor + 1 < literal.len() && literal[cursor + 1..].starts_with(closing_quote) {
                cursor += 2;
            }

            if !raw {
                if literal[cursor..].starts_with('{') {
                    let index = match find_matching_bracket(&literal[cursor..].to_string(), '{', '}') {
                        Ok(index) => index,
                        Err(e) => return Err(e),
                    };
                    cursor += index;
                }
            }

            if literal[cursor..].starts_with(closing_quote) {
                closed = true;
            }
            cursor += 1;
        }
        Ok((Literal::String(literal[opening_quote.len()..cursor-1].to_string(), raw), cursor))
    }
    
    pub fn tokenize_literal(literal: &String) -> Result<(Literal, usize), LexerError> {
        let true_re = regex::Regex::new(r"^true").unwrap();
        let false_re = regex::Regex::new(r"^false").unwrap();
        let none_re = regex::Regex::new(r"^none").unwrap();
        // 1.23e-4 or 1.23e4 or 1.23 or 1e-4 or 1e4
        let float_re = regex::Regex::new(r"^((-?[0-9]+)\.[0-9]+([eE]-?[0-9]+)?|(-?[0-9]+[eE]-?[0-9]+))").unwrap();

        let string_beginning_re = regex::Regex::new(r#"^("|'|r"|r')"#).unwrap();

        if true_re.is_match(literal) {
            Ok((Literal::Bool(true), 4))
        } else if false_re.is_match(literal) {
            Ok((Literal::Bool(false), 5))
        } else if none_re.is_match(literal) {
            Ok((Literal::None, 4))
        } else if string_beginning_re.is_match(literal) {
            Self::tokenize_string(literal)
        } else if float_re.is_match(literal) {
            Self::tokenize_float(literal)
        } else {
            Self::tokenize_int(literal)
        }
    }
}

