use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "name".to_string(),
            documentation: Some(Documentation::String("The name of the contract.".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "creationCode".to_string(),
            documentation: Some(Documentation::String("Memory byte array that contains the creation bytecode of the contract. This can be used in inline assembly to build custom creation routines, especially by using the create2 opcode. This property can not be accessed in the contract itself or any derived contract. It causes the bytecode to be included in the bytecode of the call site and thus circular references like that are not possible.".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "runtimeCode".to_string(),
            documentation: Some(Documentation::String("Memory byte array that contains the runtime bytecode of the contract. This is the code that is usually deployed by the constructor of C. If C has a constructor that uses inline assembly, this might be different from the actually deployed bytecode. Also note that libraries modify their runtime bytecode at time of deployment to guard against regular calls. The same restrictions as with .creationCode also apply for this property.".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
    ]
}
