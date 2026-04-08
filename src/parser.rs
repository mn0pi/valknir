use std::collections::HashMap;
use tree_sitter::{Node, Parser, Tree};
use tree_sitter_c;

use crate::analysis::state::{AllocationMap, AllocationState};

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

pub fn find_nodes(node: Node, source: &str, allocations: &mut AllocationMap) {
    if node.kind() == "assignment_expression" {
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");

        if let (Some(left), Some(right)) = (left, right) {
            if right.kind() == "call_expression" {
                if let Some(func_name) = get_function_name(right, source) {
                    if func_name == "malloc" {
                        if let Some(var_name) = get_identifier_name(left, source) {
                            let line = node.start_position().row + 1;

                            allocations
                                .insert(var_name.clone(), AllocationState::Allocated { line });

                            println!("[valknir] malloc: {} at line {}!", var_name, line);
                        }
                    }
                }
            }
        }
    } else if node.kind() == "call_expression" {
        if let Some(func_name) = get_function_name(node, source) {
            if func_name == "free" {
                if let Some(arg_node) = node.child_by_field_name("arguments") {
                    if let Some(var_name) = extract_argument_name(arg_node, source) {
                        let line = node.start_position().row + 1;

                        let is_double_free = matches!(
                            allocations.get(&var_name),
                            Some(AllocationState::Freed { .. })
                        );

                        if is_double_free {
                            println!(
                                "[valknir] double free detected: {} at line {}",
                                var_name, line
                            );
                        }

                        allocations.insert(var_name.clone(), AllocationState::Freed { line });
                    }
                }
            }
        }
    }

    for child in node.children(&mut node.walk()) {
        find_nodes(child, source, allocations);
    }
}

fn get_function_name(node: Node, source: &str) -> Option<String> {
    let func = node.child_by_field_name("function")?;
    Some(func.utf8_text(source.as_bytes()).ok()?.to_string())
}

fn get_identifier_name(node: Node, source: &str) -> Option<String> {
    if node.kind() == "identifier" {
        Some(node.utf8_text(source.as_bytes()).ok()?.to_string())
    } else {
        None
    }
}

fn extract_argument_name(node: Node, source: &str) -> Option<String> {
    let arg = node.named_child(0)?;
    Some(arg.utf8_text(source.as_bytes()).ok()?.to_string())
}
