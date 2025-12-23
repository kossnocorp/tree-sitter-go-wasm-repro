use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_go(source: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    tree.root_node().to_sexp()
}
