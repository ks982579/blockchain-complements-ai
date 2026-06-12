//! End-to-end test: runs the built `surreal_core` binary against a temporary
//! `research/` fixture and a temporary SurrealKV database, exercising
//! `init`, `update`, `query`, and `related`. Everything lives under a
//! `tempfile::TempDir` that is cleaned up automatically when it drops.

use std::path::Path;
use std::process::Command;

const PAPER_A: &str = r#"---
id: paper-a
title: "Paper A: Blockchain Foundations"
authors:
  - Alice Author
year: 2023
venue: "Journal of Examples"
doi: "10.1111/paper-a"
url: "https://example.com/paper-a"
type: article
keywords:
  - blockchain
  - consensus
---

## Overview

Paper A introduces foundational blockchain consensus concepts.

## Background

This is a foundational paper with no prior references.

## Key Points

- Consensus mechanisms are categorized into several families.

## Conclusion

The taxonomy is useful for later surveys.
"#;

const PAPER_B: &str = r#"---
id: paper-b
title: "Paper B: AI for Consensus Optimization"
authors:
  - Bob Author
year: 2024
venue: "Journal of Examples"
doi: "10.1111/paper-b"
url: "https://example.com/paper-b"
type: article
keywords:
  - blockchain
  - ai
  - consensus
---

## Overview

Paper B applies AI techniques to optimize blockchain consensus.

## Background

This paper builds directly on paper-a's consensus taxonomy.

## Key Points

- Reinforcement learning can select consensus parameters dynamically.

## Conclusion

AI-driven consensus tuning improves throughput.
"#;

/// Path to the built `surreal_core` binary, built by `cargo test` before
/// integration tests run.
fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_surreal_core")
}

fn write_fixture(research_dir: &Path) {
    for (dir_name, contents) in [("paper-a", PAPER_A), ("paper-b", PAPER_B)] {
        let dir = research_dir.join(dir_name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("summary.md"), contents).unwrap();
    }
}

fn run(
    db_path: &Path,
    research_dir: &Path,
    args: &[&str],
) -> serde_json::Value {
    let output = Command::new(binary_path())
        .arg("--db-path")
        .arg(db_path)
        .arg("--research-dir")
        .arg(research_dir)
        .args(args)
        .output()
        .expect("run surreal_core");

    assert!(
        output.status.success(),
        "surreal_core {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("parse JSON stdout")
}

#[test]
fn init_query_related_update_end_to_end() {
    let temp = tempfile::tempdir().unwrap();
    let research_dir = temp.path().join("research");
    let db_path = temp.path().join("research.db");
    write_fixture(&research_dir);

    // init: ingest both papers and compute edges.
    let init_result = run(&db_path, &research_dir, &["init"]);
    assert_eq!(init_result["papers_ingested"], 2);
    assert_eq!(init_result["papers_with_edges"], 2);

    // query: a question about AI-driven consensus should surface paper-b.
    let query_result = run(
        &db_path,
        &research_dir,
        &["query", "How does AI optimize blockchain consensus?", "--top-k", "3"],
    );
    let results = query_result.as_array().expect("query returns array");
    assert!(!results.is_empty());
    let ids: Vec<&str> = results
        .iter()
        .map(|r| r["paper"]["id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"paper-b"));

    // related: paper-b shares the "blockchain"/"consensus" keywords with
    // paper-a (same_topic) and its Background section names paper-a (cites).
    let related_result = run(&db_path, &research_dir, &["related", "paper-b", "--depth", "1"]);
    let related = related_result.as_array().expect("related returns array");
    assert!(!related.is_empty());
    assert!(related
        .iter()
        .any(|r| r["paper"]["id"] == "paper-a" && r["relation"] == "Cites"));
    assert!(related
        .iter()
        .any(|r| r["paper"]["id"] == "paper-a" && r["relation"] == "SameTopic"));

    // update: rerunning with no new summaries should insert nothing new.
    let update_result = run(&db_path, &research_dir, &["update"]);
    assert_eq!(update_result["papers_inserted"], 0);
    assert_eq!(update_result["papers_skipped"], 2);

    // adding a third paper and re-running update should pick it up.
    let dir_c = research_dir.join("paper-c");
    std::fs::create_dir_all(&dir_c).unwrap();
    std::fs::write(
        dir_c.join("summary.md"),
        PAPER_B.replace("paper-b", "paper-c").replace("Paper B", "Paper C"),
    )
    .unwrap();

    let update_result = run(&db_path, &research_dir, &["update"]);
    assert_eq!(update_result["papers_inserted"], 1);
    assert_eq!(update_result["papers_skipped"], 2);
}
