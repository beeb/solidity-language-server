use serde_json::Value;
use std::collections::HashSet;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, Position};

pub fn get_default_completions(
    _text: &str,
    _ast_data: &Value,
    _position: Position,
) -> Vec<CompletionItem> {
    let mut completions = Vec::new();
    let mut seen_labels = HashSet::new();

    // Helper function to add completion if not duplicate and not empty
    let mut add_completion = |item: CompletionItem| {
        if !item.label.is_empty() && seen_labels.insert(item.label.clone()) {
            completions.push(item);
        }
    };

    // Add Solidity keywords
    let keywords = vec![
        "pragma", "solidity", "contract", "interface", "library", "function", "modifier", "event",
        "struct", "enum", "mapping", "address", "payable", "public", "private", "internal",
        "external", "view", "pure", "constant", "immutable", "storage", "memory", "calldata",
        "if", "else", "for", "while", "do", "break", "continue", "return", "emit", "require",
        "assert", "revert", "try", "catch", "using", "import", "from", "as", "is", "new",
        "delete", "this", "super", "selfdestruct", "constructor", "fallback", "receive",
        "virtual", "override", "abstract",
    ];

    for keyword in keywords {
        add_completion(CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        });
    }

    // Add built-in types
    let types = vec![
        "bool", "uint", "uint8", "uint16", "uint24", "uint32", "uint40", "uint48", "uint56",
        "uint64", "uint72", "uint80", "uint88", "uint96", "uint104", "uint112", "uint120",
        "uint128", "uint136", "uint144", "uint152", "uint160", "uint168", "uint176", "uint184",
        "uint192", "uint200", "uint208", "uint216", "uint224", "uint232", "uint240", "uint248",
        "uint256", "int", "int8", "int16", "int24", "int32", "int40", "int48", "int56", "int64",
        "int72", "int80", "int88", "int96", "int104", "int112", "int120", "int128", "int136",
        "int144", "int152", "int160", "int168", "int176", "int184", "int192", "int200", "int208",
        "int216", "int224", "int232", "int240", "int248", "int256", "bytes", "bytes1", "bytes2",
        "bytes3", "bytes4", "bytes5", "bytes6", "bytes7", "bytes8", "bytes9", "bytes10", "bytes11",
        "bytes12", "bytes13", "bytes14", "bytes15", "bytes16", "bytes17", "bytes18", "bytes19",
        "bytes20", "bytes21", "bytes22", "bytes23", "bytes24", "bytes25", "bytes26", "bytes27",
        "bytes28", "bytes29", "bytes30", "bytes31", "bytes32", "string", "address",
    ];

    for type_name in types {
        add_completion(CompletionItem {
            label: type_name.to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            ..Default::default()
        });
    }

    // Add global variables and functions
    let globals = vec![
        ("msg.sender", "address", "Sender of the message"),
        ("msg.value", "uint", "Number of wei sent with the message"),
        ("msg.data", "bytes", "Complete calldata"),
        ("msg.sig", "bytes4", "First four bytes of the calldata"),
        ("msg.gas", "uint", "Remaining gas"),
        ("block.timestamp", "uint", "Current block timestamp"),
        ("block.number", "uint", "Current block number"),
        ("block.difficulty", "uint", "Current block difficulty"),
        ("block.gaslimit", "uint", "Current block gaslimit"),
        ("block.coinbase", "address", "Current block miner's address"),
        ("blockhash", "function", "Hash of the given block"),
        ("gasleft", "function", "Remaining gas"),
        ("tx.origin", "address", "Sender of the transaction"),
        ("tx.gasprice", "uint", "Gas price of the transaction"),
        ("now", "uint", "Current block timestamp (deprecated)"),
        ("assert", "function", "Abort execution and revert state changes"),
        ("require", "function", "Check condition, throw if false"),
        ("revert", "function", "Abort execution and revert state changes"),
        ("keccak256", "function", "Compute the Keccak-256 hash"),
        ("sha256", "function", "Compute the SHA-256 hash"),
        ("ripemd160", "function", "Compute the RIPEMD-160 hash"),
        ("ecrecover", "function", "Recover the address associated with the public key"),
        ("addmod", "function", "Modular addition"),
        ("mulmod", "function", "Modular multiplication"),
        ("selfdestruct", "function", "Destroy the contract and send remaining funds"),
        ("type", "function", "Get type information"),
        ("abi.encode", "function", "ABI encode"),
        ("abi.encodePacked", "function", "ABI encode packed"),
        ("abi.encodeWithSelector", "function", "ABI encode with selector"),
        ("abi.encodeWithSignature", "function", "ABI encode with signature"),
        ("abi.decode", "function", "ABI decode"),
    ];

    for (name, type_info, detail) in globals {
        let kind = if type_info == "function" {
            CompletionItemKind::FUNCTION
        } else {
            CompletionItemKind::VARIABLE
        };

        add_completion(CompletionItem {
            label: name.to_string(),
            kind: Some(kind),
            detail: Some(type_info.to_string()),
            documentation: Some(Documentation::String(detail.to_string())),
            ..Default::default()
        });
    }

    // Add user-defined symbols from AST (scoped to current context)
    // TODO: Add AST-based completions

    completions
}
