---
id: blockchain-enabled_federated_learning-a_survey
title: "Blockchain-enabled Federated Learning: A Survey"
authors:
  - Cheng Li
  - Yong Yuan
  - Fei-Yue Wang
year: 2021
venue: "2021 IEEE 1st International Conference on Digital Twins and Parallel Intelligence (DTPI)"
pages: "286-289"
doi: "10.1109/DTPI52967.2021.9540163"
type: inproceedings
keywords:
  - blockchain
  - federated learning
  - smart contract
---

## Overview

This paper presents what its authors claim is the first comprehensive survey of blockchain-enabled federated learning (BeFL), addressing the challenge that federated learning (FL) — a distributed machine learning paradigm designed to train models on decentralized data while preserving privacy — suffers from single-point-of-failure risk, unverifiable parameter quality, and weak incentives for honest participation. The paper's central contribution is a six-layer conceptual technical framework for BeFL spanning infrastructure, consensus mechanism, economic incentive, smart contract, privacy protection, and application layers, along with a description of the basic BeFL workflow involving training nodes, verification nodes, and smart-contract-governed model aggregation rounds. It argues that blockchain and FL are mutually complementary at three levels: their cooperation models are structurally similar (both require multi-party distributed collaboration), they share security/privacy goals (FL protects data privacy, blockchain provides tamper-proof record-keeping via consensus), and their purposes are complementary ("creating value" via FL vs. "transferring value" via blockchain's reward mechanisms). For each of the six layers, the paper surveys existing research issues and representative solutions, citing specific systems (e.g., FedCoin, committee consensus, proof-of-stake-inspired consensus) as illustrative examples. The paper concludes that BeFL integration is still in an early/infancy stage and identifies open research areas across the six layers.

## Background

- The paper situates FL within Google's 2016 origin (the Input Method Optimization Project) and cites Kairouz et al.'s "Advances and Open Problems in Federated Learning" (arXiv:1912.04977) as the basis for classifying FL into horizontal FL, vertical FL, and federated transfer learning.
- It draws on Cisco's data-volume estimates (~850 ZB worldwide) to motivate the "data island" problem driving demand for secure, trusted data circulation and sharing.
- For consensus-layer ideas, the paper cites Chen et al.'s "Robust Blockchained Federated Learning with Model Validation and Proof-of-Stake Inspired Consensus" (arXiv:2101.03300) as an example of proof-based mechanisms adapted for BeFL, where honest nodes are rewarded with equity and the most-contributing miner creates the legal block.
- It cites Li et al.'s "A Blockchain-Based Decentralized Federated Learning Framework with Committee Consensus" (IEEE Network, 2021) as an example of alliance-based/committee consensus mechanisms improving training efficiency.
- For the economic incentive layer, it cites Liu et al.'s FedCoin (arXiv:2002.11711), which uses Shapley value (SV) computation by blockchain consensus nodes and a "proof of Shapley protocol" to allocate FL participant rewards.
- It also cites Toyoda and Zhang's incentive-aware blockchain-enabled FL platform (2019 IEEE Big Data) as a basis for designing incentive-compatible reward policies grounded in competition theory.
- For the smart contract layer's vision of AI-driven agents, it cites Połap, Srivastava, and Yu's "Agent architecture of an intelligent medical system based on federated learning and blockchain technology" (Journal of Information Security and Applications, 2021).
- For privacy protection, it references Short et al.'s "Using Blockchain Technologies to Improve Security in Federated Learning Systems" (2020 IEEE COMPSAC) as an example of incorporating privacy technologies (secure multi-party computation, differential privacy, homomorphic encryption) into BeFL frameworks.
- The paper also references Inter-Planetary File System (IPFS) as an established off-chain storage technology relevant to addressing blockchain storage/bandwidth limitations for model parameters.

## Key Points

- The paper proposes a novel six-layer conceptual framework for BeFL — infrastructure, consensus mechanism, economic incentive, smart contract, privacy protection, and application layers — as a structuring device for the field.
- It articulates a basic BeFL workflow in which a task publisher deploys smart contracts to broadcast learning tasks and node-selection rules, participants act as miners/training nodes that train local models and broadcast parameters, a subset act as verification nodes that aggregate and validate a global model, and blockchain consensus determines block acceptance and reward distribution.
- It argues that BeFL's decentralized, open, peer-to-peer infrastructure layer eliminates the single point of failure inherent to traditional FL's central coordinator, while improving scalability, load balancing, and communication anonymity as nodes freely join and exit.
- It claims BeFL's blockchain-based data verification mechanism (beyond standard transaction verification) helps guarantee the quality of uploaded model parameters from training nodes, addressing a key weakness of traditional FL.
- It identifies off-chain model storage via IPFS (with only hash values recorded on-chain) as a practical solution to the storage and bandwidth burden of recording full model parameters/gradients on the blockchain.
- It maps blockchain consensus mechanism families (vote-based, proof-based, alliance-based, randomness-based/hybrid) onto BeFL use cases, arguing that node selection for FL training rounds should additionally weigh data quality, computing resources, and network conditions, not just standard consensus criteria.
- It frames the economic incentive layer as decomposable into three design dimensions — user data quality (e.g., Shapley value-based schemes like FedCoin), user behavior (incentive-compatible reward policies), and user reputation (used both for node selection and for resolving forks by favoring high-reputation miners).
- It argues smart contracts can fully replace the centralized FL coordinator, managing task release, training/aggregation commands, update verification, data pricing, and token-based rewards in a decentralized, transparent, and tamper-proof manner.
- It proposes that future smart contracts in BeFL should evolve beyond static "if-then" rules toward AI-driven autonomous agents capable of perception, reasoning, learning, task selection, and goal-oriented decision-making.
- It surveys privacy-protection techniques applicable to BeFL (secure multi-party computation, differential privacy, homomorphic encryption) and highlights the inherent trade-offs each introduces — increased communication overhead for differential privacy and secure multi-party computation, and reliance on trusted execution environments for homomorphic encryption.
- It outlines three application domains for BeFL — federated cloud computing, smart healthcare, and smart cities/digital twins/6G IIoT — arguing BeFL can break "data island" silos while preserving privacy and improving trust in each domain.

## Conclusion

The paper concludes that integrating blockchain with federated learning is a clear and growing technological trend, and that its proposed six-layer framework provides a useful organizing structure for understanding the mutual benefits (shared cooperation models, shared security/privacy goals, and complementary "value creation" vs. "value transfer" roles) and the current state of research across infrastructure, consensus, incentives, smart contracts, privacy, and applications. However, the paper explicitly characterizes BeFL as still being "in its infancy stage," stating that "more knowledge domains need to be studied to solve these challenges." It does not present new experimental results or empirical validation of its own — its claims are framework/taxonomy contributions supported by examples drawn from cited prior work rather than original tests. Open research directions implied throughout include: designing consensus mechanisms that properly weigh FL-specific node attributes (data quality, compute, network conditions); developing more sophisticated incentive mechanisms combining data quality, behavior, and reputation; evolving smart contracts into AI-driven autonomous agents; balancing privacy-protection overhead (communication cost, trusted execution environment dependence) against security guarantees; and further work on reinforcement-learning-based and asynchronous aggregation schemes for edge/IIoT/digital-twin applications, which the paper explicitly flags as needing "further research."
