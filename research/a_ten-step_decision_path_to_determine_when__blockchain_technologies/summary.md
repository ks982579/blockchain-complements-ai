---
id: a_ten-step_decision_path_to_determine_when__blockchain_technologies
title: "A Ten-Step Decision Path to Determine When to Use Blockchain Technologies"
authors:
  - Asger B. Pedersen
  - Marten Risius
  - Roman Beck
year: 2019
venue: "MIS Quarterly Executive"
publisher: "Association for Information Systems"
volume: 18
issue: 2
pages: "1-17"
doi: "10.17705/2msqe.00010"
url: "https://research.ebsco.com/linkprocessor/plink?id=53f557e1-3dfe-35db-8802-d38dfbbedbf7"
type: article
keywords:
  - Blockchains
  - Maritime shipping
  - Supply chain management
  - Decision making
  - Technological innovations
---

## Overview

This MIS Quarterly Executive practitioner article (Pedersen, Risius, and Beck, 2019) presents a managerial "ten-step decision path" to help decision makers and system architects determine, first, whether a blockchain solution is justified for a given application and, second, which of three blockchain types (permissionless public, permissioned public/hybrid, or permissioned private) is most appropriate. The authors motivate the framework by noting that despite intense hype, blockchain databases carry real drawbacks (scalability, capacity, latency, privacy) that make them frequently inappropriate, and that existing taxonomies address technical design or business considerations but fail to answer managers' core feasibility question. Using a problem-centered design science research approach grounded in the authors' years of blockchain consulting experience and their leadership of the European Blockchain Center, they structure the path as a sequence of yes/no questions: the first seven assess feasibility, and the last three select the blockchain type. The framework is illustrated and validated by developing a proof-of-concept prototype for the Danish maritime shipping industry, built on a public permissioned Ethereum private net with Solidity smart contracts, in collaboration with DanPilot (a Danish pilotage company handling ~20,000 pilotages/year) and the Danish Maritime Authority. The central problem addressed is the maritime industry's lack of a "single source of truth," which causes paper-based redundancy, conflicting data, and multi-million-dollar compliance penalties.

## Background

The paper builds on and cites a substantial body of prior blockchain and IS work to ground its argument. It draws on Lacity (2018, MISQE) for enterprise blockchain challenges and the definition of decentralized applications (DApps), and reports market forecasts (Capgemini 2018; Deloitte/Schatsky et al. 2018) estimating $1.5-2.1 billion was spent on blockchain in supply chains in 2018, with HFS Research (Gupta 2018) finding provenance-tracking adoption outpacing banking. For technical and conceptual foundations it cites Xu et al. (2017, IEEE ICSA) on a blockchain architecture taxonomy, Glaser (2017, HICSS) on a framework for blockchain-enabled system and use-case analysis, and Risius and Spohrer (2017) on a blockchain research framework. It references Iansiti and Lakhani (2017, HBR, "The Truth About Blockchain") and a Bloomberg report (Park 2018) projecting $1 trillion in additional global trade from shipping-industry blockchain adoption by firms like Maersk, APL, Hyundai Merchant Marine, and Samsung SDS. The decision-path itself was seeded from practitioner sources (Meunier's Medium "When Do You Need Blockchain?" decision models, 2016; Zubko and Bohner's Hyperledger Fabric lessons, 2018). Trust and governance arguments cite Beck et al. (2016, ECIS) on trust-free cryptographic transactions and Beck et al. (2018, JAIS) on blockchain governance; consensus mechanisms draw on Castro and Liskov (1999) on Practical Byzantine Fault Tolerance. The design science methodology follows Peffers et al. (2007), and the paradox/trade-off framing follows Andriopoulos and Lewis (2010). The paper also notes (citing Murck 2017, HBR) the difficulty of changing blockchains, illustrated by the Ethereum DAO hard fork and Bitcoin's scaling governance crisis.

## Key Points

- The paper contributes a ten-step decision path, structured as sequential yes/no questions, that helps managers decide both whether blockchain is justified and which blockchain type to deploy; the first seven steps assess feasibility and the last three select the type.
- Step 1: A blockchain should only be considered if a shared common database is needed and a traditional database cannot meet the need; designers should weigh scalability (data volume and rate of change) and consider off-chain integration or a conventional database instead.
- Step 2: Blockchain only makes sense if multiple parties contribute, write, and update data; with a single party, a centralized database is more efficient.
- Step 3: Blockchain is appropriate only when parties have conflicting interests and/or cannot be fully trusted; its key advantage is creating a "trust-free" ecosystem where participants trust the validity of immutable stored data rather than each other.
- Step 4: Blockchain is warranted when participants want to or must avoid a trusted third party (escrow, notary, licensing authority), eliminating a central integration point that is a single point of failure.
- Step 5: Differing system-access rules among participants (distinct read/write/validation rights, governable via smart contracts) favor a blockchain over a uniform-access relational database.
- Step 6: Blockchain suits applications where the rules for transacting remain largely stable, because consensus-based, autonomously executed smart contracts are difficult to change or update.
- Step 7: A need for an objective, immutable, non-repudiable audit log (e.g., for regulatory or legal purposes) is a strong reason to use blockchain; the authors note that if regulatory demands for an auditable log are pressing, blockchain may be best regardless of other steps.
- Steps 8-10 select the blockchain type: Step 8 (is public read access required / are write rights differentiated?) distinguishes permissioned vs. permissionless; Step 9 (are transactions public to read?) distinguishes public vs. private; Step 10 (where is consensus determined—within one organization vs. between participating organizations?) distinguishes permissioned private from permissioned public/hybrid.
- The authors offer a practical heuristic: a blockchain is generally feasible if at least five of the first seven questions are answered "Yes."
- The paper distinguishes three blockchain types: permissionless public (Bitcoin, Ethereum; PoW/PoS consensus), permissioned public/hybrid (Ripple, Multichain, Eris; consensus between organizations), and permissioned private (Hyperledger Fabric, Corda; consensus within an organization via, e.g., pBFT)—and notes one platform (e.g., Hyperledger Fabric) can serve as more than one type.
- The Danish maritime shipping case validated the path: all seven feasibility questions were answered "Yes," and the type-selection steps pointed to a public permissioned blockchain, implemented as a prototype on a private Ethereum net with Solidity smart contracts and heavy/light node access.
- The authors stress the framework's transferability beyond maritime shipping to other supply-chain and document-intensive industries burdened by legacy systems, paper-based processes, and price pressure.

## Conclusion

The authors conclude that the ten-step decision path successfully helps managers identify a valid blockchain use case, the alternatives to consider, and the appropriate blockchain type, and that the maritime prototype demonstrated blockchain could establish a "single source of truth" to overcome the industry's operational inefficiencies and compliance costs. The prototype was evaluated against requirements gathered in earlier stakeholder interviews and confirmed consistent with the industry's actual needs and constraints; its real-world relevance was reinforced by the Danish government's 2018 Strategy for Denmark's Digital Growth committing to a blockchain-powered ship register, and by a shipping-company estimate that building such a system would cost well below the penalties from a single typical cargo-clearance delay. The authors are careful to caveat that the binary yes/no structure is a simplification: real design involves complex and potentially paradoxical business and design trade-offs (citing Andriopoulos and Lewis), so practitioners must still weigh broader business requirements and the inherent limitations of blockchains (capacity, latency, privacy, difficulty of changing transacting rules, and the breakdown of trust-freeness when linking digital to physical value through trusted interfaces). Open questions left for further work include how to navigate these design trade-offs in specific contexts, governance of permission/super-user roles (the Danish Maritime Authority initially acting as super user), and the empirical validation of the framework beyond a single proof-of-concept prototype in additional industries.
