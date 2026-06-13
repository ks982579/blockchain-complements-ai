---
id: blockchain-based_health_and_usage_monitoring__aerospace_structures
title: "Blockchain-based Health and Usage Monitoring Systems (HUMS) For Aerospace Structures"
authors:
  - Ravi Mukkamala
  - Eranga Bandara
  - Sachin Shetty
year: 2022
venue: "2022 IEEE/AIAA 41st Digital Avionics Systems Conference (DASC)"
pages: "1-8"
type: inproceedings
keywords:
  - Avionics
  - blockchain
  - fault-tolerance
  - IoT
  - monitoring
  - smart contracts
  - tamper-proof
---

## Overview

This paper proposes a blockchain-based architecture for avionics Health and Usage Monitoring Systems (HUMS), which are responsible for monitoring the structural and component-level health of aircraft via embedded sensors. The authors identify that traditional file-system-based storage for HUMS data is vulnerable to attack, accidental or intentional tampering, and failure, and argue that blockchain's properties of distributed replication, immutability, tamper-evidence, and cryptographic security are a natural fit for this safety-critical domain. The core contribution is a 4-layer architecture (sensors, blockchain/smart contracts, applications, users) that combines a permissioned blockchain with IPFS for bulk data storage, and that defines four example smart contracts (identity, sensor, model card, and permission contracts) to support secure data provenance, access control under the principle of least privilege, and integration of model cards and big data analytics for predictive maintenance. The work is explicitly preliminary/conceptual — no implementation or evaluation is presented — but it lays out a concrete design intended to be prototyped using the Rahasak blockchain, IPFS, and the Aplos smart contract platform (Scala/Akka actors).

## Background

The paper situates its proposal within the historical evolution of avionics health monitoring, citing Ranasinghe et al. [1] to trace HUMS from 1960s structural health monitoring through vehicle health monitoring, integrated vehicle monitoring management, prognostics and health management (PHM), and today's integrated system health management, with future systems expected to incorporate AI, deep learning, IoT, and data analytics. It grounds blockchain technology itself in Nakamoto's original Bitcoin white paper [3] and cites industry adoption statistics from Blockdata/Forkast [2] to motivate blockchain's mainstream traction (e.g., Microsoft, Amazon, J.P. Morgan, Walmart). For aviation-specific blockchain applications, the paper draws on an IATA white paper [4] exploring blockchain use cases across baggage, cargo, spare parts, and aircraft maintenance, and a later IATA white paper [12], which argues blockchain adoption is becoming "obligatory" for aircraft health management ("Maintenance of Tomorrow"), as well as studies on adoption drivers (Li et al. [5]) and industry attitudes (Lopes et al. [6], describing a "strong desire but lethargy" toward adoption). It cites multiple prototype/case studies on blockchain for aircraft maintenance records and traceability (Andrei et al. [7], Marx et al. [8] on digital twins, Aleshi [9], Rajkov [10] on spare-part tracking, Efthymiou et al. [11] on MRO applications) and a broader cluster of avionics-blockchain efforts [16]-[29] (e.g., Shen et al. [16] on trusted data exchange, Blasch et al. [21] on reliable avionics, Zhu et al. [29] on encrypted consortium blockchains for civil aviation). It also references prior work applying blockchain and IoT to human health monitoring [30]-[33] as an analogous domain. Foundational technology components are drawn from: permissioned vs. permissionless blockchain literature (Wüst & Gervais [39], De Angelis et al. [40], Neudecker & Hartenstein [42]); smart contract design via the Aplos platform (Bandara et al. [44]); federated learning (Yang et al. [45]); model cards via Google's TensorFlow Model Card Toolkit (Fang & Miao [46], Wadhwani & Jain [47]); a big data analytics framework for avionics (Burmester et al. [48]); and IPFS (Benet [57]). Gao et al. [55] is separately cited for an illustration of typical sensor placement in an aircraft HUMS. The planned implementation builds on the authors' own prior Rahasak blockchain architecture [56] and Aplos smart contract framework [44], [58], using Scala [59] and Akka actors [60].

## Key Points

- The paper proposes a novel 4-layer blockchain-based HUMS architecture (sensor layer, blockchain and smart contract layer, application layer, users layer) designed to be technology-agnostic at each layer so underlying components (e.g., blockchain platform) can be swapped without affecting other layers.
- It identifies eight core system requirements for next-generation avionics HUMS: safety, scalability, speed, size, security, performance, predictability, and maintenance-data integration.
- It proposes storing only signed hashes or key data elements of sensor records on-chain while offloading full/bulk sensor data to IPFS, as a way to address blockchain throughput limitations given high-rate sensor data generation.
- It defines four example smart contracts — identity, sensor, model card, and permission contracts — as the mechanism for managing sensor authenticity (via digital IDs/public keys), mediating data access for applications, integrating ML model provenance/usage information, and enforcing access control.
- It explicitly proposes enforcing the "principle of least privilege" via permission smart contracts so that applications/sensors can only access data relevant to their function, limiting damage from compromised components or malware.
- It argues permissioned blockchains are more appropriate than permissionless ones for HUMS because such systems operate within a single organizational trust boundary, trading some decentralization for scalability and performance.
- It proposes integrating model cards (via the Google TensorFlow Model Card Toolkit) into the blockchain architecture to store, update, and retrieve provenance and usage information for the ML/predictive models used in component health prediction.
- It outlines a concrete planned implementation stack: the Rahasak blockchain combined with IPFS storage and the Aplos smart contract platform (Scala, Akka actors), and poses six open implementation questions regarding sensor data rate handling, response time, integration with existing systems, performance tuning, on-chain/off-chain data partitioning, and compatibility with commercial data analytics tools.

## Conclusion

The paper concludes that blockchain-based architectures can plausibly address key HUMS requirements — security, fault-tolerance, tamper-evidence, scalability, and flexibility — by combining a multi-node permissioned blockchain with IPFS distributed storage and a layered smart-contract design. However, the work is explicitly described as preliminary and conceptual: "the implementation is yet to be done," and no experiments, prototypes, or quantitative evaluation are presented to validate the proposed architecture's performance, scalability, or security claims. The authors frame the next phase as building a prototype on the Rahasak blockchain and Aplos smart contract platform, and they pose several open questions that remain unanswered — whether the system can handle high-rate sensor data, what response times are achievable for different user types, how easily it integrates with existing avionics systems, how to partition data between on-chain storage and IPFS for optimal performance, and whether commercial big-data analytics tools can be integrated. These questions represent the paper's primary identified directions for future research and validation work.
