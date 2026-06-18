---
id: do_you_need_a_blockchain
title: "Do you Need a Blockchain?"
authors:
  - Karl Wüst
  - Arthur Gervais
year: 2018
venue: "2018 Crypto Valley Conference on Blockchain Technology (CVCBT)"
publisher: "IEEE"
pages: "45-54"
doi: "10.1109/CVCBT.2018.00011"
type: inproceedings
keywords:
  - blockchain
  - distributed ledger
  - permissionless
  - permissioned
  - supply chain management
  - Bitcoin
  - decentralized autonomous organizations
---

## Overview

Wust and Gervais (2018 Crypto Valley Conference) present a systematic framework for deciding when a blockchain — permissionless or permissioned — is actually the right technical tool, rather than a conventional centralized database. The paper is motivated by the uncritical enthusiasm surrounding blockchain adoption across industries, where the technology is frequently proposed for problems it does not materially improve. The authors formalize the decision space through a flowchart driven by four factors: whether multiple writers exist, whether a trusted third party (TTP) is available or always-online, whether all writers are known, and whether writers mutually trust each other. They then apply this methodology to three primary use cases — Supply Chain Management (SCM), Interbank and International Payments, and Decentralized Autonomous Organizations (DAOs) — plus briefer analyses of e-voting, IoT, land titles, intellectual property proof, and fair exchange. The core contribution is not advocacy for blockchain but a reusable decision methodology that distinguishes justified from unjustified blockchain adoption.

## Background

The paper builds on a substantial body of prior work it cites to contextualize its analysis. Bitcoin (Nakamoto 2009, [1]) is treated as the canonical permissionless blockchain and the origin of the "mutually distrusting parties without a TTP" design principle. Ethereum (Wood 2014, [2]) is cited as the first blockchain to support arbitrary smart contract execution, itself building on Nick Szabo's original smart contract formalization ([20]). Zerocash ([3]) is cited to illustrate that privacy in permissionless blockchains is achievable via computationally expensive cryptography, demonstrating the transparency-privacy tradeoff. Permissioned blockchain systems — Hyperledger Fabric ([11]) and R3 Corda ([4]) — are cited as the leading alternatives where the writer set is controlled. For consensus, the paper draws on Castro and Liskov's PBFT ([5]) as the standard Byzantine Fault Tolerant protocol used by permissioned systems, and cites Bano et al. ([28]) for a broader comparison of consensus algorithms. Bitcoin's throughput ceiling (~7 TPS, extensible to ~66 without security loss) is sourced from Gervais et al. 2016 ([6]). For interbank payments, the paper cites MAS ([12]), Bank of Canada ([13]), and the US Federal Reserve ([14]) as central banks researching DLT for settlement, and Ripple ([17]) as a live partial replacement of correspondent banking. Hashed timelock contracts (Lightning Network, [18]) and satellite chains ([19]) are cited as techniques enabling atomic cross-chain transfers. For fair exchange, Banasik et al. ([25]) and Goldfeder et al. ([26]) are cited to distinguish digital from physical goods exchange. The "DAO hack" ([23]) is cited as historical evidence of smart-contract risk. Bonneau et al. ([27]) is cited for a systematic treatment of Bitcoin properties, and Greenspan ([29]) for an earlier (non-systematic) discussion of when blockchain is pointless.

## Key Points

- A blockchain is justified only when multiple mutually distrusting writers exist and participants are unwilling to rely on an always-online trusted third party; in all other configurations, a centralized database is preferable on performance grounds.
- The paper provides a structured decision flowchart that maps application requirements (number of writers, TTP availability, writer trust, public verifiability needs) to one of four outcomes: no blockchain needed, permissionless blockchain, public permissioned blockchain, or private permissioned blockchain.
- Permissionless blockchains (Bitcoin, Ethereum) offer public verifiability and support an unknown, open writer set but incur severe throughput and latency penalties; permissioned blockchains (Hyperledger, Corda) recover performance using BFT protocols but require centralized membership management, making them closer to shared databases than to truly decentralized systems.
- Supply chain management does not inherently justify a blockchain: the "digital-to-physical interface problem" means that if any writer can inject false data (e.g., a fraudulent sensor reading), the blockchain's integrity guarantees are voided; the paper demonstrates this with a concrete tamper-proof-sensor attack scenario.
- If all SCM writers are trusted, a shared database suffices; if they are not, blockchain does not solve the underlying trust problem because data entry remains human- or machine-controlled and falsifiable.
- For interbank payments, a permissioned blockchain is well-suited: the central bank can act as a certificate authority (offline TTP) issuing write licenses, removing the need for always-online settlement intermediation while preserving currency authority.
- Current solutions such as Ripple do not eliminate trust requirements; they merely shift them from correspondent banks to gateway operators, because on-chain currencies issued by private gateways are not equivalent to real central-bank-backed currencies.
- A future system in which central banks issue on-chain currency and national blockchains interoperate via hashed timelock contracts or satellite chains could provide atomic cross-currency settlement without correspondent-banking trust requirements.
- DAOs are a legitimate permissionless blockchain use case because they involve anonymous, mutually distrusting investors with no natural TTP; however, the 2016 DAO hack demonstrates that poorly audited smart contract code creates catastrophic financial risk, and the incident shifted community consensus away from strict "code is law."
- E-voting, IoT, land titles, and fair exchange are candidate use cases where blockchain may add value, but each requires careful analysis of the specific trust model; in most IoT and land-title scenarios the physical-world interface problem re-emerges as the binding constraint.
- There is a fundamental tradeoff between transparency and privacy in any blockchain system; cryptographic techniques can reconcile them (as in Zerocash) but at significant computational cost.

## Conclusion

The paper's central finding is that blockchain is justified in a narrow class of scenarios characterized by multiple mutually distrusting writers and the absence of an acceptable always-online TTP — and is not a general-purpose upgrade over centralized databases. Applied to SCM, the authors conclude that blockchain is largely unjustified: Skuchain itself acknowledged that a single trusted database would meet most of its needs, and the physical-world data-entry problem means trust in writers cannot be avoided regardless of ledger technology. For interbank payments, the conclusion is more favorable: permissioned blockchains with central-bank-issued on-chain currency could genuinely simplify settlement and reduce correspondent-banking risk, though this requires inter-governmental coordination. DAOs are confirmed as a legitimate use case, with the caveat that smart-contract security is an open and serious problem. Open research questions identified include: (1) secure and verifiable digital-to-physical interfaces (trusted hardware oracles) that would unlock blockchain value in SCM, IoT, and land titles; (2) legal clarity on the binding status of smart contracts across jurisdictions; (3) international central-bank coordination to enable atomic cross-currency blockchain settlement; and (4) e-voting systems that simultaneously achieve security, verifiability, and voter privacy — a combination not demonstrated as of the paper's writing. The authors acknowledge that their methodology is structural and qualitative, leaving the throughput/latency dimension as an input to be weighed rather than a computed decision threshold.
