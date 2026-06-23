---
id: self-evolving_multi-agent_reinforcement_learning__open-ended_environments
title: "Self-Evolving Multi-Agent Reinforcement Learning Systems for Decentralized Decision-Making in Open-Ended Environments"
authors:
  - Harsh Wasnik
  - Yash Wasnik
  - Rishab Khatokar
  - Dhawal Gajwe
year: 2026
venue: "2026 Sixth International Conference on Advances in Electrical, Computing, Communications and Sustainable Technologies (ICAECT)"
publisher: "IEEE"
pages: "1-6"
doi: "10.1109/ICAECT68478.2026.11426025"
url: "https://research.ebsco.com/linkprocessor/plink?id=b813b29f-2448-3a45-b68e-6a3cd0d50f18"
type: inproceedings
keywords:
  - Multi-agent reinforcement learning
  - Decentralized decision-making
  - Evolutionary meta-learning
  - Open-ended environments
  - NYC TLC trip data
---

## Overview

This paper (Wasnik et al.) proposes SE-MARL, a self-evolving multi-agent reinforcement learning framework for decentralized decision-making in open-ended, non-stationary environments where there is no global controller and agents observe only local state. The central thesis is that robustness under distribution shift comes not from a single fixed learning rule but from an evolving "ecosystem" of learners whose architectures, hyperparameters, and training curricula co-adapt over time. The method wraps a lightweight population-based outer loop around standard decentralized inner-loop MARL learners, intermittently evolving architectures and hyperparameters while retaining Pareto-efficient policies, and pairs this with automatically generated curricula (POET-style) and on-policy distillation plus sparse, topology-aware local message passing to preserve coordination. The authors claim to construct a city-scale, partially observable fleet-control benchmark from publicly available NYC TLC trip records (2014-2019)—reconstructing demand fields, travel-time estimates, and shocks—and report that SE-MARL scales to thousands of agents, withstands zero-shot expansions and distribution shifts, and outperforms decentralized training/execution (DTDE) and non-evolutionary MARL baselines on task reward and regret, with ablations showing both self-evolution and curriculum signals are jointly necessary. Notably, the paper's actual Methodology and Results sections describe a completely unrelated supervised-learning experiment on the Wisconsin Diagnostic Breast Cancer tabular dataset rather than any SE-MARL or fleet-control experiment, so the claimed MARL empirical results are not substantiated by reported experimental detail in this manuscript.

## Background

The paper frames itself as building on several established lines of work, which it cites as support for its design. It says standard MARL pipelines assume stationary environments and fixed agent sets, citing centralized-critic actor-critic methods (MADDPG, Lowe et al. [1]; counterfactual COMA, Foerster et al. [2]) and monotonic value-factorization (QMIX, Rashid et al. [3]) as strong but stationarity-bound inner-loop learners. It draws its outer-loop intuition from Population-Based Training (Jaderberg et al. [4]), which it says discovers hyperparameter schedules in situ that static search misses, and from neuroevolution (NEAT, Stanley and Miikkulainen [5]), which it cites for evolving network structure alongside weights to gain capabilities and sample efficiency. For automated curricula it invokes the POET family (Wang et al. [6]) as formalizing how a population can invent increasingly complex problems and reuse stepping-stone solutions. The data grounding is attributed to the NYC TLC Trip Record Data [7]. The literature review additionally cites Dec-POMDP foundations (Oliehoek and Amato [8]; Bernstein et al. [9], who proved optimal Dec-POMDP planning is NEXP-complete), value decomposition (VDN, Sunehag et al. [10]), differentiable communication (CommNet, Sukhbaatar et al. [11]), mean-field MARL for scaling to many agents (Yang et al. [12]), and population/game-theoretic training including PSRO (Lanctot et al. [13]) and emergent autocurricula in hide-and-seek (Baker et al. [14]). Its theoretical review grounds the approach in stochastic/Markov games (Shapley [15], Littman [16]), the policy-gradient theorem (Sutton et al. [17]), deterministic policy gradients (Silver et al. [18]), safe improvement via CPI (Kakade and Langford [19]) and TRPO (Schulman et al. [20]), Nash Q-learning (Hu and Wellman [21]), and QTRAN's IGM-consistency factorization (Son et al. [22]). The disconnected supervised section cites the Wisconsin Diagnostic Breast Cancer dataset (Wolberg et al. [23]) and standard ML references (Bishop [24], Hastie et al. [25], Quinlan [26], Fawcett [27]).

## Key Points

- Proposes SE-MARL, which wraps a population-based evolutionary outer loop around decentralized inner-loop MARL learners, evolving architectures and hyperparameters intermittently while retaining Pareto-efficient policies, rather than replacing the underlying MARL update rule.
- Claims self-evolution decouples "how we train" and "what we train on" from "which MARL update rule we use," positioning CTDE critics, value-decomposition families, and message passing as interchangeable inner-loop instruments under the evolutionary outer loop.
- Integrates automatic curriculum generation (POET-inspired) so agents face a rolling frontier of scenarios—from benign to adversarial demand shocks, traffic flows, and communication failures—keeping training at the edge of competence to guard against brittle overfitting.
- Maintains coordination without any global controller via on-policy distillation and sparse, topology-aware local message passing, framed as deployable because PBT-style exchanges are local and asynchronous.
- Claims to construct a city-scale, partially observable, non-stationary fleet-control benchmark from NYC TLC trip histories (2014-2019), reconstructing demand fields, travel-time estimates, and shocks (weather, events, seasonal drift), with natural distribution shift across weekdays/weekends, seasons, and pre/post-event dynamics.
- Reports SE-MARL outperforms DTDE and non-evolutionary MARL baselines on task reward and regret, and scales to thousands of agents under zero-shot expansions, distribution shifts, and unseen agent types.
- Claims ablations show that both self-evolution (search over inductive biases and learning schedules) and curriculum pressure are jointly necessary for robustness, and that emergent specialization within the heterogeneous population is responsible for sample efficiency.
- Argues evolutionary search naturally yields a Pareto set of competencies (fairness vs. throughput, stability vs. responsiveness, communication overhead vs. joint performance) without presupposing a single scalar objective.
- Provides a unifying conceptual synthesis tying Dec-POMDP foundations, coordination-aware value factorization, differentiable communication, mean-field approximations, and population/game-theoretic training into a single "ecosystem" view of decentralized learning.

## Conclusion

The paper concludes that SE-MARL is a practical recipe for decentralized decision-making in open-ended, non-stationary worlds, asserting that its NYC TLC-grounded experiments show scaling to thousands of agents, resilience to distribution shift and zero-shot expansions, and superiority over decentralized and non-evolutionary MARL baselines on reward and regret, with ablations indicating evolutionary search and curriculum pressure are jointly necessary to elicit emergent specialization. The authors state several limitations and open questions: evolutionary outer loops add compute and wall-clock overhead requiring principled budgeting even though exchanges are asynchronous and local; the environment built from historical traces inherits biases and omissions (unobserved confounders, rare events) that can affect generalization and fairness; reliance on proxy objectives means real multi-stakeholder deployments will need explicit safety, service-equity, and emissions constraints plus auditing; and on-policy distillation, while helpful, comes with no formal stability guarantees for continual policy turnover. A major internal inconsistency undermines the empirical claims: the manuscript's Methodology and Results sections do not describe any MARL, fleet-control, or NYC TLC experiment, but instead report a Gaussian Naive Bayes versus decision-tree comparison on the Wisconsin Diagnostic Breast Cancer dataset—an apparently misplaced or templated section. Consequently the SE-MARL reward/regret/scaling results and ablations asserted in the abstract and conclusion are not actually evidenced anywhere in the paper's experimental reporting, which is the most important caveat for any researcher considering citing its empirical findings.
