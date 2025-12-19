use solidity_language_server::completion::member_access;
use solidity_language_server::symbols::extract_symbols;
use std::process::Command;
use tower_lsp::lsp_types::Position;

fn get_test_ast_data() -> Option<serde_json::Value> {
    println!("Running forge build in example directory");
    let output = Command::new("forge")
        .args(["build", "--json", "--no-cache", "--ast"])
        .current_dir("example")
        .output();

    match output {
        Ok(out) => {
            println!("Forge command succeeded, stdout len: {}", out.stdout.len());
            if !out.status.success() {
                println!("Forge command failed with status: {:?}", out.status);
                println!("Stderr: {}", String::from_utf8_lossy(&out.stderr));
                return None;
            }
            let stdout_str = String::from_utf8(out.stdout).ok()?;
            println!("Parsing JSON...");
            let json = serde_json::from_str(&stdout_str);
            match json {
                Ok(j) => Some(j),
                Err(e) => {
                    println!("JSON parse error: {:?}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Failed to run forge command: {:?}", e);
            None
        }
    }
}

#[test]
fn test_struct_member_completion_for_order() {
    let ast_data = match get_test_ast_data() {
        Some(data) => {
            println!("AST data obtained successfully");
            data
        }
        None => {
            println!("Failed to get AST data");
            return;
        }
    };

    let _symbols = extract_symbols(&ast_data);
    println!("Extracted {} symbols", _symbols.len());

    // Test dot completion for "order."
    let text = "order.";
    let position = Position {
        line: 67,
        character: 10,
    }; // After "order."

    let result = member_access::get_dot_completions(text, &ast_data, position);

    if let Some((ref comps, ref query)) = result {
        println!(
            "Got {} completions for query '{}': {:?}",
            comps.len(),
            query,
            comps.iter().map(|c| &c.label).collect::<Vec<_>>()
        );
    } else {
        println!("No completions returned");
    }

    assert!(
        result.is_some(),
        "Should return completions for 'order.'"
    );

    let (completions, _query) = result.unwrap();
    let completion_labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();

    assert!(
        completion_labels.contains(&"buyer"),
        "Completions should include buyer"
    );
    assert!(
        completion_labels.contains(&"nonce"),
        "Completions should include nonce"
    );
    assert!(
        completion_labels.contains(&"amount"),
        "Completions should include amount"
    );
    assert!(
        completion_labels.contains(&"date"),
        "Completions should include date"
    );
}
