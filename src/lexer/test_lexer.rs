#[cfg(test)]
mod test_lexer {
    use crate::lexer::*;
    use crate::lexer::token::*;

    #[test]
    fn test_index_until_boundary_excluding() {
        assert_eq!(index_until_boundary("hi!there:"), ("hi", 2));
        assert_eq!(index_until_boundary_excluding("hi!there:", vec!['!']), ("hi!there", 8));
        assert_eq!(index_until_boundary("a_b"), ("a", 1));
        assert_eq!(index_until_boundary_excluding("a_b", vec!['_']), ("a_b", 3));
    }

    #[test]
    fn test_tokenize_symbol() {
        assert_eq!(Lexer::tokenize_symbol(&String::from(":=")), Ok((Token::from_symbol(":=").unwrap(), 2)));
        assert_eq!(Lexer::tokenize_symbol(&String::from("+")), Ok((Token::from_symbol("+").unwrap(), 1)));
        assert_eq!(Lexer::tokenize_symbol(&String::from("-")), Ok((Token::from_symbol("-").unwrap(), 1)));
        assert_eq!(Lexer::tokenize_symbol(&String::from("*")), Ok((Token::from_symbol("*").unwrap(), 1)));
        assert_eq!(Lexer::tokenize_symbol(&String::from("/")), Ok((Token::from_symbol("/").unwrap(), 1)));
        assert_eq!(Lexer::tokenize_symbol(&String::from("%")), Ok((Token::from_symbol("%").unwrap(), 1)));
    }

    #[test]
    fn test_tokenize_keyword() {
        assert_eq!(Lexer::tokenize_keyword(&String::from("let")), Ok((Token::from_keyword("let").unwrap(), 3)));
        assert_eq!(Lexer::tokenize_keyword(&String::from("mut")), Ok((Token::from_keyword("mut").unwrap(), 3)));
        assert_eq!(Lexer::tokenize_keyword(&String::from("async")), Ok((Token::from_keyword("async").unwrap(), 5)));
    }

    #[test]
    fn test_tokenize_base_type() {
        assert_eq!(Lexer::tokenize_base_type(&String::from("i32")), Ok((Token::from_base_type("i32").unwrap(), 3)));
        assert_eq!(Lexer::tokenize_base_type(&String::from("u8")), Ok((Token::from_base_type("u8").unwrap(), 2)));
    }

    #[test]
    fn test_tokenize_custom_type() {
        assert_eq!(Lexer::tokenize_custom_type(&String::from("MyType")), Ok((Token::custom_type("MyType"), 6)));
    }

    #[test]
    fn test_tokenize_literal() {
        assert_eq!(Lexer::tokenize_literal(&String::from("true")), Ok((Token::literal(Literal::Bool(true)), 4)));
        assert_eq!(Lexer::tokenize_literal(&String::from("123")), Ok((Token::literal(Literal::Int(123)), 3)));
        assert_eq!(Lexer::tokenize_literal(&String::from("123.45")), Ok((Token::literal(Literal::Float(123.45)), 6)));
        assert_eq!(Lexer::tokenize_literal(&String::from("123.45e6")), Ok((Token::literal(Literal::Float(123.45e6)), 8)));
        assert_eq!(Lexer::tokenize_literal(&String::from("\"Hello, world!\"")), Ok((Token::literal(Literal::String("Hello, world!".to_string())), 15)));
        assert_eq!(Lexer::tokenize_literal(&String::from("'Hello, world!'")), Ok((Token::literal(Literal::String("Hello, world!".to_string())), 15)));
    }

    #[test]
    fn test_tokenize_identifier() {
        assert_eq!(Lexer::tokenize_identifier(&String::from("hello_world")), Ok((Token::identifier("hello_world"), 11)));
        assert_eq!(Lexer::tokenize_identifier(&String::from("x")), Ok((Token::identifier("x"), 1)));
        assert_eq!(Lexer::tokenize_identifier(&String::from("helloWorld")), Ok((Token::identifier("helloWorld"), 10)));
    }

    #[test]
    fn test_tokenize_whitespace() {
        assert_eq!(Lexer::tokenize_whitespace(&String::from("   \t\n")), Ok((Token::Whitespace(Whitespace::Newline), 5)));
        assert_eq!(Lexer::tokenize_whitespace(&String::from("\t   ")), Ok((Token::Whitespace(Whitespace::Space), 4)));
    }

    #[test]
    fn test_tokenize_next() {
        let mut lexer = Lexer::new("x := true".to_string());
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("x")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol(":=").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::literal(Literal::Bool(true))));

        let mut lexer = Lexer::new("for i in 3..200 {\nl.append(i)
        }\n".to_string());
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_keyword("for").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("i")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_keyword("in").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::literal(Literal::Int(3))));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol("..").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::literal(Literal::Int(200))));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol("{").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::Whitespace(Whitespace::Newline)));
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("l")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol(".").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("append")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol("(").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::identifier("i")));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol(")").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::Whitespace(Whitespace::Newline)));
        assert_eq!(lexer.tokenize_next(), Ok(Token::from_symbol("}").unwrap()));
        assert_eq!(lexer.tokenize_next(), Ok(Token::Whitespace(Whitespace::Newline)));
    }
}

