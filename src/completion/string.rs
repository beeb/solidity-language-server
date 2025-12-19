use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "concat".to_string(),
            detail: Some("string.concat(...) returns (string memory)".to_string()),
            documentation: Some(Documentation::String("Concatenates variable number of string arguments to one string array".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
    ]
}
