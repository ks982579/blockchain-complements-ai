_ISSN 0146-4116, Automatic Control and Computer Sciences, 2025, Vol. 59, No. 3, pp. 297–316. © Allerton Press, Inc., 2025._ 

# **Resolving the Trilemma Challenge in Blockchain: An Integrated Consensus Mechanism for Balancing Security, Scalability, and Decentralization** 

**Khandakar Md Shafin** _**[a]**_ **and Saha Reno** _**[b]**_ **[,] *** 

_a Department of CSE, CCN University of Science and Technology (CCNUST), Cumilla, 3503 Bangladesh b Department of CSE, Ahsanullah University of Science and Technology (AUST), Dhaka, 1208 Bangladesh_ _***** e-mail: reno.saha39@gmail.com_ Received March 5, 2024; revised July 25, 2024; accepted July 26, 2024 

**Abstract** —Finding a way to solve the trilemma, which requires striking a balance between scalability, security, and decentralization, is a persistent problem in the field of blockchain technology. In order to overcome this trilemma, this study presents a novel blockchain architecture that combines cuttingedge cryptography techniques, creative security protocols, and flexible decentralization mechanisms. Our framework is a new standard for secure, scalable, and decentralized blockchain ecosystems. It utilizes well-known techniques like zero knowledge proof (zk-SNARK), Schnorr VRF, elliptic curve cryptography (ECC), and in addition to innovative approaches for anomaly detection, incentive alignment, and stake distribution. The suggested system outperforms elite consensuses by obtaining 1600+ TPS, guaranteeing strong security against all known blockchain attacks without sacrificing scalability, and obtaining a strong decentralization score of 7.181, which, when compared to other blockchain systems in benchmark analysis, shows strong decentralization. 

**Keywords:** blockchain, decentralization, consensus, security, scalability **DOI:** 10.3103/S0146411625700476 

## 1. INTRODUCTION 

The world of blockchain consensus mechanisms is faced with a significant challenge known as the blockchain trilemma [1]. This issue arises due to the difficult balance between decentralization, security, and scalability in existing consensus methods [2]. Although some algorithms perform exceptionally well in certain areas like IoT or cryptocurrencies, they often fall short of providing a well-rounded solution that can be applied across various use cases [3–5]. This is because different applications have unique requirements and priorities, leading to situations where a mechanism focused on decentralization might sacrifice security or limit scalability, and vice versa [6, 7]. This lack of a comprehensive consensus solution that can effectively balance decentralization, security, and scalability hinders the broader use and potential of blockchain technology in multiple sectors [8, 9]. Tackling this problem requires innovative solutions that harmonize these conflicting goals, advancing blockchain technology towards greater flexibility and usefulness [10]. 

Ensuring the integrity and consistency of transactions in decentralized networks heavily relies on blockchain consensus mechanisms. Over time, many research papers have introduced innovative solutions to the challenges of achieving consensus in distributed systems. We will be reviewing four significant papers in this section, examining their contributions, methodologies, findings, and critiques. 

Liu and others proposed a protocol that uses trusted hardware to facilitate secure key distribution and enable Byzantine fault tolerance through verifiable secret sharing [11]. While this method shows promise in enhancing scalability and security, its dependence on specific hardware may limit its applicability in various network settings. Additionally, the complexity and maintenance overhead associated with this hardware-assisted approach could be potential drawbacks in real-world implementations. 

Gilad and his co-authors introduced a consensus algorithm based on a verifiable random function (VRF) and cryptographic sortition [12]. The Algorand protocol stands out for its impressive scalability and security, ensuring quick block confirmations and resilience against attacks. However, it relies on the assumption that the majority of nodes will act honestly, which might not hold in all adversarial contexts. 

297 

298 

KHANDAKAR MD SHAFIN, SAHA RENO 

Additionally, the use of VRFs and cryptographic methods may expose the system to vulnerabilities that require further scrutiny. 

In this research, we employ advanced cryptographic methods, adaptive stake management, and random leader selection to develop an innovative consensus framework. Our goal is to tackle the blockchain trilemma head-on by effectively balancing decentralization, security, and scalability. Additionally, our approach incorporates efficient batch agreement protocols and robust fault tolerance, ensuring transactions are processed securely and seamlessly. By integrating these state-of-the-art technologies, our proposed consensus mechanism aims to revolutionize decentralized transaction validation, paving the way for broader adoption and significant advancements in the blockchain field. 

Despite significant progress in past research, each approach has its own benefits and drawbacks. This shows the need for further work to develop a consensus mechanism that effectively balances scalability, decentralization and security. In this paper, we present several key contributions: 

• Security Procedures for Complete Defense: This system employs a complex cryptographic technique that combines Zero-Knowledge Proofs (zk-SNARKs), Schnorr Verifiable Random Functions (VRF), and Elliptic Curve Cryptography. It ensures effective computation while enhancing security and privacy through the combination of various technologies. The framework’s features—extraction, training, anomaly detection, and a stringent verification procedure—combine to strengthen the blockchain, shielding it from harmful attacks and preserving the integrity of the ecosystem as a whole. 

• Scalability Enhancement using Layer-2 Solutions: Our system uses a sharding technique to divide the state, segment the network, balance loads, and handle routing in order to address scalability difficulties. Additionally, it permits off-chain scalability through the use of Layer-2 solutions like Rollups, enabling faster and more effective transaction processing while sustaining strong security. 

• Adaptive Decentralization Strategies: This work presents novel approaches to decentralization, including as reputation-based stake distribution and adaptive recalibration. By combining historical information, incentive efficiency, and stake distribution patterns, these methods build a decentralized network that is adaptable and responsive, improving its resistance to changing threats. 

• Incentives in Network Management Alignment: In order to encourage positive conduct among network users, a new incentive compliance model is being implemented that incorporates both rewards and punishments. By encouraging constructive engagement and discouraging negative actions, this strategy promotes the long-term viability and wellbeing of the ecosystem. 

## 2. LITERATURE REVIEW 

The potential of blockchain technology to provide safe and decentralized transaction processing has attracted a lot of attention lately. Despite this, the blockchain trilemma—which requires striking a balance between scalability, security, and decentralization—continues to pose a difficulty to developing an ideal consensus mechanism. This review of the literature examines six research studies that have improved our knowledge and comprehension of blockchain consensus, assessing the approaches, outcomes, and constraints of each study. 

Nakamoto presented Bitcoin as a decentralized digital currency that runs on a peer-to-peer network in his groundbreaking paper, “Bitcoin: A Peer-to-Peer Electronic Cash System” [13]. The study provided a workable model for a safe and permissionless money by outlining a blockchain-based strategy that combines cryptographic methods, a proof-of-work (PoW) consensus, and a network that is decentralized. Scalability problems still exist despite its advances, mostly because of transaction throughput limitations and PoW consensus restrictions. Transaction efficiency has been impacted by Bitcoin’s reliance on Proof of Work (PoW) and fixed block size restriction, which have led to network congestion and increased transaction costs during periods of high usage. 

Buterin introduced Ethereum as a blockchain platform intended for the execution of decentralized apps (dApps) and smart contracts in “Ethereum: A Next-Generation Smart Contract and Decentralized Application Platform” [14]. In order to enable programmable and self-executing applications, the paper presented the development of a Turing-complete virtual machine (EVM) and presented Ether as the native cryptocurrency. Even with these improvements, there are still scaling problems because each smart contract activity must be processed by every node, which can lead to performance snags. The necessity for smart contracts to be executed everywhere raises computing requirements, which restricts the Ethereum network’s capacity to grow. 

Eyal and other researchers presented “Bitcoin-NG: A Scalable Blockchain Protocol”, which is a scalable improvement on the initial Bitcoin blockchain [15]. The plan separates the handling of transactions 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

299 

from leader election by introducing a two-tier blockchain architecture with a consensus mechanism based on leaders. Although the goal of this design is to increase scalability, the network’s decentralized structure may be compromised by the centralization risk related to the leader-based block generation process. 

In “A Byzantine Fault-Tolerant Ordering Service for the Hyperledger Fabric Blockchain Platform”, Sousa and his co-authors detail the development of a Byzantine fault-tolerant (BFT) ordering service specifically designed for Hyperledger Fabric [16]. While the BFT ordering service improves fault tolerance, there is additional computational and communication cost associated with its integration. The Hyperledger Fabric network’s scalability may be limited by its complexity, especially in situations when transaction volumes are large. 

Castro and Liskov presented the Practical Byzantine Fault Tolerance (PBFT) method in “Practical Byzantine Fault Tolerance”, with the goal of reaching consensus in systems that are distributed [17]. Compared to more straightforward consensus techniques, PBFT has higher communication and computing expenses even if it provides strong fault tolerance. The scalability of BFT-based platforms may be limited by these extra overheads, particularly in large networks with lots of participants and frequently occurring transactions. 

Abraham and other researchers describe “Scalable Byzantine Consensus via Hardware-Assisted Secret Sharing”, a Byzantine fault-tolerant consensus mechanism that improves key distribution security by utilizing trusted hardware [11]. While this method increases performance and scalability, it depends on particular hardware technologies, which may limit the protocol’s application and practical implementation in many network scenarios. 

The consensus method presented in this research article addresses the blockchain trilemma of decentralization, security, and scalability and provides a solid response to the issues raised in the six linked papers. Our method aims for an approach that is balanced across all important dimensions, in contrast to existing algorithms that frequently sacrifice one dimension for another. It has an adaptive stake recalculation system that modifies node stakes according to stake transmissions, and network involvement in order to reduce the dangers associated with centralization. By splitting the blockchain into smaller sections, or shards, sharding allows for better network speed and parallel transaction processing. Layer-2 scaling solutions address scalability difficulties seen in previous studies by reducing network congestion and increasing transaction efficiency. To provide safe and private transaction processing, we use cutting-edge cryptographic techniques such threshold encryption algorithms and verifiable random functions (VRFs). Furthermore, by including fault tolerance methods such as the PBFT Byzantine Fault Tolerance (BFT) protocol, the integrity and robustness of the consensus process are protected from errors and adversarial assaults. To sum up, our consensus algorithm provides a comprehensive solution that skillfully strikes a balance between scalability, security, and decentralization. 

## 3. METHODOLOGY 

## _3.1. Function of Epoch_ 

Epochs form the foundational framework of our consensus, serving as the primary temporal units. Operations are structured within these epochs, further dividing them into smaller time slots. This organization ensures cohesive node cooperation and minimizes conflict potential. Transactions and blocks are sequentially ordered within every epoch, preserving the consistency of the ledger’s historical data. Additionally, epochs facilitate the consensus process by allowing nodes to function at predetermined time intervals, reducing the need for continuous message exchanges to reach consensus. Under every epoch, a dynamic method recalibrates node stakes based on factors such as stake transfers, token events, and economic metrics. This method employs a weighted voting system to ensure equitable and stable adjustments. By taking regular captures of the stake distribution and employing cutting-edge cryptographic techniques, verifiable proof is produced, improving transparency and guarding against attempts to manipulate stakes. 

## _3.1.1. The Elements and Purposes of Epoch_ 

## (1) **Dynamic Stake Adjustment:** 

With the network’s advancement, node stakes evolve due to various factors. Recalibrating these stakes ensures fair participation in consensus. Key Factors for Adjustment: 

Previous Stake ( _Pt_ –1): The amount of stake a node had in the previous epoch. 

Stake Movement (move _t_ ): Any stake transfers from or to a node over the course of the period. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

300 

KHANDAKAR MD SHAFIN, SAHA RENO 

Network Contribution (contrib _t_ ): Metrics like handling of transactions and uptime that assess a node’s value to the network. 

These components are used to compute the stake during the epoch that follows, _Pt_ : 

**==> picture [305 x 11] intentionally omitted <==**

The formula for modifying the stake in accordance with the given parameters is represented by the function _g_ . The consensus process is maintained dynamic and network-adaptive through this recalibration. To indicate their respective importance, incoming and outgoing stake movements are assessed and given distinct weights ( _w_ in and _w_ out). Here is how the weighted motions are calculated: Several metrics (metric _k_ ) are used to 

**==> picture [469 x 68] intentionally omitted <==**

**==> picture [351 x 16] intentionally omitted <==**

To comprehend stake distribution and behavior, statistical aspects of the preceding stake, like mean (μprev) and the standard deviation (σprev), are examined. Integrating the prior stake, weighted steps, and weighted contribution—each component weighted appropriately—determines the ultimate stake ( _Pt_ ) for the next epoch. 

**==> picture [256 x 25] intentionally omitted <==**

**==> picture [14 x 11] intentionally omitted <==**

The ability to dynamically alter stakes guarantees that stake adjustments adapt to network changes. By taking into account the weighted effect of actions and contribution measures, it adjusts stake allocation appropriately, fostering flexibility and fairness in the consensus-building process. 

(2) **Stake distribution records:** Keeping an accurate and consistent record of stake allocation at regular intervals is necessary to guarantee security and transparency in the network. To collect these records, we use zero-knowledge proof, a sophisticated cryptographic approach. The safety and authenticity of the stake dissemination information are ensured by this mechanism. 

_Recordepoch_ = _CaptureStakeDistribution_ (Network State). (4) 

The stake dissemination record for a certain epoch is denoted by Recordepoch, and the function or process that obtained the info according to the current network state is denoted by CaptureStakeDistribution. 

## _3.1.2. Implications for Security_ 

(1) **Proportional voting and stake adjustment:** According to the “one node, one vote” principle, every node or voter in a classical voting system has an equal vote. This approach might not function well in a network when members have different investments or stakes, though. Nodes with larger stakes are probably more concerned about the health of the network and have more to lose. Because of this arrangement, it is far more expensive for bad actors to have a large enough stake to control the network. Because higher engagement typically translates into more voting power, proportional voting incentivizes nodes to participate more actively in network activities. 

Let _Tj_ represent the stake related to node _j_ , and let _M_ be a collection of nodes that exist in the network. 

**==> picture [311 x 30] intentionally omitted <==**

The overall stake _T_ sum in the network is the sum of the stakes owned by all participating nodes. 

**==> picture [271 x 29] intentionally omitted <==**

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

301 

Each node’s weight _Qj_ is computed as a percentage of its stake relative to the overall stake. 

**==> picture [266 x 27] intentionally omitted <==**

A node’s weight is reflected in the outcome associated with the vote _Uj_ it casts on a proposal. 

**==> picture [262 x 12] intentionally omitted <==**

When the total of the vote values in a progressive voting system above a predetermined threshold _R_ , which tends to correspond to a supermajority, 2 _g_ + 1, agreement is attained. 

**==> picture [337 x 47] intentionally omitted <==**

With the help of this mechanism, nodes with larger stakes are guaranteed influence based on their investment. Malicious takeovers of the network would be costly and unlikely to succeed unless a significant share was accumulated. 

(2) **Defense against stake grinding attacks:** Malicious nodes regularly changing their stakes in order to unjustly affect consensus are known as stake grinding attacks. Such assaults are effectively handled by taking pictures of the stake distribution and calculating again stakes only within epoch boundaries. 

(3) **Fault tolerance and handling malicious behavior:** The consensus method uses fault-tolerance strategies to minimize the effect of Byzantine nodes, ensuring system availability and resilience. Among the main objectives are preserving availability, guarding against interruptions and malevolent attacks, and bolstering security through redundancy erasure codes and anomaly detection algorithms. Redundancy erasure codes are coding techniques used to enhance data reliability and fault tolerance in distributed storage systems. These codes break data into fragments, encode them with additional redundant pieces, and distribute them across multiple storage nodes. In the event of node failures or data corruption, the original data can be reconstructed from a subset of the fragments, ensuring data integrity and availability. 

A key component of the consensus process, epoch operations offer security, speed, and organization. The system makes sure that there is fairness, transparency, and resilience against any attacks by taking pictures of the stake allocation and dynamically changing stakes. 

## _3.2 Choosing a Slot Leader_ 

The blockchain network’s window Leader Selection procedure determines which node is permitted to produce a new block within a designated time window. This conclusion is made with the help of the Schnorr Verifiable Random Function (Schnorr VRF), which guarantees its security, objectivity, and cryptographic verifiability. The whole process for choosing a slot leader in the distributed ledger network is shown in Fig. 1. 

**3.2.1 Setting up the slot leader selection.** Nodes get the required data in advance of the slot leader selection. Every node obtains its reputation score and stake _Tj_ from the blockchain. For the next time slot, a nonce η _j_ is created. This is predicated on a timestamp, the hash value of the most recent block, or any other unexpected statistic for the node. 

**3.2.2 VRF utilization.** The objective involves employing Schnorr VRF [18] to safely produce a random, distinct, and ascertainable value Λ _j_ for every node. 

Schnorr Verifiable Random Functions (VRFs) are cryptographic primitives that provide a mechanism to generate a random value deterministically from a given input and a private key, ensuring that this value can be verified by others using the corresponding public key. Introduced by D. Chaum and T.P. Pedersen in 1992 [19], VRFs combine the properties of pseudorandomness and public verifiability, making them suitable for various applications, such as non-interactive proofs and decentralized consensus protocols. Using the created nonce η _j_ and its own stake _Tj_ , each node executes the Schnorr VRF algorithm. The Schnorr VRF calculation is used as follows: 

**==> picture [307 x 14] intentionally omitted <==**

Every node _j_ will have a distinct, random, and cryptographically verifiable _j_ value that is peculiar to the node and the time period. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

302 

KHANDAKAR MD SHAFIN, SAHA RENO 

**Fig. 1.** Flowchart Illustrating the Slot Leader Selection Procedure. 

**Working procedure of VRF:** Three steps make up the functioning process: verification, Schnorr VRF execution, plus key creation. 

(1) **Key generation:** First, we choose the Ed25519 Elliptic curve to be used in Schnorr signature implementation. Our cryptographic procedures are built upon this curve. _G_ is the basis point of this curve, and _n_ is the order of this base point. We then produce the public and private keys. The private key, SK, is selected at random from a predetermined range, usually {1, …, _n_ – 1}. After obtaining the private key, we do a straightforward multiplication operation to determine the matching public key, PK: PK = SK × _G_ . 

## **Algorithm 1.** Appointment of Slot Leader 

1: **procedure** PREPARATION 2: **for** every user _j_ **do** 3: ( _Tj_ , _Rj_ ) ← RegainStakeAndCredibility(user _j_ ) 4: η _j_ ← ProduceSlotNonce() 5: **end for** 6: **end procedure** 7: **procedure** APPLYING SCHNORR VRF 8: **for** every user _j_ **do** 9: (Λ _j_ , PK _j_ ) ← ApplySchnorrVRF( _Tj_ , η _j_ ) 10: **end for** 11: **end procedure** 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

303 

- 12: **procedure** APPLYSCHNORRVRF( _Tj_ , η _j_ ) 

13: (SK, PK) ← ProduceKeyPair() 

14: _x_ ← Hash( _Tj_ , η _j_ ) 

15: (α, β) ← GenerateSignatureOfSchnorr( _x_ , SK) 

16: _k_ ← CreateRandomNonce() 

17: _R_ ← PointMultiplication( _k_ , _G_ ) 

18: α ← _R.x_ 19: β ← ( _k_ – α · SK) mod _n_ 20: _Y_ ← Hash(α, β) 21: _R_ ' ← (α · _G_ ) + (β · PK) 

22: SchnorrSignatureValidation(α, _R_ '. _x_ , PK) 

23: **return** ( _Y_ , PK) 

24: **end procedure** 

- 25: **procedure** APPOINTING SLOT LEADER 

26: LeaderOfSelectedSlot ← SearchSlotLeaderWithMaxLambda() 

- 27: **end procedure** 

- 28: **procedure** CONFIRMATION AND DECLARATION 

29: SlotLeaderValidation(AppointedSlotLeader) 

30: DeclareSlotLeader(AppointedSlotLeader) 31: **end procedure** 

(2) **Schnorr VRF execution:** To produce a VRF outcome and related evidence, many basic procedures are followed. To begin with, the input _x_ is hashed to provide the result _h_ , which is denoted as _h_ = Hash( _x_ ). Next, given the hashed value _h_ , a Schnorr signature is generated, which is represented as (α, β). This entails choosing an arbitrary nonce _k_ from the set {1, …, _n_ – 1} using the private key SK. The formula _R_ = _k_ × _G_ is used, in which _G_ is the chosen elliptic curve’s base point. Next, the _x_ -coordinate of _R_ , or α, is calculated. Ultimately, β = ( _k_ – α × SK) mod _n_ is the value of β. One can define the final VRF output _Y_ as Hash(α, β) or as just α. 

(3) **VRF proof verification:** This procedure verifies a VRF outcome _Y_ and the evidence (α, β) that goes with it. The input _x_ is first hashed, producing _h_ = Hash( _x_ ). After that, the value of _R_ is recalculated and denoted as _R_ '. This is found using the equation _R_ ' = α × _G_ + β × PK. Then, an important validation step makes sure that α matches the _x_ -coordinate of _R_ '. Finally, depending on how the original output _Y_ was created, the finalized VRF output _Y_ is verified as either _Y_ = Hash(α, β) or just as _Y_ = α. 

The Λ _j_ values are guaranteed to be produced randomly and cryptographically securely using Schnorr VRF. A fair procedure is ensured via the Schnorr VRF, which enables nodes to validate the Λ _j_ value despite having the capacity to forecast or alter it. Schnorr VRF offers a safe basis for choosing slot leader selection and is resistant to a variety of cryptographic assaults. The VRF will provide a distinct output for every distinct input plus private key combination. This is crucial for choosing the slot leader, as there should be a different leader chosen for each time slot. 

**3.2.3. Deciding the slot leader.** Choosing a leader during the next time period is the goal. The nodes contrast the values of Λ _j_ . The slot leader is the node with the largest Λ _j_ value, or based on some other criterion, such as the lowest Λ _j_ . For that particular time slot, the slot leader _L_ is identified. A crucial component of our decentralized network, the Slot Leader Choosing procedure is shown in flowchart form in Fig. 1. 

**3.2.4. Confirmation and declaration.** Making sure that everyone on the network decides on the slot leader is the aim. Nodes use the Schnorr VRF authentication function to confirm the election results. The details of the chosen slot leader are disseminated to the network after verification. On the chosen slot leader _L_ , all nodes agree. The network’s leader selection procedure is shown in Fig. 2. 

## _3.3. Interoperability and Partitioning_ 

In order to enable the smooth flow of assets and data between different blockchains, the consensus protocol is intended to provide cross-chain interoperability [20]. This feature increases the value of the network overall and opens up a variety of use-cases, which enriches the ecosystem. Partitioning divides the 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

304 

KHANDAKAR MD SHAFIN, SAHA RENO 

**Fig. 2.** Framework for the slot leader selection process. 

network into more manageable, smaller portions, hence addressing the inherent limits of single-chain topologies. As a result, the network’s rate of transactions grows exponentially, enabling efficient scalability [21]. A graphical illustration of the partitioning and interoperability techniques is shown in Fig. 3. 

**3.3.1. Framework for sidechains.** The use of protocols like sidechain frameworks and bridges across chains is one of the essential elements of cross-chain interoperability. These protocols create reliable, strong, and secure channels for the exchange of assets and data between various blockchain platforms. 

**==> picture [256 x 12] intentionally omitted <==**

**==> picture [19 x 11] intentionally omitted <==**

In this case, _g_ stands for the intricate formula or collection of guidelines controlling the interoperability attributes. The function may be expressed as follows to assess the interoperability ( _Interopj_ ) in terms of two important variables: the efficiency of bridge protocols ( _B_ ) and the skills of sidechain frameworks ( _S_ ). 

**==> picture [185 x 12] intentionally omitted <==**

**==> picture [19 x 10] intentionally omitted <==**

For a given context, the total amount of interoperability is represented by _Interopj_ . The weight variables _wb_ and _ws_ determine how important sidechain frameworks ( _S_ ) and bridge protocols ( _B_ ). On the basis of system priorities, the weights can be changed. Based on variables including the total number of bridges ( _num_bridges_ ), each bridge’s security and dependability ( _reliability_bridge_ and _security_bridge_ ), and the difficulty of transfers ( _complexity_transfers_ ), _InteropB_ assesses the efficacy of bridge protocols. 

_InteropB_ = _w_ 1 × _num_  bridges_ + _w_ 2 × _reliability_  bridge_ + _w_ 3 × _security_  bridge_ + _w_ 4 × _complexity_  transfers_ . 

**==> picture [19 x 11] intentionally omitted <==**

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

305 

**Fig. 3.** Flowchart of cross-chain compatibility and partitioning mechanism. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

306 

KHANDAKAR MD SHAFIN, SAHA RENO 

_InteropS_ evaluates sidechain framework performance and may be computed using the following factors: 

_InteropS_ = _w_ 5 × _num_  sidechains_ + _w_ 6 × _flexibility_  sidechain_ (14) + _w_ 7 × _scalability_  sidechain_ + _w_ 8 × _usage_  sidechains_ . 

This framework’s adaptability enables a variety of scenarios. For instance, materials or data _A_ can be moved to a different blockchain _B_ for wider ecosystem interaction, or to a sidechain _S_ for certain use-cases from the main chain _M_ . 

_Atransfer_ = _move_ ( _A_ , _M_ → _S_ ) or _move_ ( _A_ , _M_ → _B_ ). (15) 

According to this equation, assets _A_ have great flexibility and usefulness since they can be moved between the main chain _M_ to a sidechain _S_ or a different blockchain _B_ . 

**3.3.2. Sectioning states and dividing networks.** The idea of partitioning is used to solve scaling problems. In order to do this, the whole blockchain network is split up into smaller, autonomous groups called shards, every among which is capable of handling transactions and preserving a portion of the network’s data. 

_Partitioning j_ = _g_ ( _network_  divisionj_ , _state_  divisionj_ ). (16) 

The algorithmic procedure utilized to implement the partitioning mechanism is indicated by the symbol _g_ in this equation. It takes into account state division to assign distinct parts of the blockchain state to each of these shards as well as network division to disperse nodes over many shards. 

The larger, more manageable states _S_ 1, _S_ 2, …, _Sn_ , which are each maintained by a separate shard, are separated from the global state _G_ , which contains all the data and agreement states. 

**==> picture [296 x 11] intentionally omitted <==**

By dividing and distributing the global state _G_ among several shards, this equation enables parallel processing and higher throughput. 

The network’s nodes _N_ are split up among different shards _Sh_ 1, _Sh_ 2, …, _Shn_ according to factors like geographical location or performance indicators. 

**==> picture [303 x 11] intentionally omitted <==**

This formula shows how the set of every single node, _N_ , is divided into several shards, each of which is in charge of a portion of the entire number of nodes. 

Not every shard in a divided architecture will be under the same load. To guarantee an equitable distribution of transactions _T_ , sophisticated load-balancing techniques _L_ are employed. 

**==> picture [315 x 12] intentionally omitted <==**

This formula shows how the workload-balancing algorithm _L_ routes transactions _T_ to the proper shard while taking the geographical distribution, performance, and current load into account. 

A flexible and strong blockchain network is produced by combining the benefits of partitioning for scalability with sidechain frameworks for cross-chain interoperability. Interoperability among blockchains allows for significant communication with different blockchain networks; network segmentation addresses scalability by breaking the network up into smaller, more manageable chunks. The activity diagram in Fig. 3, which shows the complex connections between cross-chain and splitting in our blockchain design, illustrates how these processes work together to produce the essential building blocks of a scalable, compatible, and effective blockchain system. 

## _3.4. Layer-2 Enhancements and Reward Structures_ 

Blockchain ecosystems are becoming more resource-intensive as they expand. These needs are met by incorporating Layer-2 improvements into the consensus method, which streamlines operations and allows the network to grow without sacrificing decentralization. Adoption of blockchain may be hindered by high transaction costs [22]. These expenses are greatly reduced by Layer-2 improvements, which raises the affordability and accessibility of blockchain technology for a larger audience. Creating a rewarding system that works is essential to the network’s long-term stability and security. It encourages involvement and strengthens the network’s security against any intruders. In order to address the issues of blockchain technology’s expansion, accessibility, and security and to guarantee its continued viability and applicability in the ever-changing context of digital ecosystems, our research paper combines these components. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

307 

**Fig. 4.** Rollup approach for scaling and enhancing blockchain efficiency. 

**Fig. 5.** Handling on-chain and off-chain transactions. 

**3.4.1. Rollups as Layer-2 scaling strategy.** Through the use of this Layer-2 approach, transactions may be aggregated off-chain and subsequently transmitted in batches to the primary chain. As a result, there is less data burden on the main chain, which boosts throughput and lowers expenses. 

Mathematical Representation: 

**==> picture [355 x 13] intentionally omitted <==**

The Layer-2 solution integration algorithm is denoted by _g_ in this instance. For general scalability, it takes into account optimistic rollups and state channels for micro-transactions. We provide the Rollup model in Fig. 4. 

To ensure integrity and security, advanced cryptographic structures like optimistic execution models or zero-knowledge proofs check off-chain transactions prior to their commit on-chain. A graphical depiction of the off-chain and on-chain processing of transactions is shown in Fig. 5. 

A few key elements are involved in the transaction processing process in a blockchain environment. First, there is the Off-chain Aggregator, which is in charge of gathering and combining individual transactions ( _T_ ) off-chain. This simplifies data before it is sent to the main chain, which is the main blockchain to which these combined transactions are finally published. Using cryptographic proofs—especially zeroknowledge proofs—forms a crucial layer of validation to guarantee the security and integrity of such offchain transactions before their commitment to the blockchain. This provides a reliable method of confirming the accuracy of the data before it is integrated into the blockchain. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

308 

KHANDAKAR MD SHAFIN, SAHA RENO 

Individual transactions _T_ 1, _T_ 2, …, _Tn_ are collected off-chain. 

**==> picture [308 x 48] intentionally omitted <==**

The off-chain aggregator gathers these transactions into one large batch ( _B_ ). 

Either optimistic execution models or zero-knowledge proofs are used to create a cryptographic proof ( _P_ ) for the aggregated batch. 

_P_ = _GenerateProof_ ( _B_ ). (23) For Zero-Knowledge Rollups: _P_ = _ZKSNARKs_ ( _B_ ). (24) For Optimistic Rollups: _P_ = _OptimisticExecution_ ( _B_ ). (25) One transaction _T_ main with the combined batch _B_ and proof _P_ is sent to the main chain. _T_ main = { _B_ , _P_ }. (26) In order to confirm that _B_ comprises legitimate transactions, the main chain validates the proof _P_ . _IsValid_ = _VerifyProof_ ( _P_ , _B_ ). (27) Once verified, the batch _B_ is finalized on the main chain. _Finalize_ ( _B_ ) if _IsValid_ = True. (28) 

**3.4.2. Reward alignment mechanism.** In the framework of Layer-2 improvements, we take a thorough approach to create a reliable and safe system. Our system is based on useful ideas about game theory, that enable us to comprehend reasonable behavior inside it. We make sure that good deeds yield more rewards than bad deeds. We also use DAOs to build decentralized governance, giving the community a voice in how incentives are administered. Our system is durable and flexible as it changes because of this flexibility. Our system’s economic incentives are its key component. By providing incentives like transaction charges and staking rewards, we motivate nodes to behave in desired ways. This promotes involvement that is active. We express these motivations mathematically as: 

**==> picture [345 x 13] intentionally omitted <==**

We can see from this equation how we set up financial incentives to encourage constructive participation within the Layer-2 system. 

In blockchain ecosystems, financial incentives are a fundamental component that are important for a number of reasons. They do this, first and foremost, by acting as a catalyst for node engagement in vital network operations such as smart contract execution and transaction validation. Furthermore, these incentives make harmful activity economically unreasonable, hence enhancing ecosystem security. Furthermore, the network’s long-term development and viability are guaranteed by a welldesigned incentive scheme. 

The success of these incentives depends on a number of essential elements, such as staking—a process in which nodes pledge tokens or cryptocurrency as security for network participation—rewards—which give nodes financial advantages for legitimate participation—penalties—which impose financial penalties for wrongdoing or malicious activity—and transaction fees—which are paid to process transactions. These elements work together to provide blockchain ecosystems with a strong and long-lasting economic incentive structure. A graphical summary of our system’s reward alignment is shown in Fig. 6. 

**The workings of economic incentive infrastructure:** To act as a security deposit, nodes stake a specific quantity of cryptocurrency. 

**==> picture [315 x 11] intentionally omitted <==**

Typically, the stake _S_ , transaction charges _F_ , and additional factors like uptime and performance metrics ( _M_ ) determine the payout _R_ . 

**==> picture [279 x 11] intentionally omitted <==**

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

309 

**Fig. 6.** Reward Alignment Infrastructure. 

Here, _g_ is a formula that computes the payouts based on these parameters. Depending on the particular reward model, the equation might be linear, quadratic, or in any other form. A penalty _P_ is subtracted from a node’s stake _S_ if it behaves maliciously or neglects to fulfill its obligations. 

**==> picture [275 x 12] intentionally omitted <==**

In this case, the penalty is computed using performance measurements and the stake using another function called _h_ . After penalties, the adjusted stake _S_ ' would be: 

**==> picture [52 x 8] intentionally omitted <==**

**==> picture [20 x 11] intentionally omitted <==**

Typically, nodes that take part in transaction validation split up the transaction costs _F_ . 

_F_ = Total Transaction Fees Collected. A given node i may get the following percentage of transaction fees _Fi_ : 

**==> picture [273 x 11] intentionally omitted <==**

The function _j_ in this case distributes transaction fees according to the stake and additional metrics. A node’s overall economic gain _E_ would be equal to its share of transaction fees plus its rewards, less any penalties. 

**==> picture [279 x 11] intentionally omitted <==**

To adapt to changing network circumstances and aims, the attributes _g_ , _h_ , and _j_ can be changed via decentralized governance. 

Network resilience against assaults is improved by the combination of well-aligned incentive scheme and Layer-2 solutions. Long-term stability is aided by nodes being incentivized to behave in the best interests of the network when rewards are appropriately matched. Incentives that are appropriate incentivize nodes to take part in network activities, such as transaction validation and governance contributions. The consensus process becomes extremely effective when Layer-2 solutions, such as Rollups, are integrated with a strong reward alignment scheme. Long-term network security and sustainability are guaranteed by incentive alignment, while rollups address the immediate issues of cost and scalability. In doing so, it creates the foundation for a network which is safe and scalable while also successfully addressing some of the most important issues facing blockchain technology today. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

310 

KHANDAKAR MD SHAFIN, SAHA RENO 

## 4. RESULT ANALYSIS 

In our experimental setup, we utilized a cluster of high-performance machines to rigorously evaluate our proposed consensus mechanism. Each machine was equipped with an Intel Core i5-11400 processor, 64 GB DDR4 RAM, an NVIDIA GeForce GTX 1660 Super GPU, 512 GB NVMe SSD, an ASUS Prime B560M-A WiFi motherboard, and a 100 MBPS Ethernet interface. The cluster comprised 50 machines, with one acting as the control node and the rest as worker nodes. The machines were interconnected through a dedicated 10 MBPS Ethernet switch to ensure low-latency and high-bandwidth communication. Network conditions were emulated using NetEm to introduce controlled latency (10–100 ms) and packet loss (0–5%). The software environment included Ubuntu 20.04 LTS, a custom Python implementation of our consensus mechanism, LevelDB for blockchain state storage, and OpenSSL for cryptographic operations. 

Experiments were designed to assess performance under various conditions: transaction throughput was varied from 100 to 10000 transactions per second (TPS), adversarial stake levels ranged from 10 to 45%, and fault tolerance was tested by causing node failures. Metrics for evaluation included transaction confirmation time, consensus latency, throughput, fault tolerance, and security against double-spending attacks. This comprehensive setup ensured a thorough assessment of the consensus mechanism’s performance, resilience, and scalability under diverse scenarios. 

**Performance metrics:** To evaluate the performance of our proposed consensus mechanism, we utilized several key performance metrics, each chosen to provide a comprehensive understanding of the system’s behavior and effectiveness: 

• **Transaction confirmation time:** Measures the time taken for a transaction to be confirmed, indicating the system’s efficiency. 

• **Consensus latency:** Captures the time required to reach consensus on a new block, reflecting the speed of the consensus process. 

• **Throughput:** Assessed in transactions per second (TPS), indicating the system’s scalability and capacity to handle high transaction volumes. 

• F **ault Tolerance:** Evaluates the system’s ability to function during node failures or malicious attacks, highlighting its robustness. 

• **Security against double-spending attacks:** Measures the system’s resilience to double-spending, ensuring transaction integrity under adversarial conditions. 

By analyzing these metrics, we were able to thoroughly evaluate the performance, scalability, security, and efficiency of our proposed system. These metrics provide a holistic view of the system’s capabilities, highlighting its strengths and identifying areas for potential improve-ent. 

## _4.1. Scalability Evaluation_ 

**Comparing the throughput and latency of proposed and established consensuses:** Our research examined a series of 10 blocks, each containing different numbers of transactions from 5 to 3000. We performed a total of 20 tests for each block setup to gather data on transactions per second (TPS). This thorough analysis was conducted using a network of 50 nodes to ensure the reliability and solidity of our findings. 

The results indicate that the proposed consensus mechanism significantly outperforms traditional systems in transaction per second (TPS) rates. At 1000 transactions, the proposed system achieves 1115 TPS, compared to PoA’s 264 TPS and PBFT’s 211 TPS. As transaction count increases, the proposed system scales more effectively, showcasing superior performance at higher loads. For example, at 500 transactions, it reaches 773 TPS, while DPoS only manages 183 TPS. These findings demonstrate the proposed mechanism’s scalability and efficiency, making it a promising solution for high-volume transaction environments. Figure 7 represents this comparison. 

Figure 8 displays a dual-axis clustered column chart comparing latency among various consensus mechanisms, including our proposed system. The x-axis represents transaction count, offering a way to compare latency performance as transaction volume increases. The proposed consensus mechanism demonstrates significantly lower latency compared to other established mechanisms such as proof of work (PoW), proof of stake (PoS), delegated proof of stake (DPoS), practical byzantine fault tolerance (PBFT), and proof of authority (PoA). For instance, at a transaction count of 500, the proposed system shows a latency of 47 ms, which is considerably lower than PoW’s 3211 ms and PoS’s 449 ms. Even the faster mechanisms like PBFT and PoA exhibit higher latencies of 61 ms and 56 ms respectively. This trend holds across different transaction counts, with the proposed system consistently outperforming others. At lower transaction counts, the proposed system maintains a latency close to or slightly above PoA, but significantly better than the others. This illustrates the efficiency and potential of the proposed consensus mechanism in reducing transaction latency, making it a promising alternative for faster and more efficient blockchain operations. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

311 

**==> picture [419 x 539] intentionally omitted <==**

**----- Start of picture text -----**<br>
1800<br>| PoW<br>1600 | PoS<br>| DPoS<br>1400 | PBFT<br>| PoA<br>1200<br>| Proposed system<br>1000<br>800<br>600<br>400<br>200<br>0<br>5 100 300 500 1000 1500 2000 2500 3000<br>Transaction count<br>Fig. 7.  Differences in TPS among various consensus protocols, including the proposed system.<br>700 =o PoS 120000<br>= DPoS<br>— PBFT<br>600<br>= PoA 100000<br>= Proposed system<br>500 —-o PoW<br>80000<br>400<br>60000<br>300<br>40000<br>200<br>20000<br>100<br>0 0<br>5 100 300 500 1000 1500 2000 2500 3000<br>Transaction count<br>TPS<br>Latency, ms Latency, ms<br>**----- End of picture text -----**<br>


**Fig. 8.** Variation in latency across different consensus protocol including the proposed one. 

## _4.2. Security Evaluation_ 

**4.2.1. Forkability analysis probability.** We have determined the exact probabilities of a fork occurring for specific values of _m_ (the number of trials in terms of blocks/time slots). To understand the frequency at which certain sequences can lead to forks, we precisely calculated the likelihood of adversaries causing a fork. The probability of observing exactly _j_ forks ( _j_ successes) in _m_ trials (blocks) with a probability of success _q_ is given by the binomial distribution formula: 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

312 

KHANDAKAR MD SHAFIN, SAHA RENO 

**==> picture [226 x 143] intentionally omitted <==**

**----- Start of picture text -----**<br>
0.6<br>n  = 700<br>0.5 n  = 1400<br>n  = 2100<br>n  = 2800<br>0.4<br>0.3<br>0.2<br>0.1<br>0.0<br>0.40 0.41 0.42 0.43 0.44 0.45 0.46 0.47 0.48 0.49 0.50<br>Binominal distribution parameter<br>Probability of covert for kability<br>**----- End of picture text -----**<br>


**Fig. 9.** Charts showing the probability of forks across various binomial distributions. 

**==> picture [308 x 28] intentionally omitted <==**

 _m_  where   is the binomial coefficient, calculated as _m_ !/( _j_ ! · ( _m_ – _j_ )!). _q_ is the probability of success in each  _j_  trial (the probability of adding a new block). (1 – _q_ ) is the probability of failure in each trial (the probability of not adding a new block). _j_ is the number of successes (forks) for which we want to calculate the probability. _m_ is the total number of blocks in which the forks can occur. 

We substituted the values of _m_ , _q_ , and _j_ into the binomial distribution formula and calculated the probability of having exactly _j_ forks in _m_ blocks. The results of these calculations are presented in Fig. 9. 

**4.2.2. Analysis of double spending attack.** In Table 1, we present our comparative analysis, which includes five additional consensus mechanisms: proof of stake (PoS), delegated proof of stake (DPoS), practical byzantine fault tolerance (PBFT), proof of authority (PoA), and our proposed consensus system. The evaluation focuses on the resilience against double-spending attacks under different levels of adversarial stake participation. 

Table 1 shows the time required to confirm approximately 21 thousand transactions for each consensus mechanism, considering varying adversary stake percentages from 10% to 45%. This analysis highlights the effectiveness of each consensus model in mitigating double-spending attacks under diverse adversarial conditions. 

Starting with the widely used proof of work (PoW) consensus, we note an increase in confirmation time as the adversary stake rises. In contrast, our proposed system consistently exhibits superior performance, with significantly shorter confirmation times across all adversary stake levels. For instance, with a 45% adversarial stake, PoW requires 141 min, while our proposed system achieves the same security level in just 23 min. 

**Table 1.** The time required for transaction confirmation (in minutes) to achieve 99.9% protection against double spending attack Adversary PoW, PoS, DPoS, PBFT, PoA, Proposed stake min min min min min system, min 0.10 62.00 10.00 7.00 5.00 15.00 7.00 0.15 71.00 12.00 8.00 9.00 18.00 8.00 0.20 82.00 14.00 9.00 12.00 21.00 11.00 0.25 94.00 19.00 12.00 14.00 23.00 13.00 0.30 109.00 20.00 14.00 16.00 27.00 15.00 0.35 116.00 23.00 16.00 18.00 32.00 19.00 0.40 133.00 25.00 18.00 20.00 34.00 21.00 0.45 141.00 29.00 19.00 24.00 36.00 23.00 ~~TTT~~ AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

313 

**==> picture [361 x 250] intentionally omitted <==**

**----- Start of picture text -----**<br>
14<br>PoA<br>PoS<br>12<br>PBFT<br>Proposed system<br>10<br>8<br>6<br>4<br>2<br>0<br>0.10 0.15 0.20 0.25 0.30 0.35 0.40 0.45<br>Speedup consensus, PoW<br>**----- End of picture text -----**<br>


**Fig. 10.** Speed-up results of PoS, DPoS, PBFT, PoA and proposed system. 

When compared to proof of stake (PoS), delegated proof of stake (DPoS), practical byzantine fault tolerance (PBFT), and proof of authority (PoA), our proposed consensus consistently outperforms these alternatives in terms of confirmation time. Even at higher adversary stakes, our system demonstrates a remarkable ability to maintain a low probability of successful double-spending attacks. 

In addition to the pictorial representation of the speedup in Fig. 10, we are comparing the acceleration in transaction confirmation time between proposed system and Bitcoin in the context of a theoretical double spending attacker, with a confidence level of 99.9% based on the data from Table 1. When confronted with an adversarial stake ranging from 10 to 45%, PoS consistently exhibits a speedup compared to PoW. The speedup ranges from approximately 4.86 times faster to 6.2 times faster, DPoS speedup ranges from around 7.42 times faster to 8.86 times faster, PBFT is approximately 5.88 times faster to 12.4 times faster, PoA is around 3.92 times faster to 4.13 times faster and our proposed System speedup ranges from approximately 6.13 times faster to 8.86 times faster compared to PoW to defend the adversarial attack. 

In addition to the graphical representation of speedup in Fig. 10, we compare the acceleration in transaction confirmation times between our proposed system, PoS, DPoS, PBFT, PoA with Bitcoin(PoW), considering a theoretical double-spending attacker with a confidence level of 99.9% based on the data from Table 1. 

When faced with an adversarial stake ranging from 10 to 45%, PoS consistently shows a speedup over PoW, ranging from approximately 4.86 to 6.2 times faster. DPoS achieves a speedup of about 7.42 to 8.86 times faster, PBFT ranges from roughly 5.88 to 12.4 times faster, and PoA ranges from about 3.92 to 4.13 times faster. Our proposed system demonstrates a speedup of approximately 6.13 to 8.86 times faster compared to PoW, effectively defending against adversarial attacks. 

## _4.3. Evaluation of the Degree of Decentralization_ 

We conducted a comparative analysis of the decentralization scores across various blockchain systems to highlight the strengths and weaknesses of our proposed system. The evaluation considers several critical parameters, such as Governance, geographical distribution of nodes, resilience to attacks, and others, each scored on a scale from 0 to 10. These individual scores were aggregated into a composite score, which serves as a comprehensive measure of each system’s overall decentralization. Our proposed system, with a decentralization score of 7.181, is categorized as “Solid”. In the ranking, it stands below Monero, Bitcoin, Cardano, and Ethereum, which are classified as “Excellent” and “Strong” with scores ranging from 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

314 

KHANDAKAR MD SHAFIN, SAHA RENO 

9 

**==> picture [415 x 382] intentionally omitted <==**

**----- Start of picture text -----**<br>
8<br>7<br>6<br>5<br>4<br>3<br>2<br>1<br>0<br>Blockchain systems<br>Decentralization score<br>Monero Bitcoin Cardano Ethereum Tezos Litecoin Algorand Bitcoin Cash Dash Solana Avalanche Proposed system Polkadot Cosmos Nano IOTA Bitkoin SV Chainlink BSC Stellar Tron NEO Ripple Hedera Hashgraph EOS VeChain<br>**----- End of picture text -----**<br>


**Fig. 11.** Ranking various consensus mechanisms and our proposed system based on their decentralization scores. 

8.092 to 7.634. This indicates that while our proposed system exhibits substantial decentralization, there is still potential for enhancement when compared to the leading blockchain systems. Furthermore, our system’s performance aligns with that of Algorand, Bitcoin Cash, and Dash, which also fall within the “Solid” category. This suggests that our system possesses competitive decentralization features comparable to these established blockchains. Notably, it surpasses systems in the “Acceptable” category, such as Chainlink and Stellar, which have scores between 6.817 and 6.183, and significantly outperforms those in the “Needs Improvement” category, including Ripple and VeChain, with scores below 6.002. In summary, our proposed system demonstrates a strong level of decentralization, positioning it competitively among its peers. However, there are opportunities for further refinement to achieve the “Excellent” category and enhance its standing in the blockchain ecosystem. The comparison is graphically represented in Fig. 11. 

## 5. CONCLUSIONS 

This work introduces a hybrid consensus method that successfully strikes a balance between security, scalability, and decentralization, thus resolving the blockchain trilemma. We did this by utilizing a number of cutting-edge strategies and tactics. Two important advances are epochbased organization for efficiency and compatibility with historical records, and adaptive stake recalculation, which guarantees equitable influence in consensus. Randomness is added by using Schnorr VRF for slot leader election, and scalabil- 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

RESOLVING THE TRILEMMA CHALLENGE IN BLOCKCHAIN 

315 

ity is improved without sacrificing decentralization by using Layer-2 solutions like Rollups. Moreover, redundancy erasure codes, cross-chain compatibility, and zero-knowledge proofs are integrated to improve network efficiency, security, and interoperability. Our system is notable for its exceptional performance, able to process more than 1600 transactions per second with constant low latency, even under heavy loads of up to 3000 transactions at once. Notably, with node setup costs under $1000, we have significantly reduced the hardware costs for validators, making blockchain participation affordable and accessible. Crucially, our consensus process improves the overall integrity and security of the blockchain network by successfully guarding against problems like forkability and double spending. 

## AUTHOR CONTRIBUTION 

Khandaker Md Shafin: conceptualisation, data curation, formal analysis, investigation, methodology, resources, software, validation, visualization, writing—original draft, writing—review and editing. 

Saha Reno: conceptualisation, data curation, formal analysis, investigation, methodology, project administration, resources, software, supervision, validation, visualization, writing—original draft, writing—review and editing. 

## FUNDING 

This research was conducted without external funding and was solely supported by the personal finances of the authors. 

## DATA AVAILABILITY 

The underlying data supporting the results of this study is available upon request. Please contact the corresponding author, Saha Reno at reno.saha39@gmail.com to request access to the dataset. We are committed to providing access to our data while ensuring the confidentiality and ethical considerations associated with the research. 

## CONFLICT OF INTEREST 

The authors of this work declare that they have no conflicts of interest. 

## REFERENCES 

1. Teoh, B.Ph.Ch., Navigating the blockchain trilemma: A supply chain dilemma, _Advanced Maritime Technologies and Applications: Papers from the ICMAT 2021_ , Ismail, A., Dahalan, W.M., and Öchsner, A., Eds., Advanced Structured Materials, vol. 166, Cham: Springer, 2022, pp. 291–300. https://doi.org/10.1007/978-3-030-89992-9_25 

2. Reno, S. and Haque, Md.M., Solving blockchain trilemma using off-chain storage protocol, _IET Inf. Secur._ , 2023, vol. 17, no. 4, pp. 681–702. https://doi.org/10.1049/ise2.12124 

3. Al-Kafi, G.M.A., Golam, A., Faiza, J.T., Pal, K.R., and Reno, S., SHBF: A secure and scalable hybrid blockchain framework for resolving trilemma challenges, _International Journal of Information Technology_ , 2024, vol. 16, no. 6, pp. 3879–3890. https://doi.org/10.1007/s41870-024-01897-9 

4. Aitzhan, N.Zh. and Svetinovic, D., Security and privacy in decentralized energy trading through multi-signatures, blockchain and anonymous messaging streams, _IEEE Trans. Dependable Secure Comput._ , 2018, vol. 15, no. 5, pp. 840–852. https://doi.org/10.1109/tdsc.2016.2616861 

5. Nguyen, C.T., Nguyen, D.N., Niyato, D., Tuong, N.H., and Dutkiewicz, E., Proof-of-stake consensus mechanisms for future blockchain networks: Fundamentals, applications and opportunities, _IEEE Access_ , 2019, vol. 7, pp. 85727–85745. https://doi.org/10.1109/access.2019.2925010 

6. Wang, W., Hoang, D.T., Hu, P., Xiong, Z., Niyato, D., Wang, P., Wen, Yo., and Kim, D.I., A survey on consensus mechanisms and mining strategy management in blockchain networks, _IEEE Access_ , 2019, vol. 7, pp. 22328– 22370. 

   - https://doi.org/10.1109/access.2019.2896108 

7. Xiao, Ya., Zhang, N., Lou, W., and Hou, Y.T., A survey of distributed consensus protocols for blockchain networks, _IEEE Commun. Surv. Tutorials_ , 2020, vol. 22, no. 2, pp. 1432–1465. https://doi.org/10.1109/comst.2020.2969706 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

316 

## KHANDAKAR MD SHAFIN, SAHA RENO 

8. Lashkari, B. and Musilek, P., A comprehensive review of blockchain consensus mechanisms, _IEEE Access_ , 2021, vol. 9, pp. 43620–43652. https://doi.org/10.1109/access.2021.3065880 

9. Khan, D., Jung, L.T., and Hashmani, M.A., Systematic literature review of challenges in blockchain scalability, _Appl. Sci._ , 2021, vol. 11, no. 20, p. 9372. https://doi.org/10.3390/app11209372 

10. Atzori, M., Blockchain technology and decentralized governance: Is the state still necessary?, _SSRN Electronic Journal_ , 2015. https://doi.org/10.2139/ssrn.2709713 

11. Liu, J., Li, W., Karame, G.O., and Asokan, N., Scalable Byzantine consensus via hardware-assisted secret sharing, _IEEE Trans. Comput._ , 2018, vol. 68, no. 1, pp. 139–151. https://doi.org/10.1109/tc.2018.2860009 

12. Gilad, Yo., Algorand: Scaling Byzantine agreements for cryptocurrencies, _Proceedings of the 26th Symposium on Operating Systems Principles_ , 2017, pp. 51–68. https://doi.org/10.1145/3132747.3132757 

13. Nakamoto, S., _Bitcoin: A peer-to-peer electronic cash system_ , _Decentralized Business Review_ , 2008. 

14. Buterin, V. et al., A next-generation smart contract and decentralized application platform, _White Paper_ , 2014, vol. 3, no. 37, p. 2-1. 

15. Eyal, I. et al., Bitcoin-NG: A scalable blockchain protocol, _13th USENIX Symposium on Networked Systems Design and Implementation (NSDI 16)_ , 2016, pp. 45–59. 

16. Sousa, J., Bessani, A., and Vukolic, M., A Byzantine fault-tolerant ordering service for the hyperledger fabric blockchain platform, _2018 48th Annual IEEE/IFIP International Conference on Dependable Systems and Networks (DSN)_ , Luxembourg, 2018, IEEE, 2018, pp. 51–58. https://doi.org/10.1109/dsn.2018.00018 

17. Castro, M. and Liskov, B., Practical Byzantine fault tolerance, _Proceedings of the Third Symposium on Operating Systems Design and Implementation_ , New Orleans, 1999, Berkeley, CA: USENIX Association, 1999, vol. 99, pp. 173–186. 

18. Komlo, Ch. and Goldberg, I., FROST: Flexible round-optimized schnorr threshold signatures, _Selected Areas in Cryptography_ , Dunkelman, O., Jacobson, Jr., M.J., and O’Flynn, C., Eds., Lecture Notes in Computer Science, vol. 12804, Cham: Springer, 2021, pp. 34–65. https://doi.org/10.1007/978-3-030-81652-0_2 

19. Chaum, D. and Pedersen, T.P., Wallet databases with observers, _Advances in Cryptology—CRYPTO’ 92_ , Brickell, E.F., Ed., Lecture Notes in Computer Science, vol. 740, Berlin: Springer, 1993, pp. 89–105. https://doi.org/10.1007/3-540-48071-4_7 

20. Hei, Yi., Li, D., Zhang, Ch., Liu, J., Liu, Yi., and Wu, Q., Practical AgentChain: A compatible cross-chain exchange system, _Future Gener. Comput. Syst._ , 2022, vol. 130, pp. 207–218. https://doi.org/10.1016/j.future.2021.11.029 

21. Liu, Yi., Liu, J., Vaz Salles, M.A., Zhang, Z., Li, T., Hu, B., Henglein, F., and Lu, R., Building blocks of sharding blockchain systems: Concepts, approaches, and open problems, _Comput. Sci. Rev._ , 2022, vol. 46, p. 100513. https://doi.org/10.1016/j.cosrev.2022.100513 

22. Gangwal, A., Gangavalli, H.R., and Thirupathi, A., A survey of Layer-two blockchain protocols, _J. Network Comput. Appl._ , 2023, vol. 209, p. 103539. https://doi.org/10.1016/j.jnca.2022.103539 

**Publisher’s Note.** Allerton Press remains neutral with regard to jurisdictional claims in published maps and institutional affiliations. 

AI tools may have been used in the translation or editing of this article. 

AUTOMATIC CONTROL AND COMPUTER SCIENCES Vol. 59 No. 3 2025 

