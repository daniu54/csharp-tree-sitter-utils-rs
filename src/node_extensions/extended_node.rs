use std::rc::Rc;

use tree_sitter::Node;

use super::NodeExtensions;
use crate::WithSource;

pub struct ExtendedNode<'t, 's> {
    pub ts_node: Node<'t>,
    source: Rc<&'s str>,
}

impl<'t, 's> ExtendedNode<'t, 's> {
    pub fn new(node: Node<'t>, source: Rc<&'s str>) -> Self {
        ExtendedNode {
            ts_node: node,
            source,
        }
    }
}

impl<'t, 's> WithSource<'s> for ExtendedNode<'t, 's> {
    fn get_complete_source(self: &Self) -> Rc<&'s str> {
        self.source.clone()
    }
}

impl<'t, 's> NodeExtensions<'s> for ExtendedNode<'t, 's> {
    fn get_source(self: &Self) -> String {
        self.source[self.ts_node.start_byte()..self.ts_node.end_byte()].to_string()
    }
}
