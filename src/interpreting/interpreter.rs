use anyhow::{bail, Result};

use crate::parsing::TreeNode;

use super::QType;

pub fn interpret(tree: Vec<TreeNode>) -> Result<()> {
    println!("~~~Program Start~~~");
    for node in tree {
        interpret_node(node)?;
    }
    Ok(())
}

fn interpret_node(node: TreeNode) -> Result<Option<Box<dyn QType>>> {
    use TreeNode::*;
    match node {
        Print(node) => {
            if let Some(value) = interpret_node(*node)? {
                println!("{:?}", value);
                Ok(None)
            } else {
                bail!("Could not print non-value!");
            }
        }
        Expression(node) => interpret_node(*node),
        Value(val) => Ok(Some(val)),
    }
}
