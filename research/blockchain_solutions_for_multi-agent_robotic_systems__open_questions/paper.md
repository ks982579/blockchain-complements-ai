## Blockchain Solutions for Multi-Agent Robotic Systems: Related Work and Open Questions 

Ilya Afanasyev[1] , Alexander Kolotov[1] , Ruslan Rezin[1] , Konstantin Danilov[1] , Alexey Kashevnik[2] , Vladimir Jotsov[3] 

> 1Innopolis University, Innopolis, Russia 

> 2ITMO University, St.Petersburg, Russia 

> 3University of Library Studies and Information, Sofia, Bulgaria 

_{_ i.afanasyev, a.kolotov, r.rezin, k.danilov _}_ @innopolis.ru, alexey@iias.spb.su, v.jotsov@unibit.bg 

_**Abstract**_ **—The possibilities of decentralization and immutability make blockchain probably one of the most breakthrough and promising technological innovations in recent years. This paper presents an overview, analysis, and classification of possible blockchain solutions for practical tasks facing multi-agent robotic systems. The paper discusses blockchain-based applications that demonstrate how distributed ledger can be used to extend the existing number of research platforms and libraries for multiagent robotic systems.** 

## I. INTRODUCTION 

The perspectives of decentralization and immutable records make blockchain probably one of the most powerful technological innovation in recent years. To be ubiquitous the blockchain needs to overcome the following issues: scalability, latency and low throughput [1]. 

Permissionless public blockchains have nodes (i.e. computers in the network) that maintain a shared version of the records by storing on each node a complete information on the blockchain. This ensures the blockchain permanence from cyber attacks, which cannot destroy it just by taking out a central server. Most publicly available blockchains also have very reliable means of ensuring data integrity. The blockchain can only be updated by adding a new block without deleting and modifying the existing blocks. Therefore, it is necessary to maintain the integrity of the order of transactions. The blockchain does this through a consensus mechanism. The most widely used consensus mechanism applies proof of work (POW) algorithm, where a miner must solve a complex cryptographical operation with a large number of blocks at high-speed, in a competitive environment. Moreover, the POW algorithm requires that the majority of the participating nodes approve the transaction. In a decentralized network, the hijack of the computing power of the majority of the node looks economically non-viable [1]. However, since the blockchain grows with more users and a higher number of transactions, the transaction validation makes the transaction process slower. 

Lets summarize the key points of blockchain technology addressing them to operation of the multi-agent systems (further - robotics agents): 

- The blockchain is an append only database. As soon as data are included in the database it cannot be changed. 

- The database shapes the state of the blockchain. Examples of the blockchain state: records of all accounts 

balances, snapshot of memory controlled by executable agents. 

- The database is distributed among nodes. Every node keeps full copy of the database. 

- The node is an agent in the blockchain network. 

- The node could generate records to change the state of the blockchain. 

- The node is responsible for transferring all incoming data arrived from another node to all its neighbors. 

- All nodes are connected through peer-to-peer communication channels. 

- Some nodes must play the validator role. 

- The validators verify correctness of records to change blockchain state and approve them (e.g. by combining the records in blocks, linking the blocks together and sending new blocks to the neighbors). 

- Only validated records are applied on all the nodes to build the current state of the blockchain. 

Concluding the information above, lets highlight the strengths of the blockchain technology for multi-agent systems: 

- Data availability is achieved through multiply duplication of data and communication. 

- Consistency of data is achieved through data validation and strict rules of changes appliance. 

- No way to remove or change the data stored in the blockchain. 

- Economic or reputational incentive forces nodes to not violate the validation rules. 

The relevance of the study is explained by the priority in the development of intelligent multi-agent robotic systems capable of performing various tasks with a high degree of autonomy. Information support for the interaction of robot groups has a particular importance for operations in uncertainty conditions, external disturbances and environmental changes. Successful solution of the group interaction problem together with recording the interaction history and performing the verification task by the distributed ledger technology (blockchain), can increase the efficiency of interaction between groups of robots and expand the possibilities of their applications. 

The rest paper is structured as follows. The Section II introduces the present state of scientific and engineering devel- 

opment in the blockchain-based multi-agent systems. Section III describes and classifies the most typical cases, which we identified for blockchain-based robotics applications. Finally, we open the dialog for discussion in the Section IV. 

## II. RELATED PAPER ANALYSIS 

In recent years, research and development in the field of multi-agent robotic systems have become increasingly popular. Considerable attention is paid to the development of distributed planning systems for robot coalitions and algorithms for distributed control of robotic networks [2]–[6]. The section presents a related work analysis in the area of blockchain utilization for multi-agent robotic systems. 

The paper [7] considers the utilization of intelligent agents and multi-agent system concept for Auditable Blockchain Voting System. The aim of such system is integration of e-voting processes with blockchain technology. The main problem of the electronic voting is the low trust level of the respondents to the system since the results can be simply changed. Authors propose to utilize the multi-agent approach to increase the trust level in e-voting system. In scope of the paper [8] author propose a new family of cyber-physical systems. The proposed coupling of the data storage mechanisms, communication and consensus protocols, allows for deployment of self-sustaining cyber-physical environments in which all the mission-critical aspects in both the cyber and physical layer are effectively incentivized, coordinated, and maintained. The following benefits the blockchain technology for cyber-physical systems utlization are highlighted by authors: decentralized, replicated data storage that improves rigidness of the system; ability for anyone to join; participants reach consensus within a trustless environment; ability to maintain trust among initially unknown agents; distributed execution of atomic applications; state proposals are handled exactly as the majority of the network commands; transparency and immutability; agents remain fully autonomous, they are in complete control over their identities and private keys; blockchain data is complete, consistent in the long run and available. 

Authors of the papers [9], [10] describe in details the application of the blockchain technology for Internet of Things (IoT). Authors mentioned that IoT concept transforms human life and unleash enormous economic benefits. But however, inadequate data security and trust level are seriously limiting its wide application. Authors propose a survey where they describe in detail the application of the blockchain technology to Internet of Things environments. Similarly, IoT and Smart and Software-defined Buildings (SSDB) technologies and their cooperation in implementing Smart Spaces are considered in the article [11], which offers a reference architecture for IoT infrastructure and assumes that the blockchain may be the highway for its future development. Developing the idea of a system of smart buildings, the authors in the paper [12] integrate subsystems, such as intelligent networks, services, buildings and household appliances, into models of smart districts and even smart cities, where these subsystems efficiently interact, connect and control remotely by using blockchain 

technology to achieve a better quality of life, sustainability, energy conservation and socio-economic system development. 

Since the development of robotics, IoT, big data processing, automation and distributed register technology (DLT) leads to the fourth industrial revolution, the interaction of the smart factory components within the company and with other Industrial IoT participants providing trust, control over the distribution of resources and products, is investigated in the works [13], [14]. The paper [9], [15] discusses the utilization of the blockchain technology to supply chain. Generally supply chain is the multi-agent system where every supplier has own behaviour model and goals. In scope of the paper [15] authors propose new model of supply chain based on blockchain concept. This new model enables the concept of circular economy and eliminates many of the disadvantages of the current supply chain. 

The recent review of the state of the art in the application of blockchain technology to multi-agent systems is presented in the paper [16], which describes some cases of multi-agent systems, although it did not classify the multiagent systems by purpose and did not present multi-robot systems using the blockchain [17]–[20]. The research [17] focuses on the organization of a blockchain-based protocol for multi-agent coordination and control in the context of Unmanned Aerial Vehicles (UAVs). The article [18] considers a blockchain consensus protocol, which additionally includes a validation procedure of liability execution to prevent payment transactions to questionable service providers. The proposed methodology of a liability execution for agent-based service providers in a decentralized trading market uses a Model Checking method. As the proof-of-concept, the methodology was implemented in an application that simulates the work of a taxi with the following liability validation at the end of a completed scenario. The methodology for building the cyberphysical smart space proposed in [19] describes how to form and operate coalitions of intelligent robots using knowledge processors and information stored on the blockchain. To ensure the interaction of heterogeneous robots in the cyber-physical space, an ontology can be applied, which represents the knowledge and competencies of robots in the system. The methodology provides the fast information exchange between coalition members and smart contracts for the distribution of sensory, computational, control and service tasks between intelligent robots, embedded devices and information resources. Similarly, in the paper [20] the cyber-physical-social system is considered that is operated based on a smart space technology and blockchain concept. Authors propose case studies that include mobile robots and human participation. Interaction between robots and humans is implemented based on ontologybased publication/subscription mechanism and all information exchange traffic has been monitored and key information is stored in the blockchain network. 

Another solution that integrates robotics and blockchains is studied in the paper [21], which offers a modular architecture that utilizes the RobotChain framework [22] as a decentralized ledger for registering robotic events, smart contract 

technology for managing robots, and Oracle for processing data of any types. The modular architecture can be used in various contexts, such as production, network management, robot management, etc., since it is easy to integrate, adapt, maintain and expand for new domains. The examples of applications include: (a) task distribution between a robot network; (b) supporting methodology to assist robots in their task performance if they have cannot execute or need the specific information (e.g. a robot could not know what are the objects whereas other robots could); (c) robot productivity estimation or workability issue detection that can be valuable for industrial applications; (d) voting consensus for swarm robotics. What is more, tokens can be dropped to speed up the validation process or replaced by a reputation system for task management and consensus, since monetary value is no longer meaningful for private blockchain-based networks. 

The article [23] discusses the implementation of intelligent cyber-physical systems as multi-agent systems with the ability to schedule tasks by agents. In such multi-agent systems, the plan execution protocol should lead to proper completion and streamlining of actions, despite their distributed execution. However, in unreliable scenarios, there is a probability that agents will not follow the protocol due to faults or due to malicious reasons that lead to plan failure. In order to prevent such situations, the authors [23] proposed the proper plan execution by agents through smart contracts that will help accomplish the task even in untrusted environment. The researchers [23] developed the architectures in which smart contracts can be automatically generated from derived plans, as a result the entire system of smart, intelligent agents can be fully automated and the overall architecture will seamlessly integrate agents into one cyber-physical system. 

## III. THE CLASSIFICATION OF THE BLOCKCHAIN-BASED ROBOTICS APPLICATIONS 

This section classifies the most typical cases for blockchainbased multi-agent systems concerning robotics applications, which we discovered during our study. The identified cases are shown in Fig. 1. 

## _A. Logging agent actions with a bytecode distribution_ 

Consider an environment which consists of tons of very general agents. The agent has no predefined control program but configured as so could execute a batch of commands provided in form of a bytecode or similar. For example, an agent is a differential drive robot with sensors and gripper so the sequence of commands to execute could be: follow the wall until discover the room with the green floor, find the yellow box and bring it to the target point. The word bytecode is used here as a description of the platform-independent executable code. In other words, the same byte code will lead to the same sequence of command execution on any agent. Thus, at the moment of sending the bytecode, it is not necessary to know the exact hardware platform of the agent that must execute the commands specified in the bytecode. 

The blockchain could be used here to distribute such bytecodes to the agents: 

- The system which generates the tasks should not be connected to the agent directly. The peer-to-peer network is used to deliver the message. 

- A message will be delivered even if an agent is turned off. - The agent is able to inform constantly about its state changes (e.g. ”moved forward for 1 meter”, ”picked the box up”). The state is stored in the blockchain therefore it could be recovered quickly in case of the agent recovery after halt. 

- The delayed bytecode execution could be scheduled. 

- Two or more command consequences could be automatically queued to be executed by the agent. 

## _B. Distributed decision making by a time-limited voting_ 

Distributed decision making is a very important task in multi-agent systems, especially in SWARM systems. Blockchain application for this task had been already proposed [24]. However, proposed solution was based on the idea of sending coins to some ”proposal” addresses. In general, blockchain provides advanced technologies, which can solve this task in a better way, specifically, through smart-contracts, for instance in Ethereum. 

One can develop smart-contract(s) to build infrastructure, providing ability to implement polls with complicated behavior, like time-limited voting or vote delegating. 

By combining this approach with the previous case, we can get another interesting solution. The usage of smart-contract agents can propose some actions, formulated in bytecode. Other agents can vote for actions. Finally, collaboratively generated scenario can be gathered from the smart-contract. 

## _C. Action validation to exclude intruders or faulty agents_ 

Fig. 1. The classification of the most typical cases for using blockchain (BC) in multi-agent systems (MAS) concerning robotics applications 

Agents could be used to validate actions, location or poses of each other. Lets consider the swarm where agents perform some movements as per the general goal. Periodically the agents send information about the actual sensor information and the agent location based on odometry. Sometimes the agent could start misoperation and send incorrect data. Information received from other peers could be used to get to consensus that the agent works improperly and recovery 

procedure could start. In this case co-evolution scenarios could be further applied. The consensus based on the information received from other agents could be also used aiming to identify the robot that misbehaves intentionally e.g. it was hacked or injected by an intruder. In order to resolve the issue with performance of validators the approach which is called _Sharding_ could be used [1]. The data from the agents is combined based on their location: thus a shard is built. Validators are negotiated for the shard that is why the volume of the information to treat is reduced significantly. 

## _D. Economic Incentive for task execution optimization_ 

The financial part of the blockchain can be used as a basis for the approach to Multi-robot exploration controlled by a market economy proposed by researchers [25] from Carnegie Mellon University, where multirobot mapping was performed using the market approach to robot team coordination and to maximize the information acquisition at minimal cost. This multi-robot exploration system demonstrated reliability and adaptability to a dynamic environment and to the loss of colony members in addition to its ability to withstand communication losses and failures. The researchers found that by allowing robots to negotiate using the market architecture. The environment exploration effectiveness was improved several times for the robot team. Although the algorithm was designed to minimize the distance traveled during the exploration, the usage of time-based costs instead of distance-based costs would lead to much faster exploration. Such an approach can simplify setting priorities for some types of compared tasks to others within the market, for example, if there are other mission objectives in addition to the exploration [25]. 

## _E. Automated task dispatching via blockchain_ 

The distributed consensus existing as a part of the blockchain-based solutions could be used to perform task dispatching among competing agents for the assignment to complete the task. 

The dispatching code is written in form of the smart contract stored in the blockchain: 

- 1) The customer sends a request to perform a task to the dispatcher of the smart contract. 

- 2) The dispatcher notifies the agents about the new request. 

- 3) The agents send agreement to perform the task to the blockchain through peer-to-peer network. 

- 4) Blockchain validators define the order of the agreements as per the fee the particular agent is paying for handling of the agreement. 

- 5) The first agreement received by the dispatcher is confirmed by the code of the smart contract and the details of the order are provided to the corresponding agent. 

Thus, the market will regulate the selection of agents to be dispatched to complete the task. The agents, which were provided with a cost optimization strategy, could pay more to be selected by validators. This should result in appearing the most efficient and stable service providers. 

The case study towards the creation of such a multi-agent system is presented in the study [18]], in which an automated dispatching taxi scenario is implemented with the liability execution validation (by comparing the traveled route with a map by a validator). 

## _F. Authentication / suitability check_ 

There are situations when agents who do not trust each other but use a shared physical resource, or when charging and billing process may put users at risk of hackers attack or compromise the privacy and/or reveal their confidential information and/or location. 

One of the examples is a replaceable battery for electric vehicles (EV). Vehicles use maintenance stations to replace batteries. Both the maintenance and the vehicles owner are interested to ensure that the battery meets the quality issues and that the existing level of battery amortization is declared, since fraud with this shared resource can lead to a loss of profit and trust in such a service. For EV owners, the battery information and transaction correctness, openness, traceability and immutability is difficult to get guarantee in traditional centralized system. The trust lacking between EV owners and the maintenance station can cause a big challenge to the EV market development. The blockchain-based solutions can be used to provide battery authentication services. The smart contract code stored on the blockchain cannot be changed, hence battery amortization state is available to any deal participants. The maintenance station and the vehicle can connect to any node in the blockchain aiming to perform the battery check, so ”the man-in-the-middle attack” becomes difficult to perform the data replacement. 

The case study of such a system was proposed in the paper [26], where the issue of battery swapping and trust lacking was resolved through application of a decentralized blockchain system. Within this solution, both battery life-cycle information and all operation histories were permanently saved in the blockchain network. All key logics were driven by smart contracts, the battery price calculation and the digital currency exchange between EV owners and the station were realized by smart contracts automatically aiming to resolve the lack of trust issues. Similarly, the proposed research [27] discusses the new autonomous charging architecture without involving any human. It also provides a billing framework for Electric Autonomous Vehicles (EAVs) which allow cost-effective Machine-to-Machine (M2M) transactions using Distributed Ledger Technology (DLT), possessing greater resistance to hacker attacks and retaining confidential user information. 

## IV. DISCUSSION 

At present, the approach to organizing an immutable distributed database that stores all relevant information and provides access to agents of a multi-agent robotic system, expanding the capabilities of the system as a whole, has the considerable interest within the Fourth Industrial Revolution concept. The key component of the approach is a distributed 

ledger technology (blockchain) associated with a virtual computer, which allows agents to interact through responsible smart contracts [13], [14]. On the one hand, the reliability of the agent is mainly determined by the reputational model. However, this allows you to determine the trust level to the agent only after the fulfillment of the agreed obligations. On the other hand, the automation of obligation fulfillment by an agent can provide a verification procedure that will allow to verify the liability execution (e.g., using formal methods [18]). In addition, one of the goals of the implementation of the blockchain for a multi-agent system may be the increase of the interaction efficiency between agents by organizing more trusted information support. 

The main idea of this paper is to provide a guidance on how frameworks back-uped by blockchain solutions could be used to address practical tasks faced by multi-agent systems, especially for groups of mobile robots. The relevant reviewed studies show up that the blockchain could play a significant role in multi-agent system applications. The analysis of recent publications allow us to identify groups of tasks for blockchain-based multi-agent robotic systems, which we proposed for classification. Blockchain technologies could be used to extend existing number of platforms and libraries used by researchers or motivate them to use a common solution that is widespread and verified instead of putting efforts in developing their own code solutions to cover similar scenarios. 

Based on the state-of-the-art analysis authors conclude that at the moment the promising task in the area of multi-agent systems is the development of methodologies, models and methods focused on an intelligent support of blockchain-based agent interactions that significantly increase the level of trust in such systems. The following tasks remain ”opened” at the moment and need to be solved: 

- Development of a conceptual model of information support for a group of robots during the task performance; 

- Development of a typical ontological model of a robotic system; 

- Development of a consensus protocol for a group interaction verification before launching a task based on the information from a distributed ledger; 

- Development of a validation method for task performance by the robotic system; 

- Development of multi-agent system architecture; 

- Prototyping of a framework for an intelligent group of robots performing a collaborative task (as a proof-ofconcept). 

## REFERENCES 

- [1] ”Blockchain faq.” https://medium.com/edchain. Accessed: 2018-03-11. 

- [2] F. Bullo, J. Cortes, and S. Martinez, _Distributed control of robotic networks: a mathematical approach to motion coordination algorithms_ , vol. 27. Princeton University Press, 2009. 

- [3] H. Liu, L. Cheng, M. Tan, Z. Hou, and Y. Wang, “Distributed exponential finite-time coordination of multi-agent systems: containment control and consensus,” _Int. J. of Control_ , vol. 88, no. 2, pp. 237–247, 2015. 

- [4] Z. Peng, G. Wen, A. Rahmani, and Y. Yu, “Distributed consensusbased formation control for multiple nonholonomic mobile robots with a specified reference trajectory,” _International Journal of Systems Science_ , vol. 46, no. 8, pp. 1447–1457, 2015. 

- [5] A. V. Savkin, C. Wang, A. Baranzadeh, Z. Xi, and H. T. Nguyen, “Distributed formation building algorithms for groups of wheeled mobile robots,” _Robotics and Autonomous Systems_ , vol. 75, pp. 463–474, 2016. 

- [6] X. Wang, Z. Zeng, and Y. Cong, “Multi-agent distributed coordination control: developments and directions via graph viewpoint,” _Neurocomputing_ , vol. 199, pp. 204–218, 2016. 

- [7] M. Pawlak, A. Poniszewska-Mara´nda, and N. Kryvinska, “Towards the intelligent agents for blockchain e-voting system,” _Procedia Computer Science_ , vol. 141, pp. 239–246, jan 2018. 

- [8] R. Skowro´nski, “The open blockchain-aided multi-agent symbiotic cyberphysical systems,” _Future Generation Computer Systems_ , vol. 94, pp. 430–443, may 2019. 

- [9] F. Casino, T. K. Dasaklis, and C. Patsakis, “A systematic literature review of blockchain-based applications: current status, classification and open issues,” _Telematics and Informatics_ , 2018. 

- [10] X. Wang, X. Zha, W. Ni, R. P. Liu, Y. J. Guo, X. Niu, and K. Zheng, “Survey on blockchain for Internet of Things,” _Computer Communications_ , vol. 136, pp. 10–29, feb 2019. 

- [11] M. Mazzara, I. Afanasyev, S. R. Sarangi, S. Distefano, and V. Kumar, “A reference architecture for smart and software-defined buildings,” _arXiv preprint arXiv:1902.09464_ , 2019. 

- [12] C. Lazaroiu and M. Roscia, “Smart district through iot and blockchain,” in _2017 IEEE 6th International Conference on Renewable Energy Research and Applications (ICRERA)_ , pp. 454–461, IEEE, 2017. 

- [13] N. Teslya and I. Ryabchikov, “Blockchain-based platform architecture for industrial iot,” in _2017 21st Conference of Open Innovations Association (FRUCT)_ , pp. 321–329, IEEE, 2017. 

- [14] A. Kapitonov, I. Berman, S. Lonshakov, and A. Krupenkin, “Blockchain based protocol for economical communication in industry 4.0,” in _2018 Crypto Valley Conference on Blockchain Technology (CVCBT)_ , pp. 41– 44, IEEE, 2018. 

- [15] R. Casado-Vara, J. Prieto, F. D. la Prieta, and J. M. Corchado, “How blockchain improves the supply chain: case study alimentary supply chain,” _Procedia Computer Science_ , vol. 134, pp. 393–398, jan 2018. 

- [16] D. Calvaresi, A. Dubovitskaya, J. P. Calbimonte, K. Taveter, and M. Schumacher, “Multi-agent systems and blockchain: Results from a systematic literature review,” in _Int. Conf. on Practical Applications of Agents and Multi-Agent Systems_ , pp. 110–126, Springer, 2018. 

- [17] A. Kapitonov, S. Lonshakov, A. Krupenkin, and I. Berman, “Blockchainbased protocol of autonomous business activity for multi-agent systems consisting of uavs,” in _Workshop on Research, Education and Development of Unmanned Aerial Systems (RED-UAS)_ , pp. 84–89, IEEE, 2017. 

- [18] K. Danilov, R. Rezin, A. Kolotov, and I. Afanasyev, “Towards blockchain-based robonomics: autonomous agents behavior validation,” in _International Conference on Intelligent Systems_ , IEEE, 2018. 

- [19] N. Teslya and A. Smirnov, “Blockchain-based framework for ontologyoriented robots coalition formation in cyberphysical systems,” in _MATEC Web of Conferences_ , vol. 161, p. 03018, EDP Sciences, 2018. 

- [20] A. Kashevnik and N. Teslya, “Blockchain-Oriented Coalition Formation by CPS Resources: Ontological Approach and Case Study,” _Electronics_ , vol. 7, p. 66, may 2018. 

- [21] V. Lopes, L. A. Alexandre, and N. Pereira, “Controlling robots using artificial intelligence and a consortium blockchain,” _arXiv preprint arXiv:1903.00660_ , 2019. 

- [22] E. C. Ferrer, O. Rudovic, T. Hardjono, and A. Pentland, “Robochain: A secure data-sharing framework for human-robot interaction,” _arXiv preprint arXiv:1802.04480_ , 2018. 

- [23] A. Shukla, S. K. Mohalik, and R. Badrinath, “Smart contracts for multiagent plan execution in untrusted cyber-physical systems,” in _2018 IEEE 25th International Conference on High Performance Computing Workshops (HiPCW)_ , pp. 86–94, IEEE, 2018. 

- [24] E. C. Ferrer, “The blockchain: a new framework for robotic swarm systems,” in _Proceedings of the Future Technologies Conference_ , pp. 1037– 1058, Springer, 2018. 

- [25] R. Zlot, A. Stentz, M. B. Dias, and S. Thayer, “Multi-robot exploration controlled by a market economy,” in _Robotics and Automation, 2002. Proceedings. ICRA’02. IEEE International Conference on_ , vol. 3, pp. 3016–3023, IEEE, 2002. 

- [26] S. Hua, E. Zhou, B. Pi, J. Sun, Y. Nomura, and H. Kurihara, “Apply blockchain technology to electric vehicle battery refueling,” in _Proceedings of the 51st International Conference on System Sciences_ , 2018. 

- [27] D. Strugar, R. Hussain, M. Mazzara, V. Rivera, I. Afanasyev, and J. Lee, “Towards blockchain-based robonomics: autonomous agents behavior validation,” in _Adv. Inf. Networking & App. Workshops (WAINA)_ , 2019. 

