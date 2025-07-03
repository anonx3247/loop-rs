#[cfg(test)]
mod test_parser {
    use crate::parser::parser::*;
    use crate::lexer::*;

    #[test]
    fn test_parse_math_expr() {
        let mut lexer = Lexer::new("1 + 2 * 3".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "Add");
        assert_eq!(ast.children()[0].element(), "Int(1)");
        assert_eq!(ast.children()[1].element(), "Mul");
        assert_eq!(ast.children()[1].children()[0].element(), "Int(2)");
        assert_eq!(ast.children()[1].children()[1].element(), "Int(3)");
        
    }

    #[test]
    fn test_parse_bool_expr() {
        let mut lexer = Lexer::new("true and false".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "And");
        assert_eq!(ast.children()[0].element(), "Bool(true)");
        assert_eq!(ast.children()[1].element(), "Bool(false)");
    }

    #[test]
    fn test_parse_bool_expr_with_numbers() {
        let mut lexer = Lexer::new("(32.5 > 10) or (10 <= 20)".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap_or_else(|e| {
            println!("{:?}", e);
            panic!("Error parsing expression");
        });
        println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "Or");
        assert_eq!(ast.children()[0].element(), "Gt");
        assert_eq!(ast.children()[0].children()[0].element(), "Float(32.5)");
        assert_eq!(ast.children()[0].children()[1].element(), "Int(10)");
        assert_eq!(ast.children()[1].element(), "Lte");
        assert_eq!(ast.children()[1].children()[0].element(), "Int(10)");
        assert_eq!(ast.children()[1].children()[1].element(), "Int(20)");
    }


    #[test]
    fn test_parse_math_with_parentheses() {
        let mut lexer = Lexer::new("(1 + 2) * 3".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "Mul");
        assert_eq!(ast.children()[0].element(), "Add");
        assert_eq!(ast.children()[0].children()[0].element(), "Int(1)");
        assert_eq!(ast.children()[0].children()[1].element(), "Int(2)");
        assert_eq!(ast.children()[1].element(), "Int(3)");
    }

    
    #[test]
    fn test_parse_assignment_with_type() {
        let mut lexer = Lexer::new("mut a: i32 = 1".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
            println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "mut \"a\" : I32 =");
        assert_eq!(ast.children()[0].element(), "Int(1)");
    }

    #[test]
    fn test_parse_assignment_without_type() {
        let mut lexer = Lexer::new("mut a := 1".to_string());
        lexer.tokenize().unwrap();
        println!("{:?}", lexer.tokens);
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string());
        let ast = ast.children()[0].clone_to_node();
        assert_eq!(ast.element(), "mut \"a\" : [inferred] =");
        assert_eq!(ast.children()[0].element(), "Int(1)");
    }

    #[test]
    fn test_parse_if_else() {
        let mut lexer = Lexer::new("if a > 10 { a } else { b }".to_string());
        lexer.tokenize().unwrap();
        let mut parser = Parser::new(lexer.tokens.clone());
        let ast = parser.parse().unwrap();
        println!("{}", ast.to_string());
    }
    
}
