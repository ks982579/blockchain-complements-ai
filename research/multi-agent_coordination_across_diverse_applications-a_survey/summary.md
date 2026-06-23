---
id: multi-agent_coordination_across_diverse_applications-a_survey
title: "Multi-Agent Coordination across Diverse Applications: A Survey"
authors:
  - Lijun Sun
  - Yijun Yang
  - Qiqi Duan
  - Yuhui Shi
  - Chao Lyu
  - Yu-Cheng Chang
  - Chin-Teng Lin
  - Yang Shen
year: 2025
venue: "arXiv"
doi: "10.48550/arXiv.2502.14743"
url: "https://arxiv.org/abs/2502.14743"
type: misc
keywords:
  - Multi-agent System
  - Swarm Coordination
  - Cross-application
  - Swarm Intelligence
  - Survey
---

## Overview

This survey (Sun et al., 2025; arXiv 2502.14743) examines multi-agent coordination as the shared underlying mechanism that enables the proliferation of diverse multi-agent systems (MAS), spanning robotics, transportation, satellites, and large language models (LLMs). Its central contribution is a *unified computational framework* that interprets all coordination research through four fundamental questions: (1) what is coordination, (2) why coordination, (3) who to coordinate with, and (4) how to coordinate. The authors argue that prior MAS surveys classify work narrowly by technique or task domain and rarely unify coordination across applications; in contrast, this paper frames coordination as an iterative sequential-decision process with three components — evaluating system-level performance, social choice on *who* to coordinate with (clustering by inter-agent dependencies), and *how* to coordinate (managing dependencies). It surveys three general coordination problems (coordinated learning, communication/cooperation, conflict-of-interest resolution) and six application domains (search and rescue, warehouse/logistics, transportation, humanoid/anthropomorphic robots, satellite systems, and LLM-based MAS), then identifies open challenges in scalability, heterogeneity, and learning mechanisms, mapping each to a promising future direction. The paper's purpose is explicitly cross-disciplinary knowledge transfer: surfacing commonalities so that coordination ideas in one domain can inspire another.

## Background

The paper builds on a long lineage of coordination and distributed-AI theory. It cites Wooldridge's characterization of coordination as the "defining problem in working together" and Malone et al.'s interdisciplinary definition that coordination is "managing dependencies between activities," which the authors adopt as the cornerstone of their framework. It draws on distributed artificial intelligence (DAI) foundations (Bond and Gasser; Russell et al.'s agent-centric view) and on biological/social explanations of emergent intelligence — self-organization in biological systems (Camazine et al.), swarm intelligence (Dorigo and Theraulaz; Bayindir), collective intelligence, and Minsky's "society of mind." The framework is explicitly likened to human brainstorming and is said to have inspired brain storm optimization (BSO) and LLM "mindstorm" methods. For technical grounding, the paper draws on prior MAS surveys it positions itself against — consensus algorithms, multi-agent planning, general and Internet-focused MARL, MARL application domains, and autonomous-driving task surveys. In coordinated learning it cites the CTDE paradigm and specific algorithms (MAPPO, VDN, QMIX, MADDPG, COMA, IC3Net, CommNet, DIAL, BiCNet, SchedNet) and higher-order coordination-graph methods (LTS-CG, GACG). For conflict resolution it cites Multi-Agent Path Finding (MAPF) work (conflict-based search, meta-agents, conflict/dependency graphs) and lexicographic conventions. Application sections cite domain references such as Drew's SAR survey, Amazon/Kiva and Cainiao warehouse robotics, autonomous-driving MAS surveys, satellite swarm missions (Magnetic Nano-Probe Swarm, QB50, OLFAR), SpaceX Starlink configuration changes, and LLM-MAS systems (CAMEL, RoCo, ReAd, generative agents, MetaGPT, GPTSwarm, AutoGen, LangChain, CrewAI).

## Key Points

- The paper proposes a unified framework casting multi-agent coordination as an iterative sequential-decision process with three components: evaluate system-level performance, decide *who* to coordinate with (social choice/clustering), and decide *how* to coordinate (manage dependencies).
- It organizes all coordination research around four fundamental questions — what is coordination, why coordination, who to coordinate with, how to coordinate — and argues these are universally addressed by every coordination process yet rarely unified in prior surveys.
- It frames "who to coordinate with" as a clustering problem driven by spatio-temporal, first-order, and *higher-order* (e.g., transitive, cluster-level) inter-agent dependencies expressed in a coordination graph.
- It frames "how to coordinate" as the core of coordinated decision-making, categorizable by methodology (rule-based, game-theoretic, learning-based/MARL, evolution-based) or by application task.
- It identifies and reviews three general MAS coordination problems common to nearly all applications: coordinated/social learning (who to learn from, what to learn), communication and cooperation (who/when/what/how to communicate), and conflict-of-interest resolution (collisions, deadlocks, live-locks).
- It argues that guaranteeing safety in conflict resolution faces a trilemma: the curse of dimensionality for centralized solvers, the imperfection of rule-based distributed solutions, and the immaturity of distributed learning-based methods.
- It surveys six MAS application domains under the unified perspective — search and rescue, warehouse automation/logistics, transportation systems (traffic-signal control and autonomous driving), humanoid/anthropomorphic robots (dual-arm, dexterous hands, full humanoids), satellite systems (constellations, swarms, communications), and LLM-based MAS (decision-making and behavior simulation).
- It claims a single complex robot (e.g., a humanoid) can itself be modeled as a MAS, where subsystems, arms, or fingers are agents requiring coordination, sensor fusion, and task decomposition/transition.
- It categorizes LLM-based MAS into decision-making (collective expertise for software development, embodied intelligence, scientific research) and behavior simulation (social interaction, game-playing, recommendation), noting behavior simulation demands more diverse agent-management methods.
- It maps three open challenges to three promising future directions: scalability → hybridization of hierarchical and decentralized coordination; heterogeneity → human-MAS coordination (interaction and teaming); learning mechanisms → LLM-based MAS.
- It highlights natural brain-computer interfaces (nBCI) and computational trust modeling as enabling technologies for human-MAS teaming and the anticipated "fifth industrial revolution."

## Conclusion

As a survey, the paper does not test a hypothesis but argues that the four-question unified framework successfully captures both the commonalities and the specialties of coordination across established and emerging MAS domains, demonstrated by consistently mapping three general coordination problems and six application areas onto the same "who/how to coordinate" perspective. The authors conclude that the MAS perspective and coordination capability have already reformed, and will further revolutionize, diverse applications, and anticipate that multi-agent coordination will drive a new stage of general AI. They identify three concrete future directions tied to three open performance challenges: (1) hybrid hierarchical-plus-decentralized coordination to achieve scalability while preserving local adaptability and robustness (e.g., regional leader vehicles in traffic networks); (2) human-MAS coordination — both passive human-MAS interaction (e.g., mixed traffic with heterogeneous road users) and proactive human-MAS teaming/human-swarm interaction via nBCI and trust modeling — to handle heterogeneity and inject human intent; and (3) LLM-based MAS as the frontier of learning mechanisms. The paper explicitly notes open limitations and research questions: heterogeneous coordination is harder than homogeneous self-organization; effectively coordinating with heterogeneous human agents under societal norms is under-studied in autonomous driving; co-learning and trustworthiness in human-MAS systems remain challenging; and LLMs, being statistical fits to training data, suffer from poor generalization (e.g., hallucination) and high economic/labor training costs. The authors regard LLM-based MAS as an early-stage but accelerating trend with substantial progress still to come.
