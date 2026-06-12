## **GREENTHREAD — BLOCKCHAIN, NON-FUNGIBLE TOKEN(NFT), MODEL CARDS, SELF-SOVEREIGN IDENTITY AND IPFS ENABLED SUSTAINABLE CIRCULAR FASHION PLATFORM** 

Nadini Sahabandu Eranga Bandara Peter Foytik GoodLife X Sachin Shetty Colombo, SRI LANKA Ravi Mukkamala nadini.sahabandu@gmail.com Old Dominion University Norfolk, VA, USA {cmedawer, sshetty, pfoytik, rmukkama}@odu.edu 

Abdul Rahman Xueping Liang Deloitte & Touche LLP Florida International University AI Center of Excellence, GA, USA Miami, FL, USA abdulrahman@deloitte.com xuliang@fiu.edu 

## **ABSTRACT** 

Circular fashion aims to design, produce, and provide fashion items for long-term usage and circulation in society, while minimizing textile waste. However, promoting this practice has been challenging due to a lack of data privacy, anonymity, and security requirements of participants in the circular fashion ecosystem. In this paper, we propose “GreenThread" a blockchain, Model Cards, Self-Sovereign Identity(SSI), and IPFS-enabled sustainable circular fashion platform. It integrates a Non-Fungible Token(NFT) enabled circular fashion marketplace. Each circular fashion item can be represented as a NFT token and sold in this marketplace, appealing to the buyers’ preference to showcase their ownership of the item. Users can interact with the platform via a SSI-enabled mobile application. This capability of GreenThread ensures the anonymity, security and data privacy requirements of participants. GreenThread encodes data provenance information of circular fashion items into Model Cards and stores them in the IPFS. This way, we guarantee proof of circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer. 

**Keywords:** Blockchain, NFT, Model Cards, IPFS, Self-Sovereign Identity, Circular fashion. 

## **1 INTRODUCTION** 

Circular fashion models are gaining momentum across the fashion industry, as consumers become more and more aware of the environmental and social impacts of their clothing choices. As such, upcycling is considered a "design-based circular fashion approach," which promotes re-purposing textile waste material, 

_ANNSIM’23, May 23-26, 2023, Mohawk College, ON, CANADA; ©2023 Society for Modeling & Simulation International (SCS)_ 

357 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

both prior to and post-consumption, to create new fashion products (Yoo, Jung, Oh, et al. 2021). Further, as this refers to transforming unwanted products and textile waste into something of "higher value", the need for proving the authenticity of how the "higher value" is achieved is important. The practice of upcycled fashion is a growing trend among fashion designers, which in turn is driving the shift towards saving resources and reducing the textile waste which ends up in landfills(Yoo, Jung, Oh, et al. 2021). 

According to the Ellen MacArthur Foundation, the implementation of circular business models in the fashion industry could generate an economic opportunity worth 560 billion USD (US Dollars) (Yoo, Jung, Oh, et al. 2021). The foundation estimates that the restoration, resale, repair, customization, and rental markets in fashion are already worth over 73 billion USD, with companies such as Depop and Rent the Runway valued at over one billion USD (Vehmas, Raudaskoski, Heikkilä, Harlin, Mensonen, et al. 2018). The foundation projects that circular models in the fashion industry could grow from 3.5% of the global market today to 23% by 2030. 

One of the main challenges in promoting circular fashion is the lack of ability to trace the source of raw materials and the transparency that can be provided in the evolution of the product. While most upcycled items have a story attached to them with facts, the authenticity of the presented data is a major concern. This is based on the potential to fabricate the source to be more appealing to potential buyers. As a result, it is important to address the data privacy, anonymity, and security requirements of participants in the circular fashion industry. 

In this paper, we propose “GreenThread," a blockchain, Model Card (a structured framework to enable ML model provenance)(Bandara, Shetty, Rahman, Mukkamala, Zhao, Liang, et al. 2022), (Wadhwani and Jain 2020a), self-sovereign identity (Liu, Sun, and Schuckers 2019), and IPFS-enabled sustainable circular fashion platform. GreenThread provides circular fashion functions by enhancing privacy, transparency, traceability, anonymity, and data provenance in a scalable manner(Feng, He, Zeadally, Khan, Kumar, et al. 2019). All the circular fashion items in the platform are represented as NFT tokens(Wang, Li, Wang, Chen, et al. 2021), (Chohan and Paschen 2021), (Dowling 2022) and published in the NFT marketplace. Then, customers can purchase these items via paying crypto/fiat currency. The purchased items will be delivered to the customer via transport providers. The revenue will be shared among the entities in the circular fashion ecosystem (e.g designers, material providers, manufacturers, transport agents etc). We have designed a novel NFT token schema to represent fashion items as NFT tokens. Users (e.g. buyers, sellers) in the platform can interact with the platform via a self-sovereign identity (SSI)-enabled mobile wallet application(Liang, Shetty, Zhao, Bowden, Li, Liu, et al. 2017), (Mühle, Grüner, Gayvoronskaya, and Meinel 2018). The mobile wallet application facilitates buying and selling circular fashion items as NFT tokens. This capability of GreenThread ensures the anonymity, security, and data privacy requirements of participants. GreenThread encodes data provenance and audit information of circular fashion items into Model Cards and stores them in the IPFS peer-to-peer storage system (Benet 2014). These IPFS hashes of the items are linked to the blockchain ledger. This way, we guarantee proof of circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer. The following are our main contributions: 

1. Provided enhanced transparency and provenance of the circular fashion items with the use of Model Card objects and IPFS; 

2. Proposed a model to represent circular fashion items as NFT tokens. Designed an extensible NFT schema to represent circular fashion items as NFT tokens. 

3. Proposed an NFT token-based circular fashion marketplace to trade fashion items. 

4. Employed Self-sovereign identity-enabled mobile wallet for users to anonymize the trading functions. 

5. Implemented end-to-end circular fashion functions with blockchain smart contracts. 

358 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

**==> picture [385 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) GreenThread platform layers. (b) GreenThread platform blockchain ledger.<br>**----- End of picture text -----**<br>


Figure 1: GreenThread platform architecture. 

The rest of the paper is organized as follows. Section 2 formulates the architecture of the GreenThread platform. Section 3 presents the functionality of the platform, Section 4 summarizes the results from our performance evaluation studies. Section 5 provides a survey of related work. Finally, section 6 summarizes our contributions with suggestions for future work. 

## **2 GREENTHREAD PLATFORM ARCHITECTURE** 

GreenThread platform consists of four main layers: (1) the Stakeholder layer, (2) the Blockchain and Smart contract layer, (3) the NFT marketplace layer, (4) the Customer layer. These are shown in Figure 1(a). 

## **2.1 Resource Provider Layer** 

Multiple resource providers (e.g. designers, material providers, manufacturers, transport agents) collaborate on the GreenThread platform. Material providers provide reusable materials, designers create designs with reusable clothes. Manufacturers manufacture fashion items with the designs provided by designers and materials provided by material providers. The manufactured fashion items are sold in the NFT marketplace. Buyers purchase fashion items as NFT tokens by paying fiat currency. Once the NFT token is bought, the corresponding real fashion item is delivered to the customer. GreenThread provides web and mobile-based systems for different entities to interact with the platform (e.g. SSI-enabled mobile wallets for customers). These entities are onboarded to the platform via these mobile/web applications. To ensure anonymity, their identities are managed within the system as self-sovereign identity proofs (SSI-Proofs). While the actual identities are stored on the individual user’s mobile wallet, the proofs of the identities are stored on the blockchain as SSI-Proofs. 

## **2.2 Blockchain and Smart Contract Layer** 

GreenThread platform’s functionalities are implemented using blockchain with smart contracts running on them(Bandara, NG, DE Zoysa, Fernando, Tharaka, Maurakirinathan, and Jayasuriya 2018). The blockchain may be supported by and deployed among multiple organizations, with each organization running its own blockchain node (Figure 1(a)). The blockchain stores digital identity proofs (referred to as SSI proofs 

359 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

**==> picture [197 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Model Card encoded data provenance information.<br>**----- End of picture text -----**<br>


**==> picture [215 x 18] intentionally omitted <==**

**----- Start of picture text -----**<br>
(b) NFT token schema to represent the reusable fashion<br>item.<br>**----- End of picture text -----**<br>


Figure 2: GreenThread Model Cards and NFTs. 

(Baars 2016)) of entities such as designers, manufacturers, and customers, data provenance information (e.g. circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer) of circular fashion items, and NFT token information. The data provenance information is encoded into Model Card objects and stored in the IPFS peer-to-peer storage system. Then, the IPFS hashes of the Model Cards are linked to the NFTs. NFT tokens are generated with a custom NFT token schema Figure 2. Each blockchain node comes with a Model Card service and NFT service (Figure1(b)). Here, the Model Card and the NFT handle the Model Card object generation functions and the NFT token generation functions, respectively. 

Greenthread supports four main smart contracts: Identity contract, Coordination contract, Model Card contract, and NFT contract. The Identity contract is in-charge of the peers’ identity management (e.g., identity registration, permissions handling etc.). The reusable fashion item manufacturing and coordination between entities handled by the Coordination contract (e.g., find materials, manufacturing functions, transport functions etc.). The Model Card contract handles the data provenance function (Wadhwani and Jain 2020b) via talking with the Model Card service in the blockchain nodes. As mentioned above, the data provenance of the fashion items is encoded into Model cards and stored in the blockchain ledger. Figure 3 represents the model card generated in the GreenThread platform. It contains an end-to-end audit log of the circular fashion item. Customers who buy fashion items can fetch this provenance information via the mobile wallet and verify it. The NFT contract is in charge of NFT token creation and trading functions. The customer NFT token schema reports the circular fashion item described in Figure 2. 

## **2.3 NFT Marketplace Layer** 

We have created an NFT-based reusable fashion marketplace in GreenThread. It employs an NFT token schema to represent the reusable fashion items as NFT tokens. Figure 2 describes the schema of the NFT token in the GreenThread platform. It contains metadata such as Name, Description, Slice template URI, Creator, and a link to the Model Card objects with the data provenance information of the slice. The NFT service handles the NFT token generation functions using this schema. The generated tokens are in JSON 

360 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

format. The marketplace layer interacts with smart contracts on the blockchain to generate NFT tokens that are linked to the fashion item. The generated NFTs are stored in the blockchain ledger. 

## **2.4 Customer Layer** 

Customers are the end-users of the GreenThread platform, and they will interact with the system using SSI-enabled mobile wallet applications. Customers can purchase reusable fashion items from the NFT marketplace. When purchasing fashion items as NFTs, customers can verify the audit and provenance information of the circular fashion item via the Model Card object link to the NFT token. When verifying the provenance, the smart contract fetches the Model Card object stored in the IPFS storage and extracts the audit log from it. In this way, customers can verify end-to-end proofs of the fashion item, such as proof of circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer. 

## **3 GREENTHREAD PLATFORM FUNCTIONALITY** 

GreenThread is a platform that connects buyers, sellers, suppliers of raw materials and other intermediaries in the circular fashion space. This platform is aimed at mitigating the barriers for buyers’ purchase decisions for circular fashion, and further strengthens the motivators for the buying decision. For the buyers of sustainable and circular fashion, the motivation to purchase is driven majorly through knowing and understanding the history and evolution of the product being bought. This serves as a gateway for data collection, which will support the concept of circularity and wider adoption of the practicality of circularity. Next, this addresses reporting of the chain of activities and evolution of the product to appeal to the sentiments and buying decisions of the target market. Finally, this supports the digital representation of the unique upcycled product through NFT, supported by the attributes of blockchain in having traceability of the evolution, sourcing of material and resources for the product, its ownership transfer, etc. The following are the main functionalities of GreenThread. 

## **3.1 Identity Registration** 

The platform will facilitate the onboarding of all stakeholder groups in the circular fashion item production process, including the concept owners, designers, vendors, buyers, and other intermediaries. The onboarding will be done through the Self-Sovereign Identity approach. Stakeholders will be registered in the platform via SSI-enabled mobile wallet. When registering it captures the user’s personal identity information and generates SSI proof. The SSI proof will be uploaded to the blockchain ledger and the real identities will be stored in the identity wallet. In this way, we guarantee the privacy of the user data in the GreenThread platform. 

## **3.2 Designing the circular fashion item** 

The design of a circular fashion item differs from that of the typical fashion design process. In line with this, the system will facilitate the designer to evaluate the material, fabric or production waste to be used, followed by the sketch of the design. The platform will facilitate the conversation between the designer and the concept owner to finalize the design and record the planned usage of material, and the impact on the environment with each selected raw material. All data points at the design stage data will be recorded in the blockchain, including the concept owner, planned material, and its estimated impact on the environment. 

361 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

## **3.3 Fashion Item Manufacturing** 

The finalized design of the fashion item then goes into the process of manufacturing. During this phase, the platform will support the collaboration between material providers (i.e. vendors), which will record data points of the actual material used, quantity, material source, manufacturing process flow, and other important details in the manufacturing phase. 

## **3.4 NFT Token Issuing** 

NFT tokens will allow a virtual marketplace for tokens representing the physical circular fashion product and establishing the ownership of the actual physical goods and the products’ history of evolution. This further facilitates the integration of buyer motivation for circular fashion and circularity through an NFT marketplace. Once manufactured, the fashion items will be represented as NFT tokens. GreenThread encodes fashion items attributes and data provenance model card object information into a custom NFT token schema shown in Figure 2. 

## **3.5 Circular Fashion Item Trading** 

The generated NFT tokens will be published in the NFT marketplace in GreenThread. Participants of the GreenThread platform can buy/sell circular fashion items as NFT tokens from the marketplace. The purchased items will be delivered to the customer via transport providers. The revenue will be shared among the entities in the circular fashion ecosystem (e.g designers, material providers, manufacturers, transport agents etc). 

## **3.6 Data Privacy, Security and Anonymity** 

Our platform is designed to especially enforce the privacy, anonymity and reliability of the participants in the circular fashion industry. Users (e.g., buyers and sellers) in the platform can interact with the platform via a self-sovereign identity (SSI)-enabled mobile wallet application(Liang, Shetty, Zhao, Bowden, Li, Liu, et al. 2017), (Mühle, Grüner, Gayvoronskaya, and Meinel 2018). The mobile wallet application facilitates buying and selling circular fashion items as NFT tokens. Real identities and personal data of users are managed within the system as self-sovereign identity proofs (SSI-Proofs) stored on the blockchain, ensuring anonymity. The real identity information itself is stored on the individual user’s mobile wallet. This capability of GreenThread ensures the anonymity, security, and data privacy requirements of participants. 

## **3.7 Data Provenance** 

One of the main features of the GreenThread platform is to facilitate the data provenances and audit functions of circular fashion items. GreenThread records the circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer into Model Cards objects and stored them in IPFSbased peer-to-peer decentralized storage. Then the IPFS hashes link to the NFT tokens and are stored in the blockchain ledger. The buyers of the NFT tokens can verify the provenance information at any given time. This way, we guarantee proof of circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer. 

## **4 GREENTHREAD IMPLEMENTATION AND EVALUATION** 

The proposed GreenThread platform is designed based on a microservices architecture to operate in a highly scalable environment. Figure 3(b) shows the different microservices in the platform. The Ra- 

362 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

**==> picture [426 x 9] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) GreenThread platform functionality flow (b) GreenThread platform microservices based architecture.<br>**----- End of picture text -----**<br>


Figure 3: GreenThread platform implementation. 

hasak blockchain (Bandara, Liang, Foytik, Shetty, Ranasinghe, De Zoysa, et al. 2021) has been used as the blockchain ledger in GreenThread. The smart contract functions are implemented with the Rahasak blockchain’s Aplos (Bandara, Ng, Ranasinghe, and Zoysa 2019) framework, which is a Scala functional programming language and an Akka actor-based smart contract platform (Odersky, Altherr, Cremet, Emir, Maneth, Micheloud, Mihaylov, Schinz, Stenman, and Zenger 2004), (Gupta 2012). The JSON-encoded token structure is used to construct the NFT service. The TensorFlow Model Card Toolkit (Wadhwani and Jain 2020a) is used to build the Model Card service. IPFS and Libp2p pubsub ( **?** ) is used as the blob storage and peer-to-peer communication channel. The evaluation tests of GreenThread have been performed with this blockchain setup, using a varying number of peers. The following evaluation test results for GreenThread below are representative of a varying number of peers and records. 

**Transactions Execution Performance** – The invoke(write) and query(read) transaction performance compared in Figure 4. The invoke transactions do mutation of the underlying blockchain storage. The Query transactions only read the data from the ledger and do not update the asset status(Bandara, Liang, Foytik, Shetty, Ranasinghe, Zoysa, and Ng 2021). Due to this reason, query transactions perform higher throughput when compared to invoke transactions. 

**Transaction Execution Rate** – The number of executed and submitted transactions in a single blockchain peer is depicted in Figure 5. Between the rates of submitted transactions and executed transactions, there is a back pressure operation (Bandara, Tosh, Foytik, Shetty, Ranasinghe, and De Zoysa 2021). To manage these backpressure tasks, we employed a reactive streaming-based strategy. 

**Performance of Transaction Scalability and Latency** – The throughput of the invoke transactions is measured for a varying number of peers. Figure 6 compares the throughput of query and invoke transactions. It shows that both query and invoke transaction throughput linearly increase with the number of peers. Figure 7 shows the same for transaction latency. Clearly, the latency for query transactions is much smaller than the latency for invoke transactions. As the number of peers increases, there is a slight decrease in the latency. 

**Block Generation Time and Block Size** : First the block generation time metrics are calculated with the block size. Block size depends on the number of transactions in the block. As shown in Figure 8 it takes 8.5 

363 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

Figure 4: Transaction throughput of the blockchain. 

Figure 6: Transaction scalability of the blockchain. 

Figure 8: Block creation time #of transactions. 

Figure 5: Transaction execution rate. 

Figure 7: Transaction latency of the blockchain. 

Figure 9: Block creation time # of peers. 

364 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

seconds to create a block with 10k transaction records. The average block creation time varies based on the number of transactions in the block. 

**Block Generation Time and the Number of Peers** : Then we evaluated the block generation time metrics with the number of blockchain peers in the network. Figure 9 shows how the block generation time increased when the number of peers increased. The average block production time variable is based on the number of blockchain peers. 

## **5 RELATED WORK** 

To date, there is various research work conducted that attempts to integrate a blockchain-based ecosystem to address the issues and concerns of the circular fashion industry. In this section, we outline the main features of such work. Presented in Table 1 is the feature comparison summary of those works in comparison to GreenThread. 

**CE-Leather** , a study conducted by (Shou and Domenech 2022) on the integration of Life Cycle Assessment(LCA) and blockchain technology to promote circular fashion, based on a case study of leather handbags, found that circular fashion strategies can reduce the environmental impact of the fashion industry. The research highlighted that data acquisition and transparency are major barriers to accurately assessing impact and proposed an integrated LCA-blockchain framework as a solution for better traceability and monitoring of impact reduction potential. In **CE-Built** (Shojaei, Ketabi, Razkenari, Hakim, Wang, et al. 2021) investigates the use of blockchain technology as a robust and comprehensive infrastructure to address the need for a network that can record, store, and disseminate pertinent information for enabling a CE in the built environment. The hypothesis of this study is that the blockchain platform can provide comprehensive, transparent, and reliable material traceability and information tracking not only to a material’s source, but also its current state. **CSCM** (Wang, Luo, Zhang, Tian, Li, et al. 2020) proposed blockchain-enabled supply-chain management framework for circular fast fashion items. It tries to address the major traceability and data provenance functions of the circular fashion supply chain. **CE-Smart** (Kumar and Chopra 2022) adopt blockchain smart contract to implement the functions of the circular economy. The platform mainly focused to improve trust and transparency in supply-chain networks, shared and performance economy platforms, stakeholder participation, and governance and management of organizations. **Dresscode** (Heim and Hopper 2022) proposed an approach of the digital transformation circular fashion industry with blockchain, smart contract and NFT-based technology. They mainly focus on the business aspect and security aspects of the digital transformation of the circular fashion industry. 

Table 1: Circular Economy Platform Comparison. 

|Platform|Domain|Blockchain<br>Enabled|Scalability|Privacy<br>Level|SSI|Model Cards|Auditing &<br>Provenance|NFT<br>Support|
|---|---|---|---|---|---|---|---|---|
|GreenThread|Fashion|✓|High|High|✓|✓|✓|✓|
|CE-Leather|Fashion|✓|N/A|Mid|✗|✗|✗|✗|
|CE-Built|Built|✓|N/A|Mid|✗|✓|✓|✗|
|CSCM|Supplychain|✓|Mid|Mid|✗|✗|✓|✗|
|CE-Smart|Fashion|✓|N/A|Mid|✗|✗|✗|✗|
|Dresscode|Fashion|✗|Mid|Mid|✗|✗|✓|✓|



365 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

## **6 CONCLUSIONS AND FUTURE WORK** 

With GreenThread, we have proposed a blockchain, Model Cards, self-sovereign identity, and IPFS-enabled sustainable circular fashion platform. GreenThread provides circular fashion functions by enhancing privacy, transparency, traceability, anonymity, and data provenance in a scalable manner. All the circular fashion items in the platform are represented as NFT tokens and traded in the NFT marketplace. Then customers can purchase these fashion items as NFT tokens. We have implemented a revenue share model where NFT revenues can be distributed among the resource providers in the circular fashion ecosystem (e.g designers, material providers, manufacturers, transport agents, etc). To guarantee data privacy, anonymity, and security concerns, we have incorporated SSI-enabled mobile wallet applications for customers and resource providers in the platform. Furthermore, GreenThread encodes data provenance and audit information of circular fashion items into Model Cards and stores them in the IPFS peer-to-peer storage system. These IPFS hashes of the items are linked to the blockchain ledger. This way, we guarantee proof of circularity of source materials, manufacturing, and upcycling processes, and evidence for ownership transfer. In future works, we are planning to incorporate GreenThread platform into real customer use cases in the fashion industry. 

## **ACKNOWLEDGEMENTS** 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## **REFERENCES** 

- Baars, D. 2016. “Towards self-sovereign identity using blockchain technology”. Master’s thesis, University of Twente. 

- Bandara, E., X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa et al. 2021. “Rahasak-Scalable blockchain architecture for enterprise applications”. _Journal of Systems Architecture_ , pp. 102061. 

- Bandara, E., X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. D. Zoysa, and W. K. Ng. 2021. “SaaS - Microservices-Based Scalable Smart Contract Architecture”. In _Security in Computing and Communications_ , edited by S. M. Thampi, G. Wang, D. B. Rawat, R. Ko, and C.-I. Fan, pp. 228–243. Singapore, Springer Singapore. 

- Bandara, E., W. K. NG, K. DE Zoysa, N. Fernando, S. Tharaka, P. Maurakirinathan, and N. Jayasuriya. 2018. “Mystiko—Blockchain Meets Big Data”. In _2018 IEEE International Conference on Big Data (Big Data)_ , pp. 3024–3032. Institute of Electrical and Electronics Engineers, Inc. 

- Bandara, E., W. K. Ng, N. Ranasinghe, and K. D. Zoysa. 2019. “Aplos: Smart Contracts Made Smart”. In _Blockchain and Trustworthy Systems - First International Conference, BlockSys 2019, Guangzhou, China, December 7-8, 2019, Proceedings_ , edited by Z. Zheng, H. Dai, M. Tang, and X. Chen, Volume 1156 of _Communications in Computer and Information Science_ , pp. 431–445, Springer. 

- Bandara, E., S. Shetty, A. Rahman, R. Mukkamala, J. Zhao, X. Liang et al. 2022. “Bassa-ML—A Blockchain and Model Card Integrated Federated Learning Provenance Platform”. In _2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC)_ , pp. 753–759. Institute of Electrical and Electronics Engineers, Inc. 

- Bandara, E., D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa. 2021. “Tikiri—Towards a lightweight blockchain for IoT”. _Future Generation Computer Systems_ vol. 119, pp. 154–165. 

- Benet, J. 2014. “Ipfs-content addressed, versioned, p2p file system”. _arXiv preprint arXiv:1407.3561_ . 

366 

## _Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

- Chohan, R., and J. Paschen. 2021. “What marketers need to know about non-fungible tokens (NFTs)”. _Business Horizons_ . 

- Dowling, M. 2022. “Is non-fungible token pricing driven by cryptocurrencies?”. _Finance Research Letters_ vol. 44, pp. 102097. 

- Feng, Q., D. He, S. Zeadally, M. K. Khan, N. Kumar et al. 2019. “A survey on privacy protection in blockchain system”. _Journal of Network and Computer Applications_ vol. 126, pp. 45–58. 

- Gupta, M. 2012. _Akka essentials_ . Packt Publishing Ltd. 

- Heim, H., and C. Hopper. 2022. “Dress code: the digital transformation of the circular fashion supply chain”. _International Journal of Fashion Design, Technology and Education_ vol. 15 (2), pp. 233–244. 

- Kumar, N. M., and S. S. Chopra. 2022. “Leveraging blockchain and smart contract technologies to overcome circular economy implementation challenges”. _Sustainability_ vol. 14 (15), pp. 9492. 

- Liang, X., S. Shetty, J. Zhao, D. Bowden, D. Li, J. Liu et al. 2017. “Towards decentralized accountability and self-sovereignty in healthcare systems”. In _International Conference on Information and Communications Security_ , pp. 387–398. Springer. 

- Liu, Y., G. Sun, and S. Schuckers. 2019. “Enabling Secure and Privacy Preserving Identity Management via Smart Contract”. In _2019 IEEE Conference on Communications and Network Security (CNS)_ , pp. 1–8. Institute of Electrical and Electronics Engineers, Inc. 

- Mühle, A., A. Grüner, T. Gayvoronskaya, and C. Meinel. 2018. “A survey on essential components of a self-sovereign identity”. _Computer Science Review_ vol. 30, pp. 80–86. 

- Odersky, M., P. Altherr, V. Cremet, B. Emir, S. Maneth, S. Micheloud, N. Mihaylov, M. Schinz, E. Stenman, and M. Zenger. 2004. “An overview of the Scala programming language”. Technical report. 

- Shojaei, A., R. Ketabi, M. Razkenari, H. Hakim, J. Wang et al. 2021. “Enabling a circular economy in the built environment sector through blockchain technology”. _Journal of Cleaner Production_ vol. 294, pp. 126352. 

- Shou, M., and T. Domenech. 2022. “Integrating LCA and blockchain technology to promote circular fashion–A case study of leather handbags”. _Journal of Cleaner Production_ vol. 373, pp. 133557. 

- Vehmas, K., A. Raudaskoski, P. Heikkilä, A. Harlin, A. Mensonen et al. 2018. “Consumer attitudes and communication in circular fashion”. _Journal of Fashion Marketing and Management: An International Journal_ . 

- Wadhwani, A., and P. Jain. 2020a. “Machine Learning Model Cards Transparency Review: Using model card toolkit”. In _2020 IEEE Pune Section International Conference (PuneCon)_ , pp. 133–137. Institute of Electrical and Electronics Engineers, Inc. 

- Wadhwani, A., and P. Jain. 2020b. “Machine Learning Model Cards Transparency Review: Using model card toolkit”. In _2020 IEEE Pune Section International Conference (PuneCon)_ , pp. 133–137. Institute of Electrical and Electronics Engineers, Inc. 

- Wang, B., W. Luo, A. Zhang, Z. Tian, Z. Li et al. 2020. “Blockchain-enabled circular supply chain management: A system architecture for fast fashion”. _Computers in Industry_ vol. 123, pp. 103324. 

- Wang, Q., R. Li, Q. Wang, S. Chen et al. 2021. “Non-fungible token (NFT): Overview, evaluation, opportunities and challenges”. _arXiv preprint arXiv:2105.07447_ . 

- Yoo, F., H. J. Jung, K. W. Oh et al. 2021. “Motivators and barriers for buying intention of upcycled fashion products in China”. _Sustainability_ vol. 13 (5), pp. 2584. 

367 

_Sahabandu, Bandara, Shetty, Foytik, Mukkamala, Rahaman, and Liang_ 

## **AUTHOR BIOGRAPHIES** 

**NADINI SAHABANDU** is an alumna of Good Life X, a community promoting and driving change towards a circular economy. She is also an experienced Business Consultant and Digital Product Leader with 12+ years experience in the IT industry. She has experience in multiple domains and geographies, bridging the gap between business problems and technology. As a passionate contributor of humanizing technology, she took special interest in Circular Economy, Sustainability and ESG, combining her experience in the tech space of data, AI, blockchain, inclusive digital design and accessibility, etc. to provide solutions. Her email address is nadini.sahabandu@gmail.com. 

**ERANGA BANDARA** worked as a Senior Research Scientist at the Virginia Modeling Analysis and Simulation Center (VMASC) Virginia USA. His research interests include Distributed Systems, Blockchain, Big Data, Actor based Systems and Functional programming. He worked as a Lead Engineer at Pagero AB Sweden. With Pagero AB he was involved with research and developments in Distributed Systems, Functional Programming, Big Data, Actor based systems and DevOps. His email address is cmedawer@odu.edu. 

**SACHIN SHETTY** is an Associate Director in the Virginia Modeling, Analysis and Simulation Center at Old Dominion University and an Associate Professor with the Department of Computational Modeling and Simulation Engineering. Sachin Shetty received his PhD in Modeling and Simulation from the Old Dominion University in 2007. His laboratory has been supported by the National Science Foundation, Air Office of Scientific Research, Air Force Research Lab, Office of Naval Research, Department of Homeland Security, and Boeing. He has published over 150 research articles in journals and conference proceedings and four books. He is the recipient of Commonwealth Cyber Initiative Research Fellow, Fulbright Specialist award, EPRI Cybersecurity Research Challenge award, DHS Scientific Leadership Award and has been inducted in Tennessee State University’s million-dollar club. His email address is sshetty@odu.edu. 

**PETER FOYTIK** is currently a Lead Project Scientist at the Virginia Modeling Analysis and Simulation Center (VMASC) Virginia USA. His work has included modeling and simulation applications in transportation, department of defense, and cyber security. His research interests include Modeling and Simulation, Distributed Systems, Blockchain, and Analysis of Complex Systems. He is currently working on his PhD research which is targeted on the use of modeling and simulation to explore problem spaces. His email address is pfoytik@odu.edu. 

**RAVI MUKKAMALA** is a Professor and Chair of the Department of Computer Science at Old Dominion University, Norfolk, Virginia. He received his Ph.D. in Computer Science from the University of Iowa, Iowa, City. His primary areas of research interests include performance evaluation, distributed systems, and cybersecurity. He has published over 200 papers in refereed journals and conference proceedings in these areas. He is currently working on the privacy and security aspects of blockchain technologies as well as their applicability in different domains. His email address is rmukkama@odu.edu. 

**ABDUL RAHMAN** received Ph.D.s in mathematics and physics from Howard University. His research interests include research in innovative and novel applications of AI and ML to building improved and robust cyber network operations (CNO) capabilities. His email address is abdulrahman@deloitte.com. 

**XUEPING LIANG** is an Assistant Professor in Florida International University. Her main research focus involves security and privacy, trusted computing, distributed architecture, and blockchain. She has worked on several research and application projects using Intel SGX and blockchain, and one of her papers has been awarded the Top 50 Most Influential Papers of Blockchain in 2019. Dr. Liang received her PhD in Cyber Security from the Institute of Information Engineering at the University of Chinese Academy of Sciences in 2019. Her email address is xuliang@fiu.edu. 

368 

