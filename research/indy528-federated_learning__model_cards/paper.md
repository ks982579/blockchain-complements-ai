2022 IEEE 19th International Conference on Mobile Ad Hoc and Smart Systems (MASS) 

# — Indy528 Federated Learning Model Tokenization with Non-Fungible Tokens(NFT) and Model Cards 

Eranga Bandara _[∗]_ , Xueping Liang _[‡]_ , Sachin Shetty _[∗]_ , Ravi Mukkamala _[∗]_ , Abdul Rahman _[†]_ , Ng Wee Keong _[§]_ , 

> _∗ {_ cmedawer, sshetty, mukka _}_ @odu.edu Old Dominion University, Norfolk, VA, USA 

> _†_ abdulrahman@deloitte.com Deloitte & Touche LLP 

> _‡_ x liang@uncg.edu University of North Carolina at Greensboro, NC, USA 

> _§_ AWKNG@ntu.edu.sg Nanyang Technological University, Singapore 

_**Abstract**_ **—Non-Fungible Tokens (NFTs) are used to represent ownership of unique items. The tokens provide means to validate integrity. That is, the item is unique and cannot be replaced with something else. NFTs are securely stored in the blockchainbased decentralized storage in the format of ERC721 tokens, so it is technically difficult to modify the record of ownership or duplicate a new NFT token. Currently, NFTs are used to tokenise objects such as arts, collectables, and even real estate. In this research, we propose an open platform “Indy528” and utilize NFT tokens to represent federated machine learning (FML) models. To the best of our knowledge, this is the very first research that tries to represent machine learning models as NFTs. Different peers in the blockchain network can build FML models using a blockchain-enabled coordinator-less FML system. The data provenance information of the FML models, which records the model data changes from the origin, is recorded as Model Cards. The FML model ownership, storage locations of the FML models and data provenance information(with Model Cards) are encoded into NFT tokens and stored in the decentralized ledger. We design a novel NFT token schema i528 (which is an extension of ERC721) to represent machine learning models as NFTs. As a use-case of Indy528, we propose a concept of the NFTbased decentralized FML model marketplace. Model creators can train the FML models and publish them in the marketplace as NFTs. Parties who want to use the models can buy them from the model creators as NFTs. The buyers can view model information, accuracy, training process etc via the model card objects stored with the NFT. This design methodology provides enhanced transparency and auditability to the federated learning process while providing an open platform to share and trade machine learning models in the marketplace.** 

_**Index Terms**_ **—NFT, Blockchain; Machine Learning; Federated Learning; Model Card** 

## I. INTRODUCTION 

Cryptocurrencies being mutually interchangeable, i.e. fungible, they can be regularly traded or exchanged. This characteristic drives the suitability of cryptocurrencies as a secure medium of transaction. A particularly unique characteristic of Non-Fungible Tokens (NFT) is their unique and irreplaceable property where it is impossible for one token to equal to another [1]. NFTs are also extensible in that one NFT combined with another can “breed” a third, unique NFT. NFTs are used to represent things like digital assets (e.g songs and digital 

arts) and physical assets (e.g real estate, furniture, etc). For example, a painting can be represented as an NFT. This digital equivalent may have multiple owners where each may own it for a fraction of the cost of the original painting. Such a cost model can drive overall value and potential revenues. Previous work has investigated how blockchain can help with the tokenization of index funds, which can benefit investors and firms significantly [2]. In this research, we are proposing an open platform “Indy528” to protect the federated machine learning(FML) models as NFT tokens. To the best of our knowledge, this is the very first research that tries to represent machine learning models as NFTs. 

Federated learning (FL) is a new research area that uses a distributed/collaborative machine learning (ML) methodology [3]. The majority of existing FL systems include the following characteristics: a centralized coordinator for aggregating local machine learning models [4]; typically quite vulnerable to attacks and privacy breaches [5]; and resulting ML model transparency and provenance are often unsatisfactory [6]. The Indy528 platform addresses these issues by proposing a blockchain-enabled coordinator-less FML system [7]. The Indy528 platform can be used by numerous peers that are taking part in the FL. Each peer in the blockchain manages its own off-chain storage, which stores real data that the peer controls and holds. These peers can use a blockchainenabled coordinator-less system to create FML models. Offchain storage is used to store the created FML models. Model Cards provide data provenance information for FML models (e.g., participating clients, local model generators, aggregators, model production times, and so on) as well as quantitative analysis [8]. The FML models’ information—ownership, offchain storage locations and data provenance information(with Model Cards)—is encoded into NFT tokens and stored in the decentralized ledger. We have used Rahasak blockchain as the decentralized ledger in the Indy528. We have designed an extensible Non-Fungible Token Model using novel token schema i528 to represent machine learning models as NFT tokens. The proposed i528 token schema is an extension of the ERC721 [9] schema which is implemented in the Rahasak 

2155-6814/22/$31.00 ©2022 IEEE DOI 10.1109/MASS56207.2022.00033 

195 

blockchain. 

As a use-case of Indy528, we have proposed NFT-based decentralized marketplace to trade the ML models. Model creators and trainers can build and train the FML models and publish them in the marketplace as NFT tokens. Parties who want to use the models can buy them from the model creators as NFT tokens using fiat currency. The revenue will be shared among the model creators and trainers. The ownerships of the ML models are encoded in the NFT and saved in the blockchain ledger. In our security model, we are aligned with the security goals mentioned in [10] that the data is secure against the adversary who does not own the data, and also the trust model for blockchain implementations verified in [11]. The proposed Indy528 platform improves the federated learning process’ transparency and auditability. Each user may fully track all actions performed on the ML model, providing a comprehensive history of all operations and incremental versions that have existed. The ML model marketplace is an open platform for peers to exchange and trade machine learning models. The model card objects maintained in the ledger allow buyers to see all of the model information, training process, quantitative analysis, and evaluation. Indy528 now has a completely transparent and auditable trading system. The NFT tokens generated in this machine learning model marketplace were used to evaluate the Indy528 platform. The following are Indy528’s major contributions. 

- 1) There has been a proposal for a blockchain-enabled coordinator-less federated learning system. 

- 2) By storing model information and model reports as Model Card objects in the blockchain ledger, the federated learning process gains more transparency and provenance. 

- 3) Proposed a model to represent machine learning model information as NFT tokens along with Model Cards. 

- 4) Designed an extensible Non-Fungible Token schema i528 to represent the machine learning models as NFT tokens. 

- 5) Proposed a NFT token-based machine learning model marketplace to share/trade ML models. 

The following is a breakdown of the paper’s structure. The background information is discussed in Section 2. The design of the Indy528 platform is discussed in Section 3. The Indy528 platform’s capabilities are discussed in Section 4. Section 5 consists of a performance evaluation. Work relating to Section 6. Section 7 wraps up the Indy528 platform with recommendations for further work. 

## II. BACKGROUND 

This section provides a brief summary of each of the technologies underlying Indy528. In particular, we discuss the following concepts: Federated Learning(FL), Non-Fungible Tokens(NFT), ERC721 Tokens and Model Cards. 

## _A. Federated Learning (FL)_ 

Data exchange within businesses has gotten increasingly complex as the demand to secure organizational and individual 

data privacy has grown. Federated learning addresses these privacy problems within a coalition by eliminating the requirement for a central agency to collect data from its members [4]. Each coalition member creates a machine learning model based on local data and shares model parameters with the rest of the group. Finally, the local models are combined to form a global model. In other words, federated learning uses dispersed data to train central models. This helps to keep individual member data private [12]. 

## _B. Non Fungible Tokens (NFT)_ 

To signify ownership of unique things, Non-Fungible Tokens (NFT) are used [1]. The tokens serve as a mechanism for verifying the integrity of data. That is to say, the item is one-of-a-kind and cannot be replaced. Currently, NFTs are used to tokenise objects such as art, collectables, and even real estate. NFTs are securely stored in the blockchain-based decentralized storage in the format of ERC721 tokens [9]. ERC721 is an Ethereum token standard known as “Ethereum Request for Comments 721”. ERC721 defines a few functions that ensure it adheres to the ERC20 token standard [13]. This is done to make it easier for existing wallets to display basic token information. 

## _C. Model Cards_ 

Model cards can help developers, regulators, and downstream users alike by providing a standardized framework for reporting on ML model provenance, model usage, ethicsinformed evaluation, and a full summary of a model’s proposed uses and limitations. The Google TensorFlow framework provides the Model Card Toolkit, which can be used to produce Model Card objects [8]. 

## III. INDY528 ARCHITECTURE 

Figure 1 describes the architecture of the Indy528 platform. The proposed Indy528 platform is composed of five layers: a blockchain ledger layer, a smart contract layer, a Federated Machine Learning (FML) layer, a Model Card service layer, and an NFT service layer. The next sections go through the specifics of each layer. 

## _A. Blockchain Ledger Layer_ 

In a federated learning environment, the blockchain network can be used by various entities. Each network node (Figure 1) can run its own blockchain node. The Blockchain ledger stores peer specifics of the federated learning environment, ML model information, and NFT token information, as explained earlier in this work. Model Card objects, which are kept on the blockchain ledger, are used to store the ML model information. 

## _B. Smart Contract Layer_ 

Blockchain smart contracts are used to implement Indy528’s features. 1) Identity contract, 2) FML contract, 3) Model Card contract and 4) NFT contract are the four key smart contracts in the system. The Identity contract is in charge of the peers’ identity management (e.g identity registration, permissions handling etc). The federated learning functions are handled 

196 

## _E. NFT Service Layer_ 

We have designed NFT token schema i528 to represent machine learning models as NFT tokens. Figure 3 described the i528 schema of the NFT token of the Indy528 platform. It contains metadata such as Name, Description, Model URI, Owner, and Model card objects of the ML models. The NFT service handles the NFT token generation functions with these schemas. It generates i528 tokens as JSON format objects. This service interacts with smart contracts on the blockchain to generate NFT tokens that are linked to machine learning models. For example, when the federated learning process is finalized, the leader peer in the network generates NFT token related to the global model via interacting with this service. The generated NFT will be stored in the blockchain ledger and distributed to other peers in the network. 

## IV. INDY528 FUNCTIONALITY 

Fig. 1: Indy528 platform layered architecture. 

by the FML contract (e.g local model training, global model aggregation etc). The Model Card contract converts ML model data (e.g., produced model, aggregated model, [14], etc.) into Model Card objects, which are then stored in the blockchain ledger. The NFT contract is in charge of NFT token creation and model trading. 

## _C. Federated Machine Learning Layer_ 

Each blockchain peer provides Federated Machine Learning (FML) as a service. The stages in which these are implemented include FL pipeline creation, initial model generation, local model generation, and global model creation with the model averaging function [15]. The operations of the verification service interact with smart contracts on the blockchain. When a smart contract receives a request to train an ML model, for example, it talks with its FML service to build the local model using data from its peer’s off-chain storage. Smart contracts employ the FML service to build blocks on the blockchain and average the local models into the global model using the Stochastic Gradient Descent (SGD) approach. 

There are four main functionalities of the Indy528, 1) Identity registration, 2) Federated model training, 3) NFT token issuing and 4) Model trading. This section goes into the specifics of these functions. 

## _A. Identity Registration_ 

In the FL environment of Indy528, numerous peers collaborate. Hospitals, banks, and private enterprises are examples of peer organizations. Each peer has their own set of data, which is stored off-chain. These data will be utilized to create off-chain storage models. The platform must initially be onboarded with the peer entities. Once these peers have been onboarded, separate blockchain nodes will be deployed for each of them. All of the peers’ identifying information is stored in the blockchain ledger. The blockchain’s identity smart contract implements this identity onboarding function [16]. 

## _D. Model Card Service Layer_ 

Figure 2 shows an example model card created using the Indy528 platform. It includes data on clients with local models, peers in charge of aggregating the models, model generation times, and so on, as well as quantitative analysis and model considerations. The Model Card service is in charge of objectproducing functions that contain model information. Machine learning model data is converted into Model card objects using Model card services, which are subsequently stored on the blockchain ledger. The Model Card Services interface with the smart contracts in the blockchain to generate the model cards for the ML models (e.g, local models and global models). 

Fig. 2: Model card generated in Indy528 platform. 

197 

This information is encoded into JSON objects with the i528 schema and saved in the blockchain ledger. The trading of the model handles with this NFT token. The structure of the NFT token is represented in Figure 3. 

## _D. Model Trading_ 

As mentioned above, we have proposed NFT-based decentralized marketplace to trade the ML models. Model creators and trainers can build and train the FML models and publish them in the marketplace as NFT tokens. The leader peer will build the NFT token for the model and publish it to the marketplace. The marketplace trading functions are implemented with smart contracts. Once the leader peer published the NFT token(which is related to the global model), the parties who want to use the models can buy them from the marketplace as NFT tokens. Then the revenue will be shared among the model creators and trainers. When buying, buyers can verify the model information, training process, quantitative evaluation of the model etc, via the model card objects published in the ledger. This supports a fully transparent and auditable trading system in Indy528. 

Fig. 3: i528 NFT token schema to represent the machine learning model. 

## _B. Federated Model Training_ 

In FL, first, a leader peer will be selected among the peers based on the consensus algorithm to handle the coordination functions. The leader peer will handle the federated pipeline creation, model parameter initialization and global model aggregation functions. The leader peer initialize the federated pipeline and model parameters and published them on a block. Then other peers take the initial model and model parameters from the block and train the local ML models(according to the model parameters) with their own data set(which are stored in the off-chain storage). Once the local model training is finished, the peers generate the Model Card object with the local model information. The Model Card object comprises model information (e.g., participating clients, local model generator, aggregator peer, model generation time, model URI, and so on), quantitative analysis of the model, and Figure 2. The Model Card objects are then published as transactions into the ledger by peers. Once all peers have created their local models, the leader peers will use the stochastic gradient descent (SGD) technique to average them into a global model. The completed ML model information is then encoded into the Model Card object and saved in the block. Finally, the global model block will be transmitted to all peers. 

## V. INDY528 IMPLEMENTATION AND EVALUATION 

The implementation architecture of the Indy528 is described in Figure 4. The Rahasak blockchain has been adopted as the blockchain ledger [17], [18], with Rahasak blockchain’s Aplos smart contract platform [19], [20] to provide a customized smart contract interface. Each Rahasak blockchain node contains 4 different customized services, 1) Model Card service, 2) FML service, 3) Off-Chain storage and 4) NFT service. Pytorch and Pysyft-based python libraries are used to implement the FML service. The TensorFlow Model Card Toolkit is used to implement the Model Card service. Off-chain storage has been implemented using Apache Cassandra. The JSON encoded i528 token structure is used to construct the NFT service. Apache Kafka distributed publisher-subscriber system facilitates the peer to peer communications [17]. Rahasak-CA which is the certificate authority of the Rahasak blockchain system has been used to store the digital certificates of the 

## _C. NFT Token Issuing_ 

Once the global model is averaged, the leader peer will create NFT token for that global model and publish it into the ledger. The NFT token contains information about the metadata of the ML model, the URI of the model and the ownership of the model. The peers who participated in model training are identified as owners/contributors of the model. 

Fig. 4: Indy528 platform implementation. 

198 

Fig. 5: Local model accuracy in single peer 

101112epoch 13 1415 16 17 18 19 20 

Fig. 6: Local model training loss in single peer 

peers who participate in the federated learning process. The implemented system has been deployed using a Docker and Kubernetes-based container orchestration framework. 

We developed an ML model to detect network spam messages via the federated learning process as a use case for the performance evaluation of Indy528. Gated Recurrent Unit(GRU) neural network [21] has been used to build the models with the message data set. The blockchain network has five peers in this use case. Each peer has its own data set, which it uses to develop a local model. Finally, with the federated learning architecture, these local ML models are integrated into a global model. The generated global model is published as i528 NFT tokens on the Rahasak blockchain. The model trading happens with these tokens. The evaluation of the Indy528 has been conducted based on this use case with the following parameters. 

**Local Model Accuracy, Training Loss and Validation Loss** : In federated learning, the local peers train the initial model with the local data set. This local training process happens with a number of iterations(e.g 20,000 iterations). In this evaluation, we have examined local model accuracy, training loss and validation loss at a single peer. Local model accuracy has been measured with “Area Under the Receiver Operating Characteristic Curve (ROC AUC)”. The training loss indicates how well the model is fitting the training data, while the validation loss indicates how well the model fits new data. We train the model with multiple iterations and obtained these measures. As the number of iterations increases, the model accuracy increases and reaches an upper bound (Figure 5). Simultaneously, with the increase in model accuracy, the training loss and validation loss decrease (Figure 6 and Figure 7). 

**Federated Model Accuracy** : In this evaluation, we have measured the Model accuracy of the federated machine learning model. The Federated learning process is performed through a number of iterations to get an accurate model(e.g we trained the model with 1000 iterations in federated learning). In each iteration, the current Federated model is sent to each peer. Each peer uses its own local data and the received Federated model to retrain the local model. Each iteration further improves the local models and computes the accuracy and model loss based on its local data. The total loss and 

Fig. 7: Local model validation loss in single peer 

accuracy are computed by combining the local accuracy and losses. The resulting accuracy is shown in Figure 8. 

**Block Generation Time** : The average time to generate a block is a critical component because it determines the system’s effectiveness. Block creation time is compared to the number of transactions in the block. The time it takes to generate a block is determined by four key factors: the time it takes to elect a leader, the time it takes to replicate and broadcast data amongst peers, the time it takes to generate Merkle proofs/block hashes, and the time it takes to validate transactions. Each of these parameters grows as the number of transactions per block increases, Figure 9. The average block generation time with different transaction sets in a block is discussed in Figure 10. 

## VI. RELATED WORK 

The integration of Non-Fungible Tokens (NFT) has been the subject of several recent research initiatives. They are trying to apply Non Fungible Tokens into different application domains. The key elements and architecture of these research initiatives are outlined in this section. Table I summarizes the contrast between these projects and Indy528. 

In **NFT-Energy** [23], Karandikar et al present a blockchainbased system in the context of the electric energy industry. It employs the blockchain to record energy asset transactions among individual producers, consumers, electric vehicles, power companies and storage providers. They represent consumers’ unique information as non-fungible tokens and all other transactions such as producer-consumer transactions are recorded as fungible tokens on the blockchain. They measure 

199 

TABLE I: Non Fungible Token(NFT) platform comparison 

|Platform|Application<br>Domain|Running<br>Blockchain|Smart<br>Contract|Smart Contract<br>Language|Token<br>Format|Token<br>Storage|Trading<br>Support|Erc721<br>Support|
|---|---|---|---|---|---|---|---|---|
|Indy528|Machine Learning|Rahasak|Aplos|Scala|i528|Cassandra|||
|NFT-OpenGLAM [22]|OpenGLAM<br>Digital Arl|N/A|N/A|N/A|N/A|N/A|N/A|N/A|
|NFT-Energy [23]|Energy|Hyperledger Fabric|Chaincode|Golang|N/A|IPFS|||
|Fertile LAND [24]|Real-State|N/A|N/A|N/A|N/A|N/A|N/A||
|Tokenfication [25]|Gaming|N/A|N/A|N/A|N/A|N/A|N/A||
|NFTGAN [26]|Digital Art<br>Machine Learning|N/A|N/A|N/A|N/A|N/A|N/A||
|NFT Patent and IP [27]|Patent/IP|N/A|N/A|N/A|N/A|N/A|N/A|N/A|
|Parkchain [28]|Real-State|Ethereum|Solidity|Solidity|ERC721|N/A|||



Fig. 8: Federated model accuracy in different peers 

Fig. 10: Average block generation time with different number of transaction sets in a block. 

paper discusses the pros and cons of the underlying blockchain technologies and the use of non-fungible tokens to claim ownership of digital images. 

Fig. 9: Block generation time against the number of transactions in the block. 

the performance of the system through throughput and latency measured on a Hyperledger implementation. 

In **Parkchain** [28], Jennath et al propose a blockchainpowered parking solution for smart cities. It employs nonfungible tokens to represent unused land for setting up smart parking pools. The idea here is that landowners are willing to use their empty land for parking lots since parking is scarce in big cities. They create a non-fungible parking token for each parking lot in the given land. 

In **NFT-OpenGLAM** [22], Valeonti et al explore the opportunities and challenges in using non-fungible tokens as a means of fundraising for cash-strapped museums. There are some concerns about selling digital images as NFTs, and hence claiming ownership, while it is actually a piece of art made available for the public through OpenGLAM. The 

In **Fertile LAND** [24], Michael Dowling researches the pricing models of non-fungible tokens from an economic point of view. He looks into the varying virtual asset prices in Decentraland, a virtual land that allows one to use their creativity in exploring and/or owning one’s own virtual land. Since it is a virtual land, all assets including the land are digital assets that could be registered as non-fungible tokens. Decentraland employs Ethereum as a blockchain to register the digital assets and all transactions. 

In **Tokenfication** [25], Fowler and Pirker discuss the application of non-fungible tokens for game developers and content creators. They suggest ways in which NFTs may be used to protect ownership of digital assets in gaming such as the characters in the game, their characteristics and communication methods. Similar to exchanging game cards, players could exchange digital assets or use them in creating new games while paying the appropriate tokens as a fee to the creators. 

In **NFTGAN** [26], Shariar and Hayawi discuss ways to use generative adversarial networks (GANs) to automatically generate digital arts, and record their ownership on a blockchain as NFTs and sell them to interested art collectors. The paper explores different types of GAN techniques used to generate digital arts. The results show that the GAN generated is very close to real art in terms of the three mentioned categories. It 

200 

is clear from the study that GAN-generated digital art is here to stay and its potential as valuable NFTs is also high in the marketplace. 

In **NFT Patents and IP** [27], Bamakan et al examine the possibility of employing NFTs as patents for inventions. The traditional patent system is economically expensive as well as very time-consuming. The authors propose NFTs as a means to declare ownership of ideas and inventions. This would be cheaper and quick to declare new inventions in this system. More importantly, it can promote transparency and liquidity for the patents. The authors also propose some open research problems in this area. 

## VII. CONCLUSIONS AND FUTURE WORK 

In this paper, we have presented Indy528, a NFT and Model Card integrated federated learning model marketplace. Currently, NFTs are used to tokenise things like art, collectables, and even real estate. With this research, we are proposing an open platform Indy528 to represent federated machine learning(FML) models as NFT tokens. We have proposed a novel NFT token schema i528 to present ML models as NFT tokens. The FML models ownership, storage locations of the FML models and data provenance information(with Model Cards) are encoded into i528-based NFT tokens and stored in the decentralized ledger. To provide an innovative use case of Indy528, we propose NFT-based decentralized FML model sharing and trading in the marketplace. Model creators can train the FML models and publish them in the marketplace as NFT tokens. Parties who want to use the models can buy them from the model creators as NFT tokens. The buyers can view model information, accuracy, training process etc via the model card objects saved in the ledger. This results in enhanced transparency and auditability to the federated learning process while providing an open platform to share machine learning models. In the future, we are planning to integrate different NFT token schemes into the Indy528 platform. 

## ACKNOWLEDGEMENTS 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] Q. Wang, R. Li, Q. Wang, and S. Chen, “Non-fungible token (nft): Overview, evaluation, opportunities and challenges,” _arXiv preprint arXiv:2105.07447_ , 2021. 

- [2] R. F. Ciriello, “Tokenized index funds: A blockchain-based concept and a multidisciplinary research framework,” _International Journal of Information Management_ , vol. 61, p. 102400, 2021. 

- [3] J. Koneˇcn`y, H. B. McMahan, F. X. Yu, P. Richt´arik, A. T. Suresh, and D. Bacon, “Federated learning: Strategies for improving communication efficiency,” _arXiv preprint arXiv:1610.05492_ , 2016. 

- [4] T. Yang, G. Andrew, H. Eichner, H. Sun, W. Li, N. Kong, D. Ramage, and F. Beaufays, “Applied federated learning: Improving google keyboard query suggestions,” _arXiv preprint arXiv:1812.02903_ , 2018. 

- [5] T.-T. Kuo and L. Ohno-Machado, “Modelchain: Decentralized privacypreserving healthcare predictive modeling framework on private blockchain networks,” _arXiv preprint arXiv:1802.01746_ , 2018. 

- [6] M. Strobel, “Aspects of transparency in machine learning,” in _Proceedings of the 18th International Conference on Autonomous Agents and MultiAgent Systems_ , 2019, pp. 2449–2451. 

- [7] Y. Qu, L. Gao, T. H. Luan, Y. Xiang, S. Yu, B. Li, and G. Zheng, “Decentralized privacy using blockchain-enabled federated learning in fog computing,” _IEEE Internet of Things Journal_ , vol. 7, no. 6, pp. 5171–5183, 2020. 

- [8] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

- [9] D. Chirtoaca, J. Ellul, and G. Azzopardi, “A framework for creating deployable smart contracts for non-fungible tokens on the ethereum blockchain,” in _2020 IEEE International Conference on Decentralized Applications and Infrastructures (DAPPS)_ . IEEE, 2020, pp. 100–105. 

- [10] M. Miao, T. Jiang, and I. You, “Payment-based incentive mechanism for secure cloud deduplication,” _International Journal of Information Management_ , vol. 35, no. 3, pp. 379–386, 2015. 

- [11] F. V¨olter, N. Urbach, and J. Padget, “Trusting the trust machine: Evaluating trust signals of blockchain applications,” _International Journal of Information Management_ , p. 102429, 2021. 

- [12] M. Ashok, R. Madan, A. Joha, and U. Sivarajah, “Ethical framework for artificial intelligence and digital technologies,” _International Journal of Information Management_ , vol. 62, p. 102433, 2022. 

- [13] F. Victor and B. K. L¨uders, “Measuring ethereum-based erc20 token networks,” in _International Conference on Financial Cryptography and Data Security_ . Springer, 2019, pp. 113–129. 

- [14] M. Kamp, L. Adilova, J. Sicking, F. H¨uger, P. Schlicht, T. Wirtz, and S. Wrobel, “Efficient decentralized deep learning by dynamic model averaging,” in _Joint European Conference on Machine Learning and Knowledge Discovery in Databases_ . Springer, 2018, pp. 393–409. 

- [15] D. C. Nguyen, M. Ding, Q.-V. Pham, P. N. Pathirana, L. B. Le, A. Seneviratne, J. Li, D. Niyato, and H. V. Poor, “Federated learning meets blockchain in edge computing: Opportunities and challenges,” _IEEE Internet of Things Journal_ , 2021. 

- [16] E. Bandara, X. Liang, P. Foytik, S. Shetty, and K. De Zoysa, “A blockchain and self-sovereign identity empowered digital identity platform,” in _2021 International Conference on Computer Communications and Networks (ICCCN)_ . IEEE, 2021, pp. 1–7. 

- [17] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

- [18] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

- [19] E. Bandara, W. K. NG, K. De Zoysa, and N. Ranasinghe, “Aplos: Smart contracts made smart,” _BlockSys’2019_ , 2019. 

- [20] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa, and W. K. Ng, “Saas-microservices-based scalable smart contract architecture.” 

- [21] R. Dey and F. M. Salem, “Gate-variants of gated recurrent unit (gru) neural networks,” in _2017 IEEE 60th international midwest symposium on circuits and systems (MWSCAS)_ . IEEE, 2017, pp. 1597–1600. 

- [22] F. Valeonti, A. Bikakis, M. Terras, C. Speed, A. Hudson-Smith, and K. Chalkias, “Crypto collectibles, museum funding and openglam: challenges, opportunities and the potential of non-fungible tokens (nfts),” _Applied Sciences_ , vol. 11, no. 21, p. 9931, 2021. 

- [23] N. Karandikar, A. Chakravorty, and C. Rong, “Blockchain based transaction system with fungible and non-fungible tokens for a communitybased energy infrastructure,” _Sensors_ , vol. 21, no. 11, p. 3822, 2021. 

- [24] M. Dowling, “Fertile land: Pricing non-fungible tokens,” _Finance Research Letters_ , vol. 44, p. 102096, 2022. 

- [25] A. Fowler and J. Pirker, “Tokenfication-the potential of non-fungible tokens (nft) for game development,” in _Extended Abstracts of the 2021 Annual Symposium on Computer-Human Interaction in Play_ , 2021, pp. 152–157. 

- [26] S. Shahriar and K. Hayawi, “Nftgan: Non-fungible token art generation using generative adversatial networks,” _arXiv preprint arXiv:2112.10577_ , 2021. 

- [27] S. M. H. Bamakan, N. Nezhadsistani, O. Bodaghi, and Q. Qu, “Patents and intellectual property assets as non-fungible tokens; key technologies and challenges,” _Scientific Reports_ , vol. 12, no. 1, pp. 1–13, 2022. 

- [28] H. Jennath, S. Adarsh, N. V. Chandran, R. Ananthan, A. Sabir, and S. Asharaf, “Parkchain: a blockchain powered parking solution for smart cities,” _Frontiers in Blockchain_ , p. 6, 2019. 

201 

