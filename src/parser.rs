use crate::analysis::state::{AllocationMap, AllocationState};

use std::collections::HashMap;

use tree_sitter::{Node, Parser, Tree};
use tree_sitter_c;

pub fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("Error loadiing C program");

    parser.parse(source, None).expect("Failed to parse")
}

pub fn analyse(source: &str) {
    let mut allocations: AllocationMap = HashMap::new();

    let tree = parse(source);
    let root = tree.root_node();

    find_nodes(root, source, &mut allocations);
}

fn find_nodes(node: Node, source: &str, allocations: &mut AllocationMap) {
    let mut var_name = String::new();
    if node.kind() == "init_declarator" {
        let declarator = node.child_by_field_name("declarator");

        if let Some(declarator_node) = declarator {
            if let Some(identifier) = get_identifier_name(declarator_node, source) {
                var_name = identifier;
            }
        }
    }
    for child in node.children(&mut node.walk()) {
        if child.kind() == "call_expression" {
            if let Some(func_node) = child.child_by_field_name("function") {
                let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

                if func_name == "malloc" {
                    let line = node.start_position().row + 1;

                    allocations.insert(var_name.to_string(), AllocationState::Allocated { line });
                    println!("[valknir] malloc detected at line {}", line);
                } else if func_name == "free" {
                    let line = node.start_position().row + 1;
                    let is_double_free = matches! {
                        allocations.get(var_name.as_str().clone()),
                        Some(AllocationState::Freed { .. })
                    };

                    if is_double_free {
                        println!("double free at line: {}", line);
                    }

                    allocations.insert(var_name.clone(), AllocationState::Freed { line });
                }
            }
        }
    }

    for child in node.children(&mut node.walk()) {
        find_nodes(child, source, allocations);
    }
}

fn get_identifier_name(node: Node, source: &str) -> Option<String> {
    match node.kind() {
        "identifier" => Some(node.utf8_text(source.as_bytes()).ok()?.to_string()),
        "pointer_declarator" | "parenthesized_declarator" => {
            let inner = node.child_by_field_name("declarator")?;
            get_identifier_name(inner, source)
        }
        _ => None,
    }
}
