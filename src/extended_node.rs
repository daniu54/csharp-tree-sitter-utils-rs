use std::rc::Rc;

use crate::ExtendedTreeCursor;

pub struct ExtendedNode<'t> {
    pub ts_node: tree_sitter::Node<'t>,
    pub file_source_code: Rc<String>,
    pub source_code: String,
}

impl<'t> ExtendedNode<'t> {
    pub(crate) fn new(ts_node: tree_sitter::Node<'t>, file_source_code: Rc<String>) -> Self {
        Self {
            ts_node,
            file_source_code: Rc::clone(&file_source_code),
            source_code: file_source_code[ts_node.byte_range()].to_string(),
        }
    }
}

impl<'t> IntoIterator for &'t ExtendedNode<'t> {
    type Item = ExtendedNode<'t>;
    type IntoIter = ExtendedTreeCursor<'t>;

    fn into_iter(self) -> Self::IntoIter {
        ExtendedTreeCursor {
            ts_cursor: self.ts_node.walk(),
            source_code: Rc::clone(&self.file_source_code),
        }
    }
}
