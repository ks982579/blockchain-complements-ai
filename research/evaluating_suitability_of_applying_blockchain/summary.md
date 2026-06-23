---
id: evaluating_suitability_of_applying_blockchain
title: "Evaluating Suitability of Applying Blockchain"
authors:
  - Sin Kuang Lo
  - Xiwei Xu
  - Yin Kia Chiam
  - Qinghua Lu
year: 2017
venue: "2017 22nd International Conference on Engineering of Complex Computer Systems (ICECCS)"
publisher: "IEEE"
pages: "158-161"
doi: "10.1109/ICECCS.2017.26"
url: "https://research.ebsco.com/linkprocessor/plink?id=b066f2fd-9201-37ac-9a53-0baecdbe4a18"
type: inproceedings
keywords:
  - blockchain
  - suitability
  - evaluation
---

## Overview

This paper (Lo, Xu, Chiam, and Lu; IEEE ICECCS 2017) proposes a practitioner-oriented evaluation framework for deciding whether blockchain is an appropriate technology choice for a given use case, addressing the practical problem that little reliable product data or technology evaluation exists to guide such decisions. The authors frame blockchain as a database and computational platform with distinctive properties (immutability, non-repudiation, integrity, transparency, equal rights, and distributed trust) but also significant weaknesses (limited data privacy and poor throughput scalability), arguing it suits some scenarios while conventional databases suit others. Their contribution is a decision framework comprising seven sequential criteria (multi-party involvement, trusted-authority requirement, centralized operation, transparency vs. confidentiality, data integrity, data immutability, and high performance) arranged as a decision flow of white (main) and grey (sub) decision nodes. They validate the framework by applying it to four existing industrial blockchain trials — supply chain, electronic health records, identity management, and stock market — concluding that supply chain and identity management are suitable for blockchain whereas EHRs and stock market are not yet suitable. The framework is offered as a starter guide to reduce wasted effort on unviable use cases, motivated partly by Gartner's estimate that 90% of enterprise blockchain projects launched in 2015 would fail within 18-24 months.

## Background

The paper builds on foundational and contextual work cited from elsewhere. It positions blockchain as the technology behind Bitcoin, citing Nakamoto's 2008 peer-to-peer electronic cash paper, and references Tschorsch and Scheuermann's technical survey of decentralized digital currencies to support the claim that a blockchain is a distributed ledger able to verify and store any kind of transaction. It cites a UK Government Chief Scientific Adviser report (distributed ledger technology beyond blockchain) to situate broader exploration by startups, enterprises, and governments across supply chain, health records, voting, energy, ownership, identity, and critical infrastructure. The authors note (citing Omohundro's work on cryptocurrencies, smart contracts, and artificial intelligence) that Gartner estimated 90% of enterprise blockchain projects from 2015 would fail within 18-24 months, and cite a Deloitte survey indicating 42% of consumer-goods/manufacturing companies planned to spend at least $5 million on blockchain in 2017. For the case studies they draw on external sources: Walmart's pork-tracking supply-chain pilot (Bloomberg/Kharif); the MedRec initiative (Azaria et al., OBD 2016) for blockchain-based interoperable EHRs that store pointers to patient data on-chain; IBM and industry write-ups on blockchain identity management; and stock-market efforts including Nasdaq Linq and the ASX CHESS replacement project. The framework itself derives from the authors' investigation for the Australian government and Data61/CSIRO work (Staples et al., "Risks and Opportunities for Systems Using Blockchain and Smart Contracts," 2017), plus existing industrial products, technical forums, and academic literature.

## Key Points

- The paper proposes a blockchain suitability evaluation framework consisting of a list of criteria plus a typical decision process, designed to be applied before architecting a blockchain-based application by assessing use-case requirements.
- The framework is structured around seven sequential questions, represented as white (main) and grey (sub) decision nodes: (A) multi-party involvement, (B) trusted-authority requirement, (C) centralized operation, (D) data transparency vs. confidentiality, (E) data integrity of transaction history, (F) data immutability, and (G) high-performance requirement.
- Criterion A: blockchain is suitable when multiple parties are involved (especially where intermediaries currently mediate), since it offers neutral shared infrastructure; a single-entity system can achieve the same properties more cheaply by other means.
- Criterion B: blockchain is suitable when no trusted authority is needed or the existing trusted authority can be decentralized; blockchain shifts trust from third-party intermediaries to the blockchain software, incentive mechanisms, and oracles, rather than removing trust entirely.
- Criterion C: blockchain (with smart contracts) is not suitable for systems that require centralized operation, because no single party controls the system and governance becomes more like diplomacy than conventional product/risk management.
- Criterion D: blockchain natively favors transparency (all participants see published data, enabling validation by processing nodes); confidentiality can be added via encryption or off-chain raw data with on-chain hashes, but at the cost of performance, auditability, or the distributed-trust benefit, and consortium/private read controls do not provide confidentiality between competitors.
- Criterion E: blockchain supports data integrity and provenance for tracking assets, but achieving integrity this way may be relatively expensive, and architectures with existing tracking/hashing/signing mechanisms may not benefit.
- Criterion F: blockchain provides strong immutability and non-repudiation valuable where third parties are untrustworthy, but this can be a drawback (disputed transactions, wrong addresses, lost keys, data-entry errors, court-ordered content removal) and makes systems less adaptable than rollback-capable conventional technology.
- Criterion G: blockchain is currently not highly scalable (mainstream public chains handle ~3-20 transactions/second versus VISA's ~1,700), is unsuitable for Big Data due to full-replica redundancy, and is therefore inappropriate where high performance or real-time processing is required, though consortium/private chains with tuning perform better.
- Applying the framework to four industrial trials, the paper concludes supply chain and identity management are suitable for blockchain ("Blockchain" result), while electronic health records and stock market are better served by a conventional database ("Database" result), the distinguishing factors being EHRs' confidentiality requirement and the stock market's confidentiality plus high-performance requirement.

## Conclusion

The authors conclude that their seven-criterion framework provides a useful starter guide for organizations to examine blockchain suitability against an intended use case, and that applying it to four real industrial trials yielded actionable verdicts: supply chain and identity management benefit from blockchain (they exploit its digital nature without being hurt by current limitations), whereas EHRs and the stock market are not yet suitable due to the nature or limitations of blockchain (confidentiality requirements for EHRs, and confidentiality plus scalability/performance requirements for high-volume stock trading). The framework is presented as validated through this feasibility evaluation and as a means to reduce wasted effort on unviable use cases. The paper notes several open issues and forward-looking caveats: blockchain's scalability is described as not necessarily an inherent limitation and possibly solvable in the near future, after which stock-market use could become viable; EHRs could use blockchain as native storage once patients are willing to relinquish data privacy, with current solutions (e.g., MedRec) serving as a bridging technology; and confidentiality/privacy versus transparency remains a fundamental unresolved trade-off. The validation is limited to four qualitative case-study assessments rather than quantitative or empirical product measurement, leaving broader and more rigorous evaluation of the framework as implicit future work.
