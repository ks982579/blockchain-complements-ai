IEEE JOURNAL SUBMISSION 

1 

## A for AI Models based on Marketplace Trading Blockchain and Incentives for IoT Data 

Lam Duc Nguyen, _IEEE Graduate Member_ , Shashi Raj Pandey, _IEEE Member_ , Soret Beatriz, _IEEE Member_ , Arne Br¨oring, and Petar Popovski, _IEEE Fellow_ 

_**Abstract**_ **—As Machine Learning (ML) models are becoming increasingly complex, one of the central challenges is their deployment at scale, such that companies and organizations can create value through Artificial Intelligence (AI). An emerging paradigm in ML is a federated approach where the learning model is delivered to a group of heterogeneous agents partially, allowing agents to train the model locally with their own data. However, the problem of valuation of models, as well the questions of incentives for collaborative training and trading of data/models, have received a limited treatment in the literature. In this paper, a new ecosystem of ML model trading over a trusted Blockchainbased network is proposed. The buyer can acquire the model of interest from the ML market, and interested sellers spend local computations on their data to enhance that model’s quality. In doing so, the proportional relation between the local data and the quality of trained models is considered, and the valuations of seller’s data in training the models are estimated through the distributed Data Shapley Value (DSV). At the same time, the trustworthiness of the entire trading process is provided by the Distributed Ledger Technology (DLT). Extensive experimental evaluation of the proposed approach shows a competitive runtime performance, with a 15% drop in the cost of execution, and fairness in terms of incentives for the participants.** 

_**Index Terms**_ **—Blockchain, Federated Learning, Model Trading, Data Valuation, Shapley Value.** 

## I. INTRODUCTION 

Personal IoT devices keep generating an enormous amount of sensing data that is expected to reach 79.4 Zettabytes (ZB) globally in 2025 [1]. Several attempts to enhance and adapt business workflows have been made towards exploiting the provision of IoT data [2], [3]. In this regard, training machine learning models and data sharing are two popular uses of IoT data. Furthermore, emerging diverse platforms for accessing and sharing IoT data connects various distributed IoT devices/data sources, thereby facilitating suppliers to exchange their data [4]. 

For example, in IoT systems for air quality monitoring and emission control, Air Quality Index (AQI) is a quantity defined to estimate the degree of severity for air pollution and CO2 emission levels. AQI quantifies the concentration of various particles in the air, such as PM2.5 or PM5.0, using state-of-the-art sensor devices [5]. There are two most popular measurement methods for AQI: i) sensing-based [6], and ii) vision-based [7]. In the sensing-based method, the IoT sensor 

Lam Duc Nguyen, Shashi Raj Pandey, Soret Beatriz, and Petar Popovski are with Connectivity Section, Electronic System, Aalborg University, Denmark. Email: _{_ ndl, srp, bsa, petarp _}_ @es.aau.dk. 

Arne Br¨oring is Senior Key Expert Research Scientist with Siemens AG, Munich, Germany. Email: arne.broering@siemens.com 

**==> picture [194 x 135] intentionally omitted <==**

**----- Start of picture text -----**<br>
Machine Learning<br>Models<br>go<br>= Buyer<br>Models<br>of z ap @<br>Payment<br>OQ Op ©&<br>Seller Seller Seller<br>**----- End of picture text -----**<br>


Figure 1: A motivation example: IoT devices contribute to train an ML model for the buyer to predict CO2 emission levels and get incentives from their contributions. 

devices are delivered around the area interest, e.g., city, urban, to collect the quality of the air and emission levels. These measurements are then forwarded to the central server for further analysis and calculation of AQI. In the vision-based method, the devices with an embedded camera, such as a camera station in the road, or individual mobile phones, can take photos of a specific area and send them to the server. The server then applies advanced image processing techniques on these images to derive the analysis report of air quality. However, both methods have problems due to (i) high energy consumption for collecting data and transmission, (ii) requirement for a large dataset for a high quality AQI estimation, (iii) the server acting as a single point of failure, and (iv) data privacy concerns under General Protection Regulation (GDPR). Several recent works have addressed issues related to (i) using efficient resource management techniques and (ii) with dense sensory networks [8]. However, the primary concerns about (iii) and (iv) remain a single point of failure network topology and data privacy protection. They are yet to be addressed in an efficient manner. In addition, the regulations such as EU’s GDPR, California’s Consumer Privacy Act (CCPA), and China’s Cyber Security Law (CSL) [9] limits the reckless use/collection of personal information and fosters data privacy. Hence, a feasible joint solution to address challenges raised in (iii) and (iv) is imperative for optimized operation of the market for data exchange. 

In this regard, more recently, Federated Learning (FL) has been considered a key solution to address the privacy issue 

IEEE JOURNAL SUBMISSION 

2 

in training learning models [10]. FL is a distributed model training paradigm that aims to solve the challenges of data governance and privacy by training algorithms collaboratively rather than transferring the data itself. For example, in a typical FL setting, at first, as shown in Fig. 1, the IoT devices collect the pollution and CO2 emission levels and store them in their local database. Consider an interested organization or individual, termed a _buyer_ , willing to train an ML model to predict a specific area’s emission level. However, they do not possess sufficiently large datasets about sensing information or image data. In such a scenario, they can send their initial model to a model marketplace to find appropriate parties interested in contributing to the model training process. Then, the IoT devices, termed _sellers_ , can download the initial model and train it using their local data. After that, the IoT devices can share the updated model weights to the marketplace, where there is an aggregator to aggregate submitted local models to build global models. Based on the aggregated global models, the model buyer can use the global model to predict the emission levels with acceptable precision. In this manner, using the FL approach, we can address the problem of data privacy where data is locally trained without the need to transmit to a central server. However, from a systems perspective, a shared IT environment, such as an aggregator in the marketplace, may become a single point of failure in terms of data integrity, trust, security, and transparency [11]. 

A conventional data market is often deployed as a centralized service platform that gathers and sells raw or processed data from data owners (e.g., the trained learning models) to the consumers [12] [13]. This leads to two important concerns. _First_ , this strategy exposes the platform as a single point of security risk; the malfunctioning platform servers has serious security concerns including data leakage, inaccurate calculations results, and manipulation of data price. _Second_ , collaboration for model training raises questions in terms of how to motivate participants to participate in such an ML training endeavor. The incentive for each IoT client based on their contribution should be fair and transparent. These features are not present in a standard FL setup, as in many applications there is no clear and natural incentive mechanism for involved participants to provide quality information. This calls for a carefully designed mechanisms to reward parties economically and thus incentivize participation [14], [15]. For example, a fixed price per data point could motivate participants to collect massive amounts of low quality or fake data if there is no intermediary process to check quality of training data. Besides, another reason that may disincentives parties from sharing data could stem from privacy and integrity concerns regarding the use of participant’s data once it is shared. For instance, the sellers can re-use the data which has already been sold. 

The aforementioned challenges can be effectively handled by a Distributed Ledger Technologies (DLTs).[1] DLTs and Blockchains enable untrusted parties to share information in 

> 1In this work, the terms _Blockchain_ and DLT are used interchangeably. Blockchains are a type of DLT, where nodes maintain a copy of the ledger having embedded chains of blocks. These blocks are basically composed of digital pieces of information, particularly defined as _transactions_ . 

an immutable and transparent manner [16]. Outside of its key role in financial transactions, the applications of DLTs can be seen as a key enabler for trusted and reliable distributed IoT systems, e.g., a distributed IoT data marketplace For instance, in a Blockchain-enabled IoT data marketplace [17], Blockchain transactions include IoT sensing data, or system control messages, and these are recorded and synchronized in a distributed manner in all the involved participants of the network [18]. Furthermore, DLTs enable the preservation of all transactions in immutable records, with each record being spread across several participants. Thus, the decentralized nature of DLTs ensures security, as does the use of robust public-key encryption and cryptographic hashes. The advantages of incorporating DLTs into trading AI models in IoT systems include: i) ensuring immutability and transparency for historical AI model trading records, ii) eliminating the need for third parties, and iii) developing a transparent system for AI model trading in heterogeneous networks to prevent tampering and injection of fake data from the stakeholders, according to [19], [20]. With the wide spread of ubiquitous marketplaces recently, it became relevant to investigate the use of AI/ML model trading in marketplace environments. 

With the aforementioned motivation, we propose a Blockchain-based model trading system which enables a secured and trusted marketplace to collaboratively train ML models as well as guarantees fair incentives for every participants and privacy of data. Based on the quality of the uploaded models, which is quantified by using a distributed Data Shapley Value (DSV), the participants[2] can get the incentive based on the updated models, for example, as tokens or fiats. Note that based on our proposed system, the parties do not need to share their local data, but only provide customized models or query interface to the marketplace. Consequently, the proposed system allows multiple participants to jointly train the ML models on the marketplace based on their own training data. Buyers who need to train their ML model will pay to the market for the improvement of their model, and sellers who sell their contribution to train the ML models will get paid by the market via smart contracts. 

The main features of the proposed model trading are: 

- 1) **Trusted and transparent transactions** : The DLT is considered a trusted, tamper-proof, and transparent system in which the participants can check and follow the progress of a training task. Based on that, the model is exchanged and traded securely and transparently. 

- 2) **Valuation of Data:** The local models contributed by the trainers (service sellers) are collected and evaluated via Shapley Value (SV) extension to approximately estimate the quality of the models. 

- 3) **Fair Payment:** The participants receive their reward, which is proportional to the usefulness of their data in improving the models. The distributed incentive mechanism for FL based on SV measures participants’ contributions in the marketplace. 

> 2The terms “ _participants_ ”,“ _clients_ ” and “ _agents_ ” in this work are used to refer to “ _IoT devices_ ”. 

IEEE JOURNAL SUBMISSION 

3 

## _A. Contributions and Paper Organization_ 

- The major contributions are summarized as follows: 

- **ML Model Marketplace** : A Blockchain-based model trading system that allows participants to purchase learning models and sell contributions in training them. The system records the trading details [17] in a tamper-proof distributed ledger. 

- **Federated Data Shapley Value (SV):** We use the data SV to estimate the valuation of participants’ data and evaluate their contribution to the model during local training. We show that the standard SV value is inefficient for distributed ML and deploy an extension of standard SV for our platform. The method is robust and allows plugging any developed mapping functions related to the device’s local data into the proposed distributed Shapley mechanism for value quantification. As a result, one can design a contribution-based, efficient incentive mechanism to stimulate model trading. 

- **Performance Evaluation** : We have conducted extensive simulations and experiments, demonstrating that the proposed approach shows a competitive run-time performance, with a 15% drop in the cost of execution and fairness in terms of incentives for the participants. 

The rest of the article is organized as follows. Section II, presents the concepts of DLTs, FL, as well as definition of data valuation schemes used in this paper. This is followed by description of the system model of the marketplace for ML model trading and explain in detail how the system works. In Section III, the value of ML models is calculated using the _Approximate Federated Shapley Value (AFS)_ . Section IV contains description of the testbed and experimental results. Section V discusses related works and finally, Section VI concludes the paper. 

## II. PRELIMINARIES 

## _A. Standard FL_ 

FL is a distributed machine learning setting in which numerous entities (clients) cooperate on training a learning model without disclosing their available raw data [10]. Instead, clients distributively perform computations on their data and transfer obtained local learning parameters updates to the server for aggregation process. The aggregated model, i.e., the global model, is broadcast back to the clients for the next round of local computations resulting in the local learning parameters. The interaction between the server and clients to solve the learning problem continues until an acceptable level of model accuracy is achieved [21]. In this manner, FL offers (i) privacypreserving benefits in the model training approach by not requiring clients to share their local data to the server, and consequently, (ii) lower communication overhead by offering distributed model training paradigm and exchange of model parameters only. Therein, FL enables training AI/ML models at edge networks 

Fundamentally, there exists two main actors in the FL system: (i) the data owners, often termed as participants, and (ii) the model owner, which is the FL server. Consider a set of _N_ data owners, defined as _N_ = _{_ 1 _,_ 2 _, . . . , N }_ , where each of 

**==> picture [223 x 208] intentionally omitted <==**

**----- Start of picture text -----**<br>
Local<br>Training Server<br>ae<br>D, w Qe ® [=] we S.s<br>wi [>] Base station<br>Local Data<br>Local<br>Training<br>ce Aggregator<br>Ds [ut 5° awwl, ~ Base station  @ --><br>Local Data<br>...<br>... Aggregated<br>Local Model<br>Training<br>=) Lud RG; wal |<br>Base station<br>Local Data 'N with i d<br>Traning Nodes<br>**----- End of picture text -----**<br>


Figure 2: Standard FL. 

Table I: Summary of key notations. 

|**Notation**|**Meaning**|
|---|---|
|_MIi_|DLT miner _i_|
|_N_|Set of _N_ participants (clients)|
|_Di_|Private local dataset of user _i ∈N_|
|**M**_i_|Local model of user _i ∈N_|
|_MG_|Global model aggregated at DLTs|
|_M_0<br>_G_|Initial Global model at DLTs|
|_L_(**M**_t_<br>_i_)|Loss function|
|_η_|Learning rate|
|_Si_|Model trainer (or Seller) _i_|
|_Bi_|Model owner (or Buyer) _i_|
|_Pd_|Deposit from buyer|
|**B**|Training batch size|
|_A_|Training algorithm|
|_U_(_·_)|Utility function|
|_φi_|Valuation of data contributor _i_|
|_E_|Number of local epochs for a FL setting|
|_T_|Number of training interactions|
|_Ti_|A trade deal between _Si_ and _Bi_|
|�<br>_M_|Approximated model �<br>_M_|



them has a private dataset _Di∈N_ . In Table I, we provide the summary of key notations used in this paper. Each data owner _i_ trains a local model **M** _i_ using its dataset _Di_ and sends only the obtained local model parameters to the FL server. Then, the FL server aggregates all the collected local models to build a global model, _MG_ = » _i∈N[M][i]_[.][This][is][where,][in][princi-] ple, the FL approach differs from the traditional centralized training where _D_ = _∪i∈N Di_ is used to train a model _MT_ , i.e., data first gets aggregated centrally before the actual model training happens. In Fig. 2, we show a standard architecture and an overview mechanism of the FL training process. We assume that the data owners are honest, i.e., actual private data will be used for the local training, and correspondingly, the FL server will receive accurate local models from the data owners. Following to which, the workflow of standard FL can be described as below. 

First, considering the target application, the server de- 

IEEE JOURNAL SUBMISSION 

4 

**Algorithm 1:** Federated Averaging (FedAvg) Algorithm 

**Input:** Local mini-batch size **B** , number of participants per interaction _m_ , number of global interactions _T_ , number of local epochs _E_ , and learning rate _η_ 

**1** . **Output:** Global Model _MG_ 

**2** . **LocalTraining** ( _i_ , **M** ): Split local dataset _Di_ to mini-batches of size **B** which are included into the set _B_ . **3 for** each local epoch _j_ from 1 to _E_ **do 4 for** _each b ∈B_ **do 5 M** _←_ **M** _− η∇L_ ( **M** ; _b_ ); **6 Return M** to the server. **7 Initialized** _M_[0] _G_[.] **8 for** each interaction _t_ = _{_ 0 _,_ 1 _,_ 2 _,_ 3 _, . . . , T −_ 1 _}_ **do 9** _St ←_ (random set of _m_ clients); **10 for** each participant _i ∈St_ **in parallel do 11 M** _i[t]_[+1] _←_ **LocalTraining** ( _i, M[t] G_[)][;] **12** _M[t] G_[+1] = � _i∈N_ 1 _[D][i]_ � _Ni_ =1 _[D][i]_ **[M]** _i[t]_[+1] ; 

cides the training task and defines the corresponding data requirements. Furthermore, the server also specifies the hyperparameters of the global model and the training process, e.g., the learning rate _η_ . The server then broadcasts the initialized global model _M_[0] _G_[and the learning task to a subset of selected] participants. Next, based on the global model _M[t] G_[,][where] _[t]_ denotes the current global iteration index, i.e., the communication rounds between the participants and the server, each participant uses its local data to update their model parameters _Mi[t]_[. In doing so, during iteration] _[ t]_[, the participant] _[ i][ ∈N]_[aims] at finding the optimal parameters _M[t] i_[that][minimize][the][local] loss problem _L_ ( _Mi[t]_[)][,][defined][as][the][finite-sum][of][empirical] risk functions as 

**==> picture [174 x 21] intentionally omitted <==**

We formally call it _local iteration_ . Subsequently, the obtained local model parameters from participants are sent back to the server, where they are aggregated to get the global model parameters _MG[t]_[+1] . Eventually, the global model is then broadcast back to the data owners for the next round of local iteration; hence, the iterative process is continued. In doing so, the server minimizes the global loss function _L_ ( _MG[t]_[)][as][the][following] approximation in the distributed setting of FL: 

**==> picture [192 x 21] intentionally omitted <==**

Note that the FL process can train different ML models that essentially use the SGD method such as Support Vector Machines (SVMs), neural networks, and linear regression. 

However, a single server dependency in the traditional FL framework makes the system vulnerable to threats, such as when the server behaves maliciously. Therefore, integrating FL with DLTs should be a promising approach to address limitations [22]. 

## _B. Distributed Ledger as a Service for FL_ 

DLT is a peer-to-peer distributed ledger that records transactions in a network in a transparent and immutable manner. Besides, smart contract, which is considered as a key innovation in DLT/Blockchain area, provide programmability contracts to the DLTs, in the sense that the defined agreements in contracts are executed autonomously. With the mentioned nature advantages of DLTs and smart contract, the FL framework running on the top of DLT should be completely distributed and avoid the single point of failure issue. 

In the DLT-based FL, we assume each client device is always connected to one of the DLT miners and, if the physical connection with the current DLT miner becomes unavailable, then the device will be automatically associated with another DLT miner. In each miner-device pair, the DLT miner works as the leader of the associated IoT devices, and they are responsible for uploading and downloading data or training models. During the training process, the IoT device downloads the latest global model recorded in the ledger and trains for the updated version of the local model using their private local data. After completing the local training, the device uploads the local model to the paired DLT miner and the global aggregation process starts. In the training time, all involved IoT devices are allow to download the latest information of associated DLT miners to receive the evaluation of the IoT devices and global model updates. Finally, each IoT device publishes its local training model and enters to a new round of local iteration using the newest version of the obtained global model. In this manner, the iterative ML model training process is operated until the global model has achieved a satisfactory accuracy or convergence. 

Each miner has its verifier and block to ensure that the real models and the contributions of devices are updated. Each 

**==> picture [253 x 252] intentionally omitted <==**

**----- Start of picture text -----**<br>
FL-MNIST FL-CIFAR10<br>1.00 0.8<br>0.7<br>0.98<br>0.6<br>0.96<br>0.5<br>0.94<br>0.4<br>FL-MNIST E=1 FL-MNIST E=1<br>0.92 FL-MNIST E=3 0.3 FL-MNIST E=3<br>FL-MNIST E=10 FL-MNIST E=10<br>0.90 0.2<br>0 20 40 60 80 100 0 20 40 60 80 100<br>Epochs Epochs<br>DLT-FL-MNIST DLT-FL-CIFAR10<br>1.00 0.6<br>0.5<br>0.98<br>0.4<br>0.96<br>0.3<br>0.94<br>0.2<br>FL-MNIST E=1 FL-MNIST E=1<br>0.92 FL-MNIST E=3 0.1 FL-MNIST E=3<br>FL-MNIST E=10 FL-MNIST E=10<br>0.90 0.0<br>0 20 40 60 80 100 0 20 40 60 80 100<br>Epochs Epochs<br>Accuracy Accuracy<br>Accuracy Accuracy<br>**----- End of picture text -----**<br>


Figure 3: The accuracy of Standard FL and DLT-based FL. 

5 

IEEE JOURNAL SUBMISSION 

block contains a head and body parts. The blockhead contains a pointer to the next block, and the body part contains a set of validated transaction information. The local models are formed in transaction format and in order to make the solution scalable, the local models are recorded in IFPS storage, such that just a hash version of the models is recorded in the distributed ledger. The basic comparison between standard FL and DLT-based FL is presented in Fig.3. The accuracy is similar in both standard FL and DLT-based FL, but the time required for convergence of DLT-based FL is higher than standard FL because of extra verification and consensus in the system. 

## _C. Data valuation using Shapley value_ 

Game theory is an economic tool best-suited to analyze a system where two or more participants get involved in to achieve a desired payoff. The Shapley Value (SV) is a solution concept of fairly distributing the incentive and payoff for the involved parties in coalition [23]. In this regard, the SV applies mainly in scenarios where the contributions of each involved participant are unequal, but all the participants work in cooperation with each other to achieve the payoff. The SV of user _i_ is defined as the average marginal contribution of _i_ to all possible subsets of _D_ = _{D_ 1 _, D_ 2 _, . . . , DN }_ formed by other users as 

**==> picture [227 x 30] intentionally omitted <==**

where the function _U_ ( _·_ ) gives the value for any subset of those users, e.g., let _S_ be a subset of _N_ , then _U_ ( _S_ ) gives the value of that subset. This captures the average value of the contributions of user _i_ for subsets of all coalition of users. Intuitively, assume that the user’s data is to be collected in a random order, and that every user _i_ receives its marginal contribution for the collected data. If we average these contributions over all the possible orders of _N_ users, we obtain _φi_ ( _N, U_ ). The importance of the SV is that it is the unique value division scheme that satisfies the following desirable properties described as follows. 

- _**Symmetry**_ : For all _S ⊆N \ {i, j}_ , if user _i_ and _j_ are interchangeable, and _U_ ( _S ∪{i}_ ) = _U_ ( _S ∪{j}_ ), then, _φi_ = _φj_ . Thus, the users _i_ and _j_ contribute the same amount to every coalition of the other agents. Besides, the symmetry axiom states that such agents should receive the same payments. 

- _**Dummy User** :_ User _i_ is considered as dummy user if the amount that _i_ contributes to coalition is exactly the amount that _i_ is able to achieve alone, i.e., _∀S, i ∈/ S, U_ ( _S∪{i}_ ) _−U_ ( _S_ ) = _U_ ( _{i}_ ). According to the dummy user axiom, dummy users should be compensated exactly for the amount they achieve on their own. Users that make zero marginal contributions to all subsets of the data set, on the other hand, earn no compensation, for example, _φi_ = 0 if _U_ ( _S ∪{i}_ ) = 0 _, ∀S ⊆N \ {i}_ . 

- _**Additivity** :_ For any two _U_ 1 and _U_ 2, we have for any user _i_ , _φi_ ( _N, U_ 1 + _U_ 2) = _φi_ ( _N, U_ 1) + _φi_ ( _N, U_ 2), where the 

game ( _N, U_ 1+ _U_ 2) is defined by ( _U_ 1+ _U_ 2)( _S_ ) = _U_ 1( _S_ )+ _U_ 2( _S_ ) for every coalition _S_ . 

Based on these background knowledge, we designed a distributed marketplace for trading AI/ML models based on Blockchain and incentive mechanism in IoT environment. 

## III. SYSTEM DESIGN AND ANALYSIS 

## _A. System Components_ 

The general architecture of DLT-based model trading includes three main components: model owners or buyers _B_ , model trainers or sellers _S_ , and a distributed ledger, shown in Fig. 4. We assume that each seller or buyer owns one device in the network. Within a deal (a trade) by _Ti_ , the seller _Si ∈ S_ and buyer _Bi ∈ B_ communicate using wireless links. The AI/ML model trading procedure occurs to complete a trade between _Si_ and _Bi_ , exchanging models _Mi ∈M_ and payment _Pi_ . First and foremost, _Bi_ completes the deposit _Pd_ to _Si_ via smart contracts in reference to the requested training models, _Mi_ . After the sellers complete the requests of the buyers, in terms of accuracy, convergence time, etc, the smart contracts are autonomous executed to pay for the effort of sellers using the amount of deposit _Pd_ from buyers. Following Fig. 4, the general procedure of interaction between a single buyer _Bi_ and a single seller _Si_ can be described as follows: 

_1) Model Owner i (as buyer Bi): Bi_ could be an individual or organization who needs model training. _Bi_ sends a request _bi_ including task type, budget, deposit, amount of data, quality of data, price, discount, etc, to the marketplace via smart contracts. _bi_ will be transmitted to selected _Si_ and recorded in the ledger via transaction _Ti,_ add. After receiving the trained and aggregated models from _Si_ and marketplace which fulfills requirements regarding to, e.g., accuracy, the _Bi_ generates a transaction _Ti,_ commit which executes payment from _Bi_ ’s wallet to smart contract, forwards the payment to sellers, and generates a acknowledgment message back to the distributed ledger. 

_2) Model Trainer (as seller Si):_ The model trainers play two main roles in the system: i) collects sensing data from the environment (e.g., data from surveillance systems, environmental sensing data, and geographical data), or acts as a data hub gathering data from nearby physical devices; ii) subscribes the model training requests from the buyers, and train the models downloaded from the marketplace with the local data. Seller _Si_ earns the payment _Pi_ from _Bi_ after successful delivery of _Mi_ to _Bi_ . After the trained models achieve a certain accuracy based on the predefined agreements in the smart contract system, upon the appearance of _Ti,_ commit generated by _Bi_ , the seller _Si_ can receive the payment, e.g., via tokens, _Pi_ , which is in fact the deposited amount _Pd_ by the buyer _Bi_ , from the marketplace via smart contract. Finally, it confirms to the distributed ledger that the deal _Ti_ is completed via an acknowledgment message. 

_3) Distributed Ledgers:_ The Blockchain maintains a distributed ledger that stores the history of all traded models in the form of blocks, which are connected in a chronological order. On top of that, the smart contracts are deployed to autonomously control the order and execute payments, e.g,. 

6 

IEEE JOURNAL SUBMISSION 

**==> picture [409 x 272] intentionally omitted <==**

**----- Start of picture text -----**<br>
Payment Channels<br>Local 4 9<br>S46 Training ° = o<br>3 1<br>eta) o.@<br>Di © | Be 0 »<br>Local Data Seller Qs Re Base station ip 2 o> Buyer<br>7<br>8<br>) 2 Local Sync<br>os Training aa © O<br>5<br>Version Nonce<br>Hash of Prev Block<br>Local<br>Base station<br>Local Data Seller Updates 6 Merkle<br>Root Target<br>... ...<br>... State<br>Sync<br>Local<br>Training TX TX TX TX<br>Global<br>Updates<br>5al ne @. (ooo<br>lug @® — *e"i: v|<br>Base station DLT Network<br>Local Data<br>Seller<br>Traning Nodes<br>[<br>...<br>...<br>**----- End of picture text -----**<br>


Figure 4: DLT-based ML Model Trading Framework Architecture. 

large payment or micro-payments from involved participants without the need of human intervention. In a distributed manner, the smart contracts ensure transparency, trust and automotive of exchanging data among parties. These features can be deployed based on the negotiation between model owners and customers via _Ti,_ deploy. Furthermore, any change in smart contracts, for example, the amount of data or the model price, or updates in the discount offers, can be made via _Ti,_ update. 

In a trading system, there are an enormous amount of data exchanged among parties. Thus, increasing the number of transactions leads to slower transaction processing time and, consequently, the system’s overall speed. This is reasonable as every Blockchain node needs to store and execute a computational task to validate every single transaction. Therefore, to minimize the cost of storage and execution, the trading system should record only the important data, such as payment history, aggregated global models, which could be hashed and recorded at the distributed ledger. Meanwhile, the raw data can be recorded in the distributed off-line storage component. In detail, after both model sellers _Si_ and model owners _Bi_ have fulfilled requirements defined by smart contracts, the _Ti,_ settle is autonomously executed to query the payment _Pi_ from _Bi_ . Then, the payment _Pi_ is transferred to _Si_ ’s private wallet, while the aggregated model _Mi_ is delivered to the storage address of _Bi_ . In the scope of this study, we assume that the data services (e.g., data storage, trading and task dispatching) are implemented on top of a permissionless Ethereum Blockchain [24]. In this work, the control data and AI/ML models are formatted into normal Ethereum transactions. Furthermore, in order to improve efficiency, only the digest of each transaction 

is recorded in the distribute on-chain ledger, and the raw data is stored off-chain by using IPFS (InterPlanetary File System). 

## _B. Communication Workflow_ 

In the DLT-based FL model trading network, we revisit the notation used to denote data owners and define the number of participants as _N_ = _{_ 1 _,_ 2 _,_ 3 _, . . . , N }_ . The miner _MIi_ of DLT network is associated randomly with the IoT device. For simplicity, we consider the case which one miner is assigned to each physical IoT device. Each IoT device has to determine its own learning task-related dataset size and upload it to the ledger system to receive a reward. A distributed SV incorporates the quality of local data[3] to determine the corresponding quality of the local model. We realize that, in some case e.g healthcare, it would be better to extract features and measuring the quality based on real quality and not quantity, but it is out of this research scope. In addition, in resource-constraint IoT environments, to reduce latency and optimize energy consumption, each device’s local controller performs local optimizations to establish the best scheduling policy for device resources, such as CPU cycles scheduling. 

The workflow of the system is described as below. 

_Step 1 (Model Initialization)_ : The buyer _Bi_ initiates a model _Mi_ which needs to be trained and publishes to the DLTbased marketplace. The initial model is formed in the DLT transaction format _T_ publish. 

_Step 2 (Publish initialized model to the ledger)_ : Then, the transaction _T_ publish including initialized model _Mi_ is verified 

> 3The quality of data signifies the size of dataset used in model training, similar to [25]. In this study, we do not consider feature attributes of data to quantify its quality. 

IEEE JOURNAL SUBMISSION 

7 

and recorded to the distributed ledger. At the same time, there might be many available models on the marketplace. 

_Step 3 (Model Seller download train-required models)_ : The potential seller _Si_ can see the list of available models on the marketplace and choose to download a copy of one or multiple models of interest to train with its local data of _Si_ . 

_Step 4 (Local Model Training)_ : After downloading the models from the distributed ledger, the sellers train the model based on their local data. The device, i.e., the seller _Si_ , has its own local dataset _Di_ . Local model training aims to minimize the loss function _f_ ( **M** _i, Si_ ), where **M** _i_ is the local model of device _di_ and _Di_ is its local dataset. 

_Step 5 (Local trained model is updated to the ledger)_ : Next, the device _Si_ is randomly associated with the miner _MIi_ to which it uploads the trained local model to the distributed ledger via smart contract. The smart contract has functions to record the updated local models from clients via DLT interface, e.g., Web3. 

_Step 6 (Cross-verification of the local models):_ After receiving the local model published by IoT device in the format of transactions, the DLT miner _MIi_ put the local model in newly generated blocks and broadcasts the model to other DLT miners in the network. Next, until other DLT miners receive the broadcasted blocks, including the local models of clients, they will verify the accuracy of local models and put the models to the new generated blocks. During this process, all the aggregated models are broadcasted to the all DLT miners in the system, and DLT miners will compare the consistency and accuracy among aggregated models. To that end, the most one will be chosen as the correct global model. Then, DLTs miners record the correct global model and the contribution of the IoT devices into the distributed ledger via smart contracts features. Otherwise, the rest of global models are considered as faulty updates. 

_Step 7 (The generation and propagation of blocks)_ : In order to generate a new block in the distributed ledger, DLT miners need to compute a block hash for mining and solve a cryptographic puzzle based on SHA-256, which is a one-way hash function. As defined in popular Proof-of-Work (PoW) Blockchain, e.g., Bitcoin, Ethereum, DLT miners perform a PoW algorithm until it finds a desired nonce value or receives a new generated block from other DLT miners [26]. There is a case, however, that the _MIi ∈ MI_ acts as the DLT miner that finds the needed nonce value at the earliest, and its candidate block is generated as a new block and propagated to the other DLT miners in the network. Meanwhile, the chain can be engaged in forked problem in which multiple DLT miners find out a nonce value at the same moment. To address this issue, we use an ACK message that allows DLT miners to transmit only when each DLT miner gets the new block, which determines whether there is a fork on the main chain. Then, the DLT miner _MI_ 0, which creates that newly generated block, will wait for a waiting time defined by the block ACK. Otherwise, if a fork is generated again, the process back to to previous phase to resolve the issue. 

_Step 8 (Settlement)_ : After the model accuracy achieves a particular value in the smart contract, the smart contract settles the deal between buyer and sellers. The finalized model is 

**Algorithm 2:** Standard Federated Shapley FedAvg 

**Input:** Local mini-batch size **B** , number of participants _m_ per interaction, _T_ is number of global interactions, number of local epochs _E_ , and learning rate _η_ . **Output:** : _M[T]_ **1** . **Initialize** _M_[0] _S_[,][where] _[S][⊆N]_[=] _[ {]_[1] _[,]_[ 2] _[, . . . , n][}]_[.] **2 for** each subset _S ⊆N_ **do 3 for** each round _t_ = _{_ 0 _,_ 1 _,_ 2 _, . . . , T −_ 1 _}_ **do 4 for** each client _i ∈S_ = _{_ 1 _,_ 2 _,_ 3 _, . . . , n}_ **do 5** Transmit _M[t]_ to _n_ clients; **6** _M[t] i[←][ModelUpdate]_[(] _[i,][ M][t]_[)][;] **7** _δS_[(] _[t] ,i_[+1)] _←M[t] S,i[−M][t]_[;] _|Di|_ **8** _M[t] S_[+1] _←M[t] S_[+][ �] _[n] i_ =1 � _ni_ =1 _[|][D][i][|][·][ δ] S[t]_[+1] _,i_[;] **9 for** _i ∈S_ = _{_ 1 _,_ 2 _,_ 3 _, . . . , n}_ **do 10** _φi_ = _n_[1] � _S⊆N \{i} U_ ( _M[t] S∪_ ( _{ni|S|}−_[)] 1 _[−]_[)] _[U]_[(] _[M] S[t]_[)] ; **11 Return** _M[T]_ , and _φ_ 1 _, φ_ 2 _, . . . , φn_ . **12 ModelUpdate** ( _i, M_ ): **13 for** local epoch _e_ = _{_ 1 _,_ 2 _,_ 3 _, . . . , E}_ **do 14 for** batch _b_ **do 15** _M ←M − η∇L_ ( _M_ ; _b_ ); **16 Return** _M[T]_ to central server. 

updated to the buyer and the incentive is funded to the sellers. 

_Step 9 (Incentive to Sellers)_ : Based on the contribution of each seller, the smart contract computes their contribution and transfer to sellers appropriate funds. The smart contract provides a mechanism of transparent and immutable recording and accounting contribution logs on the distributed ledger. Based on the contribution history from ledger, the clients can receive the incentives and rewards in tokens via off-chain payment channels. 

_Step 10 (Record receipt to the Ledger)_ : All bills and receipts are recorded immutably in the distributed ledger, which allows participants to check and control their deal. Besides, we also implemented the off-chain storage solution named IPFS to store hashes of data locations on the ledger instead of raw data files. The hashes can be used to query the exact file or models through the DLT systems. 

## _C. Distributed Shapley Value (DSV) Calculation_ 

The Standard Federated Shapley Value (SFSV) calculates the SV of data contributors based on equation (2). SFSV trains federated models based on the different subsets _S_ of contributors, and these models are evaluated on the standard test set. However, computing the SV directly according to SFSV is time-consuming because models on all the combinations of data sets need to be trained and evaluated. By default, the SV computes the average contribution of a data source to every possible subset of other data sources. So that, evaluating the SV incurs significant communication and computation cost when the data is decentralized [27]. Consequently, for data SV 

IEEE JOURNAL SUBMISSION 

8 

|**Algorithm 3:** AFS Algorithm|**Algorithm 3:** AFS Algorithm|||
|---|---|---|---|
||**Input: Input**: Local minibatch size **B**, number||of|
||participant _m_ per interaction, _T_ is number of|||
||global interactions, number of local epochs _E_,|||
||and learning rate _η_.|||
|**1 **|**Output: Output**: _MT_ , and _φ_1_, φ_2_, . . . , φn_.<br> **Initialize** _M_0_,_ �<br>_M_0<br>_S_, where _S ⊆N_ =_{_1_,_2_,_3_. . . , N}._|||
|**2 **|**for** _each round t_=_{_0_,_1_,_2_, . . . , T}_ **do**|||
|**3**|Transmit _Mt_ to _i ∈S_ clients;|||
|**4**|_Mt_<br>_i ←ModelUpdate_(_i, Mt_);|||
|**5**|_δt_+1<br>_i_<br>_←Mt_<br>_i −Mt_ , _∀i ∈N_;|||
|**6**<br>**7**<br>**8**|_Mt_+1 _←Mt_ + �_n_<br>_i_=1<br>_|Di|_<br>�_n_<br>_i_=1 _|Di|_ _· δt_+1<br>_i_<br>;<br>**for** _each S ⊆N_ **do**<br>�<br>_Mt_+1<br>_S_<br>_←_�<br>_Mt_<br>_S_ +�<br>_i∈S_<br>_|Di|_<br>�<br>_i∈S |Di|_ _· δt_+1<br>_i_||;|
|||||
|**9 **|**Initialize** _m_= 0.|||
|**10 **<br>**11**<br>**12**<br>**13**<br>**14**<br>**15**<br>**16**<br>**17**<br>**18**|**while** Convergence criteria not meet **do**<br>_m_=_m_+ 1;<br>_πm_: random permutation of clients with <br>samples to collaboratively train _MT_ ;<br>_vm_<br>0 _←U_(�<br>_M_0<br>_∅_);<br>**for** _n ∈{_1_,_2_, . . . , |S|}_ **do**<br>**if** _|U_(_M|S|_)_−vm_<br>_n−_1_| < PT_ **then**<br>_vm_<br>_n_ =_vm_<br>_n−_1;<br>**else**<br>_S ←{πm_[1]_, πm_[2]_, . . . , πm_[_n_]_}_;<br>_MT_<br>_S ←_�<br>_i∈S_<br>_|Di|_<br>�<br>_i∈S |Di|_ _·_ �<br>_MT_<br>_i_ ;<br>_vm_<br>_n ←U_(_MT_<br>_S_);|data||
|**19**|_φπm_[_n_] _←m−_1<br>_m φπm−_1[_n_]+ 1<br>_m_(_vm_<br>_n −vm_<br>_n−_1);|||
|**20 **|**Return** _MT_ , and _φ_1_, φ_2_, . . . , φn_.|||
|**21 **<br>**22 **|**ModelUpdate** (_i, M_):<br> **for** local epoch _e_=_{_1_,_2_,_3_, . . . , E}_ **do**|||
|**23**<br>**24**|**for** batch _b_ **do**<br>_M ←M −η∇L_(_M_;_b_);|||
|**25**|**Return M** to ledger.|||



in the FL environment, the methods in [25], [28] to calculate SV introduce extra training rounds on combinations of datasets from different data providers. Furthermore, the cost for extra rounds for training models could be expensive when the data volume is large. Therefore, there is a need for new strategies to evaluate the data value in FL. 

The main idea to that end is to exploit the gradients information during the training process of the global model _M_ to approximately reconstruct the local models trained with different combinations of the client’s datasets. Thereby, our approach (as described in Algorithm 3) eliminates the burden for the local models to be frequently re-trained to evaluate clients’ contributions. In fact, the SV does not consider the order of data sources. However, in FL, it is of significant importance to take into account the order of data used for the model training so as to ensure a fair convergence. Furthermore, the updates of model are enforced to diminish over time by using, for example, a decaying learning rate [29]. Hence, 

the sources used towards the end of the learning process could be less influential than those used earlier. Therefore, to accommodate these attributes of learning properties in the decentralized model training paradigm of FL, we need to define new and efficient ways to compute SV. In this regard, based on the neutrality of FL, the SV for FL (FSV) could be computed in two different strategies. 

The first method (called Single-Cal) reconstructs models by updating the initial global model _M_ in FL with the gradients in different rounds and calculates the FSV by the performance of these reconstructed models. For example, if we want to reconstruct the model of _M_ ( _i,j_ ) trained on the datasets of _Di_ and _Dj_ of corresponding users, use the gradients information from sellers _i_ and _j_ in each round to update the initial global model _M_ generated by the buyer. Then, the contribution is calculated using equation (2). 

The second method (called Multi-Cal) calculates FSV in each training round by updating the global model _M_ from the previous training round with the value of gradients in the current training round. Next, the FSVs are aggregated from multiple rounds to get the final result. Therefore, there is no extra training process needed; these methods are considered efficient. The main difference between these two strategies is that the first method approximates models through complete global iterations and only evaluates them to find SV afterwards. The second one approximates and evaluates models for every global iteration and calculates the marginal contribution for each global iteration. So that makes the second method more computationally expensive than the first one. To address this issue, we propose a new algorithm AFS based on the first approach with the use of Truncated Monte-Carlo (TMC) [25] in Algorithm 3. In principle, AFS is an engineered derivative of the TMC algorithm to characterize clients’ contributions with their available data samples in the collaborative training framework. In doing so, AFS evaluates the marginal contribution of each client instead of a subset of training data points, unlike the TMC method; thus, allowing clients to engage in the ML model trading with the offered incentive signals based on their data contributions. 

Specifically, the first part of the algorithm shows the operation of the distributed ledger, lines 1–8 and lines 10–19. In line 1, the global model _M_[0] and reconstructed models based on different chosen subsets _S ⊆N_ = _{_ 1 _,_ 2 _, . . . , N }_ are initialized. Next, the distributed ledgers broadcast the global model _M[t]_ to _n_ selected clients in each global training round _t_ in line 3, and then receive the updates _M[t] i_[from][these] clients in line 4 and the gradients of clients _δi[t][,][ ∀][i][∈S]_[for] model aggregation are computed. After that, the global model is updated in line 6 as 

**==> picture [206 x 28] intentionally omitted <==**

Next, instead of updating all the local models _M[t] i[,][ ∀][i][∈S]_ in every global interactions _t_ = _{_ 0 _,_ 1 _, . . . , T }_ , we can update only _n_ models _M_[�] _[t] i_[and][compute] _[M][T] S_[directly][as][a][weighted] average at the end, as _M_[�] _[t] i_[+1] _← M_[�] _[t] i_[+] � _ni_ =1 _|Di[|][D][i][|][δ] i[t]_[+1] . 

IEEE JOURNAL SUBMISSION 

9 

We observe that evaluation of a model incurs a considerable cost in terms of time, especially if the test set is large. And with the basic idea of a single-round algorithm, we must reconstruct and evaluate 2 _[n]_ models. Hence, we applied the method of TMC to decrease the computation cost and developed a tailored variant of TMC, i.e., the AFS algorithm, to address this issue. The details of the adapted TMC method is as follows. First, we sample a random permutation of clients _π[m]_ with their data samples used to train the global model [28]. After that, we scan from the first clients to the last client and calculate the marginal contribution of every new client’s data in the training process. By repeating this process over multiple permutations, the approximation of SV is the average of all the calculated marginal contributions. The while loop is run until certain convergence criteria are met. In this work, we stop the loop when the average percentage change after a TMC iteration _m_ is less than a certain threshold. 

In line 21, the trained federated model _M[T]_ and the SVs are finally obtained. The local training part for the clients (lines 22–25) show how the clients use private data to train the model received from the distributed ledger. The clients use the classical gradient descent algorithm and report their updated local models **M** _i|i_ = _{_ 1 _,_ 2 _,_ 3 _,...,n}_ to the distributed ledger. 

## _D. Performance bound on AFS algorithm_ 

An analytical bound on the AFS algorithm can be derived by taking the properties of TMC sampling approach into account [25]. We consider AFS estimates the contribution of the individual client in the federated setting for a supervised learning task with probability at least (1- _α_ ) that our estimator error is _ϵ_ . Then, we are interested in evaluating the general performance bound on AFS such that 

**==> picture [201 x 163] intentionally omitted <==**

**----- Start of picture text -----**<br>
50<br>20<br>10<br>5<br>0 [2]<br>0 0.02 0.04 0.06 0.08<br>**----- End of picture text -----**<br>


Figure 5: Performance analysis of AFS algorithm. 

_−_ 2 _|S|_ ∆[2] E( _φi_ )) _≥_ ∆ _≤_ 2 exp _v_[2] . Taking the average of � � � marginal contributions on the left-hand side of the inequality, and combining it with (4), we get Pr( _|φ[F] − φ[S] | ≥ ϵ_ ) _≤ −_ 2 _|S|ϵ_[2] 2 exp _v_[2] . This concludes the proof. � � 

In Fig. 5, we show the results on performance analysis of AFS algorithm. We observed variability in the minimum permutation _|S|_ required to ensure a defined deviation between the average value of contributions across clients, as measured using AFS, and the standard SV. For tighter bounds, the number of required permutations is large. This is intuitive, as the distributed ledger expects a larger sampling value _|S|_ to define better confidence bound on the performance that minimizes the approximation error using AFS. 

**==> picture [176 x 12] intentionally omitted <==**

where _φ[S]_ = _< φ[S]_ 1 _[, φ][S]_ 2 _[, . . . , φ][S] n[>]_[is][the][vector][of][Shapley] contributions generated by the standard SV and _φ[F]_ = _< φ[F]_ 1 _[, φ][F]_ 2 _[, . . . , φ][F] n[>]_[is][the][approximation][FSV][using][the][pro-] posed AFS method. Assume that we know the data distribution of clients to evaluate its marginal contribution. Then, the sampling _|S|_ made during the evaluation process reflects the bound on the obtained approximation of AFS. Without loss of generality, we assume it is possible to quantify the range _v_ of the client’s data marginal contribution in improving the global model. Then, we have the following Lemma 1. 

**Lemma 1.** _Considering the TMC sampling approach [25] for evaluating the Shapley value, we have a minimum permutation |S| for a known range of the client’s marginal contribution v defining an upper bound of O v on AFS such that α ≥ |S|_ � � _−_ 2 _|S|ϵ_[2] 2 exp _v_[2] _satisfies for_ 0 _≤ ϵ, α ≤_ 1 _._ � � 

_Proof._ The proof can be derived using Hoeffding theorem [30] for a known range of marginal contribution of clients. In practice, the distributed ledger can reuse the average of marginal contributions of clients, with known range _v_ , to derive a sampling permutation _|S|_ . Then, we have Pr _i∈S⊆N_[(] _[φ][i][ −]_ �� 

## IV. PERFORMANCE EVALUATION 

## _A. Experimental Settings_ 

To demonstrate the applicability of our proposed system, we implement a proof-of-concept for the trading model in an IoT network. In this section, we introduce enabling technologies involved with the prototype. 

_1) Distributed Ledger:_ In this study, we implement Ethereum platform for the experimental. Ethereum[4] is a distributed public blockchain network that focuses on running programming code of any decentralized application. Specifically, Ethereum is a platform for sharing information across the globe that cannot be manipulated or changed. Ethereum has its own cryptocurrency, called _Ether_ (ETH), and its own programming language, called _Solidity_ . The decentralized applications on the network is called ~~_D_~~ _apps_ . Practically, Ethereum provides a convenient platform for development and smart contracts system to integrate with FL. We run Ethereum network via Ganache[5] which is a personal blockchain for rapid Ethereum distributed application development. 

> 4https://ethereum.org/ 

> 5https://www.trufflesuite.com/ganache 

IEEE JOURNAL SUBMISSION 

10 

Table II: EXECUTION COSTS OF SMART CONTRACTS 

Figure 6: Blockchain-enabled model trading testbed. The testbed includes DLT Ethereum network running over Ganache, IPFS storage to address scalability issue, monitor dashboard and 5 IoT raspberry devices standing for marketplace participants as well as DLT clients. 

_2) Datasets:_ In the scope of this study, we conducted the experiments on the MNIST data set. The dataset contains around 60,000 training images and over 10,000 testing images. Each client holds a part of dataset locally depending on the scenarios. 

_3) IoT Devices and Workstation:_ We use Raspberry Pi 3 with the following configurations: Pytorch, OS Raspbian GNU/Linux 10, and Python version 3.7. We note that CUDA is not available for the model. The workstation has the system configurations as CPU i7-7700HQ, GPU GTX, Pytorch, OS Linux Ubuntu 20.04 , Python version 3.7.8 using Anaconda, and the CUDA version 11. These IoT devices are connected via Wifi access point. 

_4) Evaluation Metrics.:_ We consider several performance metrics for comparison. 

- **Cost of Smart Contract:** We study the fees made by users to compensate for the computing energy required to process and validate transactions on the Ethereum. 

- **Incentive per worker:** The amount of tokens delivered to sellers based on their contributions of training the model. 

- **Maximum Different** : The performance score function _U_ is chosen to be the accuracy function. The SVs are then calculated according to the different schemes. For comparison of the accuracy of the SV, all SVs calculated are first standardized by scaling them by a common factor such that _ni_ =1 _[φ][i]_[= 1][.] 

This is appropriate because profit distribution will likely be based on the percentage contribution. Then, the **maximum different** _Dmax_ measures the maximum difference that a data provider should be allocated by the definition and by approximated calculation. The calculation is shown as below: 

**==> picture [187 x 18] intentionally omitted <==**

|**Smart Contracts**<br>Contract Registry|**From**<br>0x283D382F|**Gas**<br>**Ether**<br>**USD**<br> 1459430<br>15.9_·_10_−_5<br>0.0723<br>~~TTT~~|**Gas**<br>**Ether**<br>**USD**<br> 1459430<br>15.9_·_10_−_5<br>0.0723<br>~~TTT~~|**Gas**<br>**Ether**<br>**USD**<br> 1459430<br>15.9_·_10_−_5<br>0.0723<br>~~TTT~~|**Gas**<br>**Ether**<br>**USD**<br> 1459430<br>15.9_·_10_−_5<br>0.0723<br>~~TTT~~|
|---|---|---|---|---|---|
|AddWorker|0x283D382F|452467||45.2_·_10_−_5|0.0692|
|AddWorker|0x283D382F|452545||45.2_·_10_−_5|0.0692|
|AddWorker|0x283D382F|452436||45.2_·_10_−_5|0.0692|
|AddWorker|0x283D382F|452545||45.2_·_10_−_5|0.0692|
|AddWorker|0x283D382F|452436||45.2_·_10_−_5|0.0692|
|ModelTransmission|0x5846F427|19374||19.3_·_10_−_5|0.1621|
|ModelTransmission|0x9dD8Fd06|243482||24.3_·_10_−_5|0.0902|
|ModelTransmission|0x98HF8F94|228779||22.3_·_10_−_5|0.1121|
|ModelTransmission|0x8H9FH780 253924|0x8H9FH780 253924||25.3_·_10_−_5|0.0951|
|ModelTransmission|0x0932FD99|263924||19.3_·_10_−_5|0.0571|
|ModelTraining<br>ModelTraining|0x5846F427 <br>0x9DD8Fd06 253924|223924<br>0x9DD8Fd06 253924||22.3_·_10_−_5<br>25.3_·_10_−_5|0.1021<br>0.0951|
|ModelTraining|0x98HF8F94|193924||19.3_·_10_−_5|0.0571|
|ModelTraining|0x8H9FH780 253924|0x8H9FH780 253924||25.3_·_10_−_5|0.0951|
|ModelTraining<br>ModelAggregation|0x0932FD99 <br>0x5846F427|253924<br> 324942||19.3_·_10_−_5<br>32.4_·_10_−_5|0.0571<br>0.0766|
|ModelAggregation<br>ModelAggregation|0x9dD8Fd06 <br>0x98HF8F94|283445<br> 214939||22.4_·_10_−_5<br>21.4_·_10_−_5|0.0408<br>0.0709|
|ModelAggregation|0x8H9FH780 253924|0x8H9FH780 253924||25.3_·_10_−_5|0.0951|
|ModelAggregation<br>Settlement<br>PayChannelExecute|0x0932FD99 <br>0x283D382F <br>0x283D382F|193924<br> 212559<br> 212538||19.3_·_10_−_5<br>21.3_·_10_−_5<br>21.2_·_10_−_5|0.0571<br>0.0712<br>0.0702|



* 1 Ether = 10[9] Gwei; 1 USD = 246,940.5627 Gwei 

## _5) Scenarios:_ 

- **(S1)** . In scenario 1, the compared algorithms have same distribution with same dataset size, i.e., each client dataset _Di, ∀i ∈N_ has the same amount of training image samples. 

- **(S2)** . In scenario 2, we introduce the case with same distribution but different dataset size. The training set is divided randomly into 5 parts with the same ratio of data size. 

- **(S3)** . In scenario 3, we use different distribution with same dataset size. Each client’s dataset _Di, ∀i ∈ {_ 1 _,_ 2 _,_ 3 _,_ 4 _,_ 5 _}_ has the same size, but the training images are not equally divided for each digit. 

- **(S4)** . In scenario 4, we consider the case having an added noise feature with same dataset. First, we split the training set in a similar manner as **(S1)** . Afterwards, we generate Gaussian noise for the dataset. This is done by adjusting the standard deviation of the normal distribution. 

## _B. Results_ 

_1) Smart Contract Execution Cost:_ In this part, the proofof-concept of proposed model trading platform is deployed in a private Ethereum Blockchain called _Ganache_[6] . In distributed application _Dapps_ , the smart contract plays as key role in controlling and autonomously executing pre-defined agreements between the participants. We implemented and tested smart contracts using Remix IDE[7] . In Ethereum network, there is a fee called _gas_ , needed to pay for any operation or transaction execution that changes the DLT states, which guarantees that smart contracts running in Ethereum Virtual Machine (EVM) [24] will be terminated eventually. In the 

**==> picture [124 x 19] intentionally omitted <==**

IEEE JOURNAL SUBMISSION 

11 

**==> picture [235 x 412] intentionally omitted <==**

**----- Start of picture text -----**<br>
30<br>25<br>20<br>15 20<br>Seller 1<br>10 15<br>2 3 4 Seller 2<br>Seller 3<br>5 Seller 4<br>Seller 5<br>0<br>1 2 3 4 5 6<br>Training Round<br>(a) Received incentive per client with the dataset distribution<br>2:2:2:2:2.<br>40<br>30<br>20<br>Seller 1<br>Seller 2<br>10<br>Seller 3<br>Seller 4<br>Seller 5<br>0<br>1 2 3 4 5 6<br>Training Round<br>Incentive (% of received tokens)<br>Incentive (% of received tokens)<br>**----- End of picture text -----**<br>


- (a) Received incentive per client with the dataset distribution ratio 2:2:2:2:2. 

(b) Received incentive per client with the dataset distribution ratio 3:3:2:1:1. 

Figure 7: Incentive of clients. 

scope of this research, we used Gwei[8] to evaluate the cost of different operations, for example, _AddWorker, ModelTransmisson, ModelTraning,_ or _Settlement_ in the model trading process. The result is demonstrated in Table. II. 

_2) Incentive per client:_ In Fig. 7, we show the comparison of received incentives by each training client based on their contribution to the global model training. The incentive is equivalent to tokens clients receive. As expected, in Fig. 7a, where the MNIST dataset is divided equally with a ratio of 2:2:2:2:2 for five involved clients, the amount of tokens they receive are almost similar as expected. In Fig. 7b, we show the comparison of the received percentage of tokens that clients can achieve with the dataset ratio of 3:3:2:1:1. We observe that client 1 and client 2 has the same amount of dataset, so they receive the same amount of tokens for their contribution, similar to the case for clients 4 and 5. 

**==> picture [247 x 335] intentionally omitted <==**

**----- Start of picture text -----**<br>
Scenario 1<br>0.25<br>Exact Single-Cal Multi-Cal AFS<br>0.2<br>0.15<br>1 2 3 4 5<br>Scenario 2<br>0.6<br>0.4<br>0.2<br>0<br>1 2 3 4 5<br>Scenario 3<br>0.4<br>0.2<br>0<br>1 2 3 4 5<br>Scenario 4<br>0.4<br>0.2<br>0<br>1 2 3 4 5<br>Seller ID<br>SV<br>SV<br>SV<br>SV<br>**----- End of picture text -----**<br>


Figure 8: Shapley Value of each seller in different methods 

Note that the sellers can train the models with poor quality, which, in fact, reduces the stability and performance of the global models. In this regard, there exist several mechanisms to handle such dishonest reporting of parameters in the FL setting, such as [13], [31], [32]. Similar to this, the DLT keeps track of the contribution of devices and the gradient information and the size of data samples to regularly infer (check) the relationship between the expected model quality, reported data samples, and the obtained SV as Fig. 8; hence, dealing untruthful reporting. However, the detailed study of this mechanism is out of scope for this work. In Fig. 8, the _AFS_ shows a better performance while other methods turns out quite random SVs, especially in scenario 2 and 4 where the size of dataset is random and noise added. 

_3) Execution time and maximum different comparison:_ In Fig. 9, we show the time performance of exact FL, Single-Cal, Multi-Cal, and AFS protocol. The Multi-Cal algorithm is more computational expensive than the Single-Cal algorithm. The standard exact method is the slowest one because the standard Shapley Value is naturally not compatible with the Federated Learning. In the scenario 1, each worker has same quality and quantity of dataset, so that we expect each worker has same contribution and receive equally the amount of incentive. The results show that the Single-Cal and AFS algorithm have higher efficiency in execution time. The exact method is around 5 times slower than the rest of methods because 

8https://www.cryps.info/ 

IEEE JOURNAL SUBMISSION 

12 

**==> picture [247 x 367] intentionally omitted <==**

**----- Start of picture text -----**<br>
Scenario 1 Scenario 1<br>15000 0.08<br>0.06<br>10000<br>0.04<br>5000<br>0.02<br>0 0<br>Scenario 2 Scenario 2<br>15000 0.08<br>0.06<br>10000<br>0.04<br>5000<br>0.02<br>0 0<br>Scenario 3 Scenario 3<br>15000 0.1<br>10000<br>0.05<br>5000<br>0 0<br>Scenario 4 Scenario 4<br>15000 0.08<br>0.06<br>10000<br>0.04<br>5000<br>0.02<br>0 0<br>Methods Methods<br>AFS AFS<br>AFS AFS<br>AFS AFS<br>AFS AFS<br>Single-CalMulti-Cal Exact Single-Cal Multi-Cal<br>Single-CalMulti-Cal Exact Single-Cal Multi-Cal<br>Single-CalMulti-Cal Exact Single-Cal Multi-Cal<br>Single-CalMulti-Cal Exact Single-Cal Multi-Cal<br>max<br>D<br>Run-time<br>max<br>D<br>Run-time<br>max<br>D<br>Run-time<br>max<br>D<br>Run-time<br>**----- End of picture text -----**<br>


Figure 9: Comparison of execution time and _D_ max between algorithms. 

of frequent model retrain process. Meanwhile, the _Dmax_ of methods are relatively low, around 0 _._ 05. Similar in scenario 2 with the same size of dataset and different distribution, AFS and Single-Cal have better performance in running time and the accuracy. However, in scenario 4 with more noisy data, the Multi-Cal shows better results, _≈_ 10% in run-time but and _≈_ 15% in terms of maximum different value. 

## V. RELATED WORKS 

In this section, we first present the current works on asset trading based on Blockchain and data valuation. 

**Blockchain-based asset trading** . With the spread of ubiquitous marketplaces, it became relevant to explore the application of IoT data trading in marketplace environments. For instance, the authors in [33] considered a dynamic decentralized marketplace and introduced the architecture for trading IoT data accordingly. The approach involves a 3- tier method is used: 1) data provider, 2) broker and 3) data consumer. The primary purpose of DLTs in their function is to manage the conditions of agreements between the parties involved. In addition, the design has a reputation system that penalizes members and lowers their rating. The authors in [34] invested the optimization problem of revenue maximization 

with envy-free guarantee. The authors studied two scenarios including unit demand consumers and single minded consumers, and showed the optimization problem is APX-hard for both scenarios, which can be efficiently addressed by a logarithmic approximation. The authors in [35] took into account the trading of IoT streaming data with the presented marketplace model, where fraudulent activity during data exchange is limited. To do so, the authors introduced periodic checkpoints during data trading. In [36], the authors proposed another marketplace which flows of IoT data are the main digital assets exchanged utilizing Oracles for the off-chain queries.The authors in [31] presented a trading mode based on smart contracts. In particular, the authors employ arbitration that handles disputes during the data trading, particularly, over the data availability, and incorporates AI/ML to ensure fairness during data exchange. 

**Data Valuation** . Evaluating the value of data has been received significant attention from both academia and industrial areas. Several works studied data valuation strategies and their applications. In this regard, the authors in [37] defined the data valuation in several categories, such as: (i) query-based pricing, where prices are attached to user-initiated queries [21], [38], [39], (ii) data attribute-based pricing, where the price model considers data attributes, such as the age of data and its credibility, using the mechanism of public price registries [40], and (iii) auction-based pricing, where the price is dynamically set following auction mechanisms [41], [42]. In [25],multiple approximation strategies for optimizing the computation complex of SV for training data are introduced. Besides, the authors proposed an soltiion to compute exact SC in specific scenario, e.g nearest neighbor classifiers. Besides, the SV also is applied in various AI/ML application, for example, to measure the importance of model features [43], [44]. In specific, the authors addressed the problem when the same data points get the same values, and relationship between data distributions and SV function. In addition, the authorsproposed an idea of distributional SV occurs resemblance to the Aumann-SV [45]. In practical manner, the authors in [46] proved that the performance of model training can be improved by removing the data with low SV value. In contrast, the performance will be decreased if we deleting the training data with high SV values. 

## VI. CONCLUSION 

In this paper, we proposed a DLT-based marketplace for trading ML models, which helps companies and organizations train their learning models in a scalable and efficient manner. An incentive mechanism exists to stimulate participants in joining and training the learning models on the marketplace, which pays participants based on their contributions to train the model. To that end, an extended Data Shapley Value (DSV) for the federated environment is proposed to measure each participant’s contribution in the model training process. Finally, with extensive experimental evaluations with Ethereum Blockchain to build a marketplace for model trading using smart contracts and IoT devices acting as participants, we demonstrated the design and performance of the proposed ecosystem. 

IEEE JOURNAL SUBMISSION 

13 

## VII. ACKNOWLEDGMENT 

This work has received funding from the European Union’s Horizon 2020 research and innovation programme under grant agreement No. 957218 (Project IntellIoT). 

## REFERENCES 

- [1] I. Report, “The growth in connected iot devices is expected to generate 79.4zb of data in 2025, according to a new idc forecast.” [Online]https: //www.idc.com, 2019. (Accessed on 12/04/2020). 

- [2] Z. Huang, X. Su, Y. Zhang, C. Shi, H. Zhang, and L. Xie, “A decentralized solution for iot data trusted exchange based-on blockchain,” in _2017 3rd IEEE International Conference on Computer and Communications (ICCC)_ , pp. 1180–1184, IEEE, 2017. 

- [3] C. Perera, “Sensing as a service (s2aas): Buying and selling iot data,” _arXiv preprint arXiv:1702.02380_ , 2017. 

- [4] W. Mao, Z. Zheng, and F. Wu, “Pricing for revenue maximization in iot data markets: An information design perspective,” in _IEEE INFOCOM 2019-IEEE Conference on Computer Communications_ , pp. 1837–1845, IEEE, 2019. 

- [5] B. Bishoi, A. Prakash, V. Jain, _et al._ , “A comparative study of air quality index based on factor analysis and us-epa methods for an urban environment,” _Aerosol and Air Quality Research_ , vol. 9, no. 1, pp. 1–17, 2009. 

- [6] J. Jo, B. Jo, J. Kim, S. Kim, and W. Han, “Development of an iot-based indoor air quality monitoring platform,” _Journal of Sensors_ , vol. 2020, 2020. 

- [7] Y. Liu, J. Nie, X. Li, S. H. Ahmed, W. Y. B. Lim, and C. Miao, “Federated learning in the sky: Aerial-ground air quality sensing framework with uav swarms,” _IEEE Internet of Things Journal_ , vol. 8, no. 12, pp. 9827–9837, 2021. 

- [8] S. Moltchanov, I. Levy, Y. Etzion, U. Lerner, D. M. Broday, and B. Fishbain, “On the feasibility of measuring urban air pollution by wireless distributed sensor networks,” _Science of The Total Environment_ , vol. 502, pp. 537–547, 2015. 

- [9] N. Gruschka, V. Mavroeidis, K. Vishi, and M. Jensen, “Privacy issues and data protection in big data: a case study analysis under gdpr,” in _2018 IEEE International Conference on Big Data (Big Data)_ , pp. 5027– 5033, IEEE, 2018. 

- [10] B. McMahan, E. Moore, D. Ramage, S. Hampson, and B. A. y Arcas, “Communication-efficient learning of deep networks from decentralized data,” in _Artificial intelligence and statistics_ , pp. 1273–1282, PMLR, 2017. 

- [11] B. Xu, L. Da Xu, H. Cai, C. Xie, J. Hu, and F. Bu, “Ubiquitous data accessing method in iot-based information system for emergency medical services,” _IEEE Transactions on Industrial informatics_ , vol. 10, no. 2, pp. 1578–1586, 2014. 

- [12] R. Radhakrishnan and B. Krishnamachari, “Streaming data payment protocol (sdpp) for the internet of things,” in _2018 IEEE International Conference on Internet of Things (iThings) and IEEE Green Computing and Communications (GreenCom) and IEEE Cyber, Physical and Social Computing (CPSCom) and IEEE Smart Data (SmartData)_ , pp. 1679– 1684, IEEE, 2018. 

- [13] C. Niu, Z. Zheng, F. Wu, X. Gao, and G. Chen, “Achieving data truthfulness and privacy preservation in data markets,” _IEEE Transactions on Knowledge and Data Engineering_ , vol. 31, no. 1, pp. 105–119, 2018. 

- [14] D. C. Langevoort, “Fraud and insider trading in american securities regulation: Its scope and philosophy in a global marketplace,” _Hastings Int’l & Comp. L. Rev._ , vol. 16, p. 175, 1992. 

- [15] S. R. Pandey, N. H. Tran, M. Bennis, Y. K. Tun, A. Manzoor, and C. S. Hong, “A crowdsourcing framework for on-device federated learning,” _IEEE Transactions on Wireless Communications_ , vol. 19, no. 5, pp. 3241–3256, 2020. 

- [16] S. Nakamoto, “Bitcoin: A peer-to-peer electronic cash system,” tech. rep., Manubot, 2008. 

- [17] L. D. Nguyen, I. Leyva-Mayorga, A. N. Lewis, and P. Popovski, “Modeling and analysis of data trading on blockchain-based market in iot networks,” _IEEE Internet of Things Journal_ , vol. 8, no. 8, pp. 6487– 6497, 2021. 

- [18] B. Chen, D. He, N. Kumar, H. Wang, and K.-K. R. Choo, “A blockchainbased proxy re-encryption with equality test for vehicular communication systems,” _IEEE Transactions on Network Science and Engineering_ , vol. 8, no. 3, pp. 2048–2059, 2021. 

- [19] L. D. Nguyen, A. E. Kalor, I. Leyva-Mayorga, and P. Popovski, “Trusted wireless monitoring based on distributed ledgers over nb-iot connectivity,” _IEEE Communications Magazine_ , vol. 58, no. 6, pp. 77– 83, 2020. 

- [20] T. Wang, C. Zhao, Q. Yang, S. Zhang, and S. C. Liew, “Ethna: Analyzing the underlying peer-to-peer network of ethereum blockchain,” _IEEE Transactions on Network Science and Engineering_ , vol. 8, no. 3, pp. 2131–2146, 2021. 

- [21] P. Kairouz, H. B. McMahan, B. Avent, A. Bellet, M. Bennis, A. N. Bhagoji, K. Bonawitz, Z. Charles, G. Cormode, R. Cummings, _et al._ , “Advances and open problems in federated learning,” _arXiv preprint arXiv:1912.04977_ , 2019. 

- [22] H. Kim, J. Park, M. Bennis, and S.-L. Kim, “Blockchained on-device federated learning,” _IEEE Communications Letters_ , vol. 24, no. 6, pp. 1279–1283, 2019. 

- [23] A. E. Roth, _The Shapley value: essays in honor of Lloyd S. Shapley_ . Cambridge University Press, 1988. 

- [24] G. Wood _et al._ , “Ethereum: A secure decentralised generalised transaction ledger,” _Ethereum project yellow paper_ , vol. 151, no. 2014, pp. 1– 32, 2014. 

- [25] R. Jia, D. Dao, B. Wang, F. A. Hubis, N. Hynes, N. M. G¨urel, B. Li, C. Zhang, D. Song, and C. J. Spanos, “Towards efficient data valuation based on the shapley value,” in _The 22nd International Conference on Artificial Intelligence and Statistics_ , pp. 1167–1176, PMLR, 2019. 

- [26] X. Ding, J. Guo, D. Li, and W. Wu, “An incentive mechanism for building a secure blockchain-based internet of things,” _IEEE Transactions on Network Science and Engineering_ , vol. 8, no. 1, pp. 477–487, 2021. 

- [27] Y. Qu, L. Gao, T. H. Luan, Y. Xiang, S. Yu, B. Li, and G. Zheng, “Decentralized privacy using blockchain-enabled federated learning in fog computing,” _IEEE Internet of Things Journal_ , vol. 7, no. 6, pp. 5171– 5183, 2020. 

- [28] A. Ghorbani and J. Zou, “Data shapley: Equitable valuation of data for machine learning,” in _International Conference on Machine Learning_ , pp. 2242–2251, PMLR, 2019. 

- [29] Z. Charles and J. Koneˇcn`y, “On the outsized importance of learning rates in local update methods,” _arXiv preprint arXiv:2007.00878_ , 2020. 

- [30] W. Hoeffding, “A combinatorial central limit theorem,” _The Annals of Mathematical Statistics_ , pp. 558–566, 1951. 

- [31] W. Xiong and L. Xiong, “Smart contract based data trading mode using blockchain and machine learning,” _IEEE Access_ , vol. 7, pp. 102331– 102344, 2019. 

- [32] A. Agarwal, M. Dahleh, and T. Sarkar, “A marketplace for data: An algorithmic solution,” in _Proceedings of the 2019 ACM Conference on Economics and Computation_ , pp. 701–726, 2019. 

- [33] P. Gupta, S. Kanhere, and R. Jurdak, “A decentralized iot data marketplace,” _arXiv preprint arXiv:1906.01799_ , 2019. 

- [34] R. Iyengar, J. P. Near, D. Song, O. Thakkar, A. Thakurta, and L. Wang, “Towards practical differentially private convex optimization,” in _2019 IEEE Symposium on Security and Privacy (SP)_ , pp. 299–316, IEEE, 2019. 

- [35] S. Bajoudah, C. Dong, and P. Missier, “Toward a decentralized, trustless marketplace for brokered iot data trading using blockchain,” in _2019 IEEE International Conference on Blockchain (Blockchain)_ , pp. 339– 346, IEEE, 2019. 

- [36] P. Missier, S. Bajoudah, A. Capossele, A. Gaglione, and M. Nati, “Mind my value: a decentralized infrastructure for fair and trusted iot data trading,” in _Proceedings of the Seventh International Conference on the Internet of Things_ , pp. 1–8, 2017. 

- [37] T. Wang, J. Rausch, C. Zhang, R. Jia, and D. Song, “A principled approach to data valuation for federated learning,” in _Federated Learning_ , pp. 153–167, Springer, 2020. 

- [38] D. Leroy, A. Coucke, T. Lavril, T. Gisselbrecht, and J. Dureau, “Federated learning for keyword spotting,” in _ICASSP 2019-2019 IEEE International Conference on Acoustics, Speech and Signal Processing (ICASSP)_ , pp. 6341–6345, IEEE, 2019. 

- [39] P. Upadhyaya, M. Balazinska, and D. Suciu, “Price-optimal querying with data apis,” _Proceedings of the VLDB Endowment_ , vol. 9, no. 14, pp. 1695–1706, 2016. 

- [40] J. R. Heckman, E. L. Boehmer, E. H. Peters, M. Davaloo, and N. G. Kurup, “A pricing model for data markets,” _iConference 2015 Proceedings_ , 2015. 

- [41] M. Mihailescu and Y. M. Teo, “Dynamic resource pricing on federated clouds,” in _2010 10th IEEE/ACM International Conference on Cluster, Cloud and Grid Computing_ , pp. 513–517, IEEE, 2010. 

- [42] J.-S. Lee and B. Hoh, “Sell your experiences: a market mechanism based incentive for participatory sensing,” in _2010 IEEE International_ 

IEEE JOURNAL SUBMISSION 

14 

_Conference on Pervasive Computing and Communications (PerCom)_ , pp. 60–68, IEEE, 2010. 

- [43] S. Cohen, G. Dror, and E. Ruppin, “Feature selection via coalitional game theory,” _Neural Computation_ , vol. 19, no. 7, pp. 1939–1961, 2007. 

- [44] E. Strumbelj and I. Kononenko, “An efficient explanation of individual classifications using game theory,” _The Journal of Machine Learning Research_ , vol. 11, pp. 1–18, 2010. 

- [45] R. J. Aumann and L. S. Shapley, _Values of non-atomic games_ . Princeton University Press, 2015. 

- [46] S. Tang, A. Ghorbani, R. Yamashita, S. Rehman, J. A. Dunnmon, J. Zou, and D. L. Rubin, “Data valuation for medical imaging using shapley value and application to a large-scale chest x-ray dataset,” _Scientific reports_ , vol. 11, no. 1, pp. 1–9, 2021. 

**Lam Duc Nguyen** (S’20) is a Ph.D. Fellow at Aalborg University. He obtained Master Degree in Computer Science at Seoul National University, and a Bachelor in Telecommunication at Hanoi University of Science and Technologies in 2019 and 2015, respectively. His research includes Distributed Systems, Blockchain, Smart Contracts, the Internet of Things, and applying Blockchain and Federated Learning to enhance the efficiency of Blockchain| th based IoT monitoring Networks. He receives Outstanding Paper Award for the research about scaling Blockchain in Massive IoT at the IEEE World Forum Internet of Things 2020, travel grant from Linux Foundation 2020, Best Research Award for a solution of Blockchain-based CO2 Emission Trading from VEHITS 2021. He is Hyperledger Member, IEEE Student Member, and IEEE ComSoc Student Member. 

**Shashi Raj Pandey** (M’21) is currently working as a Postdoctoral Researcher at the Connectivity Section, Aalborg University. He received the B.E. degree in = ElectricalCommunicationand Electronicsfrom Kathmanduwith a University,specializationNepalin in 2013, and the Ph.D. degree in Computer Science and Engineering from Kyung Hee University, Seoul, South Korea, in August, 2021. He served as a Network Engineer at Huawei Technologies Nepal Co. Pvt. Ltd, Nepal from 2013 to 2016. His research interests include network economics, game theory, wireless communications, data markets and distributed machine learning. 

**Arne Br¨oring** is a Senior Key Expert Research Scientist at Siemens Technology in Munich. He received his PhD in 2012 from the University of Twente (Netherlands). Dr. Br¨oring has contributed to over 90 publications in the field of distributed systems and has served on various program committees and editorial boards. His research interests range from distributed system designs, over sensor networks, and Semantic Web, to the Internet of Things. @ At Siemens, he has been in charge of the technical & scientific coordination of large EU research projects (BIG IoT and IntellIoT). Before joining Siemens, Dr. Br¨oring worked for the Environmental Systems Research Institute in Zurich, the 52°North Open Source Initiative, and led the Sensor Web and Simulation Lab at the University of M¨unster. 

**Petar Popovski** (Fellow, IEEE) is a Professor at Aalborg University, where he heads the section on Connectivity and a Visiting Excellence Chair at the University of Bremen. He received his Dipl.-Ing and M. Sc. degrees in communication engineering from the University of Sts. Cyril and Methodius in Skopje and the Ph.D. degree from Aalborg University in 2005. He is a Fellow of the IEEE. He received an ERC Consolidator Grant (2015), the Danish Elite Researcher award (2016), IEEE Fred W. Ellersick prize (2016), IEEE Stephen O. Rice prize (2018), Technical Achievement Award from the IEEE Technical Committee on Smart Grid Communications (2019), the Danish Telecommunication Prize (2020) and Villum Investigator Grant (2021). He is a Member at Large at the Board of Governors in IEEE Communication Society, Vice-Chair of the IEEE Communication Theory Technical Committee and IEEE TRANSACTIONS ON GREEN COMMUNICATIONS AND NETWORKING. He is currently an Area Editor of the IEEE TRANSACTIONS ON WIRELESS COMMUNICATIONS and, from 2022, an Editor-in-Chief of IEEEE JOURNAL ON SELECTED AREAS IN COMMUNICATIONS. Prof. Popovski was the General Chair for IEEE SmartGridComm 2018 and IEEE Communication Theory Workshop 2019. His research interests are in the area of wireless communication and communication theory. He authored the book “Wireless Connectivity: An Intuitive and Fundamental Guide”, published by Wiley in 2020. 

**Beatriz Soret** [M’11] received her M.Sc. and Ph.D. degrees in Telecommunications from the Universidad de Malaga, Spain, in 2002 and 2010, respectively. She is currently an associate professor at the Department of Electronic Systems, Aalborg University, and a Senior Research Fellow at the Communications Engineering Department, University of Malaga. Her research interests include LEO satellite communications, distributed and intelligent IoT, timing in communications, and 5G and post-5G systems. 

