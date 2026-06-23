~~a~~ Received: 25 June 2023 ~~a~~ Revised: 8 April 2024 Accepted: 8 August 2024 DOI: 10.1002/ett.5037 

**S U R V E Y A R T I C L E** 

## **An extensive multivocal literature review of blockchain technology: Evolution, challenges, platforms, security, and interoperability** 

## **Monika Rajesh Bhatia** | **Manish Kumar** 

CSE Department, Punjab Engineering College (Deemed to be University), Chandigarh, India 

## **Correspondence** 

Monika, CSE Department, Punjab Engineering College (Deemed to be University), Chandigarh, India. Email: m.bishnoi29@gmail.com 

## **Abstract** 

Blockchain technology has gained enormous interest from industry and academia recently. Technology enthusiasts are exploring its use case beyond cryptocurrencies and claim that blockchain technology can overcome the inefficiencies of centralized systems. In this study, we continue the work of previous authors, aiming to provide a more comprehensive understanding of the technical aspects of blockchain. This study is the first of its kind to review and analyze the current status of different technical aspects of blockchain technology influencing its adoption. We performed an extensive multivocal review to ~~(i) demonstrate the progress of blockchain, (ii) discuss the challenges related~~ to the wide-scale adoption of the technology, (iii) present a detailed analysis of blockchain platforms, (iv) highlight security and interoperability issues followed by the solutions proposed in the literature. We have considered 259 peer-reviewed research papers and the gray literature related to 40 blockchain platforms to provide an in-depth analysis of blockchain technology. In conclusion, this comprehensive survey provides a holistic view of blockchain technology’s progress. It identifies challenges, trends, and future research directions, serving as a valuable resource for researchers and practitioners seeking to navigate the dynamic blockchain landscape. 

## **1** | **INTRODUCTION** 

Blockchain (BC) is the first decentralized distributed ledger technology providing immutable, transparent, and auditable data storage. Blockchain technology is growing fast and is changing very rapidly as the different domains recognize its use cases. Every domain and industry is trying to modify and improve blockchain technology to make it more applicable to their use case scenarios. Transparency, immutability, lower cost due to removing costly intermediaries, and smart contracts(SCs) are blockchain technology’s main properties[1] leveraged by all blockchain 3.0 applications. Blockchain has progressed so fast[2] from merely a distributed decentralized electronic cash system to a technology capable of doing complex computations on a distributed ledger. Its ability to execute turning complete programs on a distributed ledger and solve real-life problems of provenance tracking, auditing, supply chain management, identity management, and electronic voting[1,3] has gained blockchain tremendous attention from academia and industry. 

Most of the survey studies found in the literature are related to blockchain applications, smart contract security, challenges, and improvements related to specific aspects of blockchain Technology. These aspects are consensus algorithms, 

© 2024 John Wiley & Sons Ltd. **1 of 50** 

wileyonlinelibrary.com/journal/ett 

_Trans Emerging Tel Tech_ . 2024;35:e5037. https://doi.org/10.1002/ett.5037 

MONIKA et al. 

**2 of 50** 

privacy and anonymity, scalability, security and interoperability. No survey study has done an extensive multivocal (considering both white and gray literature) study considering the progress of blockchain technology in terms of its platform development, security, and cross-chain communication. The security of blockchain platforms/Decentralized Applications (DApps) and interoperability among them are the two most important aspects affecting blockchain adoption and sustainability. This study carries out a large-scale multivocal study by considering gray literature (books, technical reports, websites, white papers, and GitHub repositories) on blockchain platforms and research literature on blockchain security and interoperability aspects to lay out the current status of blockchain technology. 

We elaborate on the results of our study by firstly showing the progress of blockchain over the years by discussing its journey from merely a decentralized distributed ledger to a technology capable of running complex decentralized applications. We have briefly explained the advancement of blockchain technology in terms of scalability, consensus mechanisms, platforms, DApps, and real-life applications. Risks and barriers blockchain technology faces are explained next. We have figured out eight major adoption challenges for blockchain technology and presented the research work done to solve these challenges. We have proposed a taxonomy of security issues to give readers a new outlook on blockchain security. This taxonomy divides blockchain security issues into two major types: Infrastructure-specific and smart contract-specific. Next, interoperability concerns are discussed to wrap up the overview and present the current status of blockchain technology. 

Second, diving deeper, we discuss the blockchain platforms in detail as platform selection plays a crucial part in developing a blockchain solution for any real-life problem scenario. This study presents different categories of blockchain platforms, platforms specific to each category, and characteristics of each platform to give readers an understanding of available options. Further, we highlight the current development status of platforms/DApps and future trends according to the industry. Third, we conclude the study by discussing the security and interoperability solutions developed for blockchain technology, followed by future trends in the respective aspects. 

## **2 RELATED WORK** 

Until now, many surveys have been conducted across the globe by different researchers concerning the different aspects of blockchain technology. By analyzing all such surveys, we have found that most surveys consider only one particular aspect of the technology, like its privacy, security, interoperability, or applications. There are only a few surveys[4–18] that cover multiple aspects of blockchain technology but, none that explain the current status of blockchain technology in terms of where it stands today. We have considered five aspects of the technology: Blockchain evolution, challenges/limitations, platforms, security, and interoperability to present the current standing of the technology. Table 1 compares the existing surveys covering multiple technical aspects of blockchain technology. 

None of the above-listed surveys cover all five aspects of blockchain technology considered in this study. Nine of the surveys mentioned above have discussed and compared consensus algorithms. Only the study[13] has briefly discussed blockchain platforms. Huang et al.[11] has discussed some of the interoperability solutions to enhance the scalability of blockchains. The surveys discussing security issues are,[5,15] where[5] is primarily based on privacy issues and solutions. 

## **3 REVIEW METHODOLOGY** 

The main aim of this review study is to determine the current status of blockchain technology in terms of its adoption, the development trend of blockchain platforms, interoperability, and security concerns hindering its wide-scale adoption. A set of RQs was framed, as shown in Table 2, to carry out the extensive literature review. We considered five prominent electronic databases: ACM Digital Library, IEEE eXplore, ScienceDirect, Springer, and Wiley, to keep the perspective wide and extracted thousands of research and review papers. We have considered articles that appeared in the search till June 2022 as analyzing such a large amount of papers and writing a comprehensive review needs time. We present an extensive but not systematic literature review of blockchain technology. We used “Blockchain” as the search keyword to be included in the abstract of the article. This study does not include non-English papers, posters, and keynotes. A total of 6776 papers were extracted till February 2020, and a further 3420 papers were selected for March 2020 to June 2022. The inclusion criteria for selecting research articles for this study are determined by the following questions: 

1. Does the article cover the evolution of blockchain technology? 

MONIKA et al. 

**3 of 50** 

|||
|---|---|
|et al.|**3 of**|
|**TA B L E 1**<br>Comparison of existing reviews with the proposed study.<br>**Ref.**<br>**Year**<br>**Focus**<br>**BCG/BCE**<br>**Limitations/**<br>**Challenges**<br>**Platform**<br>**Security**<br>**IOP**<br>**Remarks**<br>[4]<br>2019<br>Consensus algorithms,<br>applications, challenges<br>_X_<br>✓<br>_X_<br>_X_<br>_X_<br>Discussed BC architecture, consensus algorithms,<br>applications, challenges<br>[5]<br>2019<br>Security, Privacy<br>✓<br>_X_<br>_X_<br>✓<br>_X_<br>Discussed BC basics, security, and privacy supported in<br>Bitcoin and enhancements needed, compared consensus<br>algorithms, and presented the available security and privacy<br>solutions for all the BCs.<br>[6]<br>2020<br>BC progress and assessment<br>initiatives<br>✓<br>_X_<br>_X_<br>_X_<br>_X_<br>Analyzed studies related to BC assessment and adoption<br>[7]<br>2020<br>BC properties, advantages,<br>pitfalls<br>_X_<br>✓<br>_X_<br>_X_<br>_X_<br>Presented mapping and association among BC properties,<br>advantages, and pitfalls<br>[8]<br>2020<br>BC architecture, properties,<br>challenges<br>_X_<br>✓<br>_X_<br>_X_<br>_X_<br>Systematically discussed BC architecture, properties, and<br>consensus algorithms. Presented challenges, research gaps,<br>and future directions.<br>[9]<br>2020<br>BC architecture, Consensus<br>Protocols, scalability, privacy,<br>BC frameworks<br>_X_<br>Only scalability<br>and privacy<br>_X_<br>Briefly<br>discussion of SC<br>issues and<br>improvements<br>_X_<br>Discussed BC architecture, compared consensus algorithms,<br>presented scalability and privacy concerns and<br>improvements.<br>[10]<br>2020<br>Consensus algorithms, BC<br>applications, and adoption,<br>challenges<br>✓<br>✓<br>_X_<br>_X_<br>_X_<br>Compared consensus algorithms. BC cryptography, BC<br>applications, and BC adoption throughout the years are<br>discussed. Challenges and future research directions<br>presented.<br>[11]<br>2021<br>Performance, scalability, BC<br>analytical models<br>_X_<br>Briefly<br>_X_<br>_X_<br>Briefly<br>Discussed studies for measuring and improving performance<br>and scalability. Discussed Modelings, Techniques, and<br>Theories for Better Understanding Blockchains Category<br>[12]<br>2021<br>BC network, mining,<br>sharding, and off-chain<br>payment channels<br>_X_<br>Briefly<br>_X_<br>_X_<br>_X_<br>Provided the BC networking questions and discussed<br>solutions to address them<br>[13]<br>2021<br>BC characteristics, issues, BC<br>evolution, consensus<br>algorithms, security<br>✓<br>✓<br>Briefly<br>Briefly<br>_X_<br>Discussed BC characteristics and their related issues,<br>compared consensus mechanisms, briefly discussed BC<br>platforms and security issues.<br>[14]<br>2022<br>Interoperability, Platforms,<br>consensus algorithms<br>✓<br>_X_<br>_X_<br>_X_<br>_X_<br>Compared 19 platforms and various consensus algorithms.<br>Presented layered reference meta-architecture and research<br>taxonomy for BI, state-of-art for BI, challenges, and future<br>research directions.<br>(Continues)||



MONIKA et al. 

**4 of 50** 

|**TA B L E 1**<br>Continued.|**Limitations/**|**Ref.**<br>**Year**<br>**Focus**<br>**BCG/BCE**<br>**Challenges**<br>**Platform**<br>**Security**<br>**IOP**<br>**Remarks**|[15]<br>2022<br>Security and Privacy<br>✓<br>_X_<br>_X_<br>✓<br>_X_<br>Discussed attacks and defense methods related to|mining, network communication, smart contracts,|mining, network communication, smart contracts,|and privacy theft of BC|and privacy theft of BC|[16]<br>2022<br>BC use-cases and<br>✓<br>✓<br>_X_<br>_X_<br>_X_<br>Discussed BC architecture, benefits, and|applications, Consensus<br>applications. Compared consensus algorithms and|algorithms, and challenges.<br>presented research challenges.|[17]<br>2022<br>BC Publication trend,<br>_X_<br>_X_<br>_X_<br>_X_<br>Discussed BC characteristics, compared consensus|applications, challenges<br>algorithms, discussed publication trends from|different perspectives, BC applications, and research|different perspectives, BC applications, and research|challenges.|[18]<br>2022<br>BC architecture design<br>_X_<br>_X_<br>_X_<br>Briefly<br>_X_<br>Presented a taxonomy of major dimensions for|decisions, BC threats and<br>blockchain architecture design decisions, quality|attacks<br>attributes per dimension, mapped and linked threats,|attacks<br>attributes per dimension, mapped and linked threats,|attacks, and design decisions.|This study<br>–<br>BC evolution, adoption<br>✓<br>✓<br>✓<br>✓<br>✓<br>Details discussion of BC platforms, BC infrastructure|barriers, platforms, security,<br>security and SC security issues and solutions,|barriers, platforms, security,<br>security and SC security issues and solutions,|interoperability<br>Interoperability challenges and solutions. BC current|status, adoption barriers, and future research|directions.|_Note_: (BCG/BCE: Blockchain Generations/Blockchain Evolution, IOP: Interoperability). Considered:✓, Not considered:_X_.|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|||||||||||||||||||||||||||||
|||||||||||||||||||||||||||||



MONIKA et al. 

**5 of 50** 

**TA B L E 2** Research questions, sub-questions and motivations. 

## ~~**Research questions**~~ 

1. What is the current status of blockchain technology? 

- 1.1 What is the progress of blockchain since its inception? 

- 1.2 What are the challenges and limitations affecting the adoption of blockchain technology in various domains? 

- 1.3 What are the issues related to blockchain security? 

- 1.4 What are the interoperability issues related to blockchain systems? 

2. What is the current status of blockchain platforms? 

- 2.1 What are the different types of platforms, their characteristics, and their application areas? 

- 2.2 What are the techniques, algorithms, and protocols used by blockchain platforms? 

- 2.3 What is the current development trend of blockchain platforms? 

3. What is the current status of blockchain systems in terms of security and interoperability? 

- 3.1 What are the different interoperability techniques applied to blockchain systems, and what are their limitations? 

- 3.2 What techniques have been proposed to handle security issues related to blockchain, and what are their limitations? 

## ~~**Motivation**~~ 

It would help in understanding the current status of blockchain technology in terms of its progress since its inception, factors affecting its adoption, security issues troubling developers and its interoperability concerns to provide researchers a more in-depth perspective of the technology. 

It would walk researchers through the different categories of blockchain platforms available, and theiruse-cases as of present and in the future. It would also enable readers to ~~understand the insights of the platforms like their technical~~ details, current development status along with some future research directions requiring urgent attention. 

It would report on how security and interoperability are the most crucial factors in determining the sustainability of blockchain technology by explaining the challenges faced by the researchers and research areas that need further investigation. 

   - 3.3 What are the open research challenges and future trends related to the interoperability of blockchain systems? 

   - 3.4 What are the open research challenges and future research trends related to blockchain security? 

2. Does the review article focus on the adoption or sustainability of blockchain technology? 

3. Does the research/review article adequately describe/compare/evaluate any blockchain aspect mentioned in RQs such as platforms, security, or interoperability? 

4. Does the research article compare the studies related to blockchain aspects mentioned in RQs? 

5. Does the article add any value to answering the RQs? 

The Exclusion criteria for the papers are as follows: 

1. Literature discussing the application of blockchain technology for a specific domain/sector. 

2. Research articles discussing any individual protocol/theory/proposal for blockchain aspects like consensus algorithm, privacy, anonymity, and scalability as there are comprehensive surveys already present regarding these aspects. 

After rigorously following the inclusion and exclusion criteria mentioned above, 259 peer-reviewed research papers were shortlisted to be analyzed in detail. We have included gray literature on blockchain platforms in our study to deliver invaluable insights, as only a little peer-reviewed published literature is available. For gathering the gray literature official website of CoinMarketCap[19] owned by Binance Holdings Limited and official GitHub repositories of blockchain platforms are used. 

In this study, we have not considered research articles on scalability, anonymity, privacy, consensus algorithms, energy consumption, and blockchain applications in different domains. However, comprehensive surveys related to these aspects of blockchain technology are considered to provide the latest knowledge to the readers. These aspects have already been studied in detail by researchers around the world and it is not possible to include literature regarding every aspect of blockchain technology in a single review. The survey studies related to scalability,[20–22] anonymity and privacy,[23–26] consensus algorithms,[27–30] energy consumption,[31–33] and applications of blockchain[3,34,35] can be referred by readers to get a thorough understanding of research work done in these fields. 

MONIKA et al. 

**6 of 50** 

## **4 FINDINGS** 

The motive of the study is to thoroughly investigate the available literature to answer the research questions mentioned in Table 2. After the rigorous analysis of 259 research articles and the gray literature related to blockchain platforms, this section presents the findings arranged in sequential order according to the research questions. 

## **4.1 Current status of blockchain technology** 

Blockchain technology is growing at a fast pace and is constantly changing. It is imperative to understand the progress of blockchain and identify the factors holding it back from wide-scale adoption. 

## 4.1.1 Progress of blockchain 

The blockchain concept came from Bitcoin, designed as a peer-to-peer electronic cash system in 2008. Researchers studied the protocol behind the Bitcoin system and named it blockchain due to its architecture consisting of blocks linked to each other. Blocks in a blockchain are connected so that ith block stores the hash of i-1th block, i-1th block stores the hash of i-2th block, and it continues like this. This linkage makes a chain of blocks and contributes to the immutability property of blockchain. The fundamental use of blockchain is to store the data in an append-only ledger, and data, once stored in the ledger, cannot be altered. Public blockchains like Bitcoin are fully transparent; all the transactional data stored in blocks is publicly available and can be audited anytime. Blockchain is a P2P network of nodes where all nodes share the exact copy of the ledger. Transactions on a blockchain are signed by the private key of the sender and are eventually bundled together to form a block by miners. Bitcoin blockchain uses the concept of Proof-of-Work (PoW) consensus mechanism, also known as Nakamoto consensus, to construct, validate, and append a new block into the blockchain. Miners of the Bitcoin blockchain network solve a computationally complex problem for creating a new block and get incentives for it as rewards in the form of bitcoins. A transaction on the Bitcoin blockchain is considered final only after its inclusion in a verified block, which becomes part of the main chain and not the fork chain. A new block is added roughly every 10 min, and initially, Bitcoin’s throughput was only 3.3–7 transactions/second[4] due to the small block size of 1 MB and slow block creation rate. Bitcoin follows the Unspent Transaction Output(UTXO) model, where a new transaction uses outputs from previous transactions and produces new outputs. 

Initially, blockchain was considered as distributed, decentralized, immutable, public, and transparent electronic software for transacting digital tokens called bitcoins. Its primary use case was in the financial sector only as a cryptocurrency for payment systems. Bitcoin blockchain has several issues like scalability, privacy, massive energy consumption, and limited support for programming. Blockchain technology gained momentum due to its unique architecture, and several altcoins were developed to enhance the technology. Monero and Zerocash were developed to increase the privacy of transactions,[9,36] and Bitcoin Cash was forked from the Bitcoin blockchain to improve scalability by increasing block size from 1 to 8 MB.[36] Primecoin[4,37,38] used modified PoW to search for the largest prime number chains rather than finding a hash to have some mathematical application. Peercoin[9,38] is the first to use Proof-of-Stake (PoS) to reduce energy consumption. Bitcoin and all the altcoins designed to improve different aspects of the Bitcoin blockchain are known as Blockchain 1.0. 

In 2014, Ethereum was introduced for incorporating programmability in blockchain technology. Ethereum consists of an Ethereum Virtual Machine (EVM) that is used to compile Turing complete Smart Contracts (SCs). A smart contract is a code that can be deployed and executed on a blockchain. Ethereum is the first blockchain platform to support advanced programming constructs and bring the concept of DApps. Dabbagh et al.[39] stated in their bibliometric study that blockchain has gained the research community’s interest and significant publications since 2016. Unlike bitcoin and other altcoins, Ethereum uses an account-based model consisting of two types of accounts: An external account controlled by the user’s private key and a contract account controlled by its code. Ethereum 1.0 also uses the PoW consensus mechanism, but with Ethash – a ASIC resistant hash function, its block creation time and throughput (14–16 TPS) are better than Bitcoin. However, it still suffers from the same limitations as the Bitcoin blockchain, such as scalability, transaction latency, and energy consumption. Much research has been done and is still going on to improve the scalability issues of blockchain platforms. The study[20] discusses all the major scalability solutions developed on Layer 0 (Data propagation layer), Layer 1 (On-chain), and Layer 2(off-chain) of blockchain. The authors have discussed payment channels, sidechains, cross-chain solutions like Cosmos and Polkadot, sharding, and DAG-based blockchain solutions proposed 

MONIKA et al. 

**7 of 50** 

in the literature to increase blockchains’ scalability and make them interoperable. Ethereum and other smart contract supporting platforms,[40,41] like Hyperledger, Corda, EOS, and Codius, are considered Blockchain 2.0. 

With the invention of Ethereum, blockchain technology’s use case shifted from merely a cryptocurrency or payment system to technology with transformative potential to be used in other domains. There are numerous DApps deployed on different blockchain platforms like Ethereum, TRON, Steem, Hive, BNB chain, Polygon, and many others.[42] These DApps are related to games, exchanges, finance, storage, wallets, governance, health, energy, and many other domains. The current status of DApps will be explained in Section 4.2.3. Ethereum’s ERC-20 standard[41,43] brought the concept of token creation on top of the Ethereum blockchain and enabled users to represent digital rights over assets. With the establishment of the ERC-20 standard, a new crowd funding mechanism called Initial Coin Offering (ICO) emerged, through which companies issue their product-related crypto tokens to investors. ICOs provide companies with a fast and easy way to raise funds without sharing equity, but investors need to be aware of frauds and Ponzi schemes. With the increasing popularization of blockchain applications, the concept of Decentralized Autonomous Organization (DAO)[44] emerged, where the operational behavior of an organization can be encoded as SCs. DAOs can run autonomously on a blockchain without involving any trusted third party. 

Maesa et al.[1] have presented a study of blockchain applications related to six domains: electronic voting, identity management, healthcare, decentralized notary, records management, supply chain, and intellectual property protection. Blockchain 3.0 is described as the real-life applications of blockchain technology[3,34,45–51] in different domains. As there are no established standards for developing, governing, and combining blockchain platforms, much research is still going on for developing interoperability techniques[52,53] for distributed ledgers. The interoperability of blockchains among themselves and other traditional database systems is a major aspect of defining the progress and sustainability of blockchain technology. As Ethereum is the most used public platform for blockchain enhancements and DApps development, its protocol improvement from Ethereum 1.0 to Ethereum 2.0 will be a turning point for the blockchain community. Ethereum 2.0[54] brings its complete transition to the PoS consensus mechanism to reduce energy consumption and make it more sustainable. Blockchain technology is making decentralized finance[55] possible by leading the emergence of new business models such as decentralized currency, decentralized payment services, decentralized fundraising, and decentralized contracting. 

## 4.1.2 Challenges and limitations affecting the adoption of blockchain 

Blockchain technology has ample potential, but it has certain limitations, as pointed out by numerous studies in the literature. Prewett et al.[56] presented some significant obstacles to blockchain adoption, such as scalability, system integration, lack of standardization, regulatory uncertainty, the complexity of blockchain applications, and lack of knowledge. They have also discussed the risks associated with adopting blockchain technology, such as architecture and design risk, endpoint/oracle risk, data security and confidentiality risk, storage, smart contract risk, compliance risk, vendor risk, contractual risk, and private key management. The authors[49] have done an SLR focused on business-related blockchain research and discussed the top challenges and risks associated with blockchain in the business literature. The study[57] has presented 27 perceived risks affecting blockchain adoption and discussed the required risk management. Researchers, developer communities, and standards organizations worldwide are working on these challenges and risks associated with blockchain technology. Many recent survey studies[10,16,58,59] have discussed the limitations and challenges related to the adoption of blockchain technology. The authors[10] have also discussed the reports of Deloitte, PWC, Stanford University, and Cambridge University, showing that blockchain has been getting more adoptions since 2019. This section discusses eight significant challenges found in literature affecting the wide-scale adoption of blockchain technology in other domains. All eight challenges shown in Figure 1 are discussed in detail further. 

## _Security (vulnerabilities and attacks)_ 

Due to blockchain characteristics like immutability, P2P network, and publically available data, blockchain has many security concerns. These security issues can be selfish mining, service attacks, smart contract vulnerabilities, hash functions-related attacks, and many more. We have provided a taxonomy for these attacks in Section 4.1.3 and have discussed them in detail. Section 4.3.2 explains the proposed solutions for these attacks. Another big security concern related to blockchain technology is smart contracts. Smart contract development differs from traditional software development due to its unique design. Due to its infancy stage of development, SCs suffer from many vulnerabilities and attacks. The studies[61–63,106,107] have performed analytical research regarding smart contract development, performance, security, and 

**8 of 50** MONIKA et al. ~~—lwyiSs~~ 

MONIKA et al. 

**F I G U R E 1** Analysis of survey studies related to blockchain adoption, challenges, and limitations. 

verification. We have provided a taxonomy of smart contract vulnerabilities in Section 4.1.3 with brief details of each vulnerability. Section 4.3.2 explains the security analysis solutions and verification techniques found in the literature for smart contracts. 

## _Scalability and performance_ 

A network’s ability to handle an increasing number of peers and a large number of transactions in a reasonably short period is measured as its scalability.[108] Transaction throughput and transaction latency are the fundamental scalability issues in PoW-based blockchains because of the strong immutability guarantee provided by these blockchains.[109] As discussed in Section 4.1.1, Bitcoin’s transaction throughput is 3–5 TPS, Ethereum’s throughput is 14–16 TPS, and both are significantly less than Visa’s throughput. Proof of Concepts (PoX) consensus algorithms[23] are designed to improve the performance of PoW in terms of security, fairness, transaction throughput, and sustainability. Some studies[1,23] have shown that researchers tried to improve the performance by increasing block size and reducing confirmation time in Nakamoto-like protocols. However, such a re-parameterization approach is constrained by the network bandwidth and security requirement of blockchain. Zhou et al.[20] have extensively studied major scalability issues and solutions of blockchain. The authors have divided the scalability solutions into Layer 0, Layer 1, and Layer 2 solutions. Layer 1 solutions include block data-related solutions (SigWit, block compression, increasing block size), different consensus, sharding, and DAG-based distributed ledgers. Layer 2 solutions include payment channels, sidechains, off-chain computations (outsource complex computation tasks to off-chain market), and cross-chains (Relay, Cosmos, Polkadot). The study[64] has mentioned that the increasing size of the blockchain is also a significant scalability and sustainability issue 

MONIKA et al. 

**9 of 50** 

due to the limited bandwidth and storage space of network peers. Yang[65] discussed that in blockchain, every node needs to store the entire blockchain history. The authors stated that a single node cannot keep on storing the data indefinitely and proposed two ways to resolve this issue: by expanding the blockchain storage or restructuring the blockchain. Smart contract transactions are executed serially[66] by miners hence, impacting the performance of any DASerial execution of SCs cannot take advantage of concurrent multicore architectures. Hence, parallel/concurrent execution of smart contracts similar to the one proposed by[67] needs to be explored further to increase throughput. 

## _Consensus mechanism_ 

Blockchain being a decentralized P2P system, consensus plays a significant role in maintaining data consistency over the peer nodes. In the Bitcoin blockchain, a new block gets added to the chain every 10 min because of the PoW consensus mechanism. Block time is kept 10 min to maintain the security of the chain in the presence of forks. A new block is considered final only after a certain number of blocks are mined on top of it (i.e., 6 in the case of Bitcoin blockchain), thus further increasing the finality time of a transaction. Thus, consensus is a significant factor behind low transaction throughput and poor scalability of the blockchains. Since the inception of blockchain technology, numerous consensus algorithms[27,37,110] (PoS, DPoS, PoET, PoB, PoA, PBFT, Tendermint, Casper) have been proposed, hence choosing the suitable consensus algorithms for developing a blockchain-based application is a challenging task. The authors[27,29] have performed extensive studies on consensus algorithms regarding their designs and impact on different application domains. Wan et al.[37] have classified consensus algorithms into permissionless and permissioned consensus and examined each algorithm’s pros and cons. The authors have also stated that the energy consumption of most consensus algorithms, such as PoS, DPoS, PBFT, Ripple, PoA, Casper, Snow White, Aglorand, and Ouroboros Genesis, is low except for just PoW. The consensus mechanism is also the underlying reason behind the scalability and security issues of PoW-based blockchains. 

## _Energy consumption/High computation cost_ 

PoW-based blockchains maintain the immutability and security of data using a complex problem-solving technique. This problem-solving process leads to enormous energy consumption and is a challenging issue with the increasing adoption of blockchain technology. The authors[33] have analyzed the computing power demanded in the PoW process on the Bitcoin blockchain, considering a period from January 3, 2009 to June, 5 2018. The study concluded that during June 2018, energy consumption was between 15.47 and 50.24 TWh per year for difficulty recalculation, and the maximum demand statistic was between the installed capacities of Finland (approx. 16 GW) and Denmark (approx.14 GW). Energy consumption happens at the consensus layer[36] during the block mining process. The miners spend energy competing for new block creation and, in return, get an incentive if their block gets selected. As all the miners compete and only one miner’s block gets selected, the mining process leads to an enormous waste of resources and questions the sustainability of the blockchain protocol for public blockchains.[68] 

As the development on top of the Ethereum blockchain grows with time, the gas fee is also increasing. Many factors[82] affect the price of Ethereum gas fees, such as the number of miners, pending transactions, the value of the USD/Ether pair, and average electricity prices worldwide. The number of miners and pending transactions are the primary factors for gas fee hikes. The high gas fee is also a significant barrier to public blockchain adoption as it makes the execution of the developed solutions very costly compared to centralized solutions. Blockchain platforms are shifting from PoW to PoS consensus, and newer platforms are being developed using consensus algorithms other than PoW. The merge of the Ethereum mainnet with the PoS beacon chain has reduced the energy consumption of Ethereum by ∼99.95%.[54] 

## _Privacy and anonymity_ 

Blockchain technology provides anonymity (identity privacy) as it uses pseudonyms and not the users’ real identities but does not provide transaction privacy in the case of public blockchains. Blockchain technology suffers from transaction linkability, private key management issues, and non-erasable data against the requirements imposed by GDPR. The authors[69] have extensively analyzed the studies mentioning privacy and anonymity issues and the improvement proposals for the Bitcoin blockchain. The study[70] described different tiers of anonymity (pseudo, set anonymity, full anonymity, confidential transactions) and techniques (pseudonymous addresses, ring signature, mixers, commitments, ZKP, bulletproofs, and stealth addressing) used for achieving different levels of anonymity. They have concluded that most cryptocurrencies use a primitive level of anonymity, that is, pseudo-anonymity. Feng et al.[25] studied the existing surveys related to privacy issues of blockchain. They have done a comprehensive analysis of privacy protection mechanisms related to anonymity and transaction privacy preservation. The authors[24] carried out a systematic study of privacy-preserving research solutions related to blockchain technology. They have provided a taxonomy of 

MONIKA et al. 

**10 of 50** 

privacy-preserving techniques by dividing the proposed solutions into four categories: key management, identity data anonymization, transaction data anonymization, and on-chain data Protection. 

## _Interoperability_ 

Blockchains, by default, do not interact with each other and act as standalone ledgers. This inherent nature of blockchains is the real cause behind their interoperability issues. Numerous blockchains have been designed for different application domains having different requirements.[111] Blockchains need to interoperate so that applications designed using blockchain technology can use each other’s features for example, the interaction between logistics management and supply chain management applications. The authors[52] performed an analytical study of interledger approaches and stated that there are no established standards for combining different blockchains, and interoperability is considered a challenging and active research area. Interoperability challenges and solutions proposed for tackling those are explained in Sections 4.1.4 and 4.3.1, respectively. 

## _Regulatory concerns_ 

The study[56] has mentioned regulatory uncertainty as one of the significant concerns hindering blockchain adoption. Existing regulations do not concern concepts used by Blockchains like cryptographic signatures and smart contracts. Moreover, blockchain is a distributed technology; hence, nodes attached to a public blockchain could be in different states or countries having different laws and regulations. The authors[71] state that the absence of regulations is an enabler for blockchain development as early attempts to regulate a technology before understanding it properly can be disastrous for its development. The studies[4] also pointed out that laws and regulations can affect blockchain technology’s development. Hence, regulatory bodies should pay close attention to its rapid innovation and the systematic risks it can cause to the financial system. The study[66] has stated that in the absence of laws and regulations, blockchain is being used for criminal activities, such as money laundering, the black market, ICO frauds/scams, and Ponzi Schemes. The authors[72] have concluded that research proposals for regulation designs are needed to help prevent fraud, taxation, ownership issues, and ICO regulation. Most of the firms are hesitant to adopt blockchain technology because of the absence of governance and regulations. 

## _Standardization/inexperienced developers_ 

Blockchain is still in its infancy stage. The lack of standardization hinders[56] the thorough understanding of the terminology and interoperability aspect of the technology. Developers[60] face numerous difficulties while developing smart contract applications because of the immutability of SCs after being deployed. Other smart contract-related things like public code, immature compilers, lack of code auditing and verification tools, money-handling, and lack of expertise in SC programming also add to difficulties. The study[50] has mentioned that organizations like the Institute of Electrical and Electronics Engineers (IEEE) Standards Associations, the International Organization for Standardization (ISO), and Standards Australia are working on standardizing blockchain technology. 

## 4.1.3 Issues related to blockchain security 

The security aspect of blockchain technology holds utmost importance as it is a distributed technology, and no trusted third party is responsible for handling the security issues that can arise in a blockchain system. Security issues in blockchain can be related to blockchain protocol/ infrastructure or applications deployed on the blockchain using smart contracts. Most of the studies in the literature have discussed attacks that can happen related to blockchain platforms and countermeasures proposed for such attacks. Some studies discuss attacks specific to the Bitcoin protocol, others discuss security issues related to PoS-based blockchain, and some only mention the smart contract vulnerabilities. This section discusses the survey studies related to blockchain security and then presents the taxonomy of blockchain security issues proposed in this study. The authors[5] have discussed blockchain’s security and privacy aspects. The authors have discussed blockchain’s basic security requirements, such as consistency, tamper-resistance, Distributed Denial-of-Service (DDoS), resistance-to-double spending, and pseudonymity for systems to work correctly. The authors[73] have discussed the resiliency challenges of the Bitcoin blockchain and categorized the resiliency attacks that happened on the Bitcoin blockchain into four categories: system and protocol level, network level, cryptographic level, and other minor system-level threats. Wan et al.[74] have empirically studied bug characteristics related to blockchain systems. They have 

MONIKA et al. 

**11 of 50** 

considered eight open-source blockchain projects and have studied their bug reports from GitHub. The authors have categorized 1108 bug reports into 10 bug categories using card-sorting techniques. They have concluded that the frequency distribution of bugs follows a similar pattern across different projects and programming languages. 

Averin A. and Averina O.[75] have briefly explained the seven types of problems that arise because of the choice of consensus algorithms. The authors have presented 19 types of possible attacks that make blockchain vulnerable. They have also discussed 25 cyber-attacks on blockchain systems over 9 years, from 2010 to 2018. Wang et al.[76] have presented a security analysis of the Bitcoin blockchain at all six layers: application layer, smart contract layer, incentive layer, consensus layer, network layer, and data layer. Zhang et al.[77] have briefly discussed the security and trust issues related to different layers of blockchain and the methods available to address these issues. The authors[78] have presented a stacked model of the security reference architecture (SRA) consisting of network, consensus, replicated state machine, and application layer to identify security vulnerabilities, threats, and countermeasures. They have also embedded the threat risk assessment standard ISO/IEC 15408 with the proposed SRA to present a blockchain-specific version. The study[106] has categorized the 40 Ethereum vulnerabilities according to network, consensus, data, and application layer, along with their causes. The authors have also discussed the relationship between these vulnerabilities and 29 attacks (already happened or possible attacks) accompanying 51 defense mechanisms proposed in the literature for the Ethereum Ecosystem. After analyzing all the survey studies and security solutions found in the literature, we propose a new taxonomy, as shown in Figure 2 for the security issues related to blockchain. This taxonomy considers both infrastructure (blockchain protocol) level and smart contract (application level) attacks and vulnerabilities. None of the studies in the past has classified blockchain security issues considering multiple factors like the level at which vulnerabilities occur (protocol level or application level), attacker type (malicious or criminal), and functionality affected (services or ledger data). This taxonomy divides the security issues into two main categories: Infrastructure Specific (IF) and Smart Contract (SC) specific. 

**F I G U R E 2** A taxonomy for blockchain security issues. 

MONIKA et al. 

**12 of 50** 

## _Infrastructure specific_ 

Next, infrastructure-specific attacks and vulnerabilities found in the literature are discussed. The studies[23,112,79–83] have explored the various attacks happening due to the peer-to-peer nature of blockchain. They have also discussed the attacks due to protocol design, cryptographic vulnerabilities, malicious miners, criminal behavior, and smart contracts applications. Wang et al.[84] have studied how random numbers are used in ECDSA and can lead to the leakage of private keys. The authors[85] provide systematic studies for long-range attacks of PoS protocols and discusses the mitigation techniques proposed in the literature. The authors[86] have presented state-of-the-art security concerns in blockchains due to consensus mechanisms, cryptocurrency, and security attack types. Our study segregates infrastructure-specific attacks/vulnerabilities into five categories: Consensus related attacks, Cryptographic related attacks, Service attacks, Criminal activities, and Malicious miners/Node attacks. Table 3 defines all types of infrastructure-specific attacks/vulnerabilities and reference studies discussing these attacks. We have arranged the security issues in the decreasing order of the number of citations. 

## _Smart contract specific_ 

This section discusses smart contract security issues and survey studies analyzing smart contracts from a security perspective. The authors[100] have explained the key blockchain features that may lead to security issues, such as decentralization and tamper-proofing, open-source code, public ledgers, immaturity of blockchain platforms, smart contract languages, and misunderstanding of common practices. Zheng et al.[101] discussed challenges faced and recent advances in all phases of a smart contract: creation (readability, functional issues), deployment (contract correctness, dynamic control flow), execution (trustworthy oracle, transaction ordering dependencies, execution efficiency), and completion phase (privacy security, Ponzi and honeypot scams). The study[102] has comprehensively surveyed Ponzi schemes on the Ethereum blockchain. The authors collected 184 smart contracts implementing Ponzi schemes and categorized them according to the money distribution pattern: tree-shaped, chain-shaped, water-fall, handover, and others. They have also provided users with two recommendations to detect these schemes: check the advertisements properly and analyze the contract code and the transaction log. Angelo et al.[103] explored the quantitative and temporal features of SCs to understand the different types of contracts deployed on the Ethereum blockchain. They categorized the SCs into the following categories: loners (never called), destructed contracts (called self-destruct), mayflies (extremely short lifespan), sleepers (contract with a long sleep time: time between creation and first call), bonker contracts (with no functional deployed code), breeders (a contract that creates at least 1000 contracts), active contracts (if it receives at least one call during its lifetime), busy bee (a contract with more than 1000 interactions) and casual workers (contracts with less than 1000 interactions). 

The study[104] identified 28 security smells in SCs, and categorized those according to the context they were found in, such as dependence on the environment, design and deployment issues, misuse of trust, control of execution, reentrancy, unsafe external interaction, and vulnerable coding practice. Zou et al.[60] have performed an analytical study to understand developers’ challenges while developing SCs. The authors conducted semi-structured interviews with 20 Github developers and industry professionals. They used the card sorting technique to group the interview results into six development challenges groups: Security, debugging, programming language, Ethereum virtual machines, gas, online learning resources and community support. In this study, we divided the SC-specific issues found in the literature into six categories: untrustworthy oracles, under-optimized SCs, SC vulnerabilities, inexperienced developers, criminal SCs, and scams. SC vulnerabilities are the design defects found in smart contracts, which have led to considerable losses in the past and can be risky in the future if not appropriately tackled before the deployment of smart contracts. Table 4 presents all types of SC security issues. 

## 4.1.4 Interoperability issues related to blockchain systems 

Originally, Blockchain technology was used for financial services, but due to its fast-paced development, it is now being used in other domains as well. Many organizations are incorporating blockchain technology due to its P2P, decentralized, immutable, auditable, transparent, and secure nature. Industries are using it for data provenance, tractability, data management, privacy perseverance, and increased efficiency. Many survey studies[1,3,47,50,65,68,133–136] have discussed the applications of blockchain technology in different domains like Healthcare, E-governance, IoT, Identity Management, Energy sector, Smart Cities and Supply chain. However, as every domain has different requirements, blockchain development has led to siloed blockchain systems over the years. Blockchain systems are standalone ledgers and do not interact with external sources, legacy systems, or other blockchain systems. We are witnessing scattered blockchains with different 

MONIKA et al. 

**13 of 50** 

**TA B L E 3** Blockchain infrastructure-specific issues. 

||~~**S.N.**~~<br>~~**Security issue**~~<br>~~**Definition**~~<br>~~**Citations**~~<br>1<br>Double<br>spending<br>If a receiver is optimistic and releases the product or<br>amount before the sender’s transaction is mined and added<br>to the block, the sender can send the same transaction to<br>another receiver and double-spend.<br>[5,13,23,75,112,80,81,83,85–90]<br>2<br>Selfish mining<br>A selfish miner doesn’t broadcast his mined blocks and<br>~~keeps mining them on his private chain to increase his~~<br>rewards and stale blocks in the blockchain.<br>[23,73,76,78,112,81,83,85–87,89,91–93]<br>3<br>51% attack or<br>Hash rate<br>An attacker or a mining pool attains the majority of the<br>mining hash rate of the network to manipulate the<br>blockchain.<br>[5,13,23,75,79,82,83,85–86,89–90,92–93,113]<br>4<br>DDOS attack<br>When multiple systems flood a network resource with what<br>are known as connection requests,messages,or other types<br>of communication packets, the goal of this type of attack is<br>to slow down or crash the system.<br>[5,13,23,106,73,75,76,78,112,80,83,89,93]<br>5<br>Eclipse attack<br>An attacker gains control of all of a node’s connections to<br>the network and controls the node’s view of the distributed<br>ledger and network operations.<br>[13,23,75,76,78,112,80–81,89,93–94]<br>6<br>Sybil attack<br>An attacker creates multiple pseudonymous identities from<br>~~a single node on the blockchain and tries to deceive genuine~~<br>users. It is very similar to a phishing attack.<br>[13,23,73,75,76,80,82,85,89–90,93]<br>7<br>Routing attack<br>An attacker hijacks the traffic of the network and routes it to<br>somewhere else. It is considered as part of the<br>man-in-middle attack.<br>[13,23,73,75,78,112,82,89]<br>8<br>Bribery attack<br>Also called a short-range attack. Validators or minors are<br>bribed to work on specific blocks or forks. An attacker can<br>pay an amount equal to or more than the block reward to<br>dishonest miners.<br>[23,73,75,76,78,85]<br>9<br>Finney attack<br>When a malicious miner does not broadcast a pre-mined<br>block, and it contains a fraudulent transaction. The<br>pre-mined block is kept secret until goods or benefits are<br>received for the coins and broadcasted.<br>[13,23,73,75,80,89]<br>10<br>Vector 76 attack<br>Combination of Double spend and Finneyattack. Also<br>known as the Single Confirmation attack.<br>[13,23,73,75,80,86]<br>11<br>Blockchain fork<br>When the nodes in the network have different views about<br>the state of the blockchain, it causes a split in the network.<br>Usually, it is unintentional and caused due to flaws in the<br>network.<br>[79,81,83,88,95]<br>12<br>Grinding attack/<br>precomputation<br>attack<br>If a PoS-based blockchain uses only the previous block’s<br>hash value while selectingthe validator,then an attacker<br>can somehow manipulate the hash values by adjusting the<br>block content in a few attempts to his benefit.<br>[27,76,78,112,85]<br>13<br>Time hijacking<br>When a set of miners collude and try to skew/distort a target<br>node’s timestamp by providing wrong time details/incorrect<br>time. A node uses a timestamp to validate new blocks.<br>Hence, now, the target node would reject the new blocks<br>with a timestamp greater than a pre-decided timestamp.<br>[23,73,112,80,89]<br>14<br>Transaction<br>malleability<br>When an attacker can alter a transaction before it is added<br>to a valid block,sender address,receiver address,and<br>amount cannot be altered, but other transaction parameters<br>can be altered, generating a new transition id.<br>[23,73,75,80]<br>~~(Continues)~~|
|---|---|



MONIKA et al. 

**14 of 50** 

**TA B L E 3** Continued. 

||~~**S.N.**~~<br>~~**Security issue**~~<br>~~**Definition**~~<br>~~**Citations**~~<br>15<br>Punitive and<br>feather-forking<br>An adversary with a hash rate_>_50% of the network can blacklist transactions from<br>a specific user and fork if other miners resist doing so. Even if an adversary has less<br>than 50% of hash power, he can insist on blacklisting certain transactions and<br>retaliate forking if required.<br>[23,73,78,112]<br>16<br>Block withholding<br>A mining pool member never submits the mined block by not sharing<br>~~Full-Proof-of-Works (FPoWs) and submits Partial-Proof-of-Works (PPoWs) to~~<br>sabotage the pool’s revenue.<br>[23,73,96,97]<br>17<br>Wallet thefts<br>Software wallets controlled by third parties can be compromised. An adversary can<br>steal or destroy private keys related to wallets.<br>[23,73,83,89]<br>18<br>Weak ECDSA<br>Reusingthe same random number in ECDSA can lead toprivate keyleakage or<br>wallet theft and cause a huge loss to the wallet owner.<br>[75,80,84]<br>19<br>Simple long-range<br>attack<br>Long-range attacks in PoS-based blockchains are similar to selfish mining in<br>PoW-based blockchains as the attacker is adding new blocks secretly into his<br>private chain by manipulating timestamps and validating blocks ahead of time.<br>[76,112,85]<br>20<br>Replayattack<br>When an attacker spoofs the communication between twogenuine users andgains<br>access to hash keys to act as a valid user.<br>[80,82,98]<br>21<br>Posterior<br>corruption<br>When a validator who has left the network colludes with the attacker and provides<br>his key, the attacker can create and sign the previously validated blocks on his<br>private chain.<br>[5,78,85]<br>22<br>Nothingat stake<br>A node in a PoS-based blockchain can extend one or more blocks when a fork<br>happens in the network because it does not have anything at stake.<br>[75,78,112]<br>23<br>DNS attack<br>An attacker can manipulate the attack surfaces of DNS and feed a new joiner of the<br>network with a fake list of peers, fake blocks, and invalid transactions.<br>[78,112,83]<br>24<br>Race attack<br>An adversary can mine on a private chain until the required number of<br>~~confirmations for a transaction, retrieve goods, and move his computing power~~<br>from the malicious chain to the main chain.<br>[73,75,89]<br>25<br>Stake bleeding<br>A malicious minor does not validate a new block on the main chain during his turn<br>but validates a block on his private chain. Gradually, the attacker’s stakes decrease<br>in the main chain but increase in his private chain; thus, he has more chances of<br>getting elected as a validator. An attacker in this way can stall the main chain.<br>[85,98]<br>26<br>Brute-force<br>When a minor has enough resources and mines privately and does not release<br>~~privately mined blocks and tries to launch a double spend by later on releasing a~~<br>chain of blocks long enough.<br>[23,73]<br>27<br>Hash length<br>extension attack<br>A hash of a signed message can be modified by appending some malicious data to<br>the original message without knowing the shared secret. SHA 256 is also<br>susceptible to length extension attacks.<br>[75,80]<br>28<br>Liveness attack<br>In PoS-based blockchains, most of the network’s validators start acting maliciously<br>~~by not validating blocks and trying to halt the creation of new blocks by not~~<br>participating in the validation process.<br>[81,85]<br>29<br>Phishing attack<br>When an attacker creates a fake version of the wallet or website and tries to fool<br>users and steal their funds.<br>[76,82]<br>30<br>Fork after<br>withholding<br>Combination of selfish mining and block withholding. Attackers divide their<br>mining power into two: honest miningand malicious mining. Malicious miners<br>join a target pool, do not share FPOW, and withhold the block. Later on, they can<br>release the pre-mined block to create a fork in the network.<br>[23,73]<br>31<br>Ransomware<br>The criminal uses malware to exploit a vulnerability in windows and asks for<br>bitcoin ransom.<br>[73,81]<br>32<br>Black market<br>Cryptocurrencies can be used easilyin the underground market due to their<br>anonymity properties and cause harm to social security.<br>[81,99]|
|---|---|



MONIKA et al. 

**15 of 50** 

**TA B L E 4** Smart contract-specific issues of blockchain. 

||||~~**S.N.**~~<br>~~**Security issue**~~<br>1<br>Untrustworthy oracles<br>2<br>Under optimized SC<br>3<br>Inexperienced<br>developers/Lack of<br>community support<br>4<br>Criminal SCs<br>**Scams**<br>5<br>Ponzi schemes<br>6<br>Honeypot<br>~~**SC vulnerabilities**~~<br>7<br>Reentrancy<br>8<br>Timestamp<br>~~dependency/Time~~<br>constraint<br>9<br>Unchecked math<br>10<br>Transaction-ordering<br>~~dependency~~<br>11<br>Generating<br>randomness<br>12<br>Exception disorders/<br>Mishandled exceptions<br>13<br>Call to the unknown<br>14<br>Tx. origin<br>15<br>Stack size limit|~~**Description**~~<br>~~**Citations**~~<br>Endpoints or third-party devices that provide<br>information to a smart contract on the blockchain<br>can act maliciously.<br>[46,56,66,101,105,114]<br>SCs having dead code, expensive operations, loop<br>~~fusion, and repeated confusion can lead to gas~~<br>wastage.<br>[89,101,115,116]<br>Due to the technology’s immaturity and lack of<br>support for debugging in Solidity language,<br>developers often make mistakes.<br>[56,60,117]<br>SCs are designed to leak confidential information<br>of their users.<br>[66,81,89,115]<br>An investment fraud that pays the older investors<br>~~with the revenue collected from new investors. All~~<br>types of Ponzi schemes are live on the blockchain.<br>[101,102,114,118–119]<br>Smart contracts are designed vulnerable<br>purposely to trap their users.<br>101,114<br>When during a non-recursive function call, a<br>callee can re-enter into the function before its<br>termination due to the fallback mechanism in<br>Solidity.<br>[106,81,90,93,95,100,101,104–105,120–130]<br>Time constraints use block timestamps for<br>~~triggering critical conditions, and malicious~~<br>miners can alter block timestamps.<br>[106,81,90,93,100,101,104–105,122,124–127,131]<br>Uint in Solidity suffers from overflow and<br>underflow errors and needs proper checks during<br>implantation.<br>[90,100,104,105,120,122–123,125,127,131]<br>Transaction order in a block can never be guessed<br>~~by a user and depends on the miner of that~~<br>specific block.<br>[106,81,90,93,100,104,105,114,120–121,126–127]<br>A Pseudo random generator implemented using<br>SCs uses block hash, block number, and<br>timestamps of future blocks and can be altered<br>maliciously by miners.<br>[106,81,100,104,105,120,123–125]<br>An exception occurring in the called contract, if<br>not handledproperly,can halt its execution<br>abruptly without returning an exception to the<br>caller contract.<br>[81,100,104,105,124–126]<br>Calls to external contracts can cause undesired<br>vulnerabilities like the execution of malicious<br>code.<br>[81,104,105,120–122,130]<br>Tx. origin provides the address of the call’s<br>~~initiator, not the immediate caller, and should not~~<br>be used for authorization purposes.<br>[106,104,105,120,122–123,131]<br>An adversary can invoke a victim contract on an<br>almost full call stack, and if an exception is not<br>handled correctly by the victim contract, the<br>adversary can produce his desired output.<br>[60,106,81,104,105,120,127]||||
|---|---|---|---|---|---|---|---|



~~(Continues)~~ 

MONIKA et al. 

**16 of 50** 

## **TA B L E 4** Continued 

||~~**S.N.**~~<br>16<br>17<br>18<br>19<br>20<br>21<br>22<br>23<br>24<br>25<br>26<br>27|~~**Security issue**~~<br>~~**Description**~~<br>~~**Citations**~~<br>Exposed functions/<br>Keeping secrets<br>Even a private variable’s value can be inferred on blockchain as<br>transaction data is always public.<br>[81,101,104,105,122]<br>Type casts<br>When the caller doesn’t cast the callee’s interface into the callee’s<br>address while performing direct calls.<br>[106,81,104,105,122]<br>Gasless send<br>A transaction fails if enough gas is not provided for a function call.<br>[104,105,124,131]<br>Ether lost<br>Ethers mistakenlysent to an orphan(not associated with anyuser<br>or contract account) cannot be retrieved.<br>[106,81,104,105,120,123]<br>Immutable<br>bugs/mistakes<br>SCs cannot be patched after being deployed. So if a deployed SC<br>contains some bug, it can be exploited by an attacker.<br>[81,104,105,120]<br>Unpredictable state<br>~~(Dynamic Libraries)~~<br>Smart contract state cannot be determined prior as it depends on<br>~~the order of transactions selected by miners, and dynamic libraries~~<br>can deceive users as its content can be changed.<br>[104,105,122–123]<br>Denial-of-Service<br>(DoS)<br>An attacker can launch a DoS attack on an SC by providing<br>computationally expensive data and blocking its normal working.<br>[106,104,105,122–123]<br>Gas costly pattern<br>Thegas-costly pattern in smart contracts results in out-of-gas<br>exceptions.<br>[105,122,123,131]<br>External calls/<br>Delegate Calls<br>External codes can execute malicious code and are considered risky.<br>[106,90,104,105,124]<br>Send instead of transfer<br>When usingsend,the returned value needs to be checked for<br>transaction failure or success while transfer checks it automatically.<br>[104,105,122]<br>Redundant fallback<br>Fallback function can be abused to become the owner of the<br>contract, if not used properly by the developer of the contract.<br>[104,105,122]<br>Unprotected<br>self-destruct<br>When an attacker not the owner of the SC can call self-destruct<br>and kill the SC<br>[100,132]|||
|---|---|---|---|---|



characteristics (transaction format, hash algorithms, consensus algorithms, throughput, finality) and distinct networks unable to leverage each other’s functionality. Industries and application domains require different types of blockchain systems with unique features; hence, one platform cannot fulfill all the requirements. Rather a collaboration of dissimilar blockchains interacting with each other would be the best solution. Interoperability among blockchain systems is the top concern of decision-makers interested in implementing blockchain solutions for their organizations. Blockchains are considered interoperable when users of one blockchain can send/ receive assets or data from the users on another blockchain. As per the study,[137] blockchain interoperability can be achieved from three different perspectives: 

1. Interoperability between a blockchain and a traditional centralized system. 

2. Interoperability between two blockchain systems (both permissionless, both permissioned, or one permissionless, and one permissioned). 

3. Interoperability between smart contracts of decentralized applications. 

Blockchain systems are not interoperable because of a lack of standardization,[52,56] as the technology is still in its infancy stage. Blockchain platforms differ at the architectural level, for example consisting of a chain of blocks or a DAG. At the transaction level, they reach a consensus using different consensus mechanisms. Blockchain platforms use different hash functions and support different languages for application development. Due to the lack of standards, blockchain developers utilize its decentralized nature to develop various decentralized applications. Lack of data standardization at the transaction level in the different blockchains hinders achieving interoperability. Blockchains should use the same consensus in order to interoperate, but as different blockchain platforms use different consensus mechanisms, it adds difficulty in achieving interoperability. Interoperability can be achieved at different levels like business, legal, technical, and semantic levels. Most of the literature work is related to technical and semantic level interoperability. Blockchain interoperability can be achieved in the following two types: 

MONIKA et al. 

**17 of 50** 

1. Asset exchange/transfer: Exchange of assets between different blockchains without using any trusted third party. for example, sending ether to a user on the Ethereum blockchain and getting bitcoin in return on the Bitcoin blockchain or using bitcoin in a decentralized application deployed over Ethereum. With the transfer, values are moved from one ledger to another. It is achieved by locking the original value on one ledger and creating the corresponding value on another. 

2. Data transfer: Reading the data of one blockchain using a smart contract on another blockchain or executing something on one blockchain that affects the other blockchain. 

Data transfer-related interoperability is difficult to achieve but will be more beneficial as its use cases are more advanced than simple asset exchange. Interoperability between different blockchain systems is also addressed as interledger/inter-blockchain and cross-chain communication. Cross-chain approaches need to be leveraged to enhance blockchain interoperability. Section 4.3.1 explains the solutions proposed by researchers and industry people to make blockchains interoperable. 

## **4.2 Status of the development of blockchain platforms** 

Numerous blockchain platforms have been developed since the inception of the Bitcoin blockchain in 2009. We focused this section on studying the available platforms in detail and providing a detailed analysis with a newer perspective. 

## 4.2.1 Different types of platforms with their characteristics and application areas 

Blockchain platforms from the point of view of governance are of three types[4,9,35,65] : Public (Permission-less), Private (Permissioned), and Consortium (Hybrid). 

## _Public blockchain platforms_ 

These platforms are open-source; anyone can download and join the respective blockchain. Users of a public blockchain can transact, mine, or perform auditing on the chain, that is, read and write access is available to every platform user. Every transaction that happens on a blockchain is logged and remains there forever. Hence, public blockchains are transparent and can be audited for any transaction that happened on the network. Every user has his copy of the blockchain locally, synchronized with the global copy using complete decentralized consensus. Public blockchains are considered anonymous because only a user’s blockchain address is visible on the network. That address is not linked to a user’s identity; a single user can hold multiple addresses. However, since blockchain data is publicly available, all the transactions are carried out from an address, and anyone can see its balance. Hence, public blockchains are pseudo-anonymous. Anonymous users can act maliciously, but as blockchain is secured using PoW, its data remains safe until someone acquires 51% of the network’s hash power. Public blockchains have low transaction throughput due to their larger P2P network size and computationally complex consensus mechanism. Public blockchain platforms are of two types: permissioned public and permissionless public. Bitcoin and Ethereum are two prominent permissionless public blockchains released in 2009 and 2015, respectively. Ripple is an example of a permissioned public blockchain. 

Some of the applications of public blockchains are as follows: Augur[138] is an Ethereum-based decentralized prediction market platform allowing users to create a prediction market on any topic. Storj[139] is an open-source, decentralized cloud storage platform built on the Ethereum blockchain. Uniswap[140] is a fully decentralized cryptocurrency exchange allowing traders to trade ERC-20 tokens on the Ethereum blockchain directly from their wallets. Brave[141] is a decentralized web browser that maintains user privacy by blocking ads and website trackers. 

## _Private blockchain platforms_ 

A single organization governs private blockchains. Participants of a private blockchain are known in advance and are pre-approved. Consensus on the state of blockchain is achieved using voting or multiparty consensus, hence does not require computing complex mathematical puzzles. Each node connected to the blockchain can participate in a consensus mechanism, but only authorized nodes can perform a write operation. Private blockchains mostly use Proof of Stake (PoS) or Practical Byzantine fault (PBFT) as their consensus algorithms. PoS blockchains consume less power than Pow-based blockchains due to voting-based consensus. Private blockchains are partially decentralized as validators of the chain are 

MONIKA et al. 

**18 of 50** 

restricted. Private blockchains scale better than public ones due to their smaller network size and less complex consensus mechanisms. They are more susceptible to attacks due to the same reasons. Private blockchains have a throughput higher than public blockchains because of their consensus mechanism. They are mainly used for private sharing or exchange of data. Ripple, Hyperledger, and Corda are some examples of private blockchains. They are also of two types: permissioned private and permission-less private. Hyperledger is an example of a private permissioned blockchain, and the LTO network is a permissionless private blockchain. 

Some of the applications of private blockchains are: Walmart[1,142] has developed private blockchains based on Hyperledger Fabric for tracking the provenance of their products. Xage Security[143] provides blockchain-enabled security solutions for industrial enterprises and operations. Blockgraph[144] is a blockchain platform for the Tv industry to share information and insights across their collective platforms without disclosing recognizable user data to third parties. 

## _Consortium blockchain platforms_ 

Consortium blockchains, also known as federated blockchains, are operated by a group of people or multiple organizations. They are partially private and permissioned since more than one organization controls the governance. Multi-signature schemes are used for block validation in consortium blockchains. Consortium blockchain nodes are predefined with granted read and write permissions according to user roles. Consortium blockchains work well in the case of highly regulated businesses for cross-company and cross-discipline solutions. Consortium blockchains improve workflow, traceability, and responsibility among organizations. Hyperledger, Corda, and Quorum platforms work as consortium blockchains as well. 

## 4.2.2 Techniques, algorithms, and protocols used by blockchain platforms 

To enhance distributed ledger technology, numerous blockchain platforms are being developed. Most of the blockchain platforms are developed by the industry to improve the security, privacy, throughput, energy consumption, and transaction latency of blockchain platforms. We have used the Coinmarketcap[19] website to extract the blockchain platform list. It is a website that lists most blockchain platforms/projects along with the necessary information about a platform/project like market cap, volume, official website URL, link to whitepaper, GitHub, community, and project explorer. It lists two types of cryptocurrencies, that is, coins and tokens. A _coin_ is the native currency of a blockchain like bitcoin and Ether. A _token_ is a cryptocurrency built on another blockchain like the ERC-20 token built on the Ethereum blockchain. We have considered only the coins for our study as we need information about blockchain platforms rather than the apps developed using those platforms. Coins on Coinmarketcap are listed in descending order of their market cap. The total market cap denotes the popularity and influence of coins in society and the blockchain industry. We have considered the first fifty coins listed as of June 2022. Only the blockchain platforms with their mainnet already launched are considered in this study. Next, we have considered the Github commits of the coins as a screening criterion and have finally considered 37 platforms with Github’s highest commits. We have excluded three platforms, bitcoin cash, bitcoin SV, and ecash, from our study since these are forks of Bitcoin, which is already considered in the study. So, we have finalized 37 platforms from Coinmarketcap and have considered three additional enterprise/private blockchain platforms, that is, Hyperledger, Quorum, and Corda, to study their characteristics further. The reason for choosing these three platforms is that these are the prominently used enterprise blockchain platforms in the blockchain industry. 

We have extracted information about the mainnet launch date of the platform, architecture type (permission-less or permissioned), whether the platform support smart contract, consensus protocol used by the platform, programming language support for the development of smart contracts, implementation language of the platform, the hash function used by PoW based platforms, the official website of the platform. All the required information is extracted using official websites, community blogs, project explorers, whitepapers, and GitHub repositories of the platforms. Table 5 shows the information regarding platforms. The hash function is mentioned only in the platforms that use PoW and require a hash algorithm while mining a new block. Abbreviations used in Table 5 column headings are as follows: Type: Architecture Type, SS: SC Support, CP: Consensus Protocol, PL: Programming Language, SL: Scalable, IL: Implementation Language of the platform, HF: Hash Function used, Link: Platform official website. 

Next, we discuss the available research studies on blockchain platforms, their characteristics, cryptographic algorithms, and comparative analysis of platforms. Pervez et al.[145] presented a comparative analysis of different Directed Acyclic Graph (DAG) based blockchains, including Nxt, IOTA, Orumesh, DagCoin, Byteball, Nano, and XDAG. The 

MONIKA et al. 

**19 of 50** 

||**Platform**<br>**Year**<br>**Type**<br>**SS**<br>**CP**<br>**PL**<br>**SL**<br>**IL**<br>**HF**<br>**URL**|Bitcoin<br>2009<br>Permission-less<br>N<br>PoW<br>bitcoin-script<br>N<br>C++<br>SHA-256<br>https://bitcoin.org/|Litecoin<br>2011<br>Permission-less<br>N<br>PoW<br>N<br>C++<br>scrypt<br>https://litecoin.org/|Ripple<br>2012<br>Permissioned,<br>N<br>RPCA<br>X<br>Y<br>C++<br>–<br>https://ripple.com/xrp/|Public|Dogecoin<br>2013<br>Permission-less<br>N<br>PoW<br>X<br>N<br>C++<br>scrypt<br>https://dogecoin.com/|Dash<br>2014<br>Permission-less<br>N<br>PoW<br>X<br>N<br>C++<br>X11<br>https://www.dash.org/|Monero<br>2014<br>Permission-less<br>N<br>PoW<br>X<br>N<br>C++<br>RandomX<br>https://www.getmonero.org/|Stellar<br>2014<br>Permission-less,<br>Y<br>SCP<br>X<br>Y<br>C++, GO, Python<br>–<br>https://www.stellar.org/|private|Ethereum 1.0<br>2015<br>Permission-less<br>Y<br>PoW-Ethash<br>Solidity<br>N<br>Go<br>Keccak256<br>https://ethereum.org/|Qtum<br>2016<br>Permission-less<br>Y<br>PoS<br>bitcoin-script<br>N<br>C++<br>–<br>https://qtum.org/|Zcash<br>2016<br>Permission-less<br>N<br>PoW<br>X<br>N<br>C++, Rust, Python<br>Equihash<br>https://z.cash/|Decred<br>2016<br>Permission-less<br>N<br>Hybrid PoW &<br>X<br>N<br>Go<br>–<br>https://www.decred.org/|PoS|IOTA<br>2016<br>Permission-less<br>Y<br>DAG- PoW<br>X<br>N<br>Java<br>Kerl[keccak_384]<br>https://www.iota.org/|Quorum<br>2016<br>Permissioned,<br>Y<br>PBFT<br>Solidity<br>N<br>Go<br>–<br>https://consensys.net/quorum/|private|private|Corda<br>2016<br>Permissioned,<br>Y<br>#<br>Kotlin, Java<br>Y<br>Kotlin<br>–<br>https://www.r3.com/corda-platform/|private|Cardano<br>2017<br>Permission-less<br>Y<br>Ouroboros PoS<br>Plutus and<br>Y<br>Haskell<br>–<br>https://cardano.org/|Cardano<br>2017<br>Permission-less<br>Y<br>Ouroboros PoS<br>Plutus and<br>Y<br>Haskell<br>–<br>https://cardano.org/|Marlowe|Marlowe|EOS<br>2018<br>Permissioned,<br>Y<br>DPoS<br>C++<br>Y<br>C++<br>–<br>https://eos.io/|Public|TRON<br>2018<br>Permission-less<br>Y<br>DPoS<br>Solidity<br>Y<br>Java<br>–<br>https://tron.network/|TRON<br>2018<br>Permission-less<br>Y<br>DPoS<br>Solidity<br>Y<br>Java<br>–<br>https://tron.network/|TRON<br>2018<br>Permission-less<br>Y<br>DPoS<br>Solidity<br>Y<br>Java<br>–<br>https://tron.network/|BlockStack<br>2018<br>Permission-less<br>Y<br>PoX<br>Clarity<br>–<br>Rust<br>–<br>https://www.blockstack.org/|(Stacks)|Tezos<br>2018<br>Permission-less<br>Y<br>PoS<br>Michelson<br>N<br>Ocaml<br>–<br>https://tezos.com/|Tezos<br>2018<br>Permission-less<br>Y<br>PoS<br>Michelson<br>N<br>Ocaml<br>–<br>https://tezos.com/|Vechain<br>2018<br>Permission-less<br>Y<br>PoA<br>Solidity, Vyper<br>Y<br>Go<br>–<br>https://www.vechain.org/|Hyperledger<br>2018<br>Permissioned,<br>Y<br>PBFT<br>Go, Java,<br>Y<br>Go<br>–<br>https://www.hyperledger.org/use/fabric|private<br>Javascript|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|||||||||||||||||||||||||||||||||||||||



|**20 of 50**<br>**TA B L E 5**<br>Continued.<br>**Platform**<br>**Year**<br>**Type**<br>**SS**<br>**CP**<br>**PL**<br>**SL**<br>**IL**<br>**HF**<br>**URL**|Near<br>2018<br>permission-less<br>Y<br>Thresholded<br>Rust,<br>Y<br>Rust<br>–<br>https://near.org/|Near<br>2018<br>permission-less<br>Y<br>Thresholded<br>Rust,<br>Y<br>Rust<br>–<br>https://near.org/|Near<br>2018<br>permission-less<br>Y<br>Thresholded<br>Rust,<br>Y<br>Rust<br>–<br>https://near.org/|PoS<br>AssemblyScript|Hedera<br>2018<br>Permissioned,<br>Y<br>Asynchronous<br>Solidity<br>Y<br>JavaScript<br>–<br>https://hedera.com/|public<br>Byzantine fault|tolerant (aBFT)|Zilliqa<br>2019<br>Permission-less<br>Y<br>PoW<br>Scilla<br>Y<br>C++<br>Ethash<br>https://www.zilliqa.com/||Cosmos<br>2019<br>Permission-less<br>Y<br>Tendermint<br>Rust<br>Mainly Go<br>–<br>https://cosmos.network/|Theta<br>2019<br>Permission-less<br>Y<br>MBFT<br>Solidity<br>–<br>Go & C<br>–<br>https://www.thetatoken.org/|Algorand<br>2019<br>Permission-less<br>Y<br>Pure PoS<br>TEAL<br>Go<br>–<br>https://algorand.foundation/|Flow<br>2019<br>Permission-less<br>Y<br>PoS variant<br>Cadence<br>Y<br>Go<br>https://flow.com/|Flow<br>2019<br>Permission-less<br>Y<br>PoS variant<br>Cadence<br>Y<br>Go<br>https://flow.com/|HotStuff|fantom<br>2019<br>Permission-less<br>Y<br>Lachesis-Asynchronous<br>Solidity<br>Y<br>Go<br>–<br>https://fantom.foundation/|Byzantine fault|tolerant (aBFT)|XDC network<br>2019<br>Permissioned<br>Y<br>delegated<br>Solidity<br>Y<br>Go<br>Keccak-256, ECDSA<br>https://www.xdc.org/|proof-of-stake|proof-of-stake|(XDPoS)|klaytn<br>2019<br>Permission-less<br>Y<br>Modified<br>Solidty<br>Y<br>Go<br>–<br>https://www.klaytn.foundation/|Istanbul BFT|Elrond<br>2020<br>Permission-less<br>Y<br>PoS<br>Rust, C/C++<br>Y<br>Go<br>–<br>https://elrond.com/|(MultiverseX)|Binance chain<br>2020<br>Permission-less<br>Y<br>PoSA<br>Solidity<br>Y<br>Go<br>–<br>https://www.binance.org/en/|Filecoin<br>2020<br>Permission-less<br>Y<br>PoRep, PoSt<br>X<br>-<br>Go<br>–<br>https://filecoin.io/|Avalanche<br>2020<br>Permission-less<br>Y<br>A&S<br>Solidity<br>Go<br>–<br>https://www.avalabs.org/|Polkadot<br>2020<br>Permission-less<br>Y<br>GRANDPA and<br>Rust, Solidity<br>Y<br>Rust<br>–<br>https://polkadot.network/|BABE|Solana<br>2020<br>Permission-less<br>Y<br>hybrid PoS and<br>Rust, C, C++<br>Y<br>Rust<br>SHA-256<br>https://solana.com/|Proof of History|Polygon<br>2020<br>Permission-less<br>Y<br>PoS<br>Solidity<br>Y<br>Golang<br>–<br>https://polygon.technology/||Casper<br>2021<br>Permission-less<br>Y<br>PoS<br>Rust<br>Y<br>Rust<br>–<br>https://casper.network/en-us/|MONIKA<br>Mina<br>2021<br>Permission-less<br>Y<br>Ouroboros<br>Samisika<br>TypeScript<br>Y<br>Ocaml<br>Poseidon<br>https://minaprotocol.com/|et al.<br>Abbreviations:−, information not available; A&S, Avalanche & Snowman consensus protocol; DBFT, Delegated BFT; DPOS, Delegated PoS; MBFT, Multi-BFT consensus; N, NO; PoRep, Proof of Replication; PoSA,<br>Proof of Staked Authority; PoSt, Proof of Spacetime; PoX, Proof of Transfer; RPCA, Ripple Protocol Consensus Algorithm; SCP, Stellar Consensus Protocol; VBFT, Verifiable random function BFT; X, does not support.|21613915, 2024, 11, Downloaded from https://onlinelibrary.wiley.com/doi/10.1002/ett.5037 by <Shibboleth>-member@iu.de, Wiley Online Library on [22/06/2026]. See the Terms and Conditions (https://onlinelibrary.wiley.com/terms-and-conditions) on Wiley Online Library for rules of use; OA articles are governed by the applicable Creative Commons|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
||||||||||||||||||||||||||||||||||||||||License|



MONIKA et al. 

**21 of 50** 

authors have compared data structures, consensus algorithms, transaction validation, ledger size, scalability, and popularity, and outlined some best practices for designing a DAG-based blockchain architecture. Dong et al.[146] presented a performance evaluation framework for DAG-based distributed systems named DAGBENCH. The authors have evaluated the performance of IOTA, Nano, and Byteball in terms of throughput, latency, scalability, success indicator, resource consumption, transaction fee, and transaction data size using two types of workload (value/data transfer oriented & and transaction query oriented). The authors pointed out the advantages, shortcomings, and future development scope of all three systems. They have concluded that IOTA consumes the highest computational and memory resources, Nano cannot transfer arbitrary data, and Byteball’s performance is the worst among all three systems under consideration. Saad et al.[147] have presented the overview of IOTA and briefly discussed some of the other DAG systems, like Byteball, Dagcoin, and Hashgraph Consensus. Dinh et al.[148,149] have discussed four main concepts required for understanding blockchain platforms: distributed ledger, consensus, cryptography, and smart contract, and compare the available platforms according to these concepts. They presented the first evaluation framework, BLOCKBENCH, for analyzing the performance of private blockchains. BLOCKBENCH uses APIs to integrate any private blockchain and benchmark its overall and component-wise performance using different workloads. They present the comparative analysis of Ethereum, Parity, and Hyperledger with two macro and four micro benchmarks. The authors have concluded that blockchains are far from traditional databases in processing workloads. 

Ciatto et al.[150] have applied the LINDA coordination model on top of Ethereum, Hyperledger, and R3 Corda to assess the coordination among distrusted processes. The authors have concluded that blockchain is a promising technology to work as a pillar of the distributed coordination process. However, some open issues are still there, and further research is required in this field. Aldweesh et al.[151] implemented OpBench, a CPU performance benchmark system, to estimate the CPU efforts required to execute each opcode on EVM. OpBench runs independently of the blockchain platforms and can help miners choose which smart contract to execute to gain higher rewards. The authors showed a significant difference in rewards obtained per unit of CPU time for executing different opcodes. Performance varies across different clients, and Go clients perform faster than the Python client, irrespective of the OS. The ratio between the fee obtained and the CPU invested matters the most, not the CPU time spent. 

Qiu et al.[152] performed the SWOT analysis of two cross-border remittance systems: SWIFT and Ripple. Latter has many advantages over the former except for a few minor issues. SWIFT system still leads the remittance market as it has been in use for the last 45+ years. Seeing the dramatic challenges posed by Ripple as it offers 24*7 services and has low transaction costs, Swift has started the Global Payment Innovation Initiative (GPII) to enhance its services. Wanga et al.[153] have systematically studied cryptographic primitives used in the top 30 mainstream blockchains. The authors have categorized the cryptographic primitives used for blockchains into two types: primary primitives such as hashes, digital signature algorithms for tamper-proofing, verifiability, and consensus. The second type of primitives are algorithms used for privacy and anonymity aspects of blockchain. Storublevtcev[154] has considered the top five blockchains, Ethereum, Hyperledger, R3 Corda, Ripple, and Quorum, briefly explained cryptographic algorithms used by these platforms, and presented the strengths of all of these algorithms. 

Moreno-Sanchez et al.[155] studied the architecture and progress of the Ripple network since its release. IOweYou(IOU) credit paths set it apart from other cryptocurrency platforms in settling cross-border transactions. Ripple performs path-based transactions in a few seconds with a minimal fee using credit links. The authors have studied and experimented with the effect of rippling and faulty gateways and how the Ripple network manages these issues. Lokhava et al.[156] have presented the Stellar Consensus Protocol’s (SCP) Model, protocol, and formal verification. Stellar is a blockchain-based payment system mainly designed for international transactions. SCP is a quorum-based Byzantine agreement protocol with open membership. Amoordon et al.[157] present a Byzantine Fault Tolerant (BFT) based blockchain named Tendermint. The authors have explained the characteristics and weaknesses of Tendermint, which has a vision of one blockchain for one application. Any Tendermint node has two processes: Tendermint core and application. Tendermint core handles network and consensus layer working and connects to the application layer using Application Blockchain Interface (ABCI). Tendermint uses a proof of stake consensus algorithm and IBC protocol for interoperability among Tendermint blockchains. Due to its light client nodes requiring less storage, Tendermint can be used for IoT devices. Buterin et al.[158] designed and developed Casper, the Friendly Finality Gadget (FFG), as a smart contract on the Ethereum testnet. It is a Byzantine fault-tolerant protocol’s simplified version implementing voting for checkpoints. It solves the two main problems of PoS protocols, nothing at stake, by inflicting a penalty on the misbehaving validators and long-range attacks by prioritizing finalized checkpoints over PoW. Casper FFG maintains safety in the short term 

MONIKA et al. 

**22 of 50** 

and liveness in the long term, thus striking a balance between a blockchain’s safety and liveness properties. The proposed protocol can be overlaid on any PoW or PoS-based blockchain. 

Hrga et al.[159] presented a technical overview of Initial Coin Offering (ICO). They proposed a crowd sale platform, implemented it on Ethereum, and evaluated it for security features, performance, and cost of running an ICO. They showed that deploying a crowd sales smart contract costs more than any other interaction with the deployed contract functionally. Blockchain’s immutability needs to be kept in mind while developing SC for an ICO, as it cannot be modified later in case of any bug unless designed as mutable. Rehman et al.[160] presented a detailed analysis of trust issues related to cryptocurrencies and provided a taxonomy discussing the main trust aspects. The authors have done a comparative analysis of the top 10 cryptocurrencies. They have mentioned the requirements and potential solutions for trust issues in three categories: immediate solutions, short-term solutions, and long-term solutions. The study concluded that creating a highly trustworthy cryptocurrency ecosystem is a long-term task requiring more time. 

Fan et al.[161] presented a systematic survey on blockchain performance evaluation approaches by categorizing them into empirical and analytical methods. The empirical analysis methods are further classified into performance benchmarking, monitoring, experimental analysis, and simulation. The study also compares three analytical modeling approaches: Markov chains, queueing models, and stochastic Petri nets. The authors conclude that performance monitoring is the best solution for evaluating the performance of public blockchains, along with some open issues and future directions for the performance evaluation methods. The study[162] constructed three-level indicators for evaluating the performance of blockchain platforms in terms of technical performance, market performance, and popularity index. The authors also proposed a modified global Data Envelopment Analysis(DEA)-Malmquist index and evaluated the dynamic performance of 31 public blockchains. Farshidi et al.[163] modeled the blockchain platform selection as a Multicriteria Decision-Making (MCDM) problem. The proposed decision model is embedded in the Decision Support System (DSS) as a knowledge base to ease decision-making. The authors have conducted three case studies with ShareCompany BIQH, DUO, and Veris Foundation to prove the effectiveness and usefulness of the proposed DSS. The authors[164] have discussed the primary characteristics of six blockchains: Bitcoin, ZCash, Monero, Ethereum, Ripple, and IOTA, regarding their data structures. They have discussed the tools for analyzing blockchain data and concluded that present data science tools and algorithms do not apply to blockchain; hence, a new suite of AI tools and algorithms for systematic and efficient data analysis is required. 

## _Summary_ 

Following are the insights and future trends drawn from our analysis related to blockchain platforms: 

1. Earlier blockchain platforms were developed using PoW consensus. However, as PoW requires high computation power, high-energy consumption, and provides slow confirmation and low throughput, mining is not beneficial to its users, and many mining pools are shutting down. Hence, there is a visible transition from PoW to PoS or PBFT consensus mechanism in the latest blockchain platforms. 

2. Go, C++, Rust, and Java are the programming languages predominantly used for platform development. Go, and C++ are the top two languages used by 18 and 13 platforms, respectively. 

3. Solidity is the most used language for smart contract/DApp development. 

4. Ethereum is the most used public platform for creating DApps and Hyperledger is the most used permissioned platform by the industry/organization for implementing organization-specific blockchain solutions. 

5. 80% of platforms considered in this study support smart contract development. Hence, they can be used for DApp development in different domains according to their strengths. 

6. Zilliqa is the first blockchain platform that implements sharding to resolve the scalability issue of blockchain technology. 

7. Some distributed ledgers use the concept of DAG rather than a chain of blocks like IOTA. 

8. Governments and regulators need to make legal frameworks. Stakeholders need to recognize the underlying implications of cryptocurrencies to create a long-lasting crypto operating ecosystem. 

9. Blockchain performance evaluation methods require more analysis and investigation like better data collection methods. Some new evaluation frameworks and workloads are required for comparing scalability solutions at different layers of blockchain. 

MONIKA et al. 

**23 of 50** 

**==> picture [246 x 107] intentionally omitted <==**

**----- Start of picture text -----**<br>
8 8<br>7<br>6<br>3<br>2<br>1 1 1 1 1 1<br>2009 2011 2012 2013 2014 2015 2016 2017 2018 2019 2020 2021<br>Year<br>Number of Platforms<br>**----- End of picture text -----**<br>


**F I G U R E 3** A time-based count of blockchain platforms. 

## 4.2.3 Current development trend of blockchain platforms 

Ethereum’s inception in 2015 is the main force behind the development of numerous platforms. Before Ethereum, most platforms were forked from Bitcoin and focused on solving the issues with the Bitcoin blockchain. Ethereum brought the concept of an account-based transaction model and smart contracts. Hence, subsequent platforms are majorly account-based and support smart contract development. Smart contracts are used to develop DApps over the blockchain platforms. We will explain the current status of DApps in this section. There were around 13 000+ DApps listed on the DappRadar website[42] as of June 2022, out of which 3695 are deployed on the Ethereum blockchain. However, this number was 1749[165] till January 2019. These DApps are developed over various blockchain platforms such as Ethereum, Hive, EOS, ICON, TRON, and Klaytn. Ethereum is the most used platform for DApp development. These DApps are related to gaming, gambling, finance, social, exchanges, media, wallets, marketplaces, governance, security, property, storage, identity, social, insurance, and energy. DApp development has highly influenced the growth of blockchain platforms. To improve the blockchain platforms for DApps and make platforms suitable for different types of domains, new platforms evolved rapidly from 2015 to 2020. Figure 3 shows the time-based count of blockchain platforms considered under this study according to their mainnet launch year. 

We found 8877 cryptocurrencies listed on Coinmarketcap[19] as of June 2022. Cryptocurrency is a digital or virtual asset in the form of a token or coin that is stored on a distributed ledger. Ethereum is the most used platform for token development using its ERC-20 standard. Other platforms used for token development are EOS, TRON, Hive, BNB chain, Polygon, and many more. Out of 8877 listed cryptocurrencies, only 945 are coins native to blockchain platforms. Figure 3 also shows a decline in the release of new blockchain platforms. Hence, the blockchain development trend is shifting from platform development to application development. Industries and organizations are more interested in enhancing already developed blockchain platforms and using them to solve problems related to their domains. 

## **4.3 Research status in terms of security and interoperability** 

Security and Interoperability are crucial aspects of blockchain, deciding its adoption and sustainability. Security and interoperability issues are discussed in Sections 4.1.3 and 4.1.4 respectively. This section discusses the solutions proposed, research challenges, and future trends related to both the aspects. 

## 4.3.1 Different types of interoperability techniques 

This section explains the interoperability solutions proposed in the literature to achieve cross-chain communication. Vo et al.[166] have provided the vision for the inception of the internet of blockchains, where decentralized distributed networks can exchange data, value, and state transition. They have described the challenges faced while implementing the interaction between blockchains. These challenges include discovery, orchestration, scalability, oracle system and data protection, cross-chain transaction processing, data integration, and cross-chain analytics. Hardjono et al.[167] stated that if blockchain has to become a mainstream technology, it should satisfy the same fundamental goals of the Internet. The 

MONIKA et al. 

**24 of 50** 

**TA B L E 6** Categories of interoperability solutions according to survey studies found in the literature. 

||**Paper**<br>Vasilios A. Siris et al.52<br>T. Koens et al.137<br>Ilham A. Qasse et al.168<br>Belchior et al.53<br>Peter Robinson169|**Year**<br>2019<br>2019<br>2019<br>2021<br>2021|**Interoperability**<br>**solution categories**<br>Atomic Cross-Chain Transactions<br>Transactions Across A Network<br>W3C Interledger Protocol (ILP)<br>Bridging Approaches<br>Sidechains<br>Ledger-of-Ledgers Approaches<br>Notary schemes<br>~~Relay schemes~~<br>Hash-locking<br>Sidechains<br>Blockchain Router<br>Smart Contracts<br>Industrial Solution<br>Pubic Connectors<br>~~Blockchain of Blockchains~~<br>Hybrid Connectors<br>Value Swapping<br>Crosschain messaging<br>State Pinning techniques|**Examples of each category**<br>Atomic Swaps using HTLC<br>Lightning, Raiden<br>ILPv1, ILPv4<br>Blocknet, ARK, BTCRelay, POA, Aion, Wanchain<br>Elements, Liquid, Plasma, Rootstock, Cardano<br>Polkadot, Cosmos<br>Polkadot, Cosmos<br>~~BTCRelay~~<br>Atomic Swaps<br>RSK, Elements Alpha, Plasma, POA network,<br>Mimblewimble<br>Architectures using blockchain routers<br>Architectures using smart contracts<br>Cosmos, Polkadot, ICON, Aion, Wanchain, Blocknet,<br>Interledger protocol, ARK, Hyperledger Quilt,<br>Metronome, Block Collider<br>Sidechains, Relays, Notary schemes (Exchanges), HTLC.<br>~~Polkadot, Cosmos~~<br>Trusted relays- Hyperledger Cactus, Blockchain<br>Agnostic Protocols, Blockchain Migrators<br>HTLC, ILPv4, BTC-relay, Xclaim, Pegged sidechains,<br>Plasma, Wanchain<br>Ion, Chain bridge, Celo, Cosmos, Polkadot<br>Merge pinning, Tethered blockchain, Anonymous<br>pinning, Threshold pinning.||
|---|---|---|---|---|---|



authors have stated that a design philosophy similar to the internet is required for blockchain to become the future technology for global commerce. Internet’s architecture and scalable operation from the last three decades have proven that interoperability is crucial to survivability. Lohachab et al.[14] presented a 7-layered reference meta-architecture for achieving interoperability among heterogeneous blockchains irrespective of their implementation and functional differences. The authors have also provided blockchain research taxonomy and compared state-of-the-art interoperability projects. Table 6 shows the interoperability solution types presented by different survey studies. 

As blockchain interoperability is still in its early phase, researchers categorize the cross-chain techniques according to their understanding of the solutions. Polkadot and Cosmos are categorized under three categories: ledger of ledger approaches, notary schemes, and industry solutions, as shown in Table 6. POA is also categorized differently as a bridging solution and a sidechain solution. BTCRelay is also categorized into different categories. Industrial solution is the term mainly used for cross-chain solutions developed by the industry. It is observed that interoperability solution types are still evolving as new techniques are being developed, and it can lead to the merging of two or more solution types to provide a full-fledged interoperability solution. We divide the blockchain interoperability solutions into four types, as shown in Figure 4. 

## _Atomic swaps/Smart contract solutions_ 

Atomic swaps[52] are also known as atomic cross-chain transactions related to trading digital assets between two independent blockchains. These transactions are called atomic because transactions either happen entirely or not at all. Usually, 

MONIKA et al. 

**25 of 50** 

**F I G U R E 4** Blockchain interoperability solution types. 

transaction atomicity is provided by a trusted third party or a multi-sig notary scheme in digital asset exchange. So that no user loses their asset, smart contracts can be used instead of a trusted third party to ensure atomicity in a blockchain ecosystem. Smart contracts based on hash-locks and time-locks known as Hashed Time Lock Contracts (HTLC) are used to implement atomic swaps. A _hash-lock_ is a cryptographically secured lock that can only be unlocked by revealing a secret s whose hash, that is, h = H(s), is configured in a lock. A _time-lock_ is a lock based on time conditions that prevent a transaction from happening until a specific amount of time has elapsed. For using hash lock for atomic swaps between two blockchains, locks on both blockchains must use the same hash function. Cross-chain trading is usually performed using centralized exchanges, but atomic swaps provide a decentralized solution for trading and can significantly help decentralized exchanges. Time lock-in atomic swaps depend on the block time of the blockchains involved, which varies from blockchain to blockchain depending on the consensus algorithm used. In digital trading, exchange rates of assets can change during the execution of atomic swaps, and it provides financial call options that allow users to choose between proceeding with the trade or aborting it. The first atomic swap was performed in September 2017 between Decred and Litecoin.[52] HTCLs are the most flexible and practical solution for implementing interoperability solutions for public blockchains. 

Herlihy[170] has modeled cross-chain swaps as a directed graph, where parties involved in a cross-chain transfer are modeled as vertexes and asset transfers are modeled as arcs. The author has provided an atomic swap protocol using hashed time-lock contracts which works for every pair (D, L) where D = (V, A) is a strongly connected graph and L _⊂_ V is a feedback vertex set for D. The proposed protocol is vulnerable to a weak denial-of-service attack and can be improved further. Li et al.[171] used the concept of distributed private satellite chains consisting of an arbitrary number of stakeholders. A subset of stakeholders act as validators of the satellite chain and form the policies to maintain the ledger. Asset transfer among satellite chains is implemented using smart contracts. Dagher et al.[172] proposed a healthcare record management system using smart contracts where a healthcare provider uses a private blockchain to manage patients’ data, and patients use a public blockchain to access their medical records. The proposed system uses hashing and encryption to secure patient data and enables data sharing between heterogeneous blockchains. Borkowski et al.[173] designed a protocol for cross-blockchain token transfer called DeXTT. They have designed a Pan-Based Token (PBT), which is not locked to a single blockchain and can be traded across blockchains using the DeXTT protocol. This protocol keeps the PBTs synchronized among the wallets across all blockchains and provides eventual consistency of crypto assets. DeXTT can handle the total failure of the participating blockchains as long as one blockchain is operating correctly. The protocol works for similar types of blockchains. The authors[174] have proposed a Hash Time-Locked Contract (HTLC) based asset exchange solution for public blockchains. The study has also formulated time-lock equations for calculating the value of the locking period to be used in atomic swaps between public blockchains. 

Martin Westerkamp[175] has provided a concept for making smart contracts portable between EVM-compatible blockchains. The author has provided a toolbox for implementing all the necessary steps for migrating a smart contract, that is, fetching runtime smart contract code, reconstructing states, generating identical byte code, and deploying the contract on the target blockchain. Anyone on the blockchain can verify the equality of both contracts by comparing their state trie hashes. The author has given two migration methods: a single transaction migration for small smart contracts and three contracts to migrate large contracts. Han et al.[176] have proved that atomic swap is an American Call Option without the premium. Hence, atomic swap is unfair to swap participants. They have quantified the unfairness of the swap protocol and used the Cox-Ross Rubinstein Model, a binomial options pricing model, to estimate the premium for atomic swaps. They have shown that the estimated premium is 2%–3% of asset value for cryptocurrencies, which is more than stocks and fiat currencies, that is, 0.3%. They have improved the traditional atomic swap protocol to make it 

MONIKA et al. 

**26 of 50** 

fair by implementing premium. The improved protocol is implemented in Solidity and supports atomic swaps: currency exchange, and American Call Option styles. 

Yang et al.[177] designed a value exchange mechanism for token transfer between blockchains. They have used the Simple Payment Verification (SPV) proof to verify the availability of funds and (2,2) multi-signature address to send and receive funds. The exchange process is executed off-chain, and the final result transaction is stored on-chain. The mechanism is presented theoretically only. Hiroki and Jay[178] have presented a cross-chain payment protocol for Bitcoin payments on a consortium chain without the intervention of any trusted third party. Niji protocol runs from payment to service provision autonomously. They have implemented the protocol with an EVM consortium blockchain and have proven its feasibility. XCLAIM[179] implements a non-interactive cross-chain exchange of cryptocurrency-backed assets using verifiable smart contracts. It uses proof-of-punishment to enforce users’ adherence to system rules and currently supports exchange between Ethereum and Bitcoin. Lightning Network and Raiden[52] implement micro-payment channels for off-chain data exchange on Bitcoin and Ethereum, respectively. Both networks implement payments using HTLC contracts. 

_Limitations._ Atomic swaps can only be used to implement asset exchange, not asset transfer. We need transaction atomicity and transaction confirmation when implementing an interoperability solution for blockchains. Transactions are considered confirmed once they become part of a verified block, and it depends on the transaction finality time of a particular blockchain. Atomic swaps only provide transaction atomicity and need another cross-chain mechanism to support transaction confirmation. A malicious user can deliberately delay the swap by not depositing the assets on time to leverage the fluctuating cryptocurrency exchange rate. Another way of making unfair trade can be by not releasing secrets timely and waiting for favorable conditions. 

## _Sidechains_ 

Sidechains[52,168,180] are blockchains connected with the main chain with a two-way peg. A two-way peg allows the transfer of assets between the mainchain and the sidechain. The sidechain’s protocol and consensus mechanism can be different from the main blockchain. Such flexibility allows users to avail themselves of additional features and functionalities of sidechains using their assets of the main chain. Transaction time is usually less on the sidechain due to less traffic; hence using a sidechain improves the delay for mainchain transactions. Transaction costs also could be significantly lower on the sidechain than the main chain. Improved transaction time and less transaction cost can improve the mainchain’s scalability. Although pegged to the main chain, sidechains are firewalled so that a bug in the sidechain does not affect the main chain. A two-way peg mechanism can be designed using three approaches: 

_Centralized two-way peg._ A trusted third party handles asset transfer in a centralized approach.[142] The third party controls the locking and unlocking of assets on both chains. A centralized two-way peg is easy to implement because a single party oversees the transaction between chains. Transaction time is also less in this approach due to centralized verification by simple proofs. However, this approach suffers from a single point of failure and leads toward centralization. 

_Federated two-way peg._ In a Federated approach,[180] a group of notaries or entities controls the locking and unlocking of assets rather than a single centralized notary. The transfer of assets occurs when most notaries agree and sign the transaction. A Federated two-way peg improves decentralization over a centralized approach and does not suffer from a single point of failure. If an attacker gains access to the private keys of most notaries, then funds can be stolen. 

_Two-way peg based on simplified payment verification (SPV) proofs._ This approach[180] uses SPV proofs that a transaction has been added in a valid block and miners have mined new blockchain over that block. Hence, it is confirmed. SPV used lightweight clients that download only blockchain headers, not the entire blockchain, to verify that a transaction belongs to a legitimate block. This two-way peg approach completely removes the need for a third party and is fully decentralized. However, SPV-based two-way pegs are slow because users need to wait for transaction confirmation and reorganization time. Liquid and Rootstock (RSK) are two-way pegged sidechains connected to Bitcoin. LOOM and POA are two-way pegged to Ethereum. RSL, LOOM, and POA can execute Turing complete smart contracts. RSK can perform 300–1000 txn/s, and POA can perform 60 txn/s.[180] Element is a blockchain platform that can run as a standalone ledger or a sidechain. It is built upon the codebase of Bitcoin. Liquid and Elements both support confidential transactions. Plasma[52,168,169] is a hierarchical tree of sidechains designed for the Ethereum blockchain. 

MONIKA et al. 

**27 of 50** 

_Advantages of sidechains._ Sidechains improve the scalability of the main chain due to faster transaction time and lesser transaction traffic on their chain. Sidechains provide additional functionalities for primary chain users, like game development on the sidechain, interoperability between blockchains, ability to execute smart contracts in the case of Bitcoin. 

_Disadvantages of sidechains._ The four mentioned sidechains follow a federated two-way mechanism. Hence introduces centralization in the blockchain environment. The liquid and RSK sidechains are not open-source and have a high computation code for running a liquid node. Loom has limited support for windows. Sidechains’ security depends on the main chain; a compromised main chain can exploit the sidechain’s functionality. 

## _Bridging solutions (Relay schemes)_ 

Bridging solutions[52] provide one-way or two-way communication between two parties without involving any third party or extra ledger. Communication can be related to the transfer and exchange of assets between two blockchains of similar type. Bridging solutions are implemented using smart contracts and other modules running on both interconnected chains to monitor and relay the transaction. Hence, they can also be called relay schemes. Bridging solutions are different from atomic swaps as they provide additional functionalities apart from only the asset exchange. 

Blocknet[52,168] is a protocol for blockchain interoperability for communication and exchange between different blockchains and was launched in 2014. It is a POS-based service providing blockchain and comprises two main components, XBridge and XRouter. XBridge provides a decentralized exchange layer to Blocknet for connecting independent blockchains using atomic swaps. XRouter is a decentralized communication layer that provides registry services and service lookup to relay cross-chain messages to the intended blockchain. Wanchain[52,168] is an Ethereum based-project for implementing cross-chain transactions using smart contracts. Wanchain employs specialized nodes that use multi-party computation processes to carry out cross-chain transactions. Assets are locked on the original blockchain, and their corresponding representations are created on the destination blockchain using smart contracts for different operations. Locked assets can only be released after their value is sent back to the original blockchain and their representations are destroyed on the destination chain. The specialized nodes manage the locking and unlocking of assets. Metronome[14,168,181] is a project to create a cryptocurrency that can hop from one blockchain to another. Currently, its token ‘MET’ can hope between Ethereum and Ethereum classic blockchain. The token can hop between source and recipient blockchains using a proof-of-exit receipt. The three primary components of Metronome are import, export, and validate for carrying out the transfer of tokens. 

Abebe et al.[182] have provided a reference architecture for cross-chain data transfer between enterprise blockchains. The authors have also proposed a protocol for trusted cross-network data transfer using local relays and system contracts. They have developed a proof of concept of the proposed protocol and have evaluated it based on the following metrics: security, ease of use and adaption, generalization, and extendibility. Some of the other bridging solutions are ARK,[52,181] Aion,[52] BTC Relay,[52,169] and Ion.[14,169] 

## _Notary schemes (Ledger of ledger)_ 

The interoperability solution that employs an intermediary between two blockchain platforms is known as a notary scheme. An intermediary can be a single party or a group of parties handling events between two blockchains. Notary schemes can be centralized or decentralized depending upon the nature of the intermediary. Suppose the intermediary is a trusted third party controlled by an organization. In that case, it is a centralized notary scheme, and if all the users of the system control the intermediary, then it is a decentralized notary scheme. If a notary scheme connects different types of ledgers, it can also be known as a ledger of ledger approach. Centralized and decentralized exchanges facilitating the trading of cryptocurrencies are notary schemes. Two prominent ledgers of ledger frameworks are Cosmos and Polkadot. 

Cosmos[52,137] network comprises many different independent blockchains. It consists of cosmos hubs and zones. Both cosmos hubs and zones implement BFT-based consensus Tendermint. Different zones interact which each other using Inter-Blockchain Communication (IBC) protocol with the cosmos hub at the center, recording all the transactions. Zones submit only inter-blockchain transactions to hubs and have their own policies and governance mechanisms. So, every transaction between two zones is recorded at the zones and cosmos hub. Special zones, called pegged zones, are used to connect the Tendermint blockchain with any other probabilistic blockchain like a PoW-based blockchain. Cosmos is built to be used for different types of use cases and has become an internet of blockchains. 

MONIKA et al. 

**28 of 50** 

Polkadot[52,137] is also a ledger of ledger technology like Cosmos. It uses the concept of relay chains and parachains. A _relay chain_ is a permissionless ledger that registers and finalizes transactions between parachains. Parachains are independent permissioned or permission-less blockchains that want to interact with each other. Polkadot also constitutes bridges to connect parachains to external networks like Bitcoin and Ethereum. The Polkadot network incorporates four consensus roles: Nominators, Validators, Collators, and Fishers. Validators stake the DOT token and manage the consensus for the relay chain and nominators, helping secure the chain by voting for trustworthy validators. Collators help the validators collect transactions and produce blocks; fishermen are the bounty hunters who report destructive behaviors to validators. 

In cosmos and Polkadot, the state is stored at four places: independent blockchains, relay chain/cosmos hub, and peg zone/ bridge that leads to data redundancy. Cosmos and Polkadot reach consensus differently, treat scalability differently, and do not interoperate. Hence, applications developed using Cosmos cannot interact with an application developed using Polkadot. 

ICON[53,181] is another project for connecting multiple blockchains launched by South Korea. Its native token, ICX, is used to carry out smart contract functions within the ICON ecosystem. The ICON network uses an improved version of BFT as its consensus algorithm. 

## _Miscellaneous interoperability studies_ 

Interledger Protocol(ILPv1)[52,169] is a payment network to allows atomic transfer with the help of a network of connectors between different types of ledgers. Connectors act, as routers that apply currency exchanges, and users have to trust connectors to settle payments among themselves. ILPv4 resolves the long timeout shortcoming of ILPv1 by using low-value packets to carry transaction information. Wang et al.[183] have given the concept of a blockchain router derived from the architecture of the internet. The proposed architecture is a complex blockchain star network, where the connected blockchains are called sub-chains and can send and receive messages among each other using a chain router. There are four types of nodes: Validators, Nominators, Surveillants, and Connectors. Connectors connect the sub-chains to the blockchain router. Validators and Nominators verify the blocks generated for cross-chain communication requested by sub-chains. Surveillants check the behavior of the blockchain router and prevent any accidental, wrongful behavior. The consensus algorithm applied is similar to PBFT. Jin et al.[184] have divided the interoperability schemes into two categories: passive interoperation and active interoperation and have also discussed the main challenges in achieving interoperability in the blockchain. They have proposed a novel architecture for interoperability by defining modules like data generator at the data layer, cross-chain communication at the network layer, verification module at the consensus layer, cross-chain contract at the contract layer, and cross-chain API at the application layer. They have also proposed Monitor Multiplexing Reading (MMR) solution for passive cross-chain communication. Experimental results have shown that MMR reduces CPU utility and I/O overhead by 13.7% and 13.8% compared to Polling- Based Reading (PBR). 

Liu et al.[185] have proposed and implemented HyperService, a platform for interoperability and programmability for heterogeneous blockchains. They have proposed a blockchain-neutral state transition model, the Universal State Model (USM), to state transitions across heterogeneous blockchains. The platform’s main two modules are HSL, a high-level programing language for writing smart contracts for cross-chain DApps, and UPI (Universal inter-blockchain protocol). UPI is a generic, secure, and financially secure protocol for handling DApp executions. Their implemented prototype for end-to-end DApp executions shows minimal latency in the order of seconds. Quant overledger[14,53] is a blockchain-agnostic gateway API that can be used to create decentralized applications to run on any blockchain regardless of its ledger implementation and consensus mechanism. The authors[186] have studied the notion of trust in blockchain oracle solutions. The authors have compared the oracles solutions: Provable, Towncrier, Pricegeth, Witnet, Augur, Chainlink, ASTRAEA, and Aeternity based on trust features like security, authenticity, confidentiality, and accessibility. They have concluded that further research is still required in terms of new standards for data formats and novel privacy and security mechanisms to accomplish a fully reliable oracle platform. 

## 4.3.2 Techniques proposed for blockchain security issues 

## _Infrastructure security solutions_ 

In this study, articles related to analyzing individual attack types are not considered to limit the survey; however, review studies related to infrastructure security are analyzed thoroughly to determine the available solutions. The authors[78] have discussed the defense mechanisms proposed by researchers against the attacks on the network, consensus, replicated 

MONIKA et al. 

**29 of 50** 

state machine, and application layer of blockchain networks. The selection of a consensus algorithm is the most crucial factor from the security point of view while developing a blockchain or choosing a blockchain for DApp development. Hence, this section includes studies related to consensus algorithm advancements and comparison. Table 7 presents the solutions proposed for the infrastructural security of blockchains. 

## _Smart contract solutions_ 

Smart contract security consists of two main steps: security vulnerability analysis and formal verification of smart contracts. Security vulnerability analysis techniques for smart contracts are discussed first, followed by formal verification techniques. 

_Security Vulnerability Analysis._ The study[191] discusses twenty-seven vulnerability detection tools and techniques. The study[192] **c** onsidered 53 papers and categorized them into studies on security assurance and correctness verification of smart contracts. The authors[193] have discussed how Blockchain-Oriented Software Engineering (BOSE) can prevent smart contract attacks and have emphasized three main improvement areas: best practice and development methodology, design patterns, and testing of smart contracts. Mense et al.[105] have briefly explained the vulnerabilities that can affect the security of smart contracts. They have also provided a comparative analysis of seven vulnerability detection tools: Oyente, Securify, Remix, SmartCheck, F*, Mythril, and Gaspe. The study[194] has systematically analyzed 90 smart contract papers and has categorized them into three categories: Smart contract-related security methods, performance improvements, and decentralized applications. Parizi et al.[195] have analyzed the effectiveness and accuracy of four smart contract security analysis tools: Oyente, Mythril, Securify, and SmartCheck. They have concluded that SmartCheck is statistically more effective than the other automated security testing and Mythril is significantly more accurate. 

The authors[90] have examined 10 security analysis tools (Slither, Mythx, Mythri, Manticore, Securify, Smartcheck, Echidna, Oyente, Vandal, and Zeus) and pointed out their effectiveness in detecting smart contract vulnerabilities. Huang et al.[100] have divided the SC life cycle into four phases similar to the software lifecycle: security design, security implementation, testing before deployment, and monitoring and analysis. They have also discussed the security analysis and implementation techniques used in each phase. The study[196] has presented six security patterns for SCs: Check-Effects-Interaction, emergency stop, speed bump, rate limit, mutes, and balance limit to help developers provide safety measures against some severe security issues or attacks. Gupta et al.[120] reviewed security schemes for SC using traditional mechanisms (TEE, SMPC, ZKP), and privacy-preserving artificial intelligence techniques for blockchain. 

The study[114] analyzed the challenges regarding blockchain software development. It divided them into six categories: SC testing, SC code analysis, SC code metrics, SC security, DApp performance measurement, and BC application. The authors have discussed the proposed solutions for each category and the challenges that need further attention. The study[197] has categorized smart contract security issues into eight types and has discussed 18 solution types for these issues. The authors have concluded that more automated tools and effective design approaches, such as homomorphic encryption, are required to make SCs more secure. The study[198] has discussed the security issues of smart contracts and provided a mapping between SC vulnerabilities and SC security solutions. The authors[199] have studied 45 existing Smart Contract Languages (SCLs) and categorized them into five types: Domain-specific SCL, formally verifiable SCL, easy-to-use SCL, legally enforceable SCL, and business process SCL. The study has also identified 10 critical suitability and expressiveness properties that SCLs should possess to make SCs legally enforceable. Dixit et al.[200] have identified 10 models/frameworks and categorized them according to their automation approaches. The authors also presented 11 legal parameters for automated SC development and categorized them into four criteria classes. They have also compared the model w.r.t to their legal relevance. The studies[201,202] have briefly discussed many aspects of SCs, such as their applications, technical challenges, standardization, legal challenges, and business models. Table 8 presents the solutions, tools, and languages developed for SC security arranged year-wise. 

Table 9 represents the findings of comparative studies related to smart contract security analysis tools. The authors[61] have concluded that most of the tools are insufficient as they require desired security properties to be described in a specific language for vulnerability detection. They have also stated that there is a trade-off between full coverage and high accuracy and users of tools need to keep that in mind. Zhou et al.[112] have stated that most of the research works compute the accuracy and efficiency of tools as per self-defined metrics. The authors have also concluded that the creation of benchmarks and unified metrics for the comparison of security tools is challenging because of inconsistent definitions of vulnerabilities. Zhang et al.[244] proposed a new framework by extending IEEE Standard Classification for Software Anomalies and divided 49 kinds of bugs into 9 categories with subcategories. The authors also constructed a dataset of problematic SCs called Jiuzhou containing 176 SCs including contracts that contain bugs, contracts with bugs fixed and 

MONIKA et al. 

**30 of 50** 

**TA B L E 7** Blockchain infrastructure security solutions proposed in the literature. 

|||**IF security issue**<br>**category**<br>**Consensus related**<br>**Cryptography**<br>**related**<br>**Service attacks**<br>**Malicious**<br>**miners/nodes**||**Solution**<br>**citation**<br>[85]<br>[79]<br>[81]<br>[27]<br>[187]<br>[37]<br>[93]<br>[188]<br>[189]<br>[93]<br>[190]<br>[23]<br>[15]||**Description**<br>_POS attacks related solutions_: Longest chain rule, moving checkpoints, key-evolving<br>cryptography, plenitude rule, context-aware transactions, economic finality, Trusted<br>Execution Environment (TEE).<br>_Solutions for 51% hash attack_: insert observers, use alerts among peers, increase<br>computational power, and disincentivize large mining pools. For privacy: mixing<br>and anonymous solutions._Solutions for the forking problem_: backward compatibility<br>change, decreased propagation delay, fork resolving policy.<br>SMARTPOOL: A decentralized mining pool implemented through a smart<br>contract, more efficient than centralized mining pools, provides expected rewards<br>to honest miners, avoids the centralization of mining power, robust against the<br>attackers resubmitting their shares in different batches.<br>Detailed study of unique characteristics of consensus algorithms, comparison of<br>different PoX schemes for permissionless blockchains, and how the selection of<br>consensus mechanism impacts an application.<br>Discussed the basics of a consensus algorithm, comparative analysis of Nakamoto<br>consensus, Ripple, PBFT, PoET, chain-based PoS, and BFT style PoS on the basics of<br>security and complexity.<br>Taxonomy of consensus classification, comparison of PoW, PoS, DPoS, PBFT,<br>Ripple, PoA, Casper, Snow White, Algorand, Ouroboros Genesis.<br>Briefly discussed blockchain-based cyber threat intelligence (CTI) framework and<br>Trustchain with a novel algorithm to protect from_Sybil attack_.<br>Cryptographic concepts proposed and used in blockchains over the years for<br>security and privacy._Signature schemes_: multi-signature, blind signature, ring<br>signature, threshold signature,_Zero-Knowledge Proofs (ZKP_): zk-SNARK,_Access_<br>_control_: role-based access control,attribute-based access control,Encryption<br>scheme, Secure Multi-Party Computation (SMPC), Secret sharing, Commitment<br>scheme, Accumulator, Oblivious Transfer, Oblivious Ram, Proof of Retrievability,<br>Post-Quantum cryptography, Lightweight cryptography, Verifiable random<br>function, Obfuscation.<br>Game theory approaches like non-cooperative game, splitting game, mean<br>payoff-game, stochastic game, sequential game, stackelberg game, repeated game,<br>extensive-form game, coordination game for_preventing selfish mining attacks, DoS_<br>_attacks, and majority attacks_.<br>Abrupt change on the RTT and updating UDP messages regularly to avoid_routing_<br>_attacks_. Discussed solutions for_DDoS attacks and Ecplise attacks_.<br>_Eclipse attack_: random section of IP addresses when making an outbound<br>connection, Feeler and anchor connections, increasing the size of the tables,<br>banningunsolicited addresses,allowingmore outgoingconnections._Routing_<br>_attack_: diversifying and increasing autonomous systems connections, monitoring<br>round-trip time, using different channels and ports, encrypting messages, and<br>requesting data from multiple peers.<br>A thorough study regarding security and privacy solutions of Bitcoin blockchain<br>security. Game-theoretic approaches can be used to reach Nash equilibrium<br>between honest and malicious miners. Hence, can avoid_block withholding, selfish_<br>_mining_, and discarding attacks and can solve the incentive distribution problem.<br>_Defense Mechanisms against Block withholding_: Income distribution model, revenue<br>adjustment distribution model, mining agreement adjustment, zero block addition<br>model, Evolutionary model, Miner credit mechanism, Credit value model.|
|---|---|---|---|---|---|---|



MONIKA et al. 

**31 of 50** 

**TA B L E 8** Security analysis tools, techniques, and domain-specific languages for smart contracts. 

||**S.N.**<br>**Ref.**<br>**Year**<br>**Description**<br>**Tool/Technique/**<br>**Language**<br>1<br>[126]<br>2016<br>CFG builder, symbolic execution. Z3 bit-vector solver. Static tool.<br>Oyente<br>2<br>[115,116]<br>2017, 2018<br>Gasper detects 3 gas-costly patterns using symbolic execution,<br>~~Gasreducer detects 24 anti-patterns for under-optimized SCs,~~<br>bytecode-to-bytecode optimization.<br>Gasper, GasReducer<br>3<br>[128]<br>2018<br>CFG, parser coverts SC to C++code, fuzzy engine, run-time trace,<br>and code detector. Dynamic tool.<br>ReGuard<br>4<br>[124]<br>2018<br>Generates fuzzinginputs accordingto SC ABI,test oracles,detects<br>and types of vulnerabilities. Dynamic fuzzing.<br>ContractFuzzer<br>5<br>[118,119]<br>2018<br>Detects Ponzi schemes using XGBoost. Random forest, 131 Ponzi<br>scheme SCs and 1251 non-Ponzi scheme SCs tested. Static.<br>–<br>6<br>[203]<br>2018<br>Uses the concept of Effective Callback Freedom to detect<br>reentrancy attacks for solidity code, open-sourced.<br>ECFchecker<br>7<br>[204]<br>2018<br>Uses taint analysis and an attack detector to detect reentrancy<br>attacks for EVM bytecode at runtime.<br>Sereum<br>8<br>[205]<br>2018<br>Gas-focused vulnerabilities. Vandal decompiler. Dataloglanguage<br>to analyze CFGs. Static tool.<br>MadMax<br>9<br>[122]<br>2018<br>Detects SC vulnerabilities, ANTLR parser, XML parser tree, and<br>Xpath queries. Static tool.<br>SmartCheck<br>10<br>[206]<br>2018<br>Inspection tool,mirror-based reflection,and decompilation<br>techniques, Abstract Syntax Tree(AST).<br>SmartInspect<br>11<br>[207]<br>2018<br>Object-Oriented metrics for Solidity SCs, modified ANTLR<br>grammar, AST. Static tool.<br>SolMet<br>12<br>[208]<br>2018<br>S-gram semantic aware technique: N-gram language modeling and<br>~~lightweight statistical labeling, used Ether* for improving the~~<br>efficiency of ReGuard.<br>Ether*<br>13<br>[209]<br>2018<br>Uses symbolic execution to detect the presence of trace<br>vulnerability, works on EVM bytecode, and is open source.<br>Maian<br>14<br>[210]<br>2018<br>Symbolic execution and taint analysis to detect integer bugs,works<br>for both Solidity and EVM bytecode, and is open source.<br>Osiris<br>15<br>[211]<br>2018<br>Extension of Oyente, produces a rule-based representation (RBR)<br>of EVM bytecode from CFGs, open source.<br>EthIR<br>16<br>[212]<br>2018<br>Combines abstract interpretation with symbolic execution to<br>~~traverse all possible paths of an EVM bytecode to check whether~~<br>SC meets predefined properties or not, and is open source.<br>Securify<br>17<br>[213]<br>2018<br>Forms a topological graph for developers, and uses symbolic<br>execution and syntax analysis to detect logical vulnerabilities for<br>Solidity codes.<br>SASC<br>18<br>[125]<br>2019<br>Multi-objective oriented path search (MOPS),<br>~~critical-path-coverage using dynamic symbolic execution and taint~~<br>analysis.<br>**–**<br>19<br>[214]<br>2019<br>Deduces all gas upper bounds for public functions of SCs. Uses<br>Symbolic execution, and works for both EVM bytecode and Solidity.<br>Gastap<br>20<br>[121]<br>2019<br>Detects non-determinism of Ethereum. Systematically<br>instrumented and augmented LLVM IR, taint analysis. Static tool.<br>NPChecker<br>21<br>[215]<br>2019<br>EVM level, detect overflow vulnerabilities, transactions’ taint<br>analysis-based tracking. Dynamic tool.<br>EasyFlow<br>~~(Continues)~~|
|---|---|



MONIKA et al. 

**32 of 50** 

**TA B L E 8** Continued 

||**S.N.**<br>**Ref.**<br>**Year**<br>**Description**<br>**Tool/Technique/**<br>**Language**<br>22<br>[216]<br>2019<br>Event-ordering (EO) bugs, partial-order reduction techniques, symbolic<br>execution, happens-before relation, and fuzzing of events. Dynamic tool.<br>EthRacer<br>23<br>[132]<br>2019<br>ML model, Mythril, and Slither for static analysis and labeling, Support<br>~~Vector Machine, neural network, Random forest, and decision tree to~~<br>train the model.<br>–<br>24<br>[123]<br>2019<br>ANTLR Parser, XML parse tree, dom4j detector.<br>NeuCheck<br>25<br>[131]<br>2019<br>Vulnerabilitydetection,Assertions for correctnesspropertychecks,<br>InputGenerator, fault seeding tool, Dynamic.<br>SolAnalyser<br>26<br>[217]<br>2019<br>Mutation testing, AST, supports under-defined testnet.<br>MuSC<br>27<br>[218]<br>2019<br>Clone detection and bugdetection,ANTLR,normalization,code<br>embedding using Fasttext into numerical vectors.<br>SmartEmbed<br>28<br>[219]<br>2019<br>A toolchain decompiles the SCs from EVM to high-level 3-address code,<br>a full-fledged framework with program analysis libraries.<br>Gigahorse<br>29<br>[220]<br>2019<br>A smart contract solution based on the light-weightedquantum blind<br>signature, quantum entanglement, secure against quantum attacks.<br>**–**<br>30<br>[221]<br>2019<br>Path-based test coverage for an SC model, whole transaction basis path<br>set and bounded transaction interactions, k-bounded transaction<br>coverage criteria for SC.<br>**–**<br>31<br>[222]<br>2019<br>Systematic automated testingtechnique for SCs,considers the<br>interaction between front-end and SCs, read-write graph, taint Analysis.<br>Sungari<br>32<br>[223]<br>2019<br>Test generation for Ethereum SC, time and cost-effective test-suite<br>generation, random-based and NSGA-II-based multi-objective<br>approaches.<br>**–**<br>33<br>[224]<br>2019<br>Agraph-based framework for calculatingsoftware metrics for SCs,<br>object-oriented design metrics, ANTLR, AST, and Meta-model instances.<br>**–**<br>34<br>[225]<br>2019<br>Uses symbolic execution and satisfiability modulo theories for Solidity<br>analysis, provides several API for user-defined analysis, open source.<br>Manticore<br>35<br>[226]<br>2019<br>Provides partial analysis on EVM bytecode to generate control-flow<br>~~graphs for critical sections of SCs like fund transfers using symbolic~~<br>execution.<br>sCompile<br>36<br>[227]<br>2019<br>A new approach for learning a fuzzer from imitating a symbolic<br>execution expert, works on solidity code and open-source.<br>Learning-based<br>Fuzzer(ILF)<br>37<br>[228]<br>2019<br>A vulnerability detection framework for Solidity, uses data flow analysis<br>~~and taint analysis, IR SlithIR, Static Single Assignment (SSA) form,~~<br>code optimization, and Code review.<br>Slither<br>38<br>[127]<br>2020<br>Detects six types of vulnerabilities XGBoost as the multi-label Classifier,<br>SMOTETomke sampling method, predictive Micro-F1, and Macro-F1<br>over 96%. Static.<br>**–**<br>39<br>[229]<br>2020<br>Further extends Gastap for providing gas consumption optimization<br>~~suggestions. uses symbolic execution and works for both EVM and~~<br>solidity.<br>Gasol<br>40<br>[230]<br>2020<br>Gas optimization of SCs using Max-SMT, works on EVM bytecode,<br>open source.<br>Syrup<br>41<br>[231]<br>2020<br>Detects all arithmetic bugs with negligible false positives, uses<br>~~counter-example-guided inductive synthesis(CEGIS) style verification,~~<br>works on Solidity, open source.<br>VERISMART<br>~~(Continues)~~|
|---|---|



MONIKA et al. 

**33 of 50** 

**TA B L E 8** Continued 

||**S.N.**<br>**Ref.**<br>**Year**<br>**Description**<br>**Tool/Technique/**<br>**Language**<br>42<br>[232]<br>2020<br>Uses symbolic execution and delayed predicate abstraction to verify<br>whether a Solidity code meets the provided security requirements.<br>VerX<br>43<br>[233]<br>2020<br>Static analysis approach to abstract EVM bytecode semantics in the<br>form of horn clauses to express reachability properties.<br>eThor<br>44<br>[234]<br>2020<br>Analysis of user-defined properties and assertions by using fuzzing,<br>works on Solidity and Vyper SCs, open-source.<br>Echidna<br>~~**Languages**~~<br>45<br>[129]<br>2017<br>A safeprogramminglanguage to handle re-entrancyattacks and<br>money leakage problems.<br>Obsidian<br>~~46~~<br>~~[235]~~<br>~~2017~~<br>~~A domain-specific high-level language for financial purposes~~<br>~~Findel~~<br>47<br>[236]<br>2017<br>A security proof and open-source language for bitcoin,intermediary<br>representations, verifiable with Coq.<br>Simplicity<br>48<br>237<br>2018<br>Type-safe programming language for SCs, uses caller’s capabilities to<br>~~save SC functions from unauthorized users and asset type to ensure~~<br>atomicity.<br>Flint<br>~~49~~<br>~~[238]~~<br>~~2018~~<br>~~A specification language for SCs for collaborative design purposes.~~<br>~~SPESC~~<br>50<br>[239,240]<br>2018/2019<br>BitML, a domain-specific language for Bitcoin SC; compiler to<br>translate BitML contracts to Bitcoin contracts,computational attacks<br>can be analyzed through the Symbolic model. BitML-based toolchain<br>to verify Bitcoin SCs.<br>BitML based<br>toolchain<br>51<br>[241]<br>2018<br>A high-level alternative language for Solidity with a stronger type<br>~~system, designed to facilitate symbolic execution and formal proof in~~<br>Coq<br>Lolisa<br>52<br>[242]<br>2019<br>Type safe intermediate-level programming language for SCs,<br>~~foundational calculus-based, a framework for lightweight verification~~<br>of smart contracts.<br>Scilla<br>53<br>[243]<br>2020<br>A calculus for Solidity,supports aprecise definition of the behavior of<br>smart contracts<br>Featherweight<br>Solidity|
|---|---|



crafted contracts. They have also concluded that there are still 10 kinds of bugs that cannot be detected by any of these tools. Ren et al.[245] created a publicly available unified benchmark suite for SC security testing, which integrates artificially constructed contracts, confirmed vulnerable contracts, and unlabeled real-world contracts. They have emphasized that the evaluation process should take the following factors into consideration: (a) a set of diverse test suites; (b) a unified execution environment with suitable runtime parameters; (c) more quantitative and multi-dimensional performance metrics. Durieux et al.[246] created a novel, extendable execution framework called SmartBugs to test the SC security tools on the same execution environment. It contains two datasets, one consisting of annotated vulnerable Solidity SCs and other containing all the available SCs from the Ethereum blockchain that have Solidity source code available in Etherscan. 

From the experimental findings in Table 9, several observations were discerned: Slither and Mythril exhibit superior coverage, Mythril demonstrates the highest precision, Remix provides the top recall rate, and SmartCheck emerges as the most effective tool. Additionally, Slither, Solhint, SmartCheck, Neucheck, and SolAnalyzer are identified as the most efficient tools in terms of speed. 

Upon a comprehensive analysis of the research papers presented in Table 9, it becomes evident that there is no unified experimental setting for comparing security analysis tools for smart contracts (SC). Notably, none of the tools discussed in the literature addresses all known vulnerabilities of SCs. The absence of a universally accepted benchmark for evaluating SC security analysis tools is highlighted, emphasizing the necessity for such a benchmark. It is acknowledged that isolated evaluations with varying experimental settings, such as different maximum depth searches, may yield misleading or deceptive results. 

**34 of 50** 

|**34 of 50**|**34 of 50**||||||||||||||||||||||||||||||||||||||||MONIKA|MONIKA|MONIKA|et|al.|al.|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
||||||||||||||||||||||||||||||||||||||||||||||||
||||||||||||||||||||||||||||||||||||||||||||||||
|**9**<br>Comparative analysis of smart contract security tools.|**Tools Considered**<br>**Findings**<br>**Performance Parameters considered**||Oyente, Securify, Osiris, Sereum. SmartCheck,<br>**•** SmartCheck, DefectChecker, contractWard, and sFuzz are<br>**•** Accuracy, efficiency, vulnerabilities covered.|sFuzz, DefectChecker, ContractWard,<br>better in overall coverage.|MadMax, NPChecker, ContractFuzzer<br>**•** NPChecker, MadMax, Osiris, and Sereum are for specific|vulnerability detection.|**•** ContractWard is relatively fast and more accurate although|can only detect pre-defined vulnerabilities.|**•** The NPChecker is slower, however, can find new vulnerabil-||ity patterns.|Oyente, Securify, Remix and SmartCheck<br>**•** SmartCheck outperformed the other tools in terms of effec-<br>**•** vulnerabilities covered, accuracy, effectiveness. 23|tiveness.<br>vulnerable and 21 audited smart contracts were|**•** Oyente performed the best in terms of accuracy.<br>analyzed.||Smartcheck, Securify, Oyente, Remix<br>**•** SmartCheck showed better FDR*and FNR*.<br>**•** Accuracy in terms of TP, FP, FN, FDR, FNR.||NeuCheck, Securify, Mythril<br>**•** Neucheck is better than Securify and Mythril in terms of<br>**•** Accuracy in terms of TP, FP, FN, FDR, FNR. Time|analysis speed and easy deployment.<br>overhead is also calculated.|**•** Securify detected the largest number of vulnerabilities.|SolAnalyser, Oyente, Securify, Maian,<br>**•** SolAnalyser is better than the other five tools in terms of<br>**•** Vulnerabilities covered, precision and recall calcu-|SmartCheck and Mythril<br>analysis time per contract and recall.<br>lated by testing the tools with 12 866 contracts.||**•** The precision of Securify is better than SolAnayser for arith-||metic vulnerabilities.|EtherTrust, EthIR, KEVM, Securify, Zeus,<br>**•** Five tools found particularly inspiring are FSolidM, KEVM,<br>**•** Theoretical analysis from other surveys and Vul-|MAIAN, sCompile Manticore, Mythril,<br>Securify, Maian, and Mythril.<br>nerabilities covered compared.|teEther, E-EVM, Erays, Rattle, Osiris, SolMet,<br>**•** Securify is the most advanced regarding formal guarantees.|Ether*, SASC, Oyente, ContractLarva,<br>FSolidM, Porosity, Remix-I, Smartcheck,<br>**•** KEVM is mature though requires expertise to use.||Solgraph, Vandal, Gasper, Reguard<br>**•** Maian discards false positives better than others.|SmartCheck, Oyente, Securify, Mythril.<br>**•** Found SmartCheck to be the most effective static security<br>**•** Accuracy and effectiveness. Receiver Operating|testing tool for Solidity smart contracts on the Ethereum<br>Characteristic (ROC5) analysis was used to show||blockchain but perhaps less accurate than Mythril.<br>the recall versus precision relationship changes.|Osiris, Zeus<br>**•** Osiris is better than Zeus for detecting Integer bugs.<br>**•** Correctness and effectiveness via an empirical|analysis on 883 contracts.|Securify, Oyente, Mythril<br>**•** Securify performs better in detecting transaction order<br>**•** Compliance and violation patterns in terms of|dependence, re-entrancy, handled exceptions, and restricted<br>true/flase warnings, violations, unreported vulner-|transfers.<br>abilities (failing to report certain vulnerabilities).|**•** A dataset of 100 SCs contracts for which all the|warnings were manually classified into true and|false warnings was considered.||(Continues)|
|**TA B L E **|**Study**||[112]|||||||||[105]||||[122]||[123]|||[131]||||||[191]||||||[195]||||[210]||[212]||||||||
||||||||||||||||||||||||||||||||||||||||||||||||
||||||||||||||||||||||||||||||||||||||||||||||||



MONIKA et al. 

|MO<br>**TA B L E 9**<br>Continued.|NIKA<br>**Study**<br>**Tools Considered**<br>**Findings**<br>**Performance Parameters considered**||et al.<br>[216]<br>EthRacer, Oyente<br>**•** EthRacer performs better in finding event-ordering<br>bugs than Oyente.<br>**•** Empirically analyzed 10 423 SCs for EO bugs. Man-<br>ual post-hoc analysis was also performed for 100|et al.<br>[216]<br>EthRacer, Oyente<br>**•** EthRacer performs better in finding event-ordering<br>bugs than Oyente.<br>**•** Empirically analyzed 10 423 SCs for EO bugs. Man-<br>ual post-hoc analysis was also performed for 100|randomly selected SCs.|[219]<br>Gigahorse, Porosity, Vandal<br>**•** Gigahorse is a better Ethereum decompiler than Van-<br>**•** Empirical analysis on 6.6 million contract to check|dal and Porosity.<br>scalability, completeness and precision.|[244]<br>Maian, Mythril, Osiris, Oyente, Securify,<br>**•** Coverage of Slither is the highest and the coverage of<br>**•** Vulnerabilities covered (Coverage), precision, and|Slither, SmartCheck, Remix3, SolidityCheck<br>tools based on control flow (or data flow) analysis is<br>recall.|usually low.|**•** Mythril has highest precision and Remix has the high-|est recall rate.|**•** The authors recommended to use Mythril, Slither, and|**•** The authors recommended to use Mythril, Slither, and|Remix for contract analysis.|[245]<br>Securify, SmartCheck, Slither, Oyente, Mythril,<br>**•** The code size, vulnerability pattern of the test suite,<br>**•** Vulnerabilities<br>covered<br>(Coverage),<br>precision,|Osiris, ContractFuzzer, sFuzz and ILF.<br>configuration parameters have a great impact on tool<br>recall|performance.|**•** Different tools have different performance on different|metrics.|[246]<br>HoneyBadger, Maian, Manticore, Mythril,<br>**•** 1. All tools combined are only able to detect 42% of the<br>**•** Combined accuracy of tools, Vulnerabilities cov-|Osiris, Oyente, Securify, Slither, Smartcheck<br>vulnerabilities from the dataset of annotated vulnera-<br>ered by each tool, average execution time per con-|Osiris, Oyente, Securify, Slither, Smartcheck<br>vulnerabilities from the dataset of annotated vulnera-<br>ered by each tool, average execution time per con-|ble SCs (48 out of 115).<br>tract and total time for all the contracts by each|**•** Mythril, the most accurate tool is able to detect only<br>tool.|27% of the vulnerabilities.|**•** The state-of- the-art techniques are far from being|**•** The state-of- the-art techniques are far from being|perfect, still likely producing too many false positives.|[247]<br>A deep insight of 86 static analysis and<br>**•** The symbolic execution, fuzz testing, constraint solv-<br>**•** Vulnerability covered, analysis approach, false pos-|dynamic analysis tools.<br>ing, code instrumentation, and code transforma-<br>itives, average execution time.|tion are most popular top five analysis approaches|employed by most tools.|**•** Top five vulnerabilities checked or detected by static or|dynamic analysis are: re-entrancy, arithmetic under-|flow/overflow, gas related, ToD, Timestamp depen-|flow/overflow, gas related, ToD, Timestamp depen-|dency.|**•** The Slither, Solhint, and Smart check perform better in|average execution time.|**•** Slither, Mythril, and Oyente provides better vulnera-|**35**<br>bility detection than others.|**of 50**<br>*FDR (False Discovery Rate)=FP / (TP+FP), FP=False Positive, TP=True Positive, FNR (False negative rate)=FN / (TP+FN), FN=False Negative.|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|||||||||||||||||||||||||||||||||||||||||||||
|||||||||||||||||||||||||||||||||||||||||||||



MONIKA et al. 

**36 of 50** 

_Formal verification._ Formal modeling and verification can play an essential role in maintaining the security of smart contracts. Formal verification gives the correctness proof of a system and is applied during the system’s design phase. The smart contract application to be verified is formally modeled first using model checking or theorem-proving techniques along with the required system properties and then verified formally. Formal verification can detect errors early in the design phase and save a lot of effort and money, as errors can be rectified quickly. Murray et al.[248] have discussed two types of formal methods: model checking and theorem proving, used for SCs and also discussed six research studies using these methods for formal verification of SCs. The study[117] has systematically studied formalization approaches used for SCs. The authors have discussed techniques such as theorem proving, symbolic execution, model checking, formal modeling, finite state machine, logic-based approach, behavioral modeling, and formal reasoning to solve one or more vulnerabilities of SCs. They have also discussed the Domain-Specific Languages (DSLs) used or proposed for blockchain SCs, such as B language, PROMELA, F*, ETHERLITE, Securify, Simplicity, dSLAC, SPESC, Lem, Findel, SMAC, and SCILLA. 

The authors[63] have investigated the formal models (Contract-level and program-level), formal specification techniques (temporal logics, hoare logics, path-level patterns, and other logics), and formal verification techniques (model checking, theorem proving, symbolic execution, program verification, and run-time verification) used for smart contracts in the literature till June 2020. The study[249] investigated SC formal verification methods and discussed five theorem-proving methods, six model-checking methods, and one run-time verification method for checking SC correctness. The authors have also presented 13 vulnerability detection tools for the security assurance of SCs. Table 10 shows the formal verification techniques proposed and used for smart contracts arranged year-wise. 

## 4.3.3 Open research challenges and future trends of blockchain interoperability 

As discussed earlier, blockchain interoperability can be achieved through the exchange or transfer of value. In exchange of value, the exchange rate has to be managed, whereas, in transfer, the old chain loses ownership of the asset, and the new chain gets it. HTLCs are mainly used for exchange purposes. Blockchain interoperability started from token/ crypto exchange; however, the focus is shifting toward attaining other types of interoperability, like sharing data among blockchains, sharing blockchain states, and DApps interacting with each other. The blockchain interoperability spectrum needs to be widened to enhance interaction between public and private blockchains, between legacy systems and blockchains, between DApps on the same blockchain, and between DApps on different blockchains. Sidechains, Relays, and HTLCs are the most practical, common, and easy-to-implement interoperability solutions for public blockchains. Next, we discuss interoperability challenges faced by the developers, and some future research directions are also provided: 

1. In HTLC solutions, handling the timeouts is a concern. Sometimes timeouts can be too long in the case of a public blockchain because of PoW-based consensus mechanisms. 

2. Ledger of ledger and other emerging interoperability solutions are inclined toward generalized interoperability. Cosmos and Polkadot allow users to create customized apps that interact with the other apps on the respective platform. However, cosmos users cannot interoperate with users of another ledger of ledger platforms. Ledger of ledger approaches require further research and improvements to make different platforms interoperable. 

3. Cross-chain technologies like sidechains also increase the scalability of the blockchain and open up the value circulation channel between the chains. However, sidechains and bridging solutions are costlier than HTLC and ILP solutions as they require validator nodes. Validator nodes, if not chosen carefully, can compromise the security of sidechains. 

4. Lack of standardization is the main hindrance to blockchain interoperability. According to the study,[167] standardized transaction format and syntax, and a minimal operation set common to all blockchains are required to make blockchains interoperable. 

5. One or more interoperability solutions types mentioned in this survey need to be merged to achieve a full-fledged and more practical solution for different blockchains to interact. 

6. Most of the solutions are theoretical and lack practical implementations; this gap needs to be merged. 

7. Handling transaction state finality between deterministic and probabilistic blockchains is an open research area. As blockchains lack a standard confirmation time; hence, further attention is needed to make different blockchains interoperable. 

8. Trustless cross-chain messaging techniques and cross-chain consensus need further research. 

MONIKA et al. 

**37 of 50** 

**TA B L E 10** Formal verification solutions for smart contracts. 

||~~**S.N.**~~<br>~~**Ref.**~~<br>~~**Year**~~<br>~~**Keywords**~~<br>1<br>[250]<br>2016<br>Solidity* and EVM* tools convert Solidity and bytecode to F*, OCaml, runtime safety,<br>and functional correctness.<br>2<br>[251]<br>2016<br>Town Crier, a link between blockchain front end (SC) and SGX trusted hardware<br>~~backend. Provides authenticated data feed to SCs, formally analyze using Universal~~<br>Composability (UC) framework.<br>3<br>[252]<br>2018<br>Hoare logic to formulate logical rules of SCs, verification using Isabelle/HOL theorem<br>prover.<br>4<br>[253]<br>2018<br>Modeled blockchain and user behavior along with SC modeling, Behavior Interaction<br>~~Priorities (BIP), and timed automata Statistical Model Checking (SMC) probability~~<br>evaluation.<br>5<br>[254]<br>2018<br>PROMELA, SC as non-deterministic automata, SC properties as Linear Temporal<br>Logic, SPIN model checker.<br>6<br>[255]<br>2018<br>State Machine Modeling,NusSMV model checker,SCproperties as Computation Tree<br>Logic.<br>7<br>[43]<br>2018<br>Executable EVM Formalization using K-framework, KEVM correctness verification<br>using Ethereum official test suite.<br>~~8~~<br>~~[256]~~<br>~~2019~~<br>~~Hierarchical model, Colored Petri Net (CPN) tools, ASK-CTL with state-space tools.~~<br>9<br>[257]<br>2019<br>Toy parser generator, AST, Tool chain produces PROMELA code, SPIN checker.<br>10<br>[258]<br>2019<br>Symbolic execution,higher-order theoremproving,definitional interpreter in Gallina,<br>SCs written in Lolisa, Latc mechanism of Coq.<br>11<br>[259]<br>2019<br>SAFEVM: Uses C verification engine, assert and require verification annotations, uses<br>Oyenete and EthIR framework, C+SV-COMP translator, C verifiers: CPAchecker,<br>VeryMax, and SeaHorn used.<br>~~12~~<br>~~[260]~~<br>~~2019~~<br>~~SCs are modeled using SAPIC (the applied pi-calculus) and verified using Tamarin prover.~~<br>13<br>[261]<br>2019<br>Language and system independent modeling, SC modeled as a state transition system<br>as Kripke structure, NuSMV model checker was used.<br>14<br>[262]<br>2020<br>Functional programs verification, deep embedding as an AST for meta-theoretical<br>~~properties reasoning, shallow embedding as a Coq function for concrete properties of~~<br>programs, MetaCoq plug-in, Acorn SC language.|
|---|---|



9. The difference in the scope of scripting languages poses challenges for blockchain interoperability. 

10. Present solutions connect public/private chains with similar chains or provide an ecosystem to create similar types of interoperable blockchains. Further research is required to connect permission-less and permissioned blockchains or connect the ledger of ledger approach to already present public blockchains. 

11. Further research is required to make blockchains interoperable with legacy (centralized) systems to turn the concept of a decentralized web into a reality. 

## 4.3.4 Open research challenges and future trends of the blockchain security 

In this study, we have provided a taxonomy of the security of blockchains by segregating security issues into two categories: Infrastructure-specific, that is, related blockchain architecture, and smart contract-specific, that is, related to applications developed on top of a blockchain using the concept of smart contracts. Next, we discuss open research challenges and some future directions related to both: 

## _Infrastructure specific_ 

We have only considered the survey studies related to infrastructure security issues and solutions. Still, this study provides readers with a good review of infrastructure-specific attacks and their proposed solutions. Most of the attacks mentioned 

MONIKA et al. 

**38 of 50** 

in the literature are possible and can happen in the worst-case scenario but have not been exploited in real life. The study[75] has presented 25 attacks that happened on blockchain systems, out of which only two are due to smart contract vulnerabilities, and the rest 23 are infrastructure-specific attacks by cybercriminals. Most of these infrastructure attacks[23,73,75] are related to hacking cryptocurrency exchanges either by launching a DDOS/Phishing attack or gaining access to passphrases of wallets such as Mt. Gox attack, Bitstamp attack, ShapeShift attack, Bitfinex attack, and Veritaseum attack. In Section 4.3.2, we have listed the techniques and solutions proposed by researchers and used by blockchain platforms to avoid infrastructure-specific attacks. Some of the open research challenges and future directions are as follows: 

1. An in-depth understanding of the security of blockchain technology is required. As blockchain technology is growing fast, platform consensus algorithms are changing, and privacy techniques are improving. Security concerns of PoW blockchains are different from PoS-based blockchains. 

2. Cryptographic algorithms used in blockchain are safe, but their security can be compromised with the advancement in quantum computing. Quantum resistance cryptographic algorithms are the need of the hour. 

3. Researchers have used machine learning techniques to detect anomalies and frauds on the Bitcoin network. Further research can apply ML techniques to other platforms and detect other types of anomalies/anomalous behavior, like doing data feature calculations for large amounts of network communication data to avoid attacks on decentralized networks. 

4. Upcoming blockchain platforms are shifting their focus from PoW toward PoS consensus as it requires less computation leading to less wastage of energy. However, for PoS-based blockchains, Long-range attacks are the most significant threats. Long-range attacks threaten the fundamental property of blockchain technology, that is, immutability. A malicious miner can rewrite the entire blockchain due to the slow discovery of the attack. 

5. The centralization of mining power needs to be controlled in already developed PoW blockchains. Disincentivizing large mining pools can help prevent the 51% attack. 

6. Privacy of transaction and anti-cryptojacking approaches need further research to avoid criminal activities against blockchain users. 

7. Some novel countermeasures are required for consensus-based attacks like collusion attacks in DPoS-based blockchains, double-spend, and Sybil attacks in shared-based blockchains. 

## _Smart contract specific_ 

The development of SC applications is significantly different from traditional software developments. Smart contracts are immutable once deployed; the irreversibility of SCs leads to increasing vulnerabilities and becomes a limitation for blockchain. SC applications’ working is greatly affected by the underlying blockchain platform. Security analysis and functional correctness of SCs are fundamental aspects of DApps. The DAO attack and Parity wallet attacks, as mentioned by,[75,90,193] have caused losses of millions of dollars just because of unrecognized smart contract bugs. Security audits play an essential role in detecting vulnerabilities before the deployment of SCs. SmartDec, Solidified, Zeppelin, and DejaVu[105] are some of the firms that provide smart contract security audits. Security audits are considered time-consuming and costly hence are not exercised commonly. We have identified the following challenges from the literature, which need immediate/further attention to make blockchain applications more secure, sustainable, and trustworthy. 

1. The semantic gap between SC and its actual execution is very crucial. The lack of expertise of developers in blockchain technology and its DSLs creates a mismatch between the intended and actual behavior of smart contracts. 

2. Serial execution of smart contracts limits the performance of DApps, and this is because of the limited throughput of blockchain. 

3. It is challenging to formalize Turing complete languages. Solidity, the most prominently used programming language for SCs, is Turing complete. 

4. Numerous security analysis tools have been found in the literature, but there are no standard benchmarks to assess the tools’ quality. Tools designed for SCs should be open source to build trust and make it sustainable by reusability of code segments. 

5. Benchmark datasets of SCs are not available to check the efficiency and accuracy of security analysis tools. 

6. To avoid Ponzi schemes, users should be aware of how to check the authenticity of smart contracts, read the smart contract thoroughly, and check the transaction log of the smart contract. 

MONIKA et al. 

**39 of 50** 

7. Detecting still unknown vulnerabilities and optimizing the vulnerability detection methods to increase the accuracy and efficiency of security analysis tools. 

8. Designing a full-fledged security verification solution for a smart contract is required to guarantee security vulnerability analysis and formal functional verification of an SC. 

9. Automating vulnerability detection, formal testing, formal modeling, and formal verification requires further research. 

10. More powerful interactive debuggers are required to make SC debugging automatic and more effortless. SC compilers need to be bug-free. Complier formalization and testing are needed. 

11. Source code-level gas estimation and optimization tools are required. 

12. Lack of secure third-party libraries, lack of community support, and lack of best practices and standards for SC development 

13. SC programming languages are not mature and change frequently. Developers are not able to get up-to-date documentation. Formally tested high-level domain-specific languages are needed. 

14. Automatic smart contract patching, like in traditional software, would make the maintenance of SC much easier. 

15. New intermediate-level languages like Scilla are required to design safer smart contracts. 

16. Blockchain-based software engineering can be used to address security at the system level and make smart contract systems more robust. 

17. Incorporating a general programming language verifier in the blockchain scenario is challenging for platforms that use a general-purpose programming language to code SCs. 

18. The correctness of verification matters the most for formal verification techniques, and the limitation of memory and time while executing the contract for verification questions the correctness of the proofs. 

19. Lack of design paradigms or patterns for SCs. 

## **5 DISCUSSION** 

We reviewed 259 research papers published in renowned journals, conferences, and workshops until June 2022 and classified them into different areas. There is no extensive literature review on blockchain discussing its current status in platforms’ development and factors affecting its adoption and sustainability. The studies[39,263,264] analyzed the publication trend related to blockchain technology to show that the interest of researchers has shifted from Bitcoin to blockchain technology. The study[48] presents a systematic study on blockchain research discussing challenges and top benefits from a business and society perspective, excluding computer science-related and technical papers. The authors[3,45] have systemically studied use cases and blockchain technology applications. Hence, to analyze blockchain technology’s growth from a technical perspective, we framed three research questions, each with further sub-questions, to figure out the current status of the technology. The focus of this study is different and comprehensive than previously mentioned studies, including the latest research work published and gray literature up to June 2022. The results of RQ-1 show all the issues related to blockchain adoption, security, and interoperability. RQ-3 results discuss all the solutions proposed to improve the security and interoperability of blockchains with yet-to-solve future challenges that require further research. Next, we will present the main findings of our study in a concise manner. 

## **5.1 Current status of key research areas** 

Different aspects of blockchain technology are interrelated, for example, the consensus algorithm affects the computation cost and scalability, affecting the adoption of the technology. Hence the adoption of blockchain technology depends on progress in all aspects, such as consensus mechanism, scalability, privacy and anonymity, standardization, regulatory concerns, security, and interoperability. Among all the aspects, standardization and regulatory concerns are the two areas that need more advancements. 

Much research is done on improving the consensus, privacy, security, and scalability issues. Switching from PoW to PoS/other fast finality consensus mechanisms or sharding based consensus algorithms is still a challenging task. Development of layer-2 solutions like payment channels, sidechains, cross-chain communications, and off-chain computations are the main factors affecting the adoption of blockchain. Regulatory uncertainties constitute a significant risk hindering blockchain adoption and need further attention. 

MONIKA et al. 

**40 of 50** 

Use cases and applications of blockchain have been explored substantially in recent years, as numerous platforms have been developed, and DApp development is still receiving much attention from developers worldwide. There is no standard benchmark for comparing blockchain platforms, no standard workloads for assessing different layers of a blockchain, and no standard metrics for the quality assessment of a platform. Performance evaluation of blockchains is a research area that needs urgent attention. As the usage of layer-2 solutions by blockchain platforms increases, the scalability and applicability of blockchain will be enhanced, and it can solve the problems of centralized systems. 

The security of smart contracts in blockchain technology has been studied thoroughly in the literature. The immaturity of blockchain technology and the lack of experienced developers are the leading causes behind the security issues of the technology. There is much scope in designing DSLs and formal validation of DSLs, compliers, and SC solutions. Developing a standard benchmark for assessing the quality of security analysis tools would be a breakthrough. We have observed an urgent need for an analysis study on security solutions for infrastructure-specific security issues. 

Blockchain interoperability is an open research area and is still in its infancy stage. The inherent nature of blockchain technology to act as a standalone ledger and the lack of standardization for the development of platforms hinder cross-chain communication. Most of the research work is related to achieving interoperability in permissioned blockchains. Interoperability among public blockchains needs to be explored more. Accomplishing blockchain interoperability will take some time, but it is the most critical aspect of blockchain technology to make it attain its full potential. 

## **6 CONCLUSION** 

In this study, we present a comprehensive survey on the progress of blockchain technology with a specific focus on the platforms, security, and interoperability aspects of blockchain. We divided 259 studies into distinct categories related to different blockchain aspects and discussed each category in detail. Our study shows the progress of blockchain from 2009 till the present time. We have identified and presented eight major challenges affecting blockchain adoption. Our study lays out a taxonomy related to security issues to provide users with a clear picture of security concerns at different levels of blockchain. 

After analyzing 40 blockchain platforms, our study shows that much work has been done on developing blockchain platforms to serve the needs of different application domains. We have analyzed that the current development trend is DApps oriented. According to our analysis, interoperability among different platforms and DApps will decide the future of blockchain. Hence, we have discussed interoperability issues faced by blockchain and corresponding solutions developed by academia and industry. We have presented 32 infrastructure-specific security issues and 27 smart contract-specific issues to be kept in mind while designing any blockchain platform or blockchain-based application. Our study reports 53 security solutions, including tools, domain-specific languages, and techniques to design safer smart contracts. This study can help researchers look for open-research issues as we have outlined numerous challenges and future trends related to blockchain technology. Our study shows that much research has been done and is still going on in every aspect of blockchain technology, as it is a new technology having much potential. 

## **AUTHOR CONTRIBUTIONS** 

Monika: Conceptualization, Investigation, Methodology, Writing-original draft. Rajesh Bhatia, Manish Kumar: Conceptualization, Validation, Supervision, Review, and editing. 

## **FUNDING INFORMATION** 

The authors declare that no funds, grants, or other support were received during the preparation of this manuscript. 

## **CONFLICT OF INTEREST STATEMENT** 

The authors have no relevant financial or nonfinancial interests to disclose. 

## **DATA AVAILABILITY STATEMENT** 

Data sharing not applicable to this article as no datasets were generated or analysed during the current study. 

MONIKA et al. 

**41 of 50** 

## **ORCID** 

_Monika_ https://orcid.org/0000-0002-7306-9450 

## **REFERENCES** 

1. Di Francesco Maesa D, Mori P. Blockchain 3.0 applications survey. _J Parallel Distrib Comput_ . 2020;138:99-114. doi:10.1016/j.jpdc.2019.12.019 

2. Risius M, Spohrer K. A Blockchain research framework: what we (don’t) know, where we go from here, and how we will get there. _Bus Inf Syst Eng_ . 2017;59(6):385-409. doi:10.1007/s12599-017-0506-0 

3. Abou Jaoude J, George Saade R. Blockchain applications – usage in different domains. _IEEE Access_ . 2019;7:45360-45381. doi:10.1109/ACCESS.2019.2902501 

4. Monrat AA, Schelén O, Andersson K. A survey of blockchain from the perspectives of applications, challenges, and opportunities. _IEEE Access_ . 2019;7:117134-117151. doi:10.1109/ACCESS.2019.2936094 

5. Zhang R, Xue R, Liu L. Security and privacy on blockchain. _ACM Comput Surv_ . 2019;52(3):1-34. doi:10.1145/3316481 

6. Colomo-Palacios R, Sánchez-Gordón M, Arias-Aranda D. A critical review on blockchain assessment initiatives: a technology evolution viewpoint. _J Softw Evol Process_ . 2020;32(11):1-11. doi:10.1002/smr.2272 

7. Islam I, Munim KM, Oishwee SJ, Islam AKMN, Islam MN. A critical review of concepts, benefits, and pitfalls of Blockchain technology using concept map. _IEEE Access_ . 2020;8:68333-68341. doi:10.1109/ACCESS.2020.2985647 

8. Butijn BJ, Tamburri DA, Van Den Heuvel WJ. Blockchains: a systematic multivocal literature review. _ACM Comput Surv_ . 2020;53(3):1-15. doi:10.1145/3369052 

9. Kolb J, Abdelbaky M, Katz RH, Culler DE. Core concepts, challenges, and future directions in blockchain: a centralized tutorial. _ACM Comput Surv_ . 2020;53(1):1-39. doi:10.1145/3366370 

10. Sanka AI, Irfan M, Huang I, Cheung RCC. A survey of breakthrough in blockchain technology: adoptions, applications, challenges and future research. _Comput Commun_ . 2021;169:179-201. doi:10.1016/j.comcom.2020.12.028 

11. Huang H, Kong W, Zhou S, Zheng Z, Guo S. A survey of State-of-the-art on Blockchains: theories, Modelings, and tools. _ACM Comput Surv_ . 2021;54(2):1-42. doi:10.1145/3441692 

12. Dotan M, Pignolet YA, Schmid S, Tochner S, Zohar A. Survey on Blockchain networking: context, State-of-the-art, challenges. _ACM Comput Surv_ . 2021;54(5):1-34. doi:10.1145/3453161 

13. Bhutta MNM, Khwaja AA, Nadeem A, et al. A survey on Blockchain technology: evolution, architecture and security. _IEEE Access_ . 2021;9:61048-61073. doi:10.1109/ACCESS.2021.3072849 

14. Lohachab A, Garg S, Kang B, et al. Towards interconnected Blockchains: a comprehensive review of the role of interoperability among disparate Blockchains. _ACM Comput Surv_ . 2022;54(7):1-39. doi:10.1145/3460287 

15. Chen Y, Chen H, Zhang Y, Han M, Siddula M, Cai Z. A survey on blockchain systems: attacks, defenses, and privacy preservation. _High-Confidence Comput_ . 2022;2(2):100048. doi:10.1016/J.HCC.2021.100048 

16. Shrimali B, Patel HB. Blockchain state-of-the-art: architecture, use cases, consensus, challenges and opportunities. _J King Saud Univ – Comput Inf Sci_ . 2022;34(9):6793-6807. doi:10.1016/j.jksuci.2021.08.005 

17. Gad AG, Mosa DT, Abualigah L, Abohany AA. Emerging trends in Blockchain technology and applications: a review and outlook. _J King Saud Univ – Comput Inf Sci_ . 2022;34:6719-6742. doi:10.1016/j.jksuci.2022.03.007 

18. Ahmadjee S, Mera-Gómez C, Bahsoon R, Kazman R. A study on Blockchain architecture design decisions and their security attacks and threats. _ACM Trans Softw Eng Methodol_ . 2022;31(2):1-45. doi:10.1145/3502740 

19. Cryptocurrency Prices, Charts And Market Capitalizations | CoinMarketCap. https://coinmarketcap.com/ 

20. Zhou Q, Huang H, Zheng Z, Bian J. Solutions to scalability of Blockchain: a survey. _IEEE Access_ . 2020;8:16440-16455. doi:10.1109/aCCESS.2020.2967218 

21. Hafid A, Hafid AS, Samih M. Scaling Blockchains: a comprehensive survey. _IEEE Access_ . 2020;8:125244-125262. doi:10.1109/ACCESS.2020.3007251 

22. Nasir MH, Arshad J, Khan MM, Fatima M, Salah K, Jayaraman R. Scalable blockchains — a systematic review. _Future Gener Comput Syst_ . 2022;126:136-162. doi:10.1016/j.future.2021.07.035 

23. Conti M, Sandeep KE, Lal C, Ruj S. A survey on security and privacy issues of bitcoin. _IEEE Commun Surv Tutor_ . 2018;20(4):3416-3452. doi:10.1109/COMST.2018.2842460 

24. Bernal Bernabe J, Canovas JL, Hernandez-Ramos JL, Torres Moreno R, Skarmeta A. Privacy-preserving solutions for Blockchain: review and challenges. _IEEE Access_ . 2019;7:164908-164940. doi:10.1109/ACCESS.2019.2950872 

25. Feng Q, He D, Zeadally S, Khan MK, Kumar N. A survey on privacy protection in blockchain system. _J Netw Comput Appl_ . 2019;126:45-58. doi:10.1016/j.jnca.2018.10.020 

26. Andola N, Raghav VK, Yadav SV, Verma S. Anonymity on blockchain based e-cash protocols – a survey. _Comput Sci Rev_ . 2021;40:100394. doi:10.1016/j.cosrev.2021.100394 

27. Wang W, Hoang DT, Hu P, et al. A survey on consensus mechanisms and mining strategy management in Blockchain networks. _IEEE Access_ . 2019;7:22328-22370. doi:10.1109/ACCESS.2019.2896108 

28. Xiao Y, Zhang N, Lou W, Hou YT. A survey of distributed consensus protocols for Blockchain networks. _IEEE Commun Surv Tutor_ . 2020;22(2):1432-1465. doi:10.1109/COMST.2020.2969706 

MONIKA et al. 

**42 of 50** 

29. Lashkari B, Musilek P. A comprehensive review of Blockchain consensus mechanisms. _IEEE Access_ . 2021;9:43620-43652. doi:10.1109/ACCESS.2021.3065880 

30. Singh A, Kumar G, Saha R, Conti M, Alazab M, Thomas R. A survey and taxonomy of consensus protocols for blockchains. _J Syst Archit_ . 2022;127:102503. doi:10.1016/j.sysarc.2022.102503 

31. Truby J. Decarbonizing bitcoin: law and policy choices for reducing the energy consumption of Blockchain technologies and digital currencies. _Energy Res Soc Sci_ . 2018;44:399-410. doi:10.1016/j.erss.2018.06.009 

32. Vranken H. Sustainability of bitcoin and blockchains. _Curr Opin Environ Sustain_ . 2017;28:1-9. doi:10.1016/j.cosust.2017.04.011 

33. Küfeoglu _̆_ S, Özkuran M. Bitcoin mining: a global review of energy and power demand. _Energy Res Soc Sci_ . 2019;58:101273. doi:10.1016/j.erss.2019.101273 

34. Mohanta BK, Jena D, Panda SS, Sobhanayak S. Blockchain technology: a survey on applications and security privacy challenges. _IOT_ . 2019;8:100107. doi:10.1016/j.iot.2019.100107 

35. Ali Syed T, Alzahrani A, Jan S, Siddiqui MS, Nadeem A, Alghamdi T. A comparative analysis of Blockchain architecture and its applications: problems and recommendations. _IEEE Access_ . 2019;7:176838-176869. doi:10.1109/ACCESS.2019.2957660 

36. Yuan Y, Wang FY. Blockchain and cryptocurrencies: model, techniques, and applications. _IEEE Trans Syst Man Cybern Syst_ . 2018;48(9):1421-1428. doi:10.1109/TSMC.2018.2854904 

37. Wan S, Li M, Liu G, Wang C. Recent advances in consensus protocols for blockchain: a survey. _Wirel Netw_ . 2020;26(8):5579-5593. doi:10.1007/s11276-019-02195-0 

38. Mahmoud QH, Lescisin M, AlTaei M. Research challenges and opportunities in blockchain and cryptocurrencies. _Internet Technol Lett_ . 2019;2(2):e93. doi:10.1002/itl2.93 

39. Dabbagh M, Sookhak M, Safa NS. The evolution of blockchain: a bibliometric study. _IEEE Access_ . 2019;7:19212-19221. doi:10.1109/ACCESS.2019.2895646 

40. Wust K, Gervais A. Do you need a blockchain? _2018 Crypto valley conference on blockchain technology (CVCBT)_ . Proc-IEEE; 2018:45-54. doi:10.1109/CVCBT.2018.00011 

41. Xu M, Chen X, Kou G. A systematic review of blockchain. _Financ Innov_ . 2019;5:27. doi:10.1186/s40854-019-0147-z 

42. Top Blockchain Dapps | DappRadar. https://dappradar.com/rankings 

43. Hildenbrandt E, Saxena M, Rodrigues N, et al. KEVM: a complete formal semantics of the ethereum virtual machine. _Proc – IEEE Comput Secur Found Symp_ . 2018;2018:204-217. doi:10.1109/CSF.2018.00022 

44. Wang S, Ding W, Li J, Yuan Y, Ouyang L, Wang FY. Decentralized autonomous organizations: concept, model, and applications. _IEEE Trans Comput Soc Syst_ . 2019;6(5):870-878. doi:10.1109/TCSS.2019.2938190 

45. Shen C, Pena-Mora F. Blockchain for cities – a systematic literature review. _IEEE Access_ . 2018;6:76787-76819. doi:10.1109/ACCESS.2018.2880744 

46. Salah K, Rehman MHU, Nizamuddin N, Al-Fuqaha A. Blockchain for AI: review and open research challenges. _IEEE Access_ . 2019;7:10127-10149. doi:10.1109/ACCESS.2018.2890507 

47. Ølnes S, Ubacht J, Janssen M. Blockchain in government: benefits and implications of distributed ledger technology for information sharing. _Gov Inf Q_ . 2017;34(3):355-364. doi:10.1016/j.giq.2017.09.007 

48. Perera S, Nanayakkara S, Rodrigo MNN, Senaratne S, Weinand R. Blockchain technology: is it hype or real in the construction industry? _J Ind Inf Integr_ . 2020;17:100125. doi:10.1016/j.jii.2020.100125 

49. Frizzo-Barker J, Chow-White PA, Adams PR, Mentanko J, Ha D, Green S. Blockchain as a disruptive technology for business: a systematic review. _Int J Inf Manage_ . 2020;51:102029. doi:10.1016/j.ijinfomgt.2019.10.014 

50. Al-Jaroodi J, Mohamed N. Blockchain in industries: a survey. _IEEE Access_ . 2019;7:36500-36515. doi:10.1109/ACCESS.2019.2903554 

51. Gao W, Hatcher WG, Yu W. A survey of blockchain: techniques, applications, and challenges. _Proc – Int Conf Comput Commun Netw_ . 2018:1-11.doi:10.1109/ICCCN.2018.8487348 

52. Siris VA, Nikander P, Voulgaris S, Fotiou N, Lagutin D, Polyzos GC. Interledger approaches. _IEEE Access_ . 2019;7:89948-89966. doi:10.1109/ACCESS.2019.2926880 

53. Belchior R, Vasconcelos A, Guerreiro S, Correia M. A survey on Blockchain interoperability: past, present, and future trends. _ACM Comput Surv_ . 2021;54(8):1-41. doi:10.1145/3471140 

54. Ethereum 2.0 (Eth2). https://ethereum.org/en/upgrades/merge/ 

55. Chen Y, Bellavitis C. Blockchain disruption and decentralized finance: the rise of decentralized business models. _J Bus Ventur Insights_ . 2020;13:e00151. doi:10.1016/j.jbvi.2019.e00151 

56. Prewett KW, Prescott GL, Phillips K. Blockchain adoption is inevitable—barriers and risks remain. _J Corp Account Financ_ . 2020;31(2):21-28. doi:10.1002/jcaf.22415 

57. Drljevic N, Aranda DA, Stantchev V. Perspectives on risks and standards that affect the requirements engineering of blockchain technology. _Comput Stand Interf_ . 2020;69:103409. doi:10.1016/j.csi.2019.103409 

58. Ali O, Jaradat A, Kulakli A, Abuhalimeh A. A comparative study: Blockchain technology utilization benefits, challenges and functionalities. _IEEE Access_ . 2021;9:12730-12749. doi:10.1109/ACCESS.2021.3050241 

59. Akram SV, Malik PK, Singh R, Anita G, Tanwar S. Adoption of blockchain technology in various realms: opportunities and challenges. _Secur Priv_ . 2020;3(5):1-17. doi:10.1002/spy2.109 

60. Zou W, Lo D, Kochhar PS, et al. Smart contract development: challenges and opportunities. _IEEE Trans Softw Eng_ . 2021;47(10):2084-2106. doi:10.1109/TSE.2019.2942301 

MONIKA et al. 

**43 of 50** 

61. Hu B, Zhang Z, Liu J, et al. A comprehensive survey on smart contract construction and execution: paradigms, tools, and systems. _Patterns_ . 2021;2(2):100179. doi:10.1016/j.patter.2020.100179 

62. Kushwaha SS, Joshi S, Singh D, Kaur M, Lee HN. Systematic review of security vulnerabilities in Ethereum Blockchain smart contract. _IEEE Access_ . 2022;10:6605-6621. doi:10.1109/ACCESS.2021.3140091 

63. Tolmach P, Li Y, Lin SW, Liu Y, Li Z. A survey of smart contract formal specification and verification. _ACM Comput Surv_ . 2022;54(7):1-38. doi:10.1145/3464421 

64. Eklund PW, Beck R. Factors that Impact Blockchain Scalability. _Proceedings of the 11th International Conference on Management of Digital EcoSystems_ . ACM; 2019:126-133. doi:10.1145/3297662.3365818 

65. Lu Y. The blockchain: State-of-the-art and research challenges. _J Ind Inf Integr_ . 2019;15:80-90. doi:10.1016/j.jii.2019.04.002 

66. Wang S, Ouyang L, Yuan Y, Ni X, Han X, Wang F-Y. Blockchain-enabled smart contracts: architecture, applications, and future trends. _IEEE Trans Syst Man Cybern Syst_ . 2019;49(11):1-12. doi:10.1109/TSMC.2019.2895123 

67. Anjana PS, Kumari S, Peri S, Rathor S, Somani A, Vandenbogaerde B. An Efficient Framework for Optimistic Concurrent Execution of Smart Contracts. _27th Euromicro International Conference on Parallel, Distributed and Network-Based Processing (PDP)_ . IEEE; 2019:1220-1222. doi:10.1109/EMPDP.2019.8671637 

68. Casino F, Dasaklis TK, Patsakis C. A systematic literature review of blockchain-based applications: current status, classification and open issues. _Telemat Inform_ . 2019;36:55-81. doi:10.1016/j.tele.2018.11.006 

69. Kus Khalilov MC, Levi A. A survey on anonymity and privacy in bitcoin-like digital cash systems. _IEEE Commun Surv Tutor_ . 2018;20(3):2543-2585. doi:10.1109/COMST.2018.2818623 

70. Alsalami N, Zhang B. SoK: A Systematic Study of Anonymity in Cryptocurrencies. _IEEE Conference on Dependable and Secure Computing (DSC)_ . IEEE; 2019. doi:10.1109/DSC47296.2019.8937681 

71. dos Santos AP, Chaczko Z. Blockchain: Status-Quo, Enablers And Inhibitors. _26th International Conference on Systems Engineering (ICSEng)_ . IEEE; 2018:1-6. doi:10.1109/ICSENG.2018.8638187 

72. Kher R, Terjesen S, Liu C. Blockchain, bitcoin, and ICOs: a review and research agenda. _Small Bus Econ_ . 2020;56:1699-1720. doi:10.1007/s11187-019-00286-y 

73. Rahouti M, Xiong K, Ghani N. Bitcoin concepts, threats, and machine-learning security solutions. _IEEE Access_ . 2018;6:67189-67205. doi:10.1109/ACCESS.2018.2874539 

74. Wan Z, Lo D, Xia X, Cai L. Bug characteristics in Blockchain systems: a large-scale empirical study. _IEEE Int Work Conf Min Softw Repos_ . 2017;413-424. doi:10.1109/MSR.2017.59 

75. Averin A, Averina O. Review of Blockchain Technology Vulnerabilities and Blockchain-System Attacks. _2019 International Multi-Conference on Industrial Engineering and Modern Technologies (FarEastCon)_ . IEEE; 2019:1-6. doi:10.1109/FarEastCon.2019.8934243 

76. Wang H, Wang Y, Cao Z, Li Z, Xiong G. An Overview of Blockchain Security Analysis. _Communications in Computer and Information Science_ . Vol 970. Springer; 2019:55-72. 

77. Zhang P, Zhou M. Security and Trust in Blockchains: architecture, key technologies, and open issues. _IEEE Trans Comput Soc Syst_ . 2020;7(3):790-801. doi:10.1109/TCSS.2020.2990103 

78. Homoliak I, Venugopalan S, Reijsbergen D, Hum Q, Schumi R, Szalachowski P. The security reference architecture for Blockchains: toward a standardized model for studying vulnerabilities, threats, and defenses. _IEEE Commun Surv Tutor_ . 2021;23(1):341-390. doi:10.1109/COMST.2020.3033665 

79. Shah R, Sridaran R. A study on Security and Privacy Related Issues in Blockchain Based Applications. _2019 6th International Conference on Computing for Sustainable Global Development (INDIACom)_ . IEEE; 2019:1240-1244. 

80. Dasgupta D, Shrein JM, Gupta KD. A survey of blockchain from security perspective. _J Bank Financ Technol_ . 2019;3(1):1-17. doi:10.1007/s42786-018-00002-6 

81. Li X, Jiang P, Chen T, Luo X, Wen Q. A survey on the security of blockchain systems. _Future Gener Comput Syst_ . 2020;107:841-853. doi:10.1016/j.future.2017.08.020 

82. Holbrook J. Blockchain security and threat landscape. _Architecting Enterprise Blockchain Solutions_ . Wiley; 2020:323-347. 

83. Saad M, Spaulding J, Njilla L, Kamhoua CA, Nyang D, Mohaisen A. Overview of attack surfaces in Blockchain. _Blockchain for Distributed Systems Security_ . Wiley; 2019:51-66. 

84. Wang Z, Yu H, Zhang Z, Piao J, Liu J. ECDSA weak randomness in bitcoin. _Future Gener Comput Syst_ . 2020;102:507-513. doi:10.1016/j.future.2019.08.034 

85. Deirmentzoglou E, Papakyriakopoulos G, Patsakis C. A survey on long-range attacks for proof of stake protocols. _IEEE Access_ . 2019;7:28712-28725. doi:10.1109/ACCESS.2019.2901858 

86. Quamara S, Singh AK. A systematic survey on security concerns in cryptocurrencies: State-of-the-art and perspectives. _Comput Secur_ . 2022;113:102548. doi:10.1016/j.cose.2021.102548 

87. Gervais A, Karame GO, Wüst K, Glykantzis V, Ritzdorf H, Capkun _[̌]_ S. On the Security and Performance of Proof of Work Blockchains. _Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ . Vol 24-28. ACM; 2016:3-16. doi:10.1145/2976749.2978341 

88. Natoli C, Gramoli V. The Balance Attack or Why Forkable Blockchains are Ill-Suited for Consortium. _2017 47th Annual IEEE/IFIP International Conference on Dependable Systems and Networks (DSN)_ . Vol 2017. IEEE; 2017:579-590. doi:10.1109/DSN.2017.44 

89. Guo H, Yu X. A survey on blockchain technology and its security. _Blockchain Res Appl_ . 2022;3(2):100067. doi:10.1016/j.bcra.2022.100067 

MONIKA et al. 

**44 of 50** 

90. Sayeed S, Marco-Gisbert H, Caira T. Smart contract: attacks and protections. _IEEE Access_ . 2020;8:24416-24427. doi:10.1109/ACCESS.2020.2970495 

91. Chicarino V, Albuquerque C, Jesus E, Rocha A. On the detection of selfish mining and stalker attacks in blockchain networks. _Ann Des Telecommun Telecommun_ . 2020;75(3–4):143-152. doi:10.1007/s12243-019-00746-2 

92. Ikeda K. Security and Privacy of Blockchain and Quantum Computation. _Advances in Computers_ . Vol 111. 1st ed. Elsevier Inc.; 2018:199-228. 

93. Cheng J, Xie L. A survey of security threats and defense on Blockchain. _Multimed Tools Appl_ . 2021;80:30623-30652. doi:10.1007/s11042-020-09368-6 

94. Xu G, Guo B, Su C, et al. Am I eclipsed? A smart detector of eclipse attacks for Ethereum. _Comput Secur_ . 2020;88:101604. doi:10.1016/j.cose.2019.101604 

95. Saad M, Cook V, Nguyen L, Thai MT, Mohaisen A. Partitioning attacks on bitcoin: colliding space, time, and logic. _IEEE 39th International Conference on Distributed Computing Systems (ICDCS)_ . 2019;2019:1175-1187. doi:10.1109/ICDCS.2019.00119 

96. Wu D, Liu XD, Yan XB, Peng R, Li G. Equilibrium analysis of bitcoin block withholding attack: a generalized model. _Reliab Eng Syst Saf_ . 2019;185:318-328. doi:10.1016/j.ress.2018.12.026 

97. Elliott S. Nash Equilibrium of Multiple, Non-Uniform Bitcoin Block Withholding Attackers. _2019 2nd International Conference on Data Intelligence and Security (ICDIS)_ . IEEE; 2019:144-151. doi:10.1109/ICDIS.2019.00029 

98. Zamyatin A, Al-Bassam M, Zindros D, et al. SoK: Communication across Distributed Ledgers. 2019 http://www0.cs.ucl.ac.uk/staff/M .AlBassam/publications/crosschain.pdf 

99. Kadena E, Holicza P. Security Issues in the Blockchain(ed) World. _2018 IEEE 18th International Symposium on Computational Intelligence and Informatics (CINTI)_ . Vol 2018. IEEE; 2018;211-216. doi:10.1109/CINTI.2018.8928212 

100. Huang Y, Bian Y, Li R, Zhao JL, Shi P. Smart contract security: a software lifecycle perspective. _IEEE Access_ . 2019;7:150184-150202. doi:10.1109/ACCESS.2019.2946988 

101. Zheng Z, Xie S, Dai HN, et al. An overview on smart contracts: challenges, advances and platforms. _Future Gener Comput Syst_ . 2020;105:475-491. doi:10.1016/j.future.2019.12.019 

102. Bartoletti M, Carta S, Cimoli T, Saia R. Dissecting Ponzi schemes on Ethereum: identification, analysis, and impact. _Future Gener Comput Syst_ . 2020;102:259-277. doi:10.1016/j.future.2019.08.014 

103. Di Angelo M, Salzer G. Mayflies, breeders, and busy bees in ethereum: smart contracts over time. _Proceedings of the Third ACM Workshop on Blockchains, Cryptocurrencies and Contracts_ . ACM; 2019:1-10. doi:10.1145/3327959.3329537 

104. Demir M, Alalfi M, Turetken O, Ferworn A. Security Smells in Smart Contracts. _2019 IEEE 19th International Conference on Software Quality, Reliability and Security Companion (QRS-C)_ . IEEE; 2019:442-449. doi:10.1109/QRS-C.2019.00086 

105. Dika A, Nowostawski M. Security Vulnerabilities in Ethereum Smart Contracts. _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ . IEEE; 2018:955-962. doi:10.1109/Cybermatics_2018.2018.00182 

106. Chen H, Pendleton M, Njilla L, Xu S. A survey on Ethereum systems security: vulnerabilities, attacks, and defenses. _ACM Comput Surv_ . 2020;53(3):1-43. doi:10.1145/3391195 

107. Macrinici D, Cartofeanu C, Gao S. Smart contract applications within blockchain technology: a systematic mapping study. _Telemat Inform_ . 2018;35(8):2337-2354. doi:10.1016/j.tele.2018.10.004 

108. Neudecker T, Hartenstein H. Network layer aspects of permissionless blockchains. _IEEE Commun Surv Tutor_ . 2019;21(1):838-857. doi:10.1109/COMST.2018.2852480 

109. Gourisetti SNG, Mylrea M, Patangia H. Evaluation and demonstration of Blockchain applicability framework. _IEEE Trans Eng Manag_ . 2020;67(4):1142-1156. doi:10.1109/TEM.2019.2928280 

110. Niranjanamurthy M, Nithya BN, Jagannatha S. Analysis of Blockchain technology: pros, cons and SWOT. _Clust Comput_ . 2018;5(2):1-15. doi:10.1007/s10586-018-2387-5 

111. Onik MMH, Miraz MH. Performance Analytical Comparison of Blockchain-as-a-Service (BaaS) Platforms. In: Miraz M, Excell P, Ware A, Soomro S, Ali M, eds. _Emerging Technologies in Computing. iCETiC 2019_ . Lecture Notes of the Institute for Computer Sciences, Social Informatics and Telecommunications Engineering. Vol 285. Springer International Publishing; 2019:3-18. 

112. Zhou A, Milani HZ, Fard AM, Makanju A. The State of Ethereum smart contracts security: vulnerabilities, countermeasures, and tool support. _J Cybersecurity Priv_ . 2022;2(2):358-378. doi:10.3390/JCP2020019 

113. Wang F, Chen Y, Wang R, et al. An experimental investigation into the hash functions used in Blockchains. _IEEE Trans Eng Manag_ . 2020;67(4):1404-1424. doi:10.1109/TEM.2019.2932202 

114. Vacca A, Di Sorbo A, Visaggio CA, Canfora G. A systematic literature review of blockchain and smart contract development: techniques, tools, and open challenges. _J Syst Softw_ . 2021;174:110891. doi:10.1016/j.jss.2020.110891 

115. Chen T, Li X, Luo X, Zhang X. Under-optimized smart contracts devour your money. _2017 IEEE 24th International Conference on Software Analysis, Evolution and Reengineering (SANER)_ . IEEE; 2017:442-446. doi:10.1109/SANER.2017.7884650 

116. Chen T, Li Z, Zhou H, et al. Towards Saving Money in Using Smart Contracts. _IEEE/ACM 40th International Conference on Software Engineering: New Ideas and Emerging Technologies Results (ICSE-NIER)_ . IEEE/ACM; 2018:81-84. doi:10.1145/3183399.3183420 

117. Singh A, Parizi RM, Zhang Q, Choo KKR, Dehghantanha A. Blockchain smart contracts formalization: approaches and challenges to address vulnerabilities. _Comput Secur_ . 2020;88:101654. doi:10.1016/j.cose.2019.101654 

118. Chen W, Zheng Z, Cui J, Ngai E, Zheng P, Zhou Y. Detecting Ponzi Schemes on Ethereum: Towards Healthier Blockchain Technology. _WWW’18: Proceedings of the 2018 World Wide Web Conference_ . ACM; 2018:1409-1418. doi:10.1145/3178876.3186046 

MONIKA et al. 

**45 of 50** 

119. Chen W, Zheng Z, Ngai ECH, Zheng P, Zhou Y. Exploiting Blockchain data to detect smart Ponzi schemes on Ethereum. _IEEE Access_ . 2019;7:37575-37586. doi:10.1109/ACCESS.2019.2905769 

120. Gupta R, Tanwar S, Al-Turjman F, Italiya P, Nauman A, Kim SW. Smart contract privacy protection using AI in cyber-physical systems: tools, techniques and challenges. _IEEE Access_ . 2020;8:24746-24772. doi:10.1109/ACCESS.2020.2970576 

121. Wang S, Zhang C, Su Z. Detecting nondeterministic payment bugs in Ethereum smart contracts. _Proc ACM Program Lang_ . 2019;3:1-29. doi:10.1145/3360615 

122. Tikhomirov S, Voskresenskaya E, Ivanitskiy I, Takhaviev R, Marchenko E, Alexandrov Y. Smartcheck: Static Analysis of Ethereum Smart Contracts. _IEEE/ACM 1st International Workshop on Emerging Trends in Software Engineering for Blockchain (WETSEB)_ . IEEE/ACM; 2018:9-16. doi:10.1145/3194113.3194115 

123. Lu N, Wang B, Zhang Y, Shi W, Esposito C. NeuCheck: a more practical Ethereum smart contract security analysis tool. _Softw Pract Exp_ . 2019;51:1-20. doi:10.1002/spe.2745 

124. Jiang B, Liu Y, Chan WK. ContractFuzzer: Fuzzing Smart Contracts for Vulnerability Detection. _33rd IEEE/ACM International Conference on Automated Software Engineering (ASE)_ . IEEE/ACM; 2018:259-269. doi:10.1145/3238147.3238177 

125. Fu M, Wu L, Hong Z, Zhu F, Sun H, Feng W. A critical-path-coverage-based vulnerability detection method for smart contracts. _IEEE Access_ . 2019;7:147327-147344. doi:10.1109/ACCESS.2019.2947146 

126. Luu L, Chu DH, Olickel H, Saxena P, Hobor A. Making Smart Contracts Smarter. _CCS’16: Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ . Vol 24-28. ACM; 2016:254-269. doi:10.1145/2976749.2978309 

127. Wang W, Song J, Xu G, Li Y, Wang H, Su C. ContractWard: automated vulnerability detection models for Ethereum smart contracts. _IEEE Trans Netw Sci Eng_ . 2020;4697:1-1144. doi:10.1109/tnse.2020.2968505 

128. Liu C, Liu H, Cao Z, Chen Z, Chen B, Roscoe B. ReGuard: Finding Reentrancy Bugs in Smart Contracts. _IEEE/ACM 40th International Conference on Software Engineering: Companion (ICSE-Companion)_ . IEEE/ACM; 2018:65-68. doi:10.1145/3183440.3183495 

129. Coblenz M. Obsidian: A Safer Blockchain Programming Language. _2017 IEEE/ACM 39th International Conference on Software Engineering Companion (ICSE-C)_ . IEEE/ACM; 2017:97-99. doi:10.1109/ICSE-C.2017.150 

130. Sharma P, Jindal R, Dutta M. A review of smart contract-based platforms, applications, and challenges. _Clust Comput_ . 2021;1:395-421. doi:10.1007/s10586-021-03491-1 

131. Akca S, Rajan A, Peng C. SolAnalyser: A Framework for Analysing and Testing Smart Contracts. _2019 26th Asia-Pacific Software Engineering Conference (APSEC)_ . IEEE; 2019:482-489. doi:10.1109/APSEC48747.2019.00071 

132. Momeni P, Wang Y, Samavi R. Machine Learning Model for Smart Contracts Security Analysis. _2019 17th International Conference on Privacy, Security and Trust (PST)_ . IEEE; 2019:1-6. doi:10.1109/PST47121.2019.8949045 

133. Nawari NO, Ravindran S. Blockchain and the built environment: potentials and limitations. _J Build Eng_ . 2019;25:100832. doi:10.1016/j.jobe.2019.100832 

134. Maull R, Godsiff P, Mulligan C, Brown A, Kewell B. Distributed ledger technology: applications and implications. _Strateg Chang_ . 2017;26(5):481-489. doi:10.1002/jsc.2148 

135. Li Y. Emerging blockchain-based applications and techniques. _Serv Oriented Comput Appl_ . 2019;13(4):279-285. doi:10.1007/s11761-019-00281-x 

136. Zhang K, Jacobsen HA. Towards Dependable, Scalable, and Pervasive Distributed Ledgers with Blockchains. _2018 IEEE 38th International Conference on Distributed Computing Systems (ICDCS)_ . IEEE; 2018:1337-1346. doi:10.1109/ICDCS.2018.00134 

137. Koens T, Poll E. Assessing interoperability solutions for distributed ledgers. _Pervasive Mob Comput_ . 2019;59:101079. doi:10.1016/j.pmcj.2019.101079 

138. Augur – The World’s Most Accessible, No-Limit Betting Platform. https://augur.net/ 

139. Storj – Decentralized Cloud Storage. https://www.storj.io/ 

140. Uniswap | Home. https://uniswap.org/ 

141. Secure, Fast & Private Web Browser with Adblocker | Brave Browser. https://brave.com/ 

142. Walmart Case Study – Hyperledger. https://www.hyperledger.org/learn/publications/walmart-case-study 

143. Home – Xage Security. https://xage.com/ 

144. Blockgraph: The Identity Operating System for Convergent TV. https://www.blockgraph.co/ 

145. Pervez H, Muneeb M, Irfan MU, Ul Haq I. A Comparative Analysis of DAG-Based Blockchain Architectures. _2018 12th International Conference on Open Source Systems and Technologies (ICOSST)_ . IEEE; 2019:27-34. doi:10.1109/ICOSST.2018.8632193 

146. Dong Z, Zheng E, Choon Y, Zomaya AY. DAGBENCH: A Performance Evaluation Framework for DAG Distributed Ledgers. _IEEE 12th International Conference on Cloud Computing (CLOUD)_ . IEEE; 2019:264-271. doi:10.1109/CLOUD.2019.00053 

147. Saad A, Park SY. Decentralized Directed Acyclic Graph based DLT Network. _COINS’19: Proceedings of the International Conference on Omni-Layer Intelligent Systems_ . ACM; 2019:158-163. doi:10.1145/3312614.3312647 

148. Dinh TTA, Liu R, Zhang M, Chen G, Ooi BC, Wang J. Untangling Blockchain: a data processing view of Blockchain systems. _IEEE Trans Knowl Data Eng_ . 2018;30(7):1366-1385. doi:10.1109/TKDE.2017.2781227 

149. Dinh TTA, Wang J, Chen G, Liu R, Ooi BC, Tan K-L. Blockbench. _Proceedings of the 2017 ACM International Conference on Management of Data_ . ACM; 2017:1085-1100. doi:10.1145/3035918.3064033 

150. Ciatto G, Bosello M, Mariani S, Omicini A. _Comparative Analysis of Blockchain Technologies under a Coordination Perspective_ . Vol 1047. Springer International Publishing; 2019. 

MONIKA et al. 

**46 of 50** 

151. Aldweesh A, Alharby M, Mehrnezhad M, Van Moorsel A. OpBench: A CPU Performance Benchmark for Ethereum Smart Contract Operation Code. _2019 IEEE International Conference on Blockchain (Blockchain)_ . Vol 2019. IEEE; 2019:274-281. doi:10.1109/Blockchain.2019.00043 

152. Qiu T, Zhang R, Gao Y. Ripple vs. SWIFT: transforming cross border remittance using Blockchain technology. _Procedia Comput Sci_ . 2019;147:428-434. doi:10.1016/j.procs.2019.01.260 

153. Wang L, Shen X, Li J, Shao J, Yang Y. Cryptographic primitives in blockchains. _J Netw Comput Appl_ . 2019;127:43-58. doi:10.1016/j.jnca.2018.11.003 

154. Storublevtcev N. Cryptography in Blockchain. _Lecture Notes in Computer Science (Including Subseries Lecture Notes in Artificial Intelligence and Lecture Notes in Bioinformatics)_ . Springer International Publishing; 2019:495-508. 

155. Moreno-Sanchez P, Modi N, Songhela R, Kate A, Fahmy S. Mind Your Credit: Assessing the Health of the Ripple Credit Network. _International World Wide Web Conference Committee_ . ACM; 2018:329-338. doi:10.1145/3178876.3186099 

156. Lokhava M, Losa G, Mazières D, et al. Fast and Secure Global Payments with Stellar. _SOSP’19: Symposium on Operating Systems Principles_ . ACM; 2019:80-96. doi:10.1145/3341301.3359636 

157. Amoordon A, Rocha H. Presenting Tendermint: Idiosyncrasies, Weaknesses, and Good Practices. _2019 IEEE International Workshop on Blockchain Oriented Software Engineering (IWBOSE)_ . IEEE; 2019:44-49. doi:10.1109/IWBOSE.2019.8666541 

158. Buterin V, Reijsbergen D, Leonardos S, Piliouras G. Incentives in Ethereum’s hybrid Casper protocol. _Int J Netw Manag_ . 2020;30(5):1-25. doi:10.1002/nem.2098 

159. Hrga A, Bencic FM, Zarko IP. Technical Analysis of an Initial Coin Offering. _2019 15th International Conference on Telecommunications (ConTEL)_ . IEEE; 2019:1-8. doi:10.1109/ConTEL.2019.8848532 

160. Rehman MHU, Salah K, Damiani E, Svetinovic D. Trust in Blockchain Cryptocurrency Ecosystem. _IEEE Trans Eng Manag_ . 2020;67(4):1196-1212. doi:10.1109/TEM.2019.2948861 

161. Fan C, Ghaemi S, Khazaei H, Musilek P. Performance evaluation of Blockchain systems: a systematic survey. _IEEE Access_ . 2020;8:126927-126950. doi:10.1109/ACCESS.2020.3006078 

162. Zhou Z, Li R, Cao Y, Zheng L, Xiao H. Dynamic performance evaluation of Blockchain technologies. _IEEE Access_ . 2020;8:217762-217772. doi:10.1109/ACCESS.2020.3040875 

163. Farshidi S, Jansen S, Espana S, Verkleij J. Decision support for Blockchain platform selection: three industry case studies. _IEEE Trans Eng Manag_ . 2020;67(4):1109-1128. doi:10.1109/TEM.2019.2956897 

164. Akcora CG, Gel YR, Kantarcioglu M. Blockchain networks: Data structures of Bitcoin, Monero, Zcash, Ethereum, Ripple, and Iota. _WIREs Data Mining and Knowledge Discovery_ . Wiley; 2022:1-35. doi:10.1002/widm.1436 

165. Wu K, Ma Y, Huang G, Liu X. A first look at blockchain-based decentralized applications. _Softw Pract Exp_ . 2021;51(10):2033-2050. doi:10.1002/spe.2751 

166. Tam Vo H, Wang Z, Karunamoorthy D, Wagner J, Abebe E, Mohania M. Internet of blockchains: Techniques and Challenges Ahead. _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ . IEEE; 2018:1574-1581. doi:10.1109/Cybermatics_2018.2018.00264 

167. Hardjono T, Lipton A, Pentland A. Toward an interoperability architecture for Blockchain autonomous systems. _IEEE Trans Eng Manag_ . 2019;4:1-12. doi:10.1109/tem.2019.2920154 

168. Qasse IA, Abu Talib M, Nasir Q. Inter Blockchain Communication. _Proceedings of the ArabWIC 6th Annual International Conference Research Track_ . ACM; 2019:1-6. doi:10.1145/3333165.3333167 

169. Robinson P. Survey of crosschain communications protocols. _Comput Netw_ . 2021;200:108488. doi:10.1016/j.comnet.2021.108488 

170. Herlihy M. Atomic Cross-Chain Swaps. _Proceedings of the 2018 ACM Symposium on Principles of Distributed Computing_ . ACM; 2018:245-254. doi:10.1145/3212734.3212736 

171. Li W, Sforzin A, Fedorov S, Karame GO. Towards scalable and private industrial blockchains. _BCC 2017 – Proceedings of the ACM Workshop on Blockchain, Cryptocurrencies and Contracts, co-Located with ASIA CCS_ . ACM; 2017:9-14. doi:10.1145/3055518.3055531 

172. Dagher GG, Adhikari CL, Enderson T. Towards Secure Interoperability between Heterogeneous Blockchains using Smart Contracts. _Future Technologies Conference (FTC) 2017_ . SAI Conferences; 2017:73-81. 

173. Borkowski M, Sigwart M, Frauenthaler P, Hukkinen T, Schulte S. Dextt: deterministic cross-Blockchain token transfers. _IEEE Access_ . 2019;7:111030-111042. doi:10.1109/access.2019.2934707 

174. Monika R, Bhatia AJ, Singh B. Hash time locked contract based asset exchange solution for probabilistic public blockchains. _Clust Comput_ . 2022;25(6):4189-4201. doi:10.1007/s10586-022-03643-x 

175. Westerkamp M. Verifiable Smart Contract Portability. _2019 IEEE International Conference on Blockchain and Cryptocurrency (ICBC)_ . IEEE; 2019:413-421. doi:10.1109/BLOC.2019.8751335 

176. Han R, Lin H, Yu J. On the Optionality and Fairness of Atomic Swaps. _AFT’19: Proceedings of the 1st ACM Conference on Advances in Financial Technologies_ . ACM; 2019:62-75. doi:10.1145/3318041.3355460 

177. Yang S, Wang H, Li W, Liu W, Fu X. CVEM: A Cross-Chain Value Exchange Mechanism. _Proceedings of the 2018 International Conference on Cloud Computing and Internet of Things_ . ACM; 2018:80-85. doi:10.1145/3291064.3291073 

178. Fan K, Ren Y, Yan Z. Niji: Autonomous Payment Bridge between Bitcoin and Consortium Blockchain. _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ . IEEE; 2018:1349-1354. doi:10.1109/Cybermatics 

MONIKA et al. 

**47 of 50** 

179. Zamyatin A, Harz D, Lind J, Panayiotou P, Gervais A, Knottenbelt W. XCLAIM: Trustless, Interoperable, Cryptocurrency-Backed Assets. _Proceedings – IEEE Symposium on Security and Privacy_ . IEEE; 2019:193-210. doi:10.1109/SP.2019.00085 

180. Singh A, Click K, Parizi RM, Zhang Q, Dehghantanha A, Choo KKR. Sidechain technologies in blockchain networks: an examination and state-of-the-art review. _J Netw Comput Appl_ . 2020;149:102471. doi:10.1016/j.jnca.2019.102471 

181. Monika, Bhatia R. Interoperability Solutions for Blockchain. _2020 International Conference on Smart Technologies in Computing, Electrical and Electronics (ICSTCEE)_ . IEEE; 2020:381-385. doi:10.1109/ICSTCEE49637.2020.9277054 

182. Abebe E, Behl D, Govindarajan C, et al. Enabling Enterprise Blockchain Interoperability with Trusted Data Transfer (Industry Track). _Proceedings of the 20th International Middleware Conference Industrial Track_ . IEEE; 2019:29-35. doi:10.1145/3366626.3368129 

183. Wang H, Cen Y, Li X. Blockchain Router: A Cross-Chain Communication Protocol. _ACM International Conference Proceeding Series_ . ACM; 2017:94-97. doi:10.1145/3070617.3070634 

184. Jin H, Dai X, Xiao J. Towards a Novel Architecture for Enabling Interoperability Amongst Multiple Blockchains. _2018 IEEE 38th International Conference on Distributed Computing Systems (ICDCS)_ . IEEE; 2018:1203-1211. doi:10.1109/ICDCS.2018.00120 

185. Liu Z, Xiang Y, Shi J, et al. Hyperservice: Interoperability and Programmability Across Heterogeneous Blockchains. _CCS ’19: Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security_ . ACM; 2019;549-566. doi:10.1145/3319535.3355503 

186. Al-Breiki H, Rehman MHU, Salah K, Svetinovic D. Trustworthy Blockchain oracles: review, comparison, and open research challenges. _IEEE Access_ . 2020;8:85675-85685. doi:10.1109/ACCESS.2020.2992698 

187. Xiao Y, Zhang N, Li J, Lou W, Hou YT. Distributed Consensus Protocols and Algorithms. _Blockchain for Distributed Systems Security_ . Wiley; 2019:25-50. 

188. Raikwar M, Gligoroski D, Kralevska K. SoK of used cryptography in Blockchain. _IEEE Access_ . 2019;7:148550-148575. doi:10.1109/ACCESS.2019.2946983 

189. Liu Z, Luong NC, Wang W, et al. A survey on Blockchain: a game theoretical perspective. _IEEE Access_ . 2019;7:47615-47643. doi:10.1109/ACCESS.2019.2909924 

190. Zaghloul E, Li T, Mutka MW, Ren J. Bitcoin and Blockchain: security and privacy. _IEEE Internet Things J_ . 2020;7(10):10288-10313. doi:10.1109/JIOT.2020.3004273 

191. di Angelo M, Salzer G. A Survey of Tools for Analyzing Ethereum Smart Contracts. _2019 IEEE International Conference on Decentralized Applications and Infrastructures (DAPPCON)_ . Vol 2019. IEEE; 2019:69-78. doi:10.1109/DAPPCON.2019.00018 

192. Liu J, Liu Z. A survey on security verification of Blockchain smart contracts. _IEEE Access_ . 2019;7:77894-77904. doi:10.1109/ACCESS.2019.2921624 

193. Destefanis G, Marchesi M, Ortu M, Tonelli R, Bracciali A, Hierons R. Smart Contracts Vulnerabilities: A Call for Blockchain Software Engineering? _2018 International Workshop on Blockchain Oriented Software Engineering (IWBOSE)_ . IEEE; 2018:19-25. doi:10.1109/IWBOSE.2018.8327567 

194. Rouhani S, Deters R. Security, performance, and applications of smart contracts: a systematic survey. _IEEE Access_ . 2019;7:1-50779. doi:10.1109/ACCESS.2019.2911031 

195. Parizi RM, Dehghantanha A, Choo KKR, Singh A. Empirical Vulnerability Analysis of Automated Smart Contracts Security Testing on Blockchains. _CASCON’18: Proceedings of the 28th Annual International Conference on Computer Science and Software Engineering_ . ACM; 2018:103-113. 

196. Wohrer M, Zdun U. Smart contracts: Security Patterns in the Ethereum Ecosystem and Solidity. _2018 International Workshop on Blockchain Oriented Software Engineering (IWBOSE)_ . IEEE; 2018:2-8. doi:10.1109/IWBOSE.2018.8327565 

197. Wang Z, Jin H, Dai W, Choo KKR, Zou D. Ethereum smart contract security research: survey and future research opportunities. _Front Comput Sci_ . 2021;15(2):152802. doi:10.1007/s11704-020-9284-9 

198. Hewa TM, Hu Y, Liyanage M, Kanhare SS, Ylianttila M. Survey on Blockchain-based smart contracts: technical aspects and future research. _IEEE Access_ . 2021;9:87643-87662. doi:10.1109/ACCESS.2021.3068178 

199. Dwivedi V, Pattanaik V, Deval V, Dixit A, Norta A, Draheim D. Legally enforceable smart-contract languages. _ACM Comput Surv_ . 2021;54(5):1-34. doi:10.1145/3453475 

200. Dixit A, Deval V, Dwivedi V, Norta A, Draheim D. Towards user-centered and legally relevant smart-contract development: a systematic literature review. _J Ind Inf Integr_ . 2020;26:2022. doi:10.1016/j.jii.2021.100314 

201. Ante L. Smart contracts on the blockchain – a bibliometric analysis and review. _Telemat Inform_ . 2020;57:2021. doi:10.1016/j.tele.2020.101519 

202. Hewa T, Ylianttila M, Liyanage M. Survey on blockchain based smart contracts: applications, opportunities and challenges. _J Netw Comput Appl_ . 2020;177:2021. doi:10.1016/j.jnca.2020.102857 

203. Grossman S, Abraham I, Golan-Gueta G, et al. Online detection of effectively callback free objects with applications to smart contracts. _Proc ACM Program Lang_ . 2018;2:1-28. doi:10.1145/3158136 

204. Rodler M, Li W, Karame GO, Davi L. _Sereum: Protecting Existing Smart Contracts against Re-Entrancy Attacks_ . University of Duisburg-Essen; 2019. doi:10.14722/ndss.2019.23413 

205. Grech N, Kong M, Jurisevic A, Brent L, Scholz B, Smaragdakis Y. MadMax: surviving out-of-gas conditions in Ethereum smart contracts. _Proc ACM Program Lang_ . 2018;2:1-27. doi:10.1145/3276486 

206. Bragagnolo S, Rocha H, Denker M, Ducasse S. SmartInspect: Solidity Smart Contract Inspector. _2018 International Workshop on Blockchain Oriented Software Engineering (IWBOSE)_ . IEEE; 2018:9-18. doi:10.1109/IWBOSE.2018.8327566 

207. Hegeds P. Towards Analyzing the Complexity Landscape of Solidity Based Ethereum Smart Contracts. _2018 IEEE/ACM 1st International Workshop on Emerging Trends in Software Engineering for Blockchain (WETSEB)_ . IEEE/ACM; 2018:35-39. doi:10.1145/3194113.3194119 

MONIKA et al. 

**48 of 50** 

208. Liu H, Liu C, Zhao W, Jiang Y, Sun J. S-gram: Towards Semantic-Aware Security Auditing for Ethereum Smart Contracts. _2018 33rd IEEE/ACM International Conference on Automated Software Engineering (ASE)_ . IEEE/ACM; 2018:814-819. doi:10.1145/3238147.3240728 

- _́_ 

- 209. Nikolic I, Kolluri A, Sergey I, Saxena P, Hobor A. Finding the Greedy, Prodigal, and Suicidal Contracts at scale. _Cryptography and Security_ . ACM; 2018:653-663. doi:10.1145/3274694.3274743 

210. Torres CF, Schütte J, State R. Osiris: Hunting for Integer Bugs in Ethereum Smart Contracts. _34th Annual Computer Security Applications Conference (ACSAC)_ . ACM; 2018:664-676. doi:10.1145/3274694.3274737 

211. Albert E, Gordillo P, Livshits B, Rubio A, Sergey I. EthIR: A Framework for High-Level Analysis of Ethereum Bytecode. _Automated Technology for Verification and Analysis_ . Springer; 2018:513-520. doi:10.1007/978-3-030-01090-4_30/COVER 

212. Tsankov P, Dan A, Drachsler-Cohen D, Gervais A, Bünzli F, Vechev M. Securify: Practical Security Analysis of Smart Contracts. _Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security_ ; 2018:67-82. doi:10.1145/3243734.3243780 

213. Zhou E, Hua S, Pi B, et al. Security Assurance for Smart Contract. _2018 9th IFIP International Conference on New Technologies, Mobility and Security, NTMS 2018_ . IEEE; 2018:1-5. doi:10.1109/NTMS.2018.8328743 

214. Albert E, Gordillo P, Rubio A, Sergey I. Running on Fumes: Preventing Out-of-Gas Vulnerabilities in Ethereum Smart Contracts Using Static Resource Analysis. _International Conferenceon Verification and Evaluation of Computer and Communication Systems_ . Springer; 2019:63-78. doi:10.1007/978-3-030-35092-5_5 

215. Gao J, Liu H, Liu C, Li Q, Guan Z, Chen Z. EASYFLOW: Keep Ethereum Away from Overflow. _IEEE/ACM 41st International Conference on Software Engineering: Companion Proceedings (ICSE-Companion)_ . IEEE/ACM; 2019:23-26. doi:10.1109/ICSE-Companion.2019.00029 

216. Kolluri A, Nikolic I, Sergey I, Hobor A, Saxena P. Exploiting the Laws of Order in Smart Contracts. _Proceedings of the 28th ACM SIGSOFT International Symposium on Software Testing and Analysis_ . ACM; 2019:363-373. doi:10.1145/3293882.3330560 

217. Li Z, Wu H, Xu J, Wang X, Zhang L, Chen Z. MuSC: A Tool for Mutation Testing of Ethereum Smart Contract. _2019 34th IEEE/ACM International Conference on Automated Software Engineering (ASE)_ . IEEE/ACM; 2019:1198-1201. doi:10.1109/ASE.2019.00136 

218. Gao Z, Jayasundara V, Jiang L, Xia X, Lo D, Grundy J. SmartEmbed: A Tool for Clone and Bug Detection in Smart Contracts through Structural Code Embedding. _2019 IEEE International Conference on Software Maintenance and Evolution (ICSME)_ . IEEE; 2019:394-397. doi:10.1109/ICSME.2019.00067 

219. Grech N, Brent L, Scholz B, Smaragdakis Y. Gigahorse: Thorough, Declarative Decompilation of Smart Contracts. _2019 IEEE/ACM 41st International Conference on Software Engineering (ICSE)_ . IEEE/ACM; 2019:1176-1186. doi:10.1109/ICSE.2019.00120 

220. Cai Z, Qu J, Liu P, Yu J. A blockchain smart contract based on light-weighted quantum blind signature. _IEEE Access_ . 2019;7(2):138657-138668. doi:10.1109/ACCESS.2019.2941153 

221. Wang X, Xie Z, He J, Zhao G, Nie R. Basis Path Coverage Criteria for Smart Contract Application Testing. _2019 International Conference on Cyber-Enabled Distributed Computing and Knowledge Discovery (CyberC)_ . IEEE; 2019:34-41. doi:10.1109/CyberC.2019.00016 

222. Gao J, Liu H, Li Y, et al. Towards Automated Testing of Blockchain-Based Decentralized Applications. _IEEE/ACM 27th International Conference on Program Comprehension (ICPC)_ . IEEE/ACM; 2019:294-299. doi:10.1109/ICPC.2019.00048 

223. Wang X, Wu H, Sun W, Zhao Y. Towards Generating Cost-Effective Test-Suite for Ethereum Smart Contract. _IEEE 26th International Conference on Software Analysis, Evolution and Reengineering_ . IEEE; 2019:549-553. doi:10.1109/SANER.2019.8668020 

224. Vandenbogaerde B. A Graph-Based Framework for Analysing the Design of Smart Contracts. _ESEC/FSE 2019 Proceedings of the 2019 27th ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software Engineering_ . ACM; 2019:1220-1222. doi:10.1145/3338906.3342495 

225. Mossberg M, Manzano F, Hennenfent E, et al. Manticore: A User-Friendly Symbolic Execution Framework for Binaries and Smart Contracts. _2019 34th IEEE/ACM International Conference on Automated Software Engineering (ASE)_ . IEEE/ACM; 2019:1186-1189. doi:10.1109/ASE.2019.00133 

226. Chang J, Gao B, Xiao H, Sun J, Cai Y, Yang Z. sCompile: Critical Path Identification and Analysis for Smart Contracts. _IEEE International Conference on Formal Engineering Methods_ . IEEE; 2019:286-304. doi:10.1007/978-3-030-32409-4_18/COVER 

227. He J, Balunovic _́_ M, Ambroladze N, Tsankov P, Vechev M. Learning to Fuzz from Symbolic Execution with Application to Smart Contracts. _Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security_ . ACM; 2019:531-548. doi:10.1145/3319535.3363230 

228. Feist J, Grieco G, Groce A. Slither: A Static Analysis Framework for Smart Contracts. _2019 IEEE/ACM 2nd International Workshop on Emerging Trends in Software Engineering for Blockchain (WETSEB)_ . IEEE/ACM; 2019:8-15. doi:10.1109/WETSEB.2019.00008 

229. Albert E, Correas J, Gordillo P, Román-Díez G, Rubio A. GASOL: Gas Analysis and Optimization for Ethereum Smart Contracts. _Tools and Algorithms for the Construction and Analysis of Systems_ . Springer; 2020:118-125. doi:10.1007/978-3-030-45237-7_7/COVER 

230. Albert E, Gordillo P, Rubio A, Schett MA. Synthesis of Super-Optimized Smart Contracts Using Max-SMT. _International Conference on Computer Aided Verification_ . ACM; 2020:177-200. doi:10.1007/978-3-030-53288-8_10/FIGURES/6 

231. So S, Lee M, Park J, Lee H, Oh H. VERISMART: A Highly Precise Safety Verifier for Ethereum Smart Contracts. _2020 IEEE Symposium on Security and Privacy_ . IEEE; 2020:1678-1694. doi:10.1109/SP40000.2020.00032 

232. Permenev A, Dimitrov D, Tsankov P, Drachsler-Cohen D, Vechev M. VerX: Safety Verification of Smart Contracts. _IEEE Symposium on Security and Privacy_ . IEEE; 2020:1661-1677. doi:10.1109/SP40000.2020.00024 

233. Schneidewind C, Grishchenko I, Scherer M, Maffei M. EThor: Practical and Provably Sound Static Analysis of Ethereum Smart Contracts. _CCS’20: Proceedings of the 2020 ACM SIGSAC Conference on Computer and Communications Security_ . ACM; 2020:621-640. doi:10.1145/3372297.3417250 

234. Grieco G, Song W, Cygan A, Feist J, Groce A. Echidna: effective, usable, and fast fuzzing for smart contracts. _ISSTA 2020: Proceedings of the 29th ACM SIGSOFT International Symposium on Software Testing and Analysis_ . ACM; 2020:557-560. doi:10.1145/3395363.3404366 

MONIKA et al. 

**49 of 50** 

235. Biryukov A, Khovratovich D, Tikhomirov S. Findel: Secure Derivative Contracts for Ethereum. _Financial Cryptography and Data Security_ . Springer; 2017:453-467. doi:10.1007/978-3-319-70278-0_28/COVER 

236. O’Connor R, Blockstream C. Simplicity: A New Language for Blockchains. _PLAS 2017 – Proceedings of the 2017 Workshop on Programming Languages and Analysis for Security_ . ACM; 2017:107-120. doi:10.1145/3139337.3139340 

237. Schrans F, Eisenbach S, Drossopoulou S. Writing Safe Smart Contracts in Flint. _Programming’18: Companion Proceedings of the 2nd International Conference on the Art, Science, and Engineering of Programming_ . ACM; 2018:218-219. doi:10.1145/3191697.3213790 

238. He X, Qin B, Zhu Y, Chen X, Liu Y. SPESC: A Specification Language for Smart Contracts. _2018 IEEE 42nd Annual Computer Software and Applications Conference (COMPSAC)_ . IEEE; 2018:132-137. doi:10.1109/COMPSAC.2018.00025 

239. Bartoletti M, Zunino R. BitML: A Calculus for Bitcoin Smart Contracts. _CCS’18: Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security_ . ACM; 2018;83-100. doi:10.1145/3243734.3243795 

240. Atzei N, Bartoletti M, Lande S, Yoshida N, Zunino R. Developing Secure Bitcoin Contracts with BitML. _ESEC/FSE 2019– Proceedings of the 2019 27th ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software Engineering_ . ACM; 2019:1124-1128. doi:10.1145/3338906.3341173 

241. Yang Z, Lei H. Lolisa: formal syntax and semantics for a subset of the solidity programming language in mathematical tool coq. _Math Probl Eng_ . 2020;2020:1-15. doi:10.1155/2020/6191537 

242. Sergey I, Nagaraj V, Johannsen J, Kumar A, Trunov A, Hao KCG. Safer smart contract programming with Scilla. _Proc ACM Program Lang_ . 2019;3:1-30. doi:10.1145/3360611 

243. Crafa S, Di Pirro M, Zucca E. Is Solidity Solid Enough? _International Conference on Financial Cryptography and Data Security_ . Springer; 2020:138-153. doi:10.1007/978-3-030-43725-1_11/COVER 

244. Zhang P, Xiao F, Luo X. A Framework and DataSet for Bugs in Ethereum Smart Contracts. _2020 IEEE International Conference on Software Maintenance and Evolution (ICSME)_ . IEEE; 2020:139-150. doi:10.1109/ICSME46990.2020.00023 

245. Ren M, Yin Z, Ma F, et al. Empirical Evaluation of Smart Contract Testing: What is the Best Choice? _Proceedings of the 30th ACM SIGSOFT International Symposium on Software Testing and Analysis, ISSTA’21_ . ACM; 2021:566-579. doi:10.1145/3460319.3464837 

246. Durieux T, Ferreira JF, Abreu R, et al. Empirical Review of Automated Analysis Tools on 47,587 Ethereum Smart Contracts. _Proceedings of the AC M/IEEE 42nd International Conference on Software Engineering_ . ACM/IEEE; 2020:530-541. 

247. Kushwaha SS, Joshi S, Singh D, Kaur M, Lee HN. Ethereum smart contract analysis tools: a systematic review. _IEEE Access_ . 2022;10:57037-57062. doi:10.1109/ACCESS.2022.3169902 

248. Murray Y, Anisi DA. Survey of Formal Verification Methods for Smart Contracts on Blockchain. _10th IFIP International Conference on New Technologies, Mobility and Security (NTMS)_ . IEEE; 2019:1-6. doi:10.1109/NTMS.2019.8763832 

249. Almakhour M, Sliman L, Samhat AE, Mellouk A. Verification of smart contracts: a survey. _Pervasive Mob Comput_ . 2020;67:101227. doi:10.1016/j.pmcj.2020.101227 

250. Bhargavan K, Delignat-Lavaud A, Fournet C, et al. Formal Verification of Smart Contracts. _PLAS’16: Proceedings of the 2016 ACM Workshop on Programming Languages and Analysis for Security_ . ACM; 2016:91-96. doi:10.1145/2993600.2993611 

251. Zhang F, Cecchetti E, Croman K, Juels A, Shi E. Town crier: An Authenticated Data Feed for Smart Contracts. _CCS’16: Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ . Vol 24-28. ACM; 2016:270-282. doi:10.1145/2976749.2978326 

252. Amani S, Bortin M, Bégel M, Staples M. Towards Verifying Ethereum Smart Contract Bytecode in Isabelle/HOL. _CPP 2018: Proceedings of the 7th ACM SIGPLAN International Conference on Certified Programs and Proofs_ . ACM; 2018:66-77. doi:10.1145/3167084 

253. Abdellatif T, Brousmiche K-L. Formal Verification of Smart Contracts Based on Users and Blockchain Behaviors Models. _2018 9th IFIP International Conference on New Technologies, Mobility and Security (NTMS)_ . Vol 2018. IEEE; 2018:1-5. doi:10.1109/NTMS.2018.8328737 

254. Bai X, Cheng Z, Duan Z, Hu K. Formal Modeling and Verification of Smart Contracts. _Proceedings of the 2018 7th International Conference on Software and Computer Applications_ . ACM; 2018:322-326. doi:10.1145/3185089.3185138 

255. Nehai Z, Piriou PY, Daumas F. Model-Checking of Smart Contracts. _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ . IEEE; 2018:980-987. doi:10.1109/Cybermatics_2018.2018.00185 

256. Liu Z, Liu J. Formal verification of blockchain smart contract based on colored petri net models. _Proc Int Comput Softw Appl Conf_ . 2019;2:555-560. doi:10.1109/COMPSAC.2019.10265 

257. Osterland T, Rose T. Model checking smart contracts for Ethereum. _Pervasive Mob Comput_ . 2019;63(2):69-76. doi:10.1109/ASEW.2019.00032 

258. Yang Z, Lei H. FEther: an extensible definitional interpreter for smart-contract verifications in coq. _IEEE Access_ . 2019;7:37770-37791. doi:10.1109/ACCESS.2019.2905428 

259. Albert E, Correas J, Gordillo P, Román-Díez G, Rubio A. SAFEVM: A Safety Verifier for Ethereum Smart Contracts. _Proceedings of the 28th ACM SIGSOFT International Symposium on Software Testing and Analysis_ . ACM; 2019:386-389. doi:10.1145/3293882.3338999 

260. Li X, Su C, Xiong Y, Huang W, Wang W. Formal Verification of BNB Smart Contract. _5th International Conference on Big Data Computing and Communications (BIGCOM)_ . IEEE; 2019:74-78. doi:10.1109/BIGCOM.2019.00021 

261. Kongmanee J, Kijsanayothin P, Hewett R. Securing Smart Contracts in Blockchain. _34th IEEE/ACM International Conference on Automated Software Engineering Workshop (ASEW)_ . IEEE/ACM; 2019:69-76. doi:10.1109/ASEW.2019.00032 

262. Annenkov D, Botsch Nielsen J, Spitters B. ConCert: A Smart Contract Certification Framework in Coq. _CPP 2020: Proceedings of the 9th ACM SIGPLAN International Conference on Certified Programs and Proofs_ . IEEE/ACM; 2020:215-228. doi:10.1145/3372885.3373829 

263. Firdaus A, Razak MFA, Feizollah A, Hashem IAT, Hazim M, Anuar NB. _The Rise of “Blockchain”: Bibliometric Analysis of Blockchain Study_ . Springer International Publishing; 2019:1289-1331. 

MONIKA et al. 

**50 of 50** 

264. Zhou L, Zhang L, Zhao Y, Zheng R, Song K. A scientometric review of blockchain research. _Inf Syst E-Bus Manag_ . 2021;19(3):757-787. doi:10.1007/s10257-020-00461-9 

**How to cite this article:** Monika, Bhatia R, Kumar M. An extensive multivocal literature review of blockchain technology: Evolution, challenges, platforms, security, and interoperability. _Trans Emerging Tel Tech_ . 2024;35(11):e5037. doi: 10.1002/ett.5037 

