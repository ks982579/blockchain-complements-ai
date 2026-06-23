---
id: blockchain_and_the_future_of_the_internet__comprehensive_review
title: "Blockchain and the Future of the Internet: A Comprehensive Review"
authors:
  - Fakhar ul Hassan
  - Anwaar Ali
  - Mohamed Rahouti
  - Siddique Latif
  - Salil Kanhere
  - Jatinder Singh
  - Ala Al-Fuqaha
  - Umar Janjua
  - Adnan Noor Mian
  - Junaid Qadir
  - Jon Crowcroft
year: 2020
venue: "arXiv"
publisher: "arXiv"
doi: "10.48550/arXiv.1904.00733"
url: "https://arxiv.org/abs/1904.00733"
type: misc
keywords:
  - Blockchain
  - Decentralization
  - Network applications
  - Cryptography and Security
---

## Overview

This paper (Hassan et al., 2020) is a comprehensive survey of blockchain-based network applications and the prospects for a decentralized Internet. Its central framing is that blockchain challenges the centrally-trusted infrastructure that dominates today's Internet, advocating a migration from the network's traditional "end-to-end principle" to a "trust-to-trust principle" in which users retain control over trust decisions about their own data. The survey first builds up the technical foundations (distributed ledgers, hashing-based block chaining, transaction chains/coins, distributed consensus, smart contracts, public vs. private blockchains, digital assets/NFTs, and digital identity), then re-imagines a broad set of Internet components and application domains through the lens of decentralization, and finally catalogs the open challenges blocking mass adoption. A distinguishing feature versus prior surveys is its explicit treatment of legal and regulatory ramifications, especially the tension between blockchain immutability and the EU GDPR's "right to be forgotten." The authors position the work as a guiding reference manual for researchers and practitioners deciding whether and how to translate an existing centralized use case into a blockchain-based solution or build one from the ground up.

## Background

The paper draws heavily on prior and foundational work to support its argument. It frames the trust-to-trust design principle citing Ali's Princeton dissertation on a "new Internet," and grounds its technical exposition in Nakamoto's Bitcoin whitepaper (the first blockchain financial application, Proof-of-Work, SPV lightweight clients) and Wood/Buterin's Ethereum (smart contracts, the EVM, dApps). It cites the SHA-2/SHA-256 hash family and Rogaway-Shrimpton on hash-function properties for the immutability argument. For consensus, it builds on others' work covering Proof-of-Stake (e.g., Algorand by Chen and Micali), Proof-of-Authority (citing PoA-vs-PBFT analyses), and Practical Byzantine Fault Tolerance (Castro-Liskov) rooted in Lamport's Byzantine Generals problem. For identity it references Sovrin (Evernym), Idemix/U-Prove anonymous credentials, Camenisch-Lysyanskaya dynamic accumulators, and the Charm crypto framework. It surveys others' systems across domains: Namecoin, Blockstack and its virtualchain for decentralized naming/DNS; Blocknet (XBridge/XRouter) and Interledger (HTLAs) for inter-blockchain routing; BPIIoT, IBM Watson IoT, IBM-Samsung ADEPT, Filament, and modum.io for IoT; DECENT and SingularDTV for content distribution; Storj, Sia, Filecoin/IPFS, MaidSafe for distributed storage; Steem and reputation systems for online social networks; Guardtime's KSI, REMME, Obsidian, and Tezos for cybersecurity; and CertCoin and Merkle-Patricia-tree-based schemes for decentralized PKI. It also situates its regulatory discussion against the GDPR text, the W3C, IGF, FinCEN, ESMA, SEC/CFTC/IRS/FTC positions, Gartner's $176B value forecast, and prior surveys (Zheng et al., Yli-Huumo et al., Salah et al. on Blockchain-for-AI, Ali et al. on IoT) that it compares itself against in Table I.

## Key Points

- Blockchain advances a paradigm shift from the Internet's end-to-end principle to a "trust-to-trust" principle, giving users decentralized control over trust decisions about their own data and enabling distributed trust, consensus, and trusted/transparent auditability without a mediating third party.
- The survey's distinguishing contribution over prior blockchain surveys is its combined coverage of a wide range of network application domains plus explicit legal/regulatory analysis (notably GDPR), rather than focusing on a single use case such as IoT.
- It provides an evolutionary technical primer showing how hashing chains blocks into a tamper-evident append-only ledger, how transaction chains (coins/tokens) use digital signatures for provenance, and how distributed consensus (PoW, PoS, PoA, PBFT) trades off energy cost, throughput, and trust assumptions.
- It distinguishes public/permissionless blockchains (anonymous, e.g., Bitcoin, Ethereum) from private/permissioned ones (identity-bound, confidentiality-supporting, e.g., Hyperledger Fabric channels), and surveys digital assets, NFTs (ERC-721 vs. fungible ERC-20), land-registry digitization, and blockchain-based digital identity (Sovrin).
- It surveys decentralized alternatives across Internet components and applications: SDN/NFV control-plane decentralization, decentralized naming/DNS (Namecoin, Blockstack), inter-blockchain routing (Blocknet, Interledger), decentralized email, IoT, content distribution, distributed cloud storage, online social networks, cybersecurity, and PKI/Certificate Authorities.
- It argues blockchain interoperability is analogous to BGP/inter-AS routing and proposes that storing BGP attributes (AS prefixes, peering agreements) immutably on-chain could enable more trusted, transparent, auditable routing, while noting scalability and latency caveats.
- It analyzes how GDPR roles (data controller vs. processor) map onto blockchain deployment scenarios, conjecturing that in PoW networks the whole network or processing-power-weighted nodes act as processors, and in permissioned consortia accountability can be weighted by stake (PoS).
- It offers concrete reconciliations between blockchain immutability and GDPR's "right to be forgotten": encrypt-and-delete-the-key, store only off-chain data with on-chain hashes (with "hash peppering" to prevent reconstruction), and Hyperledger-style confidential channels that can be invalidated by deleting their cryptographic material.
- It catalogs adoption challenges: governance/operational/regulatory uncertainty (illustrated by the DAO hack draining ~$70M via a recursive-call exploit and the WannaCry ransomware attack), scalability (transaction throughput, storage growth, block-size/interval tradeoffs, Lightning Network, sharding), security/privacy concerns (selfish mining, 51% attacks, web-tracker deanonymization), sustainability/energy costs, limited anonymity, and usability/key-management difficulties.
- It identifies AI/ML and big-data analytics as complementary directions, e.g., AI-driven "intelligent oracles" and smarter smart contracts, and big-data analytics over blockchain for real-time fraud detection and trading-pattern prediction.

## Conclusion

The paper concludes that blockchain has substantial potential to drive a more decentralized, transparent, and democratic Internet by redefining digital trust through distributed consensus and tamper-evident record-keeping, and it demonstrates this breadth by mapping concrete blockchain solutions onto a wide spectrum of network applications. As a survey it does not test a single hypothesis empirically; rather it supports its thesis by aggregating existing systems and identifying recurring patterns, limitations, and trade-offs across domains. The authors are candid that blockchain remains "in its infancy" and that widespread adoption is impeded by unresolved challenges in scalability (throughput and storage), security and privacy (selfish mining, 51% attacks, weak anonymity, deanonymization), sustainability and energy consumption, usability and key management, and especially governance/regulation. They highlight several open research directions: power-efficient consensus for resource-constrained IoT devices, QoS guarantees for delay-sensitive services such as email, secure interoperability/routing among heterogeneous blockchains, standardization for cross-border legal jurisdiction, future-proof methods (against quantum computing) for reconciling immutability with GDPR's right to be forgotten, securing virtualized SDN/IoT assets, and deeper integration of AI/ML and big-data analytics. The overarching takeaway is offered as a reference manual to help researchers and practitioners make informed, sustainable decisions about adopting blockchain-based solutions.
