use crate::scanning::Token;

use super::tree::*;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn parse_tokens(tokens: Vec<Token>) -> Vec<TreeNode> {
        let mut parser = Self { tokens, index: 0 };
        parser.parse()
    }

    fn parse(&mut self) -> Vec<TreeNode> {
        let mut tree = Vec::<TreeNode>::new();
        while self.tokens[self.index] != Token::Eof {
            tree.push(self.statement());
        }

        tree
    }

    fn statement(&mut self) -> TreeNode {
        match &self.tokens[self.index] {
            Token::Identifier(x) if x == "print" => self.print_stmt(),
            _ => self.expression_stmt(),
        }
    }

    fn print_stmt(&mut self) -> TreeNode {
        self.index += 1;
        TreeNode::Print(self.expression().into())
    }

    fn expression_stmt(&mut self) -> TreeNode {
        TreeNode::Expression(self.expression().into())
    }

    fn expression(&mut self) -> TreeNode {
        self.index += 1;
        match &self.tokens[self.index - 1] {
            Token::Number(num) => TreeNode::Value(Box::new(*num)),
            Token::Identifier(id) => TreeNode::Value(Box::new(id.clone())),
            _ => todo!(),
        }
    }
}
