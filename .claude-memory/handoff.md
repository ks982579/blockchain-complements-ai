# Handoff

Read this first, then `.claude-memory/001-init/PROGRESS.md` for the live checklist and
`.claude-memory/001-init/PLAN.md` for the full architecture/design rationale.

## What this project is

Building a pipeline so the user (writing a research paper, slow reader) can query their
~48 source papers (in `research/<topic>/`) via a vector DB and get answers with source
citations. Five-step lifecycle: PDF->Markdown, per-paper summaries, ChromaDB ingestion,
query CLI, query/answer skill.

## Current status (as of 2026-06-12)

- **Step 1 (done)**: `.claude-memory/001-init/` has `PLAN.md` and `PROGRESS.md`.
- **Step 2 (done)**: `apps/pdf_to_md.py` (uses `pymupdf4llm`, run via `uv run
  apps/pdf_to_md.py`) converted all 48 `research/*/` PDFs to `paper.md`. PDFs kept in
  place. Notes on OCR'd/duplicate PDFs in `.claude-memory/002-pdf-conversion/NOTES.md`.
- **Step 3 (in progress)**: `.claude/skills/summarize-paper/SKILL.md` is built and
  working well — runs the 4 drafter subagents in parallel, then a
  reviewer/corrector consensus loop (capped at 3 rounds), then assembles
  `summary.md`. No skill changes needed so far; it has produced clean PASS-on-first-try
  results each time.
  - **6 of 48 done**: `accounting_and_auditing_with_blockchain_tech_and_AI__review`,
    `ai_agents_meet_blockchain__multi-agents`, `AI_and_BC_integration_in_business__content_analysis`,
    `ai-driven_optimization_of_blockchain_scalability__privacy_protection`,
    `ai_model_cards-state_of_the_art__automated_use`, and `bias_in_data-driven_ai_systems`.
  - **42 remaining** — see `.claude-memory/001-init/PROGRESS.md` per-paper table for
    which directories still have `summary.md` = `[ ]`. Next unchecked row is #6
    `a_marketplace_for_trading_AI__IoT_data`.
- **Step 4 (not started)**: `apps/chroma_cli.py` with `init`/`update`/`query`
  subcommands, ChromaDB `PersistentClient` in `.chroma/`, default local embeddings.
  Chunk `summary.md` by `##` section, tag each chunk with `section` metadata
  (overview/background/key_points/conclusion) + frontmatter fields + `id`.
- **Step 5 (not started)**: `.claude/skills/research-query/SKILL.md` — takes a
  question, runs `chroma_cli.py query`, synthesizes a cited answer.

## How to continue

- If the user says "summarize the next paper" / "summarize N papers": invoke the
  `summarize-paper` skill, which picks the next `[ ]` row in `PROGRESS.md`, and updates
  `PROGRESS.md` after each one. This is working smoothly — keep using it as-is.
- Once all 48 summaries are done (or the user wants to start earlier — they said they'll
  begin Step 4 once summarization wraps up), build Step 4 (`apps/chroma_cli.py`) per the
  spec in `PLAN.md`, then Step 5.
- Project conventions: use `uv` for all Python (no bare pip/venv), scripts live in
  `apps/`, not `scripts/`.
