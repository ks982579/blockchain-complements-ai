---
id: betaweb-towards_a_blockchain-enabled_trustworthy_agentic_web
title: "BetaWeb: Towards a Blockchain-enabled Trustworthy Agentic Web"
authors:
  - Zihan Guo
  - Yuanjian Zhou
  - Chenyi Wang
  - Linlin You
  - Minjie Bian
  - Weinan Zhang
year: 2025
venue: "arXiv"
doi: "10.48550/arXiv.2508.13787"
url: "https://research.ebsco.com/linkprocessor/plink?id=b2c731ff-30dd-3aac-99a8-c2be82446762"
type: misc
keywords:
  - BetaWeb
  - Agentic Web
  - Blockchain
  - Trustworthy AI
  - Multiagent Systems
---

## Overview

This is a position and framework paper proposing "BetaWeb" (β-Web), a blockchain-enabled trustworthy Agentic Web for LLM-based multi-agent systems (LaMAS). The authors argue that current Agentic AI ecosystems are fragmented and platform-centric ("walled gardens" and "digital feudalism" under giants like Google Agentspace and AWS Marketplace), and that the emerging open "Agentic Web" vision is blocked by three core challenges: privacy protection and risk control, coordination complexity and data management, and value measurement and individual incentives. The paper contends that existing centralized or semi-centralized trust paradigms (public key infrastructure, consortium/federated models, trusted execution environments) cannot meet open, heterogeneous, cross-domain demands, and maps blockchain's native strengths (decentralized consensus, immutable records, programmable smart contracts, zero-knowledge proofs) directly onto those three challenges. Its main contributions are a general BetaWeb architecture (abstracting all interactions into on-chain task request/execution/feedback procedures over three party levels and four core system modules), a five-stage evolutionary roadmap from "Isolated Silos" to "Full Autonomy," a comparative analysis of existing blockchain-plus-LaMAS products, and a discussion of open challenges across technological, ecosystem, and societal dimensions. A central conceptual claim is that BetaWeb advances the web paradigm from Web3 (data ownership) toward "Web3.5," which emphasizes ownership of agent capabilities and the monetization of intelligence.

## Background

The paper builds on a large body of cited prior work rather than presenting empirical results of its own. It frames the rise of LaMAS as an evolution of LLM-based AI agents from rule-based passive units into autonomous perceiving/learning/deciding/executing entities and from single-node to distributed multi-agent architectures (citing Wang et al. 2024a, Guo et al. 2024, Gao et al. 2025, Yang et al. 2024). It adopts the "Agentic Web" concept from Yang et al. (2025b) as a fully-connected, agent-driven network enabling a machine-to-machine economy, and carefully distinguishes Agentic AI (ecological paradigm shift), LaMAS (concrete systems), and Agentic Web (infrastructure). The critique of platform centralization frames the current state as "digital feudalism" and draws on cited work on walled gardens controlled by centralized entities (Scheuerman 2024), big-data price discrimination (Pandey & Caliskan 2021), algorithmic manipulation (Ienca 2023), and data-sharing opacity (Bietti 2025). Its argument that prior trust paradigms are inadequate cites surveys of PKI failures (Dumitrescu & Pouwelse 2024), consortium/federated blockchains (Bai et al. 2021; Ksystra et al. 2022), and trusted execution environments (Geppert et al. 2022; Sardar & Fetzer 2023). Blockchain capabilities are grounded in cited work on zero-knowledge proofs (Zhou et al. 2024; Čapko et al. 2022), distributed ledgers and consensus (Bellaj et al. 2024; Islam et al. 2024), smart contracts (Xu et al. 2024), DAOs (Bonnet & Teuteberg 2024), and incentive mechanisms (Han et al. 2022). The five-stage roadmap is explicitly paralleled to OpenAI's five-level AGI framework. Existing-product analysis references real platforms including HajimeAI, ChainOpera AI, AGNTCY (Web3-oriented), and Legion/PayEgis and BOP/Xinghuo (regulation/compliance-oriented). Challenge discussions cite agent security threats (Deng et al. 2025), on-chain/off-chain scalability (Chiedu et al. 2025), attribution problems such as last-touch attribution (Sriram et al. 2022), and moral/legal responsibility attribution for AI agents (Ayad & Plaks 2025).

## Key Points

- The paper introduces BetaWeb, a blockchain-enabled trustworthy Agentic Web that integrates blockchain with LaMAS to provide a trustworthy, scalable, decentralized infrastructure for large-scale, heterogeneous, cross-domain autonomous agent interaction.
- It maps blockchain's three native strengths onto the Agentic Web's three core challenges: immutable on-chain records plus zero-knowledge proofs address privacy/risk; consensus and distributed ledgers address coordination/data-synchronization complexity; programmable smart contracts address verifiable value attribution and incentive distribution.
- It argues for a paradigm upgrade from Web3 (centered on data ownership and monetizing data) to "Web3.5," centered on ownership of agent capabilities and monetization of intelligence (a capability economy / agent-to-agent machine economy).
- It proposes a general BetaWeb framework that abstracts all interactions into standardized on-chain task procedures (request, execution, feedback), with the full lifecycle of agents and tasks anchored to the blockchain for verifiable identity, immutable records, and transparent governance.
- It redefines three key involved parties: users shift from operators to consumers/managers/owners of agents; agents evolve from tools into autonomous self-governing entities; and agentic workflows shift from static, linear, manually-driven processes to dynamic, closed-loop, self-optimizing ecosystems.
- It defines four core system modules across two layers: upper business modules (task management and task execution) and lower support modules (agent management and rule management/on-chain governance).
- It presents a five-stage evolutionary roadmap: S1 Isolated Silos (human-controlled, siloed), S2 Pilot Decentralization (human-led with limited cross-platform collaboration), S3 Assisted Execution (agent-assisted, freeing human labor), S4 Hybrid Governance (human-agent co-governance), and S5 Full Autonomy (autonomous LaMAS with humans setting only high-level intents/goals), with specified enabling technologies and milestones per stage.
- It assesses that current real-world blockchain-plus-LaMAS products largely remain at Stage 2 (Pilot Decentralization), constrained by an unresolved tension among high-concurrency processing, strong trust guarantees, and data privacy protection.
- It claims a bidirectional relationship: blockchain enables LaMAS while increasingly autonomous agents (especially in Hybrid Governance and Full Autonomy stages) reciprocally drive innovation in blockchain on-chain governance mechanisms.
- It catalogs open challenges across technological (cross-chain exchange, on-chain performance/scalability, secure multi-agent planning against malicious agents), ecosystem (incentive/attribution fairness, agent marketplaces, cross-domain identity), and societal (value alignment, responsibility attribution, legal lag, public safety) dimensions.

## Conclusion

The paper concludes that blockchain offers fundamental trust mechanisms and a trustless collaborative environment for LaMAS and can enable decentralized collaboration, trusted governance, and enhanced autonomy, supporting a more open, secure, and efficient agent ecosystem; it emphasizes that the future blockchain-BetaWeb relationship will be bidirectional and co-evolutionary. As a position/framework paper, its claims are argued conceptually and supported through structured synthesis, a staged roadmap, and qualitative product analysis rather than experimental validation; the authors explicitly note that existing products only reach Stage 2 and that the triple demands of concurrency, trust, and privacy remain unreconciled. The paper foregrounds substantial open research questions: technologically, achieving large-scale on-chain deployment without sacrificing performance, low-latency cross-domain consensus, verifiable hybrid on-chain/off-chain computation, and security against malicious agents; on the ecosystem side, fair value attribution (e.g., last-touch attribution, forward traceability), agent-marketplace incentive design, and cross-domain identity/trust; and societally, cross-cultural value alignment, responsibility attribution for autonomous agent behavior, lagging legal frameworks, and the impact of self-governing systems on social and economic structures. The authors state their future work is to build and experimentally test a BetaWeb prototype to validate blockchain-driven decentralized governance and autonomy in real-world applications.
