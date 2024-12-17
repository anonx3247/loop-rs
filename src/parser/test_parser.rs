#[cfg(test)]
mod test_parser {
    use crate::parser::*;
    use crate::lexer::*;

    #[test]
    fn test_parse_math_expr() {
        let mut lexer = Lexer::new("1 + 2 * 3".to_string());
        lexer.tokenize().unwrap();
        let parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string(0));
        assert_eq!(ast.token, Token::Operator(Operator::Add));
        assert_eq!(ast.children[0].token, Token::Operator(Operator::Mul));
        assert_eq!(ast.children[0].children[0].token, Token::Literal(Literal::Int(3)));
        assert_eq!(ast.children[0].children[1].token, Token::Literal(Literal::Int(2)));
        assert_eq!(ast.children[1].token, Token::Literal(Literal::Int(1)));
    }
}
