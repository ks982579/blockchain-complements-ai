---
id: a_survey_on_blockchain-based_federated_learning
title: "A Survey on Blockchain-Based Federated Learning"
authors:
  - Lang Wu
  - Weijian Ruan
  - Jinhui Hu
  - Yaobin He
year: 2023
venue: "Future Internet"
publisher: "MDPI"
volume: 15
issue: 12
pages: "400"
doi: "10.3390/fi15120400"
url: "https://doi.org/10.3390/fi15120400"
type: article
keywords:
  - blockchain
  - federated learning
  - security and privacy
  - Internet of Things
---

## Overview

This paper is a 2023 survey examining the integration of blockchain technology with federated learning (FL), a field the authors term blockchain-based federated learning (BFL). The work addresses a gap in prior BFL surveys, which the authors argue focused mainly on describing BFL frameworks and classifications without deeply analyzing the specific FL problems that blockchain integration solves. The paper provides background on FL and blockchain fundamentals, categorizes existing BFL architectures into three types based on how the blockchain participates in the FL workflow, and analyzes BFL's mitigation of five core FL challenges: decentralization (removing single points of failure), incentive mechanisms, attack resistance (especially poisoning attacks), privacy protection, and efficiency enhancement. It further surveys BFL applications across IoT, Industrial IoT (IIoT), Internet of Vehicles (IoV), and smart healthcare, and closes with a discussion of open challenges around privacy, scalability/efficiency, and security in BFL systems. The contribution is primarily a structured, comparative synthesis intended to orient researchers toward unresolved problems at the FL-blockchain intersection.

## Background

The paper situates FL as introduced by McMahan et al. (2017), describing it as a distributed ML paradigm where clients train locally and a central server aggregates model updates without collecting raw data, motivated by data silo problems and regulations like the GDPR (citing Albrecht 2016). It draws heavily on prior work documenting privacy attacks against FL gradient updates: Carlini et al. extracted sensitive memorized data (e.g., bank card numbers) from recurrent neural networks; Fredrikson et al. demonstrated model inversion attacks recovering patient dosage information; Hitaj et al. used GAN-based attacks on model aggregation to steal client data; and Geiping et al. showed gradient information alone can reconstruct input images regardless of network architecture. The paper also cites communication-efficiency concerns in FL (Konečný et al., McMahan et al.) and the non-IID data challenge relative to traditional distributed learning (Wang et al., CMFL). On the blockchain side, it builds on Nakamoto's original Bitcoin design (2008) and Zheng et al.'s six-layer blockchain architecture framework (data, network, consensus, incentive, contract, application layers), as well as the "impossible triangle" concept of consensus tradeoffs (decentralization, scalability, security) attributed to Wang et al. The paper positions itself against six prior BFL surveys (Toyoda & Zhang; Hou et al.; Wahab et al.; Nguyen et al.; Issa et al.; Li et al.), each characterized as either framework-classification-focused or domain-specific (e.g., mobile edge computing, IoT), arguing none deeply analyzed the specific FL problems BFL resolves across broader application domains. Throughout, the survey synthesizes dozens of individual BFL framework proposals (e.g., BlockFL, DeepChain, FLChain, CrowdSFL, PermiDAG, BFEL, Biscotti, PriModChain) as evidence for its categorization and functional analysis.

## Key Points

- This paper proposes a three-way categorization of BFL frameworks based on the blockchain's role: (1) blockchain directly replaces the central aggregation server (the most common early approach), (2) blockchain forms part of a multi-level/hybrid architecture (e.g., permissioned main chain plus local DAGs or sub-chains) alongside the FL process, and (3) blockchain serves an auxiliary role without directly participating in FL training/aggregation (e.g., data-sharing ledger, reputation storage, or trustworthy data verification).
- The paper argues BFL addresses five core FL weaknesses—single point of failure, lack of incentive mechanisms, poisoning attacks, privacy policy defects, and low communication efficiency—and organizes its functional analysis of the literature around these five corresponding capabilities: decentralization, incentive mechanisms, attack resistance, privacy protection, and efficiency enhancement.
- It claims decentralization-focused BFL designs (e.g., FLChain's channel-based ledgers, multi-layer/multi-chain structures, PriModChain's combination of differential privacy with Ethereum smart contracts, and hybrid permissioned blockchain/DAG architectures) substantively reduce the security risk posed by a single central aggregator.
- The paper asserts that blockchain-based incentive mechanisms (reputation scoring via subjective logic, contract theory-based rewards, weight-based client selection, DRL-based pricing strategies) provide a practical solution to FL's lack of native incentives for honest participation.
- It claims consensus-based verification mechanisms (committee consensus, proof-of-validation, multi-Krum, reputation-based rejection schemes, decentralized voting) deployed in BFL effectively mitigate poisoning attacks by filtering malicious model updates before they reach the global model.
- The paper documents that privacy-preserving BFL designs combine blockchain immutability with supplementary cryptographic techniques—homomorphic encryption (Martinez et al.), verifiable secret sharing (Biscotti/Shayan et al.), local differential privacy (Lu et al., Qi et al., Zhang et al.'s ADPFedAvg), and PKI-based encryption (Mahmood et al.)—because blockchain alone does not guarantee data privacy.
- It claims efficiency-enhancement techniques in BFL (off-chain distributed hash tables with on-chain pointers, asynchronous FL with node selection via DPoS, committee consensus reducing verification scope, gradient compression, hyperparameter optimization with elastic weight consolidation) are necessary to offset the overhead blockchain integration introduces.
- The paper compiles application case studies across four domains—IoT (data sharing, PoQ consensus, 6G integration), IIoT (fault detection, federated averaging variants for non-IID data), smart healthcare (Ethereum-based alliances, COVID-19 diagnosis models, IoMT security), and IoV (vehicular trust, asynchronous FL, knowledge sharing)—as well as emerging areas including content caching, location prediction, UAV-assisted crowdsensing, disaster response, and news recommendation.
- The paper claims BFL research remains largely theoretical and immature, with most frameworks addressing only the trust/single-point-of-failure problem while leaving privacy, efficiency, and security issues comparatively underexplored.

## Conclusion

The survey concludes that BFL is a "nascent" interdisciplinary field that has made meaningful progress in solving FL's trust and single-point-of-failure problems by leveraging blockchain's decentralization, immutability, and incentive structures, but that this integration introduces new problems inherited from blockchain itself. It identifies three open challenge areas: (1) privacy concerns, since public ledgers expose FL training data and metadata to all participants, and private ledgers that mitigate this trade off accuracy and accessibility, with differential privacy and homomorphic encryption offered as partial but costly solutions; (2) efficiency, performance, and scalability limitations, noting even leading blockchain platforms (Bitcoin ~4 tx/s, Ethereum ~12 tx/s) are far below conventional systems (VISA's millions of tx/s), with encryption overhead, PoW costs, and large-model storage compounding FL's existing communication burden—sidechains and newer consensus platforms (Algorand, IOTA) are noted as partial mitigations; and (3) security concerns, including vulnerability to 51% attacks, mining-power centralization undermining decentralization, and lack of provable security for newly proposed consensus algorithms used in BFL (e.g., ACK-based fork detection). The paper presents no new experimental results of its own (it is a literature survey), so its "findings" are qualitative syntheses rather than empirically tested hypotheses. It explicitly flags that many proposed BFL frameworks are incomplete or purely theoretical, calling into question their practical applicability, and frames the development of provably secure, scalable, and privacy-preserving BFL consensus mechanisms as the central open research direction for the field.
