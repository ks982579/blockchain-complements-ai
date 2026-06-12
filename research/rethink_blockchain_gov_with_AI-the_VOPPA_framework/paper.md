**==> picture [102 x 35] intentionally omitted <==**

## _Systematic Review_ 

## **Rethinking Blockchain Governance with AI: The VOPPA Framework** 

**Catalin Daniel Morar[1,2,] * , Daniela Elena Popescu[2] , Ovidiu Constantin Novac[2] and David Ghiurău[1,2]** 

- 1 Department of Computers and Information Technology, Politehnica University of Timisoara, 2 V. Parvan Blvd, 300223 Timisoara, Romania 

- 2 Department of Computers and Information Technology, Faculty of Electrical Engineering and Information Technology, University of Oradea, 410087 Oradea, Romania 

- Correspondence: catalin-daniel.morar@student.upt.ro 

## **Abstract** 

Blockchain governance has become central to the performance and resilience of decentralized systems, yet current models face recurring issues of participation, coordination, and adaptability. This article offers a structured analysis of governance frameworks and highlights their limitations through recent high-impact case studies. It then examines how artificial intelligence (AI) is being integrated into governance processes, ranging from proposal summarization and anomaly detection to autonomous agent-based voting. In response to existing gaps, this paper proposes the Voting Via Parallel Predictive Agents (VOPPA) framework, a multi-agent architecture aimed at enabling predictive, diverse, and decentralized decision-making. Strengthening blockchain governance will require not just decentralization but also intelligent, adaptable, and accountable decision-making systems. 

**Keywords:** blockchain governance; decentralized systems; decision-making; smartcontracts; DAO; autonomous agents; artificial intelligence; natural language processing; machine learning; large language models 

## **1. Introduction** 

Academic Editor: Paolo Bellavista 

Received: 29 July 2025 Revised: 13 September 2025 Accepted: 2 October 2025 Published: 4 October 2025 

**Citation:** Morar, C.D.; Popescu, D.E.; Novac, O.C.; Ghiurău, D. Rethinking Blockchain Governance with AI: The VOPPA Framework. _Computers_ **2025** , _14_ , 425. https://doi.org/10.3390/ computers14100425 

**Copyright:** © 2025 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license (https://creativecommons.org/ licenses/by/4.0/). 

Blockchain is increasingly leveraged as a foundational technology for distributed applications, including supply chain management, digital identity, Internet of Things (IoT), and education [1–3]. As blockchain deployments proliferate, governance mechanisms by which rules are set, decisions made, and responsibilities assigned, becomes a critical determinant of system effectiveness [4]. 

Unlike traditional centralized architectures, where a single authority enforces uniform decisions and bears accountability, blockchain systems rely on distributed decision rights and collective rule enforcement. This shift distributes power but introduces trade-offs: consensus-driven change can slow adaptation, and decentralization may dilute clarity about who is accountable [5]. 

While the topic of blockchain governance has attracted growing attention, the existing literature remains unevenly distributed across conceptual frameworks, empirical insights, and technical challenges. Several recent studies have sought to map the DAO landscape, focusing on taxonomies, functionalities, and real-world implementations [6–9]. Other works have concentrated on governance vulnerabilities, emphasizing security risks, and low voter participation [10,11]. A complementary set of studies has proposed improvements inspired by corporate or legal governance traditions [4,12], while emerging standards aim to formalize common terminology and structures [13]. 

https://doi.org/10.3390/computers14100425 

_Computers_ **2025** , _14_ , 425 

_Computers_ **2025** , _14_ , 425 

2 of 25 

Despite these valuable contributions, existing research often addresses governance either in isolation from real-world breakdowns or without integrating recent advances in AI. Most notably, although recent works acknowledge the potential of AI to support decentralized systems [14–16], few offer concrete architectural proposals for integrating AI technology in governance workflows. 

In this context, our study contributes by offering a layered and comparative synthesis of governance models and failure cases, a structured analysis of current and emerging AI applications in decentralized governance, and a novel architecture, VOPPA, designed to enhance decentralization, diversity, and fault tolerance in autonomous decision-making. 

To identify the relevant literature, the initial search was conducted across MDPI: Basel, Switzerland, IEEE Xplore: New York, NY, USA, and other academic and web-based sources using the keywords “blockchain” AND “governance,” with a particular focus on studies that also explore applications of AI. A selection process based on predefined exclusion and inclusion criteria, as can be seen in Table 1, was applied to ensure the relevance, quality, and topical focus of the reviewed studies. 

**Table 1.** Exclusion and inclusion criteria. 

|**Exclusion**|**Inclusion**|
|---|---|
|Older than fiveyears|Focus on blockchaingovernance systems|
|Lacks scientific rigor|Discusses AI applications in blockchain<br>governance contexts|



Figure 1 illustrates the process of identifying and selecting the initial set of resources used in this study. A total of 103 sources were first screened based on titles and abstracts, applying predefined exclusion criteria. Following full-text review and duplicate removal, 75 resources remained eligible. Applying the inclusion criteria further narrowed the selection to 52 core publications. To enhance coverage, backward and forward snowballing were then performed, allowing the inclusion of additional relevant studies identified through citation tracking. 

**==> picture [164 x 280] intentionally omitted <==**

**----- Start of picture text -----**<br>
:<br>Figure 1. Resource identification process.<br>**----- End of picture text -----**<br>


_Computers_ **2025** , _14_ , 425 

3 of 25 

The following sections are structured around four central research questions derived from the reviewed literature and identified knowledge gaps: 

- What types of governance models exist in blockchain ecosystems, and how do they differ? (Section 2) 

- What can we learn from recent blockchain governance failures? (Section 3) 

- What role can AI play in enhancing decentralized governance? (Section 4) 

- How can blockchain governance be reimagined to address emerging needs and future complexity? (Sections 5 and 6) 

## **2. Blockchain Governance: Layers, Models, and Challenges** 

Blockchain governance encompasses the mechanisms, structures, and processes by which decisions are formulated, negotiated, and implemented within a blockchain network. It represents the set of arrangements that enable stakeholders to collectively steer, regulate, and coordinate the evolution and operation of a blockchain system to which they all contribute [17]. In practice, this involves a blend of technical rules encoded in the protocol and social mechanisms (off-chain discussions, developer decisions) that together facilitate coordination in a decentralized system [18]. 

Public blockchains like Bitcoin and Ethereum face unique governance issues because anyone can participate, and there is no central authority. By contrast, private or permissioned blockchains usually rely on more traditional or centralized governance, since a known consortium or organization can decide on upgrades and rules [19,20]. 

In this section, we examine the different dimensions across which blockchain governance operates, the principal governance models, and the main challenges confronting blockchain governance. 

## _2.1. Governance Layers_ 

Recent studies conceptualize blockchain governance as a multi-layered system involving both technical and social components. A widely accepted distinction is that between on-chain governance, defined as formalized, software-encoded decision-making processes embedded directly into the protocol or smart contracts, and off-chain governance, which encompasses informal, human-led coordination such as developer discussions or community deliberation [21,22]. 

More detailed frameworks identify three discrete governance layers: the on-chain protocol layer, the off-chain development layer, and the off-chain community layer, each capturing a different aspect of how governance is enacted across the blockchain ecosystem [17,23], as can be seen in Figure 2. The summarization of the three layers can be found in Table 2. 

**Figure 2.** Governance layers in blockchain systems. 

_Computers_ **2025** , _14_ , 425 

4 of 25 

**Table 2.** A summary of the three blockchain governance layers as described in the recent literature [17,23]. 

|**Governance Layer**|**Location**|**Main Actors**|**Characteristics**|
|---|---|---|---|
||||Automated, code-based|
|On-chain Protocol<br>Layer|On the blockchain (core protocol)|Validators, token<br>holders|governance (e.g., protocol<br>upgrades, parameter changes)|
||||via on-chain voting|
||||Technical proposal design, code|
|Off-chain|Outside the blockchain (dev|Core developers,|review, and software updates|
|Development Layer|tools, repos)|maintainers|coordinated via informal|
||||processes|
|Off-chain<br>Community Layer|Social and institutional space|Users, foundations,<br>and communities|Broad consensus-building and<br>ecosystem alignment through<br>discussions|



Although these layers are often described separately, in practice, they form a tightly coupled governance cycle. Off-chain discussions within development and community layers usually serve as the entry point for new ideas, conflict resolution, or refinement of technical proposals. Once a sufficient level of social consensus is achieved, proposals are translated into code changes and submitted for on-chain voting or execution. 

While the three-layer governance model (on-chain protocol, off-chain development, offchain community) effectively captures the origin and nature of decision-making processes in blockchain ecosystems, a complementary way to describe governance mechanisms is to consider what part of the system is technically affected and what functional purpose the governance decision serves (Figure 3). 

**Figure 3.** Dimensions of blockchain governance. 

The first dimension, location, focuses on the specific technical component of the blockchain that is directly altered by governance decisions. Research shows that governance actions primarily affect two key layers: the protocol (network) layer and the smart contract (application) layer [24]. 

The technical location affected by a governance action often determines the nature and scope of its intended purpose, as it is highlighted in Figure 4. The protocol layer includes consensus rules, block production parameters, and validator selection criteria. Changes here typically require software updates by nodes or, in on-chain governance systems, are enacted through formal voting mechanisms embedded in the protocol [25,26]. In contrast, the smart contract layer governs decentralized applications (dApps) and DAOs: here, governance decisions alter operational parameters, upgrade contract modules, conflict resolution, or adjust treasury and access controls via smart-contract–based mechanisms [7,27,28]. 

_Computers_ **2025** , _14_ , 425 

5 of 25 

**Figure 4.** Elaborated dimensions of blockchain governance. Location—orange; Purpose—yellow. 

## _2.2. Governance Models_ 

Blockchain governance frameworks can generally be categorized into three major models: on-chain, off-chain, and hybrid, as it is shown in Figure 5. Each of these approaches defines how decisions are proposed, debated, and enacted across decentralized networks, reflecting different trade-offs between transparency, decentralization, efficiency, and adaptability [29]. 

**Figure 5.** Governance models. 

In on-chain governance, the rules and procedures for making changes are codified within the blockchain itself, allowing decisions to be executed automatically through smart contracts. Off-chain governance, by contrast, relies on informal structures, such as developer discussions, community forums, and social consensus, outside the blockchain’s technical layer. Hybrid governance attempts to integrate the strengths of both by combining formalized, automated voting with the flexibility of off-chain deliberation [30,31]. 

## 2.2.1. On-Chain 

On-chain governance represents a model in which governance procedures are fully integrated into the blockchain protocol itself. Under this approach, participants (typically 

_Computers_ **2025** , _14_ , 425 

6 of 25 

token holders or network validators) can propose and vote on changes to the system using blockchain-native mechanisms, as can be seen in Figure 6. All proposals, votes, and decisions are recorded immutably on-chain, and approved actions are automatically executed via smart contracts without requiring human intervention [29,32]. 

**Figure 6.** On-chain governance model. 

This model is often praised for its transparency and automation. Since governance interactions are publicly verifiable and tamper-resistant, on-chain governance can ensure accountability and eliminate the need for off-chain negotiation. Moreover, its reliance on deterministic execution minimizes ambiguity in how decisions are implemented [29,32,33]. 

Despite these advantages, several limitations persist. Participation is frequently low, as not all stakeholders are incentivized or equipped to engage in governance. Additionally, because voting power is commonly tied to token ownership, the system is vulnerable to power centralization, where large stakeholders exert disproportionate influence. Another concern is rigidity; once a decision is executed, reversing or correcting it can be extremely difficult, particularly in the absence of emergency mechanisms [29]. 

A prominent example of on-chain governance in practice is Tezos, a blockchain specifically designed to support protocol evolution without requiring disruptive hard forks. It features a native governance layer where participants can propose, test, and vote on protocol upgrades. Once a proposal passes through a formalized multi-stage voting process, it is automatically integrated into the network, embodying the principle of self-amendment [34]. 

However, while Tezos offers a structured and transparent governance mechanism, it also faces significant limitations. Voting power is directly proportional to token holdings, which raises concerns about governance centralization and elite capture. Moreover, the complexity of the proposal and voting process can discourage average users from participating, contributing to low voter turnout. These factors may undermine the system’s inclusiveness, despite its technical elegance and procedural clarity. 

## 2.2.2. Off-Chain 

Off-chain governance refers to decision-making processes that occur outside the formal boundaries of the blockchain protocol. Rather than being codified at the protocol level, governance decisions are developed through informal mechanisms such as community discussions, developer coordination, social signaling, and consensus-building across different stakeholder groups (Figure 7). These interactions typically take place on forums, GitHub repositories, or social media [29,30,35,36]. 

_Computers_ **2025** , _14_ , 425 

7 of 25 

**Figure 7.** Off-chain governance model. 

This model offers a high degree of flexibility and adaptability, enabling nuanced debate and iterative refinement of proposals before implementation. It allows stakeholders to negotiate trade-offs, consider broader ecosystem impacts, and account for social, economic, and ethical dimensions that cannot easily be encoded in software. As such, off-chain governance is particularly well-suited to managing complex or controversial protocol changes [29,30,35,36]. 

However, the lack of transparency and formal accountability are persistent challenges. Because decisions are made outside the blockchain, there is often no public, auditable record of deliberations. Moreover, influence tends to be unevenly distributed, with prominent developers, mining pools, or foundation members exerting outsized control over key decisions. This can reduce inclusivity and erode trust in the fairness of the governance process. 

A well-known example of off-chain governance is Ethereum, where protocol upgrades are proposed through Ethereum Improvement Proposals (EIPs). These proposals are openly discussed by core developers, community members, and stakeholders until a general consensus is reached [37]. Final decisions are not enforced by protocol rules but rely on voluntary coordination and social legitimacy; a process that, while inclusive in theory, can be slow, opaque, and vulnerable to influence from centralized actors. 

## 2.2.3. Hybrid 

As it is highlighted in Figure 8, hybrid governance models integrate elements from both on-chain and off-chain governance, aiming to strike a balance between the transparency and automation of on-chain mechanisms and the flexibility and deliberative depth of off-chain processes. In such systems, proposals are often discussed and refined in informal settings before being submitted to a formal on-chain vote. This dual-layered approach allows communities to combine human judgment and social consensus with the technical enforcement and auditability of blockchain-based execution [29,38]. 

The hybrid model seeks to address the limitations of each individual governance type. By enabling open discussion prior to formal voting, it improves stakeholder engagement and proposal quality. At the same time, on-chain execution mechanisms ensure that finalized decisions are enforced consistently and without ambiguity [29,38]. 

_Computers_ **2025** , _14_ , 425 

8 of 25 

**Figure 8.** Hybrid governance model. 

Nevertheless, this integration comes with its own set of challenges. Managing coordination between off-chain dialog and on-chain enforcement introduces complexity and may lead to governance bottlenecks. Furthermore, hybrid systems do not fully eliminate the risk of centralization: influence can still be unequally distributed in both layers, via token-weighted voting on-chain and dominance by vocal or well-connected actors off-chain. The additional procedural overhead can also slow down decision-making, especially when conflicting interests emerge across governance layers. 

A prominent implementation of hybrid governance is Cardano, particularly through its Project Catalyst initiative. This platform invites community members to propose ideas, deliberate in structured off-chain formats, and then vote using an on-chain mechanism powered by ADA. The process aims to combine inclusivity and transparency, offering a more comprehensive model for decentralized coordination. However, like other governance models, Project Catalyst may be susceptible to issues such as inconsistent participation and variable vote quality, particularly as the platform scales. These potential challenges highlight that hybrid governance remains an evolving model, requiring further refinement to ensure robustness, fairness, and long-term effectiveness. 

To provide a clearer overview of the three governance models discussed, we summarize their key characteristics and associated challenges in Table 3. This comparative perspective helps highlight the trade-offs each model presents in terms of decentralization, transparency, flexibility, and operational complexity. 

As illustrated above, no single governance model offers a definitive solution to the complex demands of decentralized systems. On-chain governance ensures transparency and automation but often struggles with participation and equitable influence. Off-chain governance allows for richer deliberation but lacks formal accountability and auditability. Hybrid models aim to combine the strengths of both approaches, yet introduce new coordination challenges and may inherit the limitations of each. Ultimately, the choice of governance model reflects a set of trade-offs, and the effectiveness of any approach depends 

_Computers_ **2025** , _14_ , 425 

9 of 25 

on the specific context of the blockchain ecosystem, the nature of its stakeholders, and the evolving requirements of the network. 

**Table 3.** Comparative summary of blockchain governance models. 

|**Model**|**Characteristics**|**Limitations**|
|---|---|---|
|On-chain|Protocol-embedded governance rules<br>Smart contract–based proposal execution<br>Transparent and auditable voting<br>Direct stakeholder participation|Low voter turnout<br>Token-based voting may lead to power<br>concentration<br>Diffcult to reverse decisions once executed<br>Limited adaptability|
|||Lack of transparency and formal|
||Governance via community discussions,|accountability|
|Off-chain|developer input, and informal consensus<br>High deliberative fexibility|Infuence concentrated among vocal actors<br>or developers|
||Human-centric coordination mechanisms|Slower and less predictable|
|||decision-making|
|Hybrid|Combines off-chain deliberation with<br>on-chain execution<br>Balances fexibility and automation<br>Encourages broader community<br>engagement|Coordination complexity between layers<br>Susceptible to both token-based<br>centralization and informal dominance<br>Participation and vote quality variability|



## **3. Case Studies** 

To further underscore the critical role of governance in blockchain ecosystems, this section presents a selection of real-world case studies. These examples highlight how different governance models have functioned under pressure, revealing both their strengths and vulnerabilities. By examining specific incidents from various platforms, we aim to illustrate how governance structures directly impact network stability, decision-making processes, and the ability to respond to crises or evolving community needs. 

## _3.1. The DAO Attack_ 

A landmark case in the evolution of blockchain governance is the attack on the DAO, a decentralized investment vehicle built on Ethereum in 2016. Structured as a fully on-chain governance system, the DAO allowed token holders to vote on funding proposals via smart contracts, with decisions executed automatically and without centralized oversight. While this model showcased the potential of code-driven coordination, it also revealed the risks of entrusting critical decisions to rigid, pre-programmed logic [39,40]. 

Shortly after launch, a flaw in the smart contract was exploited, enabling an attacker to redirect a substantial portion of funds in a way that, although technically permitted by the contract, clearly violated the community’s expectations (Figure 9). The Ethereum protocol itself lacked any built-in mechanism to halt or reverse such actions. This forced the community to coordinate off-chain, through discussions among developers, users, and miners, to deliberate a response. The resolution came in the form of a controversial hard fork, which effectively split the Ethereum blockchain into two networks [39,41,42]. 

This incident underscored the limitations of pure on-chain governance and demonstrated that even highly decentralized systems require robust and responsive governance frameworks capable of addressing emergencies. The DAO attack remains a defining example of why blockchain ecosystems must balance automated rule enforcement with mechanisms for timely, collective decision-making in moments of crisis. 

_Computers_ **2025** , _14_ , 425 

10 of 25 

**Figure 9.** The DAO attack representation. 

## _3.2. Build Finance DAO Takeover_ 

Another illustrative case of governance vulnerability occurred in early 2022, when Build Finance DAO, a decentralized organization operating on a token-based governance model, became the target of a hostile takeover. As it is shown in Figure 10, the attack was made possible by the acquisition of a sufficient number of governance tokens by a single actor, who then passed a malicious proposal to assume full control over the DAO [43–45]. 

**Figure 10.** Build Finance DAO attack representation. 

Once the proposal was approved, due to low voter turnout and the absence of safeguards, the attacker was able to initiate actions that disrupted the organization’s operations and integrity. This event exposed a critical flaw in token-weighted governance systems: when participation is minimal and safeguards are weak, a determined individual or entity can seize control and act unilaterally, regardless of the broader community’s intentions [43–45]. 

The Build Finance incident serves as a cautionary example of how governance capture can occur in decentralized environments, particularly when decision-making power is 

_Computers_ **2025** , _14_ , 425 

11 of 25 

concentrated and engagement from token holders is limited. It highlights the need for governance mechanisms that not only ensure fair voting processes but also protect against apathy and manipulation, factors that can compromise the very principles of decentralization. 

## _3.3. The Bitcoin Cash (BCH) Fork_ 

The split between Bitcoin and BCH in 2017 stands as a significant example of how the absence of a formal governance framework can lead to deep and irreversible fragmentation within a blockchain ecosystem. At the core of the disagreement was the question of how to scale Bitcoin to accommodate a growing number of transactions. As it is represented in Figure 11, one group advocated for increasing the block size limit, while others preferred implementing solutions like Segregated Witness [46–48]. 

**Figure 11.** BCH fork representation. 

Without an institutionalized process for deliberation or conflict resolution, the debate escalated into a contentious and prolonged stalemate. Coordination was largely informal, relying on off-chain deliberation, often with no clear path to consensus. In the absence of an agreed-upon governance mechanism, the disagreement culminated in a hard fork, resulting in the creation of BCH as a separate chain [46–48]. 

This case illustrates how a lack of structured governance can leave even the most decentralized and secure blockchains vulnerable to ideological splits and network fragmentation. It reinforces the idea that decentralized systems still require clearly defined coordination mechanisms to manage change and resolve disputes effectively, especially as communities grow and diverge in vision. 

## _3.4. Compound Governance Attack_ 

In 2024, Compound Finance, a leading DeFi protocol, experienced a significant governance challenge when a well-resourced actor (or coordinated group) launched a series of aggressive proposals designed to seize control of the platform’s governance (Figure 12). By accumulating a high concentration of COMP tokens and capitalizing on extremely low voter participation, they succeeded in passing controversial changes that benefitted their specific interests [49–51]. 

_Computers_ **2025** , _14_ , 425 

12 of 25 

**Figure 12.** Compound attack representation. 

This event highlighted the inherent vulnerabilities of token-weighted governance mechanisms, particularly when voter participation is low. Despite its decentralized structure, the protocol was effectively influenced by a single well-coordinated entity with significant token holdings. Moreover, the open proposal process created opportunities for governance exploitation. While the Compound community eventually intervened to mitigate the outcome, the incident revealed that DeFi governance models without robust engagement thresholds or proposal screening mechanisms remain exposed to governance capture [49–51]. 

This case shows the need for more resilient governance frameworks that combine token-weighted voice with safeguards such as proposal pre-approval to prevent opportunistic takeovers. It serves as a contemporary warning that decentralization on paper may hide centralized control in practice. 

## _3.5. Beanstalk Governance Attack_ 

In April 2022, Beanstalk, a permissionless stablecoin protocol, suffered a severe governance exploit due to structural weaknesses in its on-chain voting system and insufficient execution delays. As it is highlighted in Figure 13, an attacker used a flash loan to temporarily acquire a supermajority of governance power via Liquidity Provider (LP) tokens and pushed through a malicious proposal. Using the emergencyCommit function, they executed the proposal and transferred a large portion of Beanstalk’s treasury to their own account [52–54]. 

The swiftness and stealth of the exploit (combined with token-weighted voting and the absence of protective execution mechanisms) made it nearly impossible for the community to detect or respond in time. The attacker was able to execute the proposal and exit with the funds before any intervention could occur, resulting in a substantial loss [52–54]. 

The Beanstalk exploit served as a powerful reminder that decentralized governance systems must be designed with built-in resilience. Mechanisms such as time-locks and minimum participation thresholds should be incorporated to protect against flash-loandriven exploits. 

_Computers_ **2025** , _14_ , 425 

13 of 25 

**Figure 13.** 

The five case studies presented in this section demonstrate that blockchain governance, despite its technological sophistication and decentralization, remains a structurally fragile 

inadequate or poorly designed governance mechanisms can lead to significant disruption, loss of funds, or fragmentation of communities. 

The DAO attack, Build Finance takeover, and Beanstalk flash loan exploit exemplify the dangers of automated governance without safeguards, where technical finality overrides human judgment. The BCH fork illustrates how the absence of formal governance structures can push ideological disagreements into irreversible splits. Meanwhile, the Compound incident reflects how low engagement and concentration of voting power can open the door to opportunistic control even in mature ecosystems. 

Collectively, these cases underline that no governance model is immune to failure, and that decentralization alone does not guarantee resilience or fairness. Effective blockchain governance requires not just open participation and transparency, but also carefully engine 

inte 

viability will increasingly depend on the maturity, adaptability, and inclusiveness of their governance frameworks. 

## **4. AI in Blockchain Governance** 

AI has historically been developed and applied across a wide range of centralized domains, including industrial automation, scientific research, finance, healthcare, and digital content creation. In these contexts, AI systems have been primarily designed to optimize structured processes, support decision-making, analyze large datasets, and automate cognitive and creative tasks. From enabling medical diagnostics and fraud detection to assisting in language modeling, music generation, and scientific discovery, AI has become a foundational tool for augmenting human capabilities in data-rich environments [55–57]. 

As blockchain systems have matured beyond their initial financial applications, AI has increasingly been investigated as a complementary technology capable of enhancing the scalability, automation, and responsiveness of decentralized infrastructures. In the 

_Computers_ **2025** , _14_ , 425 

14 of 25 

recent literature, AI has been applied to various blockchain-related tasks, including smart contract auditing, fraud detection, predictive analytics for crypto markets, and dynamic pricing [58–60]. At the protocol level, machine learning (ML) models are used to detect anomalies in network behavior, support adaptive consensus mechanisms, and improve cybersecurity by anticipating attack vectors [61]. In content-centric ecosystems, generative AI has been integrated into decentralized platforms to enable autonomous creation and curation of digital assets [62]. 

Moreover, AI has played a growing role in improving user experience and operational workflows across dApps and blockchain-based ecosystems. Applications such as summarizing complex proposals, estimating community sentiment, or recommending optimal voting strategies illustrate a shift toward more data-driven and intelligent coordination mechanisms [14,63,64]. 

Among these applications, blockchain governance stands out as a particularly distinct and strategically important use case, where the integration of AI has the potential to reshape decision-making processes, enhance stakeholder participation, and improve systemic responsiveness. As decentralized systems become increasingly complex, AI-supported governance is emerging not just as a possibility, but as a critical area of innovation. 

One of the most common applications of AI in blockchain governance is the automated analysis and summarization of governance proposals (Figure 14). As decentralized autonomous organizations (DAOs) scale, users are faced with increasingly complex and lengthy proposals, which can limit participation and slow decision-making. To address this, some DAO communities have begun integrating natural language processing (NLP) tools that assist in extracting key information, summarizing content, and enabling more efficient proposal evaluation [63–66]. 

**Figure 14.** Blockchain governance AI assistant representation. 

Quan et al. [64] apply sentiment-aware NLP methods to analyze community responses within DAOs and to explore the structure of governance discourse. Their findings provide insights that can support improved understanding and engagement with governance proposals in decentralized environments. Similarly, in a case study of KlimaDAO presented in [63], an AI-assisted governance framework is introduced that leverages chain-of-thought reasoning and personalized recommendations to enhance decision-making clarity and stakeholder engagement. 

_Computers_ **2025** , _14_ , 425 

15 of 25 

However, these tools remain experimental and face several limitations. Most notably, the lack of contextual understanding in highly technical or ideologically polarized proposals can lead to misrepresentation or oversimplification. 

Another noteworthy application of AI in blockchain governance focuses on the detection of abnormal or potentially malicious voting behavior, as it is highlighted in Figure 15, particularly in the context of DAOs. As DAO governance is increasingly targeted by Sybil attacks and coordinated manipulation efforts, researchers have begun exploring AI-driven anomaly detection models to preserve voting integrity [67,68]. 

**Figure 15.** Blockchain governance AI guard representation. 

DuPont [68] proposes a graph deep learning approach to detect Sybil-like behavior in DAO voting networks. By combining graph autoencoders with Graph Convolutional Neural Networks (GCNNs), the method learns latent representations of voter relationships and applies clustering algorithms to isolate anomalous voting clusters. The technique was tested on Snapshot.org data and proved effective in reducing the voting graph by 2–5% through the identification and removal of suspected Sybil accounts. 

In a related direction, the article [67] presents a hybrid anomaly detection framework for DAO environments, combining multiple structural indicators, such as entropy, motif frequency, and clustering coefficients, into a unified index using a Boltzmann machine. This system is integrated with a conversational AI agent that alerts DAO participants to anomalies during governance processes. 

While these approaches highlight the potential of AI to safeguard decentralized decision-making, their deployment remains limited. Most implementations function in post hoc analyses rather than in real-time voting systems. Moreover, the effectiveness of anomaly detection is heavily dependent on data transparency, which varies significantly across DAO platforms. 

One of the most novel and rapidly emerging applications of AI in blockchain ecosystems is the deployment of autonomous AI agents (software entities capable of independently perceiving, reasoning, and executing actions) [69,70]. 

In the existing literature, two roles are distinguished: In the first, AI agents are governed entities, whose identities, behaviors, and ethical compliance are regulated via blockchain protocols [71,72]. For instance, the ETHOS framework [70] proposes a decentralized registry of agents, using smart contracts, discrete identity tokens, and zero-knowledge 

_Computers_ **2025** , _14_ , 425 

16 of 25 

proofs to ensure accountability and auditability of agent behavior. Complementarily, the LOKA Protocol [69] presents layered infrastructure, including identity, intent communication, and ethical consensus mechanisms, to secure multi-agent ecosystems. 

In contrast, a second direction, central to our research interest, views AI agents as active participants in blockchain governance. A comprehensive survey [73] highlights how AI agents are used not only for governance automation (e.g., voting, proposal generation), but also as full-fledged participants in DAO decision-making processes, creating and managing proposal cycles and asset allocation autonomously within a transparent on-chain context (Figure 16). 

**Figure 16.** AI agents’ representation. 

When equipped with large language models (LLMs), these agents gain contextawareness and the ability to engage in natural language interactions, thereby bridging communication between humans and the system [74,75]. This integration supports scalability, adaptability, and continuous refinement of decisions based on real-time data, while maintaining trust through blockchain’s immutable infrastructure. Moreover, their capacity to learn and collaborate enhances collective decision-making and operational responsiveness [76]. 

The AILVE DAO initiative introduces a single autonomous AI agent, built using NLP and ML, that actively participates in DAO governance. It automatically reads and votes on proposals based on learned preferences [77]. While this approach improves efficiency and reduces manual involvement, it also raises concerns about model bias and the risks of over-reliance on a centralized AI component in decentralized systems. 

Ding et al. [78] propose a parallel intelligence framework for DAO governance, which integrates artificial systems, computational experiments, and parallel execution (the ACP method) to simulate and optimize decision-making. This approach allows researchers to model DAO dynamics in a controlled environment using multi-agent simulations that replicate participant behavior under different governance scenarios. A similar ACP-based approach is presented by Liang et al. [79], where the casCAD2 system is used to simulate and evaluate DAO mechanisms using artificial agents. However, these models rely on 

_Computers_ **2025** , _14_ , 425 

17 of 25 

simplified, rule-based agents, rather than real AI in the ML sense. These agents are not trained on data, do not perform prediction or learning, and lack the adaptive behavior typical of modern AI systems. They are used exclusively for simulation purposes, not for making real-time or autonomous decisions within an actual DAO. 

Multi-agent voting councils have been proposed as a theoretical governance design pattern in which diverse LLM-based AI agents deliberate and vote on proposals. While diversity and redundancy help mitigate hallucinations and individual failures, the final decision is often made by a designated coordinator agent, introducing a partially centralized element to an otherwise deliberative architecture [80]. 

Further advancing this paradigm, ref. [81] highlights DeAgents, self-sovereign, LLMpowered agents that hold private keys, manage on-chain assets, issue proposals, cast votes, and communicate autonomously with each other without human control. These agents are built on a modular architecture combining a LLM reasoning core, a secure execution environment (TEE), and an on-chain interface. While they embody a new level of autonomy, the authors highlight the inherent risks of model hallucination and emphasize the need for robust verification and governance mechanisms. 

A notable direction is represented by the ASAP Protocol system, which incorporates autonomous AI agents to facilitate DAO governance. These agents analyze proposals and autonomously vote on behalf of users, enabling delegated participation and decisionmaking without requiring direct user input. Architecturally, the system relies on AI agents built on top of LLMs [82]. While this enables scalable, transparent, and automated governance, it also raises concerns around model reliability and accountability. 

The integration of AI agents in blockchain governance remains a nascent yet rapidly evolving field. As current initiatives demonstrate, from single-agent systems to modular and self-sovereign agent architectures, the potential for enhanced autonomy, scalability, and decision-making is significant. However, these implementations often raise concerns around centralization, model reliability, and the robustness of deliberative mechanisms. Moreover, several of the proposed architectures remain largely theoretical, with limited practical deployment or empirical validation. This landscape highlights the urgent need for novel approaches that balance automation with diversity and decentralization, especially in the design of autonomous voting systems. 

## **5. Introduction of VOPPA Framework** 

As outlined in the previous section, among the most recent and rapidly emerging directions in blockchain governance is the integration of autonomous AI agents. These agents promise scalable, real-time decision-making and reduced human intervention. However, as highlighted earlier, current implementations often remain limited in scope, theoretical in nature, or prone to issues such as centralization and model reliability and trust. In response to these limitations, we propose a novel multi-agent architecture designed to enable predictive, diverse, and decentralized autonomous voting. 

The architecture we introduce, VOPPA, is a theoretical framework aimed at exploring how a collective of autonomous AI agents might support more resilient and data-driven governance processes in decentralized systems. As illustrated in Figure 17, VOPPA envisions a setting where multiple AI agents operate in parallel, each independently analyzing proposals and casting votes based on predicted impact, rather than token-based preferences or human consensus mechanisms. 

Each agent in the VOPPA system incorporates two core components: an NLP module (e.g., SBERT) that encodes the textual content of a proposal into a semantic vector, and an ML predictor (e.g., Random Forest Regressor) that estimates the proposal’s potential impact based on patterns learned from historical data. Unlike models designed to mimic human 

_Computers_ **2025** , _14_ , 425 

18 of 25 

voting, VOPPA are trained to predict quantifiable outcomes, such as protocol usage metrics, financial performance, or community engagement, following proposal implementation. 

**Figure 17.** The conceptual architecture of the VOPPA framework. 

While LLMs could, in theory, perform both semantic understanding and impact prediction in a unified manner, VOPPA deliberately separates these components for greater transparency, modularity, control, and resource efficiency. Using a distinct NLP encoder and a supervised ML regressor enables clearer interpretability of each stage, facilitates targeted retraining, and reduces the risk of opaque, unexplainable outputs. Furthermore, LLMs often require significant computational resources and raise concerns related to bias propagation and uncontrollable behavior, factors that are particularly problematic in highstakes governance contexts [83,84]. 

To promote diversity and reduce systemic bias, the system relies on a group of agents, ideally an odd number (5, 7, 9, or more). Higher agent counts generally increase system robustness and the diversity of perspectives. The choice of an odd number ensures that ties are avoided in majority voting. The optimal number of agents, however, depends on a balance between diversity and computational efficiency. A small number of agents (e.g., 3) may limit robustness and expose the system to bias, while a very large number increases overhead and slows decision-making. In practice, the number of agents would need to be calibrated through simulation experiments on historical data, evaluating predictive accuracy, resilience, and cost-performance trade-offs. 

Each agent may use different model parameters or decision thresholds, and may also be trained on datasets originating from different sources. This parallelism is essential to 

_Computers_ **2025** , _14_ , 425 

19 of 25 

ensuring robustness: each agent processes the proposal independently and casts a binary vote (“yes” if the predicted impact exceeds a defined threshold, such as 0.6, for example). To strengthen redundancy and diversity, VOPPA assumes that agents are intentionally designed to differ in their training data, model configurations, and evaluation strategies. Such heterogeneity reduces the likelihood of correlated errors and helps preserve decentralization by ensuring that no single perspective dominates the collective outcome. 

In the case of proposals with multiple options, VOPPA applies a majority-elimination strategy: If no single option achieves a majority in the first round, the option(s) with the lowest support are removed, and voting is repeated among the remaining choices. This iterative process continues until one outcome receives a clear majority, avoiding deadlock while preserving agent autonomy. 

The aggregation of individual votes can be managed through smart contracts, as it is represented in Figure 17, or delegated to an off-chain governance layer, depending on the system’s architecture and trust model. The framework remains agnostic to implementation specifics, allowing integration either as a fully autonomous governance module or as a decision support mechanism. 

The full content of each governance proposal, including its title, detailed description, and associated documentation, is expected to be stored off-chain, using distributed or centralized storage systems. This approach avoids overloading on-chain resources while ensuring that all agents have access to the complete proposal text for analysis. 

To make the framework more accessible, VOPPA can also be understood as a simple four-step cycle: proposals are first submitted; each predictive agent independently analyzes the proposal; agents cast individual votes; the results are aggregated into a collective decision. This high-level workflow is illustrated in Figure 18, providing a simplified view that complements the more detailed conceptual architecture. 

**Figure 18.** Simplified workflow of the VOPPA framework. 

While VOPPA is presented here as a theoretical construct, it is intended as a foundation for future applied research. Upcoming work will explore its implementation and evaluate its feasibility and performance in governance environments. At first glance, the framework 

_Computers_ **2025** , _14_ , 425 

20 of 25 

offers promising advantages, such as modularity, agent diversity, predictive reasoning, and resistance to centralized capture. However, several limitations are also apparent: the accuracy of impact prediction depends heavily on data quality; managing large agent ensembles may introduce additional system overhead; ethical concerns persist regarding transparency, value alignment, and accountability in machine-driven governance; and autonomous agents may also be targeted directly, through private key compromise or model tampering, enabling malicious actors to capture their votes. These trade-offs will be a central focus in the next phase of this research. 

## **6. Discussion and Future Directions** 

The findings of this study highlight the growing complexity and fragility of blockchain governance as decentralized systems expand in scale, diversity, and societal impact. Current governance models (on-chain, off-chain, and hybrid) exhibit unique strengths but also expose significant vulnerabilities, particularly in relation to participation equity, manipulation resistance, and adaptability during crises. Case studies such as the DAO attack, the Build Finance DAO takeover, and the Beanstalk exploit illustrate how technical finality, governance capture, and insufficient safeguards can lead to systemic breakdowns, even in well-intentioned decentralized frameworks. 

At the same time, the integration of AI into blockchain governance emerges as both a promising and contentious evolution. AI-assisted systems offer tools for enhancing proposal comprehension, detecting manipulation, and enabling scalable autonomous decisionmaking. However, their deployment raises essential concerns around accountability and the potential for centralization through opaque algorithmic agents. 

The VOPPA framework proposed in this article addresses some of these challenges by advancing a multi-agent architecture for decentralized, predictive governance. Unlike token-based or preference-driven voting, VOPPA introduces outcome-based reasoning by training predictive agents to assess the likely impact of governance proposals. This shifts the focus from subjective consensus to data-informed deliberation. Moreover, the use of multiple agents with diverse training pathways promotes decentralization, diversity, robustness, mitigates single-point failures, and increases resistance to manipulation. 

Although VOPPA introduces an innovative approach to blockchain governance through predictive, multi-agent decision-making, it remains a theoretical construct. Its practical deployment presents significant challenges, particularly in ensuring agent diversity and aligning their behavior with community values. The effectiveness of such systems critically depends on the quality, fairness, and representativeness of training data, highlighting the need for robust pipelines for data collection, curation, and ongoing validation. In addition to these aspects, scalability and real-world deployment represent major challenges. Training and running multiple predictive agents requires computational resources, and integrating their outputs into blockchain environments raises concerns regarding latency and transaction throughput. 

Ethical risks also require careful attention. Beyond issues of transparency and accountability, there are broader concerns related to fairness, bias, and the potential displacement of human judgment by automated processes. Addressing these risks requires careful attention to how such systems are designed and monitored, as well as ensuring that appropriate safeguards and human oversight remain in place. 

Contesting an AI-driven outcome would require transparent logging of agent decisions and the ability for human stakeholders to trigger appeals or override mechanisms, ensuring that final accountability remains with the community rather than the autonomous agents. 

Further practical considerations for VOPPA relate to the choice of training data, evaluation criteria, and computational costs. Suitable data sources may include governance 

_Computers_ **2025** , _14_ , 425 

21 of 25 

proposals, forum discussions, and historical voting records, all of which provide context for training predictive agents. Besides predictive accuracy, evaluation can also consider diversity, resilience, and fairness in the collective results. Finally, computational complexity and resource requirements must be carefully balanced against scalability, as deploying large ensembles of agents may become prohibitive without efficient optimization. 

Future research must therefore focus on translating this conceptual framework into trustworthy, scalable implementations that preserve decentralization while enhancing decision-making intelligence, including prototype development and simulation-based experiments on historical datasets to provide empirical validation. 

## **7. Conclusions** 

This article has explored the evolving landscape of blockchain governance, highlighting the limitations of current models and illustrating their vulnerabilities through critical case studies. While decentralization remains a core principle, effective governance requires more than distributed authority; it demands robust coordination, accountability, and adaptability to emerging threats and complexities. 

AI, particularly in the form of autonomous agents and predictive models, holds significant potential to enhance blockchain governance by enabling scalable, data-informed, and resilient decision-making. However, as demonstrated through the proposed VOPPA framework, the integration of AI must be approached with caution, ensuring alignment with community values, transparency, and systemic fairness. 

As blockchain ecosystems mature, the ability to govern effectively and securely will become a decisive factor for long-term viability. AI-driven frameworks like VOPPA mark an important step toward rethinking how decisions can be made in decentralized contexts, not by replacing human agency, but by reinforcing it through structured and data-informed processes. Advancing this paradigm requires continued research, cross-disciplinary collaboration, and a commitment to building governance tools that are not only technically robust but also aligned with community values and practical realities. 

**Author Contributions:** Conceptualization, C.D.M.; methodology, C.D.M.; formal analysis, C.D.M.; investigation, C.D.M.; writing—original draft preparation, C.D.M.; writing—review and editing, C.D.M., D.E.P., O.C.N. and D.G.; visualization, C.D.M.; supervision, D.E.P. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This research was funded by University of Oradea. 

**Data Availability Statement:** Not applicable. 

**Conflicts of Interest:** The authors declare no conflicts of interest. 

## **Abbreviations** 

The following abbreviations are used in this manuscript: 

|ACP|Artifcial Systems, Computational Experiments, and Parallel Execution|
|---|---|
|AI|Artifcial Intelligence|
|BCH|Bitcoin Cash|
|DAO|Decentralized Autonomous Organization|
|dApp|Decentralized Application|
|EIP|Ethereum Improvement Proposal|
|GCNN|Graph Convolutional Neural Network|
|IoT|Internet of Things|
|LLM|Large Language Model|
|LP|Liquidity Provider|



_Computers_ **2025** , _14_ , 425 

22 of 25 

ML Machine Learning NLP Natural Language Processing VOPPA Voting Via Parallel Predictive Agents 

## **References** 

1. Sebestyen, H.; Popescu, D.E.; Zmaranda, R.D. A Literature Review on Security in the Internet of Things: Identifying and Analysing Critical Categories. _Computers_ **2025** , _14_ , 61. [CrossRef] 

2. Silaghi, D.L.; Popescu, D.E. A Systematic Review of Blockchain-Based Initiatives in Comparison to Best Practices Used in Higher Education Institutions. _Computers_ **2025** , _14_ , 141. [CrossRef] 

3. Morar, C.D.; Popescu, D.E. A Survey of Blockchain Applicability, Challenges, and Key Threats. _Computers_ **2024** , _13_ , 223. [CrossRef] 4. Vavilis, S.; Niavis, H.; García-Mauriño, L.R. On Addressing Governance Challenges in Blockchain Networks. In Proceedings of the 2024 6th Conference on Blockchain Research & Applications for Innovative Networks and Services (BRAINS), Berlin, Germany, 9–11 October 2024; pp. 1–4. [CrossRef] 

5. Schädler, L.; Lustenberger, M.; Spychiger, F. Analyzing decision-making in blockchain governance. _Front. Blockchain_ **2023** , _6_ , 1256651. [CrossRef] 

6. Calzada, I. Decentralized Web3 Reshaping Internet Governance: Towards the Emergence of New Forms of Nation-Statehood? _Future Internet_ **2024** , _16_ , 361. [CrossRef] 

7. Nechesov, A.; Ruponen, J. Empowering Government Efficiency Through Civic Intelligence: Merging Artificial Intelligence and Blockchain for Smart Citizen Proposals. _Technologies_ **2024** , _12_ , 271. [CrossRef] 

8. Liu, L.; Zhou, S.; Huang, H.; Zheng, Z. From Technology to Society: An Overview of Blockchain-Based DAO. _IEEE Open J. Comput. Soc._ **2021** , _2_ , 204–215. [CrossRef] 

9. Wang, Q.; Yu, G.; Sai, Y.; Sun, C.; Nguyen, L.D.; Xu, X.; Chen, S. A First Look into Blockchain DAOs. In Proceedings of the 2023 IEEE International Conference on Blockchain and Cryptocurrency (ICBC), Dubai, United Arab Emirates, 1–5 May 2023; pp. 1–3. [CrossRef] 

10. Duijn, B.H.; Roca, J.B.; Romme, A.G.L.; Weggeman, M. The Seven Capital Sins in the Governance of Blockchain Ecosystems. _IEEE Eng. Manag. Rev._ **2023** , _51_ , 13–17. [CrossRef] 

11. Kharman, A.M.; Smyth, B. DAO Governance Protocols and their Vulnerabilities. In Proceedings of the 2024 6th Conference on Blockchain Research & Applications for Innovative Networks and Services (BRAINS), Berlin, Germany, 9–11 October 2024; pp. 1–2. [CrossRef] 

12. Koning, H.; Linden, T.V. Decentralized Governance inspired by Corporate Governance? In Proceedings of the 2022 IEEE 24th Conference on Business Informatics (CBI), Amsterdam, The Netherlands, 15–17 June 2022; Volume 2, pp. 104–110. [CrossRef] 

13. _IEEE Std 2145-2023_ ; IEEE Trial-Use Recommended Practice for Framework and Definitions for Blockchain Governance. IEEE: New York, NY, USA, 2024. [CrossRef] 

14. Alibaši´c, H. A Multi-Paradigm Ethical Framework for Hybrid Intelligence in Blockchain Technology and Cryptocurrency Systems Governance. _FinTech_ **2025** , _4_ , 34. [CrossRef] 

15. Balcerzak, A.P.; Nica, E.; Rogalska, E.; Poliak, M.; Klieštik, T.; Sabie, O.-M. Blockchain Technology and Smart Contracts in Decentralized Governance Systems. _Adm. Sci._ **2022** , _12_ , 96. [CrossRef] 

16. Dolhopolov, A.; Castelltort, A.; Laurent, A. Implementing Federated Governance in Data Mesh Architecture. _Future Internet_ **2024** , _16_ , 115. [CrossRef] 

17. Ullah, I.; Havinga, P.J.M. Governance of a Blockchain-Enabled IoT Ecosystem: A Variable Geometry Approach. _Sensors_ **2023** , _23_ , 9031. [CrossRef] [PubMed] 

18. Fischer, A.; Valiente, M.C. Blockchain governance. _Internet Policy Rev._ **2021** , _10_ , 1–10. [CrossRef] 19. Chen, X.; He, S.; Sun, L.; Zheng, Y.; Wu, C.Q. A Survey of Consortium Blockchain and Its Applications. _Cryptography_ **2024** , _8_ , 12. [CrossRef] 

20. Joannou, D.; Kalawsky, R.; Martínez-García, M.; Fowler, C.; Fowler, K. Realizing the Role of Permissioned Blockchains in a Systems Engineering Lifecycle. _Systems_ **2020** , _8_ , 41. [CrossRef] 

21. Reijers, W.; Wuisman, I.; Mannan, M.; Filippi, P. Now the Code Runs Itself: On-Chain and Off-Chain Governance of Blockchain Technologies. _SSRN Electron. J._ **2018** , _40_ , 821–831. [CrossRef] 

22. Kiayias, A.; Lazos, P. SoK: Blockchain Governance. In Proceedings of the 4th ACM Conference on Advances in Financial Technologies, Cambridge, MA, USA, 19–21 September 2022; Association for Computing Machinery: New York, NY, USA, 2023; pp. 61–73. [CrossRef] 

23. Pelt, R.; Jansen, S.; Baars, D.; Overbeek, S. Defining Blockchain Governance: A Framework for Analysis and Comparison. _Inf. Syst. Manag._ **2021** , _38_ , 21–41. [CrossRef] 

24. De Filippi, P.; Wright, A. _Blockchain and the Law: The Rule of Code_ ; Harvard University Press: Cambridge, MA, USA, 2018. [CrossRef] 

_Computers_ **2025** , _14_ , 425 

23 of 25 

25. Oyinloye, D.P.; The, J.S.; Jamil, N.; Alawida, M. Blockchain Consensus: An Overview of Alternative Protocols. _Symmetry_ **2021** , _13_ , 1363. [CrossRef] 

26. _Fully on-Chain DAOs on the Internet Computer_ ; Crypto Valley: Zug, Switzerland, 2023. Available online: https://internetcomputer. org/whitepapers/Fully%20on-chain%20DAOs%20on%20the%20Internet%20Computer.pdf (accessed on 18 June 2025). 

27. van Vulpen, P.; Heijnen, H.; Mens, S.; Kroon, T.; Jansen, S. Upgradeable diamond smart contracts in decentralized autonomous organizations. _Front. Blockchain_ **2024** , _7_ , 1481914. [CrossRef] 

28. Li, J.; Liang, X.; Qin, R.; Wang, F.-Y. From DAO to TAO: Finding the Essence of Decentralization. In Proceedings of the 2023 IEEE International Conference on Systems, Man, and Cybernetics (SMC), Oahu, HI, USA, 1–4 October 2023; pp. 4283–4288. [CrossRef] 

29. Morar, C.-D.; Popescu, D.E.; Novac, O.C. Understanding Blockchain Governance: Key Models and Issues. In Proceedings of the 2025 18th International Conference on Engineering of Modern Electric Systems (EMES), Oradea, Romania, 29–30 May 2025; pp. 1–4. [CrossRef] 

30. Blockchain Governance Models Compared: On-Chain vs Off-Chain Decision Making. Available online: https://www.rapidinnovation. io/post/blockchain-governance-models-compared-on-chain-vs-off-chain-decision-making (accessed on 20 June 2025). 

31. Tan, E.; Mahula, S.; Crompvoets, J. Blockchain governance in the public sector: A conceptual framework for public management. _Gov. Inf. Q._ **2022** , _39_ , 101625. [CrossRef] 

32. On-Chain Governance: Definition, Types, vs. Off-Chain. Available online: https://www.investopedia.com/terms/o/onchaingovernance.asp (accessed on 20 June 2025). 

33. Laatikainen, G.; Li, M.; Abrahamsson, P. A system-based view of blockchain governance. _Inf. Softw. Technol._ **2023** , _157_ , 107149. [CrossRef] 

34. Governance and Validation on Tezos. Available online: https://www.tezosagora.org/learn (accessed on 22 June 2025). 

35. Nelaturu, K.; Du, H.; Le, D.-P. A Review of Blockchain in Fintech: Taxonomy, Challenges, and Future Directions. _Cryptography_ **2022** , _6_ , 18. [CrossRef] 

36. Blockchain Governance in the Wild. Available online: https://cryptoeconomicsystems.pubpub.org/pub/blockchain-governancewild/release/1 (accessed on 23 June 2025). 

37. Introduction to Ethereum Governance. Available online: https://ethereum.org/en/governance (accessed on 23 June 2025). 

38. Filippi, P.; Mcmullen, G. _Governance of Blockchain Systems: Governance of and by Distributed Infrastructure_ ; Blockchain Research Institute: Toronto, ON, Canada, 2018. 

39. Mehar, M.I.; Shier, C.; Giambattista, A.; Gong, E.; Fletcher, G.; Sanayhie, R.; Kim, H.M.; Laskowski, M. Understanding a Revolutionary and Flawed Grand Experiment in Blockchain: The DAO Attack. _J. Cases Inf. Technol._ **2019** , _21_ , 19–32. [CrossRef] 

40. What Was the DAO? Available online: https://www.gemini.com/cryptopedia/the-dao-hack-makerdao (accessed on 25 June 2025). 41. Dhillon, V.; Metcalf, D.; Hooper, M. The DAO Hacked. In _Blockchain Enabled Applications_ ; Apress: Berkeley, CA, USA, 2017; pp. 67–78. [CrossRef] 

42. Liu, Y.; Lu, Q.; Paik, H.-Y.; Yu, G.; Zhu, L. Decision Models for Selecting Patterns in Governance-driven Blockchain Systems. In Proceedings of the 2023 IEEE International Conference on Blockchain (Blockchain), Danzhou, China, 17–21 December 2023; pp. 307–314. [CrossRef] 

43. Build Finance DAO Suffers Governance Takeover Attack. Available online: https://cryptobriefing.com/build-finance-daosuffers-governance-takeover-attack/ (accessed on 26 June 2025). 

44. Build Finance DAO Falls to Governance Takeover. Available online: https://decrypt.co/92970/build-finance-dao-falls-togovernance-takeover (accessed on 26 June 2025). 

45. Build Finance DAO Suffers Hostile Governance Takeover, Loses $470,000. Available online: https://www.theblock.co/post/1341 80/build-finance-dao-suffers-hostile-governance-takeover-loses-470000 (accessed on 26 June 2025). 

46. What is Bitcoin Cash, and How Does BCH Work? Available online: https://cointelegraph.com/learn/articles/what-is-bitcoincash-and-how-does-bch-work-a-beginners-guide (accessed on 27 June 2025). 

47. Hu, B.; Miller, J. Are Cryptocurrency Forks Wealth Creating? _J. Risk Financ. Manag._ **2023** , _16_ , 510. [CrossRef] 

48. What Is Bitcoin Cash (BCH) and How Does It Work? Available online: https://www.ccn.com/education/crypto/what-is-bitcoincash-bch-the-first-major-fork-of-bitcoin-explained/ (accessed on 27 June 2025). 

49. Key Takeaways from the Golden Boys’ Attack on Compound DAO. Available online: https://cointelegraph.com/news/keytakeaways-golden-boys-attack-compound-dao (accessed on 28 June 2025). 

50. Compound Governance Attack Reveals Inherent Vulnerabilities of DAOs. Available online: https://thedefiant.io/news/defi/ compound-governance-attack-reveals-inherent-vulnerabilities-of-daos (accessed on 28 June 2025). 

51. The Compound Finance Governance Attack: A Recap and Its Implications. Available online: https://research.despread.io/ compound-finance-governance-attack/ (accessed on 28 June 2025). 

52. Beanstalk Hack Analysis & POC. Available online: https://blockapex.io/beanstalk-hack-analysis-poc/ (accessed on 30 June 2025). 53. Beanstalk Cryptocurrency Project Robbed After Hacker Votes to Send Themself $182 Million. Available online: https://www. theverge.com/2022/4/18/23030754/beanstalk-cryptocurrency-hack-182-million-dao-voting (accessed on 30 June 2025). 

_Computers_ **2025** , _14_ , 425 

24 of 25 

54. Hack Analysis: Beanstalk Governance Attack, April 2022. Available online: https://medium.com/immunefi/hack-analysisbeanstalk-governance-attack-april-2022-f42788fc821e (accessed on 30 June 2025). 

55. Esteva, A.; Robicquet, A.; Ramsundar, B.; Kuleshov, V.; DePristo, M.; Chou, K.; Cui, C.; Corrado, G.; Thrun, S.; Dean, J. A guide to deep learning in healthcare. _Nat. Med._ **2019** , _25_ , 24–29. [CrossRef] 

56. Jordan, M.; Mitchell, T.M. Machine Learning: Trends, Perspectives, and Prospects. _Science_ **2015** , _349_ , 255–260. [CrossRef] 

57. Ghiurău, D.; Popescu, D.E. Distinguishing Reality from AI: Approaches for Detecting Synthetic Content. _Computers_ **2025** , _14_ , 1. [CrossRef] 

58. Shah, H.; Shah, D.; Jadav, N.K.; Gupta, R.; Tanwar, S.; Alfarraj, O.; Tolba, A.; Raboaca, M.S.; Marina, V. Deep Learning-Based Malicious Smart Contract and Intrusion Detection System for IoT Environment. _Mathematics_ **2023** , _11_ , 418. [CrossRef] 

59. Rane, N.; Choudhary, S.; Rane, J. Blockchain and Artificial Intelligence (AI) integration for revolutionizing security and transparency in finance. _SSRN Electron. J._ **2023** . [CrossRef] 

60. Dhanushkodi, K.; Thejas, S. AI Enabled Threat Detection: Leveraging Artificial Intelligence for Advanced Security and Cyber Threat Mitigation. _IEEE Access_ **2024** , _12_ , 173127–173136. [CrossRef] 

61. Venkatesan, K.; Rahayu, S. Blockchain security enhancement: An approach towards hybrid consensus algorithms and machine learning techniques. _Sci. Rep._ **2024** , _14_ , 1149. [CrossRef] 

62. Yadlapalli, A.; Mohite, N.; Pawar, V.; Sachdeva, S. Artificially Intelligent Decentralized Autonomous Organization. In Proceedings of the 2019 4th International Conference on Information Systems and Computer Networks (ISCON), Mathura, India, 21–22 November 2019; pp. 667–671. [CrossRef] 

63. Chen, J.; Hsu, C.; Tsai, Y. Intelligent Decentralized Governance: A Case Study of KlimaDAO Decision-Making. _Electronics_ **2025** , _14_ , 2562. [CrossRef] 

64. Quan, Y.; Wu, X.; Deng, W.; Zhang, L. Decoding Social Sentiment in DAO: A Comparative Analysis of Blockchain Governance Communities. In Proceedings of the 2024 IEEE 24th International Conference on Software Quality, Reliability, and Security Companion (QRS-C), Cambridge, UK, 1–5 July 2024; pp. 216–224. [CrossRef] 

65. Ao, L.; Liu, H.; Zhang, H. AgentDAO: Synthesis of Proposal Transactions Via Abstract DAO Semantics. _arXiv_ **2025** , arXiv:2503.10099. [CrossRef] 

66. Chahar, S.; Kaur, K.; Kaswan, K.S.; Dhatterwal, J.S. Explainable AI in Blockchain System for Decentralized Governance. In Proceedings of the 2025 International Conference on Pervasive Computational Technologies (ICPCT), Greater Noida, India, 8–9 February 2025; pp. 725–729. [CrossRef] 

67. Ikeda, Y.; Hadfi, R.; Ito, T.; Fujihara, A. Anomaly detection and facilitation AI to empower decentralized autonomous organizations for secure crypto-asset transactions. _AI Soc._ **2025** , _40_ , 3999–4010. [CrossRef] 

68. DuPont, Q. New Online Communities: Graph Deep Learning on Anonymous Voting Networks to Identify Sybils in Polycentric Governance. _arXiv_ **2024** , arXiv:2311.17929. [CrossRef] 

69. Ranjan, R.; Gupta, S.; Singh, S. LOKA Protocol: A Decentralized Framework for Trustworthy and Ethical AI Agent Ecosystems. _arXiv_ **2025** , arXiv:2504.10915. [CrossRef] 

70. Chaffer, T.J.; Goins, C.; Okusanya, B.; Cotlage, D.; Goldston, J. Decentralized Governance of Autonomous AI Agents. _arXiv_ **2025** , arXiv:2412.17114. 

71. Gürpinar, T. Towards web 4.0: Frameworks for autonomous AI agents and decentralized enterprise coordination. _Front. Blockchain_ **2025** , _8_ , 1591907. [CrossRef] 

72. Udokwu, C.; Voicu-Dorobant,u, R.; Ogunyemi, A.A.; Norta, A.; Sturua, N.; Craß, S. Leveraging Blockchain for Ethical AI: Mitigating Digital Threats and Strengthening Societal Resilience. _Future Internet_ **2025** , _17_ , 309. [CrossRef] 

73. Karim, M.M.; Van, D.H.; Khan, S.; Qu, Q.; Kholodov, Y. AI Agents Meet Blockchain: A Survey on Secure and Scalable Collaboration for Multi-Agents. _Future Internet_ **2025** , _17_ , 57. [CrossRef] 

74. Ly, R.; Shojaei, A. Autonomous Building Cyber-Physical Systems Using Decentralized Autonomous Organizations, Digital Twins, and Large Language Model. _arXiv_ **2024** , arXiv:2410.19262. [CrossRef] 

75. Introducing GoverNoun: A DAO Governance Agent and Politician. Available online: https://medium.com/@amm16/ introducing-governoun-a-dao-governance-agent-and-politician-34bb348ba263 (accessed on 2 July 2025). 

76. Yu, E.; Yue, W.; Jianzheng, S.; Xun, W. Blockchain-based AI Agent and Autonomous World Infrastructure. In Proceedings of the 2024 IEEE Conference on Artificial Intelligence (CAI), Singapore, 25–27 June 2024; pp. 278–283. [CrossRef] 

77. Ailve DAO. Available online: https://ethglobal.com/showcase/ailve-dao-9bi3b (accessed on 3 July 2025). 

78. Ding, W.; Liang, X.; Hou, J.; Li, J.; Rouabah, Y.; Yuan, Y.; Wang, F. A Novel Approach for Predictable Governance of Decentralized Autonomous Organizations Based on Parallel Intelligence. _IEEE Trans. Syst. Man. Cybern. Syst._ **2023** , _53_ , 3092–3103. [CrossRef] 

79. Liang, X.; Ding, W.; Qin, R.; Hou, J.; Yuan, Y.; Wang, X.; Wang, F. From cadCAD to casCAD2: A Mechanism Validation and Verification System for Decentralized Autonomous Organizations Based on Parallel Intelligence. _IEEE Trans. Comput. Soc. Syst._ **2024** , _11_ , 2853–2862. [CrossRef] 

_Computers_ **2025** , _14_ , 425 

25 of 25 

80. Patterns for Democratic Multi-Agent AI: Voting-Based Council—Part 1. Available online: https://medium.com/@edoardo. schepis/patterns-for-democratic-multi-agent-ai-voting-based-council-part-1-9a9164a173ff (accessed on 3 July 2025). 

81. Hu, B.; Liu, Y.; Rong, H. Trustless Autonomy: Understanding Motivations, Benefits and Governance Dilemma in Self-Sovereign Decentralized AI Agents. _arXiv_ **2025** , arXiv:2505.09757. [CrossRef] 

82. Asap. Available online: https://ethglobal.com/showcase/asap-1o86x (accessed on 3 July 2025). 

83. Calderon, N.; Reichart, R. On Behalf of the Stakeholders: Trends in NLP Model Interpretability in the Era of LLMs. In Proceedings of the 2025 Conference of the Nations of the Americas Chapter of the Association for Computational Linguistics: Human Language Technologies, Albuquerque, NM, USA, 29 April–4 May 2025; Association for Computational Linguistics: Stroudsburg, PA, USA, 2025; pp. 656–693. [CrossRef] 

84. Raza, M.; Jahangir, Z.; Riaz, M.B.; Saeed, M.J.; Sattar, M.A. Industrial applications of large language models. _Sci. Rep._ **2025** , _15_ , 13755. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

