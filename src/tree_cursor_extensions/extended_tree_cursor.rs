use std::rc::Rc;

use tree_sitter::TreeCursor;

use crate::{node_extensions::ExtendedNode, with_source::WithSource};

pub struct ExtendedTreeCursor<'t> {
    cursor: TreeCursor<'t>,
    source: Rc<String>,
}

impl<'t> ExtendedTreeCursor<'t> {
    pub fn new(cursor: TreeCursor<'t>, source: &Rc<String>) -> Self {
        ExtendedTreeCursor {
            cursor,
            source: source.clone(),
        }
    }
}

impl<'t> WithSource for ExtendedTreeCursor<'t> {
    fn get_complete_source(self: &Self) -> Rc<String> {
        self.source.clone()
    }
}

impl<'t> Iterator for ExtendedTreeCursor<'t> {
    type Item = ExtendedNode<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = ExtendedNode::new(self.cursor.node(), self.source.clone());

        if !self.cursor.goto_first_child() {
            if !self.cursor.goto_next_sibling() {
                loop {
                    if !self.cursor.goto_next_sibling() {
                        if !self.cursor.goto_parent() {
                            return None;
                        }
                    } else {
                        return self.next();
                    }
                }
            }
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::node_extensions::{ExtendedNode, NodeExtensions};

    use super::ExtendedTreeCursor;
    use colored::Colorize;
    use regex::Regex;

    use tree_sitter::Parser;

    #[test]
    fn should_return_expected_elements() {
        let mut parser = get_parser();

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

        let tree = parser.parse(code, None).unwrap();

        let root_node = tree.root_node();

        let cursor = root_node.walk();

        let it = ExtendedTreeCursor::new(cursor, &Rc::new(code.to_string()));

        let nodes = it.collect::<Vec<ExtendedNode>>();

        contains_node_with_kind_and_text_regex("using_directive", "using System;", &nodes);

        contains_node_with_kind_and_text_regex(
            "namespace_declaration",
            "namespace HelloWorld",
            &nodes,
        );

        contains_node_with_kind_and_text_regex(
            "attribute",
            &regex::escape(r#"ClassAttribute("AttributeValue")"#),
            &nodes,
        );

        contains_node_with_kind_and_text_regex("class_declaration", "class Program", &nodes);

        contains_node_with_kind_and_text_regex(
            "attribute",
            &regex::escape(r#"MethodAttribute("MethodAttributeValue1", "MethodAttributeValue2")"#),
            &nodes,
        );

        contains_node_with_kind_and_text_regex(
            "method_declaration",
            &regex::escape(r#"public Task Run() {}"#),
            &nodes,
        );

        contains_node_with_kind_and_text_regex(
            "method_declaration",
            &regex::escape(r#"static void Main(string[] args)"#),
            &nodes,
        );

        contains_node_with_kind_and_text_regex(
            "expression_statement",
            &regex::escape(r#"Console.WriteLine("Hello, World!");"#),
            &nodes,
        );

        contains_node_with_kind_and_text_regex(
            "comment",
            &regex::escape(r#" this is a comment"#),
            &nodes,
        );
    }

    #[test]
    fn should_be_well_behaved_for_empty_input() {
        let mut parser = get_parser();

        let code = "";

        let tree = parser.parse(code, None).unwrap();

        let root_node = tree.root_node();

        let cursor = root_node.walk();

        let it = ExtendedTreeCursor::new(cursor, &Rc::new(code.to_string()));

        let nodes = it.collect::<Vec<ExtendedNode>>();

        assert!(nodes.is_empty());
    }

    fn contains_node_with_kind_and_text_regex(kind: &str, regex: &str, nodes: &Vec<ExtendedNode>) {
        let regex = Regex::new(regex).unwrap();

        let contains = nodes
            .iter()
            .any(|n| n.ts_node.kind() == kind && regex.is_match(&n.get_source()));

        if !contains {
            let candidates: Vec<String> = nodes
                .iter()
                .filter(|n| n.ts_node.kind() == kind)
                .map(|n| n.get_source())
                .collect();

            let kind = kind.to_string().yellow();
            let regex = regex.to_string().yellow();

            let candidates_help: String = match candidates.len() {
                0 => format!(
                    "Found {} nodes of the kind {kind} in the entire input! Perhaps specify a different kind then {kind}?",
                    "no".red()
                ),
                1 => format!(
                    "Found {} node of same kind with text: \n---\n{}\n---\nPerhaps you meant that one?",
                    "one".yellow(),
                    candidates[0].to_string().yellow()
                ),
                _ => format!(
                    "Found {} nodes of same kind with texts: [\n---\n{}\n---\n].Perhaps you meant one of those?",
                    candidates.len().to_string().yellow(),
                    candidates
                        .iter()
                        .map(|c| c.yellow().to_string())
                        .collect::<Vec<String>>()
                        .join("\n---\n---\n")
                ),
            };

            panic!(
                "Expected to find tree sitter node of kind {kind} and text matching regex {regex} but none was found.\nhelp: {candidates_help}\n",
            );
        }
    }

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_c_sharp::language())
            .unwrap();

        parser
    }
}
