# Factcheck report

Result: **CORRECTED**

- Overview: PASS
- Background: PASS
- Key Points: CORRECTED (5 issues — hedged numbers made precise)
- Conclusion: CORRECTED (1 issue — RQ answer counts made precise)

The draft deliberately hedged several figures (the abstract only gave 2232→98→13); the full paper body supports exact numbers, which were restored:

1. Funnel: 2,232 collected (SpringerLink 1541, IEEE 382, ScienceDirect 135, arXiv 118, ACM 47, Wiley 9); 31 duplicates removed → 2,201; title/abstract → 98; eligibility → 15; 2 more excluded → 13.
2. Mechanism counts: zk-proofs 7/13 (4 of those using ZoKrates); PKI/X.509 3; web of trust 3; 4 other cryptographic schemes.
3. Temporal: PKI/X.509 2017–2021; zk-proofs 2021–2023 (de-hedged).
4. Platforms: Ethereum 9, Hyperledger Indy 2, Hyperledger Fabric 2, Bitcoin 2 (2017–2018), 1 unspecified.
5. On-chain vs off-chain: 8 on-chain, 5 off-chain; PoC: all but one (Damgård et al. purely theoretical); gas figures (Bruschi/Muth several million; Heiss ~600k; Gallersdörfer 500k–1.5M).
6. RQ answers: 8/13 on-chain verification (RQ1); zk-proofs 7 works, PKI and WoT 3 each (RQ2).

Verified correct (no change): three RQs and dual trust gap; Damgård et al. as the theoretical paper; first-of-its-kind review claim; GDPR-excludes-personal-data definition; all background citations.
