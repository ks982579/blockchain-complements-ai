6706 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

# Toward Transparent and Incentive-Compatible Collaboration in Decentralized LLM Multi-Agent Systems: A Blockchain-Driven Approach 

Minfeng Qi _, Member, IEEE_ , Tianqing Zhu _, Member, IEEE_ , Lefeng Zhang , Ningran Li _, Graduate Student Member, IEEE_ , Yu-an Tan _, Senior Member, IEEE_ , and Wanlei Zhou _, Life Fellow, IEEE_ 

_**Abstract**_ **—Large Language Models (LLMs) have enabled autonomous agents capable of complex reasoning and collaboration, yet coordinating such agents in decentralized environments remains challenging due to the lack of centralized control for aligning behavior and sustaining long-term cooperation. This paper addresses this challenge from a mechanism-design perspective and proposes a behavior-shaping incentive mechanism for decentralized LLM-based multi-agent systems. The mechanism explicitly models agent utility as a function of task rewards, capability mismatch, and workload, and couples short-term utility optimization with long-term trust formation through dynamic reputation updates and capability-weight adaptation. By jointly shaping task assignment probabilities, reputation evolution, and skill profiles over repeated interactions, the mechanism incentivizes cooperative behavior and discourages strategic misrepresentation or overload. To operationalize this incentive design in a trust-minimized setting, we implement a lightweight blockchain-based enforcement layer that provides verifiable identity binding, task assignment commitment,andimmutableloggingofincentive-relevantstatetransitions. We evaluate the proposed mechanism through 50-round simulations involving 20 agents and 100 tasks, using GPT-4-based agents integrated with Solidity smart contracts. The results demonstrate improved task success rates, stable utility distributions, and emergentagentspecialization,highlightingtheeffectivenessofincentivecentric design for trustworthy coordination among decentralized LLM agents.** 

_**Index Terms**_ **—Large language models, multi-agent systems, blockchain, smart contracts, and incentive mechanisms.** 

## I. INTRODUCTION 

ECENT progress in Large Language Models (LLMs) has **R** reshaped how we build intelligent systems. With strong 

Received18September2025;revised20December2025;accepted22January 2026. Date of publication 29 January 2026; date of current version 17 February 2026. This work was supported by the National Natural Science Foundation of China under Grant 62402008 and in part by the Macao Science and Technology Development Fund (FDCT) under Grant 0012/2025/RIC. Recommended for acceptance by Dr. Tong Wu. _(Corresponding author: Tianqing Zhu.)_ 

Minfeng Qi, Tianqing Zhu, Lefeng Zhang, and Wanlei Zhou are with the Faculty of Data Science, City University of Macau, Macau SAR 314100, China (e-mail: mfqi@cityu.edu.mo; tqzhu@cityu.edu.mo; lfzhang@cityu.edu.mo; wlzhou@cityu.edu.mo). 

Ningran Li is with the School of Computer and Mathematical Sciences, The University of Adelaide, Adelaide, SA 5005, Australia (e-mail: ningran.li@adelaide.edu.au). 

Yu-an Tan is with the Department of Computer Science and Engineering, Beijing Institute of Technology, Beijing 100811, China (e-mail: tan2008@bit.edu.cn). Digital Object Identifier 10.1109/TNSE.2026.3659486 

abilities in reasoning, planning, and language understanding, LLMs have become the core of a new class of autonomous agents. These agents are now used in diverse areas such as scientific research, vulnerability detection, robotics, and complex problem solving [1], [2], [3], [4], [5], [6]. 

As tasks become more demanding, multi-agent systems (MAS) built on LLMs are emerging as a promising solution for scalable intelligence. By dividing work across multiple agents, MAS can support specialization and parallel execution that help tackle challenges beyond the reach of a single agent. Popular frameworks like MetaGPT [7], AutoGen [8], AgentVerse [9], AgentCoder [10] reflect this direction, enabling agents to plan, communicate, and coordinate. Yet, despite growing interest, a key question remains: How can we ensure that autonomous agents interact in a way that is _trustworthy_ , _transparent_ , and aligned with _incentives_ , especially in decentralized settings? 

Many existing LLM-MAS frameworks rely on strong assumptions. They often depend on centralized control, fixed communication rules, or trusted participants [11], [12], [13], [14]. These constraints make it difficult to apply such systems in open or adversarial environments, where agents may vary in design, pursue their own goals, or behave unpredictably. In particular, current frameworks face three major limitations: 

   - r _Opaque collaboration mechanisms:_ Many frameworks depend on off-chain coordination, where task assignment and communication happen outside any verifiable system [15]. This lack of transparency makes it hard to audit interactions or resolve disputes, especially in untrusted environments. 

   - r _Missing incentive structures:_ Without mechanisms for rewards or penalties, agents lack motivation to report their capabilities honestly or complete tasks with care [16]. This weakens collaboration, encourages poor task selection, and leads to unstable performance. 

   - r _Limited scalability:_ Fixed protocols and centralized controllers do not scale well as agent populations grow or environments change [17]. This creates bottlenecks in coordination and reduces the system’s ability to operate effectively in real-world, open settings. 

- Several recent studies have attempted to address these short- 

- comings through different design strategies, yet each remains limited in scope. DeCoAgent [18] introduces a decentralized task marketplace, where smart contracts manage agent 

2327-4697 © 2026 IEEE. All rights reserved, including rights for text and data mining, and training of artificial intelligence and similar technologies. Personal use is permitted, but republication/redistribution requires IEEE permission. See https://www.ieee.org/publications/rights/index.html for more information. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6707 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

registration, capability discovery, and task publication. However, its design emphasizes standardization rather than dynamic incentives. BlockAgents [19] takes a different direction by embedding a Proof-of-Thought consensus protocol into the collaboration workflow. While this protocol improves robustness against Byzantine behaviors, it does not fully address heterogeneous participation. LLM-Net [20] proposes a blockchainbased expert network in which specialized LLM providers are selected according to immutable reputation records maintained by validators. Although this approach highlights specialization, it assumes that reputation can serve as a tamper-resistant proxy for agent quality. Taken together, these systems provide useful starting points but still fall short of simultaneously ensuring transparency, incentive alignment, and scalability in open multiagent environments. 

_Our Approach:_ To address these challenges, we take a mechanism-design perspective and propose a _behavior-shaping incentive framework_ for decentralized LLM-based multi-agent systems. Rather than treating blockchain as a mere logging substrate, our approach focuses on how agent utilities, task allocation, and long-term trust can be jointly designed to align agent behavior in the absence of centralized control. The key insight is that sustained cooperation among autonomous LLM agents emerges not from communication alone, but from carefully structured incentives that couple short-term rewards with long-term reputation and capability evolution. 

At the core of our approach is an incentive mechanism that (i) assigns tasks probabilistically based on capability alignment, reputation, and workload, (ii) penalizes strategic overcommitment and capability mismatch through an explicit utility model, and (iii) continuously reshapes agent behavior via performancedriven reputation updates and capability adaptation over repeated interactions. This mechanism is designed to encourage truthful capability declaration, balanced participation, and specialization, while discouraging free-riding and opportunistic behavior. 

To operationalize this incentive design in a trust-minimized setting, we implement a lightweight blockchain-based enforcement layer. Smart contracts are used selectively to bind agent identities, commit task assignments, and record incentiverelevant state transitions, while computation-intensive operations such as LLM inference and message exchange remain off-chain. This separation allows the system to maintain transparency and auditability without treating blockchain as a highthroughput coordination substrate. 

To contextualize our design choices, Table I summarizes the key differences and trade-offs between existing decentralized LLM-MAS approaches and our incentive-centric framework. 

_Contributions:_ In summary, this work makes the following key contributions: 

➀ We formulate decentralized LLM-based multi-agent coordination as a _behavior-shaping incentive design_ problem and develop a concrete incentive mechanism that aligns agent behavior over repeated interactions through utility-based task allocation, performance-driven reputation updates, adaptive capability modeling, and workload-aware balancing, thereby encouraging cooperative behavior while discouraging strategic misrepresentation and overcommitment. 

TABLE I 

COMPARISON OF BLOCKCHAIN-ENABLED LLM-MAS 

**==> picture [252 x 188] intentionally omitted <==**

TABLE II 

COMPARISON OF TRADITIONAL MAS AND BLOCKCHAIN-ENHANCED MAS 

**==> picture [244 x 80] intentionally omitted <==**

➁ To operationalize the proposed incentive design in a trustminimized setting, we implement a lightweight blockchainbased enforcement layer (Section IV) that provides verifiable identity binding, task assignment commitment, and immutable logging of incentive-relevant state transitions. 

➂ We conduct extensive simulation-based evaluations (Section V) with heterogeneous agents, demonstrating improved task success rates, stable utility distributions, and emergent specialization under decentralized conditions. 

## II. PRELIMINARIES 

## _A. Blockchain Fundamentals_ 

Blockchain is a distributed ledger technology that achieves data transparency, immutability, and high security through cryptographic techniques and decentralized consensus mechanisms [21], [22], [23]. These core characteristics make blockchain a valuable infrastructure component for enhancing transparency in traditional MAS. In our proposed framework, blockchain serves several main purposes: ensuring the transparency of task allocation, enforcing incentive protocol through smart contracts, and enabling trustworthy communication among agents (as summarized in Table II). 

_Blockchain data structure:_ The blockchain is organized as a linear, tamper-resistant sequence of blocks, where each block is linked to its predecessor via a cryptographic hash of the previous block’s header [24], [25]. The hash of a block satisfies 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6708 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

TABLE III 

EXPERIMENTAL ENVIRONMENT CONFIGURATION 

the following relationship: 

**==> picture [158 x 11] intentionally omitted <==**

where _H_ prev denotes the hash of the previous block, enabling the chain-like linkage; _T_ is a timestamp indicating when the block was created; _M_ root is the Merkle root summarizing all transactions within the block; and _Nonce_ is a variable adjusted in proof-of-work (PoW) consensus to satisfy specific hash difficulty requirements. Any unauthorized alteration of block data will change _H_ block, thus invalidating the chain’s integrity. 

Blockchain uses a Merkle tree to organize transactions within each block [26]. A Merkle tree is a binary tree in which leaf nodes contain the hash of individual transactions, and non-leaf nodes contain hashes derived from their children: 

**==> picture [116 x 11] intentionally omitted <==**

where _M_ left and _M_ right represent the hash values of the left and right child nodes, respectively. The Merkle root _M_ root, stored in the block header, enables secure verification of the inclusion of a specific transaction in the block. For instance, to verify whether a transaction is part of a block, one needs only to check if the computed hash path from the transaction up to the Merkle root matches the root recorded in the block header. 

_On-chain and off-chain storage:_ In practice, storing all data directly on-chain is inefficient and costly [27]. Therefore, blockchain-based systems often adopt a hybrid storage model that combines on-chain and off-chain storage. Off-chain data is typically managed using distributed file systems such as the InterPlanetary File System (IPFS) or conventional databases. The integrity of off-chain data is ensured by recording its cryptographic hash on-chain [28]. This enables verification via the following condition: 

**==> picture [251 x 25] intentionally omitted <==**

This mechanism ensures that any tampering with off-chain data can be detected by a mismatch in its hash, thereby maintaining the verifiability and trustworthiness of externally stored information. 

Fig. 1. LLM-MAS Closed-Loop Architecture. The agent continuously interactswiththeenvironmentthroughfourmajorcomponents:(i) _PerceptionModule_ senses raw inputs from the environment; (ii) _Brain Module_ manages reasoning, knowledge, and planning; (iii) _Action Module_ executes decisions that affect the environment; and (iv) updated environmental states are fed back into perception, forming a closed loop. 

## _B. Multi-Agent Systems_ 

MAS are composed of multiple autonomous agents that interact within a shared environment to accomplish individual or collective objectives [29], [30]. These agents engage in a continuous cycle of perception, decision-making, and action, allowing them to adapt to dynamic contexts, cooperate or compete with other agents, and address complex problems beyond the capacity of a single agent [31], [32] (as illustrated in Fig. 1). 

_Perception:_ Perception refers to the agent’s ability to sense and interpret environmental signals. This process transforms raw inputs (such as text queries, sensor data, or visual information) into structured internal representations that guide subsequent reasoning [33]. In the case of LLM agents, this often involves parsing natural language input, extracting relevant entities or instructions, and mapping them into task-relevant states or semantic structures. 

_Cognition and decision-making:_ At the core of each agent lies a reasoning module, often referred to as its cognitive engine or “brain.” This component handles information processing, memory management, contextual reasoning, and task planning [34], [35]. It stores relevant knowledge, processes new inputs in light of that knowledge, and decides on appropriate actions. Reasoning can be implemented via predefined rules, symbolic logic, or data-driven models such as reinforcement learning. LLMs in particular provide strong generalization capabilities, allowing agents to respond flexibly to previously unseen instructions. 

_Action:_ After processing environmental input and selecting a course of action, the agent performs an action that either affects the external environment or influences other agents. This action may take various forms, including issuing API calls, generating responses, altering internal state, or sending structured messages [36]. The outcome of an action changes the environment, thereby initiating a new cycle of perception. 

_Environment:_ In MAS, the environment represents the external context with which agents continuously interact [37]. For LLM-based agents, this environment can be physical, such as a robotic workspace; digital, such as web APIs and databases; or interaction-specific, such as a user’s intent or a conversational 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6709 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

history [38]. The environment serves as both the source of stimuli for agent perception and the target of agent actions. 

_Agent loop:_ This perception-decision-action process forms a continuous loop that governs the agent’s behavior. As the environment evolves, agents re-perceive, re-evaluate, and react, enabling adaptation to dynamic conditions and coordination with other agents [39]. This loop is particularly important in collaborative settings where agent behaviors must remain synchronized with environmental changes and peer activity. 

## III. BEHAVIOR-SHAPING INCENTIVE MECHANISM 

As a central innovation of our framework, we design a behavior-shaping mechanism (Alogrithm 1) that couples shortterm utility optimization with long-term trust shaping. Specifically, agent behavior is dynamically regulated through a protocol that (i) computes utilities based on reward, workload, and capability mismatch, (ii) updates reputation scores via performancesensitive smoothing, and (iii) adjusts capability weights according to task-specific outcomes. By integrating these components, the mechanism not only rewards high-quality task execution but also penalizes poor performance, thereby steering agents toward cooperative behavior over interactions. 

_Mechanism-design perspective:_ We emphasize that this module is not merely an implementation component of the system, but a mechanism design for decentralized agent collaboration in the economic and game-theoretic sense. In our setting, each agent is a self-interested decision-making entity that strategically chooses which tasks to pursue and how much effort to exert under limited capacity. The mechanism shapes these strategic choices through an explicit utility structure and repeatedinteraction feedback: it aligns individual incentives with systemlevel objectives by coupling immediate payoffs with future selection advantages induced by reputation and capability updates. The blockchain layer serves as an enforcement substrate that makes the mechanism verifiable and tamper-evident (e.g., immutable logs and auditable updates). 

## _A. Utility-Based Incentive Function_ 

In our framework, each participating entity is modeled as an autonomous agent operating in a decentralized environment. Let _A_ = _{A_ 1 _, A_ 2 _, . . . , An}_ denote the set of all agents. Each agent _Ai ∈A_ is characterized at time step _t_ by a profile consisting of three core components: its reputation score _ρ[t] i[∈]_[[0] _[,]_[ 1]][,][a] capability vector **w** _i[t]_[= (] _[w][i,]_[1] _[, . . . , w][i,d]_[)] _[ ∈]_[R] _[d] ≥_ 0[, and a workload] level _ℓ[t] i[∈]_[N][. The reputation score reflects the agent’s historical] performance quality, the capability vector encodes proficiency across _d_ skill domains, and the workload level records the number of active or pending tasks assigned to the agent. 

Tasks are periodically issued into the environment. Each task _Tj_ is defined by a reward value _Rj >_ 0, a required capability vector **r** _j ∈{_ 0 _,_ 1 _}[d]_ , and an expected execution window or deadline _τj_ . The binary capability vector **r** _j_ specifies which skill dimensions are required for completing the task, i.e., _rj,k_ = 1 indicates that skill _k_ is necessary. 

**Algorithm 1:** Behavior-Shaping Incentive Mechanism for Decentralized Agents Coordination. 

**==> picture [226 x 277] intentionally omitted <==**

To allocate tasks effectively, the system defines a task assignment policy _πi[t]_[(] _[T][j]_[)] _[ ∈]_[[0] _[,]_[ 1]][ (9)][, which represents the probability] of assigning task _Tj_ to agent _Ai_ at time _t_ . This assignment probabilityisderivedfromthescoringfunction (8) thatconsiders the agent’s reputation, capability match, and current workload. 

The expected utility that agent _Ai_ receives at time _t_ when being evaluated for task _Tj_ is defined as: 

**==> picture [191 x 13] intentionally omitted <==**

where _πi[t]_[(] _[T][j]_[)] _[ ·][ R][j]_[denotes][the][expected][reward][the][agent][may] earn, and _c[t] i_[(] _[T][j]_[)][ denotes the cost of executing the task. This cost] function encapsulates both the agent’s current workload and the degree of mismatch between its skills and the task requirements. Formally, we define: 

**==> picture [195 x 14] intentionally omitted <==**

where _β >_ 0 and _γ >_ 0 are system-defined coefficients, _ℓ[t] i_[is] the current workload, and **w** ˆ _i[t][∈{]_[0] _[,]_[ 1] _[}][d]_[is][the][binarized][capa-] bility vector derived from **w** _i[t]_[by thresholding, i.e.,] _[w]_[ˆ] _i,k[t]_[= 1][ if] _wi,k[t][≥][θ]_[, and 0 otherwise. The] _[ L]_[1][ norm] _[ ∥]_ **[r]** _[j][−]_ **[w]**[ˆ] _i[t][∥]_[1][ quantifies] the number of required skills that the agent lacks. 

Thus, based on (2) and (1), the agent’s utility function becomes: 

**==> picture [253 x 55] intentionally omitted <==**

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6710 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

(i.e., a smaller skill mismatch). In contrast, agents are discouraged from accepting tasks when they are overloaded ( _ℓ[t] i_[is large)] or underqualified (i.e., many components of **r** _j_ are not satisfied by **w** ˆ _i[t]_[).] 

The incentive structure is thus designed to promote truthful reporting of capabilities and discourage strategic misrepresentation. Since utility is explicitly penalized by capability mismatch and task overload, agents have no rational incentive to overstate their abilities or accept tasks beyond their capacity. Moreover, as will be demonstrated in subsequent sections, this utility function integrates naturally with our dynamic reputation update rule and capability-weight learning mechanism, forming a self-regulating incentive structure that rewards cooperative and capable behavior over repeated interactions. 

## _B. Reputation Update Rule_ 

To incentivize high-quality behavior and maintain long-term agent accountability, our framework integrates a dynamic reputation update mechanism. Each agent _Ai ∈A_ maintains a scalar reputation score _ρ[t] i[∈]_[[0] _[,]_[ 1]][, which is updated over time based on] its observed performance across assigned tasks. This reputation score plays a central role in influencing future task assignment probabilities and serves as a trust indicator in agent selection procedures. 

Let _Si[t][∈]_[[0] _[,]_[ 1]][denote][the][performance][score][that][agent] _[A][i]_ receives upon completing a task at time step _t_ . This score is computed based on the task outcome, and can be determined either through automated metrics (e.g., accuracy, latency) or feedback from the task issuer. To ensure stability and gradual adaptation, wedefinethereputationupdateruleusinganexponentialmoving average (EMA) as follows: 

**==> picture [185 x 13] intentionally omitted <==**

where _λ ∈_ [0 _,_ 1) is a smoothing coefficient that determines how strongly past reputation influences future values. A higher value of _λ_ yields more inertia (slower change), while a lower _λ_ allows faster responsiveness to recent performance. 

This update rule ensures that an agent’s reputation gradually converges toward its average task performance. Since _Si[t][∈]_[[0] _[,]_[ 1]] and _ρ[t] i[∈]_[[0] _[,]_[ 1]][,][the][update][equation][preserves][boundedness:] _ρ[t] i_[+1] _∈_ [0 _,_ 1] for all _t_ . The EMA design naturally implements a forgetting mechanism, allowing agents to recover from past errorsovertimewhilestillpenalizingconsistentlypoorbehavior. 

The performance score _Si[t]_[is computed as a weighted aggre-] gation of multiple task-specific indicators: 

**==> picture [180 x 12] intentionally omitted <==**

where _qi[t][∈]_[[0] _[,]_[ 1]][is][the][task][quality][score][(e.g.,][correctness][or] user rating), _d[t] i[∈]_[[0] _[,]_[ 1]][ is a normalized delay ratio (actual time] divided by deadline), and _α_ + _δ_ = 1 are weighting coefficients. This formulation penalizes delayed or substandard executions and rewards timely, high-quality completions. 

A key property of the reputation rule is that it aligns with the agent’s utility function _Ui[t]_[(3)][. Since task allocation probabilities] _πi[t]_[are][influenced][by] _[ ρ][t] i_[,][maximizing][long-term][utility requires] agents to improve their reputation. Let the expected cumulative 

utility over a horizon _T_ be denoted by: 

**==> picture [174 x 30] intentionally omitted <==**

Substituting the utility expression from the previous section and noting that _πi[t][∝][ρ][t] i_[, it becomes clear that maximizing][ E][[] _[U][i]_[]] requires maintaining a high _ρ[t] i_[across] _[t]_[.][This][creates][a][natu-] ral incentive for rational agents to pursue good behavior over time. 

## _C. Capability Weight Adjustment_ 

In addition to reputation, each agent _Ai ∈A_ maintains a dynamic capability vector _**w**[t] i[∈]_[[0] _[,]_[ 1]] _[K]_[, where each element] _[ w] i,k[t]_ quantifies the agent’s expertise or reliability in a specific skill dimension _k ∈{_ 1 _,_ 2 _, . . . , K}_ . These capability weights serve as fine-grained indicators that complement the scalar reputation score and facilitate task-specific matching in heterogeneous multi-agent environments. 

Let Tag _k_ denote the _k_ -th capability tag in the global capability taxonomy _T_ = _{_ Tag1 _,_ Tag2 _, . . . ,_ Tag _K}_ . The value _wi,k[t][∈]_[[0] _[,]_[ 1]][indicates][the][proficiency][level][of][agent] _[A][i]_[on][tag] Tag _k_ at time _t_ . Initially, each agent declares a self-assessed capability vector _**w**_[0] _i_[during][registration.][However,][the][system] dynamically updates these values based on the agent’s historical task performance in each capability domain. 

For each completed task _Tj_ with required tag set _Tj ⊆T_ , the agent receives a capability-specific performance score _s[t] i,k[∈]_ [0 _,_ 1] for each tag Tag _k ∈Tj_ . This local score reflects how well the agent performed the sub-tasks associated with that capability dimension. The weight update rule is again modeled using exponential smoothing: 

**==> picture [240 x 15] intentionally omitted <==**

where _μ ∈_ [0 _,_ 1) is the capability parameter that governs the sensitivity to new observations. Tags that are not involved in the task (Tag _k ∈T/ j_ ) retain their previous weights: _wi,k[t]_[+1][=] _[ w] i,k[t]_[.] The use of tag-specific smoothing allows agents to improve (or degrade) their perceived skill levels over time in a way that is tightly aligned with actual task performance. Agents who consistently perform well in certain domains will see corresponding capability weights increase, making them more likely to be selected for similar tasks in the future. Conversely, agents with poor or unstable performance in a domain will experience weight decay, reducing their assignment likelihood for those tags. 

## _D. On-Chain Enforcement Via Smart Contracts_ 

To ensure verifiability and autonomy of coordination among agents, our framework leverages on-chain enforcement through smart contracts. All state transitions, including agent registration, task assignment, behavioral logging, performance evaluation, and reputation updates, are conducted on-chain to benefit from blockchain’s immutability and auditability. 

Each smart contract maintains the relevant state variables and enforces lifecycle transitions via publicly verifiable transactions. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6711 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

Fig. 2. System architecture of blockchain-based LLM multi-agent collaboration. The system is structured into three logical layers. The _Off-chain Layer_ handles user interactions, LLM capability extraction, storage, and performance evaluation. The _Blockchain Layer_ supports agent registration, task allocation, execution logging, and incentive mechanisms via smart contracts deployed on Ethereum. The _Agent Layer_ consists of autonomous agents operating in a decentralized environment, each maintaining reputation, capability, load, and public key information. 

When an agent _Ai_ joins the system, its declared capabilities _Ci_ , public key, and initial reputation _ri_[0][are][stored][through] the invocation of the registerAgent function. All agent-specific operations thereafter, such as authentication, workload updates, and reputation adjustments, are tracked and enforced through designated contract interfaces. 

Tasks _Tj_ are registered on-chain via submitTask, which stores the task’s metadata, including capability requirements requiredTags _j_ and execution constraints. The assignment policy _πi[t]_[(] _[T][j]_[)][ is computed on-chain by evaluating agent scores based] on capability match, current load, and historical reputation. The selected agent _Ai_ is recorded through a state update and confirmed by emitting a TaskAssigned event. 

During task execution, all behavioral evidence, including task acceptance, intermediate updates, and result submission, is reported using the logAction interface. Each log entry binds a specific action to an agent-task pair, forming a verifiable tuple _⟨Ai, Tj,_ action _, t⟩_ . These logs serve as audit trails. 

Upon task completion, performance evaluation and reputation updates are enforced on-chain via the updateReputation function. The function takes the task score TS _i,j_ as input and computes the updated reputation _ri[t]_[+1] using a predefined rule. This ensures that all reputation transitions are transparent and irrevocable. 

## IV. SYSTEM DESIGN 

To enable transparent and autonomous coordination among LLM-based agents, we implement the proposed behaviorshaping incentive mechanism (Section III) through a smart contract-driven interaction protocol (cf. Fig. 2). In this section, we briefly describe the supporting system components that make the mechanism executable in a decentralized setting. 

The protocol contains three functional modules: (1) agent registration and authentication, (2) communication and messaging, (3) decentralized task allocation. 

## _A. Registration and Authentication Module_ 

This module binds each agent to a verifiable cryptographic identity and restricts participation to registered agents, providing the basis for accountable reputation and capability tracking. 

_Registration:_ Each agent generates a local public–private key pair and registers by calling registerAgent, submitting _AgentID_ , _PublicKey_ (using secp256k1), _Role_ , _Capabilities_ (tags), and an optional initial _Reputation_ . The contract validates uniqueness and input format, stores metadata in a mapping indexed by the agent address, initializes the workload counter, and emits an AgentRegistered event. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6712 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

**==> picture [243 x 151] intentionally omitted <==**

Listing 1: Standard Message Structure. 

_Authentication and eligibility checks:_ To participate in interactions, an agent proves ownership of its key via signature-based challenge by invoking authenticate; the contract verifies the signature via ecrecover against the registered address. For taskrelated operations, the contract enforces a lightweight capability gate via verifyCapabilities, checking whether the agent’s registered tags satisfy the task’s required tags. 

ensuring that higher-scoring agents are more likely to be selected while still allowing exploration. 

_Task representation and updates:_ Tasks are described in natural language and mapped to required capability tags _Cj_[req] by an LLM-based capability extractor. After assignment, the selected agent’s workload _ℓ[t] i_[is updated and the task state is recorded for] auditability. Upon completion, task outcomes trigger reputation and capability updates as specified in Section III, closing the feedback loop. 

_Optional decomposition:_ For tasks exceeding a complexity threshold _θc_ , tasks can be decomposed into subtasks with inherited capability requirements, and the same allocation rule is applied per subtask. In our implementation, task complexity is measured using a lightweight structural metric derived from the task description, defined as a weighted combination of (i) the number of required capability tags, (ii) the estimated number of atomic action steps inferred from the instruction, and (iii) the presence of explicit sequential dependencies (e.g., conjunctions such as “and then”). Formally, a task is marked as complex if Complexity( _Tj_ ) _> θc_ , where Complexity( _·_ ) is computed offchain during task parsing. 

## V. EXPERIMENTAL SETUP 

## _A. System Implementation_ 

## _B. Communication and Messaging Module_ 

Communication provides the evidence and coordination signalsneededfordecentralizedcollaboration.Messagesaresigned by senders to ensure authenticity, and timestamps help mitigate replay attempts and support basic ordering. 

_Hybrid logging:_ To balance auditability with cost, we adopt hybrid logging: message hashes and essential metadata are anchored on-chain, while full payloads can be stored off-chain (e.g., IPFS) when needed. This design preserves verifiability without incurring excessive on-chain storage overhead. 

_Message schema:_ Messages follow a standardized schema that includes _sender_ , _receiver_ , _timestamp_ , _message_type_ , _message_body_ , and _signature_ (Listing 1). The unified schema enables automatic parsing and downstream validation (e.g., task eligibility checks and capability matching). 

## _C. Task Allocation Module_ 

Task allocation selects agents based on capability match, reputation, and current workload, thereby instantiating the assignment policy used by the incentive mechanism. Let _πi[t]_[(] _[T][j]_[)] _[ ∈]_ [0 _,_ 1] denote the probability that agent _Ai_ is assigned to task _Tj_ at time _t_ . We compute a utility-driven score: 

**==> picture [236 x 14] intentionally omitted <==**

where CapMatch( _Cj_[req] _[,]_ _**[ w]** i[t]_[) =] _|C_ 1 _j_[req] _[|]_ � _k∈Cj_[req] _[w] i,k[t]_[is][the][average] tag-wise capability weight over required capabilities. The assignment probability is computed by a softmax over eligible agents: exp( _Si[t] ,j_[)] _πi[t]_[(] _[T][j]_[) =] (9) ~~�~~ _Ak∈Aj_[exp(] _[S] k,j[t]_[)] _[,]_ 

Our system consists of three main components: (1) the LLMbased agent layer, (2) the smart contract coordination layer, and (3) the user-facing frontend. 

_(1) Agent layer:_ Each agent is instantiated as an independent process equipped with access to OpenAI’s GPT-4 API. Agents communicate via signed messages (using ECDSA over secp256k1) and locally maintain their reputation scores, capability profiles, and decision-making modules. 

_(2) Smart contract layer:_ The blockchain backend is implemented using the Ethereum Virtual Machine (EVM). All contract logic, including agent registration, task assignment, and reputation updates, is encoded in Solidity. We used Hardhat for development and deployed contracts on a local Ganache testnet to simulate realistic blockchain execution. 

_(3) Frontend and middleware:_ We implemented a lightweight React-based frontend that allows users to submit tasks, monitor agent performance, and visualize blockchain logs. A FastAPIbased backend mediates between agent behavior and smart contract calls, including transaction signing, gas estimation, and off-chain capability evaluation. 

The entire system was developed and tested on a machine with the following specifications (Table III): 

## _B. Simulated Task Environment_ 

_Task corpus and capability taxonomy:_ We derive our task set from a modified subset of the ALFRED benchmark [40], a well-known environment used in multi-agent learning and embodied task planning. ALFRED provides hierarchical natural language instructions grounded in realistic household environments, which are particularly suitable for simulating multi-skill task execution in distributed agent systems. For our purposes, 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6713 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

TABLE IV 

CAPABILITY TAG TAXONOMY USED FOR AGENT PROFILING 

**==> picture [226 x 126] intentionally omitted <==**

we extract 100 representative tasks involving both atomic goals (e.g., “pick up the book”) and composite objectives (e.g., “put the red mug on the kitchen table and turn off the lights”). 

Each task is automatically annotated with 2–4 relevant capability tags drawn from a curated global taxonomy _T_ = _{_ Tag1 _,_ Tag2 _, . . . ,_ Tag10 _}_ . This taxonomy captures diverse functional dimensions and serves as the semantic interface between task descriptions and agent skill profiles (as presented in Table IV). 

Each task _Tj_ is thus encoded as a binary capability requirement vector **r** _j ∈{_ 0 _,_ 1 _}_[10] , where each component _rj,k_ = 1 indicates that the task demands skill Tag _k_ . For example, a composite instruction such as “pick up the knife and place it on the stove” mayrequire Tag1 (objectrecognition), Tag4 (manipulation),and Tag5 (navigation). 

_Agent initialization and skill specialization:_ We simulate a population of 20 autonomous agents _A_ = _{A_ 1 _, A_ 2 _, . . . , A_ 20 _}_ , each configured with distinct capability vectors, reputation scores, and initial workload states. These agents emulate heterogeneous expertise profiles common in decentralized multi-agent systems. 

To construct the initial capability distribution, we assume that agents exhibit specialization rather than uniform proficiency. For each agent _Ai_ , we sample its continuous capability vector **w** _i_[0] _[∈]_[[0] _[,]_[ 1]][10][from a Beta distribution][ Beta][(] _[α]_[ = 2] _[, β]_[= 5)][.] This asymmetric distribution is skewed toward lower values, reflecting the intuition that most agents are only proficient in a few domains, while few are highly versatile. The choice of parameters _α_ = 2 _, β_ = 5 yields moderate sparsity and aligns with empirical patterns observed in expert population models. 

To enhance inter-agent comparability, all vectors are min-max normalized across each agent’s 10-dimensional skill profile. We then binarize each vector using a global threshold _θ_ = 0 _._ 4, resulting in a discrete skill declaration vector **w** ˆ _i_[0] _[∈{]_[0] _[,]_[ 1] _[}]_[10][,] where _w_ ˆ _i,k_[0][= 1][if][and][only][if] _[w] i,k_[0] _[≥][θ]_[.][This][binarized][rep-] resentation serves as the basis for capability matching during task allocation and closely mirrors real-world agent registration processes, where skill declarations are coarse-grained rather than continuous. 

Each agent is initialized with a neutral reputation score _ρ_[0] _i_[=] 0 _._ 5,reflectingalackofpriortaskperformancehistory.Thisvalue serves as the starting point for our exponential reputation update 

TABLE V 

TASK ALLOCATION METRICS (AVG OVER 50 ROUNDS) 

**==> picture [249 x 95] intentionally omitted <==**

process and ensures fairness in early-round allocations. To simulate asynchronous participation and limited concurrency, we assign initial workload levels _ℓ_[0] _i[∈{]_[0] _[,]_[ 1] _[,]_[ 2] _[}]_[ sampled uniformly] at random. 

Additionally, we store each agent’s public key and declared capability profile on-chain via the registerAgent contract function during system bootstrap. This guarantees that all subsequent identity verification, capability claims, and behavior logs are cryptographically verifiable and traceable within the immutable system ledger. 

_Simulation execution:_ The simulation spans 50 discrete time steps (referred to as rounds), each modeling a cycle of task issuance, agent bidding, execution, and state updates. 

At the beginning of each round _t ∈{_ 1 _, . . . ,_ 50 _}_ , the environment injects a new batch of 5 tasks _{T_ 1 _[t][, . . . , T][ t]_ 5 _[}]_[into][the] task queue. These tasks are selected from the ALFRED-derived benchmark pool and retain their annotated capability vectors **r** _j_ and reward values _Rj_ . The tasks are broadcast to all agents via the emitTaskBatch smart contract function and stored on-chain. Each agent _Ai ∈A_ evaluates its expected utility _Ui[t]_[(] _[T][j]_[)][ for] each task _Tj_ in the current batch. This evaluation incorporates three elements: (i) the agent’s current workload _ℓ[t] i_[, (ii) capabil-] ˆ ity alignment (as measured by _∥_ **r** _j −_ **w** _i[t][∥]_[1][),][and][(iii)][the][task] assignment policy _πi[t]_[(] _[T][j]_[)][, as defined in][ (9)][. The resulting utility] values are thresholded, and agents only bid for tasks where _Ui[t]_[(] _[T][j]_[)] _[ >]_[ 0][, ensuring rational bidding behavior.] 

Agents submit bids through the submitBid function, attaching a signed message that includes: the agent’s identifier, the task ID, the binarized capability vector **w** ˆ _i[t]_[,][the][computed][utility,][and][a] digital signature using the agent’s private key. The smart contract receives all bids and executes the assignment procedure on-chain by invoking the scoring-based task assignment policy. 

Once assignments are finalized, each selected agent proceeds to “execute” its task in a simulated performance environment. Task outcome is probabilistically determined by a ground-truth simulator, where the success probability _p_ success _∈_ [0 _,_ 1] depends on: 

**==> picture [242 x 13] intentionally omitted <==**

where _η ∈_ (0 _,_ 1] is a base success rate coefficient, _ζ ∈_ [0 _,_ 1] penalizes overload, and CapMatch computes the normalized overlap between required and capabilities. The result is a binary task success flag (1 = success, 0 = failure), which drives 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6714 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

Fig. 3. Performance trends across 50 simulation rounds for multiple task allocation metrics in LLM-based multi-agent systems. Each subplot visualizes actual measurements and a fitted linear trend line with its _R_[2] value to show consistency and correlation. 

subsequent updates to the agent’s reputation _ρ[t] i_[and][capability] weights _wi,k[t]_[, based on the update rules detailed in Section][ III][.] All major system actions, including emitTaskBatch, submitBid, logAction, and updateReputation, are implemented as Ethereum, compatible Solidity smart contracts and executed over a local private Ethereum network (based on Hardhat + Ganache). Each action emits a corresponding blockchain event with encoded parameters, enabling full auditability via standard tools like Web3 log parsers. 

## VI. RESULTS AND ANALYSIS 

## _A. Task Allocation Efficiency_ 

To assess the performance of our proposed task allocation mechanism, we conducted a simulation of 50 rounds of interactions between 100 tasks and 20 agents. The metrics reported in Table V reflect task completion success, quality, agent utility, and load balancing behavior. 

_Task success rate (cf. Fig. 3(a)):_ Across 50 simulation rounds, the average task success rate reached **86.77%** , with an initial value of 80.15% in early rounds that improved to 94.49% in later rounds. This increasing trend illustrates the positive feedback loop built into the system: agents with higher reputations and more refined capabilities were prioritized in task selection, resulting in more successful executions over time. 

_Mean task quality (cf. Fig. 3(b)):_ The mean task quality score TS _j,i_ , derived from both correctness and delay metrics, steadily improved from an initial average of 0.75 to a final value of 0.89, with an overall mean of **0.82** . This shows that tasks were increasingly handled by agents with better skill alignment. We observed a decline in task quality standard deviation from 0.061 to 0.030 across the simulation horizon, indicating reduced variability in performance. This trend demonstrates that the system successfully discouraged suboptimal task-agent 

matches, as poorly performing or overloaded agents were deprioritized in the assignment process. 

_Capability match score (cf. Fig. 3(c)):_ The capability match score, which quantifies how well assigned agents align with the required skills of each task, exhibited steady improvement over the 50-round simulation. In early rounds, the average capability match score hovered around 0.68–0.70, indicating moderate alignment with some degree of skill mismatch. However, as agents refined their bidding strategies and specialized in highreward domains, this score improved noticeably. By round 25, the average capability match rose to approximately 0.735, and by round 50, it reached 0.765, with a maximum round value of **0.794** . This upward trend suggests that agents increasingly learned to bid selectively for tasks that matched their strongest skill domains. 

_Agent utility (cf. Fig. 3(d)):_ The average agent utility per round reached **5.02** , increasing from 3.52 in the early simulation to 6.43 in the final rounds. This gain reflects the reinforcement of incentive compatibility: agents that reported capabilities truthfully and maintained high reputations were rewarded with higher-value task allocations and better-matched workloads. 

_Load distribution (cf. Fig. 3(e)):_ Throughout the simulation, we monitored the system’s ability to balance load across the agent population. In the early rounds, a few agents dominated task execution due to favorable initial capabilities. However, as the reputation system evolved, task allocation became more evenly distributed across eligible agents, avoiding persistent overload on specific individuals. The final rounds showed smoother workload curves across agents. 

_Task allocation delay (cf. Fig. 3(f)):_ The average delay between task issuance and final assignment was approximately 0.99 rounds, with variation between 0.43 and 1.56. Initial delay in Round 1 (1.14) reflects the system’s cold start and lack of optimized matching. Over time, delays decreased, with mid- 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6715 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

and late-stage rounds (e.g., Round 25 and 50) showing improved responsiveness (0.58 and 0.77 respectively). This confirms that our on-chain task assignment policy reduces latency as the system evolves. 

## _B. Reputation and Capability Evolution_ 

To evaluate the impact of our incentive and learning mechanisms on agent behavior, we analyze the evolution of reputation scores _ρ[t] i_[and capability entropy over 50 simulation round. These] metrics respectively reflect agent reliability and skill specialization dynamics. 

_Reputation dynamics:_ Contrary to sharp bifurcation or polarization, agent reputation scores demonstrated relatively stable and bounded evolution. By round 50, the mean final reputation across the 20 agents was 0 _._ 488 (Std = 0 _._ 044), with the highest and lowest values being 0 _._ 558 and 0 _._ 429, respectively. No agents crossed the high-performance threshold ( _>_ 0 _._ 8), nor did any fall below the critical underperformance boundary ( _<_ 0 _._ 3). This convergence suggests a moderate equilibrium state under our reward-compatible mechanism, potentially resulting from balanced task distribution and relatively homogenous agent behavior. Fig. 4(a) visualizes these dynamics, showing bounded but individualized reputation trajectories. 

_Capability specialization:_ Capability vectors exhibited similarly stable characteristics. The entropy of each agent’s skill weight vector _**w**[t] i_[was monitored to assess specialization tenden-] cies. At initialization, agents presented a mean entropy of 2 _._ 31 bits. After 50 rounds, the mean entropy remained consistent at 2 _._ 315 bits (Std = 0 _._ 073), with no agent dropping below the specialization threshold of 1 _._ 4 bits. These findings suggest that agents retained generalist profiles, with limited capability concentration. This behavior may be attributed to relatively uniform feedback across capabilities, indicating room for introducing stronger task-tag selectivity or differential reinforcement. Fig. 4(b) illustrates these entropy patterns. 

## _C. Agent Incentivization and Specialization_ 

To assess whether our incentive model drives rational agent behavior and task-domain specialization, we tracked key behavioral metrics over 50 simulation rounds, including task bidding behavior, utility trends, and tag-specific dominance distributions. 

_Task bidding behavior:_ At the beginning of the simulation, agentsexhibitedexploratorybehavior,submittingbidsfornearly allavailabletasksregardlessofskillmatch.Specifically,inround 1, the system recorded a global task bid rate of 92%, indicating that most agents evaluated potential utility as positive even for tasks with marginal capability alignment. However, as agents received feedback in the form of success scores, reputation updates, and capability weight adjustments, they became more discerning. By round 20, the global bid rate declined to 72.9%, and and further declined to 55% by round 40 and subsequent rounds. This drop reflects a learned strategy: agents increasingly avoided tasks for which they were underqualified or overloaded, thereby reducing unnecessary task contention and failure. The 

Fig. 4. Per-agent analysis of reputation and capability diversity. (a) shows the reputation variance, while (b) presents entropy as a proxy for multi-skill potential. 

downward trend in bid rate (see Fig. 5(a)) supports the hypothesis that agents adapted their bidding decisions based on accumulated experience and utility optimization. 

_Utility improvement:_ Our utility formulation, which penalizes capability mismatch and overload, encouraged agents to specialize and improve task selection. The average per-agent utility rose from 3.34 in round 1 to 7.0 by round 46, with standard deviation narrowing from 2.41 to 1.92, indicating that utility gains became more consistent across agents (see Fig. 5(b)). These gains were not due to increased task rewards (which remained fixed per dataset) but rather improved alignment between assigned tasks and the agents’ skill profiles. High-utility agents typically achieved both high capability match scores and stable reputations, resulting in more frequent task selection and reward accrual. 

_Emergence of specialization:_ A core effect of our feedbackdriven capability adjustment rule is the emergence of domain specialization. We measured each agent’s dominant capability tag based on their final capability weight vector and aggregated expert counts across all 10 tags. By round 50, a distinct clustering emerged: 4 agents (20%) specialized in Tag_2, and another 3 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6716 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

Fig. 5. Analysis of agent bidding behavior and capability specialization. (a) Task bid rate decreases over time, suggesting increased agent selectivity; (b) Mean utility steadily increases, indicating improved task-agent alignment; (c) Polar plot illustrates the distribution of dominant capability tags among agents. 

Fig. 6. Overview of blockchain-layer metrics during LLM-MAS operation: (a) number of events emitted per round, (b) transaction throughput and gas usage trends, (c) confirmation latency over time, and (d) failure rate under operational uncertainty. 

agents (15%) became dominant experts in Tag_7. These two domains together accounted for 35% of all specialists, confirming a self-organized concentration of expertise driven by positive reinforcement. Other tags also saw expert accumulation (e.g., Tag_4, Tag_9). The final distribution of tag dominance is visualized in Fig. 5(c). 

TABLE VI 

GAS USE AND ESTIMATED EXECUTION COST OF CORE CONTRACT FUNCTIONS 

## _D. On-Chain Performance and Smart Contract Analysis_ 

To assess the performance of our blockchain-based execution framework, we collected and analyzed key operational metrics over 50 simulation rounds. These include transaction throughput, gas usage, confirmation latency, failure rate, and event emission patterns. 

_Transaction throughput and gas consumption (cf. Fig. 6(a)):_ On average, the system processed approximately 67.32 transactions per round, covering agent registration, task submission, assignment, behavioral logging, and reputation or capability updates. The average gas consumption per transaction was 147,865 units, with values ranging from 130,599 to 170,391 units (Table VI). To make these figures more tangible for readers outside the blockchain community, we additionally report an approximate real-world cost estimate under a representative low-congestion setting, assuming a gas price of 5 Gwei and an ETH price of $2,000 (illustrative values). Under this assumption, incentive-critical operations such as task assignment and reputation updates incur execution costs on the order of $0.5–$1 per call. While these costs are substantially lower than peak mainnet conditions, they nevertheless represent a non-negligible economic overhead, which motivates our design 

choice to restrict blockchain usage to incentive-relevant state transitions rather than fine-grained agent coordination. We note that actual deployment costs depend on network conditions and market prices, and the reported values should be interpreted as order-of-magnitude estimates rather than fixed costs. 

_Confirmation time and responsiveness (cf. Fig. 6(b)):_ The average confirmation time across rounds was 2.18 seconds, with the fastest confirmations completing in 1.5 seconds and the slowest at 3.19 seconds. These low-latency results suggest that the system remains responsive under moderate agent-task traffic and is suitable for real-time applications where timely task allocation is essential. 

_Fault tolerance and event emission (cf. Fig. 6(c) & (d)):_ The average number of failed transactions per round was low at 0.86, with no rounds exceeding 3 failures. This indicates a high reliability of the execution pipeline and proper validation of agent inputs before on-chain invocation. The average number 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6717 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

of emitted events per round was 106.82, which includes task broadcasts, bids, assignments, and logging actions. These logs serve as verifiable traces and enable external auditors to reconstruct full histories from immutable ledger records. 

_Discussion: scalability and economic feasibility:_ While blockchain-based enforcement provides strong guarantees of transparency, auditability, and trust minimization, it also introduces inherent limitations in terms of transaction cost and throughput. Our experimental results indicate that incentiverelevant operations such as task assignment, behavioral logging, and reputation or capability updates incur non-trivial gas consumption, even with optimized contract logic. This observation highlights a fundamental tension: deploying all coordination logic fully on-chain would be economically infeasible for largescale agent collaboration under current public blockchain pricing models. Accordingly, the reported gas costs should not be interpretedasevidencethatblockchainoffersacost-freesolution to decentralized coordination, but rather as a quantification of the overhead required to maintain verifiable incentive enforcement. 

Toaddressthislimitation,ourframeworktreatstheblockchain as a selective enforcement layer rather than a high-throughput execution substrate. Only state transitions that affect long-term incentives and accountability (e.g., task assignment finalization, performance evidence anchoring, and reputation or capability updates) are committed on-chain, while computation-intensive operations such as LLM inference, fine-grained communication, and intermediate reasoning are executed off-chain. This hybrid design significantly reduces on-chain load while preserving the transparency necessary for incentive alignment. Nevertheless, scalability remains an inherent constraint: as the number of agents or tasks grows, cumulative on-chain costs may still become prohibitive if updates are performed too frequently. Potential mitigation strategies include batching incentive updates, periodic settlement of reputation changes, or deployment on lower-cost execution environments such as layer-2 rollups or permissioned blockchains, which we leave as important directions for future work. 

## VII. RELATED WORK 

## _A. LLM-Based Multi-Agent Systems_ 

_General-purpose coordination frameworks:_ Recent years have witnessed a surge of interest in LLM-based multi-agent systems (LLM-MAS), driven by the observation that groups of specialized agents can outperform monolithic models on complex tasks. Early frameworks such as AutoGPT, BabyAGI, and LangGraphexploredagentchainingandtool-augmentedreasoning. Subsequent systems formalized agent roles and workflows. MetaGPT [7], ChatDev [41], and AgentVerse [9] adopt rolebased or organization-inspired structures to coordinate agents in software engineering tasks. AgentLite [42] and DyLAN [43] emphasize modularity and dynamic agent selection, enabling runtime adaptation. 

Other works investigate alternative coordination paradigms. SoT [44] accelerates reasoning through parallel skeleton construction. DMAS [45] adopts a decentralized peer-to-peer topology, while ACORM [46] relies on a centralized controller. 

CAMEL [47] demonstrates how structured prompts alone can induce cooperative behavior. CORY [48] integrates multi-agent reinforcement learning to co-evolve agent policies. 

_Domain-specific agent collectives:_ Beyond general frameworks, LLM-MAS have been deployed in domain-specific settings. In software engineering, multi-agent systems automate coding, testing, security analysis, and penetration testing through specialized agents [49], [50], [51], [52]. In robotics and embodied AI, LLM agents are combined with low-level controllers to enable planning, navigation, and manipulation [53], [54], [55], [56], [57]. In scientific discovery, agent collectives assist with hypothesis generation, experiment planning, and structured debate [58], [59], [60]. 

_Limitations:_ Despite impressive empirical results, most existing LLM-MAS operate in closed or semi-centralized settings. Trust assumptions are implicit, agent behavior is largely guided by prompt engineering or heuristics, and there is little support for verifiableaccountabilityorincentive-compatiblecoordinationin open environments. 

## _B. Incentive and Alignment Mechanisms for LLMs_ 

_Internal alignment via reward modeling:_ A large body of work studies how to align LLM behavior with human preferences through internal learning signals. RLHF remains the dominant paradigm, using learned reward models and reinforcement learning [61], [62]. More recent approaches such as DPO [63], [64] and its variants [65], [66] simplify optimization by embedding preferences directly into the training objective. Other lines of work refine reward granularity through processlevel rewards [67], hierarchical rewards [68], or multi-objective alignment schemes [69], [70]. 

These methods primarily focus on _single-model alignment_ during training or fine-tuning, rather than strategic interactions among multiple autonomous agents. 

_External incentives and game-theoretic modeling:_ When LLMs interact with other agents or humans, external incentives become critical. Game-theoretic analyses model LLMs as strategic actors in markets, communication games, or adversarial settings [71], [72], [73], [74]. Mechanism design has been applied to LLM-mediated auctions [75], data markets and fine-tuning services [76], [77], and peer-evaluation systems [78]. Self-play and adversarial training further explore how competition shapes behavior [79]. 

However, these studies typically analyze incentives _in isolation_ , without integrating them into a full multi-agent coordination loop with persistent reputation and task allocation. 

## _C. Lessons Learned_ 

- Based on the above literature, we distill several key lessons: 

- r _Coordination alone is insufficient:_ Existing LLM-MAS focus on communication, role assignment, or workflow design, but lack explicit incentive mechanisms to regulate strategic behavior in open settings. 

- r _Internal alignment does not address multi-agent incentives:_ Reward-model-based alignment techniques shape 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6718 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

- individual model behavior, but do not govern interactions among autonomous agents with competing objectives. 

- r _Game-theoretic incentives lack system integration:_ Prior incentive and mechanism design studies analyze specific interaction scenarios, yet rarely connect incentives to endto-end agent coordination, task allocation, and long-term trust. 

- r _Trust and accountability remain underexplored:_ Few LLM-MAS provide verifiable guarantees on agent identity, behavior logging, and incentive enforcement in decentralized environments. 

## VIII. CONCLUSION AND FUTURE WORK 

This work formulates decentralized LLM-based multi-agent coordination as a _behavior-shaping incentive design_ problem. Our primary contribution is an incentive mechanism that aligns agent behavior over repeated interactions through utility-based task allocation, performance-driven reputation updates, and adaptive capability modeling. Simulation results show that this mechanism promotes cooperative behavior, discourages strategic misreporting and overload, and leads to emergent specialization among agents. The blockchain layer functions as a supporting enforcement substrate that operationalizes these incentive rules in a transparent and verifiable manner. 

Several directions remain for future work. First, a more comprehensive economic analysis under dynamic pricing and strategic participation is needed. Second, extending the framework to adversarial or partially malicious agents would enable deeper game-theoretic investigation. Finally, large-scale deployment will require tighter off-chain integration and cost-aware enforcement mechanisms. 

## APPENDIX 

## TABLE VII 

NOTATION TABLE FOR MULTI-AGENT FRAMEWORK 

**==> picture [245 x 360] intentionally omitted <==**

penalty. This configuration ensures that agents are strongly discouraged from accepting tasks for which they lack required skills, while moderate workload accumulation is tolerated to allow parallelism. 

## _A. Notation Table_ 

Tofacilitateaclearunderstandingofthesystem’smechanisms and mathematical formulations, Table VII summarizes all the key symbols used throughout the paper. The notation covers core concepts such as agent capabilities, task properties, reputation dynamics, utility modeling, assignment probabilities, etc. We also include relevant smart contract function names and emitted events in the table. 

_Reputation and capability smoothing parameters:_ For the exponential update rules of reputation (4) and capability weights (7), we set the smoothing coefficients to _λ_ = 0 _._ 7 and _μ_ = 0 _._ 6, respectively. These values balance stability and adaptability: reputation evolves more conservatively to reflect long-term trust, whereas capability weights adapt more quickly to recent taskspecific performance. 

## _B. Hyperparameter Configuration_ 

To ensure reproducibility of our experiments, we list the key hyperparameters used in the incentive mechanism and task allocation process. Unless otherwise stated, all parameters are 

## REFERENCES 

- [1] T. Yang et al., “AutoHMA-LLM: Efficient task coordination and execution in heterogeneous multi-agent systems using hybrid large language models,” _IEEE Trans. Cogn. Commun. Netw._ , vol. 11, no. 2, pp. 987–998, Apr. 2025. 

_Task allocation scoring weights:_ In the task allocation scoring function (8),wesettheweightsto _λ_ 1 = 0 _._ 5 _, λ_ 2 = 0 _._ 3 _, λ_ 3 = 0 _._ 2, placing the highest emphasis on capability matching, followed by reputation, and then workload. This choice reflects the intuition that task–skill alignment is the primary determinant of execution quality, while reputation captures longer-term reliability and workload discourages agent overcommitment. 

- [2] L. Zhou et al., “Semantic information extraction and multi-agent communication optimization based on generative pre-trained transformer,” _IEEE Trans. Cogn. Commun. Netw._ , vol. 11, no. 2, pp. 725–737, Apr. 2025. 

- [3] Z.Yao,Z.Tang,W.Yang,andW.Jia,“EnhancingLLMQoSthroughcloudedge collaboration: A diffusion-based multi-agent reinforcement learning approach,” _IEEE Trans. Serv. Comput._ , vol. 18, no. 3, pp. 1412–1427, May/Jun. 2025. 

- [4] C. Sun, S. Huang, and D. Pompili, “LLM-based multi-agent decisionmaking: Challenges and future directions,” _IEEE Robot. Automat. Lett._ , vol. 10, no. 6, pp. 5681–5688, Jun. 2025. 

_Utility cost coefficients:_ In the cost function (2), we use making: Challenges and future directions,” _IEEE Robot. Automat. β_ = 0 _._ 1 for workload cost and _γ_ = 0 _._ 4 for capability mismatch vol. 10, no. 6, pp. 5681–5688, Jun. 2025. Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6719 

QI et al.: TOWARD TRANSPARENT AND INCENTIVE-COMPATIBLE COLLABORATION IN DECENTRALIZED LLM MULTI-AGENT SYSTEMS 

- [5] M. Qi, D. He, Q. Wang, and L. Zhang, “Viper strike: Defeating visual reasoning captchas via structured vision-language inference,” 2026, _arXiv:2601.06461_ . 

- [6] Z. Wei et al., “Advanced smart contract vulnerability detection via LLMpowered multi-agent systems,” _IEEE Trans. Softw. Eng._ , vol. 51, no. 10, pp. 2830–2846, Oct. 2025. 

- [7] S. Hong et al., “MetaGPT: Meta programming for a multi-agent collaborative framework.,” in _Proc. Int. Conf. Learn. Representations_ , 2024. [Online]. Available: https://openreview.net/forum?id=VtmBAGCN7o 

- [8] Q. Wu et al., “AutoGen: Enabling next-gen LLM applications via multiagent conversation framework,” 2023, _arXiv:2308.08155_ . 

- [9] W. Chen et al., “AgentVerse: Facilitating multi-agent collaboration and exploring emergent behaviors in agents,” in _Proc. 12th Int. Conf. Learn. Representations_ , 2024. [Online]. Available: https://openreview.net/forum?id= EHg5GDnyq1 

- [10] D.Huang,J.M.Zhang,M.Luck,Q.Bu,Y.Qing,andH.Cui,“AgentCoder: Multi-agent-basedcodegenerationwithiterativetestingandoptimisation,” 2023, _arXiv:2312.13010_ . 

- [11] T. Guo et al., “Large language model based multi-agents: A survey of progress and challenges,” in _Proc. 33rd Int. Joint Conf. Artif. Intell._ , 2024, pp. 8048–8057. 

- [12] K. Cinkusz and J. A. Chudziak, “Towards LLM-augmented multiagent systems for agile software engineering,” in _Proc. 39th IEEE/ACM Int. Conf. Automated Softw. Eng._ , 2024, pp. 2476–2477. 

- [13] W. Epperson et al., “Interactive debugging and steering of multi-agent ai systems,” in _Proc. 2025 CHI Conf. Hum. Factors Comput. Syst._ , 2025, pp. 1–15. 

- [14] L. Zhang, Y. Zhai, T. Jia, X. Huang, C. Duan, and Y. Li, “AgentFM: Roleaware failure management for distributed databases with LLM-driven multi-agents,” in _Proc. 33rd ACM Int. Conf. Found. Softw. Eng._ , 2025, pp. 525–529. 

- [15] H. Liang, Z. Chang, and C. K. Ahn, “Hybrid event-triggered intermittent control for nonlinear multi-agent systems,” _IEEE Trans. Netw. Sci. Eng._ , vol. 10, no. 4, pp. 1975–1984, Jul./Aug. 2023. 

- [16] Q. Pan, J. Wu, J. Li, W. Yang, and M. Guizani, “Blockchain and multi-agent learningempoweredincentiveirsresourceschedulingforintelligentreconfigurable networks,” _IEEE/ACM Trans. Netw._ , vol. 32, no. 2, pp. 943–958, Apr. 2024. 

- [17] L. Yuan et al., “Multi-agent incentive communication via decentralized teammate modeling,” in _Proc. AAAI Conf. Artif. Intell._ , 2022, pp. 9466–9474. 

- [18] A. Jin, Y. Ye, B. Lee, and Y. Qiao, “DecoAgent: Large language model empowered decentralized autonomous collaboration agents based on smart contracts,” _IEEE Access_ , vol. 12, pp. 155234–155245, 2024. 

- [19] B. Chen, G. Li, X. Lin, Z. Wang, and J. Li, “BlockAgents: Towards byzantine-robust LLM-based multi-agent coordination via blockchain,” in _Proc. ACM Turing Award Celebration Conf. China_ , 2024, pp. 187–192. 

- [20] Z.-K. Chong, H. Ohsaki, and B. Ng, “LLM-Net: Democratizing LLMs-asa-Service through blockchain-based expert networks,” in _Proc. 14th Int. Conf. Software Comput. Appl._ , 2025, pp. 313–320. 

- [21] M. Qi et al., “SOK: Bitcoin layer two (l2),” _ACM Comput. Surv._ , vol. 58, pp. 1–37, 2024. 

- [22] D. Xiao, C. Zhang, H. Deng, J. Liang, L. Wang, and L. Zhu, “Parallelizing universal atomic swaps for multi-chain cryptocurrency exchanges,” in _Proc. 34th USENIX Secur. Symp._ , 2025, pp. 4073–4092. 

- [23] Z. Li, S. Gao, Z. Peng, S. Guo, Y. Yang, and B. Xiao, “B-DNS: A secure and efficient DNS based on the blockchain technology,” _IEEE Trans. Netw. Sci. Eng._ , vol. 8, no. 2, pp. 1674–1686, Apr./Jun. 2021. 

- [24] P. Kumar, R. Kumar, A. Kumar, A. A. Franklin, S. Garg, and S. Singh, “Blockchain and deep learning for secure communication in digital twin empowered industrial IoT network,” _IEEE Trans. Netw. Sci. Eng._ , vol. 10, no. 5, pp. 2802–2813, Sep./Oct. 2023. 

- [25] N.Lietal.,“Blockchaincross-chainbridgesecurity:Challenges,solutions, and future outlook,” _Distrib. Ledger Technol., Res. Pract._ , vol. 4, no. 1, pp. 1–34, 2025. 

- [26] C. Jiang, X. Liu, S. Wang, J. Liu, and Y. Zhang, “EFEVD: Enhanced feature extraction for smart contract vulnerability detection,” in _Proc. 33rd Int. Joint Conf. Artif. Intell._ , 2024, pp. 4246–4254. 

- [27] R. Wang, Y. Zhang, and L. Peng, “Anomaly detection service for blockchain transactions using minimal substitution-based label propagation,” _IEEE Trans. Serv. Comput._ , vol. 17, no. 5, pp. 2054–2066, Sep./Oct. 2024. 

- [28] M. Qi, Z. Xu, Z. Wang, S. Chen, and Y. Xiang, “Databox-based delivery service via blockchain,” in _Proc. IEEE Int. Conf. Web Serv._ , 2022, pp. 79–84. 

- [29] Z. Wang, Y. Yu, W. Zheng, W. Ma, and M. Zhang, “MACREC: A multiagent collaboration framework for recommendation,” in _Proc. 47th Int. ACM SIGIR Conf. Res. Develop. Inf. Retrieval_ , 2024, pp. 2760–2764. 

- [30] B. Jiang, Y. Xie, X. Wang, W. J. Su, C. J. Taylor, and T. Mallick, “Multimodal and multi-agent systems meet rationality: A survey,” in _Proc. Int. Conf. Mach. Learn. Workshop LLMs Cogn._ , 2024. [Online]. Available: https://openreview.net/forum?id=9Rtm2gAVjo 

- [31] D. Chen, X. Liu, and W. Yu, “Finite-time fuzzy adaptive consensus for heterogeneous nonlinear multi-agent systems,” _IEEE Trans. Netw. Sci. Eng._ , vol. 7, no. 4, pp. 3057–3066, Oct./Dec. 2020. 

- [32] P. P. Santos et al., “Centralized training with hybrid execution in multiagent reinforcement learning via predictive observation imputation,” _Artif. Intell._ , vol. 348, 2025, Art. no. 104404. 

- [33] Z. Pan et al., “Robust watermarking for federated diffusion models with unlearning-enhanced redundancy,” _IEEE Trans. Dependable Secure Comput._ , no. 1, pp. 1–15, 2025. 

- [34] H. Zhao, Z. Pan, Y. Wang, Z. Ying, L. Xu, and Y.-A. Tan, “Personalized label inference attack in federated transfer learning via contrastive meta learning,” in _Proc. AAAI Conf. Artif. Intell._ , 2025, pp. 22777–22785. 

- [35] J. Lim and P. Tsiotras, “CBS-budget (CBSB): A complete and bounded suboptimal search for multi-agent path finding,” _Artif. Intell._ , vol. 346, 2025, Art. no. 104349. 

- [36] X. Ren, H. Liang, Y. Wang, C. Zhang, Z. Xiong, and L. Zhu, “BESA: Boosting encoder stealing attack with perturbation recovery,” _IEEE Trans. Inf. Forensics Secur._ , vol. 20, pp. 10007–10018, 2025. 

- [37] Z. Pan et al., “Feature-based machine unlearning for vertical federated learning in IoT networks,” _IEEE Trans. Mobile Comput._ , vol. 24, no. 6, pp. 5031–5044, Jun. 2025. 

- [38] Z. Xu et al., “The trust paradox in LLM-based multi-agent systems: When collaboration becomes a security vulnerability,” 2025, _arXiv:2510.18563_ . 

- [39] F. Guan, T. Zhu, W. Chang, W. Ren, and W. Zhou, “Large language models merging for enhancing the link stealing attack on graph neural networks,” _IEEE Trans. Dependable Secure Comput._ , vol. 22, no. 6, pp. 6809–6825, Nov./Dec. 2025. 

- [40] M. Shridhar et al., “Alfred: A benchmark for interpreting grounded instructions for everyday tasks,” in _Proc. IEEE/CVF Conf. Comput. Vis. Pattern Recognit._ , 2020, pp. 10740–10 749. 

- [41] C. Qian et al., “Communicative agents for software development,” in _Proc. 62nd Annu. Meeting Assoc. Comput. Linguistics_ , 2024, pp. 15174–15186. 

- [42] Z. Liu, W. Yao, J. Zhang, L. Yang, Z. Liu, and J. Tan, “Agentlite: A lightweight library for building and advancing task-oriented LLM agent system,” 2024, _arXiv:2402.15538_ . 

- [43] Z. Liu, Y. Zhang, P. Li, Y. Liu, and D. Yang, “Dynamic LLM-agent network: An LLM-agent collaboration framework with agent team optimization,” 2023, _arXiv:2310.02170_ . 

- [44] X. Ning, Z. Lin, Z. Zhou, Z. Wang, H. Yang, and Y. Wang, “Skeletonof-thought: Prompting LLMs for efficient parallel generation,” in _Proc. 12th Int. Conf. Learn. Representations_ , 2024. [Online]. Available: https: //openreview.net/forum?id=mqVgBbNCm9 

- [45] Y. Chen, J. Arkin, Y. Zhang, N. Roy, and C. Fan, “Scalable multi-robot collaboration with large language models: Centralized or decentralized systems?,” in _Proc. IEEE Int. Conf. Robot. Automat._ , 2024, pp. 4311–4317. 

- [46] Z. Hu, Z. Zhang, H.-Y. Li, C. Chen, H. Ding, and Z. Wang, “Attentionguided contrastive role representations for multi-agent reinforcement learning,” in _Proc. 12th Int. Conf. Learn. Representations_ , 2024. [Online]. Available: https://openreview.net/forum?id=LWmuPfEYhH 

- [47] G. Li, H. Hammoud, H. Itani, D. Khizbullin, and B. Ghanem, “Camel: Communicative agents for” mind” exploration of large scale language model society,” _Adv. Neural Inf. Process. Syst._ , vol. 36, pp. 51 991–52 008, 2023. 

- [48] H. Ma et al., “Coevolving with the other you: Fine-tuning LLM with sequential cooperative multi-agent reinforcement learning,” in _Proc. Adv. Neural Inf. Process. Syst._ , 2024, pp. 15497–15525. 

- [49] W. Tao, Y. Zhou, Y. Wang, W. Zhang, H. Zhang, and Y. Cheng, “Magis: LLM-based multi-agent framework for github issue resolution,” in _Proc. Adv. Neural Inf. Process. Syst._ , 2024, pp. 51963–51993. 

- [50] S. Kang, J. Yoon, and S. Yoo, “Large language models are few-shot testers: Exploring LLM-based general bug reproduction,” in _Proc. IEEE/ACM 45th Int. Conf. Softw. Eng._ , 2023, pp. 2312–2323. 

- [51] G. Deng et al., “ _{_ PentestGPT _}_ : Evaluating and harnessing large language models for automated penetration testing,” in _Proc. 33rd USENIX Secur. Symp._ , 2024, pp. 847–864. 

- [52] Y. Dong, X. Jiang, Z. Jin, and G. Li, “Self-collaboration code generation via chatgpt,” _ACM Trans. Softw. Eng. Methodol._ , vol. 33, pp. 1–38, 2024. 

- [53] Z. Mandi, S. Jain, and S. Song, “Roco: Dialectic multi-robot collaboration with large language models,” in _Proc. 2024 IEEE Int. Conf. Robot. Automat._ , Yokohama, Japan, 2024, pp. 286–299. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

6720 

IEEE TRANSACTIONS ON NETWORK SCIENCE AND ENGINEERING, VOL. 13, 2026 

- [54] H. Zhang et al., “Building cooperative embodied agents modularly with large language models,” in _Proc. Int. Conf. Neural Inf. Process. Syst. Found. Models Decis. Mak. Workshop_ , New Orleans, LA, USA, 2024. [Online]. Available: https://openreview.net/forum?id=EnXJfQqy0K 

- [55] S. S. Kannan, V. L. Venkatesh, and B.-C. Min, “Smart-LLM: Smart multiagent robot task planning using large language models,” in _Proc. 2024 IEEE/RSJ Int. Conf. Intell. Robots Syst._ , 2024, pp. 12140–12147. 

- [56] G. Zhu, R. Zhou, W. Ji, and S. Zhao, “LamaRL: LLM-aided multi-agent reinforcement learning for cooperative policy generation,” _IEEE Robot. Automat. Lett._ , vol. 10, no. 7, pp. 7476–7483, Jul. 2025. 

- [57] T. Yao, Y. Xu, H. Wang, X. Qiu, K. Althoefer, and P. Qi, “Multi-agent fuzzy reinforcement learning with LLM for cooperative navigation of endovascular robotics,” _IEEE Trans. Fuzzy Syst._ , pp. 1–11, 2025. 

- [58] A. M. Bran, S. Cox, O. Schilter, C. Baldassari, A. D. White, and P. Schwaller, “Chemcrow: Augmenting large-language models with chemistry tools,” _Nature Mach. Intell._ , vol. 6, no. 5, pp. 525–535, 2024. 

- [59] A. Ghafarollahi and M. J. Buehler, “Protagents: Protein discovery via large language model multi-agent collaborations combining physics and machine learning,” _Digit. Discovery_ , vol. 3, no. 7, pp. 1389–1409, 2024. 

- [60] T. Liang et al., “Encouraging divergent thinking in large language models through multi-agent debate,” in _Proc. 2024 Conf. Empirical Methods Natural Lang. Process._ , 2023, pp. 17889–17904. 

- [61] Y. Cao et al., “Survey on large language model-enhanced reinforcement learning: Concept, taxonomy, and methods,” _IEEE Trans. Neural Netw. Learn. Syst._ , vol. 36, no. 6, pp. 9737–9757, Jun. 2025. 

- [62] F. Peiyuan et al., “Agile: A novel reinforcement learning framework of LLM agents,” in _Proc. Adv. Neural Inf. Process. Syst._ , 2024, pp. 5244–5284. 

- [63] R. Rafailov, A. Sharma, E. Mitchell, S. Ermon, C. D. Manning, and C. Finn, “Direct preference optimization: Your language model is secretly a reward model,” in _Proc. 37th Int. Conf. Neural Inf. Process. Syst._ , 2023, pp. 53728–53741. 

**Minfeng Qi** (Member, IEEE) received the M.Eng. degree in information systems from Monash University, Clayton, VIC, Australia, in 2019, and the Ph.D. degree in computer science from the Swinburne University of Technology, Hawthorn, VIC, Australia, in 2023. He is currently an Assistant Professor with the City University of Macau, Macau, China. His research interests include blockchain security and AI security. 

**Tianqing Zhu** (Member, IEEE) received the B.Eng. degree in chemistry and the M.Eng. degree in automation from Wuhan University, Wuhan, China, in 2000 and 2004, respectively, and the Ph.D. degree in computer science from Deakin University, Docklands, VIC, Australia, in 2014. She is currently a Professor with the Faculty of Data Science, City University of Macau, Macau, China. Her research interests include privacy-preserving, cybersecurity, and privacy in artificial intelligence. 

**Lefeng Zhang** received the B.Eng. and M.Eng. degrees from the Zhongnan University of Economics and Law, Wuhan, China, in 2016 and 2019, respectively, and the Ph.D. degree from the University of Technology Sydney, Ultimo, NSW, Australia, in 2024. He is currently an Assistant Professor with the City University of Macau, Macau, China. His research interests include game theory and privacypreserving. 

- [64] S. Xu et al., “Is DPO superior to PPO for LLM alignment? A comprehensive study,” in _Proc. 41st Int. Conf. Mach. Learn._ , 2024, pp. 54983–54998. 

- [65] J. Wu et al., “ _δ_ -DPO: Direct preference optimization with dynamic _δ_ ,” in _Proc. 38th Int. Conf. Neural Inf. Process. Syst._ , 2023, pp. 129944–129966. 

- [66] W. Zeng, Y. Lan, J. He, and G. Neubig, “Trust region direct preference optimization,” _Adv. Neural Inf. Process. Syst._ , vol. 37, pp. 27 463–27 489, 2024. 

- [67] H. Lightman et al., “Let’s verify step by step,” in _Proc. 12th Int. Conf. Learn. Representations_ , 2023. [Online]. Available: https://openreview.net/ forum?id=v8L0pN6EOi 

- [68] Y. Li et al., “Hierarchical reward models for long-form text generation,” _Findings Assoc. Comput. Linguistics_ , 2024, pp. 7817–7831. 

- [69] H. Wang, W. Xiong, T. Xie, H. Zhao, and T. Zhang, “Interpretable preferences via multi-objective reward modeling and mixture-of-experts,” in _Proc. Conf. Assoc. Comput. Linguistics - Empir. Methods Natural Lang. Process._ , Miami, FL, USA, 2024, pp. 10582–10592. 

- [70] M. Qi, Z. Wang, S. Chen, and Y. Xiang, “A hybrid incentive mechanism for decentralized federated learning,” Distributed Ledger Technologies: _Res. Pract._ , vol. 1, no. 1, pp. 1–15, 2022. 

- [71] F. Yao, C. Li, H. Li, Y. Ghale, and Z.-L. Zhang, “Human vs. generative AI in content creation competition: Symbiosis or conflict?,” in _Proc. 41st Int. Conf. Mach. Learn._ , 2024, pp. 56885–56913. 

- [72] L. Zhang, T. Zhu, P. Xiong, W. Zhou, and P. S. Yu, “A robust gametheoretical federated learning framework with joint differential privacy,” _IEEE Trans. Knowl. Data Eng._ , vol. 35, no. 4, pp. 3333–3346, Apr. 2023. 

- [73] C. Fan, J. Chen, Y. Jin, and H. He, “Can large language models serve as rational players in game theory? a systematic analysis,” in _Proc. AAAI Conf. Artif. Intell._ , 2024, pp. 17960–17967. 

- [74] S. Mao et al., “Alympics: LLM agents meet game theory,” in _Proc. 31st Int. Conf. Comput. Linguistics_ , 2025, pp. 2845–2866. 

- [75] P. Duetting et al., “Mechanism design for large language models,” in _Proc. ACM Web Conf._ , 2023, pp. 144–155. 

- [76] B. Laufer, J. Kleinberg, and H. Heidari, “Fine-tuning games: Bargaining and adaptation for general-purpose models,” in _Proc. ACM Web Conf._ , 2023, pp. 66–76. 

- [77] Y. Liu, Y. Jia, J. Jia, D. Song, and N. Z. Gong, “DataSentinel: A gametheoreticdetectionofpromptinjectionattacks,” in _Proc.IEEESymp.Secur. Privacy_ , 2025, pp. 2190–2208. 

- [78] J. Duan et al., “GTBench: Uncovering the strategic reasoning capabilities of LLMs via game-theoretic evaluations,” in _Proc. Adv. Neural Inf. Process. Syst._ , 2024, pp. 28219–28253. 

- [79] E. Akata, L. Schulz, J. Coda-Forno, S. J. Oh, M. Bethge, and E. Schulz, “Playing repeated games with large language models,” _Nature Hum. Behav._ , vol. 9, pp. 1380–1390, 2025. 

**Ningran Li** (Graduate Student Member, IEEE) received the B.Sc. degree in information systems, the M.Sc. degree in information technology, and the Ph.D. degree in computer science from the Swinburne University of Technology, Melbourne, Australia, in 2024. She is currently an Assistant Professor with the University of Adelaide, Adelaide, SA, Australia. Her research interests mainly include cybersecurity, blockchain security, and cross-chain technologies. 

**Yu-an Tan** (Senior Member, IEEE) received the B.Eng. degree in computer software in 1991 and the Ph.D. degree in computer science in 2003. He has Teaching and Research Experience of more than 26 years, and has been a Professor with Beijing Institute of Technology, Beijing, China, since 2010. He contributes for peer-reviewed more than 100 journal articles and conference papers, including five papers of the top 1% of Essential Science Indicators. His research interests include artificial intelligence security, cybersecurity, and storage sub-systems. He is a Senior Member of China Computer Federation. He was the recipient of more than 20 research funds from the National Natural Science Foundation of China and the National Key Research and Development Program of China. 

**Wanlei Zhou** (Life Fellow, IEEE) received the B.Eng. and M.Eng. degrees in computer science and engineering from Harbin Institute of Technology, Harbin, China, in 1982 and 1984, respectively, the Ph.D. degree in computer science and engineering from The Australian National University, Canberra, ACT,Australia,in1991,andtheD.Sc.degree(Higher Doctoral degree) from Deakin University, VIC, Australia, in 2002. He is currently the Vice Rector of A the City University of Macau, Macau, China. He has authored or coauthored more than 400 papers in refereed international journals and refereed international conferences proceedings, including many articles in IEEE transactions and journals. His research interests include security, privacy-preserving, and distributed systems. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:54:15 UTC from IEEE Xplore.  Restrictions apply. 

