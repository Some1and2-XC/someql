use std::fmt::{Display, Debug, Formatter};
use std::sync::{Arc, RwLock};

type Node<T> = TreeNode<Arc<T>>;
type LockNode<T> = Arc<RwLock<TreeNode<Arc<T>>>>;

pub trait SyntaxNode {
    fn to_string(&self) -> String;
}

impl Display for dyn SyntaxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

impl Debug for dyn SyntaxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

impl SyntaxNode for &str {

    fn to_string(&self) -> String {
        return format!("{}", self);
    }

}

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

pub enum Datatypes {
    I8,
    U8,
    I32,
    U32,
    I64,
    U64,
    BOOL
}
