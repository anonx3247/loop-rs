use crate::lexer::token;
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn find_first_token_skip_brackets(&mut self, token: &token::Token, tokens: &[token::Token]) -> Result<Option<usize>, ParseError> {
        let mut i = 0;
        while i < tokens.len() {
            if tokens[i] == *token {
                return Ok(Some(i));
            }
            match tokens[i] {
                token::Token::Bracket(_) => {
                    let close_pos = self.find_matching_bracket(&tokens, i)?;
                    i = close_pos + 1;
                }
                _ => i += 1,
            }
        }
        Ok(None)
    }

    pub fn find_matching_bracket(&mut self, tokens: &[token::Token], loc: usize) -> Result<usize, ParseError> {
        let bracket_pairs = [
            (token::Token::Bracket(token::Bracket::OpenParen), token::Token::Bracket(token::Bracket::CloseParen)),
            (token::Token::Bracket(token::Bracket::OpenBrace), token::Token::Bracket(token::Bracket::CloseBrace)), 
            (token::Token::Bracket(token::Bracket::OpenBracket), token::Token::Bracket(token::Bracket::CloseBracket))
        ];
        let mut count = 1;
        let close_pos = loc;
        let mut open_pos = close_pos;
        match bracket_pairs.iter().find(|(_open, close)| tokens[close_pos] == *close) {
            Some((open, close)) => {
                while count > 0 && open_pos < tokens.len() - 1 {
                    open_pos += 1;
                    if tokens[open_pos] == *close {
                        count += 1;
                    } else if tokens[open_pos] == *open {
                        count -= 1;
                    }
                }
                Ok(open_pos)
            }
            _ => Err(ParseError::NoMatchingBracket),
        }
    }
} 