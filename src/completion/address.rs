use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
    CompletionItem {
        label: "balance".to_string(),
        detail: Some("<address>.balance (uint256)".to_string()),
        documentation: Some(Documentation::String("balance of the Address in Wei".to_string())),
        kind: Some(CompletionItemKind::PROPERTY),
        ..Default::default()
    },
    CompletionItem {
        label: "code".to_string(),
        detail: Some("<address>.code (bytes memory)".to_string()),
        documentation: Some(Documentation::String("code at the Address (can be empty)".to_string())),
        kind: Some(CompletionItemKind::PROPERTY),
        ..Default::default()
    },
    CompletionItem {
        label: "codehash".to_string(),
        detail: Some("<address>.codehash (bytes32)".to_string()),
        documentation: Some(Documentation::String("the codehash of the Address".to_string())),
        kind: Some(CompletionItemKind::PROPERTY),
        ..Default::default()
    },
    CompletionItem {
        label: "call".to_string(),
        detail: Some("<address>.call(bytes memory) returns (bool, bytes memory)".to_string()),
        documentation: Some(Documentation::String(
            "issue low-level `CALL` with the given payload, returns success condition and return data, forwards all available gas, adjustable".to_string()
        )),
        kind: Some(CompletionItemKind::METHOD),
        ..Default::default()
    },
    CompletionItem {
        label: "delegatecall".to_string(),
        detail: Some("<address>.delegatecall(bytes memory) returns (bool, bytes memory)".to_string()),
        documentation: Some(Documentation::String(
            "issue low-level `DELEGATECALL` with the given payload, returns success condition and return data, forwards all available gas, adjustable".to_string()
        )),
        kind: Some(CompletionItemKind::METHOD),
        ..Default::default()
    },
    CompletionItem {
        label: "staticcall".to_string(),
        detail: Some("<address>.staticcall(bytes memory) returns (bool, bytes memory)".to_string()),
        documentation: Some(Documentation::String(
            "issue low-level `STATICCALL` with the given payload, returns success condition and return data, forwards all available gas, adjustable".to_string()
        )),
        kind: Some(CompletionItemKind::METHOD),
        ..Default::default()
    },
    ]
}