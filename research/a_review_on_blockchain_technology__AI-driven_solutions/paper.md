## **A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions** 

MOETEZ ABDELHAMID, ISITCOM, University of Sousse, Hammam Sousse, Tunisia LAYTH SLIMAN, Efrei Research Lab., Efrei Paris Pantheon-Assas University, Villejuif, France RAOUDHA BEN DJEMAA, ISITCOM, University of Sousse, Hammam Sousse, Tunisia GUIDO PERBOLI, Politecnico di Torino, Torino, Italy 

Blockchain provides several advantages, including decentralization, data integrity, traceability, and immutability. However, despite its advantages, blockchain suffers from significant limitations, including scalability, resource greediness, governance complexity, and some security related issues. These limitations prevent its adoption in mainstream applications. Artificial Intelligence (AI) can help addressing some of these limitations. This survey provides a detailed overview of the different blockchain AI-based optimization and improvement approaches, tools and methodologies proposed to meet the needs of existing systems and applications with their benefits and drawbacks. Afterward, the focus is on suggesting AI-based directions where to address some of the fundamental limitations of blockchain. 

## CCS Concepts: • **Security and privacy** → _Systems security_ ; _Distributed systems security_ ; 

Additional Key Words and Phrases: Blockchain, distributed ledger technology, smart contract, optimization, AI, ML 

## **ACM Reference Format:** 

Moetez Abdelhamid, Layth Sliman, Raoudha Ben Djemaa, and Guido Perboli. 2024. A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions. _ACM Comput. Surv._ 57, 3, Article 73 (November 2024), 39 pages. https://doi.org/10.1145/3700641 

## **1 Introduction** 

Blockchain technology enables decentralized record-keeping through distributed ledgers shared across peer-to-peer networks. First proposed in 2008 as the underlying framework for Bitcoin [125], blockchain provides advantages like tamper-resistance, transparency, and elimination of central authorities. These features make blockchain appealing for applications like healthcare, supply chain management, and financial systems [113]. However, blockchain also faces adoption limitations including lack of scalability, performance, security vulnerabilities, high energy consumption, and storage requirements [66]. Researchers have sought solutions to optimize blockchain and 

Authors’ Contact Information: Moetez Abdelhamid, ISITCOM, University of Sousse, Hammam Sousse, Sousse, Tunisia; e-mail: mohamed.moaetez@gmail.com; Layth Sliman (Corresponding author), Efrei Research Lab., Efrei Paris PantheonAssas University, Villejuif, France; e-mail: layth.sliman@efrei.fr; Raoudha Ben Djemaa, ISITCOM, University of Sousse, Hammam Sousse, Sousse, Tunisia; e-mail: raoudhaham@yahoo.fr; Guido Perboli, Politecnico di Torino, Torino, Piemonte, Italy; e-mail: guido.perboli@polito.it. Permission to make digital or hard copies of all or part of this work for personal or classroom use is granted without fee provided that copies are not made or distributed for profit or commercial advantage and that copies bear this notice and the full citation on the first page. Copyrights for components of this work owned by others than the author(s) must be honored. Abstracting with credit is permitted. To copy otherwise, or republish, to post on servers or to redistribute to lists, requires prior specific permission and/or a fee. Request permissions from permissions@acm.org. © 2024 Copyright held by the owner/author(s). Publication rights licensed to ACM. ACM 0360-0300/2024/11-ART73 https://doi.org/10.1145/3700641 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:2 

address these challenges. Approaches involve enhancements across the blockchain architecture layers, including rollups, payment channels, multi-chain mechanisms, block data compression, and consensus protocols. 

This article provides a comprehensive analysis of existing solutions, designed to address the blockchain challenges. This is done in order to identify the problems that have not been well studied or not yet resolved and that can be addressed by the research community in the future. Thus, we analyze existing solutions and compare the different approaches that are sought to improve and optimize blockchain. More specifically, we are focusing on **machine learning** ( **ML)** and **artificial intelligence** ( **AI** )-based solutions. This is due to the fact that it is believed that AI and ML are major driving forces that can significantly contribute to resolving and mitigating blockchain drawbacks. 

The rest of the article is organized as follows: Section 2 defines this study’s goals and presents our study contributions and methodologies. In Section 3, we discuss the findings to answer our defined Research Questions by describing and analyzing the existing solutions, mainly the AI & ML-based solutions. In the final part of this section, we provide research direction for further solutions or disclosing issues that need to be resolved. Section 4 concludes this article. 

## **2 Methodology for Achieving Research Goals** 

In this study, we aim to analyze existing works and findings and we summarize the research done to improve and optimize blockchain technology. We are particularly interested in AI and ML-based works. Hence, we define the following **Research Questions** ( **RQ** ) and attempt to answer them in this study. 

- **RQ1** What are the most innovative emerging applications of Blockchain technology in the different sectors? 

- **RQ2** What are the primary challenges impeding the widespread adoption of Blockchain? 

- **RQ3** What are state-of-the-art solutions being developed to address the key challenges faced by Blockchain technology? 

- **RQ4** How can the integration of Artificial Intelligence enhance the capabilities and applications of Blockchain technology? 

- **RQ5** What are the promising AI-driven solutions currently being utilized in Blockchain, and what novel opportunities do they present? 

- **RQ6** What critical issues remain unresolved in Blockchain technology, and how can AI offer potential solutions for them? 

To answer these research questions, we followed the guidelines for conducting a systematic literature review as outlined by David Budgen and Pearl Brereton [39] which are: planning, conducting, and reporting. In what follows, we describe the method we used to find the most prominent and significant articles and studies that match to our objectives. 

## **Search Strategy** 

To identify a list of pertinent studies that have addressed blockchain issues, we used specific keyword combinations in the search string. The searches were conducted according to the title, keywords, and abstract. 

The primary and secondary keywords utilized for searches are shown in Table 1. Keywords are essential to finding pertinent literature for the research. According to the study’s objective, specific sentences were chosen to reach a clear vision of how to optimize blockchain through different solutions. In our research, the term “Blockchain” was used as the fundamental keyword along with the term “Optimization” as a primary keyword. All the findings that have been retrieved are from 2015 to the end of 2023. This demonstrates that the topic is new because, before 2015, no 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:3 

Table 1. Search Keywords 

|Fundamental keywords|Blockchain|
|---|---|
|Primar y key words|Optimization, Improvement|
|Secondar y Key words|Distributed ledger, Smart contract|



Table 2. Inclusion/Exclusion Criteria 

|Inclusion|Exclusion|
|---|---|
|1. The selected document needs to highlight|1. The conference version of research that has|
|optimizations made to the blockchain.|been published in the journal.|
|2. Selected academic research papers must be|2. Studies just focusing on improving applications|
|empirical research about Blockchain optimization.|and solutions using Blockchain.|
|3. Selected academic research papers must be||
|reviewed in a conference or journal||
|4. Whitepapers should be cited several times in||
|published research works.||



Fig. 1. Search strategy. 

study has been done in the mentioned domain. Our survey focused solely on articles, white papers and conference papers. 

In the first phase, the results of our study were filtered using the criteria for inclusion and exclusion as illustrated in Table 2 above to reach works that are interested in optimizing blockchain. 

## **Search Query** 

The query strings were as follows: (“Blockchain” OR “distributed ledger” OR “smart contract”) AND (“Optimization” OR “Problems” OR “Improvement”). 

Figure 1 below illustrates our search strategy. Around 158 documents were obtained from the previous query executed on multiple search engines for scientific articles and publications. All 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:4 

Table 3. Comparative Study of This Survey with Existing Surveys 

|**Aspect**|**Our** **Survey**|**Existing** **Surveys** **[** **6** **,** **72** **,** **80** **,** **100** **,** **121** **,** **146** **,**<br>**152** **,** **196** **,** **207** **]**|
|---|---|---|
|Scope|Comprehensive review of AI-optimized<br>blockchain solutions.|Various applications of blockchain and AI<br>in IoT, AIoT, edge computing, and so on.|
|||Decentralization of AI applications,|
|Challenges|Sustainability, Scalability, Efciency, Security,|Decentralized intelligence, Data|
|Addressed|Privacy, Governance and dependency.|Management, Sustainability, Scalability,|
|||Efciency, Security, Privacy.|
|Solutions<br>Discussed|AI-enhanced blockchain solutions.|General blockchain and AI solutions for IoT<br>and AI applications.|
|||General integration of AI and blockchain,|
|Unique<br>Insights|Emphasis on AI optimization in blockchain<br>technology.|harnessing the advantages aforded by<br>blockchain technology such as transparency<br>and security–without focusing on resolving|
|||existing blockchain challenges.|
|Future|Proposes future research directions focusing|General future directions in AI and|
|Directions|on AI-driven blockchain optimizations.|blockchain research.|



papers were analyzed through metadata such as the title of the work, the source, the author’s name, keywords, the count of citations, cited references, and publication year. Based on our objective of finding solutions related to blockchain optimization, and using the inclusion/exclusion criteria we indicated, the number of remaining papers was 74. These publications were enough to answer our first three research questions. Then to answer the rest of the questions, we retained 30 research papers that match to our focus on AI and ML-Based solutions. 

After selection, to extract detailed information for our survey, the 74 papers were read thoroughly. From each paper, we extract relevant data and categorize them. We found that the used optimization techniques we could identify were Sharding, Segregated Witness, Hard Forks, State Channels, Sidechains, Lightning Network, Cross chain, Rollups, Consensus protocol optimization, and AI or ML-based solutions. We selected the 30 studies and contributions that focus on optimizing and resolving blockchain issues using AI and ML. Based on these works, we introduced state-of-the-art methodologies and solutions for resolving blockchain problems by using AI. In the end, we provided guidelines to aid future research in this direction. 

The following Table 3 compares key aspects of our survey with those of existing surveys to underscore our unique contributions and the specific focus areas addressed in our research. This comparison highlights the comprehensive nature of our review, the challenges we address, the solutions discussed, and the future directions proposed, positioning our survey as a valuable resource for researchers and practitioners aiming to explore AI-optimized blockchain solutions. 

## **3 Discussion** 

In this section, we delve into the supporting evidence for the various research questions. Each one is allocated its own distinct subsection. Within each subsection, we present the overarching evidence, primary industry and research findings, and engage in discussions regarding potential future trajectories. 

## **3.1 What are the Most Innovative Emerging Applications of Blockchain Technology in the Different Sectors?** 

Blockchain technology has many interesting properties such as trustfulness, traceability, transparency and decentralization [151, 161, 162]. Experts believe that blockchain technology has the 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:5 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

potential to become the next breakthrough technology following the Internet due to the significant advantages it has. 

This has a wide range of applications, as the technology can be used to provide solutions in fields including supply chain, voting, healthcare, public administration, identity management, and financial markets [113, 134]. 

For instance, MedChain [58] is a solution based on permissioned blockchain, which gives patients full control over their own medical records. Smart contracts can increase the efficiency and trustfulness of health care systems [135]. By eliminating intermediaries and providing decentralization and automation whilst providing transparency and trust. Blockchain technology also has the potential to address some major issues in financial markets such as the delay generated the traditional intermediated stock markets processes [96, 175]. Additionally, blockchain can be used for voting systems to increase voting systems’ trustworthiness [1]. In other domains, blockchain can be used to support the insurance marketplace transactions between insurance companies, clients, and policyholders. Smart contracts have been used to help insurance businesses negotiate, procure, and register insurance policies [74]. Blockchain has been used for personal identity management to store, validate and protect an individual’s identity from theft or being used in fraudulent activities [16, 67]. In addition, blockchain is an excellent choice for addressing supply chain problems, as it operates as a decentralized Peer-to-Peer mechanism that establishes trust, traceability and transparency between participants who took part in different activities over time [133, 154, 183, 185]. 

Furthermore, some of the most cutting-edge and emerging applications of blockchain technology encompass the metaverse, **non-fungible tokens** ( **NFTs** ), and decentralized identity management. The metaverse, an expansive virtual reality environment where users can engage and interact, utilizes blockchain to secure transactions, verify ownership, and enable interoperability across different virtual platforms. For instance, Decentraland is a blockchain-based virtual world where users can create, explore, and trade virtual properties and assets [97]. NFTs, which are unique digital assets authenticated via blockchain, have transformed the realms of digital art, collectibles, and intellectual property by ensuring authenticity and rarity. An example is CryptoPunks [205], a collection of unique 24x24 pixel art images, each verified on the Ethereum blockchain and highly valued in the digital art market [189]. In the realm of identity management, blockchain facilitates decentralized systems that grant individuals control over their personal data, enhancing privacy, security, and providing seamless authentication across various services without relying on central authorities. SelfKey [155] is an example of a blockchain-based identity management system that allows users to securely store their personal information and authenticate their identity for various services [124, 203]. 

These pioneering applications highlight blockchain’s adaptability and its transformative impact across multiple sectors, indicating its significant role in the future of technology and digital innovation. 

## **3.2 What are the Primary Challenges Impeding the Widespread Adoption of Blockchain?** 

Blockchain, despite its advantages, has still has prominent limitations, i.e., _security_ and _scalability/performance_ , that hinder its adoption in mainstream applications. In this section, we first discuss the security limitations and then switch to scalability/performance. 

## **Security Limitations** 

We study the security limitation in the context of _attacks_ and _vulnerabilities_ . To better picture and describe this section, a table that explains attacks with the details regarding the cause of the 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:6 

attack, the layer of the blockchain that is affected as the result of the attack, the affected blockchain processes (note that in our categorization, blockchain consists of different processes, explained next), the actor that is affected because of the attack, and the version of the blockchain that the attack can target. 

Attacks have different types that can target specific blockchain layers, they can be triggered by specific entities/tasks in the blockchain. Thus, to better describe how an attack is generated, what parts/layers/versions of blockchain are affected, and what the reasons are behind each attack, we have provided the following categorization for a blockchain system. Then, when describing each attack, we refer back to the categorized attacks. 

- **Blockchain versions:** Blockchain since its invention has four versions, version 1.0, 2.0, 3.0, and 4.0 as described below: 

- **1.0:** Transaction-based blockchain. 

- **2.0:** Transaction and Smart Contact-based blockchain. 

- **3.0:** DApp (Decentralized Application) based blockchain, including the Metaverse and NFTs. 

- **Blockchain layers :** Blockchain is made up of layers that together form a full solution that ranges from data management to allowing user-facing applications. The following five levels make up the blockchain architecture layers: 

- **Network layer:** P2P, Verification protocols. 

- **Consensus layer:** Consensus Protocols PoS, PoW, and so on. 

- **Data layer:** Data Block, Chain structure, merkle tree, hash function, timestamp, and so on. 

- **Execution layer:** EVM, compiler script code. 

- **Application layer:** Cryptocurrencies, asset management. 

- **Blockchain Processes:** a Blockchain process is the execution of a list of instructions that can be executed by one or many entities. Following are the detailed Blockchain processes: 

- **Network discovery:** Finding peers and informing them that a new peer has joined the blockchain network. 

- **Transaction creation:** Generally, to initiate a transaction, users provide the value and the address of the beneficiary, and then the transaction is signed and broadcasted to the network. The closest peers validate the transaction, and if it is valid, they forward it to other peers. 

- **Mining:** Mining is based on a consensus mechanism. In mining, the miner selects unverified transaction from mempool and add meta-data, and calculate the proof for the new block. 

- **Block validation:** When the miner broadcasts a new block, every node performs the validation process. 

- **Blockchain Actors:** Any entity that can perform an action or participate in a blockchain network is known as a Blockchain actor, all actors are listed bellow: 

- **Miner:** An actor in charge of transaction verification and validation, creating, signing, and publishing new blockchain blocks. 

- **Mining Pool:** Miners form a pool when they pool their resources to solve the problem. Due to the complexity of some blockchains, like Bitcoin, it is practically impossible for a single tiny miner to resolve the proof task, they pool their computing power to do the task collectively. 

- **User:** Any person who creates a blockchain transaction. 

- **Seller:** Any person or company that accepts cryptocurrency for payments. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:7 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

- **Exchange:** Any entity that provides for their clients a trade crypto-currencies service, that let them exchange their crypto-currencies to traditional money or other cryptocurrencies. 

- **Blockchain Network:** It consists of the network peers, that maintain the proper functioning of the blockchain and its data validity. 

Now, we explain different attacks (see Table 10 in the appendix for a detailed list of the attacks). 

- **Vulnerability:** it refers to deficiency or flaw that might lead to an attack. [130] 

- **Attack:** It is a generic noun for items or entities that provide a persistent threat to an asset [52]. 

## — **Blockchain attacks/vulnerabilities categories:** 

- **Double-spending attacks:** A double-spending attack occurs when a user expenditure the same units of a crypto-currency more than once, in other words, [87, 148], using a set of coins several times. 

- **Mining Pools attacks:** A mining pool is formed by a collection of miners that collaborate by sharing their computing power for the block creation process, and then share the mining reward depending to their computing power used in the process. The pool-based mining method includes flaws that allow for both internal and external attacks [49]. Pool dishonest miners are those who commit internal attacks in order to get more than they should have of the reward or to disrupt the operation of the pool’s honest miners and distance them from successful mining attempts. In external attacks targeting pools, dishonest miners in the pool might exploit their superior hash power to execute a double spending attack. 

- **Wallet attacks:** It occurs when a hacker or scammer tries to break the privacy of blockchain users and steals the user’s passphrase or private keys. Unfortunately, it is possible due to the flaws in the algorithms of the encryption [36] and the generation of the public and private keys [159], Clients’ Wallet has the risk to being attacked. 

- **Blockchain Network attacks:** The blockchain network is defined by its p2p nature, which contains all nodes that maintain and administer the blockchain protocols. Blockchain Network attack is practically similar to other distributed network attacks, which represents any violation of the network protocols [197]. To sum up, a blockchain network attack happens when cybercriminals or blockchain network intruders look for blockchain network vulnerabilities and exploit them to manipulate network flow for their own profit. 

- **Smart Contract attacks:** A smart contract attack is a type of exploit that targets vulnerabilities in the code of a smart contract (see Table 11). An attacker can use various methods to manipulate or disrupt the normal functioning of the contract and steal funds or gain unauthorized access. Generally, smart contract security attacks come from script bugs or conditions [12]. 

## **Scalability Limitations** 

Blockchain is seen as a highly effective technology for the upcoming future and it is gaining more attraction with each day, but unfortunately, it is still not adopted by most of the current solutions. Businesses and government decision-makers are still hesitant about blockchain due to its scalability/performance issues, which remain a hot topic. Blockchain scalability/performance issues are always related to blockchain network traffic, high blockchain traffic can lead to challenges in handling an increasing number of transactions within the agreed timeframes. The number of nodes in the network, the complexity of the consensus protocol, and the size of data that must be kept on the blockchain are among the factors that impact blockchain scalability/performance (see Table 9). 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:8 

Over the course of the last decade, noteworthy things have been observed in blockchain, driven by the increasing adoption of blockchain technology across various platforms. Below, we have compiled a list of them. 

- —The average transaction confirmation time has increased. 

- —Increasing the block size which led the system to become more costly, slower, and unsustainable. 

- —The difficulty of mining a block has increased and hence the computation power and resources needed for the blockchain’s proper functioning have also increased. 

- —Network transaction fees have increased. 

There are factors that directly influence the scalability/performance of blockchain, we listed below some of them. 

- **Consensus mechanism:** This refers to the method used in the blockchain network to obtain agreement among diverse dispersed nodes on a single common blockchain state [206]. This is usually accomplished through a voting procedure. Consensus mechanisms are based on protocols and algorithms that ensure transaction propagation, validation, and finalization over the blockchain network. As the consensus mechanism is the main part of a blockchain, the choice of a consensus mechanism is closely tied to the performance of the blockchain network [95]. 

- **Transaction Throughput:** This refers to the rate of valid transactions that the protocol can confirm in a defined time period (usually per second) [123]. 

- **Confirmation time (Aka Latency)** This indicates the entire period a user must wait to confirm their transaction. In other words, it means from the moment a user initiates a transaction until it is finally placed in a confirmed block [129]. 

- **Block Size:** Capacity of a block that can be filled with transactions [76]. 

- **Storage:** This refers to the entire volume of capacity that a blockchain network may utilize. 

- **Computation energy:** This refers to the considerable amount of energy required for mining a block. 

- **Network load/Transaction pooling-queuing:** This corresponds to the total number of transactions processed by the network [60]. 

- **Node configuration:** This refers to each node (peer) system configuration, i.e., CPU, RAM, and network speed [149]. A node characteristic such as node speed network directly affects the data propagation through the Blockchain network, and other characteristics such as the number of processors, CPU speed, and RAM that generally defines the number of transactions that can be processed by the node affects the blockchain network load. 

- **Number of nodes:** This refers to the total number of nodes (peers) interconnected through the blockchain network [111]. 

- **Transaction Cost:** This is the total cost of confirming a transaction in the blockchain. Transaction fees are a necessary component of most blockchain systems, as they are used to pay the network’s miners for verifying and processing transactions. As more users adopt blockchain payments, the number of transactions on the network has increased massively, which gives rise to higher transaction fees and causes many transactions to remain in the queue without processing for a long time [15]. Additionally, a transaction-prohibitive cost can discourage users from using blockchain. 

- **Transaction payload size:** In a blockchain network the transactions are propagated to all the nodes in the network. Thus the transaction payload matters as the larger the payload the more time is required to be propagated to the entire network. This affects performance/scalability. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:9 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

- **Smart contract complexity:** The complexity of the smart contract can affect performance/scalability. The smart contract execution/processing incurs a cost to the system that is measured by the latency. The latency can be increased if the complexity of the smart contract is increased, due to (i) data validation, (ii), or the number of reads/writes from/to the blockchain. This clearly affects performance/scalability [78]. 

As explained before, _integrity_ is one of the key properties of the Blockchain. This is often achieved with the use of the hash function. The complexity of a hash algorithm can affect the performance of a Blockchain in several ways. Generally, a complex and sophisticated algorithm necessitates more computational power to mine new blocks, which can slow down the overall process of adding new blocks to the Blockchain. On the other hand, a simpler algorithm may have faster processing time but it can also be more susceptible to hacking attempts. As a result, while choosing hash algorithms, it is critical to strike a balance between security and performance. 

Different Blockchains have adopted different hash function algorithms. For the sake of brevity, we do not go into the details. See Table 8 in the Appendix for a list and a description of the main hash functions. 

## **3.3 What are State-of-the-Art Solutions Being Developed to Address the Key Challenges Faced by Blockchain Technology?** 

There have been a significant number of works/solutions proposed to deal with the aforementioned limitations. In the following section, we analyze and compare the solutions of each type. Table 4 represents an overview of the existing solutions divided into 5 categories: _Payment Channel, Block data, Consensus, MultiChain, and Rollup_ . For each category, we explain the methodologies used for addressing the limitations, and in the last column, we name the existing solutions corresponding to each category. 

_3.3.1 Rollup._ Rollup solutions proposed off-chain computation to lower the cost of contract validation. Truebit, an Ethereum smart contract, delegates expensive computations to a trusted third part the solver and rewards a challenger who checks the solver’s work. This facilitates reliable intensive applications since Ethereum gas costs limit on-chain computations [179]. Arbitrum, an Optimistic Rollup, also uses off-chains contract verification to improve its scalability andit is designed for private contracts. It incentivizes parties to reach off-chain agreements about contract behavior, enabling miners to verify signatures without accessing the contract. This is designed for private contracts [86]. In summary, both Truebit and Arbitrum reduce computation costs and improve scalability via off-chain processing and selective data access. They incentivize proper computations through verification rewards and penalties. 

_3.3.2 Payment Channel._ It is a provisionally off-chain trade channel that is used to shift some transactions from the primary chain to this channel to minimize the transaction volume of the primary chain and to increase the system throughput. The Lightning network [84], which is used by Bitcoin, and the Raiden network [141] for Ethereum are two examples of payment channel methods. 

- —Lightning Network – The volume of Bitcoin transactions has increased significantly in recent years. This revealed the major flaws of bitcoin such as long transaction times and exorbitant transaction fees. To address these drawbacks, experts developed a new alternative solution, _Lightning network_ . In nutshell, the Lightning network’s main objective/goal is to create a trade channel between two Bitcoin nodes that work (transfer/handle transactions) outside the main chain; and this can enhance transaction throughput. This approach 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:10 

Table 4. Blockchain Optimization Solutions 

|**Categories**|**Methodologies**|**Solutions**|
|---|---|---|
|Rollup, “ZK-Rollups,|The goal is to reduce network congestion and lower|Truebit [ 179 ] , Arbitrum [ 86 ]|
|optimistic rollups|fees by transferring computational data storage||
||away from the primary layer of the blockchain.||
|Payment Chanel|It is a temporary of-chain trading channel, where|Lightning Network [ 84 ], Raiden Network|
||transactions are directed to alleviate the|[ 141 ], Sprites [ 117 ], DMC [ 59 ]|
||transaction volume on the main chain and enhance||
||system throughput.||
|MultiChain|1- Side chain: is a separate blockchain from the|Plasma [ 85 ], Pegged sidechain [ 17 ], Liquidity|
||main chain designed to streamline transaction|network [ 99 ]|
||operations. It enables the transfer of data that||
||undergoes one-way transformation before being||
||reintegrated into the main chain.||
||2- Cross chain: is the process of integrating|Polkadot [ 40 ], Cosmos [ 51 ]|
||multiple blockchains to create a large||
||interconnected network, all the while ensuring||
||seamless interoperability among them.||
||3- Sharding: it involves splitting the blockchain|Elastico [ 107 ], OmniLedger [ 90 ], RapidChain|
||network into numerous shards. This strategy seeks|[ 200 ], RsCoin [ 56 ], Monoxide [ 188 ],|
||to distribute the workload across multiple chains to|ChainSpace [ 5 ], Zilliqa [ 177 ], QuarkChain|
||enhance throughput and decrease latency.|[ 139 ], Elrond [ 176 ], Ostraka [ 109 ], Harmony|
|||[ 77 ], Ethereum 2.0 [ 70 ], ZyConchain [ 162 ]|
|Block data|Solutions exploring methods to reduce data, which|SegWit [ 104 ], Bitcoin-Cash [ 41 ], CUB [ 195 ],|
||include increasing the number of transactions per|Txilm [ 64 ], compact block relay [ 119 ], Jidar|
||block or implementing block compression|[ 55 ], Litecoin [ 142 ], Optimized aggregation|
||techniques.|scheme [ 57 ]|
|Consensus|Enhance the consensus protocol for achieving|BlockML [ 116 ], PoAI [ 43 ], PoDL [ 47 ], SPoDL|
||higher throughput and reduced latency.|[ 106 ], PoA [ 25 ], PoB [ 88 ], PoSpace [ 68 ],|
|||PoWeight [ 8 ], PoC [ 115 ], PoI [ 8 ], PoSearch|
|||[ 153 ],PoL [ 37 ], PoLE [ 94 ], PoUW [ 20 ],|
|||PoDLwHO [ 118 ], PoH [ 32 ], PoFL [ 138 ],|
|||PoKW [ 105 ], DAIBCN [ 92 ], Optimized PoW|
|||[ 128 , 190 ], PII [ 48 ] , Optimized PPoV [ 194 ],|
|||GBT-CHAIN [ 187 ], B-PDA [ 14 ], Optimized|
|||MEC [ 101 ], Optimzed Edge Computing|
|||Framework [ 41 ]|



comprises three phases: (i), _initialization phase_ (i.e., channel creation), (ii), _trading_ , and (iii), _channel closure_ . 

The initialization phase as the name suggests performs the initialization tasks. It first creates a channel (a Lightning channel is created between two peers on the Lightning Network). This channel will be recorded in the Bitcoin main chain. Through this channel, the two participants first deposit a specific number of tokens (higher than the total number of transactions that will be transferred between the two). This occurs before the two participants begin their actual transactions with each other. Then the trade phase starts, where the two participants start sending transactions. During transaction exchanges, if a malicious transaction is detected (i.e., cheating occurred), then a counterparty will receive the whole channel’s money (the deposit set at the beginning) as punishment. Once the trading phase is finished/completed (i.e., all the transactions between the two parties are completed), then the third phase, i.e., channel closure) is triggered. When the channel is closed, each participant’s total quantity of tokens is sent to the main chain. Hence, a significant number of transactions are completed off-chain, with only a pair of transactions being recorded on the Bitcoin chain. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:11 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

—Raiden Network – It is a payment channel designed/adopted by Ethereum. Raiden designed and implemented similarly to the Lightning network. The only difference between the two networks is in the type of transactions that they can process/accept. The Raiden accepts all types of ERC20 tokens, while the Lightning Network only accepts Bitcoin transactions. 

In the last decade payment channels have been extensively studied, with many Lightning Network implementations being released. There are also numerous alternative off-chain payment channel options, such as Sprites [117] and Bitcoin DMC (Duplex Micropayment Channels) [59]. 

_3.3.3 Multi Chain._ A multi-chain Blockchain system refers to the use of multiple interconnected blockchain networks working together in order to achieve a specific goal or set of goals, such as increased scalability, interoperability, and security. This can be accomplished by creating new blockchain networks or by connecting existing ones. Multi-chain has three categories which are: Side chain, Cross chain and sharding, which are explained one by one in the following 

_Side chain_ . A Sidechain is a distinct blockchain that is linked to a primary blockchain, enabling the movement of digital assets or information between the two chains [158]. This allows for the main blockchain to remain secure and decentralized, while also providing additional functionality or scalability to the sidechain. One of the main benefits of using a sidechain is that it allows for greater flexibility in terms of the types of transactions that can be performed and the types of assets that can be transferred. Plasma is an Ethereum scaling framework allowing infinite side chains in a tree structure [85]. Pegged sidechains enable asset transfers between blockchains securely and atomically [17]. Finally, Liquidity Network uses Merkleized Interval Trees to represent user balances, enabling real-time transactions without limits [99]. It ensures high throughput and growth while eliminating fees and delays. Sidechains like Plasma, Pegged, and Liquidity Network improve scalability and interoperability. They allow new features on sidechains while retaining mainchain security. Structures like Merkleized Interval Trees provide balances for fast, unlimited transactions. 

_Cross-chain_ . Cross-chain is another solution to addressing the scalability limitation. It has been attracted/used by many and has become one of the popular solutions being adopted. Cosmos [51] is an example of Cross-chain which is a blockchain ecosystem made up of interconnected blockchains. The network is made up of several different blockchains, each of which is referred to as a zone. Those zones can connect with one another with an Inter-Blockchain Communication protocol, which allows heterogeneous blockchains to communicate, thanks to consensus methods like Tendermint consensus [63]. Polkadot [40] is a cross-chain system that includes a relay chain for connecting different blockchains. 

_Sharding_ . Sharding is another solution devised for addressing scalability. It is a database partitioning strategy that splits up a huge database into faster, smaller, and easily manageable fragments, called “shards” [198]. Technically, shard is synonymous with horizontal partitioning. In blockchain, it divides the entire blockchain network into shards or committees, which are smaller groupings of nodes. The shards work in parallel to process distinct transactions and keep a separate ledger. When sharding is applied in blockchain, the network experiences a different life cycle. As depicted in Figure 2, the life-cycle is composed of: (i) a bootstrapping stage, and (ii), epochs with random duration. Each epoch is composed of two phases: consensus and reconfiguration. The consensus phase is where the calculation, the mining of the blocks, and the verifications are performed. The reconfiguration phase varies from one protocol to another. However, generally, at the end of each epoch the committees are reconfigured according to the information of the current mining committees, so the implementation of the new configuration, i.e., the reorganization of the shards and committees of the next epoch always depends on the committees and the previous state of the blockchain. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:12 

Fig. 2. Sharded blockchain lifecycle. 

Elastico [107] was the first to concretize the idea of sharding on the blockchain confirming the idea that as long as there are miners, we can generate blocks. But to achieve high scalability, it sacrificed the aspect of security in the network of the committees of the blockchain. RsCoin [56] is another sharding solution for computation and storage, but loosing one of the fundamental principles of the blockchain: decentralization. What Omniledger [90] has added over the public sharding protocols that preceded it, is the “trust but verify” concept and “Atmoix” protocol which solves the problem of UTXOs (unspent transaction output) blocked for cross sharded transactions that was a big flaw for Elastico. However, OmniLedger has also some drawbacks, such as forcing users to actively participate in cross-shard transactions, which represents a significant assumption for most lightweight users. Moreover, Denial of service attacks appears to be a threat to OmniLedger, i.e., for a malicious user who can exploit the atomic cross-shard protocol to lock arbitrary transactions. QuarkChain [139], founded by Qi Zhou, is a blockchain platform that employs a unique heterogeneous sharding solution. It is a two-layered ecosystem consisting of a root chain and multiple shards. This allows for the creation of custom shard chains within the network, each with its own consensus mechanism and miners or validators, resulting in increased flexibility. Monoxide [188] makes use of asynchronous consensus zones, each one can be thought of as a shard. This protocol is built on the account/balance transaction model, which is analogous to a bank account model, rather than UTXO transaction models. Monoxide is classified as a kind of blockchain-sharding method in theory. RapidChain [200], it is the first solution with communication sharding. As it allows sharding in calculation, storage, and communication, it is called a “full sharded” blockchain: a blockchain technology based on sharding that is extremely scalable to huge networks. Like the others, Rapidchain has some drawbacks as it is sensitive to partitioning attacks and has responsiveness problems. Lastly, we have Ostraka [109] which is unlike the previous sharding approaches, it shards the node rather than the entire system. 

_3.3.4 Block Data._ The Blockchain issues are generally related to block size in several ways. Increasing block size obviously allows having more transactions in a block. The same result may be achieved via block compression, which also reduces storage overhead. There are also many other alternatives that investigate data reduction strategies. The method of segregating the digital signature of the transaction is known as a segregated witness (SegWit). It consists of no longer saving the signature data with the transaction data but placing them in a separate structure called a witness [104]. Bitcoin also attempted to modify the block size in its network to address scalability. To achieve this, Bitcoin divided the network into two _Bitcoin_ and _Bitcoin-Cash_ by performing a hard fork in 2017. The block size of Bitcoin-Cash [42] raised to 8 _MB_ , which is significantly higher compared to the previous setting where the size was from 1 _MB_ . Following that, Bitcoin-Cash block size was updated again to 32 _MB_ . The Bitcoin-Cash average block interval has remained unchanged at 10 minutes. The transaction throughput can theoretically be considerably enhanced. The stress 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:13 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

test, done in 2018, corroborated this. Improving block size can directly improve blockchain capacity scalability from both a theoretical and practical standpoint. However, because intra-blockchain bandwidth is limited, limitless growth increases the size of each block, making it difficult to transport. As a result, just raising the block size is not a long-term solution. 

Danzi et al. [57] proposed a method whereby nodes within blockchain networks aggregate blockchain data into periodic updates, aiming to alleviate the communication overhead among interconnected devices. This synchronization process can be customized to meet real-time information needs. Adhering to Ethereum specifications, this aggregation approach reduces communication costs. Data on the application layer is gathered and shared only when required by end devices, optimizing communication expenses. However, this strategy is constrained by the limited computational and storage capabilities of systems, despite its benefits in cutting communication costs. 

_Consensus_ . The Consensus Algorithm is responsible for preserving the integrity and security of the whole blockchain. Consensus consists of achieving a particular agreement among all participants in the network. However, certain algorithms, such as Proof-of-Work, consume significant energy to achieve a single agreement by solving a hash puzzle and have limited throughput, making them unsuitable for blockchain. Hence, researchers attempted to propose different solutions to address the shortcomings of the PoW and optimize the performance of the blockchain. Alternative consensus protocols provide different criteria for block validation rights to improve aspects like sustainability, security, and fairness over PoW. We have listed some of the existing solutions below: 

- —Proof of Stake (PoS) selects validators based on their stake to confirm transactions and add blocks. Validators receive rewards, incentivizing participation [127]. 

- —Proof of Activity (PoA) combines PoW for block creation with PoS for validation by randomly selected validators. This enhances security [25]. 

- —Proof of Burn (PoB) allows miners to “burn” tokens for mining rights, making it more energy efficient than PoW. The more coins burnt, the higher the mining capacity [88]. 

- —Proof of Space (PoSpace) grants mining rights based on allocated disk storage space [68]. It excludes malicious miners via storage capacity challenges. Sometimes it is also referred aProof-of-Capacity (PoC) [115]. 

- —Proof of Importance (PoI) uses multiple criteria to assess node importance beyond just stake. It aims to improve on the limitations of PoS [8]. 

- —Proof of Weight draws on the Algorand model to use comparative weighting schemes beyond token stake for block assignment [8]. 

- —Proof of Search (PoS) allows the use of compute power for optimization problems instead of hashing. Nodes are rewarded for contributions and solutions [153]. 

- —Optimized Parallel Proof of Vote (PPoV) [194] combines vote-based and proof-based consensus mechanisms, processing multiple blocks in parallel to optimize resource use. It includes dynamic node sharding and a three-stage consensus process (Prepare, Vote, Commit), enhancing security and efficiency. 

- —GBT-CHAIN [187] addresses the blockchain trilemma by dynamically selecting Byzantine Fault Tolerance (BFT) consensus algorithms such as Zyzzyva, HotStuff, and Vote-as-a-Proof (VaaP) based on system requirements and network conditions. The framework operates across physical, network, and consensus layers, using a “proposal-broadcast-voting” model to enhance scalability and consistency. 

- —The B-PDA framework [14] uses both multi-proof-of-work and proof-of-stack consensus mechanisms that aimed at streamlining workload management. This is achieved by categorizing requests into on-chain and off-chain channels. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:14 

Table 5. Benefits and Limitations of Scalability Solutions 

|**Solutions**|**Benefts**|**Limitations**|
|---|---|---|
|Rollup|Reducing fees and gas consumption for users.|Solutions depends on third party verifcation.|
||Fast transaction throughput.||
|Payment Channel|Faster payment.|Users are not scaled.|
||High throughput.|Low protection.|
||Real-time payments.|Designed just for payments.|
||Low transaction fees.|Smart contracts not adopted.|
||Scalable.||
||Privacy protection.||
|Multi Chain|High throughput.|Long wait to withdraw funds.|
||Communicates among heterogeneous blockchains.|Complicated implementation.|
||Highly scalable.|Genesis epoch and starting new epochs are|
||High security.|sensitive phases.|
||Reduce transaction costs and speed up operations.||
|Block Data|Increase the throughput.|Generates hard fork.|
||Increasing Block size.|Low scalability.|
||Fixing transaction malleability problem.||
||Reduces UTXO growth.||
|Consensus|Increase the throughput.|Depends on new actors: suppliers, keepers.|
||Investing computational energy in meaningful tasks.|The blockchain depends highly on the new|
||Improve blockchain performance by selecting best|types of tasks proposed in the solutions.|
||nodes in the network.||



—Liu et al. [101] introduced a a novel mobile edge computing (MEC) blockchain-based framework. Within this framework, computation-intensive PoW tasks can be offloaded to nearby edge computing nodes, and the cryptographic hashes of blocks can be stored in the mobileedge computing server. 

—Casado-Vara et al. [41] presented a novel approach to data management, leveraging blockchain technology at the edge computing layer. Their method integrates a smart controller with a miner processing unit, enhancing the computational capabilities of Raspberry Pi for executing smart contracts. This innovation slashes energy consumption during data transformation and block creation by substituting traditional miners with smart controllers. 

We summarize the benefits and limitations of each scalability solution in Table 5. 

## **3.4 How can the Integration of Artificial Intelligence Enhance the Capabilities and Applications of Blockchain Technology?** 

In this section, to establish a foundation for exploring the potential applications of artificial intelligence (AI) in blockchain technology, we first examine the core principles of AI itself by providing a comprehensive understanding of the basic principles, processes and applications underpinning artificial intelligence. 

Artificial Intelligence (AI) refers to the simulation of human-like intelligence processes by machines, aiming to enable them to perform tasks that typically require human cognitive abilities. It encompasses a broad spectrum of techniques and technologies that enable machines to learn, reason, problem-solve, perceive, and interact with their environment. AI extends beyond mere automation, as it strives to imbue machines with the capacity to understand context, adapt to changes, and improve performance over time. 

The scope of AI is vast and continually expanding. It encompasses several subfields [91], including machine learning, natural language processing, computer vision, robotics, expert systems, 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:15 

and more. AI has applications in diverse domains such as healthcare, finance, transportation, entertainment, and manufacturing. Its potential lies in augmenting human capabilities, enhancing decision-making processes, and propelling innovation across industries. Below we will explore the key branches of Artificial Intelligence (AI) and the fundamental concept of intelligent agents. **Key Branches of Artificial Intelligence:** 

—Machine Learning and its Types: 

Machine learning (ML) is a specialized branch of artificial intelligence that enables machines to learn from data without explicit structure, improving their performance over time It includes a variety of learning techniques: 

- _Supervised learning:_ In this method, models are trained on data, allowing results to be predicted based on new information unseen. 

- _Unsupervised learning:_ Here, models uncover patterns in unlabeled data, aiding in tasks like clustering and dimensionality reduction. 

- _Reinforcement learning:_ This approach involves training models to make a series of reward-based decisions, resulting in learned behavior. 

- _Deep learning:_ A subset of ML, deep learning employs neural networks with multiple layers to tackle complex problems like image and speech recognition. 

- —Natural Language Processing: 

- **Natural Language Processing** ( **NLP** ) involves enabling computers to understand, interpret, and produce human speech. It includes services such as sentiment analysis, language translation and chatbots. NLP technology has revolutionized interactions between humans and machines, enabling meaningful interactions across languages and contexts. 

- —Computer Vision: Computer vision revolves around machine learning algorithms for interpreting and understanding visual information from images or video. This area includes image recognition, object recognition, face recognition, and so on. Mimicking human vision, computer vision technology finds utility in applications such as autonomous vehicles, medical imaging, and surveillance systems. 

- —Expert Systems: 

- Expert systems combine human knowledge with AI to make decisions in specific areas. These systems mimic the knowledge of human experts, offering recommendations or solutions based on predetermined rules and knowledge. Expert systems benefit in areas where expert knowledge and decision-making are critical, such as medical diagnosis, budgeting, and problem solving. 

- The array of AI disciplines, encompassing machine learning, natural language processing, computer vision, and expert systems, collaboratively propel the enhancement of AI’s capacities, enabling it to mimic and enhance human cognitive abilities across diverse domains. 

## **Intelligent Agents:** 

As Russell and Norvig explain in their book [145], picture a system capable of independently traversing intricate surroundings, gaining knowledge from its interactions, and adjusting its choices to accomplish particular objectives. This encapsulates the concept of an intelligent agent, a dynamic domain within artificial intelligence. These agents serve as a connection between the virtual and real worlds, observing their environment, making informed decisions, and executing actions to achieve goals. 

Their significance is unquestionable. Within healthcare, intelligent agents scrutinize medical data to identify illnesses, suggest treatment strategies, and even simulate surgical 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:16 

Table 6. Phase Two Inclusion/Exclusion Criteria 

|Inclusion|Exclusion|
|---|---|
|1. The paper must be empirical research about|1. Articles only interested in improving|
|integrating artifcial intelligence or machine learning|blockchain without using artifcial intelligence.|
|in Blockchain.||
|2. The paper needs to highlight the optimizations|2. Studies focusing on the integration of|
|made on Blockchain by the integration of AI or ML.|blockchain to improve AI.|
|3. Paper with a view of the concept of blockchain||
|issues that can be optimized with AI.||



procedures. In finance, they’re employed for tailored investment guidance, fraud detection, and automated trading algorithms. Robotics employs them for self-guided movement, object handling, and intricate task decision-making. From customized education to intelligent homes, these agents are deeply integrated into diverse sectors, streamlining operations, refining choices, and augmenting human capacities [201]. 

With the progression of technology, the complexity of these agents also evolves. They persistently acquire knowledge, adjust, and develop, expanding the horizons of what can be achieved. Their increasing prevalence holds the potential to overhaul countless sectors, reshaping our interactions with the environment and imprinting a lasting influence on the future [7]. 

Creating smart agents capable of handling complex environments necessitates dependable structures. These frameworks guide the design and programming, ensuring the agents function as intended. The **Belief-Desire-Intention** ( **BDI** ) model emerged as a powerful response to this need [61]. Without robust frameworks, intelligent agents could flounder, their actions unpredictable and potentially harmful. The BDI model provides a compass, guiding them toward clear objectives and well-defined behaviors. 

As mentioned before, blockchain has serious limitations, namely security, scalability, and performance. AI is a promising solution that can improve blockchain technology by addressing its shortcomings. Some researchers have already applied AI in blockchain to enhance the performance of the technology. However, it is not been widely studied/applied for addressing these issues. We believe AI can transform blockchain. Hence, in the next section, we explain the existing solutions based on AI to give some direction for future work to apply AI. 

## **3.5 What are the Promising AI-Driven Solutions Currently being Utilized in Blockchain, and What Novel Opportunities do they Present?** 

In this phase, we filtered our previously selected works through the inclusion and exclusion criteria, as mentioned in Table 6 below to reach works that are interesting and in relation to AI and ML. Various improvements have been made by AI to optimize and resolve blockchain issues. Each article was read in its entirety, with pertinent data collected and summarized. The topic of each study focused on a specific issue. Collected data and issues are in Table 7 below. Solutions are repartitioned according to focused issues by articles. Solutions are divided into different categories: Sustainability, Security, Scalability, Efficiency, and Governance & dependency. The identified topics show that most of the studies are focused on Sustainability (Optimizing energy consumption), Security (detecting Blockchain layer intrusion), and Scalability (integrating AI to perform collaborative learning without centralized data), Efficiency, governance, and dependency were mentioned equally but less than the previous ones. This includes predicting the best nodes for mining tasks, 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:17 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

Table 7. Artificial Intelligence-Based Blockchain Solutions 

|**Work**|**Sustainability** **Scalability** **Security** **Efciency** **Governance** **and** **Dependency**|
|---|---|
|Bravo-Marquez et al. [ 37 ]|+|
|Lan et al. [ 94 ]|+|
|Chenli, Li et al. [ 47 ]|+|
|Luo, Yang et al. [ 106 ]|+|
|Baldominos et al. [ 20 ]|+|
|Mittal et al. [ 118 ]|+|
|Merlina et al. [ 116 ]|+|
|Blocki et al. [ 32 ]|+|
|Qu et al. [ 138 ]|+|
|Liu et al. [ 102 ]|+|
|Zhang et al. [ 202 ]|+|
|Yun et al. [ 199 ]|+|
|Qiu et al. [ 137 ]|+|
|Scicchitano et al. [ 150 ]|+|
|Agarwal et al. [ 3 ]|+|
|Tang et al. [ 173 ]|+|
|Bogner [ 35 ]|+|
|Firdaus et al. [ 73 ]|+|
|Dey [ 62 ]|+|
|Chen [ 44 ]|+|
|Liang et al. [ 98 ]|+|
|Chen et al. [ 46 ]|+|
|Salimitari et al. [ 147 ]|+|
|Chen et al. [ 43 ]|+|
|Lundbaek et al. [ 105 ]|+|
|Kumaresh et al. [ 92 ]|+|
|Wang et al. [ 190 ]|+|
|Nguyen et al. [ 128 ]|+|
|Mohammadi et al. [ 120 ]|+|
|Chouli et al. [ 48 ]|+|



enhancing mining power, integrating AI to reduce blockchain dependencies, and achieving an autonomous blockchain that relies on high-bandwidth and high-computing power miners and honest network participants for proper functioning. We also have solutions that have resolved multiple issues but have made radical changes to the core and the participants of the blockchain. 

_3.5.1 Sustainability._ From the standpoint of large-scale complex systems, there is not a unified vision of artificial intelligence or machine learning-backed blockchain system energy consumption optimization as mentioned in the works below. 

WekaCoin [37] is a Blockchain built on the “Proof-of-Learning” consensus protocol. By ranking the models produced for a specific task, Proof-of-Learning achieves distributed consensus. As part of the proof of learning, WekaCoin network participants specifically miners participate in the network by providing their computing power to solve complex mathematical problems. As a result, the protocol makes the computational process task easier for people having greedy training 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:18 

or calculating tasks, such as machine learning, where large amounts of data and computational resources are required to train effective models. 

Lan, Yixiao and Liu, Yuan, and Li, Boyang [94] have presented a new consensus mechanism, the Proof of Learning (PoLe), that uses the computing capacity to reach neural network optimization (ANN), instead of wasting energy. PoLe creators proved that PoLe could reach a more stable block generation rate than PoW, which allows more efficient transaction processing. 

Chenli, Changhao, et al. introduce Proof of Deep Learning (PoDL) [47] consensus, which obliges miners to train deep learning models rather than hash calculation, and they introduce qualified deep learning models as proofs. PoDL is the first consensus system that uses deep learning to maintain blockchains. 

Based on the approach adopted in PoDL [47], Luo et al. presented S-PoDL [106], another deep learning-based consensus protocol. In S-PoDL, all the nodes are separated into sections for training several deep-learning models, at the same time, supplied by model requesters. Each section has a designated node that serves as the section leader, and all other nodes abide by the decisions made by this leader to train a requested model. The leader is the one who assesses the accuracy of the models submitted by the section nodes. Consequently, the communication overhead between nodes is reduced. As the leader requires the section nodes to work on specific models, S-PoDL outperforms PoDL in terms of computing performance and results. 

Coin.AI [20] is based on Proof-of-Useful-Work (PoUW) consensus mechanism. The main idea behind PoUW is to incentivize computational work that is useful, rather than just wasteful computational work used in PoW to secure the network. So the nodes train deep learning models, and when their results achieve a certain accuracy, then they can add a new block. Other nodes verify a block by checking the transactions included in the block and the proof (the provided result for a training model). 

PoDLwHO [118] is another AI-based solution applied to optimize the sustainability of blockchain. PoDLwHO [118] uses the inane computations of PoW for training DL models by setting hyperparameters instead of calculating purposeless hash values. The contribution of this approach is to use many Bayesian constructs consisting of the evaluated samples in order to help miners to get the best accuracy of specific deep learning problems by using the previous results in a more beneficial way. In other words, till now PoDLwHO is the only solution that retains the nodes’ results even if they do not reach the requested threshold. 

BlockML [116] is an alternative system that replaces PoW hash-puzzles with the training of machine learning models. This approach distinguishes itself from other approaches by the concept of the absence of new coins being minted after block generation. In fact, in BlockML coins are not created for newly generated blocks. This helps to counter malignant entities that may attempt to complete their own task because the participant can adopt any role as a supplier or a miner. 

In other solutions, e.g., Coin.AI, which uses “Miners” and “keepers”, the miners are responsible for training the models and adding blocks to the blockchain and the keepers store the models in the blockchain. 

In Coin AI [20], the miners or the trainers should agree on the ML task that they will train its model before starting the process. So, when using this approach, miners cannot train multiple models of different ML tasks in the same epoch, which is unlike other approaches where miners can work on several ML tasks at the same time. After the model creation, the model must be stored somewhere. There are three model storage approaches that can be used as follows. 

- —The first option is to store the model directly in the blockchain, which is used by [94] and [47]. This approach has a disadvantage. That is, the model would use a very high ratio of block storage on most occasions, most certainly more than 99.9% of it. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:19 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

- —The second option is to store the model in a centralized server, similar to [118], and provide a cloud storage link to the participant. This solution could be acceptable in a private blockchain configuration, i.e., used in companies/organizations. 

- —The third option is to store the model in a distributed system. As examples of this approach, we can name [37] and [20], where several blockchain nodes, i.e., keepers, store the models. And in another example, i.e., wekaCoin [37] and BlockMl [116], the model is stored in IPFS [24], i.e., a decentralized distributed file system. 

The article [32] proposes a Proof of Human work (PoH). It is proof that a person makes a reasonable effort to resolve a problem. A PoH problem must be fairly difficult to be completed by a human. A PoH problem must be difficult to be resolved by a computer without human aid, even the computer that created the puzzle. CAPTCHAs [33] (Completely Automated Public Turing test to tell Computers and Humans Apart), on the other hand, are just challenging to solve for other computers, not for the computer that created the problem. Also, Proof of Human work is verifiable by a machine without the need for a person’s intervention or interaction with the creator of the PoH. 

_3.5.2 Scalability._ Scalability in the context of blockchain refers to a system’s ability to scale as the number of users increases. Latency (transaction confirmation time), bootstrap time (transaction verification time), and the cost produced for every validated transaction are all examples of scalability issues. Overall, one or more of these scaling limitations restrict the efficiency of a blockchain system. Because each block contains a certain quantity of transactions, traditional centralized data mining approaches are failing to deal with this condition. However, new ML consensus algorithms based on federated learning as POFL achieved solutions that learn from dispersed data sources, which provided a worldwide optimal solution for the target blockchain system. 

To address PoW’s shortcoming, Proof of Federated Learning [138] (PoFL) re-invests the energy consumed in PoW for the meaningless hash task into federated learning. To reach a consensus, PoFL uses federated learning to tackle meaningful challenges with practical value. Requesters post challenges like analysis and image recognition as jobs, coupled with the accompanying prizes to incentive miners. To reach a consensus, miners must complete these tasks as proof of work. 

A novel dynamic sharding blockchain framework named SkyChain [202] addressed the limitations of static sharding in existing blockchain systems using DRL to adjust the resharding interval, the number of shards, and the block size in real-time. By doing so, SkyChain ensures a long-term balance between performance and security in the constantly evolving blockchain environment. 

To address the high throughput demands of blockchain-enabled Industrial Internet of Things (IIoT) systems, Liu et al. [102] introduced a novel performance optimization framework utilizing **deep reinforcement learning** ( **DRL** ). This advanced framework dynamically tunes critical parameters, including the selection of block producers, optimization of consensus algorithms, adjustment of block sizes, and modification of block intervals. By employing DRL, the system can adaptively enhance these elements to boost system scalability, allowing IIoT networks to effectively manage increasing data volumes and operational complexities. There is also Reference [199] that employed the **deep Q-network** ( **DQN** ) algorithm to dynamically adjust the already mentioned parameters and also the number of shards. This approach seeks to optimize these settings while maintaining the security of the system. In a similar vein, the study in Reference [137] introduced a DRL-enabled adaptive blockchain scheme, which not only improved scalability but also addressed the diverse needs of users. By leveraging a DRL algorithm, the system can select the most suitable consensus protocol based on the users’ quality of service requirements. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:20 

_3.5.3 Security._ Blockchain suffers from some security limitations, such as application layer vulnerabilities, data encryption techniques, such as: vulnerabilities that can result in a successful attack, brute force, ciphertext-only, and known-plaintext attacks; additionally, implementation errors caused by improper implementation of encryption algorithms can result in security vulnerabilities and weaknesses. 

Computational Intelligence is commonly employed in security systems to improve the security of computer systems and networks, it is used for Intrusion Detection and attack prevention. Computational intelligence is also applied to keep blockchain systems secure. The benefit of applying it in blockchain systems is to have strong and secure encryption algorithms, which play a crucial role in safeguarding sensitive blockchain data. Also, it can be applied to enhance the system attack-defence mechanism to guard against attacks [38, 75]. 

The work presented in Reference [150] proposes a method for detecting anomalies based on a deep learning encoder-decoder model trained using aggregated data taken from blockchain activity monitoring. The suggested anomaly detection method employs a neural encoder-decoder model capable of encoding information about the ledger’s state into a latent space and then reconstructing the original data from this space. The fundamental principle is that the encoding/decoding procedure retains the data’s essential features whenever the current condition is consistent. Anomaly situations, on the other hand, have inconsistencies in their values, which leads to a failed reconstruction. There are also, other works that adopted the same concept for detecting anomalies like [35, 73, 173] and [62] that discussed the challenge of blockchain behavior pattern categorization and proposed solutions as PeerClassifier [173], a deep learning-based technique to handle the issue. 

Blockchain technology is seen as a viable option for maintaining privacy in applications due to its ability to securely and decentralize storage and transfer of confidential data [181]. Examples of privacy-preserving applications of blockchain technology are: Zero-knowledge proofs like Zcash [23], Homomorphic computation [204], and **Multiparty computation** ( **MPC** ) [9], these methods enable processing and verifying data without disclosing it. Also, on the side of privacy-preserving by smart contracts [22], we have the automation of the execution of transactions, particularly private transactions, like sharing confidential information [182]. As blockchain technology becomes increasingly prevalent, it is being integrated into more and more projects. However, as its popularity grows, the potential security issues also increase. It is critical to process transactions in a secure manner. Rachit Agarwal et al. [3] offers a method for detecting fraudulent accounts based on the temporal structure of blockchains. In this solution, they used supervised and unsupervised Machine learning algorithms. 

T–S fuzzy ANN presented by [44] can represent a traceability chain with each block and its neighbor. Their main contribution consists of minimizing the quantity of irrelevant data in order to infer more coherent linked transaction data from its computed entire history and participant nodes. 

Liang et al. [98] introduce a novel approach to combat anomaly intrusions in blockchain systems using a collaborative clustering-characteristic-based data fusion technique. Their system amalgamates data from multiple sources and employs an AI model to train and scrutinize data clusters, thereby enhancing the precision of intrusion detection. By applying a mathematical model, the system detects abnormal characteristics within blockchain data and performs a weighted aggregation of information across various nodes. 

To combat Ponzi scheme fraud, which is a fraudulent investment operation where returns to earlier investors are paid from the capital of newer investors rather than from profit earned, Chen et al. [46] presented a method where relevant features were extracted from user accounts and 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:21 

operation codes of numerous smart contracts then analyzed using the XGBoost machine learning algorithm to identify potential Ponzi schemes within these contracts. 

On the Hyperledger Fabric blockchain platform, and with a relatively low tolerance for malicious activities, Salimitari et al. [147] developed an external detection algorithm using supervised machine learning, positioned before the consensus protocol as a preliminary step. This detection algorithm validated the compatibility of new data and discarded suspicious entries, thereby enhancing the network’s fault tolerance for the subsequent consensus phase. 

The security solutions mentioned above generally operate based on the actions of a participant in a blockchain to detect any intrusion, malicious actions, or problems with smart contracts. However, these solutions analysis and response processes take a significant amount of time, making it difficult to quickly detect attacker behavior and intentions. Currently, there is no solution that can identify compromised identities and security flaws in real-time. This means there is still a lack of robust systems or solutions that can simulate blockchain attacks to prevent a breach or stop it before it occurs. Therefore, these solutions are missing the prediction feature that will make them respond immediately to malicious problems. 

_3.5.4 Efficiency._ The number of available resources in a blockchain network is known in advance. As a result, as in the works below, AI can perform active and dynamic node selection to speed up the mining process and enhance overall system performance by allowing the network to select the most efficient nodes for mining at any given time. The selection process can take into account a range of factors, including the computational ability of the node, its network connectivity, and the amount of work it currently has. By closely monitoring and picking the most suitable nodes for mining, the network can guarantee that blocks are mined quickly and efficiently. 

The idea of Chen et al. [43] is a Machine Learning-based super node selection algorithm “Proof of artificial intelligence” (POAI) to save resources and speed up the block generation process. A method to intelligently select a “Minor” node, is provided, by following these steps. First, computing the ATN (Average Transaction Number), a function that serves as the primary criterion for choosing nodes, based on a comprehensive assessment of the average number of transactions a node receives per time period, the value of the participants by CNN. Then under a certain threshold, the process of selection is carried out to get the list of random nodes and super nodes. Finally, generating a pool of nodes and minors (random nodes and super nodes) based on their characteristics. 

The work [105] suggests a new proof-of-work consensus based on machine learning. Just a fraction of miners does the hash calculation job. In general, PoW is used to nominate a leader, who may recommend the next block on the chain. However, in this method, two control mechanisms are used to limit the PoW mining race for the next block: A Dynamic Whitelist and Cryptographic Sortition. 

**Decentralized Artificial Intelligence enabled Blockchain Network** ( **DAIBCN** ) [92] uses a decision tree architecture, so its algorithm picks miner nodes automatically. It is no longer necessary for miner nodes to calculate sophisticated hash functions in order to qualify for reward. In addition, data pruning is done on the blocks to allow for better storage and to minimize the chain length. The blocks mined every second raise the volume of data, as a result, the pruning process will be used frequently. This study clearly demonstrates that integrating AI in blockchain system has various benefits, including lower node energy consumption, reduced time, and storage savings, all of which contribute to a more efficient blockchain network. 

The main contribution of Wang et al. [190] is finding the best permissionless blockchain mining approach based on a machine learning algorithm “reinforcement learning”. In this work, the consensus starts by choosing a leader who oversees packaging transactions, creating a block and 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:22 

appending this block to the blockchain. To avoid malicious nodes from gaining blockchain control, selecting a leader should be unpredictable. Because in permissionless blockchain, anonymity is intrinsically built as an objective. It should address the Sybil assault, in which an attacker generates members with different identities to maximize its chances to get selected as leader. The objective of using PoW is to randomly pick a leader from participants of each round with a probability proportional to each participant computer capacity to overcome the aforesaid difficulties. 

Because PoW requires a lot of computing power and time, mobile devices are unfit to do it efficiently. Consequently, in the mobile blockchain network, edge computing paradigm was proposed by Nguyen et al. [128] to offload mining from mobile devices. The edge computing paradigm is composed of an edge computing service provider and mobile users, i.e., miners. Edge computing service provider owns resources for computing that are spread around the network to deliver computing resource services to mobile consumers. Because a single mobile user is allotted to a single edge computing resource unit, the mobile users compete for the unit. To finish block mining, mobile user is enticed to pay extra fees as much as he will continue using edge computing unit. Artificial intelligence contribution in this solution is introducing an ECSP [71] (Edge Computing Service Provider), auction based on neural network architecture. The neural network architecture used in this approach implements ECSP allocation and payment regulations; and, ensures that any auction mechanism discovered by the network is the best one. 

A fork is an event in which a blockchain diverges into two separate paths, typically arising from a disagreement among miners regarding the consensus process. This divergence not only poses security risks but also escalates costs, time, and energy consumption. To mitigate these substantial risks and costs associated with blockchain forks, Mohammadi et al. study [120] employed machine learning techniques to forecast these events. The research evaluated the prediction accuracy of blockchain forks using four prominent machine learning methods: K-Nearest Neighbors, Naive Bayes, Decision Tree, and Multilayer Perceptron. By comparing these methods, the study aimed to determine the most effective approach for predicting forks, thereby enhancing blockchain security and efficiency. 

References [43, 105] and [92] try to decrease latency and save resources by selecting super nodes in the network for mining. But they are centralized solutions at the mining level as it abandons the equity between the nodes in the network by having a whitelist for [105] and a list of super and random nodes for [43]. These solutions optimize the blockchain well but will be unable to handle a high blockchain throughput because they did not use all the computing power provided by the blockchain network solutions. [190] and [128] are interesting solutions as they increased blockchain throughput and latency performance for blockchains based on POW consensus protocol, but they failed to overcome the principle of using a lot of energy to finally get a useless result. 

_3.5.5 Governance and Dependency._ In the realm of complete governance and minimal reliance, there are solutions such as Neurochain aiming to realize the notion of an autonomous blockchain. This entails an approach governed by Artificial Intelligence, enabling a reduction in dependency on powerful nodes, as the need for complex hash puzzles to validate blocks is eliminated. 

NeuroChain [48] utilizes bots, or software agents, that are present on every participant’s blockchain, creating a decision and communication ecosystem. Their algorithms introduce a novel consensus mechanism based on evaluating each member’s interactions within the network and the quality of their exchanges. By sharing and assessing this information, the network achieves a faster and more energy-efficient selection of the member responsible for confirming a block while simultaneously performing a beneficial task. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:23 

## **3.6 What Critical Issues Remain Unresolved in Blockchain Technology, and How can AI Offer Potential Solutions for them?** 

As mentioned in the previous part, the Blockchain has several issues including sustainability, scalability, security, efficiency, and Governance and dependence. Artificial Intelligence approaches can enhance such limitations. For instance, deep learning algorithms can classify Blockchain peers’ activity patterns automatically by analyzing large amounts of data, such as transaction and communication data, to identify patterns and classify the activity of different peers. This classification of patterns could be useful for identifying malicious or suspicious activity on a blockchain network to detect hostile peers [173] and to take appropriate action to protect the network’s security. Using artificial intelligence prediction algorithms to discover unidentified paths exploits in blockchainbased applications to improve security, as to use machine learning algorithms to examine the transaction data on the blockchain to spot any irregularity that could indicate a vulnerability or in another case also it can be used to simulate different types of hacking attempts on a blockchain network and reveal any weaknesses that bad actors could take advantage of [73]. Additionally, the supervised machine learning system in the blockchain network can identify and block the Majority intrusion [62] or abnormal behaviors [35]. AI can be utilized to enhance various aspects beyond Blockchain consensus, such as achieving traceability decisions quickly instead of consensus decisions [44] or in the case of offloading the mining tasks from the mobile devices and adopting Edge Computing [128] to calculate resources allocation and to define conditional payment rules between mobile miners and edge computing service providers. 

There are still challenges that can be improved such as 

_3.6.1 Blockchain Governance._ Many articles have prophesied that soon machines will take over and manage human life. That future is already here, or at least a chunk of it is. On the blockchain, smart contracts, a sort of language, have been used to negotiate contracts between users, determining who will be paid. Smart contracts, for the time being, are not all that “smart,” and are confined to only the most basic types of contracts. AI may soon be capable of dealing with increasingly complicated scenarios such as automating mortgage system, automating escrow processes, generating a person’s digital identity that cannot be counterfeited, and managing income tax return declarations to prevent any tampering with tax returns. 

Therefore, having an autonomous blockchain backed by AI, intelligence is related to the evolution of smart contracts capabilities, so if we can implement all complex AI algorithms and protocol in smart contracts, blockchain which will be based on this smart contract and will be governed by an artificial intelligence entity being capable of performing tamper-proof and unbiased automated arbitration. All choices would be recorded on the blockchain, making them justifiable and transparent. 

_3.6.2 Scalability._ The increasing number of users and transactions on blockchain networks are putting a strain on the current blockchain solutions, and it is crucial that new and innovative solutions are developed to address this scalability issue. Furthermore, with the increasing adoption of blockchain technology in various industries, the demand for scalability solutions will only continue to grow. Without addressing this issue, the full potential of blockchain technology cannot be realized and its widespread adoption may be hindered. Today’s solutions for scalability in blockchain technology are still struggling to keep up with the rapid pace of growth. From these solutions, we can mention the pruning technique based on artificial intelligence for blocks with transactions that will not be used in future blocks. Future works should combine or add other techniques to the existing ones in order to achieve high scalability that can meet the needs of blockchain and that can keep up with the rapid growth of blockchain technology. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:24 

_3.6.3 Data Gatekeeper._ AI can help intelligently to deal with open source and open data for decision-making for changes made to it by checking or aggregating data from external sources to check their coherence and validity. Also, the widespread of using open data in the main existing systems and solutions around the world, set a new challenge of having intelligent open data. As blockchain-based data resources become more widely available, both businesses and consumers will require assistance with data accessibility, exploitation, and interpretation. AI’s capabilities make it ideal for such jobs. 

## **4 Conclusions** 

We made a comprehensive study on solutions that optimize Blockchain in many ways. We presented six research questions, and to answer them, we searched for articles in knowledge databases. The study’s first part highlighted technologies or solutions that optimized blockchain in any way. The second part focused on a specific type which groups solutions using artificial intelligence or machine learning algorithms to optimize blockchain, and simultaneously seeks solutions for Blockchain issues that are not yet solved or approached and that we can resolve with AI. As stated in the review, energy consumption and security are the trendiest topics in new research. Blockchain controlled by artificial intelligence or with consensus based on machine learning can use the energy of the mining job for significant results as training machine learning models. Concerning security, studies revealed that AI can identify and prevent assaults, as well as increase security and privacy in a variety of ways. 

From a managerial point of view, some key points arise. Blockchain has immense potential for business applications due to advantages like decentralization, transparency, and immutability. However, limitations around scalability, performance, security, and energy use need to be addressed for mainstream adoption. Managers should keep track of emerging optimization techniques like rollups, payment channels, multi-chain approaches, and consensus protocol enhancements to determine which solutions can best improve the scalability, efficiency, and sustainability of their blockchain implementations. Integrating AI and ML into blockchain systems can significantly enhance capabilities and address limitations. Managers should explore use cases around sustainability, scalability, security, efficiency, and governance where AI/ML integration can provide major benefits. In particular, AI/ML modules can be used to surrogate the optimization processes without affecting the performances of the blockchain system [38]. Managers should actively explore and incorporate these solutions to maximize the benefits and minimize the drawbacks of blockchain technology for business applications. Tracking ongoing research and developments can help managers capitalize on innovations in this rapidly evolving domain. 

In the future, the fusion of blockchain with artificial intelligence (AI) has the potential to create a slew of new blockchain-related advancements. However, the convergence of these two technologies is still in its infancy and therefore presents many research challenges that must be addressed, e.g., consensus protocols problems, standardization, normalization, scalability, and so on. Future study in this field has tremendous potential. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:25 

## **Appendices** 

## **A Glossary of Terms** 

|**Terms**|**Explanation**|
|---|---|
|ATN|A function for evaluating a node in Proof of Artifcial intelligence approach, which|
||serves as the primary criterion for choosing nodes, based on a comprehensive|
||assessment of the average number of transactions a node receives per time period.”.|
|Blockchain trilemma|It is the fact that no blockchain has been able to optimize simultaneously scalability,|
||decentralization, and security.|
|Consensus|A set of operations that starts with declaring a transaction as ofcial and ends with|
||mutually validating the results.|
|Consensus algorithm|In general, it is an algorithm for mutually verifying data on a distributed ledger. The|
||most well-known ones are Proof of Work and Proof of Stake, as well as other|
||methods.|
|Data gatekeeper|His role is to protect applications and data by using specifc tools to validate requests|
||and data.|
|Decentralized governance|It means that a blockchain transaction cannot be completed by a single central|
||authority.|
|Hard Forks|Is a signifcant modifcation to a network’s protocol that allows previously invalid|
||blocks and transactions to become legitimate, or vice versa.|
|Hash Functions|A key technology used in the Blockchain. It is a mathematical equation.|
|Node|In a communication network, it is a relay point, branch point, or endpoint.|
|Of-chain computation|Method making transaction calculations out of the mainnet.|
|Rollup|Rollups execute transactions of the main blockchain and then upload the data to the|
||main blockchain when consensus is attained. Rollups are classifed into two|
||types: _Optimistic_ _rollups_ : presume transactions are genuine by default and only|
||performs computation in the case of a challenge, using a fraud proof. _Zero-knowledge_|
||_rollups_ : performs computation of-chain and gives a validity proof to the chain.|
|Selfsh mining|Is a dishonest cryptocurrency mining technique in which one miner or a group|
||solves a hash, creates a new block, and keeps it of the public blockchain. This step|
||generates a fork, which is then mined to gain an advantage over the public|
||blockchain.|
|Sharding|A processing segmentation, as in distributed databases. Transactions are grouped|
||according to distinct shards and sent to a separate server based on the cluster.|
|Token|Tokens are virtual currencies that are unique to blockchains. Tokens are virtual|
||currencies that are used on blockchains to pay fees for asset management, and so on.|
|Users are not scaled|The solution has a predetermined number of users, e.g., Payment Channel solutions.|
|Artifcial intelligence:|Is the capacity to understand or interpret information by the machine. Artifcial|
||intelligence uses many techniques and approaches, such as support vector machines,|
||neural networks, swarm, and fuzzy logic...and so on.|
|Machine Learning:|ML is a type of computer programing that uses probability statistics to enable|
||machines to learn on their own without needing explicit programming **[** **174** **]** .|
|Supervised ML|A basic but rigorous technique. The operator provides the computer with examples|
||of the required inputs and outputs, and then the computer searches for solutions|
||based on those inputs to obtain those outputs [ 103 ].|
|Unsupervised ML:|In this type of machine learning, the algorithm is left alone to determine the input|
||structure [ 174 ]. This method can be the goal itself, or it can be means to a specifc|
||end.|
|Reinforcement ML|A computer program must accomplish a specifc task. The learning program receives|
||feedback in the form of “rewards” or “punishments” to recognize the most efective|
||behavior in a given context.|



ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:26 

## **B Abbreviations** 

|**Abbreviations**|**Original** **terms**|
|---|---|
|AI|Artifcial intelligence|
|ANN|Artifcial neural network|
|ATN|Average Transaction Number|
|BlockML|Block based on Machine Learning tasks|
|CAPTCHA|Completely Automated Public Turing test to tell Computers and Humans Apart|
|CNN|Convolutional Neural Network|
|DAIBCN|Decentralized Artifcial Intelligence enabled Blockchain Network|
|ECSP|Edge Computing Service Provider|
|IPFS|Interplanetary File System|
|MEC|Mobile Edge Computing|
|ML|Machine learning|
|MPC|Multiparty computation|
|Optimized PoW|Optimized Proof of Work|
|P2P|Peer to peer|
|PII|Proof of Involvement and integrity|
|PoA|Proof of Activity|
|PoAI|Proof of Artifcial Intelligence|
|PoB|Proof of Burn|
|PoC|Proof of capacity|
|PoDL|Proof of deep learning|
|PoDLwHO|Proof of deep learning with hyperparameter optimization|
|PoFL|Proof of Federated Learning|
|PoH|Proof of Human work|
|PoI|Proof of importance|
|PoKW|Proof of Kernel Work|
|PoL|Proof of Learning|
|PoLe|Proof of Learning|
|PoS|Proof of Stake|
|PoSearch|Proof of Search|
|PoSpace|Proof of Space|
|PoUW|Proof of Useful Work|
|PoW|Proof of Work|
|PPoV|Parallel Proof of Vote|
|SPoDL|separate proof of deep learning|
|Tx|Transaction|
|ZK rollup|Zero Knowledge rollup|



ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:27 

## **C Detailed Tables** 

Table 8. Most known Consensus Hash Algorithms 

|Hash algorithm|Description|Blockchain|
|---|---|---|
|SHA-256 [ 157 ]|SHA 256 is a member of the 2 Secure Hash Algorithm (SHA 2). It was a|Bitcoin, Bitcoin Cash,|
||collaborative project between the NSA and NIST. The 256 in the name|Bitcoin SV, Namecoin,|
||refers to the fnal hash digest value. The algorithm will always return 256|RSK, Nxt, Stacks,|
||bits regardless of the input size. It is true that the sha256 mining process|Peercoin|
||requires so much energy, but it has a high hash rate and is one of the||
||fnest algorithms for dealing with security problems.||
|Ethash [ 144 ]|Ethash was Ethereum’s PoW mining algorithm, it uses a function called|Ethereum, Ethereum|
||Keccak or SHA-3, as well as Dagger-Hashimoto hashes. Ethash is among|Classic, WhaleCoin,|
||the algorithm that uses memory intensively.|Expanse, Musicoin|
|Cryptonight [ 122 ]|CryptoNight is on the same degree of mystery as bitcoin, as we do not|Bytecoin, Dero, Monero,|
||know who invented it. The algorithm provides absolute anonymity and|Electroneum|
||was designed to be suitable for CPU and GPU mining as well as ASIC||
||resistance.||
|Scrypt [ 184 ]|Scrypt is a highly efcient hash algorithm designed to handle massive|Dogecoin, Litecoin,|
||volumes of data while still providing cryptographic integrity. It|ReddCoin, Carboncoin,|
||guarantees and facilitates restore when needed. This algorithm needs|Nyancoin, 42-coin|
||more RAM than other encryption algorithms, but it is faster and uses less||
||energy than the SHA256 algorithm.||
|X11 [ 26 ]|The X11 is not a hash algorithm in itself, but rather the union of 11 hash|Dash, Polis, SmartCoin|
||algorithms that are performed serially to get the fnal hash. The objective||
||behind its construction is to assure perfect security. This algorithm is||
||designed for GPU mining.||
|Blake2 [ 31 ]|BLAKE2 is a cryptographic hash algorithm that is quicker than MD5,|Siacoin, Verge, Decred,|
||SHA-1, SHA-2, and SHA-3 while remaining at least as safe as the most|Nano|
||recent standard SHA-3. Its remarkable attributes include speed,||
||simplicity, and robust security. It has two variants, Blake2s (32 bits) and||
||Blake2b (64bits).||



Table 9. Metrics for Some of the Best known Blockchains (blockchair.com) 

|Blockchain|Consensus|Through-put|Block Time|Confrmation|Node|Blockchain|Power Consumption|
|---|---|---|---|---|---|---|---|
||algorithm|(TPS)|(Sec)|time (Sec)|count|size (GB)|(kWh/Transaction)|
|Bitcoin|PoW|7|600|360|6708|445.66|1173|
|Ethereum|PoW (until mid-|15|15|360|6000|537|87.29|
||2022) – PoS (now)|||||||
|LiteCoin|PoW|53|150|1800|1,039|86.72|18.522|
|Qtum|PoS|70|120|1200|1,253|N/A|N/A|
|Waves|PoS|100|60|2|N/A|N/A|N/A|
|EOS|DPoS|4000|30|1.5|N/A|N/A|N/A|
|Cardano|DPoS|257|20|600|3173|94.24|0.547|
|TRON|DPoS|2000|3|300|N/A|N/A|N/A|
|Ripple|PBFT|1500|3,6|4|N/A|N/A|0.0079|
|Stellar|PBFT|1000|4,8|4|N/A|N/A|N/A|
|DASH|PoA|56|150|360|4,246|27.94|N/A|
|NEO|dBFT|1000|15|15|N/A|N/A|N/A|
|NEM|PoI|10000|60|30|N/A|N/A|N/A|



ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:28 

Table 10. Blockchain Attacks and Vulnerabilities 

|**Blockchain**|Attack name|Causes|Afected|Involved|Involved Blockchain|Afected|
|---|---|---|---|---|---|---|
|**Attacks**|||Blockchain|Blockchain|Processes|Blockchain|
|**categories**|||versions|layers||actors|
|**Double-**|||||||
|**Spending**|||||||
|**attacks**|||||||
||51% Attack [ 65 ]|Consensus|1.0, 2.0|Network,|Mining, Network|Network,|
|||mechanism||Consensus, Data|discovery, Block|Sellers,|
||||||Validation,|Exchange,|
||||||Transaction Creations,|Miner|
||Alternative History|Transaction|1.0, 2.0|Consensus|Mining, Transaction|Sellers|
||Attack [ 29 ]|Verifcation|||Creations, Block||
|||process|||Validation||
||Finney Attack [ 27 ]|Transaction|1.0, 2.0|Consensus|Mining, Transaction|Sellers|
|||Verifcation|||Creations, Block||
|||process|||Validation||
||Race Attack [ 30 ]|Transaction|1.0, 2.0|Consensus|Transaction Creations,|Sellers|
|||Verifcation|||Mining||
|||process|||||
||Vector76 Attack/|Transaction|1.0, 2.0|Consensus|Transaction Creations,|Exchange|
||one-confrmation|Verifcation|||Mining, Block||
||attack [ 28 ]|process|||Validation||
|**Mining/Pool**|||||||
|**attacks**|||||||
||Selfsh Mining [ 89 ]|Consensus|1.0, 2.0|Network,|Mining|Mining pool|
|||mechanism||Consensus||miners|
||Block-discard Attack|Consensus|1.0, 2.0|Network,|Mining|Mining pool|
||[ 18 ]|mechanism||Consensus||miners|
||Bribery Attack [ 69 ]|Consensus|1.0, 2.0|Network,|Mining, Transaction|Mining pool|
|||mechanism||Consensus|Creations, Block|miners|
||||||Validation||
||Block-Withholding|Consensus|1.0, 2.0|Network,|Mining|Mining pool|
||Attack [ 193 ]|mechanism||Consensus||miners|
||Fork-After-withhold|Consensus|1.0, 2.0|Network,|Mining|Mining pool|
||Attack [ 93 ]|mechanism||Consensus||miners|
||Pool Hopping|Consensus|1.0, 2.0|Network,|Mining|Mining pool|
||Attack [ 50 ]|mechanism||Consensus||miners|
|**Clients’/Wallet**|||||||
|**attacks**|||||||
||Vulnerable Digital|Poor|1.0, 2.0|Data|Signature generation|Users wallets|
||signature [ 36 ]|randomness|||||
||Users’ addresses|Public nature of|1.0, 2.0|Data|Signature generation|Users wallets|
||vulnerability: Lack|the blockchain|||||
||of control in address||||||
||creation [ 11 ]||||||
||Collison and|Flaws in|1.0, 2.0|Data|Signature generation|Users wallets|
||Pre-Image Attack|ECDSA, SHA256|||||
||[ 192 ]|and RIPEMD|||||
|||160|||||
||Flawed key|Flaws in|1.0, 2.0|Data|Signature generation|Users wallets|
||generation [ 191 ]|implementing|||||
|||ECDSA|||||
||Bugs and Malware|Client design|1.0, 2.0|Data|Signature generation|Users wallets|
||[ 186 ]|faws|||||
|||||||(Continued)|



ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:29 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

Table 10. Continued 

|**Blockchain**|Attack name|Causes|Afected|Involved|Involved Blockchain|Afected|
|---|---|---|---|---|---|---|
|**Attacks**|||Blockchain|Blockchain|Processes|Blockchain|
|**categories**|||versions|layers||actors|
|**Network**|||||||
|**attacks**|||||||
||Sybil Attack [ 136 ]|Forge identities|1.0, 2.0|Network|Network discovery|Network,|
|||||||miners, users|
||Eclipse Attack [ 53 ]|Flaws in|1.0, 2.0|Network|Network discovery|Users, Miners|
|||blockchain|||||
|||protocols -|||||
|||outgoing|||||
|||connections|||||
||Refund Attack [ 112 ]|Bitcoin refund|1.0, 2.0|Application|Payment protocol|Exchanges,|
|||policy|||authentication and|users, sellers|
||||||refund||
||Balance Attack [ 126 ]|Consensus|1.0, 2.0|Network,|Network discovery|Users, miners|
|||mechanism||Consensus|||
||Punitive and Feather|Consensus|1.0, 2.0|Network,|Mining|Users|
||forking Attack [ 108 ]|mechanism||Consensus, Data|||
||DDoS Attack [ 54 ]|External|1.0, 2.0|Network,|Network discovery,|Users, exchange,|
|||resources,||Consensus|Mining, Transaction|sellers, miners,|
|||contracts|||Creations, Block|pools, network|
|||underpriced|||Validation||
|||operations|||||
||Transaction|Flaws in|1.0, 2.0|Network,|Transaction Creations,|Exchanges|
||Malleability Attack|blockchain||Consensus, Data|Mining||
||[ 132 ]|protocols|||||
|||-Transaction ID|||||
||Timejacking Attack|Flaws in|1.0, 2.0|Network,|Transaction Creations,|Miners|
||[ 156 ]|blockchain||Consensus, Data|Mining||
|||protocols -|||||
|||timestamp|||||
|||handling|||||
||Partition Routing|Flaws in|1.0, 2.0|Network,|Network discovery|Users, Miners|
||Attack [ 180 ]|Internet routing||Consensus, Data|||
|||- routing|||||
|||manipulations|||||
||Delay Routing|Flaws in|1.0, 2.0|Network,|Network discovery|Users, Miners|
||Attack [ 10 ]|Internet routing||Consensus, Data|||
|||- routing|||||
|||manipulations|||||
|**Smart**|||||||
|**Contracts**|||||||
|**attacks**|||||||
||Vulnerabilities in|Program design|2.0|Execution|Transaction Creations|Contracts|
||contracts source|faws||||owner, sellers|
||code||||||
||Vulnerabilities in|EVM design|2.0|Execution|Transaction Creations,|Contracts|
||EVM Bytecode|faws|||Mining|owner, sellers|
||Vulnerabilities in|Program design|2.0|Network,|Transaction Creations,|Users, Sellers,|
||Blockchain|faws||Consensus|Mining|contracts owner,|
||Eclipse Attack on|EVM design|2.0|Network,|Network discovery|Miners, users|
||Smart contract|faws||Consensus|||
||blockchain [ 110 ]||||||
||Low-level attacks|underprice|2.0|Consensus, Data|Transaction Creations,|Miners,|
||[ 45 ]|operations|||Mining|network, users,|
|||||||sellers,|
|||||||exchange.|



ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:30 

Table 11. Most Common Smart Contract Vulnerabilities 

|Category of smart|Vulnerability|Cause|Real-World Attacks|
|---|---|---|---|
|contract vulnerability||||
|Vulnerabilities in|Reentrancy [ 170 ]|Improper Workfow caused by a non-recursive|The DAO Attack [ 114 ]|
|contracts source code||function re-enter before termination.||
||Exception disorders [ 13 ]|Irregularity in exception handling.|King of The Ether Attack [ 83 ]|
||Selfdestruct [ 171 ]|Improper Access Control.|Parity Library bug [ 178 ]|
||Arithmetic issues [ 164 ]|Incorrect Calculation.|PoWHcoin attack [ 21 ]|
||Keeping secrets [ 163 ]|Contracts private felds, secrecy is not guaranteed.|Multi-player games|
||Type casts|No expectation is thrown if type mismatched.|–|
||Gasless Send [ 13 ]|Invoking the Callee’s out-of-gas exception.|–|
||Call to the unknown [ 13 ]|Call of a non-existent function.|The DAO Attack [ 114 ]|
||tx.origin [ 166 ]|Using the variable for authorization could make a|–|
|||contract vulnerable if an authorized account calls||
|||into a malicious contract.||
||Default visibility [ 2 ]|Public visibility for functions and variables whose|Parity Wallet(First Hack) [ 140 ]|
|||visibility is not set.||
||⟨ Delegatecall ⟩to|Include Untrusted Control Party Functionality.|Parity Wallet(Second Hack) [ 4 ]|
||insecure contracts [ 167 ]|||
||External contract|Include Untrusted Control Party Functionality.|Honey Pot [ 160 ]|
||referencing [ 169 ]|||
|Vulnerabilities in EVM|Immutable bugs [ 131 ]|Defective contracts cannot be patched or|Rubixi [ 19 , 79 ] , GovernMental|
|||recovered.|Attack [ 19 ]|
||Stack size limit [ 82 ]|The number of values exceeds 1024 in the stack.|GovernMental Attack [ 19 ]|
||Ether lost in transfer [ 81 ]|Ether sent to orphan address is lost.|–|
|Vulnerabilities in|Generating randomness|Malicious miner biases the outcome of random|PRNG contract [ 143 ]|
|Blockchain|[ 172 ]|number.||
||Unpredictable state [ 168 ]|Concurrent Execution with Improper|Attack on Bancor [ 34 ]|
|||Synchronization.||
||Time constraints [ 165 ]|Altered Timestamp by a malicious miner.|GovernMental attack [ 19 ]|



## **References** 

- [1] ActBlue. 2022. _Fair Fight Donate via Actblue_ . Retrieved 2022-04-26 from https://secure.actblue.com/donate/fair-fightreproductive-rights 

- [2] Abdulrasheed Adediran. 2022. _Visibility in Solidity_ . Retrieved accessed 2023-01-11 from https://medium.com/ coinmonks/visibility-in-solidity-e758a4739c95 

- [3] Rachit Agarwal, Shikhar Barve, and Sandeep Shukla. 2021. Detecting malicious accounts in permissionless blockchains using temporal graph properties. _Applied Network Science_ 6, 1 (2021), 9. DOI:https://doi.org/10.1007/ s41109-020-00338-3 

- [4] Anthony Akentiev. 2018. _Parity Multisig Hacked. Again_ . Retrieved accessed 2022-12-25 from https://medium.com/ chain-cloud-company-blog/parity-multisig-hack-again-b46771eaa838 

- [5] Mustafa Al-Bassam, Alberto Sonnino, Shehar Bano, Dave Hrycyszyn, and George Danezis. 2017. Chainspace: A Sharded Smart Contracts Platform. Retrieved from https://arxiv.org/abs/1708.03778 

- [6] Shuyun Shi, Debiao He, Li Li, Neeraj Kumar, Muhammad Khurram Khan, and Kim-Kwang Raymond Choo. 2020. Applications of blockchain in ensuring the security and privacy of electronic health record systems: A survey. _Computers & Security_ 97 (2020), 101966. DOI:https://doi.org/10.1016/j.cose.2020.101966 

- [7] Amjad Almusaed, Ibrahim Yitmen, and Asaad Almssad. 2023. Enhancing smart home design with AI models: A case study of living spaces implementation review. _Energies_ 16, 6 (2023). DOI:https://doi.org/10.3390/en16062636 

- [8] Averin Andrey and Cheskidov Petr. 2019. Review of existing consensus algorithms blockchain. _2019 International Conference “Quality Management, Transport and Information Security, Information Technologies” (IT&QM&IS)_ , 124–127. DOI:https://doi.org/10.1109/ITQMIS.2019.8928323[Online; accessed 2022-05-04]. 

- [9] Marcin Andrychowicz, Stefan Dziembowski, Daniel Malinowski, and Lukasz Mazurek. 2014. Secure multiparty computations on bitcoin. In _2014 IEEE Symposium on Security and Privacy_ . 443–458. DOI:https://doi.org/10.1109/SP.2014. 35 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:31 

## A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

- [10] Maria Apostolaki, Aviv Zohar, and Laurent Vanbever. 2017. Hijacking bitcoin: Routing attacks on cryptocurrencies. In _2017 IEEE Symposium on Security and Privacy (SP’17)_ . 375–392. DOI:https://doi.org/10.1109/SP.2017.29 

- [11] Giuseppe Ateniese, Antonio Faonio, Bernardo Magri, and Breno de Medeiros. 2014. Certified bitcoins. In _Applied Cryptography and Network Security_ , Ioana Boureanu, Philippe Owesarski, and Serge Vaudenay (Eds.). Springer International Publishing, Cham, 80–96. 

- [12] Nicola Atzei, Massimo Bartoletti, and Tiziana Cimoli. 2017. A survey of attacks on ethereum smart contracts (SoK). In _Principles of Security and Trust_ , Springer Berlin Heidelberg, Berlin, Heidelberg, 164–186. 

- [13] Nicola Atzei, Massimo Bartoletti, and Tiziana Cimoli. 2017. A survey of attacks on ethereum smart contracts SoK. In _Proceedings of the 6th International Conference on Principles of Security and Trust—Volume 10204_ . Springer-Verlag, Berlin, 164–186. DOI:https://doi.org/10.1007/978-3-662-54455-6_8 

- [14] Abdullah Ayub Khan, Asif Ali Laghari, Mamoon Rashid, Hang Li, Abdul Rehman Javed, and Thippa Reddy Gadekallu. 2023. Artificial intelligence and blockchain technology for secure smart grid and power distribution Automation: A State-of-the-Art review. _Sustainable Energy Technologies and Assessments_ 57 (2023), 103282. DOI:https: //doi.org/10.1016/j.seta.2023.103282 

- [15] Davide Azzolini, Fabrizio Riguzzi, and Evelina Lamma. 2019. Studying transaction fees in the bitcoin blockchain with probabilistic logic programming. _Information_ 10, 11 (2019), 335. DOI:https://doi.org/10.3390/info10110335 

- [16] Djuri Baars. 2016. _Towards self-sovereign identity using blockchain technology_ . Ph. D. Dissertation. 

- [17] Adam Back, Matt Corallo, Luke Dashjr, Mark Friedenbach, Gregory Maxwell, Andrew Miller, Andrew Poelstra, Jorge Timón, and Pieter Wuille. 2014. Enabling blockchain innovations with pegged sidechains. https://blockstream.com/ sidechains.pdf 

- [18] Lear Bahack. 2013. _Theoretical Bitcoin Attacks with less than Half of the Computational Power (draft)_ . Retrieved from https://arxiv.org/abs/1312.7013 

- [19] T. Bahrynovska. 2017. _History of Ethereum Security Vulnerabilities, Hacks, and Their Fixes_ . Retrieved accessed 202212-25 from https://applicature.com/blog/blockchain-technology/history-of-ethereum-security-vulnerabilitieshacks-and-their-fixes/ 

- [20] Alejandro Baldominos and Yago Saez. 2019. Coin.AI: A Proof-of-Useful-Work scheme for blockchain-based distributed deep learning. _Entropy_ 21, 8 (2019). DOI:https://doi.org/10.3390/e21080723 

- [21] Eric Banisadr. 2019. _How $800k Evaporated from the PoWH Coin Ponzi Scheme Overnight_ . Retrieved accessed 2022-1225 from https://medium.com/@ebanisadr/how-800k-evaporated-from-the-powh-coin-ponzi-scheme-overnight1b025c33b530 

- [22] Carsten Baum, James Hsin yu Chiang, Bernardo David, and Tore Kasper Frederiksen. 2022. Eagle: Efficient Privacy Preserving Smart Contracts. _Cryptology ePrint Archive, Paper 2022/1435_ . Retrieved from https://eprint.iacr.org/2022/ 1435 https://eprint.iacr.org/2022/1435 

- [23] Eli Ben Sasson, Alessandro Chiesa, Christina Garman, Matthew Green, Ian Miers, Eran Tromer, and Madars Virza. 2014. Zerocash: Decentralized anonymous payments from bitcoin. In _2014 IEEE Symposium on Security and Privacy_ . 459–474. DOI:https://doi.org/10.1109/SP.2014.36 

- [24] Juan Benet. 2014. IPFS - Content Addressed, Versioned, P2P File System. DOI:https://doi.org/10.48550/ARXIV.1407. 3561 

- [25] Iddo Bentov, Charles Lee, Alex Mizrahi, and Meni Rosenfeld. 2014. Proof of Activity: Extending Bitcoin’s Proof of Work via Proof of Stake [Extended Abstract]y. 42, 3 (December 2014), 34–37. DOI:https://doi.org/10.1145/2695533. 2695545 

- [26] bit2me. 2022. _Top Most used Mining Algorithms in Blockchain_ . Retrieved accessed 2023-01-04 from https://academy. bit2me.com/en/top-algoritmos-de-mineria-mas-utilizados/ 

- [27] bit2me. 2022. _What is a Finney Hack or Finney Attack?_ Retrieved accessed 2023-01-02 from https://academy.bit2me. com/en/que-es-un-hackeo-finney-ataque-finney 

- [28] bit2me. 2022. _What is Vector Attack 76?_ Retrieved accessed 2023-01-02 from https://academy.bit2me.com/en/que-esataque-de-vector-76/ 

- [29] bitcoin wiki. 2022. _Irreversible Transactions_ . Retrieved accessed 2023-01-02 from https://en.bitcoin.it/wiki/ Irreversible_Transactions 

- [30] bitdegree. 2022. _What is Race Attack?_ Retrieved accessed 2023-01-02 from https://www.bitdegree.org/crypto/learn/ crypto-terms/what-is-race-attack 

- [31] BLAKE2. 2017. _BLAKE2 – Fast Secure Hashing_ . Retrieved accessed 2023-01-04 from https://www.blake2.net/ 

- [32] Jeremiah Blocki and Hong-Sheng Zhou. 2016. Designing proof of human-work puzzles for cryptocurrency and beyond. In _Theory of Cryptography_ , Springer Berlin Heidelberg, Berlin, Heidelberg, 517–546. 

- [33] Prakash Bodkhe, P. Mulkalwar, and Head. 2021. CAPTCHA techniques: An overview. _International Journal on Recent and Innovation Trends in Computing and Communication_ 5, 9 (March 2021), 15–21. 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:32 

- [34] Ivan Bogatyy. 2017. _Implementing Ethereum Trading Front-runs on the Bancor Exchange in Python_ . Retrieved accessed 2022-12-25 from https://hackernoon.com/front-running-bancor-in-150-lines-of-python-with-ethereumapi-d5e2bfd0d798 

- [35] Andreas Bogner. 2017. Seeing is understanding: Anomaly detection in blockchains with visualized features. In _Proceedings of the 2017 ACM International Joint Conference on Pervasive and Ubiquitous Computing and Proceedings of the 2017 ACM International Symposium on Wearable Computers (UbiComp’17)_ . Association for Computing Machinery, Maui, Hawaii, 5–8. DOI:https://doi.org/10.1145/3123024.3123157 

- [36] Joppe Bos, J. Halderman, Nadia Heninger, Jonathan Moore, Michael Naehrig, and Eric Wustrow. 2014. Elliptic curve cryptography in practice, Vol. 8437. DOI:https://doi.org/10.1007/978-3-662-45472-5_11 

- [37] Felipe Bravo-Marquez, Steve Reeves, and Martín Ugarte. 2019. Proof-of-Learning: A blockchain consensus mechanism based on machine learning competitions. In _2019 IEEE International Conference on Decentralized Applications and Infrastructures (DAPPCON)_ . 119–124. DOI:https://doi.org/10.1109/DAPPCON.2019.00023 

- [38] Maria Elena Bruni, Vittorio Capocasale, Marco Costantino, Stefano Musso, and Guido Perboli. 2023. Decentralizing electric vehicle supply chains: Value proposition and system design. _2023 IEEE 47th Annual Computers, Software, and Applications Conference (COMPSAC)_ , 1756–1761. DOI:https://doi.org/10.1109/COMPSAC57700.2023.00271 

- [39] David Budgen and Pearl Brereton. 2006. Performing systematic literature reviews in software engineering. _Proceedings—International Conference on Software Engineering_ , Vol. 2006. 1051–1052. DOI:https://doi.org/10.1145/ 1134285.1134500 

- [40] Jeff Burdges, Alfonso Cevallos, Peter Czaban, Rob Habermeier, Syed Hosseini, Fabio Lama, Handan Kilinc Alper, Ximin Luo, Fatemeh Shirazi, Alistair Stewart, and Gavin Wood. 2020. Overview of Polkadot and its Design Considerations. Retrieved from https://arxiv.org/abs/2005.13456 

- [41] Roberto Casado-Vara, Fernando De La Prieta, Javier Prieto, and Juan Corchado Rodríguez. 2018. Blockchain framework for IoT data quality via edge computing. 19–24. DOI:https://doi.org/10.1145/3282278.3282282 

- [42] Bitcoin Cash. 2022. _Bitcoin Cash—Argent Liquide électronique pair-à-pair_ . Retrieved accessed 2022-03-29 from https: //bitcoincash.org/ 

- [43] Jianwen Chen, Kai Duan, Rumin Zhang, Liaoyuan Zeng, and Wenyi Wang. 2018. An AI Based Super Nodes Selection Algorithm in BlockChain Networks. DOI:https://doi.org/10.48550/ARXIV.1808.00216 

- [44] Rui-Yang Chen. 2018. A traceability chain algorithm for artificial neural networks using TS fuzzy cognitive maps in blockchain. _Future Gener. Comput. Syst._ 80, C (March 2018), 198–210. DOI:https://doi.org/10.1016/j.future.2017.09.077 

- [45] Ting Chen, Xiaoqi Li, Ying Wang, Jiachi Chen, Zihao Li, Xiapu Luo, Man Ho Au, and Xiaosong Zhang. 2017. An adaptive gas cost mechanism for ethereum to defend against under-priced DoS attacks. DOI:https://doi.org/10.1007/ 978-3-319-72359-4_1 

- [46] Weili Chen, Zibin Zheng, Jiahui Cui, Edith Ngai, Peilin Zheng, and Yuren Zhou. 2018. Detecting ponzi schemes on ethereum: Towards healthier blockchain technology. In _Proceedings of the 2018 World Wide Web Conference (WWW’18)_ , International World Wide Web Conferences Steering Committee, Lyon, France, 1409–1418. DOI:https: //doi.org/10.1145/3178876.3186046 

- [47] Changhao Chenli, Boyang Li, Yiyu Shi, and Taeho Jung. 2019. Energy-recycling Blockchain with Proof-of-DeepLearning. In _2019 IEEE International Conference on Blockchain and Cryptocurrency (ICBC)_ . 19–23. DOI:https://doi. org/10.1109/BLOC.2019.8751419 

- [48] B. Chouli and F. Goujon. 2019. _NeuroChain: The Intelligent Blockchain_ . Retrieved March 29, 2022 from https://www. neurochaintech.io/pdf/NeuroChain_White_Paper.pdf 

- [49] Mauro Conti, E. Sandeep Kumar, Chhagan Lal, and Sushmita Ruj. 2018. A survey on security and privacy issues of bitcoin. _IEEE Communications Surveys & Tutorials_ 20, 4 (2018), 3416–3452. DOI:https://doi.org/10.1109/COMST. 2018.2842460 

- [50] Eugenio Cortesi, Francesco Bruschi, Stefano Secci, and Sami Taktak. 2022. A new approach for Bitcoin pool-hopping detection. _Computer Networks_ 205 (2022), 108758. DOI:https://doi.org/10.1016/j.comnet.2021.108758 

- [51] Cosmos. 2022. _Cosmos Network_ . Retrieved accessed 2022-03-29 from https://cosmos.network 

- [52] Jerry M. Couretas. 2022. _Taxonomy of Cyber Threats_ . Springer International Publishing, 37–56. DOI:https://doi.org/ 10.1007/978-3-030-88559-5_3 

- [53] Qianyi Dai, Bin Zhang, and Shuqin Dong. 2022. Eclipse attack detection for blockchain network layer based on deep feature extraction. _Wireless Communications and Mobile Computing_ 2022, 1 (2022), 1451813. DOI:https://doi.org/10. 1155/2022/1451813 

- [54] Qian-yi Dai, Bin Zhang, and Shu-qin Dong. 2022. A DDoS-Attack detection method oriented to the blockchain network layer. _Security and Communication Networks_ 2022, 1 (2022), 5692820. DOI:https://doi.org/10.1155/2022/5692820 

- [55] Xiaohai Dai, Jiang Xiao, Wenhui Yang, Chaofan Wang, and Hai Jin. 2019. Jidar: A Jigsaw-like data reduction approach without trust assumptions for Bitcoin system. In _2019 IEEE 39th International Conference on Distributed Computing Systems (ICDCS)_ . 1317–1326. DOI:https://doi.org/10.1109/ICDCS.2019.00132 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:33 

- [56] George Danezis and Sarah Meiklejohn. 2016. Centrally banked cryptocurrencies. In _Porceedings of the Network and Distributed System Security Symposium (NDSS’16)_ (2016-02-22). DOI:https://doi.org/10.14722/ndss.2016.23187 

- [57] Pietro Danzi, Anders E. Kalør, Čedomir Stefanović, and Petar Popovski. 2019. Delay and communication tradeoffs for blockchain systems with lightweight IoT clients. _IEEE Internet of Things Journal_ 6, 2 (2019), 2354–2365. DOI:https: //doi.org/10.1109/JIOT.2019.2906615 

- [58] Eman-Yasser Daraghmi, Yousef-Awwad Daraghmi, and Shyan-Ming Yuan. 2019. MedChain: A design of blockchainbased system for medical records access and permissions management. _IEEE Access_ 7 (2019), 164595–164613. DOI:https://doi.org/10.1109/ACCESS.2019.2952942 

- [59] Christian Decker and Roger Wattenhofer. 2015. A fast and scalable payment network with bitcoin duplex micropayment channels. DOI:https://doi.org/10.1007/978-3-319-21741-3_1 

- [60] Xia Deng, Junbin Shao, Le Chang, and Junbin Liang. 2021. A blockchain-based authentication protocol using cryptocurrency technology in LEO satellite networks. _Electronics_ 10, 24 (2021). DOI:https://doi.org/10.3390/ electronics10243151 

- [61] Saurabh Deochake. 2022. Belief-desire-intention (BDI) multi-agent system for cloud marketplace negotiation. (06 2022). 

- [62] Somdip Dey. 2018. Securing majority-attack in blockchain using machine learning and algorithmic game theory: A proof of work. In _2018 10th Computer Science and Electronic Engineering (CEEC)_ . 7–10. DOI:https://doi.org/10.1109/ CEEC.2018.8674185 

- [63] Omar Dib, Kei-Léo Brousmiche, Antoine Durand, Eric Thea, and Elyes Ben Hamida. 2018. Consortium blockchains: Overview, applications and challenges. _International Journal On Advances in Telecommunications_ (2018). Retrieved from https://hal.science/hal-02271063 

- [64] Donghui Ding, Xin Jiang, Jiaping Wang, Hao Wang, Xiaobing Zhang, and Yi Sun. 2019. _Txilm: Lossy Block Compression with Salted Short Hashing_ . 

- [65] Rohul Amin Dipu. 2020. 51% ATTACKS ON BLOCKCHAIN: A SOLUTION ARCHITECTURE FOR BLOCKCHAIN TO SECURE IOT WITH PROOF OF WORK. DOI:https://doi.org/10.13140/RG.2.2.13763.12328 

- [66] Mingxiao Du, Ma Xiaofeng, Zhang Zhe, Wang Xiangwei, and Chen Qijun. 2017. A review on consensus algorithm of blockchain. In _2017 IEEE International Conference on Systems, Man, and Cybernetics (SMC’17)_ (2017-10-01). 2567–2572. DOI:https://doi.org/10.1109/SMC.2017.8123011 

- [67] Paul Dunphy and Fabien A. P. Petitcolas. 2018. A first look at identity management schemes on the blockchain. _IEEE Security & Privacy_ 16, 4 (7 2018), 20–29. DOI:https://doi.org/10.1109/MSP.2018.3111247 

- [68] Stefan Dziembowski, Sebastian Faust, Vladimir Kolmogorov, and Krzysztof Pietrzak. 2015. Proofs of space. 585–605. DOI:https://doi.org/10.1007/978-3-662-48000-7_29 

- [69] Ghader Ebrahimpour and Mohammad Sayad Haghighi. 2021. Analysis of Bitcoin Vulnerability to Bribery Attacks Launched Through Large Transactions. Retrieved from https://arxiv.org/abs/2105.07501 

- [70] Ethereum. 2022. _Ethereum Sharding 2.0_ . Retrieved accessed 2022-03-29 from https://ethereum.org 

- [71] Yuqi Fan, Lunfei Wang, Weili Wu, and Dingzhu Du. 2021. Cloud/edge computing resource allocation and pricing for mobile blockchain: An iterative greedy and search approach. _IEEE Transactions on Computational Social Systems_ 8, 2 (2021), 451–463. DOI:https://doi.org/10.1109/TCSS.2021.3049152 

- [72] Mohamed Amine Ferrag and Lei Shu. 2020. Blockchain technologies for the Internet of Things: Research issues and challenges. _IEEE Internet of Things Journal_ 7, 5 (2020), 3247–3266. 

- [73] Ahmad Firdaus, Nor Anuar, Mohd Faizal, Ibrahim Hashem, Syafiq Bachok, and Arun Kumar. 2018. Root exploit detection and features optimization: Mobile device and blockchain based medical data management. _Journal of Medical Systems_ 42, 6 (2018), 112. DOI:https://doi.org/10.1007/s10916-018-0966-x 

- [74] Valentina Gatteschi, Fabrizio Lamberti, Claudio Demartini, Chiara Pranteda, and Víctor Santamaría. 2018. Blockchain and smart contracts for insurance: Is the technology mature enough? _Future Internet_ 10, 2 (20 2 2018), 20. DOI:https://doi.org/10.3390/fi10020020 

- [75] Gabriele Gatti, Cataldo Basile, and Guido Perboli. 2023. An expert system for automatic cyber risk assessment and its AI-based improvements. _2023 IEEE 47th Annual Computers, Software, and Applications Conference (COMPSAC)_ , 1434–1440. DOI:https://doi.org/10.1109/COMPSAC57700.2023.00220 

- [76] Abdelatif Hafid, Abdelhakim Senhaji Hafid, and Mustapha Samih. 2020. Scaling blockchains: A comprehensive survey. _IEEE Access_ 8 (2020), 125244–125262. DOI:https://doi.org/10.1109/ACCESS.2020.3007251 

- [77] Harmony. 2022. _Harmony Whitepaper_ . Retrieved accessed 2022-03-29 from https://harmony.one/whitepaper.pdf 

- [78] Péter Heged?s. 2019. Towards analyzing the complexity landscape of solidity based ethereum smart contracts. _Technologies_ 7, 1 (2019). DOI:https://doi.org/10.3390/technologies7010006 

- [79] Pete Humiston. 2018. _Smart Contract Attacks [Part 2] - Ponzi Games Gone Wrong_ . Retrieved accessed 2022-12-25 from https://hackernoon.com/smart-contract-attacks-part-2-ponzi-games-gone-wrong-d5a8b1a98dd8 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:34 

- [80] Adedoyin Ahmed Hussain and Fadi Al-Turjman. 2021. Artificial intelligence and blockchain: A review. _Transactions on Emerging Telecommunications Technologies_ 32, 9 (2021), e4268. DOI:https://doi.org/10.1002/ett.4268 

- [81] immunebytes. 2020. _A Detailed Analysis of What Happens when Ether is Lost in a Transfer_ . Retrieved accessed 2023-01-11 from https://www.immunebytes.com/blog/a-detailed-analysis-of-what-happens-when-ether-islost-in-a-transfer/ 

- [82] immunebytes. 2020. _Size Limit in Smart Contracts_ . Retrieved accessed 2023-01-11 from https://www.immunebytes. com/blog/size-limit-in-smart-contracts/ 

- [83] Post-Mortem Investigation. 2016. _King of the Ether_ . Retrieved accessed 2022-12-25 from https://www.kingoftheether. com/postmortem.html 

- [84] Poon Joseph and Dryja Thaddeus. 2022. _Lightning-network-paper_ . Retrieved accessed 2022-03-29 from https:// lightning.network/lightning-network-paper.pdf 

- [85] Poon Joseph and Buterin Vitalik. 2022. _Plasma: Scalable Autonomous Smart Contracts_ . Retrieved March 29, 2022 from https://www.plasma.io/plasma.pdf 

- [86] Harry Kalodner, Steven Goldfeder, Xiaoqi Chen, S. Matthew Weinberg, and Edward W. Felten. 2018. Arbitrum: Scalable, private smart contracts. _27th USENIX Security Symposium (USENIX Security 18)_ , 1353–1370. Retrieved fromhttps://www.usenix.org/conference/usenixsecurity18/presentation/kalodner 

- [87] Ghassan O. Karame, Elli Androulaki, and Srdjan Capkun. 2012. Double-spending fast payments in bitcoin. In _Proceedings of the 2012 ACM Conference on Computer and Communications Security_ (Raleigh, North Carolina, USA) (CCS’12). Association for Computing Machinery, New York, NY, USA, 906–917. DOI:https://doi.org/10.1145/2382196.2382292 

- [88] Kostis Karantias, Aggelos Kiayias, and Dionysis Zindros. 2020. Proof-of-burn. 523–540. DOI:https://doi.org/10.1007/ 978-3-030-51280-4_28 

- [89] Michal Kedziora, Patryk Kozłowski, Michał Szczepanik, and Piotr Jóźwiak. 2020. _Analysis of Blockchain Selfish Mining Attacks_ . 231–240. DOI:https://doi.org/10.1007/978-3-030-30440-9_22 

- [90] Eleftherios Kokoris-Kogias, Philipp Jovanovic, Linus Gasser, Nicolas Gailly, Ewa Syta, and B. Ford. 2018. OmniLedger: A secure, scale-out, decentralized ledger via sharding. _2018 IEEE Symposium on Security and Privacy (SP)_ (2018). DOI:https://doi.org/10.1109/SP.2018.000-5 

- [91] Konan Jean-Claude Kouassi. 2022. A Comprehensive Overview of Artificial Intelligence. 173–194. DOI:https://doi. org/10.2139/ssrn.4311525 

- [92] S. Kumaresh and K. B. P. Iyer. 2021. Decentralised artificial intelligence enabled blockchain network model. _Turkish Journal of Computer and Mathematics Education (TURCOMAT)_ 12, 10 (28 4 2021), 3797–3805. DOI:https://doi.org/10. 17762/turcomat.v12i10.5074 

- [93] Yujin Kwon, Dohyun Kim, Yunmok Son, Eugene Vasserman, and Yongdae Kim. 2017. Be selfish and avoid Dilemmas: Fork after withholding (FAW) attacks on Bitcoin. In _(CCS’17)_ , Association for Computing Machinery, Dallas, Texas, USA, 195–209. DOI:https://doi.org/10.1145/3133956.3134019 

- [94] Yixiao Lan, Yuan Liu, and Boyang Li. 2020. _Proof of Learning (PoLe): Empowering Machine Learning with Consensus Building on Blockchains_ . 

- [95] Bahareh Lashkari and Petr Musilek. 2021. A comprehensive review of blockchain consensus mechanisms. _IEEE Access_ PP (03 2021), 1–1. DOI:https://doi.org/10.1109/ACCESS.2021.3065880 

- [96] Larissa Lee. 2015. New kids on the blockchain: How bitcoin’s technology could reinvent the stock market. _SSRN Electronic Journal_ (2015). DOI:https://doi.org/10.2139/ssrn.2656501 [Online; accessed 2022-05-04]. 

- [97] Lik-Hang Lee, Tristan Braud, Peng Yuan Zhou, Lin Wang, Dianlei Xu, Zijun Lin, Abhishek Kumar, Carlos Bermejo, and Pan Hui. 2024. All one needs to know about metaverse: A complete survey on technological singularity, virtual ecosystem, and research agenda. _Found. Trends Hum.-Comput. Interact._ 18, 2–3 (November 2024), 100–337. DOI:https: //doi.org/10.1561/1100000095 

- [98] Wei Liang, Lijun Xiao, Ke Zhang, Mingdong Tang, Dacheng He, and Kuan-Ching Li. 2021. Data fusion approach for collaborative anomaly intrusion detection in blockchain-based systems. _IEEE Internet of Things Journal_ PP (01 2021), 1–1. DOI:https://doi.org/10.1109/JIOT.2021.3053842 

- [99] Liquidity. 2022. _Liquidity Network_ . Retrieved accessed 2022-03-29 from https://liquidity.network/research/ 

- [100] Zibin Zheng, Shaoan Xie, Hong-Ning Dai, Xiangping Chen, and Huaimin Wang. 2017. An Overview of Blockchain Technology: Architecture, Consensus, and Future Trends. DOI:https://doi.org/10.1109/BigDataCongress.2017.85 

- [101] Mengting Liu, F. Richard Yu, Yinglei Teng, Victor C. M. Leung, and Mei Song. 2018. Computation offloading and content caching in wireless blockchain networks with mobile edge computing. _IEEE Transactions on Vehicular Technology_ 67, 11 (2018), 11008–11021. DOI:https://doi.org/10.1109/TVT.2018.2866365 

- [102] Mengting Liu, Richard Yu, Yinglei Teng, and Mei Song. 2019. Performance optimization for blockchain-enabled industrial Internet of Things (IIoT) systems: A deep reinforcement learning approach. _IEEE Transactions on Industrial Informatics_ PP (02 2019), 1–1. DOI:https://doi.org/10.1109/TII.2019.2897805 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:35 

- [103] Yiming Liu, F. Richard Yu, Xi Li, Hong Ji, and Victor C. M. Leung. 2020. Blockchain and machine learning for communications and networking systems. _IEEE Communications Surveys & Tutorials_ 22, 2 (2020), 1392–1431. DOI:https://doi.org/10.1109/COMST.2020.2975911 

- [104] Eric Lombrozo, Johnson Lau, and Pieter Wuille. 2015. _Segregated Witness (Consensus layer)_ . Retrieved from https: //bips.xyz/141 

- [105] Leif-Nissen Lundbæk, Daniel Janes Beutel, Michael Huth, Stephen Jackson, Laurence Kirk, and Robert Steiner. 2018. Proof of kernel work: A democratic low-energy consensus for distributed access-control protocols. _Royal Society Open Science_ 5, 8 (8 2018), 180422. DOI:https://doi.org/10.1098/rsos.180422 

- [106] Xiong Luo, Pan Yang, Weiping Wang, Yang Gao, and Manman Yuan. 2021. S-PoDL: A two-stage computationalefficient consensus mechanism for blockchain-enabled multi-access edge computing. _Physical Communication_ 46 (2021), 101338. DOI:https://doi.org/10.1016/j.phycom.2021.101338 

- [107] Loi Luu, Viswesh Narayanan, Chaodong Zheng, Kunal Baweja, Seth Gilbert, and Prateek Saxena. 2016. A secure sharding protocol for open blockchains. In _Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ (2016-10-24). 17–30. DOI:https://doi.org/10.1145/2976749.2978389 

- [108] Antonio Magnani, Luca Calderoni, and Paolo Palmieri. 2018. Feather forking as a positive force: Incentivising green energy production in a blockchain-based smart grid. In _Proceedings of the 1st Workshop on Cryptocurrencies and Blockchains for Distributed Systems_ (Munich, Germany) (CryBlock’18). Association for Computing Machinery, New York, NY, 99–104. DOI:https://doi.org/10.1145/3211933.3211951 

- [109] Alex Manuskin, Michael Mirkin, and Ittay Eyal. 2019. _Ostraka: Secure Blockchain Scaling by Node Sharding_ . 

- [110] Yuval Marcus, Ethan Heilman, and Sharon Goldberg. 2020. Low-resource eclipse attacks on ethereum’s peer-to-peer network. _IACR Cryptol. ePrint Arch._ 2018 (2020), 236. 

- [111] Ahmad Akmaluddin Mazlan, Salwani Mohd Daud, Suriani Mohd Sam, Hafiza Abas, Siti Zaleha Abdul Rasid, and Muhammad Fathi Yusof. 2020. Scalability challenges in healthcare blockchain system—a systematic review. _IEEE Access_ 8 (2020), 23663–23673. DOI:https://doi.org/10.1109/ACCESS.2020.2969230 

- [112] Patrick McCorry, Siamak Shahandashti, and Feng Hao. 2017. Refund attacks on bitcoin’s payment protocol. 581–599. DOI:https://doi.org/10.1007/978-3-662-54970-4_34 

- [113] Thomas McGhin, Kim-Kwang Raymond Choo, Charles Zhechao Liu, and Debiao He. 2019. Blockchain in healthcare applications: Research challenges and opportunities. _Journal of Network and Computer Applications_ 135 (2019), 62– 75. DOI:https://doi.org/10.1016/j.jnca.2019.02.027 

- [114] Muhammad Mehar, Charlie Shier, Alana Giambattista, Elgar Gong, Gabrielle Fletcher, Ryan Sanayhie, Henry M. Kim, and Marek Laskowski. 2017. Understanding a revolutionary and flawed grand experiment in blockchain: The DAO attack. _Journal of Cases on Information Technology_ 21, 1 (2017), 19–32. DOI:https://doi.org/10.2139/ssrn.3014782 

- [115] Abdul Wahab and Waqas Mehmood. 2018. _Survey of Consensus Protocols_ . Retrieved from https://arxiv.org/abs/1810. 03357 

- [116] Andrea Merlina. 2019. BlockML: A useful proof of work system based on machine learning tasks. In _Proceedings of the 20th International Middleware Conference Doctoral Symposium (Middleware’19)_ , Association for Computing Machinery, Davis, California, 6–8. DOI:https://doi.org/10.1145/3366624.3368156 

- [117] Andrew Miller, Iddo Bentov, Surya Bakshi, Ranjit Kumaresan, and Patrick McCorry. 2019. Sprites and state channels: Payment networks that go faster than lightning. 508–526. DOI:https://doi.org/10.1007/978-3-030-32101-7_30 

- [118] Anshul Mittal and Swati Aggarwal. 2020. Hyperparameter optimization using sustainable proof of work in blockchain. _Frontiers in Blockchain_ 3 (20 5 2020). DOI:https://doi.org/10.3389/fbloc.2020.00023 

- [119] Jelena Mišić, Vojislav B. Mišić, and Xiaolin Chang. 2020. On the benefits of compact blocks in Bitcoin. In _ICC 2020 - 2020 IEEE International Conference on Communications (ICC)_ . 1–6. DOI:https://doi.org/10.1109/ICC40277.2020. 9149418 

- [120] Shahriar Mohammadi and Elnaz Rabieinejad. 2020. Prediction forks in the blockchain using machine learning. In _Proceedings of the 3rd Conference on Mechanical, Electrical, and Computer_ . https://civilica.com/doc/1162237 

- [121] Bhabendu Kumar Mohanta, Debasish Jena, Soumyashree S. Panda, and Srichandan Sobhanayak. 2019. Blockchain technology: A survey on applications and security privacy challenges. _Internet of Things_ 8 (2019), 100107. DOI:https: //doi.org/10.1016/j.iot.2019.100107 

- [122] monerodocs. 2022. _Cryptonight_ . Retrieved accessed 2023-01-04 from https://monerodocs.org/proof-of-work/ cryptonight/ 

- [123] Ahmed Afif Monrat, Olov Schelén, and Karl Andersson. 2020. Performance evaluation of permissioned blockchain platforms. In _2020 IEEE Asia-Pacific Conference on Computer Science and Data Engineering (CSDE’20)_ . 1–8. DOI:https: //doi.org/10.1109/CSDE50874.2020.9411380 

- [124] Xiaoyang Zhu and Youakim Badr. 2018. A survey on blockchain-based identity management systems for the internet of things. In _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ . 1568–1573. DOI:https://doi.org/10.1109/Cybermatics_2018.2018.00263 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:36 

- [125] Satoshi Nakamoto. 2009. Bitcoin: A peer-to-peer electronic cash system. _Cryptography Mailing list at_ https:// metzdowd.com (24 3 2009). 

- [126] Christopher Natoli and Vincent Gramoli. 2016. The balance attack against proof-of-work blockchains: The R3 testbed as an example. Retrieved from https://arxiv.org/abs/1612.09426 

- [127] Cong T. Nguyen, Dinh Thai Hoang, Diep N. Nguyen, Dusit Niyato, Huynh Tuong Nguyen, and Eryk Dutkiewicz. 2019. Proof-of-Stake consensus mechanisms for future blockchain networks: Fundamentals, applications and opportunities. _IEEE Access_ 7 (2019), 85727–85745. DOI:https://doi.org/10.1109/ACCESS.2019.2925010 

- [128] Cong Nguyen, Zehui Xiong, Ping Wang, and Dusit Niyato. 2018. Optimal auction for edge computing resource management in mobile blockchain networks: A deep learning approach. 1–6. DOI:https://doi.org/10.1109/ICC.2018. 8422743 

- [129] Giang-Truong Nguyen and Kyungbaek Kim. 2018. A survey about consensus algorithms used in blockchain. _Journal of Information Processing Systems_ 14, 1 (2018), 101–128. DOI:https://doi.org/10.3745/JIPS.01.0024 

- [130] P. K. Paul, A. Bhuimali, Sreeramana Aithal, and Rajesh Rajamony. 2019. Vulnerability in information technology and computing- a study in technological information assurance. _International Journal of Management, Technology, and Social Sciences_ (11 2019), 87–94. DOI:https://doi.org/10.47992/IJMTS.2581.6012.0074 

- [131] Aleksandros Toplidis Pavel Zverev. 2022. _Cost of Failure in Smart-Contract Development_ . Retrieved accessed 202301-11 from https://maddevs.io/blog/cost-of-failure-in-smart-contract-development/ 

- [132] paxful. 2020. _Bitcoin Transaction Malleability Explained_ . Retrieved accessed 2023-01-11 from https://paxful.com/ university/bitcoin-transaction-malleability-explained/ 

- [133] G. Perboli, V. Capocasale, and D. Gotta. 2020. Blockchain-based transaction management in smart logistics: A sawtooth framework. _Proceedings—2020 IEEE 44th Annual Computers, Software, and Applications Conference, COMPSAC 2020_ . DOI:https://doi.org/10.1109/COMPSAC48688.2020.000-8 

- [134] Guido Perboli, Stefano Musso, and Mariangela Rosano. 2018. Blockchain in logistics and supply chain: A lean approach for designing real-world use cases. _IEEE Access_ 6, 1 (2018), 62018–62028. DOI:https://doi.org/10.1109/ ACCESS.2018.2875782 

- [135] Marc Pilkington. 2017. _Can Blockchain Improve Healthcare Management? Consumer Medical Electronics and the IoMT_ . DOI:https://doi.org/10.2139/ssrn.3025393 

- [136] Swathi Punathumkandi, Chirag Modi, and Dhiren Patel. 2019. Preventing sybil attack in blockchain using distributed behavior monitoring of miners. 1–6. DOI:https://doi.org/10.1109/ICCCNT45670.2019.8944507 

- [137] Chao Qiu, Xiaoxu Ren, Yifan Cao, and Tianle Mai. 2021. Deep reinforcement learning empowered adaptivity for future blockchain networks. _IEEE Open Journal of the Computer Society_ 2 (2021), 99–105. DOI:https://doi.org/10. 1109/OJCS.2020.3010987 

- [138] Xidi Qu, Shengling Wang, Qin Hu, and Xiuzhen Cheng. 2021. Proof of federated learning: A novel energy-recycling consensus algorithm. _IEEE Transactions on Parallel and Distributed Systems_ 32, 8 (2021), 2074–2085. DOI:https://doi. org/10.1109/TPDS.2021.3056773 

- [139] QuarkChain. 2022. _QuarkChain—A Flexible, Scalable, and User-oriented Blockchain Infrastructure By Applying Blockchain Sharding Technology_ . Retrieved accessed 2022-03-29 from https://quarkchain.io/ 

- [140] Haseeb Qureshi. 2017. _A Hacker Stole $31M of Ether—how it Happened, and what it Means for Ethereum_ . Retrieved accessed 2022-12-25 from https://www.freecodecamp.org/news/a-hacker-stole-31m-of-ether-how-it-happened-andwhat-it-means-for-ethereum-9e5dc29e33ce/ 

- [141] Raiden. 2022. _Raiden Network_ . Retrieved accessed 2022-03-29 from https://raiden.network/ 

- [142] Jeff Reed. 2018. _Litecoin: An Introduction to Litecoin Cryptocurrency and Litecoin Mining_ . 

- [143] Arseniy Reutov. 2018. _Predicting Random Numbers in Ethereum Smart Contracts_ . Retrieved accessed 2022-12-25 from https://blog.positive.com/predicting-random-numbers-in-ethereum-smart-contracts-e5358c6b8620 

- [144] Rodrigo Herrerai. 2022. _ETHASH_ . Retrieved accessed 2023-01-04 from https://ethereum.org/en/developers/docs/ consensus-mechanisms/pow/mining-algorithms/ethash/ 

- [145] Stuart Russell and Peter Norvig. 2010. _Artificial Intelligence: A Modern Approach_ (3 ed.). Prentice Hall. 

- [146] Khaled Salah, M. Habib Ur Rehman, Nishara Nizamuddin, and Ala Al-Fuqaha. 2019. Blockchain for AI: Review and open research challenges. _IEEE Access_ 7 (2019), 10127–10149. DOI:https://doi.org/10.1109/ACCESS.2018.2890507 

- [147] Mehrdad Salimitari, Mohsen Joneidi, and Mainak Chatterjee. 2019. AI-enabled blockchain: An outlier-aware consensus protocol for blockchain-based IoT networks. In _2019 IEEE Global Communications Conference (GLOBECOM’19)_ . 1–6. DOI:https://doi.org/10.1109/GLOBECOM38437.2019.9013824 

- [148] Sarwar Sayeed and Hector Marco-Gisbert. 2019. Assessing blockchain consensus and security mechanisms against the 51% attack. _Applied Sciences_ 9, 9 (2019). DOI:https://doi.org/10.3390/app9091788 

- [149] Markus Schäffer, Monika Di Angelo, and Gernot Salzer. 2019. _Performance and Scalability of Private Ethereum Blockchains_ . 103–118. DOI:https://doi.org/10.1007/978-3-030-30429-4_8 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

73:37 

- [150] Francesco Scicchitano, Angelica Liguori, Massimo Guarascio, Ettore Ritacco, and G. Manco. 2020. A Deep Learning Approach for Detecting Security Attacks on Blockchain. In _Italian Conference on Cybersecurity_ . Retrieved from https://api.semanticscholar.org/CorpusID:216555088 

- [151] Pratima Sharma, Rajni Jindal, and Malaya Borah. 2020. Blockchain technology for cloud storage: A systematic literature review. _Comput. Surveys_ 53, 4 (2020), 32. DOI:https://doi.org/10.1145/3403954 

- [152] Meng Shen, Aijing Gu, Jiawen Kang, Xiangyun Tang, Xiaodong Lin, Liehuang Zhu, and Dusit Niyato. 2023. Blockchains for artificial intelligence of things: A comprehensive survey. _IEEE Internet of Things Journal_ 10, 16 (2023), 14483–14506. DOI:https://doi.org/10.1109/JIOT.2023.3268705 

- [153] Naoki Shibata. 2019. Proof-of-search: Combining blockchain consensus formation with solving optimization problems. _IEEE Access_ PP (28 11 2019), 1–1. DOI:https://doi.org/10.1109/ACCESS.2019.2956698 

- [154] ShipChain. 2022. _ShipChain - Learn Everything About Blockchain, Cryptocurrency & Investing_ . Retrieved 2022-03-21 from https://shipchain.io 

- [155] Mohammed Shuaib, Noor Hafizah Hassan, Sahnius Usman, Shadab Alam, Surbhi Bhatia, Arwa Mashat, Adarsh Kumar, and Manoj Kumar. 2022. Self-sovereign identity solution for blockchain-based land registry system: A comparison. _Mobile Information Systems_ 2022, 1 (2022), 8930472. DOI:https://doi.org/10.1155/2022/8930472 

- [156] Guðmundur Sigurðsson, Alberto Giaretta, and Nicola Dragoni. 2019. Vulnerabilities and security breaches in cryptocurrencies. 288–299. DOI:https://doi.org/10.1007/978-3-030-14687-0_26 

- [157] simplilearn. 2022. _A Definitive Guide to Learn The SHA-256 (Secure Hash Algorithms)_ . Retrieved accessed 2023-01-04 from https://www.simplilearn.com/tutorials/cyber-security-tutorial/sha-256-algorithm 

- [158] Amritraj Singh, Kelly Click, Reza M. Parizi, Qi Zhang, Ali Dehghantanha, and Kim-Kwang Raymond Choo. 2020. Sidechain technologies in blockchain networks: An examination and state-of-the-art review. _Journal of Network and Computer Applications_ 149 (2020), 102471. DOI:https://doi.org/10.1016/j.jnca.2019.102471 

- [159] Ayushi Singh, Gulafsha Shujaat, Isha Singh, Abhishek Tripathi, and Divya Thakur. 2019. A survey of blockchain technology security. _Int. J. Comput. Eng. Res. Trends_ 6, 4 (April 2019), 299–303. 

- [160] JAGJIT SINGH. 2021. _What is a Honeypot Crypto Scam and how to Spot it?_ Retrieved accessed 2022-12-25 from https: //cointelegraph.com/news/what-is-a-honeypot-crypto-scam-and-how-to-spot-it 

- [161] Nasrin Sohrabi and Zahir Tari. 2020. On the scalability of blockchain systems. In _2020 IEEE International Conference on Cloud Engineering (IC2E’20)_ . IEEE, 124–133. 

- [162] Nasrin Sohrabi and Zahir Tari. 2020. ZyConChain: A scalable blockchain for general applications. _IEEE Access_ 8 (2020), 158893–158910. DOI:https://doi.org/10.1109/ACCESS.2020.3020319 

- [163] solidity-by example. 2022. _Accessing Private Data_ . Retrieved accessed 2023-01-11 from https://solidity-by-example. org/hacks/accessing-private-data/ 

- [164] solidity-by example. 2022. _Arithmetic Overflow and Underflow_ . Retrieved accessed 2023-01-11 from https://solidityby-example.org/hacks/overflow/ 

- [165] solidity-by example. 2022. _Block Timestamp Manipulation_ . Retrieved accessed 2023-01-11 from https://solidity-byexample.org/hacks/block-timestamp-manipulation/ 

- [166] solidity-by example. 2022. _Delegatecall_ . Retrieved accessed 2023-01-11 from https://solidity-by-example.org/hacks/ phishing-with-tx-origin/ 

- [167] solidity-by example. 2022. _Delegatecall_ . Retrieved accessed 2023-01-11 from https://solidity-by-example.org/hacks/ delegatecall/ 

- [168] solidity-by example. 2022. _Front Running_ . Retrieved accessed 2023-01-11 from https://solidity-by-example.org/ hacks/front-running/ 

- [169] solidity-by example. 2022. _Hiding Malicious Code with External Contract_ . Retrieved accessed 2023-01-11 from https: //solidity-by-example.org/hacks/hiding-malicious-code-with-external-contract/ 

- [170] solidity-by example. 2022. _Re-Entrancy_ . Retrieved accessed 2023-01-11 from https://solidity-by-example.org/hacks/ re-entrancy/ 

- [171] solidity-by example. 2022. _Self Destruct_ . Retrieved accessed 2023-01-11 from https://solidity-by-example.org/hacks/ self-destruct/ 

- [172] solidity-by example. 2022. _Source of Randomness_ . Retrieved accessed 2023-01-11 from https://solidity-by-example. org/hacks/randomness/ 

- [173] Huayun Tang, Yingying Jiao, Butian Huang, Changting Lin, Shubham Goyal, and Bei Wang. 2018. Learning to classify blockchain peers according to their behavior sequences. _IEEE Access_ 6 (2018), 71208–71215. DOI:https://doi. org/10.1109/ACCESS.2018.2881431 

- [174] Sudeep Tanwar, Qasim Bhatia, Pruthvi Patel, Aparna Kumari, Pradeep Kumar Singh, and Wei-Chiang Hong. 2020. Machine learning adoption in blockchain-based smart applications: The challenges, and a way forward. _IEEE Access_ 8 (2020), 474–488. DOI:https://doi.org/10.1109/ACCESS.2019.2961372 

- [175] Don Tapscott and Alex Tapscott. 2018. How Blockchain Will Change Organizations. 43–56. DOI:https://doi.org/10. 7551/mitpress/11645.003.0010 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

M. Abdelhamid et al. 

73:38 

- [176] The Elrond Team. 2022. _A Highly Scalable Public Blockchain via Adaptive State Sharding and Secure Proof of Stake_ . Retrieved accessed 2022-03-29 from https://elrond.com/assets/files/elrond-whitepaper.pdf 

- [177] The Zilliqa Team. 2022. _The Zilliqa Project: A Secure, Scalable Blockchain Platform_ . Retrieved accessed 2022-03-29 from https://docs.zilliqa.com/positionpaper.pdf 

- [178] Parity Technologies. 2017. _A Postmortem on the Parity Multi-Sig Library Self-Destruct_ . Retrieved accessed 2022-12-25 from https://www.parity.io/blog/a-postmortem-on-the-parity-multi-sig-library-self-destruct/ 

- [179] Jason Teutsch and Christian Reitwießner. 2019. _A Scalable Verification Solution for Blockchains_ . 

- [180] Muoi Tran, Inho Choi, Gi Moon, Viet Anh Vu, and Min Kang. 2020. A stealthier partitioning attack against bitcoin peer-to-peer network. 894–909. DOI:https://doi.org/10.1109/SP40000.2020.00027 

- [181] Quang Tran, Benjamin Turnbull, Hao-tian Wu, A. J. S. Silva, Katerina Kormusheva, and Jiankun Hu. 2021. A survey on privacy-preserving blockchain systems (PPBS) and a novel PPBS-based framework for smart agriculture. _IEEE Open Journal of the Computer Society_ PP (01 2021), 1–1. DOI:https://doi.org/10.1109/OJCS.2021.3053032 

- [182] Alpesh Vaghela and Suthar Anil Kumar. 2022. Medical health data privacy preservation using smart contract. _AIP Conference Proceedings_ 2451, 1 (2022), 020052. DOI:https://doi.org/10.1063/5.0095361 

- [183] VeChain. 2022. _VeChain_ . Retrieved accessed 2022-03-21 from https://vechain.com/cn/ 

- [184] Lane Wagner. 2020. _Basic Intro to the Scrypt Hash_ . Retrieved accessed 2023-01-04 from https://blog.boot.dev/ cryptography/very-basic-intro-to-the-scrypt-hash/ 

- [185] Waltonchain. 2022. _Waltonchain_ . Retrieved 2022-03-21 from https://www.waltonchain.org/#/ 

- [186] Zhiyuan Wan, David Lo, Xin Xia, and Liang Cai. 2017. Bug characteristics in blockchain systems: A large-scale empirical study. 413–424. DOI:https://doi.org/10.1109/MSR.2017.59 

- [187] Han Wang, Hui Li, Abla Smahi, Mingrui Xiao, and Shuo-Yen Robert Li. 2023. GBT-CHAIN: A system framework for solving the general trilemma in permissioned blockchains. _Distrib. Ledger Technol._ (aug 2023). DOI:https://doi.org/ 10.1145/3615871Just Accepted. 

- [188] Jiaping Wang and Hao Wang. 2019. _Monoxide: Scale Out Blockchain with Asynchronous Consensus Zones_ . DOI:10. 13140/RG.2.2.32017.48489 

- [189] Qin Wang, Rujia Li, Qi Wang, and Shiping Chen. 2021. _Non-Fungible Token (NFT): Overview, Evaluation, Opportunities and Challenges_ . Retrieved from https://arxiv.org/abs/2105.07447 

- [190] Taotao Wang, Soung Chang Liew, and Shengli Zhang. 2021. When blockchain meets AI: Optimal mining strategy achieved by machine learning. _International Journal of Intelligent Systems_ 36 (1 2 2021), 2183–2207. DOI:https://doi. org/10.1002/int.22375 

- [191] Wiki. 2015. _Emerging Risk Report – 2015_ . Retrieved accessed 2023-01-11 from https://assets.lloyds.com/assets/pdfbitcoin-bitcoin-final/1/pdf-bitcoin-bitcoin-final.pdf 

- [192] Wiki. 2019. _Collision Attack_ . Retrieved accessed 2023-01-11 from https://en.bitcoinwiki.org/wiki/Collision_attack 

- [193] Di Wu, dong Xiang-Liu, bin Xiang-Yan, Rui Peng, and Gang Li. 2019. Equilibrium analysis of bitcoin block withholding attack: A generalized model. _Reliability Engineering & System Safety_ 185 (2019), 318–328. DOI:https://doi. org/10.1016/j.ress.2018.12.026 

- [194] Zhenwei Xiao, Hui Li, Han Wang, Guoliang Yang, Qiongwei Ye, Shusheng Zou, Ping Lu, and Qi Lyu. 2022. Optimizing parallel proof of vote consensus based on mimic security in consortium blockchains. In _2022 IEEE International Conference on Big Data (Big Data)_ . 3215–3224. DOI:https://doi.org/10.1109/BigData55660.2022.10021130 

- [195] Zihuan Xu, Siyuan Han, and Lei Chen. 2018. CUB, a consensus unit-based storage scheme for blockchain system. In _2018 IEEE 34th International Conference on Data Engineering (ICDE’18)_ . 173–184. DOI:https://doi.org/10.1109/ICDE. 2018.00025 

- [196] Ruizhe Yang, F. Richard Yu, Pengbo Si, Zhaoxin Yang, and Yanhua Zhang. 2019. Integrated blockchain and edge computing systems: A survey, some research issues and challenges. _IEEE Communications Surveys & Tutorials_ 21, 2 (2019), 1508–1532. DOI:https://doi.org/10.1109/COMST.2019.2894727 

- [197] Maleh Yassine, Mohammad Shojafar, Mamoun Alazab, Imed Romdhani, and Ujjwal KC. 2020. _Blockchain for Cybersecurity and Privacy: Architectures, Challenges, and Applications_ . 

- [198] Guangsheng Yu, Xu Wang, Kan Yu, Wei Ni, J. Andrew Zhang, and Ren Ping Liu. 2020. Survey: Sharding in blockchains. _IEEE Access_ 8 (2020), 14155–14181. DOI:https://doi.org/10.1109/ACCESS.2020.2965147 

- [199] Jusik Yun, Yunyeong Goh, and Jong-Moon Chung. 2021. DQN-based optimization framework for secure sharded blockchain systems. _IEEE Internet of Things Journal_ 8, 2 (2021), 708–722. DOI:https://doi.org/10.1109/JIOT.2020. 3006896 

- [200] Mahdi Zamani, Mahnush Movahedi, and Mariana Raykova. 2018. RapidChain: Scaling blockchain via full sharding. In _Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security_ (2018-10-15). 931–948. DOI:https://doi.org/10.1145/3243734.3243853 

- [201] Caiming Zhang and Yang Lu. 2021. Study on artificial intelligence: The state of the art and future prospects. _Journal of Industrial Information Integration_ 23 (2021), 100224. DOI:https://doi.org/10.1016/j.jii.2021.100224 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

73:39 

## A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions 

- [202] Jianting Zhang, Zicong Hong, Xiaoyu Qiu, Yufeng Zhan, Song Guo, and Wuhui Chen. 2020. SkyChain: A deep reinforcement learning-empowered dynamic blockchain sharding system. 1–11. DOI:https://doi.org/10.1145/3404397. 3404460 

- [203] Lei Zhang, Zhaohui Wu, Yu Chen, Jing Zhang, and Isaac Woungang. 2020. Decentralized identity: Toward a privacy-preserving and secure authentication scheme for 5G-enabled IoT. _IEEE Internet of Things Journal_ 8, 2 (2020), 1504–1515. 

- [204] Rui Zhang, Rui Xue, and Ling Liu. 2022. Security and privacy for healthcare blockchains. _IEEE Transactions on Services Computing_ 15, 6 (2022), 3668–3686. DOI:https://doi.org/10.1109/TSC.2021.3085913 

- [205] Yufan Zhang, Zichao Chen, Luyao Zhang, and Xin Tong. 2022. Visualizing Non-Fungible Token Ethics: A Case Study On CryptoPunks. _ArXiv_ abs/2206.12922 (2022). Retrieved from https://api.semanticscholar.org/CorpusID:250073142 

- [206] Xingxiong Zhu. 2019. Research on blockchain consensus mechanism and implementation. _IOP Conference Series: Materials Science and Engineering_ 569 (08 2019), 042058. DOI:https://doi.org/10.1088/1757-899X/569/4/042058 

- [207] Yiping Zuo, Jiajia Guo, Ning Gao, Yongxu Zhu, Shimei Jin, and Xiao Li. 2023. A survey of blockchain and artificial intelligence for 6G wireless communications. _IEEE Communications Surveys & Tutorials_ 25 (2023), 2494–2528. Retrieved from https://api.semanticscholar.org/CorpusID:258685503 

Received 2 September 2023; revised 15 July 2024; accepted 1 October 2024 

ACM Comput. Surv., Vol. 57, No. 3, Article 73. Publication date: November 2024. 

