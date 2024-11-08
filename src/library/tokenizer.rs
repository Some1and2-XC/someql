use std::sync::Arc;
use super::ast::{TreeNode, SyntaxNode};

pub fn tokenize(input: &str) -> Arc<TreeNode<Arc<dyn SyntaxNode>>> {

    let mut out_tree: Option<Arc<TreeNode<Arc<dyn SyntaxNode>>>> = None;

    for tkn in input.split(" ") {

        let value = TreeNode::new(Arc::new(tkn));

        match &mut out_tree {
            Some(v) => {
                v.set_left(value.into());
            },
            None => out_tree = Some(value),
        }

    }

    todo!()

}
