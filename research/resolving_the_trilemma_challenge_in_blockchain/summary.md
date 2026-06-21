---
id: resolving_the_trilemma_challenge_in_blockchain
title: "Resolving the Trilemma Challenge in Blockchain: An Integrated Consensus Mechanism for Balancing Security, Scalability, and Decentralization"
authors:
  - Khandakar Md Shafin
  - Saha Reno
year: 2025
venue: "Automatic Control and Computer Sciences"
publisher: "Allerton Press, Inc."
volume: 59
issue: 3
pages: "297-316"
doi: "10.3103/S0146411625700476"
url: "https://doi.org/10.3103/S0146411625700476"
type: article
keywords:
  - blockchain
  - decentralization
  - consensus
  - security
  - scalability
---

## Overview

This paper by Shafin and Reno (2025) presents a hybrid blockchain consensus mechanism designed to simultaneously address all three pillars of the blockchain trilemma: security, scalability, and decentralization. The framework integrates multiple cryptographic primitives -- zk-SNARKs for zero-knowledge proofs, Schnorr Verifiable Random Functions (VRF) for leader selection, and Elliptic Curve Cryptography (ECC) for key generation -- alongside adaptive stake management, sharding-based network partitioning, Layer-2 rollup scaling, and an incentive alignment model with rewards and penalties. Evaluated on a 50-node testbed, the proposed system achieves over 1600 TPS with low latency, demonstrates resilience against double-spending at adversary stakes up to 45%, and scores 7.181 on a composite decentralization metric (rated "Solid"), placing it competitively among established blockchains like Algorand and Dash though below Monero, Bitcoin, Cardano, and Ethereum. The work is published in Automatic Control and Computer Sciences (Vol. 59, No. 3, 2025).

## Background

The paper frames its contribution around the blockchain trilemma concept, citing Teoh (2022) on the inherent trade-off between decentralization, security, and scalability. It builds on Nakamoto's Bitcoin (2008) and Buterin's Ethereum (2014) as foundational systems that excel in decentralization and programmability but suffer from throughput limitations due to PoW and universal smart contract execution, respectively. Eyal et al.'s Bitcoin-NG (2016) is cited as a scalability improvement through leader-based two-tier architecture, though the authors note its centralization risks. The paper references Castro and Liskov's PBFT (1999) and Sousa et al.'s BFT ordering for Hyperledger Fabric (2018) for fault tolerance techniques, while acknowledging their communication overhead at scale. Liu et al.'s hardware-assisted Byzantine consensus (2018) is noted for its scalability gains but criticized for hardware dependency limiting broader applicability. Gilad et al.'s Algorand (2017) is cited for VRF-based cryptographic sortition and its honest-majority assumption. Schnorr VRF implementation draws on Chaum and Pedersen (1992) and the FROST threshold signature scheme (Komlo and Goldberg, 2021). Sharding concepts reference Liu et al. (2022), cross-chain interoperability draws on Hei et al.'s AgentChain (2022), and Layer-2 solutions reference Gangwal et al.'s survey (2023). The paper also cites Reno and Haque (2023) and Al-Kafi et al. (2024) as prior work from the same research group on off-chain storage and hybrid blockchain frameworks.

## Key Points

- The consensus mechanism uses epoch-based organization where node stakes are dynamically recalibrated each epoch based on previous stake, stake movements (weighted for incoming vs. outgoing), and network contribution metrics (transaction handling, uptime), producing a fair and adaptive stake allocation.
- Slot leader selection employs Schnorr VRF over the Ed25519 elliptic curve, generating cryptographically verifiable random values per node per time slot; the node with the highest VRF output is appointed leader, and all nodes can verify the selection using the corresponding public key.
- Proportional (stake-weighted) voting replaces one-node-one-vote, requiring a supermajority threshold for consensus, which makes malicious network takeover economically prohibitive and defends against stake grinding attacks by restricting recalibration to epoch boundaries.
- The system incorporates sharding for state and network partitioning, distributing nodes across shards based on geographic location or performance indicators, with load-balancing algorithms routing transactions to appropriate shards for parallel processing.
- Cross-chain interoperability is supported through sidechain frameworks and bridge protocols, with a formalized interoperability metric combining bridge reliability, security, complexity, and sidechain flexibility/scalability.
- Layer-2 scaling uses rollups (both zero-knowledge and optimistic variants) to aggregate transactions off-chain, generate cryptographic proofs, and submit batched transactions to the main chain, reducing on-chain data burden and transaction costs.
- The incentive alignment model combines staking deposits, performance-based rewards (computed from stake, transaction fees, and uptime metrics), and penalties for malicious behavior or neglect, with governance parameters adjustable through DAOs.
- Fault tolerance is enhanced through redundancy erasure codes for data reliability across distributed storage and anomaly detection algorithms for identifying malicious behavior.
- Experimental evaluation on a 50-machine cluster (Intel i5-11400, 64 GB RAM each, NetEm-emulated network conditions) shows the system achieves 1115 TPS at 1000 transactions and exceeds 1600 TPS at higher loads, outperforming PoW, PoS, DPoS, PBFT, and PoA.
- Latency measurements show the proposed system at 47 ms for 500 transactions, compared to PoW's 3211 ms, PoS's 449 ms, PBFT's 61 ms, and PoA's 56 ms.
- Against double-spending attacks with 45% adversarial stake, the system confirms approximately 21,000 transactions in 23 minutes (compared to PoW's 141 minutes), achieving 6.13 to 8.86 times speedup over PoW across adversary stake levels of 10-45%.
- Forkability analysis using binomial distribution shows that fork probability decreases significantly with increasing block counts (n = 700 to 2800), demonstrating strong resistance to adversarial forking.
- The composite decentralization score of 7.181 ("Solid") is derived from parameters including governance, geographical node distribution, and attack resilience, placing the system above Chainlink, Stellar, Ripple, and VeChain but below Monero (8.092), Bitcoin, Cardano, and Ethereum (7.634).
- Validator hardware costs are kept under $1000, lowering the barrier to participation and supporting broader decentralization.

## Conclusion

The paper concludes that its hybrid consensus mechanism successfully balances all three dimensions of the blockchain trilemma. The system's throughput (1600+ TPS) and low latency demonstrate strong scalability, its resilience to double-spending and fork attacks at high adversarial stakes (up to 45%) validates its security claims, and its decentralization score of 7.181 confirms competitive decentralization. The authors acknowledge that the decentralization score, while "Solid," falls below the "Excellent" tier occupied by systems like Monero and Bitcoin, indicating room for improvement. The paper does not present a formal theoretical proof of security guarantees or formally verify the protocol against known attack taxonomies beyond double-spending and forkability. The experimental setup is limited to 50 nodes, leaving open questions about performance at larger network scales. The paper does not discuss real-world deployment considerations such as network heterogeneity, validator incentive dynamics over long time horizons, or potential vulnerabilities in the cross-chain interoperability layer. No explicit future research directions are stated, though the gap between the system's decentralization score and top-ranked blockchains, along with the need for larger-scale validation, represent clear avenues for further work.
