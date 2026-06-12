# Research Paper Pipeline: PDF → Markdown → Summaries → ChromaDB → Q&A Agent

## Context

The user is writing a research paper and has 50 source articles in `research/<topic>/`,
each currently as a PDF (plus assorted reference metadata files: `.bib`, `.ris`, `.nbib`,
`.txt`, `.md`, `source.txt`, etc.). Reading 50 PDFs is too slow. The goal is a pipeline
that converts each PDF to markdown (cheap for an LLM to read), produces a structured
`summary.md` per paper (with citation-ready frontmatter), ingests those summaries into
ChromaDB, and provides a query CLI returning the top 5-10 relevant chunks with their
source paper for citation. Long-term memory (`.claude-memory/`) tracks decisions and
progress across sessions, since summarizing 50 papers will span multiple sessions.

This is a multi-step lifecycle. This plan covers the full architecture and lays out each
step as its own numbered memory project; this session executes through Step 2 (memory
setup + PDF→MD conversion script) and gets Step 3 (summarization skill) ready to run.
Steps 4-5 (ChromaDB + query CLI) are designed now but can be implemented once summaries
start existing.

## Step 1 — Long-term memory setup (`.claude-memory/`)

- Create `.claude-memory/001-init/PLAN.md` — a copy of this plan (the architecture record).
- Create `.claude-memory/001-init/PROGRESS.md` — the durable index future sessions read
  first. Tracks: which steps (002-005 below) exist/are done, and a checklist of all 50
  `research/` directories with columns for `paper.md` done / `summary.md` done. This is
  the resumability mechanism for Step 3.
- Note in PROGRESS.md: directories `research/ConvergenceOfAIandBlockchain` and
  `research/the_convergence_of_ai_and_blockchain__road_ahead` were checked — the former
  no longer exists (already resolved), only the latter remains. No further action needed.

## Step 2 — PDF → Markdown script (`.claude-memory/002-pdf-conversion/`)

- Set up a project-local virtualenv via `uv` (`uv venv`, `uv add ...`, creating a
  `pyproject.toml`/`uv.lock`) so deps don't pollute system Python. Install `pymupdf4llm`.
- `apps/pdf_to_md.py`:
  - Walks `research/*/`, finds the `*.pdf` in each directory.
  - Converts via `pymupdf4llm.to_markdown(pdf_path)`, writes `paper.md` alongside the PDF
    (PDF is kept, not removed).
  - Skips directories that already have `paper.md` unless `--force` is passed.
  - Supports an optional positional arg to process a single directory.
  - Prints a summary (converted / skipped / failed) at the end.
- Run it across all 50 `research/` directories now, fix any conversion failures (e.g.
  scanned/image-only PDFs may need a fallback note), and record results in
  `002-pdf-conversion/NOTES.md` (any PDFs that converted poorly).
- Update `001-init/PROGRESS.md` checklist with `paper.md` status per directory.

## Step 3 — Summarization skill (`.claude-memory/003-summarization/`)

- Create a Claude Code skill `.claude/skills/summarize-paper/SKILL.md` that processes
  **one** `research/<dir>/` at a time (resumable — invoked repeatedly across sessions):
  - Reads `paper.md`.
  - Reads all reference/metadata files in the directory (anything not `paper.md`,
    `summary.md`, or `*.pdf` — i.e. `.bib`, `.ris`, `.nbib`, `.enw`, `.txt`, other `.md`)
    to extract bibliographic metadata.
  - Writes `summary.md` with:
    - YAML frontmatter: `title`, `authors` (list), `year`, `venue`/`journal`/`publisher`,
      `volume`, `issue`, `pages`, `doi`, `url`, `type`, `keywords`, and `id` (the
      directory name — used as the unique key for ChromaDB updates). No abstract.
    - `## Overview` — short, dense paragraph: what the paper explains and what it adds.
    - `## Background` — what prior work/claims this paper draws on (kept for the
      researcher's own context; tagged separately for ChromaDB, see Step 4).
    - `## Key Points` — bullet list of the paper's claims (for easy citation).
    - `## Conclusion` — findings, whether claims held up, open questions raised.
  - After writing, marks the directory as done in `001-init/PROGRESS.md`.
- This session: build the skill and run it on 1-2 papers to validate the format, then
  let the user (or future sessions) continue running it across the remaining papers.

## Step 4 — SurrealDB ingestion: Rust core + Python wrapper (`.claude-memory/004-vector-db/`)

**Revised 2026-06-12**: switched from ChromaDB to SurrealDB, and from a pure-Python
implementation to a Rust binary (using the official `surrealdb` Rust crate) wrapped by a
thin Python CLI. Rationale for SurrealDB: it's a multi-model DB supporting both vector
search (HNSW/MTREE indexes + `<|K,DIST|>` KNN operator in SurrealQL) and native graph
relations (`RELATE` + traversal), so the same store holds paper/chunk nodes for semantic
search *and* a citation/topic graph between papers — queries can combine "find similar
chunks" with "what does this paper build on / what's in the same cluster." Rationale for
Rust: exercise the Rust SDK directly (no `surrealdb` Python SDK, no PyO3/maturin bridge);
Python's role is limited to building the Rust binary and shelling out to it.

### Architecture

- **`apps/surreal_core/`** — a Rust crate (binary), built with `cargo`:
  - `Cargo.toml` deps: `surrealdb` (v2, `features = ["kv-surrealkv"]`), `tokio`
    (`features = ["macros", "rt-multi-thread"]`), `serde`/`serde_json`, `fastembed`
    (wraps ONNX Runtime via `ort`, ships `AllMiniLML6V2` — local embedding model, no
    API key, auto-downloads weights on first run).
  - Connects via `Surreal::new::<SurrealKv>(".surreal/research.db")` (embedded,
    file-backed, no separate server process). Add `.surreal/` to `.gitignore`.
  - Subcommands (clap-based CLI), each subcommand: open DB connection -> do work ->
    drop connection cleanly before exit (embedded SurrealKv is single-process; a
    held-open handle from a prior invocation would lock out the next one — see
    "Connection lifetime" below):
    - `init`: (re)creates `paper`/`chunk` tables + `DEFINE INDEX ... HNSW` on
      `chunk.embedding` (`DIST COSINE`, dimension = 384 for AllMiniLM-L6-v2). Scans
      every `research/*/summary.md` (path passed as arg from Python), parses
      frontmatter, chunks by `##` section, embeds each chunk via `fastembed`, inserts
      `paper` + `chunk` records. Second pass: `RELATE` `same_topic` edges from
      frontmatter `keywords` overlap, and `cites` edges where a `## Background` section
      can be confidently matched to another paper's `id` in the corpus.
    - `update`: for each `summary.md`, skip if its `paper` record + all chunks already
      exist; otherwise (re)insert and recompute its edges.
    - `query "<question>" [--top-k N]`: embeds the question via `fastembed`, runs
      `<|K,DIST|>` KNN over `chunk.embedding`, returns top 5-10 (default 8) chunks with
      text + joined paper metadata (title, authors, year, doi, section, source dir) as
      JSON on stdout.
    - `related <paper-id> [--depth N]`: graph traversal over `cites`/`same_topic` edges
      (1-2 hops), JSON on stdout.
  - All subcommands print a single JSON object/array to stdout (success) or a JSON
    `{"error": "..."}` to stderr with non-zero exit (failure) — Python parses this, no
    human-readable text mixed into stdout.

- **`apps/surreal_cli.py`** — thin Python wrapper, the only thing the user/skills invoke
  (`uv run apps/surreal_cli.py <subcommand> ...`):
  - Checks whether `apps/surreal_core/target/release/surreal_core` exists and is newer
    than the crate's source files (`src/`, `Cargo.toml`/`Cargo.lock`); if stale or
    missing, runs `cargo build --release --manifest-path apps/surreal_core/Cargo.toml`
    (streaming cargo's output so build errors are visible) before proceeding. This means
    the user never runs `cargo` by hand — editing the Rust source and re-running the
    Python CLI rebuilds automatically.
  - Forwards the subcommand + args to the built binary via `subprocess.run`, parses the
    JSON result, and pretty-prints it (or formats it for the calling skill).

### Connection lifetime ("how connections are maintained across languages")

There is no persistent in-memory connection shared between Python and Rust — each
`surreal_cli.py` invocation is a fresh OS process that spawns a fresh `surreal_core`
process, which opens the embedded SurrealKv file, does one unit of work, and exits
(dropping the connection so the file lock is released). **Persistence lives entirely on
disk** (`.surreal/research.db`), not in a shared process or socket. This is the
appropriate model for embedded SurrealKv (single-process access only) and for a CLI
tool invoked occasionally by skills/users — no daemon needed. If a future need arises for
a long-lived connection (e.g. a server process handling many queries without
re-opening the file each time), that would mean switching `surreal_core` itself into a
small persistent server (`ws://` mode) — out of scope for now, noted here for later.

### Background chunks

Background-section chunks are tagged `section: background` in metadata so the agent
can distinguish "this paper's own claim" from "context this paper cites from elsewhere"
when forming an answer — but they remain searchable.

## Step 5 — Query/Answer agent (`.claude-memory/005-query-agent/`)

- Create a skill `.claude/skills/research-query/SKILL.md`: takes a natural-language
  question, runs `apps/surreal_cli.py query`, and synthesizes an answer that cites the
  source paper(s) (title, authors, year, doi/url) for each claim used — flagging if the
  best matches are `section: background` (meaning: supporting context, possibly look at
  the cited original source instead). If the user's question is more exploratory
  ("what else relates to paper X"), the skill can also call `surreal_cli.py related` to
  pull in graph-connected papers alongside the vector search results.

## Files/dirs to be created this session

- `.claude-memory/001-init/PLAN.md`, `.claude-memory/001-init/PROGRESS.md`
- `.venv/`, `requirements.txt`
- `apps/pdf_to_md.py` (run across all `research/` dirs)
- `.claude/skills/summarize-paper/SKILL.md` (built + smoke-tested on 1-2 papers)
- `.claude-memory/002-pdf-conversion/NOTES.md`, `.claude-memory/003-summarization/` (notes)

## Verification

- `apps/pdf_to_md.py`: spot-check 2-3 generated `paper.md` files for readable
  headings/body text vs. the source PDF.
- `summarize-paper` skill: run on 1-2 directories, confirm `summary.md` frontmatter has
  all expected fields populated correctly from the `.bib`/`.ris` files, and that section
  content is dense/useful.
- (Step 4/5, future session): `surreal_cli.py init` (confirm it triggers a `cargo build
  --release` on first run, then reuses the binary on subsequent runs unless source
  changes) then `query` with a sample question, confirm results return correct source
  attribution; `surreal_cli.py related <id>` returns sensible neighbors for at least one
  paper with keyword overlap. Confirm repeated invocations don't hit SurrealKv file-lock
  errors (each process cleanly drops its connection before exit).
