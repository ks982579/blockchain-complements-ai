2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

# — Bassa-ML A Blockchain and Model Card Integrated Federated Learning Provenance Platform 

Eranga Bandara _[∗]_ , Sachin Shetty _[∗]_ , Abdul Rahman _[†]_ , Ravi Mukkamala _[∗]_ , Juan Zhao _[‡]_ , Xueping Liang _[§]_ , 

> _∗ {_ cmedawer, sshetty, mukka _}_ @odu.edu Old Dominion University, Norfolk, VA, USA 

> _†_ abdul@vt.edu 

Hume Center for National Security and Technology, Commonwealth Cyber Initiative (CCI), Virginia Polytechnic University, Arlington, Virginia USA 

> _‡_ juan.zhao@vanderbilt.edu Vanderbilt University Medical Center, Nashville, Tennessee USA 

> _§_ x liang@uncg.edu University of North Carolina at Greensboro, NC, USA 

_**Abstract**_ **—Federated learning is a collaborative/distributed machine learning system which is designed to address the privacy issues in centralized machine learning systems. The transparency and provenance of a machine learning model are important aspects of federated learning systems since they impact peoples’ lives in various domains (e.g., from healthcare to personal finance to employment). However, most of the existing federated learning systems deal with centralized coordinators which are vulnerable to attacks and privacy breaches. Also, they do not provide any standard transparency and provenance mechanisms for the resulting models. In this paper, we propose a blockchain and Model Card-based integrated federated learning system “BassaML” providing enhanced transparency and trust for the models. Model parameter sharing, local model generation, model averaging, and model sharing functions are implemented using smart contracts. The generated models, model training information, and model reports are stored in the blockchain ledger as Model Card Objects. This results in enhanced transparency and auditability to the federated learning process.** 

_**Index Terms**_ **—Blockchain; Machine Learning; Federated Learning; Model Card; Big Data** 

## I. INTRODUCTION 

Federated learning (FL) is a new research area that uses a collaborative/distributed machine learning (ML) model [1]. Most of the existing FL systems possess the following traits: use of a centralized coordinator to aggregate the local ML models [2]; typically quite vulnerable to attacks and privacy breaches [3], [4]; and resulting ML model transparency and provenance are not often satisfactory [5]. In this paper, we are proposing Bassa-ML, a blockchain and Model Card [6], [7] integrated federated learning [1], [8] platform to address these issues. It is a blockchain-based coordinator-less federated learning system with clear model transparency and provenance in a decentralized federated learning environment [9], [10]. Bassa-ML’s blockchain network can be deployed among multiple peers that are participating in FL (e.g., multiple hospitals, banks, government organizations). Each peer in the blockchain manages its own off-chain storage, storing real data, controlled and held by the peer. Each peer in the 

Bassa-ML blockchain network can establish supervised or unsupervised machine learning models [11] (known as local models) with the existing data on its own off-chain storage. When generating blocks in the blockchain network, these local models are aggregated/averaged into the global model using a federated learning approach. 

The model parameter sharing, local model generation, model averaging, model storing, and sharing functions are implemented with smart contracts of the Bassa-ML blockchain platform. Bassa-ML stores the generated ML models (local and global models), their information, and model reports in the blockchain ledger as Model Card [6], [7] objects. The Model Card object in Bassa-ML contains model information (e.g., participating clients, the generators of local models, the aggregators, the model generation times, etc.) and quantitative analysis and considerations about the model. The Model Card objects stored in the blockchain ledger can be viewed by all participating parties. It provides a means to audit the system and adds more transparency to the (FL) process. All actions that are performed on the ML model are completely traceable by each user giving a clear history of all operations and incremental versions that existed. Once ML models are generated, these models can be integrated into blockchain smart contracts to do predictions based on real-time data. In this work, we pursue incorporating Model Cards as they add more trust to blockchain applications while preventing attacks such as misrepresented or spoofed machine learning model iterations. 

As a use-case of the Bassa-ML platform, we have discussed exploring the integration of FL into the healthcare domain. The blockchain network is deployed in multiple hospitals where each has its own data set. In particular, we have used a data set about inflammations of the bladder in different blockchain peers and trained a model to diagnose acute inflammations of the bladder [12]. An FL pipeline is built and averaged as a global model with the data from different hospitals in the network. The evaluation of the Bassa-ML platform has been 

753 

978-1-6654-3161-3/22/$31.00 ©2022 IEEE 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

performed based on this use case. The following are the main contributions from Bassa-ML. 

- 1) Blockchain-enabled coordinator-less federated learning system has been proposed. 

- 2) The ML model parameter sharing, model training and averaging functions are implemented with blockchain smart contracts. 

- 3) Enhanced transparency and provenance to the federated learning process by storing model information and model reports in the blockchain ledger as Model Card objects. 

- 4) Provided a mechanism to integrate machine learning models with blockchain smart contracts for predictions on real-time data. 

- 5) Bassa-ML platform has been integrated with healthcare domain with diagnosis of acute inflammations of bladder as a use case with the federated learning approach. 

The paper is organized as follows. Section 2 discusses the background information. Section 3 discusses the architecture of the Bassa-ML platform. Section 4 discusses the federated learning process of the Bassa-ML platform. Section 5 performance evaluation of Bassa-ML. Section 6 related work. Finally, Section 7 concludes the Bassa-ML platform with suggestions for future work. 

Fig. 1: Bassa-ML platform architecture. 

## _C. Federated Learning (FL)_ 

## II. BACKGROUND 

This section provides a brief summary of each of the technologies underlying Bassa-ML. In particular, we discuss the following concepts: Blockchain, Smart Contract, Federated Learning and Model Cards. 

## _A. Blockchain_ 

Blockchain maintains a tamper-evident shared digital ledger that records data in a public or private peer-to-peer network. It guarantees a decentralized trust system without involving trusted third parties. Multiple partners (nodes) can exist in the blockchain network and each partner (node) has a copy of the data being maintained. The data on the blockchain is organized into blocks. A block contains a set of records (transactions). The records (transactions) are organized into a Merkle tree where the leaves are the records and each intermediate node stores the hash value of the child nodes [13]. 

## _B. Smart Contracts_ 

A smart contract is akin to a trusted middle-person situated between blockchain clients and the blockchain storage which enables higher-level functions. Client requests can be directed to smart contracts that execute the logic needed to provide a complex service, such as managing application state, enforcing governance, or verifying credentials. Using smart contracts, users do not need to perform queries to access data from the blockchain. Instead, smart contracts provide a programming interface so that applications can interact with the underlying blockchain nodes [14]. 

FL is a collaborative machine learning technique which is designed to address the privacy issues of the centralized machine learning system [2]. FL trains central models on decentralized data. This helps preserve the privacy of data on various peers as only the weight updates are shared with the centralized model while the data remains on each peer and we can still train a model using that data. 

## _D. Model Cards_ 

Model cards can provide a structured framework for reporting on ML model provenance, model usage, ethics-informed evaluation and a detailed overview of a model’s suggested uses and limitations that developers, regulators, and downstream users alike can benefit from. The Model Card Toolkit, which is provided by the Google TensorFlow framework [7], [15], can be used to generate Model Card objects. 

## III. BASSA-ML ARCHITECTURE 

The architecture of the Bassa-ML platform is discussed in Figure 1. The proposed Bassa-ML platform contains three main components: Blockchain ledger, Federated Machine Learning (FML) service, and Model Card service. The blockchain network is deployed at multiple peers (e.g., multiple hospitals). Each blockchain node contains a federated learning service and Model Card service. We now look at the details of each of these components. 

## _A. Blockchain Ledger_ 

The blockchain network can be deployed at multiple entities in the federated learning environment. Each entity in the 

754 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

network can run its own blockchain node (Figure 1). As discussed earlier in this paper, the Blockchain ledger stores peer details of the federated learning environment and ML model information. The ML model information is encoded into Model Card objects which are maintained in the blockchain ledger. 

The FL functions in the Bassa-ML are implemented with blockchain smart contracts. There are mainly four smart contracts in the Bassa-ML: Identity contract, Model contract, Supervisor contract and Model Card contract. The Identity contract is responsible for the identity management of the entities (e.g., hospitals) in the federated learning environment. The Model contract manages local model generation by the peers. Peers generate local models with the existing data on their own off-chain storage. Supervisor contracts handle model initialization and model averaging functions. In model averaging, it takes the stochastic gradient distribution (SGD) values of the local models and aggregates them into a global model when generating blocks in the blockchain [16]. Model card contract encodes ML model information (e.g., generated model, aggregated model, etc.) into Model Card objects and stores them in the blockchain ledger. 

## _B. Federated Machine Learning Service_ 

Each blockchain peer comes with its own Federated Machine Learning (FML) service. These are implemented in the following stages: FL pipeline creation, initial model generation, local model generating, and global model creation with the model averaging function. Blockchain smart contracts interact with the functions implemented in the verification service. For example, when a request is received to train an ML model, the smart contract interacts with its FML service and generate the local model with the data stored in its peer’s off-chain storage. When generating blocks in the blockchain, smart contracts interact with the FML service and average the local models into the global model using the Stochastic Gradient Descent (SGD) method. 

## _C. Model Card Service_ 

The Model Card service handles the Model Card object generation functions that contain model information (e.g., participating clients with local models, peers responsible for aggregating the models, model generation times, etc.), quantitative analysis of the model and considerations about the model. Model card services encode ML model information into Model card objects which are stored in the blockchain ledger. The smart contracts in the blockchain interact with the Model Card Services to generate the model cards of the ML models (e.g, local models and global models). 

## IV. BASSA-ML FUNCTIONALITY 

In this section, we describe some of the primary functions offered by the Bassa-ML platform. 

**Algorithm 1:** Training pipeline initialization 

- **1 INITIALIZE TRAINING PIPELINE: 2** Find available blockchain peers _p_ (1 _, ..., n_ ) in the network 

- **3** Use the underlying blockchain consensus algorithm to choose a leader peer _pi_ 

- **4** Define initial ML model, training parameters and any algorithm-related information 

## **5 GENERATE INITIAL BLOCK:** 

- **6** Create initial block _bi_ with initial ML model, training parameters, and algorithm-related information 

- **7** Save _bi_ in ledger and broadcast it to other peers in the network 

## _A. Identity Registration_ 

Multiple peers are collaborating in the FL environment in Bassa-ML. The peers can be organizations such as hospitals, banks, or private companies. In this paper, we have demonstrated deploying Bassa-ML on a hospital network. Consider a scenario where five hospitals A, B, C, D, and E are in the blockchain network. These hospitals have their own data storage (off-chain storage) with individual health data. The hospitals collaboratively work for building ML models in a FL approach with the data stored in their own off-chain storage. 

First, the peer entities need to be onboarded to the platform. When these peers are onboarded, separate blockchain nodes will be deployed for each of them. All the identity information of the peers are stored in the blockchain ledger. This identity onboard function is implemented in the identity smart contract in the blockchain. 

## _B. Federated Learning Pipeline Initialization_ 

In FL, first, the ML pipeline needs to be initialized with model parameters. Then each peer will train a new ML model with their own data set. Finally, these generated models will be aggregated/averaged into a global model. In the Bassa-ML federated learning environment, the training pipeline initialization will be done when generating the initial block. The initial block will be generated by the leader peer chosen by the blockchain consensus algorithm [13], [17]. The initial ML model and model parameters are stored and added as blockchain transactions. Finally, the generated block will be broadcast to other peers in the network. If we assume that peer E is chosen as the leader to generate the initial block in our healthcare care scenario, it will create the block with the initial ML model and model parameters as shown in Figure 2. The federated learning pipeline initialization functionality is discussed in Algorithm 1. 

## _C. Local Model Generation_ 

Once the leader peer publishes the block with the initial model and model parameters, other peers (in the abovementioned scenario peers A, B, C, D) take the block and start 

755 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

Fig. 2: Bassa-ML federated learning pipeline initialization. 

Fig. 3: Bassa-ML federated learning model averaging. 

## **Algorithm 2:** Local model training 

**1** Wait till publishing initial block _bi_ 

## _D. Global Model Aggregation_ 

## **2 for** _each peer p_ = 1 _, ..., n_ **do** 

- **3 LOCAL MODEL TRAINING:** 

- **4** Fetch initial block _bi_ from the ledger and get the ML model and training parameters 

- **5** Train the model with the data in the off-chain storage 

- **6** Generate Model Card with local model information 

- **7 PUBLISH LOCAL MODEL UPDATES:** 

- **8** Create transaction _ti_ with Model Card information and local model parameters 

- **9** Publish _ti_ to the ledger 

- **10 end** 

to train the ML model according to the model parameters. Other peers will build a new ML model with the data on their own off-chain storage. The built model and the model information are saved in the blockchain ledger as Model Card objects. The Model Card Object contains model information (e.g., participating clients, generator of local models, the aggregator peer, time of model generation, etc.), quantitative analysis of the model, and considerations about the model. Model Card service in each blockchain peer encodes ML model information into Model Card objects and saves them in the ledger. The local model generation process is summarized in Algorithm 2. 

Once peers generate the local models with the local data set, these models will be aggregated/averaged into a global model. Unlike traditional federated learning systems, blockchain-enabled Bassa-ML does not have a centralized coordinator/leader to aggregate the ML models. Models will be averaged by a peer when generating a block. The block generating peer will be selected by the consensus algorithm of the blockchain. The block generating peer will take the local model information (which is stored in the ledger as Model Card objects) of each peer and average them according to the stochastic gradient descent (SGD) algorithm. Then it encodes the final ML model information into the Model Card object and saves them in the block. Figure 4 shows the structure of an example Model Card object generated by the BassaML platform. Finally, the block with the global model will be broadcast to other peers in the network as depicted in Figure 3. The global model averaging process is discussed in Algorithm 3. 

## V. BASSA-ML IMPLEMENTATION AND EVALUATION 

Basa-ML is implemented on the Rahasak blockchain – a highly scalable blockchain for enterprise applications. The smart contracts are implemented with Rahasak blockchain’s Aplos smart contract platform [14], [18]. The FML service is implemented with Pytorch [19] and Pysyft [20] based python libraries. The Model Card service is implemented with the TensorFlow Model Card Toolkit [15]. Apache Kafka [21] has been used as the message broker of the platform 

756 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

**Algorithm 3:** Global model aggregation 

- **1** Wait till the majority of the peers complete the local model training 

## **2 GLOBAL MODEL AGGREGATION:** 

- **3** Get transaction list _t_ (1 _, ..., n_ ) from ledger 

- **4** Get local model parameters from the transaction list _t_ (1 _, ..., n_ ) 

- **5** Average local model parameters with _SGD_ algorithm and generate global model 

- **6** Generate Model Card with global model information 

- **7** Create block _bi_ +1 with global model information, Model Card information, and transactions 

Fig. 5: Local model accuracy in single peer 

- **8** Save block _bi_ +1 in the ledger and broadcast it to other peers 

## **9 UPDATE FINAL MODEL:** 

## **10 for** _each peer p_ = 1 _, ..., n_ **do** 

- **11** Fetch block _bi_ +1 from ledger 

- **12** Verify transactions in the block **13** If the block _bi_ +1 is valid, fetch global model from the block 

- **14 end** 

and is deployed with Docker and Kubernetes based container orchestration system. 

As a federated learning use-case of the Bassa-ML, we have built an ML model for diagnosing acute inflammations of the bladder. Logistic regression algorithm [11] has been used to build the models using inflammations of bladder health data set. In this use case, a blockchain network is deployed at five peers (five hospitals). Each peer has its own data set on 

Fig. 6: Local model training loss in single peer 

which it applies the Logistic Regression algorithm to build a local model. Finally, these local ML models are averaged into a global model with a federated learning approach. The evaluation of the Bassa-ML has been performed based on this use case. 

**Local Model Accuracy and Training Loss** : We measured local model accuracy and training loss at a single peer. We run Logistic Regression model with 20,000 interactions and calculated the training loss and model accuracy. As the number of iterations increases, the model accuracy increases and reaches an upper bound (Figure 5), and the training loss decreases (Figure 6). 

**Federated Model Accuracy and Training Loss** : In the federated learning, we trained the model with 1000 iterations. A copy of the shared model is sent to each peer at each iteration. Each peer used its own local data to train the local model. Each local model improves slightly. Then we compute the total loss and accuracy as shown in Figure 7. Figure 8 shows how the total training loss varies at different peers in each iteration. 

Fig. 4: Model card generated in Bassa-ML platform. 

**Block generation time** . We have measured the block generation time in the Bassa-ML federated learning system by increasing the number of participated peers in the blockchain (up to 7). Figure 9 shows the effect of adding new peers to the block generation time. Figure 10 shows the average block generation time for having a different number of participated peers in the network. In this evaluation, we have repeated each 

757 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

Fig. 7: Federated model accuracy in different peers 

Fig. 8: Federated model training loss in different peers 

experiment 100 times, each with different peer sets, and plotted the average values. Adding peers to a cluster requires each peer to validate transactions in the block and recalculate the block header. Therefore, block generation time rises as more peers are added. 

## VI. RELATED WORK 

Several recent research projects have focused on integrating blockchain and federated learning. In this section, we outline the main features and architecture of these research projects. The comparison summary of these efforts and Bassa-ML is presented in Table I. 

**DeepChain** [4] is a blockchain and federated machine learning-based auditable, privacy-preserving, deep learning platform. It guarantees data privacy for each participant and provides auditability for the whole ML training process. It uses blockchain to provide data confidentiality, computation auditability, and incentives for parties to participate in collaborative training. **BlockFL** [22] proposes a blockchain-based federated machine learning architecture where mobile devices’ local learning model updates are exchanged and verified by leveraging blockchain. Instead of using central coordination in federated learning, they propose blockchain-based decentralized coordination. 

**ModelChain** [3] is a blockchain-based privacy-preserving machine learning framework for the healthcare domain. To overcome the security and robustness vulnerabilities of the traditional centralized architectures, ModelChain 

Fig. 9: Block creation time against the number of peers. 

Fig. 10: Average block generation time with different number of peers in the cluster. 

adopts blockchain-based decentralized architecture to provide privacy-preserving machine learning. **BAFFLE** [23] is a Blockchain-Based Aggregator Free Federated Learning platform. By eliminating a central aggregator, it overcomes several issues encountered with a centralized aggregator. 

**Poster** [24] proposes a blockchain-based privacy-preserving federated learning (BC-based PPFL) platform. It uses an Ethereum-based blockchain ledger to record information flows regarding FL tasks, participating clients, local and global model updates, etc. **Chained-FL** [25] proposes a blockchainbased chained anomaly detection model for federated learning. It is illustrated with use in an intrusion detection system. The main contribution of this work is evaluating the practical feasibility of auditing a federated deep-learning process by using blockchain. 

## VII. CONCLUSIONS AND FUTURE WORK 

In this paper, we have presented Bassa-ML, a blockchain and Model Card-based integrated federated learning provenance platform. The existing federated learning systems are designed with a centralized coordinator which is vulnerable to attacks. The proposed system overcomes several key concerns faced in centralized systems. While individual nodes (peers) develop local ML models based on their local data, the resulting models and parameters are shared through the blockchain platform. The model parameter sharing, local model generation, model averaging, and model sharing functions are imple- 

758 

2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC) 

TABLE I: Blockchain-enabled decentralized learning platform comparison 

|Platform|Running<br>Blockchain|Application<br>Domain|Data Analytic|Machine<br>Learning|Incremental<br>Tranning|Federated<br>Learning|Auditing &<br>Provenance|Model<br>Reporting|Real-Time<br>Predictions|
|---|---|---|---|---|---|---|---|---|---|
|Bassa-ML|Rahasak|Any||||||||
|DeepChain [4]|Corda|Any||||||||
|Breaking Bad [26]|Bitcoin|Cryptocurrency||||||||
|TMLC [27]|Ethereum|N/A||||||||
|BlockFL [22]|N/A|Mobile||||||||
|ModelChain [3]|ModelChain|Healthcare||||||||
|BAFFLE [23]|Ethereum|Any||||||||
|Poster [24]|Ethereum|N/A||||||||
|Chained-FL [25]|MultiChain|Any||||||||



mented with smart contracts implemented on the platform. The generated ML models and their information are stored in the blockchain ledger as Model Card Objects. This work demonstrates the feasibility of providing enhanced transparency and provenance with the federated learning process. As a use-case, we have implemented a federated machine learning model to diagnose acute inflammations of the bladder where the blockchain network is deployed in multiple hospitals. Each hospital has its own data set. A Federated learning pipeline is built and averaged with the global model with the data on different hospitals. In the future, we are planning to integrate libraries into Bassa-ML. 

## ACKNOWLEDGEMENTS 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] J. Koneˇcn`y, H. B. McMahan, F. X. Yu, P. Richt´arik, A. T. Suresh, and D. Bacon, “Federated learning: Strategies for improving communication efficiency,” _arXiv preprint arXiv:1610.05492_ , 2016. 

- [2] T. Yang, G. Andrew, H. Eichner, H. Sun, W. Li, N. Kong, D. Ramage, and F. Beaufays, “Applied federated learning: Improving google keyboard query suggestions,” _arXiv preprint arXiv:1812.02903_ , 2018. 

- [3] T.-T. Kuo and L. Ohno-Machado, “Modelchain: Decentralized privacypreserving healthcare predictive modeling framework on private blockchain networks,” _arXiv preprint arXiv:1802.01746_ , 2018. 

- [4] J. Weng, J. Weng, J. Zhang, M. Li, Y. Zhang, and W. Luo, “Deepchain: Auditable and privacy-preserving deep learning with blockchain-based incentive,” _IEEE Transactions on Dependable and Secure Computing_ , 2019. 

- [5] M. Strobel, “Aspects of transparency in machine learning,” in _Proceedings of the 18th International Conference on Autonomous Agents and MultiAgent Systems_ , 2019, pp. 2449–2451. 

- [6] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

- [7] H. Fang and H. Miao, “Introducing the model card toolkit for easier model transparency reporting,” 2020. 

- [8] T. Ryffel, A. Trask, M. Dahl, B. Wagner, J. Mancuso, D. Rueckert, and J. Passerat-Palmbach, “A generic framework for privacy preserving deep learning,” _arXiv preprint arXiv:1811.04017_ , 2018. 

- [9] Y. Lu, X. Huang, K. Zhang, S. Maharjan, and Y. Zhang, “Blockchain empowered asynchronous federated learning for secure data sharing in internet of vehicles,” _IEEE Transactions on Vehicular Technology_ , vol. 69, no. 4, pp. 4298–4311, 2020. 

- [10] Y. Qu, L. Gao, T. H. Luan, Y. Xiang, S. Yu, B. Li, and G. Zheng, “Decentralized privacy using blockchain-enabled federated learning in fog computing,” _IEEE Internet of Things Journal_ , vol. 7, no. 6, pp. 5171–5183, 2020. 

- [11] R. Sathya and A. Abraham, “Comparison of supervised and unsupervised learning algorithms for pattern classification,” _International Journal of Advanced Research in Artificial Intelligence_ , vol. 2, no. 2, pp. 34–38, 2013. 

- [12] R. Upstill-Goddard, D. Eccles, J. Fliege, and A. Collins, “Machine learning approaches for the discovery of gene–gene interactions in disease data,” _Briefings in bioinformatics_ , vol. 14, no. 2, pp. 251–260, 2013. 

- [13] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

- [14] E. Bandara, W. K. NG, K. De Zoysa, and N. Ranasinghe, “Aplos: Smart contracts made smart,” _BlockSys’2019_ , 2019. 

- [15] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

- [16] M. Kamp, L. Adilova, J. Sicking, F. H¨uger, P. Schlicht, T. Wirtz, and S. Wrobel, “Efficient decentralized deep learning by dynamic model averaging,” in _Joint European Conference on Machine Learning and Knowledge Discovery in Databases_ . Springer, 2018, pp. 393–409. 

- [17] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

- [18] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa, and W. K. Ng, “Saas-microservices-based scalable smart contract architecture.” 

- [19] A. Paszke, S. Gross, F. Massa, A. Lerer, J. Bradbury, G. Chanan, T. Killeen, Z. Lin, N. Gimelshein, L. Antiga _et al._ , “Pytorch: An imperative style, high-performance deep learning library,” _Advances in neural information processing systems_ , vol. 32, pp. 8026–8037, 2019. 

- [20] A. Ziller, A. Trask, A. Lopardo, B. Szymkow, B. Wagner, E. Bluemke, J.M. Nounahon, J. Passerat-Palmbach, K. Prakash, N. Rose _et al._ , “Pysyft: A library for easy federated learning,” in _Federated Learning Systems_ . Springer, 2021, pp. 111–139. 

- [21] J. Kreps, N. Narkhede, J. Rao _et al._ , “Kafka: A distributed messaging system for log processing,” in _Proceedings of the NetDB_ , 2011, pp. 1–7. 

- [22] H. Kim, J. Park, M. Bennis, and S.-L. Kim, “On-device federated learning via blockchain and its latency analysis,” _arXiv preprint arXiv:1808.03949_ , 2018. 

- [23] P. Ramanan, K. Nakayama, and R. Sharma, “Baffle: Blockchain based aggregator free federated learning,” _arXiv preprint arXiv:1909.07452_ , 2019. 

- [24] S. Awan, F. Li, B. Luo, and M. Liu, “Poster: A reliable and accountable privacy-preserving federated learning framework using the blockchain,” in _Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security_ , 2019, pp. 2561–2563. 

- [25] D. Preuveneers, V. Rimmer, I. Tsingenopoulos, J. Spooren, W. Joosen, and E. Ilie-Zudor, “Chained anomaly detection models for federated learning: An intrusion detection case study,” _Applied Sciences_ , vol. 8, no. 12, p. 2663, 2018. 

- [26] M. A. Harlev, H. Sun Yin, K. C. Langenheldt, R. Mukkamala, and R. Vatrapu, “Breaking bad: De-anonymising entity types on the bitcoin blockchain using supervised machine learning,” in _Proceedings of the 51st Hawaii International Conference on System Sciences_ , 2018. 

- [27] A. B. Kurtulmus and K. Daniel, “Trustless machine learning contracts; evaluating and exchanging machine learning models on the ethereum blockchain,” _arXiv preprint arXiv:1802.10185_ , 2018. 

759 

