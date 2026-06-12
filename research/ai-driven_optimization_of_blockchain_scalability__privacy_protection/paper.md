_**algorithms**_ 

## _Review_ 

## **AI-Driven Optimization of Blockchain Scalability, Security, and Privacy Protection** 

**Fujiang Yuan[1] , Zihao Zuo[1] , Yang Jiang[1] , Wenzhou Shu[2] , Zhen Tian[3] , Chenxi Ye[4] , Junye Yang[1] , Zebing Mao[5] , Xia Huang[1] , Shaojie Gu[6] and Yanhong Peng[1,] *** 

- 1 College of Mechanical Engineering, Chongqing University of Technology, Chongqing 400054, China; yuanfujiang@ctbu.edu.cn (F.Y.) 

- 2 School of French Studies, Sichuan International Studies University, Chongqing 400031, China 3 James Watt School of Engineering, University of Glasgow, Glasgow G12 8QQ, UK 

- 4 Faculty of Science and Technology, Hong Kong Baptist University, Hong Kong 999077, China 5 Department of Engineering Science and Mechanics, Shibaura Institute of Technology, 3-7-5 Toyosu, Koto-ku, Tokyo 135-8548, Japan 

- 6 Magnesium Research Center, Kumamoto University, Kumamoto 860-8555, Japan 

- Correspondence: yhpeng@nagoya-u.jp 

Academic Editor: Sergey Y. Yurish Received: 1 April 2025 Revised: 28 April 2025 Accepted: 29 April 2025 Published: 2 May 2025 

**Citation:** Yuan, F.; Zuo, Z.; Jiang, Y.; Shu, W.; Tian, Z.; Ye, C.; Yang, J.; Mao, Z.; Huang, X.; Gu, S.; et al. AI-Driven Optimization of Blockchain Scalability, Security, and Privacy Protection. _Algorithms_ **2025** , _18_ , 263. https:// doi.org/10.3390/a18050263 

**Copyright:** © 2025 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license (https://creativecommons.org/ licenses/by/4.0/). 

**Abstract:** With the continuous development of technology, blockchain has been widely used in various fields by virtue of its decentralization, data integrity, traceability, and anonymity. However, blockchain still faces many challenges, such as scalability and security issues. Artificial intelligence, with its powerful data processing capability, pattern recognition ability, and adaptive optimization algorithms, can improve the transaction processing efficiency of blockchain, enhance the security mechanism, and optimize the privacy protection strategy, thus effectively alleviating the limitations of blockchain in terms of scalability and security. Most of the existing related reviews explore the application of AI in blockchain as a whole but lack in-depth classification and discussion on how AI can empower the core aspects of blockchain. This paper explores the application of artificial intelligence technologies in addressing core challenges of blockchain systems, specifically in terms of scalability, security, and privacy protection. Instead of claiming a deep theoretical integration, we focus on how AI methods, such as machine learning and deep learning, have been effectively adopted to optimize blockchain consensus algorithms, improve smart contract vulnerability detection, and enhance privacy-preserving mechanisms like federated learning and differential privacy. Through comprehensive classification and discussion, this paper provides a structured overview of the current research landscape and identifies potential directions for further technical collaboration between AI and blockchain technologies. 

**Keywords:** AI; blockchain; consensus; smart contract 

## **1. Introduction** 

Since its inception in 2008, the concept of blockchain has evolved from merely serving as the foundational technology for cryptocurrencies to enabling innovative applications across various industries [1]. In 2016, Bitcoin’s market capitalization surpassed USD 10 billion for the first time, marking a critical milestone that elevated public attention towards blockchain technology beyond its association with cryptocurrencies [2]. This financial threshold catalyzed a shift in perception, positioning blockchain as a decentralized peer-to-peer infrastructure capable of maintaining transparent, tamper-resistant ledgers across various domains. Consequently, the concept of “Blockchain+” emerged, promoting 

_Algorithms_ **2025** , _18_ , 263 

https://doi.org/10.3390/a18050263 

_Algorithms_ **2025** , _18_ , 263 

2 of 67 

blockchain’s integration into industries such as finance, supply chain management, healthcare, and public services [3–5]. This evolution highlights blockchain’s transition from a niche technology to a cornerstone of global digital innovation and competition. 

Blockchain is an integrated innovation that combines key technological components such as peer-to-peer networks, Byzantine fault tolerance, smart contracts, and distributed consensus algorithms [6]. By leveraging these technologies, blockchain establishes a decentralized system that enables trust formation in the digital space. Its core mechanism relies on nodes within a peer-to-peer network reaching consensus through a consensus protocol and sequentially linking data blocks into an immutable chain. Each block employs cryptographic techniques, such as digital signatures, to ensure data integrity and prevent forgery or tampering, thereby realizing a decentralized shared ledger. Compared to traditional centralized systems, this innovative paradigm reduces reliance on central authorities, lowers the cost of trust and consensus, and enhances system security and reliability. As a result, blockchain presents profound transformations and challenges to conventional social structures and operational models [7]. 

The general-purpose architecture of blockchain enables its flexible expansion across various application domains. Although no official standard exists, the development of blockchain technology is commonly described as progressing through three generational trends, as shown in Figure 1: 

- Blockchain 1.0 is generally associated with programmable digital cryptocurrencies, with a primary focus on value transfer, such as Bitcoin, digital assets, and online payments. However, this early phase faced limitations in scalability and offered relatively limited functionality [8]. 

- Blockchain 2.0 is characterized by the introduction of smart contracts, enabling blockchain not only to store and execute computer programs but also to facilitate trusted digital contract management. This trend expanded blockchain applications into the financial sector, supporting stock trading, clearing, private equity, and other services, thereby promoting the digital collaboration of real-world business systems [9]. 

- Blockchain 3.0 further extends blockchain use beyond the financial sector into broader industries, such as the Internet of Things, mining, electric transport management [10], industrial process optimization [11], privacy protection, and product traceability. By leveraging its decentralized and trustless architecture, blockchain in this phase aims to reduce inter-organizational friction, enhance large-scale collaboration, and drive higher productivity for individuals and enterprises [12]. 

Nowadays, blockchain has been applied across various fields, including finance [13], agriculture [14], food [15], supply chain management [16], and healthcare. For instance, in food safety traceability, consortium blockchain enables real-time data sharing and collaborative mechanisms, addressing challenges such as information asymmetry and the complexity of traditional traceability processes. This not only enhances the efficiency of food safety traceability but also reduces labor and time costs. Despite the promising prospects of blockchain technology, it still faces numerous challenges. 

- Scalability Issues: Due to the inherent limitations of block size and the time-consuming consensus process, blockchain networks struggle with transaction throughput and data storage constraints. These factors significantly limit the efficiency and feasibility of large-scale applications. The scalability challenges faced by common consensus algorithms in blockchain are shown in Table 1. 

- Security Risks: The widely used PoW consensus mechanism requires multiple block confirmations—typically at least six—to probabilistically ensure transaction secu- 

_Algorithms_ **2025** , _18_ , 263 

3 of 67 

   - rity, leading to delays in finalizing transactions. Moreover, if a single entity gains control of more than 51% of the total network computing power, it could launch a double-spending attack or maliciously fork the blockchain, posing severe threats to the integrity and security of the system. The security issues faced by common consensus algorithms in blockchain are shown in Table 2. 

- Privacy Protection Challenges: While blockchain provides a certain degree of anonymity, it is not immune to privacy risks. For instance, attackers can analyze transaction histories and trace network node IP addresses to infer users’ identities. Additionally, improper implementation of decentralized applications (DApps) may lead to unintended exposure of sensitive data, exacerbating privacy concerns and further complicating regulatory compliance. 

**Table 1.** Scalability issues of common consensus algorithms. 

|**Consensus Algorithm**|**Main Scalability Issues**|**Description**|
|---|---|---|
|PoW|Low Throughput, High Energy<br>Consumption|Block creation is limited by diffculty; TPS is<br>low and hard to scale|
|PoS|Node Centralization, Sync Dependency|Requires up-to-date state sync; less suitable<br>for large distributed systems|
|DPoS|Limited by Supernodes|Faster than PoW/PoS, but scalability is<br>constrained bylimited nodeparticipation|
|||High communication overhead;|
|PBFT|O(n^2) Message Complexity|performance degrades with increasing node|
|||count|
|RAFT|Poor State Sync for Many Nodes|Suitable for small-scale systems only; does<br>not scale well|
|Paxos|Complex Implementation, Performance<br>Bottleneck|Multi-phase protocol has high latency and<br>poor scalability|



**Table 2.** Security issues of common consensus algorithms. 

|**Consensus Algorithm**|**Main Security Issues**|**Description**|
|---|---|---|
|||A single entity controlling 51% of total hash|
|PoW|51% Attack, Selfsh Mining|power can manipulate the blockchain; selfsh|
|||miningleads to temporaryforks|
|PoS|Rich-get-richer, Long-range Attack, Nothing<br>at Stake|Stake centralization increases over time; easy<br>to rewrite chain history; validators may vote<br>on multiple forks|
|DPoS|Centralization Risk, Vote Manipulation|A few supernodes control consensus; voting<br>power can be bought or manipulated|
|||Small node sets are vulnerable to targeted|
|PBFT|Node Exposure, DoS Attacks|DoS; limited fault tolerance (up to f faulty|
|||nodes)|
|||Cannot defend against malicious nodes; only|
|RAFT|Not Byzantine Fault Tolerant|tolerates crash faults in trusted|
|||environments|
|Paxos|Weak Byzantine Resistance|Assumes honest behavior; not suitable<br>against data forgery or malicious attacks|



_Algorithms_ **2025** , _18_ , 263 

4 of 67 

**Figure 1.** Blockchain development trends. 

With its efficient data processing, autonomous learning, and advanced pattern recognition capabilities, AI has become a key driving force in the advancement of blockchain technology, with applications spanning edge computing, big data monitoring, supply chains, digital twins, and metaverse exploration. He et al. [17] emphasize that blockchain, due to its decentralized, trustless, and immutable nature, is well suited for edge-centric IoT scenarios while also highlighting the integration of AI into edge computing. Islam et al. [18] propose a blockchain-based AI-driven initiative that utilizes drones for pandemic surveillance, demonstrating the feasibility of autonomous monitoring. D’Souza et al. [19] introduce a secure pharmaceutical supply chain management system that leverages blockchain and AI, incorporating the Rasa chatbot to enhance functionality. Suhail et al. [20] conduct a comprehensive survey on blockchain-based digital twins, discussing research trends, advantages, and challenges in this field. Guergov et al. [21] analyze the convergence of IoT, AI, and blockchain, underscoring the necessity of hybrid models for seamless integration. Yang et al. [22] explore the intersection of blockchain, AI, and the metaverse, highlighting the potential for interdisciplinary research and collaboration. Furthermore, Kumar et al. [23] examine the applications and benefits of integrated AI and blockchain platforms across various business verticals, identifying ten key areas with significant potential advantages. Junaid et al. [24] investigate advancements in emerging technologies, including IoT, AI, and blockchain, with a particular focus on healthcare, emphasizing their critical role in healthcare management systems. Finally, Xu et al. [25] highlight the role of key enabling technologies, such as digital twins, AI, and blockchain, in the development of digital twin-driven Industrial Internet of Things applications. 

_Algorithms_ **2025** , _18_ , 263 

5 of 67 

In this survey, we conduct a comprehensive analysis of existing research, providing insights into how machine learning and artificial intelligence are generally integrated with blockchain technology and their specific applications. Our study focuses on four key aspects: optimizing consensus algorithm efficiency through AI, enabling automated vulnerability detection and remediation in smart contracts, constructing verifiable data architectures, and enhancing privacy protection using federated learning. Table 3 presents a comparison between our survey and existing survey studies. 

**Table 3.** Comparison of existing reviews on AI-driven blockchain optimization. 

|**Author**|**Year**|**Core Mechanics**|**Pros**|**Cons**|
|---|---|---|---|---|
|Verma, D. et al. [26]|2022|Application of blockchain and<br>AI in plastic recycling.|Improve recycling effciency<br>and support environmental<br>protection.|Data privacy and security<br>issues not involved; lack of<br>practical cases.|
|Badidi, E. et al. [27]|2022|Edge AI and blockchain for<br>smart sustainable cities.|Provide smart city application<br>cases to improve<br>infrastructure management.|Lack of blockchain<br>performance optimization and<br>privacy protection solutions.|
||||Solve privacy issues of||
|Haddad, A. et al. [28]|2022|Blockchain + AI for electronic<br>health records.|electronic health records and<br>improve data management|Scalability optimization of<br>blockchain not discussed.|
||||effciency.||
|Kumar, R. et al. [29]|2023|AI-driven blockchain<br>applications in public health.|Covers multiple public health<br>applications and provides<br>real-world examples.|Lacks adequate discussion on<br>technical challenges and<br>deployment scenarios.|
|Uddin, M. et al. [30]|2024|Integration of Metaverse,<br>blockchain, and AI.|Innovative analysis<br>emphasizing digital currency<br>and data security.|Lack of factual industry<br>application cases and<br>insuffcient in-depth analysis.|
|Omidian, H. et al. [31]|2024|Synergy of blockchain and AI<br>in healthcare.|Emphasis on data privacy,<br>security, and innovative<br>solutions.|Lack of technical<br>implementation steps and<br>practical issues.|
|Zhou, Q. et al. [32]|2022|Application of AI, big data,<br>and blockchain in food safety.|Emphasis on data traceability<br>with practical application<br>value.|Lack of in-depth discussion on<br>privacy protection and<br>blockchain optimization.|
||||Clearly point out the||
|||Comprehensively summarize|systematic challenges of the||
|||the optimization role of AI in|four core modules of||
|||the four core modules of|blockchain (consensus|Lack of actual case|
|Our Survey|2025|blockchain (consensus|algorithm, smart contract,|demonstrations in each|
|||algorithm, smart contract,|privacy protection, data|category.|
|||privacy protection, data|retrieval) in scalability,||
|||retrieval).|security, and privacy, and||
||||analyze them in detail.||



## **2. Introduction to Related Technologies** 

This section provides an in-depth introduction to blockchain and AI technologies, with a particular focus on the four core technologies of blockchain: distributed ledger, consensus mechanism, smart contracts, and cryptographic security. Additionally, it elaborates on key AI technologies, including machine learning, deep learning, and reinforcement learning, exploring their roles and development trends across various application scenarios. 

## _2.1. Blockchain_ 

Blockchain technology, as a distributed database, stores data in an encrypted manner and ensures data security and immutability through its chain structure [33]. Its decentralized nature enables distributed data sharing and management, ensuring reliability 

_Algorithms_ **2025** , _18_ , 263 

6 of 67 

without the need for trusted intermediaries [34]. Consensus algorithms facilitate agreement among multiple nodes, safeguarding data security, while smart contracts, as self-executing code, automatically execute when predefined conditions are met, further enhancing the application value of blockchain [35]. 

## 2.1.1. Block Structure 

A blockchain block is composed of two primary elements: the Block Header and the Block Body. Its cryptographic design, combined with a layered architecture, establishes the fundamental technical basis for data immutability [36]. This structure ensures secure data storage through the following three layers: 

- (1) The information encapsulation layer of the block header 

The block header serves as the core verification unit of the blockchain, encapsulating six key fields: version number, timestamp, previous block hash, Merkle root, nonce, and target hash. Among them, the Merkle root—computed via a layered binary hash tree—ensures transaction integrity by uniquely summarizing all transactions within the block [37]. 

- (2) The data storage layer of the block structure 

The block body organizes verified transactions using a Merkle tree structure, where leaf nodes represent hashed transactions and non-leaf nodes result from recursive hash combinations. This design enables efficient O(log n) transaction verification and supports lightweight node validation. All transactions must pass digital signature and input–output checks before inclusion [38]. 

- (3) The collaborative layer of the security mechanism 

The block header and body are cryptographically linked, ensuring immutability. Any modification in transaction data alters the Merkle root, invalidating subsequent blocks. Combined with timestamps and hash chaining, this layered architecture preserves data integrity while enabling scalable storage and efficient validation.The structure of a block is illustrated in Figure 2. 

**Figure 2.** Block structure diagram. 

_Algorithms_ **2025** , _18_ , 263 

7 of 67 

## 2.1.2. Consensus Algorithm 

The reliability of the blockchain system is highly dependent on the design of the consensus mechanism [39], a technology that realizes efficient transaction verification by coordinating the protocols among network nodes and is decisive for maintaining data consistency and operational integrity. 

Currently, blockchain can be categorized into public, private, and consortium chains based on permission scope and access control mechanisms [40]. Each type exhibits distinct characteristics and serves different application scenarios. Public chains, known for their high level of decentralization, transparency, and censorship resistance, allow any participant to join, validate transactions, and contribute to the consensus process. They are widely utilized in areas such as cryptocurrency transactions, decentralized finance, and non-fungible tokens [41]. However, the open nature of public chains often leads to scalability and energy consumption challenges. 

In contrast, private chains operate under restricted access where only authorized participants can read, write, or audit the blockchain. They prioritize data security, privacy protection, and efficient management, operating with a significantly lower degree of decentralization. This model is particularly suitable for industries such as finance, healthcare, and supply chain management, where regulatory compliance, sensitive data handling, and operational efficiency are critical. 

Consortium chains, also known as federated blockchains, integrate features from both public and private chains. They are governed by a group of pre-selected nodes or organizations, achieving a balance between multiparty cooperation, transparency, and controlled decentralization [42]. Consortium chains offer enhanced performance, scalability, and trust through partial decentralization while maintaining higher privacy standards compared to public chains. Their strengths in traceability, auditability, and governance mechanisms facilitate transaction tracking, cross-organizational collaboration, and comprehensive historical record management, making them ideal for applications in interbank clearing, supply chain traceability, and cross-border payments. Table 4 presents a detailed comparison of blockchain categories in terms of decentralization level, security, scalability, transparency, and typical application scenarios [43]. 

**Table 4.** Comparison of blockchain categories. 

|**Type**|**Public Blockchain**|**Consortium Blockchain**|**Private Blockchain**|
|---|---|---|---|
|**Participants**|Anyone|Consortium members|Internal to an organization|
|**Consensus Mechanism**|PoW / PoS / DPoS|PBFT / Kafka / BFT|Raft / Paxos|
|**Incentive Mechanism**|Required|Optional|Not required|
|**Degree of Decentralization**|Decentralized|Partially decentralized|(Partially) centralized|
|**Data Consistency**|(weak) consistency|(strong) consistency|(strong) consistency|
|**Network Scale**|Large|Relatively large|Small|
|**Typical Applications**|Cryptocurrencies|Payments|Auditing|



Consensus algorithms play a crucial role in distributed systems [44], particularly in blockchain technology, where they ensure that all nodes in the network agree on the validity of transactions [45]. Different consensus mechanisms exhibit various strengths and weaknesses, influencing aspects such as security, energy consumption, throughput, and decentralization. The following Table 5 provide a detailed discussion of several mainstream consensus algorithms. 

_Algorithms_ **2025** , _18_ , 263 

8 of 67 

**Table 5.** Comparison of consensus algorithms. 

|**Consensus**<br>**Algorithm**|**Core Description**|**Application**<br>**Examples**|**Reward Mechanism**|**Advantages**|**Disadvantages**|
|---|---|---|---|---|---|
||Consensus is||Rewards miners|||
||achieved through||with block rewards|High security,|High energy|
|PoW|competitive|Bitcoin|and transaction fees,|resistant to Sybil|consumption, low|
||cryptographic||but computationally|attacks|throughput|
||puzzles||expensive|||
|PoS|Nodes gain block<br>validation rights<br>based on the amount<br>and duration of<br>cryptocurrency held|Ethereum 2.0|Holders receive<br>block rewards,<br>malicious actors may<br>be penalized|Low energy<br>consumption,<br>effcient|May lead to wealth<br>centralization|
|DPoS|Representatives are<br>elected to validate<br>and confrm<br>transactions|EOS, TRON|Delegates receive<br>rewards, malicious<br>representatives can<br>be voted out|High effciency, low<br>energy consumption|Potential<br>centralization, risk<br>of delegate collusion|
||Consensus is|||||
|PBFT|reached through<br>multiple rounds of<br>voting, suitable for a<br>small number of|Hyperledger Fabric,<br>fnancial blockchain|No economic<br>incentives, relies on<br>node trust|Low latency, high<br>throughput|No cryptocurrency<br>incentives, malicious<br>nodes may impact<br>consensus|
||nodes|||||
||Consensus is|||||
|Kafka|achieved based on a<br>reliable message<br>queue, commonly<br>used in enterprise|Hyperledger Fabric<br>(ordering service)|No economic<br>incentives, relies on<br>access control|High throughput,<br>suitable for private<br>chains|Relies on centralized<br>nodes, single point<br>of failure risk|
||architectures|||||
||Leader-based|||||
|Raft|replication for log<br>consistency, suitable<br>for small-scale|Enterprise private<br>blockchains|No economic<br>incentives|Simple<br>implementation,<br>easy deployment|Limited application<br>scope, performance<br>affected by leader|
||distributed systems|||||
|Paxos|Achieves consensus<br>in distributed<br>systems through<br>message passing|Google Chubby,<br>AWS DynamoDB|No economic<br>incentives|Suitable for strong<br>consistency<br>databases|High algorithmic<br>complexity,<br>signifcant<br>communication<br>overhead|



## (1) PoW 

PoW achieves consensus through competitive computation of cryptographic hash puzzles, ensuring transaction security and immutability. It is widely used in public blockchains such as Bitcoin. The primary advantage of PoW is its high security, effectively preventing Sybil attacks and maintaining network decentralization. However, PoW requires significant computational resources, leading to high energy consumption. According to the Cambridge Bitcoin Electricity Consumption Index (2024), the Bitcoin network consumes approximately 120 TWh of electricity annually, with each transaction consuming around 707 kWh, categorizing PoW as a high-energy consensus mechanism. Additionally, the limited block production speed results in low transaction throughput, making it challenging to support large-scale applications. 

_Algorithms_ **2025** , _18_ , 263 

9 of 67 

## (2) PoS 

PoS determines block validation rights based on the amount and duration of cryptocurrency holdings, avoiding the energy-intensive computations of PoW. Ethereum 2.0 adopts this mechanism. The main advantages of PoS include low energy consumption and high efficiency, which enhance transaction processing speed and reduce environmental impact. After Ethereum’s transition from PoW to PoS in 2022 (“The Merge”), its energy consumption decreased by over 99.95%, dropping to approximately 0.0026 TWh per year, with each transaction consuming only about 0.03 kWh, making PoS a low-energy consensus mechanism. However, PoS may lead to wealth centralization, as large token holders gain greater control over the network, potentially undermining decentralization [46]. 

## (3) DPoS 

DPoS relies on community voting to elect delegate nodes that execute consensus, significantly improving transaction processing efficiency. It is widely adopted by blockchains such as EOS and TRON. The key advantages of DPoS include high efficiency and low energy consumption, enabling rapid processing of large-scale transactions and enhancing system throughput. Energy studies estimate the EOS blockchain consumes only about 0.001 TWh annually, significantly lower than PoW-based systems, confirming that DPoS is a low-energy consensus approach. However, the delegate selection mechanism may result in power centralization, and there is a risk of collusion among delegates, which can weaken decentralization. 

As Algorithm 1 shows, the DPoS consensus mechanism operates through four main phases. Initially, all stakeholders ( _S_ ) vote to select a small group of delegates ( _D_ ), typically much fewer in number than the stakeholders, who are entrusted with the task of block production. During each round, delegates take turns to collect transactions, produce blocks, sign them, and broadcast them to the network. Each received block is validated by other nodes to ensure integrity and correctness. If any delegate misbehaves, such as missing blocks or producing invalid data, stakeholders have the right to revoke their delegation through a new voting process. This approach ensures efficient, scalable, and democratic block validation while significantly reducing the number of nodes involved in the consensus process compared to traditional proof-of-work mechanisms. 

## (4) PBFT 

PBFT achieves consensus through multiple rounds of voting among a limited number of nodes, making it suitable for high-trust private blockchain environments such as Hyperledger Fabric. Its main advantages are low latency and high throughput, making it particularly well suited for financial blockchain applications. 

PBFT does not involve mining or computational races. As a result, its energy consumption is extremely low, typically less than 0.0001 TWh annually, especially in enterprise deployments (IBM Blockchain Whitepaper, 2022). However, PBFT lacks economic incentives and relies on the honesty of participating nodes. If a significant number of malicious nodes are present, the consensus process may be compromised. 

_Algorithms_ **2025** , _18_ , 263 

10 of 67 

**Algorithm 1** DPoS Consensus Process 

1: **System Parameters:** 

2: _S_ = _{ss_ 1,, _s_ 2, ...,, ..., _sN}N}}_ 

2: _S_ = _{ss_ 1,, _s_ 2, ...,, ..., _sN}N}} ▷_ Set of all stakeholders 3: _D_ = _{d_ 1, _d_ 2, ..., _dM}_ where _M ≪ N ▷_ Set of elected delegates 4: _round_  length_ = _M ▷_ Each delegate produces one block per round 

5: **procedure** VOTING PHASE 

6: Each stakeholder votes for trusted delegates. 

7: Top _M_ candidates with most votes are selected as delegates. 

8: **end procedure** 

## 9: **procedure** BLOCK PRODUCTION PHASE 

10: **for** each round **do** 

11: **for** each delegate _di_ in scheduled order **do** 12: Collect valid transactions into a block. 13: Sign the block with private key. 14: Broadcast the block to the network. 

15: **end for** 16: **end for** 17: **end procedure** 

## 18: **procedure** BLOCK VALIDATION PHASE 

19: **for** each received block **do** 

20: **if** block is valid (signature and transactions) **then** 21: Append the block to the local blockchain. 

22: **else** 23: Report delegate misbehavior (e.g., missed block, invalid block). 24: **end if** 25: **end for** 26: **end procedure** 

## 27: **procedure** DELEGATE REPLACEMENT 

28: **if** a delegate misbehaves or misses blocks **then** 29: Stakeholders can vote to remove and replace the delegate. 30: **end if** 31: **end procedure** 

The PBFT consensus process is illustrated in Algorithm 2. Initially, the system defines _N_ = 3 _f_ + 1 nodes to tolerate up to _f_ faulty nodes. The primary node is selected based on the current view number. The consensus proceeds through several phases: 

- (1) Client Request: A client initiates the process by signing and sending a transaction request to the primary node. 

- (2) Pre-Prepare Phase: Upon verifying the client’s request, the primary node broadcasts a `PRE-PREPARE` message containing a digest of the request. 

- (3) Prepare Phase: Replica nodes validate the `PRE-PREPARE` message and broadcast `PREPARE` messages to all other nodes. 

- (4) Commit Phase: When a node receives 2 _f_ matching `PREPARE` messages, it broadcasts a `COMMIT` message. 

- (5) Reply Phase: After collecting 2 _f_ + 1 matching `COMMIT` messages, the node executes the operation and sends a reply to the client. 

- (6) View Change: If a timeout occurs or the primary is suspected to be faulty, nodes initiate a `VIEW-CHANGE` protocol to select a new primary node. This multi-phase voting ensures consensus integrity even in the presence of faulty or malicious participants. 

_Algorithms_ **2025** , _18_ , 263 

11 of 67 

**Algorithm 2** PBFT Consensus Process 

1: **System Parameters:** 

2: _N_ = 3 _f_ + 1, _V_ = _{v_ 1, _v_ 2, ..., _vN}_ , _primary_ = _vk_ , where _k_ = _view_ mod _|V|_ 

3: Initialize _seq_  num_ = 0 

4: **procedure** CLIENTREQUEST( _c_ , _m_ ) 5: _σ_ = Sign( _c_ , _⟨_ REQUEST, _m_ , _t⟩_ ) 6: Send _⟨_ REQUEST, _m_ , _t_ , _c_ , _σ⟩_ to primary 7: **end procedure** 

8: **procedure** PREPREPARE( _m_ , _t_ , _c_ , _σ_ ) 9: **if** verify( _σ_ ) and _seq_  num_ = _n_ **then** 10: Broadcast _⟨_ PRE-PREPARE, _v_ , _n_ , _d_ = _H_ ( _m_ ) _⟩_ 11: **end if** 12: **end procedure** 13: **procedure** PREPARE( _vi_ ) 14: **if** received valid _⟨_ PRE-PREPARE, _v_ , _n_ , _d⟩_ **then** 15: Broadcast _⟨_ PREPARE, _vi_ , _n_ , _d⟩_ 16: **end if** 17: **end procedure** 

18: **procedure** COMMIT( _vi_ ) 19: **if** received 2 _f ⟨_ PREPARE, _vj_ , _n_ , _d⟩_ **then** 20: Broadcast _⟨_ COMMIT, _vi_ , _n_ , _d⟩_ 21: **end if** 22: **end procedure** 

23: **procedure** REPLY 24: **if** received 2 _f_ + 1 _⟨_ COMMIT, _vj_ , _n_ , _d⟩_ **then** 25: Send _⟨_ REPLY, _v_ , _t_ , _r⟩_ to client 26: **end if** 27: **end procedure** 

28: **procedure** VIEWCHANGE( _vi_ ) 29: **if** timeout or primary fault detected **then** 30: Broadcast _⟨_ VIEW-CHANGE, _vi_ , _v_ + 1, _P_ , _Q⟩_ 31: **end if** 32: **end procedure** 

## (5) Kafka 

The Kafka consensus mechanism is based on a reliable message queue and is commonly used in enterprise blockchain architectures, such as the ordering service in Hyperledger Fabric. Kafka offers high throughput, making it ideal for private blockchain applications with demanding performance requirements. Kafka-based systems rely on server-based message queues with minimal computational complexity, resulting in very low energy consumption, often far below 0.0001 TWh annually, particularly in controlled enterprise environments. However, it depends on centralized ordering nodes, introducing a single point of failure risk, which makes it unsuitable for decentralized blockchain networks [47]. 

## (6) Raft 

Raft employs a leader election mechanism for log replication, making it suitable for small-scale distributed systems such as enterprise private blockchains. Its advantages include simple implementation and easy deployment while maintaining a high level of 

_Algorithms_ **2025** , _18_ , 263 

12 of 67 

consistency. Raft-based consensus typically operates within private server clusters with modest computational loads, and its overall energy footprint is minimal, making it a low-energy consensus protocol. However, Raft’s applicability is limited to small-scale environments, and its performance is influenced by the leader node. If the leader fails, consensus delays may occur. 

As Algorithm 3 shows, the Raft consensus algorithm is designed to achieve strong consistency across a distributed system in an intuitive and structured manner. It operates through several coordinated phases. First, leader election is initiated when a follower node does not receive a heartbeat signal within a timeout period, transitioning to a candidate and requesting votes from other nodes. A node becomes the leader if it secures a majority vote (at least _⌈N_ /2 _⌉_ ). Once elected, the leader is responsible for accepting client requests, appending them to its log, and replicating these entries to follower nodes via AppendEntries Remote Procedure Calls. Followers validate and append these entries, responding with acknowledgments. Upon receiving acknowledgments from the majority, the leader commits the entries and informs followers of the commitment. Raft ensures safety through leader exclusivity and log consistency and guarantees system termination under stable network conditions. 

## (7) Paxos 

Paxos achieves consensus through multiple rounds of message passing, emphasizing data consistency. It is widely used in strong consistency databases such as Google Chubby and AWS DynamoDB. 

Paxos excels in high-fault-tolerance environments, ensuring stable consensus and maintaining data integrity. Despite its theoretical soundness, Paxos is considered energyefficient in practice due to its use in tightly controlled data center environments. However, its high communication overhead limits its scalability in blockchain applications, and realworld energy consumption remains low compared to PoW, generally under 0.0001 TWh annually. However, the algorithm is computationally complex and requires substantial communication overhead, which constrains system performance. The Paxos consensus process is shown in Figure 3 below. Compared to other consensus mechanisms, Paxos is less suitable for blockchain applications in terms of throughput and efficiency [48]. 

**Figure 3.** Paxos consensus flow diagram. 

_Algorithms_ **2025** , _18_ , 263 

13 of 67 

**Algorithm 3** Raft Consensus Process 

- 1: **System Parameters:** 

2: _Nodes_ = _{n_ 1, _n_ 2, ..., _nN}_ 

_▷_ Set of all nodes 

3: Each node is in one of three states: **Follower** , **Candidate** , or **Leader** 

## 4: **procedure** LEADER ELECTION 

5: **if** Follower does not receive heartbeat within timeout **then** 6: Convert to Candidate and increment term number. 7: Vote for self and request votes from other nodes. 8: **end if** 9: **if** Candidate receives majority votes _≥ N_ 2 **then** � � �� 10: Become new Leader. 11: **end if** 12: **end procedure** 

## 13: **procedure** LOG REPLICATION 

14: **if** Leader **then** 15: Accept client requests. 16: Append entries to own log. 17: Send **AppendEntries** RPCs to followers. 18: **end if** 19: **for** each Follower **do** 20: **if** received **AppendEntries** RPC from Leader **then** 21: Verify and append new entries to log. 22: Send acknowledgment to Leader. 23: **end if** 24: **end for** 25: **end procedure** 

## 26: **procedure** COMMIT ENTRIES 

27: **if** Leader receives acknowledgments from majority nodes **then** 28: Mark entries as committed. 29: Inform followers of the commit. 30: **end if** 31: **end procedure** 

## 32: **procedure** SAFETY AND TERMINATION 

- 33: Only entries from a Leader that has majority support are committed. 34: Election safety and log matching properties are guaranteed. 

35: **end procedure** 

## 2.1.3. Smart Contract 

Smart contracts are essentially programs stored on a blockchain that automatically execute specific actions when predefined conditions are met. Typically, smart contracts are used to automate the execution of agreements, allowing all participants to quickly verify transaction outcomes without the need for intermediaries. This enhances efficiency and eliminates unnecessary delays. Additionally, smart contracts can streamline business processes by automatically triggering subsequent actions once certain conditions are fulfilled [49]. 

Smart contracts operate on an “if/when...then...” logic, with conditions encoded on the blockchain. Once triggered and verified, the network automatically executes actions such as fund transfers or asset registration, with outcomes immutably recorded and accessible only to authorized parties [50]. Contract terms are defined by specifying transaction rules, encoding logic conditions, handling exceptions, and establishing dispute resolution mechanisms to ensure reliability and protect participants’ interests [51]. 

_Algorithms_ **2025** , _18_ , 263 

14 of 67 

The smart contract execution process involves a series of automated steps designed to leverage the inherent technical features of blockchain. As Algorithm 4 shows, initially, the contract is deployed onto the blockchain, with its code and initial state stored immutably in the distributed ledger. Upon receiving an input event, the contract autonomously checks if predefined conditions are satisfied. If so, the corresponding function is executed; otherwise, the input is ignored. During execution, the contract validates input data and blockchain state to ensure correctness. Successful validation results in the state update and event emission, while failure leads to transaction rejection. The immutability of the deployed contract guarantees that neither its code nor its rules can be altered postdeployment, ensuring transparency and trustworthiness. Additionally, contract execution and enforcement are decentralized, with all blockchain nodes independently verifying and applying the contract logic, thus eliminating single points of failure and promoting a trustless operational environment. 

## **Algorithm 4** Smart Contract Execution Process 

## 1: **Initialization:** 

2: Deploy contract _C_ onto the blockchain. 

3: Store contract code and initial state in distributed ledger. 

4: **procedure** TRIGGER EVENT( _input_  event_ ) 

5: **if** predefined condition _Cond_ is satisfied **then** 6: Execute corresponding contract function _f_ ( _Cond_ ) 7: **else** 8: Ignore input; no execution occurs. 9: **end if** 10: **end procedure** 

11: **procedure** CONTRACT EXECUTION( _f_ ( _Cond_ )) 

12: Validate input data and current blockchain state. 13: **if** Validation succeeds **then** 14: Update the state variables according to _f_ ’s logic. 15: Emit events or update outputs transparently. 16: **else** 17: Reject transaction. 18: **end if** 19: **end procedure** 

20: **procedure** IMMUTABILITY AND VERIFICATION 21: Once deployed, _C_ cannot be modified. 22: Every transaction involving _C_ is cryptographically verifiable. 23: **end procedure** 

## 24: **procedure** DECENTRALIZED ENFORCEMENT 

25: All blockchain nodes independently validate and enforce contract execution. 26: No single point of failure or centralized control exists. 27: **end procedure** 

As shown in Table 6, a smart contract is an automated protocol based on blockchain technology that executes predefined operations immediately when specified conditions are met, without the need for human intervention. As shown in Figure 4 below, its core advantages are primarily reflected in the following four aspects: (1) Execution Efficiency and Accuracy: Smart contracts leverage digitalization and automation to significantly reduce human involvement, minimize errors, prevent transaction delays, and enhance overall operational efficiency. (2) Trust and Transparency: The execution records of smart 

_Algorithms_ **2025** , _18_ , 263 

15 of 67 

contracts are stored on the blockchain, allowing all participants to access the same data. This ensures transparency, reduces the risk of manipulation or data tampering, and enhances system credibility. (3) Security: Smart contracts rely on blockchain encryption algorithms and a chain-based structure, making data extremely difficult to alter or forge. This ensures the integrity and reliability of transactions. (4) Cost Reduction: By minimizing dependence on traditional intermediaries, smart contracts streamline transaction processes, lower management and operational costs, and optimize contract execution efficiency, ultimately improving economic benefits. 

**Table 6.** Advantages of smart contracts. 

|**Advantage**|**Description**|**Applicable Scenarios**|**Potential Challenges**|**Typical Application**|
|---|---|---|---|---|
||Automatically executes||||
|Speed, Efficiency, and<br>Accuracy|without human<br>intervention, reducing<br>errors and improving|Supply chain, payment<br>settlement|High computational<br>costs, slow execution<br>for complex contracts|Supply chain<br>settlement, insurance<br>claims|
||transaction efficiency.||||
|Trust and Transparency|Public and transparent<br>data reduces<br>information asymmetry<br>and enhances trust.|DeFi, government<br>record-keeping|Potential privacy<br>leakage, requires<br>privacy protection<br>technologies|Transparent charity,<br>decentralized<br>exchanges|
||Encryption and||||
|Security|consensus mechanisms<br>ensure transaction<br>security and|Identity verification,<br>medical data storage|Code vulnerabilities<br>may be exploited by<br>hackers|Digital identity,<br>electronic health<br>records|
||immutability.||||
||Reduces reliance on||||
|Cost Savings|intermediaries,<br>lowering transaction|Finance, real estate|High development and<br>computational costs|Securities clearing, P2P<br>lending|
||and operational costs.||||
||Automatically executes||||
|Automation and<br>Autonomy|when predefined<br>conditions are met,<br>minimizing human|Logistics, enterprise<br>automation|Immutable code<br>requires rigorous<br>testing|Supply chain payments,<br>smart insurance|
||intervention.||||



**Figure 4.** Blockchain smart contract schematic. 

_Algorithms_ **2025** , _18_ , 263 

16 of 67 

## 2.1.4. Cryptography Technology 

Blockchain can essentially be understood as a distributed ledger or a decentralized database [52]. As its unique technological advantages continue to drive research and adoption across various industries, blockchain primarily relies on cryptographic techniques to manage and secure vast amounts of data [53]. 

Cryptography employs advanced mathematical principles and algorithms to ensure the secure storage and transmission of data [54], allowing only authorized recipients to access and process the information [55]. Among its core components, encryption plays a crucial role by encoding data in a manner that renders it unintelligible to unauthorized parties, thereby protecting it from potential attacks and unauthorized modifications. 

In blockchain systems, encryption techniques are generally classified into two main categories: symmetric encryption and asymmetric encryption. Symmetric encryption utilizes a single key for both encryption and decryption, whereas asymmetric encryption employs a pair of keys—a public key for encryption and a private key for decryption—enhancing security and data integrity during transmission. These cryptographic techniques serve as the foundation of blockchain security, safeguarding data privacy and ensuring the reliability of transactions [56]. The comparison table between symmetric and asymmetric encryption is shown in Table 7. 

**Table 7.** Comparison of symmetric and asymmetric encryption. 

|**Feature**|**Symmetric Encryption**|**Asymmetric Encryption**|
|---|---|---|
|**Key Usage**|Same key for encryption and decryption|Public key for encryption, private key for decryption|
|**Security**|Vulnerable to key interception|More secure due to separate keys, reducing risk of<br>compromise|
|**Speed**|Encryption and decryption are effcient|Slower due to complex mathematical operations|
|**Key Distribution**|Requires secure key exchange|Only the public key is shared, ensuring security|
|**Scalability**|Diffcult to manage as user numbers increase|More scalable as no pre-shared keys are needed|
|**Usage Scenarios**|Bulk data encryption and secure storage|Digital signatures, key exchange, secure communication|
|**Common Algorithms**|DES, 3DES, AES, RC4, RC5, Blowfsh|RSA, ECC, DSA, ElGamal|



## (1) Symmetric encryption 

Symmetric encryption is one of the earliest encryption techniques, where both the sender and the receiver use the same set of rules for encrypting and decrypting data. In simple terms, the encryption key and the decryption key are identical [57]. A wellknown example of symmetric encryption is the Caesar cipher, which applies a basic substitution technique to encode messages. Some widely used symmetric encryption algorithms include DES, 3DES, Blowfish, IDEA, RC4, RC5, RC6, and AES. These algorithms vary in terms of security strength, performance, and application scenarios, with AES (Advanced Encryption Standard) being one of the most commonly adopted due to its strong security and efficiency [58]. Compared to public-key (asymmetric) encryption, symmetric encryption generally offers faster processing speeds, making it more suitable for scenarios requiring high-speed data encryption. However, symmetric encryption also presents certain challenges. Since the same key is used for both encryption and decryption, the security of symmetric encryption depends not only on the strength of the encryption algorithm but also on the secure management of the key itself. When using symmetric encryption for secure data transmission, the encryption key must be shared with the recipient, which introduces the issue of key transmission security [59]. In realworld applications, such secure transmissions typically take place over modern wireless communication infrastructures, such as Wi-Fi or 5G networks [60]. These environments provide high-speed connectivity, which complements the low-latency nature of symmetric encryption. Nevertheless, they also introduce additional risks such as signal interception, 

_Algorithms_ **2025** , _18_ , 263 

17 of 67 

unauthorized access points, or man-in-the-middle attacks, all of which further underscore the importance of robust encryption and secure key exchange mechanisms. 

(2) Asymmetric encryption 

Asymmetric encryption is a cryptographic technique integrated into blockchain to meet security and ownership verification requirements. The key distinction between asymmetric and symmetric encryption lies in the use of keys: symmetric encryption relies on a shared key for both encryption and decryption, whereas asymmetric encryption uses a “key pair” consisting of a public key and a private key for these operations [61]. 

The advantage of asymmetric encryption is that it effectively addresses security challenges related to key transmission. The public key can be openly shared, while the private key remains confidential, ensuring the security of the encryption process. However, while asymmetric encryption offers higher security, it does not directly verify the identity of the sender, which introduces the potential risk of sender impersonation. 

Common algorithms in asymmetric encryption include RSA and ECC (Elliptic Curve Cryptography). Asymmetric encryption is typically classified into three main types: the Integer Factorization Problem, where encryption relies on the product of two large prime numbers, the Discrete Logarithm Problem, which exploits the computational difficulty of solving discrete logarithms and uses strong one-way hash functions for asymmetric distributed encryption, and Elliptic Curve Cryptography, which uses elliptic curves on a plane to generate specific asymmetric encryption values [62]. 

As Algorithm 5 shows, asymmetric encryption uses a pair of keys: a public key for encryption and a private key for decryption. It begins by generating two large primes _p_ and _q_ , computing _n_ = _p × q_ and Euler’s totient _φ_ ( _n_ ) = ( _p −_ 1)( _q −_ 1). A public exponent _e_ is selected such that gcd( _e_ , _φ_ ( _n_ )) = 1, and the private exponent _d_ satisfies _e × d ≡_ 1 (mod _φ_ ( _n_ )). Encryption transforms plaintext into ciphertext via _c_ = _m[e]_ mod _n_ , while decryption recovers the original message with _m_ = _c[d]_ mod _n_ . The security relies on the computational difficulty of factoring _n_ . 

**Algorithm 5** Asymmetric Encryption Process 

1: **Initialization:** 

2: Select two large prime numbers _p_ and _q_ . 

3: Compute modulus _n_ = _p × q_ and Euler’s totient _φ_ ( _n_ ) = ( _p −_ 1)( _q −_ 1). 

4: **procedure** KEY GENERATION 5: Choose public exponent _e_ such that 1 _< e < φ_ ( _n_ ) and gcd( _e_ , _φ_ ( _n_ )) = 1. 6: Compute private exponent _d_ such that _e × d ≡_ 1 mod _φ_ ( _n_ ). 7: Output public key ( _e_ , _n_ ) and private key ( _d_ , _n_ ). 8: **end procedure** 9: **procedure** ENCRYPTION( _plaintext_ ) 10: Convert _plaintext_ to an integer _m_ . 11: Compute ciphertext _c_ = _m[e]_ mod _n_ . 12: Output _c_ . 13: **end procedure** 

14: **procedure** DECRYPTION( _ciphertext_ ) 15: Compute message _m_ = _c[d]_ mod _n_ using private key. 16: Convert integer _m_ back to _plaintext_ . 17: Output _plaintext_ . 18: **end procedure** 

19: **procedure** SECURITY ASSURANCE 20: The security relies on the computational difficulty of factoring _n_ into _p_ and _q_ . 21: Only the private key holder can efficiently decrypt the ciphertext. 22: **end procedure** 

_Algorithms_ **2025** , _18_ , 263 

18 of 67 

## _2.2. Artificial Intelligence_ 

Artificial Intelligence [63] encompasses multiple core technological domains, including machine learning (ML) [64] , deep learning (DL) [65] , natural language processing (NLP) [66] , and computer vision (CV). Within machine learning, supervised learning is primarily used for classification and prediction, unsupervised learning facilitates clustering and anomaly detection, and semi-supervised learning enhances model performance by leveraging a small amount of labeled data. Reinforcement learning (RL), on the other hand, is widely applied in game AI and autonomous driving [67]. 

In the early days of its birth, AI was mainly used for basic data analysis and pattern recognition, such as natural language processing, speech recognition, face recognition, logistics analysis, and intelligent investment. AI at this stage is mainly used to improve data processing efficiency, business decision-making, and interactive experience. With the advancement of AI technology, AI begins to penetrate industry scenarios to support more complex automation tasks, such as intelligent energy saving [68], agricultural product inspection, intelligent robots [69], intelligent water tanks [70], etc. AI at this stage gradually moves from data processing to intelligent services and automated systems, improving the operational efficiency of various industries. 

Nowadays, AI has entered the era of high intelligence, with stronger autonomous learning and decision-making capabilities, such as GPT-4o [71], unmanned driving, genetic testing, deep search (Deepseek) [72], adaptive learning [73], and so on. In the future, AI will be widely used in the fields of healthcare, transportation, education, etc., to achieve a highly intelligent society, and the development trend of AI-driven applications is shown in Figure 5 below. 

**Figure 5.** AI-driven application trends schematic. 

Deep learning a significant subfield of machine learning, leverages Convolutional Neural Networks (CNNs) for image processing, Recurrent Neural Networks (RNNs) for sequential data analysis, and Generative Adversarial Networks (GANs) for data generation. In natural language processing, advancements in text analysis, machine translation, and speech recognition drive the development of intelligent interactions [74]. Similarly, computer vision powers applications such as object detection and image segmentation, these technologies define the mainstream directions of modern AI, driving the widespread 

_Algorithms_ **2025** , _18_ , 263 

19 of 67 

development and application of intelligent systems. Below, we primarily introduce machine learning, deep learning, as these AI technologies are widely applied in blockchain to address its various challenges. 

## 2.2.1. Machine Learning 

Machine learning [75] is a discipline that explores how computers can simulate human learning to acquire knowledge, enhance skills, and optimize system performance [76]. Based on different learning paradigms, ML can be categorized into four main types: supervised learning, which relies on labeled data to train models for predicting unknown data; unsupervised learning, which processes unlabeled data for pattern recognition and data clustering; semi-supervised learning, which leverages a small amount of labeled data alongside a large volume of unlabeled data to reduce annotation costs and improve model performance [77]; and reinforcement learning, which enables an agent to interact with its environment through a reward mechanism to learn optimal decision-making strategies [78]. 

As shown in Table 8, supervised learning relies on labeled data for classification and regression tasks, ensuring high accuracy but requiring extensive data annotation. Unsupervised learning uncovers hidden patterns in unlabeled data, commonly used for clustering and dimensionality reduction, though its results may lack interpretability. Semi-supervised learning combines labeled and unlabeled data to improve performance while reducing labeling costs [79]. Reinforcement learning, driven by interaction with the environment, excels in sequential decision-making but demands high computational resources and extensive training. These paradigms are applied across various domains, from image recognition and anomaly detection to autonomous systems and robotics. We will briefly introduce several commonly used machine learning models and algorithms in blockchain applications. 

## (1) Decision Tree Model 

The decision tree model [80] is an intuitive and efficient data mining tool widely used in classification and regression problems. This method constructs a tree-like structure that decomposes complex decision processes into a series of simple judgments, thereby enabling effective data partitioning and prediction. 

The basic components of the model include the root node, internal decision nodes, and terminal nodes (leaf nodes), which correspond to the overall dataset, splitting conditions, and final decision outcomes, respectively [81]. 

This Figure 6a shows the boosting algorithm of ensemble learning. It starts with training the first weak learner with the original dataset D1. Then, the sample weights are iteratively updated based on the prediction results of each weak learner to increase the weight of the misclassified samples. The new dataset Dn is used to train new weak learners. Finally, the results of multiple weak learners are integrated into a strong learner to give the final prediction. The model performance is improved by focusing on difficult-to-classify samples and iterative integration. 

During the construction process, commonly used evaluation metrics include information entropy, information gain, and the Gini coefficient, which measure the change in data purity before and after partitioning. For example, information entropy can be expressed as follows: 

**==> picture [262 x 26] intentionally omitted <==**

_Algorithms_ **2025** , _18_ , 263 

20 of 67 

where _S_ represents the current dataset, and _pi_ represents the proportion of samples belonging to class _i_ is denoted. Information gain reflects the reduction in uncertainty brought about by a particular feature in the dataset partitioning, and its formula is given by the following: 

**==> picture [320 x 28] intentionally omitted <==**

Here, _A_ is the candidate feature, _Sv_ is the feature, and the subset corresponding to the value v of feature _A_ is denoted. 

**Figure 6.** ( **a** ) Schematic diagram of the decision tree boosting model. ( **b** ) SVM core concept diagram. ( **c** ) LSTM unit structure diagram. ( **d** ) MLP feedforward neural network structure diagram. ( **e** ) Schematic diagram of XGBoost model for incremental construction of decision tree. ( **f** ) CNN network architecture diagram. 

_Algorithms_ **2025** , _18_ , 263 

21 of 67 

**Table 8.** Comparative analysis of the four major machine learning paradigms. 

|**Comparison Criteria**|**Supervised Learning**|**Unsupervised**<br>**Learning**|**Semi-Supervised**<br>**Learning**|**Reinforcement**<br>**Learning**|
|---|---|---|---|---|
||||Combines a small|Involves an agent|
||Utilizes labeled data to|Employs unlabeled data|amount of labeled data|interacting with the|
|**Defnition**|train models for<br>predicting outputs of|to discover inherent<br>structures or patterns|with a large volume of<br>unlabeled data to|environment to learn<br>strategies that|
||new input data.|within the data.|enhance learning|maximize cumulative|
||||performance.|rewards.|
|**Data Requirements**|Requires a substantial<br>amount of high-quality<br>labeled data.|Needs a large quantity<br>of unlabeled data.|Uses a combination of a<br>small set of labeled data<br>and a large set of<br>unlabeled data.|Gathers data through<br>interactions with the<br>environment,<br>eliminating the need for<br>pre-labeled data.|
|**Primary Tasks**|Classifcation,<br>regression.|Clustering,<br>dimensionality<br>reduction, association<br>rule learning.|Classifcation,<br>regression,<br>semi-supervised<br>clustering.|Sequential<br>decision-making, policy<br>optimization.|
|**Common Algorithms**|Linear Regression,<br>Logistic Regression,<br>Support Vector Machine<br>(SVM), Decision Tree,<br>Random Forest, Neural<br>Networks.|K-Means Clustering,<br>Hierarchical Clustering,<br>Principal Component<br>Analysis (PCA),<br>Autoencoders.|Self-training,<br>Autoencoders,<br>Graph-based<br>Semi-Supervised<br>Learning.|Q-Learning, Deep Q<br>Network (DQN), Policy<br>Gradient Methods,<br>Actor-Critic Methods.|
|**Advantages**|High predictive<br>accuracy with<br>interpretable results.|Capable of uncovering<br>hidden patterns in data<br>without the need for<br>labeled data.|Enhances model<br>performance by<br>leveraging unlabeled<br>data, reducing labeling<br>costs.|Effective in learning<br>optimal strategies<br>within complex and<br>uncertain<br>environments.|
|||||Learning processes may|
|**Disadvantages**|Dependence on large<br>amounts of labeled<br>data, leading to high<br>labeling costs.|Results may lack clear<br>interpretability and are<br>challenging to evaluate.|Increased model<br>complexity and<br>potential instability<br>during training.|require extensive<br>interactions, resulting<br>in long training times<br>and high computational|
|||||resource demands.|
|**Application Areas**|Image recognition,<br>speech recognition,<br>spam detection, stock<br>price prediction.|Customer segmentation,<br>anomaly detection,<br>gene data analysis,<br>market basket analysis.|Medical image analysis,<br>text classifcation, web<br>content categorization.|Game AI, robotic<br>control, autonomous<br>driving, resource<br>management.|



## (2) Support Vector Machine Model 

The Support Vector Machine (SVM) [82] is a powerful algorithm widely used in machine learning for both classification and regression tasks. As a supervised learning method, its primary objective is to identify an optimal hyperplane in the feature space that effectively separates different classes of data points while maximizing the margin between them. The fundamental principle of SVM is to determine the hyperplane that best divides the dataset, which, in a d-dimensional feature space, corresponds to a (d-1)-dimensional subspace. 

Figure 6b shows the core concept of SVM. In the feature space, SVM looks for an optimal hyperplane (Maximum Margin Hyperplane) to separate samples of different categories. Sample points on the boundary are called support vectors. The distance between the positive hyperplane (Positive Hyperplane) and the negative hyperplane (Negative Hyperplane) is maximized to improve the generalization ability of the model. 

To achieve this, SVM leverages support vectors—data points that are closest to the hyperplane—which play a crucial role in defining its position and orientation [83]. One 

_Algorithms_ **2025** , _18_ , 263 

22 of 67 

of the key advantages of SVM is its ability to handle high-dimensional datasets while maintaining strong generalization capabilities. Furthermore, SVM is applicable to both linear and nonlinear classification problems, with the latter being addressed through the use of various kernel functions that map input features into a higher-dimensional space, allowing for more complex decision boundaries. 

## (3) Random Forest Model 

The Random Forest model [84] is an ensemble learning method. Its basic idea is to construct a large number of randomly generated decision trees and combine the predictions from each tree to improve the overall model’s stability and generalization ability. During the construction process, the model reduces the risk of overfitting commonly associated with individual decision trees by performing Bootstrap sampling on the original data and randomly selecting a subset of features at each node. This approach effectively captures the underlying complex relationships within the data. To illustrate the prediction mechanism of Random Forest, the following formula is used. For regression problems, the final prediction result of the Random Forest is the average of the outputs from all the decision trees, and its mathematical expression is as follows: 

**==> picture [232 x 27] intentionally omitted <==**

where _T_ represents the total number of decision trees, and _ht_ ( _x_ ) is the prediction output of the _t_ -th tree for the input _x_ . This formula reflects the basic idea of reducing prediction variance through mean aggregation. 

## (4) XGBoost Model 

XGBoost [85] is an efficient and scalable gradient boosting framework. Its core idea is to build decision trees incrementally using an additive model, minimizing prediction errors by optimizing the objective function while also constraining model complexity to improve generalization ability and stability [86]. The XGBoost model incremental construction of the decision tree is shown in Figure 6e. In each iteration, XGBoost uses a second-order Taylor expansion to approximate the loss function, thereby capturing the variation in the objective function more accurately and accelerating the convergence rate. The objective function of XGBoost combines training error and a regularization term, and its expression is given by 

**==> picture [269 x 27] intentionally omitted <==**

**==> picture [250 x 22] intentionally omitted <==**

where _l_ ( **y** _i_ , ˆ **y**[(] _[t]_[)] ) represents the loss value for the _i_ -th sample, and **y** ˆ[(] _[t]_[)] is the prediction result of the model after the _t_ -th iteration. The regularization term Ω( _fk_ ) is used to penalize model complexity, and _γ_ and _λ_ are the tuning parameters for the number of leaf nodes and the weights of the leaf nodes, respectively. 

In each iteration, the model updates the overall output by adding the prediction contribution of the new tree, and its mathematical expression is 

**==> picture [244 x 13] intentionally omitted <==**

where _ft_ ( _xi_ ) represents the prediction contribution of the t-th tree for the sample _xi_ . This formula reflects the detailed process of how XGBoost approximates the model by progressively accumulating the outputs of decision trees [87]. 

_Algorithms_ **2025** , _18_ , 263 

23 of 67 

## 2.2.2. Deep Learning 

Machine learning methods can be broadly categorized into two major types: supervised learning and unsupervised learning. Supervised learning primarily encompasses classification and regression tasks, while unsupervised learning focuses on problems such as clustering and association analysis. Deep learning falls under the domain of supervised learning and is based on neural network technology, which traces its origins back to the 1950s and 1960s, when it was known as the perceptron. The perceptron consisted of an input layer, an output layer, and a single hidden layer. The input feature vectors were transformed through the hidden layer to produce classification results at the output layer [88]. 

Deep learning involves learning the intrinsic patterns and hierarchical representations of sample data. The knowledge acquired during this learning process significantly enhances the interpretation of data such as text, images, and audio. The ultimate goal of deep learning is to enable machines to develop human-like analytical and learning capabilities, allowing them to recognize and process textual, visual, and auditory information effectively [89]. 

As a sophisticated machine learning algorithm, deep learning has achieved remarkable breakthroughs, particularly in speech and image recognition, far surpassing previous technologies in these domains. It has also made significant advancements in various fields, including search technology, data mining, machine learning, machine translation, natural language processing, multimedia learning [90], speech processing, recommendation systems, and personalized technology. By enabling machines to emulate human sensory perception and cognitive processes, deep learning has addressed numerous complex pattern recognition challenges, thereby driving substantial progress in artificial-intelligence-related technologies [91]. 

The fundamental models in deep learning can be broadly classified into three categories: multilayer perceptron models, deep neural network models, and recurrent neural network models. Representative architectures for these categories include Deep Belief Networks for MLP models, Convolutional Neural Networks for DNN models, and Recurrent Neural Networks for sequential data processing [92]. 

Table 9 compares several mainstream deep learning algorithms in recent years, including CNNs, RNNs, LSTM, GRU, Transformer, GANs, and Variational Autoencoders. CNNs are primarily used for computer vision tasks, leveraging convolutional operations to extract features while improving computational efficiency through parameter sharing. RNNs and their variants, LSTM and GRU, are designed for sequential data processing, with LSTM mitigating the vanishing gradient problem via gating mechanisms, whereas GRU achieves similar performance with reduced computational complexity [93]. Transformers employ self-attention mechanisms to enhance long-range dependency modeling, making them highly effective in natural language processing tasks. GANs utilize adversarial training to generate high-quality data but are prone to mode collapse during training. VAEs, based on probabilistic modeling, produce coherent data distributions with stable training, though the generated samples may lack fine details compared to GANs [94]. 

_Algorithms_ **2025** , _18_ , 263 

24 of 67 

**Table 9.** Comparison of deep learning algorithms. 

|**Algorithm**|**Core Principle**|**Main Applications**|**Advantages**|**Disadvantages**|
|---|---|---|---|---|
||Extracts spatial||||
|CNN|features through<br>convolutional layers,<br>reduces dimensionality<br>with pooling layers,<br>and classifes using|Computer vision<br>(image classifcation,<br>object detection,<br>semantic segmentation)|Spatial invariance,<br>parameter sharing<br>reduces computation|Requires large datasets,<br>struggles with<br>sequential data|
||fullyconnected layers||||
|RNN|Maintains sequential<br>dependencies through<br>recurrent connections|Natural language<br>processing, time-series<br>forecasting|Suitable for sequential<br>data, capable of<br>learning long-term<br>dependencies|Training diffculty<br>(vanishing/exploding<br>gradient), ineffcient<br>parallelization|
|LSTM|Introduces gating<br>mechanisms (forget,<br>input, and output<br>gates) to improve<br>RNNs|Machine translation,<br>speech recognition|Solves vanishing<br>gradient issue, retains<br>long-term<br>dependencies|High computational<br>complexity, slower<br>training compared to<br>RNNs|
|GRU|A simplifed version of<br>LSTM with merged<br>forget and inputgates|Speech recognition,<br>text generation|Lower computational<br>cost than LSTM, similar<br>performance|Slightly less expressive<br>power than LSTM|
|Transformer|Utilizes self-attention<br>mechanisms and<br>feedforward networks,<br>independent of<br>sequential order|Machine translation,<br>conversational AI,<br>large models (e.g., GPT,<br>BERT)|Highly parallelizable,<br>strong long-range<br>dependency learning|High computational<br>complexity,<br>resource-intensive for<br>long sequences|
||Composed of a||||
|GAN|generator and a<br>discriminator in an<br>adversarial setup,<br>learning to generate|Image generation, data<br>augmentation,<br>super-resolution|Strong generative<br>capability, useful for<br>data augmentation|Unstable training,<br>prone to mode collapse|
||realistic data||||
|VAE|Uses probabilistic<br>modeling to generate<br>data, with a latent<br>variable controlling the<br>generator|Image generation,<br>anomaly detection|Smooth latent space,<br>stable training|Generated images may<br>be blurry, lacks fne<br>details compared to<br>GANs|



## (1) MLP Model 

In recent years, the advancement of deep learning has brought renewed attention to the Multilayer Perceptron (MLP), making it a prominent research focus. MLP has been widely applied across various domains, including image processing, speech recognition, and natural language processing, demonstrating exceptional performance in tasks such as object detection, image classification, semantic segmentation, and machine translation [95]. 

MLP is a type of feedforward neural network capable of mapping input vectors to output vectors. Its architecture typically comprises multiple fully connected layers of neurons, where each neuron (except those in the input layer) employs a nonlinear activation function and is trained using the backpropagation algorithm [96]. During model training, network weights are first initialized. The input data then undergo forward propagation, where the weighted sum is computed in the hidden layers and transformed through an activation function to produce intermediate representations. The output layer subsequently generates prediction results, and a loss function—such as Mean Squared Error (MSE) or 

_Algorithms_ **2025** , _18_ , 263 

25 of 67 

Cross-Entropy—is computed based on the ground truth labels. The backpropagation algorithm is then employed to compute gradients and optimize the network parameters. The MLP model architecture is illustrated in Figure 6d. 

## (2) CNN Model 

CNN is a deep feedforward neural network characterized by local connectivity and weight sharing, specifically designed to process grid-structured data such as images and audio [97]. Its fundamental architecture consists of three primary layers: the convolutional layer, the pooling layer, and the fully connected layer. The CNN grid structure is shown in Figure 6f. 

1. Convolutional Layer: As the core component of CNNs, the convolutional layer applies convolutional filters (kernels) that slide over the input data (e.g., image pixel matrices) to perform convolution operations. Different kernels can extract distinct features, such as edges, textures, or higher-level semantic information [98]. Furthermore, convolution operations exhibit weight sharing, meaning that the same kernel is applied across different regions of the input data using identical parameters. This significantly reduces the number of trainable parameters and enhances computational efficiency [99]. 

2. Pooling Layer: The pooling layer is responsible for dimensionality reduction while preserving essential features, thereby reducing computational complexity and improving model robustness. The most common pooling techniques are max pooling and average pooling. Max pooling retains the maximum value in a local region, whereas average pooling computes the mean value. This operation not only enhances computational efficiency but also increases the model’s tolerance to minor spatial transformations [100]. 

3. Fully Connected Layer: The fully connected layer integrates the features extracted from previous layers and is primarily used for classification or regression tasks. In this layer, each neuron is connected to all neurons in the preceding layer, resembling hidden layers in traditional neural networks. Through weighted summation and activation functions, the fully connected layer produces the final classification output or task-specific predictions [101]. 

- (3) LSTM Model 

LSTM is a specialized type of Recurrent Neural Network (RNN) designed to process and predict time-series or sequential data with long-term dependencies [102]. It effectively mitigates the vanishing and exploding gradient problems present in traditional RNNs. The fundamental unit of an LSTM consists of three gating mechanisms—the forget gate, input gate, and output gate—along with a cell state [103]. These three gates regulate the flow of information using the sigmoid activation function and element-wise multiplication, ensuring that the strength of the information flow is controlled without introducing additional information. The LSTM unit structure diagram is illustrated in Figure 6c. 

In the figure, _Ct−_ 1 represents the cell state (memory state) at time _Xt_ is the current input, and _ht−_ 1 denotes the output of the neural unit at time _t −_ 1. The forget gate’s output at time _t_ is _ft_ , the input gate’s output is _it_ , and the output gate’s output is _ot_ . The function _σ_ represents the sigmoid activation, while _Ct_ is the cell state at time _t_ , and _ht_ is the neural unit’s output at time _t_ . 

To prevent excessive memory retention from interfering with the processing of new inputs, the forget gate determines how much information from the previous cell state should be preserved. It computes a weight based on _xt_ and _ht−_ 1, processed through a sigmoid activation function to produce a vector with values between 0 and 1—where 

_Algorithms_ **2025** , _18_ , 263 

26 of 67 

0 signifies forgetting and 1 indicates full retention. The forget gate is mathematically expressed as follows: 

**==> picture [258 x 19] intentionally omitted <==**

In Equation (7), _Wf_ represents the forget gate weight, _b f_ denotes the bias, and _σ_ is the sigmoid function. The input gate determines how much of the current input information should be retained in the state of the neural unit at the current time step, thereby controlling the information that needs to be updated [104]. The input _Xt_ and _ht−_ 1 output values pass through the input gate and are then combined with the input and output values processed by a tanh function to generate new control parameters. The mathematical expression for the input gate is as follows: 

**==> picture [265 x 53] intentionally omitted <==**

In Equation (10), the cell state is updated as _Ct_ , and _Wi_ represents the input gate weight. The output gate determines which information from the current cell state is transmitted to _ht_ . Both _Xt_ and _ht−_ 1 first pass through the output gate to define the scope of the output information. Then, in combination with the tanh function, a selected _Ct_ portion of the memory information is processed, and the final neural network output value _ht_ is determined. The mathematical expression for the output gate is as follows: 

**==> picture [257 x 11] intentionally omitted <==**

**==> picture [237 x 12] intentionally omitted <==**

In Equations (8) and (9), _Wo_ represents the output gate weight. 

## **3. AI-Driven Blockchain Solutions** 

In recent years, blockchain technology has gained widespread adoption across various sectors, including finance, supply chain management, the Internet of Things, healthcare, and government administration, owing to its decentralized, secure, transparent, and tamper-resistant characteristics [105]. For instance, in the financial sector, blockchain supports digital currencies, smart contracts, and decentralized finance; in supply chain management, it provides reliable product traceability; and in healthcare, it enhances the security of electronic health records, ensuring data privacy and secure sharing. A notable application of blockchain is in the energy and transportation sector. The Mobility Open Blockchain Initiative, in collaboration with Honda, General Motors, PGE, and others, introduced the first blockchain-based Electric Vehicle Grid Integration standard. This standard facilitates secure, decentralized communication and transaction recording between electric vehicles, charging infrastructure, and grid operators. By incorporating AI techniques, such as real-time optimization of charging schedules and predictive energy demand modeling, the EVGI framework enables use cases like vehicle-to-grid integration, tokenized carbon credits, and peer-to-peer energy exchange. These innovations promote a decentralized, low-carbon, and intelligent energy ecosystem, showcasing the synergy of AI and blockchain in addressing energy sustainability and climate challenges [106]. 

Despite its increasing adoption, blockchain technology is still evolving, with continuous improvements in four key areas: consensus mechanisms, smart contracts, privacy protection, and data retrieval. As a fundamental component of blockchain, consensus mechanisms ensure data consistency across distributed networks and enhance system resilience against attacks [107]. Smart contracts, serving as a trusted intermediary, enable 

_Algorithms_ **2025** , _18_ , 263 

27 of 67 

the automated execution of agreements within decentralized networks, reducing human intervention while enhancing transaction transparency and efficiency. Moreover, blockchain offers a high degree of anonymity, as all stored transactions are encrypted, effectively safeguarding user privacy. Additionally, blockchain-based query systems typically incorporate data and hash indexing, organizing information to facilitate efficient data retrieval. However, blockchain technology still faces several challenges. Consensus mechanisms may suffer from performance bottlenecks and centralization risks; smart contracts are prone to vulnerabilities and upgrade difficulties; privacy-preserving techniques often entail high computational complexity; and data retrieval still encounters issues related to efficiency, cost, and usability. These limitations necessitate further research and improvement, as illustrated in Table 10. 

**Table 10.** Comparison of blockchain core technologies. 

|**Aspect**|**Consensus**<br>**Mechanisms**|**Smart Contracts**|**Privacy Protection**|**Data Retrieval**|
|---|---|---|---|---|
|**Performance**|Low TPS, high<br>latency [108]|Execution<br>ineffciency [109],<br>highgas costs [110]|High computational<br>overhead (e.g., ZKP,<br>FHE) [111]|Slow query speed,<br>limited scalability|
|**Security**|51% attack risk, PoS<br>centralization|Vulnerabilities,<br>irreversible<br>deployment|TEE vulnerabilities,<br>MPC security<br>concerns|Data integrity risks,<br>potential<br>manipulation|
|**Decentralization**|Validator oligopoly<br>risk|Platform<br>dependence (e.g.,<br>Ethereum)|Requires trusted<br>third parties (e.g.,<br>MPC and FHE)|Often relies on<br>centralized indexing|
|**Scalability**|Limited node<br>expansion, slow<br>fnality|High on-chain<br>storage and<br>computation costs|ZKP and MPC<br>scalability issues|High storage costs,<br>ineffciency in large<br>datasets|
|**Privacy**|Transparent<br>transactions, weak<br>fnancialprivacy|Code and execution<br>transparency|High cost for<br>privacy-preserving<br>computations|Publicly accessible<br>data may expose<br>user behavior|
|**Energy Consumption**|High in PoW,<br>moderate in<br>PoS [112]|Gas fees fuctuate,<br>ineffcient execution|Cryptographic<br>computations are<br>energy-intensive|Storage and<br>verifcation require<br>signifcant resources|
||High confrmation|Diffcult to upgrade|Complex|Slower than|
|**Usability**|time, node|and fx|deployment, high|traditional|
||reliability required|vulnerabilities|technical threshold|databases|
||Diffcult to regulate,|Hard to modify|Potential restrictions|Cross-chain data|
|**Regulatory Compliance**|irreversible|contracts per legal|on privacy|access challenges,|
||transactions|requirements|technologies|regulatoryconcerns|
|**Cost**|High mining and<br>staking costs [113]|High gas fees,<br>expensive audits|Expensive<br>cryptographic<br>computations|High storage and<br>retrieval costs|



Dividing AI-driven blockchain research into four categories—consensus algorithms, smart contracts, privacy protection, and data retrieval—helps clearly present the core issues and challenges within each research direction, enhancing the structural coherence and readability of the paper. This classification facilitates targeted analysis, allowing for a comparison of the strengths and weaknesses of different approaches, while also highlighting the comprehensive impact of AI technologies across various blockchain domains [114]. Additionally, it promotes interdisciplinary integration and provides clear direction and insights for future research. The existing challenges of AI-powered blockchain are shown 

_Algorithms_ **2025** , _18_ , 263 

28 of 67 

in Figure 7 below. The diagram shows multiple applications of AI in blockchain, including smart contract vulnerability detection, consensus algorithm optimization, data privacy protection, and efficient data storage. The security, scalability, and privacy protection of blockchain are improved by techniques such as deep learning, decision trees, federated learning, and optimized Merkle trees [115]. 

**Figure 7.** Schematic of the existing challenges of AI-driven blockchain. 

## _3.1. AI-Driven Consensus Algorithm_ 

AI-driven consensus algorithms are emerging as a core driving force in the intelligent evolution of blockchain systems, aiming to enhance security, efficiency, and scalability through advanced machine learning techniques. These algorithms primarily focus on three key optimization areas: dynamic node selection, malicious behavior detection, and resource allocation prediction. In dynamic adaptive consensus mechanisms, reinforcement learning can adjust PoW or PoS parameters in real time—such as dynamically modifying block generation intervals or probabilities—to improve system throughput and responsiveness. A notable example is XG-PBFT, which utilizes graph neural networks to optimize communication paths between nodes, reducing the communication complexity of traditional PBFT from O(n²) to O(n). 

In terms of security, deep learning models such as LSTM are employed to analyze historical behavior of nodes and identify abnormal patterns, including selfish mining, sybil attacks, or 51% attacks. These detection mechanisms are often integrated with reputationbased systems (e.g., CE-PBFT) to effectively filter out malicious nodes and reduce the risk of consensus centralization. For resource optimization, federated learning (FL) and distributed edge intelligence frameworks enable efficient task allocation across edge nodes, thereby minimizing energy consumption and network latency while preserving data privacy. Systems like FGADL-DEVCA have demonstrated the ability to save up to 30% of energy costs in multi-node environments. 

Based on a comprehensive analysis of existing research, current optimizations in AIdriven blockchain consensus primarily focus on improvements in POW and POS consensus mechanisms. These AI-driven consensus algorithms can be categorized according to the main challenges they address, such as resource-constrained environments, security and privacy concerns, economic incentives, and resource optimization. Broadly, these can be 

_Algorithms_ **2025** , _18_ , 263 

29 of 67 

classified into four categories: AI-driven IoT/Edge computing consensus mechanisms, AI-powered security and privacy protection consensus, AI-driven economic incentives and resource optimization consensus, and AI-driven intelligent computing and deep learning consensus. 

## 3.1.1. AI-Powered Authentication and IoT Consensus Optimization Algorithms 

In the IoT environment, authentication is crucial due to the vast number of connected devices and the highly sensitive nature of the data they handle, such as those in medical devices, smart homes, and industrial control systems. Therefore, achieving efficient consensus in large-scale IoT networks while minimizing authentication consensus latency has become a pressing challenge that requires urgent resolution. These challenges are primarily characterized by resource constraints, high heterogeneity, frequent node mobility, and the need for ultra-low latency, which traditional consensus mechanisms often fail to address effectively. To address these challenges, Dutta et al. [116] introduced a significant advancement in the Proof of Authentication (PoAh) consensus algorithm, specifically designed for resource-constrained IoT devices. Building upon the original PoAh consensus, this enhanced iteration, referred to as PoAh 2.0, integrates AI at the block creator node level. This novel approach enables the generation of blockchain transactions embedded with AI-determined sensitivity and other relevant metadata, representing a pioneering concept in the field. Sun et al. [117] proposed an AI-assisted consensus algorithm tailored for vehicular networks, integrating vehicle-based metrics with neural networks. The approach incorporates key indicators such as vehicle online duration, performance, and behavior. A comprehensive model and a hierarchical classification method, combined with a BP neural network, are utilized to optimize interconnectivity. Additionally, the Informer model is employed to predict future vehicle online durations, effectively mitigating the issue of primary node vehicle dropout in collaborative IoV networks. Alruwaili et al. [118] also developed a deep-learning-based data edge verification and consensus method (FGADLDEVCA) for IoT-cloud platforms. The primary objective of FGADL-DEVCA is to integrate blockchain technology to enhance security in IoT-cloud environments while leveraging deep learning models for efficient fault detection. Furthermore, Sasikumar et al. [119] developed a novel blockchain-assisted data edge validation and machine learning consensus algorithm (BDEV-CAML) for the purpose of IoT fault detection. The proposed BDEV-CAML technique integrates the advantages of blockchain, IoT, and machine learning models to enhance the credibility, efficiency, and security of IoT networks. However, current research still faces several gaps. Firstly, there is a lack of unified AI-driven lightweight consensus frameworks that are adaptable to a wide range of IoT scenarios, particularly in ultra-dense or rapidly changing environments. Secondly, most existing works focus on static or semi-dynamic network topologies, which limits their applicability to mobile or highly dynamic IoT networks such as UAV swarms or autonomous vehicular systems. Moreover, the integration of privacy-preserving AI techniques with blockchain-based consensus remains underexplored, particularly in scenarios involving federated or split learning across distributed IoT devices. 

Potential solutions include the design of adaptive, modular AI consensus layers that can dynamically switch consensus modes based on real-time network metrics; the development of federated reinforcement learning models to allow collaborative learning without compromising data privacy; and the integration of privacy-preserving computation (e.g., homomorphic encryption and differential privacy) with consensus validation to address privacy and scalability trade-offs. Future research should also explore multiobjective optimization frameworks, balancing security, latency, energy consumption, and trustworthiness in heterogeneous IoT environments. 

_Algorithms_ **2025** , _18_ , 263 

30 of 67 

As shown in Table 11, this category of consensus mechanisms is primarily designed for IoT devices and edge computing environments, where computational resources are often constrained. These approaches enhance consensus efficiency by improving device authentication and optimizing online status management, thereby reducing latency and communication overhead. Moreover, they enable resource-limited devices to participate in the consensus process more reliably while ensuring data integrity. However, some solutions still require substantial computational power, making them less suitable for low-capacity devices. 

**Table 11.** Comparison of AI-powered authentication and IoT consensus optimization algorithms. 

|**Algorithm**|**Core Mechanism**|**Communication**<br>**Complexity**|**Key Advantages**|**Existing Limitations**|
|---|---|---|---|---|
||AI-powered||Suitable for IoT|Dependence on AI|
|PoAh 2.0 [116]|dynamic|_O_(_N_)|devices, enhanced|computational|
||authentication||security|capacity|
|IoV Consensus [117]|Vehicle prediction +<br>BP neural network|_O_(_N_log_N_)|Reduces leader node<br>dropouts|Limited accuracy in<br>vehicle time<br>prediction|
|FGADL-DEVCA [118]|Deep learning for<br>transaction<br>verifcation|_O_(_N_2)|Improves data<br>integrity|High computational<br>cost|
|BDEV-CAM [119]|Machine-learning-<br>enhanced blockchain<br>verifcation|_O_(_N_log_N_)|Enhance IoT security|High computational<br>demands|



## 3.1.2. AI-Driven Security and Privacy-Preserving Consensus 

Security and privacy protection are fundamental concerns in blockchain systems as they ensure the integrity, confidentiality, and trustworthiness of distributed data. In various applications, such as financial transactions, healthcare, and decentralized identity management, safeguarding sensitive information against cyber threats, unauthorized access, and data breaches is crucial. To address these challenges, AI-driven consensus mechanisms for security and privacy protection have emerged as a promising solution. By integrating AI with blockchain consensus, these mechanisms enhance anomaly detection, prevent malicious node behavior, and improve data protection through techniques such as differential privacy, federated learning, and deep-learning-based intrusion detection. 

Zhang et al. [120] proposed an architecture in which devices (also referred to as nodes) can reach consensus on task results using off-chain smart contracts and private data. The proposed distributed computing architecture accelerates compute-intensive and dataintensive supervised classification algorithms, even in resource-constrained environments. This architecture significantly enhances privacy protection, preventing the leakage of distributed data. Sasikumar et al. [121] aimed to design and implement a consensus mechanism that combines blockchain and AI to enable successful big data analytics. This work presents an improved IIoT network based on the DPoS algorithm, which integrates blockchain and AI for real-time data transmission. To accelerate block generation in the IIoT network, nodes use the improved DPoS to reach consensus, select representatives, and store block information in transaction nodes. Kumar et al. [122] proposed a blockchainsupported eXplainable AI (XAI) to enhance decision-making capabilities in network threat detection within intelligent healthcare systems. Specifically, the proposed method first implements the Clique Proof of Authority (C-PoA) consensus to verify and store data across multiple cloud providers using blockchain. Secondly, a novel deep-learning-based 

_Algorithms_ **2025** , _18_ , 263 

31 of 67 

threat tracking model is constructed by combining parallel stacked long short-term memory (PSLSTM) networks with multi-head attention mechanisms to improve attack detection. Extensive experiments confirm that it has the potential to be used by cybersecurity analysts as an enhanced decision support system. Kim et al. [123] investigated vulnerabilities in automotive safety committees and standards, using AI-based LSTM and blockchain consensus algorithms for anomaly detection in geo-tagged train carriage hacking attacks. Additionally, in the context of automotive safety, an algorithm was explored that uses LSTMbased anomaly detection techniques to predict normal and abnormal values in automotive communication networks, which are primarily divided into internal and external networks. 

As shown in Table 12, the above research has made multi-dimensional optimizations in terms of privacy protection, consensus efficiency, threat detection, and system interpretability by integrating blockchain and artificial intelligence technologies, such as off-chain smart contracts to improve computing performance in resource-constrained environments, improved DPoS to achieve efficient data transmission in IIoT networks, XAI to enhance medical network security decisions, and LSTM-based anomaly detection to optimize the security of Internet of Vehicles systems. However, these works still face many challenges, including the balance between privacy protection and computing efficiency, the difficulty of model deployment on resource-constrained devices, the centralization risk of representative node elections, the lack of interpretability of deep models, and the lack of cross-platform collaboration mechanisms. Existing research has not fully solved the problem of trusted consensus and task collaboration in heterogeneous IoT environments and lacks systematic performance evaluation and incentive mechanism design for the “on-chain + off-chain” architecture. In the future, the practicality and security of blockchain in intelligent IoT systems can be further improved through technical paths such as lightweight AI models and federated learning, verifiable off-chain computing, incentive-enhanced consensus mechanisms, multimodal XAI fusion, and multi-chain collaboration protocols. 

**Table 12.** AI-driven blockchain security consensus. 

|**Algorithm**|**Core Mechanism**|**Communication**<br>**Complexity**|**Key Advantages**|**Existing Limitations**|
|---|---|---|---|---|
|Nakamoto + AI [120]|Off-chain validation<br>+ AI data analysis|_O_(_N_log_N_)|Suitable for high<br>computational tasks|High resource<br>consumption|
|DPoS for IIoT [121]|AI-optimized DPoS<br>election|_O_(_k_log_N_)|Lower energy use,<br>improved effciency|Dependence on<br>delegates|
|C-PoA [122]|PoA with LSTM for<br>threat detection|_O_(_N_)|Improves blockchain<br>security|Limited by PoA<br>governance|
|LSTM Security<br>Consensus [123]|LSTM-based<br>malicious node<br>detection|_O_(_N_2)|Suitable for<br>automotive security|High computational<br>overhead|



## 3.1.3. AI-Driven Federated Learning and Privacy-Preserving Consensus 

While blockchain technology provides decentralization and transparency, privacy protection has become an increasingly critical issue. As blockchain applications expand, particularly in sectors like finance, healthcare, and identity management, ensuring the security of sensitive information and preventing unauthorized access and data breaches have become urgent challenges. In this context, AI offers new solutions to optimize blockchain privacy protection. AI-driven consensus mechanisms for security and privacy protection, using techniques such as anomaly detection, differential privacy, and data integrity protection, can significantly enhance the privacy capabilities of the system. By integrating AI with blockchain, AI can optimize data privacy protection in the consensus process, 

_Algorithms_ **2025** , _18_ , 263 

32 of 67 

for example, by using federated learning and deep learning techniques to strengthen data analysis and privacy computation. Additionally, nodes can deploy computational models internally, assigning incentive scores based on their computational capabilities to enhance node participation and activity. Moreover, performance-based metrics can be used to promptly identify and eliminate malicious nodes, thereby improving overall consensus efficiency and enhancing privacy protection. Cui et al. [124] proposed a novel blockchain-enabled federated learning (BCFL) scheme tailored for the IoV and incorporating differential privacy mechanisms. To meet the stringent performance requirements of IoV environments, the approach integrates a federation chain with PBFT consensus, which improves efficiency compared to traditional public blockchains. In addition, the method employs the Differential Privacy Stochastic Gradient Descent (DP-SGD) algorithm in the local training process of Federation Learning to enhance privacy preservation. Wang et al. [125] designed an innovative block structure, novel transaction types, and creditbased incentives. The proposed PF-PoFL scheme supports fully decentralized AI task outsourcing, joint mining, model evaluation and reward distribution while being resistant to fraud and Sybil attacks. In addition, PF-PoFL employs a user-level differential privacy mechanism to prevent implicit privacy leakage during federated learning model training. Deng et al. [126] developed an innovative economic model to manage monetary, trade, and supply and demand policies in AIBC. The study proposed an upper-layer DPoEV incentive consensus algorithm for economic policy implementation and introduced an underlying DABFT distributed consensus algorithm that adaptively enforces the DPoEV. In addition, the results of the study showed that the economic model can be efficiently implemented by the DPoEV-DABFT dual consensus architecture of AIBC. Alrubei et al. [127] proposed a decentralized and distributed architecture to implement Distributed Artificial Intelligence (DAI) using IoT hardware platform. The trained DAI system is deployed via IoT devices, where each IoT device acts as one or more neurons in the DAI layer. The approach relies on decentralized, self-managed blockchain technology to facilitate trusted interaction and information exchange between distributed neurons. The platform is built and customized for IoT systems to efficiently handle DAI-related tasks. In addition, the study designs and implements a novel consensus mechanism combining PoA and PoW. Zhi et al. [128] combined distributed deep learning with blockchain technology and proposed a blockchain consensus mechanism based on distributed deep learning proof-of-workload (BCDDL) to reduce the waste of computational resources in blockchain networks. Jogunola et al. [129] proposed a framework and explored perspectives related to achieving an effective integration of blockchain and artificial intelligence (BC-AI) in a carbon emissions trading system (ETS). Goh et al. [130] proposed a secure Trust Delegated Consensus Blockchain (TDCB-D3P) scheme based on the Priority Experience Replay (D3P) mechanism to address scalability and security issues simultaneously. The scheme utilizes Deep Reinforcement Learning techniques to optimize the blockchain performance. 

As shown in Table 13, the aforementioned consensus optimization schemes focus on improving computational resource utilization, optimizing resource scheduling, reducing energy consumption, and minimizing unnecessary computational overhead. According to References [124,127,129], AI-driven optimization reduced consensus latency by approximately 25%, decreased transaction processing costs by about 15%, and lowered energy consumption by around 20% compared to traditional blockchain systems. The unit of energy consumption reduction is measured in kilowatt-hours (kWh), and cost reductions are calculated based on transaction fees in USD. For instance, some approaches enhance task allocation in PoW consensus to make the computation process more efficient, or, based on the PBFT model, deploy computational models within nodes where the better the node’s computational performance, the more incentives it receives. This encourages 

_Algorithms_ **2025** , _18_ , 263 

33 of 67 

higher participation from nodes in the consensus network, thereby improving overall efficiency and reducing communication overhead. Although the above methods have achieved positive results, they still face many challenges, such as the trade-off between differential privacy and model performance, system stability under multi-consensus mechanisms, fairness of task incentive mechanisms, and on-chain compatibility of AI computing. Future research can further promote the integration and implementation of blockchain and AI in the field of privacy computing by introducing multi-level privacy protection strategies, establishing a unified and reliable AI incentive and evaluation system, developing lightweight models and asynchronous update mechanisms, and exploring cross-chain privacy collaboration protocols. 

**Table 13.** AI-driven economic incentives and resource optimization consensus. 

|**Algorithm**|**Core Mechanism**|**Communication**<br>**Complexity**|**Key Advantages**|**Existing Limitations**|
|---|---|---|---|---|
|BCFL [124]|PBFT + Federated<br>Learning|_O_(_N_2)|Enhanced data<br>privacy|High resource<br>consumption|
|PF-PoFL [125]|Credit-based<br>federated consensus|_O_(_N_log_N_)|Privacy-preserving,<br>secure|Slower training time|
|DABFT [126]|AI-based Byzantine<br>fault tolerance|_O_(_N_log_N_)|Improved fault<br>resilience|Data quality<br>dependent|
|PoA + PoW for<br>IoT [127]|AI-enhanced PoA<br>and PoW|_O_(_N_log_N_)|Optimized for IoT<br>devices|High PoW<br>computational cost|
|DPoEV [128]|Economic incentive<br>with AI evaluation|_O_(_k_log_N_)|Suitable for<br>blockchain economy|Data dependency|
|DRL + PoW [129]|DRL for PoW task<br>offoading|_O_(_N_2)|Reduces<br>computational<br>overhead|Requires stable<br>network|
|D3P [130]|Double DQN +<br>trusted delegate<br>consensus|_O_(_N_log_N_)|Improved effciency,<br>security|Long training<br>process|



## 3.1.4. AI-Driven Blockchain Consensus Others 

Some studies primarily focus on computationally intensive tasks, such as deep learning training and smart contract execution, leveraging AI to enhance consensus computation efficiency. Kim et al. [131] proposed a series of blockchain algorithms based on natural language processing. The primary function of the Multi-Layer ANN for Blockchain Consensus Algorithm is not merely to use given data as-is to form a classifier but rather to train transformations optimized for the given data by incorporating structures known as hidden layers while simultaneously forming a classifier. Depending on the number of neurons in the hidden layers, the model can also perform dimensionality reduction or expansion on the given data. A multi-layer ANN, consisting of two layers of neurons, can be meticulously designed for enhanced performance. This structure also represents the architecture of the Multi-Layer ANN blockchain consensus algorithm, where all layers—except for the output and input layers—are designed as hidden layers that operate in conjunction with the consensus algorithm during the protocol process. Chen et al. [132] proposed a novel framework based on Deep Reinforcement Learning (DRL) to offload the computationally intensive tasks of PoW to edge servers within a blockchain-based Mobile Crowd Sensing (MCS) system. The proposed framework enables the derivation of an optimal PoW task offloading strategy in complex and dynamic MCS environments. 

_Algorithms_ **2025** , _18_ , 263 

34 of 67 

In addition, some researchers have introduced machine learning methods into consensus node classification, which can be divided into unsupervised nodes according to their performance, and select good nodes to participate in the consensus to reduce the communication overhead and delay of malicious nodes, and the schematic diagram of AI algorithm-driven consensus classification is shown in Figure 8 below. This figure shows the application of AI in blockchain consensus mechanism, using AI grouping algorithm to intelligently group network nodes, generate candidate blocks, and select the final block to be written into the blockchain through the consensus group so as to optimize the consensus efficiency and improve the system performance. This type of research is also the future development direction of AI-driven blockchain consensus now. For example, Xiaowei et al. [133] proposed an improved PBFT algorithm based on XGBoost grouping called XG-PBFT. The method first trains the dataset by learning important parameters that affect the performance of nodes, which are used as node classification criteria. Subsequently, the dataset is trained using the XGBoost algorithm, and the nodes newly added to the system are grouped according to the trained model. In addition, XG-PBFT assigns reputation values to nodes based on their performance, and nodes with higher reputation values are more likely to become master nodes, thus enhancing the efficiency and security of the consensus process. Xiao et al. [134] proposed a novel consensus algorithm called Credit Evaluation-based Practical Byzantine Fault Tolerance (CE-PBFT). This algorithm introduces an improved node credit evaluation model that comprehensively considers node completion rate, consensus attenuation, and historical behavior to quantify the reliability status of nodes during system operation. Through this mechanism, CE-PBFT effectively enhances the overall reliability and security of the system. Additionally, this study innovatively applies the decision tree algorithm to analyze network node behavior, thereby optimizing the consensus decision-making process and simplifying existing consensus protocols. 

**Figure 8.** AI algorithms drive blockchain consensus node classification. 

_Algorithms_ **2025** , _18_ , 263 

35 of 67 

As shown in Table 14, these consensus mechanisms are primarily applied in specialized industries or domains, such as academic credential verification and distributed computing, to ensure data integrity and reliability [135]. For instance, blockchain-based diploma authentication systems can effectively prevent certificate forgery. While these solutions demonstrate high effectiveness within their respective fields, their applicability is limited due to their lack of generalizability across broader use cases. 

**Table 14.** AI-Integrated blockchain for specialized applications. 

|**Algorithm**|**Core Mechanism**|**Communication**<br>**Complexity**|**Key Advantages**|**Existing Limitations**|
|---|---|---|---|---|
|XG-PBFT [133]|Using XGBoost for<br>node grouping and<br>reputation score<br>allocation|_O_(_N_)|Suitable for edge<br>devices|Requires additional<br>computational<br>overhead|
|Degree Certifcate<br>Verifcation [131]|NLP + Blockchain for<br>credential validation|_O_(_N_log_N_)|Prevents academic<br>fraud|Dependence on NLP<br>accuracy|
|Multi-Party<br>Consensus [132]|AI-enhanced<br>Nakamoto consensus|_O_(_N_2)|Suitable for<br>privacy-preserving<br>computing|High computational<br>cost|
||Using decision tree||||
|CE-PBFT [134]|algorithms for node<br>grouping and<br>reputation score|_O_(_N_)|Suitable for edge<br>devices|Requires additional<br>computational<br>overhead|
||allocation||||



## _3.2. AI-Driven Smart Contract_ 

With the rapid development of blockchain technology, smart contracts, as its core component, have shown great potential in achieving automatic execution and decentralized collaboration. However, current smart contracts still face many challenges in terms of security, execution efficiency, domain adaptability, and resource management. AI, as a technical means with strong learning and pattern recognition capabilities, has been widely used in various life cycle links of smart contracts and has become an important engine to promote their performance improvement and security. 

At present, the research focus of the integration of artificial intelligence and blockchain is on smart contract vulnerability detection and the construction of trust mechanisms in specific scenarios, and it has also expanded to multiple directions such as code optimization, automatic generation, and access control. In order to sort out the existing research results and clearly distinguish different technical paths and application scenarios, this paper divides the research on smart contracts driven by artificial intelligence into four categories: smart contract security and vulnerability detection, smart contract optimization and generation, smart contract applications in specific fields, and smart contract access control and resource management. A detailed classification and comparison of these studies are presented in Table 15. 

_Algorithms_ **2025** , _18_ , 263 

36 of 67 

**Table 15.** Classification and comparison of AI-driven smart contract research. 

|**Category**|**Main Research**<br>**Focus**|**Core Techniques**|**Application**<br>**Scenarios**|**Major Contributions**|
|---|---|---|---|---|
||Identifying and||||
|Smart Contract<br>Security and<br>Vulnerability<br>Detection|detecting security<br>vulnerabilities such<br>as re-entrancy<br>attacks, integer<br>overfows, and<br>timestamp|Static/Dynamic<br>Analysis, GNN,<br>Transformer, LSTM,<br>Fuzzing.|Ethereum, DeFi,<br>Supply Chain.|Improving<br>vulnerability<br>detection accuracy<br>and reducing false<br>positives/negatives.|
||dependencies.||||
|Smart Contract<br>Optimization and<br>Generation|Enhancing smart<br>contract reusability,<br>automation,<br>classifcation, and<br>effciency.|NLP, AI-based<br>recommendation<br>systems,<br>meta-modeling<br>(UML), Code<br>Annotation.|Blockchain<br>Development, EVM<br>Contracts, Smart<br>Contract Libraries.|Reducing<br>development time,<br>improving contract<br>code quality.|
|Smart Contract<br>Applications in<br>Specifc Domains|Utilizing smart<br>contracts in<br>real-world<br>applications such as<br>smart cities, supply<br>chain management,<br>and healthcare.|Blockchain + Smart<br>Contracts,<br>Decentralized<br>Storage (IPFS),<br>Attribute-Based<br>Access Control.|Smart Cities,<br>Agriculture, Food<br>Supply Chain,<br>Healthcare.|Demonstrating the<br>feasibility of smart<br>contract applications<br>and improving data<br>reliability and<br>traceability.|
||Research on access||||
|Smart Contract<br>Access Control and<br>Resource<br>Management|control mechanisms,<br>privacy protection,<br>and computational<br>resource<br>optimization in|SDN + Smart<br>Contracts,<br>Decentralized<br>Databases, ABAC<br>Mechanism.|IoT, Distributed<br>Systems, Security<br>Management.|Enhancing smart<br>contract scalability,<br>security, and effcient<br>resource utilization.|
||smart contracts.||||



In terms of smart contract security, AI is widely used for vulnerability detection and repair suggestion generation. Through GNN, deep learning models, and natural language processing technology, automatic identification and classification of contract logic defects can be achieved, significantly improving the security assurance before contract deployment. Especially in the identification of classic vulnerabilities such as reentrancy attacks and integer overflows, AI models show higher detection accuracy and scalability than traditional tools. 

For the optimization and generation of smart contracts, AI promotes the automatic reconstruction and code generation of contract structures through reinforcement learning, neural architecture search, and pre-trained language models, thereby reducing redundant logic, reducing gas fees, and improving contract execution efficiency. Related research also combines machine learning with contract bytecode analysis to achieve automatic slimming and efficient execution path reasoning. 

In specific industry scenarios, such as finance, medical care, logistics, etc., AI technology is used to build knowledge graphs, multi-agent interaction models, and cross-chain semantic adaptation mechanisms to improve the understanding and response capabilities of smart contracts to complex business rules. This makes smart contracts easier to adapt to multi-chain deployment and dynamic business needs. 

In addition, AI optimization solutions are gradually being introduced into smart contract access control and resource management. Researchers use graph learning, federated 

_Algorithms_ **2025** , _18_ , 263 

37 of 67 

learning, and reinforcement learning methods to model user permissions and conduct behavioral analysis, achieving dynamic allocation of resources and adaptive management of security boundaries. At the same time, explainable AI technology has also begun to be applied to the permission decision-making process to improve its compliance and regulatory compliance. 

## 3.2.1. AI-Driven Smart Contract Security and Vulnerability Detections 

Blockchain smart contract vulnerabilities can lead to irreversible economic losses, while traditional detection methods suffer from high false positives, omissions, and difficulty in dealing with complex attacks. As the complexity of smart contracts increases, code concealment, and new attack methods make detection more difficult, AI-driven vulnerability detection improves detection accuracy and reduces false positives and omissions through deep learning and graph neural networks and also has adaptive learning capabilities to identify variant attacks and provide intelligent remediation solutions. Compared with traditional methods, AI has significant advantages in efficiency, accuracy, and adaptability and can more effectively protect smart contract security. 

Smart contracts, as a crucial application of blockchain technology, are susceptible to a variety of potential vulnerabilities during execution. As shown in Table 16, these flaws may pose serious threats to the security and reliability of smart contracts. Below is a detailed overview of several key categories of smart contract vulnerabilities: 

## (1) Arithmetic Vulnerabilities 

Arithmetic vulnerabilities primarily arise from errors in mathematical operations within smart contracts, most notably overflow and underflow. In programming, data types typically have fixed value ranges. When the result of an operation exceeds these limits, overflow or underflow occurs. The root causes often include a lack of boundary checking and the absence of secure math libraries. Such libraries can detect and prevent abnormal results during computation. For instance, in ERC20 token contracts, overflow may occur during token increment or decrement operations. This could lead to incorrect token calculations—such as unintentionally increasing or decreasing token balances—ultimately allowing attackers to mint tokens illegitimately. Exploiting such a flaw, a malicious user could inflate their token balance and transfer the excess tokens to another address, resulting in financial loss. 

## (2) Re-entrancy Attacks 

Re-entrancy attacks exploit the recursive nature of function calls in smart contracts. Specifically, the attack occurs when a contract makes an external call (e.g., to another contract or function) and, before the contract’s state is updated, an attacker re-enters the contract through a fallback or recursive function. This is possible due to the lack of appropriate safeguards or function locking mechanisms. A well-known example is the infamous DAO attack, where the attacker repeatedly invoked the withdrawal function before the contract updated the balance. This allowed multiple unauthorized withdrawals, eventually draining a significant amount of funds and triggering a hard fork of the Ethereum blockchain. 

## (3) Unchecked Calls 

Unchecked call vulnerabilities stem from the failure to verify the return values of low-level function calls. In Solidity, functions like call() and delegatecall() may fail due to network issues, contract state problems, or unexpected behaviors. If the developer does not explicitly check whether the call was successful, it may lead to hidden execution failures or undesired behavior. One notable incident is the Parity Wallet vulnerability, where critical contract operations were carried out based on unverified return values. As a result, users 

_Algorithms_ **2025** , _18_ , 263 

38 of 67 

might assume that operations such as fund transfers were successfully completed, while in reality, the funds remained in the original address or were misdirected, causing unintended financial consequences. 

## (4) Inconsistent Access Control 

Inconsistent access control issues are mainly due to improper or missing configuration of role-based permissions in smart contracts. Ideally, different roles (e.g., administrator, user) should have clearly defined privileges, ensuring that only authorized entities can perform sensitive actions. However, if these mechanisms are misconfigured—such as assigning administrative rights to regular users or failing to enforce restrictions—the contract becomes vulnerable. Such flaws may allow unauthorized users to perform critical actions like modifying contract parameters or transferring funds, thereby compromising the integrity and security of the entire contract system. 

**Table 16.** Comparison of key categories of smart contract vulnerabilities. 

|**Type**|**Defnition**|**Causes**|**Typical Example**|**Potential Impact**|
|---|---|---|---|---|
|Arithmetic<br>Vulnerabilities|Errors in<br>mathematical<br>operations such as<br>overfow and<br>underfow.|Lack of boundary<br>checks and absence<br>of safe math libraries.|Overfow in ERC20<br>tokens.|Incorrect<br>calculations,<br>unauthorized fund<br>transfers.|
|Re-entrancy Attacks|Exploiting recursive<br>calls to re-enter the<br>contract before state<br>updates.|State changes occur<br>after external calls;<br>absence of function<br>locks.|The DAO attack.|Repeated fund<br>withdrawals, total<br>asset drain.|
|Unchecked Calls|Failure to verify the<br>return value of<br>low-level function<br>calls.|Ignoring<br>success/failure<br>checks for`call()`,<br>`delegatecall()`, etc.|Parity Wallet bug.|Logic errors, silent<br>failures, potential<br>loss of funds.|
|Inconsistent Access<br>Control|Improper or missing<br>role-based<br>restrictions for<br>function access.|Misconfgured or<br>absent access control<br>mechanisms.|Governance function<br>override.|Unauthorized users<br>gain control of<br>sensitive operations.|



Lyu et al. [136] first introduced the Contract Dependency Graph (CDG) to analyze vulnerabilities in Ethereum smart contracts. The CDG characterizes inter-contract dependencies formed by DELEGATECALL-type internal transactions in Ethereum. They then proposed three general definitions of security violations related to the CDG to identify potential victim contracts affected by different types of vulnerable contracts. To validate these security violations, they constructed a CDG using 195,247 active smart contracts from the latest Ethereum blocks and successfully detected three representative known vulnerabilities. Yang et al. [137] proposed CrossFuzz, a fuzzing-based approach for crosscontract vulnerability detection. Their method leverages data flow information to optimize mutation strategies, significantly improving the efficiency of fuzz testing in identifying cross-contract vulnerabilities. Gao et al. [138] introduced an automated approach to learning the characteristics of Solidity-based smart contracts, which is particularly beneficial for clone detection, error detection, and contract verification. Wei et al. [139] conducted a comprehensive survey on smart contract quality assurance, covering vulnerability identification, the evaluation of detection tools, and dataset development. They developed an annotated dataset as a benchmark, including ten common vulnerability types, serving as a critical resource for future research. Additionally, they conducted extensive experiments 

_Algorithms_ **2025** , _18_ , 263 

39 of 67 

evaluating 14 vulnerability detection tools, providing comparative analyses that highlight their strengths and limitations. Vidal et al. [140] proposed OpenSCV, a novel open and hierarchical classification system for smart contract vulnerabilities. OpenSCV is designed to be community-driven, aligning with current industry practices while remaining adaptable to future modifications and advancements. Wang et al. [141] introduced HomoDec, a timestamp vulnerability detection technique for smart contracts based on code homogeneity analysis. The core idea is to compare the tested smart contract’s code against a database of known vulnerability patterns, assessing similarities to determine whether the tested contract exhibits timestamp-related vulnerabilities. Fei et al. [142] developed MSmart, a novel tool that transforms smart contract source code into an intermediate representation and detects vulnerabilities based on XPath rules. They extended SmartCheck’s intermediate representation rules to identify a broader range of vulnerabilities and optimized existing rules to accommodate the increasing complexity of smart contracts. Chen et al. [143] introduced DefectChecker, a symbolic execution-based method and tool designed to detect eight types of contract defects that may lead to undesirable behaviors on the Ethereum blockchain. DefectChecker identifies these defects directly from smart contract bytecode. Lin et al. [144] developed CRPWarner (contract-related Rug Pull Risk Warner) to identify malicious functionalities within smart contracts and issue warnings about potential rug pull risks. Wu et al. [145] proposed a Hybrid Attention Mechanism (HAM) model for detecting security vulnerabilities in smart contracts. Their approach extracts code snippets from source code, focusing on critical areas where vulnerabilities are likely to occur. Zhang et al. [146] introduced CBGRU, a hybrid deep learning model that strategically integrates various word embeddings (Word2Vec and FastText) with multiple deep learning architectures (LSTM, GRU, BiLSTM, CNN, and BiGRU). The model extracts features using different deep learning methods and combines them to enhance smart contract vulnerability detection. Hwang et al. [147] proposed CodeNet, a code-oriented CNN architecture designed to detect vulnerable smart contracts while preserving semantic and contextual information. To further improve CodeNet’s performance, they developed a data preprocessing module that transforms smart contracts into images while maintaining local structural integrity. Gao et al. [148] explored the use of graph neural networks (GNNs) combined with expert knowledge for smart contract vulnerability detection. Their approach transforms the rich control and data flow semantics of source code into a contract graph and introduces a novel temporal message propagation network to extract graph features from a normalized graph. These extracted features are then integrated with expert-designed patterns to develop a comprehensive detection system. Ivanov et al. [149] proposed a novel smart contract security testing method called Transaction Encapsulation. The core idea involves executing transactions locally on fully synchronized yet isolated Ethereum nodes, allowing for a preview of transaction sequence outcomes in the blockchain’s current state. To ensure testing accuracy, their system, TxT, deterministically verifies whether a given transaction sequence follows the same execution path within the blockchain’s current state. 

Yu et al. [150] introduced TxMirror, an innovative data-driven framework that detects bytecode-level smart contract vulnerabilities by symmetrically simulating transactions. Beyond logical analysis, TxMirror customizes the Ethereum Virtual Machine (EVM) for vulnerability detection and stores stack data and logical dependencies in a bi-linked forest structure, enabling efficient indexing of logical relationships. This approach allows TxMirror to directly examine custom EVM stack data within transactions without requiring repeated transaction replays or EVM bytecode-level tracing. Li et al. [151] presented a multimodal feature fusion-based approach for smart contract vulnerability detection. Their method extracts three types of modal features from the smart contract lifecycle, leveraging both static and dynamic characteristics. By integrating Graph Convolutional Networks 

_Algorithms_ **2025** , _18_ , 263 

40 of 67 

(GCN) and Bidirectional Long Short-Term Memory (Bi-LSTM) networks, the system effectively detects various vulnerabilities in smart contracts. Pasqua et al. [152] proposed a new method based on symbolic execution of the EVM operand stack, enabling precise jump address parsing within EVM bytecode and constructing an accurate Control Flow Graph (CFG) of compiled smart contracts. Additionally, they extended EtherSolve with two specialized detectors for identifying Ethereum’s two most prominent vulnerabilities—Reentrancy and Tx.origin misuse. Qian et al. [153] introduced a hybrid approach combining Bi-LSTM and an attention mechanism for multi-label vulnerability detection in smart contract opcode sequences. The process begins with preprocessing opcode data, transforming it into a feature matrix suitable for neural network input. The system then employs an attention-enhanced Bi-LSTM model to classify smart contracts with multiple vulnerabilities. Peng et al. [154] developed RNVulDet, a tool that integrates taint analysis techniques to automatically identify poor randomness vulnerabilities and detect corresponding exploit transactions. Le et al. [155] proposed a comprehensive approach leveraging pre-trained Large Language Models (LLMs) for Ethereum smart contract vulnerability detection. By applying transformer-based LLMs, their method utilizes the models’ ability to understand and analyze Solidity code, effectively identifying potential security vulnerabilities. Wang et al. [156] introduced a metric learning and Bi-LSTM-based vulnerability detection framework for Ethereum smart contracts, optimizing the model’s feature space. Their approach was evaluated on a large-scale dataset containing four major types of vulnerabilities: arithmetic vulnerabilities, re-entrancy attacks, unchecked calls, and inconsistent access control mechanisms. 

As shown in Tables 17 and 18, in the domain of smart contract security and vulnerability detection, research primarily centers on enhancing detection accuracy and efficiency through techniques such as graph analysis, symbolic execution, machine learning, and deep learning. Studies demonstrate high efficacy in identifying prevalent vulnerabilities like re-entry attacks, integer overflows, and timestamp dependencies, with significantly reduced false-positive rates. However, existing approaches exhibit notable limitations, including substantial computational costs, reliance on frequent rule updates, and diminished performance in detecting complex vulnerabilities. Notably, AI-driven methods, while adept at recognizing known threats, often lack generalizability to emerging attack vectors such as cross-chain exploits and compositional vulnerabilities. Furthermore, current methodologies infrequently integrate multi-modal analyses that correlate code behavior with transactional patterns, thereby constraining system robustness and threat coverage. Future advancements may lie in exploring self-supervised learning frameworks and multi-modal neural architectures to bolster defensive capabilities against unknown adversarial strategies. 

**Table 17.** Comparison of smart contract security and vulnerability detection research. 

|**Author**|**Core Innovation**|**Algorithm**|**Performance**<br>**Comparison**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
||Contract|||||
|Lyu et al. [136]|Dependency<br>Graph (CDG) for<br>vulnerability|Graph analysis|14.7% victim<br>contract<br>detection|Identifes<br>vulnerability<br>chains|Relies on CDG<br>quality|
||propagation|||||
|Yang et al. [137]|Cross-contract<br>fuzzing-based<br>vulnerability<br>detection|Fuzzing, data<br>fow analysis|Improved<br>detection<br>effciency|Effective for<br>multi-contract<br>scenarios|High<br>computation cost|



_Algorithms_ **2025** , _18_ , 263 

41 of 67 

**Table 17.** _Cont._ 

|**Author**|**Core Innovation**|**Algorithm**|**Performance**<br>**Comparison**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
|Gao et al. [138]|Structural code<br>embedding for<br>clone/error<br>detection|Code embedding|90% clone<br>detection rate|Enhances<br>vulnerability<br>detection|Requires existing<br>vulnerability<br>databases|
|Wei et al. [139]|Comprehensive<br>survey of 40<br>vulnerabilities|Survey study|Benchmarked<br>14 detection tools|Provides<br>classifcation<br>framework|Cannot detect<br>new<br>vulnerabilities|
|Vidal et al. [140]|OpenSCV<br>hierarchical<br>taxonomy|Machine<br>learning<br>classifcation|Improved<br>classifcation<br>clarity|Open-source and<br>extendable|Subjective<br>expert-based<br>classifcation|
|Wang et al. [141]|Timestamp<br>vulnerability<br>detection using<br>code<br>homogeneity|Homogeneity-<br>based similarity<br>matching|Identifed<br>vulnerable<br>contracts<br>effciently|Effective for<br>timestamp-<br>related<br>vulnerabilities|Limited to<br>specifc<br>vulnerability<br>types|
||Improved|||||
|Fei et al. [142]|SmartCheck-<br>based<br>vulnerability<br>analysis tool|Intermediate<br>representation<br>and XPath rules|Reduced false<br>positives and<br>false negatives|Detects more<br>vulnerability<br>types than<br>SmartCheck|Complexity in<br>rule optimization|
||(MSmart)|||||
|Chen et al. [143]|Symbolic<br>execution for<br>defect detection<br>(DefectChecker)|Bytecode-level<br>symbolic<br>execution|88.8% F-score,<br>0.15s per contract|Fast and accurate<br>detection|Limited to EVM<br>bytecode|
||Rug pull risk|||||
|Lin et al. [144]|detection for<br>DeFi smart<br>contracts|Analyzing<br>contract<br>functionality|91.8% precision,<br>85.9% recall|High detection<br>rate for rug pull<br>risks|Requires labeled<br>datasets|
||(CRPWarner)|||||
||Hybrid Attention|||||
|Wu et al. [145]|Mechanism<br>(HAM) for<br>security|Attention<br>mechanism|93.36%<br>re-entrancy<br>detection|Combines static<br>and dynamic<br>analysis|High<br>computational<br>overhead|
||vulnerabilities|||||
|Zhang et al. [146]|CBGRU hybrid<br>deep learning<br>model for<br>vulnerability<br>detection|Combination of<br>CNN, LSTM,<br>BiGRU|Improved<br>performance on<br>SmartBugs<br>dataset|High detection<br>accuracy|Requires<br>signifcant<br>computational<br>resources|
|Hwang<br>et al. [147]|CodeNet:<br>CNN-based<br>vulnerability<br>detection|Code<br>transformation<br>to images|Competitive<br>detection<br>performance|Novel approach<br>using deep<br>learning|Potential loss of<br>semantic<br>information|



_Algorithms_ **2025** , _18_ , 263 

42 of 67 

**Table 18.** Comparison of smart contract security and vulnerability detection research. 

|**Author**|**Core Innovation**|**Algorithm**|**Performance**<br>**Comparison**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
||||Detection|||
|Gao et al. [148]|GNN with<br>expert<br>knowledge|Contract graph<br>representation|accuracy: 89.15%<br>re-entrancy,<br>89.02%<br>timestamp|Incorporates<br>domain<br>knowledge with<br>GNN|High complexity<br>in graph<br>processing|
||||dependency|||
|Gao et al. [149]|Transaction<br>encapsulation for<br>security testing<br>(TxT)|Local execution<br>of transactions<br>on isolated<br>Ethereum nodes|96.5%<br>transactions<br>verifed, 83.8%<br>vulnerability<br>coverage|High accuracy<br>and coverage|Requires<br>synchronized<br>node execution|
|Yu et al. [150]|TxMirror: EVM-<br>stack-based<br>vulnerability<br>detection|Transaction<br>simulation and<br>double link<br>forest indexing|Effective<br>detection of<br>unknown<br>vulnerabilities|Novel EVM stack<br>analysis<br>approach|Limited<br>applicability<br>beyond<br>Ethereum|
|Jinggang<br>et al. [151]|Multi-modal<br>feature fusion<br>with deep<br>learning|GCN and<br>Bi-LSTM<br>integration|90.96% accuracy<br>for integer<br>overfow<br>detection|Fuses static and<br>dynamic features|High training<br>complexity|
|Pasqua<br>et al. [152]|Control Flow<br>Graph (CFG)<br>reconstruction<br>for smart<br>contracts|Symbolic<br>execution-based<br>CFG analysis|More precise<br>CFG extraction<br>than existing<br>methods|Enhances static<br>vulnerability<br>analysis|Requires<br>symbolic<br>execution<br>overhead|
|Qian et al. [153]|Multi-label<br>vulnerability<br>detection using<br>Bi-LSTM|Neural network<br>classifcation for<br>multiple<br>vulnerabilities|Over 85%<br>accuracy for all<br>detected<br>vulnerabilities|Detects multiple<br>vulnerabilities<br>simultaneously|Potential<br>overftting to<br>training data|
|Peng et al. [154]|RNVulDet:<br>Random number<br>vulnerability<br>detection|Taint analysis<br>and attack<br>detection|44,264 attack<br>transactions<br>detected|Fast detection<br>speed (2.98 s per<br>contract)|Requires a large<br>labeled dataset|
|Le et al. [155]|Transformer-<br>based<br>vulnerability<br>detection|Fine-tuned LLM<br>for Solidity code<br>analysis|Effective<br>detection of<br>security faws|High<br>interpretability<br>with transformer<br>models|Requires high<br>computational<br>resources|
|Wang et al. [156]|Triplet Loss and<br>Bi-LSTM for<br>vulnerability<br>detection|Metric<br>learning-based<br>feature<br>optimization|93.25% accuracy<br>for re-entrancy<br>detection|Optimized<br>feature<br>representation|Limited to<br>predefned<br>vulnerability<br>categories|



3.2.2. AI-Driven Smart Contract Optimization and Generation 

Blockchain smart contracts effectively solve the centralized trust problem in several specific scenarios, such as decentralized finance (DeFi), supply chain management, and e-government. In order to further boost the efficiency and adaptability of smart contracts, scholars at home and abroad have attempted to introduce AI into smart-contract-related research. 

Table 19 summarizes key approaches to smart contract optimization, highlighting that while these methods enhance automation, efficiency, and maintainability, many are 

_Algorithms_ **2025** , _18_ , 263 

43 of 67 

limited by platform dependence or additional metadata requirements. Park et al. [157] proposed a smart contract proxy to enhance the reusability of smart contracts in blockchain environments. Their approach provides a framework for comparing and reusing smart contracts, with the proposed proxy serving as a reference model to improve the flexibility and reusability of smart contract management in blockchain ecosystems. Sanchez-Gomez et al. [158] presented the first overview of a UML-based smart contract metamodel, which enables the implementation of these objectives. This metamodel is expected to be further refined in future work to represent blockchain environments and facilitate automated testing. Gec et al. [159] introduced a novel recommendation system designed to support the low-cost development and deployment of EVM-based smart contracts. The system’s algorithm assists developers by suggesting smart contract templates that align with their requirements and are compatible with fog computing architectures. 

**Table 19.** Summary of smart contract optimization and generation approaches. 

|**Category**|**Key Techniques**|**Advantages**|**Limitations**|
|---|---|---|---|
|Smart Contract Agents|Autonomous code<br>generation, lifecycle<br>management|High automation, reduced<br>human effort|May rely on predefned<br>behaviours or agents|
|Meta-Modeling|Contract abstraction and<br>model-drivengeneration|Facilitates reuse, improves<br>modularity|Often tied to specifc<br>ecosystems|
|AI Recommendation<br>Systems|Code completion and<br>contract suggestion<br>engines|Boosts effciency, leverages<br>historical data|Requires large training<br>data and context awareness|
|Code Annotation|Semantic tagging and|Enhances maintainability|Needs standardization and|
|Optimization|structured metadata|and readability|metadata consistency|
|NLP Disambiguation|Natural language to code<br>translation|Lowers entry barrier,<br>enables text-driven<br>development|Prone to ambiguity, may<br>reduce accuracy in<br>complex logic|



Chen et al. [160] proposed an Annotation-Assisted Smart Contract Generation Method. To evaluate its effectiveness, the method was tested using BLEU scores for bilingual quality assessment, along with security analysis tools such as Mythril and VaaS. The results were then compared with those of existing approaches. Tong et al. [161] introduced a new SLA management model that leverages SDN-enabled WSNs composed of wireless nodes to interact with smart contracts in a simplified manner. The proposed model ensures the persistence of network metrics and SLA compliance through smart contracts, eliminating the need for third-party audits in payment and compensation procedures. 

Table 20 presents five representative approaches to smart contract optimization, each enhancing reusability, automation, or compatibility, while facing limitations such as metadata dependence, ecosystem constraints, or implementation immaturity. 

_Algorithms_ **2025** , _18_ , 263 

44 of 67 

**Table 20.** Comparison of smart contract optimization and generation research. 

|**Author**|**Core Innovation**|**Algorithm**|**Performance**<br>**Comparison**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
|Park et al. [157]|Smart contract<br>broker for<br>reusability|Broker<br>mechanism|Improved reuse|Enhances<br>fexibility|Needs metadata<br>management|
|Sanche<br>et al. [158]|Smart contract<br>meta-modeling<br>(UML)|UML-based<br>modeling|Standardized<br>modeling<br>framework|Aids automation|Needs further<br>extension|
|Gec et al. [159]|Smart contract<br>recommender<br>system|OpenZeppelin +<br>ML|Reduced<br>development<br>time|Supports EVM<br>contracts|Relies on<br>OpenZeppelin<br>ecosystem|
||AI-assisted|||||
|Chen et al. [160]|contract<br>annotation|Code annotation<br>+ AI|BLEU score<br>+27%|High automation|Limited to<br>specifc domains|
||generation|||||
||AI-assisted word|||||
|Tong et al. [161]|segmentation for<br>contract|AI-based<br>segmentation|Multi-language<br>support|Compatible|Requires NLP<br>capabilities|
||generation|||||



## 3.2.3. AI-Driven Smart Contract Applications in Specific Domains 

Nowadays, blockchain smart contracts have been widely used. For an AI-driven implementation of smart contracts in specific applications, relevant scholars have done the following research: Margret et al. [162] proposed a tamper-proof framework for storing and retrieving data. The framework leverages AI techniques to evaluate the mathematical computations of smart contracts, including overhead, execution time, average computational cost, standard deviation, throughput, and resource utilization. 

To address the issues of widespread fake restaurant reviews, unreliable rating authenticity, and low customer satisfaction with recommended dishes, Zhang et al. [163] introduced a peer-to-peer architecture based on blockchain smart contracts. Additionally, AI-powered predictive analytics were employed to enhance data analysis and improve recommendation accuracy. Xiao et al. [164] proposed a plastic-credit-driven incentive system composed of a recyclability index (RI) and plastic credits to promote plastic recycling and enhance the quality of recyclable plastics. This system relies on a market self-regulation mechanism to guide industry participants in self-regulating and optimizing plastic production and trading processes. Wang et al. [165] proposed a consortium blockchain and smartcontract-based framework for tracking and tracing agricultural supply chain workflows. This approach enhances traceability and data sharing while mitigating information silos among enterprises, reducing the reliance on centralized institutions. Moreover, it improves the integrity, reliability, and security of transaction records. Garcia et al. [166] explored the potential of leveraging smart contracts in Byzantine Fault Tolerant (BFT) blockchain platforms. Specifically, they examined the BFT blockchain platforms Tendermint and Hyperledger Besu and applied them to a decentralized e-prescription management case study to evaluate their effectiveness and feasibility. Agrawal et al. [167] investigated the design of a blockchain-based resource-sharing collaborative framework, leveraging smart contracts to facilitate decentralized resource allocation. This framework is particularly well suited for broader networks or ecosystems beyond conventional supply chains, where established collaboration mechanisms and hierarchical structures exist to enhance resource management transparency and data authenticity. 

_Algorithms_ **2025** , _18_ , 263 

45 of 67 

As shown in Table 21, the research on smart contract applications in specific domains such as smart cities, food supply chains, agriculture, and healthcare has primarily focused on leveraging their capabilities in data management, trust mechanisms, and traceability. Studies demonstrate that smart contracts can enhance data integrity, reduce reliance on intermediaries, and improve supply chain transparency. However, many of these solutions exhibit high computational resource consumption, raising concerns about their scalability and applicability in large-system deployments. This is primarily because AI-assisted smart contracts require complex data preprocessing, feature extraction, and model inference, all of which involve intensive computations. Additionally, maintaining security and trust often necessitates mechanisms such as Trusted Execution Environments (TEEs) or zero-knowledge proofs (ZKPs), further increasing the computational load. Frequent on-chain and off-chain interactions for model verification and the potential need for continuous model updates or retraining exacerbate resource demands. Furthermore, while AI-enhanced smart contract studies show promise, they predominantly remain at the proof-of-concept stage, lacking comprehensive validation in real-world, large-scale scenarios. Critical performance bottlenecks, including transaction throughput and latency under high-load conditions, are often overlooked. Addressing these challenges requires advancing modular and reusable smart contract architectures, alongside integrating AI techniques to predict and optimize performance in high-frequency, resource-constrained transaction environments. 

**Table 21.** Comparison of smart contract implementations in specific applications. 

|**Author**|**Core Innovation**|**Algorithm**|**Application**<br>**Scenario**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
|Margret<br>et al. [162]|Smart contract<br>framework for<br>smart cities|Blockchain +<br>Smart Contracts|Smart Cities|Solves data<br>management|High<br>computational<br>demand|
|Zhang et al. [163]|Peer-to-peer<br>food delivery<br>automation|Smart contract<br>automation|Food supply<br>chain|Trust-enhancing,<br>commission-free|Limited<br>transaction<br>throughput|
|Xiao et al. [164]|Smart<br>contract-aided<br>plastic credit<br>scheme|Blockchain +<br>Smart Contracts|Plastic recycling<br>governance|Promotes<br>self-regulated<br>plastic trading,<br>improves<br>recyclability|Requires<br>industry-wide<br>adoption|
||Blockchain-|||||
|Wang et al. [165]|based<br>agricultural<br>product|IPFS + Smart<br>Contracts|Agricultural<br>supply chain|Improves data<br>integrity|Dependent on<br>distributed<br>storage|
||traceability|||||
|Garcia et al. [166]|Smart contracts<br>in PBFT-based<br>blockchains for<br>medical<br>prescriptions|PBFT Blockchain<br>(Tendermint,<br>Hyperledger<br>Besu) + Smart<br>Contracts|Decentralized<br>e-prescription<br>system|Effcient for<br>multi-<br>stakeholder<br>applications|Limited<br>scalability<br>compared to<br>PoW-based<br>solutions|
|Agrawal<br>et al. [167]|Blockchain-<br>based supply<br>chain<br>collaboration<br>framework|Ethereum +<br>Smart Contracts|Supply chain<br>collaboration|Ensures data<br>authenticity and<br>quality in supply<br>networks|Challenges in<br>integration with<br>existing supply<br>chains|



_Algorithms_ **2025** , _18_ , 263 

46 of 67 

## 3.2.4. AI-Driven Smart Contract Access Control and Resource Management 

With the increasing adoption of blockchain technology across industries, smart contracts have become a key component in automating and securing digital transactions. However, traditional smart contracts face several limitations in access control and resource management, especially in large-scale decentralized environments. Traditional access control models, such as Role-Based Access Control (RBAC) and Attribute-Based Access Control (ABAC), often struggle with flexibility, adaptability, and computational efficiency when deployed in dynamic and heterogeneous blockchain ecosystems. AI-driven mechanisms have been introduced into smart contract systems. AI enhances access control by making real-time, context-aware decisions and optimizes resource management through predictive analytics and automated allocation policies. Zhang et al. [168] integrated blockchain-based smart contracts with the ABAC model, proposing a distributed and reliable access control framework for smart cities. Karakoc et al. [169] proposed a novel SLA (Service Level Agreement) management model that utilizes SDN (Software Defined Networking)-enabled WSNs (Wireless Sensor Networks) composed of wireless nodes to facilitate seamless interaction with smart contracts. The proposed model ensures the persistence of network performance metrics and SLA specifications through smart contracts, thus eliminating the need for intermediaries to audit payment and compensation procedures. Guo et al. [170] address the above challenges by introducing a privacy-preserving decentralized database system in a parallel smart contract model. The proposed system implements oblivious access interactions, which are inherently asymmetric and require significant computational effort on the smart contract side. By slightly reducing the overhead of oblivious memory (ORAM) and mitigating the problems associated with centralized servers, the system enhances privacy and decentralization in blockchain-based applications. 

As shown in Table 22, the research on access control and resource management in blockchain systems highlights significant limitations in existing solutions. Current approaches predominantly rely on static ABAC strategies, which prove inadequate for the highly dynamic and decentralized nature of blockchain environments. Additionally, decentralized data access mechanisms such as Oblivious RAM introduce substantial computational overheads and demonstrate poor compatibility with smart contract execution. To address these challenges, future research should explore two key avenues: first, developing AI-driven dynamic access control systems capable of real-time adaptation to contextual changes, and second, innovating lightweight secure storage models to enable efficient and scalable resource management within blockchain ecosystems. These advancements aim to overcome performance bottlenecks while enhancing security and adaptability in decentralized architectures. 

**Table 22.** Comparison of smart contract access control and resource management research. 

|**Author**|**Core Innovation**|**Method/Algorithm**|**Application**<br>**Scenario**|**Advantages**|**Limitations**|
|---|---|---|---|---|---|
|Zhang et al. [168]|ABAC for smart<br>contract access<br>control|ABAC model|Access<br>management|Strong security|High<br>computational<br>resources|
||SLA management|||||
|Karakoc et al. [169]|using<br>SDN-integrated|SDN + Smart<br>Contracts|Wireless sensor<br>networks|Energy-effcient|Suitable for<br>specifc networks|
||smart contracts|||||
|Guo et al. [170]|Decentralized<br>database access<br>control|ORAM + Parallel<br>Contracts|Database<br>management|Enhances privacy|High<br>computational<br>complexity|



_Algorithms_ **2025** , _18_ , 263 

47 of 67 

## _3.3. AI-Driven Privacy Protection_ 

In today’s data-driven era, privacy protection has become a crucial issue. With the rapid development of artificial intelligence, the IoT, and Big Data technologies, huge amounts of sensitive data are being collected, stored, and analyzed, such as medical records, financial transactions, and user behavior data. However, there is an increasing risk of data leakage, unauthorized access, and malicious attacks, which not only threaten users’ personal privacy but may also lead to the leakage of confidential corporate information. 

FL is an emerging decentralized machine learning approach that allows multiple clients to train models locally and share only the model update parameters, not the raw data, thus enabling federated modeling without exposing private data. The core idea of FL is to perform collaborative learning across multiple devices or organizations while minimizing the risk of data leakage, reducing the risk of data leakage. FL mainly consists of horizontal federated learning (for data with the same features but different users), vertical federated learning (for data with shared users but different features), and federated migration learning (for the case of uneven data distribution), which is widely used in finance, healthcare, and IoT. Privacy protection is a key challenge in blockchain systems. Traditional blockchain architectures make users’ private information vulnerable to attacks due to the openness and transparency of data. Federated learning combined with blockchain technology enables decentralized model training without sharing raw data while relying on blockchain’s distributed ledger, immutability, and smart contracts to enhance data security and transparency. Compared with traditional centralized privacy protection mechanisms, federated-learning-based blockchain privacy protection schemes can reduce the risk of data leakage, reduce computation and communication costs, and improve data trustworthiness and thus have become a hot direction in privacy protection research in recent years. The blockchain and federated learning schematic is shown in Figure 9 below. This figure shows the blockchain and AI-based connected car architecture. The lower layer is the On-Board Unit (OBU) interacting with the roadside base station through wireless communication, and the upper layer uses the combination of blockchain and AI federated learning for data processing and privacy protection to ensure data security and uploading efficiency of the Telematics system. 

At present, privacy-protection-related research has made extensive progress in which blockchain technology plays an important role in data security and privacy protection. We classify the research on blockchain privacy protection into the following two categories: (1) federated-learning-based blockchain privacy protection, i.e., the use of federated learning to train distributed models and the combination of blockchain technology in order to improve data security and privacy protection; and (2) other blockchain-based privacy protection mechanisms, such as zero-knowledge proofs, ring signatures, homomorphic encryption, differential privacy, and other methods. These methods ensure data privacy through cryptographic techniques while combining the tamperability and decentralized nature of blockchain to provide stronger privacy protection. 

(1) Federated-Learning-Based Blockchain Privacy Protection 

Despite its advantages in decentralized model training, federated learning faces several pressing challenges when applied to blockchain privacy protection. These include potential privacy leakage through model updates, heterogeneous data distributions affecting model accuracy, high communication overhead, and vulnerability to poisoning attacks from malicious nodes. To address these issues, recent studies have introduced AI-driven enhancements such as differential privacy mechanisms for noise injection, knowledge distillation techniques to reduce communication costs, and graph neural network-based aggregation to improve robustness. Moreover, blockchain-based logging and verification can be employed to trace model contributions, while smart contracts automate model 

_Algorithms_ **2025** , _18_ , 263 

48 of 67 

update validation and access control. These integrated strategies significantly enhance the trustworthiness, scalability, and privacy-preserving capabilities of federated learning in blockchain systems. 

## (2) Other Blockchain-Based Privacy Protection Mechanisms 

Traditional privacy-preserving mechanisms in blockchain, including zero-knowledge proofs (ZKPs), ring signatures, homomorphic encryption, and differential privacy, are often constrained by high computational complexity, limited scalability, and inadequate adaptability to dynamic blockchain environments. These limitations may lead to increased transaction latency, degraded system performance, and greater vulnerability to side-channel attacks or misuse. To mitigate these drawbacks, artificial intelligence has been increasingly applied to optimize these techniques. For example, deep learning models are used to accelerate ZKP generation and verification processes, reinforcement learning enables dynamic tuning of noise levels in differential privacy, and graph neural networks facilitate efficient ring signature routing. Additionally, intelligent schedulers operating within trusted execution environments (TEEs) can allocate privacy-preserving resources adaptively based on contextual data sensitivity. Collectively, these AI-driven solutions offer a promising direction to enhance the efficiency, flexibility, and resilience of privacy protection in blockchain systems. 

**Figure 9.** Schematic of federated learning combined with blockchain. 

_Algorithms_ **2025** , _18_ , 263 

49 of 67 

## 3.3.1. Other Blockchain-Based Privacy Protection Mechanisms 

Traditional blockchain-based privacy protection mainly uses encryption and zeroknowledge proofs to encrypt user data without incorporating federated learning, and many scholars have carried out related research. Selvarajan et al. proposed [171] an ArtificialIntelligence-based Lightweight Blockchain Security Model (AILBSM) for ensuring privacy and security of IIoT systems. This novel model aims to address security and privacy issues that may arise when dealing with cloud-based IIoT systems that process data in the cloud or at the edge of the network (on devices). Wang et al. [172] proposed a novel solution called Hierarchical Blockchain-Enabled Cloud Edge Choreography Federated Learning (HBCE-FL) to address these challenges. HBCE-FL aims to provide secure, intelligent, and distributed data analysis for IoT users. Miao et al. [173] studied how data sharing is achieved through model sharing, and, based on this, a secure data sharing mechanism called BP2P-FL was proposed, which employs peer-to-peer federated learning and protects the privacy of data providers. Smahi et al. [174] proposed BV-ICV, a blockchain-enabled and privacy-preserving FL framework for ICVs in edge-envisioned V2X environments, to alleviate these concerns. The system uses zero-knowledge succinct non-interactive knowledge argument (zkSNARK) validation compiled into smart contracts to prevent malicious, compromised, or even justified ICVs from uploading unreliable, incorrect, or low-quality model updates. Yang et al. [175] proposed an interpretable federated learning approach with controlled machine learning efficiency and controlled credit model decisionmaking, leading to controlled credit model complexity and transparently traceable credit decision-making mechanisms. They then proposed an interpretable federated learning training mechanism for credit data to prevent model gradient leakage trained by a single node throughout the model training period. Firdaus et al. [176] introduced the concept of distributed edge intelligence that combines the advantages of FL, differential privacy (DP), and blockchain to address the previously posed problem. Li et al. [177] proposed a blockchain-enabled federated learning (BeFL) for distributed cybersecurity malicious behaviour knowledge base architecture to ensure the security of knowledge transfer. 

As shown in Table 23, the research on integrating FL with blockchain technology aims to enhance data privacy and security in decentralized data processing across domains like healthcare, IoT, IoV, finance, and supply chains. Blockchain enables decentralized governance, secure data storage, and model validation, while FL facilitates collaborative model training without raw data sharing, mitigating leakage risks. Techniques such as differential privacy, homomorphic encryption (HE), and trusted execution environments (TEEs) are widely employed to strengthen privacy, complemented by smart contracts and reputation mechanisms to ensure system fairness and trustworthiness. However, current approaches often overlook the threat of malicious nodes injecting poisoned models, compromising collaborative learning integrity, and suffer from communication overheads that hinder performance in bandwidth- or latency-constrained environments. Moreover, the applicability of these methods remains limited to data-sensitive distributed scenarios, with algorithmic research facing challenges in scalability and accuracy. Future advancements should prioritize robust anomaly detection mechanisms to filter malicious updates and leverage reinforcement learning to dynamically optimize model synchronization intervals, thereby improving system resilience, communication efficiency, and broader applicability. 

_Algorithms_ **2025** , _18_ , 263 

50 of 67 

**Table 23.** Other blockchain-based privacy protection mechanisms. 

|**Research ID**|**Research Title**|**Key Technologies**|**Application Scenario**|**Main Contributions**|
|---|---|---|---|---|
||An Artifcial||||
|Selvarajan et al. [171]|Intelligence<br>Lightweight Blockchain<br>Security Model for<br>Security and Privacy in|Blockchain, AI,<br>Autoencoder|IIoT|Enhances anomaly<br>detection, optimizes<br>execution time|
||IIoT Systems||||
|Wang et al. [172]|Blockchain nd Access<br>Control Encryption-<br>Empowered IoT<br>Knowledge Sharing|Blockchain, Access<br>Control Encryption|IoT|Achieves secure and<br>personalized privacy<br>protection|
||An Intelligent and||||
||Privacy-Enhanced Data|||Records data-sharing|
|Miao et al. [173]|Sharing Strategy for|Blockchain, BP2P|IoT|processes, improves|
||Blockchain-|||data quality|
||Empowered IoT||||
||BV-ICVs: A||||
||Privacy-Preserving and||||
|Smahi et al. [174]|Verifable Federated<br>Learning Framework<br>for V2X Environments|Blockchain, zkSNARKs|Vehicular Networks<br>(V2X)|Prevents malicious data<br>uploads, enhances<br>privacy protection|
||Using Blockchain and||||
||zkSNARKs||||
||An Explainable||||
|Yang et al. [175]|Federated Learning<br>and Blockchain-Based<br>Secure Credit Modeling|Blockchain,<br>Explainable ML|Finance|Provides transparent<br>and traceable credit<br>decisions|
||Method||||
||A Blockchain-Assisted||||
|Firdaus et al. [176]|Distributed Edge<br>Intelligence for<br>Privacy-Preserving|Blockchain, Differential<br>Privacy|Vehicular Networks|Improves privacy<br>protection and traffc<br>prediction|
||Vehicular Networks||||
||Blockchain-||||
|Li et al. [177]|Empowered Federated<br>Learning for a 6G<br>Knowledge Base on<br>Distributed Network|Blockchain, Knowledge<br>Graphs|Network Security|Secures knowledge<br>transmission, enhances<br>attack tracing|
||Security||||



## 3.3.2. Federated-Learning-Based Privacy Protection for Blockchain 

Nowadays, the blockchain privacy protection scheme combined with federated learning can reduce the risk of data leakage and improve the security and trustworthiness of data at the same time, and many scholars at home and abroad pay attention to this aspect of learning. Wan et al. [178] proposed a federated learning privacy protection method that integrates distillation defense technology with a blockchain architecture. This approach leverages distillation defense to reduce the sensitivity of federated learning client devices to perturbations and enhances their ability to resist adversarial sample attacks. Liu et al. [179] designed a decentralized FL privacy protection framework based on blockchain, differential privacy, and generative adversarial networks (GANs). This framework effectively mitigates single points of failure and enhances resistance to inference attacks. Ma et al. [180] proposed a blockchain-based privacy-preserving federated learning (BPFL) framework that integrates FL, pairwise additive masking, and the Chinese Remainder Theorem. This framework enables mobile participants to collaborate on crowdsourced machine learning tasks while 

_Algorithms_ **2025** , _18_ , 263 

51 of 67 

preserving privacy. Wang et al. [181] proposed a blockchain-based privacy-preserving federated learning scheme, BPFL, which utilizes blockchain as the underlying distributed framework for FL. By improving Multi-Krum technology and integrating homomorphic encryption, the scheme achieves ciphertext-level model aggregation and filtering, ensuring local model authenticity while preserving privacy. Zhao et al. [182] designed a FL-based smart home system that leverages a reputation mechanism to assist home appliance manufacturers in training machine learning models based on customer data, optimizing smart home services. Zhu et al. [183] proposed UBFL, an innovative privacy-preserving federated learning mechanism that integrates blockchain technology for secure and efficient data sharing. Unlike traditional methods relying on differential privacy, UBFL employs adaptive nonlinear encryption to safeguard the privacy of UAV model updates while ensuring data integrity and accuracy. Mo et al. [184] proposed a consortium blockchain (CB) and FL-based data management framework for medical network communities, enabling secure data sharing between medical institutions and research organizations. Wang et al. [185] proposed a blockchain-based federated learning approach for privacy-preserving sharing of non-IID data in the industrial internet. This approach dynamically updates local models to mitigate the accuracy degradation caused by non-IID data. Zhang et al. [186] proposed a blockchain-based hierarchical federated learning framework designed to enhance the training of non-IID data, improve data privacy protection, and optimize federated learning performance. 

Myrzashova et al. [187] introduced an innovative framework leveraging blockchainbased federated learning to identify 15 different lung diseases while ensuring data privacy and security. Rabbani et al. [188] proposed an FL framework integrating multiple machine learning models to prevent fraudulent transactions in blockchain systems while preserving customer privacy, eliminating the need for private customer data exchange between institutions. Meng et al. [189] proposed a blockchain-based machine learning scheme for secure data sharing in distributed energy storage networks, incorporating secure federated learning to formulate data sharing as a machine learning problem. Zhu et al. [190] proposed an innovative framework that integrates Trusted Execution Environment and blockchain technology to address data security and privacy challenges in IoT supply chain management. Jia et al. [191] proposed a multi-task privacy-preserving FL framework using partitioned blockchain, enabling multiple requesters to execute different FL tasks concurrently. Zhang et al. [192] proposed a blockchain-based pseudorandom number generation (BPNG) protocol using verifiable random functions (VRFs) to ensure fairness in federated learning schemes. Dong et al. [193] proposed a benchmark evaluation approach to optimize federated person re-identification performance in blockchain-powered edge-intelligent UAV delivery systems. Dhasaratha et al. [194] integrated blockchain-reinforced federated learning for post-COVID-19 patient data in IoMT applications to enhance data privacy and security. Li et al. [195] modeled multi-source devices using federated learning and stored model parameters and reputation values on the blockchain to enhance training accuracy and ensure data privacy security. 

As shown in Tables 24 and 25, this research area explores blockchain-native privacypreserving mechanisms that leverage zero-knowledge proofs, access control encryption, and knowledge graphs to enable decentralized data sharing, financial credit modeling, and secure transactions without relying on federated learning. These techniques emphasize decentralization, tamper-proof transparency, and verifiable transactions while addressing data privacy through cryptographic methods. However, practical deployment faces challenges due to high computational and storage overheads, particularly requiring strong computing power nodes. Complementary AI-driven approaches integrating zero-knowledge proofs, homomorphic encryption, and ring signatures aim to enhance privacy in blockchain 

_Algorithms_ **2025** , _18_ , 263 

52 of 67 

systems yet suffer from increased computational complexity that limits applicability on resource-constrained devices. The current integration of AI with these privacy mechanisms often remains loosely coupled, missing deeper synergistic optimizations. Future directions focus on developing lightweight AI models specifically optimized for accelerating ZKP/FHE computations and leveraging AI-generated constraints within zero-knowledge proof systems. Such advancements could potentially reconcile computational efficiency with robust privacy guarantees in decentralized blockchain environments, addressing both the technical limitations of existing cryptographic methods and the underexplored potential of tighter AI-privacy mechanism integration. 

**Table 24.** Blockchain-integrated FL for privacy protection. 

|**Research ID**|**Title**|**Key Technologies**|**Application**<br>**Domain**|**Major**<br>**Contribution**|**Limitations**|
|---|---|---|---|---|---|
||Privacy Protection|||||
|Wan et al. [178]|in Federated<br>Learning<br>Combining<br>Distillation<br>Defense and|Blockchain, FL,<br>Distillation<br>Defense|General|Reduces client<br>sensitivity to<br>perturbations,<br>enhances security|High<br>computational cost<br>for distillation|
||Blockchain|||||
||BFG: Privacy<br>Protection|Blockchain, FL,||Prevents poisoning|Increased storage|
|Liu et al. [179]|Framework for|Differential|Healthcare|attacks, enhances|overhead due to|
||Internet of Medical<br>Things|Privacy, GAN||privacy protection|blockchain|
|Ma et al.<br>proposed [180]|Blockchain-Based<br>Privacy-<br>Preserving FL for<br>Mobile<br>Crowdsourcing|Blockchain, FL,<br>Chinese<br>Remainder<br>Theorem|Mobile<br>Crowdsourcing|Reduces<br>computation and<br>communication<br>costs, improves<br>accuracy|Vulnerable to<br>model poisoning<br>attacks|
|Wang et al. [181]|Blockchain-Based<br>Privacy-<br>Preserving FL for<br>IoV|Blockchain, FL,<br>Multi-Krum,<br>Homomorphic<br>Encryption|IoV|Ensures local<br>model authenticity,<br>encourages<br>participation|Homomorphic<br>encryption<br>increases<br>computational<br>burden|
|Zhao et al. [182]|Privacy-<br>Preserving<br>Blockchain-Based<br>FL for IoT|Blockchain, FL,<br>Reputation<br>Mechanism|IoT|Enhances<br>decentralized<br>privacy protection<br>and secure data<br>sharing|Scalability issues<br>with reputation<br>mechanism|
|Zhu et al. [183]|Privacy Protection<br>FL for UAV-MEC<br>Networks|Blockchain, FL,<br>Nonlinear<br>Encryption, RCF<br>Detection|UAV<br>Communication|Protects UAV data<br>privacy, reduces<br>data pollution<br>impact|Requires strong<br>computational<br>resources for<br>encryption|
|Mo et al. [184]|Privacy Data<br>Management<br>Mechanism Based<br>on Blockchain and<br>FL|Blockchain, FL,<br>Smart Contracts|Healthcare|Ensures secure<br>data sharing<br>among medical<br>institutions|High complexity in<br>managing smart<br>contracts|
||Blockchain-|||||
|Wang et al. [185]|Enabled FL for<br>Privacy-<br>Preserving<br>Non-IID Data<br>Sharing|Blockchain, FL,<br>Dynamic Gradient<br>Clipping|Industrial Internet|Addresses non-IID<br>data issue,<br>improves training<br>accuracy|Potential privacy<br>leakage in gradient<br>sharing|
|Zhang et al. [186]|Secure and<br>Decentralized FL<br>for Non-IID Data|Blockchain, FL,<br>Hierarchical<br>Shared Pool|Distributed<br>Computing|Improves accuracy<br>of FL with non-IID<br>data|High<br>communication<br>cost due to model<br>synchronization|



_Algorithms_ **2025** , _18_ , 263 

53 of 67 

**Table 25.** Blockchain-integrated federated learning for privacy protection. 

|**Research ID**|**Research Title**|**Key**<br>**Technologies**|**Application**<br>**Scenario**|**Main**<br>**Contributions**|**Limitations**|
|---|---|---|---|---|---|
|||||Improves||
|Myrzashova<br>et al. [187]|Blockchain-<br>Enabled FL for<br>Medical Data<br>Sharing|Blockchain, FL|Healthcare|privacy in<br>medical data<br>sharing,<br>supports disease|High latency in<br>blockchain<br>transactions|
|||||prediction||
||Enhancing|||||
|Rabbani<br>et al. [188]|Security in<br>Financial<br>Transactions<br>with Blockchain-|Blockchain, FL,<br>Multiple ML<br>Models|Finance|Detects<br>fraudulent<br>transactions,<br>enhances privacy|Lack of<br>real-world<br>deployment and<br>testing|
||Based FL|||||
|Meng et al. [189]|SFedChain:<br>Secure Data<br>Sharing in<br>Energy Storage<br>Networks|Blockchain, FL,<br>Secure<br>Computing|Energy|Protects<br>distributed<br>energy data<br>privacy|Increased<br>computational<br>and storage<br>demands|
|Zhu et al. [190]|Enhancing IoT<br>Supply Chain<br>Security with<br>Blockchain and<br>FL|Blockchain, FL,<br>Trusted<br>Execution<br>Environment|Supply Chain|Ensures secure<br>computation and<br>tamper-resistant<br>data|Limited<br>scalability due to<br>TEE constraints|
|Jia et al. [191]|Blockchain-<br>Based Multi-Task<br>Privacy-<br>Preserving FL<br>Framework|Blockchain, FL,<br>Partitioned<br>Blockchain|Industrial<br>Applications|Supports<br>multiple FL<br>tasks, improves<br>privacy|Complexity in<br>multi-task model<br>coordination|
|Zhang et al. [192]|Practical and<br>Privacy-<br>Preserving FL<br>with Verifable<br>Fairness|Blockchain, FL,<br>VRF,<br>Zero-Knowledge<br>Proof|General|Ensures fairness<br>in FL training,<br>enhances privacy|High verifcation<br>cost in<br>real-world<br>implementation|
|Dong et al. [193]|Optimizing<br>Federated<br>Person Re-ID in<br>Smart UAV<br>DeliverySystems|Blockchain, FL,<br>Decentralized<br>Mechanism|UAV Delivery|Prevents single<br>points of failure,<br>optimizes data<br>privacy|Limited<br>effectiveness in<br>dynamic<br>network<br>environments|
|Dhasaratha<br>et al. [194]|Blockchain-<br>Reinforced FL<br>for Scalable<br>IoMT Privacy<br>Protection|Blockchain,<br>Reinforcement<br>FL|Healthcare IoMT|Improves data<br>security and<br>communication<br>effciency|Needs further<br>validation in<br>large-scale IoMT<br>systems|
||Blockchain-|||Enhances||
|Li et al. [195]|Integrated FL for<br>Multi-Device<br>Privacy|Blockchain, FL,<br>Reputation<br>Mechanism|Multi-Device<br>Communication|federated<br>learning trust<br>and privacy|Reputation<br>mechanisms may<br>introduce biases|
||Protection|||security||



_Algorithms_ **2025** , _18_ , 263 

54 of 67 

## _3.4. AI-Driven Data Storage and Retrieval_ 

In the face of the growing challenge of massive data, traditional blockchain systems have obvious deficiencies in storage efficiency and retrieval performance, especially when processing large-scale, unstructured data, which is costly, and slow to write, and has large query latency, limiting its practical application in the fields of IoT, medical care, and supply chain. To address this problem, the current mainstream solution is to combine blockchain with IPFS (Interplanetary File System) to build a hybrid architecture of “on-chain index + off-chain storage”: blockchain is responsible for recording key business process data (such as production, processing, warehousing, sales, etc.) to ensure data traceability and transparency, while IPFS is dedicated to storing large files (such as quality inspection reports, medical images, etc.) and establishes a mapping link with the blockchain through hash values to ensure data integrity and immutability. The schematic diagram of combining blockchain and IPFS to store traceability data is shown in Figure 10. AI technology further improves the intelligence and efficiency of this system: in terms of storage, AI can predict data popularity and realize intelligent scheduling and dynamic hierarchical management of node resources; in terms of retrieval, with the help of deep learning, NLP, multimodal recognition and other technologies, it supports more efficient semantic retrieval and multisource information query; at the same time, it also introduces quantum-resistant hashing and privacy protection algorithms to enhance security and sustainability. Overall, the integration of AI with blockchain and IPFS not only solves the performance bottleneck but also promotes the construction of a decentralized, highly reliable and highly collaborative data system, providing strong support for the digital transformation of multiple industries. 

**Figure 10.** Blockchain and IPFS combine to solve storage burden. 

With the upgraded iteration of AI technology, the combination of AI and blockchain technology promotes the efficiency of data retrieval and storage on blockchain. Research has been carried out in this area both domestically and internationally. Cai et al. [196] introduced EncELC, a framework designed for ethereum light clients to enhance client-side protection and support rich queries on the ethereum blockchain [197]. This approach aims 

_Algorithms_ **2025** , _18_ , 263 

55 of 67 

to improve the security and functionality of light clients accessing blockchain data [198]. Khan et al. [199] proposed TREAD, a system for storing and retrieving vehicle-generated sensitive mobile data on the blockchain. By utilizing the Interplanetary File System (IPFS), TREAD addresses scalability challenges while protecting user privacy when handling large amounts of data. In the context of supply chain management, Yiu et al. [200] developed a decentralized anti-counterfeiting system using blockchain technology to ensure authenticity and traceability of physical goods. The system facilitates the retrieval and validation of trusted data sources, thereby improving the security of supply chain operations. Almaiah et al. [201] proposed a hybrid digital healthcare authentication model integrating artificial intelligence and blockchain technologies. This model addresses the authentication and data privacy challenges in smart healthcare systems, emphasizing on security and privacy in data management [202]. Singh et al. [203] introduced a smart enterprise trust management scheme using blockchain technology. By leveraging enhanced intelligence, the scheme improves trust and security in enterprise operations, demonstrating better performance than existing approaches. Wang et al. [204] introduced a hierarchical trust evaluation strategy for 5G-enabled Intelligent Transport Systems (ITS) using a heterogeneous blockchain approach. This strategy employs joint deep learning techniques to ensure reliable and low-latency communication in ITS applications. Finally, Khan et al. [205] developed a secure searchable cryptographic framework for medical IoT systems using blockchain technology. The framework integrates Binary Spring Search techniques with a hybrid deep neural network approach to enhance the security and reliability of medical data storage and retrieval. 

As shown in Table 26, a comparative analysis of AI-driven blockchain-based data retrieval and storage approaches covering areas such as healthcare, smart transportation, and enterprise trust management found that technologies such as federated deep learning, augmented intelligence, and hybrid neural networks are highly integrated, thereby enhancing security, privacy, and efficiency. These systems have leveraged various blockchain platforms, including heterogeneous architectures and IoT-integrated architectures, but still face persistent challenges. Scalability remains a key constraint due to the computational requirements of AI models and blockchain storage overhead, while interoperability issues hinder seamless data exchange. Notably, current solutions mainly deal with structured data, while semantic indexing for unstructured data types such as images, audio, and video is still imperfect. There are also gaps in IPFS integration, lacking intelligent mechanisms for hot spot prediction and dynamic data scheduling, resulting in storage redundancy. To advance the field, future research should focus on multimodal frameworks that combine large language models with CLIP for cross-modal semantic indexing, AI-driven algorithms to optimize hot spot data placement in decentralized storage, and knowledge graph-enhanced, context-aware retrieval systems to improve efficiency and user experience. 

**Table 26.** Comparison of AI-driven blockchain-based data retrieval and storage approaches. 

|**Study**|**Application**<br>**Domain**|**Blockchain**<br>**Platform**|**AI Techniques**<br>**Used**|**Privacy/Security**<br>**Mechanisms**|**Main**<br>**Contributions**|
|---|---|---|---|---|---|
|Almaiah<br>et al. [201]|Digital Healthcare|Blockchain + IoT|AI-based<br>authentication|Lightweight<br>encryption for IoT<br>security|Provides a hybrid<br>authentication<br>model for<br>healthcare.|
||||||Improves|
|Singh et al. [203]|Smart Enterprises|Blockchain|Augmented<br>Intelligence|Blockchain-based<br>trust management|enterprise security<br>and trust|
||||||management.|



_Algorithms_ **2025** , _18_ , 263 

56 of 67 

**Table 26.** _Cont._ 

|**Study**|**Application**<br>**Domain**|**Blockchain**<br>**Platform**|**AI Techniques**<br>**Used**|**Privacy/Security**<br>**Mechanisms**|**Main**<br>**Contributions**|
|---|---|---|---|---|---|
||||||Ensures|
|Wang et al. [204]|Intelligent<br>Transportation|Heterogeneous<br>Blockchain|Federated Deep<br>Learning|Hierarchical trust<br>evaluation|low-latency and<br>reliable<br>communication in|
||||||ITS.|
||||||Enhances security|
|Khan et al. [205]|Medical IoT<br>Systems|Blockchain|Hybrid Deep<br>Neural Networks|Secure Searchable<br>Encryption (Binary<br>Spring Search)|and reliability in<br>medical data<br>storage and|
||||||retrieval.|



## **4. Discussion and Future Directions** 

With the continuous development of technology, blockchain is widely used in various fields due to its decentralization, data integrity, traceability, and anonymity. However, blockchain still faces many challenges: (1) consensus algorithms have scalability bottlenecks (e.g., PoW low throughput and high energy consumption) and centralization risks (e.g., PoS node monopoly); (2) smart contracts have frequent security vulnerabilities due to untamperable code (e.g., re-entry attacks) and high development thresholds and upgrading difficulties; (3) privacy protection is limited by the conflict between transparency and privacy (e.g., public chain data exposure), insufficient technical performance (e.g., high ZKP latency), and facing compliance conflicts; (4) data retrieval is limited by high on-chain storage costs, inefficient querying (requiring full-chain scanning), and weak unstructured data processing capabilities. This paper discusses and looks at each of the following four modules. 

This paper summarizes the research on the core modules of AI-driven blockchain. Specifically, the machine learning or deep learning used drives the blockchain consensus algorithm, smart contracts, privacy protection, and data upload and retrieval. Table 27 shows the models in AI used by the relevant researchers summarized in this article to improve and apply different modules of blockchain. From the two major categories of machine learning and deep learning, the training types, algorithms/architectures and their application scenarios in blockchain are sorted out. In the field of machine learning: supervised learning uses algorithms such as decision trees and Support Vector Machines for smart contract vulnerability detection and node classification; unsupervised learning uses methods such as K-Means to achieve blockchain data clustering and transaction pattern mining; semi-supervised learning uses self-training and other methods to help privacy medical data sharing and other analysis tasks with limited labeled data; reinforcement learning uses algorithms such as Q-Learning to optimize consensus mechanisms and miner task offloading. In the field of deep learning, Convolutional Neural Networks such as LeNet can detect smart contract vulnerabilities and process image/video data stored on the chain; Recurrent Neural Networks and their variants (LSTM, etc.) are used for contract multi-label vulnerability prediction and transaction time series analysis; Transformer-based models (BERT, etc.) can perform tasks such as smart contract language modeling that combine blockchain and natural language processing; generative adversarial networks (DCGAN, etc.) play a role in privacy protection, data generation, etc. 

_Algorithms_ **2025** , _18_ , 263 

57 of 67 

**Table 27.** Classification of training methods and their application scenarios in blockchain systems. 

|**Category**|**Training Type**|**Algorithms/Architectures**|**Blockchain Application**<br>**Scenarios**|
|---|---|---|---|
||||Smart contract vulnerability|
||Supervised Learning|Decision Tree, SVM, Random<br>Forest, XGBoost|detection, node classifcation<br>(e.g., XG-PBFT uses XGBoost|
||||for node grouping)|
|Machine Learning|Unsupervised Learning|K-Means, DBSCAN, PCA,<br>AutoEncoder|Blockchain data clustering,<br>address behavior clustering,<br>transaction pattern discovery<br>Privacy-preserving medical|
||Semi-Supervised Learning|Self-training, Co-training, Label<br>Propagation|data sharing, transaction record<br>analysis, classifcation with|
||||limited labeled data|
||||Consensus optimization, miner|
||Reinforcement Learning|Q-Learning, DQN, DDPG, PPO|task offoading, block selection<br>strategies, on-chain resource|
||||scheduling|
||||Smart contract vulnerability|
||CNN|LeNet, AlexNet, ResNet,<br>1D/2D CNN|detection (e.g., CodeNet),<br>image/video pre-processing for<br>on-chain storage, multimodal|
||||data indexing|
||||Multi-label vulnerability|
|Deep Learning|RNN and Variants|LSTM, GRU, Bi-LSTM|prediction in contracts,<br>time-series analysis (e.g.,|
||||transaction forecasting)|
||||Smart contract language|
||Transformer-based Models|BERT, GPT, CodeBERT, T5, ViT|modeling, code classifcation,<br>blockchain + NLP (e.g., contract|
||||generation, audit tasks)|
||||Privacy protection, synthetic|
||GANs|DCGAN, WGAN, cGAN,<br>CycleGAN|data generation, adversarial<br>training, identity|
||||anonymization in data sharing|



## _4.1. General Discussion and Future Prospects_ 

Although the related studies summarized in this article have achieved good results in different application fields, there are still some challenges. This article discusses and studies the following four modules, respectively. 

(1) Consensus algorithms 

Existing research focuses on AI optimization of traditional consensus mechanisms (e.g., PoW and PoS) to classify nodes, detect malicious behaviours through machine learning (e.g., XG-PBFT and LSTM secure consensus), and enable low-latency authentication in IoT and edge computing (e.g., PoAh 2.0). However, existing schemes are not versatile enough and partially rely on high computing power. Future directions include dynamic adaptive consensus (RL adjusts parameters in real time), cross-chain AI co-optimization, green energy-saving consensus (AI-optimized PoS/PoW hybrid mechanism), and AI-enhanced cryptography design for quantum attack resistance. 

## (2) Smart Contracts 

Existing research mainly uses AI for vulnerability detection (static/dynamic analysis, deep learning models such as CBGRU), automated generation (NLP-assisted development), 

_Algorithms_ **2025** , _18_ , 263 

58 of 67 

and scenario-based applications (e.g., supply chain traceability). However, complex logic vulnerability detection is still insufficient, and code interpretability is limited. Future directions cover full lifecycle AI (LLM generation of secure contracts and federated learning to protect training data), formal validation enhancement (AI generation of mathematical proofs), and AI-coordinated execution of cross-chain smart contracts to ensure atomicity and consistency. 

## (3) Privacy protection 

Existing research combines FL with blockchain (e.g., the BPFL framework), zeroknowledge proofs, and TEEs to achieve data privacy, but suffers from loss of precision (DP noise) and hardware dependency. Future directions include lightweight privacy computing (edge FL + hybrid cryptography), verifiable privacy (AI-optimized zero-knowledge proof generation), and data sovereignty management (NFTed ownership + dynamic access control) to balance privacy and utility. 

In addition, in the context of data privacy, compliance with the General Data Protection Regulation (GDPR) is a critical requirement, particularly for systems involving the processing of personal data in the European Union. GDPR enforces principles such as data minimization, user consent, the right to access and erase data (“right to be forgotten”), and accountability in automated decision-making. While blockchain provides strong data integrity and transparency, its immutable nature presents challenges in achieving GDPR compliance, especially with respect to data erasure and modification. However, the integration of AI with privacy-preserving techniques such as differential privacy, federated learning, and zero-knowledge proofs can mitigate these risks. Furthermore, hybrid blockchain architectures—where sensitive data are stored off-chain while hashes are stored on-chain—can help reconcile the tension between immutability and data protection rights. Thus, the design of AI-powered blockchain systems must carefully balance innovation with regulatory compliance to ensure lawful and ethical data usage. 

## (4) Data Retrieval 

Existing research relies on blockchain + IPFS storage (e.g., TREAD system) with AIoptimized indexing (Merkle tree improvement) and semantic search (NLP parsed query), but unstructured data retrieval is inefficient. Future directions involve multimodal retrieval (CNN processing of image/video data and LLM natural language interaction), distributed storage optimization (AI prediction of data heat to adjust IPFS nodes), and quantumsafe retrieval (AI-assisted anti-quantum hash algorithms) to improve large-scale data query performance. 

## _4.2. Discussion of Open Issues and Future Prospects_ 

Despite the progress achieved, AI-driven blockchain systems face several unresolved challenges, including the lack of AI transparency and explainability, deployment difficulties due to resource constraints, and new security vulnerabilities such as adversarial attacks and data poisoning. Furthermore, standardization and interoperability across platforms remain insufficient, while privacy protection must carefully balance compliance with data regulations like GDPR. Ethical and governance issues, such as accountability for AI decisions in decentralized environments, also remain open. Addressing these challenges is crucial to ensuring that AI-empowered blockchain systems are robust, trustworthy, and widely adoptable. 

Looking ahead, promising future research directions include developing trustworthy and explainable AI models tailored for blockchain applications, integrating federated learning with verifiable privacy techniques, and leveraging large language models for smart contract generation and auditing. Advances in generative AI can enhance blockchain 

_Algorithms_ **2025** , _18_ , 263 

59 of 67 

security through synthetic adversarial testing, while AI-optimized post-quantum cryptography and green, energy-efficient consensus mechanisms will be vital for long-term sustainability. Additionally, cross-modal AI systems capable of processing multimodal data streams can open new possibilities for autonomous, intelligent decentralized applications across diverse industries. 

## **5. Conclusions** 

Blockchain still faces numerous challenges in terms of scalability, security, and privacy protection. AI, with its powerful data processing capabilities, pattern recognition, and adaptive optimization algorithms, has the potential to enhance blockchain transaction processing efficiency, strengthen security mechanisms, and optimize privacy protection strategies. Consequently, AI can effectively mitigate the limitations of blockchain in scalability and security. However, existing reviews on AI and blockchain integration primarily focus on macro-level discussions, lacking in-depth analysis and classification. Therefore, this paper focuses on the application of AI in blockchain, providing a comprehensive review of its optimization potential in scalability, security, and privacy protection. 

This study examines the current state and key challenges of four core modules: consensus algorithms, smart contracts, privacy protection, and data retrieval. Furthermore, it categorizes and discusses the progress of AI-driven blockchain research in scalability, security, and privacy protection. Findings indicate that AI can enhance consensus efficiency by dynamically optimizing the selection of optimal nodes for participation. Additionally, AI can be leveraged for resource allocation prediction, malicious behavior detection, and vulnerability identification, thereby improving blockchain security. Moreover, the integration of AI-driven federated learning with blockchain facilitates decentralized data collaboration, while AI-optimized Merkle tree indexing and distributed query techniques significantly enhance data retrieval efficiency. Finally, this paper explores future research directions for AI-driven blockchain advancements. Energy consumption and security remain core issues in this field. Future studies should focus on optimizing consensus algorithms, enhancing smart contract security, improving privacy protection mechanisms, and increasing data retrieval efficiency. Additionally, further exploration is needed in decentralized AI training, privacy-preserving computation, and computing power sharing to enhance blockchain’s scalability, security, and energy efficiency. 

**Author Contributions:** F.Y.: Writing—Original Draft, Conceptualization, and Writing—Review and Editing; Z.Z.: Conceptualization and Methodology; Y.J.: Methodology and Data Curation; W.S.: Investigation and Formal Analysis; Z.T.: Formal Analysis and Visualization; C.Y.: Resources and Visualization; J.Y.: Formal Analysis and Data Curation; Z.M.: Writing—Review and Editing; X.H.: Funding Acquisition and Methodology; S.G.: Investigation and Data Curation; Y.P.: Supervision, Project Administration, and Funding Acquisition. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This paper was supported by the Innovative Research Group of Chongqing Municipal Education Commission (CXQT19026) and the Cooperative Project between Chinese Academy of Sciences and University in Chongqing (HZ2021011). Moreover, this work was supported by the Research Startup Fund of Chongqing University of Technology (0119240197). 

**Informed Consent Statement:** Informed consent was obtained from all subjects involved in the study. 

**Data Availability Statement:** The authors declare that the article is a review article and does not relate to the dataset. 

**Acknowledgments:** This work was supported by equipment funded through the “Intelligent Connected New Energy Vehicle Teaching System” project of Chongqing University of Technology, under the national initiative “Promote large-scale equipment renewals and trade-ins of consumer goods”. 

_Algorithms_ **2025** , _18_ , 263 

60 of 67 

**Conflicts of Interest:** The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **References** 

1. Yi, J.; Wang, J.; Tan, L.; Yuan, T. HMM-Based Blockchain Visual Automatic Deployment System. _Appl. Sci._ **2024** , _14_ , 5722. [CrossRef] 

2. Bouoiyour, J.; Selmi, R. Bitcoin: A beginning of a new phase. _Econ. Bull._ **2016** , _36_ , 1430–1440. 

3. Nakamoto, S. Bitcoin Technology: A Peer-to-Peer Digital Cash Transaction. _Int. J._ **2016** , _6_ . 

4. Swan, M. _Blockchain: Blueprint for a New Economy_ ; O’Reilly Media, Inc.: Sebastopol, CA, USA, 2015. 

5. Ciaian, P.; Rajcaniova, M.; Kancs, D. The economics of BitCoin price formation. _Appl. Econ._ **2016** , _48_ , 1799–1815. [CrossRef] 

6. Al-Breiki, H.; Rehman, M.H.U.; Salah, K.; Svetinovic, D. Trustworthy blockchain oracles: Review, comparison, and open research challenges. _IEEE Access_ **2020** , _8_ , 85675–85685. [CrossRef] 

7. Kotey, S.D.; Tchao, E.T.; Ahmed, A.R.; Agbemenu, A.S.; Nunoo-Mensah, H.; Sikora, A.; Welte, D.; Keelson, E. Blockchain interoperability: The state of heterogenous blockchain-to-blockchain communication. _IET Commun._ **2023** , _17_ , 891–914. [CrossRef] 

8. Reyna, A.; Martín, C.; Chen, J.; Soler, E.; Díaz, M. On blockchain and its integration with IoT. Challenges and opportunities _Future Gener. Comput. Syst._ **2018** , _88_ , 173–190. [CrossRef] 

9. Anagnostakis, A.G.; Giannakeas, N.; Tsipouras, M.G.; Glavas, E.; Tzallas, A.T. IoT Micro-Blockchain Fundamentals. _Sensors_ **2021** , _21_ , 2784. [CrossRef] 

10. Hristova, T. Tracking mining company electric vehicles for sustainable development optimization using Distributed Ledger Technologies. _J. Sustain. Min._ **2024** , _23_ , 299–314. [CrossRef] 

11. Hristova, T.; Mitev, I.; Balev, V. _Monitoring of Geotechnical Facilities Through DLT Solution_ ; IOP Conference Series: Earth and Environmental Science; IOP Publishing: Bristol, UK, 2022; Volume 970, p. 012011. 

12. Li, X.; Zheng, Z.; Dai, H.N. When services computing meets blockchain: Challenges and opportunities. _J. Parallel Distrib. Comput._ **2021** , _150_ , 1–14. [CrossRef] 

13. Treleaven, P.; Brown, R.G.; Yang, D. Blockchain technology in finance. _Computer_ **2017** , _55_ , 14–17. [CrossRef] 

14. Johar, S.; Ahmad, N.; Asher, W.; Cruickshank, H.; Durrani, A. Research and Applied Perspective to Blockchain Technology: A Comprehensive Survey. _Appl. Sci._ **2021** , _11_ , 6252. [CrossRef] 

15. Wang, X.; Ni, W.; Zha, X.; Yu, G.; Liu, R.P.; Georgalas, N.; Reeves, A. Capacity analysis of public blockchain. _Comput. Commun._ **2021** , _177_ , 112–124. [CrossRef] 

16. Iqbal, M.; Matulevicius, R. Exploring Sybil and Double-Spending Risks in Blockchain Systems. _IEEE Access_ **2021** , _9_ , 76153–76177. [CrossRef] 

17. He, Y.; Wang, Y.; Qiu, C.; Lin, Q.; Li, J.; Ming, Z. Blockchain-based edge computing resource allocation in IoT: A deep reinforcement learning approach. _IEEE Internet Things J._ **2020** , _8_ , 2226–2237. [CrossRef] 

18. Islam, A.; Rahim, T.; Masuduzzaman, M.; Shin, S.Y. A blockchain-based artificial intelligence-empowered contagious pandemic situation supervision scheme using internet of drone things. _IEEE Wirel. Commun._ **2021** , _28_ , 166–173. [CrossRef] 

19. D’souza, S.; Nazareth, D.; Vaz, C.; Shetty, M. Blockchain and AI in pharmaceutical supply chain. In Proceedings of the International Conference on Smart Data Intelligence (ICSMDI 2021), Tamil Nadu, India, 29–30 April 2021. 

20. Suhail, S.; Hussain, R.; Jurdak, R.; Oracevic, A.; Salah, K.; Hong, C.S.; Matuleviˇcius, R. Blockchain-based digital twins: Research trends, issues, and future challenges. _ACM Comput. Surv. (CSUR)_ **2022** , _54_ , 1–34. [CrossRef] 

21. Guergov, S.; Radwan, N. Blockchain convergence: Analysis of issues affecting IoT, AI and blockchain. _Int. J. Comput. Inf. Manuf. (IJCIM)_ **2021** , _1_ , 1–17. [CrossRef] 

22. Yang, Q.; Zhao, Y.; Huang, H.; Xiong, Z.; Kang, J.; Zheng, Z. Fusing blockchain and AI with metaverse: A survey. _IEEE Open J. Comput. Soc._ **2022** , _3_ , 122–136. [CrossRef] 

23. Kumar, S.; Lim, W.M.; Sivarajah, U.; Kaur, J. Artificial intelligence and blockchain integration in business: Trends from a bibliometric-content analysis. _Inf. Syst. Front._ **2023** , _25_ , 871–896. [CrossRef] 

24. Junaid, S.B.; Imam, A.A.; Balogun, A.O.; De Silva, L.C.; Surakat, Y.A.; Kumar, G.; Abdulkarim, M.; Shuaibu, A.N.; Garba, A.; Sahalu, Y.; et al. Recent advancements in emerging technologies for healthcare management systems: A survey. _Healthcare_ **2022** , _10_ , 1940. [CrossRef] [PubMed] 

25. Xu, H.; Wu, J.; Pan, Q.; Guan, X.; Guizani, M. A survey on digital twin for industrial internet of things: Applications, technologies and tools. _IEEE Commun. Surv. Tutor._ **2023** , _25_ , 2569–2598. [CrossRef] 

26. Verma, D.; Okhawilai, M.; Dalapati, G.K.; Ramakrishna, S.; Sharma, A.; Sonar, P.; Krishnamurthy, S.; Biring, S.; Sharma, M. Blockchain technology and AI-facilitated polymers recycling: Utilization, realities, and sustainability. _Polym. Compos._ **2022** , _43_ , 8587–8601. [CrossRef] 

27. Badidi, E. Edge AI and Blockchain for Smart Sustainable Cities: Promise and Potential. _Sustainability_ **2022** , _14_ , 7609. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

61 of 67 

28. Haddad, A.; Habaebi, M.H.; Islam, M.R.; Hasbullah, N.F.; Zabidi, S.A. Systematic Review on AI-Blockchain Based E-Healthcare Records Management Systems. _IEEE Access_ **2022** , _10_ , 94583–94615. [CrossRef] 

29. Kumar, R.; Singh, D.; Srinivasan, K.; Hu, Y.C. AI-Powered Blockchain Technology for Public Health: A Contemporary Review, Open Challenges, and Future Research Directions. _Healthcare_ **2023** , _11_ , 81. [CrossRef] 

30. Uddin, M.; Obaidat, M.; Manickam, S.; Laghari, S.U.A.; Dandoush, A.; Ullah, H.; Ullah, S.S. Exploring the convergence of Metaverse, Blockchain, and AI: A comprehensive survey of enabling technologies, applications, challenges, and future directions. _Wiley Interdiscip. Rev.-Data Min. Knowl. Discov._ **2024** , _14_ , e1556. [CrossRef] 

31. Omidian, H. Synergizing blockchain and artificial intelligence to enhance healthcare. _Drug Discov. Today_ **2024** , _29_ , 104111. [CrossRef] 

32. Zhou, Q.; Zhang, H.; Wang, S. Artificial intelligence, big data, and blockchain in food safety. _Int. J. Food Eng._ **2022** , _18_ , 1–14. [CrossRef] 

33. Yao, Y.; Shi, Y.; Tian, G.; Miao, M.; Susilo, W. PSCBO: A provably secure consensus-based blockchain Oracle. _Comput. Stand. Interfaces_ **2025** , _91_ , 103892. [CrossRef] 

34. Maldonado-Ruiz, D.; Pulval-Dady, A.; Shi, Y.; Wang, Z.; El Madhoun, N.; Torres, J. NestedChain: “Blockchain-inside-a-Blockchain” new generation prototype. _Ann. Telecommun._ **2024** , _79_ , 881–899. [CrossRef] 

35. Pasdar, A.; Lee, Y.C.; Dong, Z. Connect API with Blockchain: A Survey on Blockchain Oracle Implementation. _ACM Comput. Surv._ **2023** , _55_ , 1–39. [CrossRef] 

36. Wang, X.; Jiang, X.; Liu, Y.; Wang, J.; Sun, Y. Data Propagation for Low Latency Blockchain Systems. _IEEE J. Sel. Areas Commun._ **2022** , _40_ , 3631–3644. [CrossRef] 

37. Aki. Digital signatures: A tutorial survey. _Computer_ **1983** , _16_ , 15–24. 38. Aruna, S.; Maheswari, M.; Saranya, A. _Highly Secured Blockchain Based Electronic Voting System Using sha3 and Merkle Root_ ; IOP Conference Series: Materials Science and Engineering; IOP Publishing: Bristol, UK, 2020; Volume 993, p. 012103. 

39. Mahmud, M.; Sohan, M.S.H.; Reno, S.; Sikder, M.A.B.; Hossain, F.S. Advancements in scalability of blockchain infrastructure through IPFS and dual blockchain methodology. _J. Supercomput._ **2024** , _80_ , 8383–8405. [CrossRef] 

40. Iyengar, G.; Saleh, F.; Sethuraman, J.; Wang, W. Economics of Permissioned Blockchain Adoption. _Manag. Sci._ **2023** , _69_ , 3415–3436. [CrossRef] 

41. Liu, Y.; Jia, L.; Wang, X.; Huang, H.; Zhao, Q.; Li, Z.; Sun, Y. CORAL: A blockchain protocol for handling transactions with deadline constraints. _Comput. Netw._ **2024** , _251_ , 110620. [CrossRef] 

42. Heo, J.W.; Ramachandran, G.S.; Dorri, A.; Jurdak, R. Blockchain Data Storage Optimisations: A Comprehensive Survey. _ACM Comput. Surv._ **2024** , _56_ , 1–27. [CrossRef] 

43. Hong, L.; Hales, D.N. Blockchain performance in supply chain management: Application in blockchain integration companies. _Ind. Manag. Data Syst._ **2021** , _121_ , 1969–1996. [CrossRef] 

44. Duan, C.; Jiang, R.; Zhang, Y.; Wu, B.; Li, F.; Duan, Y. Distributed medical data storage model based on blockchain technology. _Clust. Comput._ **2024** , _27_ , 4757–4777. [CrossRef] 

45. Song, J.; Zhang, P.; Qu, Q.; Bai, Y.; Gu, Y.; Yu, G. Why blockchain needs graph: A survey on studies, scenarios, and solutions. _J. Parallel Distrib. Comput._ **2023** , _180_ , 104730. [CrossRef] 

46. Liang, W.; Zhang, D.; Lei, X.; Tang, M.; Li, K.C.; Zomaya, A.Y. Circuit Copyright Blockchain: Blockchain-Based Homomorphic Encryption for IP Circuit Protection. _IEEE Trans. Emerg. Top. Comput._ **2021** , _9_ , 1410–1420. [CrossRef] 

47. Duan, J.; Gu, L.; Wang, W.; Wang, L. Chainknot: Redactable Blockchain Protocol with Forced Synchronization. _IEEE Trans. Netw. Sci. Eng._ **2024** , _11_ , 3091–3104. [CrossRef] 

48. Rajendra, Y.; Sahu, S.; Subramanian, V.; Shukla, S.K. Storage efficient blockchain models for constrained applications. _Clust. Comput._ **2023** , _26_ , 2163–2181. [CrossRef] 

49. Connors, C.; Sarkar, D. Survey of prominent blockchain development platforms. _J. Netw. Comput. Appl._ **2023** , _216_ , 103650. [CrossRef] 

50. Choi, T.M.; Siqin, T. Blockchain in logistics and production from Blockchain 1.0 to Blockchain 5.0: An intra-inter-organizational framework. _Transp. Res. Part E-Logist. Transp. Rev._ **2022** , _160_ , 102653. [CrossRef] 

51. Rakshit, S.; Jeyaraj, A.; Paul, T. SME Performance Through Blockchain Technologies. _J. Comput. Inf. Syst._ **2024** , _64_ , 204–218. [CrossRef] 

52. Wang, Y.; Wang, W.; Zeng, Y.; Yang, T. GradingShard: A new sharding protocol to improve blockchain throughput. _Peer-to-Peer Netw. Appl._ **2023** , _16_ , 1327–1339. [CrossRef] 

53. Xu, S.; Ning, J.; Ma, J.; Huang, X.; Deng, R.H. _K_ -Time Modifiable and Epoch-Based Redactable Blockchain. _IEEE Trans. Inf. Forensics Secur._ **2021** , _16_ , 4507–4520. [CrossRef] 

54. Qi, N.; Yuan, Y.; Wang, F.Y. DAG-BLOCK: A Novel Architecture for Scaling Blockchain-Enabled Cryptocurrencies. _IEEE Trans. Comput. Soc. Syst._ **2024** , _11_ , 378–388. [CrossRef] 

55. Noam, O.; Rottenstreich, O. Realizing privacy aspects in blockchain networks. _Ann. Telecommun._ **2022** , _77_ , 3–12. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

62 of 67 

|56.|Wang, Y.X.; Hsueh, Y.L. A low-storage synchronization framework for blockchain systems. _J. Netw. Comput. Appl._ **2024**,_231_,|
|---|---|
||103977. [CrossRef]|
|57.|Wan, J.; Hu, K.; Li, J.; Su, H. AnonymousFox: An Effcient and Scalable Blockchain Consensus Algorithm. _IEEE Internet Things J._|
||**2022**,_9_, 24236–24252. [CrossRef]|
|58.|Stoyanov, I.; Iliev, T.; Mihaylov, G.; Evstatiev, B.; Sokolov, S. Analysis of the cybersecurity threats in smart grid University of|
||telecommunications and post, Sofa, Bulgaria. In Proceedings of the 2018 IEEE 24th International Symposium for Design and|
||Technology in Electronic Packaging(SIITME), Iasi, Romania, 25–28 October 2018; IEEE: Piscataway, NJ, USA, 2018; pp. 90–93.|
|59.|Bikov, T.D.; Iliev, T.B.; Mihaylov, G.Y.; Stoyanov, I.S. Phishing in depth—Modern methods of detection and risk mitigation.|
||In Proceedings of the 2019 42nd International Convention on Information and Communication Technology, Electronics and|
||Microelectronics (MIPRO), Opatija, Croatia, 20–24 May 2019; IEEE: Piscataway, NJ, USA, 2019; pp. 447–450.|
|60.|Valchanov, H.; Edikyan, J.; Aleksieva, V. _A Study of Wi-Fi Security in City Environment_; IOP Conference Series: Materials Science|
||and Engineering; IOP Publishing: Bristol, UK, 2019; Volume 618, p. 012031.|
|61.|Alam, S.; Shuaib, M.; Khan, W.Z.; Garg, S.; Kaddoum, G.; Hossain, M.S.; Bin Zikria, Y. Blockchain-based Initiatives: Current state|
||and challenges. _Comput. Netw._ **2021**,_198_, 108395. [CrossRef]|
|62.|Xu, L.; Tian, T. Blockchain-enabled enterprise bleaching green regulation banking evolution game analysis. _Environ. Dev. Sustain._|
||**2024**,_26_, 27457–27483. [CrossRef]|
|63.|Khossravi, A.S.; Chen, Q.; Adelman, R.A. Artifcial intelligence in ophthalmology. _Curr. Opin. Ophthalmol._ **2025**, _36_, 35–38.|
||[CrossRef]|
|64.|Galiana, I.; Gudino, L.C.; Gonzalez, P.M. Ethics and artifcial intelligence. _Rev. Clin. Esp._ **2024**,_224_, 178–186. [CrossRef]|
|65.|Tsang, Y.P.; Lee, C.K.M. Artifcial intelligence in industrial design: A semi-automated literature survey. _Eng. Appl. Artif. Intell._|
||**2022**,_112_, 104884. [CrossRef]|
|66.|Diaz, B.; Nussbaum, M. Artifcial intelligence for teaching and learning in schools: The need for pedagogical intelligence. _Comput._|
||_Educ._ **2024**,_217_, 105071. [CrossRef]|
|67.|Khanam, S.; Tanweer, S.; Khalid, S. Artifcial Intelligence Surpassing Human Intelligence: Factual or Hoax. _Comput. J._**2021**,|
||_64_, 1832–1839. [CrossRef]|
|68.|Guo, Y.; Zhang, W.; Qin, Q.; Chen, K.; Wei, Y. Intelligent manufacturing management system based on data mining in artifcial|
||intelligence energy-saving resources. _Soft Comput._ **2023**,_27_, 4061–4076. [CrossRef]|
|69.|Carabin, G.; Wehrle, E.; Vidoni, R. A review on energy-saving optimization methods for robotic and automatic systems. _Robotics_|
||**2017**,_6_, 39. [CrossRef]|
|70.|Paryanto; Brossog, M.; Bornschlegl, M.; Franke, J. Reducing the energy consumption of industrial robots in manufacturing|
||systems. _Int. J. Adv. Manuf. Technol._ **2015**,_78_, 1315–1328. [CrossRef]|
|71.|Achiam, J.; Adler, S.; Agarwal, S.; Ahmad, L.; Akkaya, I.; Aleman, F.L.; Almeida, D.; Altenschmidt, J.; Altman, S.; Anadkat, S.;|
||et al. Gpt-4 technical report. _arXiv_**2023**, arXiv:2303.08774.|
|72.|Liu, A.; Feng, B.; Xue, B.; Wang, B.; Wu, B.; Lu, C.; Zhao, C.; Deng, C.; Zhang, C.; Ruan, C.; et al. Deepseek-v3 technical report.|
||_arXiv_**2024**, arXiv:2412.19437.|
|73.|Gligorea, I.; Cioca, M.; Oancea, R.; Gorski, A.T.; Gorski, H.; Tudorache, P. Adaptive learning using artifcial intelligence in|
||e-learning: A literature review. _Educ. Sci._ **2023**,_13_, 1216. [CrossRef]|
|74.|Turdakov, D.Y.; Avetisyan, A.I.; Arkhipenko, K.V.; Antsiferova, A.V.; Vatolin, D.S.; Volkov, S.S.; Gasnikov, A.V.; Devyatkin, D.A.;|
||Drobyshevsky, M.D.; Kovalenko, A.P.; et al. Trusted Artifcial Intelligence: Challenges and Promising Solutions. _Dokl. Math._ **2022**,|
||_106_, S9–S13. [CrossRef]|
|75.|Thai, Q.M.; Nguyen, T.H.; Lenon, G.B.; Phung, H.T.T.; Horng, J.T.; Tran, P.T.; Ngo, S.T. Estimating AChE inhibitors from MCE|
||database by machine learning and atomistic calculations. _J. Mol. Graph. Model._ **2025**,_134_, 108906. [CrossRef]|
|76.|Deb, D.; Arunachalam, V.; Raju, K.S. Daily reservoir infow prediction using stacking ensemble of machine learning algorithms. _J._|
||_Hydroinformatics_**2024**,_26_, 972–997. [CrossRef]|
|77.|Ouyang, J.; Wu, M.; Li, X.; Deng, H.; Jin, Z.; Wu, D. NeuroBCI: Multi-Brain to Multi-Robot Interaction Through EEG-Adaptive|
||Neural Networks and Semantic Communications. _IEEE Trans. Mob. Comput._ **2024**,_23_, 14622–14637. [CrossRef]|
|78.|Ning, D.S.; Zhou, Z.Q.; Zhou, S.H.; Chen, J.M. Identifcation of macrophage differentiation related genes and subtypes linking|
||atherosclerosis plaque processing and metabolic syndrome via integrated bulk and single-cell sequence analysis. _Heliyon_**2024**,_10_,|
||34295. [CrossRef] [PubMed]|
|79.|Zhou, Y.; Narsilio, G.; Makasis, N.; Soga, K.; Chen, P.; Aye, L. Artifcial neural networks for predicting the performance of heat|
||pumps with horizontal ground heat exchangers. _Front. Energy Res._ **2024**,_12_, 1423695. [CrossRef]|
|80.|Zhang, S.; Chen, X.; Ran, X.; Li, Z.; Cao, W. Prioritizing Causation in Decision Trees: A Framework for Interpretable Modeling.|
||_Eng. Appl. Artif. Intell._ **2024**,_133_, 108224. [CrossRef]|
|81.|Cohen-Shapira, N.; Rokach, L. PnT: Born-again tree-based model via fused decision path encoding. _Inf. Fusion_**2024**,_112_, 102545.|
||[CrossRef]|



_Algorithms_ **2025** , _18_ , 263 

63 of 67 

82. Que, Z.; Lin, C.J. One-Class SVM Probabilistic Outputs. _IEEE Trans. Neural Netw. Learn. Syst._ **2024** , _36_ , 6244–6256. [CrossRef] 

83. Han, C.; Zou, G.; Yeh, H.G.; Gong, F.; Shi, S.; Chen, H. Intelligent fault prediction with wavelet-SVM fusion in coal mine. _Comput. Geosci._ **2025** , _194_ , 105744. [CrossRef] 

84. Noroozi, F.; Ghanbarian, G.; Safaeian, R.; Pourghasemi, H.R. Forest fire mapping: A comparison between GIS-based random forest and Bayesian models. _Nat. Hazards_ **2024** , _120_ , 6569–6592. [CrossRef] 

85. Xu, Y.; Li, J.; Wu, A. An XGboost Algorithm Based Model for Financial Risk Prediction. _Teh. Vjesn.-Tech. Gaz._ **2024** , _31_ , 1898–1907. [CrossRef] 

86. Shua, Q.; Peng, H.; Li, J. Landslide susceptibility prediction modelling based on semi-supervised XGBoost model. _Geol. J._ **2024** , _59_ , 2655–2667. [CrossRef] 

87. Zhang, C.; Chen, J.; Li, J.; Peng, Y.; Mao, Z. Large language models for human–robot interaction: A review. _Biomim. Intell. Robot._ **2023** , _3_ , 100131. [CrossRef] 

88. Gao, Y.; Jiang, Y.; Peng, Y.; Yuan, F.; Zhang, X.; Wang, J. Medical Image Segmentation: A Comprehensive Review of Deep Learning-Based Methods. _Tomography_ **2025** , _11_ , 52. [CrossRef] 

89. Bai, X.; Peng, Y.; Li, D.; Liu, Z.; Mao, Z. Novel soft robotic finger model driven by electrohydrodynamic (EHD) pump. _J. Zhejiang Univ.-Sci. A_ **2024** , _25_ , 596–604. [CrossRef] 

90. Mao, Z.; Bai, X.; Peng, Y.; Shen, Y. Design, modeling, and characteristics of ring-shaped robot actuated by functional fluid. _J. Intell. Mater. Syst. Struct._ **2024** , _35_ , 1459–1470. [CrossRef] 

91. Peng, Y.; Wang, Y.; Hu, F.; He, M.; Mao, Z.; Huang, X.; Ding, J. Predictive modeling of flexible EHD pumps using Kolmogorov– Arnold Networks. _Biomim. Intell. Robot._ **2024** , _4_ , 100184. [CrossRef] 

92. Mao, Z.; Peng, Y.; Hu, C.; Ding, R.; Yamada, Y.; Maeda, S. Soft computing-based predictive modeling of flexible electrohydrodynamic pumps. _Biomim. Intell. Robot._ **2023** , _3_ , 100114. [CrossRef] 

93. Peng, Y.; Sakai, Y.; Funabora, Y.; Yokoe, K.; Aoyama, T.; Doki, S. Funabot-Sleeve: A Wearable Device Employing McKibben Artificial Muscles for Haptic Sensation in the Forearm. _IEEE Robot. Autom. Lett._ **2025** , 1–8. [CrossRef] 

94. Mao, Z.; Kobayashi, R.; Nabae, H.; Suzumori, K. Multimodal strain sensing system for shape recognition of tensegrity structures by combining traditional regression and deep learning approaches. _IEEE Robot. Autom. Lett._ **2024** , _9_ , 10050–10056. [CrossRef] 

95. Mao, Z.; Hosoya, N.; Maeda, S. Flexible electrohydrodynamic fluid-driven valveless water pump via immiscible interface. _Cyborg Bionic Syst._ **2024** , _5_ , 0091. [CrossRef] 

96. Lau, S.L.; Lim, J.; Chong, E.K.; Wang, X. Single-pixel image reconstruction based on block compressive sensing and convolutional neural network. _Int. J. Hydromechatron._ **2023** , _6_ , 258–273. [CrossRef] 

97. Verma, H.; Siruvuri, S.V.; Budarapu, P. A machine learning-based image classification of silicon solar cells. _Int. J. Hydromechatron._ **2024** , _7_ , 49–66. [CrossRef] 

98. Alawi, O.A.; Kamar, H.M.; Shawkat, M.M.; Al-Ani, M.M.; Mohammed, H.A.; Homod, R.Z.; Wahid, M.A. Artificial intelligencebased viscosity prediction of polyalphaolefin-boron nitride nanofluids. _Int. J. Hydromechatron._ **2024** , _7_ , 89–112. [CrossRef] 

99. Tian, Z.; Zhao, D.; Lin, Z.; Flynn, D.; Zhao, W.; Tian, D. Balanced reward-inspired reinforcement learning for autonomous vehicle racing. In Proceedings of the 6th Annual Learning for Dynamics & Control Conference, PMLR, Oxford, UK, 15–17 July 2024; pp. 628–640. 

100. Tian, Z.; Zhao, D.; Lin, Z.; Zhao, W.; Flynn, D.; Jiang, Y.; Tian, D.; Zhang, Y.; Sun, Y. Efficient and balanced exploration-driven decision making for autonomous racing using local information. _IEEE Trans. Intell. Veh._ **2024** , 1–17. [CrossRef] 

101. Lin, Z.; Tian, Z.; Zhang, Q.; Ye, Z.; Zhuang, H.; Lan, J. A conflicts-free, speed-lossless KAN-based reinforcement learning decision system for interactive driving in roundabouts. _arXiv_ **2024** , arXiv:2408.08242. 

102. Tian, Z.; Lin, Z.; Zhao, D.; Zhao, W.; Flynn, D.; Ansari, S.; Wei, C. Evaluating Scenario-based Decision-making for Interactive Autonomous Driving Using Rational Criteria: A Survey. _arXiv_ **2025** , arXiv:2501.01886. 

103. Lin, Z.; Tian, Z.; Zhang, Q.; Zhuang, H.; Lan, J. Enhanced visual slam for collision-free driving with lightweight autonomous cars. _Sensors_ **2024** , _24_ , 6258. [CrossRef] [PubMed] 

104. Lin, Z.; Zhang, Q.; Tian, Z.; Yu, P.; Lan, J. DPL-SLAM: Enhancing dynamic point-line SLAM through dense semantic methods. _IEEE Sens. J._ **2024** , _24_ , 14596–14607. [CrossRef] 

105. Radanliev, P. Cyber diplomacy: Defining the opportunities for cybersecurity and risks from Artificial Intelligence, IoT, Blockchains, and Quantum Computing. _J. Cyber Secur. Technol._ **2025** , _9_ , 28–78. [CrossRef] 

106. Yun, D.; Wu, X.; Chen, X.; Yang, Y.; Shang, Y.; Liu, S.; Gunasekeran, D.V.; Lin, D.; Liu, L.; Zhao, L.; et al. An Artificial Intelligence and Blockchain technology-based data management framework for multicenter randomized controlled trials. _Sci. Bull._ **2025** , _70_ , 856–860. [CrossRef] 

107. Kwangmuang, P.; Somabut, A.; Duangngern, P.; Changpetch, S.; Dhithjaroen, C.; Techapornpong, O.; Sarakan, P.; Chaijaroen, S.; Samat, C. Designing an AI-enhanced blockchain and FinTech curriculum for youth: A case study of educational and industrial collaboration. _Educ. Inf. Technol._ **2025** , 1–37. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

64 of 67 

108. Zheng, J.; Zhang, Y. TRBFT: An Efficient Blockchain Consensus for Edge Computing-Enabled IoT Systems. _IEEE Internet Things J._ **2025** . [CrossRef] 

109. Zhao, C.; Zhou, Y.; Zhang, S.; Sheng, Q.Z.; Zhang, Y.; Wen, S. ExClique: An Express Consensus Algorithm for High-Speed Transaction Process in Blockchains. _arXiv_ **2025** , arXiv:2501.15289. 

110. Sahraoui, S.; Bachir, A. Lightweight Consensus Mechanisms in the Internet of Blockchained Things: Thorough Analysis and Research Directions. _Digit. Commun. Netw._ 2025; _in press_ . [CrossRef] 

111. Xing, Z.; Zhang, Z.; Zhang, Z.; Li, Z.; Li, M.; Liu, J.; Zhang, Z.; Zhao, Y.; Sun, Q.; Zhu, L.; et al. Zero-Knowledge Proof-Based Verifiable Decentralized Machine Learning in Communication Network: A Comprehensive Survey. _IEEE Commun. Surv. Tutor._ **2025** , 1. [CrossRef] 

112. Verma, S.; Chandra, G.; Yadav, D. LVCA: An efficient voting-based consensus algorithm in private Blockchain for enhancing data security. _Peer-to-Peer Netw. Appl._ **2025** , _18_ , 1–19. [CrossRef] 

113. St-Laurent, G.P.; Le Billon, P. Staking claims and shaking hands: Impact and benefit agreements as a technology of government in the mining sector. _Extr. Ind. Soc._ **2015** , _2_ , 590–602. [CrossRef] 

114. Naeem, M.; Khan, A.; Rehman, A.; Farooq, S.; Mehboob, A.; Abdali, A.S.; Ahmad, B. Does Artificial Intelligence with Blockchain Reduce the Costs of the Financial Sector? In _Generative AI for Web Engineering Models_ ; IGI Global: Hershey, PA, USA, 2025; pp. 147–160. 

115. Villegas-Ch, W.; Govea, J.; Gurierrez, R.; Mera-Navarrete, A. Optimizing Security in IoT Ecosystems Using Hybrid Artificial Intelligence and Blockchain Models: A Scalable and Efficient Approach for Threat Detection. _IEEE Access_ **2025** , _13_ , 16933–16958. [CrossRef] 

116. Dutta, J.; Puthal, D. PoAh 2.0: AI-empowered dynamic authentication based adaptive blockchain consensus for IoMT-edge workflow. _Future Gener. Comput. Syst.-Int. J. Escience_ **2024** , _161_ , 655–672. [CrossRef] 

117. Sun, C.; Li, D.; Wang, B.; Song, J. AI-Enabled Consensus Algorithm in Human-Centric Collaborative Computing for Internet of Vehicle. _Symmetry_ **2023** , _15_ , 1264. [CrossRef] 

118. Alruwaili, F.F. Ensuring data integrity in deep learning-assisted IoT-Cloud environments: Blockchain-assisted data edge verification with consensus algorithms. _AIMS Math._ **2024** , _9_ , 8868–8884. [CrossRef] 

119. Vaiyapuri, T.; Shankar, K.; Rajendran, S.; Kumar, S.; Acharya, S.; Kim, H. Blockchain Assisted Data Edge Verification With Consensus Algorithm for Machine Learning Assisted IoT. _IEEE Access_ **2023** , _11_ , 55370–55379. [CrossRef] 

120. Zhang, Z.; Guo, B.; Shen, Y.; Li, C.; Suo, X.; Su, H. Nakamoto Consensus to Accelerate Supervised Classification Algorithms for Multiparty Computing. _Secur. Commun. Netw._ **2021** , _2021_ , 6629433. [CrossRef] 

121. Sasikumar, A.; Ravi, L.; Kotecha, K.; Saini, J.R.R.; Varadarajan, V.; Subramaniyaswamy, V. Sustainable Smart Industry: A Secure and Energy Efficient Consensus Mechanism for Artificial Intelligence Enabled Industrial Internet of Things. _Comput. Intell. Neurosci._ **2022** , _2022_ , 1419360. [CrossRef] [PubMed] 

122. Kumar, P.; Javeed, D.; Kumar, R.; Islam, A.K.M.N. Blockchain and explainable AI for enhanced decision making in cyber threat detection. _Softw.-Pract. Exp._ **2024** , _54_ , 1337–1360. [CrossRef] 

123. Kim, S.K. Automotive Vulnerability Analysis for Deep Learning Blockchain Consensus Algorithm. _Electronics_ **2022** , _11_ , 119. [CrossRef] 

124. Cui, C.; Du, H.; Jia, Z.; He, Y.; Wang, L. Blockchain-Enabled Federated Learning with Differential Privacy for Internet of Vehicles. _CMC-Comput. Mater. Contin._ **2024** , _81_ , 1581–1593. [CrossRef] 

125. Wang, Y.; Peng, H.; Su, Z.; Luan, T.H.; Benslimane, A.; Wu, Y. A Platform-Free Proof of Federated Learning Consensus Mechanism for Sustainable Blockchains. _IEEE J. Sel. Areas Commun._ **2022** , _40_ , 3305–3324. [CrossRef] 

126. Deng, Q. Blockchain Economical Models, Delegated Proof of Economic Value and Delegated Adaptive Byzantine Fault Tolerance and their implementation in Artificial Intelligence BlockCloud. _J. Risk Financ. Manag._ **2019** , _12_ , 177. [CrossRef] 

127. Alrubei, S.M.; Ball, E.; Rigelsford, J.M. The Use of Blockchain to Support Distributed AI Implementation in IoT Systems. _IEEE Internet Things J._ **2022** , _9_ , 14790–14802. [CrossRef] 

128. Zhi, H.; Wu, H.; Huang, Y.; Tian, C.; Wang, S. Blockchain Consensus Scheme Based on the Proof of Distributed Deep Learning Work. _IET Softw._ **2025** , _2025_ , 3378383. [CrossRef] 

129. Jogunola, O.; Adebisi, B.; Ikpehai, A.; Popoola, I.S.; Gui, G.; Gacanin, H.; Ci, S. Consensus Algorithms and Deep Reinforcement Learning in Energy Market: A Review. _IEEE Internet Things J._ **2021** , _8_ , 4211–4227. [CrossRef] 

130. Goh, Y.; Yun, J.; Jung, D.; Chung, J.M. Secure Trust-Based Delegated Consensus for Blockchain Frameworks Using Deep Reinforcement Learning. _IEEE Access_ **2022** , _10_ , 118498–118511. [CrossRef] 

131. Kim, S.K. Blockchain Smart Contract to Prevent Forgery of Degree Certificates: Artificial Intelligence Consensus Algorithm. _Electronics_ **2022** , _11_ , 2112. [CrossRef] 

132. Chen, Z.; Yu, Z. Intelligent Offloading in Blockchain-Based Mobile Crowdsensing Using Deep Reinforcement Learning. _IEEE Commun. Mag._ **2023** , _61_ , 118–123. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

65 of 67 

133. Wang, X.; Zhang, H.; Zhang, J.; Ge, Y.; Cui, K.; Peng, Z.; Li, Z.; Wang, L. An Improved Practical Byzantine Fault-Tolerant Algorithm Based on XGBoost Grouping for Consortium Chains. _Comput. Mater. Contin._ **2025** , _82_ , 1295–1311. [CrossRef] 

134. Xiao, J.; Luo, T.; Li, C.; Zhou, J.; Li, Z. CE-PBFT: A high availability consensus algorithm for large-scale consortium blockchain. _J. King Saud Univ.-Comput. Inf. Sci._ **2024** , _36_ , 101957. [CrossRef] 

135. Yuan, F.; Huang, X.; Zheng, L.; Wang, L.; Wang, Y.; Yan, X.; Gu, S.; Peng, Y. The Evolution and Optimization Strategies of a PBFT Consensus Algorithm for Consortium Blockchains. _Information_ **2025** , _16_ , 268. [CrossRef] 

136. Lyu, Q.; Ma, C.; Shen, Y.; Jiao, S.; Sun, Y.; Hu, L. Analyzing Ethereum Smart Contract Vulnerabilities at Scale Based on Inter-Contract Dependency. _CMES-Comput. Model. Eng. Sci._ **2023** , _135_ , 1625–1647. [CrossRef] 

137. Yang, H.; Gu, X.; Chen, X.; Zheng, L.; Cui, Z. CrossFuzz: Cross-contract fuzzing for smart contract vulnerability detection. _Sci. Comput. Program._ **2024** , _234_ , 103076. [CrossRef] 

138. Gao, Z.; Jiang, L.; Xia, X.; Lo, D.; Grundy, J. Checking Smart Contracts with Structural Code Embedding. _IEEE Trans. Softw. Eng._ **2021** , _47_ , 2874–2891. [CrossRef] 

139. Wei, Z.; Sun, J.; Zhang, Z.; Zhang, X.; Yang, X.; Zhu, L. Survey on Quality Assurance of Smart Contracts. _ACM Comput. Surv._ **2025** , _57_ , 1–36. [CrossRef] 

140. Vidal, F.R.; Ivaki, N.; Laranjeiro, N. OpenSCV: An open hierarchical taxonomy for smart contract vulnerabilities. _Empir. Softw. Eng._ **2024** , _29_ , 101. [CrossRef] 

141. Wang, W.; Xia, L.; Zhang, Z.; Meng, X. Smart Contract Timestamp Vulnerability Detection Based on Code Homogeneity. _IEICE Trans. Inf. Syst._ **2024** , _E107D_ , 1362–1366. [CrossRef] 

142. Fei, J.; Chen, X.; Zhao, X. MSmart: Smart Contract Vulnerability Analysis and Improved Strategies Based on Smartcheck. _Appl. Sci._ **2023** , _13_ , 1733. [CrossRef] 

143. Chen, J.; Xia, X.; Lo, D.; Grundy, J.; Luo, X.; Chen, T. DefectChecker: Automated Smart Contract Defect Detection by Analyzing EVM Bytecode. _IEEE Trans. Softw. Eng._ **2021** , _48_ , 2189–2207. [CrossRef] 

144. Lin, Z.; Chen, J.; Wu, J.; Zhang, W.; Wang, Y.; Zheng, Z. CRPWarner: Warning the Risk of Contract-Related Rug Pull in DeFi Smart Contracts. _IEEE Trans. Softw. Eng._ **2024** , _50_ , 1534–1547. [CrossRef] 

145. Wu, H.; Dong, H.; He, Y.; Duan, Q. Smart Contract Vulnerability Detection Based on Hybrid Attention Mechanism Model. _Appl. Sci._ **2023** , _13_ , 770. [CrossRef] 

146. Zhang, L.; Chen, W.; Wang, W.; Jin, Z.; Zhao, C.; Cai, Z.; Chen, H. CBGRU: A Detection Method of Smart Contract Vulnerability Based on a Hybrid Model. _Sensors_ **2022** , _22_ , 3577. [CrossRef] [PubMed] 

147. Hwang, S.J.; Choi, S.H.; Shin, J.; Choi, Y.H. CodeNet: Code-Targeted Convolutional Neural Network Architecture for Smart Contract Vulnerability Detection. _IEEE Access_ **2022** , _10_ , 32595–32607. [CrossRef] 

148. Gao, C.; Yang, W.; Ye, J.; Xue, Y.; Sun, J. sGUARD plus: Machine Learning Guided Rule-Based Automated Vulnerability Repair on Smart Contracts. _ACM Trans. Softw. Eng. Methodol._ **2024** , _33_ , 114. [CrossRef] 

149. Ivanov, N.; Yan, Q.; Kompalli, A. TxT: Real-Time Transaction Encapsulation for Ethereum Smart Contracts. _IEEE Trans. Inf. Forensics Secur._ **2023** , _18_ , 1141–1155. [CrossRef] 

150. Yu, R.; Zhang, Y.; Wang, Y.; Liu, C. TxMirror: When the Dynamic EVM Stack Meets Transactions for Smart Contract Vulnerability Detection. _Symmetry_ **2023** , _15_ , 1345. [CrossRef] 

151. Li, J.; Lu, G.; Gao, Y.; Gao, F. A Smart Contract Vulnerability Detection Method Based on Multimodal Feature Fusion and Deep Learning. _Mathematics_ **2023** , _11_ , 4823. [CrossRef] 

152. Pasqua, M.; Benini, A.; Contro, F.; Crosara, M.; Dalla Preda, M.; Ceccato, M. Enhancing Ethereum smart-contracts static analysis by computing a precise Control-Flow Graph of Ethereum bytecode. _J. Syst. Softw._ **2023** , _200_ , 111653. [CrossRef] 

153. Qian, S.; Ning, H.; He, Y.; Chen, M. Multi-Label Vulnerability Detection of Smart Contracts Based on Bi-LSTM and Attention Mechanism. _Electronics_ **2022** , _11_ , 3260. [CrossRef] 

154. Qian, P.; He, J.; Lu, L.; Wu, S.; Lu, Z.; Wu, L.; Zhou, Y.; He, Q. Demystifying Random Number in Ethereum Smart Contract: Taxonomy, Vulnerability Identification, and Attack Detection. _IEEE Trans. Softw. Eng._ **2023** , _49_ , 3793–3810. [CrossRef] 

155. Le, T.T.H.; Kim, J.; Lee, S.; Kim, H. Robust Vulnerability Detection in Solidity-Based Ethereum Smart Contracts Using Fine-Tuned Transformer Encoder Models. _IEEE Access_ **2024** , _12_ , 154700–154717. [CrossRef] 

156. Wang, M.; Xie, Z.; Wen, X.; Li, J.; Zhou, K. Ethereum Smart Contract Vulnerability Detection Model Based on Triplet Loss and BiLSTM. _Electronics_ **2023** , _12_ , 2327. [CrossRef] 

157. Park, J.; Jeong, S.; Yeom, K. Smart Contract Broker: Improving Smart Contract Reusability in a Blockchain Environment. _Sensors_ **2023** , _23_ , 6149. [CrossRef] 

158. Sanchez-Gomez, N.; Torres-Valderrama, J.; Mejias Risoto, M.; Garrido, A. Blockchain Smart Contract Meta-modeling. _J. Web Eng._ **2021** , _20_ , 2059–2079. [CrossRef] 

159. Gec, S.; Stankovski, V.; Lavbic, D.; Kochovski, P. A Recommender System for Robust Smart Contract Template Classification. _Sensors_ **2023** , _23_ , 639. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

66 of 67 

160. Chen, Y.; Hu, D.; Xu, C.; Chen, N. An Annotation Assisted Smart Contracts Generation Method. _IEEE Access_ **2024** , _12_ , 51485–51499. [CrossRef] 

161. Tong, Y.; Tan, W.; Guo, J.; Shen, B.; Qin, P.; Zhuo, S. Smart Contract Generation Assisted by AI-Based Word Segmentation. _Appl. Sci._ **2022** , _12_ , 4773. [CrossRef] 

162. Margret, M.K.; Julie, E.G. Smarter and resilient smart contracts applications for smart cities environment using blockchain technology. _Automatika_ **2024** , _65_ , 572–583. [CrossRef] 

163. Zhang, L.; Kim, D. A Peer-to-Peer Smart Food Delivery Platform Based on Smart Contract. _Electronics_ **2022** , _11_ , 1806. [CrossRef] 

164. Zhang, X.; Liu, C.; Medda, F. A Smart-Contract-Aided Plastic Credit Scheme. _IEEE Syst. J._ **2023** , _17_ , 1703–1713. [CrossRef] 

165. Wang, L.; Xu, L.; Zheng, Z.; Liu, S.; Li, X.; Cao, L.; Li, J.; Sun, C. Smart Contract-Based Agricultural Food Supply Chain Traceability. _IEEE Access_ **2021** , _9_ , 9296–9307. [CrossRef] 

166. Garcia, R.D.; Ramachandran, G.; Ueyama, J. Exploiting smart contracts in PBFT-based blockchains: A case study in medical prescription system. _Comput. Netw._ **2022** , _211_ , 109003. [CrossRef] 

167. Agrawal, T.K.; Angelis, J.; Khilji, W.A.; Kalaiarasan, R.; Wiktorsson, M. Demonstration of a blockchain-based framework using smart contracts for supply chain collaboration. _Int. J. Prod. Res._ **2023** , _61_ , 1497–1516. [CrossRef] 

168. Zhang, Y.; Yutaka, M.; Sasabe, M.; Kasahara, S. Attribute-Based Access Control for Smart Cities: A Smart-Contract-Driven Framework. _IEEE Internet Things J._ **2021** , _8_ , 6372–6384. [CrossRef] 

169. Karakoc, E.; Ceken, C. Secure SLA Management Using Smart Contracts for SDN-Enabled WSN. _KSII Trans. Internet Inf. Syst._ **2023** , _17_ , 3003–3029. [CrossRef] 

170. Guo, Z.Y.; Chen, Y.C.; Lin, H.P. Oblivious Access for Decentralized Database Systems: A New Asymmetric Framework from Smart Contracts. _Symmetry_ **2022** , _14_ , 680. [CrossRef] 

171. Selvarajan, S.; Srivastava, G.; Khadidos, A.O.; Khadidos, A.O.; Baza, M.; Alshehri, A.; Lin, J.C.W. An artificial intelligence lightweight blockchain security model for security and privacy in IIoT systems. _J. Cloud-Comput.-Adv. Syst. Appl._ **2023** , _12_ , 38. [CrossRef] 

172. Wang, J.; Li, J. Blockchain and Access Control Encryption-Empowered IoT Knowledge Sharing for Cloud-Edge Orchestrated Personalized Privacy-Preserving Federated Learning. _Appl. Sci._ **2024** , _14_ , 1743. [CrossRef] 

173. Miao, Q.; Lin, H.; Hu, J.; Wang, X. An intelligent and privacy-enhanced data sharing strategy for blockchain-empowered Internet of Things. _Digit. Commun. Netw._ **2022** , _8_ , 636–643. [CrossRef] 

174. Smahi, A.; Li, H.; Yang, Y.; Yang, X.; Lu, P.; Zhong, Y.; Liu, C. BV-ICVs: A privacy-preserving and verifiable federated learning framework for V2X environments using blockchain and zkSNARKs. _J. King Saud Univ.-Comput. Inf. Sci._ **2023** , _35_ , 101542. [CrossRef] 

175. Yang, F.; Abedin, m.Z.; Hajek, P. An explainable federated learning and blockchain-based secure credit modeling method. _Eur. J. Oper. Res._ **2024** , _317_ , 449–467. [CrossRef] 

176. Firdaus, M.; Larasati, H.T.; Rhee, K.H. A Blockchain-Assisted Distributed Edge Intelligence for Privacy-Preserving Vehicular Networks. _CMC-Comput. Mater. Contin._ **2023** , _76_ , 2959–2978. [CrossRef] 

177. Li, K.; Zhou, H.; Tu, Z.; Liu, F.; Zhang, H. Blockchain Empowered Federated Learning for Distributed Network Security Behaviour Knowledge Base in 6G. _Secur. Commun. Netw._ **2022** , _2022_ , 1–11. [CrossRef] 

178. Wan, C.; Wang, Y.; Xu, J.; Wu, J.; Zhang, T.; Wang, Y. Research on Privacy Protection in Federated Learning Combining Distillation Defense and Blockchain. _Electronics_ **2024** , _13_ , 679. [CrossRef] 

179. Liu, W.; He, Y.; Wang, X.; Duan, Z.; Liang, W.; Liu, Y. BFG: Privacy protection framework for internet of medical things based on blockchain and federated learning. _Connect. Sci._ **2023** , _35_ . [CrossRef] 

180. Ma, H.; Huang, S.; Guo, J.; Lam, K.Y.; Yang, T. Blockchain-Based Privacy-Preserving Federated Learning for Mobile Crowdsourcing. _IEEE Internet Things J._ **2024** , _11_ , 13884–13899. [CrossRef] 

181. Wang, N.; Yang, W.; Wang, X.; Wu, L.; Guan, Z.; Du, X.; Guizani, M. A blockchain based privacy-preserving federated learning scheme for Internet of Vehicles. _Digit. Commun. Netw._ **2024** , _10_ , 126–134. [CrossRef] 

182. Zhao, Y.; Zhao, J.; Jiang, L.; Tan, R.; Niyato, D.; Li, Z.; Lyu, L.; Liu, Y. Privacy-Preserving Blockchain-Based Federated Learning for IoT Devices. _IEEE Internet Things J._ **2021** , _8_ , 1817–1829. [CrossRef] 

183. Zhu, C.; Zhu, X.; Qin, T. An Efficient Privacy Protection Mechanism for Blockchain-Based Federated Learning System in UAV-MEC Networks. _Sensors_ **2024** , _24_ , 1364. [CrossRef] 

184. Mo, M.; Ji, S.; Wang, X.; Mohiuddin, G.; Ren, Y. Privacy Data Management Mechanism Based on Blockchain and Federated Learning. _CMC-Comput. Mater. Contin._ **2023** , _74_ , 37–53. [CrossRef] 

185. Wang, Q.; Dong, H.; Huang, Y.; Liu, Z.; Gou, Y. Blockchain-Enabled Federated Learning for Privacy-Preserving Non-IID Data Sharing in Industrial Internet. _CMC-Comput. Mater. Contin._ **2024** , _80_ , 1967–1983. [CrossRef] 

186. Zhang, F.; Zhang, Y.; Ji, S.; Han, Z. Secure and decentralized federated learning framework with non-IID data based on blockchain. _Heliyon_ **2024** , _10_ , e27176. [CrossRef] 

_Algorithms_ **2025** , _18_ , 263 

67 of 67 

187. Myrzashova, R.; Alsamhi, S.H.; Hawbani, A.; Curry, E.; Guizani, M.; Wei, X. Safeguarding Patient Data-Sharing: BlockchainEnabled Federated Learning in Medical Diagnostics. _IEEE Trans. Sustain. Comput._ **2025** , _10_ , 176–189. [CrossRef] 

188. Rabbani, H.; Shahid, M.F.; Khanzada, T.J.S.; Siddiqui, S.; Jamjoom, M.M.; Ashari, R.B.; Ullah, Z.; Mukati, M.U.; Nooruddin, M. Enhancing security in financial transactions: A novel blockchain-based federated learning framework for detecting counterfeit data in fintech. _PeerJ Comput. Sci._ **2024** , _10_ , e2280. [CrossRef] 

189. Meng, M.; Li, Y. SFedChain: Blockchain-based federated learning scheme for secure data sharing in distributed energy storage networks. _PeerJ Comput. Sci._ **2022** , _8_ , e1027. [CrossRef] 

190. Zhu, L.; Hu, S.; Zhu, X.; Meng, C.; Huang, M. Enhancing the Security and Privacy in the IoT Supply Chain Using Blockchain and Federated Learning with Trusted Execution Environment. _Mathematics_ **2023** , _11_ , 3759. [CrossRef] 

191. Jia, Y.; Xiong, L.; Fan, Y.; Liang, W.; Xiong, N.; Xiao, F. Blockchain-based privacy-preserving multi-tasks federated learning framework. _Connect. Sci._ **2024** , _36_ . [CrossRef] 

192. Zhang, Y.; Tang, Y.; Zhang, Z.; Li, M.; Li, Z.; Khan, S.; Chen, H.; Cheng, G. Blockchain-Based Practical and Privacy-Preserving Federated Learning with Verifiable Fairness. _Mathematics_ **2023** , _11_ , 1091. [CrossRef] 

193. Dong, C.; Zhou, J.; An, Q.; Jiang, F.; Chen, S.; Pan, L.; Liu, X. Optimizing Performance in Federated Person Re-Identification through Benchmark Evaluation for Blockchain-Integrated Smart UAV Delivery Systems. _Drones_ **2023** , _7_ , 413. [CrossRef] 

194. Dhasaratha, C.; Hasan, M.K.; Islam, S.; Khapre, S.; Abdullah, S.; Ghazal, T.M.; Alzahrani, A.I.; Alalwan, N.; Vo, N.; Akhtaruzzaman, M. Data privacy model using blockchain reinforcement federated learning approach for scalable internet of medical things. _CAAI Trans. Intell. Technol._ **2024** . [CrossRef] 

195. Li, K. A Blockchain-Integrated Federated Learning Approach for Secure Data Sharing and Privacy Protection in Multi-Device Communication. _Appl. Artif. Intell._ **2025** , _39_ . [CrossRef] 

196. Cai, C.; Xu, L.; Zhou, A.; Wang, R.; Wang, C.; Wang, Q. EncELC: Hardening and Enriching Ethereum Light Clients with Trusted Enclaves. In Proceedings of the IEEE INFOCOM 2020—IEEE Conference on Computer Communications, Toronto, ON, Canada, 6–9 July 2020; pp. 1887–1896. [CrossRef] 

197. Shen, H.; Wang, T.; Chen, J.; Tao, Y.; Chen, F. Blockchain-Based Batch Authentication Scheme for Internet of Vehicles. _IEEE Trans. Veh. Technol._ **2024** , _73_ , 7866–7879. [CrossRef] 

198. Shinde, R.; Patil, S.; Kotecha, K.; Ruikar, K. Blockchain for Securing AI Applications and Open Innovations. _J. Open Innov. Technol. Mark. Complex._ **2021** , _7_ , 189. [CrossRef] 

199. Khan, J.A.; Bangalore, K.U.; Kurkcu, A.; Ozbay, K. TREAD: Privacy Preserving Incentivized Connected Vehicle Mobility Data Storage on InterPlanetary-File-System-Enabled Blockchain. _Transp. Res. Rec._ **2021** , _2676_ , 680–691. [CrossRef] 

200. Yiu, N.C.K. Decentralizing Supply Chain Anti-Counterfeiting and Traceability Systems Using Blockchain Technology. _Future Internet_ **2021** , _13_ , 84. [CrossRef] 

201. Almaiah, M.A.; Hajjej, F.; Ali, A.; Pasha, M.F.; Almomani, O. An AI—Enabled Hybrid Lightweight Authentication Model for Digital Healthcare Using Industrial Internet of Things Cyber-Physical Systems. _Sensors_ **2022** , _22_ , 1448. [CrossRef] [PubMed] 

202. Bouachir, O.; Aloqaily, M.; Karray, F.; Saddik, A.E. AI-based Blockchain for the Metaverse: Approaches and Challenges. In Proceedings of the 2022 Fourth International Conference on Blockchain Computing and Applications (BCCA), San Antonio, TX, USA, 5–7 September 2022; pp. 231–236. 

203. Singh, S.K.; Park, J.H. TaLWaR: Blockchain-Based Trust Management Scheme for Smart Enterprises With Augmented Intelligence. _IEEE Trans. Ind. Inform._ **2023** , _19_ , 626–634. [CrossRef] 

204. Wang, X.; Garg, S.; Lin, H.; Kaddoum, G.; Hu, J.; Hassan, M.M. Heterogeneous Blockchain and AI-Driven Hierarchical Trust Evaluation for 5G-Enabled Intelligent Transportation Systems. _IEEE Trans. Intell. Transp. Syst._ **2023** , _24_ , 2074–2083. [CrossRef] 

205. Khan, S.; Khan, M.; Khan, M.A.; Khan, M.A.; Wang, L.; Wu, K. A Blockchain-Enabled AI-Driven Secure Searchable Encryption Framework for Medical IoT Systems. _IEEE J. Biomed. Health Inform._ **2025** , 1–14. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

