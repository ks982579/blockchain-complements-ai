@ _**β**_ BetaWeb 

# @ _**β**_ **BetaWeb: Towards a Blockchain-enabled Trustworthy Agentic Web** 

**Zihan Guo**[1] _[,]_[2] **, Yuanjian Zhou**[1] **, Chenyi Wang**[1] _[,]_[3] **, Linlin You**[2] **, Minjie Bian**[4] **, Weinan Zhang**[1] _[,]_[5] _[∗]_ 

> 1 Shanghai Innovation Institute 2 Sun Yat-sen University 

> 3 Zhejiang University 4 Shanghai Data Group Co., Ltd 

> 5 Shanghai Jiao Tong University `guozh29@mail2.sysu.edu.cn, wnzhang@sjtu.edu.cn` 

## **Abstract** 

The rapid development of large language models (LLMs) has significantly propelled the development of artificial intelligence (AI) agents, which are increasingly evolving into diverse autonomous entities, advancing the LLM-based multi-agent systems (LaMAS). However, current agentic ecosystems remain fragmented and closed. Establishing an interconnected and scalable paradigm for Agentic AI has become a critical prerequisite. Although Agentic Web proposes an open architecture to break the ecosystem barriers, its implementation still faces core challenges such as privacy protection, data management, and value measurement. Existing centralized or semi-centralized paradigms suffer from inherent limitations, making them inadequate for supporting large-scale, heterogeneous, and cross-domain autonomous interactions. To address these challenges, this paper introduces the blockchainenabled trustworthy Agentic Web (BetaWeb). By leveraging the inherent strengths of blockchain, BetaWeb not only offers a trustworthy and scalable infrastructure for LaMAS but also has the potential to advance the Web paradigm from Web3 (centered on data ownership) towards Web3.5, which emphasizes ownership of agent capabilities and the monetization of intelligence. Beyond a systematic examination of the BetaWeb framework, this paper presents a five-stage evolutionary roadmap, outlining the path of LaMAS from passive execution to advanced collaboration and autonomous governance. We also conduct a comparative analysis of existing products and discuss key challenges of BetaWeb from multiple perspectives. Ultimately, we argue that deep integration between blockchain and LaMAS can lay the foundation for a resilient, trustworthy, and sustainably incentivized digital ecosystem. A summary of the enabling technologies for each stage is available at `https://github.com/MatZaharia/BetaWeb` . 

**Keywords:** BetaWeb, Agentic Web, Blockchain, Trustworthy AI 

## **1 Introduction** 

With the rapid and transformative advances in large language models (LLM), artificial intelligence (AI) agents are evolving from early rule-based and passively executed units into autonomous entities endowed with end-to-end capabilities encompassing perception, learning, decision-making, and execution (Wang et al., 2024a; Guo et al., 2024). The system architectures for agents are also shifting from single-node deployments to distributed and multi-agent collaborative frameworks (Gao et al., 2025). Through shared perception, task planning, and interactive cooperation among multiple agents, these systems can accomplish increasingly complex and dynamic objectives. It is an 

1 

> _**β**_ BetaWeb 

evolution that profoundly reshapes the design principles and application paradigms, driving the advancement towards LLM-based multi-agent systems (LaMAS) (Yang et al., 2024). 

Although current LaMAS demonstrate remarkable performance in controlled environments, their potential remains constrained by a siloed reality (Petrova et al., 2025). As shown in Figure 1a, the existing ecosystems of Agentic AI (Sapkota et al., 2025) are still predominantly platform-centric under various giants, such as Google Agentspace (Google Cloud) and AWS Marketplace (Services), where users and the growing agents operate largely within the walled gardens controlled by centralized entities (Scheuerman, 2024). Such a form of digital feudalism concentrates data ownership and interaction protocols in the hands of platform gatekeepers, thereby limiting system transparency and user rights, including practices like big data price discrimination (Pandey & Caliskan, 2021), algorithmic content manipulation (Ienca, 2023), and opaque user data sharing policies (Bietti, 2025). Meanwhile, individual capabilities in the traditional Internet have been constrained by limited operational tools and fragmented interaction interfaces, which are now being lifted by unleashing unprecedented potential for large-scale autonomy and collaboration with agents, fueling an urgent demand for open and interoperable digital environments (Wang et al., 2025a). 

Especially as autonomous agents increasingly collaborate across organizational boundaries, heterogeneous platforms, and diverse application scenarios, the construction of an open, interoperable, and scalable coordination layer has become a critical task. This vision strongly aligns with the emerging concept of Agentic Web[1] (Yang et al., 2025b), which is a fully-connected architecture driven by agent participation as illustrated in Figure 1b. Its goal is to overcome the collaboration barriers of closed ecosystems, enabling a new paradigm of the machine-to-machine economy. Agentic Web also support large-scale knowledge integration and adaptive cross-domain problem-solving, and facilitate social behaviors as well as co-creation of value across organizations and platforms. 

The transition to the Agentic Web not only represents a technological evolution but also signifies an inevitable transformation of the digital ecosystem. Despite the strong appeal of this vision, its realization faces a series of urgent challenges that current digital infrastructures struggle to address. As agents increasingly transcend single-platform boundaries to operate across domains, critical issues such as trust establishment, verifiable identities, transparent interaction records, and secure coordination become ever more prominent (Raza et al., 2025; Sharma et al., 2025; Yin et al., 2024). Without a reliable and trustworthy foundational architecture, large-scale agent collaboration risks being hindered by fragmented standards, opaque governance, and susceptibility to manipulation, potentially resulting in uncontrollable outcomes (Deng et al., 2025; Puthal et al., 2025; Sharma et al., 2025). Specifically, the key challenges can be summarized into the following three areas: 

1. Privacy Protection and Risk Control: Openness brings collaboration opportunities but also amplifies challenges related to privacy breaches, unclear legal responsibilities, and risk management. Given that agents operate at higher speeds and scales compared to human users, the impact of errors or malicious actions may be exponentially magnified. Therefore, it is essential to ensure agent controllability and achieve traceability and immutability of their behaviors (Narajala & Narayan, 2025; Krishnan, 2025). 

> 2. Complexity of Coordination and Data Management: Under the prospect of agents infinitely scaling and working continuously, the volume and frequency of interactions will far exceed the current level of information flow on the Internet, substantially increasing the complexity of system coordination, data consistency, and resource scheduling. This necessitates the 

> 1Agentic AI, Agentic Web, and LaMAS are often easily confused. Conceptually, **Agentic AI** represents a foundational architectural innovation and a leap forward in intelligent organizational forms, reflecting a new understanding of agent behavior and their interactions, with an emphasis on autonomy and collaboration. **Agentic Web** , on the other hand, is a network architecture designed for agents, aimed at supporting them in accomplishing tasks and goals through communication, collaboration, and resource sharing. In contrast, **LaMAS** are systematic tools that, through specific system-level designs, support the interaction and autonomous behavior of multiple agents in practical applications. **In short, Agentic AI focuses on a paradigm shift at the ecological level, LaMAS provide the specific systems to achieve this shift, and the Agentic Web offers an Internet infrastructure for agent collaboration within or across systems.** 

2 

> _**β**_ BetaWeb ~~Oo~~ 

**==> picture [406 x 132] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Agentic AI under Giants (b) Agentic Web (c) BetaWeb<br>e®) |er| am<br>—— &, © as Bt > ( \eeew<br>Cy. Fully-Connected > Privacy i Blockchain P-S+B-SP-P-OP-S2-SD-G-<br>Walled Controllable Trustworthy<br>Complexity<br>Gardens Efficient<br>Incentive<br>Fair<br>Digital<br>Feudalism<br>**----- End of picture text -----**<br>


Figure 1: Schematic illustration of the evolution from siloed and platform-controlled agentic AI ecosystems to BetaWeb. (a) Current stage dominated by a few major giants, where users and agents are tightly connected within the “walled garden” of each platform, with only weak interplatform links, reflecting digital feudalism concerns. (b) Conceptual open Agentic Web, where users and agents are globally connected, but face challenges such as privacy protection, coordination complexity, and incentive alignment. (c) Vision of BetaWeb, where decentralized infrastructure supports trustworthy, controllable, efficient, and fair interactions among globally connected users and agents. 

construction of trust and coordination mechanisms to ensure synchronization and verifiability in cross-domain agent collaboration (Zhang et al., 2024; Qian et al., 2024; Pan et al., 2024). 

3. Value Measurement and Individual Incentives: With the shift in the digital paradigm, agents will serve not only as data consumers but also as providers of cognitive services and computing resources. There is an urgent need to establish verifiable and auditable contribution assessment mechanisms, upon which fair and transparent incentive and reward distribution can be based, thereby fostering a healthy and sustainable agent ecosystem economy (Chaffer, 2025; Ricci et al., 2024). 

These challenges collectively underscore the necessity of building a trustworthy Agentic Web. While several common paradigms can partially address the needs for trust, coordination, and governance in ecosystem construction, their inherent limitations make them ill-suited for future open, heterogeneous, and cross-domain scenarios. For example, 1) traditional public key infrastructure (Dumitrescu & Pouwelse, 2024) offers high robustness in identity authentication and encrypted communication, yet it was never designed to support universal consensus in heterogeneous networks, nor can it natively enable tamper-proof and multi-party state sharing. 2) Consortium-based (Bai et al., 2021) or federated modes (Ksystra et al., 2022) distribute decision-making power among multiple stakeholders, which can mitigate single-point control risks within a defined scope. However, they still rely on pre-established trust relationships and remain susceptible to collusion and conspiracy, rendering them unsuitable for open, large-scale, cross-domain agent collaboration. 3) Trusted execution environments (Geppert et al., 2022) and other hardware-based trust anchors (Sardar & Fetzer, 2023) provide tamper-resistant computation and isolated execution environments, but their reliance on closed, proprietary ecosystems limits scalability and deprives them of the openness and interoperability required for an Internet-scale, agent-driven economy. 

The limitations indicate that existing centralized or semi-centralized paradigms cannot fully meet the demands of a truly open, interoperable, and high-throughput system, particularly in scenarios where agents autonomously transact, require verifiable transaction records, and must support auditable contribution-based incentive mechanisms (Wang & Chen, 2022; Karim et al., 2025). Intu- 

3 

> _**β**_ BetaWeb 

itively, blockchain technology offers a compelling solution for this purpose. Its native integration of decentralized consensus, immutable record-keeping, and programmable trust logic not only enables verifiable agent identities and traceable interaction histories but also ensures the distributed nature of trust and the inherent resilience of the system, making blockchain an ideal foundation for trustworthy Agentic Web. Specifically, the advantages of blockchain can be directly mapped to the core challenges: 

1. The combination of immutable on-chain records with privacy-preserving cryptographic techniques, such as zero-knowledge proofs, enables interaction traceability while safeguarding participant privacy, significantly reducing legal and operational risks in open environments (Zhou et al., 2024). 

2. Consensus protocols and distributed ledger mechanisms in blockchain can establish a globally consistent and verifiable system state within a heterogeneous and cross-domain Agentic Web, thereby effectively addressing the high complexity of cross-domain collaboration and data synchronization (Bellaj et al., 2024). 

3. Programmable smart contracts provide a native pathway for verifiable value attribution and for automated and transparent incentive distribution. This ensures that the contributions of agents (whether in computation, knowledge provision, or decision support) can be rewarded fairly in an auditable manner (Xu et al., 2024). 

In this paper, we propose the blockchain-enabled trustworthy Agentic Web (BetaWeb, _β_ -Web). In BetaWeb, blockchain not only resolves critical technical challenges but also, when integrated with LaMAS system, lays the groundwork for a resilient, self-sustaining, and incentive-aligned machineto-machine economy. This integration provides a solid basis for future autonomous and scalable digital ecosystems. Beyond traditional Web3, which primarily focuses on data ownership, BetaWeb extends to the realms of intelligent ownership and a capability economy, upgrading the paradigm from “owning and monetizing data” to “owning and monetizing intelligence”, marking the transition towards Web3.5. 

Therefore, it is essential to re-examine LaMAS from the emerging perspective of BetaWeb, especially analyzing how foundational technologies and modular capabilities of blockchain can serve as an enabling underlying infrastructure to support open collaboration and trustworthy operation. Against this backdrop, core components and involved parties should be redefined and redesigned from an agent-centric manner, particularly in trust negotiation, autonomous interaction, and incentive alignment. Although both Agentic AI and blockchain are advancing rapidly and demonstrate clear synergies, there is still a lack of systematic research that maps and critically analyzes their deep integration. Bridging this research gap is essential for understanding and shaping BetaWeb with the evolutionary trajectory of future ecosystems, which is an endeavor that constitutes the central objective of this paper. 

In summary, the main contributions and structure of this paper are as follows. In Section 2, we examine the transformative impacts that BetaWeb may bring when deployed at scale, with a particular focus on their potential to reshape value-exchange mechanisms and relations of production. In Section 3, we present a five-stage evolutionary roadmap towards a fully autonomous and decentralized ecosystem for Agentic AI, outlining its characteristics, key enabling technologies, and major milestones along the way. In Section 4, we analyze existing products that integrate blockchain with LaMAS, situating them within the proposed evolutionary roadmap to identify current gaps and opportunities for advancement. In Section 5, we describe the key challenges and open problems from perspectives of techniques, ecosystems, and society. In Section 6, we conclude the paper by summarizing the main arguments. 

4 

> _**β**_ BetaWeb 

## **2 Blockchain-enabled Trustworthy Agentic Web** 

## **2.1 General Framework** 

Although we have analyzed the paradigm integration and technical feasibility of BetaWeb at a conceptual level, their practical implementation still demands a well-defined and systematic architecture. Specifically, it is necessary to clearly articulate how human users, agents, and the underlying agentic workflows within a decentralized, trustless, and auditable environment. Without a unified framework, the integration process risks becoming fragmented, serving only narrow application scenarios and failing to support a sustainable and scalable ecosystem (Zou et al., 2025). 

To address this issue, we outline a general framework, illustrated in Figure 2, which abstracts all interactions into standardized task procedures, including the request, execution, and feedback. Therefore, the operational logic of LaMAS within BetaWeb is redefined. Regardless of whether an request originates from a human user, an API interface, or another agent, the associated actions will be formally recorded as transactions related to the task in the blockchain. This task can be decomposed into multiple steps, namely sub-tasks, which are autonomously orchestrated and executed by agents (Sun et al., 2025). Notably, the entire lifecycle of all agents and tasks is anchored into the blockchain for recording and governing to enable verifiable and tamper-resistant process control. 

Evidently, by embedding a decentralized consensus infrastructure in BetaWeb, the framework ensures system-level verifiability of agents and tasks, with both identities and interaction processes cryptographically verifiable (Islam et al., 2024). At the same time, all related records are immutable, effectively preventing data tampering, deletion, and other erroneous or malicious operations (Karim et al., 2025). Furthermore, agent behaviors are fully auditable, enabling precise attribution of contributions and responsibilities, thereby facilitating transparent accountability mechanisms and equitable incentive distribution (Sami et al., 2024). 

Overall, this design not only provides a sustainable, scalable, and governable foundational support for building large-scale, high-concurrency, and loosely coupled BetaWeb, but also fosters the emergence of an adaptive and iteratively upgradable ecosystem, laying a solid foundation for a viable pathway towards realizing trustworthy, controllable, efficient, and fair Agentic AI (Sapkota et al., 2025). 

## **2.2 Key Involved Parties** 

The key involved parties in BetaWeb can be categorized into three major dimensions, corresponding to profound transformations at the levels of users, agents, and agentic workflows (Caetano et al., 2025). 

At the user level, there is a fundamental shift in user role. Under traditional paradigms, users are required to directly engage in specific operations and decision-making processes (Xu et al., 2023). In BetaWeb, however, users primarily focus on defining goals and requirements while delegating execution to autonomous agents. This transition liberates users from tedious task execution, enabling an identity shift from operator to the consumer, manager and owner of the agents (Leonardi, 2025). Blockchain ensures that user identities, permissions, and actions are verifiable. This not only significantly enhances user experience and reduces human effort and operational complexity but also facilitates safer and more efficient cognitive interactions and value exchanges. 

At the agent level, agents evolve from simple tools into autonomous entities endowed with perception, decision-making, learning, and execution capabilities (Wang et al., 2024a). Leveraging BetaWeb, agents are able to collaborate across domains and interact dynamically, completing complex tasks involving multiple participants (Han et al., 2024) while continuously acquiring knowledge (Wang et al., 2024b). Under trustworthy and controllable conditions, their autonomy and self-governance are substantially enhanced, enabling proactive responses to environmental and task changes, thereby advancing the overall intelligence of the system and driving it towards a truly autonomous ecosystem for Agentic AI (Hu et al., 2025b). 

5 

~~o~~ _**β**_ BetaWeb 

**==> picture [412 x 376] intentionally omitted <==**

**----- Start of picture text -----**<br>
Task Request Completed Path Pending Path Result Feedback<br>Demand Source Human User Application Interface Agent<br>Agentic Web<br>Done i ) Gp e o eYe e e e e<br>ee Orchestrating a ee ae ~ ~ eee Waiting =<br>Judging<br>Idle<br>Done Running<br>Sub-Task 1 Done Sub-Task N<br>int Result 1 Sia Result N<br>Sub-Task M<br>Result M Attacking<br>Block Header Block Body —— Transactions<br>Version<br>Agent Info Task Info<br>PrevHash<br>Identity Status Publisher Performer<br>ee B 48<br>Timestamp<br>Skills Reputation Steps Results<br>Nonce<br>Blockchain<br>BlockHash<br>**----- End of picture text -----**<br>


Figure 2: General framework of BetaWeb. All interactions are abstracted as task procedures from request to feedback, where the demand source (either a human user, an application interface, or another agent) invokes tasks that may be decomposed into multiple sub-tasks, executed autonomously by agents. The blockchain serves as the trusted substrate for the full lifecycle management of both agents and tasks, ensuring verifiable identity, immutable records, and transparent governance. 

At the agentic workflows level, it achieves high degrees of integration, automation, and systematization. Traditional agentic workflows rely heavily on manual intervention, which are characterized by static, linear, and isolated processes. They are replaced by a dynamic, closed-loop, and adaptive intelligent ecosystem (Yu et al., 2025). Within BetaWeb, agentic workflows can be autonomously completed with self-optimization, enabling effective handling of complex tasks and self-recovery from anomalies (Acharya et al., 2025). The entire workflow remains highly transparent and traceable, supporting automated execution driven by smart contracts as well as dynamic adjustments through autonomous governance, thereby constructing a highly autonomous and elastically scalable environment. 

## **2.3 Core System Modules** 

Beyond general functionalities, the core system modules in BetaWeb can be summarized into two layers. The upper layer consists of business modules responsible for specific system interactions, 

6 

> _**β**_ BetaWeb 

focusing on the full lifecycle management and execution of tasks to ensure transparency and controllability from initiation to completion. The lower support layer ensures overall system management and governance, concentrating on agent management and rule enforcement, thereby guaranteeing the rational economic incentives, effective operation of autonomous mechanisms, and controllability of agents throughout their lifecycle. These two layers complement each other to jointly build a sustainable, secure, and efficient ecosystem for future BetaWeb. 

Within the upper business modules, the first core component is task management. This encompasses the entire lifecycle of tasks, including request parsing, process orchestration, result verification, and incentive settlement (Lin et al., 2024). The immutable ledger and smart contract in blockchain render the task management process highly transparent and trustworthy, ensuring every step is traceable and thereby enhancing the security and effectiveness of task execution. 

The second business module is for task execution, responsible for the actual execution of all tasks or sub-tasks. Beyond the fundamental operation of agents, this module involves container scheduling (Xiao et al., 2024), hybrid on-chain and off-chain data storage (Tsang et al., 2024), invocation of various tools (Masterman et al., 2024), and generation of proofs (Čapko et al., 2022). Task execution relies not only on the coordinated use of distributed computing resources but also leverages blockchain to ensure verifiability and auditability of the execution process, guaranteeing the authenticity and integrity of all outcomes and promoting efficient completion of complex tasks. 

In the lower support modules, the agent management module is responsible for the entire lifecycle of agents, which includes supervision and control of their identity, capability, reputation, and account. By recording agent behaviors immutably, combined with decentralized autonomous organizations (DAO) (Bonnet & Teuteberg, 2024) and dynamic incentive mechanisms (Han et al., 2022), this module significantly enhances agent autonomy and reliability, driving the system towards higher trustworthiness and more efficient collaboration. 

Finally, the rule management module focuses on the design, administration, and on-chain governance of smart contracts, encompassing economic models, reward and penalty mechanisms, and dynamic parameter adjustments (Zutshi et al., 2021; Yuan et al., 2025). This module ensures the transparency, fairness, and adaptability of system rules, supporting multi-stakeholder co-governance and enabling continuous optimization based on real-world operational feedback, thereby maintaining ecosystem stability and sustainable development. 

## **3 Five-stage Evolutionary Roadmap** 

Table 1: Overview of the five-stage evolutionary roadmap. 

|**Stage**|**Name**|**Explanation**|
|---|---|---|
|Stage 1 (S1)|Isolated Silos|Designed LaMAS under human control with siloed governance|
|Stage 2 (S2)|Pilot Decentralization|Decentralized LaMAS still human-led with limited agentic workflows|
|Stage 3 (S3)|Assisted Execution|Agent-assisted LaMAS freeing human labor|
|Stage 4 (S4)|Hybrid Governance|Co-governed LaMAS relieving human cognitive burdens|
|Stage 5 (S5)|Full Autonomy|Autonomous LaMAS with humans setting only the overarching direction|



## **3.1 Overview** 

To systematically depict the evolutionary trajectory of BetaWeb, we propose a five-stage evolutionary roadmap comprising Isolated Silos, Pilot Decentralization, Assisted Execution, Hybrid Governance, and Full Autonomy, as summarized in Table 1. This categorization integrates the characteristics of 

7 

~~Oe~~ _**β**_ BetaWeb 

**==> picture [416 x 305] intentionally omitted <==**

**----- Start of picture text -----**<br>
Isolated Silos Pilot Decentralization Assisted Execution<br>S1 S2 S3<br>4a Bo > @.ao @.aa<br>B- (E OT [\] [OX)]<br>Minimal or Difficult Interaction Human-driven Agent-assisted<br>between Silos  ) Agentic Workflows  G Agentic Workflows<br>Hybrid Governance Full Autonomy<br>S4 S5<br>(eee Intents [ (2eey<br>or<br>Goals<br>Co-governance by Self-governance of<br>humans and agents Fully Autonomous Agents<br>**----- End of picture text -----**<br>


Figure 3: Five-stage evolution diagram of BetaWeb. Stage 1 (Isolated Silos) shows independent systems where humans drive all tasks, and agents are confined within their platforms with minimal outward interaction. Stage 2 (Pilot Decentralization) introduces cross-platform collaboration, but agentic workflows are completed under human supervision. Stage 3 (Assisted Execution) involves agents to undertake specialized duties, reducing the workload on humans. Stage 4 (Hybrid Governance) depicts large-scale distributed collaboration, where agents participate in governance while humans focus on high-value decisions. Stage 5 (Full Autonomy) represents a fully autonomous system where agents operate globally with end-to-end self-management without human intervention, requiring only the presentation of intents or goals. 

blockchain and autonomous ecosystems for Agentic AI, emphasizing the progressive enhancement of system capabilities and agentic workflows. Conceptually, it parallels the five-level framework for the development of artificial general intelligence (AGI) proposed by OpenAI (Priyanka Sharma), in which the five levels depict the evolution of AGI from passive tools to entities with capabilities that can do the work of an organization, our five-stage roadmap outlines the evolutionary path of LaMAS from passive execution to possessing autonomous collaborative and governance capabilities. 

These five stages form a progressive continuum that reflects the parallel evolution of stakeholder roles, governance mechanisms, and the maturity of the business ecosystem. Figure 3 provides a visualization of this five-stage progression, while Table 2 and Table 3 include the details of key involved parties and core system modules at each stage, respectively. Throughout this evolutionary process, the primary focus of each stage lies not only in technological improvements but also in the transforming roles of key involved parties, as well as in the shifting functional priorities and collaborative patterns of the core system modules. 

In the following five subsections, we conduct an in-depth analysis of each stage, examining their defining characteristics, major evolutionary shifts, and the strategic implications of these changes 

8 

> _**β**_ BetaWeb 

Table 2: Comparison of key involved parties across the five-stage evolutionary roadmap. 

|**Stage**|**Human Users**||**Agent Capabilities**|**Agentic Workflows**|**Agentic Workflows**|
|---|---|---|---|---|---|
|Isolated Silos|Deeply involved in every step||Restricted to specific environ-|Closed processes within individ-||
||||ments|ual systems||
|Pilot|Lead the entire task|process ac-|General-purpose<br>capabilities|Decentralization|assisted<br>by|
|Decentralization|cording to human cognition and||with single-step command exe-|hard-coded static|contracts|
||judgment||cution|||
|Assisted Execution|Reduced<br>day-to-day|execution|Internalized domain-specific ca-|Rules upgradable|by human in-|
||workload||pabilities|tervention||
|Hybrid Governance|Focus<br>on<br>essential|decision-|Possess<br>self-evolution<br>abilities,|Agent-assisted governance with||
||making and governance strategy||capable of continuous improve-|human-machine|collaborative|
||||ment in specific domains|decision-making||
|Full Autonomy|Provide only high-level require-||Fully autonomous,<br>capable of|Fully autonomous agentic gover-||
||ments, fully freed from opera-||performing social behaviors|nance with no human interven-||
||tions|||tion||



Table 3: Comparison of core system modules across the five-stage evolutionary roadmap (cont.). 

|**Stage**|**Task Management**|**Task Execution**|**Agent Management**|**Rule Management**|
|---|---|---|---|---|
|Isolated Silos|Human-centric<br>process|Completed<br>within<br>iso-|Generally<br>non-existent|Static rules maintained|
||without<br>cross-entity|lated systems|or extremely basic|manually|
||collaboration||||
|Pilot|Fixed<br>workflows<br>an-|Simply<br>invoke<br>agents,|Simple identity with ba-|Immutable<br>smart<br>con-|
|Decentralization|alyzed,<br>defined,<br>and|primarily<br>relying<br>on|sic information|tracts manually written|
||managed by humans|fixed functions or APIs||and deployed|
|Assisted Execution|Agents assist in request|Agents lead subtask exe-|Capability<br>evaluation,|Agents<br>generate<br>smart|
||parsing,<br>workflow<br>de-|cution, with capabilities|incentive<br>mechanisms,|contracts based on nat-|
||composition, and other|modularized into plug-|and<br>other<br>governance|ural<br>language<br>requests|
||upstream tasks|gable components|tools are introduced|from humans|
|Hybrid Governance|Agents assist with down-|Proprietary-capability|Controllable agents with|Agents<br>autonomously|
||stream<br>tasks<br>such<br>as|agents undergo continu-|mechanisms for behavior|generate<br>upgrade<br>pro-|
||result<br>verification,<br>but|ous iterative upgrades to|monitoring,<br>governance|posals based on system|
||humans intervene when|enhance adaptability|penalties,<br>and<br>forced|monitoring,<br>subject<br>to|
||necessary||shutdown|human approval|
|Full Autonomy|Agents manage the en-|Agents<br>possess<br>au-|Agents<br>autonomously|Agents<br>autonomously|
||tire<br>task<br>lifecycle<br>au-|tonomous upgrading and|discover,<br>recruit,<br>or|perform on-chain gover-|
||tonomously, forming an|task-space<br>exploration|remove members, form-|nance actions based on|
||end-to-end<br>closed<br>loop|capabilities,<br>enabling|ing<br>a<br>DAO<br>composed|real-time monitoring and|
||without human involve-|self-innovation<br>and|entirely of agents.|on-chain data analysis|
||ment|adaptive expansion|||



for real-world deployment and ecosystem construction. Meanwhile, we have also summarized the key technologies for each stage, which can be found in our GitHub repository[2] . 

## **3.2 Isolated Silos Stage** 

The Isolated Silos stage marks the starting point of the evolutionary roadmap, corresponding to the most prevalent current state in which BetaWeb operates independently within mutually disconnected information silos. Each isolated system, like individual islands, possesses its own rules and operational pipeline, yet lacks effective bridges for data or task interaction with the external environment (Johnson & Vera, 2019). 

> 2 `https://github.com/MatZaharia/BetaWeb` 

9 

> _**β**_ BetaWeb 

At this stage, system workflows are heavily dependent on direct human control, relying on human cognition, judgment, and actions to drive the entire process (Shneiderman, 2022). Agents function merely as passive tools executing fixed programs, restricted to completing predefined tasks within limited environments. Consequently, agentic workflows exhibit to be closed and linear, entirely dependent on manual triggering, intervention, and maintenance (Sapkota et al., 2025). 

In such a state, data, computational power, and intelligent services remain confined within the system’s boundaries, making cross-system resource exchange or collaboration nearly unattainable (Zhuang et al., 2023). While performance within a single domain may remain relatively stable, the absence of cross-domain interaction severely limits the societal value and potential for network effects. These silos not only lead to redundant construction, resource waste, and efficiency loss, but also constrain the user experience to an early-stage as “human-driven, machine-responsive” mode, falling short of forming an adaptive and dynamically interconnected Agentic Web (Sharma et al., 2025). 

Notably, in this stage, the role of blockchain has yet to be fully realized, remaining only as a latent infrastructure layer. The key to transitioning towards the next stage lies in introducing a blockchainbased trusted identity system, enabling cross-system data verification, and establishing preliminary smart contract, thus laying the technical and governance foundations for subsequent decentralized evolution. 

## **3.3 Pilot Decentralization Stage** 

The Pilot Decentralization stage represents the initial transition from isolated silos towards decentralized collaboration. At this stage, limited-scale interconnected channels begin to form in BetaWeb, analogous to opening certain flight routes among islands, creating small-scale trusted interconnection pilots. 

User identities undergo gradual transformation from solely managing the entire process to adopting composite roles as task requesters, execution supervisors, and agent owners (Borghoff et al., 2025). Leveraging blockchain-based identity systems, verifiable identity over agents and their actions are established. Simultaneously, agents acquire preliminary cross-domain collaboration, enabling partial intelligence sharing within a constrained decentralized network and supporting cooperative execution of cross-system tasks. Blockchain is introduced into agentic workflows as a critical trust hub for data verification and task settlement, ensuring data immutability and process traceability. However, task decomposition, allocation, and incentive mechanisms still largely depend on human negotiation and definition, with collaboration characterized by localized bilateral partnerships (Karim et al., 2025). 

This stage effectively breaks the fully isolated status by establishing trustworthy peer-to-peer cooperation, significantly reducing trust costs in cross-system interactions. Users begin to automate and indirectly manage the workflow via on-chain rules and smart contracts (Zou et al., 2025). The blockchain serve as the core technical foundation for securing cooperation and enabling initial multiparty resource integration. 

Utilizing blockchain during this stage faces challenges including cross-chain data exchange (Qasse et al., 2019), lightweight consensus algorithms (Biswas et al., 2019), and scalable proofs of task execution (Yang et al., 2020). On the governance side, exploring on-chain automation for reputation evaluation, task arbitration, and incentive rules is essential (He et al., 2022) but lacking. In brief, the core difficulty lies in jointly enhancing performance and security while preventing excessive ecosystem fragmentation, thus enabling steady progression towards higher levels. 

## **3.4 Assisted Execution Stage** 

In the Assisted Execution stage, BetaWeb further evolves as inter-system connectivity resembles established regular routes and trade alliances between islands, enabling efficient circulation of information exchange and resource transactions within these networks. 

10 

> _**β**_ BetaWeb 

The roles of users shift towards strategic command and goal setting, with minimal intervention in execution details. Agents possess internalized domain-specific expertise and autonomously lead the operation of sub-tasks, dynamically optimizing workflows and task allocation in real-time based on task context and smart contracts (Zheng et al., 2025b). Agentic workflows here increasingly rely on blockchain-driven management modules. The multidimensional lifecycles of agents and tasks are fully coordinated by on-chain rules, supporting automatic invocation of off-chain computational and storage resources, thus achieving efficient on-chain and off-chain collaboration (Yang, 2024). Although automation is greatly enhanced, humans remain responsible for manual monitoring and auditing, and is required to adjust rules governed by smart contracts based on real-time system feedback (Zhang et al., 2025a). 

This stage significantly improves task execution efficiency and system throughput, transforming users from executors to supervisors and effectively freeing human labor. Blockchain functions not only as a trusted foundation but also as the core platform for task scheduling and incentive distribution, promoting economic cooperation among multiple agents and forming a highly autonomous collaborative economic community. 

Multi-agent coordinated planning (Fang et al., 2024), dynamic incentive model (Xing et al., 2024), and secure proofs for hybrid on-chain and off-chain computation (Chiedu et al., 2025) are important here. Additionally, designing on-chain economic rules with adaptive and evolutionary capabilities is essential to address the increasing task complexity and agent capabilities in a dynamic environment (Nguyen Thanh et al., 2024). However, currently, agents mainly autonomously handle task execution and upstream processes, while human input for downstream monitoring, verification, and auditing remains significant. Further reducing human intervention in these areas is a focal point for future optimization. 

## **3.5 Hybrid Governance Stage** 

In the Hybrid Governance stage, BetaWeb will establish large-scale decentralized systems spanning multiple industries and domains, with a balanced power structure among users, agents, and agentic workflows. The system possesses on-chain autonomous governance capabilities while retaining human intervention rights for critical strategic decisions, forming a governance pattern driven by both technology and institutional frameworks (Chaffer et al., 2024). 

Agents not only autonomously execute tasks but also actively participate in governance processes, including rule proposals and parameter adjustments through on-chain governance activities (Valiente & Pavón, 2024). When execution results significantly deviate from expectations, humans can intervene immediately to ensure effective control. The workflows achieve full-chain autonomy coupled with real-time supervision (Edwards et al., 2025). For example, the rules management module, driven jointly by on-chain voting and smart contracts, dynamically optimizes economic models and incentive mechanisms based on actual operation, ensuring the fairness and efficiency in governance. 

The core evolution of this stage lies in a qualitative leap in governance capability, achieving a dynamic balance between autonomy and oversight. Humans primarily handle strategic formulation and critical supervision, while agents continuously improve execution and develop preliminary autonomous governance abilities, collaboratively addressing highly uncertain and complex operational environments (Zhuge et al., 2024; Zhang et al., 2025b). This governance mode provides stable institutional guarantees and controllability for large-scale distributed collaboration (Karim et al., 2025), significantly enhancing system resilience and social acceptance (Guerra et al., 2025), promoting ecosystem expansion across industries (Jovanovic et al., 2022), and laying a solid institutional foundation for the eventual transition to full autonomy. 

This stage requires breakthroughs in constructing frameworks for continuous learning and selfevolution of agents (Wang et al., 2025b), implementing low-latency cross-domain consensus mechanisms (Liu et al., 2023), and designing scalable and efficient on-chain governance frameworks (Reijers et al., 2021). Additionally, exploring the legal and ethical boundaries of human-machine co- 

11 

> _**β**_ BetaWeb 

governance, building fault-tolerance mechanisms, and rapid recovery solutions are essential to ensure the sustained system operation in the face of global emergencies (Hu et al., 2025a; de Witt, 2025). 

## **3.6 Full Autonomy Stage** 

Entering the ultimate stage of Full Autonomy, BetaWeb evolves into a self-regulating global intelligent society with fully autonomous capabilities. Routine task execution no longer depends on direct human intervention. Users only need to express high-level intents or set value goals, enabling the system to automatically allocate and execute tasks within a comprehensive framework of adaptive rules and incentive mechanisms (Chen et al., 2024). 

Here, agents possess demand-aware capabilities for self-upgrading and self-exploring (Zheng et al., 2025a), independently completing complex tasks and related social behaviors (Luo et al., 2025). They can even proactively propose new rules and adjust existing parameters on-chain. Agentic workflows achieve full automation, with task management, execution, incentive distribution, and governance all driven by blockchain, thereby constructing a closed-loop, dynamic, and boundaryless agentic collaborative ecosystem (Mukherjee & Chang, 2025). 

This stage marks the transition of BetaWeb into a genuine autonomy era, where humans are fully liberated from operational or managerial roles and instead act as guides for overarching values and objectives. Through the collaboration among agents, the system attains self-governance across industries and regions. Blockchain serves as the sole foundation for global trust and resource coordination, and even driving fundamental transformations in economic and social patterns (Tallam, 2025; Jaggavarapu, 2025; Wang et al., 2025a). 

However, full autonomy also brings numerous ethical, legal, and security issues, such as value alignment, unification of cross-cultural norms, and the profound impact of self-governance systems on human societal structures (Hu et al., 2025b; Chaffer, 2025). On the technical front, it is essential to guarantee blockchain’s high performance and adaptability on a global scale, while establishing verifiable self-evolution mechanisms and robust security protections (Fang et al., 2025). Addressing these challenges will determine the feasibility, reliability, and sustainable development of fully autonomous BetaWeb. 

## **4 Existing Products** 

Currently, products or applications combining blockchain with LaMAS exhibit a diverse development landscape, spanning broad explorations from DAOs to industry-specific LaMAS. These existing solutions continuously explore new modes of collaboration among agents, aiming to leverage blockchain to enhance system transparency, trustworthiness, and security. However, the overall market remains in an early yet rapidly evolving stage, with technologies and applications not yet fully matured, and widespread adoption facing numerous challenges. 

Specifically, within the Web3-oriented innovative attempts, many projects focus on building ecosystems for collaboration and interaction among users. For example, platforms including HajimeAI (HajimeTeam), ChainOpera AI (ChainOpera AI), and AGNTCY (AGNTCY) commonly integrate smart contracts to achieve agent identity management and task scheduling, creating interoperable LaMAS that foster effective task execution environments. These projects predominantly rely on public blockchains and open-source communities, emphasizing values of openness, transparency, and decentralization. Overall, such innovations highlight open ecosystems and value co-creation, but still require breakthroughs in commercialization and large-scale impact. 

In applications guided by government regulation and industry compliance, more products focus on meeting regulatory requirements and privacy protection, especially within critical sectors such as finance, healthcare, and public services. These products, such as Legion (PayEgis) and BOP (Xinghuo), typically leverage blockchain to ensure traceability and immutability of agent interactions, enabling regulators to dynamically monitor agent behavior and manage risks. Meanwhile, 

12 

> _**β**_ BetaWeb 

agent capability evaluation and incentive mechanisms are strictly incorporated within compliance frameworks to achieve secure and trustworthy applications. These efforts are mostly localized pilots tightly aligned with specific industry demands, prioritizing security and compliance. Although their application scope remains limited, such products play a vital role in safeguarding system security and enhancing industry trust, showing potential for scalable deployment under controlled environments. 

In summary, based on the evolutionary roadmap presented in this paper, existing products largely remain at Stage 2. Current technological and product explorations still face the complex challenge of balancing high concurrency processing, strong trust guarantees, and data privacy protection. The triple demands have yet to be fully reconciled. This fundamental tension constitutes both a developmental bottleneck and a critical focus for future breakthroughs, underscoring the broad application prospects and profound strategic value of BetaWeb via the blockchain paradigm. 

## **5 Challenges and Open Problems** 

In the systematic analysis of phased evolution and existing products, BetaWeb demonstrates substantial potential. Within this ambitious vision, it is crucial to proactively identify the multiple challenges and open questions that may arise during its development, which is an interdisciplinary puzzle. To provide a comprehensive assessment, we examine these issues across technological, ecosystem, and societal dimensions, aiming to elucidate current bottlenecks and outline directions for future research. 

## **5.1 Technological Dimension** 

The construction of highly autonomous systems in BetaWeb faces multiple technical difficulties. First, the efficiency and security of cross-chain and multi-domain data exchange require significant breakthroughs. Achieving high-performance real-time collaboration while ensuring verifiability of task execution and computational trust remains a core challenge (Xie et al., 2025; Yang et al., 2025a). Ideally, to minimize trust costs, nearly all processes should operate on-chain, yet achieving largescale on-chain deployment without compromising system performance remains an open problem. Second, multi-agent dynamic planning and task allocation algorithms must balance scalability and adaptability, ensuring stability and efficiency in complex and rapidly changing environments (Jiang et al., 2024) while resisting potential attacks such as malicious agents, task tampering, or information asymmetry (Deng et al., 2025). Third, blockchain plays a central role in the agentic workflows, but its performance, scalability, and low-latency communication still require optimization, particularly in global-scale applications (Rao et al., 2024; Gracy & Jeyavadhanam, 2021). 

## **5.2 Ecosystem Dimension** 

BetaWeb involves multiple stakeholders and cross-industry cooperation, making institutional arrangements, collaborative mechanisms, and incentive design highly complex. Conflicts of interest and cooperative game problems require resolution, particularly in fully transparent and on-chain environments (Mohammed Abdul et al., 2025; Popoola et al., 2024). While full-chain governance can enhance transparency, their fairness may still be questioned. Issues such as last-touch attribution (Sriram et al., 2022) or forward traceability (Mezzi et al., 2025) are required to be considered when redesigning the market mechanisms within a blockchain context to ensure equitable value distribution. Especially in agent marketplaces, questions concerning fair transaction of tasks, resources, and reputation, as well as incentives that prevent local interests from undermining overall ecosystem integrity, remain open (Yang et al., 2025c). Additionally, cross-domain identity verification, trust establishment, smart contract upgrades, and cross-system coordination must be systematically addressed to ensure long-term ecosystem stability and evolution (Ivaninskiy & Ivashkovskaya). 

13 

> _**β**_ BetaWeb 

## **5.3 Societal Dimension** 

The high autonomy of BetaWeb raises a series of ethical and legal issues. Alignment of cross-cultural values and rules, responsibility attribution for autonomous agent behaviors (Ayad & Plaks, 2025) and potential changes in productivity and production relations (Qian & Wexler, 2024) require careful regulation. Autonomous decisions may generate unforeseen social impacts, particularly in high-risk tasks or disputes over resource allocation, making clear responsibility assignment essential for societal trust (Chaffer et al., 2025). Fully transparent decision-making and transaction processes, while enhancing accountability, may also provoke new conflicts, exacerbated by the lag in legal frameworks, thereby potentially limiting widespread adoption (Lescrauwaet et al., 2022). Consequently, institutional innovation and policy guidance within a human-machine governance framework are necessary (Kraprayoon et al., 2025; Vigoda-Gadot & Mizrahi, 2024). Furthermore, the security and resilience of highly autonomous systems pose new requirements for public safety and social stability, necessitating effective monitoring, emergency response, and recovery mechanisms (Flammini et al., 2024; Shandilya et al., 2024; Chen et al., 2021). 

## **6 Conclusion** 

This paper systematically reviews the overall framework and evolutionary trajectory of BetaWeb, covering five stages from Isolated Silos to Full Autonomy. It provides a detailed exposition of the key involved parties, core system modules, and technical characteristics at each stage, alongside a survey and analysis of existing blockchain-enabled LaMAS products. The study highlights the significant role and immense potential of blockchain in realizing decentralized collaboration, trusted governance, and enhanced autonomy within LaMAS. Overall, blockchain not only offers fundamental trust mechanisms and a trustless collaborative environment for LaMAS but also facilitates the realization of autonomous governance, thereby contributing to the construction of a more open, secure, and efficient agent ecosystem. 

It is noteworthy that, although this paper focuses on the driving role of blockchain technology in BetaWeb, the agents themselves also reciprocally promote the development of blockchain technology. As agent autonomy continuously advances, especially in the stages of Hybrid Governance and Full Autonomy, agents increasingly engage in on-chain governance activities such as smart contract upgrades and adaptive parameter adjustments (Ye et al., 2025; Shimony, 2025). This high degree of autonomous agent behavior drives innovation and optimization of blockchain governance mechanisms, fostering the ongoing evolution of on-chain self-governance frameworks. Therefore, the future relationship between blockchain and BetaWeb will be characterized by bidirectional interaction and collaborative progress. Through continuous evolution, BetaWeb is expected to achieve broader practical deployment, thereby promoting comprehensive upgrades of the intelligent economy and intelligent society. 

As BetaWeb continues to evolve, our future work will involve developing a prototype to test its practical viability. By conducting experiments and analyzing the results, we aim to further validate the effectiveness of blockchain-driven decentralized governance and autonomy in real-world applications. These findings will provide valuable insights into the broader deployment of BetaWeb and its potential impact on the agent-centric economy and society. 

## **References** 

Deepak Bhaskar Acharya, Karthigeyan Kuppan, and B. Divya. Agentic ai: Autonomous intelligence for complex goals—a comprehensive survey. _IEEE Access_ , 13:18912–18936, 2025. 

AGNTCY. Agntcy origins. `https://docs.agntcy.org/` . Accessed: 2025-08-13. 

Reem Ayad and Jason E. Plaks. Attributions of intent and moral responsibility to AI agents. _Computers in Human Behavior: Artificial Humans_ , 3:100107, 2025. ISSN 2949-8821. 

14 

> _**β**_ BetaWeb 

- Yuhao Bai, Qin Hu, Seung-Hyun Seo, Kyubyung Kang, and John J Lee. Public participation consortium blockchain for smart city governance. _IEEE Internet of Things Journal_ , 9(3):2094– 2108, 2021. 

- Badr Bellaj, Aafaf Ouaddah, Emmanuel Bertin, Noel Crespi, and Abdellatif Mezrioui. Drawing the boundaries between blockchain and blockchain-like systems: A comprehensive survey on distributed ledger technologies. _Proceedings of the IEEE_ , 112(3):247–299, 2024. 

- Elettra Bietti. Data is infrastructure. _Theoretical Inquiries in Law_ , 26(1):55–87, 2025. 

- Sujit Biswas, Kashif Sharif, Fan Li, Sabita Maharjan, Saraju P Mohanty, and Yu Wang. Pobt: A lightweight consensus algorithm for scalable iot business blockchain. _IEEE Internet of Things Journal_ , 7(3):2343–2355, 2019. 

- Severin Bonnet and Frank Teuteberg. Decentralized autonomous organizations: a systematic literature review and research agenda. _International Journal of Innovation and Technology Management_ , 21(04):2450026, 2024. 

- Uwe M Borghoff, Paolo Bottoni, and Remo Pareschi. Human-artificial interaction in the age of agentic ai: a system-theoretical approach. _Frontiers in Human Dynamics_ , 7:1579166, 2025. 

- Arthur Caetano, Kavya Verma, Atieh Taheri, Radha Kumaran, Zichen Chen, Jiaao Chen, Tobias Höllerer, and Misha Sra. Agentic workflows for conversational human-ai interaction design. _arXiv preprint arXiv:2501.18002_ , 2025. 

- Darko Čapko, Srđan Vukmirović, and Nemanja Nedić. State of the art of zero-knowledge proofs in blockchain. In _2022 30th Telecommunications Forum (TELFOR)_ , pp. 1–4. IEEE, 2022. 

- Tomer Jordi Chaffer. Can we govern the agent-to-agent economy? _arXiv preprint arXiv:2501.16606_ , 2025. 

- Tomer Jordi Chaffer, Charles von Goins II, Bayo Okusanya, Dontrail Cotlage, and Justin Goldston. Decentralized governance of autonomous ai agents. _arXiv preprint arXiv:2412.17114_ , 2024. 

- Tomer Jordi Chaffer, Charles von Goins II, Bayo Okusanya, Dontrail Cotlage, and Justin Goldston. Decentralized governance of autonomous AI agents. _arXiv preprint arXiv:2412.17114_ , 2025. 

- ChainOpera AI. Chainopera AI: The collaborative intelligence of AI agent network. `https://pape r.chainopera.ai/` . Accessed: 2025-08-13. 

- Hualong Chen, Yuanqiao Wen, Man Zhu, Yamin Huang, Changshi Xiao, Tao Wei, and Axel Hahn. From automation system to autonomous system: An architecture perspective. _Journal of Marine Science and Engineering_ , 9(6), 2021. ISSN 2077-1312. 

- Jiaqi Chen, Yuxian Jiang, Jiachen Lu, and Li Zhang. S-agents: Self-organizing agents in open-ended environments. _arXiv preprint arXiv:2402.04578_ , 2024. 

- Chiedu Charles Chiedu, Keke Gai, Yihang Wei, and Kai Ding. A survey on on-chain-off-chain scalability in blockchain. In _2025 IEEE 11th Conference on Big Data Security on Cloud (BigDataSecurity)_ , pp. 71–82. IEEE, 2025. 

- Christian Schroeder de Witt. Open challenges in multi-agent security: Towards secure systems of interacting ai agents. _arXiv preprint arXiv:2505.02077_ , 2025. 

- Zehang Deng, Yongjian Guo, Changzhou Han, Wanlun Ma, Junwu Xiong, Sheng Wen, and Yang Xiang. Ai agents under threat: A survey of key security challenges and future pathways. _ACM Computing Surveys_ , 57(7):1–36, 2025. 

- Adrian-Tudor Dumitrescu and Johan Pouwelse. Failures of public key infrastructure: 53 year survey. _arXiv preprint arXiv:2401.05239_ , 2024. 

15 

> _**β**_ BetaWeb 

- Justin Edwards, Andy Nguyen, Joni Lämsä, Marta Sobocinski, Ridwan Whitehead, Belle Dang, Anni-Sofia Roberts, and Sanna Järvelä. Human-ai collaboration: Designing artificial agents to facilitate socially shared regulation among learners. _British Journal of Educational Technology_ , 56(2):712–733, 2025. 

- Chengyuan Fang, Jianlin Mao, Dayan Li, Ning Wang, and Niya Wang. A coordinated scheduling approach for task assignment and multi-agent path planning. _Journal of King Saud UniversityComputer and Information Sciences_ , 36(1):101930, 2024. 

- Jinyuan Fang, Yanwen Peng, Xi Zhang, Yingxu Wang, Xinhao Yi, Guibin Zhang, Yi Xu, Bin Wu, Siwei Liu, Zihao Li, et al. A comprehensive survey of self-evolving ai agents: A new paradigm bridging foundation models and lifelong agentic systems. _arXiv preprint arXiv:2508.07407_ , 2025. 

- Francesco Flammini, Cristina Alcaraz, Emanuele Bellini, Stefano Marrone, Javier Lopez, and Andrea Bondavalli. Towards trustworthy autonomous systems: Taxonomies and future perspectives. _IEEE Transactions on Emerging Topics in Computing_ , 12(2):601–614, 2024. doi: 10.1109/TETC.2022. 3227113. 

- Mingyan Gao, Yanzi Li, Banruo Liu, Yifan Yu, Phillip Wang, Ching-Yu Lin, and Fan Lai. Singleagent or multi-agent systems? why not both? _arXiv preprint arXiv:2505.18286_ , 2025. 

- Tim Geppert, Stefan Deml, David Sturzenegger, and Nico Ebert. Trusted execution environments: Applications and organizational challenges. _Frontiers in Computer Science_ , 4:930741, 2022. 

- Google Cloud. Google agentspace. `https://cloud.google.com/products/agentspace` . Accessed: 2025-08-13. 

- M Gracy and B Rebecca Jeyavadhanam. A systematic review of blockchain-based system: Transaction throughput latency and challenges. In _2021 International Conference on Computational Intelligence and Computing Applications (ICCICA)_ , pp. 1–6. IEEE, 2021. 

- Katia Guerra, Chang Koh, Victor Prybutok, and Vess Johnson. Iot and blockchain: A review and a technical-legal-social acceptance model. _Journal of Computer Information Systems_ , 65(1):47–63, 2025. 

- Taicheng Guo, Xiuying Chen, Yaqi Wang, Ruidi Chang, Shichao Pei, Nitesh V Chawla, Olaf Wiest, and Xiangliang Zhang. Large language model based multi-agents: A survey of progress and challenges. _arXiv preprint arXiv:2402.01680_ , 2024. 

- HajimeTeam. HajimeAI: Decentralized multi-agent graph on solana, democratic AI for all. `https: //www.hajime.ai/blog/01J4RP2A58X5ZDP1G51R852WM8` . Accessed: 2025-08-13. 

- Rong Han, Zheng Yan, Xueqin Liang, and Laurence T Yang. How can incentive mechanisms and blockchain benefit with each other? a survey. _ACM Computing Surveys_ , 55(7):1–38, 2022. 

- Shanshan Han, Qifan Zhang, Yuhang Yao, Weizhao Jin, and Zhaozhuo Xu. Llm multi-agent systems: Challenges and open problems. _arXiv preprint arXiv:2402.03578_ , 2024. 

- Songlin He, Yuan Lu, Qiang Tang, Guiling Wang, and Chase Qishi Wu. Blockchain-based p2p content delivery with monetary incentivization and fairness guarantee. _IEEE Transactions on Parallel and Distributed Systems_ , 34(2):746–765, 2022. 

- Botao Hu, Helena Rong, and Janna Tay. Is decentralized artificial intelligence governable? towards machine sovereignty and human symbiosis. _Helena and Tay, Janna_ , 2025a. 

- Botao Amber Hu, Yuhan Liu, and Helena Rong. Trustless autonomy: Understanding motivations, benefits and governance dilemma in self-sovereign decentralized ai agents. _arXiv preprint arXiv:2505.09757_ , 2025b. 

- Marcello Ienca. On artificial intelligence and manipulation. _Topoi_ , 42(3):833–842, 2023. 

16 

> _**β**_ BetaWeb 

- Tariqul Islam, Faisal Haque Bappy, Tarannum Shaila Zaman, Md Sajidul Islam Sajid, and Mir Mehedi Ahsan Pritom. Mrl-pos: A multi-agent reinforcement learning based proof of stake consensus algorithm for blockchain. In _2024 IEEE 14th Annual Computing and Communication Workshop and Conference (CCWC)_ , pp. 0409–0413. IEEE, 2024. 

- Ilya Ivaninskiy and Irina Ivashkovskaya. Are blockchain-based digital transformation and ecosystembased business models mutually reinforcing? the principal-agent conflict perspective. 12(4):643– 670. ISSN 2147-4281. 

- Manoj Kumar Reddy Jaggavarapu. The evolution of agentic ai: Architecture and workflows for autonomous systems. _Journal Of Multidisciplinary_ , 5(7):418–427, 2025. 

- Feibo Jiang, Yubo Peng, Li Dong, Kezhi Wang, Kun Yang, Cunhua Pan, Dusit Niyato, and Octavia A Dobre. Large language model enhanced multi-agent systems for 6g communications. _IEEE Wireless Communications_ , 2024. 

- Matthew Johnson and Alonso Vera. No ai is an island: the case for teaming intelligence. _AI magazine_ , 40(1):16–28, 2019. 

- Marin Jovanovic, Nikola Kostić, Ina M Sebastian, and Tomaz Sedej. Managing a blockchain-based platform ecosystem for industry-wide adoption: The case of tradelens. _Technological Forecasting and Social Change_ , 184:121981, 2022. 

- Md Monjurul Karim, Dong Hoang Van, Sangeen Khan, Qiang Qu, and Yaroslav Kholodov. Ai agents meet blockchain: A survey on secure and scalable collaboration for multi-agents. _Future Internet_ , 17(2):57, 2025. 

- Jam Kraprayoon, Zoe Williams, and Rida Fayyaz. AI agent governance: A field guide. _arXiv preprint arXiv:2505.21808_ , 2025. 

- Naveen Krishnan. Ai agents: Evolution, architecture, and real-world applications. _arXiv preprint arXiv:2503.12687_ , 2025. 

- Katerina Ksystra, Maria Dimarogkona, Nikolaos Triantafyllou, Petros Stefaneas, and Petros Kavassalis. Towards a methodology for formally analyzing federated identity management systems. In _International Symposium on Leveraging Applications of Formal Methods_ , pp. 382–405. Springer, 2022. 

- Paul M Leonardi. Homo agenticus in the age of agentic ai: Agency loops, power displacement, and the circulation of responsibility. _Information and Organization_ , 35(3):100582, 2025. 

- Lyytinen Lescrauwaet, Hekkert Wagner, Cheng Yoon, and Sovacool Shukla. Adaptive legal frameworks and economic dynamics in emerging tech-nologies: Navigating the intersection for responsible innovation. _Law and Economics_ , 16(3):202–220, Oct. 2022. 

- Leilei Lin, Yumeng Jin, Yingming Zhou, Wenlong Chen, and Chen Qian. Mao: A framework for process model generation with multi-agent orchestration. _arXiv preprint arXiv:2408.01916_ , 2024. 

- Yizhong Liu, Xinxin Xing, Ziheng Tong, Xun Lin, Jing Chen, Zhenyu Guan, Qianhong Wu, and Willy Susilo. Secure and scalable cross-domain data sharing in zero-trust cloud-edge-end environment based on sharding blockchain. _IEEE Transactions on Dependable and Secure Computing_ , 21(4):2603–2618, 2023. 

- Junyu Luo, Weizhi Zhang, Ye Yuan, Yusheng Zhao, Junwei Yang, Yiyang Gu, Bohan Wu, Binqi Chen, Ziyue Qiao, Qingqing Long, et al. Large language model agent: A survey on methodology, applications and challenges. _arXiv preprint arXiv:2503.21460_ , 2025. 

- Tula Masterman, Sandi Besen, Mason Sawtell, and Alex Chao. The landscape of emerging ai agent architectures for reasoning, planning, and tool calling: A survey. _arXiv preprint arXiv:2404.11584_ , 2024. 

17 

> _**β**_ BetaWeb 

- Emanuele Mezzi, Asimina Mertzani, Michael P. Manis, Siyanna Lilova, Nicholas Vadivoulis, Stamatis Gatirdakis, Styliani Roussou, and Rodayna Hmede. Who owns the output? bridging law and technology in LLMs attribution. _arXiv preprint arXiv:2504.01032_ , 2025. 

- Shezon Saleem Mohammed Abdul, Anup Shrestha, and Jianming Yong. Toward the mass adoption of blockchain: Cross-industry insights from defi, gaming, and data analytics. _Big Data and Cognitive Computing_ , 9(7):178, 2025. 

- Anirban Mukherjee and Hannah Hanwen Chang. Agentic ai: Autonomy, accountability, and the algorithmic society. _arXiv preprint arXiv:2502.00289_ , 2025. 

- Vineeth Sai Narajala and Om Narayan. Securing agentic ai: A comprehensive threat model and mitigation framework for generative ai agents. _arXiv preprint arXiv:2504.19956_ , 2025. 

- Binh Nguyen Thanh, Ha Xuan Son, and Diem Thi Hong Vo. Blockchain: the economic and financial institution for autonomous ai? _Journal of Risk and Financial Management_ , 17(2):54, 2024. 

- Xuchen Pan, Dawei Gao, Yuexiang Xie, Yushuo Chen, Zhewei Wei, Yaliang Li, Bolin Ding, Ji-Rong Wen, and Jingren Zhou. Very large-scale multi-agent simulation in agentscope. _arXiv preprint arXiv:2407.17789_ , 2024. 

- Akshat Pandey and Aylin Caliskan. Disparate impact of artificial intelligence bias in ridehailing economy’s price discrimination algorithms. In _Proceedings of the 2021 AAAI/ACM Conference on AI, Ethics, and Society_ , pp. 822–833, 2021. 

- PayEgis. Legion space. `https://space.tongfudun.com/?lang=en` . Accessed: 2025-08-13. 

- Tatiana Petrova, Aleksandr Puzikov, Boris Bliznukov, and Radu State. From semantic web and mas to agentic AI: A unified narrative of the web of agents. _arXiv preprint arXiv:2507.10644_ , 2025. 

- Oladapo Adeboye Popoola, Henry Ejiga Adama, Chukwuekem David Okeke, and Abiodun Emmanuel Akinoso. Cross-industry frameworks for business process reengineering: Conceptual models and practical executions. _World Journal of Advanced Research and Reviews_ , 22(01):1198–1208, 2024. 

- Priyanka Sharma. The path to AGI: A deep dive into openAI’s 5-level framework. `https://twim bit.com/about/blogs/the-path-to-agi-a-deep-dive-into-openais-5-level-framework` . Accessed: 2025-08-13. 

- Deepak Puthal, Amit Kumar Mishra, Saraju P Mohanty, Antonella Longo, and Chan Yeob Yeun. Shadow AI: Cyber security implications, opportunities and challenges in the unseen frontier. _SN Computer Science_ , 6(5):405, 2025. 

- Ilham A Qasse, Manar Abu Talib, and Qassim Nasir. Inter blockchain communication: A survey. In _Proceedings of the ArabWIC 6th Annual International Conference Research Track_ , pp. 1–6, 2019. 

- Chen Qian, Zihao Xie, Yifei Wang, Wei Liu, Kunlun Zhu, Hanchen Xia, Yufan Dang, Zhuoyun Du, Weize Chen, Cheng Yang, et al. Scaling large language model-based multi-agent collaboration. _arXiv preprint arXiv:2406.07155_ , 2024. 

- Crystal Qian and James Wexler. Take it, leave it, or fix it: Measuring productivity and trust in human-AI collaboration. In _Proceedings of the 29th International Conference on Intelligent User Interfaces_ , IUI ’24, pp. 370–384, New York, NY, USA, 2024. Association for Computing Machinery. ISBN 9798400705083. 

- Iqra Sadia Rao, ML Mat Kiah, M Muzaffar Hameed, and Zain Anwer Memon. Scalability of blockchain: a comprehensive review and future research direction. _Cluster Computing_ , 27(5): 5547–5570, 2024. 

18 

> _**β**_ BetaWeb 

- Shaina Raza, Ranjan Sapkota, Manoj Karkee, and Christos Emmanouilidis. Trism for agentic ai: A review of trust, risk, and security management in llm-based agentic multi-agent systems. _arXiv preprint arXiv:2506.04133_ , 2025. 

- Wessel Reijers, Iris Wuisman, Morshed Mannan, Primavera De Filippi, Christopher Wray, Vienna Rae-Looi, Angela Cubillos Vélez, and Liav Orgad. Now the code runs itself: On-chain and off-chain governance of blockchain technologies. _Topoi_ , 40(4):821–831, 2021. 

- Alessandro Ricci, Stefano Mariani, Franco Zambonelli, Samuele Burattini, Cristiano Castelfranchi, et al. The cognitive hourglass: Agent abstractions in the large models era. In _AAMAS_ , volume 24, pp. 2706–2711, 2024. 

- Hani Sami, Rabeb Mizouni, Hadi Otrok, Shakti Singh, Jamal Bentahar, and Azzam Mourad. Learnchain: Transparent and cooperative reinforcement learning on blockchain. _Future Generation Computer Systems_ , 150:255–271, 2024. 

- Ranjan Sapkota, Konstantinos I Roumeliotis, and Manoj Karkee. Ai agents vs. agentic ai: A conceptual taxonomy, applications and challenges. _arXiv preprint arXiv:2505.10468_ , 2025. 

- Muhammad Usama Sardar and Christof Fetzer. Confidential computing and related technologies: a critical review. _Cybersecurity_ , 6(1):10, 2023. 

- Morgan Klaus Scheuerman. In the walled garden: Challenges and opportunities for research on the practices of the AI tech industry. In _Proceedings of the 2024 ACM Conference on Fairness, Accountability, and Transparency_ , pp. 456–466, 2024. 

- Amazon Web Services. Aws marketplace. `https://aws.amazon.com/marketplace/` . Accessed: 2025-08-13. 

- Shishir Kumar Shandilya, Agni Datta, Yash Kartik, and Atulya Nagar. _Advancing Security and Resilience_ , pp. 459–529. Springer Nature Switzerland, Cham, 2024. ISBN 978-3-031-53290-0. doi: 10.1007/978-3-031-53290-0_8. URL `https://doi.org/10.1007/978-3-031-53290-0_8` . 

- Rishi Sharma, Martijn de Vos, Pradyumna Chari, Ramesh Raskar, and Anne-Marie Kermarrec. Collaborative agentic AI needs interoperability across ecosystems. _arXiv preprint arXiv:2505.21550_ , 2025. 

- Ori Shimony. Ai-driven daos. In _Decentralized Autonomous Organizations in the Legal Landscape_ , pp. 348–372. Edward Elgar Publishing, 2025. 

- Ben Shneiderman. Human-centered ai: ensuring human control while increasing automation. In _Proceedings of the 5th Workshop on Human Factors in Hypertext_ , pp. 1–2, 2022. 

- KV Sriram, Anup Ananda Poojary, Vineet Jawa, and Giridhar B Kamath. Return on investment and return on ad spend at the action level of AIDA using last touch attribution method on digital advertising platforms. _International Journal of Internet Marketing and Advertising_ , 17 (1-2):111–132, 2022. 

- Zhaoyan Sun, Jiayi Wang, Xinyang Zhao, Jiachi Wang, and Guoliang Li. Data agent: A holistic architecture for orchestrating data+ ai ecosystems. _arXiv preprint arXiv:2507.01599_ , 2025. 

- Krti Tallam. From autonomous agents to integrated systems, a new paradigm: Orchestrated distributed intelligence. _arXiv preprint arXiv:2503.13754_ , 2025. 

- Yung Po Tsang, Carman Ka Man Lee, Kening Zhang, Chun-Ho Wu, and WH Ip. On-chain and off-chain data management for blockchain-internet of things: A multi-agent deep reinforcement learning approach. _Journal of Grid Computing_ , 22(1):16, 2024. 

- Maria-Cruz Valiente and Juan Pavón. Web3-dao: An ontology for decentralized autonomous organizations. _Journal of Web Semantics_ , 82:100830, 2024. 

19 

> _**β**_ BetaWeb 

- Eran Vigoda-Gadot and Shlomo Mizrahi. The digital governance puzzle: Towards integrative theory of humans, machines, and organizations in public management. _Technology in Society_ , 77:102530, 2024. ISSN 0160-791X. 

- Lei Wang, Chen Ma, Xueyang Feng, Zeyu Zhang, Hao Yang, Jingsen Zhang, Zhiyuan Chen, Jiakai Tang, Xu Chen, Yankai Lin, et al. A survey on large language model based autonomous agents. _Frontiers of Computer Science_ , 18(6):186345, 2024a. 

- Siyuan Wang, Zhuohan Long, Zhihao Fan, Zhongyu Wei, and Xuanjing Huang. Benchmark selfevolving: A multi-agent framework for dynamic llm evaluation. _arXiv preprint arXiv:2402.11443_ , 2024b. 

- Yanqing Wang and Hong Chen. Blockchain: A potential technology to improve the performance of collaborative emergency management with multi-agent participation. _International Journal of Disaster Risk Reduction_ , 72:102867, 2022. 

- Yuntao Wang, Shaolong Guo, Yanghe Pan, Zhou Su, Fahao Chen, Tom H Luan, Peng Li, Jiawen Kang, and Dusit Niyato. Internet of agents: Fundamentals, applications, and challenges. _arXiv preprint arXiv:2505.07176_ , 2025a. 

- Zihan Wang, Kangrui Wang, Qineng Wang, Pingyue Zhang, Linjie Li, Zhengyuan Yang, Xing Jin, Kefan Yu, Minh Nhat Nguyen, Licheng Liu, et al. Ragen: Understanding self-evolution in llm agents via multi-turn reinforcement learning. _arXiv preprint arXiv:2504.20073_ , 2025b. 

- Zili Xiao, Ke Liu, Miao Hu, and Di Wu. Deepcts: A deep reinforcement learning approach for ai container task scheduling. In _Proceedings of the 2024 3rd Asia Conference on Algorithms, Computing and Machine Learning_ , pp. 342–347, 2024. 

- Zeqiang Xie, Xiaowei Zhang, and Xinbing Liu. Enhanced efficiency and security in cross-chain transmission of blockchain internet of ports through multi-feature-based joint learning. _Scientific Reports_ , 15(1):6199, 2025. 

- Yumei Xing, Jian Wu, Francisco Chiclana, Changyong Liang, and Ronald R Yager. An incentive mechanism-based minimum adjustment consensus model under dynamic trust relationship. _IEEE Transactions on Cybernetics_ , 54(6):3666–3678, 2024. 

- Xinghuo. Xinghuo-developer-AI-copilot. `https://github.com/caict-4iot-dev/Xinghuo-devel oper-AI-copilot` . Accessed: 2025-08-13. 

- Gang Xu, De-Lun Kong, Kejia Zhang, Shiyuan Xu, Yibo Cao, Yanhui Mao, Jianyong Duan, Jiawen Kang, and Xiu-Bo Chen. A model value transfer incentive mechanism for federated learning with smart contracts in AIoT. _IEEE Internet of Things Journal_ , 2024. 

- Wei Xu, Marvin J Dainoff, Liezhong Ge, and Zaifeng Gao. Transitioning to human interaction with ai systems: New challenges and opportunities for hci professionals to enable human-centered ai. _International Journal of Human–Computer Interaction_ , 39(3):494–518, 2023. 

- Di Yang, Chengnian Long, Han Xu, and Shaoliang Peng. A review on scalability of blockchain. In _Proceedings of the 2020 2nd International Conference on Blockchain Technology_ , pp. 1–6, 2020. 

- Haowei Yang. Swarm contract: A multi-sovereign agent consensus mechanism. _arXiv preprint arXiv:2412.19256_ , 2024. 

- Lingxiao Yang, Xuewen Dong, Zhiguo Wan, Sheng Gao, Wei Tong, Di Lu, Yulong Shen, and Xiaojiang Du. Asyncsc: An asynchronous sidechain for multi-domain data exchange in internet of things. In _IEEE INFOCOM 2025-IEEE Conference on Computer Communications_ , pp. 1–10. IEEE, 2025a. 

- Yingxuan Yang, Qiuying Peng, Jun Wang, Ying Wen, and Weinan Zhang. LLM-based multi-agent systems: Techniques and business perspectives. _arXiv preprint arXiv:2411.14033_ , 2024. 

20 

> _**β**_ BetaWeb 

- Yingxuan Yang, Mulei Ma, Yuxuan Huang, Huacan Chai, Chenyu Gong, Haoran Geng, Yuanjian Zhou, Ying Wen, Meng Fang, Muhao Chen, et al. Agentic web: Weaving the next web with AI agents. _arXiv preprint arXiv:2507.21206_ , 2025b. 

- Yingxuan Yang, Ying Wen, Jun Wang, and Weinan Zhang. Agent exchange: Shaping the future of AI agent economics. _arXiv preprint arXiv:2507.03904_ , 2025c. 

- Likai Ye, Mengliang Li, Dehai Zhao, Jiamou Sun, and Xiaoxue Ren. Bridging solidity evolution gaps: An llm-enhanced approach for smart contract compilation error resolution. _arXiv preprint arXiv:2508.10517_ , 2025. 

- Zihang Yin, Bing Bai, Yang Liu, and Tongtong Cheng. Research on distributed node resource optimization mechanism for multi-agent systems combined with blockchain technology. In _2024 IEEE International Conference on Blockchain (Blockchain)_ , pp. 536–541. IEEE, 2024. 

- Chaojia Yu, Zihan Cheng, Hanwen Cui, Yishuo Gao, Zexu Luo, Yijin Wang, Hangbin Zheng, and Yong Zhao. A survey on agent workflow–status and future. In _2025 8th International Conference on Artificial Intelligence and Big Data (ICAIBD)_ , pp. 770–781. IEEE, 2025. 

- Shijing Yuan, Qingshi Zhou, Jie Li, Song Guo, Hongyang Chen, Chentao Wu, and Yang Yang. Adaptive incentive and resource allocation for blockchain-supported edge video streaming systems: A cooperative learning approach. _IEEE Transactions on Mobile Computing_ , 24(2):539–556, 2025. 

- Lijun Zhang, Lin Li, Wei Wei, Huizhong Song, Yaodong Yang, and Jiye Liang. Scalable constrained policy optimization for safe multi-agent reinforcement learning. _Advances in Neural Information Processing Systems_ , 37:138698–138730, 2024. 

- Xiaoli Zhang, Yiqiao Song, Yuefeng Du, Chengjun Cai, Hongbing Cheng, Ke Xu, and Qi Li. Smartupdater: Enabling transparent, automated, and secure maintenance of stateful smart contracts. _IEEE Transactions on Software Engineering_ , 51(4):1266–1283, 2025a. 

- Yao Zhang, Zijian Ma, Yunpu Ma, Zhen Han, Yu Wu, and Volker Tresp. Webpilot: A versatile and autonomous multi-agent system for web task execution with strategic exploration. In _Proceedings of the AAAI Conference on Artificial Intelligence_ , volume 39, pp. 23378–23386, 2025b. 

- Boyuan Zheng, Michael Y Fatemi, Xiaolong Jin, Zora Zhiruo Wang, Apurva Gandhi, Yueqi Song, Yu Gu, Jayanth Srinivasa, Gaowen Liu, Graham Neubig, et al. Skillweaver: Web agents can self-improve by discovering and honing skills. _arXiv preprint arXiv:2504.07079_ , 2025a. 

- Jingyi Zheng, Zifan Peng, Yule Liu, Junfeng Wang, Yifan Liao, Wenhan Dong, and Xinlei He. Gasagent: A multi-agent framework for automated gas optimization in smart contracts. _arXiv preprint arXiv:2507.15761_ , 2025b. 

- Lu Zhou, Abebe Diro, Akanksha Saini, Shahriar Kaisar, and Pham Cong Hiep. Leveraging zero knowledge proofs for blockchain-based identity sharing: A survey of advancements, challenges and opportunities. _Journal of Information Security and Applications_ , 80:103678, 2024. 

- Weiming Zhuang, Chen Chen, and Lingjuan Lyu. When foundation model meets federated learning: Motivations, challenges, and future directions. _arXiv preprint arXiv:2306.15546_ , 2023. 

- Mingchen Zhuge, Changsheng Zhao, Dylan Ashley, Wenyi Wang, Dmitrii Khizbullin, Yunyang Xiong, Zechun Liu, Ernie Chang, Raghuraman Krishnamoorthi, Yuandong Tian, et al. Agent-asa-judge: Evaluate agents with agents. _arXiv preprint arXiv:2410.10934_ , 2024. 

- Zhenhua Zou, Zhuotao Liu, Lepeng Zhao, and Qiuyang Zhan. Blocka2a: Towards secure and verifiable agent-to-agent interoperability. _arXiv preprint arXiv:2508.01332_ , 2025. 

- Aneesh Zutshi, Antonio Grilo, and Tahereh Nodehi. The value proposition of blockchain technologies and its impact on digital platforms. _Computers & industrial engineering_ , 155:107187, 2021. 

21 

