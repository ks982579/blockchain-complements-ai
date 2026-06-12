# — WedaGPT Generative-AI (with Custom-Trained Meta’s Llama2 LLM), Blockchain, Self Sovereign Identity, NFT and Model Card Enabled Indigenous Medicine Platform 

Eranga Bandara _[∗]_ , Peter Foytik _[∗]_ , Sachin Shetty _[∗]_ , Ravi Mukkamala _[∗]_ , Abdul Rahman _[†]_ , Xueping Liang _[‡]_ , Ng Wee Keong _[§]_ , Kasun De Zoysa _[¶]_ , 

> _∗ {_ cmedawer, pfoytik, sshetty, mukka _}_ @odu.edu Old Dominion University, Norfolk, VA, USA 

> _†_ abdulrahman@deloitte.com Deloitte & Touche LLP 

> _‡_ xuliang@fiu.edu Florida International University, USA 

> _§_ AWKNG@ntu.edu.sg Nanyang Technological University, Singapore 

> _¶_ kasun@ucsc.cmb.ac.lk University of Colombo, Sri Lanka 

_**Abstract**_ **—Traditional and indigenous medicine, deeply rooted in ancient traditions and wisdom, plays a crucial role in global healthcare and cultural identity. These practices provide treatments for illnesses such as cancer and bone injuries, which often lack effective remedies in Western medicine. However, these valuable systems face challenges like potential knowledge loss, undervaluation of practitioners’ expertise, and the risk of fraud due to the absence of credential verification mechanisms. In this research, we introduce “WedaGPT,” a Generative AI-enabled platform that utilizes a custom-trained Meta’s Llama2 Large Language Model (LLM), Blockchain, self-sovereign identity (SSI), Non-Fungible Tokens (NFTs), and model cards to share traditional medical knowledge and address these issues. WedaGPT creates a collaborative ecosystem connecting doctors, medicine providers, therapists, patients, and technology experts, all committed to preserving and advancing traditional healing practices. This platform enables secure and transparent contributions from all stakeholders to patient well-being. Ancient medical recipe books are translated into English and digitized into PDF formats to enrich the platform’s knowledge base. These texts are used to fine-tune the Llama2 LLM, which has been quantized and optimized with Qlora for performance on consumer-grade hardware. Through a chat-based interface in the SSI-enabled mobile wallet, users can interact with the LLM and access detailed information on treatments, recipes, prescriptions, and healing methods. Additionally, users can consult remotely with doctors who prescribe treatments through this wallet. A key feature of WedaGPT is transforming ancient medicinal recipes into NFT tokens for sale on NFT marketplaces, giving traditional knowledge digital authenticity and economic value. Revenue from these sales is distributed among platform contributors, promoting equitable ownership and recognition. Medical recipe data, including treatment histories and physician details, are encapsulated in Model Cards and securely** 

**stored on the blockchain. This system offers mechanisms to verify doctors and treatments in a privacy-preserving way, potentially reducing fraud and medication errors.** 

_**Index Terms**_ **—GPT, LLM, Llama2, Blockchain, NFT, Model Card** 

## I. INTRODUCTION 

Traditional and indigenous medicine, which includes a diverse array of healing practices developed across various cultures globally, is deeply rooted in ancient traditions and local wisdom. Playing a crucial role in global healthcare, these systems provide remedies for a variety of ailments and are integral to cultural identity and community health [1]. There are a variety of traditional medical systems all over the world which could effectively treat for several illnesses, such as cancers, coma, bone injuries etc., which have no effective drugs in the western medicine [2]. Despite their value, indigenous medicine faces challenges, including the obscurity of many practices, often passed down orally or confined to local knowledge. Precious texts, like the “puskola book”(palm tree manuscript), are at risk of being lost without digital preservation [3]. This knowledge, usually transferred from generation to generation, faces the danger of fading away. Additionally, the expertise of indigenous medicine practitioners is often undervalued, with their work going unrecognized in formal healthcare systems. This leaves them marginalized and threatens the survival of these traditions. Furthermore, the lack of verification mechanisms for practitioners’ credentials and 

the authenticity of their practices poses a challenge, making it difficult to distinguish legitimate providers from fraudulent ones [4]. 

In response to significant challenges, we proposed “WedaGPT” which leverages Generative AI, including Meta’s custom-trained Llama2 LLM [5], [6], Blockchain [7]–[9], SSI, NFTs [10], and Model Cards [11], aiming to revitalize traditional medicine and support its practitioners. This platform creates a collaborative ecosystem for doctors, medicine providers, therapists, and technology experts, all working towards preserving traditional healing practices through a blockchain network. By digitizing ancient medical texts (e.g., translating and digitizing ancient medical recipe books into accessible PDF formats) and training a custom LLM, WedaGPT facilitates access to traditional knowledge through a secure chatbot interface. Essentially, the Llama2 LLM-based chatbot addresses queries related to recipes, treatments, and healing methodologies, enriching the user experience. This chat interface is embedded into SSIenabled mobile wallet applications [12]. Furthermore, users can directly communicate with remotely located doctors who may prescribe treatments through the system’s mobile wallet. Additionally, WedaGPT allows for the digital representation of medicinal recipes as NFTs, thereby adding economic value to traditional wisdom [10]. Revenue generated from NFT sales is shared among contributors, promoting equitable ownership. We also ensure data integrity and provenance through Model Cards stored on the blockchain, allowing easy verification of medical information. The major contributions of this research can be summarized as follows: 

- 1) Proposed an AI-enabled platform that facilitates the sharing of traditional medicine knowledge. 

- 2) Constructed an AI-powered knowledge base derived from ancient medicine books through a custom-trained Llama2 LLM. 

- 3) Proposed a model to represent ancient medicine recipes as NFT tokens, enabling their sale through NFT marketplaces. 

- 4) Improved the transparency and credibility of traditional medicine by encoding data provenance information of medical recipes and treatments into Model Card objects. 

## II. SYSTEM ARCHITECTURE 

Figure 1 describes the architecture of the platform. The proposed platform is composed of six layers: 1) Resource provider layer, 2) Blockchain/Smart Contract Layer, 3) LLM Layer, 5) Data Provenance Layer, 5) NFT Layer, and 6) Customer layer. 

## _A. Resource Provider Layer_ 

The Resource Provider Layer facilitates collaboration among various stakeholders, including doctors, medicine providers, manufacturers, transport agents, and translators. These participants contribute their unique expertise, enriching the ecosystem. Doctors provide traditional medical knowledge and pre- 

Fig. 1: WedaGPT platform layered architecture. 

scriptions, derived from ancient texts. Medicine providers contribute medicinal recipes, and manufacturers produce specialized medicinal products. A critical function is performed by translators, who convert ancient medical texts into English and digital formats, forming the basis for training the Llama2 LLM [6]. Indigenous medicine recipes are transformed into NFT tokens for sale on NFT marketplaces, with purchases made using fiat currency. Transport providers then deliver the medicines to customers, ensuring a smooth transaction. The platform, accessible via web and mobile systems, caters to the needs of all users, offering SSI-enabled mobile wallets for secure interactions [13]. The onboarding of all participants, including identity verification, is managed through these applications. While users’ actual identities are protected in their wallets, their verified identities are recorded on the blockchain as SSI-Proofs, ensuring security and privacy [12]. 

## _B. Blockchain and Smart Contract Layer_ 

This layer employs blockchain technology and smart contracts, providing a scalable infrastructure capable of supporting multiple organizations, each with its own blockchain node, as shown in Figure 1. It securely stores essential data, including digital identity proofs (SSI proofs) for doctors, manufacturers, customers, and translators, as well as data provenance information for medical recipes and prescriptions. This data covers patient details, treatment history, and recipe ownership, ensuring data integrity through Model Card objects stored on the blockchain [11]. NFT tokens linked to these Model Cards allow NFT owners to verify the data provenance of recipes, adhering to a customized token schema. The infrastructure is underpinned by four main smart contracts: Identity, Recipe, Model Card, and NFT contracts. The Identity contract manages peer identities, including registration and permissions. The 

Recipe contract handles operations related to medical recipes, and the Model Card contract manages data provenance. Recipe purchasers can access this information through their mobile wallets for independent verification. Finally, the NFT contract facilitates the creation and exchange of NFT tokens, which follow a unique schema reflecting their distinct attributes. 

## _C. LLM Layer_ 

At the heart of the WedaGPT platform lies the LLM Layer, where state-of-the-art artificial intelligence meets the wealth of traditional medical knowledge. This layer harnesses the power of Meta’s Llama2 LLM [14], to craft a custom-trained LLM enriched with medical books and recipes. To fine-tune the Llama2 LLM, we’ve undertaken a meticulous training process, collaborating with Qlora with 4-bit quantized pretrained language model into Low Rank Adapters (LoRA) [15]. The fine-tuned Llama2 LLM run on Ollam [16], Figure 2. The interaction with Meta’s Llama2 LLM is facilitated by Recipe smart contract through Ollam’s LLM API, Llamaindex and LangChain, a robust interface that enables streamlined and effective communication with the language model, Figure 3. Once the specialized LLM is trained, users gain access to a powerful tool for inquiries, suggestions, and information retrieval related to traditional medicine [6]. This user-friendly interface with the LLM opens up a world of insights and guidance, seamlessly bridging the gap between ancient wisdom and modern needs. Through this layer, the WedaGPT platform empowers users to explore, learn, and benefit from traditional medical knowledge with the aid of cutting-edge AI technologies. 

## _D. Data Provenance Layer_ 

The Data Provenance layer serves as the guardian of data integrity and traceability, recording and safeguarding every action and event that unfolds within the system. This layer achieves this critical task through the utilization of Model Cards, comprehensive records that house all pertinent information pertaining to medical recipes [11]. These Model Cards serve as invaluable repositories, capturing essential details such as patient status, prescription dates, and other critical information. By comprehensively documenting the prescription process, the system can provide a granular and transparent view of each medical interaction. This transparency is essential 

Fig. 2: Llama2 LLM fine-tuning. 

Fig. 3: LLM answer generation flow. 

in cultivating and maintaining trust within the system, ensuring that users can rely on the accuracy and authenticity of the information and transactions they encounter. The Model Card smart contract, residing within the blockchain ledger, plays a pivotal role in managing the lifecycle of these Model Cards. It handles functions related to model card generation, storage, and retrieval, ensuring that all necessary information is captured, structured, and properly formatted according to established standards. By leveraging the power of Model Cards and the secure foundation of the blockchain ledger, the system establishes a high degree of transparency and accountability. 

## _E. NFT layer_ 

The NFT layer plays a pivotal role in representing and safeguarding the details of medical recipes, providing a comprehensive record that encapsulates essential data, including ingredients and associated doctors. This layer further establishes a crucial link to the data provenance information, seamlessly encoded within Model Cards. The custom-trained Llama2 LLM information also included in the NFT token. A standout feature of this layer is the introduction of a custom NFT schema known as “w-528,” customized to represent recipe data as NFT objects. This schema efficiently encapsulates the intricacies of each recipe, ensuring that all relevant information is securely stored and verifiable in an immutable and privacy-preserving manner. Within the platform, these recipe NFTs are published on NFT marketplaces, where customers can acquire them by exchanging fiat currency. This innovative approach not only bestows digital and secure flavor upon traditional knowledge but also facilitates its accessibility to a global audience. Revenue generated through NFT sales is thoughtfully and equitably shared among the platform’s contributors, fostering a sense of collaborative ownership and recognition. This ensures that all participants, from recipe owners to translators and therapists, are duly acknowledged and rewarded for their invaluable 

contributions. The NFT Layer also encompasses the essential functions of NFT token generation and trading, handled by the NFT contract [10]. 

## _F. Customer Layer_ 

The Customer Layer caters to patients and service consumers, facilitating their interaction with the platform via SSI-enabled mobile wallets, which safeguard their personal information [12]. This layer plays a vital role in the ecosystem, allowing users to purchase medical recipes as NFTs from the marketplace with fiat currency. This method grants users access to traditional medical knowledge, merging ancient wisdom with modern healthcare practices. Upon purchasing a recipe, users can utilize the custom-trained Llama2 LLM for inquiries and medical advice related to their acquired recipes. Additionally, users can audit and verify the data provenance of their purchases through model card objects linked to the NFT tokens [10]. This feature increases transparency and builds trust by allowing users to independently confirm the authenticity and completeness of the medical information they utilize. 

## III. PLATFORM FUNCTIONALITY 

There are five main functionalities of the platform: 1) Peer Identity registration, 2) Recipe generation (translation of medical books, LLM tranning), 3) NFT token issuance and trading, 4) data privacy and security handling, and 5) Data provenance handling. This section describes these functions. 

## _A. Peer Identity Registration_ 

The WedaGPT platform begins by enabling the registration of peer identities, essential for establishing trust and security within the ecosystem. This involves the onboarding of doctors, medicine providers, translators, and patients through SSI-enabled mobile wallets, prioritizing privacy [12]. During registration, personal identity information is captured, and SSI proofs, validating identity authenticity, are generated and securely stored on the blockchain [7]. Users’ actual identities are protected in their individual wallets, maintaining a balance between transparency and privacy. This SSI framework allows for the verification of identities by any participant, enhancing trust and credibility across the platform. The process is facilitated by the Identity smart contract, ensuring secure and verifiable onboarding. 

## _B. LLM Fine-Tuning_ 

The essence of the WedaGPT platform is in its recipe generation process, which integrates the medical knowledge of doctors, the linguistic skills of translators, and the capabilities of Generative-AI [17]. Initially, doctors with deep-rooted traditional medical knowledge begin by developing medicinal recipes from ancient texts/books, ensuring the preservation of cultural heritage. To broaden access to this knowledge, translators then convert these texts into English, making it 

understandable worldwide. Following translation, a customtrained Llama2 LLM is developed using this data and Qlora with 4-bit quantized pretrained language model into Low Rank Adapters (LoRA) [15]. This LLM becomes a critical component of the platform, capable of handling queries, offering advice, and elucidating the details of medicinal recipes. The Recipe Smart Contract orchestrates this process, overseeing the recipe generation from traditional knowledge to digital format, thus ensuring authenticity and cultural integrity are maintained. 

## _C. NFT Token Generation and Trading_ 

The WedaGPT platform innovatively generates and trades NFT tokens, transforming traditional medical recipes into digital assets. These tokens, structured according to the “w528” schema, are repositories of information including recipe details, contributors’ names, and links to Model Cards and the LLM. Once generated, the tokens are listed on NFT marketplaces, allowing global access to traditional medicinal knowledge. Purchasers can buy these NFTs using fiat currency, facilitating the exchange of digital assets. The revenue from sales is fairly distributed among all contributors, rewarding their efforts. Additionally, the platform coordinates medicine delivery through transport services, ensuring a comprehensive user experience [10]. The NFT contract oversees the creation and trading of tokens, ensuring security and efficiency. This process not only digitalizes traditional medicine but also maintains its cultural value, promoting access and fair compensation in the digital age. 

## _D. Data Provenance and Privacy Handling_ 

One of the critical functions of the WedaGPT platform is the handling of data provenance and privacy, ensuring transparency while upholding the confidentiality of sensitive information. To achieve this, the platform employs a comprehensive approach, encoding all data provenance information into specialized Model Cards [11]. These Model Cards serve as structured repositories, encapsulating vital data such as prescribed recipes, associated patients, timestamps, and the health statuses of patients following the administration of the recipe medicines. This detailed information provides a comprehensive picture of each recipe’s journey within the system. Crucially, the WedaGPT platform affords to users who purchase recipes as NFT tokens the privilege to access and scrutinize this data provenance. This transparency provides users with a clear and transparent view of the recipe’s history and its effects, fostering trust and confidence in the platform. Moreover, the entire system operates through SSI-enabled mobile wallets. This adds an extra layer of security and data privacy, ensuring that personal and private information about patients is safeguarded without compromise. 

## IV. IMPLEMENTATION AND EVALUATION 

As a proof of concept, WedaGPT was developed in collaboration with traditional medicine practitioners from “Dibulagala Ceylon” [2]. We custom-trained the Llama2-13B LLM 

Fig. 4: Transaction scalability 

Fig. 6: Transaction search throughput of blockchain. 

Fig. 5: Block creation time and # of blockchain nodes 

using ancient medicine recipe books, specifically palm tree manuscripts, which include treatments for conditions like blood sugar and gastritis. The content of these books was expertly translated into English, providing the foundation for training the Llama2-13B LLM on the WedaGPT platform. The finetuning process utilized Qlora’s 4-bit quantized pre-trained model with Low Rank Adapters (LoRA) [15], with queries managed by Ollama APIs, LlamaIndex, and LangChain. The platform’s performance was thoroughly evaluated in two major areas: blockchain and LLM evaluations. 

In blockchain evaluation, we focused on assessing the transaction scalability, search time and block generation time of the blockchain system. We measured the throughput of both invoke and query transactions and observed linear scalability as the number of peers increased, with throughput increasing proportionally, as plotted in Figure 4. Block generation time, measured as we varied the number of peers up to 7, is presented in Figure 5. Additionally, the WedaGPT platform’s search performance, utilizing the Rahasak blockchain’s Lucene Indexbased search API for transaction information, showed that searching 2 million records took approximately 4 milliseconds, as demonstrated in Figure 6. In the LLM evaluation we assessed the responses from the LLM for queries on treatments documented in the palm manuscripts. An example response from the Llama2 LLM regarding gastritis treatment, showcasing the platform’s ability to harness ancient knowledge, is illustrated in Figure 7. 

## V. RELATED WORK 

Several researchers have explored the integration of blockchain, NFTs, SSI, and AI technologies in the design of 

Fig. 7: WedaGPTs LLM response about a treatment 

medical systems. Table I presents a comparative analysis of these projects alongside the proposed platform, emphasizing their unique features and capabilities. 

Gebreab et al. [10] introduce a system for tracing medical devices and managing ownership using blockchain and NFTs, aiming to combat supply-chain attacks on medical IoT devices. Sai et al. [18] present a secure federated learning framework for intelligent health diagnosis, featuring a blockchain-based incentive mechanism and an NFT-based marketplace. Additionally, Turki et al. [19] present a decentralized, blockchaindriven drug traceability solution that seamlessly integrates IoT devices. Zhuang et al. [13] introduces a blockchain architecture comprising four modules facilitating patient tokenization, authentication, and health information exchange. Mohammed et al. [20] present an innovative electronic medical records system utilizing NFTs on a customized permission blockchain, integrating secret sharing technology for key management efficiency and enhanced accessibility. 

## VI. CONCLUSIONS AND FUTURE WORK 

In conclusion, our research introduces “WedaGPT,” a platform that merges Generative-AI(with custom-trained Llama2 LLM), blockchain, self-sovereign identity, NFTs, and model 

TABLE I: Blockchain, NFTs, SSI, and AI enabled medical platform comparison 

|Platform|Centralized/<br>Distributed|Blockchain<br>Support|Running<br>Blockchain|Domain|Data Provenance<br>Support|NFT<br>Support|SSI<br>Support|AI<br>Integration|
|---|---|---|---|---|---|---|---|---|
|WedaGPT|Distributed|✓|Rahasak|Indigenous Medicine|✓|✓|✓|✓|
|[10]|Distributed|✓|HLF|Medical Device|✗|✓|✗|✗|
|[18]|Distributed|✓|N/A|Medical Data|✓|✓|✗|✓|
|[19]|Distributed|✓|Ethereum|Drug Traceability|✗|✓|✗|✗|
|[13]|Distributed|✓|Ethereum|Patient Data|✗|✓|✓|✗|
|[20]|Distributed|✓|N/A|Electronic Medical Record|✓|✓|✗|✗|



cards to preserve and disseminate traditional medical knowledge. This initiative creates a dynamic ecosystem, involving doctors, medicine providers, and technology experts, to enhance traditional healing practices while emphasizing security and transparency. A significant achievement of WedaGPT is utilizing ancient medical texts, translated and digitized, to train a specialized Llama2 LLM. This enables users to access extensive treatment information via a chat interface integrated into a self-sovereign identity-enabled mobile wallet. NFTs representing medicinal recipes introduce digital authenticity and economic value, with revenue shared among contributors, fostering equitable ownership. Model cards and blockchain enhance data integrity and transparency, allowing for secure verification of medical data. This approach also mitigates fraud by authenticating medical practices. In our future works, we intend to integrate a variety of open-source LLMs to enhance the capabilities of the WedaGPT platform. 

## ACKNOWLEDGEMENTS 

This work was supported in part by the DoD Center of Excellence in AI and Machine Learning(CoE-AIML) under Contract Number W911NF-20-2-0277 with the U.S. Army Research Laboratory. 

## REFERENCES 

- [1] A. A. Kurien, J. P. Ks, P. D. Walker, and T. N. Caza, “Traditional indigenous medicines are an etiologic consideration for nell1-positive membranous nephropathy,” _Kidney International_ , vol. 102, no. 6, pp. 1424–1426, 2022. 

- [2] H. De Silva, P. K. Perera, S. Jayasinghe, and S. De Silva Weliange, “Efficacy and safety of sri lankan traditional medicine regimen for knee osteoarthritis: study protocol for an open-label, active comparator, randomized controlled trial,” _Trials_ , vol. 23, no. 1, p. 955, 2022. 

- [3] A. A. Abdullahi, “Trends and challenges of traditional medicine in africa,” _African journal of traditional, complementary and alternative medicines_ , vol. 8, no. 5S, 2011. 

- [4] N. Redvers, J. Marianayagam, and B. Blondin, “Improving access to indigenous medicine for patients in hospital-based settings: a challenge for health systems in northern canada,” _International journal of circumpolar health_ , vol. 78, no. 1, p. 1577093, 2019. 

- [5] I. L. Alberts, L. Mercolli, T. Pyka, G. Prenosil, K. Shi, A. Rominger, and A. Afshar-Oromieh, “Large language models (llm) and chatgpt: what will the impact on nuclear medicine be?” _European journal of nuclear medicine and molecular imaging_ , pp. 1–4, 2023. 

   - [7] E. Bandara, X. Liang, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Rahasak-scalable blockchain architecture for enterprise applications,” _Journal of Systems Architecture_ , p. 102061, 2021. 

   - [8] E. Bandara, D. Tosh, P. Foytik, S. Shetty, N. Ranasinghe, and K. De Zoysa, “Tikiri-towards a lightweight blockchain for iot,” _Future Generation Computer Systems_ , 2021. 

   - [9] E. Bandara, W. K. Ng, K. D. Zoysa, N. Fernando, S. Tharaka, P. Maurakirinathan, and N. Jayasuriya, “Mystiko - blockchain meets big data,” in _IEEE International Conference on Big Data, Big Data 2018, Seattle, WA, USA, December 10-13, 2018_ , 2018, pp. 3024–3032. 

   - [10] S. A. Gebreab, H. R. Hasan, K. Salah, and R. Jayaraman, “Nft-based traceability and ownership management of medical devices,” _IEEE Access_ , vol. 10, pp. 126 394–126 411, 2022. 

   - [11] A. Wadhwani and P. Jain, “Machine learning model cards transparency review: Using model card toolkit,” in _2020 IEEE Pune Section International Conference (PuneCon)_ . IEEE, 2020, pp. 133–137. 

   - [12] E. Bandara, X. Liang, P. Foytik, S. Shetty, and K. De Zoysa, “A blockchain and self-sovereign identity empowered digital identity platform,” in _2021 International Conference on Computer Communications and Networks (ICCCN)_ . IEEE, 2021, pp. 1–7. 

   - [13] Y. Zhuang, C.-R. Shyu, S. Hong, P. Li, and L. Zhang, “Self-sovereign identity empowered non-fungible patient tokenization for health information exchange using blockchain technology,” _Computers in Biology and Medicine_ , vol. 157, p. 106778, 2023. 

   - [14] H. Touvron, L. Martin, K. Stone, P. Albert, A. Almahairi, Y. Babaei, N. Bashlykov, S. Batra, P. Bhargava, S. Bhosale _et al._ , “Llama 2: Open foundation and fine-tuned chat models,” _arXiv preprint arXiv:2307.09288_ , 2023. 

   - [15] T. Dettmers, A. Pagnoni, A. Holtzman, and L. Zettlemoyer, “Qlora: Efficient finetuning of quantized llms,” _Advances in Neural Information Processing Systems_ , vol. 36, 2024. 

   - [16] T. Reason, E. Benbow, J. Langham, A. Gimblett, S. L. Klijn, and B. Malcolm, “Artificial intelligence to automate network meta-analyses: Four case studies to evaluate the potential application of large language models,” _PharmacoEconomics-Open_ , pp. 1–16, 2024. 

   - [17] S. Arora, B. Yang, S. Eyuboglu, A. Narayan, A. Hojel, I. Trummer, and C. R´e, “Language models enable simple systems for generating structured views of heterogeneous data lakes,” _arXiv preprint arXiv:2304.09433_ , 2023. 

   - [18] S. Sai, V. Hassija, V. Chamola, and M. Guizani, “Federated learning and nft-based privacy-preserving medical data sharing scheme for intelligent diagnosis in smart healthcare,” _IEEE Internet of Things Journal_ , 2023. 

   - [19] M. Turki, S. Cheikhrouhou, B. Dammak, M. Baklouti, R. Mars, and A. Dhahbi, “Nft-iot pharma chain: Iot drug traceability system based on blockchain and non fungible tokens (nfts),” _Journal of King Saud University-Computer and Information Sciences_ , vol. 35, no. 2, pp. 527– 543, 2023. 

   - [20] M. A. Mohammed and H. B. Abdul Wahab, “A novel approach for electronic medical records based on nft-emr.” _International Journal of Online & Biomedical Engineering_ , vol. 19, no. 5, 2023. 

- [6] T. Brown, B. Mann, N. Ryder, M. Subbiah, J. D. Kaplan, P. Dhariwal, A. Neelakantan, P. Shyam, G. Sastry, A. Askell _et al._ , “Language models are few-shot learners,” _Advances in neural information processing systems_ , vol. 33, pp. 1877–1901, 2020. 

