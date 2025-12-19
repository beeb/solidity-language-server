use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation};

pub fn completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "length".to_string(),
            detail: Some("(member) uint256".to_string()),
            documentation: Some(Documentation::String("Arrays have a length member that contains their number of elements. The length of memory arrays is fixed (but dynamic, i.e. it can depend on runtime parameters) once they are created.".to_string())),
            kind: Some(CompletionItemKind::PROPERTY),
            ..Default::default()
        },
        CompletionItem {
            label: "push".to_string(),
            detail: Some("push()".to_string()),
            documentation: Some(Documentation::String("Dynamic storage arrays and bytes (not string) have a member function called push() that you can use to append a zero-initialised element at the end of the array. It returns a reference to the element, so that it can be used like x.push().t = 2 or x.push() = b.".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "push".to_string(),
            detail: Some("push(x)".to_string()),
            documentation: Some(Documentation::String("Dynamic storage arrays and bytes (not string) have a member function called push(x) that you can use to append a given element at the end of the array. The function returns nothing.".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
        CompletionItem {
            label: "pop".to_string(),
            detail: Some("pop()".to_string()),
            documentation: Some(Documentation::String("Dynamic storage arrays and bytes (not string) have a member function called pop() that you can use to remove an element from the end of the array. This also implicitly calls delete on the removed element.".to_string())),
            kind: Some(CompletionItemKind::METHOD),
            ..Default::default()
        },
    ]
}