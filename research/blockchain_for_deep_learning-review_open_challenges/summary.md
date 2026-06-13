---
id: blockchain_for_deep_learning-review_open_challenges
title: "Blockchain for deep learning: review and open challenges"
authors:
  - Muhammad Shafay
  - Raja Wasim Ahmad
  - Khaled Salah
  - Ibrar Yaqoob
  - Raja Jayaraman
  - Mohammed Omar
year: 2023
venue: "Cluster Computing"
volume: 26
issue: 1
pages: "197-221"
doi: "10.1007/s10586-022-03582-7"
type: article
keywords:
  - Deep learning
  - AI
  - Machine learning
  - Federated learning
  - Blockchain
  - Ethereum
  - Smart contracts
  - Security
  - Transparency
---

## Overview

This paper is a literature review that examines the integration of blockchain technology with deep learning (DL), motivated by the limitations of centralized DL systems—namely single points of failure, lack of operational transparency, weak traceability, and unreliable data provenance. The authors construct a thematic taxonomy with seven classification parameters (blockchain type, DL models, DL-specific consensus protocols, application areas, services, data types, and deployment goals) and use it to systematically categorize the existing body of work on blockchain-assisted DL frameworks. They then provide a detailed comparative review of state-of-the-art frameworks across healthcare, Internet of Vehicles (IoV), cellular traffic management, and blockchain security domains, comparing them on blockchain type, consensus protocol, DL method, dataset, strengths, and weaknesses. Finally, the paper identifies seven open research challenges (scalability, data validity/secure sharing, storage/structural enhancement, throughput/latency, cryptocurrency-related consensus and regulation, interoperability/high-speed storage, and secure economic models) that must be addressed to build trustworthy, blockchain-backed DL systems. The paper's central claim is that no prior survey had specifically and thoroughly addressed the blockchain-DL integration space (as distinct from broader blockchain-AI or blockchain-ML surveys), and it positions itself as filling that gap.

## Background

The paper situates itself relative to two prior surveys it explicitly distinguishes itself from: Salah et al.'s "Blockchain for AI: review and open research challenges" (cited as covering blockchain-AI broadly) and Tanwar et al.'s work on machine learning adoption in blockchain-based smart applications (covering blockchain-ML). It draws on Sarpatwar et al.'s work on enabling trusted AI via blockchain for the concept of model/data provenance (tracking participants, datasets, models, operations, and compute pipelines as immutable blockchain transactions) and cites Shinde et al. on blockchain for securing AI applications. Foundational blockchain concepts are grounded in Nakamoto's Bitcoin whitepaper and Wood's Ethereum yellow paper, with consensus protocol comparisons drawing on works covering Proof-of-Work, Proof-of-Stake, and Proof-of-Authority. Federated learning is grounded in McMahan et al.'s foundational "Communication-efficient learning of deep networks from decentralized data." Decentralized storage solutions referenced include IPFS (Benet), Cassandra, Swarm, and Storj, which the paper notes are commonly paired with blockchain to mitigate on-chain storage costs. The paper also draws on domain-specific prior studies for its framework comparisons—e.g., a pharmacogenomics/ovarian-cancer prediction system using one-shot learning, a proof-of-information consensus for incremental federated learning in healthcare, Zheng et al.'s data-filtration work for wearable health data, Juneja et al.'s stacked denoising autoencoder for arrhythmia classification, and several IoV studies on cooperative positioning, crowdsensing, and crowdsourcing for traffic management. DL-specific consensus mechanisms it surveys (BlockML, Committee Consensus Mechanism, WekaCoin, Proof-of-Learning) are each drawn from distinct prior proposals cited in the literature it reviews.

## Key Points

- The paper proposes a seven-parameter taxonomy (blockchain type—public/private/consortium; DL model—CNN, RNN/LSTM/GRU, GAN, DRL, geometric DL; DL-specific consensus protocols; application areas; services; data types; deployment goals) to classify blockchain-DL literature, which it presents as a novel organizing framework for this specific intersection.
- It asserts that integrating blockchain with DL yields four categories of benefit: data security (via decentralized, tamper-resistant storage and private-key access control), automatic decision-making (via traceable, smart-contract-verified outputs), cumulative/collective judgments (e.g., voting-based decisions in swarm robotics), and enhanced robustness/trust in high-accuracy DL predictions.
- The paper claims that public blockchains are more vulnerable to inference attacks (since transactions and pseudonymous addresses are publicly visible) while private and consortium blockchains offer stronger privacy guarantees—an important consideration for selecting blockchain type in DL deployments.
- It identifies and summarizes four DL-specific consensus protocols (BlockML, Committee Consensus Mechanism, WekaCoin, Proof-of-Learning) as alternatives to energy-inefficient traditional protocols like PoW, each designed to reduce convergence time or computational overhead during model training/validation.
- Through its comparative table (Table 2), the paper provides a structured cross-study comparison of ten+ blockchain-DL frameworks spanning healthcare (ovarian cancer prediction, EHR prediction, arrhythmia classification, incremental learning for data exchange), IoV (GPS correction, miner node selection, communication security), and traffic/cellular management (traffic jam prediction, traffic flow prediction), documenting each study's blockchain type, consensus protocol, DL method, dataset, strengths, and limitations.
- The paper argues that blockchain-based model/data provenance (citing Sarpatwar et al.) is a distinct and valuable deployment goal separate from "AI decision sharing," enabling verification of model trustworthiness via immutable records of training data, model ownership, and processing pipeline history.
- It claims that the majority of reviewed blockchain-DL frameworks use blockchain primarily for secure data/model storage and sharing, while a smaller subset uses DL models to protect blockchain itself from attacks (e.g., double-spending detection via utility-function-based classifiers).
- The paper identifies seven distinct open research challenges: (1) platform scalability (need for lightweight, high-ratio compression algorithms), (2) data validity and secure sharing (proposing metadata binding and reputation-aware systems as partial solutions), (3) structural enhancement and storage capacity (using DL itself for blockchain data compression), (4) throughput and latency (noting Ethereum's ~16.5 TPS vs. Hyperledger Fabric's much higher throughput as a tradeoff for DL applications), (5) cryptocurrency price prediction gaps, DL-specific consensus design, and lack of regulatory standards, (6) high-speed computing/storage device integration and platform interoperability, and (7) secure, cost-efficient economic models for resource-intensive DL-blockchain systems.
- The paper offers concluding recommendations asserting that existing blockchain-based systems cannot yet efficiently handle data-quality problems (especially in healthcare and transportation), that key performance metrics (throughput, latency, block propagation time, smart contract vulnerabilities) significantly limit current systems' effectiveness, and that DL-based data compression is a viable mitigation for blockchain's ever-growing ledger size problem.

## Conclusion

The paper concludes that blockchain and deep learning are complementary technologies whose integration can substantially improve data security, privacy, traceability, and quality-of-service in applications such as healthcare, vehicular networks, cellular traffic management, and blockchain security itself. It supports this claim primarily through its taxonomy-driven survey and comparative analysis of existing frameworks, showing that diverse combinations of blockchain types, consensus protocols, and DL models have already been applied successfully to narrow problem domains (e.g., disease prediction, traffic jam forecasting, GPS error correction). However, the review does not present new experimental results or empirical validation of its own—its contribution is organizational/analytical (taxonomy and comparison) rather than a tested hypothesis. The paper explicitly notes significant limitations across the reviewed literature: many proposed systems were evaluated on small datasets or custom/unavailable datasets, several overlook cost and security analysis (e.g., the cooperative positioning study), and trade-offs are pervasive (e.g., GRU is less memory-intensive but has lower learning capacity than LSTM; one-shot learning requires less training data but underperforms DNNs). The paper frames its seven identified challenges—scalability, data validity, storage/structural growth, throughput/latency, cryptocurrency/consensus/regulatory gaps, interoperability, and secure economic models—as open research directions requiring further work, explicitly calling for lightweight compression algorithms, DL-specific consensus protocols based on training convergence and data quality proofs, standardized regulatory frameworks for blockchain-AI systems, and cost-efficient, resource-friendly architectures as priorities for future research.
