---
id: generative-AI_BC_NFT_federated_learning__6G_environment
title: "Generative-AI(with Custom-Trained Meta's Llama2 LLM), Blockchain, NFT, Federated Learning and PBOM Enabled Data Security Architecture for Metaverse on 5G/6G Environment"
authors:
  - Eranga Bandara
  - Peter Foytik
  - Sachin Shetty
  - Amin Hassanzadeh
year: 2024
venue: "2024 IEEE 21st International Conference on Mobile Ad-Hoc and Smart Systems (MASS)"
pages: "118-124"
url: "https://api.semanticscholar.org/CorpusID:273591860"
type: inproceedings
keywords:
  - Metaverse
  - Generative-AI
  - LLM
  - Llama2
  - 5G
  - 6G
  - Blockchain
  - Federated Learning
  - NFT
---

## Overview

This paper presents a reference architecture for securing personal data and digital assets in the Metaverse by integrating Generative-AI (a custom-trained Meta Llama2 LLM), blockchain, Non-Fungible Tokens (NFTs), federated learning, and a Pipeline Bill of Materials (PBOM) verification system. The core problem addressed is that Metaverse platforms — built on behavioral-learning technologies like AR/VR and AI/ML — collect massive amounts of personal data and rely on AR/VR hardware whose software updates are vulnerable to supply-chain attacks, raising serious privacy and security concerns. The architecture organizes resource providers (AR/VR service providers, AR/VR device providers, AI/ML service providers) into a blockchain network governed by six smart contracts (Identity, Policy, Model Card, SBOM, PBOM, NFT), represents physical AR/VR devices as purchasable NFT tokens, manages user identity via Self-Sovereign Identity (SSI) wallets, enforces device communication via Zero-Trust policy rules, and uses a fine-tuned, quantized Llama2-13B model (via QLoRA) to automatically generate PBOM JSON objects documenting software update provenance. The paper claims to be the first research effort to standardize PBOM schemas and use LLM-based generation of PBOMs, and validates the design with a prototype testbed built on Ericsson 5G New Radio with an Open5GS 5G core, the Rahasak blockchain, and an on-premises Llama2 deployment via Ollama.

## Background

The paper situates its architecture within prior definitions of the Metaverse as a persistent, real-time, interoperable, decentralized 3D virtual space built on Web 3.0 principles (citing Ning et al.'s metaverse survey and Kshetri's work on Web 3.0/Metaverse branding). It draws on NFT literature (Wang et al.) for the concept of NFTs as a mechanism for owning and trading decentralized digital assets across interoperable virtual worlds (e.g., Decentraland, Sandbox, AltspaceVR). Privacy and security concerns specific to the Metaverse — including data collection/surveillance risks from AR/VR and ML "behavioral-learning" technologies — are grounded in Wang et al.'s survey on Metaverse fundamentals, security, and privacy. The supply-chain security angle builds directly on the authors' own prior work, "Let's-Trace" (Bandara et al., MILCOM 2021), which introduced a blockchain/federated-learning/TUF/in-toto cyber supply-chain provenance platform, and on the PBOM standard from pbom.dev as well as general SBOM literature (Axelsson & Larsson). The Llama2 LLM itself comes from Touvron et al. (Meta, 2023), and the fine-tuning approach relies on QLoRA (Dettmers et al., NeurIPS 2024) for 4-bit quantized low-rank adaptation, with deployment via Ollama. The federated learning component is built on the authors' earlier "BASSA-ML" platform (Bandara et al., CCNC 2022), which integrated blockchain and Model Cards into a federated learning provenance system, and on Model Card concepts from Wadhwani & Jain for ML transparency/auditability. Identity management leverages the authors' prior Self-Sovereign Identity platform work (Bandara et al., ICCCN 2021), and network security policy enforcement follows NIST's Zero Trust Architecture (Rose et al., 2020). The underlying blockchain infrastructure (Rahasak) and its Aplos smart-contract/actor system are also the authors' own earlier contributions (Bandara et al., Journal of Systems Architecture 2021; BlockSys 2019). Related Metaverse architecture proposals discussed for comparison include Bc-Metaverse, AI-Metaverse, MetaChain, Meta-FML, and a blockchain-Metaverse contribution survey, none of which the paper claims combine blockchain, NFT, federated learning, 5G/6G, and Generative-AI together.

## Key Points

- The paper introduces a five-layer decentralized Metaverse architecture (resource provider layer, blockchain/smart contract layer, LLM layer, NFT marketplace layer, end-user layer) that integrates Generative-AI, blockchain, NFTs, and federated learning to address Metaverse data privacy and security concerns.
- It defines six blockchain smart contracts — Identity, Policy, Model Card, SBOM, PBOM, and NFT — that together automate identity management, network policy enforcement, AI/ML model provenance, software bill-of-materials generation, and NFT-based device trading.
- The paper claims to be the first research effort to standardize PBOM schemas and apply a Large Language Model (custom fine-tuned Llama2-13B) to automatically generate PBOM objects for AR/VR device software updates, using QLoRA-based 4-bit quantized LoRA fine-tuning for deployment on consumer-grade hardware via Ollama.
- It proposes representing physical AR/VR devices as NFT tokens with a custom NFT schema ("i528") that links to Model Card objects, enabling users to purchase devices and verify their provenance/audit data before use.
- It proposes managing user and resource-provider identities through a Self-Sovereign Identity (SSI)-enabled mobile/desktop wallet, where real identity data stays with the user and only proofs are published to the blockchain ledger.
- It proposes a Zero-Trust security-enabled policy engine in which AR/VR device communication rules are encoded as on-chain policies and continuously checked/enforced at runtime.
- It proposes using Model Card objects recorded on-chain to capture AI/ML model data provenance (training data, aggregation peers, generation times, evaluation metrics) as a mitigation against adversarial learning attacks, combined with a blockchain-enabled coordinator-less federated learning approach for AI/ML providers to collaboratively train models transparently.
- The paper reports a prototype implementation on a 5G testbed using Ericsson New Radio with an Open5GS 5G core, the Rahasak blockchain (with Aplos smart actors) for ledger storage, and an on-premises Llama2 LLM deployment.
- Blockchain evaluation results show linear throughput scalability for invoke/query transactions as the number of peers increases, and search performance of approximately 4 milliseconds across 2 million records.
- LLM evaluation demonstrates that the custom-trained Llama2 model, given prompts encapsulating pull-request data, SBOM information, and vulnerability statuses, can generate structured JSON PBOM objects containing update creator/verifier/approver information, timestamps, test results, vulnerabilities, and pull-request status.
- A comparative table positions the proposed architecture against five prior Metaverse architectures (Bc-Metaverse, AI-Metaverse, MetaChain, Meta-FML, Metaverse-Contribution), claiming it is the only one combining high data security, blockchain, NFT support, 5G/6G integration, federated learning, and Generative-AI.

## Conclusion

The paper concludes that its proposed decentralized Metaverse architecture — combining Generative-AI (custom Llama2), blockchain, NFTs, federated learning, and SSI-based identity management — provides a feasible approach to addressing the privacy and security concerns raised by AR/VR, machine learning, and behavioral-data-driven Metaverse platforms. The authors report that their prototype testbed evaluation yielded "promising results" for both the blockchain (linear transaction scalability and fast search performance over millions of records) and the LLM component (successful generation of structured PBOM JSON objects from custom prompts). However, the evaluation appears largely qualitative/demonstrative for the LLM (no quantitative accuracy, precision, or correctness metrics for generated PBOMs are reported), and the blockchain evaluation focuses narrowly on throughput and search latency without addressing broader concerns such as security analysis of the smart contracts, real-world adversarial testing of the Zero-Trust policy engine, or user studies of the SSI wallet. The paper explicitly notes its testbed is still "presently in the process of" large-scale deployment, and states as future work the integration of more AR/VR devices into the platform and conducting more scalable performance evaluations — leaving open questions about how the system performs under realistic multi-provider, multi-device Metaverse loads, how robust the LLM-generated PBOMs are against adversarial or malformed inputs, and how the federated learning and Model Card mechanisms perform against actual adversarial/Byzantine attacks referenced but not directly tested in this work.
