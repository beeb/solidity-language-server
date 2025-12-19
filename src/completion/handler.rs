use serde_json::Value;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, Position};

use super::{elementary_type, global_symbol, keyword, member_access};

fn extract_query(text: &str, position: Position) -> Option<String> {
    let byte_offset = crate::utils::position_to_byte_offset(text, position.line, position.character);
    let before_cursor = &text[..byte_offset];
    if before_cursor.is_empty() {
        return None;
    }

    // Find the last word/identifier
    let last_word = before_cursor
        .trim_end()
        .rsplit(|c: char| !c.is_alphanumeric() && c != '_')
        .next()
        .unwrap_or("")
        .to_string();

    if last_word.is_empty() || !last_word.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.') {
        None
    } else {
        Some(last_word)
    }
}

pub fn get_completions(
    text: &str,
    ast_data: &Value,
    position: Position,
    params: &CompletionParams,
) -> (Vec<CompletionItem>, Option<String>) {
    let mut completions = Vec::new();

    // Extract the query (identifier before cursor)
    let query = extract_query(text, position);

    // Check if this is a dot completion
    let is_dot_completion = params
        .context
        .as_ref()
        .and_then(|ctx| ctx.trigger_character.as_ref())
        .map(|t| t == ".")
        .unwrap_or(false)
        || query.as_ref().map(|q| q.ends_with('.')).unwrap_or(false);

    if is_dot_completion {
        // Dot completion - need to determine type and return appropriate completions
        return get_dot_completions(text, ast_data, position);
    }

    // Default completions
    completions.extend(global_symbol::completions());
    completions.extend(elementary_type::completions());
    completions.extend(keyword::completions());

    // Add AST-based scoped completions
    completions.extend(get_scoped_completions(ast_data, text, position));

    (completions, query)
}

fn get_scoped_completions(
    ast_data: &Value,
    _text: &str,
    _position: Position,
) -> Vec<CompletionItem> {
    // Extract symbols from AST and provide completions for in-scope items
    // This is a simplified version - a full implementation would need proper scope analysis
    crate::symbols::extract_symbols(ast_data)
        .into_iter()
        .map(|symbol| {
            let kind = match symbol.kind {
                tower_lsp::lsp_types::SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
                tower_lsp::lsp_types::SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
                tower_lsp::lsp_types::SymbolKind::CLASS => CompletionItemKind::CLASS,
                tower_lsp::lsp_types::SymbolKind::INTERFACE => CompletionItemKind::INTERFACE,
                tower_lsp::lsp_types::SymbolKind::STRUCT => CompletionItemKind::STRUCT,
                tower_lsp::lsp_types::SymbolKind::ENUM => CompletionItemKind::ENUM,
                tower_lsp::lsp_types::SymbolKind::EVENT => CompletionItemKind::EVENT,
                _ => CompletionItemKind::VARIABLE,
            };
            CompletionItem {
                label: symbol.name.clone(),
                kind: Some(kind),
                detail: Some(symbol.name),
                ..Default::default()
            }
        })
        .collect()
}

fn get_dot_completions(text: &str, ast_data: &Value, position: Position) -> (Vec<CompletionItem>, Option<String>) {
    // Use the member_access module for proper type detection
    member_access::get_dot_completions(text, ast_data, position).map(|(comps, query)| (comps, Some(query))).unwrap_or_default()
}
