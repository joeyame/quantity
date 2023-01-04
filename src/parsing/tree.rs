use crate::interpreting::QType;

#[derive(Debug)]
pub enum TreeNode {
    Value(Box<dyn QType>),
    Print(Box<TreeNode>),
    Expression(Box<TreeNode>),
}
