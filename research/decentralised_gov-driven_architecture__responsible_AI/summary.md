---
id: decentralised_gov-driven_architecture__responsible_AI
title: "Decentralised Governance-Driven Architecture for Designing Foundation Model based Systems: Exploring the Role of Blockchain in Responsible AI"
authors:
  - Yue Liu
  - Qinghua Lu
  - Liming Zhu
  - Hye-Young Paik
year: 2024
venue: "arXiv"
doi: "10.48550/arXiv.2308.05962"
url: "https://arxiv.org/abs/2308.05962"
type: preprint
keywords:
  - blockchain
  - responsible AI
  - foundation models
  - governance
  - software architecture
---

## Overview

This paper addresses the lack of governance mechanisms in foundation model based AI systems (e.g., LLM-powered agents/applications) and proposes blockchain as an architectural solution. The authors identify eight specific governance challenges spanning three dimensions adapted from traditional IT governance theory: decision rights, incentives, and accountability. For each challenge, they describe a corresponding blockchain-based mechanism (e.g., smart contracts, on-chain identity, voting, token incentives, immutable logging). Building on this analysis, the paper presents a concrete reference architecture — organized into orchestration, data, and operation layers — that integrates a blockchain network with five on-chain smart contract registries (identity, RAG, operation, response, and incentive) to operationalize governance in real foundation model systems. The contribution is primarily conceptual/architectural: a structured mapping from governance problems to blockchain-based design patterns, intended as a reference for practitioners designing responsible AI systems involving foundation models.

## Background

The paper situates itself within the rapid 2023 proliferation of foundation models and LLMs (citing ChatGPT's rapid adoption, and competitor releases such as Google's Bard and Meta's LLaMA), framing foundation models as systems trained on broad data to perform diverse downstream tasks (citing Bommasani et al.'s "On the Opportunities and Risks of Foundation Models" and Teubner et al. on the LLM era). It draws on the authors' own prior work on reference architectures for foundation model based agents and systems (Lu et al., "Towards Responsible Generative AI" and "Towards Responsible AI in the Era of ChatGPT", and Lu et al.'s taxonomy of foundation model based systems through the lens of software architecture), extending those architectures by adding a blockchain governance layer. For its governance framing, the paper adopts traditional IT governance dimensions from Weill's work on how top-performing firms govern IT, and incorporates regulatory context from the European Commission's new Product Liability Directive (citing Luca and Rodríguez de las Heras Ballell) to motivate accountability and compensation concerns. The paper also draws on foundational blockchain literature — Nakamoto's Bitcoin whitepaper for the distributed ledger concept, Tschorsch and Scheuermann's survey on decentralized digital currencies, and Omohundro's work connecting cryptocurrencies, smart contracts, and AI — to establish blockchain's core properties: a distributed ledger for tamper-resistant transaction storage and a programmable "compute" infrastructure via smart contracts. Architecturally, it references emerging LLM application architecture patterns (Bornstein and Radovanovic, a16z) and examples of foundation models applied to climate (Leippold), medicine (Kung et al.), and generative agent simulations (Park et al.) to motivate the breadth of human-AI teaming scenarios needing governance.

## Key Points

- The paper identifies eight distinct governance challenges in foundation model based AI systems, organized under three governance dimensions: decision rights (D1: stakeholder decision rights and access control, D2: IP ownership of model-generated content, D3: selection of foundation models/agents/external tools), incentives (I1: motivating responsible behavior, I2: compensating parties harmed by unintended/harmful system behavior), and accountability (A1: identity management across organizations, A2: scrutiny/validation of foundation model operation/outputs, A3: responsible resource provenance and traceability).
- The paper argues blockchain can serve as a governance infrastructure by providing two core elements: a distributed ledger for tamper-resistant accountability records, and a decentralized "compute" infrastructure via smart contracts for automating decision-making and incentive distribution.
- The paper proposes that permissioned blockchain networks with embedded access control in smart contracts can manage stakeholders' decision rights (e.g., restricting access to training data or response validation to designated verifiers/auditors) (D1).
- The paper proposes deploying IP agreement templates as smart contracts that must be signed by system providers, foundation model providers, and users to resolve disputes over ownership of model-generated content (D2).
- The paper proposes a blockchain-based decentralized marketplace application for comparing and selecting foundation model based systems, agents, and external tools according to metrics such as price, processing time, and context window (D3).
- The paper proposes a token-based incentive mechanism (one or two token types representing rewards and penalties, with locking/destruction for violations) to motivate responsible behavior by stakeholders and even by foundation models/systems themselves, which can hold on-chain identities (I1), while noting this depends on improved model explainability since foundation models remain "black boxes."
- The paper proposes smart-contract-based registries where parties harmed by unintended or harmful system behavior can register claims, enabling compensation processes after accountability determination (I2).
- The paper proposes that all stakeholders (including products and AI agents/virtual representatives) register blockchain accounts as on-chain identities, supporting self-sovereign identity and tying on-chain identity to real-world identity verification for accountability (A1).
- The paper proposes storing user prompts and model-generated responses in smart contracts along with voting-based consensus schemes (e.g., one-verifier-one-vote, one-token-one-vote, weighted votes for system providers) for validating model outputs and resolving disputes among verifiers (A2).
- The paper proposes recording critical runtime data — foundation model inputs/outputs, external tool actions, RAG-retrieved data, and user consent — on-chain via an immutable ledger to enable traceability, auditability, and resource provenance attribution (A3).
- The paper presents a three-layer reference architecture (orchestration layer, data layer, operation layer) extending prior foundation model system architectures, integrating a blockchain network with five smart contract registries: identity registry, RAG registry, operation registry, response registry, and incentive registry.
- The paper describes the foundation model's role within the orchestration layer as a "software connector" performing four functions: communication, coordination, conversion, and facilitation between system components.
- The paper specifies operation-layer off-chain components (black box recorder, guardrails, verifier-in-the-loop) that interface with the on-chain registries to anchor off-chain data, log runtime operations, enforce responsible-AI compliance on responses, and feed contribution data into the incentive registry.

## Conclusion

The paper concludes that it has successfully mapped eight governance challenges (across decision rights, incentives, and accountability) to corresponding blockchain-based solutions, and has translated these into a concrete five-smart-contract architecture layered atop existing foundation model system designs. However, the work is explicitly conceptual/architectural rather than empirically validated — no prototype, implementation, or performance evaluation is presented. The authors acknowledge several open issues and limitations: incentive distribution to foundation models themselves depends on solving model explainability (since foundation models are "black boxes," it is unclear how to anticipate behavior in response to incentives); the proposed marketplace for foundation model systems requires a clear taxonomy of comparison metrics that has not yet been developed; practical deployment decisions (e.g., choice of voting scheme, encryption/access control for prompts and responses, and selection of a resource/energy-efficient consortium blockchain) are left to practitioners; and the architecture is presented as a "reference" requiring further design refinement. As future work, the authors state they plan to implement a proof-of-concept prototype with fine-grained design decisions, evaluate its performance, and further explore decentralized governance mechanisms for foundation model based systems — leaving empirical validation, scalability/cost analysis of the blockchain layer, and concrete governance/voting protocol design as open research directions.
