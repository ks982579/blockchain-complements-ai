//! JSON output convention shared by all subcommands.
//!
//! Success: a single JSON value on stdout. Failure: `{"error": "..."}` on
//! stderr plus a non-zero exit code.

use serde::Serialize;
use std::process::ExitCode;

/// Print `value` as JSON on stdout and return a success exit code.
pub fn success<T: Serialize>(value: &T) -> ExitCode {
    match serde_json::to_string(value) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(err) => failure(&err.into()),
    }
}

/// Print `{"error": "..."}` on stderr and return a failure exit code.
pub fn failure(err: &anyhow::Error) -> ExitCode {
    let payload = serde_json::json!({ "error": err.to_string() });
    eprintln!("{payload}");
    ExitCode::FAILURE
}
