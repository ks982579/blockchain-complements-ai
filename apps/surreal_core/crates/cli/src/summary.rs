//! Parsing for `research/<dir>/summary.md`: YAML frontmatter + `##`-section
//! chunking.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use vectordb::{Chunk, Paper};

/// Shape of the YAML frontmatter block in `summary.md`.
#[derive(Debug, Deserialize)]
struct Frontmatter {
    id: String,
    title: String,
    #[serde(default)]
    authors: Vec<String>,
    year: Option<i64>,
    venue: Option<String>,
    #[serde(default)]
    journal: Option<String>,
    #[serde(default)]
    publisher: Option<String>,
    #[serde(default)]
    volume: Option<serde_json::Value>,
    #[serde(default)]
    issue: Option<serde_json::Value>,
    #[serde(default)]
    pages: Option<serde_json::Value>,
    doi: Option<String>,
    url: Option<String>,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    keywords: Vec<String>,
}

/// A parsed `summary.md`: its [`Paper`] record and section [`Chunk`]s (with
/// embeddings still unset).
pub struct ParsedSummary {
    pub paper: Paper,
    /// `(section_name, text)` pairs, in document order.
    pub sections: Vec<(String, String)>,
}

/// Parse a `summary.md` file's contents into a [`ParsedSummary`].
pub fn parse(contents: &str) -> Result<ParsedSummary> {
    let (frontmatter_raw, body) = split_frontmatter(contents)
        .ok_or_else(|| anyhow!("summary.md missing YAML frontmatter delimiters (---)"))?;

    let fm: Frontmatter =
        serde_yaml_ng::from_str(frontmatter_raw).context("parsing summary.md frontmatter")?;

    let venue = fm.venue.or(fm.journal).or(fm.publisher);

    let extra = serde_json::json!({
        "volume": fm.volume,
        "issue": fm.issue,
        "pages": fm.pages,
        "type": fm.r#type,
    });

    let paper = Paper {
        id: fm.id,
        title: fm.title,
        authors: fm.authors,
        year: fm.year,
        venue,
        doi: fm.doi,
        url: fm.url,
        keywords: fm.keywords,
        extra,
    };

    let sections = chunk_sections(body);

    Ok(ParsedSummary { paper, sections })
}

/// Split `---\n<yaml>\n---\n<body>` into `(yaml, body)`.
fn split_frontmatter(contents: &str) -> Option<(&str, &str)> {
    let rest = contents.trim_start().strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let yaml = &rest[..end];
    let body = &rest[end + 4..];
    Some((yaml, body.trim_start_matches('\n')))
}

/// Split the markdown body into `(heading, text)` pairs at `##` headings.
/// Text under a heading runs until the next `##` heading (or EOF).
fn chunk_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_text = String::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(prev) = current_heading.take() {
                sections.push((prev, current_text.trim().to_string()));
                current_text.clear();
            }
            current_heading = Some(heading.trim().to_string());
        } else if current_heading.is_some() {
            current_text.push_str(line);
            current_text.push('\n');
        }
    }

    if let Some(prev) = current_heading {
        sections.push((prev, current_text.trim().to_string()));
    }

    sections
}

/// Build [`Chunk`]s for a parsed summary, given embeddings already computed
/// for each section (same order as `parsed.sections`).
pub fn build_chunks(parsed: &ParsedSummary, embeddings: Vec<Vec<f32>>) -> Result<Vec<Chunk>> {
    if embeddings.len() != parsed.sections.len() {
        return Err(anyhow!(
            "embedding count ({}) does not match section count ({})",
            embeddings.len(),
            parsed.sections.len()
        ));
    }

    Ok(parsed
        .sections
        .iter()
        .zip(embeddings)
        .map(|((section, text), embedding)| Chunk {
            paper_id: parsed.paper.id.clone(),
            section: section.clone(),
            text: text.clone(),
            embedding,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"---
id: example_paper
title: "Example Paper: A Study"
authors:
  - Jane Doe
  - John Smith
year: 2024
venue: "Example Journal"
volume: 12
issue: 3
pages: "100-110"
doi: "10.1234/example"
url: "https://example.com/paper"
type: article
keywords:
  - blockchain
  - ai
---

## Overview

This paper studies things.
It has multiple lines.

## Background

This paper builds on other_paper_id and prior work.

## Key Points

- Point one.
- Point two.

## Conclusion

Things were concluded.
"#;

    #[test]
    fn parses_frontmatter_and_sections() {
        let parsed = parse(SAMPLE).unwrap();

        assert_eq!(parsed.paper.id, "example_paper");
        assert_eq!(parsed.paper.title, "Example Paper: A Study");
        assert_eq!(parsed.paper.authors, vec!["Jane Doe", "John Smith"]);
        assert_eq!(parsed.paper.year, Some(2024));
        assert_eq!(parsed.paper.venue, Some("Example Journal".to_string()));
        assert_eq!(parsed.paper.doi, Some("10.1234/example".to_string()));
        assert_eq!(parsed.paper.keywords, vec!["blockchain", "ai"]);

        let section_names: Vec<&str> = parsed
            .sections
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        assert_eq!(
            section_names,
            vec!["Overview", "Background", "Key Points", "Conclusion"]
        );

        let overview = &parsed.sections[0].1;
        assert!(overview.contains("This paper studies things."));
        assert!(overview.contains("multiple lines"));

        let background = &parsed.sections[1].1;
        assert!(background.contains("other_paper_id"));
    }

    #[test]
    fn falls_back_to_journal_and_publisher_for_venue() {
        let no_venue = SAMPLE.replace(
            "venue: \"Example Journal\"",
            "journal: \"Journal Fallback\"",
        );
        let parsed = parse(&no_venue).unwrap();
        assert_eq!(parsed.paper.venue, Some("Journal Fallback".to_string()));
    }

    #[test]
    fn missing_frontmatter_delimiters_is_an_error() {
        let result = parse("## Overview\n\nNo frontmatter here.\n");
        assert!(result.is_err());
    }

    #[test]
    fn build_chunks_pairs_sections_with_embeddings() {
        let parsed = parse(SAMPLE).unwrap();
        let embeddings: Vec<Vec<f32>> = (0..parsed.sections.len())
            .map(|i| vec![i as f32])
            .collect();

        let chunks = build_chunks(&parsed, embeddings).unwrap();

        assert_eq!(chunks.len(), parsed.sections.len());
        assert_eq!(chunks[0].section, "Overview");
        assert_eq!(chunks[0].paper_id, "example_paper");
        assert_eq!(chunks[1].embedding, vec![1.0]);
    }

    #[test]
    fn build_chunks_errors_on_length_mismatch() {
        let parsed = parse(SAMPLE).unwrap();
        let result = build_chunks(&parsed, vec![vec![0.0]]);
        assert!(result.is_err());
    }
}
