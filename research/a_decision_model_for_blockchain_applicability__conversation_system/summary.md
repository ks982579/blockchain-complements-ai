---
id: a_decision_model_for_blockchain_applicability__conversation_system
title: "A decision model for blockchain applicability into knowledge-based conversation system"
authors:
  - Wenli Yang
  - Saurabh Garg
  - Zhiqiang Huang
  - Byeong Kang
year: 2021
venue: "Knowledge-Based Systems"
publisher: "Elsevier B.V."
volume: 220
pages: "106791"
doi: "10.1016/j.knosys.2021.106791"
url: "https://doi.org/10.1016/j.knosys.2021.106791"
type: article
keywords:
  - Blockchain platform
  - Conversation system
  - Knowledge-based
  - Decision model
  - Multiple measurements
---

## Overview

This paper (Yang, Garg, Huang & Kang, *Knowledge-Based Systems* 220, 2021) addresses how to select the most suitable blockchain platform for a knowledge-based conversation system whose knowledge base is maintained collaboratively by multiple human experts. The authors argue that current conversation systems rely on centralized servers, which create problems for trusted knowledge sharing and tamper-proof contribution assignment among experts, and that blockchain can provide decentralized, immutable, traceable storage to address these gaps. Their core contributions are threefold: (1) a structured requirement analysis of multi-expert knowledge-based conversation systems mapped to concrete blockchain design aims; (2) the reformulation of blockchain platform selection as a multi-criteria decision-making (MCDM) problem organized around four decision-item groups — decentralization architecture, storage and sharing, computing performance, and scalability; and (3) a decision model that combines three weighting/ranking measurements — AHP, Fuzzy AHP (FAHP), and Fuzzy TOPSIS (FTOPSIS) — with a consistency-checking step to produce a robust, agreed-upon platform recommendation. Applying the model with opinions from ten AI experts (collected via the Delphi method on a 0.1–0.9 scale) across Ethereum, Hyperledger Fabric, Corda, and MultiChain, the study concludes that Hyperledger Fabric is the best-fitting platform for this application.

## Background

The paper builds on prior work in two areas that it cites to motivate and ground its own method. For evaluating and selecting blockchain platforms, it cites Dinh et al.'s Blockbench benchmarking framework for private blockchains, Hileman and Rauchs' Cambridge global blockchain benchmarking study, Maple and Jackson on selecting effective blockchain solutions, Kuo et al.'s systematic review of platforms for healthcare/biomedical use, and Farshidi et al.'s decision-support method for blockchain selection with industry case studies in finance, education, and healthcare. The authors note (citing these) that benchmarking studies rely on documentation/reports that quickly go out of date, and that existing MCDM models are tuned to specific domains and may yield inconsistent results depending on which single method is chosen. For the decision-making machinery, the paper draws on established techniques from other authors: Saaty's Analytic Hierarchy Process (AHP) and its nine-point pairwise comparison scale; Chang's triangular-fuzzy-number extension that yields Fuzzy AHP; and Hwang and Yoon's TOPSIS method for order preference by similarity to an ideal solution. It also cites prior applications of these methods to blockchain-adjacent problems (e.g., Maček and Alagić's AHP evaluation of Bitcoin, Karayazi and Bereketli's Fuzzy AHP for blockchain software selection, and TOPSIS-based blockchain assessments in healthcare and Internet-of-Vehicles). On the technology side, the paper relies on cited claims about blockchain providing anonymity, safety, and data integrity, the centralized/partial/decentralized trust-architecture taxonomy, on-chain vs. off-chain storage trade-offs, consensus families (PoW, PoS, PBFT, etc.), and on-chain vs. off-chain scaling solutions such as SegWit, Lightning Network, Raiden, and Plasma. The Delphi method (Linstone and Turoff) is cited as the basis for eliciting expert applicability ratings.

## Key Points

- The paper identifies three core requirements of a multi-expert knowledge-based conversation system — content reliability and confidentiality, immediacy and accurate response, and being open-ended and extensible — and translates each into specific blockchain design aims (secure storage, decentralized contribution assignment, consensus among experts, fast search/match, fair incentive schemes, and capacity for expansion).
- It models blockchain platform selection as a multi-criteria decision-making problem with four decision-item groups: decentralization architecture, storage and sharing, computing performance, and scalability.
- It defines a four-level applicability scale (very inappropriate, inappropriate, appropriate, very appropriate) and provides design-decision tables rating architectures, storage modes, consensus protocols, incentive schemes, and scaling methods against criteria such as network performance, deployment cost, attack resistance, fault tolerance, processing speed, flexibility, and concurrent capacity.
- It proposes combining three MCDM measurements (AHP, FAHP, and FTOPSIS) with a weighted-average composition and a consistency-checking rule (|W − n| ≤ 0.5) to resolve conflicts and obtain a consistent recommendation, arguing that multiple measurements overcome the inconsistency of any single method.
- Using ten AI experts' Delphi-derived ratings, it builds membership matrices for Ethereum, Hyperledger Fabric, Corda, and MultiChain based on their technical features.
- It finds that consensus protocol, chain structure, and storage-and-sharing are consistently the three most important criteria across all three methods, with consensus protocol ranked first by all of them.
- It reports that all three methods agree on the alternative rankings (with only minor weight differences between AHP and FAHP attributable to fuzzy vs. crisp calculation), and that Hyperledger Fabric is the consistent best-fitting platform, followed by Corda, then Ethereum and MultiChain.
- It presents the framework as standardized and updatable, so new platforms or new decision methods can be slotted into the same modeling steps as the market evolves.

## Conclusion

The authors conclude that their multi-measurement decision model successfully assists decision-makers in selecting a blockchain platform aligned with the requirements of a knowledge-based conversation system, and that combining AHP, FAHP, and FTOPSIS with consistency checking yields a robust, agreed recommendation: Hyperledger Fabric is identified as the best-fitting platform among the four evaluated, owing largely to its consortium-chain partial-centralization architecture, off-chain storage, multi-chain support, and flexible incentive options. The claim that multiple measurements produce consistent results is supported — the three methods agreed on alternative rankings and on the top criteria. The paper explicitly notes limitations: the analysis depends on a relatively small pool of expert opinions who may lack knowledge to assess every criterion, and the chosen design factors/criteria could be extended with additional criteria and sub-criteria. The authors frame the recommendation as provisional and updatable as new platforms enter the market. For future work, they plan to actually implement the conversation system's knowledge base on Hyperledger Fabric — using its multi-chain structure to store domain knowledge with differentiated membership/authorities, generating and validating knowledge rules via smart contracts for transparency and tamper-resistance, and adding a reputation-based reward scheme to motivate experts — leaving the detailed chain structure, smart-contract design, and incentive mechanism as open research and engineering tasks.
