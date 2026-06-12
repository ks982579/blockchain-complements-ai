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

## Step 4 — ChromaDB ingestion (`.claude-memory/004-vector-db/`)

- Add `chromadb` to `requirements.txt`. Use a `PersistentClient` stored in `.chroma/`
  (add to a `.gitignore` if one exists) with the default local embedding function (no
  API key needed).
- `apps/chroma_cli.py` with subcommands:
  - `init`: (re)creates the `papers` collection. Scans every `research/*/summary.md`,
    parses YAML frontmatter (via `python-frontmatter`), splits the body into chunks by
    `##` section (Overview / Background / Key Points / Conclusion). Each chunk is
    embedded with metadata = frontmatter fields (minus abstract, which doesn't exist
    anyway) + `section` + `id` (directory name).
  - `update`: for each `summary.md`, check whether its `id` + section combination already
    exists in the collection; add only missing ones (so re-running after adding new
    summaries only ingests the new papers).
  - `query "<question>" [--top-k N]`: embeds the question, returns top 5-10 (default 8)
    matching chunks with their text + metadata (title, authors, year, doi, section,
    source dir) for citation.
- Background-section chunks are tagged `section: background` in metadata so the agent
  can distinguish "this paper's own claim" from "context this paper cites from elsewhere"
  when forming an answer — but they remain searchable.

## Step 5 — Query/Answer agent (`.claude-memory/005-query-agent/`)

- Create a skill `.claude/skills/research-query/SKILL.md`: takes a natural-language
  question, runs `apps/chroma_cli.py query`, and synthesizes an answer that cites the
  source paper(s) (title, authors, year, doi/url) for each claim used — flagging if the
  best matches are `section: background` (meaning: supporting context, possibly look at
  the cited original source instead).

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
- (Step 4/5, future session): `chroma_cli.py init` then `query` with a sample question,
  confirm results return correct source attribution.
