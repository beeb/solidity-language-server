use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

pub fn completions() -> Vec<CompletionItem> {
    vec![
    CompletionItem {
        label: "pragma".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "solidity".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "contract".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "interface".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "library".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "function".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "modifier".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "event".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "struct".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "enum".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "mapping".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "address".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "payable".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "public".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "private".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "internal".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "external".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "view".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "pure".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "constant".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "immutable".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "storage".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "memory".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "calldata".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "if".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "else".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "for".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "while".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "do".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "break".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "continue".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "return".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "emit".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "require".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "assert".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "revert".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "try".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "catch".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "using".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "import".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "from".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "as".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "is".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "new".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "delete".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "this".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "super".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "selfdestruct".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "constructor".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "fallback".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "receive".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "virtual".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "override".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    CompletionItem {
        label: "abstract".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    },
    ]
}