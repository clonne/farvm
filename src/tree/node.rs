use core::fmt;
use super::{Token,Error,Node};

impl Default for Node {
    fn default() -> Self {
        Node::Empty
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Empty => {
                write!(f, "(None)")
            }
        }
    }
}

pub fn build(tokens:&Vec<Token>, errors:&mut Vec<Error>) -> Node {
    Node::Empty
}
