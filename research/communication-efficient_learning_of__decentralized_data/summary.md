---
id: communication-efficient_learning_of__decentralized_data
title: "Communication-Efficient Learning of Deep Networks from Decentralized Data"
authors:
  - H. Brendan McMahan
  - Eider Moore
  - Daniel Ramage
  - Seth Hampson
  - Blaise Agüera y Arcas
year: 2017
venue: "Proceedings of the 20th International Conference on Artificial Intelligence and Statistics (AISTATS)"
doi: "10.48550/arXiv.1602.05629"
url: "https://arxiv.org/abs/1602.05629"
type: inproceedings
keywords:
  - federated learning
  - communication efficiency
  - decentralized data
  - deep networks
  - FedAvg
---

## Overview

This paper by McMahan et al. (Google, AISTATS 2017) introduces Federated Learning, a paradigm for training deep neural networks on data that remains distributed across mobile devices rather than being uploaded to a central server. The core problem addressed is the tension between the richness of on-device user data (which is privacy-sensitive and often too large to centralize) and the need for that data to improve shared models. The paper's central contribution is the FederatedAveraging (FedAvg) algorithm, which combines local SGD on each participating client with server-side weighted model averaging, and demonstrates empirically that this reduces required communication rounds by 10-100x compared to synchronized SGD baselines. Experiments span five model architectures (MLP, two CNNs, character LSTM, word LSTM) and four datasets (MNIST, CIFAR-10, Shakespeare corpus, large-scale social network posts), confirming robustness under unbalanced and non-IID data distributions that are characteristic of real-world federated settings.

## Background

This paper builds on and situates itself against a substantial body of prior work:

- Distributed model averaging (iterative averaging of locally trained models) was previously studied by McDonald et al. (2010, for perceptrons) and Povey et al. (2015, for speech recognition DNNs), and asynchronously by Zhang et al. (2015, elastic averaging SGD) — but only in data center settings with at most 16 workers, fast networks, and IID/balanced data, none of which apply to the federated setting (as this paper notes).
- Shokri and Shmatikov (2015) addressed privacy-preserving deep learning by sharing only a subset of parameters per round, but did not consider non-IID or unbalanced data and provided limited empirical evaluation.
- Neverova et al. (2016) similarly motivated keeping sensitive data on-device but did not develop a general optimization framework.
- Extensive prior work on convex distributed optimization (Balcan et al., Fercoq et al., Shamir et al., Yang, Zhang et al.) addresses communication efficiency, but assumes IID data, balanced client datasets, and far fewer clients than examples per client — all assumptions violated in the federated setting.
- Large-batch synchronous SGD (Chen et al., 2016) was shown to be state-of-the-art in data center settings and serves as the paper's FedSGD baseline.
- One-shot model averaging is known in the worst case to be no better than training on a single client (Zhang et al. 2012, Arjevani and Shamir 2015, Zinkevich et al. 2010), motivating the iterative averaging approach.
- The paper draws on Goodfellow et al. (2015) for analysis of neural network loss surface geometry to justify why parameter averaging from a shared initialization works in practice for non-convex models.
- The 2012 White House report on consumer data privacy is cited to ground the principle of "focused collection" / data minimization that federated learning embodies.
- Dropout (Srivastava et al., 2014) is referenced as an analogy for the conjectured regularization benefit of model averaging.
- Future integration with differential privacy (Dwork and Roth, 2014) and secure multiparty computation (Goryczka et al., 2013) is noted as a natural extension; Bonawitz et al. (2016) introduced a secure aggregation protocol for federated learning subsequent to this work.

## Key Points

- The paper coins the term "Federated Learning" and formally defines the federated optimization problem, characterized by non-IID data, unbalanced client datasets, massively distributed clients, and limited/expensive communication.
- FedAvg is parameterized by three quantities: C (fraction of clients selected per round), E (number of local training epochs per round), and B (local minibatch size); increasing local computation trades communication rounds for local compute.
- FedAvg achieves 10-100x reductions in communication rounds compared to FedSGD (large-batch synchronized SGD) across all tested architectures and datasets.
- On IID MNIST data, FedAvg reduces rounds to reach target accuracy by up to 35x (CNN) and 46x (2NN) relative to FedSGD; on pathological non-IID MNIST (each client holds only 2 digit classes), speedups are smaller but still substantial (2.8-3.7x).
- On the Shakespeare LSTM (a naturally unbalanced non-IID dataset), FedAvg achieves a 95x speedup on non-IID data versus only 13x on the balanced IID version, suggesting that real-world heterogeneous non-IID distributions can actually benefit more from local training.
- On a large-scale word LSTM trained on 10 million social network posts (500,000+ clients), FedAvg reaches 10.5% accuracy in 35 rounds versus 820 rounds for FedSGD — a 23x reduction.
- FedAvg consistently converges to higher final test accuracy than FedSGD (e.g., 99.44% vs. 99.22% on MNIST CNN), suggesting model averaging provides a regularization benefit analogous to dropout.
- Shared initialization is critical: parameter averaging from models trained from a common starting point produces good models; averaging models from different random initializations can produce arbitrarily bad models, consistent with the non-convex loss surface analysis from Goodfellow et al.
- Excessive local computation (very large E) can cause FedAvg to plateau or diverge, suggesting that decaying local computation per round (analogous to learning rate decay) may be beneficial in later convergence stages.
- The approach is grounded in the "data minimization" principle from the 2012 White House privacy report, since only model updates (not raw data) are ever transmitted to the server, substantially reducing the attack surface.

## Conclusion

The paper's central hypothesis — that iterative federated model averaging can dramatically reduce communication costs while remaining robust to non-IID and unbalanced data — is strongly supported across five architectures and four datasets. FedAvg achieves 10-100x communication round reductions in all tested settings, and the non-IID robustness holds even under pathological conditions (clients holding data from only 2 digit classes). The paper acknowledges several open limitations and future directions: (1) the current work does not address client dropout, corrupted updates, or temporally shifting local datasets in deployment; (2) providing formal privacy guarantees beyond practical data minimization — via differential privacy and/or secure multiparty computation — is explicitly flagged as important future work; (3) the observation that very large E can cause divergence points to an open question about optimal scheduling of local computation across training rounds; (4) the conjecture that model averaging provides dropout-like regularization is unproven and merits further investigation; and (5) the paper notes subsequent work (Bonawitz et al., Konecny et al.) has already extended FedAvg with secure aggregation and further communication-cost reductions, indicating an active and rapidly developing research space.
