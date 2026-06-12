# PDF -> Markdown Conversion Notes

Ran `uv run apps/pdf_to_md.py` across all 48 `research/` directories.

- Result: 48/48 converted successfully (0 failed). Each directory now has a `paper.md`
  alongside its `*.pdf` (PDFs kept in place).
- Several PDFs (notably `blockchain_for_ai-review_and_open_research`,
  `security_and_privacy_on_blockchain`, and a few others) are scanned/image-based and
  pymupdf4llm fell back to Tesseract OCR for those pages. Output quality spot-checked
  and is readable, but OCR'd pages may have minor character-recognition errors -
  worth a quick skim during summarization if a key figure/number looks odd.
- `blockchain_for_ai-review_and_open_research/`: had two different PDFs
  (`Blockchain_for_AI_Review_and_Open_Research_Challenges.pdf` 10.9MB and
  `Final__Blockchain_for_AI__Review.pdf` 820KB). The script picked the larger one
  (the published/full version) for `paper.md`. The smaller one is left in the
  directory unused - fine to ignore or delete later.
- `blockchain_for_deep_learning-review_open_challenges/`: had two byte-identical PDFs;
  no impact either way.
- Re-run with `--force` to regenerate `paper.md` for any directory if needed; without
  `--force` the script skips directories that already have `paper.md`.
