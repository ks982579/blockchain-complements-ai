---
id: sok-bridging_trust_into_the_blockchain__on-chain_identity
title: "SoK: Bridging Trust into the Blockchain — A Systematic Review on On-Chain Identity"
authors:
  - Awid Vaziry
  - Kaustabh Barman
  - Patrick Herbke
year: 2024
venue: "2024 6th Conference on Blockchain Research & Applications for Innovative Networks and Services (BRAINS)"
publisher: "IEEE"
pages: "1-9"
doi: "10.1109/BRAINS63024.2024.10732621"
url: "https://research.ebsco.com/linkprocessor/plink?id=46dc20cf-0d52-3622-8c44-bdd6e3f7e19f"
type: inproceedings
keywords:
  - Blockchain
  - Distributed Ledger Technology
  - On-Chain
  - Trust
  - Identity Management
  - Systematic Literature Review
---

## Overview

This paper (a Systematization of Knowledge / systematic literature review presented at IEEE BRAINS 2024 by Vaziry, Barman, and Herbke of TU Berlin) surveys the state of "on-chain identity" — mechanisms for establishing trusted, privacy-compliant, blockchain-verifiable user identities, motivated by regulatory pressure (AML, KYC, CTF) on decentralized finance and blockchain applications. The authors define on-chain identity narrowly as an externally owned account possessing an attestation (token or claim) that can be presented to an on-chain entity (smart contract) to prove a trusted authority verified the user's identity in advance, explicitly excluding storage of personal data on-chain (even encrypted) to remain GDPR-compliant. Following PRISMA, Kitchenham, and Okoli SLR guidelines, they searched scientific databases and funneled 2,232 candidate papers down through two filtering steps to 98 and finally 13 relevant studies, which were analyzed against a set of screening questions. The central finding is that on-chain identities are realized through three mechanism families — zero-knowledge proofs (zk-SNARKs), public key infrastructure / X.509 certificates, and web of trust / trust propagation — and that "trust" is the dominant unresolved research gap, manifesting both in trusting the digital representation of a physical human and in trusting the identity providers that issue on-chain confirmations. The authors claim this is the first structured literature review focused specifically on on-chain (rather than general blockchain-based) identity.

## Background

The paper builds on and cites a body of prior work to frame its argument. It situates blockchain on Nakamoto's Bitcoin (2008) and Buterin's Ethereum/smart-contract concept (2014/2015), and distinguishes permissionless from permissioned ledgers (e.g., Hyperledger Fabric/Aries/Indy, per Palma et al.). For identity, it draws on the ISO definition of identity as "a set of attributes related to an entity," the Self-Sovereign Identity (SSI) concept (Dunphy and Petitcolas; Mühle et al.), and the issuer-holder-verifier credential framework. The on-chain/off-chain distinction is taken from Eberhardt and Tai. The paper cites Azouvi et al. (2017) as the first use of the term "on-chain identity." Cryptographic foundations cited include PKI/X.509 (Cooper et al. RFC), zero-knowledge proof principles of completeness/soundness/zero-knowledge (Goldwasser, Micali, Rackoff) and zk-SNARK succinctness (Ben-Sasson et al.). For regulatory motivation it cites Wronka on DeFi financial crime/compliance and Yuyama et al. on DeFi disclosure principles. It positions itself against prior reviews: specialized surveys on blockchain identity for healthcare (Houtan et al.) and IoT (Huo et al.), and general surveys by Liu et al. (2020) and Ahmed et al. (2022/2023) — the latter concluding blockchain can improve identity management's security, privacy, and user control but that integration remains early-stage. Methodologically it relies on PRISMA (Page et al. 2021), Kitchenham and Charters' SE SLR guidelines, and Okoli's standalone SLR guide.

## Key Points

- The authors propose a precise definition of "on-chain identity": possession by an externally owned account of an attestation (token/claim) presentable to a smart contract proving a trusted authority verified the user's identity, explicitly excluding on-chain storage of personal data (even encrypted) for GDPR compliance.
- They claim to provide the first structured/systematic literature review focused specifically on on-chain verifiable identities, as opposed to general blockchain-based identity management.
- The SLR methodology funneled 2,232 papers (across SpringerLink (1541), IEEE Xplore (382), ScienceDirect (135), arXiv (118), ACM (47), Wiley (9); published 2017 onward) through PRISMA stages: 31 duplicates removed leaving 2,201, title/abstract screening to 98, a full-text eligibility check leaving 15, and in-depth screening that excluded 2 more for a final 13 included studies.
- On-chain identities are established via three mechanism families: zero-knowledge proofs (the most common, used in 7 of the 13 studies, in the form of zk-SNARKs, with 4 of those 7 using the ZoKrates library), PKI / X.509 certificates (3 studies), and web of trust / trust propagation (3 studies); a further 4 papers reference additional cryptographic schemes (e.g., Shamir's Secret Sharing, Camenisch-Lysyanskaya and Pointcheval-Sanders signatures, Pedersen commitments, MiMC ciphers).
- A temporal shift is observed: PKI/X.509 papers were published between 2017 and 2021, while zero-knowledge-proof publications were released between 2021 and 2023.
- Ethereum is the dominant platform (9 of 13 solutions), with Hyperledger Indy (2), Hyperledger Fabric (2), and Bitcoin (2) also appearing (the two Bitcoin solutions being the earliest works, from 2017–2018), and one solution leaving the DLT unspecified.
- Most solutions follow the issuer-holder-verifier model, often augmented with privileged "trust anchor"-type entities (used by 6 frameworks; e.g., registry, governance authority, trusted third party) and special revocation entities (e.g., "Judicial Authority," "Anonymity Revoker") that can unmask real-world identities.
- 8 of the 13 solutions support partial or full on-chain (smart-contract-based) identity/credential verification while the other 5 are off-chain-centric; nearly all heavy cryptographic setup (key exchange, hashing, proof/certificate generation) is performed off-chain due to cost.
- All but one of the studies implemented a proof-of-concept; only Damgård et al. proposed a purely theoretical system without an implementation; none is judged ready for live Ethereum mainnet deployment.
- Cost is the binding constraint rather than time: Bruschi et al. and Muth et al. consume several million gas units for on-chain proof verification, Heiss et al. about 600k gas (via off-chain pre-processing), and Gallersdörfer et al. 500k–1.5M gas for on-chain PKI certificate submission.
- The dominant research gap is trust, which manifests in two distinct forms: (1) how to trust the digital identity representation of a physical human (bridging real-world identity on-chain), and (2) how to trust the identity providers/issuers that issue on-chain identity confirmations.
- On the data-bridging challenge (linking on-chain attestations to real-world data), the reviewed studies fall into weak responses: ignoring the question entirely, invoking an unspecified trust anchor, or using a governmental digital ID as root of trust.

## Conclusion

The review concludes that on-chain identity is an active but niche research area: while 98 papers broadly addressed blockchain and identity, only 13 specifically targeted on-chain verifiable identities, with publication interest growing over the 2017–2023 window. The three research questions are answered: most solutions push verification logic and cryptography off-chain while registering credentials/entities on-chain, with 8 of 13 allowing on-chain smart-contract-based verification (RQ1); the prominent enabling technologies are non-interactive zero-knowledge proofs (7 works), PKI/X.509, and web of trust (3 architectures each) (RQ2); and trust is the most common, unresolved gap (RQ3). The authors find that no surveyed solution is suitable for live Ethereum mainnet deployment for three reasons: economically infeasible gas costs, unrealistic/unsatisfied initial-trust assumptions, and missing treatment of how real-world identities are bridged on-chain. They acknowledge limitations: subjectivity in the search term, inclusion criteria, and study selection, and a corpus too small for quantitative claims (suited only to qualitative analysis). Proposed future research directions include: making zero-knowledge proofs more efficient at the algorithmic level or via deeper integration into the blockchain protocol; reducing deployment/usage costs of on-chain certificates and proofs (e.g., dedicated zkp or certificate-management functionality in the protocol); and — most prominently — developing mechanisms, algorithms, patterns, and architectures to establish trusted on-chain root-of-trust actors and to identify/verify and privacy-compliantly bridge real-world humans and legal bodies into their on-chain identity representations on permissionless ledgers.
