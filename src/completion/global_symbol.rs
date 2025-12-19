use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "msg".to_string(),
            documentation: Some(Documentation::String("message".to_string())),
            kind: Some(CompletionItemKind::MODULE),
            ..Default::default()
        },
        CompletionItem {
            label: "block".to_string(),
            documentation: Some(Documentation::String("current block".to_string())),
            kind: Some(CompletionItemKind::MODULE),
            ..Default::default()
        },
        CompletionItem {
            label: "tx".to_string(),
            documentation: Some(Documentation::String("current transaction".to_string())),
            kind: Some(CompletionItemKind::MODULE),
            ..Default::default()
        },
        CompletionItem {
            label: "blockhash".to_string(),
            detail: Some("blockhash(uint blockNumber) returns (bytes32)".to_string()),
            documentation: Some(Documentation::String(
                "hash of the given block when `blocknumber` is one of the 256 most recent blocks; otherwise returns zero".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "gasleft".to_string(),
            detail: Some("gasleft() returns (uint256)".to_string()),
            documentation: Some(Documentation::String("remaining gas".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "abi".to_string(),
            documentation: Some(Documentation::String("ABI encoding and decoding".to_string())),
            kind: Some(CompletionItemKind::MODULE),
            ..Default::default()
        },
        CompletionItem {
            label: "assert".to_string(),
            detail: Some("assert(bool condition)".to_string()),
            documentation: Some(Documentation::String(
                "causes a Panic error and thus state change reversion if the condition is not met - to be used for internal errors.".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "require".to_string(),
            detail: Some("require(bool condition, string message)".to_string()),
            documentation: Some(Documentation::String(
                "reverts if the condition is not met - to be used for errors in inputs or external components. Also provides an error message.".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "revert".to_string(),
            detail: Some("revert(string memory message)".to_string()),
            documentation: Some(Documentation::String(
                "abort execution and revert state changes, providing an explanatory string".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "addmod".to_string(),
            detail: Some("addmod(uint x, uint y, uint k) returns (uint)".to_string()),
            documentation: Some(Documentation::String(
                "compute (x + y) % k where the addition is performed with arbitrary precision and does not wrap around at 2**256. Assert that k != 0 starting from version 0.5.0.".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "mulmod".to_string(),
            detail: Some("mulmod(uint x, uint y, uint k) returns (uint)".to_string()),
            documentation: Some(Documentation::String(
                "compute (x * y) % k where the multiplication is performed with arbitrary precision and does not wrap around at 2**256. Assert that k != 0 starting from version 0.5.0.".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "keccak256".to_string(),
            detail: Some("keccak256(bytes memory) returns (bytes32)".to_string()),
            documentation: Some(Documentation::String("compute the Keccak-256 hash of the input".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "sha256".to_string(),
            detail: Some("sha256(bytes memory) returns (bytes32)".to_string()),
            documentation: Some(Documentation::String("compute the SHA-256 hash of the input".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "ripemd160".to_string(),
            detail: Some("ripemd160(bytes memory) returns (bytes32)".to_string()),
            documentation: Some(Documentation::String("compute the RIPEMD-160 hash of the input".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "ecrecover".to_string(),
            detail: Some("ecrecover(bytes32 hash, uint8 v, bytes32 r, bytes32 s) returns (address)".to_string()),
            documentation: Some(Documentation::String(
                "recover the address associated with the public key from elliptic curve signature or return zero on error. The function parameters correspond to ECDSA values of the signature:\n- r = first 32 bytes of signature\n- s = second 32 bytes of signature\n- v = final 1 byte of signature\n\necrecover returns an address, and not an address payable. See address payable for conversion, in case you need to transfer funds to the recovered address.\n\n## Warning\nIf you use ecrecover, be aware that a valid signature can be turned into a different valid signature without requiring knowledge of the corresponding private key. In the Homestead hard fork, this issue was fixed for _transaction_ signatures (see EIP-2), but the ecrecover function remained unchanged.\n\nThis is usually not a problem unless you require signatures to be unique or use them to identify items. OpenZeppelin have a ECDSA helper library that you can use as a wrapper for ecrecover without this issue.".to_string()
            )),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "tload".to_string(),
            detail: Some("tload(uint256 slot) returns (uint256)".to_string()),
            documentation: Some(Documentation::String("load from transient storage".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
        CompletionItem {
            label: "tstore".to_string(),
            detail: Some("tstore(uint256 slot, uint256 value)".to_string()),
            documentation: Some(Documentation::String("store to transient storage".to_string())),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        },
    ]
}