---
id: sok-blockchain_agent-to-agent_payments
title: "SoK: Blockchain Agent-to-Agent Payments"
authors:
  - Yuanzhe Zhang
  - Yuexin Xiang
  - Yuchen Lei
  - Qin Wang
  - Tian Qiu
  - Yujing Sun
  - Spiridon Zarkov
  - Tsz Hon Yuen
  - Andreas Deppeler
  - Jiangshan Yu
  - Kwok-Yan Lam
year: 2026
venue: "arXiv"
doi: "10.48550/arXiv.2604.03733"
url: "https://arxiv.org/abs/2604.03733"
type: misc
keywords:
  - Agent-to-Agent payments
  - blockchain
  - agentic AI
  - x402
  - payment lifecycle
---

## Overview

This Systematization of Knowledge (SoK) paper by Zhang et al. (Nanyang Technological University et al.) is the first systematic treatment of blockchain-based Agent-to-Agent (A2A) payments, the emerging class of mechanisms (exemplified by x402, Stripe's MPP, and ledger-anchored micropayment designs) that let autonomous LLM agents pay one another or pay for services with minimal human intervention. It addresses a core gap: enabling agents to pay autonomously does not by itself ensure that payment intent, authorization, execution, and service outcome stay aligned, and once financial authority is delegated to agents, misalignment or adversarial manipulation translates directly into financial loss, unauthorized spending, and governance violations. The paper's central contribution is a four-stage lifecycle model — discovery, authorization, execution, and accounting — that serves as a common analytical reference for mapping heterogeneous designs and localizing risks to specific stages. Using this model, the authors categorize representative systems at each stage, derive four cross-cutting challenges (weak intent binding, misuse under valid authorization, payment–service decoupling, and limited accountability), and outline future directions for cross-stage consistency, behavior-aware control, and compositional payment workflows. The work argues that existing designs provide only partial, stage-local guarantees and fail to preserve correctness end-to-end across the workflow.

## Background

The paper builds on a broad body of prior work that it cites to motivate and frame its own analysis. It situates A2A payments in the context of agentic AI and LLM tool-use research — AutoGen multi-agent conversations, ReAct, Toolformer, ToolLLM, generative agents, and surveys of agentic systems' planning and reasoning — and points to emerging agent-native platforms such as OpenClaw and Moltbook as early prototypes of agent ecosystems. It contrasts conventional human-oriented payment rails (Mastercard documentation, W3C Payment Request API) and card-network agent extensions like Mastercard Agent Pay, arguing (citing others) that card-network authorization/settlement processes each interaction independently and is therefore ill-suited to high-frequency, low-value, multi-step agent workloads. For the blockchain substrate, it draws on Nakamoto's Bitcoin, Buterin's Ethereum smart contracts, and scaling work (sharding, ZK rollups, L2) to justify blockchain as a programmable, verifiable settlement layer, while citing blockchain-performance surveys for its latency/fee/finality constraints. The taxonomy maps specific external designs: x402, Stripe MPP, Vaziry et al. (ledger-anchored A2A + x402), Skyfire's Know Your Agent layer, Kite's agent-native payment infrastructure, EIP-8004 reputation/trust, ERC-4337 account abstraction, ERC-20 approvals, EIP-2612/Permit2 signed permits, A402's atomic service channels, Acharya's on-chain intent proofs, and deferred/liquidity-aware settlement protocols. For risks, it cites prompt-injection and protocol-exploit literature (including red-teaming of Google's agent payment protocol and the A2ASecBench benchmark), tool-squatting/AgentCard-spoofing work, and institutional/regulatory sources — NIST SP 800-63 digital identity guidelines (separating proofing, authentication, federation), FATF AML/CFT requirements for virtual assets, and Coinbase Tracer/KYT transaction screening.

## Key Points

- The paper proposes a four-stage lifecycle model for blockchain-based agentic payments — **discovery, authorization, execution, and accounting** — as a common abstraction for reasoning about and comparing heterogeneous A2A payment systems.
- **Discovery** is decomposed into discovery substrates (D1 descriptor-based via AgentCards/registries, D2 identity-augmented via Skyfire/Kite/EIP-8004, D3 behavior-derived via interaction traces and reputation) and intent-binding primitives (B1 payment-term binding via in-band x402-style challenge–response signaling, B2 context/provenance binding via identity/session delegation objects and Acharya's on-chain intent proofs).
- **Authorization** is decomposed into authorization carriers (A1 contract-mediated delegation via role-based access, A2 allowance/approval-mediated spending via ERC-20 approvals and EIP-2612 permits, A3 wallet-mediated programmable authorization via ERC-4337 account abstraction) and policy expressiveness (E1 access-level, E2 transaction-level, E3 contextual/stateful policies such as rate and cumulative-spend limits).
- **Execution and settlement** is decomposed into settlement paths (S1 direct on-chain, S2 off-chain-coordinated with deferred settlement), submission/fee orchestration (O1 client-submitted, O2 account-abstraction relayed with paymaster gas sponsorship, O3 facilitator-mediated off-chain authorization), and access-gating evidence (G1 inclusion-gated, G2 off-chain update-gated, G3 proof/attestation-gated as in A402 and Acharya).
- **Accounting** is decomposed into verification evidence (V1 on-chain transaction evidence, V2 interaction-level receipts/logs) and service–payment coupling (C1 execution-trigger coupling, C2 post hoc accountability linkage such as insurance layers, C3 enforcement-based coupling such as A402's atomic service channels using adaptor signatures).
- **Challenge 1 — weak intent binding:** discovery anchors payments to externally exposed, unauthenticated metadata, creating a phishing-like surface (counterfeit endpoints, spoofed AgentCards, typosquatting); in-band signaling fails to tie payments to a specific session, outcome, or verifiable fulfillment, so payments can stay valid under replay, mismatch, or partial/incorrect service delivery.
- **Challenge 2 — misuse under valid authorization:** authorization is a local per-transaction validity filter that assumes trustworthy transaction generation; compromised agents (via prompt injection, model manipulation, key leakage) can produce policy-compliant malicious transactions, and sequences of individually valid transactions (through fragmentation, repetition, timing, or multi-agent collusion) can violate intended spending boundaries while standing delegation drifts away from substantive user consent.
- **Challenge 3 — payment–service decoupling:** execution treats payment success/finality as completion rather than real-world service delivery; settlement is non-deterministic (latency, reordering, probabilistic finality) and depends on extended trust boundaries (bundlers, relayers, paymasters), so providers can receive valid payment without delivering correct outcomes and parties lack a shared finality/completion anchor.
- **Challenge 4 — limited accountability:** the immutable ledger records that a transfer occurred but not why or which component (user, agent, service) was responsible; on-chain payment evidence is only loosely, post hoc correlated with off-chain outcomes via logs/receipts, and transparency for auditing conflicts with privacy, creating a trade-off between incomplete attribution and excessive exposure.
- The paper additionally identifies an **authentication/identity** gap: authentication verifies access (a key, account, or endpoint) but does not establish identity proofing or attribution, leaving systems unable to determine the responsible principal or satisfy compliance (AML/CFT, KYA) requirements.
- The overarching claim is that existing A2A payment designs provide only partial, stage-local guarantees and do not maintain consistency, control, and security across the full lifecycle.

## Conclusion

The paper concludes that while blockchain enables agents to make payments, it does not inherently guarantee their correctness; existing designs offer partial guarantees at individual lifecycle stages but fail to preserve correctness across the end-to-end workflow. The authors do not run experiments — this is a systematization — and their core thesis is supported by mapping concrete designs and documented vulnerabilities onto the four-stage model, showing that the key gaps consistently lie in intent binding, delegated control over evolving agent behavior, and accountable linkage between payment and service outcomes. They outline four complementary future directions: (1) **Consistency** — protocol-level payment–service binding via a shared append-only execution record that spans the lifecycle (committing intent at discovery, policy decisions at authorization, settlement references at execution, and outcome evidence at accounting) to enable end-to-end consistency checking; (2) **Control / behavior-aware control** — governing behavior rather than isolated transactions by accounting for cumulative spend, interaction frequency, and counterparties, with dynamic tightening/revocation and decentralized identity management; (3) **Composition** — coordinating dependency-aware, cross-system payments across multi-hop, chained, and concurrent workflows spanning mixed on-chain settlement and off-chain service rails; and (4) **Formation** — supporting payable terms that are negotiated across interactions (e.g., x402 "up-to" and negotiated pricing) rather than assumed fixed before authorization and execution. Open research questions noted include closing the privacy–accountability trade-off, establishing causal attribution across the user–agent–service chain, enforcing institutional/regulatory admissibility (jurisdiction, sanctions, licensing, AML/CFT) at intent-formation time, and achieving a shared notion of workflow finality across heterogeneous settlement environments.
