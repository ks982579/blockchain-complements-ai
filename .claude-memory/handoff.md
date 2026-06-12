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
- **Step 4 is done (2026-06-12)**: `apps/surreal_core/` (Cargo workspace: `vectordb` +
  `embedder` libs behind trait/builder abstractions, `cli` binary) and
  `apps/surreal_cli.py` wrapper both built and smoke-tested with passing unit +
  integration tests (16 total). Full details, gotchas (time crate pin, disk space,
  SurrealDB record-link/escaping/KNN-literal/UNION quirks), and architecture notes in
  `.claude-memory/004-vector-db/PROGRESS.md`.
- **Next: Step 5** — `.claude/skills/research-query/SKILL.md` per `001-init/PLAN.md`
  Step 5: takes a natural-language question, runs `apps/surreal_cli.py query` (and
  `related` for exploratory asks), synthesizes a cited answer.
- Note: only 6/48 papers have `summary.md` so far — the vector store currently only
  covers those 6. Running `apps/surreal_cli.py update` after more summaries are written
  will pick up new ones incrementally.
- Project conventions: use `uv` for all Python (no bare pip/venv), scripts live in
  `apps/`, not `scripts/`. Note: this dev environment doesn't actually have `uv`
  installed — `apps/surreal_cli.py` has no third-party deps so `python3` works
  identically; mention if `uv` is unavailable when invoking it.
