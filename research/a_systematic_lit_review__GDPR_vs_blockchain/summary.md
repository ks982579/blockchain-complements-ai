---
id: a_systematic_lit_review__GDPR_vs_blockchain
title: "A systematic literature review of the tension between the GDPR and public blockchain systems"
authors:
  - Rahime Belen-Saglam
  - Enes Altuncu
  - Yang Lu
  - Shujun Li
year: 2023
venue: "Blockchain: Research and Applications"
volume: 4
issue: 2
pages: "100129"
doi: "10.1016/j.bcra.2023.100129"
url: "https://www.sciencedirect.com/science/article/pii/S2096720923000040"
type: article
keywords:
  - blockchain
  - distributed ledgers
  - privacy
  - data protection law
  - legal compliance
  - GDPR
  - EU
  - EEA
  - UK
---

## Overview

This paper is a PRISMA-compliant systematic literature review (SLR) by Belen-Saglam, Altuncu, Lu, and Li (2023) covering 114 peer-reviewed papers on the tension between public (permissionless) blockchain systems and the EU General Data Protection Regulation (GDPR). It identifies three core conflict areas: (i) the inability to exercise the right to erasure (RTBF) due to blockchain's architectural immutability, (ii) the difficulty of identifying data controllers and processors in decentralized networks where responsibility is distributed across anonymous node operators, and (iii) ambiguity in applying GDPR's territorial jurisdiction to globally distributed node infrastructure. It surveys proposed technical and legal solutions across all three areas. The paper claims to be the most comprehensive review of this tension to date, explicitly superseding two prior SLRs that each covered at most 41 papers. The work targets blockchain researchers, developers, policymakers, and lawmakers seeking to reconcile data protection law with decentralized architectures.

## Background

- The Bitcoin/blockchain literature characterizes public blockchains as architecturally immutable, transparent, and decentralized — properties that are not incidental design choices but structural guarantees, making compliance retrofits non-trivial.
- The EU Blockchain Observatory and Forum (2018) identified public permissionless blockchains as the most significant GDPR compliance challenge in the blockchain space.
- The GDPR itself is the primary legal reference: the paper engages extensively with Articles 4, 5, 6, 7, 16, 17, 22, and 25, and Recitals 26 and 30, which together define personal data, lawful bases for processing, data subject rights (including erasure and rectification), automated decision-making requirements, and privacy-by-design obligations.
- The Article 29 Data Protection Working Party opinions on pseudonymization and anonymization under the predecessor EU Data Protection Directive (1995) are cited as the authoritative regulatory position on what techniques can render data outside GDPR scope.
- National DPA positions from CNIL (France), ICO (UK), Irish DPC, and the Spanish AEPD are surveyed for their guidance on blockchain-GDPR conflicts, revealing inconsistency and regulatory fragmentation across member states.
- Two prior SLRs are cited as the closest predecessors: Haque et al. (2021, 39 papers) and Suripeddi and Purandare (2021, 41 papers) — both found insufficiently comprehensive in scope and methodology to serve as a definitive reference.
- The cryptographic technique literature is surveyed as the primary source of proposed technical mitigations: zero-knowledge proofs (ZKPs), ring signatures, Merkle trees, chameleon hashing (Ateniese et al.), secure multi-party computation (SMPC), and public-key cryptography.
- Domain-specific blockchain application literature in healthcare, IoT, education, finance, and supply chain is referenced as context for understanding how the tension manifests in practice across sectors.

## Key Points

- The SLR is the most comprehensive review of its topic to date, covering 114 papers selected from an initial pool of 472 merged candidates (deduplicated to 413) via PRISMA-compliant inclusion/exclusion criteria, surpassing both prior SLRs in scope and rigor.
- Hashed or encrypted data stored on public blockchains is classified as pseudonymized personal data (not anonymous) under GDPR Recital 26, meaning it remains within GDPR scope; there is no academic or regulatory consensus on which specific technique, if any, meets the anonymization threshold that would remove blockchain data from GDPR coverage.
- Public keys stored on-chain are personal data under GDPR Recital 30, and are structurally unaddressable by any proposed erasure mechanism: they cannot be deleted from an immutable ledger, and revoking or replacing them does not erase the historical transaction record associated with them.
- The right to erasure (RTBF) conflict appears in 53 of the 114 reviewed papers and no proposed technical solution fully resolves it without sacrificing one of blockchain's defining properties — either immutability, decentralization, or transparency must be compromised.
- The leading proposed RTBF solutions — off-chain storage with on-chain hashes, chameleon hashing (mutable hash pointers), redactable blockchains, and encryption-then-deletion (rendering data unreadable by destroying the key) — each carry residual compliance risk: off-chain storage displaces the problem; chameleon hashing undermines immutability guarantees; encryption-then-deletion leaves encrypted data on-chain whose status as "erased" under Article 17 is legally unresolved.
- No consensus exists on who constitutes the "data controller" in a public blockchain: node operators, smart contract deployers, transaction initiators, and application-layer entities have all been proposed, and no regulatory authority has issued a definitive binding ruling; this leaves the accountability regime of GDPR Article 26 (joint controllership) structurally unapplied in public blockchain deployments.
- The literature systematically neglects the use case where a third party enters another person's data on-chain without their consent (e.g., posting someone else's personal information in a transaction), leaving a significant gap in the practical analysis of who bears controller responsibility in adversarial or negligent data-entry scenarios.
- Smart contracts are the leading proposed mechanism for consent management under GDPR Article 6 and 7, but smart contracts cannot satisfy Article 22's requirement for a human-intervention mechanism in automated decision-making — the code executes deterministically without override capability by design.
- Blockchain's transparency, cryptographic data integrity, and tamper-evident audit trail positively align with GDPR's security (Article 5(1)(f) and Article 32) and transparency (Article 5(1)(a)) principles — the tension is not universally antagonistic.
- GDPR's data minimization principle (Article 5(1)(c)) conflicts with public blockchain's requirement that all validating nodes hold a full copy of all historical transaction data, and with the transparency requirement that transactions be publicly verifiable.
- Territorial scope is structurally ambiguous: GDPR applies when data subjects are EU residents, but public blockchain nodes are globally distributed and anonymous, making it practically impossible to determine which nodes are processing data subject to GDPR jurisdiction and to enforce compliance obligations on them.
- The paper calls for GDPR legislative clarification on controller definitions for decentralized systems and for blockchain design research targeting GDPR-by-design architectures, rather than treating compliance as a post hoc retrofit problem.

## Conclusion

The SLR confirms that all three core tensions between public blockchains and the GDPR are real, persistent, and unresolved. No proposed technical solution fully satisfies the right to erasure without compromising blockchain's core architectural properties; the metadata problem (public keys as permanent personal data) has no viable erasure path whatsoever. The controller identification problem remains unresolved both academically and regulatorily, with national DPAs offering inconsistent guidance and no binding EU-level ruling. Privacy-enhancing cryptographic techniques (ZKPs, SMPC, ring signatures) are promising but impose computational overhead that strains public blockchain scalability and have not been validated in production-scale deployments. The paper acknowledges several methodological limitations: Google Scholar was searched using title keywords only (potentially missing papers that treat the topic tangentially), the review is restricted to public permissionless blockchains (excluding the potentially more compliant permissioned/consortium architectures), and the analysis is conducted against the GDPR as written in 2023 without anticipating legislative revision. Open research questions explicitly flagged include: anonymization techniques robust to future quantum computing advances that could break current pseudonymization; blockchain architectural designs that can support human-intervention rights under Article 22 without sacrificing automation; systematic analysis of use cases where third parties enter others' personal data on-chain; empirical study of hybrid on-chain/off-chain ecosystems to determine where exactly GDPR obligations attach; and legislative reform proposals that clarify controller definitions and territorial scope for decentralized systems in a legally operative way.
