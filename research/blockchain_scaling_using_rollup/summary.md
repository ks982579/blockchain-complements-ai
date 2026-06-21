---
id: blockchain_scaling_using_rollup
title: "Blockchain Scaling Using Rollups: A Comprehensive Survey"
authors:
  - Louis Tremblay Thibault
  - Tom Sarry
  - Abdelhakim Senhaji Hafid
year: 2022
venue: "IEEE Access"
publisher: "IEEE"
volume: 10
pages: "93039-93054"
doi: "10.1109/ACCESS.2022.3200051"
type: article
keywords:
  - blockchain
  - scalability
  - rollups
  - second layer solutions
  - survey
---

## Overview

This paper presents the first comprehensive academic survey of rollup technology as a Layer 2 blockchain scaling solution, addressing the fundamental throughput bottleneck of popular blockchains such as Bitcoin (up to 7 TPS) and Ethereum (around 15 TPS). The authors classify rollups into two types -- optimistic rollups and zero-knowledge (ZK) rollups -- and provide detailed technical explanations of each, including the cryptographic underpinnings of ZK-SNARKs, ZK-SNORKs, and ZK-STARKs. Six major implementations are examined in depth: Optimism and Arbitrum (optimistic rollups), and StarkEx/StarkNet, zkSync, Polygon Hermez, and Loopring (ZK rollups). The paper contributes a comparative performance analysis across these implementations covering throughput, fee reduction, dispute resolution mechanisms, EVM compatibility, and adoption metrics (Total Value Locked), while also situating rollups among other Layer 2 approaches such as state channels, sidechains, Plasma, and validiums.

## Background

The paper builds on the blockchain trilemma framework (decentralization, scalability, security) that underpins much of the scaling discourse, citing foundational work by Croman et al. on scaling decentralized blockchains and Hafid et al.'s comprehensive Layer 1 scaling survey. It draws on Buterin's Ethereum whitepaper and Yellow Paper (Wood) as the technical baseline for the systems being scaled. For Layer 2 context, it references the Bitcoin Lightning Network (Poon and Dryja), Plasma (Poon and Buterin), Liquid sidechain (Nick et al.), and state channel literature (Negka and Spathoulas). The ZK proof discussion relies on Ben-Sasson et al.'s foundational ZK-STARK paper, Chen et al.'s review of ZK-SNARKs, Groth's work on pairing-based arguments, and Gabizon et al.'s PLONK protocol for universal ZK-SNORKs. The paper cites Buterin's "Incomplete Guide to Rollups" as evidence that ZK rollups are expected to dominate long-term. It also references the Huobi Research Rollup Report (Jiang) regarding the lack of decentralization in current rollup implementations, and Sguanci et al.'s Layer 2 scaling survey for data compression techniques. The vulnerability of elliptic-curve-based proofs to quantum computing attacks is noted via Proos and Zalka's work on Shor's algorithm for elliptic curves.

## Key Points

- Rollups reduce Layer 1 load by executing transactions off-chain on Layer 2 while publishing compressed transaction data on-chain, ensuring data availability and enabling deterministic re-execution of all transactions.
- Optimistic rollups operate under an "innocent-until-proven-guilty" model where aggregators post transaction batches without validity proofs; security relies on at least one honest verifier (watcher) being active to trigger dispute resolution during a challenge window.
- ZK rollups submit cryptographic validity proofs (ZK-SNARKs, ZK-SNORKs, or ZK-STARKs) alongside transaction data, enabling immediate finality without a dispute period and eliminating the need for active verifiers.
- Optimism's dispute resolution re-executes the entire disputed transaction on Layer 1 via a deployed duplicate smart contract, yielding simplicity but placing potentially heavy computation burden on Layer 1 (O(log(n)) complexity relative to smart contract data size).
- Arbitrum's dispute resolution uses a K-way Dissection (bisection) protocol to narrow disagreements down to a single AVM instruction, requiring only O(1) on-chain computation for the one-step proof, at the cost of a multi-round interactive protocol.
- Data compression techniques reduce on-chain rollup data substantially: nonces can be omitted entirely, gas price/limit reduced to 1 byte, recipient addresses compressed to 4-byte indices, and transaction amounts stored in scientific notation.
- ZK-SNARKs require an application-specific trusted setup but produce small proofs with fast verification; ZK-SNORKs (universal SNARKs, e.g., PLONK) require only a single universal trusted setup reusable across applications; ZK-STARKs require no trusted setup but generate significantly larger proofs.
- ZK-SNARKs and ZK-SNORKs rely on elliptic curve cryptography (ECDLP) and are therefore vulnerable to quantum computing attacks, whereas ZK-STARKs rely only on collision-resistant hash functions and are considered post-quantum secure.
- Theoretical throughput across implementations ranges from 2,000 TPS (Optimism, zkSync, Polygon Hermez, Loopring) to 4,500 TPS (Arbitrum), with validium modes reaching 18,000-20,000+ TPS at the cost of off-chain data availability.
- Adoption remains low: only about 2.7% of Ethereum's locked funds reside in rollups (as of June 2022), with Arbitrum holding over 56% of Layer 2 TVL; three-quarters of all rollup TVL is in optimistic rollups.
- Nearly all rollup implementations operate centralized sequencers despite decentralization being a stated core value; only Polygon Hermez operated in a fully decentralized manner at the time of writing, using a Proof of Donation auction mechanism.
- Optimistic rollups impose a 7-day withdrawal period for dispute resolution, creating a usability barrier that fast-exit liquidity pools (e.g., Boba Network) can partially mitigate but not eliminate.
- EVM equivalence is harder to achieve for ZK rollups than for optimistic rollups, leading most ZK rollup implementations to be application-specific (DEXs, payment systems) rather than general-purpose; StarkNet and zkSync aim at general computation but require new programming languages (Cairo, Zinc).
- Starkware's Volition concept proposes a hybrid model allowing users to choose per-transaction between on-chain (ZK rollup) and off-chain (validium) data availability based on their risk tolerance and throughput needs.

## Conclusion

The paper surveys all first and second layer blockchain scaling solutions, with an in-depth focus on rollups and their mode of operation. Optimistic rollups currently lead in adoption and general-purpose computation support, while ZK rollups are expected to gain more popularity in the long term thanks to faster finality (minutes vs. 7 days for withdrawals) and privacy advantages. However, actual observed throughput at the time of writing was far below theoretical maximums -- neither Arbitrum nor Optimism exceeded 1 TPS on average daily -- reflecting early-stage adoption rather than technical limitations. The authors identify three critical areas for future research: (1) decentralizing sequencer/aggregator roles, since centralized sequencers pose censorship risks; (2) developing alternatives to the fixed 7-day withdrawal period in optimistic rollups to improve usability; and (3) achieving inter-rollup operability, since EVM-equivalent and EVM-compatible rollups are not directly interoperable, and transferring funds between different rollup systems remains cumbersome despite early bridging protocols like Connext and Hop Exchange. The paper also highlights the open question of quantum vulnerability for SNARK/SNORK-based systems and the trade-off between proof complexity and trust assumptions across ZK proof technologies.
