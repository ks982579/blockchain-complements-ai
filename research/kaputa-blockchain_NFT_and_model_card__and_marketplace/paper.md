MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

# — Kaputa Blockchain, Non-Fungible Token and Model Card Integrated 5G/6G Network Slice Broker and Marketplace 

Eranga Bandara _[∗]_ , Sachin Shetty _[∗]_ , Ravi Mukkamala _[∗]_ , Abdul Rahman _[†]_ , Peter Foytik _[∗]_ , Xueping Liang _[‡]_ , Ng Wee Keong _[§]_ , 

> _∗ {_ cmedawer, sshetty, mukka, pfoytik _}_ @odu.edu Old Dominion University, Norfolk, VA, USA 

> _†_ abdulrahman@deloitte.com Deloitte & Touche LLP 

> _‡_ xuliang@fiu.edu Florida International University, USA 

> _§_ AWKNG@ntu.edu.sg Nanyang Technological University, Singapore 

_**Abstract**_ **—5G network slicing enables IoT networks to connect billions of heterogeneous objects providing high quality of service, high network capacity, and enhanced system throughput. It opens a new marketplace opportunity for cloud providers and network operators to sell portions of their networks to address specific customer needs in 5G applications. However, there are numerous open challenges to providing end-to-end slices due to complex business and engineering requirements from service and resource providers. To address these challenges, in this paper, we propose “Kaputa”, a blockchain-enabled network slice broker and NFTenabled network slice marketplace. Here, different stakeholders such as cloud providers, network operators, RAN providers, and transport network providers can collaborate to rent their resources. Kaputa orchestrates the network slices with the help of blockchain smart contracts. The orchestrated network slices will be encoded as NFT tokens and published in the Kaputa NFT marketplace. Customers can purchase the network slices from the marketplace based on their 5G application requirements via paying crypto/fiat currency. The revenue will be distributed among different providers. The data provenance information of the network slices is encoded into Model Cards and stored in the blockchain ledger. A prototype of Kaputa has been implemented with FreedomFi and OpenAirInterface 5G core. To the best of our knowledge, this is the very first research that tries to represent 5G/6G network slices as NFTs. This design methodology provides enhanced transparency and auditability to the network slice orchestration process while providing an open platform to share and trade network slices in the marketplace.** 

_**Index Terms**_ **—5G, 6G, Network Slicing, NFT, Blockchain; Model Card** 

## I. INTRODUCTION 

The concept of network slicing helps 5G to support different types of application services, which otherwise operate on a dedicated network. Network slicing enables sharing of the same physical infrastructure by dividing it into several virtual networks based on a customer or provider’s needs. Some of the core benefits involve improving overall network resource 

utilization, reducing network costs, increasing revenues, and enhancing network users’ experience [1]. Network slicing is facilitated through network “softwarization”, which involves running network functions within virtual machines or containers that are typically hosted in the cloud. Network Slices may have a constrained lifespan based on a customer’s need and/or service requirements which drives the need to flexibly provision and de-provision resources. This requires both life cycle management LCM) [2] and resource orchestration [3]. 

Before 5G, an operator is only able to offer services to the customer that the network allows. In many cases, especially for enterprise connectivity solutions like VPNs and Ethernet Leased Lines, the network requires several weeks to be provisioned after an order is placed. This not only resulted in revenue loss due to delays in connection setup but also poor customer satisfaction [4]. This inflexibility required _a priori_ preparation configuration to handle and support a large number of users and media coverage requirements. With the advent of 5G network slicing, this issue is mitigated by transforming into true digital (service) capability providers. For example, the service provider will have the ability to provide a portal or mobile app from which new-age businesses (i.e., sports leagues or an AR/VR gaming startup) to order requested bandwidth with quality of service (QoS) at a desired location for the desired duration to host an event. Furthermore, With automation in place, they will be able to get an instantaneous confirmation and make an online payment. 5G through network slicing enables automatic provisioning to support these requirements. In the backend, an end-to-end (E2E) orchestrator will ensure that the required operator network functions are instantiated and scaled out in the locations as desired by the E2E network slice, for the duration of the event with the desired QoS. At the end of the event, the resources will be released [5] for rapid (re)provisioning to support another customer requirement. 

559 

978-1-6654-8534-0/22/$31.00 ©2022 IEEE 

MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

Clearly, network slicing combined with cloud computing and storage has the potential to serve the needs of a growing community of vertical service providers (VSPs) by combining resources from different domains and offering them in an integrated session. A challenging problem stems from determining and tracking the resource requirements from the VSP’s customer requirements in addition to procuring those resources throughout the lifetime of a network slice [6]. One of the key objectives of mapping the user requirements to resource allocation is to balance the minimization of the overall cost with maximal QoS guarantees [3]. Traditional research and practice primarily focus on static deployments of network slices with delays influenced by the way the virtual network functions are deployed. This impacts the QoS offered and cost due to resource management and slicing may affect the operators’ profitability [7]. 

To address these challenges, this paper proposes a blockchain-enabled dynamic network slice broker and nonfungible token (NFT) [8] enabled network slice marketplace “Kaputa.” Different resource providers are involved in network slicing, mainly cloud providers, network / Radio Network Access (RAN) operators, 5G core service providers and transport network providers. The Kaputa network slice broker leases the resources from these actors and orchestrates the network slices by identifying optimal resource usage. The dynamic slice orchestration is automated with blockchain smart contracts. The orchestrated network slices will be encoded as NFT tokens and published in the Kaputa NFT marketplace [9]. Then, customers can purchase the network slices from this marketplace based on their 5G application requirements via paying crypto/fiat currency. The revenue will be shared among the cloud service providers and network providers. In related work, Moose which is an optimized blockchain for 5G network sliced environments [7] has been used as the decentralized ledger to support Kaputa. An extensible Non-Fungible Token Model has been proposed using novel token schema k528 to represent network slices as NFT tokens. The proposed k528 token schema is an extension of the ERC721 [10] schema which is implemented in the Moose blockchain. This design methodology provides enhanced transparency and auditability to the network slice orchestration process while providing an open platform to share and trade network slices in the marketplace. We believe that our effort first of its kind to represent, package, and manage 5G/6G network slices as NFTs. A prototype of the system has been implemented with FreedomFi [11] and OpenAirInterface 5G core [12]. This paper has the following contributions: 

- 1) Blockchain-enabled dynamic network slice broker has been developed. It enables network operators, 5G service providers, cloud providers, and transport network providers to rent their excess resources to create network slicing. 

- 2) Proposed a model to represent network slices as NFT tokens. Designed an extensible NFT schema k528 to represent the network slices as NFT tokens. 

Fig. 1: Kaputa platform layered architecture. 

- 3) Proposed a NFT token-based network slice marketplace to trade network slices. 

- 4) Data provenance information of the network slices (which represents NFT) are encoded into Model Cards and saved in the blockchain ledger. 

The rest of the paper is organized as follows. The background information is discussed in Section 2. The design of the Kaputa platform is discussed in Section 3. The Kaputa platform’s capabilities are discussed in Section 4. Section 5 consists of performance evaluation. Section 6 describes related work. Section 7 summarizes the proposed Kaputa platform with recommendations for future work. 

## II. KAPUTA ARCHITECTURE 

Figure 1 describes the architecture of the Kaputa platform. The proposed Kaputa Architecture platform is composed of four layers: 1) resource provider layer, 2) blockchain and smart contract layer, 3) NFT marketplace layer, and 4) Customer layer. Below is a brief description of each layer. 

## _A. Resource Providers_ 

Four different types of resource providers are involved in the Kaputa platform, 1) cloud providers, 2) network/RAN operators, 3) 5G core service providers, and 4) transport network providers. These providers can rent their excess resources to create network slicing. Cloud providers rent the cloud service infrastructures, and Network/RAN operators can rent 5G hardware infrastructures. 5G core service providers offer the 5G core services. Transport network providers offer network transport facilities. The Kaputa network slice broker leases the resources from these providers and orchestrates the network slices by identifying optimal resource usage. 

## _B. Blockchain and Smart Contract Layer_ 

The blockchain network will be deployed between different resource providers. A separate blockchain node can be deployed in every resource provider (i.e., cloud providers, RAN providers, 5G core providers and transport network 

560 

MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

Fig. 2: Model Card for 5G network slice data provenance. 

Fig. 3: k528 NFT token schema to represent the network slice. 

providers). The Blockchain ledger stores the digital identities of the entities, network slice details, network slice provenance information and NFT token information. The network slice templates are stored in the off-chain storage of the blockchain. Blockchain smart contracts are used to implement Kaputa’s features. 1) Identity contract, 2) Slice contract, 3) Model Card contract and 4) NFT contract are the four key smart contracts in the system. The Identity contract is in-charge of the peers’ identity management (e.g., identity registration, permissions handling etc.). The slice orchestration functions are handled by the Slice contract (e.g., slice creation, configuration, deployment etc.). Each network slice contains a separate 5G Core network and facilitates various Network Functions (NF) such as session management, authentication, policy control, and data storage. 5G Core network architecture is designed with Network Function Virtualization (NFV) and SoftwareDefined Networking (SDN) using microservices-based design pattern [13]. The different data services and requirements of the 5G Core are implemented as microservices. These microservices will be deployed as cloud-native applications. The Model Card contract handles the data provenance function [14]. The data provenance of the network slices is encoded into Model cards and stored in the blockchain ledger. Figure 2 represents the model card generated in the Kaputa platform with 5G slice data provenance information. The NFT contract is in charge of NFT token creation and trading. 

## _C. NFT Marketplace_ 

We have created NFT-based network slice marketplace in Kaputa. The novel NFT token schema k528 is introduced to represent network slices as NFT tokens. Figure 3 describes the k528 schema of the NFT token of the Kaputa platform. It contains metadata such as Name, Description, Slice template URI, Creator, and link to the model card objects with the data provenance information of the slice. The NFT service handles the NFT token generation functions with these schemas. It 

generates k528 tokens as JSON format objects. This service interacts with smart contracts on the blockchain to generate NFT tokens that are linked to network slices. For example, when slice smart contract defines network slices based on the available resources (of cloud and network providers) and network slice templates via interacting with this service. The generated NFT will be stored in the blockchain ledger and distributed to other peers in the network. 

## _D. Customers_ 

Customers are the end-users of the Kaputa platform. They will interact with the system with web/mobile-based applications. Customers can purchase the network slices from the NFT marketplace based on their 5G application requirements (e.g., QoS) via paying crypto/fiat currency. When purchasing the network slice NFTs, customers can verify the audit and provenance information of the network slices via the model card objects published in the ledger. Once the slice is purchased and configured, customers can onboard their required 5G application into the network slice. 

## III. KAPUTA FUNCTIONALITY 

There are five main functionalities of the Kaputa platform: 1) Identity registration, 2) Network Slice Orchestration, 3) NFT token issuing, 4) NFT token trading and 5) Network Slice Deployment. This section goes into the specifics of these functions. 

## _A. Identity Registration_ 

As mentioned, there are four different types of actors/resource providers in the system 1) cloud providers, 2) network/RAN operators, 3) 5G core service providers, and 4) transport network providers. The blockchain network will be formed among these actors in Kaputa. The platform must 

561 

MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

initially be onboarded with the peer actors. When onboarding, their identities will be verified and saved in the blockchain storage according to the self-sovereign identity philosophy. Once these peers have been onboarded, separate blockchain nodes will be deployed for each of them. All of the peers’ identifying information is stored in the blockchain ledger. The blockchain’s identity smart contract implements this identity onboarding function [15]. 

## _B. Network Slice Orchestration_ 

According to different quality of service (QoS) requirements, services are categorized as massive machine-type communications (mMTC), enhanced mobile broadband (eMBB), and ultra-reliable low latency(URLLC). 5G network fulfills these service requirements through network slicing. Cloud service providers and network service providers rent their resources to create network slices. Based on the resource demands, Kaputa network slice brokers (which are implemented with smart contracts) can create different types of network slices. The number of slices and their QoS requirements are decided based on analyzing the previous network slice usages in the system (e.g., recommendation system). The network slice configurations handle via network slice templates where the temples configured the QoS level, slice parameters etc. There are different pre-defined network slice templates for different QoS levels in the Kaputa. These templates are saved in the off-chain storage. Slice brokers dynamically define network slices based on the available resources (e.g., resources of cloud providers and network providers) and network slice templates. The blockchain’s slice smart contract implements this network slice orchestration function. 

## _C. NFT Token Issuing_ 

The defined network slices are represented as NFT tokens in the Kaputa. We have designed an extensible Non-Fungible Token Model using novel token schema k528 to represent network slices as NFT tokens. The proposed k528 token schema is an extension of the ERC721 [10] schema which is implemented in the Moose blockchain. Once network slices are defined their information is encoded into JSON objects with the k528 schema and saved in the blockchain ledger. The provenance information of the network slices is encoded into Model cards and stored in the ledger along with the NFT tokens. The trading of the network slices handles with these NFT tokens. The structure of the NFT token is represented in Figure 3. 

## _D. Network Slice Trading_ 

We have created NFT based network slice marketplace in Kaputa. Customers can purchase the network slices from this marketplace based on their 5G application requirements(e.g., QoS) via paying crypto/fiat currency. The money will be distributed to hardware providers and cloud providers based on their contributions. The marketplace trading functions are implemented with smart contracts. When purchasing, customers can verify the audit and provenance information of 

Fig. 4: 5G testbed architecture with FreedomFi 

the network slices via the model card objects published in the ledger. This supports a fully transparent and auditable trading system in Kaputa. This design methodology provides enhanced transparency and auditability to the network slice orchestration process while providing an open platform to share and trade network slices in the marketplace. For trading in the marketplace, we are providing web/mobile applications for clients. The NFT smart contract implemented the NFT token trading function. 

## _E. Network Slice Creation and Deployment_ 

There is a lifetime for a network slice which is decided based on the customer’s 5G application requirements. The price of the network slice NFT token will be decided based on the slice configuration/resources and the lifetime. Once the lifetime is decided, that data is also encoded in the NFT token. When a customer purchases the network slice as NFT tokens, the ownership of the slice goes to the customer. The network slice broker coordinates the resources provided by different resource providers (e.g., cloud, network providers, transport network providers, RAN/NR (5G New Radio) providers) and deploys the network slice based on the configurations defined in the NFT token. The configuration is included in the NFT tokens as a network slice template. Once the slice is deployed, customers can deploy their required 5G application on the network slice. When the lifetime of the slice expires, the slice will be automatically shut down and the resources of the slice are released to the resource pool. Finally, the related NFT token will be burned out. 

## IV. KAPUTA IMPLEMENTATION AND EVALUATION 

The platform is developed with microservices architecture to support high scalability and high transaction load. The Moose blockchain (which is an optimized blockchain for 5G network sliced environments) has been adopted as the blockchain ledger [7]. Aplos smart contract platform [16]– [18] provides a customized smart contract interface. Each Moose blockchain node contains three main services, 1) Slicing service, 2) Storage service and 3) NFT service. The TensorFlow Model Card Toolkit [19] is used to implement 

562 

MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

Fig. 5: Block creation time and # of transactions. 

Fig. 6: Average block creation time and # of transactions. 

the Model Card functions. The JSON encoded k528 token structure is used to construct the NFT service. Etcd-based distributed key-value pair storage is the configuration store of the platform. Apache Kafka distributed publisher-subscriber system facilitates the peer-to-peer communications and backpressure handling [20], [21] functions of the Kaputa. As shown in Figure 4, we have used FreedomFi 5G gateway and Indoor Radio Cell in the 5G testbed [11]. To get network slicing functions, we have deployed OpenAirInterface 5G core [12] on the AWS cloud environment. The evaluation of the Kaputa blockchain performance for a varying number of blockchain peers (1 to 20 peers) is summarized below. 

**Block Generation Time and Block Size** : The average time to generate a block is known as block generation time. This is a critical component because it determines the system’s effectiveness. First, information on block creation time is compared to the number of transactions in the block. It takes 8.5 seconds to construct a block with 10k transactions, as demonstrated in Figure 5. The average block generation time with different transaction sets in a block is discussed in Figure 6 (i.e., different block sizes). The findings were obtained for blocks with transaction sets of 2000, 6000, and 10000. The experiment was repeated 100 times and the average values were plotted. 

**Block Generation Time and the Number of Peers** : The block generation time was measured in different cases where the number of blockchain peers in the cluster (up to 7) was changed. When adding new peers to the network, the block creation time is shown in Figure 7. Figure 8 depicts the average block production time when the network has a variable number of blockchain peers. We repeated each experiment 100 times 

Fig. 7: Block creation time and # of nodes in the cluster 

Fig. 8: Average block creation time and # of nodes in the cluster. 

with different peer groups for this evaluation and plotted the average values. 

## V. RELATED WORK 

Various researchers tried to facilitate end-to-end network slice orchestration, some of them use a blockchain-based decentralized approach and others propose service-based centralized architecture. The key elements and architecture of these research initiatives are outlined in this section. Table I summarizes the contrast between these projects and Kaputa. 

**Blockchain-Slice-Broker** [1] and **Network-SliceLeasing** [22] propose blockchain-enabled network slice brokers. **Slice-Market** [23] and **DBNS** [25] proposes blockchain-enabled network slice brokers and distributed slice marketplaces. **Network-Slice-Scaling** [3] and **Slice-as-aService** [4] propose methodologies for network slice scaling and life cycle management. **NECOS** [6] and **Slice-Store** [24] propose network slice as a service solution. 

## VI. CONCLUSIONS AND FUTURE WORK 

In this paper, we have presented Kaputa, a blockchain and NFT integrated 5G network slice broker and slice marketplace. The proposed system overcomes numerous open challenges to providing end-to-end slices due to complex business and engineering requirements from service and resource providers. Different actors (e.g., cloud providers, and network operators) can collaborate on the network and sell their resources. The Kaputa network slice broker leases these resources and orchestrates the network slices by identifying optimal resource usage. The slice orchestration is automated with blockchain smart contracts. The orchestrated network slices will be encoded as 

563 

MILCOM 2022 Track 3 - Cyber Security and Trusted Computing 

TABLE I: Network slice platform comparison 

|Platform|Centralized/<br>Distributed|Blockchain<br>Support|Running<br>Blockchain|Slice<br>Marketplace|Slice<br>Leasing|NFT<br>Support|Trading<br>Support|
|---|---|---|---|---|---|---|---|
|Kaputa|Distributed|✓|Moose|✓|✓|✓|✓|
|Blockchain-Slice-Broker [1]|Distributed|✓|N/A|✗|✓|✗|✗|
|Network-Slice-Leasing [22]|Distributed|✓|Hyperledger Fabric|✗|✓|✗|✗|
|Slice-Market [23]|Distributed|✓|Hyperledger Fabric|✓|✓|✗|✗|
|DBNS|Distributed|✓|N/A|✓|✗|✗|✗|
|Network-Slice-Scaling [3]|Centralized|✗|N/A|N/A|N/A|✗|✗|
|Slice-as-a-Service [4]|Centralized|✗|N/A|✗|N/A|✗|✗|
|NECOS [6]|Centralized|✗|N/A|✓|N/A|✗|✓|
|Slice-Store [24]|Centralized|✗|N/A|✓|✓|✗|✓|



NFT tokens and published in the Kaputa NFT marketplace. Then customers can purchase the network slices from this marketplace based on their 5G application requirements via paying crypto/fiat currency. The revenue will be distributed to hardware providers and cloud providers based on their contributions. This design methodology provides enhanced transparency and auditability to the network slice orchestration process while providing an open platform to share and trade network slices in the marketplace. 

**==> picture [99 x 8] intentionally omitted <==**

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] B. Nour, A. Ksentini, N. Herbaut, P. A. Frangoudis, and H. Moungla, “A blockchain-based network slice broker for 5g services,” _IEEE Networking Letters_ , vol. 1, no. 3, pp. 99–102, 2019. 

- [2] W. Lee, T. Na, and J. Kim, “How to create a network slice?-a 5g core network perspective,” in _2019 21st International Conference on Advanced Communication Technology (ICACT)_ . IEEE, 2019, pp. 616– 619. 

- [3] J. Zhou, W. Zhao, and S. Chen, “Dynamic network slice scaling assisted by prediction in 5g network,” _IEEE Access_ , vol. 8, pp. 133 700–133 712, 2020. 

- [4] K. Abbas, T. A. Khan, M. Afaq, and W.-C. Song, “Network slice lifecycle management for 5g mobile networks: An intent-based networking approach,” _IEEE Access_ , vol. 9, pp. 80 128–80 146, 2021. 

- [5] N. Nikaein, E. Schiller, R. Favraud, K. Katsalis, D. Stavropoulos, I. Alyafawi, Z. Zhao, T. Braun, and T. Korakis, “Network store: Exploring slicing in future 5g networks,” in _Proceedings of the 10th International Workshop on Mobility in the Evolving Internet Architecture_ , 2015, pp. 8–13. 

- [6] S. Clayman, A. Neto, F. Verdi, S. Correa, S. Sampaio, I. Sakelariou, L. Mamatas, R. Pasquini, K. Cardoso, F. Tusa _et al._ , “The necos approach to end-to-end cloud-network slicing as a service,” _IEEE Communications Magazine_ , vol. 59, no. 3, pp. 91–97, 2021. 

- [7] E. Bandara, S. Shetty, A. Rahman, R. Mukkamala, and X. Liang, “Moose: A scalable blockchain architecture for 5g enabled iot with sharding and network slicing,” in _2022 IEEE Wireless Communications and Networking Conference (WCNC)_ . IEEE, 2022, pp. 1194–1199. 

- [8] Q. Wang, R. Li, Q. Wang, and S. Chen, “Non-fungible token (nft): Overview, evaluation, opportunities and challenges,” _arXiv preprint arXiv:2105.07447_ , 2021. 

- [9] R. F. Ciriello, “Tokenized index funds: A blockchain-based concept and a multidisciplinary research framework,” _International Journal of Information Management_ , vol. 61, p. 102400, 2021. 

- [10] D. Chirtoaca, J. Ellul, and G. Azzopardi, “A framework for creating deployable smart contracts for non-fungible tokens on the ethereum blockchain,” in _2020 IEEE International Conference on Decentralized Applications and Infrastructures (DAPPS)_ . IEEE, 2020, pp. 100–105. 

- [11] FreedomFi, “Freedomfi 5g.” [Online]. Available: https://freedomfi.com/ 

- [12] F. Kaltenberger, A. P. Silva, A. Gosain, L. Wang, and T.-T. Nguyen, “Openairinterface: Democratizing innovation in the 5g era,” _Computer Networks_ , vol. 176, p. 107284, 2020. 

- [13] G. Brown, “Service-based architecture for 5g core networks,” _A Heavy Reading white paper produced for Huawei Technologies Co. Ltd. Online verf¨ugbar unter: https://www. huawei. com/en/pressevents/news/2017/11/HeavyReading-WhitePaper-5G-Core-Network, letzter Zugriff am_ , vol. 1, p. 2018, 2017. 

- [14] H. Fang and H. Miao, “Introducing the model card toolkit for easier model transparency reporting,” 2020. 

- [15] E. Bandara, X. Liang, P. Foytik, S. Shetty, and K. De Zoysa, “A blockchain and self-sovereign identity empowered digital identity platform,” in _2021 International Conference on Computer Communications and Networks (ICCCN)_ . IEEE, 2021, pp. 1–7. 

- [16] E. Bandara, W. K. Ng, K. D. Zoysa, N. Fernando, S. Tharaka, P. Maurakirinathan, and N. Jayasuriya, “Mystiko - blockchain meets big data,” in _IEEE International Conference on Big Data, Big Data 2018, Seattle, WA, USA, December 10-13, 2018_ , 2018, pp. 3024–3032. 

- [17] E. Bandara, W. K. NG, K. De Zoysa, and N. Ranasinghe, “Aplos: Smart contracts made smart,” _BlockSys’2019_ , 2019. 

- [18] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa, and W. K. Ng, “Saas-microservices-based scalable smart contract architecture.” 

- [19] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

- [20] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

- [21] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

- [22] J. Backman, S. Yrj¨ol¨a, K. Valtanen, and O. M¨ammel¨a, “Blockchain network slice broker in 5g: Slice leasing in factory of the future use case,” in _2017 Internet of Things Business Models, Users, and Networks_ . IEEE, 2017, pp. 1–8. 

- [23] N. Afraz and M. Ruffini, “5g network slice brokering: A distributed blockchain-based market,” in _2020 European Conference on Networks and Communications (EuCNC)_ . IEEE, 2020, pp. 23–27. 

- [24] N. Nikaein, E. Schiller, R. Favraud, K. Katsalis, D. Stavropoulos, I. Alyafawi, Z. Zhao, T. Braun, and T. Korakis, “Network store: Exploring slicing in future 5g networks,” in _Proceedings of the 10th International Workshop on Mobility in the Evolving Internet Architecture_ , 2015, pp. 8–13. 

- [25] M. A. Togou, T. Bi, K. Dev, K. McDonnell, A. Milenovic, H. Tewari, and G.-M. Muntean, “Dbns: A distributed blockchain-enabled network slicing framework for 5g networks,” _IEEE Communications Magazine_ , vol. 58, no. 11, pp. 90–96, 2020. 

564 

