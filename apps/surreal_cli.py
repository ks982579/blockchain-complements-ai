#!/usr/bin/env python3
"""Thin wrapper around the `surreal_core` Rust binary (apps/surreal_core/).

Builds the release binary on first run (or whenever its source is newer than
the existing binary), then forwards the given subcommand and arguments to it,
parsing its JSON stdout. This is the only entrypoint users/skills should call
for init/update/query/related.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
CRATE_DIR = REPO_ROOT / "apps" / "surreal_core"
MANIFEST = CRATE_DIR / "Cargo.toml"
BINARY = CRATE_DIR / "target" / "release" / "surreal_core"

# Paths whose mtimes determine whether the binary is stale.
SOURCE_GLOBS = ["crates/*/src/**/*.rs", "Cargo.toml", "Cargo.lock", "crates/*/Cargo.toml"]


def source_files() -> list[Path]:
    files = []
    for pattern in SOURCE_GLOBS:
        files.extend(CRATE_DIR.glob(pattern))
    return files


def is_stale() -> bool:
    if not BINARY.exists():
        return True
    binary_mtime = BINARY.stat().st_mtime
    return any(f.stat().st_mtime > binary_mtime for f in source_files())


def build() -> None:
    print("surreal_cli: building surreal_core (release)...", file=sys.stderr)
    subprocess.run(
        ["cargo", "build", "--release", "--manifest-path", str(MANIFEST)],
        check=True,
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Query the research paper vector store / citation graph."
    )
    parser.add_argument(
        "--db-path",
        default=str(REPO_ROOT / ".surreal" / "research.db"),
        help="Path to the embedded SurrealKV database file",
    )
    parser.add_argument(
        "--research-dir",
        default=str(REPO_ROOT / "research"),
        help="Directory containing per-paper subdirectories with summary.md",
    )

    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("init", help="(Re)create schema and ingest all summaries")
    subparsers.add_parser("update", help="Ingest only summaries not already in the store")

    query_parser = subparsers.add_parser("query", help="Run a similarity search")
    query_parser.add_argument("question")
    query_parser.add_argument("--top-k", type=int, default=8)

    related_parser = subparsers.add_parser("related", help="Graph traversal from a paper")
    related_parser.add_argument("paper_id")
    related_parser.add_argument("--depth", type=int, default=1)

    args = parser.parse_args()

    if is_stale():
        build()

    cmd = [
        str(BINARY),
        "--db-path",
        args.db_path,
        "--research-dir",
        args.research_dir,
        args.command,
    ]
    if args.command == "query":
        cmd += [args.question, "--top-k", str(args.top_k)]
    elif args.command == "related":
        cmd += [args.paper_id, "--depth", str(args.depth)]

    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0:
        print(result.stderr.strip(), file=sys.stderr)
        return result.returncode

    data = json.loads(result.stdout)
    print(json.dumps(data, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
