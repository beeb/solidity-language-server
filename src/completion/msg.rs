use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "data".to_string(),
            detail: Some("(member) bytes calldata".to_string()),
            documentation: Some(Documentation::String("complete calldata".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "sender".to_string(),
            detail: Some("(member) address".to_string()),
            documentation: Some(Documentation::String("sender of the message (current call)".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "sig".to_string(),
            detail: Some("(member) bytes4".to_string()),
            documentation: Some(Documentation::String("first four bytes the calldata (i.e. function identifier)".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
    ]
}
