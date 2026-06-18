2018 Crypto Valley Conference on Blockchain Technology 

## Do need a Blockchain? you 

## Karl Wüst 

Department of Computer Science ETH Zurich karl.wuest@inf.ethz.ch 

## Arthur Gervais 

Department of Computing Imperial College London a.gervais@imperial.ac.uk 

_**Abstract**_ **—Blockchain is being praised as a technological innovation which allows to revolutionize how society trades and interacts. This reputation is in particular attributable to its properties of allowing mutually mistrusting entities to exchange financial value and interact without relying on a trusted third party. A blockchain moreover provides an integrity protected data storage and allows to provide process transparency.** 

**In this paper we critically analyze whether a blockchain is indeed the appropriate technical solution for a particular application scenario. We differentiate between permissionless (e.g., Bitcoin/Ethereum) and permissioned (e.g. Hyperledger/Corda) blockchains and contrast their properties to those of a centrally managed database. We provide a structured methodology to determine the appropriate technical solution to solve a particular application problem. Given our methodology, we analyze in depth three use cases — Supply Chain Management, Interbank and International Payments, and Decentralized Autonomous Organizations and conclude the article with an outlook for further opportunities.** 

## I. INTRODUCTION 

Bitcoin and its blockchain have allowed mutually mistrusting entities to perform financial payments without relying on a central trusted third party while offering a transparent and integrity protected data storage [1]. Due to these properties, blockchain as a technology has gained much attention beyond the purpose of financial transactions – distributed cloud storage, smart property, Internet of Things, supply chain management, healthcare, ownership and royalty distribution, and decentralized autonomous organizations just to name a few. 

Contrary to Bitcoin’s _permissionless_ blockchain, where any writer and reader can join at any time, so-called _permissioned_ blockchains have been proposed, where only an authorized set of entities is allowed to write and read the respective blockchain. A permissioned blockchain, however, shares similarities with a centralized database, and this naturally brings up the question whether a blockchain is better suited than a centralized database. 

In this work, we analyze the properties of different blockchain types (i.e. permissioned and permissionless) and contrast these properties to those of a centrally managed database. We provide a methodology to identify whether a blockchain is useful depending on the problem requirements, and if so, what type of blockchain might be appropriate. Based on our methodology, we evaluate in detail three use cases, namely _(i)_ supply chain management, _(ii)_ interbank and international payments and _(iii)_ decentralized autonomous 

organizations and argue if and which blockchain type make sense for the specific applications. 

The remainder of this article is organized as follows. In Section II, we briefly describe the most important background about blockchain. In Section III we provide a structured methodology to identify if a blockchain makes sense, and if yes, which type of blockchain would be appropriate. Based on our methodology, we analyze proposed use cases in detail in Section IV. In Section V, we review related work in the area, and we conclude the article in Section VI. 

## II. BACKGROUND ON BLOCKCHAIN 

In the following section, we detail the required blockchain background and the involved parties. The name blockchain stems from its technical structure — a chain of blocks. Each block is linked to the previous block with a cryptographic hash. A block is a datastructure which allows to store a list of transactions. Transactions are created and exchanged by peers of the blockchain network and modify the state of the blockchain. As such, transactions can exchange monetary amounts, but are not restricted to financial transactions only and for example allow to execute arbitrary code within socalled smart contracts. 

Before diving into the specific differences of permissionless and permissioned blockchains, we now describe the different participants of these networks. As applicable to any database system, we denote as _writer_ any entity which writes state to the database. In a blockchain this would correspond to a participant that is involved in the consensus protocol and helps growing the blockchain. As such, a writer is able to accumulate transactions within a block and append this block to the blockchain. Related work might also denominate a writer as a validator. We denote a _reader_ as any entity which is not extending the blockchain, but participating in either the transaction creation process, simply reading and analysing or auditing the blockchain. Note that we consider regulators and blockchain software maintainers to be outside of this scope. 

**Permissionless Blockchains** Bitcoin [1] and Ethereum [2] 

are instances of permissionless blockchains, which are open and decentralized. Any peer can join and leave the network as reader and writer at any time. Interestingly, there is no central entity which manages the membership, or which could ban illegitimate readers or writers. This openness implies that the written content is readable by 

978-1-5386-7204-4/18/$31.00 ©2018 IEEE DOI 10.1109/CVCBT.2018.00011 

45 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

   - any peer. With the use of cryptographic primitives however, it is technically feasible to design a permissionless blockchain which hides privacy relevant information (e.g. Zerocash[3]). 

- **Permissioned Blockchains** To only authorize a limited set of readers and writers, so called-permissioned blockchains have been recently proposed. Here, a central entity decides and attributes the right to individual peers to participate in the write or read operations of the blockchain. To provide encapsulation and privacy, reader and writer could also run in separated parallel blockchains that are interconnected. The most widely known instance of permissioned blockchains are Hyperledger Fabric and R3 Corda [4]. 

## _A. Properties_ 

In the following, we describe and compare the most relevant properties that distributed ledgers and centralized systems provide. 

- **Public Verifiability** allows anyone to verify the correctness of the state of the system. In a distributed ledger, each state transition is confirmed by verifiers (e.g. miners in Bitcoin), which can be a restricted set of participants. Any observer, however, can verify that the state of the ledger was changed according to the protocol and all observers will eventually have the same view of the ledger, at least up to a certain length. In a centralized system, different observers may have entirely different views of the state. As such, they might not be able to verify that all state transitions were executed correctly. Instead, observers need to trust the central entity to provide them with the correct state. 

- **Transparency** of the data and the process of updating the state is a requirement for public verifiability. The amount of information that is transparent to an observer, however, can differ, and not every participant needs to have access to every piece of information. 

- **Privacy** is an important property of any system. There exists an inherent tension between privacy and transparency. Privacy is certainly easier to achieve in a centralized system because transparency and public verifiability are not required for the functioning of the system. 

- **Integrity** of information ensures that information is protected from unauthorized modifications, i.e. that retrieved data is correct. The integrity of information is closely linked to public verifiability. If a system provides public verifiability, anyone can verify the integrity of the data; integrity can otherwise only be ensured if the centralized system is not compromised. 

- **Redundancy** of data is important for many use cases. In blockchain systems, redundancy is inherently provided through replication across the writers. In centralized systems, redundancy is generally achieved through replication on different physical servers and through backups. 

- **Trust Anchor** defines who represents the highest authority of a given system that has the authority to grant and revoke read and write access to a system. 

## _B. Tensions between Transparency and Privacy_ 

There exist an inherent tradeoff between transparency and privacy. A fully transparent system allows anyone to see any piece of information, i.e. no privacy is provided. Likewise, a fully private system provides no transparency. However, a system can still provide significant privacy-guarantees while making the process of state transitions transparent, e.g. a distributed ledger can provide public verifiability of its overall state without leaking information about the state of each individual participant. Privacy in a public system can be achieved using cryptographic techniques but typically comes at the cost of lower efficiency. The cryptocurrency Zerocash [3] for example makes use of computationally expensive cryptography to provide full anonymity while still providing sufficient transparency to publicly verify the ledger state. 

## III. WHERE DOES A BLOCKCHAIN MAKE SENSE? 

In general, using an open or permissioned blockchain only makes sense when multiple mutually mistrusting entities want to interact and change the state of a system, and are not willing to agree on an online trusted third party. 

To ease the decision making process, we provide a flow chart in Figure 1. We consider one or multiple parties that write the system state, i.e. a writer corresponds to an entity with write access in a typical database system or to consensus participant in a blockchain system. 

If no data needs to be stored, no database is required at all, i.e. a blockchain, as a form of database, is of no use. Similarly, if only one writer exists, a blockchain does not provide additional guarantees and a regular database is better suited, because it provides better performance in terms of throughput and latency. If a trusted third party (TTP) is available, there are two options. First, if the TTP is always online, write operations can be delegated to it and it can function as verifier for state transitions. Second, if the TTP is usually offline, it can function as a certificate authority in the setting of a permissioned blockchain, i.e. where all writers of the system are known. If the writers all mutually trust each other, i.e. they assume that no participant is malicious, a database with shared write access is likely the best solution. If they do not trust each other, using a permissioned blockchain makes sense. Depending on whether public verifiability is required, anyone can be allowed to read the state (public permissioned blockchain) or the set of readers may also be restricted (private permissioned blockchain). If the set of writers is not fixed and known to the participants, as is the case for many cryptocurrencies such as Bitcoin, a permissionless blockchain is a suitable solution. 

In Table I we contrast some properties of permissionless and permissioned blockchains, and a central database. In a centralized systems, the performance in terms of latency and throughput is generally much better than in blockchain systems, as blockchains add additional complexity through their 

46 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

**==> picture [479 x 314] intentionally omitted <==**

**----- Start of picture text -----**<br>
Do you need yes Are there yes Can you use no Are all no Permissionless<br>multiple an always writers<br>to store state? Blockchain<br>writers? online TTP? known?<br>no no yes yes<br>Are all Is public Public<br>no yes<br>writers verifiability Permissioned<br>trusted? required? Blockchain<br>yes no<br>Private<br>Permissioned<br>Blockchain<br>Don’t use<br>Blockchain<br>**----- End of picture text -----**<br>


Fig. 1: A flow chart to determine whether a blockchain is the appropriate technical solution to solve a problem (Table I should be considered in the decision making process as well). Writers refer to entities with write access to the database/blockchain, i.e. in a blockchain setting, a writer corresponds to a consensus participant. If a trusted third party (TTP) is available that is not always online, this can be used to establish a known group of writers, i.e. the TTP can function as a certificate authority in such a setting. Public and private permissioned blockchains differ in that a public blockchain allows anyone to read the contents of the chain and thus verify the validity of the stored data, while a private blockchain only allows a limited number of participants to read the chain. Note that for any blockchain based solution it is possible to make use of cryptographic primitives in order to hide privacy-relevant content. 

consensus mechanism. For example, Bitcoin can currently only sustain a throughput of approximately seven transactions per second (which could be extended to approximately 66 without compromising security [6]), while a centralized system such as Visa can handle peaks of more than fifty thousand transactions. There is a tradeoff between decentralization, i.e. how well a system scales to a large number of writers without mutual trust, and throughput, i.e. how many state updates a system can handle in a given amount of time. When making the decision of whether to use a blockchain system or not, this tradeoff should be taken into account as well. 

## IV. CASE BY CASE 

In the following Section, we outline several use cases where industrial efforts are advertising to use blockchain technology, two of which – supply chain management and interbank payments – we will discuss in detail. Where possible, we evaluate objectively how a blockchain solution might make 

sense and what the technical, security and privacy implications would be. 

## _A. Supply Chain Management_ 

In Supply Chain Management (SCM), the flow of materials and services required in manufacturing a given product is managed, which includes various intermediate storage and production cycles until the delivery to the final point of consumption. Typically, multiple companies interact and trade on a global scale within a given supply chain. Due to this complexity, associated costs of managing the inventory, processes and failure detection are particularly expensive. 

Several companies (e.g. Skuchain [7], Provenance [8], Walmart [9], Everledger [10]) advertise to provide blockchain based solutions to improve the efficiency of supply chain management solutions. Some even claim that blockchain technology paves the way to _demand_ instead of _supply_ chains, 

47 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

||Permissionless Blockchain|Permissioned Blockchain|Central Database|
|---|---|---|---|
|Throughput|Low|High|Very High|
|Latency|Slow|Medium|Fast|
|Number of readers|High|High|High|
|Number of writers|High|Low|High|
|Number of untrusted writers|High|Low|0|
|Consensus mechanism|Mainly PoW, some PoS|BFT protocols (e.g. PBFT [5])|None|
|Centrally managed|No|Yes|Yes|



TABLE I: We differentiate between permissionless, permissioned blockchains and a centralized database. Note that a permissioned blockchain can be public, for example if public verifiability of the content is desired. 

where businesses will benefit from a greater flexibility in interacting with different markets and balancing the price risks. 

Traditional SCM is driven by planning and communication. The future demand is estimated based on the past and current demand, information is pushed to the involved stakeholders that hope to get the relevant information on time to respond to changes, delays or errors. Companies decide what product is released to the market at what time, and customers indirectly drive the demand. 

In demand chain management (DCM), the customer’s interest is at the core of the chain — reduced costs, performant customer service, and faster go-to-market from idea or minimum viable product (MVP), just to name a few examples. DCM allows for this increased flexibility by requiring all stakeholders to have a real-time visibility of what consumers want and purchase. All parties of the demand chain have therefore to be tightly connected within a network. Contrary to SCM, which “optimizes the flow” and might be based on incomplete and inaccurate market assessements, DCM requires companies to have a complete and accurate view of the market to proactively choose optimal production decisions. As such, the information flow in DCM’s is _pull_ based rather than _push_ based: the stake holders do not need to wait for a notification, but can actively query the state of the chain management. 

While SCM solutions certainly can and should be improved, it is unclear why blockchain in particular is a suitable technical solution. Skuchain [7] for instance (cf. Figure 2) relies on IBM’s Hyperledger Fabric [11] as blockchain backend. Fabric’s pluggable consensus options allow for a wide range of flexibility on how many nodes are actually taking part in the consensus process. Skuchain acknowledged (upon request in private correspondance) that for most supply chain management features a single source of truth would be sufficient — as such a single trusted database at Skuchain should be sufficient to satisfy most business needs. 

Provenance [8] aims to provide another blockchain based solution for more transparency in product supply chains. Provenance does not provide any details on their technical product but claims that _data can be accessed and verified by all actors_ . Even if Provenance manages to hide the actor’s identity (as claimed in the whitepaper), such data would leak a considerable amount of business critical information from the different actors — e.g., production volume and times. 

Everledger [10] has digitally certified over 1 mio. diamonds and records every diamond permanently in the Everledger blockchain to provide a clear audit trail for stakeholders. While Everledger does not provide technical details on their solution, Everledger claims to use a hybrid model between a public and a private blockchain to benefit from the permissioned controls in private blockchains. 

**==> picture [93 x 63] intentionally omitted <==**

**==> picture [93 x 63] intentionally omitted <==**

**==> picture [208 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Traditional SCM. (b) Blockchain powered SCM.<br>**----- End of picture text -----**<br>


Fig. 2: Traditional SCM (left) compared to blockchain-based Supply Chain Management (right). Traditional SCM is distributed, i.e. there is no central entity. A blockchain powered SCM maintains a distributed ledger where participant can update and read (pull) the current SCM state. 

_1) Outlook:_ The participants of a SCM vary greatly across different supply chains and the same peers might take different roles across different supply chains. The segmentation basis for different actors in the supply chain is typically defined by their respective ownership stake of the product that is being produced. This implies that a single blockchain would be required for every supply chain that a participant is involved in — which clearly deteriorates the performance of the final solution. 

Following our methodology from Section III, a SCM certainly requires to store data. Multiple writers are involved, i.e. the different participants of the SCM that own a certain share of the final product. Skuchain acknowledged to only require a single source of trust, which would however remove the decentralized component of the blockchain, and thus be equivalent to a trusted central server. Continuing our methodology, a SCM could technically likely always use an online TTP. If that is not possible, at least all writers will be known, which leaves us to choose between a permissioned or no blockchain. 

This reasoning leaves us with the question whether all writers can be trusted. Supply chain management has the inherent problem of the interface between the digital and 

48 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

**==> picture [468 x 105] intentionally omitted <==**

**----- Start of picture text -----**<br>
�<br>�<br>� �<br>Blockchain Blockchain<br>(a) Intended Scenario: The supply truck is refrigerated. (b) Attack Scenario: The trusted sensor is in a cooled compart-<br>ment, while the rest of the truck is unrefrigerated.<br>**----- End of picture text -----**<br>


Fig. 3: An example for an attack that can be conducted against a tamperproof temperature sensor that writes collected data to the blockchain to ensure proper cooling of goods in a supply chain. The left subfigure shows the intended situation, where the delivery truch containing the goods is refrigerated, the trusted sensor measures the temperature and publishes the data to the blockchain correctly. The right subfigure shows the possible attack, where the supplier – e.g. to save costs – only cools down a small fridge inside of the truck in which he puts the sensor, while the goods are in the non-refrigerated section of the truck. 

the physical world. A human, or some machine under the control of a single writer, typically is required to register that a certain good has arrived in a warehouse, and if for example its quality is appropriate. If there is no trust in the operation of these employees, then the whole supply chain is technically compromised as any data can be supplied by a malicious writer. If, on the other hand, all writers are trusted, a blockchain is not needed as a regular database with shared write access can be used instead. Note that if through some technical means, the connection between the digital and physical world could be realized in a secure manner, then the previous reasoning might change. 

Multiple companies are currently researching possibilities of getting reliable data from some trusted hardware into the blockchain in order to achieve such a connection between the digital and physical world. As an example, some companies plan on using trusted temperature sensors to record temperature data during the delivery of food and medicine, so that this data can be used in smart contracts on the blockchain. However, even if the temperature sensors are assumed to be tamperproof, there are several issues with such a solution. First, a smart contract relying on temperature data must be sure that the key used to sign the data really belongs to the sensor that is currently used for the shipment. This can likely be solved with a sensibly designed PKI and some checks that the client has to perform before storing money in the smart contract. Second, and most importantly, a client still needs to fully trust the supplier for reliable data, because such sensors could be easily tricked even if tamperproof hardware is used. A simple attack – on a temperature sensor in a refrigerated truck – is shown in Figure 3. A supplier that e.g. wants to save cost by not cooling their delivery trucks could instead simply move the sensor into a cooled compartment or fridge while keeping the temperature in the rest of the truck unregulated. Figure 3a shows the intended situation: The sensor correctly records the temperature in the truck containing the goods. Instead, a malicious supplier could, however, use an unrefrigerated truck instead and only cool down a small compartment containing the sensor as shown in Figure 3b. 

## _B. Interbank and International Payments_ 

In this Section, we outline how interbank and international payments are currently performed in the banking system. In addition, we describe solutions based on distributed ledger technology that aim to simplify and replace the current system. Based on this understanding we explain the benefits and drawbacks of using distributed ledger technology to simplify interbank payments. 

_1) The Legacy System:_ Traditionally, in the current banking system, a transaction transferring money from an account at bank _A_ to an account at bank _B_ takes multiple steps. Contrary to cash transfers, debts in bank transfers are typically not immediately settled. 

If Alice wants to transfer $100 to Bob, Alice’s account is debited with $100 and Bob’s account should be credited with the same amount. If the accounts are at the same bank, the bank can simply apply these changes to their books because the total debit and credit amount of the bank remains identical. If Alice however, has her account at bank _A_ and Bob at bank _B_ , the total debit of bank _A_ changes when debiting Alice’s account. Similarly, if Bank _B_ credits Bob’s account without debiting another account with the same value, the sum of all debits and all credits at Bank _B_ would no longer be equal. This can be solved, if each of the banks have an account with the other bank (commonly referred to as a _Nostro_ account). Then, bank _A_ could debit Alice’s account and credit _B_ ’s account while bank _B_ would debit _A_ ’s account and credit Bob’s account while modifying the respective Nostro account. 

In practice this would lead to large debts between banks which brings a large amount of risk. Banks therefore have accounts at a central bank, which is mirrored in a local account ( _mirror account_ ) at the bank for bookkeeping, where they credit and debit the central bank. I.e., bank _A_ debits Alice’s account, informs the central bank of the payment and credits the mirror of their account at the central bank, the central bank debits the account of bank _A_ , credits bank _B_ ’s account and informs _B_ of the payment, who then debits their central bank mirror account and credits Bob’s account. The central banks are used as settlement authorities for the payments in 

49 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

**==> picture [274 x 224] intentionally omitted <==**

**----- Start of picture text -----**<br>
Bank C FED<br>Nostro Bank A Mirror FED Nostro Bank C Nostro Bank B<br>100 100 100 100<br>Mirror Bank C Pos. USD Bob (USD) Mirror FED<br>100 100 100 100<br>Alice (EUR) Pos. EUR<br>94.35 94.35 Bank B<br>**----- End of picture text -----**<br>


**==> picture [28 x 7] intentionally omitted <==**

**----- Start of picture text -----**<br>
Bank A<br>**----- End of picture text -----**<br>


Fig. 4: The logical flow of money and accounting steps involved in a traditional international payment in which Alice from Europe pays USD 100 to Bob in the USA. At Bank A, Alice implicitly buys USD with EUR, i.e. her account gets debited while the banks Euro account is credited with the same amount (EUR 94.35). The banks USD account then gets debited with the bought USD, which are credited to the mirror account of Bank C, who then debit Bank A’s Nostro account (i.e. the account that Bank A holds at Bank C) and credit the mirror account of the US central bank (FED). The FED then debits Bank C’s Nostro account and credits Bank B, who then debits the central banks mirror account and finally credit Bob’s account with the intended amount. Note that in this simplified example, fees that would occur in practice at intermediate steps are not shown. 

the currency for which they are responsible, since they are trusted to fulfill their debts (by issuing money if necessary). 

Already, three banks are involved for a single payment and in practice, additional parties such as clearing houses take part, such that low value payments can be batched and the central bank does not need to be involved in every interbank payment. For international (i.e., inter currency) payments, even more parties need to be involved, e.g., if Alice has a Euro account at bank _A_ located in the EU and bank _B_ is located in the USA. For cross currency payments, there is no single central bank that is able to settle the payments and bank _A_ does not have an account with the US central bank. 

Instead, bank _A_ has a USD account at some commercial bank _C_ in the USA, which we assume to be distinct from _B_ for this example. This bank _C_ is called _A_ ’s correspondent bank. This requires a trust relationship between banks _A_ and _C_ . In our example, some amount of Euro is debited in Alice’s account with which USD is implicitly bought by Alice at bank _A_ , i.e., _A_ ’s Euro position increases while the USD position decreases by $100. The $100 are credited to the mirror account for _A_ ’s account at Bank _C_ . Bank _C_ then debits _A_ ’s account at their bank and transfers the money to bank _B_ using the US central bank (FED) for the settlement. This money transfer is depicted in Figure 4. 

For money transfers in currencies for which a bank does 

not have a correspondent bank, additional intermediate hops may be required which adds complexity, more delays and as a consequence higher costs. 

Overall, the main drawbacks of the correspondent banking system are the long transaction confirmation time, the cost caused by the multiple intermediate hops and the trust that is required between the banks in order for the system to work. 

_2) Distributed Ledger Technology for Interbank Payments:_ Due to the high costs entailed by the correspondent banking system, many put their hopes into distributed ledger technology to simplify interbank payments. Some central banks such as the Monetary Authority of Singapore (MAS) and the Bank of Canada are working on solutions to use distributed ledger technology for interbank payments [12], [13] and the Federal Reserve Board in the US is also researching this technology [14]. In the solution of the MAS, banks deposit some amount of money with the MAS and in return receive the same amount on the distributed ledger. The ledger can then be used to immediately transfer money between the banks. While this does not allow cross currency transfers, it simplifies interbank payments within a single currency and is a first step towards replacing the payment system. 

Similarly, companies such as SWIFT [15] and Visa [16] started to develop proof of concepts for international payments using blockchain technology. While these proof of concepts 

50 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

are not yet public and very little information about them is available, other solutions using distributed ledgers that aim to simplify cross-border payments are already more developed. 

_Ripple_ [17] aims to provide a global settlement network based on a distributed ledger. Ripple only partially replaces the correspondent banking system. Banks can continue to use correspondent banks to process payments in cases where liquidity in the required foreign currency is available at low rates. Otherwise, banks can use third party liquidity providers to provide the required liquidity. 

Similar to the traditional correspondent banking system, a payment may require multiple hops if no trust relationship exists between the two banks that are parties in the transaction. Contrary to the traditional system, the payment is atomic, i.e., either all of the intermediate payments go through or none of them. In the traditional system, if something goes wrong for an intermediate payment, previous payments have to be reversed and sometimes manual intervention is required. Additionally, Ripple provides its own currency, XRP, which can be used as intermediate currency for transactions. 

XRP is the only currency on the Ripple ledger for which transactions do not entail counterparty risk. Other currencies are “issued” by gateways that need to be trusted to settle the owed debts outside of the distributed ledger if a party chooses to withdraw a deposit. This means, for example, that not all USD have the same issuer and they are not backed by the central bank, i.e., an on-chain US Dollar is not a real US Dollar and, de facto, every issuer creates a new parallel currency. Because of this gateway system, Ripple does not remove the trust relationships required in the correspondent banking system but simply shifts them to other parties, the gateways. 

This limitation could be removed if such a system would use central banks to act as gateways, since the currencies issued on Ripple would then actually correspond to the real currencies. This would remove all trust requirements for settlement other than the trust in the central banks, which is a necessity in any case when transacting in the corresponding currency. 

_3) Outlook:_ For financial applications, blockchain technology seems well suited in general, since parties are generally risk averse and do not want to rely on strong trust assumptions. We can evaluate the usefulness of blockchain technology for a given system with our methodology from Section III. If we consider a system for interbank payments, we have multiple parties, the banks, that act as writers. If we only consider single currency systems, we do have a trusted third party, the central bank. The central bank may, however, not want to act as a verifier for every transaction and may only act as a certificate authority giving out licenses to banks to participate in the system. This means that all writers of the system are known and we can use a permissioned blockchain. Whether the chain should be publicly verifiable is a matter of opinion, i.e. the blockchain can either be public or private. 

On one hand, banks likely want to keep their monetary flows private, on the other hand, having public verifiability may increase the trust of the public in the monetary system. As 

**==> picture [202 x 85] intentionally omitted <==**

**----- Start of picture text -----**<br>
A B L 2<br>C<br>L 1<br>L 3<br>D<br>**----- End of picture text -----**<br>


Fig. 5: Three individual ledgers _L_ 1, _L_ 2, and _L_ 3 that are connected through nodes _B_ and _C_ , i.e. node _B_ participates as writer in _L_ 1 and _L_ 2 and node _C_ participates in both _L_ 2 and _L_ 3. If each of these ledgers is a blockchain for one currency, a payment from _A_ to _D_ can be routed through _B_ and _C_ as atomic transaction, where _B_ and _C_ provide currency exchange. This can be achieved for example through hashed timelock contracts [18]. 

mentioned in Section II-B, this tension between transparency and privacy can be resolved at the cost of efficiency by using cryptographic techniques to provide privacy while also ensuring public verifiability. 

While current systems (such as Ripple) are not yet able to provide trustless intercurrency money transfers, the future development in this area looks promising. Many central banks currently research the possibilities of using blockchain technology for interbank payments and with centrally issued onchain currency, the value is defined by the actual value of the currency and thus interchangeable. 

If countries collaborate in designing their blockchains for interbank payments, they can be designed in a way that allow interaction between chains, e.g. to provide atomic cross currency payments as shown in Figure 5. This can be done using techniques that are also used in off-chain payment networks such as hashed timelock contracts [18] or by instantiating the blockchains as satellite chains [19]. In such a system, banks that have accounts on multiple chains can be used to exchange currency and route payments atomically internationally while removing the trust requirements of the correspondent banking system. 

## _C. Smart Contracts_ 

Smart Contracts [20] are digital contracts that are self enforcing or make it prohibitively expensive to break contract. Since a blockchain can be used as a distributed state machine without a trusted third party, the technology is well suited to support smart contracts. While Bitcoin already supports a limited set of smart contracts, Ethereum [2] was the first blockchain to support arbitrary code execution on the blockchain, allowing any kind of smart contract. 

51 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

Since contract partners do not usually fully trust each other, blockchain technology is suitable for this application if the parties do not want to rely on a trusted third party, because it can simplify trustless protocols between multiple parties. Depending on the setting and the requirements, a permissionless blockchain or a permissioned blockchain can be used. If details of the contract should remain hidden and the contract is only concerned with a limited set of known participants, a permissioned blockchain such as Hyperledger Fabric [11] may be the best fit. Otherwise – since smart contracts can also be established between multiple anonymous parties – a permissionless blockchain such as Ethereum can be used. 

Smart contracts can become especially powerful if they can be connected to other digital information and the physical world in some way. While the connection to other digital information is already possible [21], [22], further connecting the digital to the physical world would open up even more possibilities for use cases such as supply chain management (cf. Section IV-A), IoT (cf. Section IV-G) and more. 

Because practical smart contracts are relatively new technology, it is not yet clear to what extent these are legally binding, or how they should be interpreted. In many cases, the jurisdiction will not even be clear and to the best of our knowledge there has not yet been a case where a judge ruled on the interpretation of a smart contract. While the original idea of smart contracts was that “code is law”, and many in the community believed very strongly in this statement, opinions have shifted somewhat after the “DAO hack” that we discuss below. 

## _D. Decentralized Autonomous Organizations_ 

A _Decentralized Autonomous Organization_ (DAO) is an organization that is run autonomously through a set of smart contracts. In contrast to traditional organizations or companies, there is no central control or management. Instead, a DAO is defined by a set of rules encoded in smart contracts that define how the DAO behaves and how it evolves. Typically, a DAO has many investors that then decide by voting how the funds of the DAO should be invested. As the goal of such an organization is to be governed in a completely decentralized way and the investors generally don’t know or trust each other, a permissionless blockchain is naturally a good fit for such a design: The system is required to store some state and multiple mutually distrusting and possibly unknown writers exist. 

Decentralized autonomous organizations are, however, a special case. For some applications a dedicated permissioned blockchain may be useful for a single DAO. In most cases, however, DAOs do not require their own blockchain but are instead better suited to be build on top of an existing blockchain with an already existing currency (such as Ethereum [2]). 

While a blockchain is suitable for this use case, such constructs can be dangerous, as history has shown (in the socalled “DAO Hack” [23]). If the code that governs such a DAO is not very carefully written, verified, and has mechanisms in 

place to allow fixing vulnerabilities, they can become the target of attacks leading to the loss of large sums of money. 

This happened in 2016 with the famous “DAO hack” that lead to the split of the Ethereum blockchain. Before this attack, most of the community wanted smart contracts to be pieces of code that exactly codified the meaning of the contract. If the execution of the code resulted in a different outcome than originally intended by the contract parties, the rules encoded in the smart contract should be binding. Even the creators of “the DAO” stated in the conditions of their ICO that whatever the code does was its intention. In the wake of the “DAO” hack, this sentiment has become less absolute. Even though parts of the community refused to create a hard fork in Ethereum to refund the losses, most of the Ethereum community has taken a more practical stance to preserve the original intentions. 

## _E. Proof of Ownership for Intellectual Property_ 

Proof of Ownership for intellectual property is an often proposed and straightforward use case for blockchains. If the creator of some digital object wants to prove ownership at a later time, he can use a public blockchain as a time stamping service by committing to the digital object together with his identity, e.g. with a hash, and publishing that commitment on the blockchain. This allows to later prove that the object existed at that time and was associated with the respective identity. While this does not fully prove ownership, it does provide evidence of ownership if no one else can show that the object was previously published. Instead of using a blockchain, a trusted third party could provide a proof of ownership, e.g. a patent office. A public blockchain, however, eases the process of providing a proof in a decentralized way and without disclosing details of the digital object. 

## _F. E-Voting_ 

E-Voting is a problem with many difficulties. Many of the desired e-Voting properties have trade-offs. On one hand, for example, privacy is a main requirement as votes should be anonymous to prevent coercion. On the other hand, e- voting should provide some sort of public verifiability, because otherwise, the provider of the e-voting solution – or someone who managed to compromise it – might be able to change votes at will. In e-voting, many parties are involved and these parties typically do not trust each other. At the same time, e- voting systems require public verifiability, and thus, many have proposed to base e-voting systems on blockchain technology. Due to the requirements, it seems reasonable that blockchain technology can help to achieve some of the desired properties. However, to the best of our knowledge, so far no solution has been proposed that has been shown to be secure, verifiable, and private and there are still many open challenges. 

Following our methodology – and depending on the trust model – one could use an always online trusted third party, as is the case in today’s e-voting systems. However, it is possible that the state is trusted for voter registration but not for recording and tallying the votes of the election or referendum. In this case, we have an offline trusted third party, i.e. a public 

52 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

permissioned blockchain may be a good fit. Such a system could be designed as a permissioned blockchain in which political parties, NGOs or other partially trusted organizations could take roles as validators. The security depends on the distribution of the validators, however, and one has to be careful with assigning these roles. If, for example, a system assigns validator roles proportionally to the strengths of parties in the last election, no additional properties in terms of trust is gained, if a single party has a majority. 

## _G. Internet of Things_ 

Many have suggested possible use cases for blockchain technology in the Internet of Things (IoT) in combination with smart contracts with the aim to provide autonomous systems that pay for resources that they consume and get paid for resources that they provide. As the system is inherently decentralized with entities that do not trust each other, using a blockchain seems natural. However, as with supply chain management (cf. Section IV-A) the interface between the physical and the digital world poses a potential problem. If computers supply values that were read from sensors to the blockchain, the blockchain does not guarantee the correctness of these values, i.e. if smart contracts behave according to values supplied by sensors, the sensors – and whoever controls them – necessarily need to be trusted, as demonstrated by the example in Figure 3 in Section IV-A. For many cases, if e.g. only automation is desired, a blockchain may not be necessary if a trusted party can be used instead. In other cases, the specific trust assumption have to be studied and evaluated carefully to determine whether the use of a blockchain provides additional value. 

## _H. Land Titles_ 

In particular in countries where corruption might dominate and the integrity of official documents could be questionable, the use of blockchain could potentially help to provide more transparency through public verifiability. As such, several projects have started to secure land titles on a blockchain[24], but to date it is unclear to what extent these projects will sustain a wider adoption. The idea is interesting, but similar to supply chain management (cf. Section IV-A) and IoT (cf. Section IV-G) the physical world needs to be connected to the digital, as the actual rights to a property eventually need to be enforced by some authority, i.e. there exists a party that is trusted for enforcement. This trusted party could simply publish the register of land titles, which is in principle the current practice today in most jurisdictions. However, a public ledger could ensure that an authority – that is not necessarily trusted for anything other than enforcement – could not change land ownership in an undetected manner. Additionally, having land titles on a blockchain that also offers a method of payments would enable smart contracts to sell and buy property. Overall, whether or not a blockchain can be useful for a land registry depends strongly on the trust model and on other capabilities that the blockchain instantiation offers. 

## _I. Trading and Fair Exchange Protocols_ 

Fair multi-party exchange protocols have been extensively studied in the literature. Due to the recent emergence of open and decentralized blockchains (e.g. Bitcoin and Ethereum), however, the design of fair exchange protocols has recently experienced a renaissance. The exchange of digital goods is likely to be feasible without trusted dispute mediator [25], while the exchange of physical goods still requires a trusted third party in case of disputes [26]. 

As fair exchange inherently assumes mutually mistrusting parties that may even be anonymous, blockchain technology immediately seems reasonable. In some cases, trading parties may be able to use a trusted third party, but in others they may not. Following our methodology, a permissionless blockchain is likely the best fit if only digital goods are exchanged. 

## V. RELATED WORK 

Bitcoin [1], as the first open and decentralized blockchain, initiated a large development in the area. Other permissionless blockchains such as Zerocash [3] or Ethereum [2] build on the techniques used by Bitcoin and extend the possibilities through improved privacy or more expressive smart contracts. Other extensions such as hashed timelock contracts that are e.g. used in the lightning network [18] can be used to improve the throughput of blockchains or to allow transfers of digital assets between different blockchains. 

Through the emergence of Bitcoin, many companies now develop their own permissioned blockchains (e.g. Corda [4], Hyperledger Fabric [11]) where the participants are limited to a predefined set. Since the permissioned setting is simpler than a permissionless setting, these permissioned blockchains can use more efficient protocols for consensus that have been known for decades such as PBFT [5]. 

Bonneau et al. systematize some blockchain properties in [27] with a focus on cryptocurrencies and specifically Bitcoin. A good comparison of consensus algorithms for use with permissioned and permissionless blockchains was provided by Bano et al. [28] in 2017. 

## VI. CONCLUSION 

The choice between a centralized database and a permissionless or permissioned blockchain is not trivial. While this question has been discussed before [29], to the best of our knowledge, we provide in this article the first structured methodology to decide which technological solution is the most appropriate depending on which application scenario. Our methodology takes into account the required trust assumptions, application requirements, involved parties and technical characteristics such as throughput and latency. We applied our methodology to three known application scenarios that have seen wider interest to adopt blockchain technology and further discussed other use cases. We conclude that depending on the application scenario, there are indeed valid use cases for each, permissionless and permissioned blockchains, and centralized databases that need to be determined carefully. 

53 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

## REFERENCES 

- [1] Satoshi Nakamoto. Bitcoin: A peer-to-peer electronic cash system. 2009. 

- [2] Gavin Wood. Ethereum: A secure decentralised generalised transaction ledger. _Ethereum Project Yellow Paper_ , 151, 2014. 

- [3] Eli Ben Sasson, Alessandro Chiesa, Christina Garman, Matthew Green, Ian Miers, Eran Tromer, and Madars Virza. Zerocash: Decentralized anonymous payments from bitcoin. In _Security and Privacy (SP), 2014 IEEE Symposium on_ , pages 459–474. IEEE, 2014. 

- [4] Richard Gendal Brown, James Carlyle, Ian Grigg, and Mike Hearn. Corda: An introduction. _R3 CEV, August_ , 2016. 

- [5] Miguel Castro and Barbara Liskov. Practical byzantine fault tolerance. In _Proceedings of the Third Symposium on Operating Systems Design and Implementation_ , OSDI ’99, pages 173–186, Berkeley, CA, USA, 1999. USENIX Association. 

- [6] Arthur Gervais, Ghassan O. Karame, Karl Wüst, Vasileios Glykantzis, Hubert Ritzdorf, and Srdjan Capkun. On the security and performance of proof of work blockchains. In _Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ , CCS ’16, pages 

   - 3–16, New York, NY, USA, 2016. ACM. 

- [7] Skuchain, 2017. https://www.skuchain.com/. 

- [8] Provenance, 2017. https://www.provenance.org/. 

- [9] Robert Hackett. Walmart and ibm are partnering to put chinese pork on a blockchain. _Fortune_ , 2016. http://fortune.com/2016/10/19/walmartibm-blockchain-china-pork/. 

- [10] Everledger, 2017. https://www.everledger.io/. 

- [11] Elli Androulaki, Artem Barger, Vita Bortnikov, Christian Cachin, Konstantinos Christidis, Angelo De Caro, David Enyeart, Christopher Ferris, Gennady Laventman, Yacov Manevich, et al. Hyperledger fabric: A distributed operating system for permissioned blockchains. _arXiv preprint arXiv:1801.10228_ , 2018. 

- [12] Mas working with industry to apply distributed ledger technology in securities settlement and cross border payments, 2017. http: //www.mas.gov.sg/News-and-Publications/Media-Releases/2017/MASworking-with-industry-to-apply-Distributed-Ledger-Technology.aspx. 

- [13] Carolyn A. Wilkins. Fintech and the financial ecosystem: Evolution or revolution?, 2016. http://www.bankofcanada.ca/wp-content/uploads/ 2016/06/remarks-170616.pdf. 

- [14] David Mills, Kathy Wang, Brendan Malone, Anjana Ravi, Jeff Marquardt, Clinton Chen, Anton Badev, Timothy Brezinski, Linda Fahy, Kimberley Liao, Vanessa Kargenian, Max Ellithorpe, Wendy Ng, and Maria Baird. Distributed ledger technology in payments, clearing, and 

- [24] Laura Shin. The first government to secure land titles on the bitcoin blockchain expands project, 2017. https://www.forbes.com/sites/ laurashin/2017/02/07/the-first-government-to-secure-land-titles-on-thebitcoin-blockchain-expands-project/\#22e2e6fa4dcd. 

settlement. Washington, 2016. Board of Governors of the Federal Reserve System. https://doi.org/10.17016/FEDS.2016.095. 

- [15] SWIFT explores blockchain as part of its global payments innovation initiative, 2017. https://www.swift.com/news-events/pressreleases/swift-explores-blockchain-as-part-of-its-global-paymentsinnovation-initiative. 

- [16] Exploring the blockchain: cross-border settlement, 2016. https://vision.visaeurope.com/blogs/exploring-the-blockchain-crossborder-settlement. 

- [17] Ripple, 2017. https://ripple.com/. 

- [18] Joseph Poon and Thaddeus Dryja. The bitcoin lightning network: Scalable off-chain instant payments, 2015. 

- [19] Wenting Li, Alessandro Sforzin, Sergey Fedorov, and Ghassan O. Karame. Towards scalable and private industrial blockchains. In _Proceedings of the ACM Workshop on Blockchain, Cryptocurrencies and Contracts_ , BCC ’17, pages 9–14, New York, NY, USA, 2017. ACM. 

- [20] Nick Szabo. Formalizing and securing relationships on public networks. _First Monday_ , 2(9), 1997. 

- [21] Hubert Ritzdorf, Karl Wüst, Arthur Gervais, Guillaume Felley, and Srdjan Capkun.[ˇ] TLS-N: Non-repudiation over TLS Enabling Ubiquitous Content Signing for Disintermediation. In _NDSS_ , 2018. 

- [22] Fan Zhang, Ethan Cecchetti, Kyle Croman, Ari Juels, and Elaine Shi. Town crier: An authenticated data feed for smart contracts. In _Proceedings of the 2016 aCM sIGSAC conference on computer and communications security_ , pages 270–282. ACM, 2016. 

- [23] David Siegel. Understanding the dao attack. _Web. http://www. coindesk. com/understanding-dao-hack-journalists_ , 2016. 

- [25] Wacław Banasik, Stefan Dziembowski, and Daniel Malinowski. Efficient zero-knowledge contingent payments in cryptocurrencies without scripts. In _European Symposium on Research in Computer Security_ , pages 261–280. Springer, 2016. 

- [26] Steven Goldfeder, Joseph Bonneau, Rosario Gennaro, and Arvind Narayanan. Escrow protocols for cryptocurrencies: How to buy physical goods using bitcoin. 2017. 

- [27] Joseph Bonneau, Andrew Miller, Jeremy Clark, Arvind Narayanan, Joshua A Kroll, and Edward W Felten. Sok: Research perspectives and challenges for bitcoin and cryptocurrencies. In _Security and Privacy (SP), 2015 IEEE Symposium on_ , pages 104–121. IEEE, 2015. 

- [28] Shehar Bano, Alberto Sonnino, Mustafa Al-Bassam, Sarah Azouvi, Patrick McCorry, Sarah Meiklejohn, and George Danezis. Sok: Consensus in the age of blockchains. _arXiv preprint arXiv:1711.03936_ , 2017. 

- [29] Gideon Greenspan. Avoiding the pointless blockchain project, 2015. http://www.multichain.com/blog/2015/11/avoiding-pointless- 

- blockchain-project/. 

54 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 18,2026 at 06:56:07 UTC from IEEE Xplore.  Restrictions apply. 

