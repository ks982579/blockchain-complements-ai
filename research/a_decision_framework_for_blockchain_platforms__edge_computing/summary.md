---
id: a_decision_framework_for_blockchain_platforms__edge_computing
title: "A Decision Framework for Blockchain Platforms for IoT and Edge Computing"
authors:
  - Claus Pahl
  - Nabil El Ioini
  - Sven Helmer
year: 2018
venue: "Proceedings of the 3rd International Conference on Internet of Things, Big Data and Security (IoTBDS 2018)"
publisher: "SCITEPRESS"
pages: "105-113"
doi: "10.5220/0006688601050113"
url: "https://research.ebsco.com/linkprocessor/plink?id=04ce35f2-2597-3183-8ceb-434f012a46bc"
type: inproceedings
keywords:
  - Blockchain
  - Internet of Things
  - Edge Computing
---

## Overview

This paper by Pahl, El Ioini, and Helmer (Free University of Bozen/Bolzano, IoTBDS 2018) presents a decision framework to help practitioners systematically evaluate whether and how to adopt blockchain technology in Internet of Things (IoT) and edge computing contexts. The authors argue that the prevailing centralized client/server architecture with trusted brokers (SSL/TLS) becomes a bottleneck as IoT device counts grow, and that blockchains can address IoT-specific challenges in security, privacy, fault tolerance, and autonomous behavior. The core contribution is a two-part framework: a flowchart that determines (a) whether a blockchain is warranted and (b) which class of platform (public permissionless, public permissioned, or private) fits, plus a comparison table evaluating six concrete platforms (Bitcoin, Ethereum, Hyperledger Fabric, Ripple, Multichain, Eris) across eight properties (scalability, consensus, network size, anonymity, fee, block size, smart contract support, security) rated for IoT suitability. The framework is validated against three real IoT companies (Filament, Slock.it, Telit), where the framework's recommendations align with the companies' actual technology choices. The work targets the gap that prior decision frameworks address general environments rather than IoT specifically.

## Background

The paper builds on a body of prior work that it cites to support its argument. It frames blockchains as combining three established technologies (public key cryptography, distributed peer-to-peer networks, and consensus mechanisms) blended in a novel way (citing Antonopoulos 2014, Cachin 2016, Bashir 2017). It draws blockchain platform taxonomy (public permissionless, public permissioned, private) from sources including Thompson (2015). For IoT challenges, it leans on Gubbi et al. (2013), who identified numerous IoT application challenges, of which the authors focus on three. It cites Bartoletti and Pompianu (2017) for smart contracts as self-executing programs encapsulating business logic, and Asharaf and Adarsh (2017) for the fault-tolerance benefits of peer-to-peer architectures. On consensus, it references Baliga (2017) and notes Ethereum's planned Proof of Stake migration and zkSNARK collaboration with zcash (citing Blum et al. 1988 for non-interactive zero-knowledge). For platform comparison properties it draws on Macdonald et al. (2017). Crucially, the framework is explicitly built atop prior decision-oriented studies and real use cases: Wüst and Gervais (2017) ("Do you need a blockchain?"), Xu et al. (2016) treating blockchains as software connectors, and Conoscenti et al. (2016) on blockchain for IoT. In its related work, the paper additionally positions itself against Xu et al. (2017), which presents a taxonomy of blockchain-based systems. The paper says prior decision frameworks cover general environments rather than IoT specifically, which is the gap it claims to fill.

## Key Points

- Proposes a two-stage decision framework for IoT/edge contexts: an upper flowchart deciding *whether* to use a blockchain and a lower flowchart deciding *which platform class* to use.
- The "when to use" decision rests on three criteria: (1) multiple parties are involved, (2) there is interaction between those parties, and (3) no trusted third party is available; a blockchain adds value only when all three hold (otherwise it merely adds overhead).
- The platform-class choice keys on user anonymity: anonymous participants who do not know each other favor a public permissionless blockchain (e.g., Bitcoin); participants who know each other favor a permissioned blockchain (higher throughput, faster consensus), with the public/private split determined by whether public verifiability or public read access is needed.
- Identifies three IoT-specific challenges blockchains can address: confidentiality/integrity (via immutable cryptographically verifiable shared data and transaction validation), autonomous behavior (via tamper-resistant smart contracts encoding device logic), and fault tolerance (via peer-to-peer resilience to node failure).
- Provides a comparative rating table of six platforms across eight properties, asserting concrete tradeoffs, e.g., Bitcoin is one of the largest yet least scalable networks (PoW plus 1MB block limit yields low throughput, least favorable smart-contract and block-size ratings); Hyperledger Fabric handles ~100,000 tx/sec and Ripple ~1,000 tx/sec; permissioned/private platforms generally rate most favorable for IoT on scalability, block size, and security.
- Asserts transaction fees are higher for public blockchains (financial incentive for consensus) and largely negotiable a priori for permissioned ones; Bitcoin and Multichain lack native smart-contract support while Ethereum (Turing-complete Solidity/EVM) and Eris (Ethereum VM) provide it.
- Validates the framework on three companies: Filament (multi-party, no trusted third party → uses Bitcoin, chosen for maturity despite limitations, storing contract data in OP_RETURN), Slock.it (anonymous parties needing smart contracts → uses Ethereum, with whisper messages to reduce fees), and Telit (single/trusted organization → correctly predicted not to use blockchain).

## Conclusion

The authors conclude that deciding whether to use a blockchain in IoT is genuinely difficult because blockchains trade trust, security, and privacy gains against overhead and performance costs, and that platform selection is a non-trivial follow-on question. They report that applying the framework to three use cases demonstrated its efficacy: in two cases (Filament, Slock.it) pre-existing evidence of actual blockchain adoption matched the framework's recommendation, and in the third (Telit) the framework's "no blockchain" verdict matched reality while also flagging that Telit could potentially benefit from blockchain for added security and traceability. The paper thus claims its hypotheses were supported by the use-case validation. Acknowledged limitations and open questions include: the framework is coarse and intended to be refined into a recommender tool covering a wider range of categories; security needs in particular should be subdivided (e.g., sensor data provenance as a distinct IoT concern); and broader validation is constrained because few IoT blockchain application scenarios are properly documented. Future work centers on expanding the framework into this recommender tool and analyzing additional, better-documented use cases.
