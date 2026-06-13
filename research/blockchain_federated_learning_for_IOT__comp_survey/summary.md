---
id: blockchain_federated_learning_for_IOT__comp_survey
title: "Blockchained Federated Learning for Internet of Things: A Comprehensive Survey"
authors:
  - Yanna Jiang
  - Baihe Ma
  - Xu Wang
  - Guangsheng Yu
  - Ping Yu
  - Zhe Wang
  - Wei Ni
  - Ren Ping Liu
year: 2024
venue: "ACM Computing Surveys"
publisher: "Association for Computing Machinery"
volume: 56
issue: 10
pages: "Article 258"
doi: "10.1145/3659099"
type: article
keywords:
  - Federated learning
  - blockchain
  - blockfl
  - internet of things
---

## Overview

This paper is a comprehensive survey (ACM Computing Surveys, 2024) of "Blockchained Federated Learning" (BlockFL) — the integration of Blockchain technology with Federated Learning (FL) — applied across the Internet of Things (IoT). It addresses the problem that traditional centralized ML and plain FL face privacy, trust, single-point-of-failure, and reliability limitations, and that Blockchain alone faces scalability and energy issues, motivating their combination. The paper's distinctive contribution is organizing the BlockFL literature around four IoT application domains — Personal IoT (PIoT), Industrial IoT (IIoT), Internet of Vehicles (IoV), and Internet of Health Things (IoHT) — and analyzing existing models across four cross-cutting dimensions: security/privacy, trust/reliability, efficiency, and data diversity. It synthesizes dozens of specific BlockFL model proposals (e.g., FedAC, FL-Block, PermiDAG, ChainsFL, BAFL, VFChain) into comparison tables, derives asymptotic cost models (storage, communication, computation) comparing FL, Blockchain, and BlockFL, and identifies both common and domain-specific open challenges, while also proposing integration with split learning, transfer learning, and continuous learning as future directions.

## Background

The paper situates itself relative to FL's origin with McMahan et al.'s FedAvg (2016/2017), noting known limitations addressed by variants such as FedPAQ (Reisizadeh et al.), FedProx (Li et al.), and FedMA (Wang et al.), which target communication efficiency and non-IID data heterogeneity. It draws on Blockchain fundamentals from Nakamoto's Bitcoin paper and subsequent Blockchain-for-IoT literature (e.g., Makhdoom et al., Wang et al.'s capacity analyses, Yu et al.'s sharding and Proof-of-X surveys) to establish Blockchain's properties of decentralization, immutability, auditability, and autonomy via smart contracts, and the public/private/consortium taxonomy. It cites prior BlockFL-related surveys (Issa et al., Nguyen et al., Qu et al., Zhu et al., Ali et al., Javed et al., Saraswat et al.) to differentiate its scope — arguing those works either treat FL and Blockchain separately, focus on theoretical analysis without IoT-domain specificity, or cover only one or two IoT domains, whereas this survey spans four domains with a unified analytical framework. It also draws on known FL/BlockFL vulnerabilities documented elsewhere, such as honest-but-curious server and GAN-based inference attacks (Aono et al., Hitaj et al.), poisoning and Byzantine attacks (Bhagoji et al.), and consensus-based attacks (Halgamuge), as well as the BlockFL system architecture concept from Kim et al. (2019), which the paper adopts as its canonical BlockFL process model (local learning + Blockchain-based integrated calculation with miner verification, scoring, and leader election).

## Key Points

- The paper provides a four-domain taxonomy (PIoT, IIoT, IoV, IoHT) for classifying BlockFL research, arguing this is more granular than prior surveys that either ignore IoT-domain distinctions or cover only one or two domains.
- It formalizes a comparative cost model (Table 2) deriving Big-O storage, communication, and computation costs for FL, Blockchain, and BlockFL separately, showing BlockFL inherits FL's local-training efficiency but adds Blockchain-driven overhead from peer verification and consensus.
- It compiles a comparative table (Table 3) of technical features — data sharing mode, FL architecture (centralized/distributed), synchronization, chain structure, permission model, and consensus protocol — across ten named BlockFL models (e.g., Autonomous BFL, BAFL, ChainsFL, FedAC, FL-Block, Hierarchical BlockFL, MAS BlockFL, PermiDAG, Secure Data Sharing Scheme, VFChain).
- It characterizes each IoT domain's distinguishing requirements (Table 4): PIoT (medium-high security, high device diversity, large volumes of small data), IIoT (high security, low device diversity, closed networks), IoV (high security, dynamic networks, real-time requirements), IoHT (high security/privacy, heterogeneous data, identity management challenges).
- For security and privacy (Section 4), the paper concludes BlockFL provides stronger, more scalable protection than FL or Blockchain alone without centralized servers, citing domain examples such as BC-based PPFL (homomorphic encryption via Paillier variant), FDC (multiparty secure computation), BlockFedML, BlockFL data models with differential privacy, Block Hunter, two-stage IDS for IoV, BDFL, and IoHT-specific frameworks meeting six identified privacy/security requirements (per Passerat-Palmbach et al.).
- For trust and reliability (Section 5), the paper argues Blockchain's immutability/auditability enables reputation-based and verification mechanisms (e.g., VFChain, reputation metrics by Kang et al., FLchain) that improve robustness against poisoning/Byzantine attacks and outperform centralized verification frameworks like VerifyNet that remain vulnerable to single-point attacks.
- For efficiency (Section 6), the paper identifies that BlockFL's dual overhead (FL training + Blockchain block generation) requires domain-specific optimization — e.g., smart-contract-based aggregator-free designs (BAFFLE), block-generation-rate control (BAFL, FL-Block), gradient/communication compression (PAFLM, decentralized FEL), and hierarchical chain structures for IoV (Hierarchical BlockFL, drone-based BlockFL).
- For data diversity (Section 7), the paper documents that BlockFL leverages Blockchain-based incentive mechanisms (reputation, token rewards, fairness/punishment schemes) to attract diverse participants and data, citing models such as DeepChain, DAM-SE, BlockFLA, FGFL, BPFL, BESIFL, and a COVID-19 CT-imaging model using data normalization for heterogeneous medical data.
- The paper claims that combining cryptography, anomaly detection, optimization, compression, and data normalization techniques with Blockchain components (Table 9) represents the key technological pathway for advancing BlockFL.
- It identifies general open challenges common across all domains: privacy/security, resource constraints (limited compute/energy on IoT devices), scalability, and data heterogeneity, exacerbated/altered by 5G/6G ultra-low-latency requirements.
- It identifies domain-unique challenges and proposed solutions (Table 10): complex personalized smart services in PIoT (solved via transfer/split learning), complex intelligent collaboration in IIoT (transfer learning), high latency in dynamic IoV networks (continuous/online learning), and identity/permission management in IoHT (consortium Blockchains plus split learning).
- The paper proposes — as a contribution not covered elsewhere — integrating Split Learning, Transfer Learning, and Continuous Learning frameworks with BlockFL to improve scalability, cross-domain knowledge transfer, and adaptability in dynamic IoT environments.

## Conclusion

The survey concludes that BlockFL's decentralization and transparency make it a more secure and effective approach for distributed model training than traditional FL across all four IoT domains studied (PIoT, IIoT, IoV, IoHT), supporting its central claim that combining FL and Blockchain yields complementary benefits. However, it explicitly notes that overhead (computational, storage, communication) and compatibility with existing systems remain insufficiently studied, and that empirical evidence shows BlockFL convergence speed "slightly lags behind" traditional FL, indicating room for optimization. The paper finds that while common challenges (privacy, resource constraints, data heterogeneity, scalability) cut across all domains, each domain has unique pressure points — e.g., IoV's need for dynamic-environment adaptation and low-latency decision-making, and IoHT's stringent identity/permission management demands due to regulatory and sensitivity concerns. It notes that most existing studies remain largely theoretical or simulation-based rather than deployed at scale, and that current BlockFL designs underuse FL's potential to improve Blockchain consensus itself (a bidirectional benefit largely unexplored). Open research directions flagged for future work include: optimizing existing BlockFL models for practical deployment, designing new Blockchain-FL models balancing high privacy with high accuracy, exploring new consensus mechanisms and smart contract designs informed by FL model quality, addressing miner collusion and hybrid-Blockchain structure issues, and integrating split learning, transfer learning, and continuous learning to handle personalization, cross-domain knowledge sharing, and adaptation to dynamic/6G network environments.
