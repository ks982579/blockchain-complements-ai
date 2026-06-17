---
id: federated_learning-strategies_for_improving_comm_efficiency
title: "Federated Learning: Strategies for Improving Communication Efficiency"
authors:
  - Jakub Konečný
  - H. Brendan McMahan
  - Felix X. Yu
  - Peter Richtárik
  - Ananda Theertha Suresh
  - Dave Bacon
year: 2016
venue: "NIPS Workshop on Private Multi-Party Machine Learning"
doi: "10.48550/arXiv.1610.05492"
url: "https://arxiv.org/abs/1610.05492"
type: inproceedings
keywords:
  - federated learning
  - communication efficiency
  - distributed machine learning
  - structured updates
  - sketched updates
  - quantization
  - compression
---

## Overview

This paper by Konecny, McMahan, Yu, Suresh, Bacon, and Richtarik addresses a core bottleneck in Federated Learning (FL): the high uplink communication cost of transmitting model updates from client devices (typically mobile phones) to a central server. Because uplink bandwidth is substantially slower than downlink in practice and cryptographic aggregation protocols compound the cost, sending full model updates each round is prohibitive at scale. The paper proposes and evaluates two families of compression strategies — structured updates and sketched updates — showing that their combination can reduce uplink communication by up to two orders of magnitude with only modest accuracy loss on both convolutional (CIFAR-10) and recurrent (Reddit next-word prediction) network tasks. A key secondary finding is that preprocessing updates with structured random rotations (Walsh-Hadamard transforms combined with random binary diagonal matrices) consistently improves compression quality, and this step is computationally negligible in the FL setting relative to local training, unlike in conventional parallel SGD.

## Background

This paper builds extensively on prior work in federated and distributed learning. It grounds its problem setting in the FedAvg algorithm (McMahan et al., 2017), which the experiments use as the base training procedure and which established that multiple local SGD steps per round can greatly reduce communication rounds. The broader FL framework is drawn from Konecny et al. (2016) and McMahan & Ramage (2017), which introduced the federated setting for on-device intelligence. The paper cites conventional distributed ML baselines (Dean et al., 2012 on large-scale distributed deep networks; Chilimbi et al., 2014 on Project Adam; Shamir et al., 2014 on approximate Newton methods; Reddi et al., 2016 on AIDE; Ma et al., 2017 on distributed optimization with arbitrary local solvers; Zhang & Lin, 2015 on DiSCO) to contrast the controlled data-center setting with the challenging FL setting of non-i.i.d., unbalanced data and slow networks. For quantization, the paper builds on Alistarh et al. (2016) (QSGD) and Konecny & Richtarik (2016) as prior work on randomized quantization for communication-efficient SGD, and on Suresh et al. (2017) for theoretical analysis of distributed mean estimation with limited communication, which provides the theoretical backing for random rotation reducing quantization error by O(d / log d). The use of Walsh-Hadamard structured rotations to make the rotation computationally feasible (O(d log d) instead of O(d^2)) also comes from Suresh et al. (2017). For the concept of sketching more broadly, the paper cites Woodruff (2014). Model compression for the download direction is attributed to Han et al. (2015). Secure aggregation protocols that increase upload cost are attributed to Bonawitz et al. (2017). Low-rank parameter prediction for neural networks is related to Denil et al. (2013). The Reddit dataset construction follows Al-Rfou et al. (2016), and the all-convolutional CIFAR model is from Springenberg et al. (2014).

## Key Points

- Structured updates constrain each client's model update H to a restricted parameter space, either as a low-rank matrix (H = A*B, where A is a random fixed matrix transmitted as a seed and B is the only trained and communicated component) or as a sparse matrix following a random mask (also seed-representable), directly saving a factor of d1/k or 1/sparsity in uplink bits.
- Random mask structured updates substantially outperform low-rank structured updates as compression increases; random mask convergence speed (in rounds) is essentially unaffected even at aggressive compression levels.
- Sketched updates compute a full unconstrained local update and then compress it via a combination of three compatible tools: subsampling (sending only a random subset of values), probabilistic quantization (stochastic rounding to b-bit representations, with 1-bit quantization providing 32x compression over 4-byte floats), and structured random rotations before quantization.
- Preprocessing updates with a structured random rotation (product of Walsh-Hadamard matrix and random binary diagonal matrix) consistently and substantially improves quantization quality by equalizing value scales across dimensions, reducing quantization error by a theoretical factor of O(d / log d); this step costs only O(d log d) and is negligible relative to local training in the FL setting.
- Combining subsampling at 6.25% retention, 2-bit quantization, and random rotations achieves a 256x reduction in bits per layer update on CIFAR with only minor degradation in convergence.
- There is a practical tradeoff between number of clients per round and per-client communication: selecting more clients with more aggressive subsampling can match the accuracy of fewer clients sending fuller updates, which is advantageous when many clients have very limited upload bandwidth.
- On the Reddit LSTM task (763,430 users, 1.35M parameter model), the proposed sketching methods with 2-bit quantization and randomized Hadamard transforms incur no measurable loss relative to uncompressed FedAvg, even before every user's data has been touched once across 2,000 rounds.
- Structured and sketched approaches can in principle be combined (e.g., learn a structured update and then sketch it), though the paper does not experimentally evaluate this combination.

## Conclusion

The paper's central claim — that uplink communication cost in Federated Learning can be reduced by approximately two orders of magnitude without significant accuracy loss — is supported by experiments on both CIFAR-10 (convolutional model) and Reddit (LSTM). Random mask structured updates prove more effective than low-rank updates at high compression. Among sketching components, random rotations provide the most consistent benefit and are uniquely cost-effective in the FL setting. The paper acknowledges that sketched updates introduce variance (and therefore a theoretical ceiling on final accuracy slightly below uncompressed training), while structured updates can in principle converge to the same accuracy. Limitations noted include the use of artificial data partitioning for CIFAR, the gap between simulated and real deployment, and the unexplored combination of structured and sketched updates. Open research directions implied by the work include: theoretical convergence analysis of the proposed methods under non-i.i.d. data; principled methods for choosing compression hyperparameters (rank, sparsity, quantization bits) per layer or per round; integration with secure aggregation and differential privacy protocols; and extension to the download direction. The Reddit experiment demonstrates that practical FL at scale can complete useful training before exhausting any user's data, suggesting that communication compression is a prerequisite — not merely an optimization — for real-world FL deployment.
