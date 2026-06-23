---
id: blockchain_intelligence-when_blockchain_meets_ai
title: "Blockchain Intelligence: When Blockchain Meets Artificial Intelligence"
authors:
  - Zibin Zheng
  - Hong-Ning Dai
  - Jiajing Wu
year: 2020
venue: "arXiv"
doi: "10.48550/arXiv.1912.06485"
url: "https://arxiv.org/abs/1912.06485"
type: misc
keywords:
  - Blockchain
  - Artificial Intelligence
  - Smart Contract
  - Machine Learning
---

## Overview

This short IEEE article by Zheng, Dai, and Wu coins the term "blockchain intelligence" to denote using artificial intelligence (machine learning, data mining, and data visualization) to improve blockchain systems themselves. It deliberately inverts the more common research framing: whereas most prior work on AI-blockchain convergence applies blockchain to fix AI's security and privacy weaknesses, this paper focuses on applying AI to solve blockchain's intrinsic problems. It identifies three core blockchain challenges — operational maintenance of decentralized/heterogeneous systems, quality assurance of smart contracts (vulnerable, immutable code), and detecting malicious behavior masked by pseudonymity — and maps AI-driven opportunities onto each. The paper grounds the concept with a case study built on the authors' XBlock-ETH framework, which extracts and processes 8,100,000 Ethereum blocks into six structured datasets, demonstrating gas-price prediction potential and machine-learning-based Ponzi-scheme detection via Ether Flow Graphs. It closes with future directions including real-time automated maintenance, collective intelligence for smart contracts, and integrating multiple ML approaches for transaction supervision.

## Background

The paper builds on a body of cited prior work to frame both blockchain limitations and AI opportunities. It says blockchain has evolved through two phases — blockchain 1.0 (digital currency, e.g., Bitcoin) and blockchain 2.0 (smart contracts) — citing a security survey by Li et al. [1], and traces the smart-contract concept to Nick Szabo [2]. For smart-contract vulnerabilities it cites known issues including reentrancy [1], the under-optimized/gas-costly "overcharging" problem [3], randomness controlling [4], and the DAO attack [5]. It draws on existing tooling that AI/program-analysis can extend: the Oyente symbolic-execution platform for bug detection (Luu et al. [9]) and GASPER (Chen et al. [3]) for identifying seven gas-costly bytecode patterns. For operational maintenance it cites Dinh et al.'s data-processing-view benchmarking of Ethereum, Parity, and Hyperledger Fabric [8] and Zheng et al.'s real-time blockchain performance monitoring framework [6]. For malicious-behavior detection it references cross-graph analysis of Ethereum (Chen et al. [12]), SVD-based market-manipulation detection on the leaked Mt. Gox dataset (Chen et al. [13]), and prior service discovery/composition work for web services (Rodriguez-Mier et al. [11]). It notes that contrasting research efforts [14], [15] mainly exploit blockchain for AI so as to overcome the emerging security and privacy vulnerabilities of AI, and cites the authors' own big-data-analytics survey [7] on AI driving big data analytics. Statista is cited for the figure that Bitcoin held nearly 242 GB of data by Q3 2019.

## Key Points

- The paper coins and defines "blockchain intelligence" as the integration of AI with blockchain specifically to enhance blockchain systems, in contrast to prior work that applies blockchain to secure AI.
- It identifies three principal challenges in incumbent blockchain systems: (1) operational maintenance complicated by decentralization, heterogeneity, and differing consensus/throughput bottlenecks across Bitcoin, Ethereum, and Hyperledger Fabric; (2) quality assurance of immutable, vulnerable smart contracts; and (3) malicious-behavior detection hindered by pseudonymity, encryption, and massive heterogeneous data.
- It argues AI enables intelligent operational maintenance through four levels of analytics (descriptive, diagnostic, predictive, prescriptive) to detect faults, forecast failures, identify performance bottlenecks, and simulate/optimize systems.
- It argues AI enables intelligent quality assurance of smart contracts via ML-based automated bug/vulnerability detection, gas-costly-pattern identification, and AI-driven automated semantic labeling and data-driven QoS evaluation to support automated contract composition (which smart contracts currently lack compared to web services).
- It argues AI/big-data analytics enables automated malicious-behavior detection — identifying money laundering, phishing, gambling, scams, Ponzi schemes, market manipulation, and criminal-gang collusion across multiple anonymous accounts via association analysis.
- It presents the XBlock-ETH framework, which extracts three raw Ethereum data types (Block, Receipt, Trace) and processes them into six datasets: Block-and-Transaction, Internal Ether Transaction, Contract Information, Contract Calls, ERC20 Token Transactions, and ERC721 Token Transactions.
- It empirically observes that Ethereum "gasPrice" gradually decreases over time with congestion-driven peaks, and that micro-level fluctuations follow a "tidal law," implying gas price is potentially predictable.
- It demonstrates ML-based Ponzi-scheme detection by comparing Ether Flow Graphs of a normal lottery contract versus the Rubixi Ponzi contract, finding Ponzi schemes have more participants and more random payment transactions, allowing successful classification after feature extraction.

## Conclusion

The paper concludes that integrating AI with blockchain (blockchain intelligence) can drive the beneficial development of blockchain systems, supporting its thesis through conceptual mapping of AI opportunities onto blockchain challenges and a feasibility-demonstrating case study rather than through comprehensive new experiments. The case study supports its claims qualitatively: gas-price analysis suggests predictability, and the Ether Flow Graph comparison shows distinguishable patterns enabling Ponzi-scam classification. As an introductory/vision article, its evidence is illustrative rather than exhaustive, and it implicitly acknowledges practical obstacles to blockchain data analytics — multi-week, 500+ GB full-node synchronization, heterogeneous/binary/encrypted on-chain data structures, and the lack of general-purpose extraction tools. It outlines three open research directions: (1) real-time, automated operational maintenance with multi-metric monitoring and automated crash recovery; (2) collective intelligence for smart contracts, motivating all decentralized participants to contribute knowledge for global decision-making rather than relying on single-peer pattern analysis; and (3) integrating multiple machine learning approaches plus dynamic transaction graphs to extract features across heterogeneous, pseudonymous data and uncover account associations for malicious-behavior recognition.
