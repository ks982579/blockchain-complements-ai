## **LUUNU — BLOCKCHAIN, MISP, MODEL CARDS AND FEDERATED LEARNING ENABLED CYBER THREAT INTELLIGENCE SHARING PLATFORM** 

Eranga Bandara Sachin Shetty Ravi Mukkamala Old Dominion University Norfolk, VA, USA {cmedawer,sshetty, rmukkama}@odu.edu 

Abdul Rahaman 

Deloitte & Touche LLP abdulrahman@deloitte.com 

Xueping Liang 

University of North Carolina Greensboro, NC, USA x_liang@uncg.edu 

## **ABSTRACT** 

Cyber Threat Intelligence (CTI) is a process of threat data collection, processing, and analysis to understand a threat actor’s motives, targets, and attack behaviors. CTI involves highly sensitive data and any inadvertent access to it can harm the affected organization’s reputation. More importantly, CTI sharing has a dangerous outcome of inadvertently revealing the security weaknesses or vulnerabilities in an organization’s infrastructure. It is important that any proposed system of CTI sharing guarantees preserving the privacy and anonymity of the partnering organizations. In this paper, we propose "Luunu," a blockchain, MISP, Model Cards and Federated Learning enabled CTI sharing platform, to provide enhanced privacy, transparency, traceability, anonymity, and data provenance in a scalable manner. The self-sovereign identity (SSI) capability of Luunu ensures the anonymity of the participants in the CTI sharing. Further, we propose a blockchain-based federated learning system to analyze the collected CTI data from the participating organizations. 

**Keywords:** Blockchain, Federated Learning, Cyber Threat Intelligence, MISP, Model Card. 

## **1 INTRODUCTION** 

Cyber Threat Intelligence (CTI) is the information about cyber threats faced by an organization. It could include information about attacks that occurred and those that were attempted but failed. This type of knowledge allows organizations to be prepared to handle such attacks. The study of CTI helps in providing information on adversaries as well as their tactics (Mavroeidis and Bromander 2017). It enables organizations to proactively plan to prevent or handle such attacks. Understanding security vulnerabilities, threat indicators, and the mode in which the attacks are carried out are essential to combat cyber-attacks effectively. Such proactive measures help security administrators to act with speed in preventing, containing, and han- 

_ANNSIM’22, July 18-20, 2022, San Diego, CA, USA; ©2022 Society for Modeling & Simulation International (SCS)_ 

**235** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

dling these attacks. It would also reduce the damage if an organization is indeed subjected to these attacks. CTI is relevant at every level of an organization’s infrastructure including individual machines, servers, and network infrastructure (Kampanakis 2014). 

A coalition of organizations formed to share individual CTI information can accomplish more than sharing the raw data. It can involve CTI collection, analysis, summarization, highlighting, and sharing of cyber threat information collected from individual member organizations. Typically, such a system extracts attack patterns, attacker identifications, attacking malware, and the underlying attack tactics. The data is first expressed in standard formats and then analyzed (Wagner, Mahbub, Palomar, and Abdallah 2019). 

In general, the information associated with CTI is highly sensitive and has the potential to harm an organization’s reputation, if leaked. Moreover, the CTI information sharing system may also inadvertently advertise a vulnerability that may be present in an organization’s infrastructure (Johnson, Badger, Waltermire, Snyder, and Skorupka 2016). Fear of such leaks may prevent organizations from sharing their CTI information with their coalition cohorts, effectively preventing the formation of such coalitions. To encourage organizations to share their CTI without negatively impacting their organization’s security, anonymization of the participant to whom specific data pertains has been proposed. However, this introduces a situation where the data will lack credibility if its origin cannot be confirmed (Burger, Goodman, Kampanakis, and Zhu 2014). Thus, we need a solution that provides anonymization while retaining data source credibility (or provenance). 

In this paper, we propose Luunu, a blockchain, MISP (Malware Information Sharing Platform) (Wagner, Dulaunoy, Wagener, and Iklody 2016), Model Cards (a structured framework to enable ML model provenance) (Bandara, Shetty, Rahman, Mukkamala, Zhao, and Liang 2022), (Wadhwani and Jain 2020a) and Federated Learning-enabled (Yang, Andrew, Eichner, Sun, Li, Kong, Ramage, and Beaufays 2018), (Koneˇcn`y, McMahan, Yu, Richtárik, Suresh, and Bacon 2016) CTI sharing platform. Luunu addresses the main challenges in CTI sharing such as privacy, reliability, traceability, anonymity and data provenance (Feng, He, Zeadally, Khan, and Kumar 2019). It stores the CTI information on the MISP storage as Model Card objects. The data provenance information of CTI sharing is stored in the blockchain ledger. The federated learning system (Koneˇcn`y, McMahan, Yu, Richtárik, Suresh, and Bacon 2016), (Yang, Andrew, Eichner, Sun, Li, Kong, Ramage, and Beaufays 2018) in Luunu can build machine learning models to detect cyber threats (e.g., network attacks, DDOS attacks, anomaly detection, etc.). Smart contracts of the Luunu blockchain implement several functionalities for local and federated learning. It facilitates the local model generation, model parameter sharing among participants, global model averaging, and eventual model storing, and sharing. Luunu records the generated machine learning models, both local and global, on the blockchain as Model Card objects (Wadhwani and Jain 2020b), (Bandara, Shetty, Rahman, Mukkamala, Zhao, and Liang 2022). These models can be incorporated into the smart contracts to analyze any cyber attacks in real time by the peers in the network. Self-sovereign identity (SSI) (Liang, Shetty, Zhao, Bowden, Li, and Liu 2017), (Mühle, Grüner, Gayvoronskaya, and Meinel 2018) capability is integrated into this platform to enforce participant anonymity in information sharing among the coalition members. The following are our main contributions: 

1. Proposed a blockchain-based CTI sharing platform; 

2. Provided enhanced transparency and provenance of the CTI sharing by storing CTI data on MISP as model Card objects; 

3. Blockchain-enabled coordinator-less federated machine learning approach has been used to analyze the CTI data on different peers; 

4. Employed Self-sovereign identity-enabled mobile wallet for organizations to anonymously report CTI information. 

**236** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

**==> picture [363 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Luunu platform layers. (b) Luunu platform blockchain ledger.<br>**----- End of picture text -----**<br>


Figure 1: Luunu platform architecture. 

The rest of the paper is organized as follows. Section 2 formulates the architecture of the Luunu platform. Section 3 presents the functionality of the Luunu platform, Section 4 summarizes the results from our performance evaluation studies. Section 5 provides a survey of related work. Finally, section 6 concludes the Luunu platform with suggestions for future work. 

## **2 LUUNU PLATFORM ARCHITECTURE** 

As shown in Figure 1(a) Luunu platform consists of five main layers: (1) Stakeholder layer (2) Smart contract layer (3) Blockchain storage layer (4) CTI storage layer, and (5) Data analytics layer. 

## **2.1 Stakeholder Layer** 

This layer supports three types of stakeholders: incident reporters, incident viewers, and service providers (admins). Multiple organizations can collaborate on the Luunu platform. Each organization can deploy their blockchain node. Incident reporters and viewers belong to the coalition member organizations. Once an organization has joined a CTI coalition, it is onboarded to the Luunu platform. Then, the authorized incident reporters/viewers from the organization need to register their identities on the Luunu platform via the self-sovereign identity enabled Luunu mobile wallet. Their identities are managed within the system as self-sovereign identity proof (SSI-Proof), ensuring anonymity. The real identity information will be stored on the individual user’s Luunu mobile wallet and the proofs of the identities are stored in the blockchain as a SSI Proof. Service providers are the hosts of the platform and hold the admin role. There could be multiple service providers with identical permissions. Their main function is to onboard organizations into the platform and verify/approves the identities of the incident reporters and viewers. 

## **2.2 Smart Contract Layer** 

All the ledger functions in the Luunu platform are implemented using smart contracts in the blockchain. The blockchain may be deployed among multiple organizations with each organization running its blockchain 

**237** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

**==> picture [409 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Model Card encoded CTI record. (b) Model Card encoded machine learning model record.<br>**----- End of picture text -----**<br>


Figure 2: Luunu platform model cards. 

node (Figure 1(a)). It stores users’(incident reporters/viewers) digital identity proofs (referred to as SSI proofs (Baars 2016)), data provenance information of shared CTI, and machine learning model information (as Model Card objects). Each blockchain node comes with a Model Card service and federated machine learning-based data analytic service, "Fml service" as shown in Figure 1(b). Model cards in Luunu encode the CTI data and federated machine learning model-related data. The Model Card service handles the Model Card object generation functions with these data. Luunu can build machine learning models to detect cyber threats using the federated machine learning approach. These machine learning model generation functions handle in the Fml service. 

There are four main smart contracts: Identity contract, CTI contract, FML contract, and Notification contract. The identity contract implements the identity management of the stakeholders in the system. All the CTI data in the Luunu platform are stored in the MISP storage as Model Card objects. The MISP contract facilitates the functions of saving and retrieving CTI data on MISP storage. It encodes CTI data into Model Card objects interacting with the Model Card service. Then these encoded Model Card data will be saved in the MISP storage. The data provenance information of CTI sharing is stored in the blockchain ledger. FML contact handles the machine learning model generation functions. The model parameter sharing, local model generation, model averaging, model storing, and sharing functions are implemented with this smart contract. Further, it encodes machine learning model information into Model Card objects and saves it in the blockchain ledger for data provenance. 

## **2.3 MISP Layer** 

Luunu platform stores all CTI data on MISP storage as Model Card objects. The structure of the Model Card encoded CTI record is described in Figure 2(a). CTI data create/search functions handles in the MISP layer. MISP cluster is deployed along with the blockchain network. MISP cluster exposes APIs for outside to interact with its functions. There are two main functions exposed by this API, 1) create CTI events, and 2) search CTI events. The blockchain smart contracts(e.g MISP contract) save the Model Card encoded CTI events on MISP storage via interacting with the create CTI event API. Smart contracts search the CTI events by invoking the search CTI event API in the MISP storage. 

**238** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

## **2.4 Data Analytic Layer** 

Luunu platform uses blockchain-enabled coordinator less federated learning approach (Koneˇcn`y, McMahan, Yu, Richtárik, Suresh, and Bacon 2016) to build the machine learning models with the cyber threat data. These machine learning functions are implemented in the data analytic layer. The federated learning service in the Luunu blockchain peers implemented the federated learning functions, Figure 1(b). The blockchain smart contracts interact with the functions implemented in the federated learning service to build the machine learning models. The service is capable to build the machine learning models with the data stored in the off-chain storage of the blockchain peers. The generated model information will be saved in the blockchain ledger as Model Card objects. The structure of a Model Card object is described in Figure 2(b). 

## **3 LUUNU PLATFORM FUNCTIONALITY** 

## **3.1 Identity Registration** 

First, the organizations will be onboarded on the Luunu platform by the Luunu service provider. When onboarding an organization, a new blockchain can be deployed for the organization. Then the members of these organizations can register their identities on the Luunu platform via the Luunu mobile wallet. Identity registration happens through the self-sovereign identity approach. The users’ real identity data(e,g personal details, photos etc) will be stored in the mobile wallet and proof will be uploaded to the blockchain ledger along with the public key of the mobile wallet as SSI-proof. Then the admin needs to verify the user’s identity based on the SSI verification approach (Baars 2016). The identity registration and approval flows are discussed in Figure 3(a). 

## **3.2 CTI Reporting and Viewing** 

Once users registered on the platform, they can report the cyber threat incidents. The reporting can be done via the Lunnu mobile wallet. A cyber threat record contains 5 different fields, 1) date(date of the incident), 2) incident type(define incident types such as network attack, DDOS attack etc), 3) incident description, 4) incident status(pending/resolved status of the incident), 5) incident action(action taken to resolve the attack). The incident report functions are implemented in the Incident smart contract. When reporting an incident, the smart contact encodes the threat information into the Model Card object and saves it in the MISP storage. The saved MISP objects will be available for all the organizations in the network. 

## **3.3 Data Privacy, Security and Anonymity** 

Our platform has a special emphasis on the privacy, anonymity, reliability and provenance of the reported incidents. This is achieved using the following techniques, 1) All the cyber threat information will be saved on MISP storage as Model Card objects guaranteeing auditability and reliability and data provenance, 2) The anonymity of the user data is realized by using SSI-based mobile wallet applications, 3) The federated learning service in Luunu can build machine learning models to detect cyber threats(e.g network attacks, DDOS attacks, anomaly detection etc), 4) Data provenance of the federated learning is achieved by storing the machine learning model information in the blockchain ledger as Model Cards objects. 

**239** 

**==> picture [188 x 10] intentionally omitted <==**

**----- Start of picture text -----**<br>
Bandara, Shetty, Mukkamala, Rahaman, Liang<br>**----- End of picture text -----**<br>


**==> picture [436 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Luunu identity registration and incident report flow (b) Luunu platform microservices based architecture.<br>**----- End of picture text -----**<br>


Figure 3: Luunu platfrom implementation. 

## **4 LUUNU IMPLEMENTATION AND EVALUATION** 

Luunu platform is implemented on top of the Rahasak blockchain (Bandara, Liang, Foytik, Shetty, Ranasinghe, and De Zoysa 2021), (Bandara, NG, DE Zoysa, Fernando, Tharaka, Maurakirinathan, and Jayasuriya 2018). Figure 3(b) presents the architecture. Rahasak blockchain’s Aplos platform is used to implement the smart contracts (Bandara, NG, De Zoysa, and Ranasinghe 2019), (Bandara, Liang, Foytik, Shetty, Ranasinghe, Zoysa, and Ng 2021). Pytorch and Pysyft libraries (Paszke, Gross, Massa, Lerer, Bradbury, Chanan, Killeen, Lin, Gimelshein, Antiga, et al. 2019), (Ziller, Trask, Lopardo, Szymkow, Wagner, Bluemke, Nounahon, Passerat-Palmbach, Prakash, Rose, et al. 2021) comprise the federated learning functions which are included to make up the Rahasak blockchain. In addition, the TensorFlow Model Card Toolkit (Wadhwani and Jain 2020a) is used to build the Model Card service while Apache Kafka (Kreps, Narkhede, Rao, et al. 2011), (Bandara, Tosh, Foytik, Shetty, Ranasinghe, and De Zoysa 2021) is used to support the blockchain consensus, inter-service communication and back-pressure operation handling (Davis 2019), (Bandara, Tosh, Foytik, Shetty, Ranasinghe, and De Zoysa 2021). The following evaluation test results for Luunu below are representative of a varying number of peers and records. 

**Performance of Invoke Transactions** – The Invoke transactions update is made up of the creation of transaction records and the Invoke transactions update. A single peer execution is recorded where concurrent invoke transactions are recorded relative to the total number of committed transactions. Figure 4 and Figure 5 compares Luunu invoke transaction performance. 

**Performance of Query Transactions** – Assets from the ledger are the only query transactions read that do not update asset status nor create new transaction records. Figure 4 and Figure 5 summarize the query transaction performance of Luunu where concurrent query transactions are executed across all blockchain peers. 

**Performance of Transaction Scalability and Latency** – The invoke transactions (per second) are measured against the number of peers. Figure 6 shows a comparison of the query and invoke scalabilities. Figure 7 shows the decrease in the latency of transactions while taking into account the increase in the number of executed transaction counts (per second) when adding peers to the cluster. 

**240** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

Figure 4: Transaction throughput of Luunu blockchain peer. 

Figure 6: Transaction scalability of Luunu blockchain platform. 

Figure 8: Block creation time. 

Figure 5: Transaction throughput of Luunu blockchain. 

Figure 7: Transaction latency of Luunu blockchain platform. 

Figure 9: Block creation frequency. 

**241** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

**Block Generation Time** – Block generation time represents the duration of time needed to generate a block measured by the number of nodes and block transactions. Block generation time depends on four main time factors: leader election time, data replication and broadcast time between peers, Merkle proof/block hash generation time, and transaction validation time. When the number of transactions in the block and the number of peers in the network increase, each of these factors also increase. Due to this reason, block generation time also increases correspondingly. Figure 8 and Figure 9 discuss the block generation time variation with different transaction sets and the different number of blockchain peers. 

## **5 RELATED WORK** 

In this section we outline the architecture and main features of research conducted in identity management for self-sovereignty and cyber information sharing with the feature of preserving privacy (Liu, Sun, and Schuckers 2019). Table 1 displays the summary comparison of Luunu with these platforms which demonstrates the superiority of our system based on the results. 

**BCTISA** (Cha, Singh, Pan, and Park 2020) is a sustainable computing platform based on the blockchain that supports cyber intelligence. Issues with data collection along with associate requirements are identified during the process of threat information sharing. **BLOCIS** (Gong and Lee 2020) is Sybil-Resistance a Sharing Framework for Blockchain focused (Cyber) Threat Intelligence (CTI). It is a CTI framework that quickly eliminates and detects data issues related to "Sybil" attack resistance. **BloCyNfo-Share** (Badsha, Vakilinia, and Sengupta 2020) is a Fine-Grained Access Control Cybersecurity Information Sharing capability. It aims to preserve privacy while sharing information via using attribute-based encryption and proxy re-encryption. 

**SDN-CTI** (Hajizadeh, Afraz, Ruffini, and Bauschert 2020) provides collaborative defense using blockchain in SDN Networks. This technology focuses on sharing CTI in a scalable, low cost, immutable, and secure manner that is easy to deploy in a decentralized infrastructure. **CTI-Cloud** (Kamhoua, Martin, Tosh, Kwiat, Heitzenrater, and Sengupta 2015) is a cloud computing CTI sharing system. This work centers around using game theory to investigate the discovery of vulnerabilities while sharing CTI. **PP-CTI** (Badsha, Vakilinia, and Sengupta 2019) is a CTI platform for sharing and learning information for defense of infrastructures. 

Table 1: CTI sharing platform comparison. 

|Platform|Architecture|Blockchain<br>Enabled|Running<br>Blockchain|Scalability|Privacy<br>Level|SSI|Model Cards|Auditing &<br>Provenance|Data<br>Analytic|
|---|---|---|---|---|---|---|---|---|---|
|Luunu|Decentralized|✓|Rahasak|High|High|✓|✓|✓|✓|
|BCTISA|Decentralized|✓|N/A|N/A|Mid|✗|✓|✗|✗|
|BLOCIS|Decentralized|✓|N/A|N/A|Mid|✗|✓|✓|✗|
|BloCyNfo-Share|Decentralized|✓|Ethereum|Low|Mid|✗|✗|✓|✗|
|SDN-CTI|Decentralized|✓|Hyperledger|Mid|Mid|✗|✓|✗|✗|
|CTI-Cloud|Centralized|✗|N/A|Mid|Mid|✗|✗|✗|✗|
|PP-CTI|Centralized|✗|N/A|Mid|Mid|✗|✗|✗|✗|



## **6 CONCLUSIONS AND FUTURE WORK** 

With Luunu we have proposed a blockchain, MISP (Wagner, Dulaunoy, Wagener, and Iklody 2016), Model Cards and Federated Learning enabled CTI sharing platform. The proposed Luunu stores all the CTI information on the MISP storage as Model Card objects. The data provenance information of CTI sharing is stored in the blockchain ledger. The federated learning system in Luunu can build machine learning models to detect cyber threats(e.g network attacks, DDOS attacks, anomaly detection etc). Luunu addresses the main issues in CTI sharing such as traceability, reliability, privacy, scalability, anonymisation and data provenance using the self-sovereign identity approach. For future work, we will work with participating 

**242** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

organizations to develop rules or guidelines for users to post CTI information, ask questions about CTI information shared, correct the shared CTI information and address potential disputes through Luunu. We would also like to assess the quality of the shared CTI information and come up with an incentive mechanism to encourage CTI information sharing through Luunu. Collecting user feedback and developing governance mechanisms will be beneficial to further development and broader the adoption of Luunu. 

## **ACKNOWLEDGEMENTS** 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## **REFERENCES** 

- Baars, D. 2016. “Towards self-sovereign identity using blockchain technology”. Master’s thesis, University of Twente. 

- Badsha, S., I. Vakilinia, and S. Sengupta. 2019. “Privacy preserving cyber threat information sharing and learning for cyber defense”. In _2019 IEEE 9th Annual Computing and Communication Workshop and Conference (CCWC)_ , pp. 0708–0714. IEEE. 

- Badsha, S., I. Vakilinia, and S. Sengupta. 2020. “Blocynfo-share: Blockchain based cybersecurity information sharing with fine grained access control”. In _2020 10th Annual Computing and Communication Workshop and Conference (CCWC)_ , pp. 0317–0323. IEEE. 

- Bandara, E., X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa. 2021. “Rahasak-Scalable blockchain architecture for enterprise applications”. _Journal of Systems Architecture_ , pp. 102061. 

- Bandara, E., X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. D. Zoysa, and W. K. Ng. 2021. “SaaS - Microservices-Based Scalable Smart Contract Architecture”. In _Security in Computing and Communications_ , edited by S. M. Thampi, G. Wang, D. B. Rawat, R. Ko, and C.-I. Fan, pp. 228–243. Singapore, Springer Singapore. 

- Bandara, E., W. K. NG, K. DE Zoysa, N. Fernando, S. Tharaka, P. Maurakirinathan, and N. Jayasuriya. 2018. “Mystiko—Blockchain Meets Big Data”. In _2018 IEEE International Conference on Big Data (Big Data)_ , pp. 3024–3032. IEEE. 

- Bandara, E., W. K. NG, K. De Zoysa, and N. Ranasinghe. 2019. “Aplos: Smart contracts made smart”. _BlockSys’2019_ . 

- Bandara, E., S. Shetty, A. Rahman, R. Mukkamala, J. Zhao, and X. Liang. 2022. “Bassa-ML—A Blockchain and Model Card Integrated Federated Learning Provenance Platform”. In _2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC)_ , pp. 753–759. IEEE. 

- Bandara, E., D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa. 2021. “Tikiri-Towards a lightweight blockchain for IoT”. _Future Generation Computer Systems_ . 

- Burger, E. W., M. D. Goodman, P. Kampanakis, and K. A. Zhu. 2014. “Taxonomy model for cyber threat intelligence information exchange technologies”. In _Proceedings of the 2014 ACM Workshop on Information Sharing & Collaborative Security_ , pp. 51–60. 

- Cha, J., S. K. Singh, Y. Pan, and J. H. Park. 2020. “Blockchain-Based Cyber Threat Intelligence System Architecture for Sustainable Computing”. _Sustainability_ vol. 12 (16), pp. 6401. 

- Davis, A. L. 2019. “Akka Streams”. In _Reactive Streams in Java_ , pp. 57–70. Springer. 

- Feng, Q., D. He, S. Zeadally, M. K. Khan, and N. Kumar. 2019. “A survey on privacy protection in blockchain system”. _Journal of Network and Computer Applications_ vol. 126, pp. 45–58. 

**243** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

- Gong, S., and C. Lee. 2020. “BLOCIS: Blockchain-Based Cyber Threat Intelligence Sharing Framework for Sybil-Resistance”. _Electronics_ vol. 9 (3), pp. 521. 

- Hajizadeh, M., N. Afraz, M. Ruffini, and T. Bauschert. 2020. “Collaborative Cyber Attack Defense in SDN Networks using Blockchain Technology”. In _2020 6th IEEE Conference on Network Softwarization (NetSoft)_ , pp. 487–492. IEEE. 

- Johnson, C., M. Badger, D. Waltermire, J. Snyder, and C. Skorupka. 2016. “Guide to cyber threat information sharing”. Technical report, National Institute of Standards and Technology. 

- Kamhoua, C., A. Martin, D. K. Tosh, K. A. Kwiat, C. Heitzenrater, and S. Sengupta. 2015. “Cyber-threats information sharing in cloud computing: A game theoretic approach”. In _2015 IEEE 2nd International Conference on Cyber Security and Cloud Computing_ , pp. 382–389. IEEE. 

- Kampanakis, P. 2014. “Security automation and threat information-sharing options”. _IEEE Security & Privacy_ vol. 12 (5), pp. 42–51. 

- Koneˇcn`y, J., H. B. McMahan, F. X. Yu, P. Richtárik, A. T. Suresh, and D. Bacon. 2016. “Federated learning: Strategies for improving communication efficiency”. _arXiv preprint arXiv:1610.05492_ . 

- Kreps, J., N. Narkhede, J. Rao et al. 2011. “Kafka: A distributed messaging system for log processing”. In _Proceedings of the NetDB_ , pp. 1–7. 

- Liang, X., S. Shetty, J. Zhao, D. Bowden, D. Li, and J. Liu. 2017. “Towards decentralized accountability and self-sovereignty in healthcare systems”. In _International Conference on Information and Communications Security_ , pp. 387–398. Springer. 

- Liu, Y., G. Sun, and S. Schuckers. 2019. “Enabling Secure and Privacy Preserving Identity Management via Smart Contract”. In _2019 IEEE Conference on Communications and Network Security (CNS)_ , pp. 1–8. IEEE. 

- Mavroeidis, V., and S. Bromander. 2017. “Cyber threat intelligence model: An evaluation of taxonomies, sharing standards, and ontologies within cyber threat intelligence”. In _2017 European Intelligence and Security Informatics Conference (EISIC)_ , pp. 91–98. IEEE. 

- Mühle, A., A. Grüner, T. Gayvoronskaya, and C. Meinel. 2018. “A survey on essential components of a self-sovereign identity”. _Computer Science Review_ vol. 30, pp. 80–86. 

- Paszke, A., S. Gross, F. Massa, A. Lerer, J. Bradbury, G. Chanan, T. Killeen, Z. Lin, N. Gimelshein, L. Antiga et al. 2019. “Pytorch: An imperative style, high-performance deep learning library”. _Advances in neural information processing systems_ vol. 32, pp. 8026–8037. 

- Wadhwani, A., and P. Jain. 2020a. “Machine Learning Model Cards Transparency Review: Using model card toolkit”. In _2020 IEEE Pune Section International Conference (PuneCon)_ , pp. 133–137. IEEE. 

- Wadhwani, A., and P. Jain. 2020b. “Machine Learning Model Cards Transparency Review: Using model card toolkit”. In _2020 IEEE Pune Section International Conference (PuneCon)_ , pp. 133–137. IEEE. 

- Wagner, C., A. Dulaunoy, G. Wagener, and A. Iklody. 2016. “Misp: The design and implementation of a collaborative threat intelligence sharing platform”. In _Proceedings of the 2016 ACM on Workshop on Information Sharing and Collaborative Security_ , pp. 49–56. 

- Wagner, T. D., K. Mahbub, E. Palomar, and A. E. Abdallah. 2019. “Cyber threat intelligence sharing: Survey and research directions”. _Computers & Security_ vol. 87, pp. 101589. 

- Yang, T., G. Andrew, H. Eichner, H. Sun, W. Li, N. Kong, D. Ramage, and F. Beaufays. 2018. “Applied federated learning: Improving google keyboard query suggestions”. _arXiv preprint arXiv:1812.02903_ . 

- Ziller, A., A. Trask, A. Lopardo, B. Szymkow, B. Wagner, E. Bluemke, J.-M. Nounahon, J. PasseratPalmbach, K. Prakash, N. Rose et al. 2021. “PySyft: A Library for Easy Federated Learning”. In _Federated Learning Systems_ , pp. 111–139. Springer. 

**244** 

_Bandara, Shetty, Mukkamala, Rahaman, Liang_ 

## **AUTHOR BIOGRAPHIES** 

**ERANGA BANDARA** worked as a Senior Research Scientist at the Virginia Modeling Analysis and Simulation Center (VMASC) Virginia USA. His research interests include Distributed Systems, Blockchain, Big Data, Actor based Systems and Functional programming. He worked as a Lead Engineer at Pagero AB Sweden. With Pagero AB he was involved with research and developments in Distributed Systems, Functional Programming, Big Data, Actor based systems and DevOps. His email address is cmedawer@odu.edu. 

**SACHIN SHETTY** is an Associate Director in the Virginia Modeling, Analysis and Simulation Center at Old Dominion University and an Associate Professor with the Department of Computational Modeling and Simulation Engineering. Sachin Shetty received his PhD in Modeling and Simulation from the Old Dominion University in 2007. His research interests lie at the intersection of computer networking, network security and machine learning. Recently, he has been involved with developing cyber risk/resilience metrics for critical infrastructure and blockchain technologies for distributed system security. His laboratory has been supported by the National Science Foundation, Air Office of Scientific Research, Air Force Research Lab, Office of Naval Research, Department of Homeland Security, and Boeing. He has published over 150 research articles in journals and conference proceedings and four books. He is the recipient of Commonwealth Cyber Initiative Research Fellow, Fulbright Specialist award, EPRI Cybersecurity Research Challenge award, DHS Scientific Leadership Award and has been inducted in Tennessee State University’s million-dollar club. His email address is sshetty@odu.edu. 

**RAVI MUKKAMALA** is a Professor and Chair of the Department of Computer Science at Old Dominion University, Norfolk, Virginia. He received his Ph.D. in Computer Science from the University of Iowa, Iowa, City. His primary areas of research interests include performance evaluation, distributed systems, and cybersecurity. He has published over 200 papers in refereed journals and conference proceedings in these areas. He is currently working on the privacy and security aspects of blockchain technologies as well as their applicability in different domains. His email address is rmukkama@odu.edu. 

**ABDUL RAHMAN** received Ph.D.s in mathematics and physics from Howard University. His research interests include research in innovative and novel applications of AI and ML to building improved and robust cyber network operations (CNO) capabilities. His email address is abdulrahman@deloitte.com. 

**XUEPING LIANG** is an Assistant Professor in the Department of Information Systems and Supply Chain Management at the University of North Carolina at Greensboro. Before that, she served as a cybersecurity research scientist at Old Dominion University’s Virginia Modeling, Analysis, and Simulation Center and a security research analyst at Tennessee State University’s Tiger Institute. Her main research focus involves security and privacy, trusted computing, distributed architecture, and blockchain. She has worked on several research and application projects using Intel SGX and blockchain, and one of her papers has been awarded the Top 50 Most Influential Papers of Blockchain in 2019. Dr. Liang received her PhD in Cyber Security from the Institute of Information Engineering at the University of Chinese Academy of Sciences in 2019. Her email address is x_liang@uncg.edu. 

**245** 

