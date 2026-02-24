use crate::links;
use std::collections::HashMap;
use std::path::Path;
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url};

/// Compute import path edits needed when files are renamed/moved.
///
/// Uses tree-sitter to find import strings in each source file, making this
/// robust against stale or unavailable solc AST data (e.g. after a failed
/// re-index following a prior rename).
///
/// Handles two cases:
/// 1. **Other files import the renamed file** — their import path string must
///    change to reflect the new location.
/// 2. **The renamed file's own relative imports** — if the file moved to a
///    different directory, its relative imports to other (unmoved) files are
///    now wrong.
///
/// `source_files` is a list of absolute filesystem paths to `.sol` files in
/// the project.  The caller can populate this from `build.path_to_abs` keys,
/// from walking the project directory, or from foundry config paths.
///
/// `get_source_bytes` returns the source bytes for a given absolute filesystem
/// path.  In the LSP handler this checks `text_cache` (editor buffer) first,
/// then falls back to `fs::read`.
pub fn rename_imports(
    source_files: &[String],
    old_uri: &Url,
    new_uri: &Url,
    get_source_bytes: &dyn Fn(&str) -> Option<Vec<u8>>,
) -> HashMap<Url, Vec<TextEdit>> {
    let mut edits: HashMap<Url, Vec<TextEdit>> = HashMap::new();

    let old_fs_path = match old_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return edits,
    };
    let new_fs_path = match new_uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return edits,
    };

    // ── Case 1: other files that import the renamed file ────────────────
    for source_fs_str in source_files {
        let source_path = Path::new(source_fs_str);

        // Skip the file being renamed — handled in case 2.
        if source_path == old_fs_path {
            continue;
        }

        let source_dir = match source_path.parent() {
            Some(d) => d,
            None => continue,
        };

        let bytes = match get_source_bytes(source_fs_str) {
            Some(b) => b,
            None => continue,
        };

        let imports = links::ts_find_imports(&bytes);

        for imp in &imports {
            // Resolve the import path relative to the source file's directory.
            let resolved = source_dir.join(&imp.path);
            let resolved = normalize_path(&resolved);

            if resolved != old_fs_path {
                continue;
            }

            // This import points to the old file — compute the new relative path.
            let new_rel = match pathdiff::diff_paths(&new_fs_path, source_dir) {
                Some(p) => p,
                None => continue,
            };

            let source_uri = match Url::from_file_path(source_fs_str) {
                Ok(u) => u,
                Err(_) => continue,
            };

            edits.entry(source_uri).or_default().push(TextEdit {
                range: range_with_quotes(imp.inner_range),
                new_text: format!("\"{}\"", ensure_dot_prefix(&new_rel)),
            });
        }
    }

    // ── Case 2: the renamed file's own relative imports ─────────────────
    let old_dir = old_fs_path.parent();
    let new_dir = new_fs_path.parent();

    if old_dir != new_dir {
        let old_dir = match old_dir {
            Some(d) => d,
            None => return edits,
        };
        let new_dir = match new_dir {
            Some(d) => d,
            None => return edits,
        };

        let old_fs_str = match old_fs_path.to_str() {
            Some(s) => s,
            None => return edits,
        };

        let bytes = match get_source_bytes(old_fs_str) {
            Some(b) => b,
            None => return edits,
        };

        let imports = links::ts_find_imports(&bytes);

        for imp in &imports {
            // Only rewrite relative imports (starting with . or ..)
            if !imp.path.starts_with('.') {
                continue;
            }

            // Resolve the import target to an absolute filesystem path.
            let target_fs = normalize_path(&old_dir.join(&imp.path));

            // Compute new relative path from the file's new location.
            let new_rel = match pathdiff::diff_paths(&target_fs, new_dir) {
                Some(p) => p,
                None => continue,
            };

            edits.entry(old_uri.clone()).or_default().push(TextEdit {
                range: range_with_quotes(imp.inner_range),
                new_text: format!("\"{}\"", ensure_dot_prefix(&new_rel)),
            });
        }
    }

    edits
}

/// Expand a range to include the surrounding quote characters.
fn range_with_quotes(inner: Range) -> Range {
    Range {
        start: Position {
            line: inner.start.line,
            character: inner.start.character.saturating_sub(1),
        },
        end: Position {
            line: inner.end.line,
            character: inner.end.character + 1,
        },
    }
}

/// Ensure a relative path starts with `./` or `../` for Solidity import convention.
fn ensure_dot_prefix(rel: &Path) -> String {
    let s = rel.to_string_lossy();
    if s.starts_with("..") || s.starts_with('.') {
        s.to_string()
    } else {
        format!("./{s}")
    }
}

/// Apply a set of `TextEdit`s to a source string and return the new content.
///
/// Edits are sorted in reverse document order so that earlier byte offsets
/// remain valid as we splice in replacements from the end.
pub fn apply_text_edits(source: &str, edits: &[TextEdit]) -> String {
    let lines: Vec<&str> = source.split('\n').collect();

    // Convert LSP Position to byte offset in the source string.
    let pos_to_byte = |pos: Position| -> usize {
        let mut offset = 0;
        for (i, line) in lines.iter().enumerate() {
            if i == pos.line as usize {
                // LSP character offsets are UTF-16 code units.
                let mut utf16_units = 0u32;
                for (byte_idx, ch) in line.char_indices() {
                    if utf16_units >= pos.character {
                        return offset + byte_idx;
                    }
                    utf16_units += ch.len_utf16() as u32;
                }
                // Past end of line — clamp to line length.
                return offset + line.len();
            }
            offset += line.len() + 1; // +1 for the '\n'
        }
        source.len()
    };

    // Sort edits in reverse order so splicing from the end preserves offsets.
    let mut sorted: Vec<&TextEdit> = edits.iter().collect();
    sorted.sort_by(|a, b| {
        b.range
            .start
            .line
            .cmp(&a.range.start.line)
            .then(b.range.start.character.cmp(&a.range.start.character))
    });

    let mut result = source.to_string();
    for edit in sorted {
        let start = pos_to_byte(edit.range.start);
        let end = pos_to_byte(edit.range.end);
        result.replace_range(start..end, &edit.new_text);
    }

    result
}

/// Normalize a path by resolving `.` and `..` components without requiring
/// the file to exist on disk (unlike `std::fs::canonicalize`).
fn normalize_path(path: &Path) -> std::path::PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {} // skip `.`
            std::path::Component::ParentDir => {
                components.pop(); // resolve `..`
            }
            other => components.push(other),
        }
    }
    components.iter().collect()
}
