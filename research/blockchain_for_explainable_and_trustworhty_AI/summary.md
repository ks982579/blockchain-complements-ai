---
id: blockchain_for_explainable_and_trustworhty_AI
title: "Blockchain for explainable and trustworthy artificial intelligence"
authors:
  - Mohamed Nassar
  - Khaled Salah
  - Muhammad Habib ur Rehman
  - Davor Svetinovic
year: 2019
venue: "Wiley Interdisciplinary Reviews: Data Mining and Knowledge Discovery"
publisher: "Wiley"
volume: 10
doi: "10.1002/widm.1340"
url: "https://doi.org/10.1002/widm.1340"
type: article
keywords:
  - Blockchain
  - Consensus
  - Prediction models
  - Reputation
  - Smart contract
---

## Overview

This 2019 focus article (WIREs Data Mining and Knowledge Discovery) by Nassar, Salah, Rehman, and Svetinovic addresses the dual problem of black-box AI's lack of explainability and its vulnerability to bias and adversarial attacks, proposing blockchain as the missing trust layer for Explainable AI (XAI). The paper's central contribution is a blockchain-based framework in which multiple AI and XAI "predictors" (acting as trusted oracles) independently compute predictions and explanations for the same task, and a decentralized consensus mechanism (via smart contracts) aggregates these outputs into a final, auditable decision. The framework defines "technical trustworthiness" as a measurable property arising from explainability, multi-predictor consensus, and accumulated reputation, rather than relying on trust in any single AI system or its developers. It specifies concrete architectural components — registration, reputation, aggregator, and AI-task smart contracts, paired with decentralized storage (IPFS) — and an economic incentive model (staking/deposit and reward/penalty in Ethereum) to keep predictors honest. The paper situates this proposal within a broader survey of existing XAI taxonomy (interpretable models, visual interpretation, feature contribution, surrogate models, counterfactual/anchor explanations) and adversarial ML literature, then illustrates applicability through several real-world use cases (medical diagnosis, customer profiling, tax/fraud auditing, election fraud detection, and future real-time autonomous-vehicle scenarios).

## Background

- The paper situates its motivation in the regulatory landscape, citing the EU GDPR (Regulation, 2016) and its "right to explanation" for automated decisions (e.g., credit refusals, e-recruiting) as a driver for XAI research.
- It builds on DARPA's Explainable AI (XAI) initiative (Gunning, 2016) as the origin of the XAI research trend, motivated by the proliferation of adversarial attacks on deep learning (Papernot et al., 2016).
- The adversarial ML background draws on a substantial body of prior work: dataset poisoning and adversarial examples (Szegedy et al., 2013; Kurakin, Goodfellow, & Bengio, 2016), one-pixel attacks (Su, Vargas, & Kouichi, 2017), 3D-printed adversarial objects (Athalye & Sutskever, 2017), adversarial patches (Brown et al., 2017), and black-box attacks not requiring model access (Papernot et al., 2017; Biggio & Roli, 2018).
- The XAI taxonomy presented (interpretable/intrinsic models, visual interpretation methods like PDP/M-plot/ALE/ICE, feature contribution via Shapley values (Strumbelj & Kononenko, 2014; Datta, Sen, & Zick, 2016), surrogate models like LIME (Ribeiro, Singh, & Guestrin, 2016a/b), and counterfactual/anchor explanations (Wachter, Mittelstadt, & Russell, 2017)) is synthesized from existing XAI literature, including a noted limitation called the "Rashomon effect" (inconsistent explanations for the same prediction).
- The paper explicitly defers to and builds on prior work by some of the same authors — Salah, Rehman, Nizamuddin, & Al-Fuqaha (2019), "Blockchain for AI: Review and open research challenges" — for a fuller taxonomy of blockchain-AI integration, treating that paper as the foundational reference and this one as an applied extension toward XAI specifically.
- It cites Hasan & Salah (2018, 2019) for prior blockchain mechanisms it reuses or adapts: a proof-of-delivery/staking deposit mechanism for incentivizing honest oracle behavior, and a blockchain/smart-contract approach to combating deepfakes as a precedent for blockchain-verified AI outputs.
- Decentralized storage design draws on IPFS (Benet, 2014) as the mechanism for storing large prediction metadata and explanation artifacts off-chain while anchoring content hashes on-chain.
- The framing of trustworthiness draws on Thompson's "Reflections on Trusting Trust" (1984) (trusting people over software) and broader ethical AI discussions (Dignum, 2017, on responsible autonomy; Keeling, 2019, on trolley problems for autonomous vehicles; Sileno, Boer, & van Engers, 2018, on normware for trustworthy/explainable AI).
- The paper also notes a skeptical counterpoint from the deep learning community, citing Geoffrey Hinton's view (via Simonte, 2018) that demanding explanations from neural networks may be misguided, analogous to humans being unable to truly explain their own cognitive processes.

## Key Points

- This paper proposes a general blockchain-based framework that combines smart contracts, trusted oracles, and decentralized storage to make AI decision-making more trustworthy and explainable by replacing reliance on a single black-box predictor with consensus among multiple independent AI/XAI predictors.
- It defines "technical trustworthiness" as an objective, qualitative measure of confidence assignable to an AI system's output, distinct from broader ethical/moral notions of trustworthiness, and argues this technical layer is a prerequisite for implementing higher-level ethical/philosophical AI behavior.
- The paper introduces a taxonomy of XAI methods (intrinsic, visual, surrogate, example-based/counterfactual, feature-set, gradient-based) summarized in a comparative table of strengths and limitations, used to motivate the need for a consensus-based trust layer.
- It specifies a three-tier participant model: frontend users (DApp users requesting AI predictions/explanations), Predictors with AI only (PAI), and Predictors with Explanations (PXAI), each potentially running different algorithm types (deterministic vs. nondeterministic/probabilistic).
- The architecture is decomposed into a frontend DApp layer and a backend with four modules: AI access layer (Web3/JSON-RPC/REST/JMS/SOAP interfaces), AI layer (PAI and PXAI predictor pools), support services (decentralized registration and reputation services), and the blockchain platform (smart contracts plus decentralized storage such as IPFS).
- The framework defines four smart contract types — Registration SC, AI-task SC, Aggregator SC, and Reputation SC — with the Aggregator SC responsible for collecting predictor outputs (as IPFS hashes), determining majority-consensus decisions, distributing cryptocurrency rewards/penalties, and updating reputation scores.
- The paper proposes an economic incentive mechanism based on staked Ethereum deposits: predictors that report results agreeing with the majority recover double their deposit (reward plus deposit return), while dishonest or non-conforming predictors lose their stake, creating a built-in disincentive against collusion or false reporting.
- It identifies and proposes a solution to the "exactness problem": since blockchain consensus requires deterministic, identical outputs across mining nodes, the framework requires frontend DApps to specify interpretation rules (e.g., accuracy ranges/intervals) in the SLA so that probabilistic/nondeterministic AI/XAI outputs can be mapped to discrete, consensus-compatible values.
- The paper articulates a cost-trust tradeoff: increasing the number of predictors improves majority-based mitigation of collusion/dishonesty but increases the cost of leasing additional prediction nodes.
- It demonstrates the framework's applicability through five use cases — decentralized medical image diagnosis (multi-lab radiology consensus), customer credit/risk profiling for banking/insurance, government tax auditing and money-laundering/fraud detection, election/voting fraud detection, and (as a forward-looking case) real-time autonomous vehicle accident analysis, pending improvements in blockchain latency/scalability.

## Conclusion

The paper concludes that its proposed blockchain-based framework offers a plausible architectural path toward more resilient, trustworthy, and explainable AI by combining multi-predictor consensus, immutable audit trails, decentralized storage, and economic incentives for honesty — directly addressing both the black-box explainability gap and susceptibility to bias/adversarial manipulation that motivated the work. However, the paper is explicitly conceptual/architectural rather than empirically validated: no implementation, prototype, or quantitative evaluation is presented, so its claims about reduced bias, improved trust, and resilience remain hypotheses to be validated in future work. The authors acknowledge significant open challenges, most notably: (1) minimizing the need for human-in-the-loop validation of explanations, since the consensus mechanism still presumes human or DApp-defined interpretation rules for converting probabilistic outputs into deterministic, consensus-compatible decisions; and (2) real-time performance, since blockchain's current latency and scalability limitations make the framework currently unsuitable for time-critical applications like autonomous vehicle decision-making, pending future improvements in blockchain technology. The paper also flags broader unresolved infrastructure requirements — security, privacy, reliability, usability, dependability, and governance — as necessary complements to the proposed consensus/reputation/incentive mechanisms, leaving these as open research directions. Additionally, the authors note an unresolved tension within the XAI field itself (citing Hinton's skepticism) about whether deep learning systems can ever produce faithful explanations at all, which the framework sidesteps via consensus and reputation rather than resolving directly.
