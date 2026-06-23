# Factcheck report

Result: **CORRECTED**

- Overview: CORRECTED (1 issue)
- Background: PASS
- Key Points: CORRECTED (1 issue)
- Conclusion: PASS

## Issue 1 — Overview, inline arXiv id removed

- Original: "...(Jo & Park, POSTECH; arXiv 2507.14928)..."
- The arXiv ID is genuine (from RIS metadata) but does not appear in paper.md's prose; removed from the body, retained in frontmatter.

## Issue 2 — Key Points, math error in GM threshold

- Original: "...until 7 Byzantine evaluators are added (violating the GM threshold f < ⌊15/2⌋ = 6)..."
- ⌊15/2⌋ = 7, not 6. The paper states the system tolerates up to 6 Byzantine evaluators and breaks at 7 (f < ⌊15/2⌋, i.e., f must stay below 7). Corrected the wording and removed the false "= 6" equation.

## Verified correct (no change)

W/E honest-majority bounds; three protocol phases; five scoring criteria (factual contradiction, factual fabrication, instruction inconsistency, context inconsistency, logical inconsistency) with 0–20 scoring (10=neutral, C=5); GM/Krum/Bulyan thresholds ⌊(n−1)/2⌋ / ⌊(n−2)/2⌋ / ⌊(n−3)/4⌋; accuracy 71/100 vs 64 (2/3-quorum, +7%) vs 50 (majority, +21%); ~221s constant latency; baselines BlockAgents [19] and Trusted MultiLLMN [20] (HotStuff-like WBFT); 8-honest / breaks-at-7 resilience setup.
