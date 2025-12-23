use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_parse() {
    let result = tree_sitter_go_wasm_repro::parse_go(
        "package main\nfunc main() { println(\"Hello, World!\") }",
    );
    assert!(result.contains("println"));
}
