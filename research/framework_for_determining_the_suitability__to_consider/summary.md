---
id: framework_for_determining_the_suitability__to_consider
title: "Framework for determining the suitability of blockchain: Criteria and issues to consider"
authors:
  - Vikas Hassija
  - Sherali Zeadally
  - Ishan Jain
  - Aman Tahiliani
  - Vinay Chamola
  - Shashank Gupta
year: 2021
venue: "Transactions on Emerging Telecommunications Technologies"
publisher: "Wiley"
volume: 32
doi: "10.1002/ett.4334"
url: "https://research.ebsco.com/linkprocessor/plink?id=0c25b423-0d13-3d87-b05c-2d9bc578c3f3"
type: article
keywords:
  - blockchain
  - distributed ledger technology
  - suitability
  - Hashgraph
  - IOTA
---

## Overview

This 2021 survey paper (Hassija, Zeadally, Jain, Tahiliani, Chamola, and Gupta, published in Transactions on Emerging Telecommunications Technologies) addresses the problem that, amid blockchain hype, the technology is often proposed as a solution for nearly any application even where decentralization yields unreasonable operating cost and poor performance. Its central contribution is a decision framework — a set of use-case-oriented yes/no questions — for objectively assessing whether blockchain/DLT is appropriate for a given application domain, and if so, which type of blockchain (public, private, consortium, or hybrid) to use. The paper provides background on blockchain fundamentals (a six-layer architecture, transaction life cycle, blockchain types, and consensus algorithms), surveys application domains where blockchain is and is not suitable, and notably extends beyond blockchain to compare four alternative distributed ledger technologies (DLTs) — Hashgraph, IOTA, Nano coin, and Zcash — explaining when each is preferable to traditional blockchain. The framework is demonstrated on three case studies (supply chain, multidrone network, financial services), each resolving to a different recommended architecture. The authors position this work as filling a gap in prior literature, which they argue gave criteria for blockchain suitability but failed to address applications that need a DLT yet cannot use blockchain (e.g., those requiring timestamp ordering) and failed to discuss non-blockchain alternatives in detail.

## Background

The paper builds on a substantial body of prior work, which it cites to support its argument. It notes that prior surveys have explored blockchain applications across e-commerce, logistics networks, private data handling, smart communities, edge computing, machine learning, and deep learning, and that cryptocurrencies remain the most common application. Citing others, it states that earlier works (References 45-52) discussed scenarios for using blockchain in business, compared decision models for blockchain suitability, and gave criteria for choosing blockchain — but it argues these had specific gaps: Reference 48 focused narrowly on the insurance sector without discussing alternatives where decentralization is needed but other blockchain features are not; Reference 51 identified problems solvable at lower cost than blockchain but did not detail alternatives; Reference 52 reviewed blockchain and criteria for choosing a type but did not address applications needing a DLT that cannot use blockchain (e.g., blockchain has no facility to maintain timestamp ordering, whereas Hashgraph can). For technical background, the paper draws on the multilayer blockchain framework adapted from Reference 53, classical distributed-consensus precursors (2-phase commit, atomic broadcast, state machine replication, Byzantine fault tolerance), and PoW/PoS/BFT/Proof-of-Elapsed-Time consensus literature. It cites prior work claiming over 20 billion IoT devices in use, blockchain applications in industrial IoT, smart grids, healthcare (secure interoperability of health records and pharmaceutical supply chains), cloud outsourcing, and real estate. For alternatives it cites the Hashgraph gossip/virtual-voting protocol, IOTA's Tangle and fast probabilistic consensus with weighted votes (Zipf-law-modeled reputation/Mana), Nano coin's Block Lattice, and Zcash's zero-knowledge-proof privacy. It also cites prior work proposing a Hashgraph-based DLT for multidrone networks (Reference 132) and an IOTA-based feeless tolling architecture (Reference 130).

## Key Points

- The paper proposes a decision framework structured as a sequence of use-case-oriented questions that determines both whether blockchain is suitable and which blockchain type to use: (1) Do you need a shared database? (if no, use a traditional database); (2) Are there multiple writers? (3) Are untrusted stakeholders involved? (if only trusted parties and non-sensitive data, use a traditional database; if a trusted third party can establish consensus, use a centralized architecture); (4) Does the data need to be kept private? (if yes, private blockchain); (5) Is there any restriction on who controls the blockchain? (if no restriction, public blockchain); (6) Should the ledger be maintained by a group of selected organizations? (if yes, consortium blockchain; if only one organization, private blockchain).
- It describes blockchain operation as a six-layer framework: data layer (blocks with header/body, Merkle root, nonce, timestamp), network/propagation layer (P2P), consensus layer, incentive layer (miner rewards), contract layer (smart contracts), and application layer.
- It classifies blockchains into four types — public/permissionless, private/permissioned, consortium/federated, and hybrid — and tabulates their advantages, disadvantages, and example platforms (e.g., Bitcoin/Ethereum public; Hyperledger Fabric private; IBM Food Trust consortium; Ripple/Dragonchain hybrid).
- It enumerates blockchain's core advantages (distributed/no single point of failure, immutability/stability, trustless environment removing intermediaries) and disadvantages (low transaction speed ~10-minute block time, high data/storage requirements ~1 TB Bitcoin chain, high energy consumption comparable to a small European country, and pseudo-anonymity that can compromise privacy).
- It identifies application domains where blockchain is suitable: IoT (industrial IoT, smart grids/energy trading), healthcare (secure health-record interoperability, pharmaceutical supply chains), and business (cloud-outsourcing of untrusted third parties, real estate via smart contracts removing brokers/notaries).
- It argues blockchain is NOT suitable for cases with only trusted stakeholders, non-shared data, or where a trusted central authority exists — for which traditional or centralized databases are preferable (demonstrated by the Visa/financial-services case resolving to a centralized architecture).
- It presents four non-blockchain DLT alternatives with distinct selection criteria: Hashgraph (DAG data structure, gossip + virtual-voting/atomic-blast-broadcast consensus, BFT, proprietary/tamper-resistant, very high theoretical throughput, and uniquely supports transaction timestamp ordering); IOTA (Tangle DAG, fast probabilistic consensus with weighted votes, highly scalable, feeless IoT transactions); Nano coin/RaiBlocks (Block Lattice where each account has its own chain, delegated PoS, highest TPS, ideal for micropayments); and Zcash (traditional blockchain structure, PoW, zero-knowledge-proof privacy via shielded/transparent addresses, minimal fees but least scalable).
- It provides a comparison table of the four alternatives across consensus mechanism, data structure, scalability, privacy, transaction rate, and token requirement, showing trade-offs (Hashgraph/IOTA highly scalable but pseudo-anonymous; Zcash least scalable but privacy-protecting).
- It validates the framework on three case studies: supply chain (AgriDigital/CBH Group) resolves to a private blockchain; multidrone/UAV charging network resolves to Hashgraph because timestamp ordering is required; and Visa financial services resolves to a centralized architecture.
- It identifies open challenges and future research directions: vulnerability to quantum attacks (breaking encryption, enabling 51% attacks), scalability limits on transactions per unit time, defending against intelligent (machine-learning and game-theory-based) attacks, and reducing high computational power usage (critical for resource-constrained domains like UAV networks).

## Conclusion

The paper concludes that blockchain has great potential to transform industries through security, transparency, anonymity, and immutability, but that its suitability must be assessed case-by-case rather than assumed. Its central claim — that a structured question-based framework can objectively determine both whether and which blockchain (or which alternative DLT) to use — is supported through the three demonstrative case studies, each of which resolves cleanly to a distinct recommended architecture (private blockchain, Hashgraph, and centralized database, respectively), showing the framework can recommend against blockchain or toward a non-blockchain alternative when appropriate. The authors note that blockchain's drawbacks have been partly addressed by recent technical advances but that significant limitations remain. They explicitly flag open research questions: improving security against quantum attacks, addressing scalability, protecting against ML/game-theory-based intelligent attacks, and reducing computational power usage to enable adoption in resource-scarce settings such as drone networks. The survey itself acknowledges (implicitly, as a limitation) that some alternatives like Hashgraph are proprietary and not open-source, making their implementation and feasibility questionable.
