use std::rc::Rc;

pub struct ExtendedNode<'t> {
    pub ts_node: tree_sitter::Node<'t>,
    pub source_code: String,
}

impl<'t> ExtendedNode<'t> {
    pub(crate) fn new(ts_node: tree_sitter::Node<'t>, file_source_code: Rc<String>) -> Self {
        Self {
            ts_node,
            source_code: file_source_code[ts_node.byte_range()].to_string(),
        }
    }
}
