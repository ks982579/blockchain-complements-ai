---
name: summarize-paper-eco
description: Eco-friendly version of summarize-paper — summarize one paper in research/<dir>/ into a structured summary.md with citation-ready frontmatter, using only two subagents total. Use when the user asks to summarize a paper cheaply/quickly, process a research directory with minimal token usage, or continue the research-summarization pipeline in eco mode.
---

# Summarize a research paper (eco mode)

Process **one** directory under `research/` at a time and produce `research/<dir>/summary.md`.

This is a low-cost variant of `summarize-paper`. Instead of four section-drafting
subagents plus a reviewer/corrector consensus loop, this version uses **only two
subagents total**: `paper-summarizer-eco` (drafts all four sections in one pass) and
`summary-reviewer-eco` (fact-checks and corrects all four sections in one pass, no
loop). The main context acts as conductor: it picks the directory, extracts metadata
directly, spawns these two subagents sequentially (passing absolute paths via the
`prompt` field — subagent definitions are static and take no other input), and
assembles the final `summary.md`.

## Picking which directory to process

- If the user names a directory, use that one.
- If the user just says "summarize the next paper" (or similar), open
  `.claude-memory/001-init/PROGRESS.md`, find the first row in the per-paper checklist
  table where the `summary.md` column is `[ ]`, and use that directory.
- If the user says "summarize N papers" or "do the next batch", repeat this whole
  process N times sequentially (one summary.md per iteration), updating PROGRESS.md
  after each one so it stays resumable if interrupted.

## Step 1 — Extract metadata (main context, no subagents)

Read every non-`paper.md`, non-`summary.md` file in `research/<dir>/` (e.g. `*.bib`,
`*.ris`, `*.nbib`, `*.enw`, `*.txt`, other `*.md` files like `sources.md`). These are
small bibliographic files — read them directly and cross-reference against `paper.md`'s
title/author block (a quick peek at the first ~50 lines of `paper.md` is enough) to
build the frontmatter:

```yaml
id: <directory name, exactly as-is>
title: "<full paper title>"
authors:
  - <Author Name>
year: <publication year>
venue: "<journal/conference name>"
publisher: "<publisher, if known>"
volume: <if known>
issue: <if known>
pages: "<page range or article number, if known>"
doi: "<DOI, if known>"
url: "<canonical URL, if known>"
type: <article | inproceedings | misc/preprint, etc.>
keywords:
  - <keyword>
```

Omit any field you can't find — don't write `null` or `"unknown"`. Hold this
frontmatter for Step 4. Do not read the full `paper.md` in the main context.

## Step 2 — Draft the whole summary body

Spawn a single `paper-summarizer-eco` subagent. The `prompt` gives the absolute path
to `research/<dir>/paper.md` and the absolute output path `research/<dir>/summary.md`.
The agent reads `paper.md` once and writes all four sections (`## Overview`, `##
Background`, `## Key Points`, `## Conclusion`) directly to `summary.md` — no
frontmatter yet.

## Step 3 — Review and correct in one pass

Create the directory `research/<dir>/.draft/` if it doesn't exist (for the factcheck
report). Spawn a single `summary-reviewer-eco` subagent. The `prompt` gives the
absolute paths to `research/<dir>/paper.md`, `research/<dir>/summary.md`, and the
output path `research/<dir>/.draft/factcheck.md`.

The agent reads `paper.md` and the drafted summary, fixes any issues it finds directly
in `summary.md` in place, and writes a factcheck report (`PASS` or `CORRECTED` with an
itemized list) to `.draft/factcheck.md` as an audit trail. There is no consensus loop
— whatever this agent produces is final.

## Step 4 — Add frontmatter (main context)

Read `research/<dir>/summary.md` (now containing the reviewed/corrected body) and
prepend the Step 1 frontmatter, then write the file back:

```markdown
---
<frontmatter from Step 1>
---

<body from summary.md, unchanged>
```

Notes on quality:
- `id` MUST be the exact directory name (used as the unique key for ChromaDB).
- Do not include an `abstract` field or section — it's intentionally excluded.

Leave `research/<dir>/.draft/factcheck.md` in place as an audit trail — do not delete
it.

## After writing summary.md

Update `.claude-memory/001-init/PROGRESS.md`: change the `summary.md` cell for this
directory's row in the per-paper checklist from `[ ]` to `[x]`. If this completes all
48 rows, also check off the `003-summarization` step at the top of that file.
