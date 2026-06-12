Hindawi Security and Communication Networks Volume 2021, Article ID 6126247, 16 pages https://doi.org/10.1155/2021/6126247 

## _Research Article_ 

# **The Applications of Blockchain in Artificial Intelligence** 

## **Ruonan Wang** ©© **,[1][,][2] Min Luo ,[1] Yihong Wen,[3] Lianhai Wang,[4] Kim-Kwang Raymond Choo** © **,[5] and Debiao He** © **1,2** 

> _1School of Cyber Science and Engineering, Wuhan University, Wuhan, China_ 

> _2Guangxi Key Laboratory of Trusted Software, Guilin University of Electronic Technology, Guilin, China_ 

> _3$e 54th Research Institute of China Electronics Technology Group Corporation, Shijiazhuang, China_ 

_4Shandong Provincial Key Laboratory of Computer Networks, Qilu University of Technology (Shandong Academy of Sciences), Jinan 250014, China_ 

> _5Department of Information Systems and Cyber Security, University of Texas at San Antonio, San Antonio, TX 78249, USA_ 

Correspondence should be addressed to Min Luo; mluo@whu.edu.cn 

Received 21 June 2021; Accepted 8 September 2021; Published 28 September 2021 

Academic Editor: Wenxiu Ding 

Copyright © 2021 Ruonan Wang et al. )is is an open access article distributed under the Creative Commons Attribution License, which permits unrestricted use, distribution, and reproduction in any medium, provided the original work is properly cited. 

)ere has been increased interest in applying artificial intelligence (AI) in various settings to inform decision-making and facilitate predictive analytics. In recent times, there have also been attempts to utilize blockchain (a peer-to-peer distributed system) to facilitate AI applications, for example, in secure data sharing (for model training), preserving data privacy, and supporting trusted AI decision and decentralized AI. Hence, in this paper, we perform a comprehensive review of how blockchain can benefit AI from these four aspects. Our analysis of 27 English-language articles published between 2018 and 2021 identifies a number of research challenges and opportunities. 

## **1. Introduction** 

Artificial intelligence (AI), an important branch of computer science, underpins the research and development of the theory(ies), method(s), technology(ies), and application(s) for simulating, extending, and expanding human intelligence. While AI was first proposed in 1956, the interest in AI probably increased significantly after AlphaGo (an AI-based computer program) defeated Lee Sedol, the world Go champion. AI has been applied in diverse settings, ranging from healthcare [1–4] to drug discovery [5] to (medical) image recognition [6–9] to automated driving [10, 11] and so on. McKinsey Global Institute, for example, predicted that the AI market will grow to 13 trillion dollars by 2030 [12]. 

)ere are three key aspects in AI technology, namely, data, algorithm, and computing power, in the sense that significant data is required for training the algorithm to obtain a classification model, and the training process requires significant computing power. In our big data era, data can come from different sources (e.g., sensor systems, 

Internet of )ings (IoT) devices and systems, and social media platforms) and/or owned by different stakeholders. )is may lead to some challenges. 

One of the key challenges is isolated data islands, where data from one source/stakeholder is not accessible to others or the training of the AI model or it is too expensive or impractical to collect the large volume of distributed data for centralized processing and training [13, 14]. )ere is also the risk of being a single point of failure in centralized architectures [15], which may lead to data tampering. 

In addition, data from different sources can be unstructured and vary in quality. It can also be difficult to determine the source and authenticity of the data. )ere is also the risk of invalid or malicious data. All these limitations can impact on the accuracy of the prediction [16]. For example, in 2017, a group of researchers from MIT demonstrated how one can trick Google’s AI classification to classify a 3D printed turtle as a rifle [17]. It has also been shown that fake biometric features can be used to taint recognition models, to facilitate impersonation and fraud 

Security and Communication Networks 

2 

[18]. Such attacks are also referred to as adversarial machine learning in the literature, and it is an ongoing research topic [19–22]. However, it is difficult for conventional AI architectures to screen data effectively and/or track malicious data providers. Conventional centralized AI architectures may also lead to privacy disclosure and data abuse. For example, AI training tasks may need to handle sensitive user data, such as users’ medical data, which may be divulged or even tampered with during the training process. )ere are also privacy regulations that may limit the sharing of user data, even for model training. )is motivates the design of data sharing/trading platforms [23]. 

In practice, AI models are created, trained, and used by different entities. )e training process is opaque to the users, and users may not fully trust the model they are using. In addition, as AI algorithms become more and more complex, it is difficult for people to understand how the training result is obtained. Hence, a recent trend is to move away from centralized AI approaches to decentralized AI approaches. 

Compared with AI, blockchain is a relatively young technology as it was first proposed by Nakamoto and Wright in 2008 [24]. Blockchain, a peer-to-peer distributed system, ensures tamper-proofing through the underlying hash algorithm and time stamp technology. )e privacy of data stored on the blockchain is guaranteed by using some cryptographic algorithms. )rough the use of smart contracts, the program can be executed automatically to ensure the credibility of the execution results. )rough consensus mechanism and distributed ledger technology, all nodes can participate in bookkeeping and verify the transactions. )e market capitalization of Bitcoin as of February 20, 2020, was approximately 175 billion dollars [25]. As shown in Table 1, these characteristics of blockchain may overcome the challenges faced by AI; hence, it forms the focus on this paper. 

)ere has been some research on how blockchain and AI can be combined. In [26], the authors described how the integration of blockchain and AI can develop a new ecosystem for a decentralized economy in terms of decentralized data storage and management, infrastructure, and AI applications. However, there is a lack of discussion on how the specific technology of blockchain is applied and how privacy is protected. )e works in [27–29] focused on the integration of blockchain and artificial intelligence and their mutual reinforcement. In [30], the authors discussed the feasibility and benefits of combining blockchain and artificial intelligence for energy cloud management. But it does not categorise the literature available for discussion, while the application scenarios are limited. 

Many companies have also done a lot of exploration of blockchain-based AI applications. SingularityNet [31] utilized blockchain technology to build distributed AI trading marketplaces. TraneAi [32] has created a blockchain-based AI platform to accelerate the artificial intelligence training process in a decentralized manner. Neureal [33] is a distributed open source platform for artificial intelligence that provides peer-to-peer super AI computing. In general, most of the applications are aimed at constructing distributed ecosystems and infrastructures via blockchain. 

In this paper, we survey the existing literature focusing on the applications of blockchain in AI. Specifically, we searched using keywords such as (blockchain and AI) on major academic databases (e.g., IEEE Xplore, ScienceDirect, ACM Digital Library, and SpringerLink) for articles published in English between Feb, 2018 and Jan, 2021. We located over 500 articles, and we excluded articles that are not directly relevant. Eventually, we included 27 articles for discussion in this paper. 

)e rest of the paper is organized as follows. In the next two sections, we briefly introduce blockchain and AI, prior to presenting our review of the applications of blockchain in AI (i.e., data sharing, privacy protection, trusted AI decision, and decentralized AI) in Section 4. Finally, we discuss the findings, research challenges, and identified research opportunities in the last section. 

## **2. Blockchain Technology** 

)e architecture of blockchain mainly comprises the data layer, network layer, consensus layer, incentive layer, contract layer, and application layer, see also Figure 1. 

)e data layer mainly focuses on the data structure, including the hash function, digital signature, Merkle tree, asymmetric encryption, and other technologies. )e most important structure of the data layer is the block, and the block structure is shown in Figure 2. A block consists of both the block head and the block body. )e block header contains the Merkle root, timestamp, and hash value of the current block and previous block. )e block body mainly includes transaction information and Merkle tree. Each transaction is signed by the transaction’s initiator and then processed and verified by the miner. )e verified transaction is embedded in the block. )e hash value of every transaction is combined in pairs to calculate the hash, and then, the resulting hash value is combined in pairs to calculate the hash value again until the Merkle root, which is recorded in the block header. Every change to the information about every transaction stored on the blockchain affects the Merkle root. In this way, the tamper-proofing of blockchain can be realized. Every block additionally stores the hash value of the previous block and timestamp, resulting in a time sorted chain. 

)e network layer mainly contains P2P network, design of the data communication mechanism, and data verification mechanism. )ere is no centralized server in the blockchain. All messages propagate between nodes in a peerto-peer manner. All nodes maintain the blockchain together. One node generates a new block and transmits it to the other nodes. Other nodes store the copy of the block after verification. Subsequent blocks will also be generated on the basis of this block. In this way, all nodes can maintain a bottom ledger. 

)e consensus layer mainly includes various consensus algorithms. )e consensus algorithm is used to determine which node can add new blocks to the main chain. Common consensus algorithms include PoW, PoS, and PBFT. 

)e incentive layer mainly includes some incentive measures. )ere is no centralized server in the blockchain, so the safe operation of the blockchain depends on the active 

Security and Communication Networks 

3 

Table 1: Blockchain for AI. 

|||Table 1:|Blockchain for AI.|
|---|---|---|---|
||Blockchain|AI|Blockchain for AI|
|Data|(i) Trust<br>(ii) Security|(i) Ensure high quality<br>data<br>(ii) Secure data sharing|Blockchain supports AI in terms of facilitating trustworthy data and<br>secure data sharing|
|Algorithm|(i) Automation<br>(ii) Immutability<br>(iii) Traceability|(i) Need a credible training<br>process|Model training is automatically executed by the smart contract, which<br>greatly improves the credibility of the training results|
||(i)|||
|Computer|Decentralization|(i) Centralized AI-high|)e decentralized blockchain structure provides distributed computing|
|power|(ii) Trust|computing cost|power for AI|
||(iii) Traceability|||



**==> picture [236 x 267] intentionally omitted <==**

**----- Start of picture text -----**<br>
Application Smart City IoT ......<br>Layer<br>ContractLayer Script Code Algorithm ContractSmart<br>Incentive Issuing Allocation<br>Layer Mechanism Mechanism<br>Consensus<br>PoW PoS ......<br>Layer<br>Network P2P Communication Verification<br>Layer Mechanism Mechanism<br>Hash<br>Block Function Time Stamp<br>Data<br>Layer Merkle Tree Digital Asymmetric<br>Signature Encryption<br>**----- End of picture text -----**<br>


Figure 1: Blockchain architecture. 

participation of each node. At present, the commonly used incentive measures include (1) the corresponding reward for each block’s bookkeeping right and (2) the service charge for each transaction. With the development of blockchain, the design of the incentive layer of blockchain in the future is not only limited to economic rewards but also to achieve common goals. 

)e contract layer encapsulates several scripts, algorithms, and smart contracts to support the programmable features of blockchain. )rough the preset rules and conditions, it can be automatically executed without a third party. )is is the foundation of blockchain trust. )e last part is the application layer. It contains all types of blockchain applications, including finance, law, audit, and health care. 

_2.1. Smart Contract._ Smart contract was first defined by Szabo in 1994 [34]. Smart contracts have not been adopted on a large scale because of the lack of a reliable execution environment before the introduction of blockchain 

**==> picture [236 x 182] intentionally omitted <==**

**----- Start of picture text -----**<br>
block k block k+1 block k+2<br>... Hash of k Hash of k+1 Hash of k+2<br>Hash of k-1 Hash of k Hash of k+1 ...<br>Nonce Nonce Nonce<br>Time Stamp Time Stamp Time Stamp<br>... ... ...<br>Merkle Root Merkle Root Merkle Root<br>Hash<br>Hash12 Hash34<br>Hash1 Hash2 Hash3 Hash4<br>Tx1 Tx1 Tx1 Tx1<br>**----- End of picture text -----**<br>


Figure 2: Structure of blocks. 

technology. With the rapid development of distributed ledger technology, especially the large-scale deployment and application of blockchain, people pay more attention to smart contract, which is the key and important part of distributed ledger technology. 

Smart contract is a kind of computer protocol that can self execute, self enforce, self verify, and self restrict the execution of its instructions. It allows transactions to be executed between untrusted or anonymous parties without the need for a trusted third party. )ese transactions are traceable and irreversible. Smart contract consists of value, address, function, and state. )e transaction is taken as input, the corresponding code is executed, and an output event is triggered; then, the state changes according to the functional logic [35]. All parties agree on the details of the smart contract in advance, including scenarios that trigger contract execution, state transition rules, and responsibility for breach of contract. )en, the smart contract is deployed on the blockchain in the form of a code. After that, when the requirements are satisfied, the smart contract will be triggered and automatically executed. 

Ethereum is the most popular platform for the development of smart contract [36]. )e code of Ethereum smart contract is written in stack-based bytecode language and runs on Ethereum virtual machine (EVM). Solidity and Serpent are often used to write smart contracts. Hyperledger 

Security and Communication Networks 

4 

fabric can also deploy smart contract, which is called “chain code.” It is the single interaction channel with the blockchain and the only source for transaction generation. Chain code is usually developed using Go or Java. 

_2.2. Consensus Mechanism._ )ere is no trust relationship between each node in the blockchain. It is necessary to coordinate each independent node in order to share information in such a network. )erefore, the network system will decide who is the next bookkeeper through relevant protocols so as to reach a consensus of which is the consensus mechanism. )e essence of the consensus mechanism is to solve the problem of decentralized trust [37]. It is an essential technology for the independent operation of the blockchain. In the steady functioning of a blockchain system, a good consensus mechanism plays a very significant part. )e blockchain can successfully negotiate and construct a consistent blockchain structure using an effective consensus mechanism. 

)ere are two sorts of consensus algorithms. One is for non-Byzantine fault, such as RAFT and Paxos. )e other is for the Byzantine general problem [38], such as PoW, PoS, DPoS, and PBFT. )ere are two ways to deal with Byzantine fault. One is to limit the probability of malicious behavior by increasing the cost of doing evil, such as PoW and PoS. )e PoW algorithm will count computing power as cost, and the PoS algorithm will count stakes as cost. Another way is to design certain rules. Even if there are certain malicious nodes, all other nodes can still reach a consensus, such as the practical Byzantine fault tolerant algorithm. Several common consensus algorithms are described as follows: 

- (1) Proof of Work: the PoW algorithm was first proposed in Bitcoin, and its core idea is the competition of node computing power. )e miner can have the bookkeeping right by consuming a lot of computing power to calculate a hash value that meets the requirements [39]. )e block will be added to blockchain after the other nodes validating it. )en, the bookkeeping node will be rewarded. In the PoW consensus mechanism, it takes a lot of resources for malicious nodes to destroy the system (control more than 50 percent of nodes). )erefore, it can limit the malicious behavior of malicious nodes. PoW can be decentralized, and nodes can enter and leave freely. But obviously, it will cause a waste of resources and low efficiency. 

- (2) Proof of Stake: the PoS algorithm is an alternative to solve the waste of resources in the PoW algorithm. It reduces the difficulty of mining due to the number and time of tokens taken by each node. To a certain extent, it shortens the time to reach consensus and avoids a lot of waste of resources in the PoW algorithm. But, at the same time, PoS benefits wealthy miners and may lead to near monopoly. )erefore, blockchain projects using the PoS algorithm usually need to run the PoW consensus algorithm for a period of time and then convert to PoS to prevent a 

large number of stakes accumulating in a small number of nodes. 

- (3) Delegated Proof of Stake: the DPoS consensus algorithm is improved on the basis of the PoS algorithm. )e consensus process no longer requires all participating nodes to fight for the bookkeeping rights, but pick some representatives through voting. It greatly improves the efficiency of consensus. 

)ere are also some other consensus algorithms. )e comparison of consensus algorithms is shown in Table 2. 

_2.3. Taxonomy._ Generally, blockchain can be divided into three types according to the access level of blockchain data: public blockchain, private blockchain, and consortium blockchain [40]. )e comparison of these three types of blockchain is shown in Table 3. 

_2.3.1. Public Blockchain._ All records stored in the public blockchain are open and transparent to the public, and all nodes can join and leave the blockchain network freely. Everyone can check and verify the transaction and also compete for the rights to bookkeeping. Bitcoin and Ethereum are both public blockchains. 

_2.3.2. Private Blockchain._ )e private blockchain is completely controlled by an organization. Not every node is allowed to participate in the blockchain. Only those nodes from specific organizations are allowed to join the competition for bookkeeping rights. It has strict authority management for data access. 

_2.3.3. Consortium Blockchain._ It is a combination of public blockchain and private blockchain. )e nodes with permission are selected in advance to participate in the consensus process of the consortium chain. Other nodes can participate in the transactions, but cannot obtain the bookkeeping rights. )e data in the blockchain can be public or private. )e consortium chain can be seen as partially decentralized. Hyperledger fabric is a consortium blockchain. 

## **3. Artificial Intelligence** 

)e research of AI covers a wide range of topics, including machine learning, computer vision, and natural language processing. Among them, machine learning is an important technology that allows AI to imitate human thought and behavior, and most current AI programs are based on it. Machine learning has been developed over a long period of time, now has a relatively complete technical framework and mature algorithms, and has developed techniques such as deep learning, reinforcement learning, and federated learning. 

_3.1. Machine Learning._ Machine learning was first defined by Samuel [41] in 1959 as “the field of study that gives computers the ability to learn without being explicitly 

Security and Communication Networks 

5 

Table 2: Comparison of consensus algorithms. 

||Table 2: Co|mparison of consensus|algorithms.|||
|---|---|---|---|---|---|
|Characteristics|PoW|PoS|DPoS|PBFT|Raft|
|Crash fault tolerance (%)|50|50|50|33|50|
|Byzantine fault tolerance (%)|50|50|50|33|NA|
|Verifcation speed (s)|>100|<100|<100|<10|<10|
|Scalability|High|High|High|Low|Low|
|)roughput|Very low|Low|Low|Average|Very high|
|Consistency|Probabilistic|Probabilistic|Probabilistic|High|High|
|Energy consumption|Very high|Low|Low|Low|Low|



Table 3: Comparison of blockchain. 

||Table 3: C|omparison of blockchain.|omparison of blockchain.||
|---|---|---|---|---|
|Property|Public blockchain|Consortium blockchain||Private blockchain|
|Governance type|Public|Managed by a set of|participants|Managed by a single organization|
|Read permission|Public||Could be public or restricted||
|Consensus process|Without permission||With|permission|
|Data immutability|Nearly impossible to tamper||Could|be tampered|
|Transactions throughput|Low|||High|
|Centralized|No|Partial||Yes|
|Network scalability|High||Low to medium||



programmed.” As shown in Figure 3, the typical workflow of machine learning involves training and testing. In the training phase, the original data is preprocessed first. )en, feature extraction and model training are carried out based on these data. In the test phase, data preprocessing and feature extraction are required for the test dataset, and then, the test data is analysed and categorised by the training model. 

Machine learning can usually be divided into supervised learning, semisupervised learning, and unsupervised learning. Supervised learning uses the labeled data to train the model, which is used to predict. _K_ -nearest neighbor, decision tree, neural network, and SVM are all supervised learning algorithms. Unsupervised learning uses training data set with no labels. )e key of unsupervised learning is to analyze the hidden structure of data and find out whether there is a divisible set. Semisupervised learning combines supervised learning with unsupervised learning, using a few labeled data and a large amount of unlabeled data for training and classification. 

_3.2. Federated Learning._ )e model training of machine learning needs a large number of sensitive data, and data privacy is a very important issue. At the same time, data is distributed in different organizations. )ese decentralized data are usually heterogeneous and unbalanced, so it is difficult to combine data. Google first proposed federated learning in 2016 [42], which combines machine learning with distributed computing. As shown in Figure 4, the data owners train their local data to get their local submodel. )en, they will upload the updated parameters to the coordinator, which aggregates the local submodel into the federated model. In federated learning, participants only need to share their own training model parameters and do not need to share the original data, which can protect the data privacy to a certain extent. 

**==> picture [236 x 309] intentionally omitted <==**

**----- Start of picture text -----**<br>
Data Sources<br>Data Processing<br>Feature Extraction<br>Training Set Testing Set<br>Algorithm Model<br>Selection Application<br>Analysis and<br>Model Training Predict<br>**----- End of picture text -----**<br>


Figure 3: A workflow of machine learning. 

## **4. Applications of Blockchain in AI** 

_4.1.DataSharing._ Data is the most important resource of AI. )e quantity and quality of data directly affect the accuracy of AI classification results. But, in the process of sharing data, there are some problems. First, the data needed for training is controlled by different stakeholders, and they 

Security and Communication Networks 

6 

**==> picture [236 x 265] intentionally omitted <==**

**----- Start of picture text -----**<br>
Federated model<br>Coordinator<br>...<br>Sub-model 1 Sub-model 2 Sub-model 2<br>...<br>Database 1 Database 2 Database 3<br>...<br>Data owner 1 Data owner 2 Data owner 3<br>**----- End of picture text -----**<br>


Figure 4: )e logic of federated learning. 

cannot trust each other. It is difficult to authorize or verify the data. Second, there may be malicious users sharing malicious data for certain purposes. )ere are already some blockchain-based solutions to these problems. Detailed comparison is shown in Table 4. 

)e work in [43] proposed a blockchain-based, decentralized, and untrusted data market. IoTs’ equipment providers and AI solution providers can carry out transparent interaction and cooperation through this platform and realize user registration, data upload, data search, purchase, payment, and feedback through smart contract. 

)e authors in [44] proposed a SecNet architecture, which realizes secure data sharing and ownership protection through blockchain. A data storage module and an access control module are included in each SecNet node. )e use of the private data center (PDC) [48] enables users to monitor any operation of their own data to achieve fine-grained management of data access behavior. Any data ready to be shared is registered in the blockchain, and the access of other parties to the data is also verified and recorded in the blockchain. )e economic incentive of sharing data or exchanging security services between different entities is realized through smart contracts. 

Singh et al. [46] designed an IoT architecture based on blockchain and AI, called BlockIoTIntelligence. As shown in Figure 5, there are four layers in this framework: device intelligence (DI), edge intelligence (EI), fog intelligence (FI), and cloud intelligence (CI). Each intelligent device in DI is a node in the blockchain. )rough the blockchain, data is transmitted between various IoT devices in a distributed manner. And, IoTdevices share their data with the EI. )e EI transfers the processed data and the underlying computing 

tasks to the FI in a distributed manner. In FI, AI technology is mainly used to train models and make decisions. A number of fog nodes that support AI are connected with the blockchain to share intermediate parameters or architecture information to CI. Data center, the core component of the CI, and support AI are linked with blockchain to provide the service of decentralized and secure big data analysis for IoTs’ applications. 

)e work in [47] proposed a cognitive manufacturing mining process based on blockchain. )is paper improves the traditional blockchain network and uses the distributed consensus blockchain network based on sidechain to improve the problem of limited storage space of traditional blockchain. )is method stores the data of smart devices in a separate database and then maps the data in the sidechain transactions of blocks. Based on the traceability and tamperproofing of blockchain, the data loss can be effectively avoided at any stage of the cognitive production process. 

Robots are used more and more widely nowadays, and they need to interact with each other to improve their abilities. )erefore, the security data sharing between each robot is also very important. In [45], the authors proposed a data model sharing framework called RoboChain, which is used for secure data sharing between robots located in different locations. Robots constantly learn to improve their interaction ability to monitor and improve human health. All operations are performed locally. Personal data will not leave the local hub. After the model is updated, the data will be deleted. At the same time, the local repository will publish a “change” to the hub. )e hub will announce the update to the whole network. Robots on other sites will know that there are available updates on the network through their own local hub. After each model update, they need to send a model update transaction to the blockchain. )e transaction contains the parameters of the transaction update and the information of the robots participating in the consensus so that other participants can verify how the model is created. 

_4.2.PrivacyPreserving._ Privacy preserving is also a key issue. )e protection of such personal sensitive data during the sharing process is difficult, which will prevent users from sharing their data. Additionally, the data should be completely controlled by the owner, but now users need to send their own data to the service provider when using the service, resulting in the abuse of personal data by some big companies. Detailed comparison of the solutions for privacy preserving is shown in Table 5. 

)e work in [49] proposed a distributed multilayer ledger named DeepLinQ, which may enable privacy-preserving data sharing. Taking medical data as an example, a multilayer blockchain architecture is proposed in this paper, which combines the advantages of blockchain (such as complete decentralization, consensus mechanism, and user anonymity) with the current status and actual needs of Electronic Health Records (EHRs), such as EHRs has been centralized; due to the requirement of efficiency, it is impossible to use PoW, and there is a need to introduce jury and verification committee. )e underlying blockchain 

Security and Communication Networks 

7 

Table 4: Data sharing. 

||||Table 4: Data sharing.|
|---|---|---|---|
|Ref.|Use case|Technologies|Contributions|
|[43]|IoT|Blockchain smart<br>contract|Propose a blockchain-based data marketplace, where providers of IoT devices and<br>providers of AI/ML solutions will cooperate and trade with each other|
|[44]|Medical data|Blockchain smart<br>contract PDC|Propose the SecNet, an architecture that can safely store, compute, and share data in<br>the large-scale internet environment|
|[45]|Robots|Blockchain transaction|Propose RoboChain, a learning framework that enables secure, decentralized, and<br>efcient data and model sharing among multiple robot units in diferent locations|
|[46]|IoT|Blockchain|Propose a blockchain-enabled intelligent IoT architecture that enables the efective<br>combination of blockchain and AI for IoT|
|[47]|Cognitive<br>manufacturing|Sidechain|Propose a subject mining method for cognitive production based on blockchain<br>networks|



**==> picture [52 x 39] intentionally omitted <==**

**==> picture [35 x 223] intentionally omitted <==**

**----- Start of picture text -----**<br>
Cloud<br>Intelligence<br>Fog<br>Intelligence<br>Edge<br>Intelligence<br>Device<br>Intelligence<br>**----- End of picture text -----**<br>


**==> picture [184 x 34] intentionally omitted <==**

**==> picture [185 x 39] intentionally omitted <==**

Figure 5: BlockIoTIntelligence architecture. 

maintains the current properties of Ethereum, and the branch layer meets these actual needs through the design of functions. Medical information is kept off-chain in the architecture proposed in this paper, and the blockchain holds pointers that can find stored information off-chain. )is paper presents four ways to design branch layers: (1) add trustworthy advanced validators; (2) use subgroup signatures; (3) create trusted branches; (4) employ efficient consensus protocols. )is proposed architecture reflects the management of patients on their own health records, eliminates users’ concerns about privacy disclosure, and promotes data sharing between different hospitals. 

In [50], the authors proposed to run machine learning algorithms on the blockchain, where different nodes in the blockchain each calculate a part of the machine learning algorithm and cooperate to complete the whole machine learning task. In the smart home environment, the data of IoTs devices is collected to predict users’ activities. For example, a device is automatically opened when a specific 

user enters a room. )e configuration file is generated for each user and device through association rule mining and calculation of personalized parameters. )e configuration file is stored in IPFS, and the hash of the configuration file is stored on the blockchain. At the same time, the blockchain calculates another hash value based on the transaction information and stores it on a smart hub. When the user enters the room, the smartphone logs into a smart hub. )e transaction information in the blockchain is queried by the hash value stored in the smart hub. )e user configuration file is obtained on the IPFS through the hash value stored in the transaction to determine the setting of a device. )e confidentiality and authenticity of users’ data are guaranteed through the blockchain and smart contract. 

Kuo et al. [51] proposed to spread part of the model and other meta information by the metadata in the transactions of private blockchain. Each node can initialize, update, evaluate, and forward the model. Both the model and the hash of the model are included in the update transaction. Other transactions contain simply the hash of model to decrease the blockchain size. )e private chain used in this paper does not provide mining reward, and the motivation for block mining comes from the use of cross-agency data in a privacy-protected way to improve the accuracy of predictive models. At the same time, this paper proposed a proof-of-information algorithm based on the PoW algorithm. When a site trains the model with local data and publishes it, other sites, respectively, evaluate the model with their own local data. )e site which has the highest error wins the “information bid” and then trains the model with its own local data. When a site updates the model and discovers itself to be in the greatest mistake, the online learning process ends. )e current model is the consensus model. )e proof-of-information algorithm can use all patients’ data to train the model, but it does not need to transmit personal protected health information, which effectively achieves the goal of privacy preserving. 

In addition to designing the blockchain architecture and consensus mechanisms, cryptography is also often used to achieve data privacy preserving in AI. )e work in [52] proposed a blockchain-enabled learning data management method. )e tamper-proofing of blockchain can help prevent the forgery and ensure the integrity of data. )e original data and the hash of the original data are stored in the blockchain according to the schemes proposed in this paper. 

Security and Communication Networks 

8 

Table 5: Privacy preserving. 

||||Table 5: Privacy preserving.|
|---|---|---|---|
|Ref.|Use case|Technologies|Contributions|
|[49]|Healthcare|Blockchain<br>Smart contract|Propose a multilayer blockchain architecture to support privacy-preserving distributed data<br>sharing|
|[50]|Smart home|Blockchain<br>Smart contract<br>IPFS|Propose a method to run machine learning models in a decentralized manner on the<br>blockchain; diferent nodes cooperate to complete a training task|
|[51]|Healthcare|Blockchain<br>Consensus|Propose ModelChain, where partial models are transmitted through transactions of the<br>blockchain and design a proof-of-information algorithm|
|[52]|AI algorithms|Blockchain<br>Hash function|Propose a blockchain-based solution to prevent data integrity from being compromised in<br>the learning environment|
|[53]|IoT|Blockchain<br>Paillier|Propose a blockchain-based data sharing platform and construct a secure privacy-preserving<br>SVM algorithm|
|[54]|Accelerate<br>biomedical<br>Healthcare|Blockchain<br>)reshold<br>encryption|Propose to apply blockchain combined with threshold encryption to healthcare to deal with<br>data privacy issues|



)e AI model verifies the data integrity by comparing the hash value of the received original data with the hash value stored in the blockchain. 

In [53], blockchain has been used to construct a trustworthy and secure data platform across numerous data sources, and IoT data is stored on the blockchain through Paillier encryption. IoTdata providers’ sensitive information and SVM model parameters are confidential. Data providers can update the gradient without knowing the model parameters through homomorphic encryption. In this way, collusion between data providers and data analysts can be avoided. Combined with the tamper-proofing of blockchain, a secure SVM training algorithm is established to address data integrity and privacy issues. 

In healthcare, both consumers and research companies require personal data to train their deep neural networks. Data security and privacy are two of the key difficulties they encounter. Mamoshina et al. [54] proposed to apply blockchain to healthcare to deal with data privacy issues. Users can store and sell their own biological data on the blockchain architecture presented in this article and only allow organizations that have paid for them to access the data. Data validators are used to verify the data to assure the quality of the data given by users. All interactions (user uploads the data, data validator validates the data, and customer buys the data) are carried out through transactions on the blockchain, so as to all data use activities are tracked fairly. )is study also offers a threshold encryption scheme to assure data security and user privacy. Users utilize symmetric encryption to encrypt data and then divide the key and distribute it to the blockchain nodes, which act as the key managers. )e customers obtain enough keys from the key managers to decrypt the data after they buy the data. In longterm data storage, the threshold encryption scheme can effectively address the single point of failure. )e leakage of memory and single key manager will not lead to data leakage, which effectively ensures data security. 

_4.3. Trusted AI Decision._ Different organizations create, train, and use models of machine learning and AI. )e entities that train the model are from the entities 

that provide the data. Failure to pay attention to data used in training the model might lead to improper outcomes. Meanwhile, the trained model might have some restrictions if we employ biased data. All of the above operations are opaque to users, and users cannot trust the model they are using. )erefore, we need a mechanism to record the entire process of AI (model creation, training data, and training process), and these records cannot be changed or forged. Blockchain, a platform that enables numerous participants to trustly share data, has the characteristics of tamperproofing and transparency, which are very suitable for recording the whole process of machine learning. Detailed comparison of the solutions for trusted AI decision is shown in Table 6. 

)e work in [55] proposed to manage swarm robots using blockchain to deal with the problem of Byzantine robots (the robots that show arbitrary errors or malicious behavior). In the scheme proposed in this paper, each robot may be utilized as a node or a miner on the blockchain. _Registerrobot_ , _applyStrategy_ , and _vote_ are implemented by smart contract. Robots register in the blockchain through _registeRobot_ , publish their own opinions through _Vote_ , and then obtain the views of other robots through _applyStrategy_ . _applystrategy_ achieves consensus of robot views through certain strategy functions. Finally, the experiment demonstrated that the use of blockchain can detect and reject Byzantine robots from the group. )us, the decision-making results with higher accuracy are obtained. 

In [56], a trusted routing scheme for wireless sensor networks based on blockchain and reinforcement learning was proposed. )e routing information (destination address, transmitted data, etc.) between various routing nodes is stored on the blockchain. )rough the tamper-proofing of blockchain, the credibility of routing information is improved. Firstly, different data packets are represented by different blockchain tokens; secondly, when the routing node joins the blockchain network, it needs to register through the smart contract; )irdly, before sending the data packets to the next hop route, each routing node has to confirm the routing information stored in the blockchain, 

Security and Communication Networks 

9 

Table 6: Trusted AI decision. 

||||Table 6: Trusted AI decision.|
|---|---|---|---|
|Ref.|Use case|Technologies|Contributions|
|[55]|Robots|Blockchain<br>Smart contract|Propose to manage swarm robots using blockchain to deal with the problem of Byzantine robots|
|[56]|WSNs|Blockchain<br>Smart contract|Propose a blockchain and reinforcement learning-based routing information management<br>platform to guarantee trusted and efficient routing decisions|
|[57]|Military<br>platforms|Blockchain<br>Smart contract|Propose a framework combining AI, machine learning, and private blockchain to provide<br>decision support for operational centers|
|[58]|Model<br>marketplace<br>Coalition<br>operations|Blockchain<br>Transaction|Propose to utilize blockchain to record important processes of AI in an invariable and verifiable<br>way|
|[59]|Machine learning|Blockchain<br>Smart contract|Propose a trusted machine learning analysis framework which utilizes smart contract to realize<br>automatic machine learning|
|||Blockchain||
|[60]|DNN<br>applications|Edge<br>computing|Propose a novel DNN architecture combined with edge computing and blockchain technology<br>to overcome the limitation of availability, scalability, and integrity|
|||Node.js||
|[61]|Neural networks|Blockchain<br>Edge<br>computing|Propose to use edge computing to reduce the pressure of the server and utilize blockchain to<br>cope with overtraining|



and the server node confirms the information through the consensus system and publishes it in the blockchain. )en, each routing node’s learning model collects information from blockchain. According to the information, a routing scheduling scheme based on reinforcement learning is proposed to help routing nodes make better routing decisions. Due to the tamper-proofing, traceability, and consensus mechanism of blockchain, the credibility of routing information is enhanced, and the credibility of routing decision is enhanced. 

In military operational centers, commanders usually need to make timely decisions based on information from multiple intelligence sources. )e work in [57] proposed a framework combining AI, as shown in Figure 6, machine learning and private blockchain to provide decision support for operational centers. )is paper proposed to synthesize multiple data sources through multiple AI agents to predict and evaluate the current decision. Blockchain plays two roles in this framework. First, different AI agents can verify whether they are operating in the same blockchain state as other agents, to guarantee that all agents are analyzing the same dataset. It can better compare and analyze the decision results. Secondly, the AI agents are rewarded by the blockchain record to motivate the AI agents to provide decision support. 

)e work in [58] took federated learning as an example and proposed to utilize blockchain to record important processes of AI in an invariable and verifiable way. Every process is recorded as transactions in the blockchain. 

Wang [59] proposed a trusted ML analysis framework based on blockchain. )e automated execution of ML algorithms is achieved through smart contracts, and the credibility of machine learning is realized by storing data permanently. Model initialization, training, validation, scoring, evaluation, and reward allocation are automatically executed by the preset smart contract, which greatly improves the credibility of the training results. 

**==> picture [69 x 8] intentionally omitted <==**

**----- Start of picture text -----**<br>
Multiple Data Source<br>**----- End of picture text -----**<br>


**==> picture [236 x 153] intentionally omitted <==**

**----- Start of picture text -----**<br>
t Specialized ana<br>Human/Machine<br>AI Arbiter<br>Team<br>Decision Support<br>eg.<br>**----- End of picture text -----**<br>


Figure 6: Blockchain for real-time decision support. 

In the process of AI training, a lot of computing power needs to be consumed. )erefore, in [60], the authors proposed to unload a large amount of computation of the deep neural network from the cloud server to the edge server and then utilized blockchain to accomplish two things: first, encourage the edge server to accept and complete the computation; second, ensure the credibility of edge computing results. Both the embedded device and the edge computing server need to pay a deposit to the blockchain, and then, the embedded device publishes the computing task. After the 

Security and Communication Networks 

10 

Table 7: Decentralized intelligence. 

||||Table 7: Decentralized intelligence.|
|---|---|---|---|
|Ref.|Use case|Technologies|Contributions|
|[62]|Distributed ML|Blockchain|Propose a blockchain-based decentralized security learning model LearningChain|
|[63]|Multiparty<br>learning|Blockchain|Propose BEMA, a novel blockchain-based secure multiparty learning system and design “of-<br>chain sample mining” and “on-chain mining” schemes|
|[64]|Deep learning<br>Federated<br>learning|Blockchain<br>Consensus|Propose a secure and decentralized privacy-preserving deep learning framework based on<br>blockchain, which encourages participants to engage in training|
|[65]|IoT|Blockchain<br>Smart contract<br>Consensus|Propose a knowledge market of IoT based on edge-AI, which uses knowledge blockchain for<br>knowledge management and trading|
|[66]|IoT|Blockchain<br>Transaction|Propose a collective _Q_-learning combined with blockchain and deep _Q_-learning, which is<br>used to allocate computing network resources to users|
|[67]|Federated<br>learning|Blockchain<br>Fabric channel|Propose FLchain to enhance the security of federated learning and implemented on fabric|
|[68]|Federated<br>learning|Blockchain<br>Incentive<br>mechanism|Propose to utilize consortium blockchain to manage workers’ reputation in a decentralized<br>manner|
|[69]|Collaborative AI|Blockchain<br>Smart contract|Propose a framework in which everyone may openly view the model’s forecast and input data<br>to enhance the model and utilize smart contracts to host a continuously updated model|
|[70]|Machine learning|Blockchain<br>Smart contract|Propose to use blockchain technology and smart contract to create an automatic anonymous<br>machine learning model trading market|



edge servers have completed its computing tasks, it gets the reward from the embedded device to motivate the edge servers to calculate. At the same time, the credibility of the edge computing results of all the above steps is guaranteed by the smart contract, thus ensuring the reliability of the deep neural network training model. In addition, this paper proposed to use Node.js instead of EVM to deal with smart contracts and perform some complex calculations. )e changed program status and results are returned to the blockchain for storage after Node.js execution. 

Winnicka and Ke˛sik [61] also proposed to use edge computing to reduce the pressure of the server when training very large amounts of data. In addition, they also mentioned another problem of current artificial neural network training. If the server receives enormous numbers of data in a short space of time and processes them, there may be a problem of overtraining. For this problem, this paper presented to apply blockchain idea to AI. )e scheme supports the re-training of classifiers in case of urgent need through setting the priority of every task. All tasks are sent through blockchain transactions, and each task has a priority. )e higher the priority, the greater the reward. When miners receive tasks with higher priority, they can choose to interrupt the current task in order to achieve urgent needs. )e interrupted task would be broadcasted again in the future, and their training would be executed again until the task is completed, i.e., the stop condition is reached. At the same time, the paper proposed that the more rewards the device gets, the more tasks it performs, which represents the consumption of the device to a certain extent, so the device is more likely to fail. 

_4.4. Decentralized Intelligence._ A vast quantity of IoT data has been created with the fast evolution of the IoTs. )rough the AI service, we can obtain the learning results and models 

from the massive IoT data. In order to perform complex model training tasks, collaboration with multiple devices is normally needed owing to the distribution of IoT devices and edge computing devices. )ere are two ways to collaborate here. First, different IoT devices or edge devices need to share data for complete data analysis and prediction (such as intelligent monitoring, monitor in different regions needs to share data). Second, different IoT devices or edge devices share their own learning models and then aggregate the models, that is, federated learning. Detailed comparison of the solutions for decentralized intelligence is shown in Table 7. 

Federated learning is a distributed machine learning technology with privacy preserving. A large number of nodes utilize their own local data to train their own local model in a distributed manner. Besides, they only need to share their models instead of sharing original data, which may prevent the leakage of sensitive data. 

)e work in [62] proposed a decentralized security learning model LearningChain. In the learning chain network, there are two kinds of participants, data holder and computing node. )e computing node helps the holder to train the learning model, which is paid by the data holder. Finally, different data holders work together to train a global model. )e whole training process is divided into three sections. )e first step is the initialization of blockchain and the establishment of the P2P network. )e second step is local gradient computation. )e third step is global gradient aggregation. )e winning node of the PoW adopts an aggregation algorithm, updates the model, and then creates a new block containing the local model and global model information and adds it to the blockchain. )e pseudoidentity is utilized in the scheme proposed in this paper to prevent the disclosure of the data owner’s identity information. )e data holders can constantly transform the pseudoidentity information in multiple generations to 

Security and Communication Networks 

11 

further protect the identity. When a computing node creates a new block, it checks the validity of the previous block. If the block is not valid, it checks the validity of the previous block until the correct block is found to resist the attack of the Byzantine node. 

In [63], the authors proposed a secure multiparty learning system called BEMA based on blockchain. )is paper mentioned two forms of Byzantine attacks: (1) malicious participants broadcast a malicious local model to other parties for changing the outcomes of categorization; (2) malicious participants send malicious calibration information to specific parties to mislead the update process of the local model. )rough the system proposed in this paper, the first type of Byzantine attack can be effectively prevented, and the second type of Byzantine attack can be controlled in an acceptable range. BEMA consists of system initialization, off-chain sample mining, and on-chain mining. In the initialization phase, each participant broadcasts its local model parameters and stores them in the blockchain. Offchain sample mining encourages every participant to test the learning model on the blockchain with their own local data. )en, they may find data samples that can calibrate these models and broadcast them to the blockchain. In the onchain mining phase, miners will update the corresponding model with received calibration data samples after 

Although federated learning does not need to share the original data, the privacy of training data cannot still be totally protected. Some research shows that the important information about original training data may be calculated through the intermediate gradient. We need to combine the knowledge of cryptography to overcome these difficulties. For example, homomorphic encryption is utilized to preserve data privacy in [71]. And, secret sharing and symmetric encryption are utilized to preserve data privacy in [72]. 

)e work in [64] proposed a secure and decentralized privacy-preserving deep learning framework. In this framework, _N_ participants initially reached a consensus on the initial parameters of the cooperation model. )e parameters were encrypted and recorded in the blockchain through the transaction _Tx_[0] _co_[signed][by][all][parties.][For][any] participant _Pj_ , the local data is trained to get the intermediate gradient _wi,j_ , which is encrypted and recorded in the blockchain through the transaction _TxPj_ . )en, the workers will download these transactions to obtain the intermediate gradient of the participants and calculate the new collaboration gradient _wi_ +1 at round _i_ + 1 through the smart contract. )e idea of homomorphic encryption is used here. It is not necessary to decrypt the previous cooperation gradient and the intermediate gradient of the participants when calculating the new cooperation gradient. We can get the encrypted weight at round _i_ + 1 by computing _C_ ( _Wi_ +1) �(1/ _N_ ) · _C_ ( _Wi_ ) · 􏽑 _[N] j_ �1 _[C] Pj_[(][Δ] _[W] i,j_[)][. )en,] _[ C]_[(] _[w] i_ +1[)] will be attached to _Tx[i] co_[+][1][.][)is][article][also][proposed][a] consensus protocol called blockwise-ba. )ree steps are included in the consensus protocol: (1) a leader is chosen randomly to generate a new block is by utilizing cryptographic sortition mentioned in [73], (2) the new block is verified and accepted through executing a Byzantine 

agreement protocol by a committee, which comprises of participants whose transactions are included in the new block, and (3) members of the committee inform their neighboring participants of the new blocks via gossip protocol. )rough these three steps, all participants reach a consensus. 

With the continuous growth of data generated by IoT, data processing and analysis need to be transferred to the edge computing devices to reduce the burden of the cloud. )e work in [65] proposed a knowledge market of IoT based on edge-AI, which uses knowledge blockchain ( _k_ -chain) for knowledge management and trading. )e knowledge market consists of edge-AI nodes and knowledge aggregators. AI algorithms are deployed in the edge-AI nodes who receive the IoT data, analyze the data, and obtain knowledge, and they can be either a buyer or a seller in this market. Edge-AI nodes need to upload the encrypted knowledge to the nearby KAGs. KAG is an enhanced base station (BS) with better resources, which aggregate the knowledge of edge-AI nodes in the corresponding region. 

_K_ -chain is separated into two subchains: knowledge management chain (KM-chain) and knowledge trading chain (KT-chain). KAGs collect the uploaded knowledge of its coverage, generate knowledge blocks, and store them in KM-chain. At the same time, knowledge management smart contract (KMSC) is deployed in KM-chain to realize automatic knowledge management. )e consensus algorithm used in KM-chain is called proof of capacity (PoC). )at is, KAG who has contributed the most storage capacity in the past period of time can be a leader and broadcast the new knowledge block. KT-chain is used to record knowledge trading. All the trading process is completed through knowledge trading smart contract (KTSC) to ensure trading efficiency and fairness. Besides, this paper proposed a new consensus algorithm Proof of Trading (PoT) which combines PoW and PoS. For a KAG, the total number of knowledge trading currency (KC) it owns is taken as its stake. )en, the difficulty of the hash puzzle to be solved in the consensus process will be adjusted dynamically according to the stakes owned by KAGs. )e more the stake KAG owns, the easier the hash puzzle it has to solve. PoT cannot only avoid the waste of resources compared with PoW but also resist some attacks compared with PoS. )e work in [66] also deployed AI at the edge of network. )e authors combined blockchain and deep _Q_ -learning and proposed a collective _Q_ -learning, which is used to allocate computing network resources to users. First, each edge node learns locally and trains DNNS. )en, the parameters of the local model and other required records are encapsulated into the transaction. )e learning results are shared through the blockchain. )is enables multiple nodes to work together to complete a complex task. 

FLchain is proposed in [67] to enhance the security of federated learning implemented on fabric. )e scheme proposed in this paper utilizes the concept of fabric channel. )e channel is used to achieve communication isolation between at least two peers. Entities outside the channel are unable to obtain the information in the channel, so as to achieve the privacy of transactions. Every 

Security and Communication Networks 

12 

global model is trained on a different channel in FLchain. )e specific steps are as follows: (1) the device inquires the available channel and obtains the channel list; (2) the device selects a channel and registers on channel; (3) the device downloads the current global model of the channel from the blockchain; (4) the device calculates the local model with local data; (5) the device sends the local model to the blockchain; (6) after a period of time, members on channel update the global model through the consensus algorithm and generate new blocks. 

Federated learning can effectively protect privacy because it does not need to share original data. However, there may be some malicious behavior, which will influence the model’s quality. )e choice of dependable workers is highly vital for task publishers. For this problem to be solved, the authors in [68] proposed to utilize consortium blockchain to manage workers’ reputation. )e architecture of the system in this paper suggested that the task publisher evaluates the model quality and generates reputation opinions after receiving the local model uploaded by all workers. )e reputation opinions are stored in the block and maintained by the consortium blockchain after being verified by the miners of consortium blockchain. Due to the decentralization and tamper-proofing of blockchain, reputation opinions stored in blocks cannot be changed. After that, task publishers can choose reliable workers through reputation blockchain to improve the quality of the training model. 

In addition to federated learning, there are some other schemes for decentralized intelligence. )e work in [69] proposed a framework in which everyone may openly view the model’s forecast and input data to enhance the model. )at is, the model is trained by many contributors in collaboration. Verification of the incentive mechanism, data upload, and model training are accomplished through the execution of smart contract. )is paper introduces in detail the incentive mechanism used to motivate participants to provide useful data. As shown in Figure 7, the model provider saves a deposit as a reward and defines a loss function _L_ ( _h, D_ ), where _h_ represents the model and _D_ represents the dataset. Each participant needs to deposit 1 unit of currency as a deposit. )e smart contract initially pays _L_ ( _h_ 0 _, D_ ) to the first person _t_ 1, and _t_ 1 updates the model to _h_ 1 with its own data. )en, _t_ 1 need to pay _L_ ( _h_ 1 _, D_ ) to the second person _t_ 2, until the final person pays _L_ ( _ht, D_ ) to the smart contract. So, the reward for the _t_ th person is his own deposit plus _L_ ( _ht_ −1 _, D_ ) − _L_ ( _ht, D_ ). )e better the model _ht_ trained by each participant _t_ is, the less the amount it needs to pay to the next participant. )is can motivate participants to provide useful data. 

Kurtulmus and Daniel [70] proposed to use blockchain technology and smart contract to create an automatic anonymous machine learning model trading market. )e entire procedure consists of four phases. )e first phase is initialization. Bob, the organizer who wants to obtain the machine learning model, creates an Ethereum smart contract and provides the hash value for the data to the smart contract to trigger the randomization function to generate the index of training and test data. )en, Bob sends the 

**==> picture [233 x 64] intentionally omitted <==**

**----- Start of picture text -----**<br>
L (h0,D) L (h1,D) L (hT-1,D)<br>...<br>smart user1 user2 userT<br>contract<br>L (hT,D)<br>**----- End of picture text -----**<br>


Figure 7: Collaborative AI. 

training data and nonce to the smart contract. )e data can be verified by the hash value provided previously; the second stage is the submission stage. Different scheme providers can obtain training data from smart contract and submit their own training model; the third stage is the evaluation stage. )e organizer Bob sends the test data to the smart contract. If Bob does not upload the test data within the specified time, the training data is used for testing. )e submitting party calls the evaluation function to submit the evaluation score. )e fourth stage is the finalize stage. )e organizer Bob pays rewards to the best model provider. If there is no best model, the deposit will be returned to Bob. )is scheme helps users to obtain machine learning models at a certain cost and automatically trains, evaluates, and trades models through smart contracts. Organizers can obtain different models trained by decentralized committers and select the best model, which greatly improves the efficiency and credibility of model trading. 

## **5. Conclusion and Future Research** 

We surveyed the existing literature to understand the potential applications of blockchain in AI. For example, we explained how the different characteristics of blockchain can be used in supporting data sharing, privacy preserving, trusted AI decision, and decentralized intelligence. 

- (i) First, as a decentralized platform, blockchain enables data owners and data users to share or trade data in a peer-to-peer manner [23]. Because blockchain is transparent and immutable, it can minimize the potential for fraud in distributed data sharing or transaction. 

- (ii) In addition, the underlying cryptographic algorithms (hash algorithms, homomorphic encryption, threshold encryption, etc.) used to process data stored on the blockchain help ensure the confidentiality, integrity, and authenticity of sensitive data. 

- (iiii) )e use of smart contracts to automate model creation, training, sharing, decision-making, and traceability on blockchain helps ensure the credibility of decision results. 

- (iv) Incentive mechanisms can be designed on blockchain to promote the cooperation of all participants in completing the AI training tasks. 

In addition, we also identified a number of existing and emerging challenges, which will hopefully guide future research agenda. 

Security and Communication Networks 

13 

_5.1. Identity Privacy._ Blockchain privacy preserving can be either identity privacy or transaction privacy. Identity privacy preserving guarantees that an attacker cannot match an address on the blockchain to a user’s actual identity, and transaction privacy preserving guarantees sensitive data from being stolen or tampered. Most existing schemes use blockchain and decision-making process to, respectively, record the training data and protect the privacy of sensitive training data. However, identity privacy preserving is generally ignored. 

Bitcoin and Ethereum provide anonymity by using pseudonames instead of real names for managing and verifying transactions. However, the user’s real identity can still be inferred by monitoring the user’s transactions [74,75]. Androulaki et al. [76], for example, demonstrated how one can use behavior-based clustering to analyze Bitcoin transactions and consequently match 40 percent of student identities to the associated blockchain addresses, even when users had adopted the privacy measures recommended by Bitcoin. Monroe uses a number of methods to achieve anonymized transactions, such as stealth-address and ring confidential transactions (RingCT) [77]. However, the number of transactions is limited due to the use of ring signatures. 

In addition, it can be challenging to achieve blockchain identity privacy preserving, when one takes into consideration blockchain/privacy regulation, deployment difficulty, and robustness of the corresponding architecture for privacy preserving schemes and the impact of privacy preserving schemes on performance. 

)is reinforces the importance of designing lightweight privacy-preserving schemes. 

_5.2. Performance._ )e scalability of blockchain can be considered from both data storage and transaction rate. In AI applications, significant storage space is needed to record the training data and the generated transactions. However, because of the restricted blockchain storage space, it is impractical to store the complete training data. Some existing schemes use sharding [78], sidechain (see [79] for an overview of sidechains), and some other ways to mitigate the storage limitations in blockchain. In addition, the throughput of most public blockchains is very limited. For example, Bitcoin can only handle 7 transactions per second, while Ethereum can handle 7–15 transactions per second. Such rate does not meet the needs of time-sensitive tasks, for example in a smart grid environment. 

One possible solution is to design a more efficient consensus mechanism, for example by designing blockchain-based AI applications that utilize private blockchains or consortium blockchains (which can effectively improve throughput) or by designing incentive mechanisms to motivate nodes in the network to participate in the consensus (which can improve the efficiency). 

_5.3. Security of Smart Contracts._ Most blockchain-based AI applications rely on smart contracts to automate the training process. )ere may be errors and loopholes in smart 

contracts [80–82]. For example, vulnerabilities in DAO smart contract built on the Ethereum platform were exploited in an attack in 2016, which resulted in the loss of 3.6 million ethers [83]. 

Hence, designing secure smart contracts is a topic of ongoing importance [80, 81, 84]. For example, can we also design AI-based approaches to identify and repair vulnerabilities in smart contracts? 

## **Data Availability** 

)e telemetry data used to support the findings of this study are included within the article. 

## **Conflicts of Interest** 

)e authors declare that they have no conflicts of interest. 

## **Acknowledgments** 

)e work was supported by the Shandong Provincial Key Research and Development Program (no. 2020CXGC010107), Blockchain Core Technology Strategic Research Program of Ministry of Education of China (no. 2020KJ010301), National Natural Science Foundation of China (nos. 61972294, 61932016, and 62172307), Special Project on Science and Technology Program of Hubei Provience (no. 2020AEA013), Natural Science Foundation of Hubei Province (no. 2020CFA052), Wuhan Municipal Science and Technology Project (no. 2020010601012187), and Foundation of Guangxi Key Laboratory of Trusted Software (no. kx202001). )e work of K. K. R. Choo was supported only by the Cloud Technology Endowed Professorship. 

## **References** 

- [1] A. Madani, R. Arnaout, M. Mofrad, and R. Arnaout, “Fast and accurate view classification of echocardiograms using deep learning,” _NPJ digital medicine_ , vol. 1, no. 1, pp. 1–8, 2018. 

- [2] A. Choudhury and O. Asan, “Role of artificial intelligence in patient safety outcomes: systematic literature review,” _JMIR medical informatics_ , vol. 8, no. 7, Article ID 18599, 2020. 

- [3] H. Zerouaoui and A. Idri, “Reviewing machine learning and image processing based decision-making systems for breast cancer imaging,” _Journal of Medical Systems_ , vol. 45, no. 1, pp. 1–20, 2021. 

- [4] S. Secinaro, D. Calandra, A. Secinaro, V. Muthurangu, and P. Biancone, “)e role of artificial intelligence in healthcare: a structured literature review,” _BMC Medical Informatics and Decision Making_ , vol. 21, no. 1, pp. 1–23, 2021. 

- [5] E. Gawehn, J. A. Hiss, and G. Schneider, “Deep learning in drug discovery,” _Molecular informatics_ , vol. 35, no. 1, pp. 3–14, 2016. 

- [6] T.-H. Chan, K. Jia, S. Gao, J. Lu, Z. Zeng, and Y. Ma, “PCANet: a simple deep learning baseline for image classification?,” _IEEE Transactions on Image Processing_ , vol. 24, no. 12, pp. 5017–5032, 2015. 

- [7] J. Frank, T. Eisenhofer, L. Sch¨onherr, A. Fischer, D. Kolossa, and T. Holz, “Leveraging frequency analysis for deep fake image recognition,” in _Proceedings of the 37th International_ 

Security and Communication Networks 

14 

_Conference on Machine Learning_ , ICML, Vienna, Austria, 1318 July 2020. 

- [8] C. M. Dourado, S. P. P. Da Silva, R. V. M. Da N´obrega, P. P. Rebouças Filho, K. Muhammad, and V. H. C. De Albuquerque, “An open ioht-based deep learning framework for online medical image recognition,” _IEEE Journal on Selected Areas in Communications_ , vol. 39, no. 2, pp. 541–548, 2020. 

- [9] O. J. H´enaff, “Data-efficient image recognition with contrastive predictive coding,” in _Proceedings of the 37th International Conference on Machine Learning_ , ICML, Vienna, Austria, 13-18 July 2020. 

- [10] M. Bojarski, D. Del Testa, D. Dworakowski et al., “End to End Learning for Self-Driving Cars,” 2016, https://arxiv.org/abs/ 1604.07316. 

- [11] Y. Xing, C. Lv, X. Mo, Z. Hu, C. Huang, and P. Hang, “Toward safe and smart mobility: energy-aware deep learning for driving behavior analysis and prediction of connected vehicles,” _IEEE Transactions on Intelligent Transportation Systems_ , vol. 22, no. 7, pp. 4267–4280, 2021. 

- [12] J. Bughin, M. Chui, R. Joshi, J. Manyika, and J. Seong, _Notes from the AI Frontier – Modeling the Impact of AI on the World Economy_ , McKinsey Global Institute, Brussels, Belgium, 2018. 

- [13] V. S. Verykios, E. Bertino, I. N. Fovino, L. P. Provenza, Y. Saygin, and Y. )eodoridis, “State-of-the-art in privacy preserving data mining,” _ACM Sigmod Record_ , vol. 33, no. 1, pp. 50–57, 2004. 

- [14] S. Gupta, W. Zhang, and F. Wang, “Model accuracy and runtime tradeoff in distributed deep learning: a systematic study,” in _Proceedings of the IEEE 16th International Conference on Data Mining (ICDM)_ , vol. 1, pp. 171–180, IEEE, Barcelona, Spain, December 2016. 

- [15] T. N. Dinh and M. T. )ai, “Ai and blockchain: a disruptive integration,” _Computer_ , vol. 51, no. 9, pp. 48–53, 2018. 

- [16] Y. Qi and J. Xiao, “Fintech,” _Communications of the ACM_ , vol. 61, no. 11, pp. 65–69, 2018. 

- [17] A. Athalye, L. Engstrom, A. Ilyas, and K. Kwok, “Synthesizing robust adversarial examples,” in _Proceedings of the International conference on machine learning_ , pp. 284–293, February 2018. 

- [18] B. Biggio, L. Didaci, G. Fumera, and F. Roli, “Poisoning attacks to compromise face templates,” in _Proceedings of the 2013 International Conference on Biometrics (ICB)_ , pp. 1–7, IEEE, Madrid, Spain, 4-7 June 2013. 

- [19] S. Lu, L.-M. Duan, and D.-L. Deng, “Quantum adversarial machine learning,” _Physical Review Research_ , vol. 2, no. 3, Article ID 033212, 2020. 

- [20] A. Qayyum, M. Usama, J. Qadir, and A. Al-Fuqaha, “Securing connected & autonomous vehicles: challenges posed by adversarial machine learning and the way forward,” _IEEE Communications Surveys & Tutorials_ , vol. 22, no. 2, pp. 998–1026, 2020. 

- [21] L. Demetrio, A. Valenza, G. Costa, and G. Lagorio, “Waf-amole: evading web application firewalls through adversarial machine learning,” in _Proceedings of the 35th Annual ACM Symposium on Applied Computing_ , pp. 1745–1752, Brno, Czech Republic, March 2020. 

- [22] E. Quiring, D. Klein, D. Arp, M. Johns, and K. Rieck, “Adversarial preprocessing: understanding and preventing image-scaling attacks in machine learning,” in _Proceedings of the 29th USENIX Security Symposium USENIX Security 20_ , pp. 1363–1380, Boston, MA, USA, August 2020. 

- [23] W. Dai, C. Dai, K.-K. R. Choo, C. Cui, D. Zou, and H. Jin, “SDTE: a secure blockchain-based data trading ecosystem,” 

_IEEE Transactions on Information Forensics and Security_ , vol. 15, pp. 725–737, 2020. 

- [24] S. Nakamoto and C. Wright, “Bitcoin: a peer-to-peer electronic cash system,” vol. 2008, 2008. 

- [25] I. Shaikh, “Policy uncertainty and bitcoin returns,” _Borsa Istanbul Review_ , vol. 20, no. 3, pp. 257–268, 2020. 

- [26] K. Salah, M. H. U. Rehman, N. Nizamuddin, and A. AlFuqaha, “Blockchain for AI: review and open research challenges,” _IEEE Access_ , vol. 7, pp. 10127–10149, 2019. 

- [27] Z. Zhang, X. Song, L. Liu, J. Yin, Y. Wang, and D. Lan, “Recent advances in blockchain and artificial intelligence integration: feasibility analysis, research issues, applications, challenges, and future work,” _Security and Communication Networks_ , vol. 2021, Article ID 9991535, 2021. 

- [28] E. Karger, “Combining blockchain and artificial intelligence - literature review and state of the art,” in _Proceedings of the 41st International Conference on Information Systems, ICIS 2020, Making Digital Inclusive: Blending the Locak and the Global_ , J. F. George, S. Paul, R. De’, E. Karahanna, S. Sarker, and G. Oestreicher-Singer, Eds., Association for Information Systems, Hyderabad, India, December 13-16, 2020. 

- [29] Y. Wu, Z. Wang, Y. Ma, and V. C. M. Leung, “Deep reinforcement learning for blockchain in industrial iot: a survey,” _Computer Networks_ , vol. 191, Article ID 108004, 2021. 

- [30] A. Kumari, R. Gupta, S. Tanwar, and N. Kumar, “Blockchain and AI amalgamation for energy cloud management: challenges, solutions, and future directions,” _Journal of Parallel and Distributed Computing_ , vol. 143, pp. 148–166, 2020. 

- [31] “SingularityNet,” 2021, https://singularitynet.io/. 

- [32] “TranceAi,” 2021, https://github.com/TraneAI. 

- [33] “Neureal,” 2021, https://neureal.net/. 

- [34] N. Szabo, “Smart contracts: building blocks for digital markets,” _EXTROPY: $e Journal of Transhumanist $ought_ , vol. 18, no. 2, 1996. 

- [35] B. K. Mohanta, S. S. Panda, and D. Jena, “An overview of smart contract and use cases in blockchain technology,” in _Proceedings of the 2018 9th International Conference on Computing, Communication and Networking Technologies (ICCCNT)_ , pp. 1–4, IEEE, Bengaluru, India, 10-12 July 2018. 

- [36] S. Wang, Y. Yuan, X. Wang, J. Li, R. Qin, and F.-Y. Wang, “An overview of smart contract: architecture, applications, and future trends,” in _Proceedings of the 2018 IEEE Intelligent Vehicles Symposium (IV)_ , pp. 108–113, IEEE, Changshu, China, June 2018. 

- [37] C. Zhang, C. Wu, and X. Wang, “Overview of blockchain consensus mechanism,” in _Proceedings of the 2020 2nd International Conference on Big Data Engineering_ , pp. 7–12, Shanghai, China, May 2020. 

- [38] L. Lamport, R. Shostak, and M. Pease, “)e Byzantine generals problem,” _ACM Transactions on Programming Languages and Systems_ , vol. 4, no. 3, pp. 382–401, 1982. 

- [39] M. Vukolc, “)e quest for scalable blockchain fabric: proofof-work vs,” _BFT replication_ , vol. 9591, pp. 112–125, 2016. 

- [40] I.-C. Lin and T.-C. Liao, “A survey of blockchain security issues and challenges,” _IJ Network Security_ , vol. 19, no. 5, pp. 653–659, 2017. 

- [41] A. L. Samuel, “Some studies in machine learning using the game of checkers,” _IBM Journal of Research and Development_ , vol. 3, no. 3, pp. 210–229, 1959. 

- [42] H. B. McMahan, E. Moore, D. Ramage, and B. A. y Arcas, “Federated Learning of Deep Networks Using Model Averaging,” 2016, https://arxiv.org/abs/1602.05629. 

- [43] K. R. Ozyilmaz, M. Do˘gan, and A. Yurdakul, “Idmob: Iot Data[¨] Marketplace on Blockchain,” in _Proceedings of the 2018 Crypto_ 

Security and Communication Networks 

15 

_valley Conference on Blockchain Technology (CVCBT)_ , pp. 11–19, IEEE, Zug, Switzerland, June 2018. 

- [44] K. Wang, J. Dong, Y. Wang, and H. Yin, “Securing data with blockchain and ai,” _IEEE Access_ , vol. 7, pp. 77981–77989, 2019. 

- [45] E. C. Ferrer, O. Rudovic, T. Hardjono, and A. Pentland, “Robochain: a Secure Data-Sharing Framework for HumanRobot Interaction,” 2018, https://arxiv.org/abs/1802.04480. 

- [46] S. K. Singh, S. Rathore, and J. H. Park, “BlockIoTIntelligence: a blockchain-enabled intelligent IoT architecture with artificial intelligence,” _Future Generation Computer Systems_ , vol. 110, pp. 721–743, 2020. 

- [47] K. Chung, H. Yoo, D. Choe, and H. Jung, “Blockchain network based topic mining process for cognitive manufacturing,” _Wireless Personal Communications_ , vol. 105, no. 2, pp. 583–597, 2019. 

- [48] H. Yin, D. Guo, K. Wang, Z. Jiang, Y. Lyu, and J. Xing, “Hyperconnected network: a decentralized trusted computing and networking paradigm,” _IEEE Network_ , vol. 32, no. 1, pp. 112–117, 2018. 

- [49] E. Y. Chang, S.-W. Liao, C.-T. Liu et al., “Deeplinq: distributed multi-layer ledgers for privacy-preserving data sharing,” in _Proceedings of the IEEE International Conference on Artificial Intelligence and Virtual Reality (AIVR)_ , pp. 173–178, IEEE, Taichung, Taiwan, December 2018. 

- [50] K. Singla, J. Bose, and S. Katariya, “Machine learning for secure device personalization using blockchain,” in _Proceedings of the International Conference on Advances in Computing, Communications and Informatics (ICACCI)_ , pp. 67–73, IEEE, Taichung, Taiwan, 10-12 Dec.2018. 

- [51] T.-T. Kuo and L. Ohno-Machado, “Modelchain: decentralized Privacy-Preserving Healthcare Predictive Modeling Framework on Private Blockchain Networks,” 2018, https://arxiv. org/abs/1802.01746. 

- [52] J. Kim and N. Park, “Blockchain-based data-preserving ai learning environment model for ai cybersecurity systems in iot service environments,” _Applied Sciences_ , vol. 10, no. 14, p. 4718, 2020. 

- [53] M. Shen, X. Tang, L. Zhu, X. Du, and M. Guizani, “Privacypreserving support vector machine training over blockchainbased encrypted iot data in smart cities,” _IEEE Internet of $ings Journal_ , vol. 6, no. 5, pp. 7702–7712, 2019. 

- [54] P. Mamoshina, L. Ojomoko, Y. Yanovich et al., “Converging blockchain and next-generation artificial intelligence technologies to decentralize and accelerate biomedical research and healthcare,” _Oncotarget_ , vol. 9, no. 5, pp. 5665–5690, 2018. 

- [55] V. Strobel, E. C. Ferrer, and M. Dorigo, “Managing byzantine robots via blockchain technology in a swarm robotics collective decision making scenario,” in _Proceedings of the International Foundation for Autonomous Agents and Multiagent Systems_ , pp. 541–549, Richland, SC, USA, July 2018. 

- [56] J. Yang, S. He, Y. Xu, L. Chen, and J. Ren, “A trusted routing scheme using blockchain and reinforcement learning for wireless sensor networks,” _Sensors_ , vol. 19, no. 4, p. 970, 2019. 

- [57] M. Blowers, S. Scrafford, and J. Williams, “Blockchain technologies and distributed ledger systems as enablers for real time decision support,” in _Proceedings of the Disruptive Technologies in Information Sciences II_ , vol. 11013, Article ID 110130L, May 2019. 

- [58] K. Sarpatwar, R. Vaculin, H. Min et al., “Towards Enabling Trusted Artificial Intelligence via Blockchain,” in _Policy-Based Autonomic Data Governance_ , pp. 137–153, Springer, Berlin, Germany, 2019. 

- [59] T. Wang, “A Unified Analytical Framework for Trustable Machine Learning and Automation Running with blockchain,” in _Proceedings of the IEEE International Conference on Big Data (Big Data)_ , pp. 4974–4983, IEEE, Seattle, WA, USA, 10-13 Dec.2018. 

- [60] J.-Y. Kim and S.-M. Moon, “Blockchain-based edge computing for deep neural network applications,” in _Proceedings of the Workshop on INTelligent Embedded Systems Architectures and Applications_ , pp. 53–55, New York, NY, USA, October 2018. 

- [61] A. Winnicka and K. Ke˛sik, “Idea of using blockchain technique for choosingthe best configuration of weights in neural networks,” _Algorithms_ , vol. 12, no. 8, p. 163, 2019. 

- [62] X. Chen, J. Ji, C. Luo, W. Liao, and P. Li, “When machine learning meets blockchain: a decentralized, privacy-preserving and secure design,” in _Proceedings of the IEEE International Conference on Big Data (Big Data)_ , pp. 1178–1187, IEEE, Seattle, WA, USA, 10-13 Dec.2018. 

- [63] Q. Wang, Y. Guo, X. Wang, T. Ji, L. Yu, and P. Li, “Ai at the edge: blockchain-empowered secure multiparty learning with heterogeneous models,” _IEEE Internet of $ings Journal_ , vol. 7, no. 10, pp. 9600–9610, 2020. 

- [64] J. Weng, J. Weng, J. Zhang, M. Li, Y. Zhang, and W. Luo, “Deepchain: auditable and privacy-preserving deep learning with blockchain-based incentive,” _IEEE Transactions on Dependable and Secure Computing_ , vol. 2018, p. 679, 2019. 

- [65] X. Lin, J. Li, J. Wu, H. Liang, and W. Yang, “Making knowledge tradable in edge-ai enabled iot: a consortium blockchain-based efficient and incentive approach,” _IEEE Transactions on Industrial Informatics_ , vol. 15, no. 12, pp. 6367–6378, 2019. 

- [66] C. Qiu, X. Wang, H. Yao, J. Du, F. R. Yu, and S. Guo, “Networking integrated cloud-edge-end in iot: a blockchainassisted collective q-learning approach,” _IEEE Internet of $ings Journal_ , vol. 8, no. 16, pp. 12694–12704, 2020. 

- [67] U. Majeed and C. S. Hong, “Flchain: federated learning via mec-enabled blockchain network,” in _Proceedings of the 2019 20th asia-pacific network Operations and management symposium (APNOMS)_ , pp. 1–4, IEEE, Matsue, Japan, 18-20 Sept.2019. 

- [68] J. Kang, Z. Xiong, D. Niyato, S. Xie, and J. Zhang, “Incentive mechanism for reliable federated learning: a joint optimization approach to combining reputation and contract theory,” _IEEE Internet of $ings Journal_ , vol. 6, no. 6, pp. 10700–10714, 2019. 

- [69] J. D. Harris and B. Waggoner, “Decentralized and collaborative ai on blockchain,” in _Proceedings of the IEEE International Conference on Blockchain (Blockchain)_ , pp. 368–375, IEEE, Atlanta, GA, USA, 14-17 July 2019. 

- [70] A. B. Kurtulmus and K. Daniel, “Trustless Machine Learning Contracts; Evaluating and Exchanging Machine Learning Models on the Ethereum Blockchain,” 2018, https://arxiv.org/ abs/1802.10185. 

- [71] Y. Aono, T. Hayashi, L. Wang, S. Moriai, and L. T. Phong, “Privacy-preserving deep learning via additively homomorphic encryption,” _IEEE Transactions on Information Forensics and Security_ , vol. 13, no. 5, pp. 1333–1345, 2017. 

- [72] K. Bonawitz, V. Ivanov, B. Kreuter et al., “Practical secure aggregation for privacy-preserving machine learning,” in _Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security_ , pp. 1175–1191, Dallas, Texas, USA, October 2017. 

- [73] J. Chen and S. Micali, “Algorand: a secure and efficient distributed ledger,” _$eoretical Computer Science_ , vol. 777, pp. 155–183, 2019. 

Security and Communication Networks 

16 

- [74] D. K. Soni, H. Sharma, B. Bhushan, N. Sharma, and I. Kaushik, “Security issues & seclusion in bitcoin system,” in _Proceedings of the 2020 IEEE 9th International Conference On Communication Systems And Network Technologies (CSNT)_ , pp. 223–229, IEEE, Gwalior, India. 

- [75] M. Bhargavi, S. M. Katti, M. Shilpa, V. P. Kulkarni, and S. Prasad, “Transactional data analytics for inferring behavioural traits in ethereum blockchain network,” in _Proceedings of the IEEE 16th International Conference on Intelligent Computer Communication and Processing (ICCP)_ , pp. 485– 490, IEEE, Cluj-Napoca, Romania, 3-5 Sept. 2020. 

- [76] E. Androulaki, G. O. Karame, M. Roeschlin, T. Scherer, and S. Capkun, “Evaluating user privacy in bitcoin, Financial Cryptography and Data Security,” in _Proceedings of the International Conference On Financial Cryptography And Data Security_ , pp. 34–51, Springer, Okinawa, Japan, April 1-5,2013. 

- [77] A. Averin, A. Samartsev, and N. Sachenko, “Review of Methods for Ensuring Anonymity and De-anonymization in Blockchain,” in _Proceedings of the 2020 International Conference Quality Management, Transport And Information Security, Information Technologies (IT&QM&IS)_ , pp. 82–87, IEEE, Yaroslavl, Russia, 7-11 Sept.2020. 

- [78] H. Dang, T. T. A. Dinh, D. Loghin, E.-C. Chang, Q. Lin, and B. C. Ooi, “Towards scaling blockchain systems via sharding,” in _Proceedings of the 2019 International Conference on Management of Data_ , pp. 123–140, Amsterdam, Netherlands, June 2019. 

- [79] A. Singh, K. Click, R. M. Parizi, Q. Zhang, A. Dehghantanha, and K. R. Choo, “Sidechain technologies in blockchain networks: an examination and state-of-the-art review,” _Journal of Network and Computer Applications_ , vol. 149, 2020. 

- [80] Y. Zhuang, Z. Liu, P. Qian, Q. Liu, X. Wang, and Q. He, “Smart contract vulnerability detection using graph neural network,” in _Proceedings of the Twenty-Ninth International Joint Conference on Artificial Intelligence, IJCAI 2020_ , C. Bessiere, Ed., pp. 3283–3290, Yokohama, Japan, July 2020. 

- [81] X. L. Yu, O. Al-Bataineh, D. Lo, and A. Roychoudhury, “Smart contract repair,” _ACM Transactions on Software Engineering and Methodology_ , vol. 29, no. 4, pp. 1–32, 2020. 

- [82] D. He, Z. Deng, Y. Zhang, S. Chan, Y. Cheng, and N. Guizani, “Smart contract vulnerability analysis and security audit,” _IEEE Network_ , vol. 34, no. 5, pp. 276–282, 2020. 

- [83] D. Siegel, “Understanding the dao attack,” vol. 13, Article ID 2018, 2016. 

- [84] E. Zhou, S. Hua, B. Pi et al., “Security Assurance for Smart Contract,” in _Proceedings of the 2018 9th IFIP International Conference On New Technologies, Mobility And Security (NTMS)_ , pp. 1–5, IEEE, Paris, France, 2018. 

