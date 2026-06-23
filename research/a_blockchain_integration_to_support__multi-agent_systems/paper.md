Engineering Applications of Artificial Intelligence 107 (2022) 104534 

**==> picture [61 x 67] intentionally omitted <==**

Contents lists available at ScienceDirect 

## Engineering Applications of Artificial Intelligence 

journal homepage: www.elsevier.com/locate/engappai 

**==> picture [61 x 73] intentionally omitted <==**

## A Blockchain integration to support transactions of assets in multi-agent systems 

**==> picture [29 x 29] intentionally omitted <==**

## Fernando Gomes Papi[a] , Jomi Fred Hübner[a] , Maiquel de Brito[b][,][∗] 

a _Federal University of Santa Catarina UFSC/CTC/DAS/PPGEAS - PO Box 476, Florianópolis, SC, 88040-900, Brazil_ b _Federal University of Santa Catarina UFSC/CTE/CAC - R.João Pessoa, 2513 - sl.c304, Blumenau, SC, 89036-004, Brazil_ 

|A R T I C L E|I N F O|A B S T R A C T|
|---|---|---|
|_Keywords:_||Multi-Agent systems technology can offer valuable tools to develop applications in domains involving|
|Blockchain||transactions of assets. However, they usually do not have a proper support for reliable and decentralized|
|Multi-agent systems||recording of the transactions that are common in this kind of system. This support can be provided by the|
|Institutions<br>Assets||Blockchain technology. Furthermore, it is important to have means to represent in the system concepts that are<br>intangible, such as_asset_ and_ownership_. This paper presents a model of integration between Multi-Agent Systems|
|||and Blockchain where an artificial institution connects the intangible concepts related to transactions of assets|
|||to the concrete elements composing the system. An application example illustrates an implementation following|
|||the proposed integration model, showing its advantages and limitations. In the example, agents contract each|
|||other to provide services upon the payment of a dealt value through a system based on cryptocurrencies|
|||and blockchain. It highlights the essential contributions of the proposed approach to systems where agents|
|||transact assets: regulation of the system, representation of the notion of asset that does not depend on agents,|
|||and reliable recording of transactions based on Blockchain.|



## **1. Introduction** 

The field of Artificial Intelligence in general, and Multi-Agent Systems (MAS) in particular, aims to develop autonomous computational entities — the _agents_ — capable of flexible and autonomous action in dynamic contexts, that usually contain other agents, possibly developed by different parties and distributed in a network (Luck et al., 2005). Features of the agents such as autonomy, proactivity, capacity of learning, and social abilities to cooperate and negotiate, make MAS a suitable approach to develop applications in several domains, including those involving the transaction of assets. Examples of these domains are stock market (Farjam and Kirchkamp, 2015), energy negotiation in smart grids (Amato et al., 2014), e-commerce systems (Aranda-Corral et al., 2015; Cuní et al., 2004), supply chain (Dominguez and Cannella, 2020; Ito and Abadi, 2002), notary services (Pinto and Silva, 2020), personal data management (Truong et al., 2019), etc. 

Increasingly autonomous and connected computer systems and devices require means to lead them to beneficially act in asset trading systems. These means include secure, agent-independent representations and registering of the elements related to the trading, such as the assets themselves, their ownership, their transactions, etc. Without 

such means, different trader agents — and even third interested parties, such as regulatory authorities — could have discrepant views about the traded assets or could be even unable to access them. For example, a digital financial market could have rules either issued by a legal authority, such as a central bank (e.g. respect a maximum daily trade amount) or even inherent to the market itself, such as the offer and demand law. Without a proper representation of assets, the agents might not agree on or even be aware of what assets they have traded, which assets are available, what are their prices, etc. Thus, they could not be able to follow the market rules. 

A first challenge to endow MAS with the required elements to enable agents to trade assets is the assignment of ownership of these assets. Usually, a central power or arbiter (like a democratically elected governing body) makes that ownership official, e.g., when a city attests that someone is the rightful owner of a farm by providing official documents and registration. However, relying on a single, centralized entity to ensure that an asset is property of an agent could be unfeasible. If such an entity is an agent, the assignment of ownership would be subject to that agent autonomy. The agent could, for example, change its mind about who is the owner of an asset and unlawfully change its ownership. Furthermore, a single entity (even a non-autonomous one) 

- ∗ Corresponding author. 

_E-mail address:_ maiquel.b@ufsc.br (M. de Brito). 

> 1 In this paper, the word ‘‘Blockchain’’ refers to the overall concept, technology and theory, and ‘‘blockchain’’ refers to the actual implementation of a distributed ledger over a P2P network. For example, ‘‘Alice is really interested in studying Blockchain theory, and is actually working in some improvements for the Ethereum blockchain’’. 

https://doi.org/10.1016/j.engappai.2021.104534 Received 23 June 2020; Received in revised form 5 October 2021; Accepted 27 October 2021 Available online 11 November 2021 

0952-1976/© 2021 Elsevier Ltd. All rights reserved. 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

could constitute a single central point of failure, subject to problems such as network crashes and double-spending problems (Antonopoulos, 2014). To deal with these issues, the _Blockchain_ technology appears as a suitable decentralized, reliable digital support for assets possession registering.[1] It is the underlying technology that made Bitcoin viable, consisting essentially of data structure and computing protocol, distributed along a network, that records all of the currency transactions (Swan, 2015). Its cryptographic design makes the Blockchain an unhackable time-stamped database of transactions, disseminating trust along a decentralized network of nodes. 

However, even if Blockchain solves the reliable decentralization issue, a further problem arises: abstract concepts such as _asset_ and _ownership_ are not part of Blockchain. A blockchain is a secure database, but at the very core, it means nothing to an agent capable of learning and autonomously deciding its actions. Such meaning depends on a particular interpretation of each agent. For example, when the blockchain records an incoming transaction for an agent as payment for a performed action, that transaction is not automatically related neither to a payment nor to the ownership of some corresponding asset. The transaction on the Blockchain does not make concrete the concepts related to transactions of assets. Rather, it depends on the agents to assign some meaning to it. In this case, problems may arise if, for example, the agents are unable to correctly interpret that transaction or if different agents have different interpretations about the same transaction. With this in mind, a tool is needed to make such concepts concrete to the agents participating on the MAS. 

The previously discussed problems can be summarized in two points: (i) MAS frameworks have little to no tools that support decentralized, reliable transactions of assets between agents and (ii) existing technologies to solve this issue – including Blockchain – are not sufficient to provide to the agents notions such as _asset_ and _ownership_ . Considering these problems, this work aims to investigate and propose a link between Multi-Agent Systems and the Blockchain technology to support the transaction of assets between autonomous agents. The questions to be answered are: (i) Is integrating a blockchain into a MAS viable? If so, what is the best role or model for Blockchain in a MAS? (ii) Is it possible to represent the notion of assets from transactions recorded in a Blockchain? This paper aims to answer these questions, proposing a model of integration of Blockchain with MAS in a way that the records in a blockchain are meaningful from the perspective of transactions of assets. Achieving these goals is important to instrument MAS with a suitable support for transactions of assets in a context where computational systems increasingly take part in this kind of operation, dealing with each other and even with humans. The proposed model provides MAS with a reliable, decentralized recording of the transactions of assets fully integrated with the technologies usually employed in this kind of system. Besides, it endows the recorded transactions with the abstract meaning of _asset_ . Sharing such a meaning among all the agents ensures a common interpretation of what is an asset. _Asset_ becomes thus a concept of the overall system, independent of any agent. 

This paper is organized as follows: Section 2 presents the essential background of our work; Section 3 presents our proposed framework for developing MAS where agents engage in transactions of assets; Section 4 shows how the proposed framework works through an application example; Section 5 brings a discussion about the results of the work; related works are described in Section 6; and Section 7 brings some final remarks and directions for future work. 

## **2. Background** 

This work involves two essential domains: Multi-Agent Systems and Blockchain. This Section presents the elements on these topics that are essential to the proposed approach. Section 2.1 presents MAS, briefly introducing concepts such as agents, environment, and regulation in MAS, to focus later on artificial institutions, that are a key notion in this work. Section 2.2, on its turn, presents the essential elements of Blockchain and Smart Contracts. 

## _2.1. Multi-Agent Systems: environment, norms, and institutions_ 

Agents are computational entities that are situated in some environment where they are able to autonomously act to achieve their design objectives (Wooldridge, 2009). Autonomy refers to the capacity of the agents to act without the intervention of an external operator. Besides autonomy, agents have (i) social ability, being able to interact with other agents and possibly with humans, (ii) reactivity, being able to respond to changes perceived in the environment, and (iii) proactiveness, taking the initiative to achieve their goals (Wooldridge and Jennings, 1995). Finally, agents are usually capable to learn, improving their acting and adapting themselves to new circumstances (Langley et al., 2009). 

The environment in MAS can be seen from two perspectives. In the first one, it is an exogenous entity, composed of non autonomous elements external to the agents, and that are beyond the scope of the MAS design. Another perspective, which is adopted in this paper, takes the environment as an endogenous entity, being thus a first-class abstraction with proper computational representations to take part in the design of the MAS (Ricci et al., 2011). This endogenous perspective bases the _Agents and Artifacts_ metamodel (Omicini et al., 2008), that is implemented, for example, by the CArtAgO technology (Ricci et al., 2011). The building blocks of CArtAgO environments are the _artifacts_ , that represent the resources and tools that the agents can create, discover, perceive, and use at runtime (Boissier et al., 2013). The agents can perceive the state of the artifacts through their _observable properties_ and can act upon them through their available _operations_ . 

Since the agents are autonomous, their behavior can deviate from overall system expectations. For this reason, it is important to regulate their actions. There are several concepts, usually based in human societies (e.g. organizations, commitments, interaction protocols, etc.), that are applied to MAS to regulate and coordinate the behavior of the autonomous agents.[2] Among all of these social concepts, the most essential one is _norm_ . Norms are a usual way to specify and to monitor the expected behavior of the agents (Boella et al., 2008). Although there are several proposed normative models, norms in general express behaviors that are obliged, prohibited or permitted. These behaviors may be given in terms of actions produced or states holding in the system (Vos et al., 2013). 

## _2.1.1. Situated artificial institutions_ 

To cover a wide range of circumstances, norms usually abstract from the concrete elements in the environment (Aldewereld et al., 2010). For example, in an auction scenario, the norm ‘‘ _bidders are obliged to bid_ ’’ is supposed to cover all the agents that are considered as _bidders_ and all the actions that are considered as _bid_ . However, agents and actions cannot be considered _per se_ as bidders and bids. To monitor the norm compliance, as well as to guide the agents to follow the norms, it is necessary to connect the abstract concepts used in the norm specification to the elements of the environment. 

In human societies, _institutions_ assign meaning to the concrete elements of the world (Searle, 1995, 2009). Institutions state, for example, that a piece of paper counts as a dollar bill or that a person counts as a professor. Inspired by these notions, the Situated Artificial Institution (SAI) model provides the elements to specify and deploy institutions in MAS (de Brito et al., 2018). SAI institutions, thus, assign institutional meaning to the elements of the environment where the agents act, connecting the abstract concepts used in the norms to the environment. For example, in an auction scenario, the institution may state that an operation in an artifact counts as a _bid_ or that the agent that has performed a certain behavior counts as the _winner_ . This process is 

> 2 Among the huge literature in the field, the interested reader can find detailed information on regulation and coordination in MAS in Boella et al. (2007, 2009) and Andrighetto et al. (2013) and in the COIN series of workshops (http://www.pcs.usp.br/~coin/). 

2 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

called _constitution_ , that is specified through _constitutive rules_ defining the conditions to assign _status functions_ to the _environmental elements_ . These elements involved in the constitutive specification are described in the sequence: 

- **Environmental elements.** The environmental elements of interest in SAI are represented by  =  ∪ ∪ where  is the set of agents possibly acting in the system,  is the set of events that may happen in the environment, and  is the set of properties used to describe the possible states of the environment. 

- **Status Functions.** Status functions are status, with corresponding functions, that the institution may assign to the environmental elements. The set  contains all the possible status functions of a SAI and is defined as  =  ∪  ∪  where  is the set of agent-status functions (i.e. status functions assignable to agents _𝑎_  ∈ _𝐴_  ),  is the set of event-status functions (i.e. status functions assignable to events _𝑒_  ∈ _𝐸_  ), and  is the set of statestatus functions (i.e. status functions assignable to states _𝑠_  ∈ _𝑆_  ). For example, in an auction, an agent may have the agentstatus function of _winner_ , the utterance ‘‘I offer $100’’ may have the event-status function of _bid_ , and ‘‘more than 20 people placed in a room at Friday 10am’’ may have the state-status function of _minimum quorum_ for its realization. 

- **Constitutive rules.** Status functions are assigned to environmental elements according to the specification designed by the constitutive rules. The set of all constitutive rules of a SAI is represented by . A constitutive rule _𝑐_ ∈  is a tuple ⟨ _𝑥, 𝑦, 𝑡, 𝑚_ ⟩ meaning that _𝑥_ ∈  ∪  ∪{ _𝜀_ } counts as (i.e. _𝑥_ has the status function) _𝑦_ ∈  when the event _𝑡_ ∈  ∪  ∪{ _𝜀_ } has happened and while the condition represented by _𝑚_ holds.[3] 

The interpretation of constitutive rules assigns status functions to the environmental elements, building the _constitutive state_ of the system. The basic elements involved on this dynamics are the environment and the very constitutive state, described in the following: 

- **Environmental state.** The actual state of the environment is represented by _𝑋_ = _𝐴𝑋_ ∪ _𝐸𝑋_ ∪ _𝑆𝑋_ where (i) _𝐴𝑋_ is the set of agents currently participating in the system, (ii) _𝐸𝑋_ is the set of events currently occurring in the environment and (iii) _𝑆𝑋_ is the set of environmental properties describing the current environmental state. 

- **Constitutive state.** The constitutive state is composed of the standing status functions assignments (SFAs), produced by the interpretation of the constitutive rules. It is the institutional interpretation of the environment. The constitutive state is represented by _𝐹_ = _𝐴𝐹_ ∪ _𝐸𝐹_ ∪ _𝑆𝐹_ where (i) _𝐴𝐹 ⊆𝐴𝑋_ ×  is the set of agentstatus function assignments, (ii) _𝐸𝐹 ⊆𝐸𝑋_ ×  × _𝐴𝑋_ is the set of event-status function assignments and (iii) _𝑆𝐹 ⊆𝑆𝑋_ ×  is the set of state-status function assignments. 

The process of interpretation of constitutive rules that builds the constitutive state is detailed in de Brito et al. (2018). Briefly, if the actual environment matches with the elements _𝑡_ and _𝑚_ of a constitutive rule ⟨ _𝑥, 𝑦, 𝑡, 𝑚_ ⟩, then the environmental element _𝑥_ constitutes the status function _𝑦_ , producing a SFA. On the other hand, if the environment is no longer matching the conditions that lead to the production of an existing SFA, then such SFA is dropped from the constitutive state. 

When norms are inserted in a SAI, they are monitored with respect to the constitutive state, producing the normative state. The dynamics of the normative state depends on the considered normative model. A detailed discussion of normative regulation based on SAI can be found in de Brito et al. (2019). Fig. 1 illustrates the essential ideas of SAI: the agent _bob_ acts in the environment uttering the highest bid and, according to a constitutive rule, he counts – in the constitutive state – as the winner of the auction. In the normative state, _bob_ is obliged to pay its offer because a norm states that such obligation stands for the agent that counts as the winner of the auction. 

> 3 _𝜀_ represents that the element is not present in the constitutive rule. 

## _2.2. Blockchain and smart contracts_ 

Money is a ‘‘thing which serves as the generally accepted and commonly used medium of exchange’’ (Von Mises, 1966). This ‘‘thing’’ must meet essential features: it must (i) not be consumed along the exchanges, (ii) be sufficiently scarce to hold value, (iii) keep its purchasing power in proximate locations and dates, (iv) be divisible up to some point, and (v) be portable (Stroukal, 2018). Bitcoin is virtual money. It presents the aforementioned features even without corresponding physical coins. Users of bitcoin own digital keys that allow them to prove ownership of transactions in the bitcoin network, unlocking the value to spend it and transfer it to a new recipient. Bitcoins are created through a _mining_ process that involves solving a mathematical problem while bitcoin transactions are processed (Antonopoulos, 2014). 

Any existing unit of Bitcoin (also called _token_ or _coin_ ) should not be spent twice. This is similar to physical money: someone giving a bill to pay for a product no longer owns that physical piece of paper, being thus unable to spend it again on another product. However, when the money is digital, it is easy to copy it and spend it over and over. This is called the double-spending problem. In the Bitcoin system, the double spend problem, as well as the trust issues as a whole, are solved based on decentralized trust, that is achieved as an emergent property from the interactions among the participants, independent of any central trust authority (Antonopoulos, 2014). 

Every transfer of bitcoins from an owner to another (i.e. every _transaction_ ) is securely recorded in a peer-to-peer distributed ledger called _Blockchain_ (Narayanan et al., 2016).[4] This ledger is accessible to anyone through the Internet. Each transaction record is propagated across the network, rapidly reaching a large part of the computers executing the ledger (called _nodes_ ). The nodes validate the transactions to reach a distributed consensus. Invalid transactions are rejected. Confirmed transactions are recorded in _blocks_ , that are _mined_ by the nodes and then added to the blockchain. This mining process consists of solving a mathematical problem to find the hash that satisfies some requirements. This is an expensive task which miners are rewarded for with a transaction fee. 

From a technological perspective, it is possible to see the Blockchain as a database which makes it possible to check the validity and veracity of its records by registering and signing all of its state modifications. The transactions’ data are packed up in blocks that will be linked by hash pointers pointing to the previous block of data. In Bitcoin’s protocol, the cryptography algorithm used is the SHA-256 which produces a hash of 256 characters. 

Fig. 2 shows the basic structure of a blockchain. Each block contains (i) a Merkle Tree (Narayanan et al., 2016; Szydlo, 2004), which is a linked tree of hashes of transaction data in that block, along with a pointer that points to the previous block; (ii) a timestamp; (iii) a Nonce, which is a number that is generated only once in order to approve the block in the network; and (iv) the hash of the previous block. This is an essential feature of blockchains: changing the data of an older block requires to change all the hashes of all the subsequent blocks to keep consistency. The last block of a tampered network has a hash that is different from what it was supposed to. 

Therefore, the validity of the whole chain can be checked by just looking at the hash of the last block. In the case of Fig. 2, changes in the data of block 10 require to change the hash of blocks 11, 12, and so on up to the last block. These changes are replicated throughout the network, being then validated again since the blockchain is a peer-topeer technology. In that case, only someone with control of more than half of the network’s nodes would be able to propagate such changes. 

> 4 Though Bitcoin’s blockchain is not adopted in this work, it is important to look at its fundamental concepts to understand how this technology works. Later, the actual Blockchain that was used in this work – Ethereum – will be explained with further details and its usage justified. 

3 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [350 x 183] intentionally omitted <==**

**Fig. 1.** SAI overview: constitutive rules specify the building of the constitutive state from the environmental state while norms specify the building of the normative state from the constitutive state. 

**==> picture [312 x 120] intentionally omitted <==**

**Fig. 2.** Schematic of a blockchain. 

## _2.2.1. Smart contracts_ 

The Blockchain, seen as a platform for distributed computing in applications that require distributed consensus, quickly received attention as a potential tool for applications other than recording transactions of Bitcoins. Among several examples of these applications, there are those where nodes execute pieces of code implementing functions, usually related to transactions of assets of any nature. In this case, computational functions stored in the blockchain are deployed, producing outputs for determined inputs as long as the blockchain itself is executing in the decentralized network. These pieces of code are the _Smart Contracts_ . Taking advantage of the nature of blockchains, Smart Contracts are immutable. Only the parameters of their functions can be changed by the participants if there is consensus about it. A contract which two nodes agree upon is guaranteed to execute under the proper conditions (Wood et al., 2014). A simple example is a contract that transfers a particular amount of tokens from a participant _𝐴_ to a participant _𝐵_ when a bar code is read, meaning that _𝐴_ bought something from _𝐵_ . The contract may be coded in such a way that participant _𝐴_ cannot default on the agreement with _𝐵_ , and if there are no funds in the account, then the sale is never completed in the first place. Participant _𝐵_ could, for example, have the contract check if _𝐴_ has the necessary funds deposited on the contract before delivering the product. This ‘‘either wholly do some specific action, or do not do it at all’’ is a powerful characteristic of Smart Contracts that guarantee their integrity. 

Currently, the Bitcoin protocol does not support Smart Contracts. The most developed platform for this technology today is the Ethereum Project (Wood et al., 2014). The main and crucial difference between Ethereum’s and Bitcoin’s blockchain is the ability to run Smart Contracts. All other characteristics and advantages of Bitcoin’s blockchain 

are also present when using Ethereum. It can be thought of as a distributed, blockchain-based computer. It has the Ethereum Virtual Machine (EVM), that is a Turing complete machine that operates similar to traditional computers: it pushes and pops instructions executing the code of Smart Contracts, transitioning from one state to another and producing outputs that are consumed by the participants on contracts. While Bitcoin nodes receive instructions to make transactions from one address to another, Ethereum nodes receive requests to execute functions of Smart Contracts. These operations can be trusted to be true, just as transactions on Bitcoin protocol. Also similar to the Bitcoin protocol, Smart Contracts pay transaction fees for having the network execute its code. The execution leads the EVM to a new state, that is propagated to validation, leading the whole network to a new state. The outcomes of the operations are thus immutable and stored as long as the network is running. The EVM provides a programming language called _Solidity_ to program and compile the bytecode used in the Smart Contracts. Ethereum exposes various APIs, so that external applications can exchange data with the blockchain through the JSON-RPC protocol, which stands for JavaScript Object Notation — Remote Procedure Call. 

## **3. Conceptual model and system design** 

This section describes the proposed framework to build MAS where agents engage in transactions of assets. This proposal deals with two main challenges. The first one, addressed in Section 3.1, is the insertion of the Blockchain into the MAS to provide distributed, secure recording of the transactions. The second challenge, addressed in Section 3.2, refers to the problem of endowing the records stored in the blockchain with a meaning related to transactions of assets. 

4 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [206 x 167] intentionally omitted <==**

**==> picture [241 x 157] intentionally omitted <==**

**Fig. 4.** Model of the Blockchain as specific artifacts in the environment. 

**Fig. 3.** Model of the Blockchain as a generic environment. 

## _3.1. Adding Blockchain to MAS_ 

Inserting Blockchain in MAS requires a definition of its role in the system. Since a blockchain is a non autonomous entity which agents possibly perceive (e.g. to check informations about a transaction) and act upon (e.g. transferring the ownership of some asset), it can be deemed as part of the environment. From this assumption, it is necessary to discuss possible ways to integrate the blockchain to the MAS environment. Building upon a preliminary discussion presented by Papi et al. (2017), two possible approaches for such integration are presented, in Sections 3.1.1 and 3.1.2. These proposed approaches are theoretical, independent of any particular technology. Section 3.1.3, on its turn, describes how some existing technologies can implement the approach described in Section 3.1.2. 

## _3.1.1. Blockchain as a generic environment_ 

Blockchain could base the whole environment of the MAS. All the elements placed there could be coded as Smart Contracts, as they are Turing Complete machines. Every single action or transaction performed by agents (e.g. an agent delegating a task to another one, sending another agent a particular amount of currency, simply stating a fact, etc.) would be immutably recorded as long as the blockchain exists. Fig. 3 illustrates this proposed model, where agents (the human like figures) interact, through the blockchain, with other agents, Smart Contracts that are outside the scope of the MAS (the yellow and green boxes) and the Artifacts, which are the Smart Contracts defined in the context of the MAS. 

This approach has the advantage of developing every needed object on the same platform, without requiring additional technology or interfaces. However, it also presents some drawbacks. The first one regards cost (both computational and financial). There are transaction fees to execute smart contracts in the network (cf. Section 2.2.1). The costs of executing a Smart Contract depend on the complexity of its code. According to the website ethgasstation.info, a basic transaction on the Ethereum’s network could cost 0.00021 ETH, or US$0.02856 (as of February 2019). Considering a MAS with 200 agents, where half settles debts with the other half every day through one transaction per pair, this Smart Contract would amount to about US$1050 annually, on transactions alone without considering other computations done on the Smart Contract. 

Performance is another key issue in this approach. After adding a block to the Ethereum chain, the network waits for some additional blocks to be added after it to confidently state that a transaction is secured. This process takes around 5 min, which limits the use of Blockchain in systems with higher speed requirements. A third drawback of this approach regards scalability. Computational 

power, memory for storage and Internet bandwidth are bottlenecks for blockchain applications. Both Bitcoin’s and Ethereum’s ledgers were about 200 GB in size by early 2019 and this scenario tends to get worse (Chauhan et al., 2018). 

From these drawbacks, recording every action of the agents in the Blockchain seems an unfeasible approach. It makes sense when we consider human societies: we do not register every single interaction we have with other humans or objects. We record a subset of these interactions instead, including those involving promises, contracts, transactions of value and so on. From this inspiration, it is possible to consider using blockchain to base the subset of environmental artifacts that has the role of recording interactions related to transactions of assets. 

## _3.1.2. Blockchain instrumenting application artifacts_ 

Since Smart Contracts are programmable, they can be coded to support specific artifacts in the environment.[5] These artifacts serve as an interface between the contract and the system. For example, a voting system to decide whether an agent has endorsed a false statement can be programmed in a Smart Contract to store and officially count the votes that agents have deposited in the corresponding artifact. Artifacts can also implement security features, such as authorizations for actions (e.g. a method of access available only to agents playing very specific roles). These design choices depend on the requirements of the system and its goals. Fig. 4 shows how, in this model, artifacts implement the interface between agents and the blockchain. Agents can communicate with each other and also act upon the artifacts available in the environment. Among these artifacts, there might be those that are interfacing Smart Contracts. 

The scalability of this model greatly depends on the requirements of the system. The Ethereum Blockchain is a kind of a world computer where users pay nodes for computing. However, such computing is slow, with a rate of around 10 to 30 executions per second. A system that requires the advantages brought by blockchains in all of its artifacts possibly loses its speed requirements. 

Although the number of artifacts implemented in the blockchain is not necessarily limited, there is a compromise of execution time and number of interactions with the blockchain. This is observed in an application example presented in Papi et al. (2017) and extended in this paper in Section 4: implementing some environmental artifacts based on blockchain makes the execution of the system orders of magnitude slower than in a version where the same artifacts are implemented in this MAS by itself. 

> 5 Although _artifact_ is closely related to CArtAgO, this word refers, in this section, to any element in the environment. 

5 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [409 x 230] intentionally omitted <==**

**Fig. 5.** The whole integration of the system. 

## _3.1.3. Blockchain instrumenting JaCaMo artifacts_ 

The general approach presented in Section 3.1.2 can be realized in many ways depending on the technologies employed in the development of the MAS. This section describes a possible implementation of this approach considering the JaCaMo framework (Boissier et al., 2013, 2020). This is a framework for developing MAS where agents are programmed in Jason (Bordini et al., 2007), that is a variant of the AgentSpeak programming language (Rao, 1996), environments are implemented through CArtAgO, and the social aspects, including coordination and regulation, are based on oise organizations (Hübner et al., 2007). 

Since CArtAgO artifacts are based on Java, their integration with blockchain can be based on a Java implementation of the JSONRPC protocol. One of these implementations is the Web3j library,[6] that provides Java based wrappers for pre-compiled Smart Contracts implemented in the Solidity language. These Smart Contracts must implement the equivalent logic of the desired JaCaMo artifacts. Fig. 5 shows an overview of this proposal. In this structure, the artifacts (yellow box with operations and observable properties) base the code of the Smart Contract. This Smart Contract is compiled to generate a Compiled Code, represented by the dark blue box in Fig. 5. The Compiled code is then deployed to the network. The library also generates the Java Wrapper Classes, which communicate with the deployed code. A CArtAgO artifact referred as Interface Artifact imports these classes and becomes able to read a local Ethereum wallet file, deploy contracts to the network, make transactions, perform operations on the Smart Contract and so on. The Interface Artifact is able to operate and retrieve information from the network, as requested by agents, through an Accessible Node (which can be running locally or remotely). It also produces the environmental events and properties that acquire some meaning – assigned by an _institution_ – related to transactions of assets to guide the actions of the agents and to base the regulation of the system (cf. Section 3.2). 

To connect JaCaMo artifacts to Smart Contracts, the operations of the artifacts must be coded as Smart Contracts functions. Observable properties of the artifacts must be implemented in Smart Contracts as _public view_ functions whose return value corresponds to the property value. To illustrate this integration, we consider the implementation 

of the _Auction Artifacts_ , presented in Papi et al. (2017), as _Smart Contracts_ . Consider an artifact with the operation _Bid_ , that registers a bid from a participating agent, and an observable property _currentWinner_ , that states who is the current winner for that auction. To integrate this artifact to the blockchain, both the operation and the observable property must be implemented in a Smart Contract, as shown in Fig. 6. The function _placeBid_ implements in the contract the logic of the _Bid_ operation in the artifact. The function _getCurrentWinnerbyAuctionID_ returns a string that corresponds to the value of the _currentWinner_ property of the artifact. 

From the code shown in Fig. 6, the Web3j library is able to generate Java classes that are able to communicate with the Ethereum protocol when this code is deployed to the network. An excerpt of the generated code is shown in Fig. 7. 

The generated Java class is then imported into a CArtAgO artifact (Fig. 8). This artifact has a _Bid_ operation, that manages the interaction between the agents and the Blockchain instead of implementing itself the logic of a bidding function. It needs the address in the network where the Smart Contract was deployed to (‘‘ _0x0401..._ ’’), and some supporting variables to control the price it is willing to spend on operations ( _gasPrice_ , _gasLimit_ ). It also needs the access to the Ethereum Node ( _web3_ ), which is a local Http Service. Finally, it needs _credentials_ (password and Ethereum Wallet) to access the Smart Contract. The code in Fig. 8 highlights how the artifact loads the classes generated by Web3j to make the interface between the agents and the network.[7] The logic of the functions is moved from the artifact to the Smart Contract. However, from the perspective of the agents, nothing changes. 

## _3.2. Adding institutions to the system_ 

From the approach proposed in Section 3.1.3, the actions performed by the agents in the environment are recorded in the blockchain. However, these actions are not _per se_ related to transactions of assets. For example, when the agent performs the operation do_bid in the _Auction Artifact_ , a record is stored in the blockchain. However, the proposed framework does not have means to relate this operation, safely recorded in the blockchain, to a _bid_ in an auction. Such meaning depends on the interpretation made by the agents. Problems may arise 

> 7 Full code available at github.com/FerPapi/HouseBuilding_JaCaMoBlockchain. 

> 6 Available at https://web3j.io/. 

6 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [292 x 127] intentionally omitted <==**

**==> picture [249 x 72] intentionally omitted <==**

**Fig. 6.** Excerpt of Smart Contract code. 

**==> picture [286 x 63] intentionally omitted <==**

**==> picture [286 x 43] intentionally omitted <==**

**Fig. 7.** Excerpt of generated Java code. 

if, for example, the agents do not know how to make this interpretation or even if their particular interpretations are different from the real meaning of the fact recorded in the blockchain. 

To fill this gap, SAI is added to the proposed framework. SAI provides institutional meaning to events occurring and to states holding in the environment. Such meaning can thus be assigned (i) to the events produced when operations are triggered in the artifacts where the Smart Contracts are embedded in and (ii) to the observable properties of these artifacts. For example, the artifact whose code excerpt is shown in Fig. 8 could instrument an auction system that includes concepts such as _bid_ and _price_ of the items. These concepts can be introduced in the system as status functions and then constitutive rules may define that (i) the operation do_bid counts as a _bid_ (Fig. 9 - rule 1) and (ii) the observable property current_bid counts as the price of item (Fig. 9 - rule 2).[8] The records stored in the blockchain acquire thus an asset-related meaning that is given by an entity external to the agents (i.e. the institution). Such interpretation does not depend on the agents. Even new agents that join in the system share this same interpretation. 

> 8 In SAI specifications, uppercase starting words mean variables which values are bound to in runtime. 

## **4. Application example** 

Having proposed a framework to design MAS with Blockchain, this section uses an application example to show how this proposal brings to MAS the decentralized, reliable management of transactions of assets. The application scenario is commonly used to illustrate MAS concepts: the construction of a house by different agents. This example, called _build-a-house_ in this paper, extends the original implementation described by Boissier et al. (2013) in three steps. The first extension adds the possibility of agents to pay each other for their job, including transactions of assets in the example. These payments are valid exclusively inside the definition of the MAS, without any regulation (cf. Section 4.2). The second extension includes a new mechanism of payments, that become then contextualized and regulated in the context of an institution (cf. Section 4.3). The third extension integrates the Blockchain to the system (cf. Section 4.4). Thus, the MAS has (i) a SAI based regulation and (ii) artifacts that support transaction of assets via blockchain. This way, it is possible to highlight the problems that arise when implementing solutions that are more simple and naive, comparing them with the solutions and benefits brought by the proposed technologies. 

7 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**Fig. 8.** Excerpt of JaCaMo artifact code. 

**==> picture [146 x 57] intentionally omitted <==**

**==> picture [199 x 66] intentionally omitted <==**

**Fig. 9.** Example of constitutive rules. 

**Fig. 10.** Code excerpt of the play for payments of the agent Giacomo. 

## _4.1. Original example_ 

The _build-a-house_ example is a MAS implemented with the JaCaMo framework (Boissier et al., 2013) that simulates the construction of a house. In this system, an agent called _Giacomo_ aims to have a house built for himself. To this end, he launches auctions to hire companies to do each step of the building (preparing the site, laying floors, building walls, etc.). 

Each company, represented by an agent, participates in the auctions for the tasks they can perform. When the auctions are finished, the companies that have placed the lowest bid for each task are hired to perform the corresponding tasks. To simulate the performance of the building tasks, the agents execute operations in a _house_ simulator artifact. These operations are coordinated by a oise organization, that defines a logical workflow (e.g. preparing the site before laying the floor, which must precede the building of the walls, that on its turn, must be done before constructing the roof). The house is considered built when all the tasks have been performed. 

## _4.2. Adding payments to the example_ 

The original example does not envisage payments for the companies that work in the house building. For better illustrating the transactions of assets, we introduce this aspect in the example. At the beginning, Giacomo has a belief about his available balance (e.g. balance(1500)). He must pay the companies as soon as they finish their tasks. To this end, he simply subtracts the agreed value from his balance belief and informs the company about the payment. The company, on its turn, just adds that amount to its balance belief, believing it just 

**==> picture [129 x 37] intentionally omitted <==**

**Fig. 11.** Code excerpt of a company agent. 

got richer. Fig. 10 shows an excerpt of the plan executed by the agent Giacomo to pay for a Task.[9] First, he consults his belief base to check what is the artifact that has implemented the auction for the Task (line 2). Then, he checks who is the Winner of the auction for that Task as well as the dealt Price (lines 3 and 4). After that, he checks his current balance (line 5) and updates this belief discounting the price to be paid for the Task (line 6). Finally, he informs the Winner about the payment (line 7). The _tell_ performative in the message means that Giacomo has the belief service_paid(Me, Price) (Labrou and Finin, 1998). The receiver trusts Giacomo and acquires this same belief, updating the belief about its own balance (cf. Fig. 11). 

This implementation of payments is very naive and has some problems. The asset, which will be called money in a very loose sense from now on, is represented exclusively in the beliefs of the agents. Giacomo could have any value as his initial available balance since this is coded directly in the agent. He could also be designed to be malicious and not discount the agreed value from his belief, thus creating a 

> 9 In Jason, uppercase starting words mean variables which values are bound to in runtime. 

8 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

double-spending problem from the point of view of other agents. Since the payment consists simply of an information issued by Giacomo, the companies must trust that the informed value, which they add to their balance belief, is correct. These problems make clear that it is appropriate that the notion of assets not be represented only in the minds of the agents. Rather, it should have some representation that is external and independent of any agent. 

## _4.3. Institutionalizing payments_ 

From the previous, naive implementation of payments, this section evolves to a new implementation where the abstract notions related to the transactions and ownership of assets are supported by an _institution_ . This implementation advances in two points: (i) the representations related to the transactions of assets are external to the minds of the agents and (ii) there is an institution that assigns the proper meanings to these external representations. 

To externally represent the elements related to the transaction of assets, a _BankArtifact_ loosely models a real world bank. This artifact has a hash map of {key: value} pairs that represent the accounts of the agents at the bank. The observable property current_value(Agent,Value) shows the Value in the account of each Agent.[10] The agents are able to perform the following operations on this artifact: 

- makeAccount(Agent,Value). The bank creates an entry on the hash map with the name of the Agent as key, having a corresponding initial Value; 

- depositValue(Value). The bank adds the Value to the account of the client; 

- checkValue. The bank returns the available balance in the account of the agent that makes the requisition; 

- transferValue(Value, Receiver). The bank transfers a Value from the account of the agent that triggers the operation to the account of a Receiver agent. The value is subtracted from the sender and added to the receiver account. The bank performs a minimal security checking on this operation, verifying if there is an account for the receiver of the transaction, and if the sender agent has enough funds on his account to complete the transaction. 

Despite their names, these operations are not related, by themselves, with transactions of assets. For example, if a sender agent requests a transferValue operation for some receiver, both the sender and the receiver can observe a new return value of the checkValue operation. But a connection of this value with the notion of bank balance, and even the connection of the operation transferValue with the notion of _money transfer_ , depends on the interpretation from these agents. Since there is an external representation of the transactions of assets (through the _BankArtifact_ ), it is necessary to take a step further, adding an external entity that provides a unified interpretation – recognized by all the agents and other entities in the system – of the facts occurring in the environment, relating them with the notions of assets, ownership, etc. In our proposed approach, this entity is an _artificial institution_ following the SAI model. 

## _4.3.1. Adding an institution to the example_ 

In this application, it is important that notions such as _balance_ and _payment_ have a unified interpretation shared among all the elements of the system, including non autonomous ones, such as the regulative mechanisms. For example, the norm shown in Fig. 12, following the NPL normative model (Hübner et al., 2011), expresses that when the building of walls is done, the house owner is obliged to pay the value dealt in the auction for walls to the agent that is the bricklayer if this 

> 10 The artifact may implement privacy policies, such as hiding this observable property from agents other than the owner of the account. 

value is lower than his balance.[11] Similar norms can be issued for all the tasks of the house building process. To follow the norm, Giacomo needs to know how the concepts referred by the norm (walls building done, balance, role played, dealt value for tasks, and payment) are realized in the environment. The very normative monitor (i.e. the mechanism the checks whether the norm is fulfilled, violated, etc.) needs to connect the concepts used in the norms to the environmental elements. 

In SAI, norms refer to constituted status functions instead of referring directly to the environment. Since the norm shown in Fig. 12 is inserted in SAI, done(Task), balance(Owner,Value), play(Agent,Role), task_value(Task,Value), and pay(Receiver,Value) are status functions. Thus, for the norm to make sense, there must be something in the environment counting as done(Task), balance(Owner,Value), and so on. Fig. 13 illustrates an excerpt of the constitutive specification for this example. The constitutive rule 1 specifies that the observable property current_value(Agent,Value) of the _BankArtifact_ constitutes the state-status function balance(Agent,Value). From the institutional perspective, the holding of such observable property counts as Agent having some Value in his balance. The constitutive rule 2 specifies that the event _transferValue(Value,Receiver)_ produced by the agent Sender constitutes the event-status function pay(Receiver,Value). From the institutional perspective, the occurrence of such event counts as a payment of a Value to the Receiver.[12] 

SAI thus provides a shared, explicit, and unified interpretation about abstract notions, connecting them to the concrete elements composing the system. This is similar to what happens in human societies where, for instance, institutions support the connection between the notion of _money_ and paper bills or coins. While the environment, composed of artifacts, supports transactions of assets that are independent of the agents, SAI provides an institutional interpretation about the facts occurring in the environment that is also independent of the agents. However, the limitation of money being valuable exclusively in the context of this particular MAS is still holding. In this example, it is possible to observe that the agents are no longer required to trust in Giacomo to keep their balances. Instead, the _BankArtifact_ adds a layer of trust to the system, independent of any agent. However, the agents are still required to trust that it works properly, even if they do not know who designed and launched it. Adding the blockchain as a ‘‘door to the outside world’’ helps solve some of these issues, and this will be discussed over the next section. 

## _4.4. The Blockchain integration_ 

Even if the _BankArtifact_ and the SAI provide external, meaningful representations related to the transactions of assets, this approach falls short in actually giving to the agents a support for these transactions. A first issue is that the register of the transactions is kept for only while the system is executing, since there is no connection to any external system that works as a database. Besides, agents are required to trust the _BankArtifact_ even if they do not have the guarantee of its reliability to record transactions of assets. Even if the system includes persistence and reliable level of service, centralization is still a problem. Possible problems are attacks to the server (or servers) where the system runs, malicious changes in the database entries, etc. A final issue is that 

> 11 oise coordination is based on a complex set of concepts, such as _roles_ , _missions_ , _goals_ , etc. However, for simplicity, this paper abstracts this complexity, considering that this regulation is based on NPL norms, that are actually the subjacent regulative representation of oise. In NPL, as in SAI and Jason, uppercase starting words are variables. 

> 12 It is assumed that agents and other entities in the system agree on the meaning behind the status functions (e.g. _balance_ is a certain value of available money, _pay_ is the action of give money in return for something, etc.). Ontological aspects of artificial institutions are beyond the scope of this paper. 

9 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [288 x 45] intentionally omitted <==**

**Fig. 12.** Example of norm. 

**==> picture [293 x 141] intentionally omitted <==**

**Fig. 13.** Example of constitutive rules. 

the money that Giacomo uses to pay for the construction of the house means something only in the context of this particular MAS, because it is in this context that the SAI makes it count as payment. The company agents cannot, for instance, spend their received money in the context of another MAS. 

These issues can be solved by adding the Blockchain as the means of payment in the system. Recalling the concepts introduced in Section 2.2, the blockchain is a persistent and fraud-proof database for transactions of assets. It provides reliable consensus and can connect the MAS with other systems regarding the assets owned by the agents. Following the approach described in Section 3.1.2, a Smart Contract is embedded in the _BankArtifact_ . The Smart Contract has the same operations and observable properties as those available in the _BankArtifact_ , and it behaves the same. The _BankArtifact_ becomes thus an interface that connects the agents and the Institution in this MAS to the functions available in the Smart Contract. Giacomo can still use this same artifact to transfer values to the accounts of the companies, generating the same events in the environment as those of the previous implementation. SAI, on its turn, will make these same events to count as payments. The current implementation of the agents and the SAI does not need to change, but the _BankArtifact_ will need to be designed to interact with the Smart Contract. 

The Smart Contract, whose code is partially shown in Fig. 14, implements the same logic as the _BankArtifact_ previously described. It also contains a hash map that records {key: value} pairs to keep track of transactions and agents’ balances. Furthermore, it has functions to make deposits in the accounts, transfer values to other accounts and consult balance amounts. The compilation of the Smart Contract produces a bytecode that the Web3j library is able to read and generate Java classes that can communicate, via JSON-RPC, with the code that was deployed to the network. These classes are then imported in the interface artifact, as explained in Section 3.1.3. This artifact provides the interface with the operations on the Smart Contract. In this implementation, it also manages the local Ethereum account which, in turn, is the effective owner of the Smart Contract (from the point of view of the Ethereum Network). Fig. 15 shows a simplified schematic of this interfacing between Ethereum’s network and the MAS, similar to Fig. 5. 

The agents can act on this interface to manage their assets. For example, to pay for the hired services, Giacomo can act on this interface, which will make their requests on the network. Since the 

artifact manages the local Ethereum account, knows its password and knows the address of the Smart Contract in the blockchain, it is able to read the state of the Contract when it restarts if the system stops being executed. The persistence and reliability of this information is guaranteed, as explained on Section 2.2. All that the MAS needs is to access an Ethereum node synchronized to the network, which can be executed locally or remotely.[13] 

The execution of the operations in _BankArtifact_ Interface, as well as the observable properties of this artifact, are handled by the SAI constituting the status functions according to the constitutive rules. Thus, the system, in this example, is complete: the transactions performed through the _BankArtifact_ are persistent and safely recorded, being also endowed with meanings related to transactions of assets, assigned by the institution. 

## **5. Discussion** 

The first questions answered in this work refer (i) to the possibility to integrate Blockchain with MAS and (ii) to the role of the Blockchain in such integration (cf. Section 1). The developed work shows that it is possible to integrate Blockchain with MAS. From a conceptual perspective, it is possible to conclude that a proper role for Blockchain in MAS is as a component of the environment — among other ones — to cover only critical actions that need integrity of records, reliable access to critical information, modeling of assets and monetary values, promises, contracts and so on. From a technological perspective, we illustrate a possible model of integration of Blockchain with MAS by embedding Smart Contracts in CArtAgO artifacts. However, there is a trade-off for the advantages brought by Blockchains. This technology is very slow in terms of computing speed and could become quite expensive to have complex code running on it for large systems. 

Besides Blockchains, there are some remarks on performance scalability regarding SAI. The proposed framework integrates a SAI engine with the CArtAgO implementation, which manages multiple, possibly concurrent operations triggered by the agents upon the artifacts. From this environmental dynamics, SAI produces a single sequence of institutional states. This point may be a performance bottleneck since 

> 13 This Smart Contract has been deployed a test network of Ethereum called Kovan and the performed transactions can be checked at https://kovan. etherscan.io/address/0xfd9acf15dea1a1ec63bd04f7bc9b0cac2b407441#. 

10 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

**==> picture [267 x 167] intentionally omitted <==**

**==> picture [250 x 43] intentionally omitted <==**

**==> picture [331 x 178] intentionally omitted <==**

**Fig. 14.** Code excerpt of smart contract. 

**==> picture [305 x 101] intentionally omitted <==**

**Fig. 15.** Bank Artifact Interfacing Ethereum’s network. 

SAI manages simultaneous environmental facts as a sequence (de Brito et al., 2018). These facts may occur in any MAS but become more probable when the number of agents increases. 

The proposed framework integrates several technologies whose performance is affected by many variables (e.g. number, frequency, and complexity of asset transactions, number of constitutive rules triggered by the asset transactions, complexity of the different constitutive rules 

triggered by the different asset transactions, recording of transactions in the Blockchain, network performance, etc.). As mentioned previously, on average it takes about 5 min to have a transaction validated on the Ethereum network, so packing more blockchain state changes into one contract would be a smart design choice. Even though Ethereum was used in this work, there are alternative smart contract blockchain platforms, such as Solana, launched in 2019 (Painter et al., 2021), and 

11 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

Tezos, launched in 2018 (Allombert et al., 2019), that aim for faster and more scalable transactions. A performance analysis considering all these points is a future work since it requires an extensive and careful analysis of the results of experiments designed for this purpose. 

Another question answered in this work refers to the possibility of representing asset-related notions in MAS. From the developed work, we can say that, yes, it is possible, but not without some further considerations. Concepts such as _asset_ and _ownership_ are intangible and, in the human societies, depend on institutions to provide them a unified meaning related to the concrete world (Searle, 1995, 2009). SAI provides a similar process of _institutionalization_ in MAS and allows us to connect some of these intangible concepts to facts occurring in the environment where the agents act. This is described in Section 3.2 and illustrated in Section 4.3. However, it can be argued that the institutionalization of assets is done only in an indirect way. SAI can institutionalize events and states related to the transactions of assets but it cannot institutionalize the _assets_ themselves. The example in Section 4.3 shows the institutionalization of _payment_ and _balance_ but there is not an explicit connection between some environmental element and the notion of _money_ . This limitation is due to the constitution provided by the SAI model, that in its current state of development envisages institutional interpretation only for agents acting, events occurring, and states holding in the environment. This limitation also applies to different constitutive models currently proposed, that do not have particular abstractions for environmental objects, that could constitute _money_ .[14] Institutional models other than SAI, in general, do not even consider the typing of status functions. While they can enable the creation of a larger set of status functions, they lose consistency with environmental and regulative elements of the system (de Brito et al., 2018). 

Despite these limitations, SAI provides scalability with respect to constituted asset-related concepts. For simplicity, the examples presented in this paper illustrate a limited set of these possible concepts. However, this limited set can be easily extended to abstract new asset-related events such as _bids_ , _offers_ , _donations_ , _thefts_ , etc. and assetrelated states such as _asset ownership_ , _negotiation stages_ , etc. Besides, the operators _𝑡_ and _𝑚_ of constitutive rules (represented by the keywords when and while in SAI specifications) make it possible to condition the constitution of asset-related concepts to complex combinations of environmental events, environmental states, and their institutional counterparts (i.e. event-status functions and state-status functions). Finally, the implementation of CArtAgO artifacts and smart contracts can include more complex features of transaction management. 

With these answers in mind, we realized that Blockchain is a valuable addition to MAS as a support for the transaction of assets. Such transactions become reliable, decentralized, transparent, and flexible. This can become particularly useful in complex systems that involve commerce and transaction of values, decentralization of information and that are not particularly dependent on speed. Moreover, the integration with the SAI proves to be a successful way to assign, in some extent at least, meaning related to transactions of assets, ownership, etc. to the records in the Blockchain. This is specially important to make explicit to the agents how these abstract concepts are connected to the concrete environment where they act, as well as to provide a unified interpretation about these concepts, to be adopted not only by the agents but also by the regulative mechanisms. Thus, it is possible to say that the adoption of a Blockchain to the proposed problem has successfully added data integrity, traceability, transparency, and authenticity to the system. 

> 14 An analysis of the state of the art on institutional models in MAS can be found in de Brito et al. (2018). 

## **6. Related work** 

Existing works mostly use the Blockchain as a technological support of general purpose in MAS, providing a secure, distributed database to support, for example, agent communication (Kapitonov et al., 2017; Kapitonov et al., 2018) and coordination (Mariani et al., 2017; Kapitonov et al., 2018; Ciatto et al., 2018; Bonino and Vergori, 2017; Castelló Ferrer, 2019), negotiation models (Calvaresi et al., 2018), accountability (Papi et al., 2017), anonymization of datasets (Kiyomoto et al., 2017), and market simulation (Brousmichc et al., 2018).[15] Our work exploits Blockchain as a technological support for recording transactions of assets, considering two main aspects: (i) the integration of the Blockchain in the MAS and (ii) the assignment of asset-related meaning to the records in the blockchain. 

Regarding the integration in the MAS, the blockchain can be considered as part of the environment in all the related work – even when this is not explicitly stated – since it is a non autonomous element used by the agents to achieve their goals. However, most work takes this from an exogenous perspective, where the Blockchain is not part of the MAS design. An exception is the work described in Wang et al. (2019), where the used blockchain technology is selected and adapted to comply with some requirements of the application in the scenario of energy trading. However, even in this endogenous perspective, our work is different since the blockchain is enclosed in application artifacts, where the whole integration between the MAS and Blockchain is coded. From the perspective of the agents, they are accessing common environmental artifacts without requiring any additional knowledge to access and manipulate a particular blockchain technology. 

In all the related works, the meaning of the records stored in the Blockchain depends on an interpretation of each agent. Our proposal advances in this point by inserting SAI as an institutional layer over the environment where the Blockchain is embedded. In this framework, the institution is the entity, external to and independent of the agents, that imposes meaning to the records in the blockchain. All that the agents need is to be able to access the SAI infrastructure to be aware about such meaning. 

## **7. Conclusion** 

Computational systems where computer programs trade assets among themselves and even with humans require both reliability in the transactions management and a shared asset-related interpretation of the digital records. In this direction, the model described in this paper proposes to enclose blockchains in environmental artifacts that the computational traders perceive and act upon. The records stored in the blockchain receive an asset-related meaning thanks to the process of institutionalization enabled by an artificial institution. From this integration, computational systems have a reliable database whose records represent transactions of assets. 

Future works include (i) a direct institutionalization of assets to advance from the indirect institutionalization done in this paper; (ii) improvements in the Blockchain integration with MAS, making possible, for example, that different agents have their accounts in particular artifacts instead of using a single Ethereum artifact shared among all the agents in the system; (iii) the implementation of the institution in the Blockchain, so that the institutional state is also recorded in a distributed, reliable, persistent media; and (iv) performance analysis of the proposed framework, possibly including a cross comparison between different smart contracts blockchain options. 

> 15 A structured literature review on MAS and Blockchain, analyzing application scenarios, motivations, assumptions, strengths, limitations, and identifying future challenges, can be found in Calvaresi et al. (2018). 

12 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

## **CRediT authorship contribution statement** 

**Fernando Gomes Papi:** Investigation, Methodology, Software, Validation, Writing – review & editing. **Jomi Fred Hübner:** Conceptualization, Writing – review & editing, Supervision. **Maiquel de Brito:** Conceptualization, Visualization, Writing – original draft, Supervision. 

## **Declaration of competing interest** 

The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **References** 

- Aldewereld, H., Álvarez-Napagao, S., Dignum, F., Vázquez-Salceda, J., 2010. Making norms concrete. In: van der Hoek, W., Kaminka, G.A., Lespérance, Y., Luck, M., Sen, S. (Eds.), Proceedings of the 9th International Conference on Autonomous Agents and Multiagent Systems (AAMAS 2010), Vol. 1-3. IFAAMAS, Richland, SC, pp. 807–814. 

- Allombert, V., Bourgoin, M., Tesson, J., 2019. Introduction to the tezos blockchain. In: 17th International Conference on High Performance Computing & Simulation, HPCS 2019, Dublin, Ireland, July 15-19, 2019. IEEE, pp. 1–10. http://dx.doi.org/ 10.1109/HPCS48598.2019.9188227. 

- Amato, A., Martino, B.D., Scialdone, M., Venticinque, S., 2014. Multi-agent negotiation of decentralized energy production in smart micro-grid. In: Camacho, D., Braubach, L., Venticinque, S., Badica, C. (Eds.), Intelligent Distributed Computing VIII - Proceedings of the 8th International Symposium on Intelligent Distributed Computing, IDC 2014, Madrid, Spain, September 3-5, 2014. In: Studies in Computational Intelligence, Vol. 570, Springer, pp. 155–160. http://dx.doi.org/10.1007/ 978-3-319-10422-5_17. 

- Andrighetto, G., Governatori, G., Noriega, P., van der Torre, L.W.N. (Eds.), 2013. Normative multi-agent systems. In: Normative Multi-Agent Systems. In: Dagstuhl Follow-Ups, Vol. 4, Schloss Dagstuhl - Leibniz-Zentrum fuer Informatik, Dagstuhl, Germany. 

- Antonopoulos, A.M., 2014. Mastering Bitcoin: Unlocking Digital Crypto-Currencies, first ed. O’Reilly Media, Inc.. 

- Aranda-Corral, G.A., Borrego-Díaz, J., Martín, D.S., 2015. _Iabastos_ : An intelligent marketplace for agricultural products. In: Demazeau, Y., Decker, K.S., Pérez, J.B., de la Prieta, F. (Eds.), Advances in Practical Applications of Agents, Multi-Agent Systems, and Sustainability: The PAAMS Collection - 13th International Conference, PAAMS 2015, Salamanca, Spain, June 3-4, 2015, Proceedings. In: Lecture Notes in Computer Science, Vol. 9086, Springer International Publishing, Cham, pp. 255–258. http://dx.doi.org/10.1007/978-3-319-18944-4. 

- Boella, G., Noriega, P., Pigozzi, G., Verhagen, H. (Eds.), 2009. Normative multi-agent systems, 15.03. - 20.03.2009. In: Dagstuhl Seminar Proceedings, Vol. 09121, Schloss Dagstuhl - Leibniz-Zentrum für Informatik, Germany, Dagstuhl, Germany. 

- Boella, G., van der Torre, L.W.N., Verhagen, H. (Eds.), 2007. Normative multi-agent systems, 18.03. - 23.03.2007. In: Dagstuhl Seminar Proceedings, Vol. 07122, Internationales Begegnungs- und Forschungszentrum für Informatik (IBFI), Schloss Dagstuhl, Germany, Dagstuhl, Germany. 

- Boella, G., van der Torre, L., Verhagen, H., 2008. Introduction to the special issue on normative multiagent systems. Auton. Agents Multi-Agent Syst. 17 (1), 1–10. 

- Boissier, O., Bordini, R.H., Hübner, J., Ricci, A., 2020. Multi-Agent Oriented Programming: Programming Multi-Agent Systems Using JaCaMo. MIT Press, URL https://mitpress.mit.edu/books/multi-agent-oriented-programming. 

- Boissier, O., Bordini, R.H., Hübner, J.F., Ricci, A., Santi, A., 2013. Multi-agent oriented programming with JaCaMo. Sci. Comput. Prog. 78 (6), 747–761. http://dx.doi.org/ 10.1016/j.scico.2011.10.004. 

- Bonino, D., Vergori, P., 2017. Agent marketplaces and deep learning in enterprises: The composition project. In: Reisman, S., Ahamed, S.I., Demartini, C., Conte, T.M., Liu, L., Claycomb, W.R., Nakamura, M., Tovar, E., Cimato, S., Lung, C., Takakura, H., Yang, J., Akiyama, T., Zhang, Z., Hasan, K. (Eds.), 41st IEEE Annual Computer Software and Applications Conference, COMPSAC 2017, Turin, Italy, July 4-8, 2017. Volume 1. IEEE Computer Society, pp. 749–754. http://dx.doi.org/10.1109/COMPSAC.2017.46. 

- Bordini, R.H., Hübner, J.F., Wooldridge, M.J., 2007. Programming Multi-Agent Systems in AgentSpeak Using Jason. J. Wiley. 

- Brousmichc, K., Anoaica, A., Dib, O., Abdellatif, T., Deleuze, G., 2018. Blockchain energy market place evaluation: An agent-based approach. In: 2018 IEEE 9th Annual Information Technology, Electronics and Mobile Communication Conference (IEMCON). pp. 321–327. http://dx.doi.org/10.1109/IEMCON.2018.8614924. 

- Calvaresi, D., Dubovitskaya, A., Calbimonte, J.P., Taveter, K., Schumacher, M., 2018. Multi-agent systems and blockchain: Results from a systematic literature review. In: Demazeau, Y., An, B., Bajo, J., Fernández-Caballero, A. (Eds.), Advances in Practical Applications of Agents, Multi-Agent Systems, and Complexity: The PAAMS Collection. Springer International Publishing, Cham, pp. 110–126. 

- Calvaresi, D., Dubovitskaya, A., Retaggi, D., F. Dragoni, A., Schumacher, M., 2018. Trusted registration, negotiation, and service evaluation in multi-agent systems throughout the blockchain technology. In: 2018 IEEE/WIC/ACM International Conference on Web Intelligence (WI). pp. 56–63. http://dx.doi.org/10.1109/WI. 2018.0-107. 

- Castelló Ferrer, E., 2019. The blockchain: A new framework for robotic swarm systems. In: Arai, K., Bhatia, R., Kapoor, S. (Eds.), Proceedings of the Future Technologies Conference (FTC) 2018. Springer International Publishing, Cham, pp. 1037–1058. 

- Chauhan, A., Malviya, O.P., Verma, M., Mor, T.S., 2018. Blockchain and scalability. In: 2018 IEEE International Conference on Software Quality, Reliability and Security Companion, QRS Companion 2018, Lisbon, Portugal, July 16-20, 2018. IEEE, pp. 122–128. http://dx.doi.org/10.1109/QRS-C.2018.00034. 

- Ciatto, G., Mariani, S., Omicini, A., 2018. Blockchain for trustworthy coordination: A first study with LINDA and ethereum. In: 2018 IEEE/WIC/ACM International Conference on Web Intelligence, WI 2018, Santiago, Chile, December 3-6, 2018. IEEE Computer Society, pp. 696–703. http://dx.doi.org/10.1109/WI.2018.000-9. 

- Cuní, G., Esteva, M., Garcia, P., Puertas, E., Sierra, C., Solchaga, T., 2004. MASFIT: multi-agent system for fish trading. In: de Mántaras, R.L., Saitta, L. (Eds.), Proceedings of the 16th Eureopean Conference on Artificial Intelligence, ECAI’2004, Including Prestigious Applicants of Intelligent Systems, PAIS 2004, Valencia, Spain, August 22-27, 2004. IOS Press, pp. 710–714. 

- de Brito, M., Hübner, J.F., Boissier, O., 2018. Situated artificial institutions: stability, consistency, and flexibility in the regulation of agent societies. Auton. Agents Multi-Agent Syst. 32 (2), 219–251. http://dx.doi.org/10.1007/s10458-017-9379-3. 

- de Brito, M., Hübner, J.F., Boissier, O., 2019. Coupling the normative regulation with the constitutive state management in situated artificial institutions. Knowl. Eng. Rev. 34, e21. http://dx.doi.org/10.1017/S026988891900016X. 

- Dominguez, R., Cannella, S., 2020. Insights on multi-agent systems applications for supply chain management. Sustainability 12 (5), 1935. http://dx.doi.org/10.3390/ su12051935. 

- Farjam, M., Kirchkamp, O., 2015. Bubbles in hybrid markets - how expectations about algorithmic trading affect human trading. (5631), CESifo Group Munich. 

- Hübner, J.F., Boissier, O., Bordini, R.H., 2011. A normative programming language for multi-agent organisations. Ann. Math. Artif. Intell. 62 (1–2), 27–53. http: //dx.doi.org/10.1007/s10472-011-9251-0. 

- Hübner, J.F., Sichman, J.S.a., Boissier, O., 2007. Developing organised multiagent systems using the MOISE+ model: Programming issues at the system and agent levels. IJAOSE 1 (3/4), 370–395. 

- Ito, T., Abadi, S.M.M.J., 2002. Agent-based material handling and inventory planning in warehouse. J. Intell. Manuf. 13 (3), 201–210. http://dx.doi.org/10.1023/A: 1015786822825. 

- Kapitonov, A., Berman, I., Bulatov, V., Lonshakov, S., Krupenkin, A., 2018. Robonomics based on blockchain as a principle of creating smart factories. In: 2018 Fifth International Conference on Internet of Things: Systems, Management and Security. pp. 78–85. http://dx.doi.org/10.1109/IoTSMS.2018.8554864. 

- Kapitonov, A.A., Berman, I., Lonshakov, S., Krupenkin, A., 2018. Blockchain based protocol for economical communication in industry 4.0. In: Crypto Valley Conference on Blockchain Technology, CVCBT 2018, Zug, Switzerland, June 20-22, 2018. IEEE, pp. 41–44. http://dx.doi.org/10.1109/CVCBT.2018.00010. 

- Kapitonov, A., Lonshakov, S., Krupenkin, A., Berman, I., 2017. Blockchain-based protocol of autonomous business activity for multi-agent systems consisting of UAVs. In: 2017 Workshop on Research, Education and Development of Unmanned Aerial Systems (RED-UAS). pp. 84–89. http://dx.doi.org/10.1109/RED-UAS.2017. 8101648. 

- Kiyomoto, S., Rahman, M.S., Basu, A., 2017. On blockchain-based anonymized dataset distribution platform. In: 2017 IEEE 15th International Conference on Software Engineering Research, Management and Applications (SERA), pp. 85–92. 

- Labrou, Y., Finin, T., 1998. Semantics for an agent communication language. In: Singh, M.P., Rao, A., Wooldridge, M.J. (Eds.), Intelligent Agents IV Agent Theories, Architectures, and Languages. Springer Berlin Heidelberg, Berlin, Heidelberg, pp. 209–214. 

- Langley, P., Laird, J.E., Rogers, S., 2009. Cognitive architectures: Research issues and challenges. Cogn. Syst. Res. 10 (2), 141–160. http://dx.doi.org/10.1016/j.cogsys. 2006.07.004. 

- Luck, M., McBurney, P., Shehory, O., Willmott, S., 2005. Agent Technology: Computing as Interaction (A Roadmap for Agent Based Computing). University of Southampton. 

- Mariani, S., Omicini, A., Ciatto, G., 2017. Novel opportunities for tuple-based coordination: Xpath, the blockchain, and stream processing. In: Meo, P.D., Postorino, M.N., Rosaci, D., Sarné, G.M.L. (Eds.), Proceedings of the 18th Workshop "from Objects To Agents", Scilla (RC), Italy, June 15-16, 2017. In: CEUR Workshop Proceedings, Vol. 1867, CEUR-WS.org, pp. 61–64. 

- Narayanan, A., Bonneau, J., Felten, E., Miller, A., Goldfeder, S., 2016. Bitcoin and Cryptocurrency Technologies: A Comprehensive Introduction. Princeton University Press, Princeton, NJ, USA. 

- Omicini, A., Ricci, A., Viroli, M., 2008. Artifacts in the A&A meta-model for multi-agent systems. Auton. Agents Multi-Agent Syst. 17 (3), 432–456. 

- Painter, Z., Cook, V., Peterson, C.L., Dechev, D., 2021. Descriptor based consensus for blockchain transactions. In: Margara, A., Valle, E.D., Artikis, A., Tatbul, N., Parzyjegla, H. (Eds.), DEBS ’21: The 15th ACM International Conference on Distributed and Event-Based Systems, Virtual Event, Italy, June 28 - July 2, 2021. ACM, pp. 114–125. http://dx.doi.org/10.1145/3465480.3466927. 

13 

_F.G. Papi, J.F. Hübner and M. de Brito_ 

_Engineering Applications of Artificial Intelligence 107 (2022) 104534_ 

- Papi, F.G., Hübner, J.F., de Brito, M., 2017. Instrumenting accountability in MAS with blockchain. In: Baldoni, M., Baroglio, C., Micalizio, R. (Eds.), Proceedings of the First Workshop on Computational Accountability and Responsibility in Multiagent Systems Co-Located with 20th International Conference on Principles and Practice of Multi-Agent Systems, CARe-MAS@PRIMA 2017, Nice, France, October 31st, 2017. In: CEUR Workshop Proceedings, Vol. 2051, CEUR-WS.org, pp. 20–34. 

- Pinto, A., Silva, J., 2020. Revisiting blockhain use in notary services: An European perspective. In: Prieto, J., Pinto, A., Das, A.K., Ferretti, S. (Eds.), Blockchain and Applications. Springer International Publishing, Cham, pp. 101–110. 

- Rao, A.S., 1996. Agentspeak(l): BDI agents speak out in a logical computable language. In: de Velde, W.V., Perram, J.W. (Eds.), Agents Breaking Away, 7th European Workshop on Modelling Autonomous Agents in a Multi-Agent World, Eindhoven, the Netherlands, January 22-25, 1996, Proceedings. In: Lecture Notes in Computer Science, Vol. 1038, Springer, pp. 42–55. http://dx.doi.org/10.1007/BFb0031845. 

- Ricci, A., Piunti, M., Viroli, M., 2011. Environment programming in multi-agent systems: an artifact-based perspective. Auton. Agents Multi-Agent Syst. 23 (2), 158–192. http://dx.doi.org/10.1007/s10458-010-9140-7. 

- Searle, J., 1995. the Construction of Social Reality. Free Press. 

- Searle, J., 2009. Making the Social World: the Structure of Human Civilization. Oxford University Press. 

- Stroukal, D., 2018. Can bitcoin become money? Its money functions and regression theorem. Int. J. Bus. Manag. 1 (1), 36–53. http://dx.doi.org/10.20472/BM.2018.6. 1.004. 

- Swan, M., 2015. Blockchain: Blueprint for a New Economy. O’Reilly Media. 

- Szydlo, M., 2004. Merkle tree traversal in log space and time. In: Cachin, C., Camenisch, J.L. (Eds.), Advances in Cryptology - EUROCRYPT 2004. Springer Berlin Heidelberg, Berlin, Heidelberg, pp. 541–554. 

- Truong, N.B., Sun, K., Guo, Y., 2019. Blockchain-based personal data management: From fiction to solution. In: Gkoulalas-Divanis, A., Marchetti, M., Avresky, D.R. (Eds.), 18th IEEE International Symposium on Network Computing and Applications, NCA 2019, Cambridge, MA, USA, September 26-28, 2019. IEEE, pp. 1–8. http://dx.doi.org/10.1109/NCA.2019.8935049. 

- Von Mises, L., 1966. Human Action: A Treatise on Economics. Liberty Fund. 

- Vos, M.D., Balke, T., Satoh, K., 2013. Combining event-and state-based norms. In: Gini, M.L., Shehory, O., Ito, T., Jonker, C.M. (Eds.), International Conference on Autonomous Agents and Multi-Agent Systems, AAMAS ’13, Saint Paul, MN, USA, May 6-10, 2013. International Foundation for Autonomous Agents and Multiagent Systems, Richland, SC, pp. 1157–1158. 

- Wang, S., Taha, A.F., Wang, J., Kvaternik, K., Hahn, A., 2019. Energy crowdsourcing and peer-to-peer energy trading in blockchain-enabled smart grids. IEEE Trans. Syst. Man Cybern. Syst. 49 (8), 1612–1623. http://dx.doi.org/10.1109/TSMC.2019. 2916565. 

- Wood, G., et al., 2014. Ethereum: A secure decentralised generalised transaction ledger. Ethereum Proj. Yellow Pap. 151 (2014), 1–32. 

- Wooldridge, M.J., 2009. An Introduction To MultiAgent Systems, Second Edition. Wiley. Wooldridge, M., Jennings, N., 1995. Intelligent agents: Theory and practice. Knowl. Eng. Rev. 10 (2), 115–152. 

14 

