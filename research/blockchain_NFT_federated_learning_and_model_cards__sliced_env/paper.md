# Blockchain, NFT, Federated Learning and Model Cards enabled UAV Surveillance System for 5G/6G Network Sliced Environment 

Eranga Bandara _[∗]_ , Sachin Shetty _[∗]_ , Peter Foytik _[∗]_ , Abdul Rahman _[†]_ , Ravi Mukkamala _[∗]_ , Xueping Liang _[‡]_ , Nadini Sahabandu _[§]_ , 

> _∗ {_ cmedawer, sshetty, pfoytik, mukka _}_ @odu.edu Old Dominion University, Norfolk, VA, USA 

> _†_ abdulrahman@deloitte.com Deloitte & Touche LLP 

> _‡_ xuliang@fiu.edu Florida International University, Miami, FL, USA 

> _§_ nadini.sahabandu@goodlifex.com GoodLife X 

_**Abstract**_ **—In recent years, the use of UAVs has expanded to various applications such as surveillance, disaster response, agriculture, and delivery. However, traditional UAV monitoring systems rely on direct communication between the UAV and the ground pilot, which has several limitations such as limited range, poor reliability, and susceptibility to interference. To overcome these limitations, there has been significant interest in integrating UAVs into cellular networks such as 5G/6G network slicing. The flexibility of network slicing allows UAVs to operate on different slices based on their communication needs, which can improve their performance and efficiency. However, integrating UAVs into network slicing also poses several challenges, such as managing communication and permissions of UAVs and base stations, access control of UAVs, and identity management of UAVs. To address these challenges, we propose a blockchain, Non-Fungible Token(NFT), Federated Learning(FL), and ZeroTrust(ZT) security-enabled UAV monitoring platform for 5G/6G network sliced environments. We propose a novel approach in which UAVs are represented as NFT tokens within the platform. This innovative representation allows for enhanced security and trust in the system, aligning with the principles of the Zero-Trust security model, which assumes no implicit trust in any network component or user. Furthermore, we propose a FL system that operates on top of the blockchain, which can analyze data from multiple UAVs across different network slices. Our proposed FL system uses coordinator-less models, which eliminates the attacks of a centralized coordinator. As a use case, we consider a scenario where our proposed system detects anomaly communications of UAVs and identifies attack surfaces via analyzing network traffic data of UAVs using FL. The 5G system testbed implemented with FreedomFi 5G gateway and Indoor Radio Cell.** 

_**Index Terms**_ **—Blockchain, Federated Learning, 5G, 6G, ZeroTrust Security, UAV, Network Slicing.** 

## I. INTRODUCTION 

Unmanned aerial vehicle (UAV) surveillance systems have become increasingly important for real-time and automated data gathering in various domains, such as agriculture, smart factories, and smart cities. These systems utilize sensors attached to UAVs to collect data, which is then transmitted 

to a central location for monitoring and processing. The collected data is then used to build intelligence and predictions using analytics and machine learning algorithms. Figure 1 shows the main functions of the UAV monitoring systems. However, traditional UAV monitoring systems have several limitations, including reliance on simple direct communication between the UAV and the ground pilot over unlicensed spectrums, which are typically low data rate, unreliable, insecure, vulnerable to interference, difficult to monitor and manage, and have very short operation range. To overcome these limitations, there has been significant interest in integrating UAVs into cellular networks, such as 5G/6G network slicing. Network slicing provides a way to run multiple self-contained and autonomous logical networks using a shared physical infrastructure across supported network domains, such as the core network (CN) and radio access network (RAN) [1]. The flexibility of network slicing enables UAVs to operate on different slices based on their communication needs, which can improve their performance and efficiency. 

However, integrating UAVs into 5G/6G network sliced environments presents several challenges, including managing communications and permissions of UAVs and base stations, access control of the UAVs, and identity management of the UAVs. Additionally, network slicing in 5G/6G mobile networks enables billions of connected devices to transmit data at higher rates than ever before, resulting in configuration complexities and complex security management. 

To address these challenges, this paper proposes a novel Blockchain, NFT [2], Federated Learning [3], [4], and Zero Trust security [5], [6]-enabled UAV monitoring platform for 5G/6G network sliced environments. The proposed system uses a sharding-based consensus architecture to deploy a blockchain system in a 5G/6G network sliced environment. The attributes of UAVs are encoded as NFT tokens using a custom schema [7]. This approach ensures that the unique 

Fig. 1: Functions of UAV monitoring system. 

characteristics and properties of each UAV are securely stored and managed within the platform. The identities and communication permissions of the UAVs are managed with blockchain smart contracts using the Zero Trust Security model. The proposed Federated Learning system operates on top of the blockchain using coordinator-less models and can analyze the data of the UAVs across different network slices [8] and build ML models. The generated ML models are saved as Model Card objects [9], [10] in the blockchain ledger. As a use case, we consider a scenario where our proposed system detects anomaly communications of UAVs and identifies attack surfaces via analyzing network traffic data of UAVs using Federated Learning. The 5G system testbed is implemented with FreedomFi 5G gateway and Indoor Radio Cell [11]. The proposed system provides an efficient and secure solution for UAV surveillance in 5G/6G network sliced environments, which can improve the reliability, security, and efficiency of UAVs. The following are the main contributions of this research. 

Fig. 2: Overview of sharding-based blockchain architecture for 5G network sliced environment. 

Fig. 3: Local shard layered architecture. 

- 1) Proposed a novel blockchain, federated learning, and zero-trust security-enabled UAV monitoring platform for 5G/6G network sliced environments, which addresses the challenges of managing UAV communication, permissions, and security in complex environments. 

- 2) Developed a sharding-based consensus architecture for deploying the proposed blockchain system in a 5G/6G network sliced environment, which improves scalability and reduces transaction processing time. 

- 3) Implemented a federated learning system on top of the proposed blockchain platform, which enables coordinator-less models for analyzing data from multiple UAVs across different network slices while preserving privacy. 

- 4) Demonstrated the feasibility of the proposed system with a use case scenario for detecting anomaly communications of UAVs and identifying attack surfaces using Federated Learning. 

The organization of this paper is as follows. Section 2 outlines the platform’s design, Section 3 platform’s functionality, Section 4 presents a summary of previous research, and Section 5 outlines the contributions of this study and provides an outline for future work. 

## II. PLATFORM ARCHITECTURE 

In this study, our platform operates within a 5G/6G networksliced environment, allowing UAVs to adapt to different network slices based on their communication requirements. This is achieved through the deployment of a blockchain shard within each 5G network slice. These individual shards enable independent local consensus, with the potential for global consensus depending on connectivity. Each local shard comprises multiple blockchain nodes, functioning as a selfcontained blockchain network. The architecture of these local shards consists of five primary layers: the Blockchain storage layer, Smart contract layer, Federated learning layer, Off-chain storage layer, and 5G core network layer, as depicted in Figure 3. Local consensus is facilitated through Apache Kafka [12], [13], while global consensus relies on Raft [14]. The networksliced configuration of our platform is illustrated in Figure 2. In the subsequent sections, we will provide a detailed exploration of each of these layers. 

## _A. Blockchain Storage Layer_ 

The blockchain storage layer is a critical component of the proposed platform as it is responsible for storing various types 

handles the incoming sensor data from the UAV sensor devices. Once the data is received, the sensor contract validates it and stores it in the off-chain storage. The model contract is responsible for the federated learning functions, including model initialization, model parameter sharing, and model averaging. Additionally, it handles the generation of Model Card objects, which contain information about the generated ML models and their provenance. Finally, the permission contract manages the Zero Trust policy enforcement, encoded in the form of Zero Trust Security rules, and stores them in the blockchain ledger. With the use of these smart contracts, the platform ensures secure and transparent data exchange between UAVs and the network. 

## _C. Federated Learning Layer_ 

Fig. 4: i-528 NFT token schema to represent the UAV. 

of data related to UAV monitoring. This layer ensures the properties of being tamper-proof, transparent, and auditable of the stored data. The blockchain storage layer stores the identities of UAV sensor devices, Zero Trust Security policy rules [15], Federated ML models, and ML model data provenance information, such as participating peers, model parameters, and training information. The attributes of UAVs are represented as NFT tokens using a custom schema(i-528), as illustrated in Figure 4. This custom schema enables the encoding and organization of UAV attributes in a structured and standardized manner, ensuring consistency and interoperability within the platform. To ensure the transparency and traceability of the Federated Learning process, the ML model’s provenance information is stored in the form of Model Card objects [9] on the blockchain ledger. This way, the model’s details, including its performance, can be easily audited. The communication rules and permissions of the UAV sensor devices are defined through the Zero Trust Security policy rules. All the communications of the sensor devices are verified using the policy engine with Zero trust security architecture. This ensures that only authorized devices can access the data. The blockchain storage layer’s tamper-proof nature ensures that the data is secure and cannot be altered without proper authorization, making it a critical component of the proposed platform [16]. 

## _B. Smart Contract Layer_ 

The smart contract layer of the platform is responsible for implementing the ledger functions using four smart contracts. The identity contract manages the UAV sensor device identity, including registration and identity lookup. The sensor contract 

The Federated Machine Learning (FML) service plays a vital role in the proposed platform as it enables blockchain peers to take part in the federated learning process. The FML service consists of several stages—creating the FL pipeline, generating the initial model, generating local models, and producing the global model. The smart contracts on the blockchain work together with the verification service’s functions to initiate the FML process. When a request to train an ML mode is received, the FML service creates the local model using data from the peer’s off-chain storage. After the local models are produced, the Federated Machine Learning (FML) service is requested to construct blocks on the blockchain and apply model averaging algorithms such as Stochastic Gradient Descent(SGD) to aggregate the local models into the global model. This approach allows the platform to develop highquality models while keeping the privacy of the UAV sensor devices that participate in the process intact. 

## _D. Off-Chain Storage Layer_ 

The federated learning process of the platform utilizes offchain storage to store the UAV sensor data [17]. To ensure data privacy, each blockchain peer has its own off-chain storage to store and access sensitive information. The federated learning process present in each blockchain node builds the local machine learning models using the data stored in its off-chain storage. The off-chain storage ensures data security by avoiding direct storage of sensitive information on the blockchain ledger. Instead, the data is hashed and published on the ledger, making data accessible on the off-chain only to authorized parties. Moreover, the off-chain storage system ensures faster data access as the data is stored locally and can be retrieved more efficiently than the data stored on the blockchain ledger. 

## _E. 5G Core Network Layer_ 

The Network Functions (NF) available in the 5G/6G core network layer include session management, authentication, policy control, and data storage. The architecture of the 5G/6G core network is designed with a microservices-based approach, utilizing Network Function Virtualization and Software Defined Networking [18]. The microservices-based design patterns enable the implementation of various data services and 

network-sliced environment. The platform consists of four main functions: UAV sensor identity registration, zero-trust policy enforcement, federated model training, and UAV data monitoring. The system aims to ensure the integrity and confidentiality of the data by enforcing a zero-trust policy and storing information in a blockchain ledger. Federated machine learning is employed to enable collaborative model training among multiple UAVs, without the need to share raw data. Overall, the platform provides a secure and scalable solution for federated machine learning using UAV sensor data. We now discuss more details about these functions. 

## _A. UAV and Sensor Identity Registration_ 

Fig. 5: Zero Trust Security policy enforcement. 

The platform supports a diverse range of UAV devices equipped with various sensors. To effectively utilize these devices, the initial step involves onboarding their sensor identities onto the platform. During the registration process, essential details about each sensor, including the Device Name, Device Address, Communication Port, and Communication Protocol, are collected. These identity registration functions are handled by the sensor smart contract, ensuring the secure storage of information on the blockchain ledger. To enhance user experience, a user-friendly web-based application is provided to facilitate a seamless registration process. Once the sensor identities are successfully registered, they are granted trusted status, enabling active participation in data collection and sharing activities. Additionally, the platform captures and represents UAV data using the i-528 custom NFT schema. This comprehensive approach ensures the secure storage and effective management of the unique characteristics and properties associated with each UAV within the platform. 

## _B. Zero Trust Policy Enforcement_ 

Fig. 6: Model card generated in the platform. 

requirements of the 5G/6G core network as cloud-native applications. The Network Slice Selection Function (NSSF) is responsible for managing the network slicing functions of the 5G/6G core network, which enables the provision of different levels of quality of service (QoS) for different network slices [19], [20]. By utilizing network slicing, the 5G/6G core network can meet the specific requirements of the UAVs and their applications, such as low latency, high bandwidth, or high reliability. The 5G/6G core network layer plays a crucial role in providing efficient and reliable communication for the UAVs in the platform. 

## III. PLATFORM FUNCTIONALITY 

The system is designed to enable secure and efficient federated machine learning using UAV sensor data in a 

The Zero Trust policy enforcement function of the platform is responsible for ensuring secure communication between UAV sensor devices. This is achieved through the use of a rule engine that applies Zero Trust security principles to control the communication between the devices as shown in Figure 5. The rules and permissions that govern the communication (such as allowed ports and endpoints) are encoded as Zero Trust security rules and stored in the blockchain ledger. The platform enables the registration of policies for sensors during their onboarding process, and a web-based application is available for managing and defining these policies. To ensure that the communications of UAV sensors are secure and authorized, the rule engine continuously verifies their communications against the predefined Zero Trust Security rules. This provides a robust and secure framework for communication between the UAV sensor devices in the network [15]. 

## _C. Federated Model Training_ 

The platform enables privacy-preserving Federated Learning, which allows the different UAVs to connect to the blockchain network in different network slices in the 5G environment, as shown in Figure 2. The UAVs continuously send 

## IV. IMPLEMENTATION, TESTBED SETUP AND PERFORMANCE EVALUATION 

The testbed for the system has been established on a 5G network, as shown in Figure 8 which includes the deployment of the blockchain and related services on the public AWS cloud environment. The Moose blockchain [22]–[24] has been used to implement ledger functions, and Aplos smart contract [25], [26] framework has been used to implement smart contracts. Each local shard blockchain node contains two services 1) offchain storage service, and 2) federated model training service , as shown in Figure 7. Raspberry Pi devices have been utilized to simulate the UAV sensors, which communicate with the services through the 5G network. To set up the 5G network infrastructure, the FreedomFi 5G gateway and Indoor Radio Cell have been used [11]. 

Fig. 7: Blockchain system implementation (with local shards) 

sensor streaming data, which is handled by the blockchain smart contract and saved in off-chain storage for analytics. The Federated Learning system in the platform can build machine learning (ML) models using the data stored in offchain storage. For instance, a ML model can be built to detect anomalous communications of the UAVs’ sensor data and identify attack surfaces by analyzing network traffic data using Federated Learning. Each blockchain shard builds ML models with the data stored in its local off-chain storage. These models are then aggregated into the global model using the Federated Learning approach [21]. The data provenance information of the ML model is encoded into Model Cards and stored in the blockchain ledger, as shown in Figure 6. Any party can verify the ML model data provenance information and check the validity of the model using the Model Cards. 

## _D. UAV Data Monitoring_ 

The function of UAV data monitoring in this system is to continuously monitor the sensor data streams coming from the UAVs in real-time. The sensor data is stored in offchain storage and can be validated using the ML models that have been built and trained by the federated learning system in the platform. These ML models are saved in the off-chain storage of the blockchain ledger, and their data provenance information is stored in the Model cards. Once the ML models are built, they can be integrated into the blockchain smart contracts to perform real-time validation of the sensor readings. Consider a scenario where an ML model has been built to detect anomalous communications of UAVs and identify attack surfaces by analyzing network traffic data using federated learning. When real-time sensor data comes from the UAVs, the smart contract will call the ML model functions to decide whether the sensor reading is valid or not based on the pre-defined communication rules/policies of the Zero Trust policy enforcement. In this way, we can prevent various attack surfaces of the UAV sensor readings and ensure the integrity of the sensor data used in the system. 

We have implemented a use case that demonstrates the capabilities of the proposed system in detecting anomalous communications of UAVs and identifying potential attack surfaces by analyzing their network traffic data using Federated Learning. For this use case, a Gated Recurrent Unit (GRU) neural network [27], [28] has been used to develop models with IoT datasets. To accomplish this, we have established a blockchain network among various peers in the shards, where each peer node possesses a local IoT dataset. The GRU algorithm is applied to each local dataset to construct a local model. Federated Learning is utilized to average the local models into a global model. The platform has been assessed based on the performance of this use case. 

**Evaluation of Federated Model Accuracy and Training Loss** : In this study, we evaluated the accuracy and training loss of the federated machine learning model. The process of federated learning was carried out through multiple iterations to obtain a precise model. Our experiments used 1000 iterations for the federated model training. In each iteration, we deployed the current federated model on each peer. The peer used its local data and the deployed model to retrain its local model. With each iteration, the local models were improved, and their accuracy and model loss were computed based on the local data. The local accuracies and losses were aggregated to calculate the total accuracy and loss. The results of this evaluation are presented in Figure 9 and Figure 10 for accuracy and training loss, respectively. 

**Block generation time** . The efficiency of the proposed blockchain system is influenced by the block generation time and block size. We analyzed the time required to construct a block with 10,000 transactions, and it was observed that it took 8.5 seconds, as illustrated in Figure 11. Moreover, we studied the block generation time in different scenarios by altering the number of blockchain peers in the cluster, as depicted in Figure 12. We conducted experiments with different peer groups, and the average block creation time was measured. We repeated each experiment 100 times and plotted the average values. Additionally, the average block generation time was calculated for different block sizes, i.e., for transaction sets of 2,000, 6,000, and 10,000. The experiment was also repeated 

Fig. 10: Federated model training loss in different peers 

Fig. 8: System testbed architecture with FreedomFi 5G 

tionally, there are blockchain-driven marketplaces for trading machine learning models, exemplified by TMLC [31] and BAFFLE [34]. 

## VI. CONCLUSIONS AND FUTURE WORK 

Fig. 9: Accuracy for various peers in the federated model. 

100 times. 

## V. RELATED WORK 

Federated machine learning is a rapidly growing area of research with a wide range of applications. Several blockchainbased federated machine learning systems have been proposed that aim to provide data privacy, computation auditability, and incentives for participants. Our proposed platform is compared with the features of these projects and summarized in Table I. 

DeepChain [29] and BlockFL [32] are dedicated to safeguarding privacy in machine learning. They leverage blockchain technology to ensure data confidentiality, especially for sensitive patient health information. Breaking Bad [30] specializes in deanonymization using machine learning, while Chained-FL [36] focuses on intrusion detection. ModelChain [33] and Poster [35] are prominent privacypreserving machine learning frameworks in the healthcare sector. ModelChain enables collaborative machine learning across healthcare organizations while strictly preserving patient data privacy through a private blockchain network. Poster, on the other hand, acts as a federated learning platform and logs various data on the blockchain, ensuring data provenance and a trust foundation for healthcare collaborations. Addi- 

The paper presents a novel approach to deploy a blockchain system in a 5G/6G network sliced environment using shardingbased consensus architecture. The proposed system integrates blockchain, NFT, Zero Trust Security, and Federated Learning to develop a UAV monitoring platform. UAVs can operate on different network slices based on their communication requirements, with their identities and communication permissions managed by blockchain smart contracts using the Zero Trust Security model. The Federated Learning system operates on top of the cross-shard blockchain using coordinator-less models, and it eliminates the role of the central coordinator by supporting a decentralized federated learning environment. The ML models of the platform are saved as Model Card objects in the blockchain ledger, providing enhanced transparency and provenance with the federated learning process. The system can be used to detect anomaly communications of the UAVs and identify attack surfaces by analyzing network traffic data of the UAVs through Federated Learning. Future work includes integrating Tensorflow-Federated libraries into the platform. Overall, the proposed system provides a feasible 

Fig. 11: Block creation time against the number of transactions in the block. 

TABLE I: Blockchain-enabled decentralized learning platform comparison 

**==> picture [497 x 130] intentionally omitted <==**

**----- Start of picture text -----**<br>
|||||||||||
|---|---|---|---|---|---|---|---|---|---|
|Running|Application|Zero-Trust|Federated|Auditing|&|Model|Real-Time|
|Platform|
|Blockchain|Domain|Support|Learning|Provenance|Reporting|Predictions|
|UAV|Moose|5G|✓|✓|✓|✓|✓|
|DeepChain|[29]|Corda|Any|✗|✓|✓|✗|✗|
|Breaking|Bad|[30]|Bitcoin|Cryptocurrency|✗|✗|✗|✗|✗|
|TMLC|[31]|Ethereum|N/A|✗|✗|✗|✗|✗|
|BlockFL|[32]|N/A|Mobile|✗|✓|✓|✗|✗|
|ModelChain|[33]|ModelChain|Healthcare|✓|✗|✗|✗|✗|
|BAFFLE|[34]|Ethereum|Any|✗|✓|✗|✗|✗|
|Poster|[35]|Ethereum|N/A|✗|✓|✓|✗|✗|
|Chained-FL|[36]|MultiChain|Any|✗|✓|✓|✗|✗|

**----- End of picture text -----**<br>


- [7] E. Bandara, X. Liang, S. Shetty, R. Mukkamala, A. Rahman, and N. W. Keong, “Indy528—federated learning model tokenization with nonfungible tokens (nft) and model cards,” in _2022 IEEE 19th International Conference on Mobile Ad Hoc and Smart Systems (MASS)_ . IEEE, 2022, pp. 195–201. 

@’ 

- [8] Y. Lu, X. Huang, K. Zhang, S. Maharjan, and Y. Zhang, “Blockchain empowered asynchronous federated learning for secure data sharing in internet of vehicles,” _IEEE Transactions on Vehicular Technology_ , vol. 69, no. 4, pp. 4298–4311, 2020. 

- [9] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

Fig. 12: Block generation time with the different number of peers in the cluster. 

- [10] E. Bandara, S. Shetty, A. Rahman, and R. Mukkamala, “Let’strace—blockchain, federated learning and tuf/in-toto enabled cyber supply chain provenance platform,” in _MILCOM 2021-2021 IEEE Military Communications Conference (MILCOM)_ . IEEE, 2021, pp. 470–476. 

- [11] FreedomFi, “Freedomfi 5g.” [Online]. Available: https://freedomfi.com/ 

solution to monitor UAVs in a secure, decentralized, and efficient manner in a 5G/6G network sliced environment. 

## ACKNOWLEDGEMENTS 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] M.-A. Kourtis, T. Anagnostopoulos, S. Kukli´nski, M. Wierzbicki, A. Oikonomakis, G. Xilouris, I. P. Chochliouros, N. Yi, A. Kostopoulos, T. Sarlas _et al._ , “5g network slicing enabling edge services,” in _2020 IEEE Conference on Network Function Virtualization and Software Defined Networks (NFV-SDN)_ . IEEE, 2020, pp. 155–160. 

- [2] Q. Wang, R. Li, Q. Wang, and S. Chen, “Non-fungible token (nft): Overview, evaluation, opportunities and challenges,” _arXiv preprint arXiv:2105.07447_ , 2021. 

- [3] J. Koneˇcn`y, H. B. McMahan, F. X. Yu, P. Richt´arik, A. T. Suresh, and D. Bacon, “Federated learning: Strategies for improving communication efficiency,” _arXiv preprint arXiv:1610.05492_ , 2016. 

- [4] E. Bandara, X. Liang, S. Shetty, R. Mukkamala, A. Rahman, and N. W. Keong, “Skunk—a blockchain and zero trust security enabled federated learning platform for 5g/6g network slicing,” in _2022 19th Annual IEEE International Conference on Sensing, Communication, and Networking (SECON)_ . IEEE, 2022, pp. 109–117. 

- [5] C. Benza¨ıd, T. Taleb, and M. Z. Farooqi, “Trust in 5g and beyond networks,” _IEEE Network_ , 2021. 

- [6] E. Bandara, X. Liang, P. Foytik, S. Shetty, and K. De Zoysa, “A blockchain and self-sovereign identity empowered digital identity platform,” in _2021 International Conference on Computer Communications and Networks (ICCCN)_ . IEEE, 2021, pp. 1–7. 

- [12] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

- [13] J. Kreps, N. Narkhede, J. Rao _et al._ , “Kafka: A distributed messaging system for log processing,” in _Proceedings of the NetDB_ , 2011, pp. 1–7. 

- [14] D. Huang, X. Ma, and S. Zhang, “Performance analysis of the raft consensus algorithm for private blockchains,” _IEEE Transactions on Systems, Man, and Cybernetics: Systems_ , vol. 50, no. 1, pp. 172–181, 2019. 

- [15] E. Bandara, S. Shetty, R. Mukkamala, X. Liang, P. Foytik, N. Ranasinghe, and K. De Zoysa, “Casper: a blockchain-based system for efficient and secure customer credential verification,” _Journal of Banking and Financial Technology_ , vol. 6, no. 1, pp. 43–62, 2022. 

- [16] E. Bandara, X. Liang, S. Shetty, R. Mukkamala, P. Foytik, N. Ranasinghe, and K. De Zoysa, “Octopus: privacy preserving peer-to-peer transactions system with interplanetary file system (ipfs),” _International Journal of Information Security_ , vol. 22, no. 3, pp. 591–609, 2023. 

- [17] R. Kumar, N. Marchang, and R. Tripathi, “Distributed off-chain storage of patient diagnostic reports in healthcare system using ipfs and blockchain,” in _2020 International Conference on COMmunication Systems & NETworkS (COMSNETS)_ . IEEE, 2020, pp. 1–5. 

- [18] G. Brown, “Service-based architecture for 5g core networks,” _A Heavy Reading white paper produced for Huawei Technologies Co. Ltd. Online verf¨ugbar unter: https://www. huawei. com/en/pressevents/news/2017/11/HeavyReading-WhitePaper-5G-Core-Network, letzter Zugriff am_ , vol. 1, p. 2018, 2017. 

- [19] V. K. Choyi, A. Abdel-Hamid, Y. Shah, S. Ferdi, and A. Brusilovsky, “Network slice selection, assignment and routing within 5g networks,” in _2016 IEEE Conference on Standards for Communications and Networking (CSCN)_ . IEEE, 2016, pp. 1–7. 

- [20] E. Bandara, S. Shetty, R. Mukkamala, A. Rahman, P. Foytik, X. Liang, and N. W. Keong, “Kaputa-blockchain, non-fungible token and model card integrated 5g/6g network slice broker and marketplace,” in _MILCOM 2022-2022 IEEE Military Communications Conference (MILCOM)_ . IEEE, 2022, pp. 559–564. 

- [21] E. Bandara, S. Shetty, A. Rahman, R. Mukkamala, J. Zhao, and X. Liang, “Bassa-ml—a blockchain and model card integrated federated learning provenance platform,” in _2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC)_ . IEEE, 2022, pp. 753–759. 

- [22] E. Bandara, S. Shetty, A. Rahman, R. Mukkamala, and X. Liang, “Moose: A scalable blockchain architecture for 5g enabled iot with sharding and network slicing,” in _2022 IEEE Wireless Communications and Networking Conference (WCNC)_ . IEEE, 2022, pp. 1194–1199. 

- [23] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

- [24] E. Bandara, W. K. Ng, K. D. Zoysa, N. Fernando, S. Tharaka, P. Maurakirinathan, and N. Jayasuriya, “Mystiko - blockchain meets big data,” in _IEEE International Conference on Big Data, Big Data 2018, Seattle, WA, USA, December 10-13, 2018_ , 2018, pp. 3024–3032. 

- [25] E. Bandara, W. K. NG, K. De Zoysa, and N. Ranasinghe, “Aplos: Smart contracts made smart,” _BlockSys’2019_ , 2019. 

- [26] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa, and W. K. Ng, “Saas-microservices-based scalable smart contract architecture.” 

- [27] R. Dey and F. M. Salem, “Gate-variants of gated recurrent unit (gru) neural networks,” in _2017 IEEE 60th international midwest symposium on circuits and systems (MWSCAS)_ . IEEE, 2017, pp. 1597–1600. 

- [28] R. Sathya and A. Abraham, “Comparison of supervised and unsupervised learning algorithms for pattern classification,” _International Journal of Advanced Research in Artificial Intelligence_ , vol. 2, no. 2, pp. 34–38, 2013. 

- [29] J. Weng, J. Weng, J. Zhang, M. Li, Y. Zhang, and W. Luo, “Deepchain: Auditable and privacy-preserving deep learning with blockchain-based incentive,” _IEEE Transactions on Dependable and Secure Computing_ , 2019. 

- [30] M. A. Harlev, H. Sun Yin, K. C. Langenheldt, R. Mukkamala, and R. Vatrapu, “Breaking bad: De-anonymising entity types on the bitcoin blockchain using supervised machine learning,” in _Proceedings of the 51st Hawaii International Conference on System Sciences_ , 2018. 

- [31] A. B. Kurtulmus and K. Daniel, “Trustless machine learning contracts; evaluating and exchanging machine learning models on the ethereum blockchain,” _arXiv preprint arXiv:1802.10185_ , 2018. 

- [32] H. Kim, J. Park, M. Bennis, and S.-L. Kim, “On-device federated learning via blockchain and its latency analysis,” _arXiv preprint arXiv:1808.03949_ , 2018. 

- [33] T.-T. Kuo and L. Ohno-Machado, “Modelchain: Decentralized privacypreserving healthcare predictive modeling framework on private blockchain networks,” _arXiv preprint arXiv:1802.01746_ , 2018. 

- [34] P. Ramanan, K. Nakayama, and R. Sharma, “Baffle: Blockchain based aggregator free federated learning,” _arXiv preprint arXiv:1909.07452_ , 2019. 

- [35] S. Awan, F. Li, B. Luo, and M. Liu, “Poster: A reliable and accountable privacy-preserving federated learning framework using the blockchain,” in _Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security_ , 2019, pp. 2561–2563. 

- [36] D. Preuveneers, V. Rimmer, I. Tsingenopoulos, J. Spooren, W. Joosen, and E. Ilie-Zudor, “Chained anomaly detection models for federated learning: An intrusion detection case study,” _Applied Sciences_ , vol. 8, no. 12, p. 2663, 2018. 

