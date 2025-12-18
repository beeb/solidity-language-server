use serde_json::Value;
use tower_lsp::lsp_types::{Position, TextEdit, Url, WorkspaceEdit};
use crate::references;
use std::collections::HashMap;

pub fn get_identifier_at_position(source_bytes: &[u8], position: Position) -> Option<String> {
    let text = String::from_utf8_lossy(source_bytes);
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }
    let line = lines[position.line as usize];
    if position.character as usize > line.len() {
        return None;
    }
    let mut start = position.character as usize;
    let mut end = position.character as usize;

    while start > 0 
        && (line.as_bytes()[start - 1].is_ascii_alphanumeric()
            || line.as_bytes()[start - 1] == b'_')
    {
        start -= 1;
    }
    while end < line.len()
        && (line.as_bytes()[end].is_ascii_alphanumeric() || line.as_bytes()[end] == b'_')
    {
        end += 1;
    }

    if start == end {
        return None;
    }
    if line.as_bytes()[start].is_ascii_digit() {
        return None;
    }

    Some(line[start..end].to_string())
}


pub fn rename_symbol(
    ast_data: &Value,
    file_uri: &Url,
    position: Position,
    _source_bytes: &[u8],
    new_name: String
) -> Option<WorkspaceEdit> {
    let locations = references::goto_references(ast_data, file_uri, position, _source_bytes);
    if locations.is_empty() {
        return None;
    }
    let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
    for location in locations {
        let text_edit = TextEdit {
            range: location.range,
            new_text: new_name.clone(),

        };
        changes.entry(location.uri).or_default().push(text_edit);
    }
    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None
    })

}

