---
id: multi-agent_systems_and_blockchain__literature_review
title: "Multi-Agent Systems and Blockchain: Results from a Systematic Literature Review"
authors:
  - Davide Calvaresi
  - Alevtina Dubovitskaya
  - Jean-Paul Calbimonte
  - Kuldar Taveter
  - Michael Schumacher
year: 2018
venue: "Advances in Practical Applications of Agents, Multi-Agent Systems, and Complexity: The PAAMS Collection (Lecture Notes in Computer Science)"
publisher: "Springer International Publishing"
pages: "110-126"
doi: "10.1007/978-3-319-94580-4_9"
url: "https://research.ebsco.com/linkprocessor/plink?id=c52e1ebb-bf62-3da3-bea5-7627dbae8990"
type: inproceedings
keywords:
  - Multi-Agent Systems
  - Blockchain
  - Systematic Literature Review
  - trust
  - reputation
---

## Overview

This paper (Calvaresi, Dubovitskaya, Calbimonte, Taveter, and Schumacher) presents a Systematic Literature Review (SLR) of research that combines Multi-Agent Systems (MAS) with Blockchain Technologies (BCT). It addresses the problem that MAS, which manage sensitive distributed data (e.g., ambient assisted living, healthcare, energy trading), have long-standing unresolved requirements around accountability, trust, and secure interaction, and that BCT is increasingly proposed as a reconciling solution without rigorous assessment of whether such integration is justified. The contribution is three-fold: (i) an SLR over 14 primary studies that captures application domains, motivations, assumptions, requirements, strengths, and limitations of the state of the art; (ii) an analysis of the correctness and justification of applying BCT to meet MAS requirements; and (iii) a formalization of open challenges plus a forward-looking vision and roadmap for combining MAS and BCT across application scenarios (healthcare/connected health, B2B e-commerce, sharing-economy societal information systems). A key finding is that the field is nascent, with more than half of the surveyed works at a purely conceptual maturity level and the justification for BCT often questionable.

## Background

The paper builds on and cites a substantial body of prior work to frame its argument. It draws on MAS foundations defining agents as autonomous, knowledge-driven entities (Russell and Norvig) and on prior MAS work arguing that trust, reputation, and security remain open challenges affecting agent scheduling, communication, and negotiation protocols (Ramchurn/Huynh/Jennings; Yu and Singh; Hedin and Moradian; Calvaresi et al.). For blockchain fundamentals it cites Nakamoto's Bitcoin (proof-of-work consensus), Buterin's Ethereum and smart contracts, and surveys of permissioned-blockchain consensus protocols (Cachin and Vukolic, covering Hyperledger Fabric, Tendermint, R3 Corda, MultiChain), as well as Keyless Signatures Infrastructure / KSI (Guardtime, Buldas et al.) as deployed in Estonian government networks. The paper relies on cited evidence for BCT's known limitations: PoW energy cost comparable to Ireland's national consumption (O'Dwyer and Malone), throughput limits of roughly 60 transactions per second (Gervais et al.), and scalability problems (Vukolic). It cites work showing blockchain alone does not guarantee privacy/anonymity due to transaction de-identification risks (Androulaki et al.; Moser), with mitigations such as onion routing/Tor (Dingledine et al.), secret sharing (Shamir), secure multi-party computation (Ben-Or et al.), and decentralizing privacy using blockchain (Zyskind and Nathan). Methodologically it adopts the SLR procedures of Kitchenham et al. and the Goal-Question-Metric approach, reusing a protocol from the authors' own prior MAS negotiation-protocol SLR (Calvaresi et al., ICAART 2018). It also cites Wust and Gervais ("Do you need a blockchain?") as a decision tool for justifying BCT use, and prior healthcare-blockchain prototypes (MedRec, Dubovitskaya et al., Yue et al., Kuo et al.).

## Key Points

- The SLR was conducted via a GQM-derived framework with six structured research questions (SRQ1-SRQ6) covering temporal/geographic trends, motivations, application domains and requirements, mechanisms and assumptions, strengths and limitations, and stated future challenges.
- Paper selection used contextualized keyword searches (combinations of blockchain, MAS/multi-agent system, smart contract, and trust), yielding an initial 36 papers, which four inclusion criteria (Context, Purpose, Theoretical foundation, Practical contribution) reduced to 14 primary studies.
- The combination of MAS and BCT is a recent and emerging research area: almost all studies date to 2017 (10 of 14), with research concentrated notably in Estonia (4 of 14, attributed to KSI adoption) but also spanning Europe, the US, Japan, China, and Russia.
- The surveyed works cluster into four application domains: collaborative governance (e.g., transactive energy, business-collaboration conflict resolution, legal accountability via self-aware contracts); big data management (collection/anonymization in distributed environments); coordination (decentralized AI, swarm robotics, IoT tuple-based coordination); and trust/data-integrity/reputation management (P2P reputation, identity assurance, eCommerce, supply/value chains, software lifecycle).
- The field is immature: more than 50% of the 14 works are at a conceptual level, only three present prototypes, and only three provide tested evaluations.
- The dominant motivation for adding BCT to MAS is meeting application-driven requirements for accountability, transparency, traceability, identity, and trust, with smart contracts and distributed ledgers also enabling master-less distributed reputation management.
- The paper provides a mapping (Table 2) between MAS requirements (trust, reputation, data integrity, traceability, transparency, anonymity, privacy, authenticity) and BCT properties (immutability, complete history, distributed consensus, cryptographic primitives, smart contracts), noting that anonymity and privacy require additional off-blockchain mechanisms rather than BCT alone.
- The authors argue that in many surveyed works the necessity and correctness of applying BCT is questionable; e.g., logging anonymized-dataset agreements does not justify private blockchain use, and claimed elimination of a trusted third party is undercut when permissioned systems (Hyperledger Fabric) still require membership services and certificate authorities.
- The paper formalizes eight open challenges for BCT in MAS: creating a legal base; verifying smart-contract/chaincode correctness; preserving decentralization (preventing mining pools/collusion); ensuring privacy/anonymity; ensuring technology adoption; managing membership services in permissioned BCT; addressing scalability; and ensuring reliability of underlying cryptographic mechanisms.
- The authors present a concrete vision for high-benefit MAS+BCT scenarios, including connected health (actors modeled as cooperative/trusted or reputation-managed agents, with smart contracts for insurance claims/reimbursements and emergency data access via witness cothority), B2B e-commerce removing trusted mediators while storing commitments and meta-commitments, and "democratized" sharing-economy societal information systems (Uber/AirBnB-style) representing each node as an agent.

## Conclusion

The SLR concludes that interest in coupling MAS and BCT is real and growing but the state of the art is preliminary: the surveyed evidence supports the intuition that BCT properties (immutability, traceability, distributed consensus, cryptographic primitives, smart contracts) can address MAS requirements such as trust, reputation, transparency, and accountability, and that reputation/transparency/traceability matter most for competitive agent behavior while trust/accountability matter most for collaborative behavior. However, the paper's analysis only partially supports the broader claim that integration is currently justified, since most contributions are conceptual, lack in-depth demonstration, and frequently fail to address essential design decisions (public vs. private BCT, on- vs. off-chain data), and in several cases the necessity and correctness of BCT use is questionable. The authors caution that BCT's complexity and lack of scalability may outweigh its benefits for early adopters and that blockchain alone does not solve privacy/anonymity. Stated limitations and open research directions include implementing the many conceptual proposals, improving node anonymity for public-blockchain deployments, achieving large-scale and scalable reputation systems, developing economic models for data-trading platforms, and the eight formalized open challenges. The authors position their ongoing and future work as formalizing these challenges into a research roadmap and implementing conceptual solutions that demonstrate adequate, justified applications of BCT within MAS.
