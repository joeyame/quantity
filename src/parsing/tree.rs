use crate::scanning::Token;

#[derive(Debug)]
pub enum TreeNode {
    Number(f64),
    Binary(Box<TreeNode>, Token, Box<TreeNode>),
}
