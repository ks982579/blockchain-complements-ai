---
id: a_survey_on_blockchain-enabled_federated_learning_and__twin
title: "A survey on blockchain-enabled federated learning and its prospects with digital twin"
authors:
  - Kangde Liu
  - Zheng Yan
  - Xueqin Liang
  - Raimo Kantola
  - Chuangyue Hu
year: 2024
venue: "Digital Communications and Networks"
volume: 10
issue: 2
pages: "248-264"
doi: "10.1016/j.dcan.2022.08.001"
url: "https://www.sciencedirect.com/science/article/pii/S2352864822001626"
type: article
keywords:
  - Digital twin
  - Artificial intelligence
  - Federated learning
  - Blockchain
---

## Overview

This paper is a survey that maps the intersection of three technologies — Digital Twin (DT), Federated Learning (FL), and Blockchain Technology (BT) — with the central thesis that BT can fix FL's structural weaknesses (single-point failure, lack of trustworthy supervision, weak incentives), and that a more robust FL is in turn the missing ingredient for deploying AI-driven DT at scale without the "data island" problem. The authors note that no prior survey adequately covers blockchain-enabled FL across application domains while also connecting it to DT, since existing surveys either restrict themselves to edge computing (Nguyen et al.) or treat FL and BT separately in IoT without considering blockchain-based FL or DT integration (Ali et al.). To fill this gap, the paper (1) derives a general architecture for "DT supported by blockchain-enabled FL" from the reviewed literature, (2) proposes seven evaluation requirement categories (security, fault-tolerance, fairness, efficiency, cost-saving, profitability, support for heterogeneity) with finer-grained sub-requirements, (3) classifies ~25 reviewed systems into four functional categories based on the role BT plays in FL, evaluates each against the requirements in a summary table, and (4) identifies open issues and proposes future research directions, particularly around DT-FL integration and decentralized FL (DFL).

## Background

The paper builds its framing on several established lines of work: Federated Learning itself originates from McMahan et al.'s (Google) communication-efficient learning of deep networks from decentralized data, which the authors describe as the foundational FL paradigm of training a shared global model without centralizing raw data. Blockchain Technology is traced to Nakamoto's Bitcoin whitepaper (2008), with the survey explaining BT mechanics (Merkle trees, PoW, longest-chain rule, consensus, immutability/decentralization/anonymity/traceability) largely through the Bitcoin example. Digital Twin concepts are drawn from sources such as Lu et al. and Mostafa et al., framing DT as a virtual representation that enables real-time analysis and risk-free experimentation for IoT/network management. The paper situates itself against two specific prior surveys — Nguyen et al. (blockchain-enabled FL in edge computing) and Ali et al. (BT and FL applications in IoT, treated separately) — arguing both leave a gap this survey fills. For known blockchain weaknesses (low throughput, DDoS, privacy, deanonymization), the paper points readers to other surveys (Zheng et al. on blockchain overview; Xie et al. and Putz/Pernul on scalability/security; Yang et al. on edge-assisted solutions; Liu et al. on ML-based solutions) rather than addressing them itself. Several specific cryptographic/ML techniques referenced as building blocks for the reviewed systems include Verifiable Random Functions, multi-krum Byzantine-robust aggregation (Blanchard et al.), Differential Privacy, Shamir secret sharing, IPFS for off-chain storage (Benet), polynomial commitments (Kate et al.), FoolsGold/RONI for sybil mitigation (Fung et al.), and convergence results for FedAvg on non-IID data (Li et al., Zhao et al.).

## Key Points

- This paper proposes a general reference architecture for "DT supported by blockchain-enabled FL," defining roles for virtual objects, task publishers, trainers, Consensus Nodes (CNs), and miners, and an 8-step workflow from task publication through local training, virtual-object construction, model upload/verification, aggregation, and consensus-based block appending.
- This paper introduces seven evaluation requirement categories — Security (PAR, VCAR, TCAR, IAR), Fault-Tolerance (FT, relevant to PoW forking), Fairness (SDF, IDF, LMF, GMF), Efficiency (VE, CE, AE), Cost-Saving (SCS, LTCS), Profitability (Pr), and Support for Heterogeneity (SRH, SDH) — as a structured framework for comparing blockchain-enabled FL systems.
- This paper provides a four-category functional taxonomy of blockchain-enabled FL systems: (1) secure/tamper-proof maintenance of data, (2) training process coordination, (3) introduction of incentives to trainers, and (4) trainer behavior supervision, and systematically evaluates ~25 surveyed systems (e.g., Zhao et al., Lu et al., Li et al., Chai et al., Shayan et al. "Biscotti", Kang et al., Kim et al., Wang et al., Toyoda et al., Weng et al. "DeepChain", Fan et al.) against the proposed requirements in a consolidated comparison table.
- This paper distinguishes public from permissioned blockchain-enabled FL in terms of node roles and rights: in public blockchain-enabled FL, nodes can take on arbitrary roles and leave the FL task freely without restriction, whereas in permissioned blockchain-enabled FL, node roles are preassigned with constrained rights and nodes cannot leave arbitrarily.
- This paper identifies that almost no surveyed system (only 2 of ~25) satisfies Inferior Data Fairness (IDF) — i.e., comprehensive punishment of malicious trainers — indicating incentive mechanisms in the literature are reward-heavy and punishment-light.
- This paper finds that only three reviewed papers (Lu et al. [32], Cui et al. [34], Feng et al. [47]) address Local Training Cost-Saving, meaning communication latency is largely unaddressed despite being a major bottleneck for FL on resource-constrained trainers.
- This paper argues existing poison-attack detection methods (e.g., multi-krum) implicitly assume IID data, making them unreliable at distinguishing genuinely malicious updates from legitimate Non-IID contributions — a problem that DT-based approaches do not solve, since virtual models inherit the Non-IID nature of their physical counterparts.
- This paper observes that DT integration with blockchain-enabled FL is still nascent, citing only Lu et al.'s two papers ([1] and [32]) as pioneering this combination, and notes that both expose raw training data or local model updates through virtual objects, indicating privacy preservation has not yet been reconciled with DT-FL integration.
- This paper notes that all reviewed blockchain-enabled FL works address centralized FL (a central aggregation role replaced by miners/CNs), while Decentralized FL (DFL) — where nodes share and aggregate updates peer-to-peer — remains unexamined in combination with BT and DT.
- This paper proposes eight forward-looking research directions: (a) local/global model fairness via similarity-detection or watermarking against update-theft and plagiarism, (b) comprehensive incentive mechanisms combining rewards with proportional, fairness-calibrated penalties/deposits, (c) system-preference-driven balancing of privacy vs. practicability, (d) communication-cost reduction via model compression and efficient DT creation, (e) Non-IID-compatible system design distinguishing Non-IID from malicious updates, (f) trust-management-based evaluation of task publisher reliability, (g) varied virtual-object creation strategies tailored to different DT-FL purposes (e.g., DP-protected vs. resource-allocation-oriented), and (h) combining DT and BT with DFL for transparent, supervised peer-to-peer training.

## Conclusion

The survey concludes that blockchain-enabled FL is an active and partially mature research area — many systems successfully address single-point failure, poison-attack resistance, and basic incentive design — but that no existing system comprehensively satisfies all seven proposed requirement categories simultaneously, and DT integration with blockchain-enabled FL remains "in its infancy," supported by only a handful of works. The paper's hypothesis that BT can meaningfully strengthen FL is broadly supported by the reviewed evidence (decentralization removes single points of failure, immutability/traceability enable auditability and reputation-based incentives), but the survey identifies eight concrete open problems: weak model fairness (plagiarism of local/global models), absent or unfair punishment mechanisms (IDF), unresolved privacy-vs-practicability trade-offs, neglected communication cost/latency, poor support for Non-IID data without harming model generalization, lack of any mechanism to evaluate or police task publisher reliability, immature and privacy-leaking DT-FL integration, and complete absence of blockchain/DT support for decentralized FL. These gaps directly motivate the paper's proposed future directions (fair access fees, watermarking, trust management for publishers, model compression, DT-assisted communication reduction, and DT/BT-supported DFL), positioning the survey as a roadmap rather than a closed body of findings — the authors explicitly frame the field as requiring substantial further research before practical, large-scale DT deployments backed by blockchain-enabled FL become feasible.
