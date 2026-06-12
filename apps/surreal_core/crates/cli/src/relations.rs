//! Derive `cites`/`same_topic` edges between papers from frontmatter
//! keywords and Background-section text.

use std::collections::HashSet;
use vectordb::{Paper, RelationKind};

/// Computes edges from `paper` to other papers in `corpus`, based on:
/// - `same_topic`: any shared keyword (case-insensitive).
/// - `cites`: another paper's `id` appears as a whole word in `background`.
pub fn compute_edges(
    paper: &Paper,
    background: Option<&str>,
    corpus: &[Paper],
) -> Vec<(String, RelationKind)> {
    let mut edges = Vec::new();

    let own_keywords: HashSet<String> = paper
        .keywords
        .iter()
        .map(|k| k.to_lowercase())
        .collect();

    for other in corpus {
        if other.id == paper.id {
            continue;
        }

        let shares_keyword = other
            .keywords
            .iter()
            .any(|k| own_keywords.contains(&k.to_lowercase()));
        if shares_keyword {
            edges.push((other.id.clone(), RelationKind::SameTopic));
        }

        if let Some(background) = background {
            if mentions_id(background, &other.id) {
                edges.push((other.id.clone(), RelationKind::Cites));
            }
        }
    }

    edges
}

/// Whether `text` contains `id` as a standalone token (word-boundary match).
fn mentions_id(text: &str, id: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .any(|tok| tok == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paper(id: &str, keywords: &[&str]) -> Paper {
        Paper {
            id: id.to_string(),
            title: format!("Title {id}"),
            authors: vec![],
            year: None,
            venue: None,
            doi: None,
            url: None,
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            extra: serde_json::json!({}),
        }
    }

    #[test]
    fn shared_keyword_creates_same_topic_edge() {
        let a = paper("paper-a", &["Blockchain", "AI"]);
        let b = paper("paper-b", &["blockchain"]);
        let c = paper("paper-c", &["unrelated"]);

        let edges = compute_edges(&a, None, &[a.clone(), b.clone(), c.clone()]);

        assert_eq!(edges, vec![("paper-b".to_string(), RelationKind::SameTopic)]);
    }

    #[test]
    fn background_mention_creates_cites_edge() {
        let a = paper("paper-a", &[]);
        let b = paper("paper-b", &[]);
        let c = paper("paper-c", &[]);

        let background = "This work builds on paper-b and ignores paper-bx.";
        let edges = compute_edges(&a, Some(background), &[a.clone(), b.clone(), c.clone()]);

        assert_eq!(edges, vec![("paper-b".to_string(), RelationKind::Cites)]);
    }

    #[test]
    fn self_is_never_an_edge_target() {
        let a = paper("paper-a", &["blockchain"]);
        let edges = compute_edges(&a, Some("paper-a"), &[a.clone()]);
        assert!(edges.is_empty());
    }

    #[test]
    fn same_paper_can_get_both_relation_kinds() {
        let a = paper("paper-a", &["blockchain"]);
        let b = paper("paper-b", &["blockchain"]);

        let background = "Builds on paper-b.";
        let edges = compute_edges(&a, Some(background), &[a.clone(), b.clone()]);

        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&("paper-b".to_string(), RelationKind::SameTopic)));
        assert!(edges.contains(&("paper-b".to_string(), RelationKind::Cites)));
    }
}
