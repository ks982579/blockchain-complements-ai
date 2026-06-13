---
id: blockchain_for_federated_learning_toward_ml__systemic_survey
title: "Blockchain for federated learning toward secure distributed machine learning systems: a systemic survey"
authors:
  - Dun Li
  - Dezhi Han
  - Tien-Hsiung Weng
  - Zibin Zheng
  - Hongzhi Li
  - Han Liu
  - Arcangelo Castiglione
  - Kuan-Ching Li
year: 2022
venue: "Soft Computing"
volume: 26
issue: 9
pages: "4423-4440"
doi: "10.1007/s00500-021-06496-5"
url: "https://doi.org/10.1007/s00500-021-06496-5"
type: article
---

## Overview

This paper is a systemic survey of "Blockchain-based federated learning" (BCFL), a paradigm formed by integrating Blockchain technology with Federated Learning (FL) to address FL's structural weaknesses. FL allows distributed clients to train a shared model without sharing raw data, but it depends on a central aggregation server, creating trust issues, single points of failure (SPOF), DDoS vulnerability, lack of transparent contribution evaluation, and no robust mechanism for detecting malicious nodes. The authors argue Blockchain can resolve these issues via its decentralized ledger, consensus mechanisms (e.g., PoW), and smart contracts, while also enabling incentive payments proportional to training contribution. The paper's contribution is to organize the (until now unsystematized) BCFL literature from 2016–2021 into four thematic areas: (1) architectural design and deployment platforms for BCFL, (2) performance-improvement techniques for BCFL models, (3) incentive mechanism designs for FL participants using Blockchain, and (4) industrial application domains for BCFL across nine sectors. It positions itself as the first dedicated survey of this combined paradigm, distinct from prior surveys that treat FL or Blockchain separately.

## Background

The paper builds heavily on foundational FL literature, citing Google's original FL proposals (Konecny et al. 2016a/b; McMahan et al. 2017) and the FedAvg algorithm as the canonical FL training method, as well as broader FL surveys (Kairouz et al. 2019; Yang et al. 2019; Bonawitz et al. 2019; Li et al. 2020; Mothukuri et al. 2021) that the authors use as comparison points to argue no prior work addresses the Blockchain+FL combination systematically. For Blockchain background, it draws on established overviews of Blockchain architecture, consensus, and security (Zheng et al. 2017, 2018; Liang et al. 2020, 2021; Xiao et al. 2020; Gramoli 2020), describing the standard layered model (data, incentive, consensus, network, application layers) and the public/consortium/private chain taxonomy along with consensus mechanisms (PoW, PoS, DPoW, DPoS, PBFT, dBFT). It cites Kim et al. (2018) as the first work to propose the core BCFL architectural concept (using Blockchain to address private exchange and reward mechanisms in FL), with follow-on contributions from Mugunthan et al. (2020, "BlockFLow"), Ramanan et al. (2020, "Baffle"), Korkmaz et al. (2020, "ChainFL"), Kim et al. (2020, "BlockFL"), and Zhang et al. (2020, a Hyperledger Fabric-based demo). The survey also references existing open-source FL frameworks (TensorFlow Federated, PySyft, FATE, PaddleFL, FedML) as the practical software ecosystem underpinning FL implementations, and Ethereum's smart contract model (Buterin) as the enabling technology for deploying FL logic on-chain.

## Key Points

- The paper formally defines and names the "Blockchain-based federated learning" (BCFL) paradigm and claims to be the first systematic survey covering its structural design, deployment platforms, performance improvements, incentive mechanisms, and industrial applications (2016–2021).
- It categorizes FL itself into three types based on data partitioning—horizontal FL (HFL), vertical FL (VFL), and federated transfer learning (FTL)—and frames BCFL research against this taxonomy.
- It identifies a common reference architecture for BCFL (originating with Kim et al. 2018) in which Blockchain functions as a decentralized, tamper-resistant central database/coordination layer for FL, handling reward distribution and validation of local model updates.
- It surveys and compares four deployment platforms used for BCFL—Ethereum (public, PoW/PoS, used by BlockFLow, BAFFLE, ChainFL), Hyperledger Fabric (consortium, SOLO/Kafka consensus, used in Zhang et al.'s demo), EOS (consortium, DPoS/BFT, used by Martinez et al.'s framework), and Custom Blockchains (private, PBFT, used in BlockFL and Lu et al.'s dual-module system)—each offering different tradeoffs in performance, security, and customizability.
- It compiles evidence that BCFL can improve FL performance along three axes: fault tolerance and classification accuracy (e.g., ChainFL on MNIST/CIFAR-10), handling of non-IID data via federated augmentation (FAug, Jeong et al. 2018), and computational/communication efficiency (replacing oracle services with chaincode, Drungilas et al. 2021; dynamic weighting of client contributions, Kim and Hong 2019).
- It documents security-improvement approaches in BCFL, including re-encryption schemes based on ElGamal cryptography (CrowdSFL, Li et al. 2020) and reputation-based consensus improvements for filtering unreliable workers (ReliableFL, Kang et al. 2020), framed as defenses against SPOF, DDoS, and poisoning attacks.
- It surveys incentive mechanism research for BCFL across three sub-problems: handling "lazy" clients who don't contribute meaningfully (Blade-FL, Li et al. 2020/2021), assessing client contribution quality (Ma et al. 2021; Martinez et al. 2019; Toyoda et al. 2020), and effective reward/profit-allocation schemes (Shapley-value-based allocation by Liu et al. 2020; dataset-fraction-based reward schemes by Cai et al. 2020).
- It catalogs nine industrial application domains for BCFL—healthcare data processing, network security anomaly detection, IoT device failure/anomaly detection, Internet of Vehicles, 5G/6G secure communication, intelligent edge computing, fog computing, cognitive computing, and sustainable-society/defense applications—summarizing representative studies and the specific benefits (privacy, communication efficiency, robustness, incentives) claimed for each.

## Conclusion

The survey concludes that combining Blockchain and FL is a promising research direction because it strengthens data security and privacy while enabling new application scenarios that require joint modeling without raw data sharing, supporting the paper's central premise that BCFL can address FL's trust, incentive, and reliability gaps. However, the authors explicitly note their survey is largely qualitative and descriptive: it does not employ cross-referencing or quantitative bibliometric analysis to measure research trends, leaving this as an open task for future work. They also acknowledge their categorization may not capture all relevant perspectives and call for broader classification schemes in future surveys. Finally, they note that industrial applications of BCFL remain in early/theoretical stages relative to FL's already-established theoretical foundations, and recommend future research focus on expanding comparative studies of BCFL across more industrial fields and application effects.
