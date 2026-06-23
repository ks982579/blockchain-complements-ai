# Factcheck report

Result: **PASS**

- Overview: PASS
- Background: PASS
- Key Points: PASS
- Conclusion: PASS

Verified against paper.md: setup (50 rounds, 20 agents, 100 tasks, GPT-4, Solidity/EVM, Hardhat+Ganache, ALFRED-derived tasks); task-level results (success 86.77% [80.15→94.49], quality 0.75→0.89 avg 0.82, capability match ~0.765 max 0.794, utility 5.02 [3.52→6.43], bid rate 92%→55%, specialization 4 agents/20% Tag_2 and 3 agents/15% Tag_7); blockchain metrics (67.32 tx/round, 147,865 gas/tx, 2.18s confirmation, 0.86 failed/round, 106.82 events/round, $0.5–$1/call); utility model U=π·R−c with β (workload) and γ (L1 capability mismatch); reputation EMA (λ) with S combining q and d (α+δ=1); capability EMA (μ); softmax weights 0.5/0.3/0.2; three contrasted systems (DeCoAgent, BlockAgents, LLM-Net). Conclusion "tension" claim verified correct: reputation mean 0.488 (Std 0.044, range 0.429–0.558, none >0.8 or <0.3) and entropy flat 2.31→2.315 bits — the paper itself frames this as a shortfall ("limited capability concentration"), so the summary's reading is faithful. No inline arXiv id. No corrections needed.
