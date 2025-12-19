use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    let mut completions = super::address::completions();
    completions.extend(vec![
        CompletionItem {
            label: "transfer".to_string(),
            detail: Some("<address payable>.transfer(uint256 amount)".to_string()),
            documentation: Some(Documentation::String("send given amount of Wei to Address, reverts on failure, forwards 2300 gas stipend, not adjustable".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "send".to_string(),
            detail: Some("<address payable>.send(uint256 amount) returns (bool)".to_string()),
            documentation: Some(Documentation::String("send given amount of Wei to Address, returns false on failure, forwards 2300 gas stipend, not adjustable".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
    ]);
    completions
}