# 004-vector-db Progress

Atomic checklist for Step 4 (SurrealDB ingestion: Rust core + Python wrapper). Full
design rationale lives in `.claude-memory/001-init/PLAN.md` Step 4 — read that first if
anything below is ambiguous. Update this file as each item is completed; this is the
resumability mechanism for this step, same as the per-paper table in
`001-init/PROGRESS.md` is for Step 3.

Prereq note: this step does not require all 48 summaries to be done. It can be built and
tested against however many `summary.md` files exist today, then `update` re-run later
to pick up new ones.

## Architecture note (2026-06-12, during implementation)

Implemented as a Cargo **workspace** under `apps/surreal_core/`, not a single binary
crate, per user request — abstraction layers so backends (vector DB, embedding model)
can be swapped later with minimal changes to the CLI:

- `crates/vectordb/` — library. `VectorStore` trait + `VectorStoreBuilder`
  (factory/builder, `Backend::Surreal { path }` is the only variant today). The only
  module importing the `surrealdb` SDK is `src/surreal.rs` (`SurrealStore` impl).
- `crates/embedder/` — library. `Embedder` trait + `EmbedderBuilder` (`Backend::FastEmbed`
  is the only variant today). The only module importing `fastembed` is
  `src/fastembed_impl.rs` (`FastEmbedder` impl).
- `crates/cli/` — binary (`surreal_core`), imports only `vectordb` + `embedder`, never
  the underlying SDKs. Subcommand logic lives in `src/commands/{init,update,query,related}.rs`,
  plus `src/summary.rs` (frontmatter/chunk parsing), `src/relations.rs` (edge derivation),
  `src/output.rs` (JSON success/error convention).

Swapping backends later = add a new `Backend` variant + matching module in `vectordb` or
`embedder`; no changes needed in `crates/cli`.

## A. Rust crate scaffold (`apps/surreal_core/`)

- [x] Workspace scaffold (`Cargo.toml` workspace root + 3 member crates, see above)
- [x] `Cargo.toml`: deps added — `surrealdb` v2 (`kv-surrealkv`), `tokio`, `serde`,
      `serde_json`, `fastembed`, `clap` (derive), `serde_yaml_ng`, `async-trait`, `anyhow`
- [x] `.gitignore`: added `.surreal/` and `apps/surreal_core/target/`
- [x] `cargo build` succeeds

  **Gotcha**: `surrealdb-core 2.6.5` fails to compile against `time 0.3.48` (coherence
  conflict, E0119). Fixed by pinning `time` to `0.3.47` via `cargo update -p time
  --precise 0.3.47` — recorded in `Cargo.lock`. If `cargo update` is ever run and this
  resurfaces, re-pin the same way (or check for a newer `surrealdb-core` that fixes it
  upstream).

  **Disk space**: build host has very little free disk (was down to ~928MB at one
  point, crashing the linker with a Bus error on the test binary). Added `[profile.dev]
  debug = 0` to the workspace `Cargo.toml` to keep `target/` smaller. If linker crashes
  recur, `cargo clean` in `apps/surreal_core/` first (target/ was ~5.8GB before debug=0).

## B. Shared plumbing

- [x] DB connection helper — `SurrealStore::connect()` in `crates/vectordb/src/surreal.rs`,
      opens `Surreal::new::<SurrealKv>(path)`, selects ns/db `research`/`papers`
- [x] Frontmatter parser — `crates/cli/src/summary.rs::parse()`, YAML via `serde_yaml_ng`
- [x] Section chunker — `crates/cli/src/summary.rs::chunk_sections()`, splits on `##`
      headings (Background section identified by name match in `relations.rs`/`init.rs`,
      not a separate tag field)
- [x] `fastembed` wrapper — `crates/embedder/src/fastembed_impl.rs::FastEmbedder`,
      AllMiniLML6V2, 384-dim. Model auto-downloads to `~/.cache` on first run.
- [x] JSON output convention — `crates/cli/src/output.rs::{success, failure}`

## C. `init` subcommand

- [x] `DEFINE TABLE paper` + `DEFINE TABLE chunk` (SCHEMALESS) — `SurrealStore::init_schema`
- [x] `DEFINE INDEX ... HNSW` on `chunk.embedding`, dim 384, `DIST COSINE`
- [x] Scans `research/*/summary.md`, parses frontmatter, inserts `paper` records
- [x] Chunks + embeds each section, inserts `chunk` records linked via `paper` field
      (record link set via `SET paper = $rid`, not via `CONTENT` — see note below)
- [x] `same_topic` edges via keyword overlap (`relations.rs::compute_edges`)
- [x] `cites` edges via Background-section text mentioning another paper's `id` as a
      whole word
- [x] Smoke test: `surreal_core init` against the 6 existing summaries —
      `{"papers_ingested":6,"papers_with_edges":6}`, no errors

  **Gotcha**: inserting a `RecordId` by embedding it inside a `serde_json::json!({...})`
  blob passed to `CREATE chunk CONTENT $data` does NOT produce a real record link — it
  round-trips as an escaped string and `WHERE paper = $rid` then fails to match. Fixed
  by using `CREATE chunk SET paper = $rid, section = $section, ...` with `$rid` bound
  directly as a `RecordId` parameter.

  **Gotcha**: `RecordId::key().to_string()` escapes IDs containing `-`/`_` with
  backticks (e.g. `` `ai-driven_...` ``). Fixed via `record_id_to_string()` helper using
  `String::try_from(id.key().clone())` to get the raw string.

## D. `update` subcommand

- [x] Skips papers whose `id` is already in `existing_paper_ids()`
- [x] New papers: insert + recompute edges against the full on-disk corpus
- [x] Smoke test: ran `update` twice with no new summaries — both report
      `{"papers_inserted":0,"papers_skipped":6}`, no duplicates

## E. `query` subcommand

- [x] `query "<question>" [--top-k N]` (default 8), embeds via fastembed
- [x] KNN via `<|K,COSINE|>` — **note**: K must be a literal in the query string, not a
      bound parameter (SurrealQL parse error otherwise); `top_k` is interpolated via
      `format!` (safe — it's our own `usize` CLI arg, not user SQL)
- [x] Joins chunk -> parent `paper` metadata, returns JSON array
- [x] Smoke test: query "How can blockchain improve federated learning?" returned
      topically relevant chunks with full paper metadata

## F. `related` subcommand

- [x] `related <paper-id> [--depth N]` (default 1), BFS over `cites`/`same_topic` edges
- [x] JSON array output
- [x] Smoke test: `related ai_agents_meet_blockchain__multi-agents --depth 1` returned 3
      `same_topic` neighbors with correct unescaped IDs

  **Note**: SurrealQL doesn't support bare `UNION` between top-level `SELECT`
  statements (parse error) — implemented as two separate statements in one `.query()`
  chain, reading both result sets via `.take(0)` / `.take(1)`.

  **Design note**: a target paper reached via both `cites` and `same_topic` from the
  same source now appears as two separate entries (one per relation kind) — `seen` only
  gates traversal expansion, not result emission, so both relation kinds are visible.

## G. `apps/surreal_cli.py` (Python wrapper)

- [x] Staleness check: compares `target/release/surreal_core` mtime against
      `crates/*/src/**/*.rs`, `Cargo.toml`/`Cargo.lock` (root + per-crate)
- [x] If stale/missing: runs `cargo build --release --manifest-path
      apps/surreal_core/Cargo.toml`, streaming output live (stdout passed through)
- [x] Forwards subcommand + `--db-path`/`--research-dir` + subcommand args to the built
      binary via `subprocess.run`, parses JSON stdout, pretty-prints
- [x] Smoke test: first run triggered a 3m30s release build and returned
      `{"papers_ingested": 6, "papers_with_edges": 6}`; second run (no source changes)
      ran in ~0.4s with no rebuild; touching a source file triggered a fast incremental
      rebuild (~4s) before the next invocation

  **Note**: `uv` is not installed in this dev environment despite being the stated
  project convention — ran via plain `python3` (the script has no third-party deps, so
  both work identically). No code change needed either way.

## H. End-to-end verification

- [x] `init` then `query` returns correct source attribution (verified above)
- [x] `related <id>` returns sensible neighbors with keyword overlap (verified above)
- [x] Repeated invocations (`update` x2, `init` x2) don't hit SurrealKv file-lock errors

## Tests

Added automated tests (run via `cargo test` from `apps/surreal_core/`):
- `crates/vectordb/src/surreal.rs` — 5 unit tests against a `tempfile::tempdir()`-backed
  SurrealKv instance (upsert/query roundtrip, chunk replacement on re-upsert, ID
  escaping, relation set/replace, BFS traversal). Temp dirs auto-clean on drop.
- `crates/cli/src/summary.rs`, `crates/cli/src/relations.rs` — 9 unit tests for
  frontmatter parsing, section chunking, and edge-derivation logic.
- `crates/cli/tests/cli.rs` — 1 integration test that runs the built `surreal_core`
  binary end-to-end (init/query/related/update) against a fixture `research/` dir and
  temp DB, both under a `tempfile::TempDir`.

All 16 tests pass (`cargo test` from `apps/surreal_core/`). Re-running with `cargo
update` could re-break the `time` pin (see section A gotcha) or disk space (see same).

## Status

Sections A-H all complete as of 2026-06-12, with passing unit + integration tests
(section "Tests" above) and a working `apps/surreal_cli.py` wrapper. **Step 4 is done.**
Next: Step 5 — `.claude/skills/research-query/SKILL.md` (per `001-init/PLAN.md` Step 5).
