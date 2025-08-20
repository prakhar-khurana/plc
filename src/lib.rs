//! Library crate for plc_secure_checker.
//!
//! This file simply re‑exports the modules used by the binary. The
//! library layout mirrors the organisation of the checker: the AST,
//! parsers and rule implementations all live under this crate root.

pub mod ast;
pub mod parser;
pub mod rules;
use wasm_bindgen::prelude::*;

// This is the function that JavaScript will call
#[wasm_bindgen]
pub fn check_plc_code(source_code: &str, policy_json: &str, file_name: &str) -> String {
    // 1. Determine which parser to use from file_name extension
    let program = match parser::parse_file_from_str(source_code, file_name) {
        Ok(p) => p,
        Err(e) => {
            // Return a JSON error message if parsing fails
            return format!(r#"{{"error": "Parse Error: {}"}}"#, e);
        }
    };

    // 2. Load the policy from the JSON string
    let policy: rules::Policy = serde_json::from_str(policy_json).unwrap_or_default();

    // 3. Run the rules to get a Vec of results
    let results = rules::run_all_for_wasm(&program, &policy);

    // 4. Serialize the results Vec into a JSON string and return it
    serde_json::to_string(&results).unwrap_or_else(|e| {
        format!(r#"{{"error": "Result Serialization Error: {}"}}"#, e)
    })
}