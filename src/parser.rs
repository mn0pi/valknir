use tree_sitter::{Parser, Tree};
use tree_sitter_c;

pub fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("Error loadiing C program");

    parser.parse(source, None).expect("Failed to parse")
}
