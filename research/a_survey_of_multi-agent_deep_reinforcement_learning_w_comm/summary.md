---
id: a_survey_of_multi-agent_deep_reinforcement_learning_w_comm
title: "A survey of multi-agent deep reinforcement learning with communication"
authors:
  - Changxi Zhu
  - Mehdi Dastani
  - Shihan Wang
year: 2024
venue: "Autonomous Agents and Multi-Agent Systems"
publisher: "Springer Nature"
volume: 38
issue: 1
pages: "4"
doi: "10.1007/s10458-023-09633-6"
url: "https://doi.org/10.1007/s10458-023-09633-6"
type: article
keywords:
  - Multi-agent reinforcement learning
  - Deep reinforcement learning
  - Communication
  - Survey
---

## Overview

This is a 2024 survey (Zhu, Dastani, and Wang, Utrecht University, published in *Autonomous Agents and Multi-Agent Systems*) of multi-agent deep reinforcement learning with communication (Comm-MADRL), the subfield where agents learn *when*, *with whom*, *what*, and *how* to communicate as part of solving domain-specific multi-agent tasks rather than as a predefined protocol. The authors argue that existing surveys cover too few works and use categorizations too narrow (e.g., classifying only by whether messages are broadcast, targeted, or networked) to distinguish the many recent approaches that share similar assumptions. Their central contribution is a structured classification framework of **9 dimensions** spanning three components of a Comm-MADRL system: problem settings (Controlled Goals, Communication Constraints, Communicatee Type), communication processes (Communication Policy, Communicated Messages, Message Combination, Inner Integration), and training processes (Learning Methods, Training Schemes). They project **41 Comm-MADRL models** (from DIAL/RIAL/CommNet through recent AAMAS/AAAI/NeurIPS/ICML work) into this multi-dimensional space, derive trends, and propose novel research directions by combining underexplored dimension values. The framing motivates communication as the key mechanism for tackling MARL's two core problems: partial observability and non-stationarity.

## Background

The paper grounds itself in standard MARL formalisms drawn from prior work, principally the Partially Observable Stochastic Game (POSG) of Hansen et al., and its reductions (Dec-POMDP, Dec-MDP, Multi-agent MDP, POMDP), which it extends to POSG-Comm / Dec-POMDP-Comm by adding a shared message space. It recaps value-based methods (decentralized Q-learning, DQN of Mnih et al., value decomposition via VDN/QMIX/QTRAN/DOP) and policy-based methods (policy gradient theorem and REINFORCE per Sutton & Barto, actor-critic, and MADDPG of Lowe et al.) as the algorithmic substrate. It positions itself against earlier surveys it cites as too limited: Stone & Veloso and Busoniu et al. (treat communication as predefined or a future opportunity); Hernandez-Leal et al., Nguyen et al., and Papoudakis et al. (briefly review early Comm-MADRL, frame communication as a non-stationarity remedy); Oroojlooy-Jadid & Hajinezhad (broader but no categorization); Zhang et al. and Yang et al. (theoretical/networked consensus view); and especially Gronauer & Diepold (Broadcasting/Targeted/Networked) and Wong et al., which the authors say capture too few aspects. It distinguishes its scope from the *emergent language* literature (Lazaridou & Baroni, Lewis/referential games under cheap-talk settings), claiming overlap only at the intersection it terms "learning tasks with emergent language." It also draws on adjacent concepts it suggests importing: trust and reputation in MAS, fairness in resource allocation, multimodal machine learning (Poklukar et al.), Gumbel-softmax gradient estimation, and robust RL.

## Key Points

- The paper proposes a 9-dimensional classification framework for Comm-MADRL: Controlled Goals, Communication Constraints, Communicatee Type, Communication Policy, Communicated Messages, Message Combination, Inner Integration, Learning Methods, and Training Schemes — each tied to a distinct system-design question.
- It distinguishes "learning tasks with communication" (primary goal: solving domain tasks via information exchange) from "emergent language" (primary goal: learning a symbolic, human-like language), and explicitly includes their intersection ("learning tasks with emergent language") in the survey's scope, unlike prior surveys.
- It focuses specifically on *explicit, learnable, and dynamic* communication (separate messages influencing action policies at both training and execution), excluding implicit communication via domain actions or training-only gradient passing.
- Controlled Goals categorize works as Cooperative (global or local rewards), Competitive, or Mixed; the survey finds the field is dominated by cooperative settings, with non-cooperative communication largely unexplored.
- Communication Constraints split into Unconstrained vs. Constrained (Limited Bandwidth, Corrupted Messages); the survey notes most works ignore realistic constraints like cost, delay, and noise.
- Communicatee Type is classified as Agents-in-the-MAS (Nearby Agents or Other Agents) vs. Proxy (a virtual coordinating agent that does not affect the environment).
- Communication Policy is classified as Predefined (Full Communication, Partial Structure) or Learnable (Individual Control, Global Control), capturing how communication links/graphs are formed.
- Communicated Messages are categorized as Existing Knowledge (observations, histories, policies, typically RNN/MLP/CNN/GNN-encoded) vs. Imagined Future Knowledge (intentions, policy fingerprints, model-based future plans).
- Message Combination is Equally Valued (concatenation, averaging, summing) vs. Unequally Valued (handcrafted pruning, attention weights, or neural aggregation), with attention mechanisms being the dominant means of weighting.
- Inner Integration is Policy-level, Value-level, or both, depending on whether messages feed an actor, a critic/Q-function, or both; the survey notes a trend toward policy-level integration tied to the rise of actor-critic methods.
- Learning Methods for the communication protocol are Differentiable (backprop, often using Gumbel-softmax for discrete actions), Supervised (label-defined valuable communication), Reinforced (reward-driven), or Regularized (e.g., mutual-information/entropy objectives).
- Training Schemes are Centralized Learning, Fully Decentralized Learning, or CTDE (subdivided into Individual Parameters, Parameter Sharing, and Concurrent); CTDE with parameter sharing is the most widely adopted.
- The dimensions are claimed to be independent at the level of classification criteria, though specific implementations can induce interdependencies (e.g., limited bandwidth precluding full communication).
- The survey catalogs the evaluation metrics in use — Reward-based, Win/Fail Rate, Steps Taken, Communication Efficiency, and Emergence Degree (positive signaling / positive listening) — and finds communication efficiency is underused, making fair cross-method comparison difficult.
- It proposes four combined-dimension research directions: Multimodal Communication, Structural Communication (group/bridge-agent routing), Robust Centralized Unit (defending proxies/critics against malicious or noisy feedback), and expanded Learning Tasks with Emergent Language using complex symbolic message formats.

## Conclusion

The survey concludes that Comm-MADRL, while already producing a notable body of work (41 surveyed models) with significant achievements, remains a young field with substantial room to grow. Its core deliverable — the 9-dimensional framework — is presented as both an analytical lens for comparing existing works and a step-by-step design guideline (Procedure 1) for building new systems; projecting the literature into this space (Table 13) reveals concentration in cooperative goals, unconstrained or bandwidth-limited communication, existing-knowledge messages, attention-based combination, and CTDE with parameter sharing. The authors identify clear gaps and open questions: non-cooperative and partially competitive communication (including trust, reputation, and defense against deceptive messages); communication under realistic constraints (delay, asynchrony, fairness in resource allocation); richer, fine-grained communication actions and complex message formats (graphs, logical expressions) beyond single values/vectors; model-based message generation for reliable future-knowledge sharing; layered/hierarchical message integration; self-evaluated communication protocols enabling fully decentralized learning (relevant to e-commerce, networks, and blockchain settings where synchronization is costly); robust centralized proxies and critics; and multimodal/structural communication. They also flag persistent challenges in neural architecture choice/hyperparameter tuning and the explainability/human-interpretability of learned messages. Being a survey, it offers no new empirical results; its claims are organizational and taxonomic, supported by the consistency with which the 9 dimensions accommodate the surveyed literature. The overarching message is that Comm-MADRL should expand toward non-cooperative settings, heterogeneous agents, larger agent populations, and diverse data modalities, while drawing on advances from the broader MARL community.
