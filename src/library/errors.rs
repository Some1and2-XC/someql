use std::fmt::{Debug, Display};

pub trait QLErrorTrait: Debug + Display {
}

/// This struct represents a "Query Langauge Error".
/// This actually covers much more than just language errors (such as parsing etc.)
#[derive(Debug)]
pub struct QLError {
    /// This is the message to be attached with the error.
    message: String,
    /// This is the assumed line number that the error occured.
    line: Option<u32>,
}

impl Display for QLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl QLErrorTrait for QLError {
}
