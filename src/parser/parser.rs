use crate::lexer::token::*;

#[derive(Clone)]
pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }

    pub fn to_string(&self, indent: usize) -> String {
        let mut result = String::new();
        if indent == 1 {
            result.push_str(&"|--");
        } else if indent > 1 {
            result.push_str(&"|   ".repeat(indent - 1));
            result.push_str(&"|--");
        }
        result.push_str(&self.token.to_string());
        result.push('\n');
        for child in self.children.iter() {
            result.push_str(&child.to_string(indent + 1));
        }
        result
    }
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens.into_iter().rev().collect() }
    }

    pub fn parse(&self) -> Result<ASTNode, String> {
        self.parse_math_expr(&self.tokens)
    }

    fn find_first_token(&self, token: &Token, tokens: &[Token]) -> Option<usize> {
        tokens.iter().position(|t| t == token)
    }

    fn parse_math_expr(&self, tokens: &[Token]) -> Result<ASTNode, String> {
        let tokens = tokens.to_vec();
        let operators = [
            Token::Operator(Operator::Sub),
            Token::Operator(Operator::Add),
            Token::Operator(Operator::Div),
            Token::Operator(Operator::Mul)
        ];

        for op in operators.iter() {
            if let Some(pos) = self.find_first_token(&op, &tokens) {
                let mut node = ASTNode::new(tokens[pos].clone());
                node.add_child(self.parse_math_expr(&tokens[..pos])?);
                node.add_child(self.parse_math_expr(&tokens[pos + 1..])?);
                return Ok(node);
            }
        }

        if tokens.len() == 1 {
            return Ok(ASTNode::new(tokens[0].clone()));
        }

        Err("Invalid math expression".to_string())
    }


}


