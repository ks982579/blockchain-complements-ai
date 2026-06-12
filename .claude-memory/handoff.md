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
- **Step 4 (not started, revised 2026-06-12 twice)**: switched from ChromaDB to
  SurrealDB, and from pure Python to a Rust core + Python wrapper — see
  `.claude-memory/001-init/PLAN.md` Step 4 for full rationale/design.
  - `apps/surreal_core/` — Rust crate (Cargo, `surrealdb` crate w/ `kv-surrealkv`
    feature, `fastembed` for local AllMiniLM-L6-v2 embeddings, `clap` CLI) with
    `init`/`update`/`query`/`related` subcommands, embedded via
    `Surreal::new::<SurrealKv>(".surreal/research.db")`. JSON on stdout.
  - `apps/surreal_cli.py` — thin Python wrapper; auto-builds the Rust binary via `cargo
    build --release` when stale, then shells out and parses JSON. This is the only
    entrypoint users/skills call.
  - Schema: `paper` table (frontmatter per directory) + `chunk` table (per `##`
    section, vector HNSW index, `DIST COSINE`, dim 384) + graph edges
    (`cites`/`same_topic` via `RELATE`) for the `related` traversal command.
  - Connection model: no shared in-memory/socket connection between Python and Rust —
    each invocation is a fresh process pair, persistence is purely the on-disk
    `.surreal/research.db` file. Each subcommand opens, works, and cleanly drops the
    connection before exit (SurrealKv embedded mode is single-process).
- **Step 5 (not started)**: `.claude/skills/research-query/SKILL.md` — takes a
  question, runs `surreal_cli.py query` (and optionally `related` for exploratory asks),
  synthesizes a cited answer.

## How to continue

- If the user says "summarize the next paper" / "summarize N papers": invoke the
  `summarize-paper` skill, which picks the next `[ ]` row in `PROGRESS.md`, and updates
  `PROGRESS.md` after each one. This is working smoothly — keep using it as-is.
- **2026-06-12**: user decided to work on Step 4 now (got bored with summaries; 6/48
  done, can resume anytime). Step 4 has its own atomic checklist:
  `.claude-memory/004-vector-db/PROGRESS.md` (sections A-H, mirrors PLAN.md Step 4
  design). Work through it top to bottom, checking off items as completed — same
  resumability pattern as the per-paper summary table. Does not require all 48
  summaries; build/test against whatever `summary.md` files exist (currently 6), `init`
  is rerunnable via `update`.
- After Step 4, build Step 5 (`research-query` skill) per `PLAN.md`.
- Project conventions: use `uv` for all Python (no bare pip/venv), scripts live in
  `apps/`, not `scripts/`.
