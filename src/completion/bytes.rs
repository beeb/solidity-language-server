use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "concat".to_string(),
            detail: Some("bytes.concat(...) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("Concatenates variable number of bytes and bytes1, …, bytes32 arguments to one byte array".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
    ]
}
