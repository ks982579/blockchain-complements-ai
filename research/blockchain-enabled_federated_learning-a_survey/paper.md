## Blockchain-enabled Federated Learning: A Survey 

Cheng Li Yong Yuan College of Mathematics College of Mathematics Renmin University of China Renmin University of China Beijing, China Beijing, China cheng.li@ruc.edu.cn yong.yuan@ruc.edu.cn 

Fei-Yue Wang Institute of Automation, Chinese Academy of Sciences, Beijing, China Institute of Systems Engineering, Macau University of Science and Technology, Macau, China feiyue.wang@ia.ac.cn 

Blockchain is the core supporting technology underlying most digital cryptocurrencies since Bitcoin in 2008. Integrating blockchain technology can be expected to effectively solve the challenges faced by FL. As a novel decentralized and distributed computing paradigm, blockchain provides a suitable framework of model aggregation for FL. The Peer-to-Peer (P2P) network in blockchain architecture will enhance the fault tolerance capacity of the FL systems. The customized smart contracts can not only automate the management and control of the FL learning process, but also incentivize more users to participate in training. 

_**Abstract**_ **—In recent years, human society has been witnessed to evolve fast to the era of big data, rendering the data sharing and privacy protection a key issue for the development of digital economies. Federated learning, as a novel pattern for distributed machine learning, is aimed to train a centralized model from decentralized datasets while protecting user privacy, and is now intensively studied in literature. However, a variety of technical challenges, e.g., security issues, incentive mechanism design, are still awaiting further research efforts. In this respect, blockchain proves to be an elegant solution for federated learning to overcome these issues, and thus has been applied in federated learning in many scenarios with success. In this paper, we presented a comprehensive survey for blockchain-enabled federated learning, proposed its technical framework and discussed the key research issues. This work represents the first attempt for reviewing the literature aiming at integrating blockchain and federated learning, and can be expected to offer useful guidance for establishing a new infrastructure for decentralized and secured data circulation, as well as promoting the development of related industries.** 

As such, integrating blockchain into FL framework is widely considered as a novel infrastructure for circulation and sharing of data elements and an inevitable future trend. Technically,  FL and blockchain are mutually beneficial and thus can co-evolve to form a complete solution to mining joint value from distributed data owners with guaranteed security and privacy control. This mutual benefit is three-folds. First, the cooperation model is similar. Blockchain is a novel multiparty collaborative network architecture for distributed systems, while FL requires multiple entities to participate and collaborate in training models. As such, blockchain can be used as a basic architecture for FL. Second, they all featuring desirable characteristics of security and privacy protection. On the one hand, FL is designed with the aim of protecting data privacy in the process of distributed and cooperative data training. On the other hand, blockchain adopts consensus mechanisms and data verification mechanisms in its bookkeeping process, so that the recorded transactions cannot be tampered with and repudiated. Third, their applications can be expected to complement each other.  The purpose of FL is “creating value”, which is conducive to the complementarity of participants' data and improves the effectiveness of the model. Blockchain aims at “transferring value” via honestly recording and rewarding the contributions of all participants. Therefore, there is an increasing trend of research efforts dedicated in the blockchain-enabled FL (BeFL), and this motivates our work. 

## _**Keywords—blockchain, federated learning, smart contract**_ 

## I. INTRODUCTION 

With the rapid development of modern information and communications technologies, it has been witnessed an obvious trend of data explosion in smart and edge devices. According to Cisco's estimation, the total volume of data generated worldwide is close to 850 Zetta Bytes. As a result, data has become a new production factor and can be expected to play an important role in many fields. For instance, artificial intelligence technologies represented by machine learning and especially deep learning requires a large amount of data to train a perfect model. Due to the restrictions of privacy protection policies and geographic isolation, however, data is usually scattered as and limited in "data islands", resulting in a low efficiency and effectiveness in the large-scale coordination and cooperation among institutions. As such, there is an urgent need for promoting the secured and trusted data circulation and sharing. 

Federated learning (FL) was first coined by Google in 2016 in its Input Method Optimization Project. According to the data distribution among multiple participants, FL can be classified into three categories, i.e., horizontal FL, vertical FL and federated transfer learning[1]. FL uses the parameter exchange method with encryption mechanisms to jointly build a virtual common model with enhanced features including data privacy, security and legal compliance. However, FL is still faced with many technical challenges. First, since FL relies on a single central coordinator for aggregating the gradient parameters, and thus might suffer from a single-point failure risk once the central server fails. Second, FL framework cannot guarantee the quality of uploaded parameters from distributed participants, and there are still potential security hazards in the model training process. Finally, considering the variety of data quality and training costs among multiple participants, the incentive of contributing their data might differ and thus some might not be willing to honestly participate in the model training. 

In this paper, we will present an overview of BeFL. The reminder of this paper is organized as follows. In Section II, we will give the technical framework and overview of the BeFL, and discuss its basic workflow. Section III will present the detailed research progresses in the key research issues. Section IV concludes. 

## II. A CONCEPTUAL FRAMEWORK FOR BLOCKCHAIN-ENABLED FEDERATED LEARNING 

This section will propose a six-layer technical framework for BeFL and introduce its basic workflow. 

## _A. The Conceptual Framework_ 

The conceptual framework of BeFL is shown in Fig. 1. Our framework is six-layered, i.e., in a bottom-up order, infrastructure layer, consensus mechanism layer, economic incentive layer, smart contract layer, privacy protection layer, and application layer. 

978-1-6654-3337-2/21/$31.00 ©2021 IEEE 

286 

poo ----------------------------------------5 

After receiving the FL task, the participants selected as the training node will use local data samples to training a local model and broadcast their updated model parameters on the P2P network. When receiving the updated parameters of other training nodes from the network before the preset timestamp, each training node will process the received parameters from other miner, and hence generate an aggregation model according to the rules defined in the smart contract. 

The verification nodes verify the aggregation model and generate a global model according to the rules defined in the smart contract. Meanwhile, they will evaluate the contribution of the training nodes. Before the next FL round, the verification nodes will broadcast the global model parameters through the P2P network to training nodes, who will obtain the global model at this point. 

Fig. 1. Conceptural framework of blockchain-enabled federated learning 

The infrastructure layer aims at integrating the decentralized, distributed framework of blockchain as an infrastructure for FL to ensure the security of data sources. The consensus mechanism layer aims at using or improving original blockchain consensus algorithms so as to reduce meaningless wasting of computing resources and ensure the trustability of the FL results. The economic incentive layer is mainly targeted at designing incentive mechanisms to maximize the benefits of user nodes, and thereby encourage more nodes to participate and contribute high-quality private data. The smart contract layer aims to replace the coordinator with software-defined, algorithm-driven smart contracts to manage the FL process and in turn integrate with artificial intelligence technologies. The privacy protection layer uses encryption technology to ensure the security of the computing and communication process in FL. In the application layer, multiple application scenarios requiring data sharing and privacy protection are encapsulated, including but not limited to cloud computing, medical health-care, smart cities, digital twins, among others. 

Each participant starts mining as a miner in the blockchain network, and generates data blocks according to the consensus mechanism. The learning results are also stored in the blocks.  When a miner generates a new block, other miners will verify the data in the block. If a block is verified and accepted by a majority of miners, it will be appended onto the blockchain ledger, and the smart contract will reward the miner according to predefined incentive mechanism. Then, the next FL round starts. 

## III. AN OVERVIEW OF RESEARCH PROGRESSES 

This section will discuss the detailed research progresses in our six-layer model, and summarize the widely-studied research issues and solutions in each layer. 

## _A. The Infrastructure Layer_ 

Compared with traditional FL architecture, the network topology of BeFL is decentralized with equal peer nodes. The nodes not only use local data to train their local models, but also accept gradient parameters to aggregate the global model. The network is open and dynamic, and nodes can join and exit freely. Once nodes with new resources entering the system, the overall diversity and service capability are expected to be increased simultaneously, resulting in enhanced network scalability and load balancing capability. Therefore, a single point of failure will not affect the overall system, leading to high robustness and availability of the network; Meanwhile, all nodes have the relay and forwarding functionalities, which can help improve the anonymity of communication and protect personal privacy. 

## _B. Basic Workflow_ 

As is shown in Fig. 2, each participant has his/her private data, and the task publisher deploys smart contracts on the blockchain network to broadcast learning tasks, node selection rules, and authorization verification to participants.  All verified participants will join in the blockchain network as miners, and a small number of participants will be selected as verification nodes using the blockchain consensus mechanism in this FL round. 

A major limitation of traditional FL is that the quality of uploaded parameters cannot be effectively guaranteed. As such, low-quality parameters might jeopardize system = performance, resulting in an unbearable convergence time. In addition to verifying the transactions in the generated blocks, BeFL also uses the data verification mechanism to verify, and in turn guarantee the quality of the uploaded parameters of the training nodes. 

**==> picture [443 x 3] intentionally omitted <==**

**----- Start of picture text -----**<br>
5=<br>**----- End of picture text -----**<br>


Considering the limitations of on-chain storage space and communication bandwidth, although storing models and model updates on the blockchain has many advantages, it is also a huge burden for the blockchain nodes. As such, the training process of the model can be conducted in an off-chain fashion, that is, the hash value returned by Inter-Planetary File Systems (IPFS) is used for on-chain transmission instead of directly transmitting the entire model parameters or model gradients. 

Fig. 2. Basic workflow of blockchain-enabled federated learning 

This work was supported in part by the National Key R&D Program of China(2018AAA0101401), and the Science and Technology Development Fund, Macau SAR (File No. 0050/2020/A1). 

287 

## _B. The Consensus Mechanism Layer_ 

The critical part of most blockchain consensus algorithms is the process of selecting the bookkeeping node from the set of all miner nodes through diversified leaderelection mechanisms including vote-based, proof-based, alliance-based, randomness-based or other hybrid algorithms. The advantage of the existing blockchain consensus algorithms is that any completed request cannot be changed and can be seen by subsequent requests, and thus has higher security level. Meanwhile, consensus algorithms can also accept and execute requests from non-Byzantine clients, and will not be affected by those factors that cause these requests to fail in execution. 

In blockchain networks, vote-based consensus mechanisms aim to select the current round of bookkeeping node via election among miner nodes during each consensus round. Blockchain-based FL uses the blockchain consensus mechanisms to select nodes participating in training and thus can further improve the efficiency and robustness of FL.  In BeFL, it is also necessary to take additional attributes into consideration, i.e., data quality of the node, the number of computing resources, network conditions and other factors, and  integrate them with certain weights to select a proper bookkeeping node. 

Proof-based mechanisms indicate that bookkeeping nodes must prove that they outperform others in a specific ability in each consensus round. Inspired by this, similar algorithms are also designed for BeFL to guarantee the system robustness. For example, honest nodes can be rewarded with equity more frequently, and the miner who makes the most contribution in each round of communication can create a legal block[2]. 

Alliance-based mechanisms aim to elect multiple delegate nodes based on a specific method from the miner nodes, and then these delegate nodes can obtain the bookkeeping rights for each block alternatively or by election. Therefore, inspired by alliance-based consensuses, researchers have proposed to design a committee consensus mechanism, which can not only improve training efficiency, but also optimize the training process of machine learning[3]. 

## _C. The Economic Incentive Layer_ 

Blockchain is a typical economic system, and its participants obtain economic incentives by contributing computing power. Therefore, designing incentive mechanisms in blockchain-enabled FL systems can be expected to properly evaluate each node’s contribution, and in turn attract more users to participate in the training process. As such, many literatures are dedicated to designing the incentive mechanism for BeFL based on the users’ contributions. Specifically, the incentive mechanism can be designed from three aspects, that is, user data quality, user behavior, and user reputation. 

In evaluating the value of data, shaply value (SV) method is widely used in this field. For instance, Liu et.al, proposed FedCoin[4], in which the blockchain consensus nodes calculate the SV and create a new block based on the proof of the Shapley protocol. Based on this calculated SV, a distribution scheme with such characteristics as antirepudiation and anti-tampering is proposed for allocating the revenue among users in FL. 

With respect to analyzing the user behavior and encourage their positive behavior, Toyoda et.al, designed a fair and incentive-conscious federated blockchain platform, and proved its incentive compatibility based on the theory of competition[5]. By formulating a strict reward policy on the number of workers, the way they distribute the rewards, as well as the contributions of workers, any rational worker can follow the agreement and maximize the profit. 

Reputation is an important indicator for selecting users in the training process of FL.  Customers with a higher reputation are more likely to bring high-quality and reliable training to FL tasks. At the end of each training task, the user’s reputation will be updated based on the user’s behavior, and then the reputation record will be considered when selecting nodes in the next round of training. There are many advantages in the reputation mechanism, such as high-reputation miners can avoid complex majority voting for verification, thereby reducing time consumption.  When dealing with a fork, if a miner receives two or more satisfactory models from different miners at the same time, one solution is to choose the miner with the highest reputation. 

## _D. The Smart Contract Layer_ 

Smart contracts can be seen as running on the distributed blockchain ledger with preset rules, states, and conditional responses, which can be used to encapsulate, verify, and execute complex behaviors of distributed nodes to realize information exchange, value transfer, and asset transfer. More importantly, smart contract is trustworthy and reliable, because once it is released in batches, the contract is undeniable and public. 

In the decentralized framework of BeFL, the role of the central server is taken over by smart contracts to coordinate the workflow among all participating nodes. For example, it can release modeling tasks, training and model aggregation commands, and verify the updates sent by nodes.  According to the node's contribution, its data is priced, and tokens are used for rewards, etc. 

There are many benefits in using smart contracts to manage joint training. First, a copy of the global model and the related computing states are maintained on the smart contract. Model selection and aggregation are performed in a completely decentralized manner, so that user nodes can determine their own choices by themselves, which helps establish trust between nodes. Second, user nodes can use the global model copy to autonomously perform the aggregation step in each round, and update the global model independently, thereby promoting the development of global computing. Finally, with the support of the underlying consensus protocol, smart contracts help ensure the transparency, fairness and impartiality of the FL process. 

All kinds of smart contracts running on the blockchain can be regarded as the user's software-defined agent. However, the contracts currently running on various blockchain platforms are not sufficiently intelligent in that they can only be executed in accordance with preset “if-then”-type rules. In BeFL, artificial intelligence technologies such as machine learning, deep learning, and reinforcement learning will be applied to the BeFL network, so that the agent will gradually develop from basic perception, reasoning and learning to tasks selection,  goal-oriented behavior, autonomous decisionmaking and other functions[6]. 

288 

## _E. The Privacy Protection Layer_ 

The main advantage of FL is that each round of model training is completed locally and a copy of the global model is retained. During each round of training, the data is stored locally on the terminal device and never shared with the server. This computing paradigm can help improve privacy and meet the requirements of corresponding regulations. Although local data is not disclosed in FL, sometimes it is not secure enough due to the exchange of model parameters, which might leak sensitive information about the data. 

There are many privacy protection mechanisms that provide different privacy guarantees.  Secure multi-party computing, differential privacy and homomorphic encryption are the most widely used privacy protection technologies. Homomorphic encryption and secure multi-party computing require that all parties must encrypt the message before sending it, operate on the encrypted message, and decrypt the encrypted output to obtain the final result. Differential privacy aims to hide the personal information contributed by the client model update by adding a small amount of noise. In designing BeFL systems, many research efforts have incorporated these privacy technologies into their framework[7]. 

It is worth noting that in the design of the BeFL framework, it is necessary to take reducing the heavy computational overhead into consideration. For example, when using differential privacy, higher privacy requires increasing the variance of discrete Gaussian noise, but this in turn means higher communication costs. Secure multi-party computing protocols require all parties to generate secret shares of private data and exchange them with all other parties. This basic requirement inevitably leads to a high communication overhead. The implementation of homomorphic encryption relies on a trusted execution environment, and so on. Therefore, the cost of communication and the deployment environment of the terminal device are issues that must be considered when designing a privacy protection mechanism for BeFL. 

## _F. The Application Layer_ 

Federated cloud is widely-believed as the future direction of cloud computing. Edge computing in the operating environment, fog computing and terminal cloud computing are interconnected and cooperate to enhance cloud equipment. Under increasingly strict data privacy policies, BeFL-based cloud design will be more widely used. The BeFL cloud computing framework can be used as a secure storage of digital assets in different places and can be accessed through authorized chains. 

When applying machine learning to training data in medical treatment for medical institutions, the problem that needs to be considered is that the training data needs to be kept locally and to avoid inferring sensitive information from the trained model.  Insufficient data sources and insufficient labels lead to unsatisfactory performance of machine learning models, which has become the bottleneck of current smart healthcare.  Using blockchain, datasets can be collaboratively collected from different sources, and at the same time FL is used to protect the data privacy of patients, so that the use of BeFL can improve the level of hospital diagnosis and treatment. 

Data is the most important factor in building a smart city. City data consists of many data islands, such as transportation, industry, and education, etc. In order to build a smart city, it is necessary to break the data islands, realize the trustworthy and collaborative sharing of data while protecting data privacy. As a distributed ledger, blockchain can solve the problem of data trust, and FL realizes data privacy protection by sharing model parameters instead of raw data. As such, BeFL can effectively build a smart city data security sharing mechanism. 

Emerging technologies such as digital twins and sixthgeneration(6G) mobile networks have accelerated the realization of edge intelligence in the Industrial Internet of Things (IIoT). Integrating digital twins into wireless networks can help migrate real-time data processing and computing to the edge plane. The BeFL-based authorization can improve the reliability and security of the system and enhance data privacy. There have been intensive studies using reinforcement learning and asynchronous aggregation schemes to improve operating efficiency, but further research is still needed. 

## IV. CONCLUDING REMARKS 

The integration of blockchain and FL is the current trend of technological development. In this paper, we described the basic workflow of BeFL and proposed a six-layer technical framework.  In addition, we introduced its key research issues and the existing solutions in each of the six layers featuring infrastructure, consensus mechanism, economic incentives, smart contracts, privacy protection, and applications. It needs to be pointed out that the combination of blockchain and FL is still in its infancy stage, and more knowledge domains need to be studied to solve these challenges. 

## REFERENCES 

- [1] P. Kairouz _et al._ , "Advances and Open Problems in Federated Learning," p. arXiv:1912.04977. [Online]. Available: https://ui.adsabs.harvard.edu/abs/2019arXiv191204977K 

- [2] H. Chen, S. A. Asif, J. Park, C.-C. Shen, and M. Bennis, "Robust Blockchained Federated Learning with Model Validation and Proofof-Stake Inspired Consensus," p. arXiv:2101.03300. [Online]. Available: https://ui.adsabs.harvard.edu/abs/2021arXiv210103300C 

- [3] Y. Li, C. Chen, N. Liu, H. Huang, Z. Zheng, and Q. Yan, "A Blockchain-Based Decentralized Federated Learning Framework with Committee Consensus," _IEEE Network,_ vol. 35, no. 1, pp. 234-241, 2021, doi: 10.1109/MNET.011.2000263. 

- [4] Y. Liu, S. Sun, Z. Ai, S. Zhang, Z. Liu, and H. Yu, "FedCoin: A Peerto-Peer Payment System for Federated Learning," p. arXiv:2002.11711. [Online].Available:https://ui.adsabs.harvard.edu/abs/2020arXiv20021 1711L 

- [5] K. Toyoda and A. N. Zhang, "Mechanism Design for An Incentiveaware Blockchain-enabled Federated Learning Platform," in _2019 IEEE International Conference on Big Data (Big Data)_ , 9-12 Dec. 2019 2019, pp. 395-403, doi: 10.1109/BigData47090.2019.9006344. 

- [6] D. Połap, G. Srivastava, and K. Yu, "Agent architecture of an intelligent medical system based on federated learning and blockchain technology," _Journal of Information Security and Applications,_ vol. 58, p. 102748, 2021/05/01/ 2021, doi: https://doi.org/10.1016/j.jisa.2021.102748. 

- [7] A. R. Short, H. C. Leligou, M. Papoutsidakis, and E. Theocharis, "Using Blockchain Technologies to Improve Security in Federated Learning Systems," in _2020 IEEE 44th Annual Computers, Software, and Applications Conference (COMPSAC)_ , 13-17 July 2020 2020, pp. 1183-1188, doi: 10.1109/COMPSAC48688.2020.00-96. 

289 

