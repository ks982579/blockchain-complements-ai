# Factcheck report

Result: **PASS**

- Overview: PASS
- Background: PASS
- Key Points: PASS
- Conclusion: PASS

All experimental numbers verified against source tables. Bidding (Table 1): welfare 1200/1250/1320, profit 12/12.5/13.2, collusion 15.3/10.8/3.4%, index 0.72/0.78/0.85; config 100 bidders / 200 rounds. Traffic (Table 2): wait 45.3→41.8 s, variance 104.1→76.7, stability 0.72→0.83; QMIX baseline; 5 intersections / 300 steps; recovery <15 vs >25 steps. Noise: ~1170 at σ=0.4 vs <1020, ~14% better at σ=0.3; behavioral correlation >0.90. Ablation (Table 3): no-detection 10.5%/1265/0.77; no-redistribution 8.3%/1248/0.74; no-logging 14.6%/1202/0.69. Blockchain profile: PBFT/4 validators on the throughput characterization runs, 15 tx/s & 2.3 s (bidding), 12 tx/s & 1.9 s (traffic), ~2.1 s/episode 1.8 GB, ~1.2 s/step 1.1 GB. Verification tools (Mythril, Oyente, Solidity SMTChecker, Coq) and baselines/framing (MADDPG, COMA, QMIX, centralized authority, commitment scheme; MASAC/CTDE) all correct.

Note (not an error): the "four validators" PBFT figure is tied to the throughput/latency characterization runs, not the headline bidding/traffic experiments (which the paper separately notes use five and three validator nodes); the summary does not misattribute these. No corrections needed.
