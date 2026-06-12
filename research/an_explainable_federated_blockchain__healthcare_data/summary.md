---
id: an_explainable_federated_blockchain__healthcare_data
title: "An explainable federated blockchain framework with privacy-preserving AI optimization for securing healthcare data"
authors:
  - Tanisha Bhardwaj
  - K. Sumangali
year: 2025
venue: "Scientific Reports"
volume: 15
issue: 1
pages: "21799"
doi: "10.1038/s41598-025-04083-4"
url: "https://doi.org/10.1038/s41598-025-04083-4"
type: article
keywords:
  - Blockchain
  - Consensus Mechanism
  - Data Protection
  - Decentralized Learning
  - Differential Privacy (DP)
  - Federated Learning (FL)
  - Neural Network
  - Privacy Security
---

## Overview

This paper introduces PPFBXAIO (Privacy Preserving Federated Blockchain Explainable Artificial Intelligence Optimization), a unified framework for decentralized healthcare AI that integrates federated learning (FL), blockchain, explainable AI (XAI), and metaheuristic optimization to address the trust, privacy, transparency, and adversarial-robustness gaps that plague conventional FL systems in clinical settings. The framework uses SHA-256 hashing and a permissioned blockchain with lightweight consensus (e.g., PBFT) to immutably log model updates and SHAP-derived explainability metadata for auditability, Min-Max normalization for feature scaling, the Lévy Grasshopper Optimization Algorithm (LGOA) for feature selection and federated hyperparameter tuning, and an Entropy Deep Belief Network (EDBN) classifier for disease prediction and attack detection. Its central novelty is an Explainability-Guided Aggregation (EGA) scheme that feeds client-level SHAP attributions back into the federated averaging and optimization loop, creating a closed feedback cycle linking interpretability, model governance, and on-chain traceability—something prior FL-blockchain or FL-XAI work has not combined. Evaluated on the Kaggle Heart Disease and Wisconsin Breast Cancer datasets against FedAvg, FL-MPC, FL-RAEC, PEFL, and PPBEFL under extra-noise, label-flipping, STAT-OPT, and DYN-OPT adversarial attacks, PPFBXAIO achieves up to 95.07% accuracy (breast cancer) and 93.07% (heart disease), with consistently lower training loss, ~81 ms reduced latency, ~109 transactions/second higher throughput over 100 rounds, and roughly an 18% reduction in total training time despite the added XAI/blockchain overhead. The work contributes a practical, auditable architecture demonstrating that explainability and blockchain-based provenance can be co-designed with FL optimization to meet healthcare's regulatory and trust requirements while improving robustness to poisoning and privacy attacks.

## Background

This paper situates its proposed framework (PPFBXAIO) within several strands of prior work on AI, federated learning (FL), blockchain, and explainable AI (XAI), which it draws on both for motivation and for comparison baselines.

**AI's growing role in healthcare and society.** The paper cites general literature on AI adoption and trust — e.g., work on AI's disruptive societal and economic impact, technology acceptance for AI products, and human trust in AI (Kim et al.; Sohn & Kwon; Glikson & Woolley) — and on AI's expanding role in medicine and clinical decision-making (Secinaro et al.; Kuwaiti et al.), as background for why interpretability and trust matter in healthcare AI.

**Federated learning fundamentals.** The paper draws on standard FL framings — decentralized training across nodes without sharing raw data, the three-step process of task initialization, local training, and global aggregation — citing surveys and overviews of FL (Lazaros et al.; Xu et al. on FL for healthcare informatics; Li et al. on FL challenges and methods; Yurdem et al. on FL strategies and tools). It also references the Flower framework as the FL implementation platform it builds on, and cites Kong et al. for the standard three-phase FL training procedure.

**Blockchain-FL integration.** A core premise — that blockchain complements FL by addressing single-point failures, poisoning vulnerability, and lack of incentive mechanisms — is drawn from surveys on blockchain-empowered federated learning (Zhu et al.; Nguyen et al.; Issa et al.). These sources are cited as establishing that blockchain's immutable ledger and consensus mechanisms can validate model updates, reject malicious global models, and create tamper-proof records, which the paper treats as an established but underexplored complement to FL.

**XAI motivations and gaps.** The paper cites work establishing that XAI is necessary to address the "black-box" nature of AI and to build regulatory compliance and stakeholder trust (general XAI literature), while also noting (via López-Blanco et al.'s review of federated XAI, and Rahman et al.'s survey of FL-based healthcare AI) that existing XAI approaches lack consistency and transparency, and that the combination of XAI with blockchain-based FL has been largely unexplored — this gap is the explicit motivation for the proposed framework.

**Specific prior systems referenced for comparison and inspiration.** The literature review section surveys roughly twenty prior works that the paper positions itself against or builds on conceptually:
- Wang et al.'s PF-PoFL — an energy-recycling consensus mechanism for FL that uses differential privacy and game-theoretic federation formation, cited as an example of blockchain consensus innovation for FL.
- Singh et al. — blockchain-IoT-cloud integration with FL for smart healthcare.
- Djolev et al.'s FBLearn — a blockchain/IPFS-based FL platform with smart-contract-mediated aggregation (logistic regression and random forest use cases).
- Gupta et al. — a blockchain+FL framework using DenseNet-201 and FedAvg for lung disease classification, with IPFS storage.
- Dipto et al. — federated detection of red blood cell abnormalities using Grad-CAM-based XAI for result verification.
- Lohachab & Kumar's FedHFP — a federated deep learning model (comparing LSTM, RNN, CNN, ANN, GRU) for heart failure prediction, used later as a comparison baseline (FedHFP+RNN, FedHFP+LSTM).
- Wei et al.'s DeFedHDP — a fully decentralized FL aggregation method with differential privacy and One-Point Bandit Feedback for heart disease prediction, also used as a baseline (DeFedHDP+EDBN).
- Otoum et al. — TabNet combined with FL and blockchain for heart disease prediction, used as a baseline (FedAvgBC+TabNet).
- Khan et al. — an Iterative Artificial Bee Colony algorithm combined with FL for cardiac disease prediction.
- Asad & Otoum's BPPFL — a blockchain framework combining threshold Paillier encryption and threshold signature authentication for privacy-preserving FL.
- Liu et al. (Miao et al.) — FL-RAEC, a robust-aggregation framework using autoencoder-based anomaly detection and phased aggregation, used as a key comparison baseline throughout the results.
- Tian et al. — PEFL, a privacy-preserving FL-blockchain framework using a model-validated fault-tolerant federation (MFF) consensus, also a primary comparison baseline.
- Multiple works by Aitizaz Ali et al. — blockchain-enabled secure searchable encryption for healthcare IoT, blockchain-based security/privacy/reliability frameworks for digital healthcare, the HealthLock system combining blockchain with homomorphic encryption for IoT healthcare privacy, and a hybrid deep learning model for industrial IoMT using consortium blockchain and homomorphic encryption.
- Almaiah et al. — a decentralized authentication and data preservation model for IoT-based cyber-physical healthcare systems.
- Hasan et al. (multiple works) — cognitive IoT architectures for smart cities, blockchain-based federated safety-as-a-service for industrial IoT, and unsupervised ML for industrial machine failure detection.

**Algorithmic and methodological building blocks cited from elsewhere.** The paper's technical components are explicitly grounded in external sources:
- The Levy Grasshopper Optimization Algorithm (LGOA) used for feature selection and hyperparameter tuning is adopted from Wu, Wu & Wang's work enhancing the Grasshopper Optimization Algorithm with Lévy flight for engineering applications — all core equations for grasshopper swarm position updates, gravitational/wind components, and the Lévy flight step-size derivation are cited from this source.
- The Entropy Deep Belief Network (EDBN) structure-optimization formulation (information entropy bounds on hidden-layer neuron counts, reconstruction error minimization) is drawn from Jiang et al.'s work on DBN structure design based on information entropy and reconstruction error.
- SHA-256 hashing is described using its standard cryptographic properties (collision resistance, avalanche effect, one-way function) as generally established in cryptography literature, applied here to blockchain block validation.

**Attack models used for evaluation.** The four adversarial scenarios used to stress-test the framework are drawn from prior attack-model literature: the "extra noise" attack is attributed to Liu, W. et al.'s work on privacy preservation for federated learning with robust aggregation in edge computing; the label-flipping attack is attributed to Shayan et al.'s Biscotti blockchain FL system; the static optimization (STAT-OPT) attack is drawn from Fang et al.'s work on local model poisoning attacks against Byzantine-robust FL; and the dynamic optimization (DYN-OPT) attack is drawn from Shejwalkar & Houmansadr's work on optimizing model poisoning attacks and defenses for FL.

**Baseline comparison schemes.** Throughout the experimental results, the paper benchmarks PPFBXAIO against established FL schemes and frameworks from the cited literature — FedAvg (standard federated averaging), FL-MPC (FL with multi-party computation), FL-RAEC (Liu et al./Miao et al.), PEFL (Tian et al.), and PPBEFL (privacy-preserving blockchain-enabled FL) — as well as against the disease-prediction pipelines FedHFP+RNN/LSTM (Lohachab & Kumar), DeFedHDP+EDBN (Wei et al.), and FedAvgBC+TabNet (Otoum et al.). A researcher wanting to verify or extend this work should examine these baseline papers directly, since the reported performance gaps (accuracy, loss, latency, throughput) are relative to these specific prior implementations.

**Datasets.** The Heart Disease dataset (Cleveland, Hungary, Switzerland, and Long Beach V databases, sourced from Kaggle/UCI) and the Wisconsin Breast Cancer dataset (UCI, via Kaggle) are standard publicly available benchmarks used elsewhere in the FL-healthcare literature (e.g., used by Otoum et al. and Lohachab & Kumar), which the paper reuses for comparability. The paper also references larger, more realistic datasets (MIMIC-III EHR, NIH Chest X-ray14, COVIDx) as external benchmarks for a follow-up scalability evaluation, though these are mentioned only briefly relative to the main Kaggle-based experiments.

## Key Points

- This paper proposes PPFBXAIO (Privacy Preserving Federated Blockchain Explainable Artificial Intelligence Optimization), a unified framework integrating blockchain, explainable AI (XAI), and optimization into federated learning (FL) for secure, interpretable, decentralized healthcare AI.
- The framework's central novelty is the co-design of explainability and optimization with blockchain-enhanced FL: client-level SHAP-based explanations are fed back into the optimization loop (via an Explainability-Guided Aggregation, EGA, mechanism) and logged on-chain as cryptographic hashes for tamper-proof auditability, rather than treating explainability as a post-hoc add-on.
- PPFBXAIO uses SHA-256 for blockchain-backed secure model updates, Min-Max normalization for feature scaling, the Lévy Grasshopper Optimization Algorithm (LGOA) for feature selection and hyperparameter tuning of federated averaging, and an Entropy Deep Belief Network (EDBN) as the classifier for disease prediction and attack detection.
- On the Wisconsin Breast Cancer dataset, PPFBXAIO+EDBN achieves 95.07% accuracy, 95.44% precision, 96.54% recall, and 95.98% F1-score; on the Heart Disease dataset it achieves 93.07% accuracy, 91.19% precision, 95.39% recall, and 93.24% F1-score, outperforming FedHFP+RNN, FedHFP+LSTM, DeFedHDP+EDBN, and FedAvgBC+TabNet.
- Under adversarial conditions (extra noise, label-flipping, STAT-OPT, and DYN-OPT attacks), PPFBXAIO achieves the highest average accuracy (94.19% on Heart Disease and 95.59% on Breast Cancer Wisconsin) and lowest average loss (5.81% and 4.41%, respectively), outperforming FedAvg, FL-MPC, FL-RAEC, PEFL, and PPBEFL.
- PPFBXAIO reduces latency to 81 ms and improves throughput to 109 transactions/second at 100 federated rounds, outperforming FedAvg, FL-MPC, FL-RAEC, PEFL, and PPBEFL across 25-100 rounds.
- LGOA-based optimization reduces total training time by approximately 18% compared to standard federated SGD by requiring fewer global rounds to converge, despite a moderate per-round increase from the XAI module.
- Integrating blockchain (consensus validation, block creation, ledger updates) introduces roughly a 10-15% increase in per-round communication latency compared to FL without blockchain.
- The framework introduces a Proof-of-Quality (PoQ) consensus mechanism that extends Proof-of-Contribution by incorporating model divergence, historical trust scores, and explanation consistency across rounds.
- A local differential privacy mechanism based on RAPPOR (Randomized Aggregate Privacy-Preserving Ordinal Response), combined with homomorphic encryption, is used to protect client data during model update aggregation, with theoretical MSE derivations provided for the privacy estimator.
- Explanation outputs (SHAP, LIME, Grad-CAM) are sanitized into quantized/ranked feature-importance summaries, kept local to each node, and only their cryptographic hashes are recorded on a permissioned blockchain (validated via PBFT or similar lightweight consensus), preserving privacy while enabling auditability of model rationale.
- An informal user study with seven medical experts found that 71% of participants could correctly predict the model's decisions when given SHAP or Grad-CAM explanations, versus 34% without such explanations, supporting the clinical value of the explainability layer.
- Explanation stability tests under modest input perturbations showed an average cosine similarity of 0.89, and a correlation of 0.83 was found between feature-removal-based faithfulness and reduction in model confidence, indicating consistent and faithful explanations.
- An ablation study shows that removing LGOA significantly reduces classification accuracy and convergence speed, removing the XAI module preserves accuracy but eliminates model interpretability/traceability, and removing blockchain degrades privacy assurance and security by reintroducing single-point-failure and tampering risks—demonstrating each component is integral to overall framework performance.
- Scalability validation on larger, real-world datasets (MIMIC-III EHR, NIH Chest X-ray14, COVIDx) under non-IID, simulated multi-hospital/multi-device partitions showed PPFBXAIO maintains accuracy comparable to centralized training while preserving privacy guarantees and blockchain-based provenance/auditability.
- The paper provides a feature comparison table positioning PPFBXAIO against generic FL+blockchain and XAI-for-medical-FL frameworks, arguing PPFBXAIO uniquely combines integrated SHAP-based local/global explanations, on-chain logging of explanation metadata, federated optimization guided by interpretability scores, and personalized federated models guided by explainability.

## Conclusion

The paper concludes that its proposed PPFBXAIO framework — which fuses federated learning, blockchain, explainable AI (XAI), and a Levy Grasshopper Optimization Algorithm (LGOA) for feature selection/hyperparameter tuning, with an Entropy Deep Belief Network (EDBN) as classifier — successfully addresses the stated gaps in prior FL-blockchain-XAI work: lack of privacy, transparency, traceability, and robustness to adversarial attacks. The authors report that PPFBXAIO outperformed baseline and competing approaches (FedAvg, FL-MPC, FL-RAEC, PEFL, PPBEFL, and disease-prediction baselines such as FedHFP+RNN/LSTM, DeFedHDP+EDBN, FedAvgBC+TabNet) across both the Heart Disease and Wisconsin Breast Cancer datasets, achieving the headline figures cited in the abstract (95.07% accuracy / 95.44% precision / 96.54% recall / 95.98% F1 on Breast Cancer; 93.07% accuracy / 91.19% precision / 95.39% recall / 93.24% F1 on Heart Disease), along with lower loss, lower latency (down to 81 ms at 100 rounds), and higher throughput (109 transactions/sec at 100 rounds) than comparator schemes, and improved resilience under extra-noise, label-flipping, STAT-OPT, and DYN-OPT poisoning attacks.

The paper's central claims appear supported by its own reported results and ablation study: removing LGOA degraded accuracy/convergence, removing the XAI module preserved raw accuracy but eliminated interpretability/traceability for clinicians, and removing blockchain reintroduced single-point-of-failure and tampering risks — together the ablation is used to argue that each of the three components (optimization, explainability, blockchain) is "integral" to the framework's overall performance and trustworthiness claims. The authors also report a small qualitative validation: a 0.83 correlation between feature-removal "faithfulness" and confidence drop, 0.89 cosine similarity for explanation stability under perturbation, and a brief seven-expert user study where SHAP/Grad-CAM explanations improved clinicians' ability to predict model decisions (71% vs 34% without explanations) — offered as evidence that the explainability layer increases clinical interpretability and trust. The paper also claims (without much methodological detail) that the approach generalizes to larger real-world datasets (MIMIC-III, NIH Chest X-ray14, COVIDx) under non-IID partitioning while preserving accuracy comparable to centralized training.

Despite these positive results, the authors explicitly acknowledge significant limitations and open questions, framed as future work:

- **Computational and communication overhead**: blockchain integration (consensus validation, block creation, ledger updates) adds roughly 10–15% communication latency per round, and the XAI module plus LGOA increase per-round training time, raising scalability and real-time-deployment concerns — particularly for resource-constrained settings such as hospitals without specialized IT infrastructure or Wireless Sensor Network (WSN) deployments with limited sensor-node compute/communication budgets.
- **Synchronization costs grow with the number of nodes**, which the authors flag as a barrier to large-scale network performance.
- **Future research directions** explicitly proposed include: lightweight consensus mechanisms (e.g., Proof-of-Authority) to reduce blockchain overhead; off-chain storage to reduce blockchain bloat; incorporation of multi-modal medical data (ECG, X-ray, EHR) for improved diagnostic performance; integration of more advanced XAI techniques to further improve interpretability and clinician trust; and conducting clinical usability studies to validate the system in real-world deployment environments (the current evaluation relies on Kaggle datasets and a brief informal expert survey rather than a full clinical trial).
- The paper also notes practical deployment caveats — e.g., the need for modular activation of blockchain/XAI components based on institutional capability, client-side drift detection for non-IID data heterogeneity, and additional sanitization/differential-privacy steps before on-chain logging of explanation metadata, since even sanitized SHAP summaries could pose residual privacy risks.

Overall, the paper frames PPFBXAIO as a proof-of-concept that demonstrates the feasibility and measurable benefits of co-designing explainability, optimization, and blockchain-based trust mechanisms within a federated healthcare AI pipeline, while explicitly leaving real-world scalability, multi-modal data integration, lighter-weight consensus, and clinical validation as unresolved problems for future work.
