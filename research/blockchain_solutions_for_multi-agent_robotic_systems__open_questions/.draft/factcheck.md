# Factcheck report

Result: **CORRECTED**

- Overview: CORRECTED (1 issue)
- Background: PASS
- Key Points: CORRECTED→PASS
- Conclusion: CORRECTED (1 issue)

## Issue 1 — Overview, inline arXiv id removed

- Original: "...(arXiv:1903.11041, Afanasyev et al., Innopolis/ITMO/UNIBIT)..."
- The arXiv ID is genuine (from the RIS metadata) but does not appear in paper.md's prose; removed from the body and retained in frontmatter (doi/url).

## Issue 2 — Conclusion, mislocated synthesis removed

- Original final sentence: "It also flags persistent general blockchain limitations (scalability, latency, low throughput, validator performance) as constraints to address, suggesting sharding as one mitigation."
- The paper's Discussion/conclusion (Section IV) does not enumerate these as constraints or suggest sharding there. "Scalability, latency and low throughput" appears only in the Introduction; sharding only in Section III.C (validator-performance mitigation). The sentence wrongly attributed this to the conclusion. Deleted.

## Verified correct (no change)

Six-category classification (A bytecode logging, B time-limited voting, C action validation, D economic incentive, E task dispatching, F authentication), labels and ordering; six open research questions; cited works (RobotChain/RoboChain, CMU/Zlot-Stentz-Dias-Thayer market exploration, EV battery/Hua et al., Calvaresi et al. systematic review, Kapitonov UAV).
