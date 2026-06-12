---
id: ai_agents_meet_blockchain__multi-agents
title: "AI Agents Meet Blockchain: A Survey on Secure and Scalable Collaboration for Multi-Agents"
authors:
  - Md Monjurul Karim
  - Dong Hoang Van
  - Sangeen Khan
  - Qiang Qu
  - Yaroslav Kholodov
year: 2025
venue: "Future Internet"
publisher: MDPI
volume: 17
issue: 2
pages: "57"
doi: "10.3390/fi17020057"
url: "https://www.mdpi.com/1999-5903/17/2/57"
type: article
keywords:
  - AI agent
  - blockchain
  - decentralized AI
  - large language model
  - multi-agent collaboration
  - Web3
---

## Overview

This survey examines the bidirectional integration of AI agents (especially LLM-based and multi-agent systems) with blockchain technology, arguing that prior surveys have not jointly addressed how AI agents enhance blockchain operations and how blockchain enables secure, scalable, autonomous AI agent collaboration. Drawing on a systematic review (140 articles screened down to 80 via keyword search across IEEE Xplore, ACM DL, SpringerLink, ScienceDirect, Web of Science, Google Scholar/Search), the paper categorizes contributions into AI-for-blockchain (intelligent decision making, intelligent consensus, zero-shot learning, vulnerability detection) and blockchain-for-AI-agents (infrastructure, collaboration/task integration, governance, key enablers). It further maps application domains (asset management, DeFi, DAOs, supply chain, edge computing, fault management) with empirical case studies (e.g., ai16z, Terminal of Truths, Luna, ArbDoge AI, Fetch.ai, Delysium, SingularityNET+Cardano), and outlines research challenges and future directions across data privacy/security, scalability/interoperability, decentralization/efficiency, interpretability/explainability, self-sovereignty of agents, decentralized quantum computing, and ethics/regulation.

## Background

The paper situates itself relative to six prior surveys (refs [8,23,24,26,27,28]), noting that studies like Abdelhamid et al. (AI-driven solutions to blockchain scalability/security/governance issues such as Proof-of-Deep-Learning and FL-based consensus), Zhang et al. (lifecycle-based blockchain integration into AI development stages), Ressi et al. (AI for blockchain consensus/security), and Bhumichai et al. (three-stage AI-blockchain convergence model: emerging, convergence, application eras) each cover only part of the AI-agent/blockchain intersection. It also draws on Tara et al. (multi-layered ontology frameworks and IPFS-based decentralized storage to reduce agent communication latency) and Nguyen et al. (blockchain as an economic/financial institution for autonomous AI agents). Foundational technology background covers blockchain consensus mechanisms (PoW, PoS, DPoS, BFT), Web3 building blocks (DLTs, DAOs, DeFi, NFTs, decentralized storage like IPFS/Filecoin, oracles like Chainlink), Generative AI/LLMs (transformers, RLHF), and the five-stage lifecycle of Decentralized AI (DeAI) applications (task proposing, pre-training, on-training, post-training, and deployment), citing prior frameworks such as Ocean Protocol and Render Network as examples of DeAI ecosystems.

## Key Points

- AI agents enhance blockchain through intelligent decision making (e.g., LLM-driven digital twin building automation in [63]), intelligent consensus mechanisms such as BlockAgents' Proof-of-Thought (PoT) for Byzantine-robust LLM multi-agent collaboration [64], and Proof-of-Compute (PoC) for decentralized, Nash-equilibrium-incentivized computation validation [65].
- Zero-shot learning enables AI agents to perform tasks like fraud detection, anomaly identification, and cross-chain analysis in dynamic blockchain environments without extensive retraining [66,68].
- FELLMVP, an ensemble of fine-tuned LLMs using a Contract-External Function-Call (CEC) representation, achieves 98.8% accuracy and 88% F1 for smart contract vulnerability classification [67].
- Blockchain provides infrastructure for AI agents via frameworks like the Ethereum AI Agent Coordinator (EAAC, using Ethereum + IPFS for transparent agent activity logging) [70] and DeAI/LLM-based decentralized inference infrastructures with smart-contract accountability [71]; "on-chain metabolism" and DePINs further support autonomous agent growth [77].
- Blockchain-based governance frameworks (e.g., the mABC voting mechanism for root-cause analysis in microservices) demonstrate transparent, weighted-contribution decision making among multi-agent systems [72].
- Application domains identified include asset management (smart-contract-based ownership/payments [82,83]), DeFi (AI-driven trading, lending, risk assessment, NFT-based asset representations [73,84]), DAOs (LLM-powered governance and NFT-based agent identities [63,17,85]), supply chain management (decentralized task allocation and traceability [24,81]), autonomous edge computing [88], and autonomous fault management [72].
- Empirical case studies show measurable benefits: ai16z reduced operational costs by 20% via DAO-based portfolio allocation with 31,120 token holders; Terminal of Truths increased daily transaction throughput by 24%; Luna increased user retention by 45% across 71,400 token holders; ArbDoge AI engaged over 270,890 token holders.
- Major open challenges identified: data privacy/security (multi-agent eavesdropping, permission allocation), scalability/interoperability (cross-chain protocols, standardized APIs), decentralization vs. efficiency (fully on-chain AI, tokenization standards like AI-721/AI-20), interpretability/explainability (XAI for agent decisions), self-sovereignty of agents (risks of untethered/self-replicating agents, Proof-of-Personhood as mitigation), decentralized quantum computing (resource allocation, quantum steganography risks), and ethics/regulation (liability, the ETHOS framework, GDPR/AI Act, Responsible AI).

## Conclusion

The survey concludes that the convergence of blockchain's decentralized, immutable, transparent infrastructure with AI agents' adaptive decision-making capabilities establishes a foundation for new classes of decentralized intelligent applications across finance, supply chains, governance, and autonomous computing. It identifies intelligent consensus mechanisms, blockchain-based governance/voting, and real-time decision support as key enablers validated through both theoretical frameworks and empirical deployments (e.g., ai16z, Luna, ArbDoge AI). However, the paper does not present new empirical experiments of its own; rather it synthesizes existing literature and explicitly frames scalability, interoperability, privacy, interpretability, self-sovereignty, and ethical/regulatory governance as unresolved open research questions requiring further work, positioning these as a roadmap for future research in the Web3/DeAI era.
