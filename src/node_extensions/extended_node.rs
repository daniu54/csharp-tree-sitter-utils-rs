use std::rc::Rc;

use tree_sitter::Node;

use super::NodeExtensions;
use crate::WithSource;

pub struct ExtendedNode<'t> {
    pub ts_node: Node<'t>,
    source: Rc<String>,
}

impl<'t> ExtendedNode<'t> {
    pub fn new(node: Node<'t>, source: Rc<String>) -> Self {
        ExtendedNode {
            ts_node: node,
            source,
        }
    }
}

impl<'t> WithSource for ExtendedNode<'t> {
    fn get_complete_source(self: &Self) -> Rc<String> {
        self.source.clone()
    }
}

impl<'t> NodeExtensions for ExtendedNode<'t> {
    fn get_source(self: &Self) -> String {
        self.source[self.ts_node.start_byte()..self.ts_node.end_byte()].to_string()
    }
}
