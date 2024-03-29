use std::rc::Rc;

use tree_sitter::{Parser, Tree};

use crate::{ExtendedNode, ExtendedTreeCursor};

pub struct ExtendedTree {
    pub ts_tree: Tree,
    pub source: Rc<String>,
}

impl ExtendedTree {
    pub fn from_source_code(source: &str) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_c_sharp::language())
            .unwrap();

        let tree = parser.parse(source.as_bytes(), None).unwrap();

        ExtendedTree {
            ts_tree: tree,
            source: Rc::new(source.to_string()),
        }
    }
}

impl<'t> IntoIterator for &'t ExtendedTree {
    type Item = ExtendedNode<'t>;
    type IntoIter = ExtendedTreeCursor<'t>;

    fn into_iter(self) -> Self::IntoIter {
        ExtendedTreeCursor {
            ts_cursor: self.ts_tree.root_node().walk(),
            source: Rc::clone(&self.source),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ExtendedTree;

    #[test]
    fn should_return_expected_elements() {
        let code = r#"
        using System;

        namespace HelloWorld
        {
            [ClassAttribute("AttributeValue")]
            class Program
            {
                [MethodAttribute("MethodAttributeValue1", "MethodAttributeValue2")]
                public Task Run() {}

                static void Main(string[] args)
                {
                    Console.WriteLine("Hello, World!");
                    // this is a comment
                }
            }
        }
        "#;

        let tree = ExtendedTree::from_source_code(code);

        let mut it = tree.into_iter();

        let node = it
            .find(|n| n.ts_node.kind() == "class_declaration")
            .unwrap();

        assert!(node.get_source().contains("class Program"));
    }
}
