use tree_sitter::{Node, Parser, Tree, TreeCursor};
use tree_sitter_c;

pub fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("Error loadiing C program");

    parser.parse(source, None).expect("Failed to parse")
}

pub fn find_fn_calls(node: Node, source: &str, fn_name: &str) {
    let mut cursor = node.walk();

    find_nodes(node, &mut cursor, source, fn_name);
}

pub fn find_nodes(node: Node, cursor: &mut TreeCursor, source: &str, to_find: &str) {
    // Check current node
    if node.kind() == "call_expression" {
        if let Some(func_node) = node.child_by_field_name("function") {
            let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

            if func_name == to_find {
                println!(
                    "[valknir] {to_find:?} detected at line {}",
                    node.start_position().row + 1
                );
            }
        }
    }

    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            find_nodes(child, cursor, source, to_find);

            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}
