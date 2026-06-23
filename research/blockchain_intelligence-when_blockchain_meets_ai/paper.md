1 

## Blockchain Intelligence: When Blockchain Meets Artificial Intelligence 

Zibin Zheng, _Senior Member, IEEE_ , Hong-Ning Dai, _Senior Member, IEEE_ , Jiajing Wu, _Member, IEEE_ 

_**Abstract**_ **—Blockchain is gaining extensive attention due to its provision of secure and decentralized resource sharing manner. However, the incumbent blockchain systems also suffer from a number of challenges in operational maintenance, quality assurance of smart contracts and malicious behaviour detection of blockchain data. The recent advances in artificial intelligence bring the opportunities in overcoming the above challenges. The integration of blockchain with artificial intelligence can be beneficial to enhance current blockchain systems. This article presents an introduction of the convergence of blockchain and artificial intelligence (namely blockchain intelligence). This paper also gives a case study to further demonstrate the feasibility of blockchain intelligence and point out the future directions.** 

_**Keywords**_ **—** _**Blockchain; Artificial Intelligence; Smart Contract; Machine Learning**_ 

I. INTRODUCTION 

Blockchain has received extensive attention recently due to its provision of secure data sharing services with traceability, immutability and non-repudiation. Despite the merits of blockchain, the development of blockchain technologies has undergone a number of challenges including poor scalability, difficulties in operational maintenance, detecting vulnerable codes in smart contracts and identifying malicious behaviours in blockchain historical data. 

The recent advances in artificial intelligence (AI) have greatly propelled the evolution of diverse business applications. The integration of AI with blockchain has the potentials to overcome the limitations of blockchain. We name the intelligent capability bestowed by AI on the blockchain ecosystem as _blockchain intelligence_ . In particular, AI approaches such as machine learning, data mining and data visualization may help to capture the abnormal behaviours in blockchain, identify risks in transactions, detect possible vulnerable program codes in smart contracts, and so on. Consequently, proactive and autonomic actions can be made to prevent blockchain from disruptive or illegal actions, and thus make the blockchain more intelligent. 

This article aims at reviewing blockchain technologies as well as AI technologies, presenting an in-depth analysis on integrating blockchain with AI, providing implications of enabling technologies of blockchain intelligence. In contrast to most of recent studies on the integration of blockchain and AI (which mainly focuses on applying blockchain to 

Z. Zheng, J. Wu are with School of Data and Computer Science, Sun Yat-sen University, China (email: zhzibin@mail.sysu.edu.cn, wujiajing@mail.sysu.edu.cn). 

H.-N. Dai is with Faculty of Information Technology, Macau University of Science and Technology, Macau (email: hndai@ieee.org). 

overcome shortcomings of AI, such as security and privacy vulnerabilities [14]), this article mainly concentrates on improving blockchain systems by applying AI technologies. In summary, the main contributions of this article are summarized as follows: 

- We first give an overview of blockchain technology and point out the challenges in existing blockchain systems. 

- _•_ We then review the advances in AI and formally introduce the convergence of AI and blockchain followed by a discussion on opportunities brought by blockchain intelligence. 

- We next present a case study to demonstrate the feasibility of blockchain intelligence. 

II. OVERVIEW OF BLOCKCHAIN TECHNOLOGIES 

As a disruptive software technology, blockchain is reshaping diverse business sectors. Blockchain is essentially a chainlike data structure storing transactions verified by majority of nodes throughout the whole network as shown in Fig. 1. Since the committed transactions in the blockchain have been stored at every node, they are extremely difficult to be altered or falsified. Integrating with digital signature and asymmetric encryption, blockchain data is authenticated and auditable, implying non-repudiation of the transaction initiator. The length of blockchain keeps growing with the new validated transaction being appended at end of the chain. Data analytics on blockchain data can potentially extract valuable information. 

The development of blockchain technologies has experienced two phases: 1) blockchain 1.0 ( _i.e._ , symbolized by digital currency) and 2) blockchain 2.0 ( _i.e._ , symbolized by smart contracts) [1]. In blockchain 1.0, blockchain has been mainly used for digital currencies like Bitcoin. The appearance of blockchain has promoted the development of smart contracts. A smart contract essentially consists of a number of computerized contractual agreements consented by multiple parties [2]. The contractual clauses embedded in smart contracts will be triggered and automatically executed when a certain condition is satisfied ( _e.g._ , who breaches the contract will be automatically imposed with a fine). 

Smart contracts have been implemented on top of blockchain as shown in Fig. 1. The approved contractual clauses are converted into executable computer programs. The logical connections between contractual clauses have also been preserved in the form of logical flows in programs ( _e.g._ , if-else-if statement). The execution of each contract statement is recorded as an immutable transaction stored in the blockchain. Meanwhile, smart contracts guarantee appropriate access control and contract enforcement. In particular, developers can assign access permissions for each function in the contract. 

2 

**==> picture [454 x 95] intentionally omitted <==**

**----- Start of picture text -----**<br>
Genesis block Block  i Block  i + 1<br>Transaction Contract Transaction Contract Transaction Contract<br>roothash roothash roothash roothash roothash roothash<br>Smart contract Smart contract Smart contract<br>TX 1 TX 2 TX n Smart contract TX 1 TX 2 TX n Smart contract TX 1 TX 2 TX n Smart contract<br>Smart contract Smart contract Smart contract<br>**----- End of picture text -----**<br>


Fig. 1. Overview of blockchain 

Although blockchain and blockchain-enabled smart contracts are promising in reshaping various industrial sectors, the intrinsic limitations of blockchain systems also lead to the following challenges. 

_1) Operational maintenance_ . Due to the decentralization and heterogeneity of blockchain systems, it is difficult to identify the potential factors affecting the performance of blockchain. For example, the _transaction throughput_ bottleneck of Hyperledger Fabric is different from that of Bitcoin and Ethereum since different consensus algorithms are adopted. Moreover, like other software systems, smart contracts consist of a number of computer programs, which may suffer from software bugs, malicious codes and incompatibility of running environments. Consequently, it is crucial to achieve the intelligent and robust operational maintenance of complex blockchain systems. 

_2) Quality assurance of smart contracts_ . Smart contracts suffer from a number of software vulnerabilities such as _reentrancy_ vulnerability [1], overcharging issue [3], randomness controlling [4] and Decentralized Autonomous Organization (DAO) attack [5]. In addition, contract correctness is also crucial to smart contracts since it is nearly impossible to make any revisions once they are deployed on top of blockchains. However, like software systems, smart contracts often contain programming bugs which may lead to crashes or misbehaviours while it is challenging to detect and identify these bugs due to the complexity of smart contracts. 

_3) Malicious behaviour detection_ . Besides legal businesses, blockchain may be exploited for malicious activities which are nevertheless difficult to be detected due to the _pseudonymity_ of blockchain ( _i.e._ , anonymous blockchain addresses). On the other hand, the encrypted blockchain data also leads to the difficulty of detecting and identifying malicious behaviours via simply data analytics. Moreover, the massive volume and heterogeneity of blockchain data as well as diversity of user behaviours make the problem even worse. As a result, conventional classification-based methods ( _e.g._ , machine learning methods) cannot be directly applied. 

## III. OPPORTUNITIES BROUGHT BY ARTIFICIAL INTELLIGENCE 

Artificial intelligence (AI) as a broad discipline covering machine learning and cognitive computing is an ability of intelligent agents conducting intellectual tasks. The recent advances in big data, AI technologies (such as deep neural networks) and 

general purpose computer hardware such as graphic processing units (GPUs) have greatly driven the development of AI. Consequently, we have witnessed the proliferation of diverse AI applications such as computer vision, natural language processing, speech recognition, sentimental analysis. Big data plays a critical role in propelling AI as well as AI applications. For example, deep learning mainly based on deep neural networks (DNNs) has achieved superior performance thanks to the availability of massive data so that DNNs can extract (or learn) enough features from large volume of data. 

The appearance of diverse blockchain systems has generated the enormous volumes of blockchain data, which are publicly available to everyone. Take Bitcoin as an example. As reported by Statista (https://www.statista.com/) at the end of the third quarter of 2019, Bitcoin contains nearly 242 GB data. Data analytics on the massive blockchain data cannot only extract huge business values but also bring great opportunities to overcome the aforementioned challenges of blockchain systems. The recent advances in AI have also greatly driven the development of big data analytics [7]. Thus, the integration of AI and blockchain technologies can potentially overcome the aforementioned challenges of blockchain systems, thereby forming intelligent blockchain systems. We name such integration of blockchain with AI as _blockchain intelligence_ . It is worth mentioning that there are several research efforts [14], [15] on integrating blockchain with AI while most of them are mainly concentrated on exploiting blockchain for AI so as to overcome the emerging security and privacy vulnerabilities of AI. In contrast to these studies, our work in this paper is mainly focused on solving the intrinsic issues of blockchains by using AI technologies. 

We summarize the opportunities brought by AI to enhance incumbent blockchain systems as follows (as shown in Fig. 2). 

_1) Intelligent operational maintenance of blockchain._ Blockchain generates huge amount of data in a real-time manner. Analyzing blockchain data, we can detect the possible faults, forecast the failures and identify the performance bottleneck so as to tune or adjust the performance of blockchain systems. There are four different levels of data analytics including: descriptive analytics, diagnostic analytics, predictive analytics and prescriptive analytics. In particular, the work of [8] presents a common platform to evaluate the performance of three representative blockchain systems: Ethereum, Parity and Hyperledger Fabric via descriptively analyzing blockchain data. Meanwhile, the descriptive analyt- 

3 

**==> picture [437 x 122] intentionally omitted <==**

**----- Start of picture text -----**<br>
Blockchain Challenges AI & Big Data Opportunities<br>1) Intelligent operational maintenance<br>Operational maintenance<br>1 of blockchain Real-time Anomaly Intelligent<br>Machine learning Data visualization Data mining Monitoring detection restoring<br>eo<br>2) Intelligent quality assurance of smart<br>Quality assurance of contracts<br>2 smart contracts Anomaly detection Federated ML preprocessingData Defect auto-correction optimizationCode auto-<br>i |<br>Malicious behaviour 3) AI-based malicious behaviour<br>3 detection detection<br>Database Storage Computing detectionFraud ManipulationMarket Criminal gangdetection<br>management model<br>**----- End of picture text -----**<br>


Fig. 2. Opportunities brought by artificial intelligence to address challenges of blockchain 

ics of blockchain log data can help to monitor the real-time performance of blockchain systems and identify the possible faults [6]. In addition to diagnostic analytics on blockchain data, predictive analytics is also necessary to anticipate the performance bottleneck of blockchain systems. Unlike diagnostic and predictive analytics, prescriptive analytics can simulate and optimize blockchain systems so as to improve the reliability of blockchain systems. 

_2) Intelligent quality assurance of smart contracts._ Like computer software, smart contracts may contain bugs or faulty programming codes, which are vulnerable to crashes and malicious attacks. It is crucial to detect and identify bugs in smart contracts so as to achieve the ultimate goal of intelligent quality assurance of smart contracts. The work of [9] presents a symbolic-execution platform namely Oyente to detect and recognize potential bugs. Meanwhile, smart contracts are essentially program codes that are sensitive to execution cost (e.g., the execution of smart contracts is charged by gas in Ethereum). Thus, it is a necessity to identify the gascostly patterns and correct these vulnerable smart contracts. In [3], Chen et al. developed a tool namely GASPER to identify and locate seven gas-costly patterns via analyzing bytecodes of Ethereum smart contracts. Machine learning methods can be used to detect and recognize vulnerable bugs in smart contracts automatically. Moreover, the growing number of smart contracts also brings the opportunities to automate the composition of multiple contracts. In particular, we can find and identify contracts that fulfill users’ requirements and composite them together to realize more comprehensive applications. Different from conventional distributed software systems such as web services [11], smart contracts lack of semantic description and Quality-of-Service (QoS) evaluation metrics. As a result, new AI-based approaches are expected to automate labeling semantics of smart contracts and offer data-driven QoS evaluation of smart contracts. 

_3) Automated malicious behaviour detection._ The decentralized blockchain systems result in the difficulty in auditing malicious behaviours such as money laundering, phishing, gambling and scams that occurred in blockchain platforms. Blockchain systems have generated massive transaction data, which are essentially available to everyone, whereas the historical transaction data are pseudonymous through anonymizing 

account addresses. The massive blockchain data brings the opportunities in auditing and detecting malicious behaviours. Big data analytics on massive blockchain data can help to identify malicious users, recognize behaviour pattern, analyze market manipulation, detect scams. The work of [12] presents a cross-graph analysis on Ethereum data and identify several major activities occurring on Ethereum blockchain platforms. As in [10], a machine learning based approach was proposed to detect and capture Ponzi schemes that took place in Ethereum. The work of [13] analyzes the leaked transaction history of Mt. Gox Bitcoin exchange and identify a number of market manipulation patterns via singular value decomposition (SVD) method. Moreover, malicious users may exploit multiple anonymous accounts to form a criminal gang to conduct illegal activities on blockchain systems. New machine learning approaches as well as association analysis on multiple accounts are expected to address this issue. 

The advances in AI, machine learning and big data analytics bring numerous opportunities to address the aforementioned blockchain challenges. We next present a case study to demonstrate the feasibility of blockchain intelligence. 

## IV. CASE STUDY 

Big data analytics of blockchain data is beneficial to fraud recognition of transactions and vulnerability detection of smart contracts. However, it is also challenging to conduct big data analytics of blockchain data. 1) It is extremely time consuming to download the entire blockchain data due to the bulky blockchain size, e.g., it took more than one week and over 500 GB storage space to fully synchronize ( _i.e._ , download) the entire Ethereum at a newly-joined peer. 2) It requires substantial efforts in extracting and processing blockchain data. First, blockchain data is stored at clients in heterogeneous and complex data structures, which cannot be directly analyzed. Meanwhile, the underlying blockchain data is either binary or encrypted. Thus, it is a necessity to extract and process binary and encrypted blockchain data so as to obtain valuable information while this process is non-trivial as conventional data analytic methods may not work for this type of data. 3) There is no general data extract tool for blockchain data. Although several open source tools for blockchain data 

4 

**==> picture [223 x 101] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Raw data  Block N  Blockchain Peer  Ethereum Virtual Machine<br>collection  Block Header  Execute  Contract A<br>Transaction 1  Commit  ① Create  ② Call<br>Transaction 2  ③ Suicide ④ Reward<br>...  Receipt  Contract B<br>Block  Receipt  Trace Trace<br>(b) Processed<br>Datasets<br>Block and  Internal Ether  Contract  Contract  ERC20 Token  ERC721 Token<br>Transaction  Transaction  Info  Calling  Transaction  Transaction<br>**----- End of picture text -----**<br>


Fig. 3. Data processing during Ethereum transaction flow 

extraction are available, most of them can only support to extract partial blockchain data (not the entire data). 

## _A. Data Extraction from Ethereum_ 

To address the above challenges, we propose a blockchain data analytics framework namely XBlock-ETH to analyze Ethereum data. In particular, we extract raw data consisting of 8,100,000 blocks of Ethereum. Fig. 3(a) illustrates the typical Ethereum transaction execution flow from Block _N_ to EVM through blockchain peer. During this procedure, we collect the three types of blockchain raw data: Block, Receipt and Trace. Since the analysis on the raw blockchain data is difficult, we process and categorize the obtained Ethereum Blockchain data into six datasets: _(1) Block and Transaction_ , _(2) Internal Ether Transaction_ , _(3) Contract Information_ , _(4) Contract Calls_ , _(5) ERC20 Token Transactions_ , _(6) ERC721 Token Transactions_ as shown in Fig. 3(b). It is non-trivial to process the raw since it requires substantial efforts in extracting useful information from raw data and associating with six datasets. 

We then conduct statistic analysis on these refined datasets. In Ethereum, a miner has a higher priority to package the transactions with higher “gasPrice” into the block. The visualization of “gasPrice” is shown in Fig. 4. In a macro view (as shown in Fig. 4(a)), the “gasPrice” is gradually decreasing with the development of the Ethereum community, except for several peaks caused by extremely frequent transaction when the network is congested. In a micro view (as shown in Fig. 4(b)), we extract the time from 8,000,000 to 8,020,000 blocks and find that such fluctuations of “gasPrice” can be observed by the tidal law. This observation implies that the fluctuations of “gasPrice” can potentially be predicted. 

## _B. Detection Ponzi Schemes from Ethereum_ 

Blockchain can also be exploited to conduct illegal activities such as scams. For example, in [10], we propose a method to detect Ponzi scams in Ethereum blockchain through extracting and analyzing key characteristics of user accounts and operation codes in Ethereum contracts. Fig. 5 makes a comparison between a normal contract and a Ponzi scheme contract in terms of Ether Flow Graph, where the horizontal axis denotes the time line, the vertical axis represents the number of participants in a particular contract, red circles and blue circles denote the investment transactions and payment 

**==> picture [233 x 94] intentionally omitted <==**

**----- Start of picture text -----**<br>
30 4.0 1e10<br>MIN MIN<br>28 MAX 3.5 AVG<br>AVG 3.0<br>26 2.5<br>24 2.0<br>22 1.5<br>1.0<br>20 0.5<br>180 100 200 300 400 500 600 700 800 0.00 200 400 600 800 1000<br>Per 10000 Block Per 20 Block<br>(a) Macro view of GasPrice (b) Micro view of GasPrice<br>GasPrice<br>log(GasPrice)<br>**----- End of picture text -----**<br>


Fig. 4. Data visualization of Gas Price of Ethereum 

transactions, respectively. In addition, the size of circle also represents the amount of transactions, _i.e._ , the larger circle means the larger amount of transactions. 

Fig. 5(a) is a normal lottery contract while Fig. 5(b) shows a typical Ponzi scam (namely Rubixi). We can observe several significant differences between Fig. 5(a) and Fig. 5(b): 1) there are more participants in Fig. 5(b) than Fig. 5(a); 2) there are more payment transactions in Fig. 5(b) than Fig. 5(a) which exhibits more randomness in the number of transactions. After extracting the key features and applying other data mining and machine learning methods, we can successfully classify Ponzi scams from other normal activities. 

## V. CONCLUSION AND FUTURE DIRECTIONS 

This article first reviews the blockchain technologies and analyze the challenges in blockchain systems. We then introduce artificial intelligence (AI) as well as opportunities brought by AI to blockchain systems. We name such integration of blockchain and AI as _blockchain intelligence_ . We mainly discuss that AI bring benefits to blockchain in aspects of _intelligent operational maintenance of blockchain_ , _intelligent quality assurance of smart contracts_ and _automated malicious behaviour detection_ . In addition, we also give a case study to further demonstrate great potentials of blockchain intelligence. 

We believe that the integration of AI with blockchain technology will further drive the benignant development of blockchain systems. We outline the future directions in blockchain intelligence as follows. 

- _Real-time and automated operational maintenance of blockchain._ Instead of performance monitoring and fault detection, the future intelligent operational blockchain systems are expected to conduct real-time monitoring on multiple performance metrics of blockchain systems and achieve the automated restoring from crash spots. 

- _Collective intelligence bestowing smart contracts._ The current QoS assurance approaches of smart contracts are mainly based on the analysis on contract patterns ( _i.e._ , detecting and classifying vulnerable contracts) at a sole peer in the blockchain. In contrast to a single intelligent agent, _collective intelligence_ can motivate all participants to engage in analyzing and reasoning, thereby contributing their collective knowledge to make better decision in a global context. In the future, collective intelligence is expected to integrate with decentralized 

5 

**==> picture [231 x 144] intentionally omitted <==**

**----- Start of picture text -----**<br>
G<br>G GG<br>G<br>20th G GG G GGGG<br>G G G G G GGG G GGG G GG G G G G GGG G GGG GG GG G G G<br>GGGGGG G GG GGGGGGG G G GGGGGGG G G G G GG G G GG GGGGGGGG GG G G GGGGGGGGGGG GG GG G G GG G Amount<br>GG GG GG GG GG G GG G GGG GG G G G GGG G GG G GG G G G GG G G G G G G G 0.5<br>15th GG G G G GG G GG G G G GG G G GG GGG G GG GG GG G GGGGGGGG GGG G G G G GGGG G G G G G GG G G G G GGGGGGGGGGGG G G G G G G GGG G GG G GGGG GG G GGG GGGGG G G G G G GGGGGG G G GGGG G GG G GGGG GG 1.01.5<br>GG G GG GG GG GG G GG G G GG G G G G G G G G GG G 2.0<br>G G GG G GG GG G GG GG G G G GG G G G G GG G G GGGG G GG GGG G G GG G G G GGG<br>10th GG G GGGG GGGG GGG GG G GG G G G GG G G GGG G GGG G G G G G G<br>G GGG G G G G G G GG GG G G G G GGGGG GG G GG G G GG G G GG G GG G GG GGG Direction<br>GGG G G GG G G GG G G G G GG G G GGG G GG G GGG G G G G GG GG G G G Investment<br>GGGGGGGGG GGGG GGGGG G GG G G GGGGG GGGGGGGG G GGG G G GG GG GG G GGG G G GGG G G GGG GG G GGGGG G GGGG G G Payment<br>5th GGG G G G G GGG GG GG G GG GG GG G GGG GG GG G GG GGGG GG GG GG G G G G GGGG G<br>GG G G G GGGG G G G G GG G GGG G G GGG G G G G G G G GGGG G G GGG G GG G G G G<br>G G GGG GGG G G G G GG G GG G G G G G GG G GG G G GGG G GG G G GGG GGG<br>G G G GGG G GG GGGGG G G G G G G G G GG GGG G G GGG G GG G G GG G G<br>1th GG<br>2016−02−18 02−19 02−20 02−22 02−23 02−24 02−25 02−27<br>Time<br>No. of participants<br>**----- End of picture text -----**<br>


**==> picture [163 x 8] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) LooneyLottery contract ( i.e. , normal contract)<br>**----- End of picture text -----**<br>


**==> picture [231 x 144] intentionally omitted <==**

**----- Start of picture text -----**<br>
120th G<br>105th GGGGGG [G][G]<br>90th75th G [G] [G][G][G][G][GGGG] G G G GG G G G GG G [G] GG [G] G G GG G GGG GGG [GGG] G GG G [G][G][G][G] [G][G] G G G Direction G Investment<br>G [G][G][G][G] G Payment<br>60th GG [G][G] G [G] [GG][G] [G]<br>45th G [G][G][G] GG [G] G [G] [G] G [G][G][G] [G] G G GGGGG G Amount G 50<br>GG [G][G][G] G 100<br>30th G G [G] [G] G [G][G][G] [GG] G 150<br>15th0th G [G] G [G] G [G] GG G G GGGGG G [G] G [G] GG [G] G GG G GGGG G G G [G] G GGG GG GG G [G] G GGG G [G] G G [G] [G] G G GGG [G] GG GG GGGG G [G] GG G G G [G] G G G GG G G G G G G G G G G [GG][G][G] G G GG<br>2016−03−14 2016−03−20 2016−04−05 2017−08−19<br>Time<br>No. of participants<br>**----- End of picture text -----**<br>


- (b) Rubixi contract ( _i.e._ , Ponzi scam contract) 

Fig. 5. Ether Flow Graph of two smart contracts [10]. 

   - blockchain system so as to offer a trustworthy and intelligent provision of blockchain services. 

- _Integrating multiple machine learning approaches to monitor and supervise blockchain data._ Decentralization of blockchain brings the difficulty in monitoring and supervising transactions in blockchain platforms, consequently resulting in a number of illegal behaviours. Meanwhile, both heterogeneity and pseudonymity of blockchain data make this situation even worse. In the future, multiple machine learning approaches should be integrated together to extract key features from different types of blockchain data. In addition, a dynamic graph (or network) of transactions is also expected to identify the association between different accounts so as to recognize the malicious behaviours. 

## REFERENCES 

- [1] X. Li, P. Jiang, T. Chen, X. Luo, Q. Wen, “A survey on the security of blockchain systems”, _Future Generation Computer Systems_ , 2017, https://doi.org/10.1016/j.future.2017.08.020. 

- [2] N. Szabo, “The idea of smart contracts”, _Nick Szabo’s Papers and Concise Tutorials_ . http://www.fon.hum.uva.nl/rob/Courses/ InformationInSpeech/CDROM/Literature/LOTwinterschool2006/szabo. best.vwh.net/smart contracts 2.html 

   - [8] T. T. A. Dinh, R. Liu, M. Zhang, G. Chen, B. C. Ooi and J. Wang, “Untangling Blockchain: A Data Processing View of Blockchain Systems,” _IEEE Transactions on Knowledge and Data Engineering_ , vol. 30, no. 7, pp. 1366-1385, 1 July 2018 

   - [9] L. Luu, D.-H. Chu, H. Olickel, P. Saxena, and A. Hobor, “Making Smart Contracts Smarter,” _Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security_ (CCS ’16), 2016 

   - [10] W. Chen, Z. Zheng, J. Cui, E. Ngai, P. Zheng, and Y. Zhou, “Detecting Ponzi Schemes on Ethereum: Towards Healthier Blockchain Technology,” _Proceedings of the 2018 World Wide Web Conference_ (WWW ’18), 1409-1418. 

   - [11] P. Rodriguez-Mier, C. Pedrinaci, M. Lama and M. Mucientes, “An Integrated Semantic Web Service Discovery and Composition Framework,” _IEEE Transactions on Services Computing_ , vol. 9, no. 4, pp. 537-550, 1 July-Aug. 2016. 

   - [12] T. Chen et al., “Understanding Ethereum via Graph Analysis,” _IEEE INFOCOM 2018 - IEEE Conference on Computer Communications_ , 2018, pp. 1484-1492. 

   - [13] W. Chen et al., “Market Manipulation of Bitcoin: Evidence from Mining the Mt. Gox Transaction Network”, _IEEE Conference on Computer Communications_ , 2019, pp. 964-972. 

   - [14] T. N. Dinh and M. T. Thai, “AI and Blockchain: A Disruptive Integration”, _Computer_ , vol. 51, no. 9, pp. 48-53, September 2018. 

   - [15] Nassar, M, Salah, K, ur Rehman, MH, Svetinovic, D., “Blockchain for explainable and trustworthy artificial intelligence”, _WIREs Data Mining Knowl Discov_ , 2020; 10:e1340. https://doi.org/10.1002/widm.1340 

- [3] T. Chen, X. Li, X. Luo, X. Zhang, “Under-optimized smart contracts devour your money”, _Proceedings of 24th International Conference on Software Analysis, Evolution and Reengineering (SANER)_ , 2017, pp. 442–446. 

- [4] J. Bonneau, J. Clark, S. Goldfeder, “On bitcoin as a public randomness source”, _IACR Cryptology ePrint Archive_ , 1015, 2015 

- [5] The DAO, The Hack, The Soft Fork and The Hard Fork (2017). https://www.cryptocompare.com/coins/guides/ the-dao-the-hack-the-soft-fork-and-the-hard-fork/ 

- [6] P. Zheng, Z. Zheng, X. Luo, X. Chen, X. Liu, “A detailed and realtime performance monitoring framework for blockchain systems,” _2018 IEEE/ACM 40th International Conference on Software Engineering: Software Engineering in Practice Track_ , pp. 134–143, 2018. (conference proceedings) 

- [7] H.-N. Dai et al., “Big data analytics for large-scale wireless networks: Challenges and opportunities,”, _ACM Computing Surveys_ , vol. 52, no. 5, 2019 

