# Factcheck report

Result: **PASS**

- Overview: PASS
- Background: PASS
- Key Points: PASS
- Conclusion: PASS

All experimental numbers verified against the source tables. Poisoning (Table 1): BlockAgents GSM8K 78.4→77.4/Δ1.0, MATH 34.6→32.0/Δ2.6, MMLU 64.3→63.7/Δ0.6; MAD deltas 16.2/6.4/18.2; Sampling-and-Voting deltas 6.7/5.7/4.9. Backdoor ASR (Table 2): BlockAgents 0.6/1.8/3.7; MAD 46.6/36.9/49.8; Sampling-and-Voting 71.2/82.5/92.3. Ablation (Table 3): 77.4 vs 75.8, 32.0 vs 31.4, 63.7 vs 60.1. N/3 adversary bound, |M|/2 vote threshold, R=3 plateau, config (N=10 with 5 workers/5 miners, 2 malicious for poisoning, 4/10 for backdoor, R=2, GPT-3.5-Turbo), and the three evaluation dimensions (factual inconsistency, redundancy, contextual causal relevance) all confirmed. No corrections needed.
