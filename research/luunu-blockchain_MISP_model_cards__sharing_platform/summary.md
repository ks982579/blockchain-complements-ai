---
id: luunu-blockchain_MISP_model_cards__sharing_platform
title: "LUUNU — Blockchain, MISP, Model Cards and Federated Learning Enabled Cyber Threat Intelligence Sharing Platform"
authors:
  - Eranga Bandara
  - Sachin Shetty
  - Ravi Mukkamala
  - Abdul Rahaman
  - Xueping Liang
year: 2022
venue: "2022 Annual Modeling and Simulation Conference (ANNSIM)"
pages: "235-245"
url: "https://api.semanticscholar.org/CorpusID:251773322"
type: inproceedings
keywords:
  - Blockchain
  - Federated Learning
  - Cyber Threat Intelligence
  - MISP
  - Model Card
---

## Overview

This paper presents Luunu, a Cyber Threat Intelligence (CTI) sharing platform that combines blockchain, the Malware Information Sharing Platform (MISP), Model Cards, and federated learning to address privacy, anonymity, traceability, and data provenance problems inherent in inter-organizational CTI sharing. The core problem the authors target is the tension between anonymizing CTI contributors (to protect their reputations and avoid revealing infrastructure vulnerabilities) and preserving the credibility/provenance of shared data — anonymity without provenance undermines trust in the data. Luunu's design stores all CTI events as structured Model Card objects in a MISP storage layer, records provenance metadata for both CTI submissions and machine learning models on a blockchain ledger, uses self-sovereign identity (SSI) via a mobile wallet to anonymize incident reporters/viewers while still allowing identity verification by service-provider admins, and runs a coordinator-less federated learning process across blockchain peers to build threat-detection models (e.g., for DDoS, network attacks, anomaly detection) from locally held data without centralizing raw data. The platform is implemented on top of the authors' own Rahasak blockchain (using its Aplos smart contract platform), with PyTorch/PySyft for federated learning, the TensorFlow Model Card Toolkit for model card generation, and Apache Kafka for consensus and inter-service communication; the paper reports throughput, latency, scalability, and block-generation-time measurements as evidence of feasibility.

## Background

The paper situates itself within established CTI literature and several external technologies/standards it builds directly on: it cites Mavroeidis and Bromander (2017) and Kampanakis (2014) for the general framing of CTI as proactive defense knowledge about adversary tactics and infrastructure vulnerabilities, and Wagner et al. (2019) and Johnson et al. (2016) (a NIST report) for the standard CTI data formats and sharing guidance that motivate structured, machine-processable CTI exchange. The privacy/anonymity-versus-provenance tension that motivates Luunu's design is attributed to Burger et al. (2014), who argued anonymization can undermine data credibility, and Feng et al. (2019), cited as a survey establishing the core blockchain-privacy challenges (privacy, reliability, traceability, anonymity, provenance) that Luunu claims to address. The platform's components are explicitly built on prior external work: MISP itself comes from Wagner, Dulaunoy, Wagener, and Iklody (2016); the Model Card concept for ML model documentation/provenance is taken from Wadhwani and Jain (2020a/b) and from the authors' own earlier system Bassa-ML (Bandara et al. 2022); the federated learning approach (coordinator-less, with model averaging) follows Konečný et al. (2016) and Yang et al. (2018); and the SSI identity model draws on Liang et al. (2017), Mühle et al. (2018), and Baars (2016) for SSI-proof verification. The underlying blockchain infrastructure (Rahasak, its Aplos smart-contract layer, and Kafka-based consensus/back-pressure handling) is reused from the authors' prior work (Bandara et al. 2018, 2019, 2021 — multiple papers on Rahasak, Aplos, Tikiri, and SaaS microservice smart contracts). Finally, the related-work section frames Luunu against several existing blockchain-based CTI sharing systems — BCTISA (Cha et al. 2020), BLOCIS (Gong and Lee 2020), BloCyNfo-Share (Badsha et al. 2020), SDN-CTI (Hajizadeh et al. 2020), CTI-Cloud (Kamhoua et al. 2015), and PP-CTI (Badsha et al. 2019) — as the comparison baseline for its claimed feature advantages.

## Key Points

- Luunu is proposed as a blockchain-based CTI sharing platform with five layers: stakeholder layer, smart contract layer, blockchain storage layer, CTI (MISP) storage layer, and data analytics layer.
- The platform encodes all shared CTI data as "Model Card" objects stored in MISP, providing enhanced transparency, auditability, and data provenance for the sharing process.
- A blockchain-enabled, coordinator-less federated learning approach is used to analyze CTI data distributed across peer organizations without centralizing raw data, with both local and global models recorded on-chain as Model Card objects for provenance.
- Self-sovereign identity (SSI), implemented via a Luunu mobile wallet, is used so incident reporters and viewers can anonymously submit/view CTI while still allowing the platform's service-provider admins to verify identity via SSI-proofs stored on the blockchain.
- The smart contract layer is organized into four distinct contracts — Identity, CTI (MISP), FML (federated machine learning), and Notification — each handling a discrete set of platform functions (identity management/verification, CTI event creation and retrieval, model parameter sharing/averaging/storage, and notifications, respectively).
- A standardized cyber threat incident record format is defined with five fields: date, incident type, incident description, incident status, and incident action, processed by the Incident smart contract.
- Luunu is implemented on top of the authors' own Rahasak blockchain (with Aplos smart contracts), using PyTorch/PySyft for federated learning, the TensorFlow Model Card Toolkit for model card generation, and Apache Kafka for consensus and inter-service communication.
- The paper reports empirical performance evaluations covering invoke-transaction and query-transaction throughput, transaction scalability and latency as peer count increases, and block generation time/frequency as functions of transaction set size and peer count.
- The paper presents a feature comparison table (Table 1) claiming Luunu is the only one among compared platforms (BCTISA, BLOCIS, BloCyNfo-Share, SDN-CTI, CTI-Cloud, PP-CTI) that simultaneously offers decentralized architecture, high scalability, high privacy, SSI, Model Cards, auditing/provenance, and built-in data analytics.

## Conclusion

The paper concludes that Luunu successfully integrates blockchain, MISP, Model Cards, and federated learning into a single CTI sharing platform that addresses traceability, reliability, privacy, scalability, anonymization, and data provenance — the core challenges the authors identify at the outset — primarily through the SSI-based identity approach and Model-Card-encoded, blockchain-anchored provenance records. The performance evaluation results (transaction throughput, scalability, latency, and block generation time across varying numbers of peers and transactions) are presented as evidence that the architecture is feasible at scale, though the paper does not provide a comparative quantitative benchmark against the other CTI platforms discussed (the Table 1 comparison is feature-based/qualitative rather than performance-based). The authors explicitly flag several open directions as future work: developing governance rules/guidelines for posting, querying, correcting, and disputing shared CTI information; assessing the quality of shared CTI data; designing an incentive mechanism to encourage participation in CTI sharing; and collecting user feedback to inform broader adoption and further development of the platform. These open items suggest that, as presented, Luunu's governance, incentive, and data-quality-assurance mechanisms remain unaddressed and represent the main gaps for follow-on research.
