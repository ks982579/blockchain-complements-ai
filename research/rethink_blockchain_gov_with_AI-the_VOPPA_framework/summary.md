---
id: rethink_blockchain_gov_with_AI-the_VOPPA_framework
title: "Rethinking Blockchain Governance with AI: The VOPPA Framework"
authors:
  - Catalin Daniel Morar
  - Daniela Elena Popescu
  - Ovidiu Constantin Novac
  - David Ghiurău
year: 2025
venue: "Computers"
publisher: "MDPI"
volume: 14
issue: 10
pages: "425"
doi: "10.3390/computers14100425"
url: "https://www.mdpi.com/2073-431X/14/10/425"
type: article
keywords:
  - blockchain governance
  - decentralized systems
  - decision-making
  - smart contracts
  - DAO
  - autonomous agents
  - artificial intelligence
  - natural language processing
  - machine learning
  - large language models
---

## Overview

This paper is a systematic literature review (52 core publications, screened from 103 sources via title/abstract review, full-text review, and snowballing) on blockchain governance that culminates in a novel theoretical proposal, the VOPPA (Voting Via Parallel Predictive Agents) framework. It addresses the problem that existing blockchain governance models (on-chain, off-chain, hybrid) suffer from persistent issues with participation, power concentration, coordination failures, and adaptability to crises, and that while AI is increasingly discussed as a complement to blockchain governance, few prior works offer concrete architectural integrations. The paper's contribution is threefold: (1) a layered, comparative synthesis of governance models grounded in five real-world failure case studies (The DAO, Build Finance DAO, Bitcoin Cash fork, Compound, Beanstalk), (2) a structured survey of current and emerging AI applications in decentralized governance (proposal summarization, anomaly/Sybil detection, autonomous agent voting), and (3) VOPPA itself, a multi-agent architecture in which independent AI agents predict the quantifiable impact of governance proposals and vote accordingly, rather than relying on token-weighted or consensus-based voting. The authors frame VOPPA as a theoretical/conceptual contribution intended to seed future applied and empirical research, not as a deployed or evaluated system.

## Background

The paper situates itself within a body of prior work that maps the DAO landscape via taxonomies and real-world implementation surveys (citing works such as Calzada's Web3/governance analysis, Liu et al.'s overview of blockchain-based DAOs, and Wang et al.'s "First Look into Blockchain DAOs"), and other studies cataloguing governance vulnerabilities and low voter participation (e.g., Duijn et al.'s "Seven Capital Sins," Kharman and Smyth on DAO governance vulnerabilities). It draws on a three-layer governance model from Ullah and Havinga and Pelt et al., distinguishing on-chain protocol, off-chain development, and off-chain community layers, and on the on-chain/off-chain/hybrid governance taxonomy used widely in the field (citing Investopedia-style sources, Tan et al.'s public-sector governance framework, and the authors' own prior EMES 2025 conference paper). For AI-in-governance context, the paper cites NLP/sentiment-analysis work on DAO discourse (Quan et al.), a KlimaDAO case study using chain-of-thought reasoning for governance support (Chen et al.), graph deep learning for Sybil detection in DAO voting networks using graph autoencoders and GCNNs tested on Snapshot.org data (DuPont), and a hybrid anomaly-detection framework combining entropy/motif/clustering indicators via a Boltzmann machine with a conversational AI alert agent (Ikeda et al.). It also references the broader autonomous-agent literature distinguishing "governed agent" frameworks (ETHOS's decentralized agent registry using smart contracts and zero-knowledge proofs; the LOKA Protocol's layered identity/ethics infrastructure) from "agents as governance participants" frameworks (a survey by Karim et al.; the single-agent AILVE DAO; Ding et al.'s and Liang et al.'s ACP/parallel-intelligence simulation approaches using rule-based, non-learning agents; LLM-based multi-agent voting councils with a coordinator agent; DeAgents, self-sovereign LLM-powered agents holding private keys built on LLM+TEE architectures per Hu et al.; and the ASAP Protocol for delegated AI-agent voting).

## Key Points

- The paper synthesizes blockchain governance into a three-layer model (on-chain protocol layer, off-chain development layer, off-chain community layer) and a complementary two-dimensional taxonomy (location: protocol vs. smart-contract layer; purpose) describing what part of a system governance decisions affect.
- It produces a comparative table of on-chain, off-chain, and hybrid governance models, arguing none is sufficient alone: on-chain offers transparency/automation but suffers low turnout, token-based centralization, and rigidity; off-chain offers deliberative flexibility but lacks transparency and formal accountability; hybrid (e.g., Cardano's Project Catalyst) balances both but introduces coordination complexity and inherits centralization risks from each layer.
- Through five case studies (The DAO attack, Build Finance DAO takeover, the Bitcoin Cash fork, the 2024 Compound governance attack, and the Beanstalk flash-loan governance exploit), the paper argues that decentralization alone does not guarantee resilience, fairness, or security, and that governance design must explicitly account for low participation, token concentration, absence of emergency/reversal mechanisms, and lack of formal conflict-resolution processes.
- The paper claims that current AI applications in blockchain governance cluster into three categories: (1) NLP-based proposal summarization/sentiment analysis to improve stakeholder comprehension and engagement, (2) AI-driven anomaly/Sybil detection to preserve voting integrity, and (3) autonomous AI agents acting either as regulated "governed entities" or as active governance participants (voting, proposal generation, asset management).
- It asserts that existing autonomous-agent governance proposals (AILVE DAO, ACP/casCAD2 simulations, multi-agent voting councils, DeAgents, ASAP Protocol) remain largely theoretical, limited in empirical validation, and often reintroduce centralization (e.g., via a coordinator agent) or rely on simplified rule-based agents rather than true learning-based AI.
- The paper proposes VOPPA, a novel multi-agent architecture in which each agent pairs an NLP module (e.g., SBERT) for semantic encoding of proposal text with a separate supervised ML predictor (e.g., Random Forest Regressor) trained to forecast quantifiable post-implementation outcomes (protocol usage, financial performance, community engagement), rather than mimicking human preferences.
- VOPPA deliberately separates the NLP and prediction components (rather than using a unified LLM) for transparency, modularity, interpretability, targeted retraining, and to mitigate the computational cost, bias propagation, and unpredictable behavior associated with LLMs in high-stakes governance.
- The framework specifies an odd-numbered ensemble of heterogeneous agents (differing training data, model configurations, and thresholds) casting binary "yes/no" votes against a defined impact threshold (e.g., 0.6), aggregated via majority voting, with a majority-elimination procedure for multi-option proposals to avoid deadlock.
- VOPPA is architecture-agnostic regarding vote aggregation (smart-contract-based on-chain aggregation or off-chain governance layer) and assumes off-chain storage of full proposal content to avoid on-chain resource overload while keeping content accessible to all agents.
- The paper presents VOPPA as a simplified four-step cycle (proposal submission, independent agent analysis, individual voting, aggregation) intended as a foundation for future applied research, explicitly not yet implemented or empirically validated.
- The authors identify specific limitations of VOPPA as proposed: dependence of prediction accuracy on training data quality, computational/scalability overhead from running large agent ensembles, ethical concerns around transparency, value alignment and accountability, and vulnerability of individual agents to private-key compromise or model tampering that could allow vote capture.

## Conclusion

The paper concludes that effective blockchain governance requires more than decentralization—it requires coordination, accountability, and adaptability—and that the five case studies confirm current on-chain, off-chain, and hybrid models all remain structurally fragile under conditions of low participation, token concentration, and absent emergency mechanisms. It finds that AI integration (proposal summarization, anomaly detection, autonomous agents) is a promising but still largely experimental and theoretical direction, with most implementations being post hoc, narrowly deployed, or carrying centralization and reliability risks. VOPPA is presented as the paper's proposed answer to these gaps—shifting governance from token-based or preference-driven voting to outcome-based, predictive, multi-agent reasoning—but the authors are explicit that VOPPA itself is purely a theoretical/conceptual framework with no prototype, simulation, or empirical evaluation yet conducted. Open research questions and future work explicitly flagged include: determining the optimal number and diversity configuration of agents via simulation on historical data; building robust pipelines for collecting, curating, and validating training data (governance proposals, forum discussions, voting records); addressing scalability, latency, and throughput costs of running agent ensembles in blockchain environments; designing transparent logging and human appeal/override mechanisms to preserve community accountability over AI-driven outcomes; and translating the conceptual architecture into prototypes and simulation-based experiments for empirical validation of predictive accuracy, resilience, diversity, and fairness.
