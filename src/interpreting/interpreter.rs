use anyhow::{bail, Result};

use crate::parsing::TreeNode;

pub fn interpret(tree: Vec<TreeNode>) -> Result<()> {
    for node in tree {
        println!("Expression value: {}", interpret_node(node)?)
    }
    Ok(())
}

fn interpret_node(node: TreeNode) -> Result<f64> {
    use crate::scanning::Token;
    use TreeNode::*;
    match node {
        Binary(lhs, Token::Plus, rhs) => Ok(interpret_node(*lhs)? + interpret_node(*rhs)?),
        Number(val) => Ok(val),
        Binary(_, _, _) => bail!("Invalid operator!"),
    }
}
