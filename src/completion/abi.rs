use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "decode".to_string(),
            detail: Some("abi.decode(bytes memory encodedData, (...)) returns (...)".to_string()),
            documentation: Some(Documentation::String("ABI-decodes the given data, while the types are given in parentheses as second argument. Example: `(uint a, uint[2] memory b, bytes memory c) = abi.decode(data, (uint, uint[2], bytes))`".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "encode".to_string(),
            detail: Some("abi.encode(...) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("ABI-encodes the given arguments".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "encodePacked".to_string(),
            detail: Some("abi.encodePacked(...) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("Performs packed encoding of the given arguments. Note that packed encoding can be ambiguous!".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "encodeWithSelector".to_string(),
            detail: Some("abi.encodeWithSelector(bytes4 selector, ...) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("ABI-encodes the given arguments starting from the second and prepends the given four-byte selector".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "encodeWithSignature".to_string(),
            detail: Some("abi.encodeWithSignature(string memory signature, ...) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("Equivalent to `abi.encodeWithSelector(bytes4(keccak256(bytes(signature))), ...)`".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "encodeCall".to_string(),
            detail: Some("abi.encodeCall(function functionPointer, (...)) returns (bytes memory)".to_string()),
            documentation: Some(Documentation::String("ABI-encodes a call to `functionPointer` with the arguments found in the tuple. Performs a full type-check, ensuring the types match the function signature. Result equals `abi.encodeWithSelector(functionPointer.selector, (...))`".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
    ]
}