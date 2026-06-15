---
id: kaputa-blockchain_NFT_and_model_card__and_marketplace
title: "Kaputa - Blockchain, Non-Fungible Token and Model Card Integrated 5G/6G Network Slice Broker and Marketplace"
authors:
  - Eranga Bandara
  - Sachin Shetty
  - Ravi Mukkamala
  - Abdul Rahman
  - Peter Foytik
  - Xueping Liang
  - Wee Keong Ng
year: 2022
venue: "MILCOM 2022 - 2022 IEEE Military Communications Conference (MILCOM)"
pages: "559-564"
url: "https://api.semanticscholar.org/CorpusID:256241505"
type: inproceedings
keywords:
  - 5G
  - 6G
  - Network Slicing
  - NFT
  - Blockchain
  - Model Card
---

## Overview

This paper presents "Kaputa," a blockchain-enabled network slice broker and NFT-based marketplace for 5G/6G network slicing, designed to solve the business and engineering complexity of providing end-to-end network slices across multiple independent resource providers (cloud providers, RAN/network operators, 5G core service providers, and transport network providers). Kaputa orchestrates network slices via blockchain smart contracts (built on the "Moose" blockchain), encodes the resulting slices as a novel NFT token type called k528 (an extension of ERC721) that can be published and traded in a marketplace, and embeds data provenance information for each slice into Model Cards stored on the blockchain ledger for transparency and auditability. Customers browse the marketplace, purchase slices matching their QoS/application requirements using crypto or fiat currency, and revenue is automatically distributed among the contributing resource providers. The authors implemented a prototype using FreedomFi 5G hardware and the OpenAirInterface 5G core deployed on AWS, and evaluated blockchain performance (block generation time vs. transaction count and peer count) for clusters of 1-20 (and up to 7) peers. The paper claims to be the first work to represent and trade 5G/6G network slices as NFTs, contributing a layered architecture, the k528 NFT schema, an NFT-based slice marketplace, and Model Card-based provenance tracking.

## Background

The paper situates itself within the broader literature on 5G network slicing, citing foundational claims that slicing improves resource utilization, reduces costs, increases revenue, and improves user experience (citing Nour et al. [1]), and that slicing is enabled by network "softwarization" via virtualized/containerized network functions requiring lifecycle management (Lee et al. [2]) and resource orchestration (Zhou et al. [3]). It draws on the pre-5G framing that traditional networks required weeks of manual provisioning, causing revenue loss and customer dissatisfaction (Abbas et al. [4]), contrasted with 5G's promise of automated, on-demand E2E orchestration (Nikaein et al. [5]). The 5G Core architecture in Kaputa follows the service-based, NFV/SDN, microservices design pattern described in Brown's white paper on 5G core networks [13]. The system's underlying blockchain, "Moose," is an optimized blockchain for 5G-sliced IoT environments proposed in the authors' own prior work (Bandara et al. [7]), and the smart contract platform "Aplos" and related blockchain infrastructure (Mystiko, Rahasak, Tikiri) come from the same research group's earlier publications [16]-[21]. The k528 NFT schema is described as an extension of the ERC721 token standard (Chirtoaca et al. [10]), within the general NFT framework surveyed by Wang et al. [8] and tokenization concepts from Ciriello [9]. The identity management approach follows a self-sovereign identity model from the authors' prior ICCCN paper [15]. The Model Card concept for data provenance/transparency is adopted from Google's Model Card Toolkit work (Fang and Miao [14]; Wadhwani and Jain [19]). Finally, the paper positions itself against several related blockchain-based and centralized network slice broker/marketplace systems, including Blockchain-Slice-Broker [1], Network-Slice-Leasing [22], Slice-Market [23] (built on Hyperledger Fabric), DBNS [25], Network-Slice-Scaling [3], Slice-as-a-Service [4], NECOS [6], and Slice-Store [24].

## Key Points

- Kaputa proposes a four-layer architecture (resource provider layer, blockchain/smart contract layer, NFT marketplace layer, customer layer) that lets cloud providers, network/RAN operators, 5G core service providers, and transport network providers collaboratively lease excess resources for network slice creation.
- The platform implements four core smart contracts — Identity, Slice, Model Card, and NFT contracts — that respectively handle peer identity management (self-sovereign identity), slice creation/configuration/deployment, data provenance encoding, and NFT token generation/trading.
- The paper introduces k528, a novel NFT token schema extending ERC721, to represent 5G/6G network slices as tradable NFTs containing metadata such as name, description, slice template URI, creator, and a link to the associated Model Card.
- The paper claims to be the first work to represent, package, and manage 5G/6G network slices as NFTs.
- Kaputa implements an NFT-based marketplace where customers purchase network slices using crypto or fiat currency based on their application's QoS requirements (mMTC, eMBB, URLLC), with revenue automatically distributed among contributing resource providers via smart contracts.
- Data provenance information for each network slice is encoded into Model Cards (using the TensorFlow Model Card Toolkit) and stored on the blockchain ledger, allowing customers to audit and verify slice history before purchase.
- Network slices have a defined lifetime tied to NFT ownership; when the lifetime expires, the slice is automatically shut down, resources are released back to the pool, and the corresponding NFT token is burned.
- The system is built on the Moose blockchain (an optimized blockchain for 5G-sliced IoT environments from the authors' prior work) using the Aplos smart contract platform, with etcd for configuration storage and Apache Kafka for peer-to-peer communication and backpressure handling.
- A working prototype was implemented using FreedomFi 5G gateway/Indoor Radio Cell hardware and OpenAirInterface 5G core deployed on AWS.
- Performance evaluation shows block generation takes approximately 8.5 seconds for a block of 10,000 transactions, with additional measurements of average block creation time across varying transaction set sizes (2000, 6000, 10000) and varying numbers of blockchain peers (up to 7), each repeated 100 times and averaged.
- A comparison table positions Kaputa as the only system among reviewed platforms (including Blockchain-Slice-Broker, Network-Slice-Leasing, Slice-Market, DBNS, Network-Slice-Scaling, Slice-as-a-Service, NECOS, and Slice-Store) that simultaneously offers distributed architecture, running blockchain support (Moose), a slice marketplace, slice leasing, NFT support, and trading support.

## Conclusion

The paper concludes that Kaputa successfully addresses the stated challenge of enabling end-to-end network slicing across multiple independent resource providers by combining blockchain-based orchestration, NFT-based representation/trading of slices, and Model Card-based provenance tracking, all demonstrated via a working prototype on real 5G hardware (FreedomFi) and the OpenAirInterface core. The performance evaluation supports the feasibility of the blockchain layer at moderate scale (up to ~10,000 transactions per block and up to 7 peers), though the evaluation is fairly limited in scope — it focuses narrowly on block generation time as a function of transaction count and peer count, without broader system-level metrics such as marketplace transaction throughput, NFT minting/trading latency, end-to-end slice deployment time, economic/pricing mechanism validation, or security/attack-resilience analysis. The comparison table is largely qualitative (checkmarks for feature presence) rather than a quantitative benchmark against competing systems. The paper does not deeply address how slice pricing is determined, how disputes or fraudulent provenance claims would be handled, or how the system would scale to the larger peer counts (up to 20) mentioned in the evaluation setup versus the 7-peer experiments actually shown. The "Conclusions and Future Work" section is brief and does not enumerate specific future research directions beyond restating the system's design goals, leaving open questions for future research around larger-scale evaluation, economic mechanism design for the marketplace, interoperability with non-Moose blockchains, and real-world regulatory/legal considerations of representing telecom infrastructure resources as tradable crypto-assets.
