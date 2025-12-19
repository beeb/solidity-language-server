use serde_json::Value;
use std::collections::HashSet;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position};

pub fn get_dot_completions(text: &str, ast_data: &Value, position: Position) -> Option<Vec<CompletionItem>> {
    // Check if the text before position ends with '.'
    let byte_offset = crate::utils::position_to_byte_offset(text, position.line, position.character);
    let before_cursor = &text[..byte_offset];
    if !before_cursor.ends_with('.') {
        return None;
    }

    // Remove the trailing '.'
    let before_dot = &before_cursor[..before_cursor.len() - 1];

    // Trim trailing whitespace and get the last word
    let identifier = before_dot.trim_end().rsplit(' ').next().unwrap_or("").to_string();

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
                tower_lsp::lsp_types::SymbolKind::CLASS |
                tower_lsp::lsp_types::SymbolKind::INTERFACE |
                tower_lsp::lsp_types::SymbolKind::STRUCT |
                tower_lsp::lsp_types::SymbolKind::ENUM => {
                    // For contracts, interfaces, structs, enums, show their members
                    for member_symbol in &all_symbols {
                        if let Some(container_name) = &member_symbol.container_name
                            && container_name == &identifier {
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
                }
                _ => {
                    // For variables, we would need to find their type, but for now, skip
                    // TODO: Implement type resolution for variables
                }
            }
            break;
        }
    }

    // Also check for library functions via using directives
    // TODO: Parse using directives and add library functions

    if relevant_completions.is_empty() {
        // Check for global built-in objects
        match identifier.as_str() {
            "abi" => return Some(super::abi::completions()),
            "msg" => return Some(super::msg::completions()),
            "block" => return Some(super::block::completions()),
            "tx" => return Some(super::tx::completions()),
            _ => {}
        }
        None
    } else {
        Some(relevant_completions)
    }
}
