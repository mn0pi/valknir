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
    //TODO: Change this to return structured result rather than printing directly from find_nodes
    let mut allocations: AllocationMap = HashMap::new();

    let tree = parse(source);
    let root = tree.root_node();

    find_nodes(root, source, &mut allocations);
}

fn find_nodes(node: Node, source: &str, allocations: &mut AllocationMap) {
    for child in node.children(&mut node.walk()) {
        if let Some(identifier) = child.child_by_field_name("declarator") {
            let identifier_name = identifier
                .utf8_text(source.as_bytes())
                .unwrap_or("")
                .trim()
                .replace("*", "");

            if let Some(function_node) = child.child_by_field_name("value") {
                if let Some(function) = function_node.child_by_field_name("function") {
                    let func_name = function.utf8_text(source.as_bytes()).unwrap_or("");

                    if func_name == "malloc" {
                        allocations.insert(
                            identifier_name.clone(),
                            AllocationState::Allocated {
                                alloc_line: node.start_position().row + 1,
                            },
                        );
                    }
                }
            }
        }
        if let Some(func_node) = child.child_by_field_name("function") {
            let func_name = func_node.utf8_text(source.as_bytes()).unwrap_or("");

            if func_name == "free" {
                if let Some(arg_node) = child.child_by_field_name("arguments") {
                    //TODO: Only handles simple identifier arguments e.g. free(p)
                    // complex expressions e.g. free(ptr->field) not yet supported
                    if let Some(identifier_node) = arg_node.child(1) {
                        if let Some(identifier) = get_identifier_name(identifier_node, source) {
                            let line = node.start_position().row + 1;
                            if let Some(state) = allocations.get_mut(&identifier) {
                                match state {
                                    AllocationState::Allocated { alloc_line } => {
                                        *state = AllocationState::Freed {
                                            alloc_line: *alloc_line,
                                            free_line: line,
                                        };
                                    }

                                    AllocationState::Freed {
                                        alloc_line,
                                        free_line,
                                    } => {
                                        println!("[valknir] double free detected");
                                        println!("Pointer: {}", &identifier);
                                        println!("Allocation: line {}", alloc_line);
                                        println!("First free: line {}", free_line);
                                        println!("Second free: line {}", line);
                                        println!(
                                            "Explanation: Memory freed more than once. This can corrupt heap metadata and may lead to arbitrary code execution."
                                        );
                                        println!();
                                    }
                                }
                            } else {
                                println!(
                                    "[valknir] warning: free of untracked pointer {} at line {}",
                                    identifier, line
                                );
                            }
                        }
                    }
                }
            }
        }
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
        "init_declarator" => {
            let declarator = node.child_by_field_name("declarator");

            if let Some(declarator_node) = declarator {
                get_identifier_name(declarator_node, source)
            } else {
                None
            }
        }
        "arguments" => {
            let inner = node.child_by_field_name("identifier")?;
            get_identifier_name(inner, source)
        }
        _ => None,
    }
}
