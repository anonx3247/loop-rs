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
        assert_eq!(ast.element(), "Add");
        assert_eq!(ast.children()[0].element(), "Mul");
        assert_eq!(ast.children()[0].children()[0].element(), "Int(3)");
        assert_eq!(ast.children()[0].children()[1].element(), "Int(2)");
        assert_eq!(ast.children()[1].element(), "Int(1)");
    }


    #[test]
    fn test_parse_math_with_parentheses() {
        let mut lexer = Lexer::new("(1 + 2) * 3".to_string());
        lexer.tokenize().unwrap();
        let parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string(0));
        assert_eq!(ast.element(), "Mul");
        assert_eq!(ast.children()[0].element(), "Int(3)");
        assert_eq!(ast.children()[1].element(), "Add");
        assert_eq!(ast.children()[1].children()[0].element(), "Int(2)");
        assert_eq!(ast.children()[1].children()[1].element(), "Int(1)");
        
    }
}
