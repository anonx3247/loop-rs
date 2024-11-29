#[cfg(test)]
mod test_lexer {
    use crate::lexer::{Lexer, Token, Literal};

    #[test]
    fn test_tokenize_next() {
        let mut lexer = Lexer::new("let x : bool = true".to_string());
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("let")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("x")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol(":=").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::literal(Literal::Bool(true))));
    }
}

