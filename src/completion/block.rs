use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "basefee".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("base fee".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "chainid".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("current chain id".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "coinbase".to_string(),
            detail: Some("(member) address payable".to_string()),
            documentation: Some(Documentation::String("current block miner's address".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "difficulty".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("current block difficulty".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "gaslimit".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("current block gaslimit".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "number".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("current block number".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "timestamp".to_string(),
            detail: Some("(member) uint".to_string()),
            documentation: Some(Documentation::String("current block timestamp as seconds since unix epoch".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
    ]
}
