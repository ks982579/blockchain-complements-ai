Expert Systems With Applications 293 (2025) 128729 

Contents lists available at ScienceDirect 

## Expert Systems With Applications 

journal homepage: www.elsevier.com/locate/eswa 

## Review 

## Federated multi-agent reinforcement learning: A comprehensive survey of methods, applications and challenges 

Yao Jing a, Bin Guo a,∗, Nuo Li b, Ruonan Xu a, Zhiwen Yu a 

a _School of Computer Science, Northwestern Polytechnical University, Xi’an, Shaanxi, 710072, China_ b _School of Computer Science, Fudan University, Shanghai, Shanghai, 200438, China_ 

## a r t i c l e i n f o a b s t r a c t 

_Keywords:_ Federated learning Multi-agent systems Reinforcement learning Federated multi-agent reinforcement learning Distributed optimization 

Multi-Agent Reinforcement Learning (MARL) and Federated Learning (FL) have achieved significant advancements in addressing complex, large-scale problems across various domains. As demands grow for solutions that balance decentralized decision-making with data privacy, the emerging paradigm of Federated Multi-Agent Reinforcement Learning (FMARL) has drawn increasing attention. FMARL integrates the collaborative, privacypreserving framework of FL with the adaptive, multi-agent coordination of MARL, enabling efficient and secure decision-making across distributed, dynamic environments. Despite its growing popularity, FMARL research lacks a comprehensive analysis of its methods, applications, and open challenges. Therefore, this survey aims to fill this gap by systematically reviewing FMARL approaches from multiple perspectives, categorizing current methods, exploring diverse application scenarios, and discussing critical challenges. This work offers valuable insights for researchers, promoting a deeper understanding and future advancements in FMARL. 

## **1. Introduction** 

With the rapid advancement of Machine Learning (ML) (Jordan & Mitchell, 2015; Mitchell & Mitchell, 1997), it has demonstrated remarkable capabilities in addressing complex tasks, including natural language processing (Devlin, 2018; Otter et al., 2020; Vaswani, 2017), image classification (He et al., 2016; Krizhevsky et al., 2012; Simonyan & Zisserman, 2014), recommendation system (Gao et al., 2023a; Steck et al., 2021), etc. Traditionally, ML methods have relied on centralized data collection for training models, which involves aggregating data from multiple sources. This centralized approach presents significant challenges, including privacy concerns, data security risks, and complications related to data ownership. Additionally, centralized methods often face scalability issues when managing large datasets or when data is distributed across various devices. In response to these challenges, Google proposes Federated Learning (FL) (Koneˇcn`y et al., 2016; McMahan et al., 2017), a unique distributed ML framework designed to enable collaborative model training across decentralized devices. FL facilitates collaborative model training without the need to centralize data, allowing each participant to maintain control over their own data while still contributing to the overall learning process. This innovative approach not only enhances privacy and security but also improves the scalability 

and efficiency of ML applications, making it well-suited for modern data environments. 

In the field of ML, deep reinforcement learning (DRL) (Kaelbling et al., 1996) stands out as a powerful paradigm where an agent learns to make sequential decisions by interacting with an environment to maximize cumulative rewards. Classical DRL algorithms include Q-Learning, Sarsa, Policy Gradient, Actor-Critic, etc. (Sutton, 2018; Wang et al., 2022). DRL has shown remarkable success in tasks such as robotics control (Brunke et al., 2022; Kober et al., 2013), game playing (e.g., AlphaGo) (Silver et al., 2016), and autonomous driving (Kiran et al., 2021; Sallab et al., 2017). However, many real-world applications do not involve just a single agent acting independently but rather consist of multiple agents interacting within an environment. In these multiagent scenarios, traditional RL methods face limitations. Multi-Agent Reinforcement Learning (MARL) (Tan, 1993) extends the DRL framework to scenarios involving multiple interacting agents. In recent years, research on MARL has drawn considerable attention for its strong potential to address complex real-world scenarios. While MARL has achieved considerable success in many domains like IoT, Mohammadi et al. (2017), Xiong et al. (2020), games (Terry et al., 2021; Vinyals et al., 2019; Yang & Wang, 2020), and autonomous driving (Antonio & MariaDolores, 2022; Bhalla et al., 2020; Shalev-Shwartz et al., 2016), its 

> ∗ Corresponding author. 

_E-mail addresses:_ jy_jingyao@163.com (Y. Jing), guob@nwpu.edu.cn (B. Guo), linuo@mail.nwpu.edu.cn (N. Li), xuruonan@mail.nwpu.edu.cn (R. Xu), zhiwenyu@nwpu.edu.cn (Z. Yu). 

https://doi.org/10.1016/j.eswa.2025.128729 

Received 18 February 2025; Received in revised form 29 May 2025; Accepted 21 June 2025 Available online 23 June 2025 

0957-4174/© 2025 Elsevier Ltd. All rights are reserved, including those for text and data mining, AI training, and similar technologies. 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

application to real-world problems involving large-scale multi-agent systems remains challenging. First, in environments with large action and state spaces, agents’ performance can be highly dependent on sample quality, as it is nearly impossible to explore the entire sampling space comprehensively. This dependency leads to issues with sample efficiency, slowing the learning process of MARL algorithms. To address this, information exchange among agents can greatly accelerate learning; however, many existing methods, such as distributed and parallel MARL algorithms, rely on centralized servers to aggregate data, parameters, or gradients from each agent. The centralized nature of these methods presents two key limitations. Computationally, as data and agents become more dispersed across devices or locations, centralized systems struggle with scalability and increasing resource demands (Zhang et al., 2019b; Zhu et al., 2024). Additionally, relying on a central server introduces significant privacy concerns (Lei et al., 2023; Pan et al., 2019): agents may not trust the central server, and data transmissions are vulnerable to interception, posing risks to data security and agent privacy. As a result, protecting privacy and preventing information leakage have become major obstacles in advancing MARL applications in decentralized, real-world environments. 

The success of FL in supervised learning (Dong et al., 2022; Hastie et al., 2009; Zhang et al., 2021a) has inspired its application to MARL, leading to the development of Federated Multi-Agent Reinforcement Learning (FMARL) (De Oliveira et al., 2024; Li et al., 2020, 2023b; Yang et al., 2020). By combining FL’s decentralized data processing capabilities with MARL’s multi-agent decision-making framework, FMARL creates a secure, distributed MARL paradigm that enhances learning efficiency while safeguarding sensitive information. FMARL’s decentralized framework allows each agent to learn and operate independently within its local environment. Through the exchange of model updates, parameters, gradients, and other necessary information, agents collaboratively contribute to a global policy or value function without sharing sensitive raw data. This approach offers several key features that make FMARL particularly suited for large-scale, privacy-sensitive, and complex environments: 

- **Data Privacy Protection:** FMARL ensures that sensitive data remains within local environments, making it ideal for privacysensitive domains such as healthcare (El Hamdani & Benamar, 2018; Otoum et al., 2021; Pfitzner et al., 2021; Tam et al., 2022; Tiwari et al., 2023). In this domain, regulations like GDPR (Voigt & Von dem Bussche, 2017) and HIPAA (Edemekong et al., 2018) mandate strict data protection. Additionally, FMARL often incorporates encryption mechanisms to further secure data during the exchange of updates, ensuring that privacy is maintained throughout the learning process. 

- **Reduce Communication Overhead:** By enabling agents to share only model updates or parameters instead of full datasets, FMARL significantly reduces communication costs. This reduction is crucial in bandwidth-limited or high-cost communication systems, such as Internet of Things (IoT) (Moudoud et al., 2024b; Pinto Neto et al., 2023; Tong et al., 2023) and intelligent transport systems (Liu & Mao, 2023; Wang et al., 2020b), where efficient communication is essential for optimal performance. 

- **Enhance Learning Efficiency:** FMARL improves learning efficiency by facilitating collaborative updates among agents. This accelerates knowledge sharing and enhances the overall learning performance, making it particularly valuable in environments where agents must quickly adapt to dynamic conditions or solve complex tasks collaboratively. 

- **Mitigation of Limited Observation:** In many complex MARL scenarios, agents may have access to only partial information, which can be insufficient for optimal decision-making. FMARL can address this limitation by securely aggregating distributed information from multiple agents, providing a more comprehensive understanding of the environment. This aggregation enhances 

- learning accuracy while preserving the privacy of individual agents’ data. 

- **Scalability:** FMARL’s decentralized structure enhances its scalability, allowing it to efficiently manage environments with numerous agents, such as autonomous traffic management (El Hamdani & Benamar, 2018; Lin & Rubin, 2017) or smart city infrastructures (Al-Hader & Rodzi, 2009; Jin et al., 2014). The decentralized nature of the learning process allows agents to share information with the central server or other agents. As new agents join the system, they can quickly benefit from the historical experiences of the central server or other agents, without needing to start learning from scratch. This approach ensures seamless integration and accelerates the learning process, even as the number of agents grows. 

- **Robustness to Non-IID Data** : FMARL is robust in handling Independent and Identically Distributed (non-IID) data (Arafeh et al., 2022; Li et al., 2019; Zhu et al., 2021), meaning it can accommodate agents that operate in heterogeneous environments with data distributions that vary across agents. Its federated aggregation mechanisms are specifically designed to manage these diverse data distributions, making FMARL more adaptable to real-world scenarios where agent experiences may differ significantly. 

While several survey papers on FL (Nguyen et al., 2021; Zhang et al., 2021a) and MARL (Gronauer & Diepold, 2022; Zhang et al., 2021b) have been published, there is currently no comprehensive survey focused on FMARL, a relatively new and emerging field. This paper aims to fill that gap by providing an in-depth survey of FMARL, exploring its theoretical foundations, categorizing existing algorithms, and elucidating practical applications. Several existing surveys have reviewed federated reinforcement learning (FRL) (Pinto Neto et al., 2023; Qi et al., 2021). FRL primarily focuses on multiple independent singleagent learners trained separately and aggregated through a federated scheme. These agents operate in isolated environments without interaction, and collaboration is limited to model-level parameter sharing. However, many real-world applications require multiple agents to interact within a shared environment, calling for coordinated learning under privacy and communication constraints. FMARL extends FRL to support such interactive scenarios, introducing new challenges such as environment non-stationarity, credit assignment, and partial observability. As a more general framework, FMARL encompasses both traditional FRL and interactive multi-agent settings. This survey provides a unified and comprehensive overview to better reflect the broader scope and unique challenges of FMARL. 

Our survey consolidates existing knowledge from current studies and identifies critical research directions essential for addressing the current challenges in FMARL. To establish a foundational understanding, we begin with a tutorial on the fundamental concepts of FL and MARL. This tutorial lays the groundwork for a comprehensive exploration of FMARL. Building on this foundation, we provide rigorous formulaic definitions and a technical perspective on the evolutionary trajectory of FMARL, emphasizing its distinct advantages over conventional MARL approaches. We then categorize FMARL algorithms based on various dimensions. This categorization provides a structured framework for understanding the diverse methodologies within FMARL. Furthermore, we conduct a thorough review of the current literature on FMARL applications across various domains, illustrating the practical impact and versatility of FMARL. In addition to reviewing existing work, we outline future research avenues that highlight FMARL’s potential to advance collaborative, adaptive, and privacy-preserving ML within distributed environments. By synthesizing existing knowledge and identifying critical research directions, we aim to facilitate future advancements in FMARL. This approach fosters innovations that enhance the efficacy and applicability of collaborative, adaptive, and privacy-preserving ML in distributed settings. 

2 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

Specifically, this paper is organized as follows. Section 2 provides the formal definition of FMARL. Section 3 presents the architecture of FMARL. Section 4 explores the methodologies employed in FMARL. In Section 5, we categorize FMARL into its various types. Section 6 discusses the diverse applications of FMARL, and Section 7 addresses the challenges and future directions of FMARL. 

## **2. Formal of FMARL** 

FMARL is a federated learning framework adapted for multi-agent reinforcement learning. In traditional MARL, agents may exhibit cooperative, competitive, or mixed behaviors depending on the task (Salazar et al., 2024). FMARL extends federated learning to multi-agent settings, enabling distributed training across agents with privacy and communication constraints. While FMARL can in principle support both cooperative and competitive interactions, cooperative scenarios where agents jointly optimize shared objectives are the most widely studied in existing research. Accordingly, this survey adopts a unified formal definition grounded in the cooperative setting, which serves as the representative case for most FMARL applications. FMARL operates under assumptions aligned with the distributed nature of federated learning: data remains decentralized, each agent conducts training locally and independently, and model aggregation either centralized or peer-to-peer is performed periodically to facilitate collaborative learning without exposing raw data. By avoiding the exchange of raw data, FMARL enables distributed coordination and collaborative decision-making among agents while preserving data privacy. 

Building upon the foundational definitions of MARL (Yang & Wang, 2020), the core principles of FL (McMahan et al., 2017; Zhang et al., 2021a), and insights from existing works that combine the two (Pinto Neto et al., 2023; Qi et al., 2021), we compiled and proposed a general and practical formalization of FMARL. Our formulation aims to capture the common structures and challenges of real-world FMARL applications, particularly those involving collaborative multi-agent systems under federated learning constraints. Formally, FMARL consists of two fundamental stages: decentralized local training and federated aggregation. During the local learning stage, each agent independently trains its policy within its own decision-making environment. Depending on the task setting, this decision-making environment is typically modeled as a mathematical tuple (Littman, 1994; Puterman, 2014), such as a Markov Decision Process (MDP) or a Decentralized Partially Observable Markov Decision Process (Dec-POMDP) (Bernstein et al., 2002). An MDP is a single-agent reinforcement learning framework where the agent has full observability of the environment. When each agent operates in a distinct environment, the local decision-making process of each agent can be naturally defined as an individual MDP. In contrast, if multiple agents operate in a shared environment with decentralized control, where each agent can only observe partial information and their actions jointly affect global outcomes, the scenario is formally modeled as a Dec-POMDP. Dec-POMDP emphasizes decentralized decision-making by multiple agents collaboratively acting under partial observability. In order to provide a general and unified formal definition of FMARL, we abstract the decision-making environment as a generic tuple without specifying a particular model. This abstraction allows our formulation to remain applicable across various task settings and interaction structures. 

For a system with _𝑛_ agents, the decision-making environment in FMARL is formally defined by the tuple: 

(1) 

_𝛤_ = ( _𝐼,_ { _𝑖_ } _𝑖_ ∈ _𝐼 ,_ { _𝑖_ } _𝑖_ ∈ _𝐼 ,_ { _𝑃𝑖_ } _𝑖_ ∈ _𝐼 ,_ { _𝑖_ } _𝑖_ ∈ _𝐼 , 𝛾_ ) _._ 

- **Agent Set** _𝐼_ : A collection of _𝑛_ agents, _𝐼_ = {1 _,_ 2 _,_ … _, 𝑛_ }, where each agent _𝑖_ ∈ _𝐼_ operates in a decentralized manner. 

- **State Space**  _𝑖_ : The private state space of agent _𝑖_ , representing the part of the environment observable only to that agent. 

- **Action Space**  _𝑖_ : The set of actions available to agent _𝑖_ , determined by its local perspective or role in the environment. 

- **Reward Function**  _𝑖_ : The reward function _𝑟[𝑡] 𝑖_[=][ ] _[𝑖]_[(] _[𝑠][𝑡] 𝑖[, 𝑎][𝑡] 𝑖_[)][defines][the] immediate reward received by agent _𝑖_ at time step _𝑡_ , based on its local state _𝑠[𝑡] 𝑖_[and][the][action] _[𝑎][𝑡] 𝑖_[.] 

- **Transition Probability** _𝑃𝑖_ : The transition probability _𝑃𝑖_ ( _𝑠[𝑡] 𝑖_[+1] ∣ _𝑠[𝑡] 𝑖[, 𝑎][𝑡] 𝑖_[)] represents the probability of transition from the state _𝑠[𝑡] 𝑖_[to] _[𝑠][𝑡] 𝑖_[+1][after] taking action _𝑎[𝑡]_[[.]] 

   - _𝑖_[[.]] 

- **Discount Factor** _𝛾_ : The discount factor _𝛾_ ∈[0 _,_ 1) determines the importance of future rewards in the decision-making process. A smaller _𝛾_ emphasizes short-term rewards, while a larger _𝛾_ gives more weight to long-term rewards. 

Given the environment formulation, agent _𝑖_ ∈ _𝐼_ independently interacts with the environment over discrete time steps. At each time step _𝑡_ , the agent observes its local observations _𝑠[𝑡] 𝑖_[∈][] _[𝑖]_[,][takes][an][action] _[𝑎][𝑡] 𝑖_[∈][] _[𝑖]_[,] receives a reward _𝑟[𝑡] 𝑖_[,][and][transitions][to][the][next][state] _[𝑠][𝑡] 𝑖_[+1] . Through this process, the agent accumulates a local experience dataset  _𝑖_ :  _𝑖_ = { _𝑠[𝑡] 𝑖[, 𝑎][𝑡] 𝑖[, 𝑟][𝑡] 𝑖[, 𝑠][𝑡] 𝑖_[+1] } _[𝑇] 𝑡_ =1 _[,]_ (2) 

where _𝑇_ denotes the total number of time steps. 

Based on  _𝑖_ , agent _𝑖_ learns a local policy _𝜋𝑖_ mapping states to actions _𝜋𝑖_ ∶  _𝑖_ →  _𝑖_ , which is typically parameterized by _**𝜽𝒊**_ . For example, the weights of a neural network. For a stochastic policy, _𝜋𝑖_ defines a conditional probability distribution: _𝜋𝑖_ ( _𝑎[𝑡] 𝑖_[∣] _[𝑠][𝑡] 𝑖_[;] _**[ 𝜽]**[𝑖]_[)] _[,]_ (3) 

representing the probability of taking action _𝑎[𝑡] 𝑖_[given][state] _[𝑠][𝑡] 𝑖_[.] In FMARL, the optimization objective of each agent depends on the task and the required level of inter-agent collaboration. Commonly, reward structures can be categorized into three paradigms: local independent objectives, global collaborative objectives, and hybrid objectives. To provide a unified formulation that generalizes across these scenarios, we define the reward of each agent _𝑖_ as a weighted combination of local and global rewards: 

**==> picture [252 x 10] intentionally omitted <==**

_𝐽𝑖_ ( _**𝜽** 𝑖_ ) denotes the expected cumulative local reward for agent _𝑖_ , _𝐽_ global( _**𝜽**_ 1 _,_ … _,_ _**𝜽** 𝑛_ ) represents the global reward shared by all agents, and _𝜆_ ∈[0 _,_ 1] is a trade-off parameter controlling the balance between local and global rewards. 

The local expected cumulative reward of agent _𝑖_ is defined as: 

**==> picture [252 x 50] intentionally omitted <==**

**==> picture [252 x 51] intentionally omitted <==**

**==> picture [252 x 15] intentionally omitted <==**

This unified formulation covers the following typical cases: 

- **Local independent Objective** ( _𝜆_ = 1): Each agent focuses solely on optimizing its own local reward, suitable for loosely coupled or personalized applications (e.g., local resource management or userspecific services). In this case, the goal of FMARL is to enhance learning efficiency and generalization by aggregating model parameters across agents with similar objectives. Federated aggregation serves as a form of knowledge sharing to achieve coordination. Crucially, privacy is preserved as raw data remains decentralized. 

- **Global Collaborative Objective** ( _𝜆_ = 0): All agents aim to optimize a shared global reward, applicable to fully cooperative tasks such as multi-robot formation control or coordinated fleet management. 

3 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

Here, the purpose of FMARL is to enable decentralized policy alignment. Federated learning is used to synchronize local models toward a common goal, achieving global consistency without centralized data collection or control. The focus is on coordination through distributed optimization. 

- **Hybrid Objective** (0 _< 𝜆<_ 1): Agents must simultaneously pursue individual and collective goals, as seen in partially cooperative systems like collaborative IoT sensing or multi-robot exploration with mixed responsibilities. In this setting, FMARL facilitates a flexible balance between personalization and cooperation through weighted aggregation of policy parameters. It supports selective collaboration, allowing agents to improve joint performance while retaining autonomy and respecting privacy constraints. 

In FMARL, each agent _𝑖_ optimizes its local policy _𝜋𝑖_ through environment interaction and periodically participates in a federated aggregation process subject to federated constraints. Let _**𝜽**[𝑘] 𝑖_[represent][the][pol-] icy parameters of agent _𝑖_ after _𝑘_ local updates. Federated aggregation integrates the local models as: 

**==> picture [252 x 37] intentionally omitted <==**

**==> picture [252 x 23] intentionally omitted <==**

**==> picture [252 x 37] intentionally omitted <==**

- where | _𝑖_ | is the size of agent _𝑖_ ’s local dataset.  is the federated constraint set defined as: 

**==> picture [252 x 10] intentionally omitted <==**

where each component is defined as: 

- comm (Communication Constraints): Defines the admissible space of communication-related parameters, such as communication interval _𝜏_ , bandwidth usage _𝑏_ , and synchronization scheme _𝜙_ . Formally, 

- comm ∶=[{] ( _𝜏, 𝑏, 𝜙_ ) | _𝜏_ ≤ _𝜏_ max _, 𝑏_ ≤ _𝑏_ max _, 𝜙_ ∈Φ[}] _,_ 

where Φ denotes the set of allowable synchronization protocols (e.g., synchronous, asynchronous, semi-synchronous). 

- privacy (Privacy Constraints): Enforces privacy-preserving conditions on the shared information. Let _𝜖_ be the privacy budget and  the perturbation mechanism (e.g., DP noise, encryption). Then, 

- privacy ∶=[{] ( _𝜖,_ ) | _𝜖_ ≤ _𝜖_ max _,_  ∈ 𝕄[}] _,_ 

where 𝕄 is the set of valid privacy mechanisms ensuring that no raw local data is directly exposed. 

- resource (Resource Constraints): Captures limitations on local computational capacity, including memory _𝑚_ , energy _𝑒_ , and processing power _𝑐_ . Formally, 

- resource ∶=[{] ( _𝑚, 𝑒, 𝑐_ ) | _𝑚_ ≤ _𝑚_ max _, 𝑒_ ≤ _𝑒_ max _, 𝑐_ ≤ _𝑐_ max} _,_ 

ensuring that the aggregation and local training remain feasible under device heterogeneity. 

After aggregation, each agent updates its local model by synchronizing with _**𝜽**[𝑘]_ agg[+1][and][continues][local][training.][This][alternating][process][en-] sures collaborative learning while adhering to the federated constraints. 

## **3. Architecture of FMARL** 

In FMARL, each agent independently learns its policy, with collaboration taking place within the federated framework in various forms to 

enhance the overall learning process. A key factor shaping this collaboration is the underlying model architecture. In practice, two commonly adopted structures are the client-server framework (Kaur et al., 2024; McMahan et al., 2017; Yang et al., 2020; Zhang et al., 2021a), where a central coordinator aggregates updates from participating agents, and the peer-to-peer approach (Chen et al., 2024; Peake et al., 2020), where agents communicate directly with selected peers without relying on a central entity. Fig. 1 presents a schematic diagram of FMARL architecture. Fig. 1(a), the client-server architecture is depicted, illustrating how a central server aggregates updates from multiple agents. Fig. 1(b) showcases the peer-to-peer architecture, highlighting direct communication and collaboration among agents. 

In the client-server architecture, agents communicate and collaborate through a central global server. This server plays a pivotal role in collecting, aggregating, and distributing information across the network. By centralizing the learning process, this architecture allows all agents to share a unified global model or policy, ensuring consistency and coherence throughout the system. This centralized approach is particularly effective in scenarios where coordinated learning and decisionmaking are crucial. Specifically, the client-server architecture of the FMARL process typically follows these steps: 

- **Step 1: Client Selection.** Based on predefined criteria (such as data relevance or computational capability), a subset of agents {1 _,_ 2 _,_ … _, 𝑛_ } is selected to participate in each communication round. This selection can be random or based on specific strategies to improve efficiency. 

- **Step 2: Model Initialization.** A central server initializes the global model parameters _**𝜽**_[0][and][distributes][them][to][the][selected][agents.] global 

- This initialization can be random or based on pre-trained models. 

- **Step 3: Local Training.** Each selected agent _𝑖_ independently trains the local model _𝜃𝑖_ using its local dataset _𝐷𝑖_ , performing a specified number of iterations of gradient descent to update its local model parameters. Let _𝜃𝑖_ denote the updated model parameters from agent _𝑖_ . 

- **Step 4: Model Aggregation.** After completing _𝑘_ rounds of local training, local model parameters are encrypted and transmitted to the central server. The server then performs global aggregation using a specified algorithm to update the global federated model. This aggregation can be conducted either synchronously or asynchronously, depending on the availability of communication resources and predefined system constraints. 

- **Step 5: Model Update and Iteration.** The updated global model _**𝜽**[𝑘]_[+1][is][redistributed][to][all][or][selected][agents][for][the][next][round][of] global 

- local training. This process repeats iteratively until a convergence criterion is met. 

The fundamental difference between client-server and peer-to-peer architectures lies in their communication mechanisms. In the architecture (Fig. 1(b)), model exchange and fusion occur through direct parameter transmission among agents, without relying on a central server. Each agent communicates with a predefined set of peers, receiving model updates and performing local aggregation by integrating these external parameters with its own. This decentralized strategy removes single points of failure, reduces communication bottlenecks, and empowers agents with autonomy in model updates. As a result, it enhances system flexibility, scalability, and robustness, making it well-suited for largescale, distributed learning environments. Table 1 summarizes the characteristics of the client-server and peer-to-peer architectures in FMARL. It examines critical design dimensions including system control topology, model aggregation strategies, privacy mechanisms, scalability, and applicable scenarios. Understanding these characteristics helps guide the architectural choice for different FMARL deployment scenarios. 

Client-server architecture is well-suited for scenarios that require centralized coordination, consistent global updates, and a stable network environment. Its inherent structure simplifies synchronization and 

4 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Fig. 1.** Schematic diagram of FMARL architecture. 

**Table 1** 

Comparison of client-server vs. peer-to-peer architectures in FMARL. 

|Dimension|Client-server|Peer-to-peer|
|---|---|---|
|**System** **Control** **Topology**|∙ A central server orchestrates communication and aggregation|∙ Agents interact autonomously without central coordinator|
||∙ Agents passively wait for coordination signals|∙ Control emerges from local agent interactions|
|**Model** **Aggregation** **Strategy**|∙ Server performs global aggregation|∙ Agents share information with neighbors|
||∙ Supports both synchronous and asynchronous updates|∙ Aggregation is fully decentralized and often asynchronous|
|**Privacy** **Preservation** **Mechanism**|∙ Server-side: Differential privacy and encrypted aggregation|∙ Client-side: No trusted third party; agents retain local control|
|||of data|
|**Communication** **Overhead**|∙ One-to-many communication through central server|∙ Many-to-many communication among agents|
||∙ communication overhead depends on server reliability, net-|∙ Communication overhead depends on network topology, agent|
||work bandwidth, communication frequency, and model size.|interaction frequency, and update strategy.|
|**Scalability** **and** **Fault** **Tolerance**|∙ Scalability limited by server load and bandwidth|∙ Scales linearly with number of agents|
||∙ Vulnerable to single-point failure|∙ Robust to agent failure and network fluctuation|
|**Applicable** **Scenarios**|∙ Suitable for scenarios requiring centralized coordination, low|∙ Suitable for highly distributed environments that demand|
||tolerance for inconsistency, and a stable network infrastructure.|stronger privacy, greater scalability, and enhanced fault toler-|
||∙ Typical examples include:|ance through dynamic agent interactions.|
||– Smart home coordination (Lee & Choi, 2020)|∙ Typical examples include:|
||– Traffic management and signal control (Bao et al., 2023; Fu &|– Peer-to-peer energy trading (Qiu et al., 2023; Ye et al., 2025)|
||Di, 2023; Li et al., 2025; Ye et al., 2021)|– Drone swarms or robotic team collaboration (Chung et al.,|
|||2018; Lv et al., 2023).|



global policy convergence, making it ideal for applications where inconsistency across agents is not tolerable. For example, in smart home environments (Lee & Choi, 2020), a central controller can coordinate multiple appliances and devices such as household appliances, photovoltaic panels, and energy storage systems to optimize user comfort and overall energy efficiency. The centralized nature of this architecture allows for seamless integration of device behaviors through federated learning, while ensuring minimal communication delays due to the localized nature of smart homes. Another representative application is urban traffic management, where traffic signal controllers deployed across intersections collaborate to optimize vehicular flow. Centralized FMARL enables these agents to share insights and learn coordinated policies based on a global understanding of the city’s traffic dynamics, leading to reduced congestion and improved travel efficiency (Bao et al., 2023; Fu & Di, 2023; Li et al., 2025; Ye et al., 2021). 

In contrast, peer-to-peer architecture is more suitable for highly distributed and dynamic environments that prioritize scalability, privacy, and resilience. In this architecture, each agent communicates directly with a set of neighbors to exchange model updates, without relying on a central coordinator. This decentralized communication mechanism is particularly valuable in scenarios where network conditions are variable and agents may frequently join or leave the system. Peer-topeer frameworks are particularly applicable in privacy-sensitive applications, as they avoid sharing raw data with any central entity. For example, in smart grids, peer-to-peer energy trading enables scalable and 

privacy-preserving peer-to-peer energy trading in double-sided auction markets, where decentralized prosumers optimize their bidding strategies and operations through distributed coordination without relying on a central controller (Qiu et al., 2023; Ye et al., 2025). Another important application is in drone swarms (Lv et al., 2023) or robotic teams (Chung et al., 2018), where peer-to-peer communication facilitates realtime coordination and decision-making among agents to achieve tasks such as area coverage, search and rescue, or formation control without relying on a central controller. The flexibility and fault tolerance of the peer-to-peer model make it well suited for emerging edge computing systems and federated industrial applications that require autonomous decision-making and robust operation. 

## **4. Categories of FMARL** 

To facilitate a comprehensive understanding and systematic analysis of the rapidly evolving FMARL landscape, FMARL can be categorized along several key dimensions, each emphasizing a distinct technical aspect. These include update synchronization strategies (e.g., synchronous vs. asynchronous), personalization objectives (e.g., global vs. personalized), adaptability to environmental dynamics (e.g., static vs. adaptive), and model structure homogeneity (homogeneous vs. heterogeneous). Specifically, from the perspective of update strategy, FMARL can be divided into synchronous and asynchronous approaches. Synchronous FMARL (Khodadadian et al., 2022; Li et al., 2022b) requires agents to 

5 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Table 2** 

Single-dimension classification of FMARL systems. 

|Dimension|Category|Description|
|---|---|---|
|Update strategy|Synchronous FMARL (Khodadadian et al.,<br>2022; Li et al., 2022b)|All agents update their local models in coordinated rounds,<br>which simplifes aggregation and improves stability but|
|||requires strict synchronization.|
||Asynchronous FMARL (Lan et al., 2024;|Agents update independently without waiting for others,|
||Pan et al., 2022a; Shen et al., 2024)|improving scalability and robustness to delays but|
|||introducing challenges in consistency.|
|Personalization Objective|Global FMARL (Chen et al., 2023; Shabir<br>et al., 2023)|Agents collaboratively learn a single global model,<br>assuming similar data or objectives, suitable for fully|
|||cooperative tasks.|
||Personalized FMARL (Gao et al., 2023b;|Each agent maintains a local model adapted to its own|
||Nadiger et al., 2019; Xiong et al., 2024)|data or preferences, ofering better performance in|
|||heterogeneous or user-specifc scenarios.|
|Adaptivity to Dynamics|Static FMARL (Sun et al., 2023; Woo<br>et al., 2024)|Assumes that the environment and agent interactions<br>remain stationary during training, leading to simpler|
|||design but limited fexibility.|
||Adaptive FMARL (Mowla et al., 2020;|Incorporates mechanisms to dynamically adjust to|
||Qiao et al., 2022; Zhang et al., 2024b)|changing environments or agent behaviors, often using|
|||meta-learning (Ji et al., 2023) or online adaptation.|
|Model Structure Homogeneity|Homogeneous FMARL (Giuseppi et al.,<br>2025; Wang et al., 2020d)|All agents employ identical model architectures, enabling<br>direct parameter aggregation and simplifying coordination.|
||Heterogeneous FMARL (Ho et al., 2022;|Agents use diferent architectures due to varying|
||Sharma et al., 2022; Wang et al., 2020c)|capabilities or tasks, requiring fexible aggregation|
|||methods such as knowledge distillation.|



periodically synchronize their models with a central server or coordinator, ensuring global consistency but potentially suffering from delays caused by slower agents (stragglers). In contrast, asynchronous FMARL (Lan et al., 2024; Pan et al., 2022a; Shen et al., 2024) allows agents to update independently and at different times, providing greater flexibility and robustness, especially in heterogeneous or large-scale networks. Regarding personalization objectives, FMARL can be classified into personalized and global (or shared) paradigms. Personalized FMARL (Gao et al., 2023b; Nadiger et al., 2019; Xiong et al., 2024) targets scenarios where agents face heterogeneous goals or non-IID data distributions, aiming to learn individualized models while still leveraging federated knowledge sharing. This approach suits applications such as personalized recommendations or localized resource optimization. Conversely, global FMARL (Chen et al., 2023; Shabir et al., 2023) assumes a common objective across all agents and focuses on learning a unified global model, appropriate for fully cooperative tasks like multi-robot coordination or fleet management. From the perspective of adaptability to environmental dynamics, FMARL methods can be broadly categorized into static and adaptive approaches. Static FMARL (Sun et al., 2023; Woo et al., 2024) methods assume that the environment and data distributions remain stationary over time, which simplifies training but may limit effectiveness in real-world settings where conditions frequently change. Adaptive FMARL (Mowla et al., 2020; Qiao et al., 2022; Zhang et al., 2024b) are designed to continuously monitor environmental changes and dynamically adjust model parameters or learning strategies in response to dynamics. This adaptability is essential in real-world scenarios such as autonomous driving, smart grids, or real-time traffic control, where the environment evolves rapidly and unpredictably. By incorporating mechanisms like meta-learning, context detection, or online adaptation, these methods aim to maintain robust performance over time despite shifting task characteristics or agent behaviors. Considering model structure homogeneity, FMARL systems can be divided into homogeneous (Giuseppi et al., 2025; Wang et al., 2020d) and heterogeneous model settings (Ho et al., 2022; Sharma et al., 2022; Wang et al., 2020c). Homogeneous FMARL assumes that all agents employ the same model architecture, which facilitates straightforward aggregation of model parameters and smoother convergence. This uniformity is beneficial in environments where agents share similar capabilities and data characteristics. In contrast, heterogeneous FMARL involves agents using different model architectures or learning algorithms due to diverse local 

data types, computational resources, or task requirements. This heterogeneity necessitates more sophisticated coordination mechanisms, such as knowledge distillation or model translation techniques, to enable effective collaboration despite architectural differences. Table 2 summarizes these key single-dimension classifications and describes their principal characteristics. 

While these classification schemes are useful for highlighting specific technical aspects, they each capture only one aspect of FMARL. Given the complex interplay between data distribution, agent autonomy, and communication constraints, a more comprehensive and representative classification is based on the distribution of data and environmental information among agents. Among various schemes, one of the most fundamental and impactful is the classification according to the hierarchical structure of information exchange and learning, which leads to two dominant paradigms: Horizontal FMARL (HFMARL) (Han et al., 2019; Liu et al., 2019; Nadiger et al., 2019; Wang et al., 2020d), where agents operate within the same task or environment and collaboratively train shared models without exchanging raw data, and Vertical FMARL (VFMARL) (Gaba et al., 2024; He et al., 2024; Zhuo et al., 2019), where agents possess different feature spaces or perspectives and must jointly train models across heterogeneous information modalities. This classification not only captures the structural essence of collaboration in FMARL but also provides a unifying lens through which privacy, model aggregation, and learning objectives can be systematically analyzed. Therefore, this section mainly provides an in-depth discussion of HFMARL and VFMARL in terms of their architectures, algorithms, and applications. 

## _4.1. Horizontal federated multi-agent reinforcement learning_ 

Fig. 2 illustrates the architecture of HFMARL. HFMARL can be visualized as a series of parallel environments. In this framework, each agent operates in their own independent environments, where the environments are isolated and do not influence each other. Despite being independent, these environments share a common structure in terms of state and action spaces, enabling agents to address similar tasks. We assume that _𝑛_ agents observe their respective environments { _𝐸𝑖_ } _[𝑛] 𝑖_ =1[,][with] _𝐺_ representing the collection of all environments. Each environment _𝐸𝑖_ shares a similar model in terms of state transition probability and reward function, but remains independent of the others. Formally, the 

6 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Fig. 2.** HFMARL model. 

state spaces, action spaces, and environments are defined as: 

 _𝑖_ =  _𝑗 ,_  _𝑖_ =  _𝑗 , 𝐸𝑖_ ≠ _𝐸𝑗 ,_ ∀ _𝑖, 𝑗_ ∈{1 _,_ 2 _,_ … _, 𝑛_ } _, 𝐸𝑖, 𝐸𝑗_ ∈ _𝐺, 𝑖_ ≠ _𝑗,_ (12) 

where  _𝑖_ and  _𝑗_ are the state spaces, and _𝐴𝑖_ and _𝐴𝑗_ are the action spaces of agents _𝑖_ and _𝑗_ . The environments _𝐸𝑖_ and _𝐸𝑗_ are independent and ideally distributed in identical fashion. This assumption enables knowledge sharing and policy generalization across agents in multi-agent systems. Agents learn from their own experiences and exchange knowledge with others, even though they interact with different environments. This exchange significantly improves sample efficiency and accelerates the overall learning process. 

HFMARL can be applied in scenarios in which the agents may be geographically distributed, but they face similar decision-making tasks and have very little interaction with each other in the observed environments. For example, a typical example of HFMARL is the coordination of drones for environmental monitoring. Drones deployed in different geographical regions can independently explore and gather environmental data, such as air quality, temperature, or pollution levels. Due to factors like varying weather conditions, terrain types, and regulations across regions, no single drone can access all potential scenarios in its environment. However, all drones share similar operational tasks, including data collection, navigation, and obstacle avoidance. Through HFMARL, these drones can exchange learned models without transferring raw data, preserving privacy. Even if a drone encounters a previously unseen scenario, it can still rely on the shared model to make informed decisions. The diverse experiences of the drones from different regions improve the overall model, allowing for better coverage of rare environmental conditions and enhancing the robustness and accuracy of the system. Table 3 provides a comprehensive overview of the key characteristics of HFMARL, highlighting the essential features that make it suitable for various applications. It outlines the critical elements such as task similarity, independent observations, sufficient resources, and data privacy protection, which define the operational framework of HFMARL. Some applications that meet the above characteristics can be classified as HFMARL. Additionally, the table offers examples of real-world application scenarios where HFMARL can be effectively applied, including autonomous driving in the Internet of Vehicles (IoV), collaborative drone networks, distributed sensor systems for smart cities, and privacy-preserving healthcare data analysis. 

According to the definition of HFMARL, it is closely related to distributed MARL (Jiang et al., 2021; Matignon et al., 2012), and can also 

be viewed as a security-enhanced version of distributed MAR. Prior to HFMARL, there are many studies in distributed MARL (Espeholt et al., 2018; Nair et al., 2015). In distributed MARL, multiple agents are distributed across different physical locations or machines, each facing its own environment. Each agent typically operates in its local environment, optimizing its policy based on local data, while sharing information with a central controller or other agents. However, distributed MARL generally transfers agents’ experiences without addressing privacy concerns. Agents may share raw data, such as states and actions, or sometimes share model parameters directly, which could lead to privacy issues (Lei et al., 2023). In contrast, HFMARL introduces additional safeguards, such as privacy protection and communication consumption constraints, to better fit specific scenarios like IoT applications (Jarwan & Ibnkahla, 2022; Wang et al., 2020d), where data sensitivity is a significant concern. Another key difference lies in the handling of NonIdentically Distributed (Non-IID) data (Arafeh et al., 2022; Li et al., 2019; Zhu et al., 2021). While distributed MARL typically assumes that the environment state transitions follow an identical and independent distribution (IID), meaning all agents operate in identical environments, real-world scenarios often deviate from this assumption. Agents might face slightly different conditions, causing their environments to be NonIID. HFMARL addresses this challenge by enhancing the model’s generalization ability, ensuring convergence despite the Non-IID nature of the data. Table 4 summarizes the comparison between distributed MARL and HFMARL. The key differences between these two approaches make HFMARL offer several advantages over traditional methods. HFMARL’s ability to share model parameters rather than raw data ensures better privacy protection, while its capacity to handle non-IID data enhances its applicability in more diverse and dynamic scenarios. 

In HFMARL, each agent adopts DRL models independently tailored to its specific decision-making requirements. Based on their learning mechanisms and architectural framework, these models can be categorized into value-based, policy-based, and actor-critic approaches. Table 5 shows the categories of DRL algorithms. Value-based methods focus on estimating the value function, which predicts the expected return of being in a specific state or taking a particular action. Deep Q-Network (DQN) (Mnih et al., 2015) introduced the concept of deep learning into Q-learning, improving performance in complex environments. Double Deep Q-Network (DDQN) (Van Hasselt et al., 2016) reduces the overestimation bias inherent in DQN by decoupling action selection and evaluation. Prioritized experience replay (Sun et al., 2020) enhances 

7 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Table 3** 

Characteristics and suitable application scenarios for HFMARL. 

||Characteristic<br>Description<br> Application scenarios|
|---|---|
||**Task** **Similarity**<br>In HFMARL, multiple agents are tasked with similar decision-making<br>in diferent independent environments. While these environments<br>may difer, the state and action spaces must be the same across all<br>agents. This consistency allows the agents to share knowledge with<br>each other, thereby improving the overall learning process.<br>Autonomous driving in<br>IoV, collaborative drone<br>networks, distributed<br>sensor systems for smart<br>cities, privacy-preserving<br>healthcare data analysis.<br> **Independent** **Observations**<br>Each agent collects observations independently from its local envi-<br>ronment, with no access to other agents’ data. Due to factors such as<br>mobility, observation limitations, and energy constraints, the data<br>collected can difer, leading to unbalanced data distributions across<br>agents. In HFMARL, agents share knowledge to address data imbal-<br>ances and enhance learning efciency.<br> **Sufcient** **Resources**<br>In HFMARL, agents must have sufcient computing power, storage,<br>and communication bandwidth to support model training, real-time<br>updates, and knowledge sharing. These resources enable agents to<br>process large data volumes, maintain local models, and collaborate<br>efciently, ensuring efective learning and adaptation in distributed<br>environments.<br> **Data** **Privacy** **Protection**<br>Each agent’s data is stored locally and never shared directly with<br>other agents or a central coordinator. Privacy is ensured through<br>collaborative learning via model updates without exposing raw data.|



## **Table 4** 

Comparison between distributed MARL and HFMARL. 

|Aspect|Distributed MARL|HFMARL|
|---|---|---|
|**Data** **Sharing**|Direct sharing of experiences and raw|Shares model parameters instead of raw|
||data among agents, which may lead to|data. Alternatively, data can be encrypted|
||privacy concerns.|to ensure privacy and security.|
|**Model** **Update**|Can be either synchronous (global updates|Relies on local model updates and|
||after collecting data) (Clemente et al.,|aggregation of shared knowledge from|
||2017) or asynchronous (agents update the|agents to improve the global model.|
||global model independently) (Espeholt||
||et al., 2018; Horgan et al., 2018).||
|**Environmental** **Distribution** **Assumption**|Assumes IID of environment states for all|Capable of handling non-IID data, where|
||agents.|agents may experience diferent|
|||environments or have diferent data|
|||distributions.|
|**Privacy** **Protection**|Focuses less on privacy protection, often|Strong emphasis on privacy, avoiding|
||sharing raw data or experiences between|direct sharing of raw data by sharing|
||agents.|model parameters and incorporating|
|||encryption methods for secure|
|||communication.|



## **Table 5** 

## Categories of DRL algorithms. 

|Algorithm type|Representative algorithms|Key features|
|---|---|---|
|Value-Based|DQN (Mnih et al., 2015), DDQN (Van Hasselt|Discrete action spaces, Smaller state/action|
||et al., 2016), DDQN with Proportional|spaces, Deterministic environments, Value|
||Prioritization (Sun et al., 2020)|function-based learning.|
|Policy Gradient|REINFORCE (Williams, 1992), Q-Prop (Lan et al.,|Continuous action spaces, Complex control tasks,|
||2024)|Direct policy optimization, Gradient-based|
|||learning.|
|Actor-Critic|Actor-Critic (Konda & Tsitsiklis, 1999),|Combines value-based and policy-gradient|
||SAC (Haarnoja et al., 2018), A3C (Lillicrap, 2015),|methods, Balances exploration and exploitation,|
||DDPG (Lillicrap, 2015), PPO (Schulman et al.,|Efcient decision-making in complex|
||2017)|environments, Suitable for both discrete and|
|||continuous action spaces.|



sample efficiency by focusing on more informative transitions. Policybased methods directly optimize the policy that dictates the agent’s behavior. Algorithms like REINFORCE (Williams, 1992) use Monte Carlo methods to estimate gradients, while Q-prop (Lan et al., 2024) combines on-policy and off-policy learning to improve stability and efficiency. Actor-critic algorithms blend the strengths of value-based and policy-based methods. The actor updates the policy directly, while 

the critic evaluates the action-value function. Soft Actor-Critic (SAC) (Haarnoja et al., 2018) introduces entropy regularization for better exploration, A3C (Lillicrap, 2015) enables parallel training for faster convergence, and PPO (Schulman et al., 2017) enhances stability through constrained updates. Deep Deterministic Policy Gradient (DDPG) (Lillicrap, 2015) and its variants are particularly effective in continuous action spaces. By integrating FL mechanisms, HFMARL enables decen- 

8 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

tralized agents to collaboratively train models while preserving data privacy. 

In recent years, there has been growing interest in applying HFMARL frameworks to optimize various complex systems. Liu et al. (2019) develop an HFMARL framework specifically for cloud robotic navigation, where decentralized robots train local models while a cloud server performs knowledge fusion to continuously upgrade a shared model. To address potential structural discrepancies between local and shared models, the authors innovatively integrate transfer learning within the HFMARL framework. Experiments show that this method greatly improves the efficiency of RL for robot navigation. To support the growing demand for IoT services in mobile networks, Wang et al. (2020d) propose the HFMARL framework (FADE) for optimizing edge caching. This framework uses decentralized base stations (BSs) to collaboratively train predictive models through federated parameter interaction. Initial global parameters kick-start local DRL training, and refined models are aggregated to update the shared knowledge base. Simulation results demonstrate the effectiveness of FADE, showing a 92 % reduction in performance loss and a 60 % decrease in system payment compared to the centralized DRL algorithm. FADE also achieves just a 4 % performance loss compared to the ideal oracle algorithm, outperforming existing methods like least recently used (LRU), least frequently used (LFU), and first-in first-out (FIFO) by 7 %, 11 %, and 9 % in network performance, respectively. These examples demonstrate the versatility of HFMARL in addressing complex decision-making problems in dynamic and distributed environments, while ensuring privacy and efficient model training. 

## _4.2. Vertical federated multi-agent reinforcement learning_ 

Unlike HFMARL, VFMARL operates within a more complex framework, where multiple agents interact in a shared global environment but have access only to partial observations. In this setup, the observations and action spaces of each agent differ, and the global state is formed by aggregating the observations of all agents. Similarly, the global action space is the union of the individual action spaces of all agents, reflecting the collective decisions that influence the environment. Fig. 3 illustrates the structure of VFMARL. Specifically, the conditions for VFMARL can 

## be defined as: 

**==> picture [207 x 23] intentionally omitted <==**

**==> picture [220 x 8] intentionally omitted <==**

where  represents the global state space and  the global action space, both formed by aggregating the observations and actions of all agents. All agents operate within the same environment _𝐸_ , such that _𝐸𝑖_ = _𝐸𝑗_ = _𝐸_ for any agents. Notably, the observations  _𝑖_ and  _𝑗_ for different agents _𝑖_ and _𝑗_ often differ significantly due to varying perspectives or local information. 

According to the definition of VFMARL, VFMARL is similar to the cooperative MARL under partially observable environments (Monahan, 1982; Spaan, 2012). Before the development of VFMARL, many studies focused on cooperative MARL settings where agents operate with limited local observations (Vinyals et al., 2017; Yenkanchi, 2016). In such settings, multiple agents interact within the same environment, and each agent must learn and act based on its own partial view of the environment. The dynamic interactions among agents in partially observable multi-agent environments often lead to instability, making it difficult to learn effective policies (Lowe et al., 2017; Omidshafiei et al., 2017). To mitigate this, various methods introduce centralized mechanisms during training to help agents incorporate global context. For example, MADDPG (Lowe et al., 2017) adopts a centralized critic that accesses all agents’ observations and actions, enabling more stable policy updates under partial observability. However, as the number of agents grows, this approach faces scalability challenges and raises potential privacy concerns. MAPPO (Yu et al., 2022), an on-policy variant, also utilizes a centralized critic but relies on local observations during execution, striking a balance between coordination and decentralization. Other approaches, such as QMIX (Rashid et al., 2020) and VDN (Sunehag et al., 2017), address the credit assignment problem through value decomposition. QMIX uses a nonlinear mixing network to combine agents’ local value functions into a joint value, facilitating cooperation even without full observability. In contrast, VDN employs a linear decomposition of the global reward, simplifying coordination while still enabling agents to reason about their individual contributions. 

In some scenarios, where privacy protection requirements or data sharing restrictions are critical, Vertically Federated Multi-Agent Rein- 

**Fig. 3.** VFMARL model. 

9 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Table 6** 

Comparison between partially observable cooperative MARL and VFMARL. 

|Aspect|Partially observable cooperative MARL|VFMARL|
|---|---|---|
|**Data** **Distribution**|Agents typically have access to|Each agent holds a distinct subset of|
||homogeneous or overlapping features|features (vertically partitioned data).|
||(horizontally partitioned data).||
|**Observation** **Handling**|Agents may share raw observations or|Raw data remains local; agents exchange|
||inferred state representations.|model parameters, gradients, or|
|||encrypted representations.|
|**Data** **Sharing** **Mechanism**|Direct sharing of observations, actions, or|No raw data sharing; collaboration is|
||states during training.|achieved via federated learning protocols.|
|**Policy** **Learning**|Policies are trained using shared|Policies are collaboratively trained via|
||state-action information, possibly with a|secure model aggregation and encrypted|
||centralized critic.|knowledge sharing.|
|**Privacy** **Consideration**|Privacy is not a primary design goal;|Strong emphasis on privacy; uses secure|
||shared data may be exposed.|aggregation, encryption, or diferential|
|||privacy techniques.|



forcement Learning (VFMARL) has emerged as a novel paradigm for enabling cooperative learning among multiple agents under strict privacy constraints in partially observable environments. In VFMARL, agents are typically aligned toward a shared objective but possess distinct and complementary subsets of environmental features, reflecting a vertically partitioned data setting. To facilitate collaboration without compromising sensitive information, VFMARL leverages federated learning techniques that allow agents to jointly train models by securely exchanging model parameters, gradients, or intermediate representations rather than raw data. This enables agents to mutually benefit from each other’s knowledge while preserving data privacy and supporting heterogeneous observations. Such a framework is particularly advantageous in environments where agents have incomplete or limited observations, as aggregated knowledge sharing can effectively compensate for local information gaps. Table 6 outlines the key distinctions between traditional partially observable cooperative MARL and the VFMARL paradigm. 

As an emerging and evolving research direction, VFMARL is particularly well-suited for addressing cooperative multi-agent reinforcement learning tasks under strict privacy constraints and partial observability. Recent works have begun to explore practical frameworks and algorithms that embody the core principles of VFMARL in real-world applications. For instance, Zhuo et al. (2019) propose a Federated Deep Reinforcement Learning (FedRL) framework for scenarios in which agents are unable to share partial observations due to privacy concerns, and some agents do not have access to reward signals. Their method introduces a shared value network and leverages encrypted communication to enable collaborative learning without raw data exchange. Similarly, Mukherjee et al. (2023) design a vertically federated actor-critic algorithm (FedSAC) aimed at enhancing the cyber-resilience of networked microgrids owned by different stakeholders. The framework enables privacy-preserving coordination and is validated through extensive simulations. Furthermore, Ge et al. (2022) develop a secured DRL framework based on vertical federated learning, where clients exchange feature embeddings rather than raw inputs. This design not only strengthens privacy protection but also improves robustness against adversarial attacks. 

In the VFMARL framework, cooperative behavior among agents is enabled through privacy-preserving aggregation mechanisms such as parameter sharing, gradient aggregation, and intermediate representation fusion. These techniques bring a range of notable benefits, particularly in environments characterized by vertically partitioned data and partial observability. First, these aggregation methods ensure that collaboration can take place without requiring agents to share raw observations or sensitive local data. This not only preserves data privacy and adheres to security constraints, but also facilitates collaboration among entities that are unwilling or unable to exchange raw informa- 

tion due to regulatory, competitive, or ethical concerns. Second, aggregation mechanisms enhance the expressiveness and generalization of the learned policies. By incorporating diverse, complementary local knowledge from different agents, the global model can develop a more holistic understanding of the environment, even when each agent observes only a narrow slice of it. This is especially valuable under partial observability, where no single agent has access to complete state information. Third, these methods contribute to robustness and scalability. Parameter sharing and gradient aggregation allow for distributed training that scales with the number of agents, without requiring centralized control or global state awareness. At the same time, intermediate representation fusion offers a way to enrich learning with abstracted and compressed knowledge, reducing communication overhead while still benefiting from shared insights. Finally, the flexibility of these aggregation approaches enables seamless integration with advanced privacypreserving technologies, such as secure multiparty computation, homomorphic encryption, and differential privacy. This allows VFMARL to operate effectively in adversarial or sensitive domains while maintaining strong privacy guarantees. Overall, the use of these aggregation techniques allows VFMARL to achieve secure, scalable, and effective multiagent cooperation, even in challenging settings with limited observability and strict privacy constraints. 

## **5. Methodologies of FMARL** 

FMARL addresses the dual challenges of data privacy and distributed decision optimization in multi-agent collaborative tasks. Its core principles are centered on two main aspects: collaborative learning with privacy protection and distributed collaborative optimization. 

## _5.1. Collaborative learning under privacy protection_ 

One of the core characteristics of FMARL is that data is distributed across multiple agents. The goal is to enable agents to collaborate while ensuring data confidentiality. FMARL accomplishes this using privacypreserving techniques (Li et al., 2021, 2023c; Truex et al., 2019), which protect both local data and model updates during training and communication. 

## _5.1.1. Encryption methods_ 

Encryption-based methods are mainly used to protect the privacy of data interactions between agents (such as parameters, gradients, or intermediate features). Encryption-based methods can be classified into two main categories: Homomorphic Encryption (HE) Methods (Fang & Qian, 2021; Ma et al., 2022; Park & Lim, 2022; Zhang et al., 2020) and Secure Multi-Party Computation (SMPC) Methods. 

10 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

HE is a type of encryption that allows computations to be performed directly on encrypted data, without requiring decryption. This ensures that sensitive data remains encrypted throughout the process while mathematical operations (such as addition or multiplication) are applied to the ciphertext. After the computation, the encrypted result can be decrypted to reveal the final outcome, without exposing the underlying plaintext data. In the context of FMARL, HE enables agents to securely share and aggregate model updates or gradients without revealing their private data. For instance, each agent can compute gradients based on its local data, encrypt them, and send them to a central server. The server aggregates the encrypted gradients, and only the final result is decrypted. This method ensures that local data stays private during the entire process, protecting it from exposure during communication or aggregation. This approach is particularly valuable in scenarios where data security is paramount, as it prevents unauthorized access to the data even as it is being processed and shared among agents. For instance, in healthcare applications (Munjal & Bhatia, 2023; Wibawa et al., 2022; Zhang et al., 2022a) where patient data is involved, HE can facilitate the training of ML models without compromising patient confidentiality. 

For example, Xue et al. (2021) propose a novel framework tailored for clinical decision support systems. This framework integrates mobileedge computing and software-defined networking to tackle network congestion and responsiveness limitations. By leveraging a double deep Q-network (DDQN) with HE, the model allows edge nodes to collaboratively and securely optimize personalized treatment strategies, effectively enhancing adaptability and efficiency across heterogeneous medical data environments. Yang et al. (2024) tackles the issue of data silos in the scheduling of multiple Virtual Power Plants (VPPs) and enhances their ability to handle nonlinear and stochastic scheduling dynamics. They propose a secure federated training method that incorporates Deep Transformer Q-Networks, local DP, and CKKS HE to ensure privacy protection. Simulation results indicate that the proposed approach outperforms existing algorithms, offering strong privacy protection and improved economic efficiency for VPPs. Miao et al. (2021) propose a secure and reliable data sharing mechanism for IoT based on FL. The approach enhances data sharing efficiency by using an asynchronous multiple FL scheme with sub-task grading and DRL to select high-quality data nodes. Additionally, HE is integrated into the model parameter aggregation process to protect the privacy of data providers. The experimental results demonstrate that FL2S achieves high accuracy and privacy preservation for secure data sharing in various IoT applications 

SMPC (Bonawitz et al., 2017; Yao, 1982) is a cryptographic technique that splits data into multiple shares, which are distributed across different parties for computation. Each participant holds only a partial share of the data, preventing any single party from reconstructing the original data on its own. This ensures that the privacy of the data is preserved throughout the computation process. In the context of FMARL, SMPC can be used to securely aggregate model updates or gradients from different agents without exposing their private data. For example, in a scenario where multiple agents are collaboratively learning a joint model, each agent computes its local gradient based on its own data. Instead of sending the full gradient to a central server, each agent splits its gradient into shares and sends only a part of the gradient to the other agents. The participating agents then collaborate to compute the aggregated gradient using their shares. Since no single agent has access to the complete gradient or model parameters, the original data and model updates remain private. Finally, after the aggregation, the result can be shared and used to update the global model without compromising any agent’s private information. This approach ensures that all agents can contribute to the model’s learning while maintaining the confidentiality of their local data. The following steps illustrate how SMPC can be applied in FMARL: 

**==> picture [248 x 57] intentionally omitted <==**

where _𝑠_ represents the data samples and  is the loss function. 

- **Step 2: Gradient Splitting.** Each agent splits its local gradient ∇ _𝜃𝑖_ into shares. Let the share of agent _𝑖_ be denoted as {∇ _𝜃𝑖_ 1 _,_ ∇ _𝜃𝑖_ 2 _,_ … _,_ ∇ _𝜃𝑖𝑚_ }, where _𝑚_ is the number of shares. 

- **Step 3: Gradient Sharing.** The agent sends the shares ∇ _𝜃𝑖𝑗_ to other participating agents. Each agent only holds a part of the gradient and cannot access the complete gradient. 

- **Step 4: Gradient Aggregation.** Each agent receives the gradient shares from other agents. The agents collaboratively compute the aggregated gradient ∇ _𝜃𝑎𝑔𝑔_ using their local shares. This can be done through secure protocols, ensuring that no agent can reconstruct the full gradient from its own shares. 

- **Step 5: Global Gradient Update.** After the aggregated gradient ∇ _𝜃𝑎𝑔𝑔_ is computed, it is used to update the global model _𝜃𝑔𝑙𝑜𝑏𝑎𝑙_ . 

_𝜃𝑔𝑙𝑜𝑏𝑎𝑙_ = _𝜃𝑔𝑙𝑜𝑏𝑎𝑙_ − _𝛽_ ∇ _𝜃𝑎𝑔𝑔,_ (15) 

where _𝛽_ is the learning rate. 

- **Step 6: Model Update Without Data Exposure.** The updated global model is then shared with all agents. No agent ever exposes its raw data, ensuring privacy during the entire process. 

While there has been extensive research on the application of SMPC (Byrd & Polychroniadou, 2020; Kanagavelu et al., 2020; Sotthiwat et al., 2021) in FL, its use in FMARL remains relatively limited. Kanagavelu et al. (2020) propose a privacy-preserving FL framework using SMPC for secure model aggregation. To address the high communication overhead and low scalability of SMPC in a peer-to-peer setup, the authors introduce a two-phase mechanism: first, electing a small committee and then using the committee to provide SMPC-enabled aggregation for a larger group of participants. This framework is integrated into an IoT platform for smart manufacturing, allowing companies to collaboratively train high-quality models while preserving privacy, maintaining model accuracy, and improving execution efficiency compared to traditional methods. Sotthiwat et al. (2021) propose a novel method to reduce the communication and computation costs in SMPC. Instead of applying heavy SMPC to the entire local models, the authors suggest encrypting only the critical part of the model (gradients), thereby reducing overhead while preserving privacy. The method prevents deep leakage attacks and maintains high model accuracy. Experimental results on the MNIST and CIFAR-10 datasets show that the proposed partially encrypted SMPC approach significantly reduces costs compared to traditional SMPC, while achieving similar accuracy to conventional distributed learning with plaintext aggregation. 

## _5.1.2. Differential Privacy (DP)_ 

DP (Dwork, 2008; El Ouadrhiri & Abdelhadi, 2022; Naseri et al., 2020; Triastcyn & Faltings, 2019) offers an alternative strategy for privacy preservation by introducing randomness to the data before it is shared or utilized for model training. This method involves adding calibrated noise to the individual data points, obscuring the contribution of any single entry while preserving the overall statistical properties of the dataset. Despite this added noise, the overall statistical properties of the dataset are preserved, allowing for meaningful data analysis and accurate model building. In the context of FMARL, agents need to collaborate to optimize the global strategy by sharing gradients or model parameters. However, direct sharing can inadvertently expose sensitive characteristics of their local data. DP mitigates this risk by injecting noise into the gradients or parameters, preventing malicious attackers 

11 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

from inferring the original data through reverse engineering, thereby safeguarding privacy. This approach is particularly valuable in situations where privacy is critical but collaboration between agents is still necessary for effective learning. Two key components of DP play an essential role in achieving this privacy guarantee: noise injection (Grandvalet et al., 1997) and dynamic privacy budget adjustment (Errounda & Liu, 2023; Li et al., 2024). 

Noise injection is a fundamental technique employed in DP to protect the privacy of individual data points during model updates. In the context of FMARL, agents contribute to the global model by sharing their locally computed gradients or model parameters. To prevent adversaries from inferring sensitive information about any agent’s local data from these shared updates, controlled noise is added to the gradients or parameters before transmission. Typically, this noise is drawn from a probability distribution, such as the Laplace (Dwork et al., 2006) or Gaussian distribution (Dwork et al., 2014), which introduces randomness into the updates. Dynamic privacy budget adjustment addresses the trade-off between privacy and utility in collaborative learning. The privacy budget, represented by the parameter _𝜖_ , determines the level of noise added to each model update and thus governs the balance between protecting data privacy and maintaining model accuracy. A smaller _𝜖_ increases the noise level, providing stronger privacy protection by making it more difficult to infer individual data points. However, this also reduces the model’s ability to accurately learn from the data. Conversely, a larger _𝜖_ reduces the amount of noise, improving the model’s accuracy but at the expense of privacy. In FMARL systems, the privacy budget can be dynamically adjusted based on various factors such as the stage of learning, the sensitivity of the data, and the importance of privacy in the given scenario. For instance, in the early stages of training, a smaller _𝜖_ might be chosen to protect privacy more strongly, while later stages could allow for a larger _𝜖_ to enhance the model’s accuracy without significant privacy loss. This dynamic adjustment ensures that FMARL systems can adapt to the specific needs of the learning task, optimizing the trade-off between privacy and model performance. By incorporating these mechanisms, DP allows FMARL systems to perform secure, privacy-preserving collaborative learning while enabling effective data analysis and model training. McMahan et al. (2017) combine DP with federated averaging, adding noise to local gradients before aggregating them. A key contribution of this work is showing that FL can reduce communication costs by 10-100x compared to synchronized stochastic gradient descent, making 

it more efficient for resource-constrained environments while protecting data privacy. Geyer et al. (2017) address the challenge of differential attacks in FL, where clients’ contributions and data may be exposed through the aggregated model. To mitigate this, the authors propose an algorithm for client-side DP during federated optimization. The goal is to protect clients’ data while maintaining a balance between privacy and model performance. Empirical studies show that with a sufficiently large number of clients, the proposed approach can effectively preserve client-level DP with only a minor trade-off in model performance. 

Both encryption-based methods and DP techniques offer robust solutions for privacy preservation in FMARL, each with unique strengths and suitable applications. Encryption methods excel in securing data during transmission, making them particularly valuable in scenarios where data integrity and confidentiality are paramount, such as financial transactions or personal health records. In contrast, DP techniques provide strong safeguards against re-identification risks, making them ideal for applications where shared insights are essential, yet individual privacy must be maintained. The integration of these privacy preservation methods in FMARL not only enhances the security of sensitive data but also fosters collaboration among agents across diverse applications, from healthcare to finance, while addressing privacy concerns. For example, Byrd and Polychroniadou (2020) combines DP and SMPC to protect sensitive client data during collaborative ML. While DP adds noise to model parameters to safeguard privacy, SMPC further reduces the risk of private data exposure without compromising model accuracy. The system is particularly relevant for financial firms, allowing collaborative learning on tasks such as trade execution, credit origination, or fraud detection. The paper demonstrates the approach using logistic regression on a real-world credit card fraud dataset and evaluates it on an open-source simulation platform. As FMARL continues to evolve, further research will be necessary to refine these techniques and develop hybrid approaches that leverage the strengths of both methodologies. 

To provide a clear comparison of commonly used privacy-preserving techniques in FMARL, we summarize the key characteristics of various methods in Table 7. This comparison covers core mechanisms, strengths, limitations, typical application scenarios, and performance considerations. Practitioners can refer to this summary to select the most suitable method based on their specific privacy requirements, computational resources, and system constraints. 

**Table 7** 

Comparison of privacy-preserving methods in FMARL. 

|Methods|Homomorphic Encryption|Secure Multi-Party Computation|Diferential Privacy (DP)|Hybrid approaches|
|---|---|---|---|---|
||(HE)|(SMPC)|||
|Core Mechanism|Perform arithmetic|Split data into shares, distribute|Add calibrated noise to|Combine encryption and|
||operations on encrypted data|to parties; no single party sees|gradients or parameters to|noise injection for balanced|
||without decryption|full data|prevent inference of|security and privacy|
||||individual data||
|Advantages|Very high privacy; no data|No trusted third party needed;|Protects privacy while|Balance between security and|
||exposure during process;|supports complex computations;|retaining model utility;|computational efciency;|
||suitable for complex|protects data privacy|fexible noise level|improved resistance to|
||computations||adjustment|attacks|
|Disadvantages|High computational and|High communication complexity;|Noise reduces model|Complex design; still|
||communication overhead;|poor scalability; complex|accuracy; trade-of between|relatively high computation|
||slow processing|protocols|privacy and performance|and communication costs|
|Typical Application Scenarios|High privacy demand felds:|Multi-party collaboration in|Data sharing with privacy|Highly sensitive data|
||healthcare, fnance|semi-trusted networks, e.g.,|needs, e.g., user behavior|collaboration: fnancial risk|
|||industrial IoT|analysis, intelligent transport|control, smart grid|
|||||scheduling|
|Performance and Suitable Conditions|Highest privacy; best with|Efcient with limited|Flexible; suitable for|Combines advantages;|
||abundant resources;|participants; performance drops|large-scale distributed|suitable for complex|
||unsuitable for real-time or|with high communication|systems; good when privacy|scenarios; better for|
||resource-limited systems|latency; suitable for small to|requirements moderate|resource-rich, high-security|
|||medium scale||environments|



12 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Table 8** Comparison of parameter/gradient sharing methods in FMARL. 

|Methods|Full parameter sharing|Gradient sharing|Encrypted sharing|
|---|---|---|---|
|Core Mechanism|Share entire model parameters|Share gradients computed on local|Share encrypted parameters or|
||with a central server or peers|data instead of full parameters|gradients using HE or similar|
||||methods|
|Advantages|Simple to implement; enables fast|Reduces information exposure;|Strong privacy guarantees; secure|
||convergence with centralized|generally lower communication cost|aggregation without data|
||aggregation|than full models|exposure|
|Limitations|May leak sensitive information;|Gradients can still leak information,|High computational and|
||needs privacy enhancement (e.g.,|especially in high-dimensional cases|communication overhead; slower|
||DP, HE)||updates|
|Performance and Suitable Conditions|High efciency in low-latency,|Suitable for scenarios balancing|Best for high-privacy,|
||secure environments with|privacy and performance; often|resource-rich applications where|
||moderate privacy concerns|combined with noise injection (DP)|security outweighs speed|



## _5.2. Distributed collaborative optimization mechanisms_ 

In FMARL, optimizing global decision-making in a distributed environment without centralizing data storage presents a core challenge. To address this issue, efficient and secure collaborative learning between agents becomes essential. Effective collaboration is achieved through two key aspects: Parameter/Gradient Sharing Modes, which enable agents to communicate updates while ensuring privacy, and Federated Aggregation Algorithms, which aggregate the locally trained models into a global model without exposing sensitive data. These mechanisms are crucial for enabling agents to work together efficiently, preserving privacy, and minimizing communication overhead while maintaining high-quality decision-making across the network. 

## _5.2.1. Parameter/gradient sharing methods_ 

In FMARL, agents aim to collaborate on improving a global model by sharing updates without directly exchanging raw data. There are several methods in which agents can share their updates, each offering distinct trade-offs in terms of privacy protection, communication costs, and computational efficiency. Table 8 presents a comparative overview of parameter and gradient sharing methods used in FMARL. 

- **Full Parameter Sharing** (Jia et al., 2024; Xie et al., 2024). In this mode, agents send their entire model parameters (such as weights and biases) to a central server or directly to other agents for aggregation. While simple and efficient, this approach may expose sensitive information embedded within the parameters, especially if attackers can reverse-engineer the models. Therefore, privacypreserving methods such as DP or HE are often used to mitigate this risk. 

- **Gradient Sharing** (Lan et al., 2024; Yao et al., 2019). Instead of sharing entire model parameters, agents can share the gradients of their models computed based on their local data. This approach reduces the amount of information exchanged, as gradients typically contain less sensitive information than full model parameters. However, gradient sharing still carries the risk of privacy leakage, particularly when high-dimensional gradients are involved. To counter this, privacy techniques such as noise injection (via DP are often applied to the gradients before sharing. 

- **Encrypted Sharing** (Fang & Qian, 2021; Park & Lim, 2022). To provide stronger privacy guarantees, agents can share their model parameters or gradients in encrypted form. Methods like HE allow computation on encrypted data, ensuring that sensitive information is never exposed during aggregation. However, encrypted sharing often comes with high computational and communication overheads, making it suitable for scenarios where privacy is of utmost importance and the additional cost can be justified. 

## _5.2.2. Federated aggregation algorithms_ 

FMARL uses aggregation algorithms to combine updates from multiple agents, updating the global model to reflect the collective knowledge of all participants. Several federated aggregation algorithms are designed to enhance the convergence speed, stability, and performance of the global model. Table 9 summarizes several widely adopted federated aggregation algorithms in FMARL, highlighting their core mechanisms, advantages, limitations, and typical application scenarios. 

- **Federated Averaging (FedAvg)** (McMahan et al., 2017; Wang et al., 2020d). The most commonly used aggregation algorithm in FL is FedAvg, where the model parameters from each agent are averaged to compute the global model. This method assumes that all agents have relatively balanced data distributions. In some cases, agents might have data of different qualities or sizes, meaning that not all agents should contribute equally to the global model. Weighted FedAvg (Beaussart et al., 2021; Hong et al., 2022; Li et al., 2023d) introduces a weighting scheme that allows for more efficient aggregation by assigning a weight to each agent’s update based on factors like data size or update accuracy. This ensures that agents with betterquality data or more accurate updates have a greater influence on the global model. Jin et al. (2022) focus on the environment heterogeneity in FMARL, where each agent operates in a distinct environment with different state transitions. To optimize the overall performance across all environments, the authors propose two federated RL algorithms, QAvg and PAvg. They prove that these algorithms converge to suboptimal solutions, with the degree of suboptimality dependent on the extent of environment heterogeneity. 

- **Federated Proximal (FedProx)** (Cui et al., 2024; Li et al., 2020). This algorithm introduces a proximal term to the objective function to address the challenge of heterogeneous data across agents. The idea is to add a regularization term to the local objective function to avoid large divergences between agents’ local models. FedProx improves upon FedAvg by providing better robustness in scenarios where data distributions across agents vary significantly. This is particularly useful in FMARL settings, where agents may have highly diverse local environments or task-specific data. Sahoo et al. (2024b) introduce AdaFedProx, a FL method that addresses performance degradation in heterogeneous environments. AdaFedProx uses a RL-based strategy to dynamically adjust a proximal term in the objective function, incorporating client-specific information such as data distribution, system specifications, and performance feedback. This enables more effective regularization during federated training. 

- **Personalized Federated** (Deng et al., 2020; Fallah et al., 2020; Tan et al., 2022). This method goes beyond the basic federated averaging approach by introducing personalized components, such as adapters or local layers, that allow each agent to maintain a personalized 

13 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

**Table 9** 

Comparison of federated aggregation methods in FMARL. 

|Methods|FedAvg|FedProx|Personalized federated|Adaptive federated|
|---|---|---|---|---|
|Core Mechanism|Average model|Adds proximal regularization|Combines global model with|Dynamically adjusts|
||parameters equally or|to reduce model divergence|personalized local|aggregation frequency|
||weighted by data size||components|and participant selection|
|Advantages|Simple,|Better handles data|Enables personalization and|Enhances convergence,|
||communication-efcient,|heterogeneity and robustness|adapts to Non-IID data|balances resources,|
||widely used|||adapts to environment|
|||||changes|
|Limitations|Assumes balanced data;|Adds computational|More complex architecture;|Requires feedback|
||sensitive to|overhead; needs proximal|higher|mechanism and tuning;|
||heterogeneity|term tuning|communication/computation|higher complexity|
||||cost||
|Performance and Suitable Conditions|Efcient with similar|improves stability; suitable|Ideal for Non-IID data like|Useful in dynamic or|
||data quality; suitable for|for highly heterogeneous|personalized energy or|resource-constrained|
||IID or mildly|environments|healthcare|environments like edge|
||heterogeneous data|||computing|



model while still leveraging a shared global model. This approach is useful when agents’ data is highly heterogeneous (Non-IID) and requires individualized strategies while benefiting from global knowledge. Deng et al. (2020) present a privacy-preserving, cloud-free residential energy management system (EMS) using personalized federated deep reinforcement learning (PFDRL). The key innovation is the personalized approach, where a split neural network with base and personalization layers is used to optimize EMS performance for each individual household. The system employs a decentralized FL framework to keep data and models local, while enabling collaborative training across residences. Evaluations on the Pecan Street dataset show that PFDRL significantly outperforms centralized and conventional methods, particularly in terms of personalization for each client. 

- **Adaptive Federated** (Liu et al., 2021; Wang et al., 2019; Wu et al., 2023). Recent advancements in federated aggregation involve dynamic strategies that adjust based on the environment or the agents’ capabilities. Adaptive FL algorithms adjust the aggregation process based on feedback from the agents. For example, adjusting the number of local training rounds or the frequency of aggregation based on the convergence progress or the communication cost can significantly improve the overall system performance. In edge computing, the frequency of local updates and global aggregation impacts model convergence and resource usage. Wang et al. (2019) propose a control algorithm that adjusts the balance between local updates and global aggregation based on convergence boundaries, minimizing the loss function within the resource budget. Liu et al. (2021) design an adaptive asynchronous FL framework using DRL, which dynamically adjusts the proportion of nodes participating in aggregation each round and mitigates model obsolescence with a delay compensation mechanism. 

By integrating privacy-preserving techniques with federated aggregation optimization strategies, FMARL enables multi-agent systems to collaborate while ensuring data security and efficiency. The use of encryption, DP, and FL algorithms guarantees both privacy protection and scalability, making FMARL a powerful tool for tackling complex, realworld MARL tasks. 

## **6. Applications of FMARL** 

In this section, we provide an overview of existing work, examining how FMARL has been used to improve system efficiency, strengthen data security, and improve decision-making in distributed environments. Through specific use cases, we illustrate the practical implementations of FMARL and the benefits it brings to real-world scenarios. By analyzing current advances, this section offers insight into 

the growing role of FMARL in various sectors and sets the foundation for a subsequent discussion of challenges and future research directions. 

## _6.1. Intelligent transport systems (ITS)_ 

FMARL plays a crucial role in optimizing ITS within IoT-enabled smart cities. By allowing decentralized agents, such as vehicles, traffic lights, and roadside sensors, to collaborate, FMARL improves traffic flow, reduces congestion, and improves safety. It allows agents to adjust their behaviors in real-time based on local information while ensuring data privacy by sharing only model updates, not sensitive data. This decentralized approach paves the way for more efficient traffic management, ride-hailing coordination, and autonomous vehicle integration. 

## _6.1.1. Traffic management and signal control_ 

Effective traffic management and signal control are essential for alleviating urban congestion. Recent advances in FMARL have demonstrated significant potential in overcoming the limitations of traditional MARL methods, especially in complex urban traffic environments. FMARL enables decentralized agents, such as traffic lights and vehicles, to collaborate and adapt in real-time while maintaining data privacy. This results in more efficient coordination of traffic signal timings across multiple intersections, optimizing traffic flow and reducing delays. By operating in a privacy-preserving and scalable manner, FMARL offers an effective solution for dynamic and responsive traffic management in modern smart cities. 

Several recent studies have explored the application of FMARL in traffic signal control, each addressing different challenges and offering unique contributions. For example, Ye et al. (2021) propose FedLight, a federated RL approach for multi-intersection traffic signal control, aiming to overcome the limitations of traditional DRL methods that focus on individual intersections. By enabling decentralized knowledge sharing among agents, FedLight enhances collaboration across intersections, leading to improved optimization in complex traffic scenarios. Experiments using the Cityflow simulator Zhang et al. (2019a) demonstrate that FedLight improves both the convergence rate and control quality, resulting in faster optimization and better vehicle travel times. However, the study does not address potential scalability issues when applied to large-scale urban networks. This work Bao et al. (2023) introduces an FL-based RL approach that addresses the drawbacks of both centralized and distributed learning in traditional MARL. The model divides the RL network into global and local feature extraction layers, allowing the global layers to share knowledge through FL while the local layers finetune on each agent. Evaluations on traffic networks in Cologne, Germany, and Monaco show significant improvements over independent 

14 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

DQN methods. The convergence performance is enhanced by 2.29 %, and key traffic metrics such as the halting number of vehicles, waiting time of the first vehicle in each lane, and average cumulative waiting time are reduced by 39.95 %, 55.65 %, and 64.48 %, respectively. This approach demonstrates superior performance in traffic flow efficiency and travel time, with potential for further optimization through advanced aggregation algorithms and meta-learning. Fu and Di (2023) propose the use of FMARL for adaptive traffic signal control to address urban traffic congestion. Unlike centralized MARL, which incurs high communication costs and requires large-scale data, FedRL reduces communication costs while maintaining collaborative control across intersections. Through experiments conducted on real-world road networks, the paper demonstrates that FedRL outperforms both centralized and distributed RL methods, offering superior efficiency in optimizing traffic flow and mitigating congestion. Each of these studies highlights the potential of FMARL in optimizing traffic signal control, but also points to challenges such as scalability, data heterogeneity, and the complexity of real-world applications. Despite these challenges, FedRL presents a promising solution for more efficient, decentralized, and privacypreserving traffic management systems. Kumar et al. (2017) introduce Federated Control with Reinforcement Learning (FCRL), a framework that combines hierarchical and multi-agent deep reinforcement learning to address coordination problems among multiple agents in a semidecentralized model. The framework introduces a meta-controller that guides communication between agent pairs, allowing efficient exploration and learning of globally optimal policies even as the number of agents increases. FCRL is designed to tackle real-world coordination problems like distributed scheduling and urban traffic control. 

## _6.1.2. Vehicular networks_ 

In the evolving landscape of vehicular networks, enhancing communication efficiency and data management is crucial for optimizing performance. Recent studies have made significant strides in this area. For instance, Wu et al. (2024) introduce a federated caching strategy aimed at improving vehicular edge caching efficiency. Their approach predicts vehicle connectivity patterns and strategically optimizes the selection of caching vehicles, resulting in substantial improvements in caching hit rates and reduced access latency. Building on this foundation, Wang et al. (2024) propose a Federated Graph Neural Network MARL (FGNNMADRL) framework. This framework not only optimizes the age of information in vehicular edge computing scenarios but also prioritizes the protection of vehicle data privacy, highlighting the dual focus on performance and security. Further advancing resource allocation in vehicular networks, Liu et al. (2024) present a Distributed Resource Allocation algorithm utilizing FMARL (DRAFRL) for platoon-based vehicular networks within NR-V2X environments. By employing the FedAvg mechanism for model aggregation, their approach effectively reduces variance among agents, leading to improved transmission performance. Simulation results underscore this success, demonstrating a remarkable variance reduction of 93.5 % and 99.1 % compared to baseline methods, while enhancing overall resource allocation efficiency. In the realm of vehicular communications, Li et al. (2022b) tackle the issue of channel selection and power control in vehicle-to-vehicle (V2V) communications. Their proposed approach, combining FL with MARL (FedMARL), allows for dynamic adaptation to changing environments, improving transmission rates and overall communication efficiency in V2V scenarios. This approach is particularly effective in highly dynamic environments, where traditional centralized control methods struggle to keep up with the rapidly shifting network conditions. Moreover, the concept of cooperative perception emerges as a vital component in extending a vehicle’s sensing range beyond its line-of-sight. In this context, Abdel-Aziz et al. (2020) introduce a RL-based content selection method designed for cooperative perception. Their method employs a quadtree-based point cloud compression technique to optimize the sensory data shared among vehicles. By integrating federated RL, this approach not only enhances training efficiency but also enables vehicles to maximize their satisfac- 

tion with received sensory information. Notably, it improves learning speed compared to traditional non-federated methods, underscoring the advantages of collaborative frameworks in vehicular networks. 

## _6.1.3. Resource management_ 

Effective resource management is a fundamental challenge in ITS, particularly in scenarios where the efficient allocation of spectrum, energy, and computational resources is vital for optimizing performance. Recent research has explored novel approaches that leverage FL and MARL to address these issues. Chu et al. (2022) develop a decentralized framework aimed at managing the charging of plug-in electric vehicles (PEVs). Their approach employs federated DRL to coordinate charging tasks across distributed nodes, which helps decentralize control, stabilize the system, and reduce peak loads. This framework not only benefits the electric grid but also enhances economic outcomes for PEV owners, while preserving privacy by allowing collaborative decisionmaking without centralized data collection. Li et al. (2023a) address resource allocation challenges in the 6G Internet of Vehicles (IoV) ecosystem, particularly in vehicle-to-UAV (V2U) communications. Their solution, based on a Fed-D3QN algorithm, combines FMARL with dueling double deep Q-networks (D3QN). This approach optimizes channel selection and power control, effectively reducing latency and improving transmission success rates–key factors for ensuring the smooth operation of IoV services in dynamic, air-ground integrated networks. In air-ground cooperative networks, Wu et al. (2022c) introduce a federated multi-agent deep deterministic policy gradient (F-MADDPG) algorithm to optimize the trajectories of unmanned aerial vehicles (UAVs) and unmanned ground vehicles (UGVs) during emergency operations. By maximizing spectrum efficiency and using federated averaging to accelerate convergence, their method significantly improves communication performance in disaster-stricken areas where conventional infrastructure is often compromised. They further propose a distributed variant (DF-MADDPG), which reduces communication overhead and achieves substantial gains in trajectory optimization, as demonstrated by their simulation results. Zhang et al. (2019c) address challenges in cellular vehicle-to-everything (V2X) communication for safety-critical applications by proposing a joint optimization framework for transmission mode selection and resource allocation. They model the problem as a Markov decision process and develop a decentralized DRL algorithm to improve network capacity while ensuring low latency and high reliability for vehicle-to-vehicle (V2V) pairs. Their two-timescale federated DRL approach, combining vehicle clustering with FL, outperforms existing methods and efficiently supports newly activated V2V pairs in dynamic environments. 

## _6.2. Internet of Things (IoT)_ 

The rapid proliferation of IoT devices has created a highly interconnected and distributed ecosystem. These devices, ranging from sensors and actuators to edge devices, generate vast amounts of data and require complex coordination to operate efficiently. The sheer scale and heterogeneity of IoT networks pose significant challenges, especially when it comes to centralized decision-making and data privacy. FMARL offers a promising solution to these challenges by enabling distributed devices to learn collaboratively without requiring centralized control, thereby improving scalability and ensuring privacy-preserving interactions. This section explores the applications and potential of FMARL in the context of IoT systems, addressing key challenges and demonstrating its applicability in various IoT use cases. 

## _6.2.1. Resource allocation optimization_ 

Several studies have also concentrated on optimizing resource allocation in IoT contexts. For instance, Guo et al. (2022) explore resource allocation in D2D-enabled 6G networks, using FL to improve network capacity and reduce energy consumption. Their approach effectively balances the quality of service (QoS) needs of both cellular and D2D 

15 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

users, leading to marked improvements in network performance and spectrum utilization. Tianqing et al. (2021) propose a concurrent federated reinforcement learning method that addresses challenges such as limited information at edge hosts and privacy concerns associated with central servers. This method combines the privacy benefits of FL with the efficiency of DRL, using joint decision-making to arrive at global solutions. Similarly, Nguyen et al. (2020) leverage a DQN (Mnih et al., 2015) to optimize energy use and WiFi channel decisions without prior network knowledge, successfully maximizing transmission success rates while minimizing energy and channel costs. Nie et al. (2021) propose the FMARL algorithm for UAV-enabled multi-access edge computing systems, addressing the sum power minimization problem through joint optimization of resource allocation, user association, and power control. The proposed semi-distributed MAFRL framework protects user privacy using Gaussian differentials and achieves performance comparable to centralized MARL while reducing operation time by 23 %. This solution enhances both communication and computation efficiency in space-aerial-terrestrial integrated systems. Xu et al. (2021) introduce a MARL incentive mechanism to tackle nonstationary challenges in FL within heterogeneous intelligent cyber-physical systems (ICPS). By using a Stackelberg game for secure communication and resource allocation, and modeling the problem as a Dec-POMDP, the method improves policy learning without compromising privacy. The algorithm shows strong convergence and outperforms baseline methods, significantly enhancing FL performance in dynamic environments. Zhang et al. (2021c) propose a “Reinforcement on Federated” (RoF) scheme using MARL to optimize device selection and resource allocation in FL for IoT networks. The decentralized, three-layer FL architecture improves efficiency and minimizes evaluation loss, delay, and energy consumption, outperforming existing methods. Wang et al. (2023) introduce a FMARL framework for UAV-assisted MEC systems, optimizing resource allocation to minimize information delay and improve task processing. Their approach enhances data freshness and adaptability, showing superior performance over other algorithms. 

tional approaches in minimizing task residence time, end-to-end delay, and system cost. 

## _6.2.3. Energy management_ 

In the realm of smart homes, Lee and Choi (2020) introduce a federated reinforcement learning framework for managing energy consumption, integrating home appliances, solar systems, and energy storage solutions. Their innovative distributed DRL model features local home energy management systems (LHEMSs) that work in conjunction with a global server (GS). This design allows DRL agents to upload local models based on energy consumption data, which the GS aggregates to enhance the overall model’s performance. The results show significant gains in convergence speed, energy efficiency, and scalability across diverse home environments. FMARL also proves beneficial in smart buildings, where multiple agents (such as thermostats, lighting systems, and security cameras) collaborate to optimize energy use, comfort, and security. By learning cooperative policies, these devices can adjust settings like temperature and lighting in real-time while maintaining privacy. Qiu et al. (2023) apply a MARL approach to an energy and carbon allowance trading system within a building community. They address the complexities of multi-energy systems and privacy challenges in decentralized energy management using an abstract critic network combined with a deep deterministic policy gradient method integrated with FL. Their results indicate a 5.87 % reduction in total energy costs and an 8.02 % reduction in environmental costs, underscoring the effectiveness of their approach in promoting low-carbon energy solutions. Lee et al. (2021) presents a privacy-preserving energy management framework for a shared energy storage system (SESS) serving multiple smart buildings, leveraging federated reinforcement learning. The distributed DRL approach enables local building energy management systems (LBEMSs) to share parts of their trained neural networks with a global server, protecting consumer data privacy. The global server optimizes local training and manages SESS operations, demonstrating effective energy consumption optimization for heating, ventilation, and air conditioning systems across diverse building environments while maintaining privacy. 

## _6.2.2. Task offloading optimization_ 

As the computational demands of IoT applications continue to rise, FMARL offers effective strategies for optimizing task offloading across edge devices, which helps balance computational load and energy usage. For example, Ren et al. (2019) deploy multiple agents on edge nodes to manage the offloading decisions of IoT devices. This approach not only conserves energy but also ensures service quality by directing resource-intensive tasks to edge nodes. The system adapts in real time to varying workloads and network conditions, leveraging FL to train agents collaboratively. Similarly, Chen and Liu (2022) address the need to minimize energy consumption while meeting latency and resource constraints. They frame this challenge as a joint optimization problem that involves task offloading and resource allocation. To protect privacy in multi-access edge computing, they introduce a two-timescale federated DRL method using the deep deterministic policy gradient (DDPG) technique. Wu et al. (2022b) present FedAdapt, an adaptive FL framework that tackles efficiency challenges like computational limits, heterogeneity, and variable network bandwidth. By utilizing PPO, they effectively determine which deep neural network (DNN) layers should be offloaded to servers. Zang et al. (2022) propose FDOR, a federated DRL-based method for online task offloading and resource allocation in MEC systems, which incorporates wireless powered communication to enable base stations to provide energy to edge devices. Shabir et al. (2023) tackle the challenge of task offloading in vehicular fog computing, where high mobility and dynamic network topologies demand quick decision-making. They propose a FedMARL framework that optimizes task-offloading decisions using local models at vehicular nodes and a global model at fog servers. This collaborative approach improves convergence speed and reduces communication overhead while enhancing privacy. Comparative results show that their method outperforms tradi- 

## _6.3. Health care_ 

FMARL holds significant promise for advancing healthcare, particularly in addressing critical issues such as data privacy, the inherent heterogeneity across healthcare systems, and learning efficiency. In medical image diagnostics, for instance, patient data can vary significantly between institutions due to differences in imaging equipment, demographics, and diagnostic practices, leading to a non-IID data problem. In the healthcare domain, FMARL addresses key challenges in privacy, data heterogeneity, and efficiency, improving both model performance and resource optimization across diverse medical environments. 

To solve the problem of data heterogeneity, Sahoo et al. (2024a) introduce a FMARL framework to mitigate biases in heterogeneous medical data while ensuring privacy. Their approach introduces a custom loss function that ensures fairness in model aggregation, ultimately yielding a more robust global model. By integrating MARL, they enable individual healthcare agents (e.g., hospitals or diagnostic centers) to optimize their local models while contributing to a collective, privacy-preserving global model, which leads to improved performance on diverse data distributions. Sahoo et al. (2024b) develop AdaFedProx to address both data and system heterogeneity in smart healthcare applications. AdaFedProx leverages reinforcement learning to dynamically adjust a proximal term based on each client’s characteristics, thereby enhancing model stability across diverse healthcare environments and minimizing the need for intensive hyperparameter tuning. Their results demonstrate AdaFedProx’s superiority over conventional FL approaches, particularly when handling data from multiple sources with differing characteristics. 

To improve the efficiency, Seid et al. (2023) propose a FMARL framework tailored for resource allocation in UAV-enabled healthcare sys- 

16 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

tems, which is vital for remote and underserved areas. By treating resource allocation as a Markov decision process, the framework enables efficient computation offloading for Internet of Medical Things (IoMT) devices. This approach reduces latency and energy consumption while preserving data privacy, making it especially suitable for high-stakes healthcare scenarios where responsiveness and accuracy are paramount. The experimental results underscore the potential of the frameworks to deliver scalable, privacy-preserving solutions in healthcare, improving patient outcomes and optimizing resource utilization in dynamic environments. This paper (Mohammadi et al., 2017) introduces the Adaptive Federated Reinforcement Learning-Enabled System, designed to optimize kidney disease image processing in IoT-enabled healthcare. AFRLS integrates FL with DRL to enhance processing efficiency across cloud and fog nodes, significantly reducing delays in data handling and resource consumption. This paper (Tam et al., 2022) presents a FL architecture optimized for intelligent healthcare services, integrating DRL to manage resource allocation, slicing, and edge computing in large-scale IoHT networks. By prioritizing communication reliability, privacy, and delay requirements, this approach enhances bandwidth, energy, and computing efficiency across real-time and non-real-time healthcare applications, including telemedicine and telesurgery. 

These frameworks provide a comprehensive approach to optimizing healthcare systems through FMARL, improving performance across diverse medical data sources and enhancing real-time decision-making capabilities. 

## _6.4. FMARL for other applications_ 

In virtual reality (VR) services, where Liu et al. (2023) introduce a joint caching, computing, and communication (3C) strategy specifically tailored to meet the unique demands of VR environments. This innovative framework operates within a three-layer communication system that includes a cloud server, UAV-based mMIMO edge servers, and mobile devices. By optimizing service routes based on user location and content preferences, their approach achieves impressive reductions in delays (17.2 %) and energy consumption (23.5 %). In the realm of dynamic spectrum access (DSA), Chang et al. (2023) introduce Fed-MADRL, which enhances spectrum utilization while ensuring data privacy. By sharing only quantized information, this approach minimizes communication overhead and represents a significant advancement in DSA networks, outperforming traditional independent learning methods. Coordination among multiple agents is another critical area where FMARL shows promise. In the context of security, Moudoud et al. (2024a) develop MAF-DRL to bolster intrusion detection in Wireless Sensor Networks (WSNs). By integrating FL with MARL, their framework provides efficient and privacy-aware attack detection, adapting to the dynamic nature of network conditions through a trust-based scheduling mechanism. Similarly, Lin et al. (2023) tackle the joint edge association and power allocation problem in wireless networks. Their FMARL solution, based on a Dec-POMDP, enables effective policy learning without compromising sensitive data, thereby balancing connectivity and cost. Efficiency and scalability are also critical considerations. Zhang et al. (2022b) introduce a FMARL framework that enhances model accuracy, processing latency, and communication efficiency through MARL for runtime client selection. This framework demonstrates substantial operational improvements, making it well-suited for real-world applications. Additionally, Hu et al. (2021) present a federated mean-field MARL framework, which enables agents to share parameters for consistency across large-scale networks, effectively addressing the challenges of decentralized decision-making. The exploration of edge intelligence is further exemplified by Li et al. (2022a), who investigate computation task offloading within resource-constrained environments. Their cooperative architecture significantly enhances multi-user quality of experience (QoE) while reducing overall system costs. In a similar vein, Ossongo et al. (2024) propose a model utilizing network slicing and FMARL to optimize Quality of Service (QoS) in 5G networks, partic- 

ularly for LoRa devices. Their approach efficiently manages resources to minimize energy consumption while maximizing throughput. Lastly, addressing communication efficiency in FL, Cao et al. (2024) introduce FedQMIX, a node selection algorithm that adapts to non-IID data conditions. Their clustering algorithm effectively reduces communication rounds, demonstrating strong performance across various benchmark datasets. Complementing this work, Banerjee et al. (2023) develop a MARL algorithm for access point selection in cell-free massive MIMO systems, which efficiently optimizes user service in dynamic environments, achieving performance levels comparable to centralized systems. 

In summary, the diverse applications of FMARL highlight its versatility and potential to tackle complex challenges across multiple domains. As research in this field progresses, FMARL is likely to play a foundational role in driving future innovations. 

## **7. Challenges and future directions of FMARL** 

In the previous section, we underscored the increasing importance of FMARL in enabling a wide range of applications. However, despite its promising potential, FMARL faces several critical challenges that must be addressed to achieve successful real-world deployment. One of the foremost challenges is the need for efficient communication and coordination among distributed agents, particularly when dealing with large-scale, decentralized systems. Additionally, ensuring privacy and security in data-sharing processes while maintaining model performance presents a significant trade-off. The heterogeneity of data across agents further complicates the learning process, as traditional aggregation methods often fail in non-IID settings. Scalability and computational efficiency also remain pressing concerns, especially as the number of agents increases. Addressing these challenges not only paves the way for improving FMARL’s capabilities but also opens up new research directions, including advancements in privacy-preserving techniques, robust aggregation methods, and the development of more scalable and adaptive learning frameworks. These areas represent exciting opportunities for future work, offering the potential to unlock even broader applications of FMARL in complex, dynamic environments. 

## _7.1. Data heterogeneity_ 

In practical FMARL applications, agents or nodes typically have access to locally diverse datasets, each with its unique characteristics. This heterogeneity often leads to significantly varied data distributions, making it difficult for traditional aggregation techniques, such as FedAvg, to be effective. When agents encounter non-IID data, constructing a global model that accurately represents the entire learning environment becomes a major challenge. 

To address this, one promising approach is the development of personalized federated reinforcement learning methods (Deng et al., 2020; Fallah et al., 2020; Tan et al., 2022). These approaches allow each agent to maintain a locally tailored model, while still benefiting from global updates. Techniques such as meta-learning (Hospedales et al., 2021) and multi-task learning (Zhang et al., 2021a) can enhance the adaptability of agents across heterogeneous nodes, enabling better performance in non-uniform environments. Another line of research focuses on robust aggregation algorithms that are specifically designed to handle non-IID data (Zhao et al., 2018). For instance, weighted aggregation methods (Beaussart et al., 2021; Hong et al., 2022; Li et al., 2023d) that consider the data distribution at each node, or cluster-based aggregation, where agents with similar data distributions are grouped prior to aggregation, can significantly improve the accuracy and efficiency of the global model. Additionally, federated Bayesian learning (Dai et al., 2024; Kober et al., 2013) provides a probabilistic framework for dealing with uncertainty across diverse data sources, offering another promising direction for adapting models to each agent’s unique local environment. By refining these approaches, FMARL systems can effectively han- 

17 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

dle data heterogeneity, resulting in more accurate and robust learning outcomes. 

## _7.2. Communication efficiency_ 

Communication efficiency (Krouka et al., 2021; Qiao et al., 2024) is a key challenge in FMARL systems, particularly when scaling up to a large number of agents or operating in environments with limited communication resources. The frequent transmission of model updates, especially for large models, can lead to significant bandwidth consumption and high latency, which is further exacerbated in geographically distributed systems. This communication overhead not only affects the speed of information exchange but can also degrade overall system performance, as agents may spend more time transmitting data than making decisions or acting within their environments. 

To address these challenges, several technical strategies can be employed. Gradient compression (Pan et al., 2025; Xu et al., 2023) and sparsification techniques (Zhang et al., 2024a), such as top-k sparsification (Shi et al., 2019) or stochastic quantization (Alistarh et al., 2017), help reduce the size of updates exchanged between agents and central servers. These methods maintain model convergence while significantly lowering communication demands. Asynchronous communication (Lan et al., 2024; Shen et al., 2024) is another promising solution, allowing agents to update their models independently rather than waiting for synchronized communication (Pan et al., 2022b) with all other agents, thus minimizing idle time and improving overall efficiency. For larger systems, hierarchical communication architectures (Majidi et al., 2021; Sagar et al., 2025) can be adopted, where nodes are clustered based on proximity or data similarity. In such setups, local aggregation is performed within each cluster before sending updates to a global server, thereby reducing the overall communication cost while maintaining the integrity of the learning process. 

## _7.3. Scalability_ 

As the number of participating agents increases in FMARL systems, scalability becomes a critical concern. With more agents, there is a rise in communication overhead, model complexity, and coordination challenges, particularly in scenarios where collaboration between agents is essential to achieve global objectives. Ensuring that the system remains efficient and responsive under these conditions is key to the successful deployment of FMARL in large-scale applications. 

To address scalability issues, several technical approaches can be explored. One promising direction is the introduction of hierarchical multi-agent systems (Kumar et al., 2017; Zhang et al., 2021c), where agents are grouped into subgroups operating at different levels of control or abstraction. This hierarchical structure simplifies coordination, making it easier to manage a large number of agents and improving system scalability. Another approach is decentralized FL (Chen et al., 2024; Peake et al., 2020), where agents collaborate locally to update their models, reducing the need for constant communication with a central server. This method helps alleviate communication and computation bottlenecks. Additionally, employing parallel and distributed computation frameworks, such as MapReduce (Hashem et al., 2016) or graphbased models (Yu et al., 2024), can effectively handle the increased computational load, distributing the learning process across agents more efficiently. These strategies collectively enhance the scalability of FMARL, allowing it to accommodate larger and more complex systems. 

## _7.4. Multi-agent collaboration in dynamic environments_ 

In FMARL systems, agents frequently operate in dynamic, nonstationary environments, such as real-time traffic management or energy distribution, where conditions can change unpredictably. In these settings, agents must not only adapt their strategies in response to shift- 

ing environments but also collaborate effectively with other agents to achieve collective goals and optimize performance. 

One technical direction to address this challenge is the application of transfer learning (Liang et al., 2022; Saha & Ahmad, 2021), which enables agents to generalize and adapt more quickly by leveraging knowledge from previously encountered tasks. This is particularly useful when agents are faced with unfamiliar or rapidly changing environments. Additionally, multi-agent communication protocols (Pitt & Mamdani, 2000; Poslad, 2007) are essential for facilitating effective coordination in these dynamic settings. Protocols based on graph neural networks (GNNs) (Wu et al., 2022a), for example, enable more efficient information sharing between agents, allowing them to synchronize their actions and adjust their strategies in real-time. Finally, the use of adaptive reward mechanisms can further enhance collaboration by dynamically adjusting the incentives for agents based on the current state of the environment. This encourages more responsive and cooperative behaviors, ensuring that agents work together efficiently even as conditions fluctuate. 

## _7.5. Privacy-preserving mechanisms_ 

In FMARL, preserving privacy (Li et al., 2021, 2023c; Truex et al., 2019) is a critical challenge, particularly when handling sensitive data. The main difficulty lies in striking a balance between ensuring privacy and maintaining model performance, especially in dynamic and non-stationary environments. Traditional privacy-preserving techniques, such as DP and HE, while effective, often come with significant computational complexity and resource demands, limiting the system’s scalability and overall efficiency. 

To tackle these challenges, recent advancements focus on refining DP (Dwork, 2008; El Ouadrhiri & Abdelhadi, 2022; Naseri et al., 2020; Triastcyn & Faltings, 2019) to suit real-time, multi-agent settings. In FMARL, noise injection techniques must be carefully adapted to ensure that privacy is upheld without significantly affecting learning performance. HE (Fang & Qian, 2021; Ma et al., 2022; Park & Lim, 2022; Zhang et al., 2020) provides robust privacy protection, but its high computational overhead poses challenges, making it less practical for large-scale, real-time applications. Research is exploring ways to reduce this overhead or develop hybrid approaches that apply encryption selectively to optimize both security and performance. Beyond these methods, secure aggregation techniques like SMPC (Bonawitz et al., 2017; Yao, 1982), offer an alternative by allowing agents to aggregate model updates securely, without revealing sensitive individual data. SMPC avoids the computational burden of full encryption, providing a more scalable and efficient privacy-preserving solution. By advancing these mechanisms, FMARL systems can better manage the trade-offs between privacy, performance, and scalability, especially in the complex and dynamic environments in which they operate. 

## _7.6. Model convergence and computational complexity_ 

Achieving stable convergence and managing computational complexity are two of the most critical challenges in FMARL. Unlike standard federated learning, FMARL systems must contend not only with statistical heterogeneity and communication constraints but also with the non-stationarity and high variance inherent in MARL environments. 

In FMARL, convergence is hindered by several factors. First, agents are exposed to different local environments, resulting in significantly non-IID experiences and policy gradients. Second, delayed synchronization and noisy updates, due to asynchronous training or partial participation, can amplify model drift. Third, the interactive nature of multi-agent systems induces non-stationarity: each agent’s policy update changes the environment for others, creating a moving-target problem for learning. To better understand these limitations, we consider a general FMARL setting with _𝑛_ agents. Each agent _𝑖_ maintains local policy parameters _𝜃𝑖_ ∈ ℝ _[𝑑]_ and performs _𝑇_ steps of local reinforcement learning 

18 

_Expert Systems With Applications 293 (2025) 128729_ 

_Y. Jing et al._ 

updates between communication rounds. Let _𝜃_[(] _[𝑟]_[)] denote the aggregated parameters at round _𝑟_ , and _𝐽_ ( _𝜃_ ) denote the global objective. Under standard assumptions in distributed optimization: 

- **(A1)** _𝐿_ - **smoothness** (Stich, 2018): The global objective _𝐽_ ( _𝜃_ ) satisfies ‖∇ _𝐽_ ( _𝜃_ ) −∇ _𝐽_ ( _𝜃_[′] )‖ ≤ _𝐿_ ‖ _𝜃_ − _𝜃_[′] ‖. 

- **(A2) Bounded local variance** (Karimireddy et al., 2020): Local gradient noise is bounded: 𝔼[‖∇ _𝐽𝑖_ ( _𝜃_ ) − _𝑔𝑖_ ( _𝜃_ )‖[2] ] ≤ _𝜎_[2] . ∇ _𝐽𝑖_ ( _𝜃_ ) is the exact gradient of the local loss function _𝐽𝑖_ ( _𝜃_ ) at agent _𝑖_ . _𝑔𝑖_ ( _𝜃_ ) is the stochastic gradient is computed on agent _𝑖_ , often based on a mini-batch of samples, which introduces noise. _𝜎_[2] is a finite upper bound on the variance of the gradient noise. 

- **(A3) Bounded gradient dissimilarity** (Wang et al., 2020a): 

- ‖∇ _𝐽𝑖_ ( _𝜃_ ) −∇ _𝐽_ ( _𝜃_ )‖ ≤ _𝜁_ for all agent. 

Then, with a proper step size _𝜂_ , the expected gradient norm over _𝑅_ rounds satisfies: 

**==> picture [251 x 25] intentionally omitted <==**

This result suggests a fundamental trade-off: larger _𝑇_ (more local updates) reduces communication frequency, but increases the divergence between agents’ models, especially under high heterogeneity ( _𝜁_ ), which in turn slows convergence (Li et al., 2020). 

With respect to computational complexity in FMARL, we decompose the overall complexity into three parts: 

- **Local computation:** Each agent performs _𝑇_ updates per round. Assuming each update costs ( _𝑑_ ), the total local computation cost is ( _𝑛𝑅𝑇𝑑_ ). 

- **Communication:** Each agent transmits a _𝑑_ -dimensional model vector per round, yielding a total communication cost of ( _𝑛𝑅𝑑_ ). 

- **Aggregation:** Centralized averaging across _𝑛_ agents incurs a negligible cost of ( _𝑛𝑑_ ) per round. 

These theoretical insights reveal several critical design trade-offs in FMARL. Increasing the number of local updates per round ( _𝑇_ ) can significantly reduce communication overhead, which improves overall system efficiency. However, it may also cause greater model drift under non-IID conditions, potentially leading to slower or unstable convergence (Stich, 2018). On the other hand, increasing the number of participating agents ( _𝑁_ ) can enhance gradient diversity and improve learning speed, but it also increases system load and communication complexity. 

Balancing these factors is essential for practical deployment in bandwidth-limited or latency-sensitive environments such as intelligent transportation systems, collaborative robotics, and large-scale IoT applications. One promising direction is the adaptive adjustment of local update frequencies. These methods dynamically control the number of local steps or synchronization intervals based on the variability of received updates or the progress of training. This helps maintain convergence while reducing unnecessary communication cost (Liu et al., 2021; Wang et al., 2019; Wu et al., 2023). Another effective approach is to incorporate momentum into federated averaging algorithms (Yang et al., 2022). Momentum helps smooth out noisy updates, reduces oscillations, and improves convergence stability in the presence of heterogeneous or unstable learning dynamics. Future research may further explore hybrid methods that combine adaptive update scheduling, momentum-based optimization, and other coordination strategies to accelerate convergence while reducing computational and communication complexity. 

## **8. Conclusion** 

FMARL is emerging as a powerful paradigm for tackling the unique challenges associated with decentralized, multi-agent systems in privacy-sensitive and resource-constrained environments. By combining the strengths of FL and MARL, FMARL enables agents to collaborate efficiently without compromising data privacy, addressing critical issues such as secure communication, coordination, and learning 

in distributed settings. Through this comprehensive survey, we have elucidated the key methodologies, applications, and innovations that characterize FMARL. Furthermore, we have identified several pressing challenges, including the need for more efficient communication protocols, enhanced privacy-preserving mechanisms, and more scalable algorithms for high-dimensional environments. Looking ahead, FMARL’s development will hinge on addressing these challenges while exploring new applications in domains like smart cities, autonomous systems, and intelligent edge computing. This survey lays the foundation for future research, highlighting FMARL’s potential to advance multi-agent learning systems and foster innovations that can revolutionize collaborative and adaptive learning in complex, distributed environments. 

## **CRediT authorship contribution statement** 

**Yao Jing:** Conceptualization, Methodology, Literature review, Writing - original draft, Writing - review & editing; **Bin Guo:** Writing - review & editing, Supervision; **Nuo Li:** Literature review, Writing - review & editing; **Ruonan Xu:** Literature review, Writing - review & editing; **Zhiwen Yu:** Writing - review & editing, Supervision. 

## **Data availability** 

Data will be made available on request. 

## **Declaration of competing interest** 

The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **Acknowledgment** 

This work was partially supported by the National Science Fund for Distinguished Young Scholars (no. 62025205), and the National Natural Science Foundation of China (nos. 62032020, 61960206008). 

## **References** 

- Abdel-Aziz, M. K., Samarakoon, S., Perfecto, C., & Bennis, M. (2020). Cooperative perception in vehicular networks using multi-agent reinforcement learning. In _2020 54th asilomar conference on signals, systems, and computers_ (pp. 408–412). IEEE. 

- Al-Hader, M., & Rodzi, A. (2009). The smart city infrastructure development & monitoring. _Theoretical and Empirical Researches in Urban Management_ , _4_ (11), 87–94. 

- Alistarh, D., Grubic, D., Li, J., Tomioka, R., & Vojnovic, M. (2017). Qsgd: Communicationefficient sgd via gradient quantization and encoding. _Advances in Neural Information Processing Systems_ , _30_ . 

- Antonio, G.-P., & Maria-Dolores, C. (2022). Multi-agent deep reinforcement learning to manage connected autonomous vehicles at tomorrow’s intersections. _IEEE Transactions on Vehicular Technology_ , _71_ (7), 7033–7043. 

- Arafeh, M., Hammoud, A., Otrok, H., Mourad, A., Talhi, C., & Dziong, Z. (2022). Independent and identically distributed (IID) data assessment in federated learning. In _Globecom 2022-2022 IEEE global communications conference_ (pp. 293–298). IEEE. 

- Banerjee, B., Elliott, R. C., Krzymieñ, W. A., & Medra, M. (2023). Access point clustering in cell-free massive MIMO using conventional and federated multi-agent reinforcement learning. _IEEE Transactions on Machine Learning in Communications and Networking_ , _1_ , 107–123. 

- Bao, J., Wu, C., Lin, Y., Zhong, L., Chen, X., & Yin, R. (2023). A scalable approach to optimize traffic signal control with federated reinforcement learning. _Scientific Reports_ , _13_ (1), 19184. 

- Beaussart, M., Grimberg, F., Hartley, M.-A., & Jaggi, M. (2021). Waffle: Weighted averaging for personalized federated learning. arXiv preprint arXiv:2110.06978 

- Bernstein, D. S., Givan, R., Immerman, N., & Zilberstein, S. (2002). The complexity of decentralized control of markov decision processes. _Mathematics of Operations Research_ , _27_ (4), 819–840. 

Bhalla, S., Ganapathi Subramanian, S., & Crowley, M. (2020). Deep multi agent reinforcement learning for autonomous driving. In _Canadian conference on artificial intelligence_ (pp. 67–78). Springer. 

Bonawitz, K., Ivanov, V., Kreuter, B., Marcedone, A., McMahan, H. B., Patel, S., Ramage, D., Segal, A., & Seth, K. (2017). Practical secure aggregation for privacy-preserving machine learning. In _proceedings of the 2017 ACM SIGSAC conference on computer and communications security_ (pp. 1175–1191). 

Brunke, L., Greeff, M., Hall, A. W., Yuan, Z., Zhou, S., Panerati, J., & Schoellig, A. P. (2022). Safe learning in robotics: From learning-based control to safe reinforcement learning. _Annual Review of Control, Robotics, and Autonomous Systems_ , _5_ (1), 411–444. 

19 

_Expert Systems With Applications 293 (2025) 128729_ 

## _Y. Jing et al._ 

Byrd, D., & Polychroniadou, A. (2020). Differentially private secure multi-party computation for federated learning in financial applications. In _Proceedings of the first ACM international conference on AI in finance_ (pp. 1–9). 

Cao, S., Zhang, H., Wen, T., Zhao, H., Zheng, Q., Zhang, W., & Zheng, D. (2024). FedQMIX: Communication-efficient federated learning via multi-agent reinforcement learning. _High-Confidence Computing_ , _4_ (2), 100179. 

Chang, H.-H., Song, Y., Doan, T. T., & Liu, L. (2023). Federated multi-agent deep reinforcement learning (Fed-MADRL) for dynamic spectrum access. _IEEE Transactions on Wireless Communications_ , _22_ (8), 5337–5348. 

Chen, D., Zhang, K., Wang, Y., Yin, X., Li, Z., & Filev, D. (2024). Communication-efficient decentralized multi-agent reinforcement learning for cooperative adaptive cruise control. _IEEE Transactions on Intelligent Vehicles_ . _9_ (10), 6436–6449. Chen, J., Esrafilian, O., Bayerlein, H., Gesbert, D., & Caccamo, M. (2023). Model-aided federated reinforcement learning for multi-UAV trajectory planning in iot networks. In _2023 IEEE globecom workshops (GC wkshps)_ (pp. 818–823). IEEE. 

Chen, X., & Liu, G. (2022). Federated deep reinforcement learning-based task offloading and resource allocation for smart cities in a mobile edge network. _Sensors_ , _22_ (13), 4738. 

Chu, Y., Wei, Z., Fang, X., Chen, S., & Zhou, Y. (2022). A multiagent federated reinforcement learning approach for plug-in electric vehicle fleet charging coordination in a residential community. _IEEE Access_ , _10_ , 98535–98548. https://doi.org/10.1109/ACCESS. 2022.3206020 

- Chung, S.-J., Paranjape, A. A., Dames, P., Shen, S., & Kumar, V. (2018). A survey on aerial swarm robotics. _IEEE Transactions on Robotics_ , _34_ (4), 837–855. 

Clemente, A. V., Castejón, H. N., & Chandra, A. (2017). Efficient parallel methods for deep reinforcement learning. arXiv preprint arXiv:1705.04862 

Cui, J., Li, Y., Zhang, Q., He, Z., & Zhao, S. (2024). A federated learning framework using fedprox algorithm for privacy-preserving palmprint recognition. In _Chinese conference on biometric recognition_ (pp. 187–196). Springer. 

Dai, Z., Fan, F. X., Tan, C., Hoang, T. N., Low, B. K. H., & Jaillet, P. (2024). Federated sequential decision making: Bayesian optimization, reinforcement learning, and beyond. In _Federated learning_ (pp. 257–279). Elsevier. 

De Oliveira, H., Kaneko, M., & Boukhatem, L. (2024). Federated multiagent deep reinforcement learning for intelligent iot wireless communications: Overview and challenges. _IEEE Vehicular Technology Magazine_ . _19_ (4), 73–82. 

Deng, Y., Kamani, M. M., & Mahdavi, M. (2020). Adaptive personalized federated learning. arXiv preprint arXiv:2003.13461 

Devlin, J. (2018). Bert: Pre-training of deep bidirectional transformers for language understanding. arXiv preprint arXiv:1810.04805 

Dong, N., Kampffmeyer, M., Voiculescu, I., & Xing, E. (2022). Federated partially supervised learning with limited decentralized medical images. _IEEE Transactions on Medical Imaging_ , _42_ (7), 1944–1954. 

- Dwork, C. (2008). Differential privacy: A survey of results. In _International conference on theory and applications of models of computation_ (pp. 1–19). Springer. 

- Dwork, C., McSherry, F., Nissim, K., & Smith, A. (2006). Calibrating noise to sensitivity in private data analysis. In _Theory of cryptography: Third theory of cryptography conference, TCC 2006, new york, NY, USA, march 4–7, 2006. proceedings 3_ (pp. 265–284). Springer. 

- Dwork, C., Roth, A. et al. (2014). The algorithmic foundations of differential privacy. _Foundations and Trends® in Theoretical Computer Science_ , _9_ (3–4), 211–407. 

- Edemekong, P. F., Annamaraju, P., & Haydel, M. J. (2018). Health insurance portability and accountability act. 

- El Hamdani, S., & Benamar, N. (2018). Autonomous traffic management: Open issues and new directions. In _2018 international conference on selected topics in mobile and wireless networking (moWNet)_ (pp. 1–5). IEEE. 

- El Ouadrhiri, A., & Abdelhadi, A. (2022). Differential privacy for deep and federated learning: a survey. _IEEE Access_ , _10_ , 22359–22380. 

- Errounda, F. Z., & Liu, Y. (2023). Adaptive differential privacy in vertical federated learning for mobility forecasting. _Future Generation Computer Systems_ , _149_ , 531–546. 

- Espeholt, L., Soyer, H., Munos, R., Simonyan, K., Mnih, V., Ward, T., Doron, Y., Firoiu, V., Harley, T., Dunning, I. et al. (2018). Impala: Scalable distributed deep-rl with importance weighted actor-learner architectures. In _International conference on machine learning_ (pp. 1407–1416). PMLR. 

- Fallah, A., Mokhtari, A., & Ozdaglar, A. (2020). Personalized federated learning: A metalearning approach. arXiv preprint arXiv:2002.07948 

- Fang, H., & Qian, Q. (2021). Privacy preserving machine learning with homomorphic encryption and federated learning. _Future Internet_ , _13_ (4), 94. 

- Fu, Y., & Di, X. (2023). Federated reinforcement learning for adaptive traffic signal control: A case study in new york city. In _2023 IEEE 26th international conference on intelligent transportation systems (ITSC)_ (pp. 5738–5743). IEEE. 

- Gaba, S., Budhiraja, I., Kumar, V., Garg, S., & Hassan, M. M. (2024). An innovative multiagent approach for robust cyber–physical systems using vertical federated learning. _Ad Hoc Networks_ , _163_ , 103578. 

- Gao, C., Zheng, Y., Li, N., Li, Y., Qin, Y., Piao, J., Quan, Y., Chang, J., Jin, D., He, X. et al. (2023a). A survey of graph neural networks for recommender systems: Challenges, methods, and directions. _ACM Transactions on Recommender Systems_ , _1_ (1), 1–51. 

- Gao, J., Wang, W., Nikseresht, F., Govinda Rajan, V., & Campbell, B. (2023b). Pfdrl: Personalized federated deep reinforcement learning for residential energy management. In _Proceedings of the 52nd international conference on parallel processing_ (pp. 402–411). 

- Ge, J., Xie, X., Zheng, H., Chen, J., Li, H., Pang, L., & Zhao, W. (2022). A secured deep reinforcement learning model based on vertical federated learning. In _China national conference on big data and social computing_ (pp. 129–142). Springer. 

- Geyer, R. C., Klein, T., & Nabi, M. (2017). Differentially private federated learning: A client level perspective. arXiv preprint arXiv:1712.07557 

Giuseppi, A., Menegatti, D., & Pietrabissa, A. (2025). Enhancing federated reinforcement learning: A consensus-based approach for both homogeneous and heterogeneous agents. _Machine Intelligence Research_ . 

- Grandvalet, Y., Canu, S., & Boucheron, S. (1997). Noise injection: Theoretical prospects. _Neural Computation_ , _9_ (5), 1093–1108. 

- Gronauer, S., & Diepold, K. (2022). Multi-agent deep reinforcement learning: A survey. _Artificial Intelligence Review_ , _55_ (2), 895–943. 

- Guo, Q., Tang, F., & Kato, N. (2022). Federated reinforcement learning-based resource allocation in d2d-enabled 6g. _IEEE Network_ , _37_ (5), 89–95. 

- Haarnoja, T., Zhou, A., Hartikainen, K., Tucker, G., Ha, S., Tan, J., Kumar, V., Zhu, H., Gupta, A., Abbeel, P. et al. (2018). Soft actor-critic algorithms and applications. arXiv preprint arXiv:1812.05905 

- Han, Y., Li, D., Qi, H., Ren, J., & Wang, X. (2019). Federated learning-based computation offloading optimization in edge computing-supported internet of things. In _Proceedings of the ACM turing celebration conference-china_ (pp. 1–5). 

- Hashem, I. A. T., Anuar, N. B., Gani, A., Yaqoob, I., Xia, F., & Khan, S. U. (2016). Mapreduce: Review and open challenges. _Scientometrics_ , _109_ , 389–422. 

- Hastie, T., Tibshirani, R., Friedman, J., Hastie, T., Tibshirani, R., & Friedman, J. (2009). Overview of supervised learning. _The Elements of Statistical learning: Data Mining, Inference, and Prediction_ , (pp. 9–41). 

- He, K., Zhang, X., Ren, S., & Sun, J. (2016). Deep residual learning for image recognition. In _Proceedings of the IEEE conference on computer vision and pattern recognition_ (pp. 770–778). 

- He, Y., Kang, Y., Zhao, X., Luo, J., Fan, L., Han, Y., & Yang, Q. (2024). A hybrid selfsupervised learning framework for vertical federated learning. _IEEE Transactions on Big Data_ . 1–13. 

- Ho, T. M., Nguyen, K.-K., & Cheriet, M. (2022). Federated deep reinforcement learning for task scheduling in heterogeneous autonomous robotic system. _IEEE Transactions on Automation Science and Engineering_ , _21_ (1), 528–540. 

- Hong, M., Kang, S.-K., & Lee, J.-H. (2022). Weighted averaging federated learning based on example forgetting events in label imbalanced non-iid. _Applied Sciences_ , _12_ (12), 5806. 

- Horgan, D., Quan, J., Budden, D., Barth-Maron, G., Hessel, M., Van Hasselt, H., & Silver, D. (2018). Distributed prioritized experience replay. arXiv preprint arXiv:1803.00933 

- Hospedales, T., Antoniou, A., Micaelli, P., & Storkey, A. (2021). Meta-learning in neural networks: A survey. _IEEE Transactions on Pattern Analysis and Machine Intelligence_ , _44_ (9), 5149–5169. 

- Hu, F., Deng, Y., & Aghvami, A. H. (2021). Scalable multi-agent reinforcement learning algorithm for wireless networks. arXiv preprint arXiv:2108.00506 

- Jarwan, A., & Ibnkahla, M. (2022). Edge-based federated deep reinforcement learning for iot traffic management. _IEEE Internet of Things Journal_ , _10_ (5), 3799–3813. 

- Ji, Z., Qin, Z., & Tao, X. (2023). Meta federated reinforcement learning for distributed resource allocation. _IEEE Transactions on Wireless Communications_ , _23_ (7), 7865– 7876. 

- Jia, Y., Zhang, X., Beheshti, A., & Dou, W. (2024). FedLPS: Heterogeneous federated learning for multiple tasks with local parameter sharing. In _Proceedings of the AAAI conference on artificial intelligence_ (pp. 12848–12856). (vol. _38_ ). 

- Jiang, S., Huang, Y., Jafari, M., & Jalayer, M. (2021). A distributed multi-agent reinforcement learning with graph decomposition approach for large-scale adaptive traffic signal control. _IEEE Transactions on Intelligent Transportation Systems_ , _23_ (9), 14689–14701. 

- Jin, H., Peng, Y., Yang, W., Wang, S., & Zhang, Z. (2022). Federated reinforcement learning with environment heterogeneity. In _International conference on artificial intelligence and statistics_ (pp. 18–37). PMLR. 

- Jin, J., Gubbi, J., Marusic, S., & Palaniswami, M. (2014). An information framework for creating a smart city through internet of things. _IEEE Internet of Things Journal_ , _1_ (2), 112–121. 

- Jordan, M. I., & Mitchell, T. M. (2015). Machine learning: Trends, perspectives, and prospects. _Science (New York, N.Y.)_ , _349_ (6245), 255–260. 

- Kaelbling, L. P., Littman, M. L., & Moore, A. W. (1996). Reinforcement learning: A survey. _Journal of Artificial Intelligence Research_ , _4_ , 237–285. 

- Kanagavelu, R., Li, Z., Samsudin, J., Yang, Y., Yang, F., Goh, R. S. M., Cheah, M., Wiwatphonthana, P., Akkarajitsakul, K., & Wang, S. (2020). Two-phase multi-party computation enabled privacy-preserving federated learning. In _2020 20th IEEE/ACM international symposium on cluster, cloud and internet computing (CCGRID)_ (pp. 410–419). IEEE. 

- Karimireddy, S. P., Kale, S., Mohri, M., Reddi, S., Stich, S., & Suresh, A. T. (2020). Scaffold: Stochastic controlled averaging for federated learning. In _International conference on machine learning_ (pp. 5132–5143). PMLR. 

- Kaur, H., Rani, V., Kumar, M., Sachdeva, M., Mittal, A., & Kumar, K. (2024). Federated learning: A comprehensive review of recent advances and applications. _Multimedia Tools and Applications_ , _83_ (18), 54165–54188. 

- Khodadadian, S., Sharma, P., Joshi, G., & Maguluri, S. T. (2022). Federated reinforcement learning: Linear speedup under markovian sampling. In _International conference on machine learning_ (pp. 10997–11057). PMLR. 

- Kiran, B. R., Sobh, I., Talpaert, V., Mannion, P., Al Sallab, A. A., Yogamani, S., & Pérez, P. (2021). Deep reinforcement learning for autonomous driving: A survey. _IEEE Transactions on Intelligent Transportation Systems_ , _23_ (6), 4909–4926. 

- Kober, J., Bagnell, J. A., & Peters, J. (2013). Reinforcement learning in robotics: A survey. _The International Journal of Robotics Research_ , _32_ (11), 1238–1274. 

- Konda, V., & Tsitsiklis, J. (1999). Actor-critic algorithms. _Advances in Neural Information Processing Systems_ , _12_ . 

- Koneˇcn`y, J., McMahan, H. B., Ramage, D., & Richtárik, P. (2016). Federated optimization: Distributed machine learning for on-device intelligence. arXiv preprint arXiv:1610.02527 

- Krizhevsky, A., Sutskever, I., & Hinton, G. E. (2012). Imagenet classification with deep convolutional neural networks. _Advances in Neural Information Processing Systems_ , _25_ . 

20 

_Expert Systems With Applications 293 (2025) 128729_ 

## _Y. Jing et al._ 

Krouka, M., Elgabli, A., Issaid, C. B., & Bennis, M. (2021). Communication-efficient and federated multi-agent reinforcement learning. _IEEE Transactions on Cognitive Communications and Networking_ , _8_ (1), 311–320. 

- Kumar, S., Shah, P., Hakkani-Tur, D., & Heck, L. (2017). Federated control with hierarchical multi-agent deep reinforcement learning. arXiv preprint arXiv:1712.08266 

- Lan, G., Han, D.-J., Hashemi, A., Aggarwal, V., & Brinton, C. G. (2024). Asynchronous federated reinforcement learning with policy gradient updates: Algorithm design and convergence analysis. arXiv preprint arXiv:2404.08003 

- Lee, S., & Choi, D.-H. (2020). Federated reinforcement learning for energy management of multiple smart homes with distributed energy resources. _IEEE Transactions on Industrial Informatics_ , _18_ (1), 488–497. 

- Lee, S., Xie, L., & Choi, D.-H. (2021). Privacy-preserving energy management of a shared energy storage system for smart buildings: A federated deep reinforcement learning approach. _Sensors_ , _21_ (14), 4898. 

- Lei, Y., Ye, D., Shen, S., Sui, Y., Zhu, T., & Zhou, W. (2023). New challenges in reinforcement learning: A survey of security and privacy. _Artificial Intelligence Review_ , _56_ (7), 7195–7236. 

- Li, F., Shen, B., Guo, J., Lam, K.-Y., Wei, G., & Wang, L. (2022a). Dynamic spectrum access for internet-of-things based on federated deep reinforcement learning. _IEEE Transactions on Vehicular Technology_ , _71_ (7), 7952–7956. 

- Li, M., Pan, X., Liu, C., & Li, Z. (2025). Federated deep reinforcement learning-based urban traffic signal optimal control. _Scientific Reports_ , _15_ (1), 11724. 

- Li, N., Song, X., Li, K., Jiang, R., & Li, J. (2023a). Multi-agent federated DRL enabled resource allocation for air-ground integrated iov network. _IEEE Internet Computing_ . _27_ , 15–23. 

- Li, Q., Wen, Z., Wu, Z., Hu, S., Wang, N., Li, Y., Liu, X., & He, B. (2021). A survey on federated learning systems: Vision, hype and reality for data privacy and protection. _IEEE Transactions on Knowledge and Data Engineering_ , _35_ (4), 3347–3366. 

- Li, T., Sahu, A. K., Talwalkar, A., & Smith, V. (2020). Federated learning: Challenges, methods, and future directions. _IEEE Signal Processing Magazine_ , _37_ (3), 50–60. 

- Li, X., Huang, K., Yang, W., Wang, S., & Zhang, Z. (2019). On the convergence of fedavg on non-iid data. arXiv preprint arXiv:1907.02189 

- Li, X., Lu, L., Ni, W., Jamalipour, A., Zhang, D., & Du, H. (2022b). Federated multi-agent deep reinforcement learning for resource allocation of vehicle-to-vehicle communications. _IEEE Transactions on Vehicular Technology_ , _71_ (8), 8810–8824. 

- Li, Y., He, S., Li, Y., Shi, Y., & Zeng, Z. (2023b). Federated multiagent deep reinforcement learning approach via physics-informed reward for multimicrogrid energy management. _IEEE Transactions on Neural Networks and Learning Systems_ . _11_ (1), 100–114. 

- Li, Y., Wang, R., Li, Y., Zhang, M., & Long, C. (2023c). Wind power forecasting considering data privacy protection: A federated deep reinforcement learning approach. _Applied Energy_ , _329_ , 120291. 

- Li, Z., Duan, M., Yu, S., & Yang, W. (2024). Dynamicnet: Efficient federated learning for mobile edge computing with dynamic privacy budget and aggregation weights. _IEEE Transactions on Consumer Electronics_ . 

- Li, Z., Lin, T., Shang, X., & Wu, C. (2023d). Revisiting weighted aggregation in federated learning with neural networks. In _International conference on machine learning_ (pp. 19767–19788). PMLR. 

- Liang, X., Liu, Y., Chen, T., Liu, M., & Yang, Q. (2022). Federated transfer reinforcement learning for autonomous driving. In _Federated and transfer learning_ (pp. 357–371). Springer. 

- Lillicrap, T. P. (2015). Continuous control with deep reinforcement learning. arXiv preprint arXiv:1509.02971 

- Lin, Y., Bao, J., Zhang, Y., Li, J., Shu, F., & Hanzo, L. (2023). Privacy-preserving joint edge association and power optimization for the internet of vehicles via federated multi-agent reinforcement learning. _IEEE Transactions on Vehicular Technology_ , _72_ (6), 8256–8261. 

- Lin, Y.-Y., & Rubin, I. (2017). Infrastructure aided networking and traffic management for autonomous transportation. In _2017 information theory and applications workshop (ITA)_ (pp. 1–7). IEEE. 

- Littman, M. L. (1994). Markov games as a framework for multi-agent reinforcement learning. In _Machine learning proceedings 1994_ (pp. 157–163). Elsevier. 

- Liu, B., Wang, L., & Liu, M. (2019). Lifelong federated reinforcement learning: A learning architecture for navigation in cloud robotic systems. _IEEE Robotics and Automation Letters_ , _4_ (4), 4555–4562. 

- Liu, J., Xu, H., Wang, L., Xu, Y., Qian, C., Huang, J., & Huang, H. (2021). Adaptive asynchronous federated learning in resource-constrained edge computing. _IEEE Transactions on Mobile Computing_ , _22_ (2), 674–690. 

- Liu, Y., & Mao, B. (2023). On a novel content edge caching approach based on multiagent federated reinforcement learning in internet of vehicles. In _2023 32nd wireless and optical communications conference (WOCC)_ (pp. 1–5). IEEE. 

- Liu, Y., Wang, Q., Chen, J., Zhang, W., & Sun, C. (2024). Federated multi-agent deep reinforcement learning approach for resource allocation in platoon-based NR-v2x. In _2024 IEEE 99th vehicular technology conference (VTC2024-spring)_ (pp. 1–6). IEEE. 

- Liu, Z., Garg, N., & Ratnarajah, T. (2023). Multi-agent federated reinforcement learning strategy for mobile virtual reality delivery networks. _IEEE Transactions on Network Science and Engineering_ . _11_ (1), 100–114. 

- Lowe, R., Wu, Y. I., Tamar, A., Harb, J., Pieter Abbeel, O., & Mordatch, I. (2017). Multiagent actor-critic for mixed cooperative-competitive environments. _Advances in Neural Information Processing Systems_ , _30_ . 

- Lv, Z., Xiao, L., Du, Y., Niu, G., Xing, C., & Xu, W. (2023). Multi-agent reinforcement learning based UAV swarm communications against jamming. _IEEE Transactions on Wireless Communications_ , _22_ (12), 9063–9075. 

- Ma, J., Naas, S.-A., Sigg, S., & Lyu, X. (2022). Privacy-preserving federated learning based on multi-key homomorphic encryption. _International Journal of Intelligent Systems_ , _37_ (9), 5880–5901. 

- Majidi, F., Khayyambashi, M. R., & Barekatain, B. (2021). Hfdrl: An intelligent dynamic cooperate cashing method based on hierarchical federated deep reinforcement learning in edge-enabled iot. _IEEE Internet of Things Journal_ , _9_ (2), 1402–1413. 

- Matignon, L., Laurent, G. J., & Le Fort-Piat, N. (2012). Independent reinforcement learners in cooperative markov games: A survey regarding coordination problems. _The Knowledge Engineering Review_ , _27_ (1), 1–31. 

- McMahan, B., Moore, E., Ramage, D., Hampson, S., & y Arcas, B. A. (2017). Communication-efficient learning of deep networks from decentralized data. In _Artificial intelligence and statistics_ (pp. 1273–1282). PMLR. 

- Miao, Q., Lin, H., Wang, X., & Hassan, M. M. (2021). Federated deep reinforcement learning based secure data sharing for internet of things. _Computer Networks_ , _197_ , 108327. 

- Mitchell, T. M., & Mitchell, T. M. (1997). Machine learning (vol. 1). McGraw-Hill, New York. 

- Mnih, V., Kavukcuoglu, K., Silver, D., Rusu, A. A., Veness, J., Bellemare, M. G., Graves, A., Riedmiller, M., Fidjeland, A. K., Ostrovski, G. et al. (2015). Human-level control through deep reinforcement learning. _Nature_ , _518_ (7540), 529–533. 

- Mohammadi, M., Al-Fuqaha, A., Guizani, M., & Oh, J.-S. (2017). Semisupervised deep reinforcement learning in support of iot and smart city services. _IEEE Internet of Things Journal_ , _5_ (2), 624–635. 

- Monahan, G. E. (1982). State of the art–a survey of partially observable markov decision processes: Theory, models, and algorithms. _Management Science_ , _28_ (1), 1–16. 

- Moudoud, H., Abou El Houda, Z., & Brik, B. (2024a). Advancing security and trust in WSNs: A federated multi-agent deep reinforcement learning approach. _IEEE Transactions on Consumer Electronics_ . _70_ (4), 6909–6918. 

- Moudoud, H., Abou El Houda, Z., & Brik, B. (2024b). Reputation-aware scheduling for secure internet of drones: A federated multi-agent deep reinforcement learning approach. In _IEEE infocom 2024-IEEE conference on computer communications workshops (infocom wkshps)_ (pp. 1–6). IEEE. 

- Mowla, N. I., Tran, N. H., Doh, I., & Chae, K. (2020). Afrl: Adaptive federated reinforcement learning for intelligent jamming defense in fanet. _Journal of Communications and Networks_ , _22_ (3), 244–258. 

- Mukherjee, S., Hossain, R. R., Liu, Y., Du, W., Adetola, V., Mohiuddin, S. M., Huang, Q., Yin, T., & Singhal, A. (2023). Enhancing cyber resilience of networked microgrids using vertical federated reinforcement learning. In _2023 IEEE power & energy society general meeting (PESGM)_ (pp. 1–5). IEEE. 

- Munjal, K., & Bhatia, R. (2023). A systematic review of homomorphic encryption and its contributions in healthcare industry. _Complex & Intelligent Systems_ , _9_ (4), 3759–3786. 

- Nadiger, C., Kumar, A., & Abdelhak, S. (2019). Federated reinforcement learning for fast personalization. In _2019 IEEE second international conference on artificial intelligence and knowledge engineering (AIKE)_ (pp. 123–127). IEEE. 

- Nair, A., Srinivasan, P., Blackwell, S., Alcicek, C., Fearon, R., De Maria, A., Panneershelvam, V., Suleyman, M., Beattie, C., Petersen, S. et al. (2015). Massively parallel methods for deep reinforcement learning. arXiv preprint arXiv:1507.04296 

- Naseri, M., Hayes, J., & De Cristofaro, E. (2020). Local and central differential privacy for robustness and privacy in federated learning. arXiv preprint arXiv:2009.03561 

- Nguyen, H. T., Luong, N. C., Zhao, J., Yuen, C., & Niyato, D. (2020). Resource allocation in mobility-aware federated learning networks: A deep reinforcement learning approach. In _2020 IEEE 6th world forum on Internet of Things (WF-IoT)_ (pp. 1–6). IEEE. 

- Nguyen, T. G., Phan, T. V., Hoang, D. T., Nguyen, T. N., & So-In, C. (2021). Federated deep reinforcement learning for traffic monitoring in SDN-based iot networks. _IEEE Transactions on Cognitive Communications and Networking_ , _7_ (4), 1048–1065. 

- Nie, Y., Zhao, J., Gao, F., & Yu, F. R. (2021). Semi-distributed resource management in UAV-aided MEC systems: A multi-agent federated reinforcement learning approach. _IEEE Transactions on Vehicular Technology_ , _70_ (12), 13162–13173. 

- Omidshafiei, S., Pazis, J., Amato, C., How, J. P., & Vian, J. (2017). Deep decentralized multi-task multi-agent reinforcement learning under partial observability. In _International conference on machine learning_ (pp. 2681–2690). PMLR. 

- Ossongo, E., Esseghir, M., & Merghem-Boulahia, L. (2024). A multi-agent federated reinforcement learning-based optimization of quality of service in various lora network slices. _Computer Communications_ , _213_ , 320–330. 

- Otoum, S., Guizani, N., & Mouftah, H. (2021). Federated reinforcement learning-supported IDS for IoT-steered healthcare systems. In _ICC 2021-IEEE international conference on communications_ (pp. 1–6). IEEE. 

- Otter, D. W., Medina, J. R., & Kalita, J. K. (2020). A survey of the usages of deep learning for natural language processing. _IEEE transactions on Neural Networks and Learning Systems_ , _32_ (2), 604–624. 

- Pan, C., Wang, Z., Liao, H., Zhou, Z., Wang, X., Tariq, M., & Al-Otaibi, S. (2022a). Asynchronous federated deep reinforcement learning-based URLLC-aware computation offloading in space-assisted vehicular networks. _IEEE Transactions on Intelligent Transportation Systems_ , _24_ (7), 7377–7389. 

- Pan, X., Wang, W., Zhang, X., Li, B., Yi, J., & Song, D. (2019). How you act tells a lot: Privacy-leaking attack on deep reinforcement learning. In _Aamas_ (pp. 368–376). (vol. _19_ ). 

- Pan, Z., Geng, H., Wei, L., & Zhao, W. (2022b). Adaptive client model update with reinforcement learning in synchronous federated learning. In _2022 32nd international telecommunication networks and applications conference (ITNAC)_ (pp. 1–3). IEEE. 

- Pan, Z., Li, Y., Guan, Z., Liang, M., Li, A., Wang, J., & Kou, F. (2025). Rfcsc: Communication efficient reinforcement federated learning with dynamic client selection and adaptive gradient compression. _Neurocomputing_ , _612_ , 128672. 

- Park, J., & Lim, H. (2022). Privacy-preserving federated learning using homomorphic encryption. _Applied Sciences_ , _12_ (2), 734. 

- Peake, A., McCalmon, J., Raiford, B., Liu, T., & Alqahtani, S. (2020). Multi-agent reinforcement learning for cooperative adaptive cruise control. In _2020 IEEE 32nd international conference on tools with artificial intelligence (ICTAI)_ (pp. 15–22). IEEE. 

21 

_Expert Systems With Applications 293 (2025) 128729_ 

## _Y. Jing et al._ 

Pfitzner, B., Steckhan, N., & Arnrich, B. (2021). Federated learning in a medical context: A systematic literature review. _ACM Transactions on Internet Technology (TOIT)_ , _21_ (2), 1–31. 

Pinto Neto, E. C., Sadeghi, S., Zhang, X., & Dadkhah, S. (2023). Federated reinforcement learning in IoT: applications, opportunities and open challenges. _Applied Sciences_ , _13_ (11), 6497. 

Pitt, J., & Mamdani, A. (2000). Communication protocols in multi-agent systems: A development method and reference architecture. In _Issues in agent communication_ (pp. 160–177). Springer. 

Poslad, S. (2007). Specifying protocols for multi-agent systems interaction. _ACM Transactions on Autonomous and Adaptive Systems (TAAS)_ , _2_ (4), 15–es. 

Puterman, M. L. (2014). Markov decision processes: Discrete stochastic dynamic programming. John Wiley & Sons. 

Qi, J., Zhou, Q., Lei, L., & Zheng, K. (2021). Federated reinforcement learning: Techniques, applications, and open challenges. arXiv preprint arXiv:2108.11887 

Qiao, D., Guo, S., Liu, D., Long, S., Zhou, P., & Li, Z. (2022). Adaptive federated deep reinforcement learning for proactive content caching in edge computing. _IEEE Transactions on Parallel and Distributed Systems_ , _33_ (12), 4767–4782. 

Qiao, J., Zhang, Z., Yue, S., Yuan, Y., Cai, Z., Zhang, X., Ren, J., & Yu, D. (2024). Brdefedrl: Byzantine-robust decentralized federated reinforcement learning with fast convergence and communication efficiency. In _IEEE infocom 2024-IEEE conference on computer communications_ (pp. 141–150). IEEE. 

Qiu, D., Xue, J., Zhang, T., Wang, J., & Sun, M. (2023). Federated reinforcement learning for smart building joint peer-to-peer energy and carbon allowance trading. _Applied Energy_ , _333_ , 120526. 

Rashid, T., Samvelyan, M., De Witt, C. S., Farquhar, G., Foerster, J., & Whiteson, S. (2020). Monotonic value function factorisation for deep multi-agent reinforcement learning. _Journal of Machine Learning Research_ , _21_ (178), 1–51. 

Ren, J., Wang, H., Hou, T., Zheng, S., & Tang, C. (2019). Federated learning-based computation offloading optimization in edge computing-supported internet of things. _IEEE Access_ , _7_ , 69194–69201. https://doi.org/10.1109/ACCESS.2019.2919736 

- Sagar, A. S., Haider, A., & Kim, H. S. (2025). A hierarchical adaptive federated reinforcement learning for efficient resource allocation and task scheduling in hierarchical iot network. _Computer Communications_ , _229_ , 107969. 

- Saha, S., & Ahmad, T. (2021). Federated transfer learning: Concept and applications. _Intelligenza Artificiale_ , _15_ (1), 35–44. 

- Sahoo, P., Tripathi, A., Saha, S., & Mondal, S. (2024a). FedMRL: Data heterogeneity aware federated multi-agent deep reinforcement learning for medical imaging. arXiv preprint arXiv:2407.05800 

- Sahoo, P., Tripathi, A., Saha, S., Mondal, S., Singh, J. P., & Sharma, B. (2024b). Adafedprox: A heterogeneity-aware federated deep reinforcement learning for medical image classification. _IEEE Transactions on Consumer Electronics_ . _71_ (1), 1432–1441. 

- Salazar, E. J., Rosero, V., Gabrielski, J., & Samper, M. E. (2024). Demand response model: A cooperative-competitive multi-agent reinforcement learning approach. _Engineering Applications of Artificial Intelligence_ , _133_ , 108273. 

- Sallab, A. E. L., Abdou, M., Perot, E., & Yogamani, S. (2017). Deep reinforcement learning framework for autonomous driving. arXiv preprint arXiv:1704.02532 

- Schulman, J., Wolski, F., Dhariwal, P., Radford, A., & Klimov, O. (2017). Proximal policy optimization algorithms. arXiv preprint arXiv:1707.06347 

- Seid, A. M., Erbad, A., Abishu, H. N., Albaseer, A., Abdallah, M., & Guizani, M. (2023). Multiagent federated reinforcement learning for resource allocation in UAV-enabled internet of medical things networks. _IEEE Internet of Things Journal_ , _10_ (22), 19695–19711. 

- Shabir, B., Rahman, A. U., Malik, A. W., Buyya, R., & Khan, M. A. (2023). A federated multi-agent deep reinforcement learning for vehicular fog computing. _The Journal of Supercomputing_ , _79_ (6), 6141–6167. 

- Shalev-Shwartz, S., Shammah, S., & Shashua, A. (2016). Safe, multi-agent, reinforcement learning for autonomous driving. arXiv preprint arXiv:1610.03295 

- Sharma, H., Kumar, N., & Tekchandani, R. (2022). Mitigating jamming attack in 5g heterogeneous networks: A federated deep reinforcement learning approach. _IEEE Transactions on Vehicular Technology_ , _72_ (2), 2439–2452. 

- Shen, S., Shen, G., Dai, Z., Zhang, K., Kong, X., & Li, J. (2024). Asynchronous federated deep reinforcement learning-based dependency task offloading for UAV-assisted vehicular networks. _IEEE Internet of Things Journal_ . _11_ (19), 31561–31574. 

- Shi, S., Chu, X., Cheung, K. C., & See, S. (2019). Understanding top-k sparsification in distributed deep learning. arXiv preprint arXiv:1911.08772 

- Silver, D., Huang, A., Maddison, C. J., Guez, A., Sifre, L., Van Den Driessche, G., Schrittwieser, J., Antonoglou, I., Panneershelvam, V., Lanctot, M. et al. (2016). Mastering the game of go with deep neural networks and tree search. _Nature_ , _529_ (7587), 484–489. 

- Simonyan, K., & Zisserman, A. (2014). Very deep convolutional networks for large-scale image recognition. arXiv preprint arXiv:1409.1556 

- Sotthiwat, E., Zhen, L., Li, Z., & Zhang, C. (2021). Partially encrypted multi-party computation for federated learning. In _2021 IEEE/ACM 21st international symposium on cluster, cloud and internet computing (CCGrid)_ (pp. 828–835). IEEE. 

- Spaan, M. T. J. (2012). Partially observable markov decision processes. In _Reinforcement learning: State-of-the-art_ (pp. 387–414). Springer. 

- Steck, H., Baltrunas, L., Elahi, E., Liang, D., Raimond, Y., & Basilico, J. (2021). Deep learning for recommender systems: A netflix case study. _AI Magazine_ , _42_ (3), 7–18. 

- Stich, S. U. (2018). Local SGD converges fast and communicates little. arXiv preprint arXiv:1805.09767 

- Sun, C., Li, X., Wen, J., Wang, X., Han, Z., & Leung, V. C. M. (2023). Federated deep reinforcement learning for recommendation-enabled edge caching in mobile edgecloud computing networks. _IEEE Journal on Selected Areas in Communications_ , _41_ (3), 690–705. 

- Sun, W., Lei, S., Wang, L., Liu, Z., & Zhang, Y. (2020). Adaptive federated learning and digital twin for industrial internet of things. _IEEE Transactions on Industrial Informatics_ , _17_ (8), 5605–5614. 

Sunehag, P., Lever, G., Gruslys, A., Czarnecki, W. M., Zambaldi, V., Jaderberg, M., Lanctot, M., Sonnerat, N., Leibo, J. Z., Tuyls, K. et al. (2017). Value-decomposition networks for cooperative multi-agent learning. arXiv preprint arXiv:1706.05296 

- Sutton, R. S. (2018). Reinforcement learning: An introduction. _A Bradford book_ . 

- Tam, P., Song, I., Kang, S., & Kim, S. (2022). Privacy-aware intelligent healthcare services with federated learning architecture and reinforcement learning agent. In _International conference on computer science and its applications and the international conference on ubiquitous information technologies and applications_ (pp. 583–590). Springer. 

- Tan, A. Z., Yu, H., Cui, L., & Yang, Q. (2022). Towards personalized federated learning. _IEEE Transactions on Neural Networks and Learning Systems_ , _34_ (12), 9587–9603. 

- Tan, M. (1993). Multi-agent reinforcement learning: Independent vs. cooperative agents. In _Proceedings of the tenth international conference on machine learning_ (pp. 330–337). 

- Terry, J., Black, B., Grammel, N., Jayakumar, M., Hari, A., Sullivan, R., Santos, L. S., Dieffendahl, C., Horsch, C., Perez-Vicente, R. et al. (2021). Pettingzoo: Gym for multiagent reinforcement learning. _Advances in Neural Information Processing Systems_ , _34_ , 15032–15043. 

- Tianqing, Z., Zhou, W., Ye, D., Cheng, Z., & Li, J. (2021). Resource allocation in iot edge computing via concurrent federated reinforcement learning. _IEEE Internet of Things Journal_ , _9_ (2), 1414–1426. 

- Tiwari, P., Lakhan, A., Jhaveri, R. H., & Grønli, T.-M. (2023). Consumer-centric internet of medical things for cyborg applications based on federated reinforcement learning. _IEEE Transactions on Consumer Electronics_ , _69_ (4), 756– 764. 

- Tong, Z., Wang, J., Mei, J., Li, K., Li, W., & Li, K. (2023). Multi-type task offloading for wireless internet of things by federated deep reinforcement learning. _Future Generation Computer Systems_ , _145_ , 536–549. 

- Triastcyn, A., & Faltings, B. (2019). Federated learning with bayesian differential privacy. In _2019 IEEE international conference on big data (big data)_ (pp. 2587–2596). IEEE. 

- Truex, S., Baracaldo, N., Anwar, A., Steinke, T., Ludwig, H., Zhang, R., & Zhou, Y. (2019). A hybrid approach to privacy-preserving federated learning. In _Proceedings of the 12th ACM workshop on artificial intelligence and security_ (pp. 1–11). 

- Van Hasselt, H., Guez, A., & Silver, D. (2016). Deep reinforcement learning with double q-learning. In _Proceedings of the AAAI conference on artificial intelligence_ . (vol. _30_ ). 

- Vaswani, A. (2017). Attention is all you need. _Advances in Neural Information Processing Systems_ . _30_ . 

- Vinyals, O., Babuschkin, I., Czarnecki, W. M., Mathieu, M., Dudzik, A., Chung, J., Choi, D. H., Powell, R., Ewalds, T., Georgiev, P. et al. (2019). Grandmaster level in starcraft II using multi-agent reinforcement learning. _Nature_ , _575_ (7782), 350–354. 

- Vinyals, O., Ewalds, T., Bartunov, S., Georgiev, P., Vezhnevets, A. S., Yeo, M., Makhzani, A., Küttler, H., Agapiou, J., Schrittwieser, J. et al. (2017). Starcraft ii: A new challenge for reinforcement learning. arXiv preprint arXiv:1708.04782 

- Voigt, P., & Von dem Bussche, A. (2017). The eu general data protection regulation (GDPR). _A Practical Guide_ , 1st Ed., Cham: Springer International Publishing, 10(3152676), 10–5555. 

- Wang, C., Yao, T., Fan, T., Peng, S., Xu, C., & Yu, S. (2023). Modeling on resource allocation for age-sensitive mobile edge computing using federated multi-agent reinforcement learning. _IEEE Internet of Things Journal_ . _11_ (2), 3121–3131. 

- Wang, J., Liu, Q., Liang, H., Joshi, G., & Poor, H. V. (2020a). Tackling the objective inconsistency problem in heterogeneous federated optimization. _Advances in Neural Information Processing Systems_ , _33_ , 7611–7623. 

- Wang, S., Tuor, T., Salonidis, T., Leung, K. K., Makaya, C., He, T., & Chan, K. (2019). Adaptive federated learning in resource constrained edge computing systems. _IEEE Journal on Selected Areas in Communications_ , _37_ (6), 1205–1221. 

- Wang, T., Liang, T., Li, J., Zhang, W., Zhang, Y., & Lin, Y. (2020b). Adaptive traffic signal control using distributed marl and federated learning. In _2020 IEEE 20th international conference on communication technology (ICCT)_ (pp. 1242–1248). IEEE. 

- Wang, W., Wu, Q., Fan, P., Cheng, N., Chen, W., Wang, J., & Letaief, K. B. (2024). Optimizing age of information in vehicular edge computing with federated graph neural network multi-agent reinforcement learning. arXiv preprint arXiv:2407.02342 

- Wang, X., Li, R., Wang, C., Li, X., Taleb, T., & Leung, V. C. M. (2020c). Attention-weighted federated deep reinforcement learning for device-to-device assisted heterogeneous collaborative edge caching. _IEEE Journal on Selected Areas in Communications_ , _39_ (1), 154–169. 

- Wang, X., Wang, C., Li, X., Leung, V. C. M., & Taleb, T. (2020d). Federated deep reinforcement learning for internet of things with decentralized cooperative edge caching. _IEEE Internet of Things Journal_ , _7_ (10), 9441–9455. 

- Wang, X., Wang, S., Liang, X., Zhao, D., Huang, J., Xu, X., Dai, B., & Miao, Q. (2022). Deep reinforcement learning: A survey. _IEEE Transactions on Neural Networks and Learning Systems_ , _35_ (4), 5064–5078. 

- Wibawa, F., Catak, F. O., Kuzlu, M., Sarp, S., & Cali, U. (2022). Homomorphic encryption and federated learning based privacy-preserving CNN training: Covid-19 detection usecase. In _Proceedings of the 2022 European interdisciplinary cybersecurity conference_ (pp. 85–90). 

- Williams, R. J. (1992). Simple statistical gradient-following algorithms for connectionist reinforcement learning. _Machine Learning_ , _8_ , 229–256. 

- Woo, J., Shi, L., Joshi, G., & Chi, Y. (2024). Federated offline reinforcement learning: Collaborative single-policy coverage suffices. arXiv preprint arXiv:2402.05876 

- Wu, C., Wu, F., Lyu, L., Qi, T., Huang, Y., & Xie, X. (2022a). A federated graph neural network framework for privacy-preserving personalization. _Nature Communications_ , _13_ (1), 3091. 

- Wu, D., Ullah, R., Harvey, P., Kilpatrick, P., Spence, I., & Varghese, B. (2022b). Fedadapt: Adaptive offloading for iot devices in federated learning. _IEEE Internet of Things Journal_ , _9_ (21), 20889–20901. https://doi.org/10.1109/JIOT.2022.3176469 

22 

_Expert Systems With Applications 293 (2025) 128729_ 

## _Y. Jing et al._ 

Wu, H., Wang, B., Ma, H., Zhang, X., & Xing, L. (2024). Multi-agent federated deep reinforcement learning based collaborative caching strategy for vehicular edge networks. _IEEE Internet of Things Journal_ . _11_ (14), 25198–25212. 

- Wu, S., Xu, W., Wang, F., Li, G., & Pan, M. (2022c). Distributed federated deep reinforcement learning based trajectory optimization for air-ground cooperative emergency networks. _IEEE Transactions on Vehicular Technology_ , _71_ (8), 9107– 9112. 

- Wu, X., Huang, F., Hu, Z., & Huang, H. (2023). Faster adaptive federated learning. In _Proceedings of the AAAI conference on artificial intelligence_ (pp. 10379–10387). (vol. _37_ ). 

- Xie, H., Xia, M., Wu, P., Wang, S., & Huang, K. (2024). Decentralized federated learning with asynchronous parameter sharing for large-scale iot networks. _IEEE Internet of Things Journal_ . _11_ (21), 34123–34139. 

- Xiong, W., Liu, Q., Li, F., Wang, B., & Zhu, F. (2024). Personalized federated reinforcement learning: balancing personalization and experience sharing via distance constraint. _Expert Systems with Applications_ , _238_ , 122290. 

- Xiong, X., Zheng, K., Lei, L., & Hou, L. (2020). Resource allocation based on deep reinforcement learning in iot edge computing. _IEEE Journal on Selected Areas in Communications_ , _38_ (6), 1133–1146. 

- Xu, M., Peng, J., Gupta, B. B., Kang, J., Xiong, Z., Li, Z., & Abd El-Latif, A. A. (2021). Multiagent federated reinforcement learning for secure incentive mechanism in intelligent cyber–physical systems. _IEEE Internet of Things Journal_ , _9_ (22), 22095– 22108. 

- Xu, Y., Jiang, Z., Xu, H., Wang, Z., Qian, C., & Qiao, C. (2023). Federated learning with client selection and gradient compression in heterogeneous edge systems. _IEEE Transactions on Mobile Computing_ . _23_ (5), 5446–5461. 

- Xue, Z., Zhou, P., Xu, Z., Wang, X., Xie, Y., Ding, X., & Wen, S. (2021). A resourceconstrained and privacy-preserving edge-computing-enabled clinical decision system: A federated reinforcement learning approach. _IEEE Internet of Things Journal_ , _8_ (11), 9122–9138. 

- Yang, Q., Fan, L., & Yu, H. (2020). Federated learning: Privacy and incentive (vol. 12500). Springer Nature. 

- Yang, T., Feng, X., Cai, S., Niu, Y., & Pen, H. (2024). A privacy-preserving federated reinforcement learning method for multiple virtual power plants scheduling. _IEEE Transactions on Circuits and Systems I: Regular Papers_ . _72_ (4), 1939–1950. 

- Yang, Y., & Wang, J. (2020). An overview of multi-agent reinforcement learning from game theoretical perspective. arXiv preprint arXiv:2011.00583 

- Yang, Z., Bao, W., Yuan, D., Tran, N. H., & Zomaya, A. Y. (2022). Federated learning with nesterov accelerated gradient. _IEEE Transactions on Parallel and Distributed Systems_ , _33_ (12), 4863–4873. 

- Yao, A. C. (1982). Protocols for secure computations. In _23rd annual symposium on foundations of computer science (sfcs 1982)_ (pp. 160–164). IEEE. 

- Yao, X., Huang, T., Zhang, R.-X., Li, R., & Sun, L. (2019). Federated learning with unbiased gradient aggregation and controllable meta updating. arXiv preprint arXiv:1910.08234 

- Ye, Y., Zhao, W., Wei, T., Hu, S., & Chen, M. (2021). Fedlight: Federated reinforcement learning for autonomous multi-intersection traffic signal control. In _2021 58th ACM/IEEE design automation conference (DAC)_ (pp. 847–852). IEEE. 

- Ye, Z., Qiu, D., Li, S., Fan, Z., & Strbac, G. (2025). Federated reinforcement learning for decentralized peer-to-peer energy trading. _Energy and AI_ , _20_ , 100500. 

- Yenkanchi, S. (2016). Multi sensor data fusion for autonomous vehicles. Master’s thesis University of Windsor (Canada). 

- Yu, C., Velu, A., Vinitsky, E., Gao, J., Wang, Y., Bayen, A., & Wu, Y. (2022). The surprising effectiveness of ppo in cooperative multi-agent games. _Advances in Neural Information Processing Systems_ , _35_ , 24611–24624. 

- Yu, E., Liu, S., Li, Q., Chen, H., Poor, H. V., & Shamai, S. (2024). Graph-based joint client clustering and resource allocation for wireless distributed learning: A new hierarchical federated learning framework with non-IID data. _IEEE Transactions on Mobile Computing_ . _24_ (5), 3579–3596. 

- Zang, L., Zhang, X., & Guo, B. (2022). Federated deep reinforcement learning for online task offloading and resource allocation in WPC-MEC networks. _IEEE Access_ , _10_ , 9856–9867. https://doi.org/10.1109/ACCESS.2022.3144415 

- Zhang, C., Li, S., Xia, J., Wang, W., Yan, F., & Liu, Y. (2020). {BatchCrypt}: Efficient homomorphic encryption for {Cross-Silo} federated learning. In _2020 USENIX annual technical conference (USENIX ATC 20)_ (pp. 493–506). 

- Zhang, C., Xie, Y., Bai, H., Yu, B., Li, W., & Gao, Y. (2021a). A survey on federated learning. _Knowledge-Based Systems_ , _216_ , 106775. 

- Zhang, H., Feng, S., Liu, C., Ding, Y., Zhu, Y., Zhou, Z., Zhang, W., Yu, Y., Jin, H., & Li, Z. (2019a). Cityflow: A multi-agent reinforcement learning environment for large scale city traffic scenario. In _The world wide web conference_ (pp. 3620–3624). 

- Zhang, J., Li, X., Vijayakumar, P., Liang, W., Chang, V., & Gupta, B. B. (2024a). Graph sparsification-based secure federated learning for consumer-driven internet of things. _IEEE Transactions on Consumer Electronics_ . _70_ (3), 5188–5200. 

- Zhang, K., Yang, Z., & Ba¸sar, T. (2021b). Multi-agent reinforcement learning: A selective overview of theories and algorithms. _Handbook of Reinforcement Learning and Control_ , (pp. 321–384). 

- Zhang, L., Xu, J., Vijayakumar, P., Sharma, P. K., & Ghosh, U. (2022a). Homomorphic encryption-based privacy-preserving federated learning in iot-enabled healthcare system. _IEEE Transactions on Network Science and Engineering_ , _10_ (5), 2864–2880. 

- Zhang, S. Q., Lin, J., & Zhang, Q. (2022b). A multi-agent reinforcement learning approach for efficient client selection in federated learning. In _Proceedings of the AAAI conference on artificial intelligence_ (pp. 9091–9099). (vol. _36_ ). 

- Zhang, S. Q., Zhang, Q., & Lin, J. (2019b). Efficient communication in multi-agent reinforcement learning via variance based control. _Advances in Neural Information Processing Systems_ , _32_ . 

- Zhang, W., Yang, D., Wu, W., Peng, H., Zhang, N., Zhang, H., & Shen, X. (2021c). Optimizing federated learning in distributed industrial iot: A multiagent approach. _IEEE Journal on Selected Areas in Communications_ , _39_ (12), 3688– 3703. 

- Zhang, X., Peng, M., Yan, S., & Sun, Y. (2019c). Deep-reinforcement-learning-based mode selection and resource allocation for cellular v2x communications. _IEEE Internet of Things Journal_ , _7_ (7), 6380–6391. 

- Zhang, Y., Wang, S., Chen, Z., Xu, X., Funiak, S., & Liu, J. (2024b). Towards cost-efficient federated multi-agent RL with learnable aggregation. In _Pacific-Asia conference on knowledge discovery and data mining_ (pp. 171–183). Springer. 

- Zhao, Y., Li, M., Lai, L., Suda, N., Civin, D., & Chandra, V. (2018). Federated learning with non-iid data. arXiv preprint arXiv:1806.00582 

- Zhu, C., Dastani, M., & Wang, S. (2024). A survey of multi-agent deep reinforcement learning with communication. _Autonomous Agents and Multi-Agent Systems_ , _38_ (1), 4. 

- Zhu, H., Xu, J., Liu, S., & Jin, Y. (2021). Federated learning on non-IID data: A survey. _Neurocomputing_ , _465_ , 371–390. 

- Zhuo, H. H., Feng, W., Lin, Y., Xu, Q., & Yang, Q. (2019). Federated deep reinforcement learning. arXiv preprint arXiv:1901.08277 

23 

