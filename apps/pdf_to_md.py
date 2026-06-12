#!/usr/bin/env python3
"""Convert PDFs in research/<topic>/ directories to paper.md using pymupdf4llm."""

import argparse
import sys
from pathlib import Path

import pymupdf4llm

REPO_ROOT = Path(__file__).resolve().parent.parent
RESEARCH_DIR = REPO_ROOT / "research"


def find_pdf(directory: Path) -> Path | None:
    pdfs = sorted(directory.glob("*.pdf"))
    if not pdfs:
        return None
    if len(pdfs) == 1:
        return pdfs[0]
    # Multiple PDFs: prefer the largest (usually the full/published version).
    chosen = max(pdfs, key=lambda p: p.stat().st_size)
    others = ", ".join(p.name for p in pdfs if p != chosen)
    print(f"  note: multiple PDFs found, using '{chosen.name}' (also present: {others})")
    return chosen


def convert_dir(directory: Path, force: bool) -> str:
    out_path = directory / "paper.md"
    if out_path.exists() and not force:
        return "skipped"

    pdf_path = find_pdf(directory)
    if pdf_path is None:
        print(f"  warning: no PDF found in {directory}")
        return "failed"

    try:
        md_text = pymupdf4llm.to_markdown(str(pdf_path))
    except Exception as exc:
        print(f"  error converting {pdf_path.name}: {exc}")
        return "failed"

    if not md_text.strip():
        print(f"  warning: empty markdown output for {pdf_path.name} (likely scanned/image-only PDF)")
        return "failed"

    out_path.write_text(md_text, encoding="utf-8")
    return "converted"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "directory",
        nargs="?",
        help="Process a single research/<topic> directory instead of all of them",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Re-convert even if paper.md already exists",
    )
    args = parser.parse_args()

    if args.directory:
        target = Path(args.directory).resolve()
        dirs = [target]
    else:
        dirs = sorted(d for d in RESEARCH_DIR.iterdir() if d.is_dir())

    counts = {"converted": 0, "skipped": 0, "failed": 0}
    for d in dirs:
        print(f"{d.relative_to(REPO_ROOT)}")
        result = convert_dir(d, args.force)
        counts[result] += 1
        print(f"  -> {result}")

    print("\nSummary:")
    for key, value in counts.items():
        print(f"  {key}: {value}")

    sys.exit(1 if counts["failed"] else 0)


if __name__ == "__main__":
    main()
