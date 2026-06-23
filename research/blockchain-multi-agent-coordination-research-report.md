---
title: "Blockchain as a Coordination and Trust Layer for Multi-Agent Systems"
subtitle: "A source-vetted research report for the blockchain-complements-AI paper (§5)"
course: "Current Topics in Computer Science"
citation_style: IEEE
date: 2026-06-20
scope: "Multi-agent coordination only — even split of foundational MAS+blockchain and recent crypto-native agent literature"
---

# Blockchain as a Coordination and Trust Layer for Multi-Agent Systems

## Introduction

### Research objective

This report locates and assesses quality academic sources for the claim that blockchain *complements* artificial intelligence specifically in the domain of **multi-agent coordination**. It addresses the three lenses of the parent paper — benefits, applications, and challenges — but confines them to the agent-coordination sub-domain rather than to blockchain-AI integration generally. As agreed in the scope declaration, the source base is split evenly between *foundational* multi-agent-systems (MAS) work, which provides the theoretical basis, and *recent* crypto-native agent work (2023–2026), which satisfies the "current topics" remit.

### Scope

**Covered:** the classical MAS coordination-and-trust problem; the earlier blockchain-for-MAS literature that predates the large-language-model (LLM) agent wave; the recent literature in which LLM-based autonomous agents use blockchain as an identity, reputation, contract-enforcement, and settlement layer; and the emerging machine-to-machine standards (ERC-8004, x402) that operationalise these ideas. For each, benefits, applications, and challenges are mapped against the trust-versus-database test that governs the parent paper.

**Not covered:** the reverse direction (AI improving blockchain consensus, which is the paper's §6); general blockchain-AI integration outside the agent frame (provenance, federated learning, transparency, handled in §3–4); and the tokenomics or price behaviour of agent tokens, which are treated only where a project illustrates a coordination mechanism, never as financial assets.

**Governing test (assumed, not silent):** consistent with the Wüst and Gervais decision-framework that spines the paper, a source qualifies only where the coordination scenario genuinely involves mutually-distrusting agents that must share state or value without an acceptable trusted third party. This test is applied as an edge-case sub-question in its own right, because much of the recent literature does not survive it.

### Sub-questions addressed

1. What is the coordination-and-trust problem in multi-agent systems, and why does it arise? *(foundational basis)*
2. By what mechanisms does blockchain address that problem, in the pre-LLM literature? *(foundational mechanism)*
3. How does the recent LLM-agent literature use blockchain as a coordination layer? *(recent advancement)*
4. What concrete applications, deployments, and standards now exist? *(benefits and applications)*
5. What are the challenges and limitations? *(challenges)*
6. **Edge case:** Does agent coordination actually pass the "do you need a blockchain" test, or is much of this avoidable? *(critical assumption)*

---

## 1. The coordination-and-trust problem in multi-agent systems [Well-evidenced]

A multi-agent system is a set of autonomous computational agents that must act collectively despite holding only local information and pursuing potentially divergent goals. The coordination problem — how independent agents allocate tasks, reach agreement, and avoid conflict without a central controller — is one of the oldest in distributed AI; Sikora and Shaw framed it for distributed information systems as early as 1998 (source 1). Sun et al. (2025) provide a recent, comprehensive survey of multi-agent coordination across domains, useful as the modern map of the problem space independent of any blockchain claim (source 2).

The relevant complication for this paper is *trust*. When agents are operated by mutually-distrusting parties, or when any agent in an open system might be faulty or adversarial, coordination requires a mechanism to verify identity, track reputation, and guarantee that commitments are honoured. Classical MAS handled this with a trusted third party (a broker, a matchmaker, a central coordinator), which reintroduces a single point of failure and a single point of control. This is the precise gap that the blockchain-for-MAS literature claims to fill.

> [Assumption] The report treats "coordination" in the MAS sense (task allocation, agreement, conflict resolution among autonomous agents), not in the narrower distributed-systems sense of clock or state synchronisation. The sources cited use the MAS sense.

---

## 2. Foundational mechanisms: blockchain for MAS before the LLM wave [Well-evidenced]

The cornerstone source is Calvaresi, Dubovitskaya, Calbimonte, Taveter, and Schumacher (2018), a systematic literature review that is the most-cited early synthesis of MAS-and-blockchain integration (source 3). Its central thesis is the one the parent paper relies on: combining blockchain with MAS *distributes trust*, removing the need for a trusted third party and the single point of failure it represents. The authors also demonstrate a concrete reconciliation, pairing the Java Agent DEvelopment framework (JADE) with Hyperledger Fabric so that agent interactions, reputation tracking, and disagreement-management policies are implemented as smart contracts on an immutable ledger. This is the foundational citation for §5; it predates and underwrites almost everything in the recent literature.

A second foundational layer concerns *what blockchain actually adds to coordination as a formal capability*, rather than merely as storage. Ciatto, Mariani, and colleagues (2020), in a paper published in a special issue explicitly devoted to blockchain technologies for multi-agent systems, evaluate the *coordination expressiveness* of mainstream smart contracts using the Linda coordination model as a benchmark (source 4). Their finding is instructive and worth foregrounding for academic balance: standard smart-contract platforms have notable limitations in coordinating interactions *between contracts*, which their custom Tenderfone implementation is designed to overcome. This source lets the paper claim blockchain's coordination value without overstating it.

The robotics branch supplies the most tangible foundational use cases. Afanasyev et al. (2019) survey blockchain solutions for multi-agent *robotic* systems and catalogue open questions (source 5), while Peña Queralta and Westerlund (2019) examine blockchain-powered collaboration in heterogeneous robot swarms (source 6). A later integrative treatment — a 2021 *Engineering Applications of Artificial Intelligence* article on blockchain-supported asset transactions in MAS — is valuable chiefly as a citation hub, since it traces the early cluster (Kapitonov et al. 2017/2018, Mariani et al. 2017, Ciatto et al. 2018) that establishes the field's lineage (source 7).

**Synthesis of the foundational basis:** by roughly 2018–2020 the literature had already established the core claim in peer-reviewed form — blockchain substitutes distributed, tamper-evident, automatically-enforced trust for a trusted third party in agent coordination — *and* had already identified its principal limitation (coordination expressiveness and inter-contract interaction). The recent literature largely rediscovers and re-instruments this claim for LLM agents.

---

## 3. The recent advancement: blockchain coordination for LLM agents [Well-evidenced]

The current wave reframes the same problem for autonomous LLM-based agents that take actions, call tools, and transact. The anchor survey already in your project — Karim, Van, Khan, Qu, and Kholodov (2025), *AI Agents Meet Blockchain* — is the most comprehensive recent map and explicitly positions blockchain as infrastructure, governance, and key-enabler layers for agent ecosystems (source 8). It should remain the spine of §5, but it can now be supported by primary sources rather than resting on survey claims alone.

The technically strongest sub-thread is **Byzantine-robust agent coordination**, which is the cleanest instance of blockchain genuinely earning its place. The primary source is Chen, Li, Lin, Wang, and Li (2024), *BlockAgents*, published at the ACM Turing Award Celebration Conference — China (source 9). Its argument is sharp: as the number of LLM agents grows, biased or colluding agents can sabotage collaboration, and most LLM multi-agent frameworks are defenceless against such Byzantine behaviour. BlockAgents introduces a Proof-of-Thought consensus with stake-based miner designation and multi-round debate-style voting; experimentally it holds the accuracy impact of poisoning attacks below three percent and the success rate of backdoor attacks below five percent.

This sub-thread also contains an internal *methodological conflict* worth surfacing in the paper. Jo (2025), *Byzantine-Robust Decentralized Coordination of LLM Agents* (the DecentLLMs framework), critiques the leader-driven coordination that BlockAgents and the related "Trusted MultiLLMN" (which uses Weighted Byzantine Fault Tolerance) depend on (source 10). Jo's objection is that a fixed-leader design is vulnerable to targeted attacks on the leader and that consecutive malicious leaders force costly re-consensus rounds, which is especially expensive given LLM invocation latency. This is a live, unresolved disagreement about *how* to do blockchain-mediated agent consensus — exactly the kind of current-topic tension a "Current Topics" paper should report rather than paper over.

Adjacent recent primary sources broaden the picture. Ding (2025) proposes a Decentralized Multi-Agent System with trust-aware communication, combining blockchain primitives with self-sovereign identity so that agents collaborate without a single trusted third party (source 11). A 2025 framework for transparent, incentive-compatible collaboration in decentralized LLM multi-agent systems demonstrates blockchain-verified task allocation and incentive alignment in simulation, while candidly noting that it lacks real-world deployment and full economic modelling (source 12). A 2025 zero-trust survey of secure multi-LLM agentic AI situates blockchain within the broader security stack, framing it as a Byzantine-fault-tolerant, authority-free record layer for multi-LLM environments (source 13).

---

## 4. Applications, deployments, and standards [Moderately evidenced]

**Peer-reviewed applications.** The strongest applications evidence is Scientific Reports (2025), a peer-reviewed paper that embeds smart-contract-based incentive mechanisms into a Multi-Agent Soft Actor-Critic (MASAC) reinforcement-learning algorithm and validates it in two domains — automated market bidding and intelligent traffic control — reporting improved social welfare and reduced collusion (source 14). This is the most defensible single "application" citation because it is experimental, peer-reviewed, and domain-concrete, and it ties multi-agent reinforcement learning (a genuine AI-decision setting) to on-chain enforcement.

**Emerging machine-to-machine standards.** The genuinely current development — and the part that will read as 2026-fresh to your examiner — is the appearance of formal agent identity and payment standards. ERC-8004 ("Trustless Agents") defines on-chain identity, reputation, and validation registries for AI agents, extending Google's Agent-to-Agent protocol with verifiable on-chain trust; it was authored in August 2025 and reached Ethereum mainnet in early 2026 (source 15). It is paired with x402, an open standard that revives the HTTP 402 "Payment Required" status code to enable instant stablecoin micropayments between agents (source 16). For academic citation, prefer the primary technical specifications (the EIP and the x402 white paper) over the many exchange and vendor blog explainers, which are third-class under your source hierarchy.

**Academic treatment of the standards layer.** Two 2026 academic works let you cite this space rigorously rather than journalistically: a systematization-of-knowledge paper on blockchain agent-to-agent payments (source 17) and a survey of autonomous agents on blockchains covering standards, execution models, and trust boundaries (source 18). A further 2026 preprint, *The Agent Economy*, makes the conceptual case that blockchain supplies three properties no human-centric infrastructure offers agents — permissionless participation, trustless settlement, and machine-to-machine micropayments (source 19); it is useful for framing benefits, though as a preprint it is moderately rather than strongly evidenced.

> [Note on the draft's examples] Your current §5 draft cites ai16z and Luna as deployed examples. These are illustrative as projects, but they are not peer-reviewed and they sit close to the tokenomics boundary the scope excludes. The ERC-8004 / x402 standards and the BlockAgents / MASAC frameworks are stronger, more citable substitutes for the same point.

---

## 5. Challenges and limitations [Well-evidenced]

The challenges divide into engineering limits, security limits, and a definitional limit.

**Engineering limits.** The Karim survey and the broader literature converge on scalability, latency, and interoperability as the binding constraints (source 8). Consensus overhead and transaction-validation latency impair the real-time operation autonomous agents require, growing ledger-storage requirements strain lightweight agents, and cross-chain interaction among agents on heterogeneous platforms introduces both friction and new attack surfaces (double-spending, replay). Jo (2025) sharpens the latency point specifically for LLM agents: blockchain consensus interacts badly with the already-high latency of LLM invocations, so a naive design multiplies delay (source 10).

**Security limits.** Adding inter-agent trust is not free. A 2025 study introduces and empirically validates a *Trust-Vulnerability Paradox*: raising inter-agent trust to improve coordination simultaneously widens the risk of over-exposure and over-authorization (source 20). This is a valuable, non-obvious challenge citation because it complicates the simple "blockchain adds trust, therefore better" narrative. Collusion among multiple malicious agents — the very threat BlockAgents targets — is itself evidence that the security problem is intrinsic, not incidental (source 9).

**The definitional limit.** The most important challenge for your paper's integrity is whether blockchain is *warranted* at all, addressed in §6.

---

## 6. Edge case — does agent coordination pass the "do you need a blockchain" test? [Well-evidenced]

This is the critical assumption examination the scope promised. The honest answer from the literature is: *sometimes, and less often than the hype implies.*

The strongest skeptical source is peer-reviewed: Mafrur (2025), *AI-Based Crypto Tokens: The Illusion of Decentralized AI?*, in IET Blockchain (source 21). Its argument maps almost exactly onto the Wüst and Gervais test. It observes that decentralized AI networks incur real coordination overhead, consensus latency, and resource fragmentation; that production "agent" systems such as Fetch.ai in practice run agents off-chain and touch the chain only for settlement or identity verification; and it recommends explicitly validating whether decentralization is essential before introducing a token or chain — decentralization should be adopted only where it yields data ownership, censorship resistance, or trustless coordination that a centralized model cannot provide. This is the academic articulation of "do you need a blockchain," and it belongs in your discussion section.

The resolution is a *layered* one, which the convergence literature supports: heavy AI inference runs off-chain, while the chain is reserved for the narrow functions where mutually-distrusting agents genuinely cannot rely on a central server — identity, reputation, verifiable settlement, and contract enforcement. Within that envelope, the Byzantine-robust frameworks (source 9, source 10) and the standards layer (sources 15–18) describe real, non-substitutable value: an open population of agents from different operators, any of which may be adversarial, is precisely the mutually-distrusting-writers-without-an-acceptable-intermediary condition your framework requires. Outside that envelope — a single operator's internal agents coordinating behind a trusted boundary — the test fails and a database or message bus is the correct tool.

**Critical-assumption verdict:** the parent paper's thesis survives, but only in the narrow, defensible form. Multi-agent coordination is a *legitimate* showcase for blockchain-complements-AI *if and only if* the section is scoped to open, multi-operator, adversarial-tolerant agent populations. Presented broadly ("agents need blockchains"), the claim is contradicted by peer-reviewed skepticism and by the off-chain reality of deployed systems.

---

## Synthesis

Across both literatures the answer to "how does blockchain complement AI in multi-agent coordination" is consistent and, when scoped correctly, well-evidenced. Blockchain substitutes distributed, tamper-evident, automatically-enforced trust for a trusted third party, allowing autonomous agents operated by mutually-distrusting parties to verify identity, accumulate reputation, enforce commitments, and tolerate Byzantine members without a central coordinator. The foundational literature (Calvaresi et al. 2018; Ciatto/Mariani et al. 2020; the robotics cluster) established this claim, and its principal limitation, in peer-reviewed form by 2020. The recent literature re-instruments it for LLM agents (Karim et al. 2025; BlockAgents 2024; DecentLLMs 2025) and operationalises it through emerging standards (ERC-8004, x402) and peer-reviewed reinforcement-learning applications (Scientific Reports 2025).

The benefit is real but bounded. The applications are concentrated in open, adversarial-tolerant settings — agent marketplaces, cross-operator task allocation, decentralized model collaboration — and thin elsewhere. The challenges are intrinsic: latency and scalability that bite hardest precisely when agents act in real time, a trust-vulnerability paradox in which more inter-agent trust enables more abuse, and a genuine risk of adopting blockchain where a centralized server would serve better. For the paper, the decisive move is to scope §5 to the open, multi-operator, mutually-distrusting case, where the Wüst and Gervais test is passed cleanly, and to cite Mafrur (2025) as the disciplining counter-voice.

**Remaining gaps:** (a) the ERC-8004 / x402 standards literature is fast-moving and largely non-academic; treat the primary specifications as white-paper-tier and expect the peer-reviewed treatment to mature after this paper is written. (b) The BlockAgents-versus-DecentLLMs methodological dispute is unresolved; report it as an open question rather than choosing a side. (c) Several recent items are arXiv preprints (sources 11, 12, 13, 18, 19) and should be flagged as such in your reference list per your source hierarchy.

---

## Assumptions Log

**Minor assumptions**
- "Coordination" is used in the MAS sense (task allocation, agreement, conflict resolution), not the distributed-systems clock/state sense. The cited sources use the MAS sense. *(§1)*
- ai16z and Luna are treated as illustrative projects, not citable evidence, consistent with the tokenomics exclusion. *(§4)*

**Critical assumptions (examined)**
- *That multi-agent coordination is a valid showcase for blockchain-complements-AI.* Examined in §6. The assumption holds **only** under narrow scoping (open, multi-operator, adversarial-tolerant agent populations). Under broad scoping it is contradicted by peer-reviewed skepticism (Mafrur 2025) and by the off-chain reality of deployed agent systems. The paper must adopt the narrow framing for the section to be defensible.
- *That the trusted-third-party-removal claim transfers from foundational MAS to LLM agents.* The transfer is supported (the mechanism is identical), but LLM-specific latency costs (source 10) and LLM-specific collusion/deception risks (source 9) are new and must be acknowledged, not assumed away.

---

## Works Cited (IEEE)

[1] R. Sikora and M. J. Shaw, "A multi-agent framework for the coordination and integration of information systems," *Management Science*, vol. 44, no. 11-part-2, pp. S65–S78, 1998. *(verify pagination on retrieval)* x

[2] L. Sun, Y. Yang, Q. Duan, Y. Shi, C. Lyu, Y.-C. Chang, C.-T. Lin, and Y. Shen, "Multi-agent coordination across diverse applications: A survey," *arXiv preprint*, arXiv:2502.14743, 2025. x

[3] D. Calvaresi, A. Dubovitskaya, J. P. Calbimonte, K. Taveter, and M. Schumacher, "Multi-agent systems and blockchain: Results from a systematic literature review," in *Advances in Practical Applications of Agents, Multi-Agent Systems, and Complexity (PAAMS 2018)*, Springer, 2018, pp. 110–126. x

[4] S. Ciatto, S. Mariani, et al., "Blockchain-based coordination: Assessing the expressive power of smart contracts," *Information*, vol. 11, no. 1, art. 52, 2020. *(verify full author list on retrieval)* x

[5] I. Afanasyev, A. Kolotov, R. Rezin, K. Danilov, A. Kashevnik, and V. Jotsov, "Blockchain solutions for multi-agent robotic systems: Related work and open questions," in *Proc. 24th Conf. of Open Innovations Association (FRUCT)*, 2019. x

[6] J. Peña Queralta and T. Westerlund, "Blockchain-powered collaboration in heterogeneous swarms of robots," *arXiv preprint*, arXiv:1912.01711, 2019. x

[7] (Author list to verify), "A blockchain integration to support transactions of assets in multi-agent systems," *Engineering Applications of Artificial Intelligence*, 2021. *(complete authors and volume/pages on retrieval; used primarily as a citation hub for Kapitonov et al. 2017/2018, Mariani et al. 2017, Ciatto et al. 2018)* x

[8] M. M. Karim, D. H. Van, S. Khan, Q. Qu, and Y. Kholodov, "AI agents meet blockchain: A survey on secure and scalable collaboration for multi-agents," *Future Internet*, vol. 17, no. 2, art. 57, 2025, doi: 10.3390/fi17020057.

[9] B. Chen, G. Li, X. Lin, Z. Wang, and J. Li, "BlockAgents: Towards Byzantine-robust LLM-based multi-agent coordination via blockchain," in *Proc. ACM Turing Award Celebration Conf. — China 2024*, 2024, pp. 187–192, doi: 10.1145/3674399.3674445.

[10] Y. Jo, "Byzantine-robust decentralized coordination of LLM agents," *arXiv preprint*, arXiv:2507.14928, 2025.

[11] Y. Ding, "Decentralized multi-agent system with trust-aware communication," *arXiv preprint*, arXiv:2512.02410, 2025. *(verify full author list on retrieval)*

[12] (Author list to verify), "Towards transparent and incentive-compatible collaboration in decentralized LLM multi-agent systems: A blockchain-driven approach," *arXiv preprint*, arXiv:2509.16736, 2025.

[13] (Author list to verify), "Secure multi-LLM agentic AI and agentification for edge general intelligence by zero-trust: A survey," *arXiv preprint*, arXiv:2508.19870, 2025.

[14] (Author list to verify), "Blockchain-enhanced incentive-compatible mechanisms for multi-agent reinforcement learning systems," *Scientific Reports*, vol. 15, 2025, doi: 10.1038/s41598-025-20247-8.

[15] M. De Rossi, D. Crapis, J. Ellis, and E. Reppel, "ERC-8004: Trustless Agents," *Ethereum Improvement Proposal (ERC-8004)*, 2025. [Online]. *(cite the primary EIP text on retrieval)*

#here

[16] "x402: An open standard for internet-native payments," x402 white paper, 2025. [Online]. Available: https://www.x402.org/x402-whitepaper.pdf

[17] (Author list to verify), "SoK: Blockchain agent-to-agent payments," *arXiv preprint*, arXiv:2604.03733, 2026.

[18] (Author list to verify), "Autonomous agents on blockchains: Standards, execution models, and trust boundaries," *arXiv preprint*, arXiv:2601.04583, 2026. (121 pages)

[19] (Author list to verify), "The agent economy: A blockchain-based foundation for autonomous AI agents," *arXiv preprint*, arXiv:2602.14219, 2026.

[20] (Author list to verify), "The trust-vulnerability paradox in LLM multi-agent systems," *arXiv preprint*, 2025. *(locate arXiv ID and full citation on retrieval)*

[21] R. Mafrur, "AI-based crypto tokens: The illusion of decentralized AI?," *IET Blockchain*, 2025, doi: 10.1049/blc2.70015.

---

## Bibliography (read for orientation, not load-bearing)

- "Decentralized AI — why blockchain is the missing governance layer," HodlX / CryptoRank, 2025. *(third-class; orientation only — DFL and agent-marketplace framing)*
- "AI and crypto convergence forces a reality check on token narratives," Investing.com, 2026. *(third-class; supports the layered off-chain/on-chain framing in §6)*
- "AI + blockchain in 2026: 5 on-chain inference patterns," Agile Soft Labs blog, 2026. *(third-class; supports the latency/cost critique in §5)*
- GitHub, *awesome-erc8004* curated resource list. *(non-academic index; useful only to locate the primary ERC-8004 / x402 / DID-VC sources)*

---

## Note on use

Per your established workflow, each source above is a candidate for a Markdown note (Overview / Key Points / Relevance / Limitations / Open Questions). The two highest-priority notes are source 3 (Calvaresi et al. 2018, the foundational spine) and source 21 (Mafrur 2025, the disciplining counter-voice), because together they let §5 make a strong claim and then bound it honestly. Several reference entries carry a *(verify on retrieval)* flag where the search snippets did not expose the complete author list — these match the incomplete-author flags already in your main reference list.
