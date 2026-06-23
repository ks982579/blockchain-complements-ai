# Factcheck report

Result: **CORRECTED**

- Overview: CORRECTED (1 issue)
- Background: PASS
- Key Points: PASS
- Conclusion: PASS

## Critical claim — CONFIRMED TRUE

The summary's caveat is accurate: Section IV (Methodology) and Section V (Results) of paper.md describe ONLY a Wisconsin Diagnostic Breast Cancer supervised-classification experiment (GNB accuracy 0.9211 / F1 0.9371 vs depth-5 decision tree accuracy 0.9561 / F1 0.9640). There is no SE-MARL, fleet-control, or NYC TLC experiment anywhere in the experimental sections, so the SE-MARL reward/regret/scaling/ablation results asserted in the abstract and conclusion are not evidenced in the manuscript. Caveat retained in both Overview and Conclusion.

## Issue 1 — Overview, inline venue/year

- Original: "(Wasnik et al., IEEE ICAECT 2026)"
- paper.md contains no venue "ICAECT" or publication year "2026" in its prose (the only "June 2026" instances are IEEE Xplore download timestamps). Venue/year come from RIS metadata; removed from the body and retained in frontmatter.

## Verified correct (no change)

Method components (population-based outer loop, POET-style curriculum, on-policy distillation, local message passing, Pareto-efficient policy retention); NYC TLC 2014-2019 benchmark claim; Background citations (MADDPG, COMA, QMIX, PBT, NEAT, POET, Dec-POMDP NEXP-complete/Bernstein, VDN, CommNet, mean-field, PSRO, Baker, Shapley, Littman, Sutton, Silver, Kakade-Langford CPI, Schulman TRPO, Hu-Wellman Nash Q, Son QTRAN/IGM); Conclusion limitations.
