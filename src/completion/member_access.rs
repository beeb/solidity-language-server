use serde_json::Value;
use std::collections::HashSet;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position};

fn get_variable_type(ast_data: &Value, var_name: &str) -> Option<String> {
    println!("Looking for variable type of '{}'", var_name);
    // Navigate to the AST like in extract_symbols
    if let Some(sources) = ast_data.get("sources")
        && let Some(sources_obj) = sources.as_object()
        && let Some((_, contents)) = sources_obj.iter().next()
        && let Some(contents_array) = contents.as_array()
        && let Some(first_content) = contents_array.first()
        && let Some(source_file) = first_content.get("source_file")
        && let Some(ast) = source_file.get("ast") {
        find_type_in_ast(ast, var_name)
    } else {
        None
    }
}

fn find_type_in_ast(node: &Value, name: &str) -> Option<String> {
    fn extract_type(type_name_node: Option<&Value>) -> Option<String> {
        if let Some(type_name_node) = type_name_node {
            if let Some(name) = type_name_node.get("name").and_then(|v| v.as_str()) {
                // For user-defined types, take the last part after '.'
                let base_name = name.split('.').next_back().unwrap_or(name);
                return Some(base_name.to_string());
            }
            // For user-defined types
            if let Some(contract) = type_name_node
                .get("contract")
                .and_then(|c| c.get("name"))
                .and_then(|v| v.as_str())
            {
                return Some(contract.to_string());
            }
            // For user-defined types like structs
            if let Some(name) = type_name_node
                .get("pathNode")
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
            {
                return Some(name.to_string());
            }
        }
        None
    }
    if let Some(node_type) = node.get("nodeType").and_then(|v| v.as_str()) {
        println!("Checking node type: {}", node_type);
        let type_name_node = if node_type == "VariableDeclaration"
            && node.get("name").and_then(|v| v.as_str()) == Some(name)
        {
            println!("Found VariableDeclaration for '{}'", name);
            node.get("typeName")
        } else if node_type == "VariableDeclarationStatement" {
            if let Some(declarations) = node.get("declarations").and_then(|v| v.as_array()) {
                for decl in declarations {
                    if decl.get("name").and_then(|v| v.as_str()) == Some(name) {
                        println!("Found VariableDeclarationStatement for '{}'", name);
                        return extract_type(decl.get("typeName"));
                    }
                }
            }
            None
        } else {
            None
        };

        if type_name_node.is_some() {
            return extract_type(type_name_node);
        }
    }
    if let Some(nodes) = node.get("nodes").and_then(|v| v.as_array()) {
        println!("Recursing into {} nodes", nodes.len());
        for n in nodes {
            if let Some(t) = find_type_in_ast(n, name) {
                return Some(t);
            }
        }
    }
    if let Some(statements) = node.get("statements").and_then(|v| v.as_array()) {
        println!("Recursing into {} statements", statements.len());
        for s in statements {
            if let Some(t) = find_type_in_ast(s, name) {
                return Some(t);
            }
        }
    }
    if let Some(body) = node.get("body") {
        println!("Recursing into body");
        if let Some(t) = find_type_in_ast(body, name) {
            return Some(t);
        }
    }
    None
}

fn get_using_directives(ast: &Value) -> Vec<(String, String)> {
    let mut usings = Vec::new();
    if let Some(nodes) = ast.get("nodes").and_then(|n| n.as_array()) {
        for node in nodes {
            if node.get("nodeType").and_then(|nt| nt.as_str()) == Some("UsingForDirective")
                && let Some(library) = node
                    .get("libraryName")
                    .and_then(|ln| ln.get("name"))
                    .and_then(|n| n.as_str())
            {
                let type_name = node
                    .get("typeName")
                    .and_then(|tn| tn.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("*");
                usings.push((library.to_string(), type_name.to_string()));
            }
            // Recurse into subnodes
            usings.extend(get_using_directives(node));
        }
    }
    usings
}

pub fn get_dot_completions(
    text: &str,
    ast_data: &Value,
    position: Position,
) -> Option<(Vec<CompletionItem>, String)> {
    // Check if the text before position ends with '.'
    let byte_offset =
        crate::utils::position_to_byte_offset(text, position.line, position.character);
    let before_cursor = &text[..byte_offset];
    if !before_cursor.ends_with('.') {
        return None;
    }

    // Remove the trailing '.'
    let before_dot = &before_cursor[..before_cursor.len() - 1];

    // Trim trailing whitespace and get the last word
    let identifier = before_dot
        .trim_end()
        .rsplit(' ')
        .next()
        .unwrap_or("")
        .to_string();

    if identifier.is_empty() || !identifier.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // Now, find what this identifier refers to
    let all_symbols = crate::symbols::extract_symbols(ast_data);
    let mut relevant_completions = Vec::new();
    let mut seen_labels = HashSet::new();

    for symbol in &all_symbols {
        if symbol.name == identifier {
            // Found the symbol, now get its members
            match symbol.kind {
                tower_lsp::lsp_types::SymbolKind::CLASS
                | tower_lsp::lsp_types::SymbolKind::INTERFACE
                | tower_lsp::lsp_types::SymbolKind::STRUCT
                | tower_lsp::lsp_types::SymbolKind::ENUM => {
                    // For contracts, interfaces, structs, enums, show their members
                    for member_symbol in &all_symbols {
                        if let Some(container_name) = &member_symbol.container_name
                            && container_name == &identifier
                        {
                            let kind = match member_symbol.kind {
                                tower_lsp::lsp_types::SymbolKind::FUNCTION => {
                                    CompletionItemKind::FUNCTION
                                }
                                tower_lsp::lsp_types::SymbolKind::FIELD => {
                                    CompletionItemKind::FIELD
                                }
                                tower_lsp::lsp_types::SymbolKind::EVENT => {
                                    CompletionItemKind::EVENT
                                }
                                tower_lsp::lsp_types::SymbolKind::METHOD => {
                                    CompletionItemKind::METHOD
                                }
                                _ => CompletionItemKind::VARIABLE,
                            };
                            if seen_labels.insert(member_symbol.name.clone()) {
                                relevant_completions.push(CompletionItem {
                                    label: member_symbol.name.clone(),
                                    kind: Some(kind),
                                    detail: Some(member_symbol.name.clone()),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                    if !relevant_completions.is_empty() {
                        return Some((relevant_completions, identifier));
                    }
                }
                _ => {
                    // For variables, we would need to find their type, but for now, skip
                    // TODO: Implement type resolution for variables
                }
            }
            break;
        }
    }

    // Check for global built-in objects
    match identifier.as_str() {
        "abi" => return Some((super::abi::completions(), identifier)),
        "msg" => return Some((super::msg::completions(), identifier)),
        "block" => return Some((super::block::completions(), identifier)),
        "tx" => return Some((super::tx::completions(), identifier)),
        _ => {}
    }

    // Check if identifier is a variable and get its type
    if let Some(type_name) = get_variable_type(ast_data, &identifier) {
        match type_name.as_str() {
            "address" => return Some((super::address::completions(), identifier)),
            "address payable" => return Some((super::address_payable::completions(), identifier)),
            // Add other built-in types as needed
            _ => {
                // User-defined type
                if let Some(type_symbol) = all_symbols.iter().find(|s| {
                    let base_name = type_name.split('.').next_back().unwrap_or(&type_name);
                    s.name == base_name && (s.kind == tower_lsp::lsp_types::SymbolKind::STRUCT || s.kind == tower_lsp::lsp_types::SymbolKind::CLASS || s.kind == tower_lsp::lsp_types::SymbolKind::INTERFACE || s.kind == tower_lsp::lsp_types::SymbolKind::ENUM)
                }) {
                    let mut relevant_completions = Vec::new();
                    let mut seen_labels = HashSet::new();
                    for member_symbol in &all_symbols {
                        if let Some(container_name) = &member_symbol.container_name && container_name == &type_symbol.name {
                            let kind = match member_symbol.kind {
                                tower_lsp::lsp_types::SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
                                tower_lsp::lsp_types::SymbolKind::FIELD => CompletionItemKind::FIELD,
                                tower_lsp::lsp_types::SymbolKind::EVENT => CompletionItemKind::EVENT,
                                tower_lsp::lsp_types::SymbolKind::METHOD => CompletionItemKind::METHOD,
                                _ => CompletionItemKind::VARIABLE,
                            };
                            if seen_labels.insert(member_symbol.name.clone()) {
                                relevant_completions.push(CompletionItem {
                                    label: member_symbol.name.clone(),
                                    kind: Some(kind),
                                    detail: Some(member_symbol.name.clone()),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                    if !relevant_completions.is_empty() {
                        return Some((relevant_completions, identifier));
                    }
                }
            }
        }
    }

    // Also check for library functions via using directives
    let usings = get_using_directives(ast_data);
    for (library, type_name) in usings {
        if type_name == "*" {
            // Add functions from this library for any type
            for symbol in &all_symbols {
                if symbol.container_name.as_ref() == Some(&library)
                    && symbol.kind == tower_lsp::lsp_types::SymbolKind::FUNCTION
                    && seen_labels.insert(symbol.name.clone())
                {
                    relevant_completions.push(CompletionItem {
                        label: symbol.name.clone(),
                        kind: Some(CompletionItemKind::METHOD),
                        detail: Some(format!("{}.{}", library, symbol.name)),
                        ..Default::default()
                    });
                }
            }
        }
        // TODO: For specific types, check if the identifier's type matches type_name
    }

    if relevant_completions.is_empty() {
        None
    } else {
        Some((relevant_completions, identifier))
    }
}
