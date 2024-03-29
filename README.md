# csharp-tree-sitter-utils-rs
## Overview
Utilities for the [tree sitter bindings](https://docs.rs/tree-sitter/latest/tree_sitter) for the [csharp language](https://github.com/tree-sitter/tree-sitter-c-sharp/tree/master/bindings/rust).

`ExtendedTree` allows to iterate over/filter [node](https://docs.rs/tree-sitter/latest/tree_sitter/struct.Node.html)s of a [tree sitter tree](https://docs.rs/tree-sitter/latest/tree_sitter/struct.Tree.html).

# Example Usage
```rust
let code = r#"class Program { }"#;

let tree = ExtendedTree::from_source_code(code);

let node = tree
    .into_iter()
    .find(|n| n.ts_node.kind() == "class_declaration")
    .unwrap();

// view the source code of the node
let node_source_code = &node.source_code;

assert!(node_source_code.contains("class Program"));

// print the contents of the original tree sitter node
println!("{}", node.ts_node.to_sexp());

// traverse the sub-tree of a given node
for child_node in node.into_iter() {
    println!("{}", child_node.source_code);
}
```