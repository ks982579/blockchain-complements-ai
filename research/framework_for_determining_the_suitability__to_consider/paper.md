~~a~~ Received: 18 December 2020 Revised: 4 June 2021 Accepted: 3 July 2021 DOI: 10.1002/ett.4334 

**S U R V E Y PA P E R** 

## **Framework for determining the suitability of blockchain: Criteria and issues to consider** 

## **Vikas Hassija[1]** | **Sherali Zeadally[2]** | **Ishan Jain[1] Aman Tahiliani[1] Vinay Chamola[3]** | **Shashank Gupta[4]** 

1Department of CSE and IT, Jaypee Institute of Information Technology, Noida, India 

2College of Communication and Information, University of Kentucky, Lexington, Kentucky, 

3Department of Electrical and Electronics Engineering and APPCAIR, BITS-Pilani, Pilani Campus, Pilani, India 

4Department of Computer Science and Information Systems, BITS-Pilani, Pilani Campus, Pilani, India 

## **Correspondence** 

Vinay Chamola, Department of Electrical and Electronics Engineering and APPCAIR, BITS-Pilani, Pilani Campus, Pilani, India. Email:vinay.chamola@ pilani.bits-pilani.ac.in 

## **Abstract** 

Various distributed ledger technologies (DLTs), such as blockchain, have evolved significantly in recent years. These technologies provide a robust and effective solution for providing confidentiality, integrity, nonrepudiation, authentication, and transparency. While blockchain has many advantages, it has various limitations as well, such as complexity, low throughput, privacy, and so on. We discuss the issues that must be considered when deciding whether ~~to use these technologies in a given case or not. We describe the operation of~~ blockchain, application areas where blockchain is suitable, and those where it is not. We also discuss the applicability of other emerging DLTs, apart from blockchain technology, such as Hashgraph, Zcash, Nano coin, and IOTA. 

## **1** | **INTRODUCTION** 

As the name suggests, a blockchain can be considered as a database where digital information (ie, “Blocks”) is stored in a distributed network as a chain of blocks. Blockchain falls in the class of distributed ledger technologies (DLT) that allow a database to be used and maintained in a distributed manner. Blockchain provides a shared ledger of transactions[1] that can be read, verified and stored in the form of blocks, forming a chain-like structure. Depending on the size of the transaction, a block may store up to a few thousand transactions (eg, a single block in the bitcoin blockchain can store about 1 MB of data). The security and anonymity provided by blockchain relies on the use of public key based digital signatures, hash functions, and the distributed nature of the database. Participants in a blockchain are connected through a peer-to-peer (P2P) network and are independent of each other.[2] This architecture allows the sharing of resources,[3] avoids single point of failure, and reduces the likelihood of data tampering because the data is not stored or managed by specific nodes.[4-6] 

However, there are many challenges[7] that need to be resolved in the case of distributed networks. These challenges include accountability, confidentiality, integrity, nonrepudiation, and authentication, due to the absence of any central authority. Another major issue is trust because there is no central authority that can resolve any ambiguities or disagreements that may occur. Blockchain resolves all these issues. Confidentiality and authentication are ensured using two-way encryption, that is, by encrypting the data first by the sender’s private key and then by the receiver’s public key. To decrypt it, first the receiver’s private key is used and then the sender’s public key. Consequently, data remains confidential, integrity is achieved and because the data can be obtained using sender’s public key, it acts as a digital signature that authenticates the data and eliminates the possibility of nonrepudiation. Trust is obtained by introducing a consensus 

© 2021 John Wiley & Sons, Ltd. **1 of 23** 

wileyonlinelibrary.com/journal/ett 

_Trans Emerging Tel Tech_ . 2021;32:e4334. https://doi.org/10.1002/ett.4334 

HASSIJA et al. 

**2 of 23** 

algorithms that ensures that only valid data or transactions are recorded in the blockchain. Blockchain also prevents double spending.[8,9] 

As a result of these attractive features in terms of security, scalability,[10] blockchain technology has been used in various fields such as Industrial Internet of Things[11] smart health systems,[12,13] smart voting,[14] cybersecurity industries,[15] financial services,[16] and others.[17,18] However, due to the hype surrounding blockchain technology, it is often suggested as a solution for almost any application these days.[19] However, there are many cases where the option of decentralization and the core technology of blockchain result in unreasonable operating cost and poor performance.[20] 

These factors make it crucial to assess whether blockchain technology is helpful in an emerging area or not. We propose a set of guidelines to answer the very fundamental questions such as: is blockchain suitable for all the emerging areas? How can one assess if blockchain is a suitable technology to use or not? What kinds of “tests” can we use to determine if blockchain will be good to use for a specific application domain? By using the proposed guidelines, one should be able to determine if blockchain is worth considering in the newly emerging area, the type of blockchain to use, and various alternatives to the blockchain. 

## **1.1 Research contributions of this work** 

This article addresses the problem of objectively assessing the applicability of blockchain to any specific application. While existing works have presented various fields and sectors where blockchain may or may not be suitable, there are no proper guidelines to evaluate the effectiveness of blockchain and to determine the most appropriate blockchain technology for a given application. Due to the lack of work done in this direction, a need to discuss the alternatives to traditional blockchain arises. To address these issues, we present: 

1. An overview of the fundamentals of blockchain along with its advantages and disadvantages. 

2. A framework to evaluate the applicability of blockchain for a given application domain. 

3. Alternatives to traditional blockchain and when to use them. 

4. Demonstration of our proposed framework using three case studies. 

The remainder of this article is organized as follows. Section 2 presents an overview of related surveys which also discuss when and where to use blockchain. Section 3 provides an overview of blockchain technology. Section 4 presents various application domains where blockchain is being used. Section 5presents our proposed framework to determine if blockchain is a suitable technology to use or not for a particular application. Section 6 presents the alternatives to blockchain technology. Section 7presents some use-case applications that demonstrates the use of our proposed framework. Section 8 presents challenges and research directions. Finally, Section 9 presents our concluding remarks. 

## **2 LITERATURE SURVEY** 

Applications of blockchain have been widely explored in literature. E-commerce, logistic network,[21-23] private data handling,[24-26] smart communities,[27-32] edge computing,[33-36] machine learning,[37-39] and deep learning[40-44] are some of the application domains and the related works in these areas. While cryptocurrencies are the most common and popular application of blockchain, they have also been explored for use in various industrial sectors. The increased visibility of blockchain technology has led to its uses in almost every application domain. However, the suitability of blockchain for an application depends on its specific requirements. There must be some framework to help decide on the suitability of blockchain for its deployment in a specific application domain. 

In References 45,46 the authors discussed a scenario where a blockchain can be used to assist a business. The authors of Reference 47 did a brief comparison between various proposed decision models for evaluating blockchain’s suitability. In References 48-50, the authors gave some criteria to decide whether blockchain is a good option or not. The article Reference 48 focused mainly on the use cases related to the insurance sector. The advantages and disadvantages of using blockchain in different insurance related use cases are discussed. However, the use of blockchain alternatives in some other use cases where decentralization is required, but other features of blockchain are not needed, is not discussed. The authors of Reference 51 present some problems that can be solved with similar efficiency as blockchain at a low cost. The authors focused on helping architects, developers, investors, and project leaders evaluate the suitability of blockchain 

HASSIJA et al. 

**3 of 23** 

for a particular application. The solutions or the alternatives for the issues raised are not discussed in detail. Similarly, the authors of Reference 52 presented a detailed review of blockchain technology and they also discussed some of the criteria one could use to choose the type of blockchain for a particular application. However, they did not discuss the applications that need a DLT but cannot use a blockchain. For example, there is no facility in blockchain to maintain timestamp ordering. Thus, applications that can benefit significantly from distributed ledgers cannot use blockchain if they need to maintain the timestamp ordering of the transactions. But there are few other nonblockchain DLTs such as Hashgraph that can be used in such cases. In this article, we discussed in detail, the blockchain alternatives that can help in addressing the disadvantages of blockchain while using its inherent features. 

This article addresses the limitations of existing literature by proposing a framework with a comprehensive set of guidelines to not only decide if blockchain is a suitable solution for an application area but also to determine the most appropriate type of blockchain and suggest alternatives of blockchain that may provide better results, according to the application’s requirements. 

## **3 BLOCKCHAIN FUNDAMENTALS** 

Figure 1 (adapted from the one proposed in Reference 53) shows the operation of a blockchain network can be visualized as a multilayer framework. We discuss this multilayered architecture below. 

1. _Data layer:_ The lowest layer in a blockchain architecture is the data layer. This layer contains blocks to store data. Each block consists of a body and a header. The header of the current block stores the hash of the previous block and forms a chain-like structure. The genesis block (the first block) does not contain the header as it is the first in the chain. Each block has a timestamp that is the time of its creation, a nonce which is a random number calculated by the miners to meet the difficulty level and to get the block-hash and Merkle root which is the root of the Merkle tree (a binary tree of hashes which stores transactions). 

2. _Network layer:_ The main purpose of the network layer is to distribute and authenticate transactions, and facilitate internode communication. It is also known as propagation layer. Using a P2P network, this layer ensures that nodes 

**F I G U R E 1** Blockchain as a multilayer framework 

HASSIJA et al. 

**4 of 23** 

can discover and communicate with each other. Section 3.1 discusses the flow and the life cycle of a transaction in terms of initiation, validation, and so on. 

3. _Consensus layer:_ This layer is central to the existence of blockchain platforms and is the most important layer for a blockchain. In a P2P network without any central authority, it is a challenging task to create consensus between every node in the network, and this is achieved in this layer. Section 3.3 discusses various types of consensus algorithms. 

4. _Incentive layer:_ This layer acts as a driving force in maintaining a public blockchain. It addresses the economic factor and creates economically beneficial schemes for the miners. In return for the computational power spent in the mining process, miners are rewarded with the incentives (eg, some amount of digital currency).[54] 

5. _Contract layer:_ The contract layer includes different scripts, smart contracts, and algorithms to execute complex transactions securely. Smart contracts are self-executable codes with a predefined set of rules, which when met, trigger the transaction between the parties involved in the contract. These self-running codes ensure that each transaction fulfills the predefined requirements of the model. Smart contracts are fully automated, thus reducing the possibility of any fraud or theft. These contracts ensure secure transfer of digital assets involved in the transaction.[55] 

6. _Application layer:_ This layer comprises all the applications that are used by end users. The application layer enables users to interact with the blockchain network. It includes APIs, scripts and user interfaces. This layer provides data to the contract layer to connect users with the back-end system. 

Various blockchain applications, such as cryptocurrency wallets, are found at the application level. At the contract layer or execution layer, we have the smart contracts environment that determines the nature of the transactions. The security of the network depends on the participation of nodes. The incentive layer provides incentives to motivate nodes to participate in the verification process of the blockchain. Usually, blockchain offers some amount of virtual currency as a reward to the participants. For example, bitcoin provides a few bitcoins as a reward. At the consensus layer, we have a consensus mechanism which ensures agreement among the participating nodes in a network. The network layer strategies for secure propagation of transactions. The data layer, provides a data structure to store data and perform various operations on data storage. 

## **3.1 Life cycle of a transaction** 

Transactions in blockchain are not restricted to just finance and include several other types of information, and the process of recording a transaction involves many steps.[56,57] These include instructions such as querying, sharing, and storing. Once a transaction has been validated, its respective block is mined and linked to the previous block. Systems and users present in the network update their existing copy of the blockchain. 

A transaction is created every time a user tries to interact with another user in the network. Depending on the blockchain system, steps in this process of a transaction may differ. Generally, it starts with the creation of the transaction and ends when it gets recorded in the blockchain (as shown in Figure 2). Hence a transaction passes through three phases: 

1. _Start:_ Every blockchain has its predefined data structure. Each transaction is defined, according to the data model of that blockchain, the sender, and the receiver of the digital asset. The transaction must fulfill the conditions of the smart contract or simple scripts (depending on the model). 

2. _Validation:_ In this step, transactions are verified by validating peers. These transactions are then inserted into a block, which is ready to be mined. 

3. _Mining:_ In this step, miners solve a mathematical problem, using their computational power, to find a random value such that the hash of the new block has certain predefined complexity.[58,59] This block is now added to the main blockchain. 

4. _Termination:_ The updated blockchain then propagates in the network to inform each node to update its own copy. Once the block is added to the blockchain, its assets are transferred accordingly. 

## **3.2 Types of blockchain** 

1. _Public:_ In this type of blockchain (also known as _permission-less blockchain_ ) anyone can join and perform transactions, that is, anyone with an Internet connection can access a permission-less blockchain. For example, bitcoin is one of the first public blockchains.[60] 

HASSIJA et al. **5 of 23** ~~=csSS_,!}!,{[YJH,_WWJY_____1!_—#!1_—#!_#1_—1#1_—_ Ww] LEY——~~ 

**F I G U R E 2** Propagation of a transaction using blockchain 

Verification of transactions and creation of blocks are both done by the participating nodes. This implies that public blockchain becomes nonfunctional if it does not have the required nodes to participate in the validation process. 

2. _Private:_ Private blockchain (also known as permissioned blockchain) works in a restricted environment (closed network). This type of blockchain is effective for an organization that wants it for internal use-cases. In a private blockchain, only selected participants who have permission can access the blockchain.[61] Moreover, the network is maintained by a central authority, and hence, it not decentralized in nature. 

3. _Consortium:_ Consortium blockchain or _federated blockchain_ is an effective solution to provide an environment with both public and private blockchain features, that is, in this type of blockchain, some attributes of the organization are made public and remaining are made private.[62] Although consortium blockchain is not open publicly, it stills manages to be decentralized in nature because it is managed by more than one organization. A validator node can initiate or receive transactions and can also perform validation of the transactions. On the other hand, member nodes can only receive or initiate transactions.[63] 

4. _Hybrid:_ A hybrid blockchain is a combination of private and public blockchain. It combines the advantages of both private and public blockchains while limiting their disadvantages. In a hybrid blockchain, a ledger can be made accessible to everyone (publicly) by employing a public blockchain with a private blockchain that can control access to the modifications in the ledger. Ripple network is an example of a hybrid blockchain. 

Each blockchain type has its advantages and disadvantages. It is vital to understand the needs of an organization or application and then choose the type of blockchain accordingly. Table 1 presents a comparison between the types of blockchain. 

## **3.3** | **Trusting the untrusted** 

A secure transaction can be achieved with the help of smart contracts. However, the decentralized system architectures(as in blockchain) hamper the building of trust and tend to raise questions about the probability of date modification. To maintain the integrity of the data, it has to be immutable. To achieve this goal, blockchain uses _consensus algorithms_ that allow a block to be added to the blockchain only when it is agreed by all the nodes present in the network. These algorithms ensure that the data in the database (ie, in the blockchain) cannot be altered or modified after it is stored in the blockchain.[64,65] 

## 3.3.1 | Consensus algorithms 

Initially algorithms such as: 2 phase commit (2PC),[66] atomic broadcast, state machine replication,[67] Byzantine fault tolerant (BFT)[68] were proposed to achieve a consensus in distributed databases. These algorithms have low failure-resilience. 

HASSIJA et al. 

**6 of 23** 

**TA B L E 1** Comparison between different types of blockchain 

||**Type of blockchain**<br>**Advantages**<br>**Disadvantages**<br>**Applications**<br>**Some domains using**<br>**this type**<br>Public blockchain<br>Open to everyone High<br>transparency High trust<br>among users<br>Low transaction speed<br>Low scalability High<br>computational power<br>Bitcoin Ethereum Litecoin<br>NEO<br>Transparency in<br>fundraising voting<br>Private blockchain<br>Low transaction time<br>Scalable<br>Centralized low trust<br>Multichain hyperledger<br>fabric<br>Internal votingSupply<br>chain<br>Consortium<br>blockchain<br>High control over<br>resources High<br>scalability Secure<br>Less transparent Less<br>anonymity<br>Marco Polo Energy Web<br>Foundation IBM Food<br>Trust<br>Food tracking Banking<br>and payments<br>Hybrid blockchain<br>Closed ecosystem<br>~~Immune to 51% attacks~~<br>good scalability<br>Less transparent Complex<br>~~structure~~<br>Dragonchain XinFin’s<br>~~hybrid blockchain~~<br>Real estate financial<br>~~markets~~|
|---|---|



For example, in the case of 2PC, the consensus procedure gets compromised with the failure of any node. These algorithms were the precursors of consensus solutions for DLTs. Blockchains such as bitcoin and ethereum achieve consensus and maintain a coherent view among participants. Such networks are failure-resilient as long as malicious nodes remain a minority. To achieve this by charging nodes who differ from the default behavior, computational cost is introduced as a _Proof of work (PoW)_ that is needed to add a block in the blockchain. Bitcoin uses PoW to prevent it from a _Sybil attack_ .[69] PoW has low scalability and high latency and requires a significant amount of computational power. There are several alternatives for POW to avoid complexity and wastage of computational power. We discuss some of these alternatives below. 

- (i) _Proof of work:_ In this consensus mechanism every blockchain node willing to mine or validate a block solves a complex mathematical problem that requires significant computational power.[70] To solve the problem the participants have to find a hash value of the block such that it meets a certain predefined difficulty level. The node which finds the solution first is the winner and it can create a block of transactions. 

- (ii) _Proof of stake (PoS):_ The PoS works are similar to PoW but the leader is chosen based on the stakes owned by the users of that network. The assumption here is, that the user with more stakes (commitment) has less probability of being a malicious node and would not attack the blockchain. This mechanism has a high chance of becoming centralized as rich committees have more voting power and may win every election. To overcome this problem and the _nothing at stake attack_ variations of PoS have been proposed.[71] To work more efficiently, the algorithm can have restricted elections, that is, _delegated PoS_ . Proof of elapsed time[72] and proof of importance are some alternatives that prevent centralization of voting.[73] 

- (iii) _BFT:_ The BFT algorithm guarantees consensus in a network the if at least 2/3 of the participating nodes are not malicious.[74] This algorithm is not a good option for a public blockchain because BFT can only work with a limited number of participants. The objective of this mechanism is to protect the network from system failures through collective decision making. 

- (iv) _Proof of elapsed time:_ This algorithm randomly chooses the next block using fair means. Every node willing to validate blocks gets a chance to create their block by waiting for a random amount of time. Each validator adds its waiting time in its block as a proof of their waiting, and broadcasts its block in the network. The validator with the least waiting time is the winner and its block gets added in the blockchain. 

## 3.3.2 Comparison between consensus algorithms 

The previous section has discussed various consensus algorithms and problems in achieving consensus in DLTs. Comparative studies on different consensus algorithms have been presented in References 75,77,83 in terms of energy-saving, scalability, latency, throughput, and so on. Table 2 presents a summary that compares these studies. 

HASSIJA et al. 

**7 of 23** 

**TA B L E 2** A comparison between various consensus mechanisms 

|~~**Reference **~~<br>52,75<br>~~52~~<br>76<br>~~77~~<br>52,77<br>~~75~~<br>76<br>~~78~~<br>79<br>~~80,81~~<br>82|~~**Consensus algorithm**~~<br>~~**Type of DLT used Advantages**~~<br>~~**Open issues**~~<br>Proof of work<br>Permissionless<br>High node scalability<br>High computational power<br>~~Proof of stake~~<br>~~Both types~~<br>~~High tolerance power~~<br>~~Low throughput~~<br>Leased proof of stake<br>Both types<br>Prevents centralization like in proof of stake Low throughput<br>~~Delegated proof of stake~~<br>~~Both types~~<br>~~High throughput~~<br>~~High message overhead~~<br>Proof of elapsed time<br>Both types<br>Partial energy saving<br>Medium throughput<br>~~Byzantine fault tolerant~~<br>~~Permissioned~~<br>~~High energy saving~~<br>~~Low node scalability~~<br>Proof of activity<br>Both types<br>Immune to 51% attack<br>High computational power<br>~~Proof of capacity~~<br>~~Permissionless~~<br>~~Cheap and efficient~~<br>~~Less decentralization~~<br>Proof of burn<br>Permissionless<br>Greater price stability<br>High waste of currency<br>~~Tangle~~<br>~~Both types~~<br>~~High scalability~~<br>~~Conflicts in sparse network~~<br>Hashgraph (virtual voting) Both types<br>Byzantine fault tolerant<br>Large size of signatures|
|---|---|



## **4 BLOCKCHAIN APPLICATION DOMAINS** 

With the increasing demand for distributed architectures,[84] blockchain provides a possible solution for applications in various domains. In Reference 85 the authors have discussed numerous domains (such as healthcare, smart grids, and IoT.) in which blockchain has been used or has a great potential to be used.[86] Some of these domains are discussed below. For each domain, we describe the potential for using blockchain and issues that need to be overcome. 

## **4.1 Blockchain and Internet of Things** 

With the advancements in technologies, users are shifting toward smart devices and Internet of Things (IoT). More than 20 billion smart devices and IoT devices are currently in use.[87] The biggest strength of IoT that makes it a potentially viable option in many sector is its ability to share data between various devices and provide easy access.[88,89] 

The ability to share data[90] also comes with various challenges such as security, privacy, and building trust.[91,92] This becomes even more critical in domains such as healthcare, finance, defense where often there is sensitive information that needs to be protected. Blockchain provides a robust solution to address these problems as it provides a platform to safely exchange information among untrusted users.[93-96] Furthermore, the distributed nature of blockchain eliminates the risk of a single point of failure in the IoT system. We discuss some specific applications of blockchain below. 

## 1. _IoT in industries_ 

IoT has proven to be a promising technology in various industrial sectors by providing real-time remote monitoring, low-latency, smart asset tracking, and so on.[34] However, there is a high risk of attacks related to security and privacy on IoT devices. Blockchain can help mitigate these issues and provide data immutability. Many researchers have worked in this domain and have explored the challenges posed by blockchain in industrial IoT along with their possible solutions.[96-98] 

2. _IoT for smart grids_ 

Blockchain can reduce costs and remove intermediaries in the energy sector.[54,99] This distributed platform allows trading of energy with various distributed sources[100] without the need for a centralized authority. Issues such as security and privacy are critical when it comes to a smart grid system integrated with IoT. A secure, cost-effective, and reliable solution to build an infrastructure for a smart grid using blockchain has been proposed by the authors in Reference 101. 

## **4.2 Blockchain and healthcare** 

Major stakeholders for information exchange in the healthcare sector include patients, doctors, pathology, health-insurance company, and regulation committees. Healthcare systems require an efficient solution to share 

HASSIJA et al. 

**8 of 23** 

information such as patient’s updated profiles between all the stakeholders. Blockchain provides this solution along with ensuring patient’s privacy, data immutability and restricted access to the data (for security purposes).[102] Blockchain can be used for a variety of applications in the healthcare sector which include:[103] 

1. _Secure interoperability of health records:_ Health-related data is very critical and personal. Hence there should be a robust platform to exchange healthcare data between various stakeholders. The authors in Reference 104 proposed a blockchain-based scheme for secure and integrated interoperability of health related data among stakeholders. They developed a design to validate the information in a transaction and also discussed the structure of a smart contract that ensures the fulfillment of the required conditions. 

2. _Healthcare supply chains:_ In the case of pharmaceutical drugs supply, many factors such as time duration, surrounding conditions (such as temperature, humidity, and so on) play a crucial role in ensuring a safe delivery. In Reference 105, the authors have proposed a framework to use blockchain to provide a record of factors such as temperature during the transportation of pharmaceutical drugs to various stakeholders. This empowers the company to perform quality control of its products.[106] 

## **4.3 Blockchain in business** 

Blockchain provides a robust platform to build a tamper-proof and secure distributed digital ledger which is supported by a consensus mechanism.[85,107] The append-only, immutable and secure nature of blockchain makes it a prominent option for a variety of businesses. Blockchain along with a smart contact, fulfills the basic requirement of any business-related use case.[108] This technology can be used for a variety of business-related use cases such as: 

1. _Outsourcing in cloud-based business:_ Cloud computing has become a widely accepted paradigm for outsourcing services. Cloud computing enables an organization to economically and scalable access advanced services by outsourcing them.[109] Some of the major issues in outsourcing services are data security and digital payment.[110,111] One option is to only involve a trusted third-party for outsourcing, but this becomes inefficient in the growing competition, as there can be a some untrusted third-party offering better services. The authors of Reference 112 have proposed a blockchain-based solution to outsource any third-party, even if it is not trusted. 

2. _Real estate_ The real estate ecosystem deals with sales and purchase of digital or physical assets such as land, building, or company shares. Various parties (eg, owner, purchaser, tenant, broker) are involved in this process, thereby increasing the risk of record tampering or modification. Another major issue is dealing with untrusted parties because entities might not have any previous business relationships. Smart contracts can solve these issues[113] and can further eliminate the need for trusted intermediaries such as brokers and notaries.[114] In References 115,116 the authors have discussed the potential use of blockchain in secure banking. 

Some other domains where blockchain can be used include: agriculture,[117,118] communication,[119] automobile industry,[120,121] and so on. 

Sections 3 and 4 presented a brief overview on the fundamentals of blockchain, their application domains and consensus mechanisms. In the following section, we present a framework to evaluate the suitability of blockchain to any given application and discuss the criteria used by the proposed framework. 

## **5 FRAMEWORK FOR EVALUATING THE USE OF BLOCKCHAIN** 

The performance characteristic of blockchain under different metrics and operating conditions form the basis of the proposed framework for evaluating blockchain’s applicability and suitability. We first present an overview of the advantages and disadvantages of blockchain, and then use them to develop the proposed framework. 

## **5.1 Advantages and disadvantages of traditional blockchain** 

The main advantages and disadvantages of traditional blockchain technology include: 

HASSIJA et al. 

**9 of 23** 

## 5.1.1 Advantages 

- (i) _Distributed:_ The systems and the data of a blockchain are highly resilient to malicious attacks and technical failure because the data is stored in a large number of nodes present in a distributed network. Each node has its own copy of the database and this eliminates the risk of a single point of failure. In addition, even if a node goes offline, it does not affect the security and availability of the network. 

- (ii) _Stability:_ Once a block is confirmed, it is very difficult to change or modify the data stored in it. This immutability of the data makes blockchain an attractive option to store data that is sensitive to any alterations and needs to be protected against modification. 

- (iii) _Trustless environment:_ Involvement of a trusted intermediary party is often required in the traditional payment systems, such as banks and payment providers. Blockchain eliminates this need for intermediaries because the transactions, in the distributed network of nodes, are verified through a mining process. Hence, there is no need to trust a single organization which also reduces the costs of intermediaries. 

## 5.1.2 Disadvantages 

- (i) _Low transaction speed:_ As each transaction has to be stored on the blockchain, it takes time to gather all the transactions and accumulate them in a block. Although there may be a large number of transactions packed into a single block, and the average block processing time is about 10 minutes, the transaction speed is theoretically 100 transactions per minute. However, in practice, all those transactions get confirmed only after the block is mined and that takes around 10 minutes (on average). Hence, the transaction time on blockchain is fairly high.[122] 

- (ii) _High data requirement:_ Blockchain stores a copy of every transaction that has occurred till date on that network, simultaneously, on all the computers/nodes connected to the network. This is a highly data-consuming process and the current bitcoin blockchain is about 1 TB in size and is growing daily. 

- (iii) _High energy-consumption:_ The consensus algorithm used in traditional blockchain is PoW. The POW technique of computing hashes consumes a lot of energy because a large number of computers connected on the blockchain network simultaneously compete against each other to find an appropriate hash. It is estimated that the total energy consumed by the miners on the bitcoin blockchain network is equivalent to the energy requirement of a decently sized European country. 

- (iv) _Pseudo anonymity:_ Due to the decentralized nature of blockchain, transaction information are made public in the cryptocurrency. Hence, user privacy may be compromised. In Reference 123, the authors have proposed a quasi-homomorphic symmetric encryption scheme to hide the transaction amounts in a blockchain-based cryptocurrency. In Reference 124, the authors combined ring signature technology with the existing blockchain system to ensure user privacy in the transparent environment of blockchain. 

## **5.2 Steps to assess “when” to use blockchain** 

In the previous section, we presented an overview of blockchain, its advantages and disadvantages, and various domains where it can be used. This section addresses the main question “when to use blockchain technology?” The proposed framework to answer this question is based on assessing use-case oriented simple questions as Figure 3 shows. The questions in this framework not only help to assess the suitability of blockchain for an application but they it also help to determine the most suitable blockchain technology 

- (i) _Do you need a shared database?:_ We assume that there is a need of a ledger database, that is, some data related to transactions needs to be stored. Data represents the present state of the ledger, which gets updated and must be shared. However, if data does not need to be shared, a complex blockchain-based architecture is not needed. Therefore, blockchain is not required if the answer is “no” and traditional databases are preferred. 

- (ii) _Are there multiple writers to the database?:_ Opting for blockchain only makes sense when multiple copies of the data has to be stored (by multiple users) and shared among them.[52] In a blockchain, multiple users have permission to 

**10 of 23** ~~—l-wi LEY~~ 

HASSIJA et al. 

**F I G U R E 3** Framework for evaluating the application of blockchain technology. Black arrows represent the answer for the respective question 

maintain the ledger and establish consensus among the users. In contrast to traditional server-client architectures with restricted writing rights, blockchain provides an alternate option of a decentralized peer to peer network where multiple users can write to the distributed ledger. 

- (iii) _Are there untrusted stakeholders involved?:_ Blockchain provides a robust solution to conduct transactions even if the parties or stakeholders involved are not trusted. If only trusted parties are involved and data is not very sensitive (not prone to attacks or modification) then using a traditional database is preferable. However, in scenarios where the possibility of choosing a third party to establish consensus in an untrusted environment is available, using a centralized architecture is preferred. 

- (iv) _Does the data need to be kept private?:_ If no external entity is involved to provide trust management services or if the data is sensitive (even if all stakeholders are trusted) then opting for blockchain technology is effective and should be considered. If data has to be kept private, maintaining a private blockchain is a good option. 

- (v) _Do we need any restriction on who can control the blockchain?:_ If the answer to the previous question on the need for data to be kept private is a “no,” and if there is no restriction to control the blockchain (ie, the system is maintained by a public community and all the transactions are transparent and visible to everyone) then having a public blockchain is preferable. 

- (vi) _Does the ledger need to be maintained by a group of selected organizations?:_ If maintenance of the ledger cannot be done publicly, and a group of some selected organizations needs to be chosen to entrust system maintenance, the use of a consortium blockchain is preferable. Consortium blockchain has the advantages of a public blockchain (transparent, available for all) as well as those of a private blockchain (controlled by more than one organization, which makes it decentralized). A private blockchain is preferred if there must be only one organization that can control and maintain the ledger.[125] 

HASSIJA et al. **11 of 23** ~~S| LEY———~~ 

## **6** | **ALTERNATIVES TO TRADITIONAL BLOCKCHAIN TECHNOLOGY** 

The previous section, described the drawbacks of using blockchain technology. Apart from blockchain technology, there are numerous other options that one may consider (if blockchain is not a suitable solution after assessing the above criteria). The following section presents similar technologies and alternatives (Figure 4 presents an overview). 

## **6.1** | **Hashgraph** 

Hashgraph is a DLT that uses an algorithm for duplication of state machines in order to ensure BFT.[126] This is sometimes also referred to as an atomic broadcast. The main protocol that runs this DLT is a gossip protocol. In this protocol, every node needs to share all its information and transactions with any other randomly selected node. The gossip protocol supports nesting which refers to the fact a gossip can also have another gossip built in it. All participants (nodes) in the network receive an identical chronologically arranged list of transactions referred to as “total order”. The implementation and feasibility part of Hashgraph is still questionable as this is not an open-source protocol. 

1. _Data structure:_ The data structure used in Hashgraph is a directed acyclic graph (DAG) with each transaction itself acting as a block in-itself. It has information such as signature, timestamp, transaction hash, and hash of the parent transaction. 

**F I G U R E 4** Blockchain alternatives and when to select them 

HASSIJA et al. 

**12 of 23** 

2. _Consensus algorithm:_ Hashgraph uses an innovative virtual voting[127] consensus method which sends out transactions to the network as an Atomic Blast Broadcast. 

3. _Advantages over traditional blockchain:_ 

   - (i) Hashgraph is a proprietary DLT and hence tampering with it is difficult. This is a significant benefit in terms of human exposed vulnerability. 

   - (ii) Due to its gossip protocol, the net transactional data that nodes have to store is significantly lower. 

   - (iii) As this DLT does not involve block formation and waiting for each block to get transactions confirmed, the speed of transactions is fairly high. Theoretically, it is possible to even reach the speed of 1 000 000 TPS (transactions per second) on the Hashgraph protocol 

As a result of these advantages, Hashgraph offers a variety of applications in today’s technology-driven world. Having a fast consensus algorithm enables the Hashgraph cryptocurrency to have low network fees, making small microtransactions economical, particularly in IoT applications. Hashgraph enables users to store decentralized data or pointers to files on the network in a secure and transparent manner. It also enables users to deploy smart contracts. Solidity language was developed for executing smart contracts on the Ethereum network although various libraries of Solidity code have been developed after, and can be run on the Hashgraph platform.[128] 

## **6.2 IOTA** 

IOTA uses a DLT called the Tangle. Tangle is similar to the traditional blockchain in the sense that it stores the history of transactions in an immutable data structure. Unlike the bitcoin blockchain, the Tangle protocol involves attaching each transaction to two previous transactions through their hashes. This drastically reduces the amount of data required for storing transactions. 

1. _Data structure:_ The data structure of IOTA is a DAG. In this data structure, each transaction can be referred either directly or indirectly. Trunk transaction field of any transaction stores transaction hash of either an existing transaction in the Tangle or of the transaction with the next index in the bundle. The direct references of a transaction are called its Parents, and indirect ones as grandparents, similar to a family tree. Every new transaction on this network starts as a tip transaction of the DAG. These are then chosen by nodes after validating them. 

2. _Consensus algorithm:_ The latest consensus algorithm for IOTA is fast probabilistic consensus with weighted votes which is an upgrade from the earlier Tangle protocol. This is a Byzantine-tolerant consensus algorithm where the voting power of a node is proportional to its reputation. The reputation of the node is modeled using the Zipf law which shows that the performance of the algorithm increases with the Zipf exponent. Zipf law is used because it best describes Mana distribution. 

3. _Advantages over traditional blockchain:_ 

   - (i) Due to its highly scalable infrastructure, the IOTA DLT can be used for IoT device’s transactional data. 

   - (ii) IOTA, similar to Hashgraph, has a DAG data structure that enables it to achieve fast transaction times. Hence, it is nearly 100 times faster than traditional blockchain. 

   - (iii) As this DLT does not involve block formation and waiting for each block to get transactions confirmed, the speed of transactions is fairly high. Theoretically, it is possible to even reach the speed of 1 000 000 TPS on the Hashgraph protocol. 

Having advantages such as high transaction rate along with a resilient architecture,[129] IOTA provides a large application base. For example, the authors[130] used the IOTA cryptocurrency to propose a privacy-preserving tolling architecture that supports decentralized feeless transfers for providing communication with roadside infrastructures. The proposed approach incorporates several technologies that work together to provide convenience and value to drivers and road operators. 

## **6.3 Nano Coin** 

Nano coin (also known as RaiBlocks) is one of the first coins to use no-fee transfers. Instead of using the traditional blockchain structure, Nano coin use the Block Lattice structure. This enables it to achieve high transaction speed at very low to no fees. The consensus method used by Nano coin involves PoS where any offender in the network is punished by 

HASSIJA et al. 

**13 of 23** 

foreclosing his/her stake that was put into the network while joining. There are nodes in the network whose function is to solve disputes when a collision transaction arises. These nodes lose their stake of coins if found guilty of approving a false transaction. 

1. _Data structure:_ The data structure of Nano coin is not like that of a traditional blockchain. Nano coin uses the Block Lattice structure wherein each address on the network has its own blockchain (address chain). 

2. _Consensus algorithm:_ Nano coin uses a combination of both PoS and PoW as its consensus mechanism. This hybrid mechanism is also called _delegated PoS_ . 

3. _Advantages over traditional blockchain:_ Nano coin has one of the highest transaction per second speed and hence it is one of the best choices for micropayment platforms. Nano coin was one of the first to adopt directed cyclic graphs and paved the way for subsequent technologies such as Hashgraph and IOTA to come up with a more robust and fast DLT. 

## **6.4 Zcash** 

Zcash is a privacy-centric digital currency that uses a zero-knowledge proof based technique for encryption. Zcash uses two types of addresses that are either private or transparent. The transactions among the Z addresses, that is, private or shielded addresses, need not be disclosed publicly. On the other hand, the T addresses, that is, the transparent addresses, work in a similar manner to a bitcoin address where the transactions are visible to the entire world.[131] Some of the salient features of Zcash are: 

- (i) _Minimum transaction fees:_ The transaction fee on the Zcash network is one of the lowest among all cryptocurrencies (0.0001 ZEC). 

- (ii) _Privacy:_ The core principle of Zcash is to ensure consumer privacy. This is achieved by using two types of addresses (shielded and transparent). Shielded addresses are not visible publicly. Addresses and transaction details (eg, transaction amount) are not revealed in a transaction between shielded addresses. On the other hand, transactions between transparent addresses are publicly viewable on the network. 

- (iii) _Time-based and multiple signature-based transactions:_ At times when a transaction is not mined for a long time, Zcash provides the facility to refund the transaction. Zcash also has the feature of multiple signatures which enables the production of large funds through the approval of multiple parties. 

Multiple address types (private and transparent) enable different types of transactions in the network: 

- (i) _Shielding:_ Transaction from transparent address to a private address 

(ii) _Public:_ Transaction from transparent adding to a transparent address. 

(iii) _Deshielding:_ Transaction from private adding to a transparent address. 

(iv) _Private:_ Transaction from private adding to a private address. 

1. _Data structure:_ The data structure of Zcash is the same as that of a traditional blockchain. 

2. _Consensus algorithm:_ Zcash uses PoW as its consensus mechanism. 

3. _Advantages over traditional blockchain:_ As we mentioned earlier, a special feature of Zcash is its privacy protection for users. With the help of zero-knowledge proof encryption, the privacy of the users is kept intact without making any compromises on the network security. 

The four alternative technologies described above have unique advantages and disadvantages. It is vital to understand the application’s or scenario’s need and choose the technology accordingly. Table 3 presents a brief comparison of these technologies. For example, Hashgraph and IOTA are highly scalable. On the other hand, Zcash is not so scalable but it provides better privacy options. These technologies may fulfill specific needs of a user (as shown in Figure 4) which a simple blockchain is unable to do so. (i) _Nano block lattice_ gives the ability to an account to maintain its own account chain; (ii) _IOTA_ enables a blockchain-based IoT platform to help machines communicate and perform transactions without any fee; (iii) _Zcash_ provides high user privacy and uses zero knowledge proof mechanism; (iv) _Hashgraph_ enables to store transaction timestamp ordering in a distributed ledger, which is vital for a network where the order of requests has a high priority.[132] 

HASSIJA et al. 

**14 of 23** 

**TA B L E 3** A comparison between alternative technologies to blockchain 

||~~**Hashgraph**~~<br>~~**IOTA**~~<br>~~**Nano coin**~~<br>~~**Zcash**~~<br>Consensus mechanism Atomic blast Broadcast<br>FPC with weighted votes<br>Delegated proof of stake<br>Proof of work<br>~~Data structure~~<br>~~Directed cyclic graph~~<br>~~Directed cyclic graph~~<br>~~Block lattice structure~~<br>~~Traditional blockchain~~<br>Scalability<br>Highly scalable<br>Highly scalable<br>Moderately scalable<br>Least scalable<br>~~Privacy~~<br>~~Pseudo anonymous~~<br>~~Pseudo anonymous~~<br>~~Pseudo anonymous~~<br>~~Protects privacy~~<br>Transaction rate<br>10 Transactions per second 11 Transactions per second 105 Transactions per second 3 Transactions per second<br>~~Token needed~~<br>~~Yes~~<br>~~Yes~~<br>~~Yes~~<br>~~Yes~~|
|---|---|



**TA B L E 4** Analysis of use cases based on the proposed criteria 

||~~**Supply chain**~~<br>~~**Multidrone network**~~<br>~~**Financial services**~~<br>Do we need a shared database?<br>Yes<br>Yes<br>Yes<br>~~Are there multiple writers to the database?~~<br>~~Yes~~<br>~~Yes~~<br>~~Yes~~<br>Untrusted stakeholders involved?<br>Yes<br>Yes<br>Yes<br>~~Is there any trusted third party for ledger maintenance?~~<br>~~No~~<br>~~No~~<br>~~Yes~~<br>Does the data need to be kept private?<br>No<br>No<br>NA<br>Is the dataprone to attacks? Can it be secured bystoringmultiple<br>copies?<br>NA<br>NA<br>NA<br>Do we need to restrict who can control the blockchain?<br>Yes<br>No<br>NA<br>Does the ledger needs to be maintained byagroupof selected<br>organizations?<br>No<br>NA<br>NA<br>Do we need to have each node maintaining its own account chain?<br>No<br>No<br>No<br>Do we need a blockchain-basedplatform for IoT to helpmachines<br>communicate and settle transaction without fees?<br>No<br>No<br>No<br>Do we need to record transaction timestamp ordering?<br>No<br>Yes<br>No<br>~~Do we need high user privacy using zero knowledge proof?~~<br>~~No~~<br>~~No~~<br>~~No~~|
|---|---|



_Note:_ “NA” means the particular question does not need to be answered. 

## **7 USE-CASE APPLICATION** 

Section 5 presented the proposed framework for evaluating the applicability of blockchain. In this section, we demonstrate the application of the framework to three test cases, namely, supply chain, multidrone network, and financial services. Table 4 presents a summary of the results. 

## **7.1 Supply chain** 

Supply chain management is the management of the transitions of goods and services and includes all processes involved in the transformation of raw materials into final products. It defines the life cycle of a product, from the manufacturer to the end-consumer. Generally, stakeholders at different levels do not have access to the products’ information as a whole. This results in inefficiency during the processing and transfer of products between the different stakeholders in a supply chain.[133] Blockchain can significantly improve the transparency in a supply chain[134] by providing the products’ information to all the actors in the process in its entirety.[135,136] Moreover, it enables end-consumers to monitor and trace the products’ transition with the help of IoT devices. The food system, for example, is very complex, and includes producers, processors, distributors, and consumers. The sharing of information in this complex network is challenging. However, blockchain with the help of IoT devices, can give the consumers the ability to not only track where the product came 

HASSIJA et al. 

**15 of 23** 

from but how was it produced (eg, if it was produced safely, if it grows sustainably, and so on). Since 2016, AgriDigital has pioneered, the use of blockchain across agricultural supply chains. AgriDigital and CBH Group, conducted a pilot to test the application of blockchain in the Australian grain industry at CBH’s wholly owned subsidiary, Blue Lake Milling, an oats processor in Bordertown, South Australia. To formally evaluate the applicability of blockchain in such environments using the proposed framework, we can consider the use case of AgriDigital and CBH Group.[137] 

- _Do we need a shared database?:_ Yes, the information needs to be registered into a ledger and communicated to all the stakeholders involved in the product’s life-cycle. 

- _Are there multiple writers to the database?:_ Yes, supply chain management is characterized by many stakeholders such as producers, processors, distributors, retailers, and consumers. All these actors interact with the blockchain. 

- _Are there untrusted stakeholders involved?:_ Yes, actors at different stages can be unknown and untrusted for others in the supply chain. 

- _Is there any trusted third party for ledger maintenance?:_ No, there is no specific third party such as a bank or notary for ledger maintenance. 

- _Does the data needs to be kept private?:_ No, the main objective of AgriDigital is to provide data to everyone concerned and to improve transparency in the supply chain. 

- _Do we need to restrict who can control the blockchain?:_ Yes, the supply chain has some restrictions on who can access the blockchain. The stakeholders involved are given permission to interact with the blockchain. 

- _Does the ledger needs to be maintained by a group of selected organizations?:_ No, there is no need for a specific organization to maintain the ledger. 

Hence, after assessing the _when and which_ part of our framework, we get private blockchain as a solution. It is worth noting that AgriDigital and the CBH Group operate on a private Quorum network. The Quorum network use the Raft consensus mechanism, which allows the AgriDigital network to have 4 TPS.[137] 

## **7.2 Multidrone network** 

Recent technological advances in drone technology or unmanned aerial vehicles (UAVs) in various fields such as networking, defense, manufacturing, have increased their usage in private and commercial sectors.[138] With UAV technology becoming omnipresent,[139] various issues in using UAVs need to be addressed such as inter UAV communication, data storage, management in multidrone networks,[140] constrained flight time, and so on. This section discusses the solution to resolve some of the issues stated above by considering a use case and applying our framework to decide if blockchain can be used for this application domain. 

_Multidrone networks_ have a wide range of applications such as delivering goods and medical supplies, and surveillance.[138] However, one of the main constraints that hampers the applications of UAVs is their limited energy supplies (batteries are kept small to reduce the overall weight of a drone). Frequent recharging or battery replacement is required because of constrained flight time. A distributed P2P network of drones and charging stations can solve this issue and can significantly increase drones’ flight time, thereby enabling the use of drones for multiple applications.[132] We now apply the proposed framework to evaluate the applicability of blockchain in multidrone applications.[141] 

- _Do we need a shared database?:_ Yes, communication between UAVs and charging stations is needed and information related to transactions needs to be shared and registered into a ledger and communicated to all other nodes (UAVs and charging stations) present in the network. 

- _Are there multiple writers to the database?:_ Yes, various drones and can enter into the network and can request charging stations to provide charging services. Each UAV should be able to interact with the ledger. 

- _Are untrusted stakeholders involved?:_ Yes, due to no restrictions for drones to enter into the network, unknown/untrusted UAVs may enter as well. 

- _Is there any trusted third party for ledger maintenance?:_ No, there is no need for any specific third party such as banks or notaries to maintain the ledger. 

HASSIJA et al. 

**16 of 23** 

- _Does the data needs to be kept private?:_ No, a charging station can have requests from many (if not all) drones in the network and similarly a drone needs information regarding every station present in the network. 

- _Do we need to restrict who can control the blockchain?:_ No, the system is free and without any restrictions. Any node present in the network can interact with the ledger. 

After applying the above criteria, a public blockchain could be the appropriate technology. However, in this multidrone system, drones should be able to enter and leave the network at a high frequency. Hence, an accurate _time-stamp ordering_ is required to avoid any conflicts. From Figure 4, we note that the Hashgraph DLT is a promising solution to meet the requirement of this system. In addition, Hashgraph has higher transaction throughput as compared with _hashgraph_ .[142] In Reference 132 the authors have proposed a similar Hashgraph based DLT for the specified multidrone network. 

## **7.3 Financial services** 

Blockchain is emerging as a powerful and secure option for recording data and transactions in the financial sector. Many cryptocurrencies such as bitcoin, ether, and ripple. use blockchain technology and support millions of users. For example, bitcoin had between 2.9 and 5.8 million unique users (as per[143] ) in 2017. However, it still lacks scalability because it can process only a few TPS (bitcoin can process 3-4 TPS). By contrast, other payment gateways such as the Visa Network can process over 17 000 TPS.[144] We now apply the proposed to framework on the Visa payment processing, application. 

- _Do we need a shared database?:_ Yes, users need to access the database to view their transaction histories. 

- _Are there multiple writers to the database?:_ Yes, thousands of users make transaction every second and each transaction has to be stored in the database. 

- _Are untrusted stakeholders involved?:_ Yes, the company has users from all over the world, and this increases the risk of having untrusted users. 

- _Is there any trusted third party for ledger maintenance?:_ Yes, Visa Inc. verifies each transaction and maintains the ledger. 

Hence, after assessing the above criteria, it can be concluded that having a centralized architecture is beneficial. Visa Inc. also uses a centralized architecture[145] that provides customized processing across the world. 

## **8 CHALLENGES AND FUTURE RESEARCH OPPORTUNITIES** 

Numerous blockchain-based decentralized applications are being developed due to the availability of a wide range of different blockchain technology options and their advantages available. In particular, many startups have been founded based on blockchain-related technologies since bitcoin came into existence. However, there are still many challenges that still need to be addressed for this technology: 

1. _Quantum attacks:_ Quantum computing presents a major threat[146] to blockchain due to their capability in solving certain complex problems. Quantum computers can easily break existing encryption techniques[147] and may provide enough resources to perform 51% attacks in the future (Figure 5 shows various types of attacks possible of blockchain). Various studies such as References 148-152 have discussed such security issues. 

2. _Scalability:_ Many efficient consensus mechanisms are being developed to reduce the energy consumption and the time taken to process a transaction. However, blockchain still faces scalability issues when it comes to the number of transactions it can process per unit time.[153] 

3. _Protecting blockchain from intelligent attacks:_ With the increasing possibility of more advanced types of attacks such as machine learning and game-theory based attacks on blockchain networks,[154,155] it has become a necessity to secure blockchain against such attacks. 

4. _Reducing computational power usage:_ Blockchain, in general, requires high computational power which is a drain on resources. This hinders the development of the technology for many applications such as drone/UAV networks[132] where computational power is scarce. 

HASSIJA et al. 

**17 of 23** 

**F I G U R E 5** Different types of attacks on blockchain 

## **9 CONCLUSION** 

To reap the full potential of any new technology, it has to be analyzed from various perspectives. Blockchain technology has a great potential to transform industries by providing security, transparency, anonymity, and immutability. Despite being a relatively new technology, blockchain has been adopted in various fields. In this article, we have not only provided the background behind blockchain technology, but also a framework to evaluate the applicability of blockchain technology and determined possible alternatives, if needed. With the rapid advances in technologies, the drawbacks of blockchain have been addressed to some extent. However, there is still scope for future developments and this work has identified some of the areas such as improving security against quantum attacks, reducing computational power usage and more, where research opportunities exist. 

## **ACKNOWLEDGMENTS** 

This work was supported by DST-SERB (Science and Engineering Research Board (SERB)) funding received by Dr. Vinay Chamola under Project Grant File no. ECR/2018/001479. We thank the anonymous reviewers for their valuable comments which helped us improve presentation and content of this article. 

## **DATA AVAILABILITY STATEMENT** 

Data sharing not applicable to this article as no datasets were generated or analyzed during the current study. 

HASSIJA et al. 

**18 of 23** 

## **ORCID** 

_Vinay Chamola_ https://orcid.org/0000-0002-6730-3060 

## **REFERENCES** 

1. Kaur A, Nayyar A, Singh P. Blockchain: a path to the future. _Cryptocurrencies and Blockchain Technology Applications_ ; 2020:25-42. 

2. Schollmeier R. A definition of peer-to-peer networking for the classification of peer-to-peer architectures and applications. Paper presented at: Proceedings of the 1st International Conference on Peer-to-Peer Computing; 2001:101-102, Linköping, Sweden. 

3. Hassija V, Chamola V, Garg S, Krishna DNG, Kaddoum G, Jayakody DNK. A blockchain-based framework for lightweight data sharing and energy trading in V2G network. _IEEE Trans Veh Technol_ . 2020;69(6):5799–5812. http://dx.doi.org/10.1109/tvt.2020.2967052 

4. Goyal S. Centralized vs decentralized vs distributed; 2015. https://medium.com/@bbc4468/centralized-vs-decentralizedvs-distributed%41d92d463868 

5. Ren Y, Zhu F, Zhu K, Sharma PK, Wang J. Blockchain-based trust establishment mechanism in the internet of multimedia things. _Multimed Tools Appl_ . 2020;1-24. 

6. Hosen AS, Singh S, Sharma PK, et al. Blockchain-based transaction validation protocol for a secure distributed IoT network. _IEEE Access_ . 2020;8:117266-117277. 

7. Jain N, Chugh K. Security concerns of blockchain. _Blockchain for Business: How It Works and Creates Value_ ; 2021:201-230. 

8. Zyskind G, Nathan O, Decentralizing privacy: Using blockchain to protect personal data. Paper presented at: Proceedings of the 2015 IEEE Security and Privacy Workshops; 2015:180-184; San Jose, CA. 

9. Giungato P, Rana R, Tarabella A, Tricase C. Current trends in sustainability of bitcoins and related blockchain technology. _Sustain For_ . 2017;9(12):2214. 

10. Jogalekar P, Woodside M. Evaluating the scalability of distributed systems. _IEEE Trans Parallel Distrib Syst_ . 2000;11(6):589-603. 

11. Cao J, Wang X, Huang M, Yi B, He Q. A security-driven network architecture for routing in industrial Internet of Things. _Trans Emerg Telecommun Technol_ . 2021;32:e4216. 

12. Alsamhi SH, Lee B, Guizani M, Kumar N, Qiao Y, Liu X. Blockchain for decentralized multi-drone to combat COVID-19 and future pandemics: framework and proposed solutions. _Trans Emerg Telecommun Technol_ . 2021;e4255. 

13. Fernández-Caramés TM, Froiz-Míguez I, Blanco-Novoa O, Fraga-Lamas P. Enabling the internet of mobile crowdsourcing health things: a mobile fog computing, blockchain and IoT based continuous glucose monitoring system for diabetes mellitus research and care. _Sensors_ . 2019;19(15):3319. 

14. Gao S, Zheng D, Guo R, Jing C, Hu C. An anti-quantum E-voting protocol in blockchain with audit function. _IEEE Access_ . 2019;7:115304-115316. 

15. Fernández-Caramés TM, Fraga-Lamas P. A review on the application of blockchain to the next generation of cybersecure industry 4.0 smart factories. _IEEE Access_ . 2019;7:45201-45218. 

16. Hassija V, Bansal G, Chamola V, Kumar N, Guizani M. Secure lending: blockchain and prospect theory-based decentralized credit scoring model. _IEEE Trans Netw Sci Eng_ . 2020;7(4):2566–2575. https://dx.doi.org/10.1109/tnse.2020.2982488 

17. Malomo O, Rawat D, Garuba M. Security through block vault in a blockchain enabled federated cloud framework. _Appl Netw Sci_ . 2020;5(1):1-18. 

18. Sharma PK, Park JH, Cho K. Blockchain and federated learning-based distributed computing defence framework for sustainable society. _Sustain Cities Soc_ . 2020;102220. 

19. Malik AA, Tosh DK, Ghosh U. Non-intrusive deployment of blockchain in establishing cyber-infrastructure for smart city. Paper presented at: Proceedings of the 2019 16th Annual IEEE International Conference on Sensing, Communication, and Networking (SECON); 2019:1-6; Boston, MA. 

20. Singh J, Venkatesan S. Blockchain mechanism with Byzantine fault tolerance consensus for internet of drones services. _Trans Emerg Telecommun Technol_ . 2021;32:e4235. 

21. Fanning K, Centers DP. Blockchain and its coming impact on financial services. _J Corp Account Finance_ . 2016;27(5):53-57. 

22. Maurer B. Re-risking in realtime. on possible futures for finance after the blockchain. _Behemoth-A J Civ_ . 2016;9(2):82-96. 

23. Fu Y, Zhu J. Big production enterprise supply chain endogenous risk management based on blockchain. _IEEE Access_ . 2019; 7:15310-15319. 

24. Truong NB, Sun K, Lee GM, Guo Y. GDPR-compliant personal data management: a blockchain-based solution; 2019. arXiv preprint arXiv:1904.03038. 

25. Guo H, Li W, Nejad M, Shen CC. Access control for electronic health records with hybrid blockchain-edge architecture. Paper presented at: Proceedings of the 2019 IEEE International Conference on Blockchain (Blockchain); 2019:44-51; Atlanta, GA. 

26. Mertz L. (Block) chain reaction: a blockchain revolution sweeps into health care, offering the possibility for a much-needed data solution. _IEEE Pulse_ . 2018;9(3):4-7. 

27. Guan Z, Si G, Zhang X, et al. Privacy-preserving and efficient aggregation based on blockchain for power grid communications in smart communities. _IEEE Commun Mag_ . 2018;56(7):82-88. 

28. Hassija V, Bansal G, Chamola V, Saxena V, Sikdar B. Blockcom: a blockchain based commerce model for smart communities using auction mechanism. Paper presented at: Proceedings of the 2019 IEEE International Conference on Communications Workshops (ICC Workshops); 2019:1-6; Shanghai, China. 

HASSIJA et al. 

**19 of 23** 

29. Su Z, Wang Y, Xu Q, Fei M, Tian YC, Zhang N. A secure charging scheme for electric vehicles with smart communities in energy blockchain. _IEEE Internet Things J_ . 2018;6(3):4601-4613. 

30. Kianmajd P, Rowe J, Levitt K. Privacy-preserving coordination for smart communities. Paper presented at: Proceedings of the 2016 IEEE Conference on Computer Communications Workshops (INFOCOM WKSHPS); 2016:1045-1046; San Francisco, CA, USA. 

31. Alcarria R, Bordel B, Robles T, Martín D, Manso-Callejo MÁ. A blockchain-based authorization system for trustworthy resource monitoring and trading in smart communities. _Sensors_ . 2018;18(10):3561. 

32. Aggarwal S, Chaudhary R, Aujla GS, Kumar N, Choo KKR, Zomaya AY. Blockchain for smart communities: applications, challenges and opportunities. _J Netw Comput Appl_ . 2019;144:13-48. 

33. Stanciu A. Blockchain based distributed control system for edge computing. Paper presented at: Proceedings of the 2017 21st International Conference on Control Systems and Computer Science (CSCS); 2017:667-671; Bucharest, Romania. 

34. Liu H, Zhang Y, Yang T. Blockchain-enabled security in electric vehicles cloud and edge computing. _IEEE Netw_ . 2018; 32(3):78-83. 

35. Kang J, Yu R, Huang X, et al. Blockchain for secure and efficient data sharing in vehicular edge computing and networks. _IEEE Internet Things J_ . 2018;6(3):4660-4670. 

36. El Ioini N, Pahl C, Helmer S. A decision framework for blockchain platforms for IoT and edge computing. Paper presented at: Proceedings of the Conference: 3rd International Confernce on Internet of Things, Big Data and Security; 2018; Funchal, Madeira, Portugal: Scitepress. 

37. Kurtulmus AB, Daniel K. Trustless machine learning contracts; evaluating and exchanging machine learning models on the ethereum blockchain; 2018. arXiv preprint arXiv:1802.10185. 

38. Kuo TT, Ohno-Machado L. Modelchain: Decentralized privacy-preserving healthcare predictive modeling framework on private blockchain networks; 2018. arXiv preprint arXiv:1802.01746. 

39. Swan M. Blockchain thinking: the brain as a decentralized autonomous corporation [commentary]. _IEEE Technol Soc Mag_ . 2015;34(4):41-52. 

40. Weng J, Weng J, Zhang J, Li M, Zhang Y, Luo W. Deepchain: auditable and privacy-preserving deep learning with blockchain-based incentive. _IEEE Trans Depend Secure Comput_ . 2019. https://doi.org/10.1109/TDSC.2019.2952332 

41. Dai Y, Xu D, Maharjan S, Chen Z, He Q, Zhang Y. Blockchain and deep reinforcement learning empowered intelligent 5G beyond. _IEEE Netw_ . 2019;33(3):10-17. 

42. Luong NC, Xiong Z, Wang P, Niyato D. Optimal auction for edge computing resource management in mobile blockchain networks: a deep learning approach. Paper presented at: Proceedings of the 2018 IEEE International Conference on Communications (ICC); 2018:1-6; Kansas City, MO, USA. 

43. Juneja A, Marefat M. Leveraging blockchain for retraining deep learning architecture in patient-specific arrhythmia classification. Paper presented at: Proceedings of the 2018 IEEE EMBS International Conference on Biomedical & Health Informatics (BHI); 2018:393-397; Las Vegas, NV, USA. 

44. Qiu C, Yu FR, Yao H, Jiang C, Xu F, Zhao C. Blockchain-based software-defined industrial Internet of Things: a dueling deep Q-learning approach. _IEEE Internet Things J_ . 2018;6(3):4627-4639. 

45. Wüst K, Gervais A. Do you need a blockchain? Paper presented at: Proceedings of the 2018 Crypto Valley Conference on Blockchain Technology (CVCBT); 2018:45-54; Zug, Switzerland. 

46. Koens T, Poll E. What blockchain alternative do you need? Data Privacy Management, Cryptocurrencies and Blockchain TechnologyESORICS 2018 International Workshops, DPM 2018 and CBT 2018; 2018:113-129; Springer: Barcelona, Spain. 

47. Meunier S. When do you need blockchain? Decision models. https://medium.com/@sbmeunier/when%2010do%2010you%2010need% 2010blockchain%2010decision%2010mode%ls%2010a5c40e7c9ba1. Accessed date 20 April, 2021. 

48. Gatteschi V, Lamberti F, Demartini C, Pranteda C, Santamaria V. To blockchain or not to blockchain: that is the question. _IT Professional_ . 2018;20(2):62-74. 

49. Peck ME. Blockchain world-do you need a blockchain? this chart will tell you if the technology can solve your problem. _IEEE Spectr_ . 2017;54(10):38-60. 

50. Menon J. 10 questions to ask before you use blockchain; 2020. https://rb.gy/qkusr2 

51. Scriber BA. A framework for determining blockchain applicability. _IEEE Softw_ . 2018;35(4):70-77. https://doi.org/10.1109/MS.2018. 2801552 

52. Belotti M, Boži´c N, Pujolle G, Secci S. A vademecum on blockchain technologies: when, which, and how. _IEEE Commun Surv Tutor_ . 2019;21(4):3796-3838. 

53. Dinh TTA, Liu R, Zhang M, Chen G, Ooi BC, Wang J. Untangling blockchain: a data processing view of blockchain systems. _IEEE Trans Knowl Data Eng_ . 2018;30(7):1366-1385. 

54. Miglani A, Kumar N, Chamola V, Zeadally S. Blockchain for Internet of Energy management: review, solutions, and challenges. _Comput Commun_ . 2020;151:395-418. 

55. Kosba A, Miller A, Shi E, Wen Z, Papamanthou C. Hawk: the blockchain model of cryptography and privacy-preserving smart contracts. Paper presented at: Proceedings of the 2016 IEEE symposium on security and privacy (SP); 2016:839-858; San Jose, CA, USA. 

56. Singh P, Nayyar A, Kaur A, Ghosh U. Blockchain and fog based architecture for internet of everything in smart cities. _Future Internet_ . 2020;12(4):61. 

57. Singh PK, Singh R, Nandi SK, Ghafoor KZ, Rawat DB, Nandi S. Blockchain-based adaptive trust management in internet of vehicles using smart contract. _IEEE Trans Intell Transp Syst_ . 2020. 

58. Sharma PK, Kumar N, Park JH. Blockchain technology toward green IoT: opportunities and challenges. _IEEE Netw_ . 2020. 

HASSIJA et al. 

**20 of 23** 

59. Doku R, Rawat DB. IFLBC: on the edge intelligence using federated learning blockchain network. Paper presented at: Proceedings of the 2020 IEEE 6th International Conference on Big Data Security on Cloud (BigDataSecurity), IEEE International Conference on High Performance and Smart Computing,(HPSC) and IEEE International Conference on Intelligent Data and Security (IDS); 2020:221-226; Baltimore, MD, USA. 

60. Ren Y, Zhu F, Sharma PK, et al. Data query mechanism based on hash computing power of blockchain in Internet of Things. _Sensors_ . 2020;20(1):207. 

61. Ra GJ, Roh CH, Lee IY. A key recovery system based on password-protected secret sharing in a permissioned blockchain. _CMC-Comput Mater Contin_ . 2020;65(1):153-170. 

62. Lin X, Li J, Wu J, Liang H, Yang W. Making knowledge tradable in edge-AI enabled IoT: a consortium blockchain-based efficient and incentive approach. _IEEE Trans Ind Inform_ . 2019;15(12):6367-6378. 

63. Doku R, Rawat DB, Liu C. On the blockchain-based decentralized data sharing for event based encryption to combat adversarial attacks. _IEEE Trans Netw Sci Eng_ . 2020. 

64. Kim TH, Goyat R, Rai MK, et al. A novel trust evaluation process for secure localization using a decentralized blockchain in wireless sensor networks. _IEEE Access_ . 2019;7:184133-184144. 

65. Hassija V, Chamola V, Gopala Krishna DN, Kumar N, Guizani M. A blockchain and edge computing-based secure framework for government tender allocation. _IEEE Internet Things J_ . 2021;8(4):2409–2418. https://dx.doi.org/10.1109/jiot.2020.3027070 

66. Gray JN. _Notes on Data Base Operating Systems_ . New York, NY: Springer; 1978:393-481. 

67. Schneider FB. Implementing fault-tolerant services using the state machine approach: a tutorial. _ACM Comput Surv_ . 1990; 22(4):299-319. 

68. Lamport L, Shostak R, Pease M. The Byzantine generals problem. _Concurrency: The Works of Leslie Lamport_ . New York, NY: Association for Computing Machinery; 2019:203-226. 

69. Aspnes J, Jackson C, Krishnamurthy A. _Exposing Computationally-Challenged Byzantine Impostors. Technical Report YALEU/DCS/TR-1332_ . New Haven: Department of Computer Science, Yale University; 2005. 

70. Li J, Zhou Z, Wu J, et al. Decentralized on-demand energy supply for blockchain in internet of things: a microgrids approach. _IEEE Trans Comput Soc Syst_ . 2019;6(6):1395-1406. 

71. Li W, Andreina S, Bohli JM, Karame G. _Securing Proof-of-Stake Blockchain Protocols_ . New York, NY: Springer; 2017:297-315. 

72. Olson K, Bowman M, Mitchell J, Amundson S, Middleton D, Montgomery C. _Sawtooth: An Introduction_ . New York, NY: The Linux Foundation; 2018. 

73. Hassija V, Saxena V, Chamola V. A mobile data offloading framework based on a combination of blockchain and virtual voting. _Softw Pract Exper_ . 2020. 

74. Castro M, Liskov B. Practical Byzantine fault tolerance. _OSDI’99_ . Vol 99; New Orleans, LA: USENIX Association; 1999:173-186. 

75. Zheng Z, Xie S, Dai H, Chen X, Wang H. An overview of blockchain technology: architecture, consensus, and future trends. Paper presented at: Proceedings of the 2017 IEEE international congress on big data (BigData congress); 2017:557-564; Honolulu, HI, USA. 

76. Anwar H. Consensus algorithms: the root of the blockchain technology. https://101blockchains.com/consensus-algorithms-blockchain/ amp/#6. Accessed date 20 April, 2021. 

77. Bach L, Mihaljevic B, Zagar M. Comparative analysis of blockchain consensus algorithms. Paper presented at: Proceedings of the 2018 41st International Convention on Information and Communication Technology, Electronics and Microelectronics (MIPRO); 2018:1545-1550; Opatija, Croatia. 

78. Grigorchuk K. Overview of 9 blockchain consensus algorithms. https://digiforest.io/blog/blockchain-consensus-algorithms. Accessed date 20 April, 2021. 

79. Kenton W. Proof of Burn (Cryptocurrency); 2020. https://rb.gy/3vqyqv 

80. Shafeeq S, Zeadally S, Alam M, Khan A. Curbing address reuse in the IOTA distributed ledger: a cuckoo-filter-based approach. _IEEE Trans Eng Manag_ . 2020;67(4):1244-1255. https://doi.org/10.1109/TEM.2019.2922710 

81. Bramas Q. The Stability and the Security of the Tangle; 2018. 

82. Wall E. Hedera Hashgraph — Time for some FUD; 2019. https://medium.com/@ercwl/hedera-hashgraph-time-for-some-fud9e6653c115%25 

83. Cachin C, Vukoli´c M. Blockchain consensus protocols in the wild; 2017. arXiv preprint arXiv:1707.01873. 

84. Weber I, Xu X, Riveret R, Governatori G, Ponomarev A, Mendling J. Untrusted business process monitoring and execution using blockchain. Paper presented at: Proceedings of the International Conference on Business Process Management; 2016: 329-347; Springer, New York, NY. 

85. Syed TA, Alzahrani A, Jan S, Siddiqui MS, Nadeem A, Alghamdi T. A comparative analysis of blockchain architecture and its applications: problems and recommendations. _IEEE Access_ . 2019;7:176838-176869. 

86. Jindal A, Aujla GSS, Kumar N, Villari M. GUARDIAN: blockchain-based secure demand response management in smart grid system. _IEEE Trans Serv Comput_ . 2019. 

87. Statista I. Internet of Things (IoT) connected devices installed base worldwide from 2015 to 2025 (in billions); 2018. 

88. Alladi T, Chamola V, Parizi RM, Choo KKR. Blockchain applications for industry 4.0 and industrial IoT: a review. _IEEE Access_ . 2019;7:176935-176951. 

89. Praveen G, Chamola V, Hassija V, Kumar N. Blockchain for 5G: a prelude to future telecommunication. _IEEE Netw_ . 2020;34(6):106–113. https://dx.doi.org/10.1109/mnet.001.2000005 

HASSIJA et al. 

**21 of 23** 

90. Narayanaswamy J, Sampangi RV, Sampalli S. SCARS: simplified cryptographic algorithm for RFID systems. Paper presented at: Proceedings of the 2014 IEEE RFID Technology and Applications Conference (RFID-TA); 2014:32-37; Tampere, Finland. 

91. Yang Q, Lu R, Rong C, Challal Y, Laurent M, Wang S. Guest editorial the convergence of blockchain and IoT: opportunities, challenges and solutions. _IEEE Internet Things J_ . 2019;6(3):4556-4560. 

92. Yong Y, Yao D, Zhiqiang Z, Wei J, Shaoyong G. Edge computing-based tasks offloading and block caching for mobile blockchain. _Comput Mater Contin_ . 2020;62(2):905-915. 

93. Fernández-Caramés TM, Fraga-Lamas P. A review on the use of blockchain for the Internet of Things. _IEEE Access_ . 2018;6:32979-33001. 

94. Reyna A, Martín C, Chen J, Soler E, Díaz M. On blockchain and its integration with IoT. challenges and opportunities. _Futur Gener Comput Syst_ . 2018;88:173-190. 

95. Hakak S, Khan WZ, Gilkar GA, Imran M, Guizani N. Securing smart cities through blockchain technology: architecture, requirements, and challenges. _IEEE Netw_ . 2020;34(1):8-14. 

96. Le Nguyen B, Lydia EL, Elhoseny M, et al. Privacy preserving blockchain technique to achieve secure and reliable sharing of IoT data. _Comput Mater Cont_ . 2020;65(1):87-107. 

97. Miller D. Blockchain and the internet of things in the industrial sector. _IT Prof_ . 2018;20(3):15-18. 

98. Cao B, Wang X, Zhang W, Song H, Lv Z. A many-objective optimization model of industrial Internet of Things based on private blockchain. _IEEE Netw_ . 2020;34(5):78-83. 

99. Alladi T, Chamola V, Rodrigues JJ, Kozlov SA. Blockchain in smart grids: a review on different use cases. _Sensors_ . 2019;19(22):4862. 

100. Ai S, Hu D, Zhang T, Jiang Y, Rong C, Cao J. Blockchain based power transaction asynchronous settlement system. Paper presented at: Proceedings of the 2020 IEEE 91st Vehicular Technology Conference (VTC2020-Spring); 2020:1-6. 

101. Lombardi F, Aniello L, De Angelis S, Margheri A, Sassone V. A blockchain-based infrastructure for reliable and cost-effective IoT-aided smart grids; 2018; Antwerp, Belgium. 

102. Amanullah MA, Habeeb RAA, Nasaruddin FH, et al. Deep learning and big data technologies for IoT security. _Comput Commun_ . 2020;151:495-517. 

103. Chen J, Lv Z, Song H. Design of personnel big data management system based on blockchain. _Futur Gener Comput Syst_ . 2019;101:1122-1129. 

104. Transaction CP, MPI MPI. Blockchain: opportunities for health care. CP transaction; 2016. 

105. Bocek T, Rodrigues BB, Strasser T, Stiller B. Blockchains everywhere-a use-case of blockchains in the pharma supply-chain. Paper presented at: Proceedings of the 2017 IFIP/IEEE Symposium on Integrated Network and Service Management (IM); 2017:772-777; Lisbon, Portugal. 

106. Griggs KN, Ossipova O, Kohlios CP, Baccarini AN, Howson EA, Hayajneh T. Healthcare blockchain system using smart contracts for secure automated remote patient monitoring. _J Med Syst_ . 2018;42(7):130; Lisbon, Portugal. 

107. Liu C, Li K, Tang Z, Li K. Bargaining game-based scheduling for performance guarantees in cloud computing. _ACM Trans Model Perform Eval Comput Syst (TOMPECS)_ . 2018;3(1):1-25. 

108. Wan J, Li J, Imran M, Li D. A blockchain-based solution for enhancing security and privacy in smart factory. _IEEE Trans Ind Inform_ . 2019;15(6):3652-3660. 

109. Xie S, Zheng Z, Chen W, Wu J, Dai HN, Imran M. Blockchain for cloud exchange: a survey. _Comput Electr Eng_ . 2020;81:106526. 

110. Dey S, Sampalli S, Ye Q. A context-adaptive security framework for mobile cloud computing; 2015:89-95; IEEE. 

111. Hassija V, Chamola V, Saxena V, Jain D, Goyal P, Sikdar B. A survey on IoT security: application areas, security threats, and solution architectures. _IEEE Access_ . 2019;7:82721-82743. 

112. Zhang Y, Deng R, Liu X, Zheng D. Outsourcing service fair payment based on blockchain and its applications in cloud computing. _IEEE Trans Serv Comput_ . 2018. https://doi.org/10.1109/TSC.2018.2864191 

113. Karamitsos I, Papadaki M, Al Barghuthi NB. Design of the blockchain smart contract: a use case for real estate. _J Inf Secur_ . 2018;9(3):177-190. 

114. Spielman A. _Blockchain: Digitally Rebuilding the Real Estate Industry_ [PhD thesis]. Massachusetts Institute of Technology; 2016. 

115. Mainelli M, Milne A. The impact and potential of blockchain on the securities transaction lifecycle; 2016. 

116. Leinonen H. _Decentralised Blockchained and Centralised Real-Time Payment Ledgers: Development Trends and Basic Requirements_ . New York, NY: Springer; 2016:236-261. 

117. Salah K, Nizamuddin N, Jayaraman R, Omar M. Blockchain-based soybean traceability in agricultural supply chain. _IEEE Access_ . 2019;7:73295-73305. 

118. Wu HT, Tsai CW. An intelligent agriculture network security system based on private blockchains. _J Commun Netw_ . 2019;21(5):503-508. 

119. Alvarenga ID, Rebello GA, Duarte OCM. Securing configuration management and migration of virtual network functions using blockchain. Paper presented at: Proceedings of the NOMS 2018-2018 IEEE/IFIP Network Operations and Management Symposium; 2018:1-9. 

120. Brousmiche KL, Durand A, Heno T, Poulain C, Dalmieres A, Hamida EB. Hybrid cryptographic protocol for secure vehicle data sharing over a consortium blockchain; 2018:1281-1286; Taipei, Taiwan. 

121. Hassija V, Gupta V, Garg S, Chamola V. Traffic jam probability estimation based on blockchain and deep neural networks. _IEEE Trans Intell Transp Syst_ . 2021;22(7):3919–3928. http://dx.doi.org/10.1109/tits.2020.2988040 

122. Singh G, Singh A, Singh M, Sharma S, Kumar N, Choo KKR. BloCkEd: blockchain-based secure data processing framework in edge envisioned V2X environment. _IEEE Trans Veh Technol_ . 2020. 

HASSIJA et al. 

**22 of 23** 

123. Bai S, Yang G, Rong C, Liu G, Dai H. QHSE: an efficient privacy-preserving scheme for blockchain-based transactions. _Futur Gener Comput Syst_ . 2020;112:930-944. 

124. Li X, Mei Y, Gong J, Xiang F, Sun Z. A blockchain privacy protection scheme based on ring signature. _IEEE Access_ . 2020;8:76765-76772. 

125. Tian Y, Yuan J, Song H. Secure and reliable decentralized truth discovery using blockchain. Paper presented at: Proceedings of the 2019 IEEE Conference on Communications and Network Security (CNS); 2019:1-8; Washington, DC. 

126. Akhtar Z. From blockchain to hashgraph: distributed ledger technologies in the wild. Paper presented at: Proceedings of the 2019 International Conference on Electrical, Electronics and Computer Engineering (UPCON); 2019:1-6; Aligarh, India. https://doi.org/10.1109/ UPCON47278.2019.8980029 

127. Hassija V, Saxena V, Chamola V, Yu R. A parking slot allocation framework based on virtual voting and adaptive pricing algorithm. _IEEE Trans Veh Technol_ . 2021;69(6):5945–5957. https://doi.org/10.1109/TVT.2020.2979637 

128. Živi´c N, Kaduši´c E, Kaduši´c K. Directed acyclic graph as hashgraph: an alternative DLT to blockchains and tangles. Paper presented at: Proceedings of the 2020 19th International Symposium INFOTEH-JAHORINA (INFOTEH); 2020:1-4; East Sarajevo, Bosnia and Herzegovina. https://doi.org/10.1109/INFOTEH48170.2020.9066312 

129. Cullen A, Ferraro P, King C, Shorten R. On the resilience of DAG-based distributed ledgers in IoT applications. _IEEE Internet Things J_ . 2020;7(8):7112-7122. https://doi.org/10.1109/JIOT.2020.2983401 

130. Bartolomeu PC, Vieira E, Ferreira J. Pay as you go: a generic crypto tolling architecture. _IEEE Access_ . 2020;8:196212-196222. https://doi. org/10.1109/ACCESS.2020.3034299 

131. Hassija V, Chamola V, Zeadally S. BitFund: A blockchain-based crowd funding platform for future smart and connected nation. _Sustain Cities Soc_ . 2020;60:102145. 

132. Hassija V, Saxena V, Chamola V. Scheduling drone charging for multi-drone network based on consensus time-stamp and game theory. _Comput Commun_ . 2020;149:51-61. 

133. Deimel M, Frentrup M, Theuvsen L. Transparency in food supply chains: empirical results from German pig and dairy production. _J Chain Netw Sci_ . 2008;8(1):21-32. 

134. O’Leary DE. Configuring blockchain architectures for transaction information in blockchain consortiums: the case of accounting and supply chain systems. _Intell Syst Account Finance Manag_ . 2017;24(4):138-147. 

135. Li X, Lv F, Xiang F, Sun Z, Sun Z. Research on key technologies of logistics information traceability model based on consortium chain. _IEEE Access_ . 2020;8:69754-69762. 

136. Hassija V, Chamola V, Gupta V, Jain S, Guizani N. A survey on supply chain security: application areas, security threats, and solution architectures. _IEEE Internet Things J_ . 2020. 

137. Sylvester G. E-agriculture in action: blockchain for agriculture: opportunities and challenges. International Telecommunication Union; 2019. 

138. Alladi T, Chamola V, Sahu N, Guizani M. Applications of blockchain in unmanned aerial vehicles: a review. _Veh Commun_ . 2020;100249. 

139. Hassija V, Chamola V, Krishna DNG, Guizani M. A distributed framework for energy trading between uavs and charging stations for critical applications. _IEEE Trans Veh Technol_ . 2020;69(5):5391-5402. 

140. Chamola V, Hassija V, Gupta V, Guizani M. A comprehensive review of the COVID-19 pandemic and the role of IoT, drones, AI, blockchain, and 5G in managing its impact. _IEEE Access_ . 2020;8:90225-90265. 

141. Liu Y, Wang J, Song H, Li J & Yuan J Blockchain-based secure routing strategy for airborne mesh networks. Paper presented at: Proceedings of the 2019 IEEE International Conference on Industrial Internet (ICII); 2019:56-61; Orlando, FL. 

142. Khalilov MCK, Levi A. A survey on anonymity and privacy in bitcoin-like digital cash systems. _IEEE Commun Surv Tutor_ . 2018;20(3):2543-2585. 

143. Hileman G, Rauchs M. Global cryptocurrency benchmarking study. _Cambridge Centre Alternat Finance_ . 2017;33:33-113. 

144. Li K. The blockchain scalability problem & the race for visa-like transaction speed. https://towardsdatascience.com/the%2010blockchain% 2010scalability%2010problem%2010the%2010r%ace%2010for%2010visa%2010like%2010transaction%2010speed%20105cce48f9d44=:%7 E:text=Visa%20does%20around%2%01%2C700%20transactions,150%20million%20transactions%20per%20day. Accessed date 20 April, 2021. 

145. visa-net-booklet. https://usa.visa.com/dam/VCOM/download/corporate/media/visanet-technolo%gy/visa-net-booklet.pdf. Accessed date 20 April, 2021. 

146. Zhang X, Wu F, Yao W, Wang W, Zheng Z. Post-quantum blockchain over lattice. _Comput Mater Cont_ . 2020;63(2):845-859. 

147. Jurvetson S How a quantum computer could break 2048-bit RSA encryption in 8 hours; 2019. https://www.technology review.com/2019/05/30/65724/how%2010a%2010quantum%2010compute%r%2010could%2010break%20102048%2010bit%2010rsa%2010 encryption%2010in%20108%2010hours/ 

148. Aggarwal D, Brennen GK, Lee T, Santha M, Tomamichel M. Quantum attacks on Bitcoin, and how to protect against them; 2017. arXiv preprint arXiv:1710.10377. 

149. Gao YL, Chen XB, Chen YL, Sun Y, Niu XX, Yang YX. A secure cryptocurrency scheme based on post-quantum blockchain. _IEEE Access_ . 2018;6:27205-27213. 

150. Fedorov AK, Kiktenko EO, Lvovsky AI. Quantum computers put blockchain security at risk; 2018. 

151. Ikeda K. Security and privacy of blockchain and quantum computation. _Advances in Computers_ . Vol 111. Amsterdam, Netherlands: Elsevier; 2018:199-228. 

152. Rodenburg B, Pappas SP. Blockchain and quantum computing; 2017. 

HASSIJA et al. 

**23 of 23** 

153. Kim S, Kwon Y & Cho S A survey of scalability solutions on blockchain. Paper presented at: Proceedings of the 2018 International Conference on Information and Communication Technology Convergence (ICTC); 2018:1204-1207; Jeju, Korea (South). 

154. Liu Z, Luong NC, Wang W, et al. A survey on blockchain: a game theoretical perspective. _IEEE Access_ . 2019;7:47615-47643. 

155. Hassija V, Chamola V, Han G, Rodrigues JJ, Guizani M. Dagiov: a framework for vehicle to vehicle communication using directed acyclic graph and game theory. _IEEE Trans Veh Technol_ . 2020;69(4):4182-4191. 

**How to cite this article:** Hassija V, Zeadally S, Jain I, Tahiliani A, Chamola V, Gupta S. Framework for determining the suitability of blockchain: Criteria and issues to consider. _Trans Emerging Tel Tech_ . 2021;32:e4334. https://doi.org/10.1002/ett.4334 

