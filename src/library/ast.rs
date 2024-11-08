use std::fmt::{Display, Debug};
use std::sync::{Arc, RwLock};

pub trait SyntaxNode: Display + Debug { }
impl SyntaxNode for &str { }

/// Struct representing the ast tree.
#[derive(Clone, Debug)]
pub struct TreeNode<T: AsRef<dyn SyntaxNode> + Clone> {

    /// The inner value of the node
    pub data: T,
    /// The previous node in the tree
    parent: Option<Arc<RwLock<TreeNode<Arc<dyn SyntaxNode>>>>>,
    /// The left value of the node.
    left:   Option<Arc<RwLock<TreeNode<Arc<dyn SyntaxNode>>>>>,
    /// The right value of the node
    right:  Option<Arc<RwLock<TreeNode<Arc<dyn SyntaxNode>>>>>,

}

impl TreeNode<Arc<dyn SyntaxNode>> {

    pub fn new(data: Arc<dyn SyntaxNode>) -> Arc<Self> {

        return Arc::new(TreeNode {
            data,
            parent: None,
            left: None,
            right: None,
        });

    }

    pub fn set_left(&mut self, left: Arc<RwLock<TreeNode<Arc<dyn SyntaxNode>>>>) -> () {

        self.left = Some(left);

    }

    /// Function for getting the root of self.
    /// Returns None if self doesn't have a parent.
    pub fn get_root(&self) -> Option<Arc<RwLock<Self>>> {

        let mut current = match self.parent.as_ref() {
            Some(v) => v.clone(),
            None => return None,
        };

        loop {

            current = match current
                .clone()
                .read()
                .unwrap()
                .parent
                .as_ref()
            {
                Some(v) => v.clone(),
                None => return Some(current),
            }

        }

    }

}

impl From<TreeNode<Arc<dyn SyntaxNode>>> for Arc<RwLock<TreeNode<Arc<dyn SyntaxNode>>>> {

    fn from(value: TreeNode<Arc<dyn SyntaxNode>>) -> Self {
        return Arc::new(RwLock::new(value));
    }

}
