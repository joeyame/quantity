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
            tree.push(self.term());
        }

        tree
    }

    fn term(&mut self) -> TreeNode {
        let mut left = self.primary();
        while self.tokens[self.index] == Token::Plus {
            self.index += 1;
            left = TreeNode::Binary(left.into(), Token::Plus, self.primary().into())
        }
        left
    }

    fn primary(&mut self) -> TreeNode {
        self.index += 1;
        match self.tokens[self.index - 1] {
            Token::Number(num) => TreeNode::Number(num),
            _ => todo!(),
        }
    }
}
