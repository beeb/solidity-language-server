use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

pub fn completions() -> Vec<CompletionItem> {
    vec![
    CompletionItem {
        label: "bool".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint8".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint16".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint24".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint32".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint40".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint48".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint56".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint64".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint72".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint80".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint88".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint96".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint104".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint112".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint120".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint128".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint136".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint144".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint152".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint160".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint168".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint176".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint184".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint192".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint200".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint208".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint216".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint224".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint232".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint240".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint248".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "uint256".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int8".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int16".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int24".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int32".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int40".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int48".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int56".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int64".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int72".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int80".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int88".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int96".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int104".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int112".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int120".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int128".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int136".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int144".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int152".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int160".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int168".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int176".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int184".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int192".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int200".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int208".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int216".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int224".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int232".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int240".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int248".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "int256".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes1".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes2".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes3".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes4".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes5".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes6".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes7".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes8".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes9".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes10".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes11".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes12".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes13".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes14".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes15".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes16".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes17".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes18".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes19".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes20".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes21".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes22".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes23".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes24".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes25".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes26".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes27".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes28".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes29".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes30".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes31".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "bytes32".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "string".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    CompletionItem {
        label: "address".to_string(),
        kind: Some(CompletionItemKind::TYPE_PARAMETER),
        ..Default::default()
    },
    ]
}