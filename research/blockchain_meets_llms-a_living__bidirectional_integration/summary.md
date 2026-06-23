---
id: blockchain_meets_llms-a_living__bidirectional_integration
title: "Blockchain Meets LLMs: A Living Survey on Bidirectional Integration"
authors:
  - Jianghao Gong
  - Peiqi Yan
  - Yue Zhang
  - Hongli An
  - Logan Liu
year: 2024
venue: "arXiv"
doi: "10.48550/arXiv.2411.16809"
url: "https://arxiv.org/abs/2411.16809"
type: misc
keywords:
  - Blockchain
  - Large Language Models
  - Cryptography and Security
  - Artificial Intelligence
---

## Overview

This paper (arXiv 2411.16809, "Blockchain Meets LLMs: A Living Survey on Bidirectional Integration") is a living survey by Gong, Yan, Zhang, An, and Liu that maps the emerging research space at the intersection of large language models (LLMs) and blockchain. It frames the integration as bidirectional: applying LLMs to blockchain and applying blockchain to LLMs, arguing that each technology's strengths can compensate for the other's weaknesses. The motivation is that LLMs, despite advances in multimodality and explainability, suffer from privacy leakage of personal identity information (PII), hallucination, bias, unreliable probabilistic outputs, and vulnerability to data-poisoning and adversarial attacks, while blockchain offers decentralization, immutability, distributed storage, and traceability but faces its own four-fold challenge of scalability, decentralization, security, and trust (an expansion of the original trilemma). For the LLM-for-blockchain direction the survey identifies six major development directions; for the blockchain-for-LLM direction it organizes contributions around data management/privacy, decentralized model training, and decentralized AI. The contribution is a structured taxonomy plus a catalog of concrete frameworks, tools, and datasets, intended to orient researchers to current trends and open problems in this convergence.

## Background

The paper builds heavily on prior work cited as supporting context. It says (citing others) that multimodal LLMs are advancing rapidly, pointing to GPT-4 Vision and LLaVA for melanoma detection across skin tones, GPT-4 explaining neurons in other models, and cross-modal models GIT-Mol and Gemini. It cites work documenting LLM risks, including extracting training data / PII leakage, ethical concerns in healthcare, and a survey on hallucination. For blockchain foundations it cites Nakamoto's Bitcoin whitepaper (decentralization, peer-to-peer cash), Vaswani's "Attention Is All You Need" (Transformer), Brown's "Language Models are Few-Shot Learners" (GPT-3), and surveys on smart contracts, blockchain for IoT, blockchain scalability, and food-supply-chain traceability. For the integration mechanics it draws on cited federated-learning systems (ChainFL, Kim's blockchained on-device FL, Ramanan's BAFFLE, Fedstellar, VDFChain, ProxyFL), incentive-mechanism work (Stackelberg-game approaches, FDFL), and decentralized-AI proposals (GradientCoin, opML, SmartVM, Harris & Waggoner's decentralized collaborative AI). It also cites privacy techniques such as homomorphic encryption, CP-ABE, RAFT consensus, PDPChain, zk-SNARKs, and ring signatures, and references the authors' own related body of work on federated/cloud robotics, smart cities, traffic prediction, autonomous driving, and LLM network slicing as evidence of distributed-learning potential.

## Key Points

- The paper proposes a bidirectional taxonomy of blockchain-LLM integration, splitting the field into "apply blockchain to LLMs" and "apply LLMs to blockchain."
- For applying LLMs to blockchain, it identifies six major development directions, anchored largely in smart contract auditing: (1) LLM-based vulnerability detection; (2) vulnerability avoidance during automatic code completion (vulnerability-constrained decoding); (3) identification of machine-unauditable vulnerabilities/bugs (MUBs); (4) development of audit frameworks; (5) context-driven audit techniques (including prompting and dataset construction); and (6) logical reasoning and proof writing in the audit process (e.g., chain-of-thought prompting).
- It catalogs concrete smart-contract-audit tools and resources as exemplars: vulnerability-constrained decoding (Storhaug), ChatGPT for machine-unauditable bugs (Gao), the iAudit framework combining fine-tuning and LLM agents, FTSmartAudit (domain knowledge distillation + adaptive learning), AuditGPT, the SC-Bench dataset, SMARTSYS, LLMSmartSec (annotated control-flow graphs), the multi-agent SmartAudit framework, and LLM4FUZZ for LLM-guided fuzzing.
- It identifies decentralized autonomous organization (DAO) governance as an LLM-for-blockchain area, citing LLM classification of DAO proposals at ~95% accuracy (Ziegler et al.) and a DAO-governance analysis over a dataset of 16,427 DAOs revealing transparency and consistency gaps (Ma et al.).
- For applying blockchain to LLMs, it argues blockchain enables privacy-preserving data sharing and training, secure personal data protection/sharing (e.g., PDPChain consortium chain with CA, RAFT, CP-ABE), and smart-contract-based access control combined with homomorphic encryption.
- It positions blockchain as a remedy for federated learning's single-point-of-failure and incentive deficiencies through decentralized federated learning (peer-to-peer aggregation) and blockchain-based incentive mechanisms (Stackelberg games, FDFL).
- It frames decentralized AI (DAI) as a key convergence area, where blockchain ensures data integrity, fault tolerance, and democratized AI services, citing on-chain inference and decentralized LLM frameworks (GradientCoin, opML, SmartVM).
- It enumerates current application domains of the combined technology: smart contract development/management (code generation, comment generation, vulnerability detection/repair), blockchain data analysis (transaction analysis, crypto price/market/risk prediction, semantic-aware audit such as S-gram), user interaction (intelligent customer service, Q&A chatbots, personalized/explainable recommendation, fintech UX/UI), and security/risk management (malicious-behavior detection, anomaly detection, structural code embedding).
- It systematically lays out the development constraints of each technology: for LLMs, computational cost, explainability/transparency, data quality/bias, ethics/privacy, and training-data security (poisoning and adversarial-sample attacks); for blockchain, security, privacy, latency, and scalability (transaction throughput, sharding, chain storage).

## Conclusion

The survey concludes that blockchain and LLMs are complementary and that their integration holds substantial cross-disciplinary potential, with blockchain providing secure, verifiable, tamper-proof data storage for LLMs and LLMs bringing intelligent interaction, code generation, and audit capabilities to blockchain. As a review rather than an experimental study, it does not test a hypothesis itself; instead it synthesizes cited evidence (e.g., high F1/accuracy audit frameworks, ~95% DAO-proposal classification accuracy, FDFL robustness results) to argue the integration is viable and promising. It explicitly notes that blockchain services for LLMs remain an early-stage, immature field still under exploration, and that neither technology is fully developed. It highlights open challenges and future-research angles: resolving blockchain's scalability/latency/storage trade-offs (sharding, Layer-2, pruning, off-chain storage), strengthening privacy (zk-SNARKs, ring signatures), defending LLM training data against poisoning and adversarial attacks, improving explainability and governance transparency, and designing effective incentive mechanisms for decentralized training. It forecasts that convergence will be especially valuable in trust- and authentication-heavy domains such as government, finance, banking, healthcare, energy, and the Industrial IoT, and points to specialized infrastructure (e.g., LLM network slicing) as a supporting direction.
