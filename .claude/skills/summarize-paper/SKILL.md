---
name: summarize-paper
description: Summarize one paper in research/<dir>/ into a structured summary.md with citation-ready frontmatter, for later ingestion into ChromaDB. Use when the user asks to summarize a paper, process a research directory, or continue the research-summarization pipeline.
---

# Summarize a research paper

Process **one** directory under `research/` at a time and produce `research/<dir>/summary.md`.

To keep the main context window free of full-paper text, drafting and fact-checking
are delegated to dedicated subagents defined in `.claude/agents/`:
`overview-drafter`, `background-drafter`, `keypoints-drafter`, `conclusion-drafter`,
`paper-summary-reviewer`, and `paper-summary-corrector`. The main context acts as
conductor: it picks the directory, extracts metadata directly, spawns these subagents
(passing absolute paths via the `prompt` field — subagent definitions are static and
take no other input), and assembles the final `summary.md`.

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

## Step 2 — Draft each section in parallel

Create the directory `research/<dir>/.draft/` if it doesn't exist (the subagents will
write into it). Spawn four subagent calls **in a single batch** (one message, four
Agent tool calls), one per section:

| subagent_type | output file |
|---|---|
| `overview-drafter` | `research/<dir>/.draft/overview.md` |
| `background-drafter` | `research/<dir>/.draft/background.md` |
| `keypoints-drafter` | `research/<dir>/.draft/keypoints.md` |
| `conclusion-drafter` | `research/<dir>/.draft/conclusion.md` |

For each, the `prompt` should give the absolute path to `research/<dir>/paper.md` and
the absolute output path from the table above — the agent definitions already know
what to do with them.

## Step 3 — Review and correct, with a consensus loop

Spawn a single `paper-summary-reviewer` subagent. The `prompt` gives the absolute path
to `research/<dir>/paper.md`, plus, for each of the four sections, the absolute paths
to `.draft/<section>.md` and `.draft/<section>.factcheck.md`. The agent reads
`paper.md` once and writes/overwrites all four factcheck files (`PASS`, or `FAIL` with
an itemized list).

For any section(s) that come back `FAIL`:

1. Spawn a single `paper-summary-corrector` subagent covering all `FAIL` sections in
   one pass. The `prompt` gives `paper.md` plus, for each `FAIL` section, its draft
   and factcheck paths. For issues it agrees with, it rewrites the draft in place. For
   issues it disputes, it leaves the draft unchanged and appends a "## Corrector
   response" to that section's factcheck file with its reasoning, grounded in
   `paper.md` quotes — the factcheck file's status stays `FAIL`.
2. Spawn the `paper-summary-reviewer` subagent again, scoped only to the still-`FAIL`
   sections (`paper.md` plus those sections' draft and factcheck paths). It reads any
   "## Corrector response" entries and either concedes (updates the factcheck file to
   `PASS`) or restates the issue with a direct rebuttal (stays `FAIL`).
3. Repeat for sections still `FAIL`.

Sections already marked `PASS` are skipped entirely in later rounds — once the
reviewer signs off on a section, it is not revisited.

Cap the whole loop at 3 rounds total. If any section is still `FAIL` after 3 rounds
(including unresolved disputes), stop and report the issue to the user instead of
looping forever.

## Step 4 — Assemble summary.md (main context)

Once all four sections show `PASS`, read the four `.draft/<section>.md` files (small —
safe for the main context) and combine them with the Step 1 frontmatter into
`research/<dir>/summary.md`:

```markdown
---
<frontmatter from Step 1>
---

## Overview

<from .draft/overview.md>

## Background

<from .draft/background.md>

## Key Points

<from .draft/keypoints.md>

## Conclusion

<from .draft/conclusion.md>
```

Notes on quality:
- `id` MUST be the exact directory name (used as the unique key for ChromaDB).
- Do not include an `abstract` field or section — it's intentionally excluded.

Leave `research/<dir>/.draft/` (drafts + factcheck reports) in place as an audit trail
— do not delete it.

## After writing summary.md

Update `.claude-memory/001-init/PROGRESS.md`: change the `summary.md` cell for this
directory's row in the per-paper checklist from `[ ]` to `[x]`. If this completes all
48 rows, also check off the `003-summarization` step at the top of that file.
