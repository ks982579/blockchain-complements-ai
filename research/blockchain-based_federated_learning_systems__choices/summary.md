---
id: blockchain-based_federated_learning_systems__choices
title: "Blockchain-Based Federated Learning System: A Survey on Design Choices"
authors:
  - Yustus Eko Oktian
  - Sang-Gon Lee
year: 2023
venue: "Sensors"
volume: 23
issue: 12
pages: "5658"
doi: "10.3390/s23125658"
url: "https://www.mdpi.com/1424-8220/23/12/5658"
type: article
keywords:
  - blockchain
  - smart contract
  - federated learning
  - design pattern
---

## Overview

This paper is a literature survey (Sensors, 2023) of 30 research papers on blockchain-based federated learning (FL) systems, focused specifically on Ethereum-based implementations. It addresses the core problem that vanilla FL — designed for trusted, single-organization environments — fails when extended to untrusted, multi-party (cross-silo or cross-device) settings, where leakage, poisoning, malicious servers, low trainer motivation, and dropout become serious threats. The authors extract and catalog about 31 distinct design-pattern variations used across the five FL workflow stages (registration, distribution, training, evaluation, aggregation), then score each design's pros and cons on four metrics — robustness, efficiency, privacy, and fairness — using a +/- significance scale. The paper further classifies the 30 surveyed papers by which design choices and implementation characteristics (FL algorithm, library, dataset, node count, evaluation type) they adopt, revealing adoption trends and gaps. Its main contribution to the field is both a taxonomy of blockchain-FL design choices with empirically-grounded tradeoff analysis, and a roadmap of underexplored areas (model compression, asynchronous aggregation, scalability evaluation, cross-device deployments) for future research.

## Background

The paper situates itself within FL research originating from McMahan et al.'s Federated Averaging (FedAvg) algorithm (citing [1]), and frames the privacy motivation partly around GDPR (citing Albrecht [2]) and Google's applied FL deployments (citing Yang et al. [3]). It draws on Carlini et al.'s membership-inference attack work (citing [4]) as the canonical demonstration that trained models can leak private training data even without direct data access — a recurring justification for differential privacy (citing Abadi et al. [14]) and homomorphic encryption (citing Zhou & Wornell's Integer-Vector HE [54], and Moore et al.'s HE survey [53] for efficiency limitations) as countermeasures. Blockchain foundations are drawn from Nakamoto's Bitcoin (citing [5]) and Wood's Ethereum yellow paper (citing [6]), with Hyperledger Fabric (citing [20]) mentioned as an alternative decentralization substrate. The paper positions itself relative to other blockchain-FL surveys (citing Nguyen et al. [7] and Ali et al. [8]) and general FL surveys (citing Aledhari et al. [9]), arguing none of these provide a detailed design-pattern taxonomy with pros/cons analysis. The four evaluation metrics (robustness, efficiency, privacy, fairness) are adopted from Kairouz et al.'s "Advances and open problems in federated learning" (citing [12]). Cross-silo vs. cross-device FL framing comes from Huang et al. (citing [10]) and Karimireddy et al. (citing [11]). Specific cited system designs that anchor the taxonomy include BlockFLA (citing Desai et al. [36]) for trojan detection/reporting and model compression, CREAT (citing Cui et al. [44]) for gradient-clustering compression, BlockFlow and SecCL (citing Mugunthan et al. [39], Zhang et al. [48]) for contribution-score-based model selection, Toyoda and Zhang's voting/lottery mechanisms (citing [35]), and Chain FL's Online FedAvg for asynchronous aggregation (citing Korkmaz et al. [51]). Future-direction discussion references layer-2 blockchain scaling via rollups (citing Thibault et al. [77]) and Alief et al.'s layer-2 FL implementation (citing [78]), as well as Zhao et al.'s hybrid cross-silo/cross-device split-model approach with Gaussian noise (citing [79]) and Wu et al.'s "Galaxy Learning" position paper on an ideal privacy model (citing [45]).

## Key Points

- The paper identifies 31 design-pattern variations across five blockchain-FL workflow stages (registration, distribution, training, evaluation, aggregation), organized around twelve key design questions (e.g., how to attract/select trainers, distribute models, prevent leakage, select aggregators).
- It establishes a scored pros/cons table (Table 2) for each design choice against robustness, efficiency, privacy, and fairness, using a +/- significance scale, allowing systematic comparison of design tradeoffs.
- The paper claims a linear relationship between fairness and robustness: 91% of designs with positive fairness scores also have positive robustness scores, because fairness-improving mechanisms (e.g., contribution-based rewards, peer review) also limit adversarial advantage.
- It claims that robust designs tend to come at a privacy cost: 53% of designs with positive robustness scores have negative privacy scores, mainly because blockchain's "hard-to-tamper" transparency exposes models/data to all nodes, and peer-review verification requires reviewers to access full model contents.
- It claims a universal efficiency tradeoff: 100% of designs with positive robustness or privacy scores, and 91% of designs with positive fairness scores, carry negative efficiency scores — meaning robustness, privacy, and fairness cannot all be maximized simultaneously without sacrificing efficiency.
- Through classification of 30 papers (Table 3), the paper finds contribution-based rewards are used by 66.67% of papers with reward systems (mostly via dataset size or accuracy, each 37.5%); open-trainer selection is used by 56.67% of papers vs. restricted (43.33%, mostly via deposit at 41.18%).
- It finds model-leakage encryption is used by only 53.33% of papers (mostly public-key, 45.83%); blockchain or IPFS is used for model distribution by 90.32% of papers (48.39% direct blockchain storage, 41.94% IPFS+hash).
- It finds only 36.67% of papers apply DP or HE against membership-inference (data-leakage) attacks, and only 6.67% apply model compression — flagged as a major efficiency gap and research opportunity.
- It finds only 53.33% of papers implement any model-verification/reviewer mechanism before aggregation, and 66.67% have no punishment mechanism for malicious actors.
- It finds single aggregators are used by 56.67% of papers vs. multiple (43.33%); synchronous aggregation dominates at 93.33%; off-chain aggregation is used by 66.67% of papers.
- Based on implementation classification (Table 4), the paper claims FedAvg is used in 69% of surveyed papers, most experiments use ≤10 nodes (57.69%) and run on local networks rather than testnets (only 11.54% use testnet), and only 19.23% of papers release public source code.
- The paper claims 88.46% of papers evaluate accuracy (confirming blockchain-FL accuracy parity with vanilla FL) but only 50% evaluate scalability and 38.46% measure gas cost, indicating evaluation practices are skewed away from the metrics that matter for blockchain overhead.
- It proposes an "ideal" design combination for robustness/fairness (contribution-based rewards, restricted-trainer, blockchain storage, encryption, peer-reviewed verification, deposit + reputation, multiple on-chain aggregators), a separate combination for efficiency (model compression + asynchronous aggregation), and argues homomorphic encryption is preferable to differential privacy because it avoids accuracy loss while solving both security and privacy issues.
- The survey explicitly restricts its scope to Ethereum/EVM-compatible blockchain designs, acknowledging this excludes papers using custom non-EVM blockchain architectures.

## Conclusion

The survey concludes that blockchain-based FL is broadly viable, since 86.67% of surveyed papers provide working prototype implementations with accuracy comparable to vanilla FL, and that FL and blockchain are "loosely coupled" — diverse FL algorithms (FedAvg, Fusion/Ensemble, Online FedAvg, CDW FedAvg, model chunking, signSGD), ML libraries (TensorFlow, PyTorch, Keras, Syft), and datasets (MNIST, CIFAR-10, plus domain-specific health/industrial datasets) can be combined with blockchain infrastructure largely interchangeably. Its central tradeoff finding — that robustness/fairness improvements correlate with each other but both come at an efficiency cost, and robustness gains often reduce privacy — is supported empirically by the Table 2 scoring across the 31 design patterns. The paper's main limitations and open research directions include: (1) efficiency-improving designs (model compression, asynchronous aggregation) are adopted by only a handful of papers, signaling an underexplored area; (2) most experiments use small node counts (≤10) and local rather than testnet/public-network deployments, leaving scalability and gas-cost questions for cross-device settings (which require hundreds/thousands of nodes) largely unanswered; (3) homomorphic encryption remains too inefficient for practical FL despite being theoretically ideal for simultaneous security/privacy; (4) hybrid cross-silo/cross-device architectures (e.g., split CNN models with edge-server training) need further investigation for accuracy impact and generalizability beyond CNNs; (5) the survey's Ethereum-only scope may have excluded innovative non-EVM blockchain designs; and (6) low code-availability (19.23%) hampers reproducibility and future benchmarking. The authors frame improving HE efficiency and pursuing asynchronous, compression-aware, large-scale cross-device evaluations as the most promising directions for future blockchain-FL research.
