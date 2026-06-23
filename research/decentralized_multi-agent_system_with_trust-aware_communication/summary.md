---
id: decentralized_multi-agent_system_with_trust-aware_communication
title: "Decentralized Multi-Agent System with Trust-Aware Communication"
authors:
  - Yepeng Ding
  - Ahmed Twabi
  - Junwei Yu
  - Lingfeng Zhang
  - Tohru Kondo
  - Hiroyuki Sato
year: 2025
venue: "2025 IEEE International Symposium on Parallel and Distributed Processing with Applications (ISPA)"
publisher: "IEEE"
pages: "1439-1445"
doi: "10.1109/ISPA67752.2025.00198"
url: "https://research.ebsco.com/linkprocessor/plink?id=e36b4700-c83a-301a-920d-ccc8d088cc45"
type: inproceedings
keywords:
  - multi-agent system
  - blockchain
  - agent-to-agent
  - large language model
  - self-sovereign identity
---

## Overview

This IEEE ISPA 2025 paper by Ding, Twabi, Yu, Zhang, Kondo, and Sato proposes a Decentralized Multi-Agent System (DMAS) architecture intended to support a trustworthy "Internet of Agents" built from LLM-augmented autonomous agents. The authors argue that prevailing centralized multi-agent system (MAS) architectures suffer from single points of failure, censorship and arbitrary control, scalability bottlenecks, and fundamental trust deficits, and they address these by decentralizing agent discovery, communication, and trust management onto a blockchain. The DMAS contributes two coupled components: a Decentralized Agent Runtime (a hybrid on-chain/off-chain architecture with Proxy Agents, Service Agents, and a blockchain-based Verifiable Agent Registry using self-sovereign identity / DIDs) and a formalized Trust-Aware Communication Protocol that anchors request and response commitments on-chain. The protocol leverages cryptographic primitives (hashing, symmetric and asymmetric encryption) and on-chain attestations to provide the security properties named in the abstract: verifiable interaction cycles, communication integrity, authenticity, non-repudiation, and conditional confidentiality. The accompanying semi-formal security analysis (Section IV) substantiates these claims through four numbered properties: Communication Integrity, Authenticity and Non-Repudiation, Response Confidentiality, and Verifiable Condition Fulfillment. A performance evaluation on a local Ethereum/Hardhat testnet with Autogen-orchestrated GPT-4o agents demonstrates that, once on-chain confirmation latency is subtracted, the DMAS matches the off-chain efficiency of a comparable centralized MAS, validating the hybrid design's scalability.

## Background

The paper builds on a substantial body of prior work that it cites to motivate and support its design. It says (citing others) that rapid advances in LLMs and LLM-augmented agents (surveys and systems such as ReAct, LLaMA, and broader LLM evaluations [1]-[4]) have driven a shift from single agents to MAS [5]-[7] and toward an envisioned Internet of Agents (IoA) [8] where interoperable agents discover and serve one another. It draws on MAS surveys [6], [7] and reasoning/collaboration findings [17] to claim that multiple LLM agents engaging in dialogue, debate, and mutual verification improve reasoning accuracy and mitigate hallucinations and single points of failure. It cites reflective coordination work, including self-improvement [18] and COPPER [19] (which uses counterfactual reinforcement learning via PPO and a learned reflector model to refine agent prompts), as well as domain applications in software engineering [20], [21], medicine [22], [23], and the MetaGPT collaborative framework [24], while noting these largely adopt centralized coordination that limits scalability and security. The security and decentralization foundations rest on cited blockchain primitives (Ethereum [11], the Bitcoin backbone protocol [13], and Bloccess fine-grained access control [12]) and on self-sovereign identity (SSI) and decentralized data aggregation work by the same group [14]-[16]. The implementation reuses Autogen [17] for multi-agent orchestration, and the privacy/security framing references LLM privacy and security surveys [9], [10].

## Key Points

- The paper proposes a Decentralized Multi-Agent System (DMAS) architecture combining a Decentralized Agent Runtime with a Trust-Aware Communication Protocol to secure agent discovery and interaction without reliance on a central trusted third party.
- The runtime defines two agent types: Proxy Agents (PAs), which act as the user's mandatory interface and orchestrate/route requests while maintaining conversational memory and context, and Service Agents (SAs), which execute specialized tasks off-chain and are designed to be stateless/memoryless to enable parallelism and minimize on-chain footprint.
- It introduces a Verifiable Agent Registry (VAR) implemented as a smart contract, using self-sovereign identity so every agent has a cryptographically verifiable Decentralized Identifier (DID); a Verifiable Data Registry (VDR) on a public blockchain manages DID registration, update, and revocation and resolves a DID to a JSON-LD capability schema, cryptographically binding advertised capabilities to the registered agent to prevent impersonation.
- The architecture is explicitly hybrid: DIDs, resolution, and verifiable commitments live on-chain (the blockchain as trust anchor and coordination layer), while agent hosting, heavy computation, and payload transfer occur off-chain over standard protocols (HTTP/S, WebSockets) to avoid blockchain bottlenecks.
- It formalizes a scalable service discovery mechanism using a delegation pattern in which SAs can forward requests to other candidate SAs, supporting customizable strategies including a depth-first strategy (LIFO stack, Algorithm 1) and a breadth-first strategy (level-wise queue), governed by a configurable termination predicate tau (e.g., sufficient viable options, communication-count threshold, timeout, or human intervention).
- It specifies a three-step verifiable communication protocol: (1) Request Commitment, where the PA broadcasts a request transaction <DID(u), DID(s), H(P)> committing a hash of the off-chain payload before sending it; (2) Response Commitment, where the SA verifies the on-chain request, encrypts its raw response with a symmetric key, stores the ciphertext off-chain, and commits a response transaction <H(X(P)), H(encrypted response), eta> declaring conditions eta for key release; and (3) Response Retrieval, where the PA verifies ciphertext integrity, fulfills eta on-chain (e.g., payment via smart contract), and receives the symmetric key encrypted under its public key for decryption.
- The paper provides semi-formal proofs of four security properties: Communication Integrity (request and response tampering detected via hash mismatch against the immutable ledger), Authenticity and Non-Repudiation (cryptographically verifiable DIDs plus signed, immutable on-chain transaction records prevent forgery and denial for both requests and responses), Response Confidentiality (multi-layered symmetric+asymmetric encryption with conditional key release so only the authorized PA can decrypt after conditions are met), and Verifiable Condition Fulfillment (conditions eta are publicly declared on-chain and enforced/audited by smart contract state changes).
- The performance analysis decomposes end-to-end latency into on-chain cost (transaction confirmation, bounded by block time and consensus) and off-chain cost (SA computation, LLM/OpenAI API calls, P2P transfer, discovery hops), and shows that on-chain cost is a significant but decreasing fraction of total latency as concurrent load scales, pushing the bottleneck away from the blockchain.
- A comparative experiment against a Centralized MAS (CMAS) shows that, after subtracting on-chain time, the DMAS's off-chain latency curve is nearly identical to the CMAS, demonstrating that the DMAS achieves centralized-level computational/communication efficiency and that the only overhead is the blockchain cost incurred to guarantee the security properties.

## Conclusion

The paper concludes that the DMAS overcomes the core limitations of centralized MAS (single points of failure, censorship, and trust deficits) by combining a decentralized agent runtime with a trust-aware communication protocol. Its semi-formal security analysis supports the claimed properties of message integrity, agent authenticity, non-repudiation, conditional confidentiality, and verifiable condition fulfillment, and the experiments support the scalability and efficiency claims: the hybrid on/off-chain design lets the system mimic the efficiency of a centralized system for off-chain operations, with the measured latency gap between DMAS and CMAS attributable almost entirely to the blockchain interactions required for security. The authors frame these results as evidence that the DMAS is a practical foundation for a trustworthy and scalable Internet of Agents. The evaluation is conducted on a single physical machine over a local Layer-2-simulating Ethereum testnet (2-second block time) with up to 32 concurrent PAs and 32 SAs, and the security analysis is explicitly "semi-formal" and assumes the correctness/robustness of the underlying cryptographic primitives, which suggests open avenues for fully formal verification, real-world/multi-node deployment, larger-scale and adversarial evaluation, and reduction of on-chain confirmation overhead. The paper does not deeply evaluate reputation-based discovery, economic/incentive attacks, or the cost of on-chain operations beyond latency, leaving these as further research directions.
