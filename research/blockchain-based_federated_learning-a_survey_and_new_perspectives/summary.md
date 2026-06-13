---
id: blockchain-based_federated_learning-a_survey_and_new_perspectives
title: "Blockchain-Based Federated Learning: A Survey and New Perspectives"
authors:
  - Weiguang Ning
  - Yingjuan Zhu
  - Caixia Song
  - Hongxia Li
  - Lihui Zhu
  - Jinbao Xie
  - Tianyu Chen
  - Tong Xu
  - Xi Xu
  - Jiwei Gao
year: 2024
venue: "Applied Sciences"
publisher: "MDPI"
volume: 14
issue: 20
pages: "9459"
doi: "10.3390/app14209459"
type: article
keywords:
  - blockchain
  - federated learning
  - blockchain-based federated learning
  - distributed machine learning
---

## Overview

This paper is a comprehensive survey of blockchain-based federated learning (BFL), a hybrid paradigm that integrates blockchain's decentralized ledger technology with federated learning (FL) to address FL's structural weaknesses. Federated learning, introduced by Google in 2016, enables collaborative model training across distributed clients while keeping raw data local, but it suffers from single points of failure (a centralized aggregation server), vulnerability to poisoning and free-riding attacks, lack of auditability/transparency, and an absence of robust incentive mechanisms for participants. The authors argue that blockchain's properties—decentralization, immutability, transparency, consensus mechanisms, smart contracts, and cryptographic primitives (hashing, digital signatures)—directly address these gaps. The survey's core contributions are: (1) a systematic literature review based on a content-analysis methodology applied to 135 selected papers (from an initial pool of 756) drawn from 2014–2024; (2) a novel taxonomy classifying BFL systems by the specific blockchain technology employed (smart contracts, consensus algorithms, digital signatures, hash algorithms, peer-to-peer networking); (3) a problem-solution mapping across seven FL challenge categories (incentive, defense, privacy, robustness, global/architecture, transmission, audit mechanisms); and (4) a cross-domain survey of BFL applications (IIoT, Internet of Vehicles, healthcare, digital currency, UAVs/B5G, digital twins, fog computing, traffic prediction). The paper positions itself as filling a gap left by prior surveys, which it characterizes as either too narrow in scope or insufficiently systematic in describing the technical integration of blockchain and FL.

## Background

The paper situates itself within a body of established work on both federated learning and blockchain that it draws on to motivate BFL. It cites McMahan et al. (2017) and Konecny et al. (2016) as the foundational origin of federated learning at Google, and AbdulRahman et al. (2020) and Wen et al. (2022) for general surveys characterizing FL's privacy benefits and challenges. Nakamoto's (2008) Bitcoin whitepaper is cited as the origin of blockchain technology and its decentralized ledger model. The survey explicitly contrasts itself with prior BFL surveys—Hou et al. (2021) and Li et al. (2021)—which it criticizes for having "few references and insufficient content" and lacking systematic descriptions of how the two technologies combine, as well as Issa et al. (2022) and Liu et al. (2022), whose surveys focus narrowly on IoT security and digital twins respectively. It draws heavily on specific prior BFL system proposals to illustrate its taxonomy and application categories, including: BlockFLA (Desai et al. 2021) for accountable FL via hybrid blockchain; FLChain (Bao et al. 2019) for auditable, incentive-based FL markets; SABlockFL (Zhang et al. 2020) for agent-based blockchain-FL integration; DeepChain (Weng et al. 2021) for value-driven incentives and auditability; BlockFL (Kim et al. 2020) for on-device blockchained FL; Biscotti (Shayan et al. 2020) for fully decentralized P2P privacy-preserving ML; BLADE-FL (Li et al. 2022) for decentralized aggregation; BAFL (Feng et al. 2022) for asynchronous blockchain FL; and BAFFLE (Ramanan & Nakayama 2020) for aggregator-free FL. It also references foundational work on privacy-preserving techniques such as differential privacy and homomorphic encryption (e.g., Wang et al.'s BPFL, Salim et al.'s DP-BFL) and broader FL challenge literature such as Kairouz et al.'s "Advances and Open Problems in Federated Learning" (2021) as context for ongoing security and incentive issues that BFL aims to mitigate.

## Key Points

- The paper conducts a systematic literature review using a content-analysis coding methodology, screening 756 initial papers down to a final sample of 135 papers (2014–2024) sourced from Google Scholar, CNKI, Web of Science, and IEEE Xplore, with classification performed and validated by seven researchers across multiple rounds.
- It presents a structured comparison of blockchain and federated learning along dimensions of architecture category, key technology, technical nature, data storage, authentication mechanism, and application target, concluding that both share distributed structure, equal node participation, and privacy risks, but differ fundamentally in data consistency/redundancy versus data privacy/complementarity.
- It identifies and details three core categories of drawbacks in traditional FL: privacy protection failures (privacy leakage, poisoning attacks), incentive mechanism deficiencies (lack of motivation, improper reward distribution), and robustness/efficiency issues (single-point failure, lack of defense/censoring capability, robustness against malicious/lazy devices, and network overload).
- It proposes a novel taxonomy of BFL models organized around five core blockchain technologies—smart contracts, consensus algorithms, digital signatures, hash algorithms, and peer-to-peer networking—and maps each to specific solved problems, representative systems (e.g., BlockFLA, FLChain, SABlockFL, DeepChain, BlockFL, BLADE-FL, BAFL, ChainFL, Biscotti), and resulting technical characteristics.
- It articulates a "threefold mutual benefit" argument (drawing on Li et al. 2021) for why blockchain and FL are complementary: similar multi-party cooperative architectures, shared security/privacy design goals, and complementary value-creation (FL) versus value-delivery/reward (blockchain) roles.
- It organizes FL's challenge-solving capabilities under blockchain into seven distinct mechanism categories—incentive, defense, privacy, robust, global, transmission, and audit mechanisms—each mapped to specific blockchain technologies, application scenarios, and representative solutions in a dedicated comparison table.
- It surveys cross-domain applications of BFL spanning Industrial IoT (failure detection, data privacy, model tamper-resistance), Internet of Vehicles (message dissemination, knowledge sharing, privacy), healthcare (electronic health records, COVID-19 CT imaging via the BFL framework), digital currency (transaction-amount privacy via homomorphic encryption with CORDA), Beyond-5G/UAV edge intelligence, digital twin edge networks, fog computing (FL-Block), and traffic flow prediction (BFRT).
- It highlights specific incentive-design innovations enabled by blockchain, including reputation-based worker selection (Kang et al.), contract-theory-based incentives for healthcare (Lim/Niyato et al.), the Proof-of-Shapley (PoSap) consensus-based payment system (FedCoin, Liu et al.), and contribution-point-based fairness mechanisms (Liu et al.), contrasting these with prior single-dimension incentive schemes (Feng et al.) that only considered training data size.
- It identifies three open/unsolved technical problems specific to BFL: improving overall system efficiency (via decentralized federated slicing or DRL-based node selection), reducing communication overhead from FL's iterative process (via distributed learning frameworks or digital twin reinforcement learning for spectrum scheduling), and ensuring on-chain data privacy despite blockchain immutability (proposing chameleon-hash-based redactable/editable blockchains as a partial solution).
- It frames a forward-looking discussion around four themes—security, incentive design, efficiency, and the comparative advantages of smart contracts versus consensus algorithms—arguing smart contracts enable decentralized model aggregation and reward-protocol transparency, while consensus algorithms (including emerging committee-based mechanisms) improve node selection, robustness, and single-point-of-failure resistance.

## Conclusion

The paper concludes that blockchain-based federated learning is a maturing and increasingly adopted paradigm that effectively addresses many of traditional FL's core deficiencies—single points of failure, insufficient incentive structures, weak defense against poisoning/malicious actors, and lack of auditability/transparency—by leveraging blockchain's decentralization, immutability, smart contracts, and consensus mechanisms. The survey's findings support its motivating hypothesis: across the 135 reviewed papers, blockchain integration consistently improves privacy protection, robustness, fairness in reward distribution, and system transparency in FL deployments across domains such as IIoT, Internet of Vehicles, and medical systems. However, the paper is explicit that BFL is not a fully solved problem—it notes that current BFL frameworks still face efficiency trade-offs (e.g., uncertainty whether model compression and slicing techniques can balance accuracy with reduced overhead), unresolved communication-cost issues from FL's iterative training rounds, and a fundamental tension between blockchain immutability and the need for privacy-preserving, editable on-chain storage of model updates. The authors also note that many BFL approaches cannot fully prevent privacy leakage on their own and should be combined with complementary techniques like differential privacy and homomorphic encryption. As open research directions, the paper calls for continued work on privacy/security protection mechanisms, global (server-replacement) architectures, transmission/resource-allocation mechanisms, and fairness, robustness, and personalization in federated learning—framing BFL as a still-evolving field requiring periodic re-review as new prototypes and consensus/incentive designs emerge.
