use solidity_language_server::file_operations;
use std::fs;
use tower_lsp::lsp_types::Url;

// =============================================================================
// Live test with ForgeRunner (example/ directory)
// =============================================================================

#[tokio::test]
async fn test_rename_a_to_aa_produces_edit_on_b() {
    let example_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("example");
    let a_path = example_dir.join("A.sol");
    let b_path = example_dir.join("B.sol");
    assert!(a_path.exists(), "example/A.sol must exist");
    assert!(b_path.exists(), "example/B.sol must exist");

    let old_uri = Url::from_file_path(&a_path).unwrap();
    let new_path = example_dir.join("AA.sol");
    let new_uri = Url::from_file_path(&new_path).unwrap();

    // Discover source files — all .sol files in example/
    let source_files: Vec<String> = fs::read_dir(&example_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sol"))
        .filter_map(|e| e.path().to_str().map(String::from))
        .collect();

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };

    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    let b_uri = Url::from_file_path(&b_path).unwrap();
    assert!(
        edits.contains_key(&b_uri),
        "edits should contain B.sol — it imports A.sol"
    );

    let b_edits = &edits[&b_uri];
    assert_eq!(b_edits.len(), 1, "B.sol should have exactly 1 import edit");

    let edit = &b_edits[0];
    assert_eq!(
        edit.new_text, "\"./AA.sol\"",
        "newText should be quoted ./AA.sol"
    );
}

/// Test using solc_project_index to discover source files.
#[tokio::test]
async fn test_rename_a_to_aa_via_solc_project_index() {
    use solidity_language_server::config;
    use solidity_language_server::goto::CachedBuild;

    let example_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("example");
    let a_path = example_dir.join("A.sol");
    let b_path = example_dir.join("B.sol");
    assert!(a_path.exists(), "example/A.sol must exist");
    assert!(b_path.exists(), "example/B.sol must exist");

    let foundry_cfg = config::load_foundry_config(&example_dir);

    // Use solc_project_index to get the build, then extract source files.
    let ast_data = solidity_language_server::solc::solc_project_index(&foundry_cfg, None, None)
        .await
        .expect("solc_project_index should succeed for example project");

    let build = CachedBuild::new(ast_data, 0);
    let source_files: Vec<String> = build.path_to_abs.keys().cloned().collect();

    let old_uri = Url::from_file_path(&a_path).unwrap();
    let new_path = example_dir.join("AA.sol");
    let new_uri = Url::from_file_path(&new_path).unwrap();

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };

    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    let b_uri = Url::from_file_path(&b_path).unwrap();
    assert!(
        edits.contains_key(&b_uri),
        "edits should contain B.sol — it imports A.sol"
    );

    let b_edits = &edits[&b_uri];
    assert_eq!(b_edits.len(), 1);
    assert_eq!(b_edits[0].new_text, "\"./AA.sol\"");
}

// =============================================================================
// Tests with synthetic .sol sources (tree-sitter parseable)
// =============================================================================

/// Create a temporary directory with .sol files for testing.
struct TempProject {
    dir: tempfile::TempDir,
}

impl TempProject {
    fn new() -> Self {
        Self {
            dir: tempfile::TempDir::new().unwrap(),
        }
    }

    fn write_file(&self, name: &str, content: &str) -> String {
        let path = self.dir.path().join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, content).unwrap();
        path.to_str().unwrap().to_string()
    }

    fn path(&self, name: &str) -> std::path::PathBuf {
        self.dir.path().join(name)
    }

    fn uri(&self, name: &str) -> Url {
        Url::from_file_path(self.path(name)).unwrap()
    }
}

#[test]
fn test_rename_simple_import() {
    let proj = TempProject::new();

    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract A {}\n",
    );
    let b_path = proj.write_file(
        "B.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {A} from \"./A.sol\";\ncontract B is A {}\n",
    );

    let source_files = vec![a_path, b_path];

    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("AA.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    let b_uri = proj.uri("B.sol");
    assert!(edits.contains_key(&b_uri), "B.sol should have edits");
    assert_eq!(edits[&b_uri].len(), 1);
    assert_eq!(edits[&b_uri][0].new_text, "\"./AA.sol\"");
}

#[test]
fn test_rename_nobody_imports_returns_empty() {
    let proj = TempProject::new();

    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract A {}\n",
    );
    let b_path = proj.write_file(
        "B.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract B {}\n",
    );

    let source_files = vec![a_path, b_path];

    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("AA.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert!(edits.is_empty(), "no edits when nobody imports the file");
}

#[test]
fn test_rename_nonexistent_file_returns_empty() {
    let old_uri = Url::from_file_path("/tmp/nonexistent/Foo.sol").unwrap();
    let new_uri = Url::from_file_path("/tmp/nonexistent/Bar.sol").unwrap();

    let source_files: Vec<String> = vec![];
    let provider = |_: &str| -> Option<Vec<u8>> { None };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert!(edits.is_empty());
}

#[test]
fn test_rename_multiple_importers() {
    let proj = TempProject::new();

    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract A {}\n",
    );
    let b_path = proj.write_file(
        "B.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {A} from \"./A.sol\";\ncontract B is A {}\n",
    );
    let c_path = proj.write_file(
        "C.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport \"./A.sol\";\ncontract C {}\n",
    );

    let source_files = vec![a_path, b_path, c_path];

    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("AA.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert_eq!(edits.len(), 2, "both B.sol and C.sol should have edits");

    for (_uri, file_edits) in &edits {
        for te in file_edits {
            assert_eq!(te.new_text, "\"./AA.sol\"");
        }
    }
}

#[test]
fn test_rename_does_not_affect_unrelated_imports() {
    let proj = TempProject::new();

    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract A {}\n",
    );
    let b_path = proj.write_file(
        "B.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract B {}\n",
    );
    let c_path = proj.write_file(
        "C.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {B} from \"./B.sol\";\ncontract C is B {}\n",
    );

    let source_files = vec![a_path, b_path, c_path];

    // Rename A.sol — C.sol imports B.sol, not A.sol, so no edits.
    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("AA.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert!(edits.is_empty(), "C.sol imports B.sol not A.sol");
}

#[test]
fn test_move_file_updates_own_imports() {
    let proj = TempProject::new();

    let lib_path = proj.write_file(
        "lib/Math.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nlibrary Math {}\n",
    );
    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {Math} from \"./lib/Math.sol\";\ncontract A {}\n",
    );

    let source_files = vec![lib_path, a_path];

    // Move A.sol into sub/ directory.
    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("sub/A.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert!(
        edits.contains_key(&old_uri),
        "A.sol should have edits (own imports updated after move)"
    );

    let a_edits = &edits[&old_uri];
    assert_eq!(a_edits.len(), 1);
    // From sub/A.sol, lib/Math.sol is at ../lib/Math.sol
    assert_eq!(a_edits[0].new_text, "\"../lib/Math.sol\"");
}

#[test]
fn test_same_dir_rename_does_not_rewrite_own_imports() {
    let proj = TempProject::new();

    let lib_path = proj.write_file(
        "lib/Math.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nlibrary Math {}\n",
    );
    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {Math} from \"./lib/Math.sol\";\ncontract A {}\n",
    );

    let source_files = vec![lib_path, a_path];

    // Rename in same dir — own imports shouldn't change.
    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("AA.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    assert!(
        !edits.contains_key(&old_uri),
        "same-dir rename should not edit the file's own imports"
    );
}

#[test]
fn test_rename_cross_directory_import() {
    let proj = TempProject::new();

    let src_path = proj.write_file(
        "src/Token.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract Token {}\n",
    );
    let test_path = proj.write_file(
        "test/Token.t.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport {Token} from \"../src/Token.sol\";\ncontract TokenTest {}\n",
    );

    let source_files = vec![src_path, test_path];

    let old_uri = proj.uri("src/Token.sol");
    let new_uri = proj.uri("src/TokenV2.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    let test_uri = proj.uri("test/Token.t.sol");
    assert!(edits.contains_key(&test_uri));
    assert_eq!(edits[&test_uri][0].new_text, "\"../src/TokenV2.sol\"");
}

#[test]
fn test_skips_non_relative_imports() {
    let proj = TempProject::new();

    // A file with a remapped import (not starting with ./ or ../)
    let a_path = proj.write_file(
        "A.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\nimport \"forge-std/Test.sol\";\nimport {B} from \"./B.sol\";\ncontract A {}\n",
    );
    let b_path = proj.write_file(
        "B.sol",
        "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract B {}\n",
    );

    let source_files = vec![a_path, b_path];

    // Move A.sol to sub/ — only the relative import ./B.sol should be updated,
    // NOT the remapped import forge-std/Test.sol
    let old_uri = proj.uri("A.sol");
    let new_uri = proj.uri("sub/A.sol");

    let provider = |fs_path: &str| -> Option<Vec<u8>> { std::fs::read(fs_path).ok() };
    let edits = file_operations::rename_imports(&source_files, &old_uri, &new_uri, &provider);

    if let Some(a_edits) = edits.get(&old_uri) {
        // Should only have 1 edit (for ./B.sol), not for forge-std/Test.sol
        assert_eq!(
            a_edits.len(),
            1,
            "only relative imports should be rewritten"
        );
        assert!(a_edits[0].new_text.contains("B.sol"));
    }
}
