---
id: a_blockchain_integration_to_support__multi-agent_systems
title: "A Blockchain integration to support transactions of assets in multi-agent systems"
authors:
  - Fernando Gomes Papi
  - Jomi Fred Hübner
  - Maiquel de Brito
year: 2022
venue: "Engineering Applications of Artificial Intelligence"
publisher: "Elsevier Ltd"
volume: 107
pages: "104534"
doi: "10.1016/j.engappai.2021.104534"
url: "https://doi.org/10.1016/j.engappai.2021.104534"
type: article
keywords:
  - Blockchain
  - Multi-agent systems
  - Institutions
  - Assets
---

## Overview

This paper (Papi, Hübner, and de Brito, *Engineering Applications of Artificial Intelligence*, 2022) proposes a conceptual and technological model for integrating Blockchain into Multi-Agent Systems (MAS) so that autonomous agents can reliably and decentrally transact assets. It addresses two coupled problems: (i) MAS frameworks lack tools for secure, decentralized recording of asset transactions, and (ii) Blockchain alone records transactions but does not endow them with intangible meanings such as *asset* and *ownership*, leaving interpretation up to each individual agent and risking divergent or malicious readings. The authors' core contribution is to place the blockchain *inside* the MAS environment by embedding Ethereum Smart Contracts within CArtAgO artifacts (in the JaCaMo framework), and then layer a Situated Artificial Institution (SAI) on top to assign shared, agent-independent institutional meaning (e.g., that a `transferValue` event "counts as" a *payment*) to the blockchain-backed records. They validate the approach with an incrementally extended "build-a-house" application example, comparing a naive belief-based payment scheme, an institution-regulated bank artifact, and a full blockchain-integrated version. The result is a MAS in which asset transactions gain data integrity, traceability, transparency, and authenticity while remaining accessible to agents through ordinary environmental artifacts.

## Background

The paper builds heavily on prior MAS and Blockchain foundations cited from other sources. For MAS, it draws on the agent autonomy/proactivity/social-ability definitions of Wooldridge and Jennings, the endogenous *Agents and Artifacts* metamodel of Omicini et al. and its CArtAgO realization (Ricci et al.), the JaCaMo framework combining Jason agents (Bordini et al., AgentSpeak by Rao), CArtAgO environments, and oise organizations (Hübner et al.), and the normative-MAS literature on norms expressing obligations, prohibitions, and permissions (Boella et al., Vos et al.). Central to the work is the authors' own earlier Situated Artificial Institutions (SAI) model (de Brito et al., 2018/2019), inspired by Searle's account of how human institutions assign status functions to concrete elements (a paper counts as a dollar bill). For Blockchain, the paper relies on foundational descriptions from Antonopoulos, Narayanan et al., Swan, and Wood et al. (Ethereum yellow paper), covering decentralized trust, the double-spending problem, Merkle trees, mining, and Smart Contracts; it cites concrete figures (e.g., ~US$0.02856 per basic Ethereum transaction, ~5 min confirmation, ~200 GB ledger size by 2019 per Chauhan et al.) and notes alternative platforms Solana (Painter et al.) and Tezos (Allombert et al.). It situates itself against prior MAS-Blockchain work that uses blockchain mainly as a general-purpose secure database for agent communication and coordination (Kapitonov et al., Mariani et al., Ciatto et al.), negotiation (Calvaresi et al.), accountability (the authors' own Papi et al. 2017), and energy/market simulation (Wang et al., Brousmichc et al.), and references Calvaresi et al.'s systematic literature review of the field.

## Key Points

- Blockchain can be feasibly integrated into a MAS, and its appropriate role is as a *component of the environment* (an endogenous, non-autonomous element agents perceive and act upon), restricted to critical actions needing record integrity rather than recording every agent interaction.
- Recording every agent action on-chain ("Blockchain as a generic environment") is unfeasible due to transaction cost, ~5-minute confirmation latency, and storage/scalability bottlenecks; instead, blockchain should instrument only a subset of application artifacts tied to asset transactions.
- The authors propose a concrete implementation pattern: embed Ethereum Smart Contracts (Solidity) inside CArtAgO artifacts, where artifact operations map to contract functions and observable properties map to `public view` functions, bridged via the Web3j library (JSON-RPC) and an "interface artifact" — so agents access blockchain through ordinary artifacts without needing blockchain-specific knowledge.
- Layering a Situated Artificial Institution over the environment provides a shared, explicit, agent-independent interpretation of blockchain records, using constitutive rules of the form "x counts as y when t while m" to assign asset-related status functions (e.g., an operation counts as a *bid*, a `transferValue` event counts as a *payment*, a bank balance property counts as *balance*).
- Naive payment schemes where assets exist only in agents' belief bases are vulnerable to fabricated balances, refusal to deduct payment, and double-spending from other agents' perspectives — demonstrating that asset notions must be represented externally and independently of any agent.
- The combination uniquely advances prior work on two axes: an endogenous integration enclosing the blockchain within application artifacts, and an institutional layer that removes reliance on per-agent interpretation of on-chain records.
- Adding blockchain gives the MAS persistence (records survive system restarts), decentralized fraud-proof consensus, and a "door to the outside world" so that asset value is not confined to a single MAS's internal context.
- The integrated framework adds data integrity, traceability, transparency, and authenticity to MAS asset management, suited to commerce/value-transaction systems that are not speed-critical.

## Conclusion

The paper affirms both of its research questions: integrating a blockchain into a MAS is viable (best modeled as an environmental component embedded in artifacts), and asset-related notions *can* be represented in a MAS via institutionalization. The build-a-house example supports these claims, showing the progression from an insecure belief-based payment scheme to an institution-regulated, blockchain-backed system that yields persistent, reliable, meaning-endowed transaction records (deployed and verifiable on Ethereum's Kovan test network). The authors are explicit about limitations: institutionalization of assets is achieved only *indirectly* — SAI can institutionalize events (payment) and states (balance) but cannot directly institutionalize the *asset* (e.g., *money*) itself, because SAI and competing institutional models lack abstractions for environmental objects. Performance is a recurring caveat: blockchain is slow (~5 min validation, ~10–30 executions/sec on Ethereum) and potentially expensive at scale, and SAI itself can bottleneck because it serializes concurrent environmental facts into a single sequence of institutional states. Stated future work includes (i) direct institutionalization of assets, (ii) per-agent accounts in dedicated artifacts rather than one shared Ethereum artifact, (iii) recording the institutional state itself on-chain for distributed persistence, and (iv) a rigorous performance analysis cross-comparing smart-contract platforms (e.g., Ethereum vs. Solana vs. Tezos).
