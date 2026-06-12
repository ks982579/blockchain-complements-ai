2024 IEEE 21st International Conference on Mobile Ad-Hoc and Smart Systems (MASS) 

# Generative-AI(with Custom-Trained Meta’s Llama2 LLM), Blockchain, NFT, Federated Learning and PBOM enabled Data Security Architecture for Metaverse on 5G/6G Environment 

Eranga Bandara _[∗]_ , Peter Foytik _[∗]_ Sachin Shetty _[∗]_ , Amin Hassanzadeh _[†]_ , 

> _∗ {_ cmedawer, pfoytik, sshetty _}_ @odu.edu 

Old Dominion University, Norfolk, VA, USA 

> _†_ amin.hassanzadeh@accenture.com Accenture Technology Labs, Arlington VA, USA 

_**Abstract**_ **—The Metaverse is an integrated network of 3D virtual worlds accessible through a virtual reality headset. Its impact on data privacy and security is increasingly recognized as a major concern. There is a growing interest in developing a reference architecture that describes the four core aspects of its data: acquisition, storage, sharing, and interoperability. Establishing a secure data architecture is imperative to manage users’ personal data and facilitate trusted AR/VR and AI/ML solutions within the Metaverse. This paper details a reference architecture empowered by Generative-AI, Blockchain, Federated Learning, and Non-Fungible Tokens (NFTs). Within this architecture, various resource providers collaborate via the blockchain network. Handling personal user data and resource provider identities is executed through a Self-Sovereign Identity-enabled privacy-preserving framework. AR/VR devices in the Metaverse are represented as NFT tokens available for user purchase. Software updates and supply-chain verification for these devices are managed using a Software Bill of Materials (SBOM) and a Pipeline Bill of Materials (PBOM) verification system. Moreover, a custom-trained Llama2 LLM from Meta has been integrated to generate PBOMs for AR/VR devices’ software updates, thereby preventing malware intrusions and data breaches. This Llama213B LLM has been quantized and fine-tuned using Qlora to ensure optimal performance on consumer-grade hardware. The provenance of AI/ML models used in the Metaverse is encapsulated as Model Card objects, allowing external parties to audit and verify them, thus mitigating adversarial learning attacks within these models. To the best of our knowledge, this is the very first research effort aimed at standardizing PBOM schemas and integrating Language Model algorithms for the generation of PBOMs. Additionally, a proposed mechanism facilitates different AI/ML providers in training their machine learning models using a privacy-preserving federated learning approach. Authorization of communications among AR/VR devices in the Metaverse is conducted through a Zero-Trust security-enabled rule engine. A system testbed has been implemented within a 5G environment, utilizing Ericsson new Radio with Open5GS 5G core.** 

_**Index Terms**_ **—Metaverse, Generative-AI, LLM, Llama2, 5G, 6G, Blockchain, Federated Learning, NFT** 

## I. INTRODUCTION 

Metaverse is defined as a three-dimensional virtual reality space where users can interact with digital objects and with each other in an immersive surrounding. It is persistent, real- 

time, infinite, self-sustaining, interoperable, and decentralized [1]. It has Web 3.0 as its foundation [2] and is directly related to such technologies as blockchain, augmented and mixed reality, and Non-Fungible Tokens (NFT) [3]. Here, a user is immersed in a virtual space where they can do everything they do in real life, such as visiting exciting places, meeting people, and buying or selling works of art and real estate. Today, many blockchain-based platforms use NFT and cryptocurrency to provide an ecosystem for creating, owning, and monetizing decentralized digital assets [3]. In Metaverse different groups build different virtual worlds(e.g., Decentraland, Sandbox, AltspaceVR etc). As people move from one virtual world to the other, say from Decentraland to AltspaceVR, they may also want to move their assets from one to the other. When two virtual worlds are interoperable, the underlying blockchain can authenticate proof of ownership of the digital goods in both virtual worlds. Since Metaverse is a social platform utilizing interactive technologies such as virtual reality, augmented reality and machine learning, its impact on data privacy and security is now being questioned [4]. Since it promotes virtual platform users to lead a visual life within, the possibility of data collection and surveillance is likely to increase within it. In addition, since the underlying technologies (e.g., virtual reality, augmented reality, machine learning and AI) are behavioral-learning technologies, they tend to collect massive amounts of personal information, thereby threatening user privacy [4]. It could result in increased phishing attacks, malware attacks, data breaches, loss of personal data to entities such as marketers and advertisers, exposure of private information such as user biometrics, brainwave patterns, health data, etc. Moreover, the reliance on AR/VR devices for Metaverse interaction exposes users to risks associated with software updates, which can be compromised through supply-chain attacks [5]. Thus, safeguarding these technologies and the data they handle is imperative to protect users within the Metaverse. 

To address the imperative for interoperability and address the data privacy and security concerns of Metaverse users, 

2155-6814/24/$31.00 ©2024 IEEE DOI 10.1109/MASS62177.2024.00026 

118 

## _A. Resource Provider Layer_ 

Multiple resource providers exist in the proposed system Metaverse system, 1) VR/AR service providers(e.g. Decentraland, Sandbox, AltspaceVR etc), 2) AR/VR device providers and 3) AI/ML service providers. VR service providers facilitate VR/AR software services. AR/VR device providers provide AR/VR hardware devices. They sell physical AR/VR devices to customers. These devices in the system are represented as NFT tokens. The device’s communication policies are encoded into Zero-Trust security rules [13] and saved in the blockchain ledger. When customers purchase AR/VR devices, the ownership of the NFT token is transferred to the user in Metaverse. AI/ML providers facilitate AI/ML-based learning services(e.g., ML models). Their ML models can be trained with blockchain-enabled coordinator-less federated learning sytes [11]. The AR/VR service providers can use the models built by the AI/ML service providers. 

Fig. 1: Proposed Metaverse system architecture. 

this paper introduces a pioneering architecture that integrates Generative-AI (utilizing Meta’s Llama2 LLM) [6], [7], Blockchain [8], [9], PBOM [10], Federated Learning [11], and NFTs. We establish a prototype of this architecture on a test platform, demonstrating its feasibility and performance. The following highlights our primary contributions in this research: 

- 1) Introduction of a blockchain-enabled system, automating the data security functions within the Metaverse. 

- 2) Application of a custom-trained Llama2 LLM to generate PBOMs for AR/VR device software updates. 

- 3) Proposal for a model representing AR/VR devices as NFT tokens available for user purchase. 

- 4) Capture data provenance information of AI/ML models used in the Metaverse as Model Cards. 

- 5) Implementation of a prototype for the proposed system testbed with Ericsson RAN/Open5GS 5G core. 

## II. PROPOSED SYSTEM ARCHITECTURE 

Given the concerns surrounding privacy and security breaches for Metaverse users, we propose a novel decentralized Metaverse architecture integrating Generative-AI [7], Blockchain, Federated Learning, and NFT technologies to address these issues. The proposed system architecture is visually represented in Figure 1. The system primarily consists of five integral layers. The initial layer is the resource provider layer, distributed among multiple resource providers. Following is the blockchain and smart contract layer, facilitating the decentralized architecture. Subsequently, the LLM layer integrates a custom-trained Llama2 LLM to generate SBOM/PBOMs [10], [12] for the AR/VR device software updates, effectively preventing malware invasions and data breaches within these devices. The NFT marketplace layer is next in line, supporting transactions. Lastly, the end-user layer serves as an interface for user interaction with the system. 

## _B. Blockchain and Smart Contract Layer_ 

The blockchain network will be supported by different resource providers. A separate blockchain node can be deployed in each resource provider (AR/VR service providers, AR/VR device providers, AI/ML service providers). The Blockchain ledger stores the digital identities of the users/resource providers, NFT token information, Zero trust security policy details and data provenance information. Blockchain smart contracts are used to implement the main Metaverse features. It has four key smart contracts in the system: (1) Identity contract, (2) Policy contract, (3) Model Card contract, (4) SBOM contract, (5) PBOM contract and (6) NFT contract. The Identity contract is in charge of the users/service providers’ identity management (e.g., identity registration, permissions handling etc). The Policy contract is responsible for handling network policies(communication permissions and rules) of the AR/VR devices. The policies are encoded as Zero Trust Security rules and stored in the blockchain ledger. The Model Card contract handles the data provenance function. The data provenance information of the AI/ML models is encoded into Model cards [14]. Figure 2 shows an example model card created using the Metaverse platform. It includes data on clients with local models, peers in charge of aggregating the models, model generation times, and so on, as well as quantitative analysis and model considerations. The Syft and Grypy libraries are harnessed through the SBOM contract to expertly handle the intricate task of SBOM generation for the system’s AR/VR device software packages. These libraries play a pivotal role in scanning, identifying, and documenting the various components of the software libraries, ensuring the utmost clarity and security. PBOM standard provides a real-time list of software lineage, from the first line of code all the way to release [10]. PBOM Contract contract encompasses the custom-trained Llama-2 LLM [6] responsible for generating PBOMs for the software updates of AR/VR devices. These PBOMs are created as structured JSON schemas, encompassing critical details such as software update(e.g pull request) information(creator, versifier, approver, timestamps), 

119 

Fig. 3: Fine-tune Llama2 LLM with Qlora and deploy with Ollama. 

contracts leverage the capabilities of the Llama2 LLM through Ollam’s LLM API and LlamaIndex. 

## _D. NFT Marketplace Layer_ 

Fig. 2: Model card generated in Metaverse platform. 

test results, identified vulnerabilities, and the status of the pull request. The NFT contract is in charge of NFT token creation and trading. 

## _C. LLM Layer_ 

In contemporary software development practices, AR/VR device software systems are developed using the Continuous Integration/Continuous Delivery (CI/CD) approach. The development process begins with the creation of pull-requests via CI systems like GitHub, where developers submit their code changes for review. This process includes detailed verification, approval, and merging stages, ensuring that only thoroughly vetted and validated code modifications advance through the pipeline. Tracking of end-to-end pipeline verification using methods like PBOMs is crucial to prevent various software supply-chain attacks on these AR/VR devices. A critical component of our system is the LLM Layer, which employs a custom-trained Llama2 Language Model (LLM) to generate PBOMs for AR/VR devices’ software updates, thereby safeguarding against malware intrusions and data breaches. To prepare this Llama2 LLM, we’ve undertaken a meticulous training process, collaborating with Qlora with 4-bit quantized pretrained language model into Low Rank Adapters (LoRA) [15], Figure 3. The Llama2 LLM is specifically tailored to excel in the task of PBOM generation. During the training phase, we meticulously feed the model with a rich dataset, encompassing pull-request information, test result data, software package vulnerability scan results, statuses, and the desired PBOM format represented as a JSON schema [10]. As a result, the Llama2 LLM becomes proficient in responding to requests for the generation of new PBOMs for software updates, utilizing the input data provided, which typically includes pull-request details, SBOM information, statuses, and more. The finetuned Llama2 LLM run on Ollam [16]. The blockchain smart 

We are proposing an NFT-based marketplace for AR/VR devices in the Metaverse. The AR/VR device providers(e.g., manufacturers) present their AR/VR devices in the Metaverse as NFT tokens. We have designed a novel NFT token schema i528 to represent AR/VR devices. NFT token schema defines metadata such as Name, Description, and link to the Model Card objects with the data provenance information. The users of the platform can buy AR/VR devices from the service providers as NFT tokens. Once the token is bought, the ownership of the token will be handed over to the corresponding user. The AR/VR hardware device(physical device) which corresponds to the NFT token will be delivered to the user. Then the user can use the hardware device on the Metaverse. 

## _E. End User Layer_ 

The end-users interact with the Metaverse system via AR/VR devices. The identities of the users will be captured via the mobile wallet application based on the Self-Sovereign Identity(SSI) [17] approach. The user’s real identity data will be stored in the user’s physical mobile wallet. The proof of the data will be uploaded to the blockchain ledger. As mentioned above, users can purchase AR/VR devices from the device providers. When purchasing the AR/VR device NFTs, users can verify the audit and provenance information of the AR/VR devices via the Model Card objects published in the ledger. The software updates of the AR/VR devices can be verified with SBOM/PBOM enabled cyber supplychain provenance system [5]. These purchased devices can be used to interact with the Metaverse. There are different AR/VR service providers in the system. Users can enroll in their AR/VR spaces by paying via cryptocurrency. 

## III. SYSTEM FUNCTIONALITY 

This section discusses the specific functionalities of each layer in the system. 

## _A. Resource Provider Onboarding_ 

As mentioned, there are three different types of resource providers in the system 1) AR/VR service providers, 2) AI/ML service providers and 3) AR/VR device providers. The blockchain network will be formed between these resource 

120 

providers. The platform must initially be onboarded with the peer actors. When onboarding their identities will be verified and saved in the blockchain storage according to the SelfSovereign identity philosophy. Once these peers have been onboarded, separate blockchain nodes will be deployed for each of them. All of the peers’ identifying information is stored in the blockchain ledger. The blockchain’s identity smart contract implements this identity onboarding function [17]. 

## _B. Service Management_ 

Once resource providers are onboarded to the system, they can register their services. AR/VR providers register their AR/VR services. The AR/VR hardware providers register their AR/VR hardware devices as NFT tokens in the system(virtual representation of real hardware) where users can purchase. The software updates and supply-chain verification data of the AR/VR devices are encoded with SBOM/PBOM metadata. Anyone who purchases the AR/VR device can verify the software updates and cyber supply-chain provenance data via the SBOM/PBOM metadata files. All the communication rules of the AR/VR devices are encoded as Zero-Trust security policies. These policies are also stored in the decentralized blockchain ledger. AI/ML providers will build and train the AI/ML models using blockchain-enabled coordinator-less federated learning approach [11]. These ML models can be purchased and used by the AR/VR service providers can purchase. The data provenance information of the ML models, which records the model data changes from the origin, is recorded as Model Cards. The ML model ownership, storage locations of the ML models and data provenance information are encoded as Model Card objects and stored in the decentralized ledger. When buying a model, buyers can verify the model information, training process, quantitative evaluation of the model etc, via the Model Card objects published in the ledger. 

## _C. User Identity Registration_ 

The identities of the users will be captured via the mobile wallet application based on the Self-Sovereign Identity(SSI) approach. Mobile wallets capture users’ identity data such as personal information, images, contact details etc. The captured real identity data will be stored in the user’s physical mobile wallet. The Self-Sovereign Identity proof will be generated for the user and published in the blockchain ledger as an anonymous identity of the user. 

## _D. User Enrollment for Virtual Spaces_ 

Different AR/VR service providers can provide their own spaces. Users can enroll on these virtual spaces. If the virtual space is free to access, users can freely enroll for the space. If virtual space charges enrollment free, users need to pay that amount via crypto tokens before enrolling on the space. When enrolling on the space, the user’s identities will be verified. User identity verification will happen via the Self-Sovereign identity verification approach [17]. Users will use their AR/VR devices which they have purchased as NFT tokens to browse the virtual space functions. 

## IV. DATA PRIVACY AND SECURITY 

We are proposing four-dimensional security measures that can be adapted to address the previously mentioned privacy and security concerns of the Metaverse. We focus on 1) Managing user identities and personal data on a privacy-preserving approach, 2) Managing AR/VR devices and software supplychain data provenance, 3) Facilitating trusted communication of the AR/VR devices and 4) Enhancing transparency, and auditability of the ML models to prevent adversarial attacks [18]. This design methodology provides enhanced transparency and auditability to the Metaverse while providing an open platform to share and trade Metavese functions(e.g., enroll AR/VR services, and purchase AR/VR devices) in the marketplace. 

## _A. Manage Personal Data_ 

To manage users’ data(e.g., personal data, identities) we are proposing a Self-Sovereign Identity-enabled decentralized identity system for users. Self-sovereign identity enabled the sharing of the proofs of the user data between parties in the Metaverse, without revealing the actual data. Users will be given a Self-Sovereign Identity enabled mobile wallet/desktop wallet application. The identities will be captured from the mobile wallet. The proof of the identity will be uploaded to the blockchain-based decentralized ledger and real identity information will be stored in the wallet. 

## _B. AR/VR Devices Software Supply-Chain Verification_ 

We represent the information of AR/VR devices (device identity data and provenance data) in the form of NFT tokens. Software updates and supply-chain data verification for AR/VR devices are managed using an SBOM/PBOM-enabled cyber supply-chain provenance system [5]. The generation of SBOM/PBOM data within the system is executed through a custom-trained Llama2 LLM. This AI-driven PBOM and SBOM generation significantly bolsters security measures by proactively identifying and addressing vulnerabilities. This approach mitigates oversights and ensures the prompt mitigation of vulnerabilities. Upon the purchase of AR/VR device NFTs, software updates for these devices can be automatically verified through the SBOM/PBOM data within the system. This prevents malware invasions and data breaches within the AR/VR devices. Moreover, this system permits only trusted and verified AR/VR devices to operate within the network. Before these devices operate within the network, their identities will be onboarded to the Metaverse as NFT tokens. Only registered AR/VR devices will have communication privileges within the Metaverse system. This integrated system functions as a comprehensive defense mechanism, significantly enhancing system security and resilience against a variety of potential threats. 

## _C. Zero Trust Security Enforcement_ 

Further,r to facilitate trusted and secure communication of the AR/VR devices, a Zero-Trust security-enabled network policy engine is introduced to the Metaverse. All the communication rules of the AR/VR devices are encoded as Zero-Trust 

121 

security policies. These policies and stored in the decentralized blockchain ledger. The communication rules/policies can be registered when users buy AR/VR devices. We are providing a web-based application to define the communication rules/policies of the devices. When devices are communicating in the Metaverse network, the communication rules(e.g., allowed communication endpoints and restricted endpoints) are constantly checked and verified. Only verified and authorized devices are allowed to communicate in the network. 

## _D. Attack Mitigation of AI/ML Models_ 

The machine learning models are prone to Adversarial attacks. The most common reason is to cause a malfunction in a machine learning model. An adversarial attack might entail presenting a model with inaccurate or misrepresentative data as its training or introducing maliciously designed data to deceive an already trained model. We are proposing a methodology to address these concerns using Model Cards. The data provenance information of the ML models, which records the model data changes from the origin, is recorded as Model Cards. The ML model ownership, storage locations of the ML models and data provenance information are encoded as Model Card objects and stored in the decentralized ledger [11]. When buying a model, buyers can verify the model information, training process, quantitative evaluation of the model etc, via the Model Card objects published in the ledger. Further, we are proposing a privacy-preserving federated learning-based approach to building machine learning models for the AI/ML providers in the Metaverse. Different providers can collaborate on a blockchain-enabled coordinator-less federated learning system to build machine learning models. This supports a fully transparent and auditable AI/ML model-sharing process in Metaverse. 

## V. IMPLEMENTATION, TESTBED SETUP AND EVALUATION 

We are presently in the process of establishing a large-scale testbed deployment utilizing Ericsson’s new Radio, Open5GS 5G core, and an on-premises LLM, as illustrated in Figure 4. The blockchain and other services are deployed on a private Cloud. The AR/VR devices communicate with these services through the 5G network. Rahasak blockchain has been used as the ledger storage in the system [8], [9]. The smart contracts implemented on Rahasak blockchain’s Aplos smart actor system [19], [20]. The llama2-13 B model has been used and trained to generate the PBOM generation. The LLM fine-tuning/training process has been made through Qlora with 4-bit quantized pre-trained language model into Low Rank Adapters (LoRA) [15]. An in-depth evaluation of the platform’s performance across two major areas 1) blockchain evaluation, and 2) LLM evaluation. 

## _A. Evaluation of Blockchain_ 

In this evaluation, we focused on assessing the transaction scalability and search performance of the blockchain. We measured the throughput of both invoke and query transactions 

Fig. 4: Proposed large-scale testbed architecture with Ericsson new Radio, Open5GS 5G core and on-prem Llama-2 LLM in VMASC Virginia US. 

Fig. 5: Transaction scalability 

and observed linear scalability as the number of peers increased, with throughput increasing proportionally, as depicted in Figure 5. Additionally, the platform’s search performance demonstrated that searching 2 million records took approximately 4 milliseconds, as shown in Figure 6. 

## _B. Evaluation of LLM_ 

The Llama2 LLM has been custom-trained to specialize in generating PBOMs. In its training phase, the model is provided with an extensive dataset that includes pull-request details, test outcomes, vulnerability scan reports for software packages, various statuses, and the target PBOM format structured as a JSON schema. Consequently, the Llama2 LLM is adept at producing new PBOMs for software updates of AR/VR devices, making use of the provided input data, such as details from pull-requests, SBOM data, and various statuses. The blockchain smart contract instructs custom-trained Llama2 LLM to generate PBOMs based on its trained data and input data. This is achieved via custom prompts, which guide the custom-trained Llama2 LLM in understanding the specific requirements and context of each service. Figure 7 an example prompt which used to instruct the LLM. The prompts are designed to encapsulate the nuances of inputs(e.g pull request information, SBOM information, vulnerability statuses etc), thereby enabling the LLM to generate targeted PBOM. Upon receiving these prompts, the Llama2 LLM employs its extensive language understanding and generation capabilities to produce PBOM that aligns with the provided inputs. Fig- 

122 

**==> picture [203 x 16] intentionally omitted <==**

**----- Start of picture text -----**<br>
No of transactions<br>Fig. 6: Transaction search throughput of blockchain.<br>**----- End of picture text -----**<br>


Fig. 8: PBOM object generate by LLM. 

Fig. 7: PBOM generation prompt. 

ure 8 shows an example of PBOM object generation by the LLM. These PBOMs are created as structured JSON schemas, encompassing critical details such as software update(e.g pull request) information(creator, verifier, approver, timestamps), test results, identified vulnerabilities, and the status of the pull request. 

## VI. RELATED WORKS 

Various researchers propose different architectures and reviews of the metaverse. The key elements and architecture of these research initiatives are outlined in this section. The comparison summary of these works is presented in the table I. 

Blockchain for Metaverse [21] proposes the use of blockchain technology to address data security concerns in virtual environments. Fusing Blockchain and AI with Metaverse [22] reviews the current state-of-the-art research 

on various components of the Metaverse, digital currencies, AI applications, and blockchain-powered technologies. MetaChain [23] introduces a novel blockchain-based architecture for Metaverse applications. Blockchain-based Federated Learning for Industrial Metaverses [24] utilizes cross-chain technology to enhance privacy protection in the industrial metaverse. Contribution of Blockchain in the Development of Metaverse [ **?** ] focuses particularly on the privacy and security aspects. 

## VII. CONCLUSION AND FUTURE WORK 

As the Metaverse utilizes interactive technologies such as virtual reality, augmented reality, and machine learning, privacy advocates have raised concerns about its approach to privacy and security. A growing concern with the emergence of the Metaverse is its potential impact on data privacy and security. Given these privacy worries, users might be hesitant to fully embrace the Metaverse. In this research, we propose a novel decentralized Metaverse architecture, leveraging Generative-AI, Blockchain, NFT, and Federated Learning to address these privacy and security concerns. User personal data and resource provider identities are managed through a Self-Sovereign Identity-enabled privacy-preserving architecture. AR/VR devices within the Metaverse are represented as 

123 

TABLE I: Metaverse architecture reviews comparison 

|Platform||Centralized/<br>Distributed|Data Security<br>Level|Blockchain<br>Integration|AI<br>Integration|NFT<br>Support|5G/6G<br>Integration|Federated Learning<br>Integration|Generative-AI<br>Integration|
|---|---|---|---|---|---|---|---|---|---|
|Metaverse 5G/6G<br>(This paper)||Distributed|High|✓|✗|✓|✓|✓|✓|
|Bc-Metaverse [21]||Distributed|High|✓|✗|✗|✗|✗|✗|
|AI-Metaverse [22]||Centralized|Mid|✓|✓|✗|✗|✗|✗|
|MetaChain|[23]|Distributed|Mid|✓|N/A|✗|✗|✗|✗|
|Meta-FML [24]||Distributed|High|✓|✓|✗|✗|✓|✗|
|Metaverse-Contribution|[25]|Distributed|Mid|✓|N/A|✗|✗|✗|✗|



NFT tokens available for user purchase. The verification of software updates and supply-chain for these AR/VR devices is conducted using an SBOM/PBOM-enabled Llama2 LLMbased supply-chain provenance system, effectively preventing malware invasions and data breaches. Furthermore, we propose a mechanism for various AI/ML providers to train their machine learning models utilizing a privacy-preserving federated learning approach. Communication among AR/VR devices in the Metaverse is authorized through a Zero-Trust security-enabled rule engine. The proposed system testbed has been implemented within a 5G environment, utilizing Ericsson new Radio with Open5GS 5G core. The evaluation of the blockchain and LLM has yielded promising results for the platform. In our future work, we plan to integrate more AR/VR devices into the platform and conduct scalable performance evaluations. 

## ACKNOWLEDGEMENTS 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning (CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] H. Ning, H. Wang, Y. Lin, W. Wang, S. Dhelim, F. Farha, J. Ding, and M. Daneshmand, “A survey on metaverse: the state-of-the-art, technologies, applications, and challenges,” _arXiv preprint arXiv:2111.09673_ , 2021. 

- [2] N. Kshetri, “Web 3.0 and the metaverse shaping organizations’ brand and product strategies,” _IT Professional_ , vol. 24, no. 02, pp. 11–15, 2022. 

- [3] Q. Wang, R. Li, Q. Wang, and S. Chen, “Non-fungible token (nft): Overview, evaluation, opportunities and challenges,” _arXiv preprint arXiv:2105.07447_ , 2021. 

- [4] Y. Wang, Z. Su, N. Zhang, D. Liu, R. Xing, T. H. Luan, and X. Shen, “A survey on metaverse: Fundamentals, security, and privacy,” _arXiv preprint arXiv:2203.02662_ , 2022. 

- [5] E. Bandara, S. Shetty, A. Rahman, and R. Mukkamala, “Let’strace—blockchain, federated learning and tuf/in-toto enabled cyber supply chain provenance platform,” in _MILCOM 2021-2021 IEEE Military Communications Conference (MILCOM)_ . IEEE, pp. 470–476. 

- [6] H. Touvron, L. Martin, K. Stone, P. Albert, A. Almahairi, Y. Babaei, N. Bashlykov, S. Batra, P. Bhargava, S. Bhosale _et al._ , “Llama 2: Open foundation and fine-tuned chat models,” _arXiv preprint arXiv:2307.09288_ , 2023. 

- [7] S. Arora, B. Yang, S. Eyuboglu, A. Narayan, A. Hojel, I. Trummer, and C. R´e, “Language models enable simple systems for generating structured views of heterogeneous data lakes,” _arXiv preprint arXiv:2304.09433_ , 2023. 

   - [9] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

   - [10] pbom.dev, “Pbom - pipeline bill of materials,” https://pbom.dev/, 2024, accessed: 2024-04-21. 

   - [11] E. Bandara, S. Shetty, A. Rahman, R. Mukkamala, J. Zhao, and X. Liang, “Bassa-ml—a blockchain and model card integrated federated learning provenance platform,” in _2022 IEEE 19th Annual Consumer Communications & Networking Conference (CCNC)_ . IEEE, 2022, pp. 753–759. 

   - [12] V. Axelsson and F. Larsson, “Understanding the software bill of material for supply-chain management in open source projects,” 2023. 

   - [13] S. W. Rose, O. Borchert, S. Mitchell, and S. Connelly, “Zero trust architecture,” 2020. 

   - [14] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

   - [15] T. Dettmers, A. Pagnoni, A. Holtzman, and L. Zettlemoyer, “Qlora: Efficient finetuning of quantized llms,” _Advances in Neural Information Processing Systems_ , vol. 36, 2024. 

   - [16] T. Reason, E. Benbow, J. Langham, A. Gimblett, S. L. Klijn, and B. Malcolm, “Artificial intelligence to automate network meta-analyses: Four case studies to evaluate the potential application of large language models,” _PharmacoEconomics-Open_ , pp. 1–16, 2024. 

   - [17] E. Bandara, X. Liang, P. Foytik, S. Shetty, and K. De Zoysa, “A blockchain and self-sovereign identity empowered digital identity platform,” in _2021 International Conference on Computer Communications and Networks (ICCCN)_ . IEEE, 2021, pp. 1–7. 

   - [18] G. Kamhoua, E. Bandara, P. Foytik, P. Aggarwal, and S. Shetty, “Resilient and verifiable federated learning against byzantine colluding attacks,” in _2021 Third IEEE International Conference on Trust, Privacy and Security in Intelligent Systems and Applications (TPS-ISA)_ . IEEE, 2021, pp. 31–40. 

   - [19] E. Bandara, W. K. NG, K. De Zoysa, and N. Ranasinghe, “Aplos: Smart contracts made smart,” _BlockSys’2019_ , 2019. 

   - [20] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, K. De Zoysa, and W. K. Ng, “Saas-microservices-based scalable smart contract architecture.” 

   - [21] T. R. Gadekallu, T. Huynh-The, W. Wang, G. Yenduri, P. Ranaweera, Q.-V. Pham, D. B. da Costa, and M. Liyanage, “Blockchain for the metaverse: A review,” _arXiv preprint arXiv:2203.09738_ , 2022. 

   - [22] Q. Yang, Y. Zhao, H. Huang, Z. Xiong, J. Kang, and Z. Zheng, “Fusing blockchain and ai with metaverse: A survey,” _IEEE Open Journal of the Computer Society_ , vol. 3, pp. 122–136, 2022. 

   - [23] C. T. Nguyen, D. T. Hoang, D. N. Nguyen, and E. Dutkiewicz, “Metachain: A novel blockchain-based framework for metaverse applications,” _arXiv preprint arXiv:2201.00759_ , 2021. 

   - [24] J. Kang, D. Ye, J. Nie, J. Xiao, X. Deng, S. Wang, Z. Xiong, R. Yu, and D. Niyato, “Blockchain-based federated learning for industrial metaverses: Incentive scheme with optimal aoi,” _arXiv preprint arXiv:2206.07384_ , 2022. 

   - [25] S. Mishra, H. Arora, G. Parakh, and J. Khandelwal, “Contribution of blockchain in development of metaverse,” in _2022 7th International Conference on Communication and Electronics Systems (ICCES)_ . IEEE, 2022, pp. 845–850. 

- [8] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

124 

