---
id: security_and_privacy_on_blockchain
title: "Security and Privacy on Blockchain"
authors:
  - Rui Zhang
  - Rui Xue
  - Ling Liu
year: 2019
venue: "ACM Computing Surveys"
publisher: "Association for Computing Machinery"
volume: 52
issue: 3
pages: "Article 51, 34 pages"
doi: "10.1145/3316481"
url: "https://doi.org/10.1145/3316481"
type: article
keywords:
  - Blockchain
  - security
  - privacy
---

## Overview

This ACM Computing Surveys article (2019) by Zhang, Xue, and Liu provides a comprehensive, structured review of the security and privacy landscape of blockchain technology, motivated by the observation that prior surveys largely focused on enumerating attacks or proposing isolated countermeasures rather than systematically organizing the underlying security and privacy properties. The paper addresses this gap by first explaining how blockchain works mechanically (hash chained storage via hash pointers and Merkle trees, digital signatures, and consensus), then categorizing seven security/privacy requirements for online transactions (consistency, integrity, availability, double-spending prevention, confidentiality, anonymity, unlinkability), distinguishing "basic" properties already supported by Bitcoin (consistency, tamper-resistance, DDoS resistance, double-spend resistance, pseudonymity) from "additional" properties that need enhancement (unlinkability, confidentiality, 51% attack resistance). It then surveys representative consensus algorithms (PoW, PoS, BFT-based, and several novel variants) and a broad set of cryptographic and systems techniques (mixing, anonymous signatures, homomorphic encryption, attribute-based encryption, secure multi-party computation, zero-knowledge proofs, TEE-based and game-based smart contract verification) used to close the gap between Bitcoin's basic guarantees and the stronger privacy/security demands of broader blockchain applications. Its main contribution is a unifying taxonomy (Table 2) that maps each requirement to a property and then to concrete techniques, intended as both an entry point for non-experts and a reference map for researchers.

## Background

The paper situates itself among prior security/privacy surveys it builds on and distinguishes itself from: Bonneau et al.'s SoK on Bitcoin and cryptocurrencies (anonymity analysis and privacy-enhancing methods), Karame's overview of Bitcoin's security/scalability risks and mitigations, Conti et al.'s survey of Bitcoin loopholes and risks, and Li et al.'s survey of attack cases and exploited vulnerabilities across popular blockchain systems. Foundational cryptographic building blocks cited include Haber and Stornetta's 1991 proposal for a cryptographically secured chain of blocks and their 1993 work (with Bayer) introducing Merkle trees for efficient timestamping — both predating Bitcoin by nearly two decades. The blockchain itself originates from Nakamoto's 2008/2009 Bitcoin whitepaper and implementation, with Ethereum (Buterin) introducing the account-based transaction model and smart contracts (Blockchain 2.0). The CAP theorem (Gilbert and Lynch, 2002) is invoked to frame blockchain's consistency/availability/partition-tolerance tradeoffs, and the "Bitcoin guarantees strong vs. eventual consistency" debate draws on Wattenhofer's and Sirer's analyses (with Pass et al.'s formal proofs of consistency and liveness for Nakamoto consensus under bounded-delay assumptions, and Pass-Shi's FruitChain extension). The Byzantine Generals Problem (Lamport, Shostak, Pease, 1982) underlies the discussion of consensus, with PBFT (Castro and Liskov, 1999) as the classical practical solution. For privacy-enhancing techniques, the paper draws on Mixcoin and CoinJoin/CoinShuffle (mixing), group signatures (Chaum and van Heyst, 1991) and ring signatures (used in CryptoNote/Monero), attribute-based encryption (Sahai-Waters 2005 and later multi-authority extensions), Yao's secure two-party computation (1982/1986) and Goldreich-Micali-Wigderson's generalization to MPC (1987), Goldwasser-Micali-Rackoff zero-knowledge proofs (1985) and their non-interactive variant (Blum-Feldman-Micali, 1988), zk-SNARKs (Bitansky et al., 2012) as used in Zcash, and Intel SGX-based trusted execution environments as used in Ekiden and PoET.

## Key Points

- The paper proposes a categorization of seven security and privacy requirements for online transactions: ledger consistency across institutions, transaction integrity, system/data availability, double-spending prevention, transaction confidentiality, user identity anonymity, and transaction unlinkability.
- It argues that Bitcoin's blockchain inherently provides five "basic" security properties — consistency, tamper-resistance, DDoS resistance, double-spending resistance, and pseudonymity — achieved through the combination of hash-chained storage (hash pointers + Merkle trees), ECDSA digital signatures, and PoW consensus.
- The paper claims Bitcoin's blockchain achieves a tradeoff form of CAP (specifically partition tolerance and consistency with deferred availability via the ω=6 confirmation rule), offering "far stronger than eventual consistency" — a position the authors note is debated in the literature.
- It identifies two "additional" properties not natively supported by Bitcoin — unlinkability and confidentiality — and frames these as the central open challenges for privacy-preserving blockchain applications, arguing that pseudonymity alone is insufficient to prevent de-anonymization via address-linkage analysis.
- The paper provides a comparative classification of blockchains into public, consortium, and private types (Table 1), differentiating them by number of trust authorities required and speed of consensus.
- It presents a structured comparison of consensus algorithms (Table 3), grouping PoW/PoS/DPoS as "eventual consistency" mechanisms suited to public blockchains versus PBFT/Paxos/Raft as "strong consistency, no-fork" mechanisms suited to consortium/private blockchains, with associated efficiency, resource consumption, and fault-tolerance tradeoffs.
- It surveys multiple PoS variants (chain-based vs. BFT-style), explaining the "nothing-at-stake" problem and how penalty-based designs (e.g., Ethereum's Casper, Peercoin's "coin age," Snow White, Ouroboros) address it.
- The paper reviews newer consensus mechanisms — Sleepy consensus, Proof of Elapsed Time (Intel SGX-based), Proof of Authority, and Proof of Reputation — and notes documented SGX vulnerabilities affecting PoET along with proposed mitigations.
- It compiles a taxonomy of additional privacy-enhancing techniques (mixing services like Mixcoin/CoinJoin/CoinShuffle; anonymous signatures via group and ring signatures; homomorphic encryption; attribute-based encryption; secure multi-party computation as in Enigma; non-interactive zero-knowledge proofs as in Zcash/zk-SNARKs; TEE-based smart contracts as in Ekiden; and game-based verification as in TrueBit and Arbitrum), summarized with their applications, advantages, and disadvantages (Table 4).
- The paper asserts three overarching design principles: (1) no single technology fully solves blockchain security/privacy, so combinations are typically needed (e.g., Enigma combining SMPC and TEE); (2) every added security/privacy technique introduces new attack surfaces or tradeoffs; and (3) there is an inherent tradeoff between security, privacy, and efficiency that must guide technique selection per application context.

## Conclusion

The survey concludes that while Bitcoin's blockchain successfully delivers its core inherent guarantees (consistency, tamper-resistance, DDoS resistance, double-spend resistance, pseudonymity), only a small subset of real-world blockchain platforms currently achieve the full set of additional security and privacy properties (unlinkability, confidentiality, resistance to majority/51% attacks) that broader applications demand. The authors do not present new empirical results but rather synthesize and organize existing findings, concluding that a deep understanding of these properties is essential for establishing trust in blockchain deployments and for guiding future defense techniques. They explicitly flag several open challenges: the practical deployment of attribute-based encryption on blockchains remains unsolved (issuance/revocation of attribute certificates in decentralized settings); homomorphic encryption and SMPC remain computationally inefficient for complex functions; SGX/TEE-based consensus and smart contracts (PoET, Ekiden, Enigma) still face unresolved hardware-level attack vectors; and zero-knowledge-based privacy (zk-SNARKs/Zcash) remains less efficient than non-private alternatives. The paper closes by conjecturing that developing lightweight cryptographic algorithms and other practical, efficiency-conscious security/privacy methods will be key to future blockchain development — framing efficiency-privacy-security tradeoff research as the primary direction for future work.
