---
id: securing_federated_learning_with_blockchains-a_systematic_literature_review
title: "Securing federated learning with blockchain: a systematic literature review"
authors:
  - Attia Qammar
  - Ahmad Karim
  - Huansheng Ning
  - Jianguo Ding
year: 2023
venue: "Artificial Intelligence Review"
publisher: "Springer"
volume: 56
issue: 5
pages: "3951-3985"
doi: "10.1007/s10462-022-10271-9"
url: "https://research.ebsco.com/linkprocessor/plink?id=71c9a9c1-ae82-35a0-ae3c-66d0925fc74f"
type: article
keywords:
  - Federated learning
  - Blockchain
  - Security
  - Privacy
  - Blockchain-based FL
  - Systematic literature review
---

## Overview

This paper (Qammar, Karim, Ning, Ding; *Artificial Intelligence Review*, 2023) is a systematic literature review (SLR) of how blockchain can be integrated into Federated Learning (FL) to compensate for FL's inherent security, privacy, and trust limitations. FL trains a shared global model by aggregating locally computed model updates without sharing raw data, but its reliance on a central aggregation server creates single-point-of-failure risk, offers no transparent record of model updates, and lacks any incentive mechanism to motivate honest participation. The authors argue that blockchain's decentralization, immutability, traceability, consensus, smart contracts, and cryptocurrency-based incentives directly address these gaps. Following Kitchenham's SLR methodology and a PRISMA screening process, they distill 41 studies (from 1,298 identified, 2016–June 2022) and organize blockchain-based FL (BCFL) approaches into three themes: (1) security and privacy, (2) records and rewards, and (3) verification and accountability. The review further catalogs FL attack types, BCFL architecture and workflow, deployment frameworks, and concludes with nine open issues and five future research directions, claiming to be the first SLR specifically on securing FL with blockchain.

## Background

The paper builds heavily on prior work it cites to frame both FL and blockchain. It credits McMahan et al. (2017) with introducing FL at Google and the FedAvg aggregation algorithm, and cites variants such as FedProx (Li et al. 2020c) for system heterogeneity, FedMa, and FedOpt (Wang et al. 2020; Asad et al. 2020). It references Yang et al. (2019) for the three FL categories—Horizontal (HFL), Vertical (VFL), and Federated Transfer Learning (FTL)—and points to FL applications in healthcare, finance, transportation, NLP, and smart cities (Xu et al. 2020; Chen et al. 2020; Long et al. 2020 and others). For blockchain, it draws on Zheng et al. (2017) and Niranjanamurthy et al. (2018) for the public/private/consortium taxonomy, Xiao et al. (2020) for the five components of consensus algorithms, and Khan et al. (2021) for smart contracts. The paper distinguishes itself from a related prior survey by Li et al. (2021a), which discussed BCFL architecture/types/incentives but, the authors say, was not a systematic review covering security, incentives, attack detection, and defense together. It also cites attack-specific literature (e.g., Fraboni et al. 2021a on free-riding, Wang et al. 2019a on eavesdropping, Eyal and Sirer 2014 and Sapirshtein et al. 2017 on selfish mining) and tools/techniques it recommends from elsewhere, such as Blanchard et al. (2017) multi-Krum defense, Ben-Sasson et al. (2018) ZK-STARKs, ZEUS smart-contract verification (Kalra et al. 2018), and IPFS (Benet 2021) for off-chain storage.

## Key Points

- The review follows Kitchenham's SLR methodology with PRISMA screening across IEEE Xplore, ACM Digital Library, SpringerLink, and ScienceDirect, narrowing 1,298 records to 53 eligible and 41 finally included studies (2016–June 2022), and is positioned as the first SLR specifically on blockchain-based FL.
- It defines five research questions: RQ1 (which FL security/privacy attacks blockchain can solve), RQ2 (which blockchain characteristics benefit FL), RQ3 (state-of-the-art BCFL approaches in security/privacy, records/rewards, verification/accountability), RQ4 (research challenges/new issues from BCFL), and RQ5 (future directions).
- It enumerates six classes of attacks against traditional FL that blockchain can mitigate: single point of failure (SPoF), DoS/DDoS, free-riding, poisoning (data and model poisoning, including reverse and random poisoning), man-in-the-middle (session hijacking, IP spoofing), and eavesdropping (which can escalate to jamming/DoS).
- It identifies seven blockchain characteristics that improve FL versus the traditional centralized design: decentralization, traceability, immutability, incentives, integrity, reliability, and trust (with trust comprising liveliness and loyalty enforced via consensus and smart contracts).
- It presents a six-component BCFL architecture (FL participants, FL–blockchain integration middleware, miners, smart contract, consensus algorithm, blockchain network) and a seven-step one-epoch workflow (local training, smart-contract execution, local model upload, mining, consensus, block addition, global-model download), noting on-chain vs. off-chain (IPFS) storage trade-offs.
- It compares five blockchain deployment frameworks used in BCFL—Ethereum (public, PoW, Solidity), Hyperledger Fabric (private, PBFT/Raft, chaincode), EOS.IO (public/consortium, DPoS), FISCO BCOS (consortium, PBFT/Raft), and Corda (consortium, PBFT/Raft)—with their consensus, languages, and support level for FL.
- Theme 1 (security and privacy): it surveys frameworks including BytoChain (Proof-of-Accuracy, Byzantine-resistant), ChainsFL (Raft + DAG two-layer), BLADE-FL (anti-SPoF, lazy-participant detection), BFEL (Proof-of-Verifying, gradient compression), BFLC (committee consensus), Kumar et al. (differential privacy + homomorphic encryption on Ethereum/IPFS), Biscotti (VRF + Proof-of-Federation + multi-Krum against Sybil/poisoning), and Fed-BC.
- Theme 2 (records and rewards): it surveys FedCoin (Proof-of-Shapley payment distribution), Martinez et al. (Class-Sampled Validation-Error Scheme), Kang et al. (reputation-aware participant selection), Behera et al. (Ethereum smart-contract incentives), and FL-MAB (multi-dimensional auction client selection/monetization).
- Theme 3 (verification and accountability): it surveys VFChain (Dual Skip Chain for verifiable/auditable FL), BC-based PPFL (Awan et al., immutable ledger tracking), BlockFLA (hybrid public/private blockchain accountability against backdoor attacks), Lo et al. (trustworthy FL with weighted-fair training, evaluated on COVID-19 X-rays), and BlockFlow (auditing scores reflecting honest vs. malicious contributors).
- It catalogs nine open issues: malicious miners, miner-selection difficulty, the "dark side" of immutable storage (un-rectifiable errors, irreversible smart contracts), smart-contract exploitation/vulnerabilities, blockchain-framework vulnerabilities (e.g., EOS.IO and Ethereum exploits), malicious end-devices, asynchrony of end-devices, synchronization slowdown from lazy participants, and blockchain forking.

## Conclusion

The SLR concludes that integrating blockchain into FL can mitigate most attacks afflicting conventional FL by providing a decentralized, secure, and robust architecture, immutable and traceable record-keeping via smart contracts, and incentive mechanisms that reward verified contributions—answering RQ1 through RQ3 affirmatively across the surveyed literature. The authors note, however, that BCFL is not without cost and introduces its own challenges (answering RQ4): blockchain's storage burden and high energy consumption (notably for PoW), gas-cost complexity in Ethereum/Solidity, the difficulty of verifying malicious participants under off-chain computation, and the open issues listed above. For RQ5, they propose concrete future directions: an authentication/device-registration scheme for participants, static analysis and code auditing of smart contracts (e.g., ZEUS), mechanisms for miner selection/verification (including leader election), adding privacy to Ethereum-based FL via zero-knowledge proofs (ZK-STARKs), and contract-management lifecycle tools to address immutability/irreversibility limits. They also stress that purely financial cryptocurrency rewards may be insufficient to motivate broad enterprise adoption, calling for aligned incentives, clearer goals, and wider cryptocurrency adoption. The paper frames itself as a foundation for developing robust BCFL systems that manage these open issues.
