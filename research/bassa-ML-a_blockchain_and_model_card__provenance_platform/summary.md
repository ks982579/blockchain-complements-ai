---
id: bassa-ML-a_blockchain_and_model_card__provenance_platform
title: "Bassa-ML — A Blockchain and Model Card Integrated Federated Learning Provenance Platform"
authors:
  - Eranga Bandara
  - Sachin S. Shetty
  - Abdul Rahman
  - Ravi Babu Mukkamala
  - Juan Zhao
  - Xueping Liang
year: 2022
venue: "2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC)"
publisher: "IEEE"
pages: "753-759"
url: "https://api.semanticscholar.org/CorpusID:246751788"
type: inproceedings
keywords:
  - Blockchain
  - Machine Learning
  - Federated Learning
  - Model Card
  - Big Data
---

## Overview

This paper introduces Bassa-ML, a blockchain-based federated learning (FL) platform that integrates Google's Model Card framework to provide transparency, auditability, and provenance for collaboratively trained machine learning models. It addresses two core weaknesses of conventional FL systems: reliance on a centralized coordinator (a single point of failure vulnerable to attack and privacy breaches) and the absence of standardized mechanisms for tracking model provenance, training history, and versioning. Bassa-ML eliminates the centralized aggregator by implementing model parameter sharing, local model training, model averaging, and model sharing as blockchain smart contracts (Identity, Model, Supervisor, and Model Card contracts), with the block-generating peer—chosen by the blockchain's consensus algorithm—performing aggregation via stochastic gradient descent (SGD). All generated models, training metadata, and quantitative model reports are encoded as Model Card objects and stored immutably in the blockchain ledger, giving every participating peer a complete, auditable history of model iterations. The platform is built on the Rahasak blockchain with Aplos smart contracts, PyTorch/PySyft for FL, TensorFlow's Model Card Toolkit, and Apache Kafka for messaging, and is evaluated through a healthcare use case where five simulated hospital peers collaboratively train a logistic regression model to diagnose acute bladder inflammation.

## Background

The paper situates itself within a growing body of work on combining blockchain with federated/decentralized machine learning to overcome the privacy and centralization risks of traditional FL (citing Konečný et al. and Yang et al. on federated learning fundamentals, and Ryffel et al. on privacy-preserving deep learning frameworks such as PySyft). It draws on prior critiques that centralized FL coordinators are vulnerable to attacks and privacy breaches (citing works on FL security and applied FL in production systems like Google Keyboard), and on Strobel's discussion of transparency challenges in ML as motivation for provenance mechanisms. The Model Card concept itself is adopted from Google's TensorFlow Model Card Toolkit (Fang and Miao; Wadhwani and Jain), which the paper cites as the standard for structured reporting of model usage, ethics considerations, and limitations. On the blockchain side, the paper builds on its authors' own prior infrastructure work, including the Rahasak scalable enterprise blockchain and its Aplos smart contract platform, and the Tikiri lightweight IoT blockchain, as well as general blockchain/Merkle-tree and consensus concepts. For the model-averaging mechanism, it cites Kamp et al.'s work on dynamic model averaging via SGD for decentralized deep learning. The healthcare use case (diagnosing acute bladder inflammation) relies on a benchmark dataset and methodology referenced from Upstill-Goddard et al.'s work on ML for gene-gene interaction discovery, and the logistic regression approach is grounded in Sathya and Abraham's comparison of supervised/unsupervised classification algorithms. The related-work section positions Bassa-ML against a cluster of prior blockchain-FL systems—DeepChain, BlockFL, ModelChain, BAFFLE, a poster on blockchain-based privacy-preserving FL, and Chained-FL—each of which the paper says addressed subsets of decentralization, auditability, or privacy but lacked integrated, standardized model reporting.

## Key Points

- Bassa-ML proposes a blockchain-enabled, coordinator-less federated learning architecture in which no single party aggregates models; instead, the peer selected by the blockchain consensus algorithm to generate the next block performs model averaging.
- The platform implements model parameter sharing, local model generation, model averaging (via SGD), and model storing/sharing entirely through four smart contracts: an Identity contract, a Model contract, a Supervisor contract, and a Model Card contract.
- Bassa-ML stores model information (participants, generators, aggregators, generation times), quantitative analysis, and usage considerations for both local and global models as Model Card objects directly in the blockchain ledger, providing enhanced transparency and provenance to the federated learning process.
- This Model Card integration provides complete traceability of all operations and incremental model versions across all participating peers, enabling auditing and guarding against misrepresented or spoofed model iterations.
- The architecture comprises three components per peer: a blockchain ledger node, a Federated Machine Learning (FML) service (PyTorch/PySyft-based), and a Model Card service (TensorFlow Model Card Toolkit-based), coordinated via Apache Kafka and deployed with Docker/Kubernetes.
- Bassa-ML provides a mechanism for integrating trained ML models directly into blockchain smart contracts to enable real-time predictions on incoming data.
- The platform was implemented on the Rahasak blockchain using its Aplos smart contract platform, demonstrating practical feasibility on an existing enterprise blockchain infrastructure.
- A healthcare use case was demonstrated and evaluated: a five-hospital-peer blockchain network collaboratively trains a logistic regression model on an inflammation-of-the-bladder dataset, producing both local models per peer and a federated global model.
- Empirical evaluation shows local model accuracy increases and training loss decreases over 20,000 iterations at a single peer, and federated model accuracy/loss improve across 1000 iterations with model sharing among peers.
- Block generation time was measured for up to seven participating peers (averaged over 100 runs per configuration), showing that block generation time increases as more peers are added because each peer must validate transactions and recompute the block header.
- A comparative table positions Bassa-ML against DeepChain, Breaking Bad, TMLC, BlockFL, ModelChain, BAFFLE, the Poster BC-PPFL system, and Chained-FL across dimensions including blockchain platform used, application domain, support for data analytics, ML, incremental training, federated learning, auditing/provenance, model reporting, and real-time predictions.

## Conclusion

The paper concludes that Bassa-ML successfully demonstrates the feasibility of a decentralized, coordinator-free federated learning system that provides enhanced transparency and provenance by combining blockchain smart contracts with Model Card-based reporting. The healthcare use case (acute bladder inflammation diagnosis across five simulated hospital peers) showed that local models improve in accuracy and reduce training loss over training iterations, and that federated training across peers with model sharing also shows each local model improving slightly, with total loss and accuracy computed across peers. The block generation time experiments confirm a practical scalability cost: as more peers join the network, block generation time increases due to per-peer transaction validation and block header recomputation, which the paper notes but does not deeply analyze as a limitation. The paper does not report comparative quantitative results against the other blockchain-FL systems discussed in related work (DeepChain, BlockFL, ModelChain, BAFFLE, etc.)—the comparison table is feature-based rather than performance-based, leaving open questions about how Bassa-ML's overhead, throughput, and accuracy compare empirically to these alternatives. The evaluation is also limited to a single small dataset and a simple logistic regression model with five simulated peers, so generalizability to larger peer networks, more complex models (e.g., deep neural networks), or other domains beyond healthcare remains untested. As future work, the authors state they plan to integrate additional libraries into Bassa-ML, though they do not specify which libraries or what capabilities these would add—leaving an open direction for extending the platform's ML model support, privacy-preserving techniques, or scalability mechanisms.
