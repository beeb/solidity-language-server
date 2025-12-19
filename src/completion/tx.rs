use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "gasprice".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("gas price of the transaction".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "origin".to_string(),
            detail: Some("(member) address".to_string()),
            documentation: Some(Documentation::String("sender of the transaction (full call chain)".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
    ]
}
