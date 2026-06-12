pub mod init;
pub mod query;
pub mod related;
pub mod update;

use crate::summary::{self, ParsedSummary};
use anyhow::{Context, Result};
use std::path::Path;

/// Load and parse every `summary.md` found under `research_dir/*/summary.md`.
pub fn load_summaries(research_dir: &Path) -> Result<Vec<ParsedSummary>> {
    let mut summaries = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(research_dir)
        .with_context(|| format!("reading research dir {}", research_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();

    for dir in entries {
        if !dir.is_dir() {
            continue;
        }
        let summary_path = dir.join("summary.md");
        if !summary_path.exists() {
            continue;
        }
        let contents = std::fs::read_to_string(&summary_path)
            .with_context(|| format!("reading {}", summary_path.display()))?;
        let parsed = summary::parse(&contents)
            .with_context(|| format!("parsing {}", summary_path.display()))?;
        summaries.push(parsed);
    }

    Ok(summaries)
}
