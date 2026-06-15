---
id: toward_trustworthy_AI-BC-based__learning_systems
title: "Toward Trustworthy AI: Blockchain-Based Architecture Design for Accountability and Fairness of Federated Learning Systems"
authors:
  - Sin Kit Lo
  - Yue Liu
  - Qinghua Lu
  - Chen Wang
  - Xiwei Xu
  - Hye-Young Paik
  - Liming Zhu
year: 2023
venue: "IEEE Internet of Things Journal"
publisher: "IEEE"
volume: 10
issue: 4
pages: "3276-3284"
doi: "10.1109/JIOT.2022.3144450"
type: article
keywords:
  - Accountability
  - AI
  - blockchain
  - fairness
  - federated learning
  - machine learning
  - responsible AI
  - smart contract
---

## Overview

This paper presents a blockchain-based architecture for federated learning (FL) systems that targets two specific dimensions of "trustworthy AI" / responsible AI: accountability and fairness. The authors argue that conventional FL systems cannot trace which local data and local models contributed to a global model, making it impossible to hold any stakeholder (hospital, client device, central server) accountable for poor model performance, and that heterogeneous (non-IID) client data distributions cause models to be biased toward majority classes. To address these issues, the paper proposes (1) a smart contract-based "data-model registry" that hashes and records data versions, local model versions, and global model versions on-chain to create an immutable provenance/audit trail without storing raw data or full models on-chain, and (2) a "weighted fair training data set sampler" algorithm that reweights local training data sampling based on the inverse class distribution of a shared test set, so underrepresented classes (e.g., COVID-19 positive X-rays) are sampled more frequently. The approach is instantiated and evaluated on a COVID-19 chest X-ray classification use case using ResNet50 and GhostNet models trained across three simulated hospital clients, plus a Parity PoA consortium blockchain for the smart contract operations. The contribution is positioned as an architectural design (built on an existing FL reference architecture, FLRA) combined with a concrete fairness algorithm, evaluated for both ML performance/fairness and blockchain operational latency.

## Background

- The paper situates itself within the broader "trustworthy AI" / responsible AI literature, citing Jobin et al.'s survey of global AI ethics guidelines and Thiebes et al.'s work on trustworthy AI as framing for why accountability and fairness matter as core ethical principles for ML systems.
- Federated learning itself is attributed to McMahan et al. (Google, 2016), introduced as a way to train models on decentralized client data without transferring raw data, addressing privacy regulations such as the EU's GDPR.
- The architecture is explicitly built on top of "FLRA," a reference architecture for federated learning systems previously developed by overlapping authors (Lo et al.). The paper also cites prior architectural pattern work for FL systems (Lo et al., arXiv:2101.02373) alongside FLRA as evidence that federated learning is vulnerable to accountability issues, though the architecture's design is explicitly based on FLRA.
- The paper cites the non-IID data problem as characterized by Sattler et al., framing it as the root cause of fairness issues in FL due to skewed/personalized client data distributions.
- Prior blockchain-for-FL accountability work is cited as direct precedent/competitors: FLChain (Bao et al.) for auditable decentralized FL with incentives, Zhang et al.'s blockchain-based FL for IIoT device failure detection, Kang et al.'s reputation-based worker selection scheme, and Kim et al.'s blockchained on-device FL architecture for verifying local model updates.
- The "off-chain data storage" architectural pattern (Xu et al., pattern collection for blockchain applications) is cited as the basis for the design choice to store hashed values on-chain while keeping actual models/data off-chain.
- For the fairness side, the paper references commercial fairness toolkits — Microsoft's Fairlearn and IBM's AI Fairness 360 — as well as academic work on fairness-aware/agnostic federated learning (Mohri et al.; Du et al.) as the broader landscape its weighted sampler contributes to.
- For the COVID-19 use case, the paper builds on a body of prior FL-for-medical-imaging work it cites: Choudhury et al. (adverse drug reaction prediction), Vaid et al. (mortality prediction for hospitalized COVID-19 patients), Liu et al. (conventional FL for COVID-19 chest X-rays), Kumar et al. (blockchain-federated learning with CT imaging), and Zhang et al. (dynamic fusion-based FL for COVID-19 detection) — positioning this paper's contribution relative to these but distinguishing it via its explicit focus on accountability/fairness architecture.
- Model architectures used in evaluation (ResNet50, GhostNet) and the GFL federated learning framework are adopted from existing open-source/published work rather than developed in this paper.

## Key Points

- The paper proposes a blockchain-based trustworthy FL architecture comprising four components — central server, client, blockchain, and a data-model registry smart contract — designed as an extension/instantiation of the existing FLRA reference architecture.
- It introduces a smart contract-driven "data-model provenance registry" that records hashed versions of local training data, local models, and global models, mapping data and local models to the global model versions they contributed to, enabling auditability without exposing raw private data or storing full models on-chain.
- To handle the practical constraints of blockchain (limited block size and on-chain transparency/privacy concerns), the architecture combines hashing (to compress model updates to fixed-length values) with asymmetric/symmetric encryption (clients encrypt hash values before uploading; central server obtains decryption keys via an out-of-scope channel) to preserve confidentiality of model parameters recorded on-chain.
- The paper proposes a "weighted fair training data set sampler" algorithm that computes per-class sampling weights as the inverse of each class's proportion in a shared test dataset, then uses these weights to oversample underrepresented classes (e.g., COVID-19-positive X-rays) during each client's local training epochs.
- The fairness algorithm relies on two stated assumptions: it is designed for horizontal FL settings (same feature space, different sample spaces across clients) rather than vertical FL, and it assumes all clients share a test dataset with the same class distribution so that computed weights are consistent across clients.
- Empirically, on a 21,220-image real-world COVID-19 X-ray dataset split across three simulated clients (each with a normal-skewed, non-IID class distribution), models (ResNet50 and GhostNet) trained with the weighted fair sampler achieved lower training loss and higher training/test accuracy than the same models trained without it, in nearly all of 12 experiments (one exception: ResNet50's test accuracy in one experiment group).
- Confusion matrix analysis on a separately constructed balanced test set (3,600 X-rays, equal per-class) shows that models trained without the fair sampler skew predictions toward "normal," while models trained with the fair sampler produce more correct COVID-19 predictions and higher overall test accuracy, supporting the claim of improved fairness and generalizability.
- The blockchain component (Parity PoA consortium chain, Solidity smart contracts, 5-second block interval, 80M gas limit) was benchmarked across single, parallel, and continuous upload scenarios from three clients, with average upload latency around 5ms and a maximum continuous-upload latency of ~16ms, while read/decryption operations incur negligible latency since they generate no new transactions — supporting the claim that the architecture is operationally feasible at this scale.
- The paper frames its scope explicitly as addressing only the accountability and fairness dimensions of trustworthy/responsible AI, not the full set of responsible AI principles.

## Conclusion

The paper concludes that its blockchain-based architecture is feasible for enabling accountability in federated learning (via the smart contract data-model registry, with acceptable latency overhead from blockchain operations) and that the weighted fair training data sampler measurably improves fairness, generalization, and accuracy relative to a default (unweighted) FL training setup, based on the COVID-19 X-ray classification experiments with ResNet50 and GhostNet. The hypothesis that combining provenance tracking (hashing + encryption on-chain, off-chain model/data storage) with a class-reweighted sampler can jointly improve both auditability and model fairness without prohibitive performance cost is broadly supported by the reported results, though the evaluation is limited to a single use case (COVID-19 chest X-ray detection), a small federated setup (one central server, three clients), and a small consortium blockchain configuration, raising open questions about scalability to larger numbers of clients/participants and generalization to other domains or vertical FL settings (which the authors explicitly note the fairness algorithm is not designed for). The authors acknowledge the work covers only the accountability and fairness aspects of trustworthy AI, leaving other responsible AI principles (e.g., privacy beyond what FL inherently provides, robustness, explainability) unaddressed. They also note that secure key/decryption-key exchange between clients and the central server was left out of scope. As future work, the authors propose exploring incentive mechanisms built on blockchain and smart contracts to further improve fairness and trustworthiness in federated learning systems.
