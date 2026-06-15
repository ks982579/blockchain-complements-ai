---
id: the_applications_of_BL_in_AI
title: "The Applications of Blockchain in Artificial Intelligence"
authors:
  - Ruonan Wang
  - Min Luo
  - Yihong Wen
  - Lianhai Wang
  - Kim-Kwang Raymond Choo
  - Debiao He
year: 2021
venue: "Security and Communication Networks"
publisher: "John Wiley & Sons, Ltd / Hindawi"
volume: 2021
issue: 1
pages: "6126247"
doi: "10.1155/2021/6126247"
url: "https://doi.org/10.1155/2021/6126247"
type: article
---

## Overview

This paper is a survey on how blockchain technology can be applied to address structural problems in artificial intelligence systems, focusing on four application areas: secure data sharing, privacy preservation, trusted AI decision-making, and decentralized AI/intelligence. The authors motivate the survey by noting that AI depends on data, algorithms, and computing power, and that conventional centralized AI architectures suffer from "isolated data islands," single points of failure, opaque/untrustworthy training processes, and vulnerability to malicious or adversarial data. Blockchain's properties — tamper-proofing via hashing and Merkle trees, decentralized consensus, smart contract automation, and cryptographic privacy guarantees — are presented as complementary strengths that can mitigate these AI weaknesses. The paper reviews 27 English-language articles published between 2018 and 2021, selected from over 500 candidates found via keyword searches across IEEE Xplore, ScienceDirect, ACM Digital Library, and SpringerLink, and organizes them into comparison tables by use case, technology, and contribution. Its contribution to the field is primarily organizational/taxonomic: it provides a structured categorization of blockchain-for-AI literature and identifies concrete open research challenges (identity privacy, blockchain performance/scalability, and smart contract security) to guide future work.

## Background

The paper situates itself relative to prior surveys and combined blockchain-AI works, noting that an earlier review (Salah et al. [26]) described integration of blockchain and AI for a decentralized economy but lacked discussion of specific blockchain technologies and privacy protection; other works [27-29] discussed mutual reinforcement of blockchain and AI generally; and [30] examined blockchain-AI integration specifically for energy cloud management without a broader literature categorization. It draws on foundational blockchain concepts including Nakamoto and Wright's original Bitcoin proposal [24], Szabo's definition of smart contracts [34], and standard consensus mechanism literature covering PoW, PoS, DPoS, PBFT, and Raft, as well as the Byzantine Generals Problem [38]. On the AI side, it builds on Samuel's classical definition of machine learning [41] and Google's introduction of federated learning [42] as the basis for privacy-preserving distributed training. It also cites real-world evidence for AI's vulnerabilities, including MIT researchers tricking Google's image classifier into misclassifying a 3D-printed turtle as a rifle [17] and adversarial poisoning of biometric recognition systems [18], framing these as motivations for blockchain-based trust and data-integrity mechanisms. For its discussion of open challenges, the paper references work on de-anonymizing Bitcoin transactions via behavior-based clustering (Androulaki et al. [76], matching 40% of student identities to blockchain addresses), Monero's ring-signature-based anonymity (RingCT [77]), blockchain sharding [78] and sidechains [79] for scalability, and documented smart contract vulnerabilities such as the 2016 DAO attack that resulted in the loss of 3.6 million ether [83]. It also references commercial blockchain-AI platforms (SingularityNet [31], TraneAi [32], Neureal [33]) as evidence of industry interest in decentralized AI ecosystems.

## Key Points

- This paper categorizes blockchain-AI integration literature (27 articles, 2018-2021) into four application domains: data sharing, privacy preserving, trusted AI decision, and decentralized intelligence, each with a comparison table of use case, technologies, and contributions.
- This paper argues that blockchain's decentralized, peer-to-peer architecture enables data owners and users to share or trade data directly (e.g., IoT data marketplaces [43], SecNet [44], RoboChain [45]) without relying on a trusted central intermediary, reducing fraud risk via transparency and immutability.
- This paper shows that cryptographic techniques layered on blockchain — including hashing, homomorphic encryption (e.g., Paillier encryption for SVM training [53]), and threshold encryption (e.g., Mamoshina et al.'s healthcare data marketplace [54]) — can be used to preserve confidentiality, integrity, and authenticity of sensitive AI training data, including in healthcare contexts (DeepLinQ [49], ModelChain [51]).
- This paper documents how smart contracts can automate and record the entire AI lifecycle (model initialization, training, validation, scoring, reward allocation), as in Wang's trusted ML analysis framework [59], thereby improving the credibility and traceability of AI training and decision outcomes.
- This paper identifies blockchain-based mechanisms for managing Byzantine or unreliable participants in distributed AI, including swarm robot governance via smart contracts [55], reputation management for federated learning workers via consortium blockchain [68], and Byzantine-attack-resistant multiparty learning systems such as BEMA [63].
- This paper surveys incentive-mechanism designs that use blockchain tokens/transactions to motivate participation in distributed AI training, including edge-computing reward schemes [60], priority-based task scheduling with rewards [61], and collaborative AI loss-function-based payment schemes [69].
- This paper reviews decentralized intelligence architectures combining blockchain with federated learning (LearningChain [62], DeepChain [64], FLchain [67]) and edge-AI knowledge markets (k-chain with Proof of Trading consensus [65]), as well as automated machine-learning model marketplaces using Ethereum smart contracts [70].
- This paper proposes a high-level synthesis (Table 1) mapping specific blockchain characteristics (trust, security, automation, immutability, traceability, decentralization) to specific AI needs (trustworthy/secure data, credible training, distributed computing power).
- This paper identifies three open research challenges for blockchain-AI integration: (1) identity privacy preservation, which is largely neglected by existing schemes despite known de-anonymization risks in pseudonymous blockchains; (2) performance/scalability limitations of blockchain storage and transaction throughput (e.g., Bitcoin ~7 tx/s, Ethereum ~7-15 tx/s) relative to AI's data and time-sensitive needs; and (3) smart contract security, given documented vulnerabilities and attacks, suggesting AI-based vulnerability detection/repair as a potential research direction.

## Conclusion

The paper concludes that blockchain can meaningfully benefit AI across the four surveyed dimensions — supporting peer-to-peer data sharing/trading, enhancing privacy and data integrity through cryptographic mechanisms, increasing the credibility and traceability of AI decisions via smart contracts, and enabling incentive-driven decentralized/collaborative AI training. These conclusions are supported by synthesizing concrete examples from the 27 reviewed articles rather than original experiments, so the paper's "findings" are essentially a structured literature-based argument rather than empirical results of its own. The authors explicitly flag several unresolved issues as directions for future research: identity privacy preservation in blockchain-based AI systems remains largely unaddressed even though pseudonymous identities can be de-anonymized through transaction analysis; blockchain scalability (storage limits and low transaction throughput) constrains its practicality for data- and time-intensive AI workloads, with sharding, sidechains, private/consortium chains, and improved consensus/incentive design suggested as partial mitigations; and smart contract security remains an ongoing concern given real-world exploits like the DAO attack, with the authors posing an open question about whether AI itself could be used to detect and repair smart contract vulnerabilities — a potentially fruitful reciprocal research direction (AI for blockchain, in addition to blockchain for AI).
