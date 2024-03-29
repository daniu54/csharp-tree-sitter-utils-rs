use std::rc::Rc;

use tree_sitter::Node;

use super::NodeExtensions;

pub struct ExtendedNode<'t> {
    pub ts_node: Node<'t>,
    pub source: Rc<String>,
}

impl<'t> NodeExtensions for ExtendedNode<'t> {
    fn get_source(&self) -> String {
        self.source[self.ts_node.start_byte()..self.ts_node.end_byte()].to_string()
    }
}
