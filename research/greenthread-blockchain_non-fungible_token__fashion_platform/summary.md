---
id: greenthread-blockchain_non-fungible_token__fashion_platform
title: "GreenThread — Blockchain, Non-Fungible Token(NFT), Model Cards, Self-Sovereign Identity and IPFS Enabled Sustainable Circular Fashion Platform"
authors:
  - Nadini Sahabandu
  - Eranga Bandara
  - Peter Foytik
  - Sachin Shetty
  - Ravi Mukkamala
  - Abdul Rahman
  - Xueping Liang
year: 2023
venue: "2023 Annual Modeling and Simulation Conference (ANNSIM)"
publisher: "Society for Modeling & Simulation International (SCS)"
pages: "357-243"
url: "https://api.semanticscholar.org/CorpusID:259278935"
type: inproceedings
keywords:
  - Blockchain
  - NFT
  - Model Cards
  - IPFS
  - Self-Sovereign Identity
  - Circular fashion
---

## Overview

This paper presents "GreenThread," a blockchain-based platform designed to support circular fashion — the practice of reusing, upcycling, and recirculating textile materials and garments to reduce waste. The core problem addressed is that circular fashion adoption is hampered by a lack of trust: buyers cannot verify the provenance claims (source materials, manufacturing process, upcycling history) attached to items, and participants lack adequate privacy, anonymity, and security guarantees when interacting on a shared platform. GreenThread's contribution is an integrated architecture combining permissioned blockchain smart contracts, Model Cards (structured provenance metadata objects) stored on IPFS, Non-Fungible Tokens (NFTs) representing physical fashion items in a marketplace, and Self-Sovereign Identity (SSI)-enabled mobile wallets for anonymous yet verifiable participant identities. The platform is built on the authors' prior Rahasak blockchain and Aplos smart-contract framework, and the paper reports microservice-based implementation details along with throughput, latency, and block-generation performance evaluations across varying numbers of blockchain peers and transaction volumes. The work positions itself within a small but growing body of blockchain-for-circular-economy research and argues that GreenThread is differentiated by combining SSI, Model Cards, NFTs, and high scalability in a single fashion-domain platform — features that comparable systems (CE-Leather, CE-Built, CSCM, CE-Smart, Dresscode) lack in combination.

## Background

The paper situates its motivation in industry projections from the Ellen MacArthur Foundation (cited via Yoo, Jung, Oh, et al. 2021 and Vehmas et al. 2018), which estimate circular fashion business models could represent a $560 billion economic opportunity and grow from 3.5% to 23% of the global fashion market by 2030, with resale/rental markets (e.g., Depop, Rent the Runway) already exceeding $73 billion. It draws on Yoo et al. (2021) for the framing of upcycling as a "design-based circular fashion approach" and for consumer motivators/barriers around upcycled product purchasing. Technically, the paper builds on several of the authors' own prior systems: the Rahasak blockchain (Bandara, Liang, Foytik, Shetty, Ranasinghe, De Zoysa, et al. 2021) as the underlying ledger, the Aplos smart-contract framework (Bandara, Ng, Ranasinghe, and Zoysa 2019) built on Scala/Akka actors (Odersky et al. 2004; Gupta 2012), and the Bassa-ML Model Card framework for ML provenance (Bandara, Shetty, Rahman, Mukkamala, Zhao, Liang, et al. 2022). The Model Card concept itself derives from the TensorFlow Model Card Toolkit (Wadhwani and Jain 2020a/b). For SSI, the paper relies on prior surveys and frameworks of self-sovereign identity (Mühle, Grüner, Gayvoronskaya, and Meinel 2018; Baars 2016; Liu, Sun, and Schuckers 2019) and the authors' earlier healthcare decentralized-identity work (Liang, Shetty, Zhao, Bowden, Li, Liu, et al. 2017). For decentralized storage, it cites IPFS (Benet 2014) and Libp2p pubsub for peer-to-peer communication. For NFTs, it references general NFT overview/evaluation literature (Wang, Li, Wang, Chen, et al. 2021; Chohan and Paschen 2021; Dowling 2022). Finally, the related-work section reviews and benchmarks against five other blockchain-for-circular-economy systems: CE-Leather (Shou and Domenech 2022, integrating Life Cycle Assessment with blockchain for leather handbags), CE-Built (Shojaei et al. 2021, blockchain for circular economy in the built environment), CSCM (Wang, Luo, Zhang, Tian, Li, et al. 2020, blockchain supply-chain management for fast fashion), CE-Smart (Kumar and Chopra 2022, smart contracts for circular economy trust/governance), and Dresscode (Heim and Hopper 2022, NFT/smart-contract digital transformation of circular fashion supply chains).

## Key Points

- GreenThread proposes an architecture with four layers — Resource Provider (Stakeholder), Blockchain and Smart Contract, NFT Marketplace, and Customer layers — to coordinate designers, material providers, manufacturers, transport agents, and buyers in a circular fashion ecosystem.
- The platform encodes end-to-end data provenance and audit information (circularity of source materials, manufacturing steps, upcycling processes, and ownership-transfer history) into Model Card objects stored on IPFS, with IPFS hashes linked to NFT tokens on the blockchain ledger, providing verifiable proof of circularity claims.
- The paper proposes a novel, extensible NFT token schema for representing circular fashion items, encoding metadata such as name, description, slice template URI, creator, and a link to the associated Model Card provenance object, generated as JSON tokens.
- GreenThread implements an NFT-based marketplace where manufactured circular fashion items are tokenized, traded for crypto or fiat currency, and physically delivered via transport providers, with a revenue-sharing model distributing proceeds among designers, material providers, manufacturers, and transport agents.
- The platform employs SSI-enabled mobile wallet applications so that participants' real identities are stored locally on-device while only SSI-Proofs are recorded on the blockchain, aiming to guarantee anonymity, security, and data privacy for all stakeholders.
- GreenThread defines four core smart contracts — Identity, Coordination, Model Card, and NFT contracts — that respectively handle peer identity/permissions management, manufacturing/logistics coordination, provenance encoding via the Model Card service, and NFT token creation/trading.
- The system is implemented as a microservices architecture on the Rahasak blockchain using the Aplos (Scala/Akka) smart-contract framework, the TensorFlow Model Card Toolkit for provenance objects, and IPFS/Libp2p pubsub for decentralized storage and peer-to-peer communication.
- Performance evaluations show that query (read) transactions achieve higher throughput and lower latency than invoke (write/mutation) transactions; both throughputs scale roughly linearly with the number of blockchain peers, while latency decreases slightly as peer count increases.
- The evaluation reports that generating a block containing 10,000 transaction records takes approximately 8.5 seconds, with block generation time varying based on both transaction count and number of peers; a reactive streaming-based backpressure strategy is used to manage submitted-vs-executed transaction rate mismatches.
- A comparative feature table positions GreenThread as the only surveyed platform (versus CE-Leather, CE-Built, CSCM, CE-Smart, and Dresscode) to simultaneously offer high scalability, high privacy via SSI, Model Card-based auditing/provenance, and NFT support within the fashion domain.

## Conclusion

The paper concludes that GreenThread successfully demonstrates an integrated platform meeting its stated design goals: enhanced privacy, transparency, traceability, anonymity, and data provenance for circular fashion at scale, achieved through the combination of blockchain smart contracts, NFT-based trading, SSI-enabled identity, and IPFS-stored Model Card provenance records. The performance evaluation supports claims of scalability — throughput scales linearly with peer count and latency does not degrade (and slightly improves) as peers are added — though the reported figures (e.g., 8.5 seconds to produce a block of 10k transactions) are presented descriptively without comparison against baseline blockchain systems or formal scalability bounds, so the practical significance of these numbers relative to alternatives is not established. The comparison table against five related circular-economy/fashion blockchain systems is largely qualitative (checkmarks for feature presence) rather than empirically benchmarked, leaving open how GreenThread's claimed advantages translate into measurable real-world benefits. The authors explicitly note that future work involves deploying GreenThread into real customer use cases in the fashion industry — meaning the system as described has not yet been validated with actual fashion-industry stakeholders, end-to-end supply chains, or real consumer adoption, which represents the primary open research direction flagged by the paper.
