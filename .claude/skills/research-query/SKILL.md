---
name: research-query
description: Answer a question about the user's research corpus using the SurrealDB vector store, with citations to the source papers. Use when the user asks a question about blockchain/AI research topics, wants to find what the literature says about something, or asks what else relates to a given paper.
---

# Research query

Answer the user's question using `apps/surreal_cli.py`, which queries the vector store
built from `research/<dir>/summary.md` files (see `.claude-memory/004-vector-db/PROGRESS.md`
for the underlying schema/CLI details). Always ground the answer in the returned chunks
and cite sources — never answer from general knowledge alone.

## Step 1 — Run the vector search

```
python3 apps/surreal_cli.py query "<user's question, rephrased as a search query if helpful>" --top-k 8
```

This returns a JSON array of chunks, each with:
- `paper`: frontmatter metadata (`id`, `title`, `authors`, `year`, `venue`, `doi`, `url`, ...)
- `section`: which summary section the chunk came from (`Overview`, `Background`, `Key Points`, `Conclusion`)
- `text`: the chunk text
- `distance`: cosine distance (lower = more similar)

If `--top-k 8` doesn't surface enough relevant material, you can re-run with a higher
`--top-k`, or rephrase the question and re-query — but don't loop more than 2-3 times.

## Step 2 — Optionally expand via the citation graph

If the question is exploratory (e.g. "what else relates to paper X", "what builds on
this", "what other papers cover Y"), or if the top results cluster around one or two
papers and the user seems to want broader coverage, also run:

```
python3 apps/surreal_cli.py related <paper-id> --depth 1
```

using the `id` of a paper from the query results. This returns graph-connected papers
(`relation`: `Cites` or `SameTopic`, plus `depth`) with the same frontmatter shape. Use
this to surface additional relevant papers, not to replace the vector search.

## Step 3 — Synthesize a cited answer

Write a direct answer to the user's question, grounded in the returned chunks:

- For each claim, cite the source paper inline, e.g. `(Author et al., Year)` or
  `(Title, Year)` — use whatever's available in `paper.authors`/`paper.title`/`paper.year`.
- At the end, list full source details for every paper cited: title, authors, year,
  venue, and `doi`/`url` if present.
- **Flag `Background`-section matches**: if the best/only matches for a claim come from
  a chunk with `"section": "Background"`, that's the *citing paper's* summary of related
  work, not its own original contribution. Say so explicitly (e.g. "Paper X's background
  section discusses Y, citing [original source if named in the text]; consider checking
  that original source directly") rather than attributing the claim to the citing paper
  as if it were their finding.
- If results from `related` were used, note which papers came from the citation graph
  vs. the direct vector search, especially if their relevance is more topical/adjacent
  than a direct hit.
- If the returned chunks don't actually address the question (high `distance`, off-topic
  text), say so plainly rather than forcing an answer — the corpus may not cover it yet
  (currently only a subset of the 48 papers have `summary.md` and are ingested).

## Notes

- `apps/surreal_cli.py` auto-builds the Rust binary on first use or after source
  changes — the first invocation in a session may take longer.
- The vector store only covers papers with a `summary.md` (see
  `.claude-memory/001-init/PROGRESS.md` for which ones). If the user asks about a paper
  that's not yet summarized, mention that it's not in the index yet.
