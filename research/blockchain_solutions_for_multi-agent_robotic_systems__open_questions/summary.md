---
id: blockchain_solutions_for_multi-agent_robotic_systems__open_questions
title: "Blockchain Solutions for Multi-Agent Robotic Systems: Related Work and Open Questions"
authors:
  - Ilya Afanasyev
  - Alexander Kolotov
  - Ruslan Rezin
  - Konstantin Danilov
  - Alexey Kashevnik
  - Vladimir Jotsov
year: 2019
venue: "arXiv"
doi: "10.48550/arXiv.1903.11041"
url: "https://arxiv.org/abs/1903.11041"
type: misc
keywords:
  - Robotics
  - Multiagent Systems
  - Blockchain
  - Distributed Ledger
---

## Overview

This short related-work paper (Afanasyev et al., Innopolis/ITMO/UNIBIT) surveys, analyzes, and classifies blockchain (distributed ledger technology) solutions for multi-agent robotic systems, motivated by the rise of autonomous robot groups and the Fourth Industrial Revolution. It frames the problem as one of trusted, verifiable information support for robot coalitions operating under uncertainty, external disturbances, and environmental change, where recording interaction history and verifying agent behavior can improve coordination efficiency. The authors first restate blockchain's core properties (append-only distributed database, peer-to-peer node network, validator-based consensus such as proof-of-work) and map them onto multi-agent system needs (data availability via replication, data consistency via validation, immutability, and economic/reputational incentives). Its central contribution is a classification of six typical use-case categories for blockchain-based multi-agent robotic applications, illustrated with literature examples, followed by a list of open research questions. The paper positions itself as guidance for using blockchain-backed frameworks to extend existing robotics platforms and libraries rather than reinventing similar code.

## Background

The paper builds heavily on prior work it cites to support its argument. It draws on distributed control and motion-coordination literature for robotic networks and multi-robot formation/consensus control (citing Bullo et al., Liu et al., Peng et al., Savkin et al., Wang et al.) to establish the multi-agent robotics context. For blockchain fundamentals (scalability, latency, throughput limits, proof-of-work, sharding), it relies on a blockchain FAQ source. It surveys application domains established elsewhere: blockchain e-voting via intelligent agents (Pawlak et al.), open blockchain-aided symbiotic cyber-physical systems (Skowronski), blockchain for IoT (Casino et al.; Wang et al.), smart/software-defined buildings and smart districts/cities (Mazzara et al.; Lazaroiu and Roscia), Industrial IoT and Industry 4.0 economical communication (Teslya and Ryabchikov; Kapitonov et al.), and blockchain supply chains enabling circular economy (Casado-Vara et al.). It notes that a prior systematic review of blockchain plus multi-agent systems (Calvaresi et al.) did not classify MAS by purpose nor cover multi-robot systems. It cites domain-specific robotics-blockchain work it extends: UAV coordination protocols (Kapitonov et al.), liability-execution validation via model checking for agent service providers (Danilov/Rezin/Afanasyev et al.), ontology-based robot coalition formation in cyber-physical/smart spaces (Teslya and Smirnov; Kashevnik and Teslya), the RobotChain/RoboChain decentralized ledger and consortium-blockchain robot control with Oracles (Lopes et al.; Ferrer et al.), smart contracts for multi-agent plan execution in untrusted systems (Shukla et al.), Ferrer's blockchain framework for robotic swarms (coin-to-proposal voting), Carnegie Mellon's market-economy multi-robot exploration (Zlot, Stentz, Dias, Thayer), and blockchain EV battery refueling and autonomous EV charging/billing (Hua et al.; Strugar et al.).

## Key Points

- The paper proposes a classification of the most typical use cases for using blockchain in multi-agent robotic systems, organized into six categories (summarized in its Fig. 1).
- Category A, "Logging agent actions with bytecode distribution": blockchain delivers platform-independent executable bytecode tasks to general-purpose agents over a peer-to-peer network, enabling delivery even when an agent is offline, persistent immutable logging of state changes for fast recovery after halt, scheduled/delayed execution, and automatic queuing of command sequences.
- Category B, "Distributed decision making by time-limited voting": smart contracts (e.g., on Ethereum) can implement polls with complex behavior such as time-limited voting or vote delegation, improving on prior coin-to-proposal-address swarm voting; combined with bytecode distribution, agents can propose bytecode actions and collaboratively vote to assemble a scenario.
- Category C, "Action validation to exclude intruders or faulty agents": agents mutually validate each other's actions, location, and poses (e.g., via odometry/sensor cross-checks), reaching consensus to detect malfunctioning or maliciously hacked/intruder agents and trigger recovery; sharding by agent location reduces validator workload.
- Category D, "Economic incentive for task execution optimization": the financial layer of blockchain can underpin market-economy approaches to multi-robot coordination (e.g., CMU multi-robot exploration/mapping), improving exploration effectiveness and robustness to member loss and communication failures, with time-based rather than distance-based costs enabling faster exploration and flexible task prioritization.
- Category E, "Automated task dispatching via blockchain": a dispatcher smart contract distributes tasks among competing agents, where validators order agent agreements by the fee paid, letting the market select efficient, stable service providers (exemplified by an automated taxi-dispatching scenario with liability validation).
- Category F, "Authentication / suitability check": blockchain provides trust and tamper-resistant verification when mutually distrusting agents share a physical resource, illustrated by replaceable EV battery authentication (life-cycle and amortization state on-chain, smart-contract-driven pricing/payment) and autonomous EV charging/billing with M2M transactions resistant to attacks while preserving user confidentiality.
- The paper argues blockchain strengthens multi-agent robotic systems by providing data availability through replication, data consistency through validation and strict update rules, immutability, and economic/reputational incentives that discourage validation-rule violations.
- It contends agent reliability can be governed by a reputational model (assessable only after obligations are fulfilled) complemented by automated obligation verification using formal methods, increasing trust and interaction efficiency among initially untrusted agents.

## Conclusion

The paper concludes that blockchain can play a significant role in multi-agent robotic system applications and that its analysis of recent publications supports grouping such applications into the proposed classification of task types. As an overview/related-work paper it presents no new experiments; rather, it synthesizes existing studies as evidence and offers the classification as guidance for researchers to adopt common, verified blockchain-backed frameworks instead of building bespoke solutions. The authors identify intelligent support of blockchain-based agent interactions—raising trust levels in such systems—as the most promising near-term research direction, and enumerate open problems: developing a conceptual model of information support for a group of robots during task performance; a typical ontological model of a robotic system; a consensus protocol for verifying group interaction before launching a task using distributed-ledger information; a validation method for task performance by robotic systems; a multi-agent system architecture; and a proof-of-concept framework prototype for an intelligent robot group performing a collaborative task.
