**==> picture [60 x 35] intentionally omitted <==**

## _Article_ 

## **Technological Convergence of Blockchain and Artificial Intelligence: A Review and Challenges** 

**Nakhoon Choi[1] and Heeyoul Kim[2,] *** 

- 1 Department of Computer Science, Kyonggi University, Suwon 16227, Republic of Korea; nakhoon.choi@kyonggi.ac.kr 

- 2 Division of Computer Science and Engineering, Kyonggi University, Suwon 16227, Republic of Korea ***** Correspondence: heeyoul.kim@kyonggi.ac.kr 

**Abstract:** Blockchain and artificial intelligence are two of the most prominent technologies in computer science today and have attracted considerable attention from various research communities. Recently, several initiatives have been launched to explore the combination of these two pioneering technologies. The main goal is to combine the data integrity, privacy, and decentralization properties of blockchain with the ability of artificial intelligence to process, analyze, predict, and refine massive data sets. The combination of blockchain and AI technologies is expected to address key challenges in the digital realm, such as data security, transparency, and streamlined decision-making. However, there is a problem that many studies have focused on the advancement of a single technology as the main perspective. To overcome these recent research limitations, we provide a broad view of the combination of blockchain and artificial intelligence and analyze the limitations of existing research and their causes. Furthermore, we identify challenges and attempts to be addressed through this analysis. The analysis in this paper is organized into a comprehensive section dedicated to the application of artificial intelligence in blockchain and vice versa. Based on our analysis, we identify existing challenges and propose a novel framework for researchers to overcome these limitations, thus expanding new research opportunities. 

**Keywords:** artificial intelligence; blockchain; technological convergence 

Academic Editors: Krzysztof Szczypiorski and Ping-Feng Pai 

Received: 20 November 2024 Revised: 20 December 2024 Accepted: 26 December 2024 Published: 27 December 2024 

**Citation:** Choi, N.; Kim, H. Technological Convergence of Blockchain and Artificial Intelligence: A Review and Challenges. _Electronics_ **2025** , _14_ , 84. https://doi.org/ 10.3390/electronics14010084 

**Copyright:** © 2024 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license (https://creativecommons.org/ licenses/by/4.0/). 

## **1. Introduction** 

Blockchain and artificial intelligence are considered to be the two most innovative and influential technologies today. While each technology is receiving considerable attention individually, their combination presents even greater possibilities and challenges. This paper provides an in-depth analysis of recent research and initiatives on the combination of blockchain and artificial intelligence and explains how this is driving innovation in various industries. This research presents a comprehensive review of current research and efforts exploring the combination of blockchain and artificial intelligence. This paper examines the challenges and solutions involved in combining blockchain technology and artificial intelligence and explores future directions for their convergence. Throughout this paper, the terms ‘combination’ and ‘convergence’ are used interchangeably to refer to the use of these technologies together to create new value and capabilities. Convergence involves the discovery of new technological horizons and attempts to achieve a higher level of combination through the convergence of technologies. 

Blockchain and AI are recognized as independent innovative technologies, but the synergies that occur when they are combined and utilized in a complementary manner are much greater. Looking at the technical possibilities that enable the combination of these 

_Electronics_ **2025** , _14_ , 84 

https://doi.org/10.3390/electronics14010084 

_Electronics_ **2025** , _14_ , 84 

2 of 35 

two technologies, blockchain not only makes data forgery and alteration more difficult in a decentralized network format rather than a centralized server but also contributes greatly to data transparency and integrity by enabling transparent transactions. This function of blockchain not only verifies the source and accuracy of the data used by AI but also enhances transparency by securely recording and storing decisions in an immutable manner. This greatly improves the user’s trust in the decisions and results of artificial intelligence. This approach is well suited to address the issues of model reliability and single point of failure, which are major technical drawbacks of AI cited in the literature [1]. In addition, the distributed network of blockchains is optimally structured to implement federated and distributed learning, making it well-suited to promote learner participation through incentive systems. This provides an effective solution to the problem of accessing confidential data for the training of AI models [2] and enables the development of AI models that can provide more accurate insights. 

In addition, artificial intelligence (AI) is a technological innovation that excels at solving complex problems through advanced analytical functions, including pattern recognition and predictive modeling. AI models identify patterns in vast datasets to support decision-making through in-depth analysis and predictive insights. As such, AI is increasingly being recognized as a key technology for identifying irregular transaction patterns and predicting and responding to security threats in large-scale environments. This is especially suitable for analyzing the vast amount of cryptocurrency transaction data in blockchain networks, identifying and mitigating fraud or abnormal behavior to improve network stability [3,4]. In addition, AI can analyze blockchain data to optimize network operations and propose innovative consensus algorithms to improve system efficiency and performance. These developments highlight the synergistic potential of combining blockchain and AI [5]. This convergence is essential for digital transformation and offers unique opportunities that go beyond the capabilities of each technology alone [6]. This integration brings significant research and business benefits, including robust open digital trust structures, improved personalized services, and a thriving decentralized digital economy. Companies that leverage this integration can improve customer relationships and operational efficiency. To achieve these results, a comprehensive understanding of blockchain and AI, the convergence of the two technologies, the added value of this convergence, and the necessary preparatory steps are required. 

The continuous development and change in technology require deep understanding and careful observation to predict future trends and gain technical insights. In particular, it is very important to monitor the development of advanced technologies such as artificial intelligence and blockchain. These technologies not only have considerable potential individually but also show synergy when combined. This can be a great help in predicting the impact on society and various industries. Therefore, thorough research and analysis of the technological combination of blockchain and artificial intelligence is essential. This white paper takes a close look at the current state of development and the future potential of artificial intelligence and blockchain. The goal is to provide profound insights to academia and industry by examining these technologies from various perspectives and emphasizing their importance. 

## _Purpose and Contribution_ 

Existing research has been limited to taking an approach that is limited to the individual development of blockchain and artificial intelligence (AI) or specific use cases. This study aims to go beyond these limitations and systematically analyzes the broad industrial and academic potential of the convergence of the two technologies. This will review the current status of the technological combination of blockchain and AI and identify the key challenges that need to be addressed for effective convergence. In 

_Electronics_ **2025** , _14_ , 84 

3 of 35 

particular, this study aims to provide a balanced view of how blockchain and AI can work in a complementary way, overcoming the limitations of previous research, which has mainly focused on identifying the superiority of a particular technology. In addition, we investigate the current status of various studies in the field of combining blockchain and artificial intelligence technologies, and based on this, we derive major technical challenges such as data reliability, scalability, personal information protection, and cross-platform compatibility through the convergence of the two technologies. 

This study also examines the currently proposed technical approaches and solutions and proposes future research directions based on them. This process presents potential strategies that can contribute to expanding new innovations and application possibilities, as well as how the combination of blockchain and AI can overcome the limitations of existing technologies. This paper focuses on the convergence of blockchain and AI, which goes beyond the simple combination of technologies to increase practical utility and applicability in various industries and applications. This paper systematically explains how the convergence of the two technologies indicatively reconstructs existing limitations and explores future-oriented development directions. 

Accordingly, this white paper is structured as follows: Section 2 introduces the basic concepts and background of blockchain and artificial intelligence technologies. Section 3 presents an overview of the entire study and explains the classification system used in this research paper. In Section 4, we look at the advantages of blockchain and AI convergence and highlight important technical combination factors. Section 5 discusses the various challenges currently facing the industry. In Section 6, the future research directions for convergence obtained through the analysis of this paper and new possibilities for the community are presented, and the justification and knowledge differentiation of this paper are discussed. Finally, Section 7 concludes the paper by summarizing the main findings. 

## **2. Background Technology** 

## _2.1. Overview of Blockchain Technology_ 

Blockchain is a type of distributed ledger technology (DLT) that has a significant impact on enhancing data transparency and security [7]. This section describes the basic concepts and principles of blockchain technology, its key features and advantages, and various use cases. As the name suggests, the basic structure of blockchain is a data structure consisting of ‘blocks’ and ‘chains’. Blocks store transaction data and are linked together in a chain. Each block contains the hash value of the previous block, a timestamp, and transaction information. This structure ensures that all transactions in the blockchain are linked in sequence. In a blockchain, the process of creating a block and connecting it to a chain is called ‘consensus’, and there are various consensus algorithms, such as proof-of-work, proof-of-stake, and proof-of-stake-based methods, depending on the structure and characteristics of the blockchain [8]. Through this consensus, the block created by the elected block creator is distributed to all participants in the network. Participants then verify the validity of this block. If the verification is successful, the block is added to the chain and this information is broadcast to all network participants. Some blockchains, such as Ethereum and EOS, support a programmable development environment through smart contracts [9]. Smart contracts are program code that runs on a blockchain network and automatically execute when conditions are met. Unlike traditional contracts, these smart contracts enable secure and reliable transactions without the involvement of a third party. The advantage of smart contracts is that the content of the contract is publicly recorded on the blockchain and cannot be changed. Smart contracts are only executed on the network after being validated by the pre-execution of a verifier (such as a miner), ensuring a safe and integrity-guaranteed execution environment. 

_Electronics_ **2025** , _14_ , 84 

4 of 35 

Blockchains can be classified as public or private depending on the participation requirements of network members. Public blockchains such as Bitcoin, Ethereum, and EOS are widely adopted and allow anyone to participate by adopting various consensus algorithms depending on the platform. To explain the consensus algorithm, let us take the Proof of Work (PoW) algorithm used by Bitcoin, the most representative blockchain platform, as an example. Figure 1a shows the transaction flow of the Bitcoin network. It takes about 10 minutes to create a block in Bitcoin. Therefore, for an attacker to manipulate a previous block, they must update the network after changing all the connections from the target block to the current block. To successfully attack a blockchain network, an attacker must control more than 51% of the network participants or forge all previous blocks and update the network before a new block is created. However, this is mathematically and physically impossible, so once the data are written to a block, they cannot be changed, ensuring the integrity of all records [10]. The data recording mechanism of this blockchain is verified through the network to ensure the integrity, reliability, and permanence of all data sorted in chronological order [11,12]. These characteristics of blockchain data include history management and log data that monitor the history of changes in the status of all items. In addition, blockchain cryptocurrency assets have traceable features through this log data. 

**Figure 1.** Structure and organization of the Blockchain: ( **a** ) Bitcoin and ( **b** ) Hyperledger Fabric. 

_Electronics_ **2025** , _14_ , 84 

5 of 35 

The ‘London Upgrade’ of Ethereum [13], which is the most widely used public blockchain platform that supports smart contracts, is an update that has brought about significant changes to the smart contract execution and network economic model, with the key being the improvement of the gas fee system through EIP-1559. Instead of the existing auction-based fee model, a base fee was introduced to increase fee predictability during network congestion, and a deflationary element was added to reduce the supply of Ethereum through a design that burns some fees. This has improved the reliability and efficiency of smart contract-based applications, providing a predictable fee structure in various fields such as DeFi and NFT, while strengthening the economic sustainability of the Ethereum network. However, this did not lower the gas cost itself, and there is also criticism due to the decrease in miner profits, so it remains an issue that will need to be continuously reviewed along with future scalability issues. 

Hyperledger Fabric is a permissioned blockchain platform that allows only participants pre-approved by network administrators to join and is designed to be flexible to meet a variety of business needs. It is part of the Linux Foundation-led Hyperledger project, which emphasizes scalability and data privacy for blockchain networks. Fabric has a unique architecture and consensus process that distinguishes it from other blockchain platforms, and the structure shown in Figure 1b illustrates these features. The operation process of Fabric consists of four main steps: transaction creation, transaction verification, block generation and consensus, and block distribution. In the first step, a client receives a request from a user, creates a transaction, and submits it to the network. The transaction is structured in JSON format and contains data related to the requested action. The client uses a certificate authenticated by a membership service provider (MSP) for a trust-based connection [14]. In the second step, the generated transaction is forwarded to the peer node. The peer verifies the logical validity of the transaction by executing a chaincode (smart contract). The chaincode defines the business logic of the network, and the peer executes it to validate that the transaction conforms to network policies. During the verification process, the peer simulates the state of the transaction and generates a temporary result, which is signed and forwarded for verification with other peers. This process enables parallel processing of transactions and contributes to faster network processing. In the third step, the validated transactions are collected and sorted by an Ordering Service Node (OSN). The OSN is responsible for bundling transactions into blocks and distributing those blocks to peer nodes. Fabric uses order consensus instead of the Proof of Work (PoW) method of public blockchains to minimize resource consumption while maintaining high processing speeds. During the block generation process, the OSN determines the order of transactions, which is a key step in maintaining network consistency. In the fourth step, the block generated by the OSN is propagated to all peer nodes in the network. Peers verify the validity of the block and add it to their blockchain. During this process, the peers verify that the transactions in the block are valid and do not conflict with the existing state. This process ensures the integrity of transactions and strengthens the data reliability of the network. Hyperledger Fabric provides a Membership Service Provider (MSP) to manage the identity and permissions of network participants. The MSP leverages a public key infrastructure (PKI) to issue certificates for participants and acts as a key element in maintaining the security of the network. Participants are also restricted in their activities on the network based on roles and permissions defined through the MSP. This structure achieves the high security and data privacy required in a private blockchain environment. The structural design of the fabric maximizes the performance and scalability of the network through the parallel verification of transactions, consensus based on ordering services, and a modular authentication scheme, as shown in Figure 1b. This positions Hyperledger 

_Electronics_ **2025** , _14_ , 84 

6 of 35 

Fabric as a private blockchain platform that delivers high performance and reliability for a variety of industries, including finance, supply chain, healthcare, and more. 

Hyperledger Besu [15] is another enterprise-grade blockchain project developed by the Hyperledger Foundation and is based on the Ethereum protocol. Besu offers Ethereum compatibility on both public and private networks and supports extensive integration with Ethereum’s smart contracts and tools. This contrasts with Fabric’s flexible, modular architecture that supports a variety of consensus mechanisms and development languages, and its focus on the data privacy and security requirements of enterprise users. This difference stems from Fabric’s emphasis on strict data separation between network participants and building custom private blockchains, while Besu emphasizes a network that supports interaction with the Ethereum ecosystem. 

Blockchain uses a thoroughly decentralized network model where every participant holds a copy of the data, rather than relying on a centralized server. This structure makes it very difficult to forge or tamper with data, enabling transparent transactions. These characteristics are the reason why blockchain is designated as a DLT. Some of the characteristics that stem from the above-mentioned consensus process and data sharing by all network entities can be summarized as follows: 

- Data integrity: Data integrity is ensured because data recorded on the blockchain cannot be altered without the consent of network participants through a consensus algorithm. This characteristic increases the reliability of data by preventing them from being tampered with. 

- Transparency: All transactions and data on a blockchain are exposed to network participants, and all participants have the same ledger, which increases data transparency. This increased transparency increases the clarity of transactions and helps to detect and prevent fraud and misconduct. 

- Security: Blockchains use cryptographic techniques such as hash algorithms and elliptic curve cryptography to protect data, ensuring strong security. This feature prevents data leaks and ensures data safety. It also enables effective privacy protection through transactions using identifiers such as wallet accounts. 

- Decentralized: Blockchain stores data on a distributed network rather than a centralized server, and is not managed by a single node or server, eliminating single points of failure. This architecture improves network resilience and ensures continuous service availability. 

Based on the features and benefits described above, blockchain is being used in a variety of sectors. In the financial sector, it is proposed to be a highly efficient solution due to its reliability, data integrity, and ability to be automated through smart contracts. It is also widely used to increase retail transparency, verify authenticity, store and share data, and prevent fraud. Blockchain is also being used in supply chain management, healthcare, real estate, and art transactions. 

## _2.2. Overview of Artificial Intelligence Technology_ 

Artificial intelligence (AI) is a cutting-edge technology that enables machines to mimic human-like thinking and learning. This broad area includes a variety of techniques, including machine learning, natural language processing, computer vision, and more. This section explains the basic concepts and principles of AI, its key features and benefits, and various use cases. AI enhances the capabilities of machines so that they can autonomously learn from data and make informed predictions or decisions [16]. It efficiently recognizes patterns within large datasets for prediction or decision-making. Machine learning, an important subfield of AI, uses data to give machines the ability to learn on their own. Machine learning algorithms utilize training data to train a model, which is then used to make predictions on new data. Deep learning, a subfield of machine learning, 

_Electronics_ **2025** , _14_ , 84 

7 of 35 

focuses on leveraging artificial neural networks to learn complex patterns. This approach facilitates advanced classification and analysis capabilities that go beyond simple graphical representations. Deep learning is widely used in a variety of fields, including image and speech recognition, natural language processing, and more. 

Figure 2 illustrates a taxonomy of artificial intelligence technologies, showing the different techniques that represent the subdivisions of machine learning: supervised learning, unsupervised learning, reinforcement learning, and deep learning based on artificial neural networks. The figure further explains the breakdown of deep learning. Artificial intelligence technologies span many subfields. In the context of this paper, the two most important ones are federated learning and anomaly detection. Federated learning, a subtype of machine learning [17], refers to a method that uses data distributed across multiple devices or servers to train a global model. Unlike traditional centralized learning, federated learning performs training autonomously on each device, with the results sent to a central server to update the model. This approach improves privacy and efficiency by keeping data local to each device, preventing outside access or breaches. Another important AI technology, anomaly detection [18], is used to examine data patterns to identify deviations from set norms. This technique has been widely applied in fields such as fraud detection, system security, and quality control, where it plays an important role in identifying irregular patterns that may indicate an underlying problem. Anomaly detection typically deals with much smaller amounts of anomalous data compared to normal data, so pinpointing these anomalies is a significant challenge. However, artificial intelligence is emerging as a powerful tool that can effectively address this issue. At the same time, the assurance and trustworthiness of data have become a pressing issue, and the importance of skillful data management and selection is increasingly emphasized [19]. 

**Figure 2.** Classification and inclusion relationship of artificial intelligence technology. 

Learning in artificial intelligence essentially involves instructing a computer to perform a specific task. This process, which typically utilizes specific algorithms, allows the computer to identify patterns within data and make predictions or decisions based on these insights. There are many forms of learning methods used, including supervised learning, unsupervised learning, and reinforcement learning, but the cornerstone of AI is still centered on learning from data. This emphasizes the essential role that robust and relevant data plays in learning and developing efficient and effective AI models. These data, which are the key to training AI models, can encompass a variety of formats, including text, images, sounds, numbers, and more. Training data instruct algorithms to recognize patterns and make informed predictions. The availability of rich and diverse training data is critical and significantly improves model 

_Electronics_ **2025** , _14_ , 84 

8 of 35 

performance by providing comprehensive training coverage. Both the quality and quantity of data have a significant impact on the accuracy of the model. High-quality training data help build more accurate and reliable models. In turn, a substantial pool of data enables models to achieve better generalization, leading to more accurate predictions for new, unseen data. This highlights the importance of skillfully managing and effectively selecting training data during the AI model training process. 

Artificial intelligence helps you make informed predictions and decisions for specific situations by learning from the right data or algorithms. Some of the key features and benefits of artificial intelligence include the following: 

- Automate: AI can be used to automate repetitive tasks to increase efficiency and relieve human workloads. This automation can also reduce errors due to human oversight, strengthening the overall accuracy and reliability of your work. 

- Predictive ability: AI excels at identifying patterns in vast datasets and predicting future events based on these patterns. This capability provides proactive insights into different scenarios, which can significantly aid in the decision-making process and predicting future trends. 

- Personalization: AI greatly enhances user experience by learning user behavior patterns to facilitate personalized service delivery. This personalization ensures more relevant and efficient interactions, which increases user satisfaction and engagement. 

- • Real-time processing: AI’s ability to process significant amounts of data in real-time is a significant advantage. This ability supports instant decision-making and real-time service delivery, improving responsiveness and efficiency in a variety of applications and scenarios. 

The unique features and benefits of AI are being leveraged in a variety of areas. For example, in the area of product recommendations, AI leverages a user’s purchase history and behavioral patterns to make personalized product suggestions. This personalized approach allows users to seamlessly discover products that match their preferences and needs, enhancing their shopping experience. In a broader context, AI makes real life easier by streamlining processes, reducing errors, and improving efficiency, in areas as diverse as healthcare, transportation, and facial and behavioral recognition. The application of AI in these areas enhances service delivery and enables more personalized and efficient user interactions. AI is also making notable progress in the professional sphere, as evidenced by the growing use of chatbots and speech recognition systems. Recent research and examples demonstrate AI’s ability to promote efficiency, accuracy, and convenience in everyday and professional situations, highlighting its broad and growing impact. 

## **3. Research Trends in Blockchain and AI Combination** 

## _3.1. Research Overview_ 

The combination of blockchain and artificial intelligence is positioned as a pivotal movement in modern technological innovation and is expected to bring about transformative changes in various fields and have a significant impact on the future social and economic landscape. Recognizing this significance, this study focuses on exploring and structuring the recently published research and practices of blockchain and AI integration. It seeks to identify the technical architecture of the combination of blockchain and AI, reveal the potential benefits and opportunities arising from the combination, identify ongoing research trends, and interpret the implications for future technological developments. These insights into the combination of technologies will guide and inform future research directions in the field of blockchain and AI integration. 

_Electronics_ **2025** , _14_ , 84 

9 of 35 

Previous review papers on the convergence of blockchain and artificial intelligence (AI) have largely focused on specific applications or limited examples of integration. This approach often limits blockchain to its function as a data repository, preventing a full exploration of the potential synergies that the convergence of these two technologies can provide. To overcome these limitations, this study provides a broader and more comprehensive analysis of the convergence of blockchain and AI, systematically examining the potential and current state of this integration across a variety of industries and applications. While existing research and analytical studies have tended to view the convergence of blockchain and AI from the perspective of one technology augmenting the other, our study separates the approaches of both technologies in order to examine their mutual development potential and identify the challenges that need to be overcome. In doing so, we identify deeper technical challenges to the combination of the technologies that previous research has not adequately addressed and address the current state of these challenges. We also suggest research directions to ensure that blockchain and AI can work more organically and complementarily. We propose a framework that extends the role of blockchains beyond that of a simple database to better support AI learning and computation within blockchain networks. 

This paper provides a comprehensive survey of recent literature focusing on the intersection of blockchain and artificial intelligence (AI). The survey categorizes research papers into two main scenarios: The use of blockchain to enhance AI and the application of AI to advance blockchain technology. Figure 3 provides a detailed scheme of the research categorization. 

**Figure 3.** A taxonomy for papers proposing technologies through the combination of blockchain and artificial intelligence. 

When AI is utilized in blockchain, a structured classification is used that divides it into three layers depending on the specific technical application: Layer 0, Layer 1, and Layer 2. Layer 0 involves the combination of AI technology with blockchain protocol layer operations (including transaction generation, block generation, consensus algorithms, transaction validation, network protocols, and data protocols). This combination plays a key role in optimizing and improving the efficiency and security of these fundamental blockchain operations. Layer 1 involves the data layer, which includes the actual data, transactions, and ledgers generated by the blockchain. At this layer, AI technology is strategically used for data anomaly detection, pattern detection, and trend detection, contributing to improving the reliability and integrity of blockchain data. Layer 2, or the application layer, applies AI to areas such as smart contracts and DApps to enhance functions such as DAO, contract integrity, and platform operations. 

When blockchain technology is utilized for AI development, it is categorized into three classes—Class 1, Class 2, and Class 3—based on the cross-sectional application of AI technology. Class 1 involves the use of blockchain to build and realize a federated learning 

_Electronics_ **2025** , _14_ , 84 

10 of 35 

platform. This category includes research focused on developing blockchain-based learning platforms and proposing blockchain protocols for distributed learning. Class 2 involves using blockchain to train AI models, keep records, record learning timelines, and improve model reliability. It aligns with the Class 1 distributed learning classification but also includes non-distributed learning scenarios such as synchronizing asynchronous learning over blockchain. Class 3 emphasizes the use of blockchain for the secure sharing of training and testing data, improving the data stability and reliability of AI learning platforms. Class 4 includes improving the operational efficiency of AI systems with blockchain, supporting ethical compliance orientation of AI frameworks, and AI governance and regulatory compliance. This classification spans a wide range of industries, including healthcare, transport, and supply, and provides a broad and multi-faceted view of current research trends and potential future directions for the combination application of blockchain and AI technologies. 

## _3.2. The Research Paper Collection Process_ 

This study set clear criteria and collected data to systematically analyze recent research trends and technological synergies in the convergence of blockchain and artificial intelligence (AI). To ensure the timeliness and accuracy of the research, we focused on academic papers published in the last five years (2019–2024), and used major academic databases such as IEEE Xplore, SpringerLink, ScienceDirect, and ACM Digital Library, with a focus on Google Scholar. In this process, search keywords such as ‘blockchain’, ‘artificial intelligence’, ‘technology combination’, ‘convergence’, ‘blockchain–AI Integration’, and ‘Blockchain Artificial Intelligence Convergence Applications’ were used to comprehensively expand the scope of the research. 

In the process of selecting papers, we prioritized papers with a citation count above a certain level to ensure the reliability of the research and excluded short conference papers of less than 10 pages to ensure the depth of analysis and logical completeness of the paper, and focused on research of 10 pages or more. The final selection of academic data was classified into cases where blockchain improves the performance of AI and cases where AI strengthens the security and operational efficiency of blockchain to analyze the bidirectional technological interaction. 

In addition, this study did not rely solely on academic papers but also collected and analyzed the latest trends and technology reports from companies and industries. To better understand the practical applications of blockchain and AI in the industry, we used white papers, technology reports, and industry analysis reports published by major global companies and technology leaders. For example, we reviewed blockchain–AI integration strategies and case reports from major technology companies such as IBM, Microsoft, and Google, and included reports from industry research organizations such as Gartner and McKinsey in our analysis. In addition, we collected and analyzed the latest trends and technology reports from companies and industries, rather than relying solely on academic papers. For example, we reviewed blockchain–AI integration strategies and case reports from major technology companies such as IBM, Microsoft, and Google, and included reports from industry research organizations such as Gartner and McKinsey in our analysis. We also collected and analyzed the latest trends and technology reports from companies and industries, rather than relying solely on academic papers. This allowed us to identify practical application cases, industrial adoption status, and technical challenges that are difficult for academia to address, and strengthen the practical implications of the research. 

_Electronics_ **2025** , _14_ , 84 

11 of 35 

## _3.3. Research Survey on Artificial Intelligence for Blockchain_ 

This section summarizes examples and research on the application of AI to blockchain. The papers surveyed are organized similarly to the layers of a blockchain. This includes the protocol layer (L0), which includes the underlying protocols and algorithms of the blockchain; the data layer (L1), which covers areas such as anomaly and breach detection through transactions, blocks, and ledger data generated within the blockchain and the application layer (L2), where AI is utilized in various blockchain-based platforms and systems. This section highlights the technical solutions that AI can provide for the advancement of blockchain, revealing new opportunities and different directions. It also identifies the potential for continued technological advancements and new commercialization for the industrialization of blockchain. This section explores the use of AI for technical improvements in blockchain and new paths for future blockchain research. It is expected to provide new cognitive perspectives for researchers studying blockchain and those utilizing AI. 

## 3.3.1. Layer 0—Blockchain Protocol 

This layer section explores research focused on innovation at the blockchain protocol level. We explore the latest research that aims to overcome the current structural limitations of blockchains and integrate artificial intelligence techniques into the system and protocol layers of blockchains to improve the performance and reliability of the overall system. The insights and technical capabilities of AI herald a new path for the evolution of blockchain protocols, highlighting the potential for broader applications of blockchain. 

First, Team Nebula AI [20], a research team dedicated to the improvement and innovation of blockchain protocols and platforms, proposes a next-generation blockchain network by innovating at the blockchain protocol level. The proposed platform converts GPU-based mining machines into AI computing services to mitigate the energy costs incurred by traditional proof-of-work systems. It presents a model for constructing a decentralized artificial intelligence computing blockchain (NBAI). Through this approach, the authors identify significant consumers of computing power used for mining and propose a model to build a complete value-added economic system. 

T. Wang et al. propose a novel approach to mining algorithms within blockchain protocols for optimizing mining algorithms [21]. The researchers endeavor to determine the optimal blockchain mining strategy through reinforcement learning. They design a novel multidimensional RL algorithm to solve the Markov Decision Process (MDP) problem and use it to dynamically identify the optimal mining strategy. As a result, the optimal (or near-optimal) strategy can be identified without detailed knowledge of the blockchain network model. Experimental results demonstrate that it can maintain good performance in blockchain networks that evolve over time. 

Alrubei et al. [22] present a blockchain platform designed to support AI-driven IoT applications. In the work presented, AI models act as nodes within the blockchain to improve the reliability of the blockchain protocol. The platform enhances the resilience of the AI engine, ensures data integrity, and ensures that the AI engine only accesses trusted and up-to-date data. The study used AI engines to authenticate data flows in the blockchain network and block nodes that introduce fake or harmful data into the system during the data upload protocol through the blockchain’s transaction blocks. 

Furthermore, Khan, Abdullah Ayub, et al. [23] propose a collaborative framework in their research on efficient management within a cross-chain environment. The framework leverages the synergies of blockchain, IoT, and artificial intelligence to digitize small and medium-sized enterprises. The research provides a solution for cross-chain platforms through an IoT-enabled permissionless network structure called B-SME. This solves 

_Electronics_ **2025** , _14_ , 84 

12 of 35 

the challenge of lightweight stakeholder authentication. It also improves the speed of ledger management and optimization when transferring information between cross-chains, reduces the system’s computational resource usage, and optimizes both network bandwidth and storage capacity. 

The research focused on integrating AI into blockchain’s protocol and system layers primarily targets protocol efficiency and improvement. In particular, AI plays an important role in honing blockchain mining protocols and node discovery protocols. However, modern research trends are moving beyond simple optimization and making blockchain an integral aspect of training AI models. For example, initiatives like Nebula and blockchain technology for federated learning are pioneering a new paradigm by envisioning blockchain as the foundation for AI learning. Advances in this area of research suggest that the convergence of blockchain and artificial intelligence will be a key theme in future technological advancements. 

## 3.3.2. Layer 1—Data 

This section highlights various research efforts that use AI-based techniques to manage, refine, process, and analyze the vast amounts of data, such as transactions and blocks, generated by blockchains. It covers how to analyze large-scale blockchain data using various methods, such as deep learning, machine learning, and graph-based analysis, to improve the ecosystem of blockchain platforms and provide solutions to various problems such as fraud, unfair contracts, and contract flaws that may occur within blockchains. 

Among the research focused on anomaly detection to identify fraud and scams, Z. Zheng [24] proposes solutions to the problems of maintaining operations, the quality assurance of smart contracts, and detecting malicious behavior in blockchain data that have emerged as challenges for existing blockchains. The authors collect raw blockchain data such as blocks, receipts, and traces from the Ethereum network to closely analyze transaction data. Through data analysis, they detected Ponzi schemes using artificial intelligence and proposed a prediction model. Also, to detect anomalies in blockchain platforms, T. Chen et al. [25] presented a cross-graph analysis of massive transaction data from the Ethereum blockchain. Their work characterizes and analyzes the main activities of blockchain platforms, and proposes new solutions to security issues. W. Chen et al. [26] set out to identify patterns of market manipulation using the singular value decomposition (SVD) method to analyze transaction history and transactions on Bitcoin exchanges. Deepa, M. [27] proposes a technique using unsupervised learning for cost-effective anomaly detection in the blockchain. To achieve this, they classify blockchain network nodes into clusters and use erasure coding techniques to help reconstruct missing transactions. They then sort transaction details by address and collect time series data attributes through rolling window aggregation. Anomaly detection is performed using K-means clustering, and simulation results validate that the proposed method is cost-effective. Finally, Shayegan, Mohammad Javad, et al. [28] use anomaly detection techniques to pinpoint irregular and fraudulent behavior in blockchain networks. Instead of detecting anomalies in individual addresses or wallets, the study observes unusual user behavior through a collective anomaly methodology and constructs clustering through a trimmed_K-means algorithm. This proposed method not only minimizes the computational overhead required for feature extraction, but also enables the identification of a larger number of anomalous users and anomalous behaviors. 

With a focus on platform security and privacy preservation, Deebak [29] introduces a framework designed to preserve privacy within smart contracts. The framework presents an approach to analyzing data transactions and sharing by leveraging the Extreme Gradient Boosting (XGBoost) algorithm. It proposes a way to optimize network load, which represents the fluctuations in transmission rates of untrusted network connections, 

_Electronics_ **2025** , _14_ , 84 

13 of 35 

and to streamline and secure human interactions, system activity, service notifications, security risks, fraudulent billing, and more in smart contracts. Furthermore, in the area of breach detection and attack recognition, Liu, Tong, et al. [30] provide a technique to detect and classify DDoS attacks that may affect blockchain-based smart transportation systems through deep learning in AI. AI identifies and classifies DDoS attacks, and uses a hybrid deep learning technique that merges autoencoders (AEs) and multilayer perceptrons (MLPs) to enhance and improve the data and transaction layers of the blockchain. This approach focuses on enhancing security at the data and transaction levels of the blockchain to facilitate automated feature extraction and rapid classification. In another study on intrusion detection [31], Mansour, Romany F, propose a novel poor and rich optimization through the PRO-DLBIDCPS approach, a deep learning model tailored for blockchain-based intrusion detection in cyber-physical systems (CPS) environments. They deploy ABi-GRNN to preserve bidirectional contextual data for feature sequences by using bidirectional RNNs to retain both preceding and subsequent data. They also presented an effective intrusion detection model in CPS environments by utilizing an attention mechanism to describe the correlation between data and output. 

The data and transaction layers of blockchains inherently present many challenges because they manage vast amounts of information. In recent years, efforts have been made to harness the vast amount of data generated by blockchains using techniques such as anomaly detection and breach detection to make the platform more reliable and secure. AI’s superior data processing capabilities can be an important tool in overcoming these challenges. In particular, AI can play an important role in overcoming blockchain’s data processing constraints to improve the efficiency and reliability of the platform. These research advances highlight the potential synergies between blockchain and AI in addressing future security and optimization challenges. 

## 3.3.3. Layer 2—Application 

This section presents examples where artificial intelligence has been integrated into blockchain-based platforms, systems, and application layers. Here, AI has been utilized to enhance a variety of blockchain-based applications or to overcome challenges unique to blockchain applications. Proposals for blockchain-based applications span a variety of domains, including healthcare, supply chain, and industry. In these areas, AI complements existing or proposed applications or opens up new possibilities and benefits through technology convergence. 

The healthcare sector is a prime example of where blockchain’s advantages can shine, especially given the sensitivity of medical and diagnostic data. Ray, Rajarshi Raj, and others [32] propose a personalized healthcare platform for online disease treatment in the critical area of healthcare. Their proposed platform provides NLP-based personalized healthcare along with data analytics. It integrates chatbots to automate certain processes to provide personalized analytics to users, healthcare professionals, and healthcare institutions. Crucially, the platform ensures the integrity and reliability of medical and treatment data through blockchain. Griewing, Sebastian et al. [33] also demonstrate a digital workflow for breast cancer treatment that harnesses the power of artificial intelligence and blockchain technology. The study describes the ability to monitor the environment, collect and analyze data, utilize AI-driven engines for processing and prediction, and provide actionable insights. All of these outcomes are available on a public blockchain platform. The study hypothesizes that blockchain-enhanced digital workflows have the potential to not only solve the challenges of breast cancer treatment but also enhance the privacy and sovereignty of patient data. 

_Electronics_ **2025** , _14_ , 84 

14 of 35 

Raja, Gunasekaran, et al. [34] study the use of artificial intelligence for platform-based blockchain applications, proposing a secure, decentralized, multilateral computing protocol tailored for the Internet of Vehicles (IoV) environment. Accounting for the advancement of autonomous driving technology, this protocol provides an innovative solution for vehicular networks. The protocol brings security to IoV devices by introducing blockchain for authentication and security, which is essential for data transmission. It also provides an AI-powered blockchain that can autonomously write smart contracts to speed up transaction verification and improve energy efficiency. In another study focused on socio-economic supply chains, Xiao, Wenjing, et al. in [35] describe how they utilize both AI and blockchain technologies to devise a natural gas IoT framework within a smart city. The system uses a temporal pattern attention-based LSTM (TPA-LSTM) model to intuitively identify potential changes in natural gas movements to build a secure, blockchain-driven natural gas trading mechanism, which can maximize the benefits of both sellers and buyers while ensuring smooth contract transactions. In particular, the published results demonstrate the model’s superior ability to predict real-time natural gas production and dynamically match transactions based on evolving sales demand. Further emphasizing the supply chain aspect, Mao, Dianhui [36] aims to enhance supervision and management efficiency within the food supply chain by innovating a blockchain-centric credit assessment mechanism. The system, which is specifically tailored for food supply chain management, aggregates traders’ credit assessments on the blockchain. It utilizes deep learning technology to ensure the reliability and robustness of these assessments. As a result, these aggregated credit scores are then adopted as an assessment metric to strengthen regulatory oversight and management processes. Finally, Kamble, Sachin S., et al. [37] explore the spectrum of industrial supply chains. Their approach, based on the machine learning capabilities of AI, builds a predictive model that explains the likelihood of industry players adopting blockchain in the supply chain domain. This endeavor provides a clear picture of a company’s propensity, intentions, and rationale for blockchain adoption. This holistic understanding not only facilitates decision-making for enterprise stakeholders but also helps predict an organization’s readiness to adopt this technology. 

Seong-Kyu Kim [38], who works on blockchain-based record platforms, presents a system that seamlessly integrates hash function-based verification processes and artificial intelligence into a blockchain-based degree certificate verification platform. Their proposal, based on natural language processing, offers a new method of degree verification that works without the need for traditional public certificates. Instead, it uses hash algorithms to ensure authenticity and security. With this approach, they propose a more robust security protocol to enhance data protection. 

At the application layer of blockchain, attempts are being made to overcome the myriad limitations and challenges of blockchain-centric platforms through the combination of artificial intelligence. The fundamental properties of blockchain, particularly data integrity and transparency, combined with the sophisticated data processing, analytics, and predictive capabilities of AI, serve to amplify the inherent value of the platform. This combination enhances the performance and reliability of blockchain apps while opening up new applications and opportunities. Together, blockchain and artificial intelligence highlight the enormous potential inherent in their collaborative synergy. This partnership promises to be a pivotal development in the future of digital platforms. 

In this section, we introduced a number of studies focused on bringing AI into the blockchain space. Table 1 summarizes the goals and methodologies of the studies shown, categorized by their respective layers. Both blockchain and AI are positioned as transformative forces in the evolution of modern technology. The convergence of the two technologies provides an unparalleled layer of solutions, especially in the areas of data 

_Electronics_ **2025** , _14_ , 84 

15 of 35 

robustness, transparency, and process optimization. This research has been instrumental in maximizing the potential of blockchain in areas such as improving smart contract efficiency, rapid transaction verification, in-depth data analysis, and enhanced security. Artificial intelligence not only amplifies the computational power of blockchain but also improves user experience and boosts overall performance. These attempts at technology convergence confirm that there are endless possibilities when blockchain and AI work together to enhance each other’s capabilities. Continued research and development will lead to breakthrough innovations. 

**Table 1.** Classification of research papers in the section of artificial intelligence for blockchain. 

|**Layer**|**Task**<br>**Purpose**<br>**Approach**|
|---|---|
|Protocol<br>Layer|Mining, Consensus<br>PoW Energy Cost Improvement<br>Learning for Consensus [20]<br>Mining Algorithm Optimization<br>Multidimension Reinforcement Learning [21]|
||IoT Engine<br>Reliability of Data Upload Protocol<br>Data Flow Verifcation [22]<br>Lightweight Stakeholder Certifcation<br>Collaborative Framework [23]|
|Data<br>Layer|Fraud, Scam<br>Anomaly Detection<br>Raw Data Analysis [24]<br>Cross Graph Analysis [25]<br>Singular Value Decomposition [26]<br>Cluster-based Unsupervised Learning [27]<br>Collective Anomaly Approach [28]|
||Privacy, Security<br>Smart Contract Privacy<br>eXtreme Gradient Boosting [29]<br>DDoS Detection<br>Autoencoder + Multilayer Perceptron [30]<br>Intrusion Detection<br>PRO-DLBIDCPS [31]|
|Application<br>Layer|Healthcare<br>Susceptible Disease Treatment<br>Natural Language Processing [32]<br>Therapeutic Workfow<br>Blockchain-based AI Engine [33]|
||IoT Application<br>Internet of Vehicle<br>Distributed Secure Multiparty Computing [34]<br>Supply Chain<br>Temporal Pattern Attention-Based LSTM [35]<br>Credit Evaluation System [36]<br>Adoption Decision Support [37]|
||Document<br>Degree Certifcate<br>Natural Language Processing, Hashing [38]|



## _3.4. Research Survey on Blockchain for Artificial Intelligence_ 

3.4.1. Class 1—Federated Learning 

This section of the class explores research in the combination of blockchain with subdomains of artificial intelligence that specialize in distributed and federated learning. The research in this section focuses on the integration and application of blockchain technology to build and enhance distributed learning environments across multiple clients, with particular emphasis on recent research efforts to adapt existing blockchain platforms for federated learning AI and to ensure secure and transparent sharing of training data and model updates within a federated learning framework over the blockchain. 

Federated learning, a subset of deep learning, has gained traction in a variety of fields. This research approach enhances the confidentiality of raw data samples because 

_Electronics_ **2025** , _14_ , 84 

16 of 35 

each node independently manages its local data. It also emphasizes the participation of each local learning node in updating the global model. To address these significant challenges, Li, Dun, et al. explore blockchain-based federated learning (BCFL) in [39]. This approach seamlessly combines federated learning, a subset of artificial intelligence that encompasses decentralized deep learning techniques, with blockchain. The paper provides a comprehensive exploration of BCFL while introducing this new paradigm. It describes the blockchain ecosystem, technical obstacles to federated learning, and innovative ways blockchain can enhance federated learning. It also highlights federated learning applications enhanced by multiple incentive mechanisms and presents real-world application scenarios of BCFL. In a study to effectively implement federated learning and enhance participation rate, Mugunthan [40] leverages Ethereum and IPFS to build a decentralized privacy-centric federated learning platform that does not require a central trust authority. He proposes a model where agents can mutually share models that comply with differential privacy standards. The proposed model uses blockchain smart contracts to evaluate each agent’s contribution to learning and incorporates a reward system for commendable behavior. The methodology also extends to analyzing potential threats in the contribution evaluation process and the complexity of data sharing. To counter malicious updates while categorizing participants in updating the global model, Toyoda, Kentaroh, et al. [41] introduce a federated learning protocol. The design aims to exclude participants who can benefit from deviating from the traditional cooperation protocol, a problem that stems from the inherently distributed nature of federated learning. This innovative protocol uses a combination of competition theory and blockchain to measure contributions to the global model. This assessment forms the basis of a customized reward system and associated distribution algorithm. This approach seeks to instill a sense of common purpose in the learning process of the global model and spark the interest of all participants, while at the same time monitoring individual behavior. Hye Sung Kim et al. [42] also envision a blockchain-centric federated learning structure. It is designed to address the problems prevalent in federated learning, such as reliance on a high-overhead central server, vulnerabilities associated with single points of failure, inadequate global model updates, and issues surrounding local rewards. The architecture facilitates the exchange and verification of local model updates through a blockchain network rather than a traditional central server, and pays out rewards through a built-in system. Furthermore, local models are honed by applying local data samples for each device through a stochastic variance-reducing gradient descent technique, and then converge to a comprehensive global model through a distributed approximate Newton method, all while protecting the original integrity of local data. Zhang, Qiong, et al. [43] demonstrate a practical application of federated learning within a blockchain framework. Their demonstration illustrates secure server-client communication, especially when the federated learning server and its client base are distributed across geographically diverse entities. The demonstration scenario also details how a global neural network model can be trained using distributed datasets to ensure the confidentiality of each local client’s raw training data. 

In addition to research that addresses the procedural challenges of federated learning over blockchains, there is a large body of work that seeks to amplify distributed artificial intelligence and leverage the potential of blockchains. For example, Liang, Wei, et al. [44] introduced an intrusion detection model for blockchain systems that utilizes a data convergence approach based on collaborative clustering features. The detection model establishes data clusters from a large dataset of transactions within the blockchain to tailor the detection model to high-frequency data clustering attributes. This is conducted through relevant fuzzy classes, which improves the uniformity in identifying abnormal intrusions within the blockchain system. Furthermore, to enforce the proprietary nature of raw data from local nodes during 

_Electronics_ **2025** , _14_ , 84 

17 of 35 

federated learning, Lu, Yunlong, et al. presented an intelligent data-sharing platform tailored for industrial IoT in [45]. This work addresses the inherent security issues by using blockchain for data distribution collected from sensors to enhance data privacy and security. It also mitigates the trust imbalance among data owners to enable data retention and management. The result is federated learning, where numerous data owners jointly improve a global model without exposing raw data. Durga, R. [46] also designed a learning paradigm to respond to challenges such as the rapid spread of viruses, including COVID-19, and the lack of reliable testing instruments. The proposed model includes a federated learning framework based on blockchain. The framework focuses on the collective evolution and training of global deep learning models, the extraction of robust feature maps through capsule networks, and the replacement of traditional dense classification layers with extreme learning machines (ELMs) to enhance prediction accuracy. The model emphasizes data protection when exchanging data between hospitals via blockchain, facilitates the integration of different models, and ensures data integrity and security. 

As artificial intelligence continues to evolve, federated learning is materializing as a pivotal area of research with a focus on data privacy and protection. The essence of federated learning lies in improving the proficiency of a global model without directly transferring local data to the global model. The convergence of blockchain technology reinforces this principle, ensuring data integrity, transparency, and a secure learning journey. The convergence of blockchain and federated learning paves the way for integrating insights from numerous data sources while protecting the confidentiality of data owners. Progress in this research endeavor is expected to play an important role in shaping the future trajectory of artificial intelligence. However, despite several recent studies, attempts to converge blockchain and AI for federated learning have been observed to have a number of limitations, the main ones being scalability and efficiency due to the limitations of the blockchain protocol itself. 

## 3.4.2. Class 2—Model Learning 

This section takes a closer look at the research surrounding blockchain technology, which spans a variety of learning methodologies and plays a pivotal role in advancing artificial intelligence models. It specifically emphasizes processes such as pre-training and post-training. It also explores the role of blockchain as a data archive and a way to orient model training. The Class 2 section included studies that applied distributed learning and federated learning methodologies but had limited or additional contributions. These studies contributed more focused on learning methodologies and model maintenance and management and focused on understanding how blockchain amplifies the learning efficiency and transparency of AI models through distributed learning and federated learning. 

Pokhrel and Shiva Raj [47] introduced a framework for integrating blockchain-based federated learning designs into the autonomous vehicle domain. Their work is based on blockchain-based federated learning but has been categorized as a C2 class due to its unique contribution to learning. The study leveraged a blockchain consensus algorithm and updated reward strategy to optimize in-vehicle machine learning for artificial intelligence models in autonomous vehicles. It ensured that in-vehicle updates and the global model were recorded on a blockchain that was authenticated and accessible to all vehicles in the network. Vehicles were then incentivized based on the amount of learning updates they provided to the global model. Kumar, Rajesh, et al. also described a framework to address the scarcity and unreliability of test kits due to the COVID-19 pandemic in [48]. Their work focused on building supervised learning-based machine learning models and leveraging the latest data to improve the accuracy of CT image analysis while promoting collaboration and ensuring data privacy. They advocated for a blockchain-based federated learning system to facilitate consensus 

_Electronics_ **2025** , _14_ , 84 

18 of 35 

among research and medical institutions, convergence of training models, and supervision of the entire learning journey and participants. To facilitate learning and ensure meticulous documentation, You, Jie, and [49] integrated blockchain with reinforcement learning, a subset of artificial intelligence. They conceptualized each blockchain node as a reinforcement learning agent, determining the origin and connectivity of blocks through Markov decisions. The proposed model allowed reinforcement learning algorithms to deeply understand a wide range of possibilities and converge rapidly with the growth of the blockchain. Their work describes the blockchain as a decentralized learning system for reinforcement learning algorithms, capturing the essence of blockchain by improving and meticulously documenting the learning trajectory of artificial intelligence models. 

The medical and healthcare sector is evolving rapidly with technological advancements. In this progressive environment, blockchain and artificial intelligence are modern additions to healthcare solutions. In one study that introduced blockchain to train artificial intelligence models in healthcare, Chen et al. introduced “Health-chain”, a blockchain-based framework for disease classification in [50]. The framework was designed for two disease recognition tasks: breast cancer detection and electrocardiogram arrhythmia classification. By using asynchronous collaborative modeling techniques over blockchain, multiple institutions can train local models using patient records. They can also collaborate asynchronously over the blockchain network to ensure patient data privacy during model development. The study produced encouraging results in Healthchain simulations for two diseases. In another exploration centered around a network framework, Rahman, Mohamed Abdur, et al. [51] described the potential of blockchain in the realm of the Internet of Health Things (IoHT). The presented framework utilizes blockchain smart contracts, trust management protocols, participant authentication mechanisms, and device data encryption to process data collected from IoHTs. The findings reinforce the high security and scalability offered by blockchain, highlighting the significant relevance of blockchain in the IoHT healthcare management spectrum. Further emphasizing the role of blockchain in the healthcare sector, Hathaliya, Zigna, et al. [52] studied the application of blockchain within artificial intelligence systems for patient management. The study detailed a blockchain-based remote patient monitoring system to facilitate healthcare through remote patient supervision. It proposed a combination of decentralized artificial intelligence and blockchain to securely store and monitor patient data in real-time. The system leverages the inherent immutability and transparency of blockchain to ensure the security and confidentiality of patient data while streamlining data exchange between healthcare organizations. 

In artificial intelligence, key steps such as pre-training and mid-training play a pivotal role in improving the performance and efficiency of models. Models developed through these processes manage the complexity and diversity of data to provide more effective solutions to real-world problems. However, data privacy and security issues can arise during the learning process, model construction, and user management. Blockchain technology addresses these issues, ensuring data transparency, integrity, and security during the training process of AI models. The decentralized nature of blockchain aligns seamlessly with the learning trajectory of AI models, potentially improving efficiency and reliability. As a result, the combination of blockchain in AI model training opens up new avenues of research and development in this area. 

## 3.4.3. Class 3—Data Sharing 

This section focuses on data sharing, a key enabler for skillfully training and utilizing AI models. Data, often referred to as the fuel of AI, can significantly improve model performance when it is abundant and diverse. However, the effective dissemination of 

_Electronics_ **2025** , _14_ , 84 

19 of 35 

data often faces challenges such as data security, ownership, and privacy. The adoption of blockchain technology can provide an effective solution to these issues. In this section, we take a closer look at research that highlights the role of blockchain in secure data storage and sharing, and verification of data provenance and quality. 

In the area of artificial intelligence in healthcare, Tagde, Priti, et al. in [53] closely examined the impact of blockchain and artificial intelligence on healthcare metrics, drawing data from the Web of Science and other organizations. The researchers conclude that the convergence of the integrity of blockchain and the analytical power of artificial intelligence can raise the standards and efficiency of healthcare. The study highlights the importance of managing and sharing medical data and hints at the revolutionary changes this convergence will bring to healthcare. It also describes the potential for building trusted AI models in eHealth through blockchain. Another study, reference [54], presented a patient-centered healthcare system in response to the COVID-19 outbreak. In the realm of data management, blockchain acts as a sentinel to protect and disseminate patients’ health records. Blockchain’s immutability and transparency reinforce data integrity, and artificial intelligence leverages these data for predictive modeling and diagnosis. The synergy of these technologies enhances patient data security, facilitates real-time information sharing, and streamlines healthcare delivery. Blockchain also enables transparent sharing of data while protecting patient confidentiality, as data provenance and access rights can be clearly traced. 

In the field of supply chain data management and sharing, Zhang and Xin [55] introduced an approach to mitigate the risks inherent in supply chain logistics through the convergence of blockchain-based logistics service models and artificial intelligence. The paper presented an improved ant colony algorithm to optimize mobile robots within the supply chain. It also proposes a distributed, reliable, and error-free data-sharing paradigm via blockchain. Their results show that this method can reduce the risks associated with supply chain business systems by half. In addition, Hua, Gaofeng, et al. [56] ventured to apply artificial intelligence federated learning algorithms specifically for railway management systems. Their approach leverages blockchain and asynchronous collaborative learning algorithms to circumvent the need for a central server, thereby solving the “data island” problem, a widespread phenomenon of limited data sharing in industrial settings. To control the trains, data from each train were coordinated through an SVM-based mixture kernel, while the overarching global model utilized class ordering techniques overseen by blockchain smart contracts. 

In another exploration focused on data management and sharing, Zhan and Tiffany [57] turned their attention to fostering trustworthy AI tailored for blockchain-based cryptocurrencies. They emphasized the irreversibility of blockchain transactions and proposed ways to increase data trustworthiness. They acknowledged that many AI systems are vulnerable to threats, leading to decreasing trust in AI, and argued that trustworthy AI must be achieved by emphasizing that AI systems must operate efficiently, ethically, and morally to ensure harmonious interactions with humans. Du, Yao, and others [58] emphasized the growing importance of AI due to advances in mobile edge computing (MEC) and IoT. They noted that the rise of distributed machine learning in combination with MEC has made IoT data more valuable. While this has enabled on-the-ground training, it has also raised data security concerns. The paper summarizes important insights in the areas of blockchain-based MEC, machine learning, and secure data sharing. It also presents real-world examples and the challenges posed by blockchain-integrated intelligence. Their innovative proposal is to build a network centered on IoT devices through blockchain-based edge computing, subsequently forming a blockchain network, and embedding artificial intelligence in this network. 

_Electronics_ **2025** , _14_ , 84 

20 of 35 

## 3.4.4. Class 4—AI Governance 

The combination of blockchain and AI contributes significantly to the management of the AI models themselves. It not only improves the efficiency of training and use of AI models but also plays an important role in their lifecycle and versioning. Blockchain can also be used to enhance AI governance, including management systems and compliance with ethical standards, to ensure the fairness and accountability of AI models. 

Blockchain technology has the potential to significantly improve the operational efficiency, lifecycle management, and model versioning of MLOps. P. Ruf’s research [59] explores how blockchain can enhance the modularity and accountability of MLOps pipelines for cyber-physical systems, and details how this approach can contribute to the reliability and performance monitoring of systems. R. Subramanya [60] presents the application of MLOps to electricity market forecasting to integrate blockchain technology for data and model versioning and discusses how this technology ensures data integrity and reproducibility. D.S. Battina [61] presents the design of an intelligent DevOps platform that integrates blockchain and analyzes how it facilitates automation and collaboration of MLOps. 

Blockchain has an important role to play in enhancing the ethical use of AI technology. J. Kucera [62] examines how blockchain technology can support ethical AI development, specifically emphasizing how blockchain’s immutability and transparency can improve fairness in AI decision-making. M.M. Sharif [63] analyzes the ethical use of blockchain within organizations, offering concrete ways to increase transparency and accountability. R. Asif [64] proposes a blockchain-based governance framework to support responsible AI development and shows how to ensure fairness and accountability in AI systems. 

Blockchain can effectively support the governance and risk management of AI systems. D. di Prisco [65] analyzes the impact of blockchain and AI technologies on corporate governance, highlighting how these technologies are changing regulatory compliance and risk management. M. El Khatib [66] looks at how blockchain technology can improve decision-making and e-governance in project and program management and specifically addresses how governments and the private sector need to adapt to the new regulatory environment. 

Known for its emphasis on data transparency, integrity, and security, blockchain technology shows significant potential for data sharing in the AI space. In particular, as mobile edge intelligence and machine learning generate massive amounts of data, blockchain is becoming increasingly important for secure sharing and processing. Efficient data sharing and learning in a distributed environment, such as IoT devices, remains a challenge. Blockchain is emerging as a technology that can play a pivotal role in addressing these complexities. In this section, we review numerous studies and methodologies that highlight the improvements that blockchain brings to AI data-sharing mechanisms. The combination of blockchain and AI heralds a new paradigm in data sharing. 

We have described several research efforts that utilize blockchain for AI. Table 2 describes the goals and proposed technologies based on the challenges presented by the research presented in each layer. Both AI and blockchain are at the forefront of innovation in their respective domains, and the potential for blockchain to contribute to AI has received considerable attention. Here, we explore various research papers detailing the instrumental role of blockchain in AI data management, security, and computational processing. Blockchain’s inherent decentralization and transparency make it extremely useful for tracking the provenance and quality of training data for AI models while securing data sharing and storage. In addition, features such as automated learning protocols and incentivized data sharing via smart contracts further sophisticate the learning and deployment dynamics of AI. In summary, blockchain technology has the potential to make AI more transparent, robust, and scalable. The insights gained from these studies illustrate 

_Electronics_ **2025** , _14_ , 84 

21 of 35 

the enormous potential inherent in the symbiosis between blockchain and AI. There are high hopes that the convergence of these two powerful technologies will lead to even closer cooperation in further research and real-world applications. 

**Table 2.** Classification of research papers in the section of blockchain for artificial Intelligence. 

|**Layer**|**Task**|**Purpose**<br>**Approach**|
|---|---|---|
|Federated<br>Learning|Framework,<br>Architecture|Blockchain-based Federated Learning<br>Incentive Mechanism [39]<br>Data Privacy<br>Contribution Scoring, Incentive Mechanism [40]<br>Preventing Unjust Enrichment<br>Theory of Competition, Contribution Measurement [41]|
|||Local Data Security<br>Local Model Update Validation [42]<br>Demo Experiment [43]<br>Security Enhancement [45]|
|||Intrusion Detection<br>Data Fusion Approach [44]<br>FL-Model Reliability<br>Extreme Learning Machine [46]|
|Model<br>Learning|Framework|Blockchain-based Federated Learning<br>Update Compensation Approach [47]<br>Reinforcement Learning Acceleration<br>Consensus for Learning [49]|
||Healthcare|Reliability<br>Blockchain-based Learning [48]<br>Local Data Security<br>Asynchronous Cooperative Modeling [50]<br>IoHT<br>Survey [51]<br>Data Privacy<br>Blockchain-based Distributed AI [52]|
|Data<br>Sharing|Healthcare|Medical Data Sharing<br>Medical Index Analysis [53]<br>Data Management<br>Predictive Modeling and Diagnosis [54]|
||Supply Chain|Risk Management<br>Enhanced Ant-colony Algorithm [55]<br>Data Island Problem<br>Asynchronous Collaborative Learning [56]|
||Data|Model Reliability<br>Transaction Irreversibility [57]<br>Data Security<br>Edge Computing Blockchain [58]|
|AI<br>Governance|MLOps|Cyber-Physical System<br>MLOps Pipeline [59]<br>Electricity Market Forecasting<br>MLOps [60]<br>Intelligent DevOps<br>MLOps [61]|
||Ethical AI|AI Decision-making Process<br>Survey [62]<br>Organization Ethical<br>Survey [63]<br>Responsible AI Dev<br>Blockchain-based Governance [64]|
||AI Compliance,<br>Governance|Compliance, Risk<br>Organization Governance [65]<br>Dicision-making, E-Governance<br>AI, Blockchain Governance [66]|



## **4. Combination of Blockchain and AI** 

## _4.1. Potential Benefits of Combining Blockchain and Artificial Intelligence_ 

The combination of blockchain and artificial intelligence opens up new avenues for harnessing the collective strengths of both technologies. We have identified a wide range of technology combination studies and confirmed various technical synergies. These synergies enhance data transparency and security, improve the learning and prediction capabilities of AI, and amplify the usefulness of blockchain. This section aims to evaluate the benefits and future possibilities of combining blockchain and artificial intelligence from the perspective of technology, which were derived from the collection and analysis of research papers. 

_Electronics_ **2025** , _14_ , 84 

22 of 35 

Blockchain’s inherent data integrity and permanence ensure a secure and immutable record of all data transactions. This property of blockchain helps AI to track and verify the origin and change history of training data meticulously, making AI models more resistant to fraudulent data manipulation during training. The intersection of these technologies emphasizes a robust mechanism for verifying the origin and accuracy of the data processed by AI, ensuring both the reliability of AI models and the results of their learning. This enhanced reliability amplifies the legitimacy of the research and learning process, increasing trust and faith in AI-based solutions. In addition, the robust data security and user protection mechanisms of blockchain enhance the security of the data processed by AI. This blockchain property ensures a secure and immutable record of all data transactions. This property of blockchain helps AI to meticulously track and verify the source and change history of training data, making AI models more resistant to fraudulent data manipulation. The intersection of these technologies highlights a robust mechanism for verifying the source and accuracy of the data processed by AI, ensuring both the reliability of AI models and the results of training. This enhancement is especially pivotal in sensitive areas such as healthcare, where the security of patient and medical data is of utmost importance [67,68]. The secure data management infrastructure of blockchain not only ensures data integrity and reliability but also provides robust record-keeping, secure data sharing, and efficient access control. This comprehensive security framework greatly improves the efficiency and safety of AI in processing large amounts of data, enhancing the potential for collaboration between blockchain and AI in various areas. In essence, the convergence of blockchain and AI has emerged as a powerful alliance, greatly enhancing data security, transparency, and the overall efficiency of AI learning and prediction, thereby expanding the horizons of possibilities in various fields. 

Artificial intelligence has the potential to greatly enhance the security protocols of blockchain networks. AI can proactively detect abnormal transaction patterns and respond to security threats. This enhanced security and automated detection of malicious activity will strengthen the system to immediately identify and mitigate instances of money laundering, phishing, gambling, and fraud within blockchain systems and platforms. Recent research has begun to pinpoint these malicious activities, and AI is increasingly playing a role in enhancing blockchain security [69]. AI also has significant capabilities for the intelligent operation and maintenance of blockchain technology. Given the large amounts of data generated in real-time on blockchains, AI can be used as a practical tool to help identify and eliminate performance bottlenecks by analyzing these data to detect potential flaws and predict system failures [70]. Quality assurance of blockchain smart contracts is also an area of AI. Smart contracts operate autonomously once they are registered on the blockchain, but like traditional computer software, they are not immune to bugs or programming errors. These vulnerabilities, along with the cases of operational failures caused by fee-related problems inherent to blockchain program execution, highlight the indispensability of AI. AI can facilitate intelligent quality assurance by automatically detecting and alerting problems in program code during the development and registration stages, or by suggesting corrective actions through advanced pattern detection mechanisms. 

AI’s role in a wide range of areas, from rapid verification of transactions to prioritizing transactions, has the potential to significantly improve system performance when combined with the operational efficiency of blockchain. The symbiotic combination of AI and blockchain technology offers new opportunities and potential benefits, ready to transform and drive innovation across a wide range of industries [71,72]. 

_Electronics_ **2025** , _14_ , 84 

23 of 35 

## _4.2. Technological Foundations of Combination Blockchain and Artificial Intelligence_ 

Figure 4 shows the individual technological elements of blockchain and artificial intelligence and some of the technology areas that can benefit from the convergence of the two. The figure is the result of summarizing the key research from the technology combination papers introduced in the main text, showing the potential capabilities of blockchain and artificial intelligence used for each combination study and the technological innovations derived from the integration of the detailed technology areas. It highlights areas that can be strengthened by combining the privacy, transparency, decentralization, and smart contracts of blockchain with the machine learning, automation, adaptability, and data processing capabilities of artificial intelligence. This combination serves to address the technical flaws inherent in each technology individually. After reviewing the results of several studies, the following technical elements enable the successful integration of blockchain and artificial intelligence. 

**Figure 4.** Elements for technological combination of blockchain and artificial intelligence and examples of complementary combination technologies. 

## 4.2.1. Blockchain Technology 

- Smart Contract: A smart contract is an automated contract that runs on a blockchain network. Smart contracts, which are written in various programming languages and distributed across the blockchain network, ensure automated, error-free execution and accurate input processing, and enhance reliability by recording records and processes within the blockchain. These features automate the execution of AI algorithms and record the results on the blockchain, thereby improving the transparency, verifiability, and reliability that AI models may lack. 

- Distributed data storage: Blockchains, which take advantage of the characteristics of distributed networks, go beyond the limitations of centralized servers and ensure data integrity and transparency. This distributed data storage provides enhanced security storage and sharing by strengthening the reliability of the training and testing data essential for training AI models. 

- Encryption technology: Blockchain-encrypted data storage enhances the security of data that is processed and trained by artificial intelligence. This encryption ensures data confidentiality to prevent unauthorized access or manipulation while enabling AI to use blockchain encryption technology to predict and respond to security threats. 

_Electronics_ **2025** , _14_ , 84 

24 of 35 

- Distributed computing: The ability of blockchain to distribute computing tasks among all network participants increases the efficient use of computing resources, reducing the time and cost associated with large-scale computing tasks. This distribution enables AI to learn and process in a decentralized network, providing the computing power needed to manage vast amounts of data and complex AI models, thereby improving the speed of learning and processing. 

## 4.2.2. Artificial Intelligence Technology 

- Data analysis and processing: Artificial intelligence is excellent at analyzing and processing large datasets. This ability can be used to improve the performance and security of a blockchain by examining transaction data or network status data generated within the blockchain. In addition, artificial intelligence can discover new research directions by learning the vast amount of data in a blockchain, and this exploration can enable informed predictions or decisions. 

- Automation and optimization: The ability of AI to solve complex problems or automate repetitive tasks can be used to automatically execute smart contracts on the blockchain or to support the optimization of blockchain networks. In addition, AI-based optimization techniques can improve the programming stability of blockchain networks through detailed smart contract fee calculations and comprehensive security analysis. 

- Machine Learning: Machine learning, which is one of the branches of artificial intelligence, emphasizes the creation of evolving algorithms by learning from data to improve performance. This technology can be used to identify various patterns within the blockchain, and these insights can be used to improve blockchain performance or strengthen security. The vast amount of data generated by blockchains can be used to explore graph-based anomaly detection or new pattern detection and suggest new research directions. 

- Explainability: Artificial intelligence has the ability to clearly explain the training data, the learning process, and the decision-making process, and provides verifiable results (for networks that support explainable learning processes). When combined with the transparency of blockchain, it provides significant benefits in improving user trust in blockchain and AI systems and ensuring the explicit trustworthiness of the explained AI model. 

These technological elements promote the convergence of blockchain and artificial intelligence. In addition, services or systems that use artificial intelligence tend to be more receptive to this convergence and reflect openness to the integration of blockchain technology in proportion to the perceived benefits and ease of use [73]. This openness provides new opportunities to leverage the collective strengths of both technologies. The combination of these two technologies can greatly improve data transparency, security, and the learning and prediction capabilities of artificial intelligence. In addition to these improvements, the technological combination has opened up the possibility of securing innovation and leadership in new areas such as the metaverse [74] and 5G [75], and has strengthened the capabilities of blockchain and artificial intelligence. 

## **5. Challenges** 

Blockchain and AI, the representative technologies of the Fourth Industrial Revolution, are making innovative progress in their own independent areas. However, the convergence of these two technology areas poses new technical challenges. We have previously compiled examples of the combination of technologies that are specific to each area of blockchain and artificial intelligence. In conclusion, recent attempts at combining technologies have encountered several limitations, which are characterized by the protocol-related problems 

_Electronics_ **2025** , _14_ , 84 

25 of 35 

of blockchain or limitations in the direction of technology utilization. We analyze various studies to establish the problems and limitations of these issues, analyze the nature and origin of the problems, and examine existing solutions and their limitations. We also predict how the convergence of blockchain and artificial intelligence will affect industry and academia in the medium to long term. This analysis is expected to play an important role in providing depth and direction for research on the convergence of blockchain and artificial intelligence. Through the analysis of previous research, the following are the foreseeable challenges in the technological convergence of blockchain and artificial intelligence. 

## _5.1. Trusted Oracle_ 

Blockchains ensure the integrity, transparency, and permanence of internal data, and smart contracts allow blockchain participants to invoke events, functions, and control operations and reliable inputs and outputs. However, smart contracts do not automatically generate events or functions internally, nor do they automatically import or generate data from external sources. External data must be provided by one or more nodes or network participants. To overcome these limitations, trusted oracles have been proposed as a solution. These oracles act as intermediaries between external data sources and the blockchain with the authority delegated by network participants. However, these oracles can distort or manipulate data due to errors that occur during the intermediation process. In addition, due to the immutability of the blockchain, incorrect data recorded on the blockchain remain permanently, increasing the need for trusted oracles. However, relying on oracles can lead to potential centralization and cause network complexity and instability in managing oracle trust and monitoring authority. The convergence of blockchain and artificial intelligence will make data uploads for training AI models, node aggregation for federated learning, or blockchain protocols for trusted oracles important. However, the problems posed by oracles require more complex and multidimensional solutions. Possible problems include missing data labels, indexes, and appendix values, and increased inaccuracy of learning models due to distorted data input. There is also a risk of AI model attacks through inaccurate data insertion or intentional data label manipulation during large-scale AI model training processes such as distributed learning and federated learning, which can also lead to attacks on experimental data. If a centralized oracle manipulates model results or falsely discloses model performance, the decentralization principle of the blockchain could be undermined and the network could be disrupted. 

To create a reliable oracle for blockchain data, a method of verifying data accuracy using data sources from multiple oracles is proposed. A method of increasing data reliability by aggregating the results of multiple oracles in a centralized manner rather than a decentralized manner is considered. In addition, a complementary oracle council based on an incentive system may be proposed through a mutual monitoring system for each oracle node. This aggregation of oracles can be used not only to provide data sources but also to improve the reliability-based blockchain mining protocol. In addition, the adoption of the Hyperledger Fabric mining method for private blockchains is expected to dramatically improve the performance of blockchain networks. In addition, it is predicted that it would be reasonable from a risk management perspective for such an oracle system to be included in the penalty system through an on-chain (side-chain) protocol rather than an off-chain protocol in order to become an internal system of the blockchain protocol and exist in the network. 

## _5.2. Scalability_ 

Currently, blockchain data protocols have much smaller transaction and block sizes than typical server-based networks. For example, the block size of Bitcoin is limited to 1 MB, allowing only about 2500 transactions per block. When considering the combination 

_Electronics_ **2025** , _14_ , 84 

26 of 35 

of blockchain and AI, the significant computational requirements and massive data requirements of AI stand out. However, given the current blockchain framework, it is not optimal to execute such intensive calculations directly. The inherent constraints of blockchain, such as transaction processing speed and data storage capacity, can hinder the performance and efficiency of AI tasks. In addition, the nature of blockchain can reduce network efficiency because all nodes must perform the same calculations to verify transactions. 

Several innovative solutions have been developed to address these scalability issues. Layer 2 scaling solutions, such as Plasma [76] and state channel technology [77,78] are being considered as viable means to improve the processing speed of blockchain transactions. Another approach is the off-chain approach, in which AI calculations are performed outside the blockchain, and only the resulting data are recorded on-chain. However, even in this scenario, the development of a reliable bridge [79] or a robust high-speed communication system that connects the blockchain and off-chain calculations is important. Solving these scalability problems will lay the foundation for the symbiotic convergence of blockchain and AI, allowing the two to operate as complementary protocols. These technological advances are expected to pave the way for next-generation platforms, where deep learning can be used as a mining algorithm to facilitate concurrent learning and mining. 

## _5.3. Data Privacy_ 

Artificial intelligence uses vast amounts of data to optimize its learning and prediction capabilities. At the same time, blockchain technology inherently guarantees the permanence and transparency of data. Combining the characteristics of these two fields brings numerous benefits, but also presents obvious problems. There is a risk of permanently storing sensitive user information on the blockchain. Given the immutability of blockchains (the property that recorded data are resistant to modification or deletion), there is growing concern about potential privacy violations, especially when inaccurate or sensitive data are accessible. 

Several innovative methods are being studied to address these privacy concerns. Cryptographic methods such as Zero Knowledge Proof (ZKP) [80] and Homomorphic Encryption [81] are emerging as potential solutions, which allow data to be processed or stored on the blockchain without compromising confidentiality. In addition, utilizing blockchain smart contracts to set data access permissions or specify data usage can be an effective strategy. Solving the problem of storing personal data on the blockchain through strategies such as multi-signature protocols or sophisticated homomorphic encryption can greatly improve the secure sharing of medical records and personal process data. However, there are limitations to the practical application due to the computational time and amount of homomorphic encryption and zero-knowledge proofs. In order to solve data privacy in the process of combining blockchain and AI technologies, the blockchain network needs to be improved from the protocol part. 

## _5.4. Combination of Consensus Algorithms and Artificial Intelligence_ 

In a blockchain network, consensus algorithms play an important role in ensuring data consistency and integrity among participants. Artificial intelligence offers promising ways to improve these algorithms or develop new ones. However, the convergence of these two fields presents inherent challenges. Optimizing AI-based consensus algorithms in the wrong direction can inadvertently threaten the stability and security of the blockchain. Given that these algorithms maintain the stability of the network and deter malicious actors, AI-based modifications or new proposals require thorough stability and predictability assessments. 

The solution is to use artificial intelligence to evaluate the efficiency of the consensus algorithm and verify its stability through simulation. In addition, the optimization of these algorithms should be carried out gradually through thorough testing and verification 

_Electronics_ **2025** , _14_ , 84 

27 of 35 

before actual deployment. The solution is AI, which is suitable for testing and optimization, and can greatly enhance the simulation environment. AI has the potential to improve network performance by appropriately adjusting the parameters of the consensus algorithm. For example, AI can dynamically optimize the difficulty level of Proof of Work (PoW) or dynamically optimize the preconditions of Proof of Stake (PoS). The integration of consensus algorithms and artificial intelligence has enormous potential to simultaneously improve blockchain efficiency and stability. In terms of the combination of blockchain and artificial intelligence, the consensus algorithm needs to have a research direction of Proof of Useful Work (PoUW). PoUW is an energy-efficient consensus that reprocesses and reuses other energy used in other areas or within the blockchain for consensus and is treated as a method to increase the efficiency of the existing consensus. However, to realize this potential, careful research and verification are required. 

## _5.5. Interoperability_ 

Interoperability is emerging as a cornerstone of the convergence of blockchain and artificial intelligence, enabling seamless data exchange and collaboration across different systems and networks. However, achieving this in the current technological landscape is a challenge. Numerous blockchain platforms such as Ethereum, Bitcoin, Hyperledger Fabric, and Corda each offer their own unique protocols and standards, making interoperability complex. In addition, the diversity of data formats and structures used by different blockchain networks or AI systems can hinder data consistency and there is a risk of data loss during the conversion process. In particular, when processing sensitive information, secure data transmission is of the utmost importance to avoid exposure to privacy and security vulnerabilities. 

Various technological advancements are being sought to solve these problems. For example, Cosmos’ Inter-Blockchain Communication (IBC) [82] protocol facilitates dialogue between various blockchain networks. Polkadot [83] acts as a bridging chain to simplify data conversion between blockchains. Chainlink [84] bridges the gap between various smart contracts and external data sources. There are also efforts to improve interoperability between countless artificial intelligence solutions through APIs and standardization initiatives. These efforts aim to unify data formats and structures, improving data consistency and operational efficiency. As the integration of blockchain and AI progresses, prioritizing interoperability and innovation will become increasingly important and will require continuous technology and research development. 

## _5.6. Inefficiency of Data Storage_ 

Blockchains inherently store all transactions and status changes in an immutable ledger. This ensures transparency and integrity, but it is not optimized for large-scale data storage, and there is a limit to how much it can increase storage costs. In particular, artificial intelligence requires the efficient storage and processing of vast amounts of data, including training data, model parameters, and intermediate calculation results, which is difficult to meet with the existing structure of blockchain. Layer 2 solutions have emerged to address these issues, and examples include state channels, plasma, and rollups [85]. These technologies aim to optimize data storage and processing costs by efficiently distributing on-chain and off-chain tasks. For example, rollups reduce the load on the blockchain network and improve scalability by processing data off-chain and only recording the results on-chain. Furthermore, the use of distributed file systems such as the Internet Protocol File System (IPFS) is also attracting attention. IPFS provides a way to store data in a distributed network outside the blockchain and record only the hash value on the blockchain, which guarantees 

_Electronics_ **2025** , _14_ , 84 

28 of 35 

data integrity while dramatically reducing storage costs. This allows for more efficient storage and access to the large-scale learning data and intermediate results required by AI. 

While many recent technical studies have been conducted in the form of compromising and giving up decentralization for the efficiency of data storage in blockchains, we argue that the direction of technological development goes beyond simply reducing storage costs and that the combination of the transparency of blockchains and the data processing capabilities of AI is the key to building a more practical and scalable system. In addition to IPFS, permanent storage solutions such as Arweave [86] and decentralized storage platforms (Filecoin, Storj, etc.) are emerging as new alternatives, and these technologies are expected to play a key role in promoting the convergence of blockchain and AI. Ultimately, solving the inefficiency of blockchain data storage is essential for integration with AI, and future research and technological development will continue to evolve around three axes: data processing speed, cost reduction, and scalability. 

## _5.7. Model Update and Version Management_ 

The integration of blockchain and AI creates challenges for model updates and versioning. The immutability of blockchain conflicts with the ever-evolving dynamics of AI. One potential solution is to include external model storage while maintaining only the model metadata and hash values on the blockchain. This approach allows the blockchain to authenticate data integrity, while still allowing the actual model data to be managed externally in an adaptable manner. Nevertheless, on-chain storage is essential in areas that prioritize data transparency, integrity, and reliability. In these cases, smart contracts can efficiently track model versions. However, limitations in data protocols and mining algorithms create obstacles, such as real-time synchronization issues. Addressing these issues, along with the efficiency of blockchain data storage, is an essential part of the future convergence of AI and blockchain platforms. 

## _5.8. Regulation and Standard_ 

As blockchain and AI evolve, the need for regulatory frameworks and standards becomes urgent. Regulatory bodies are establishing guidelines to protect user privacy, data security, and technology stability. At the same time, there is a movement to ensure technology compatibility and interoperability through industry standards. Standardization not only facilitates seamless system interaction, but also reduces development time, optimizes resources, and improves overall efficiency. The adoption of standardization is crucial for these technologies to become firmly established across sectors and remains a prerequisite for technological progress. 

## **6. Future Perspective and Discussion** 

## _6.1. Future Perspective_ 

The convergence of artificial intelligence and blockchain is a key task of the Fourth Industrial Revolution. When these two technologies converge, it is expected to promote groundbreaking changes in various fields and create new opportunities. Although various challenges and solutions for the convergence of the two technologies are being presented, there is an urgent need to present a new framework in which the two technologies can exert a synergistic effect. The new framework that needs to be presented is the ‘Convergence Framework’, which operates as a high-potential artificial intelligence framework based on the blockchain protocol. It is necessary to facilitate the complex calculations of AI while ensuring data transparency, security, and integrity, and support the massive data processing of AI. In addition, the performance and accuracy of AI models must be verified transparently to ensure reliability and protect data privacy. 

_Electronics_ **2025** , _14_ , 84 

29 of 35 

- Develop energy-efficient consensus algorithms: As part of the integration of blockchain and AI, the focus of research could employ methods to reprocess and integrate the computation required to train AI models into the blockchain consensus process. This could promote energy efficiency and better resource utilization and could lead to the development of new consensus algorithms. 

- Recording AI model update weights: The objective of the research is to develop an optimized framework for efficiently performing AI model updates and versioning on the blockchain. Possible methods discussed include sidechain technology and a framework for providing blockchain-based storage. This can support the continuous learning and improvement of AI models and performance enhancements while maintaining transparency on the blockchain. 

- Enhancing data privacy and security: Research is needed to address data privacy and security issues that may arise during the integration of blockchain and AI. Framework designs need to consider the tradeoffs between the transparency and decentralization of blockchain and privacy protection. This includes the development of cryptographic techniques and data protection mechanisms that can protect privacy while ensuring data accuracy. 

- Application to federated learning platforms: Considering the above, this framework is particularly suitable for federated learning platforms. Federated learning involves jointly training a global model while each participating node independently manages its local data. By leveraging the inherent trust and transparency of blockchain, data privacy and security can be enhanced. You can use blockchain to securely manage model updates and data exchange, and build incentive structures to encourage participation. While keeping the consensus algorithm secure, you can evaluate each node’s contribution to consensus performance based on its learning history and performance. 

The convergence of blockchain and AI is emerging as a key area of next-generation data-centric technologies. Therefore, it is necessary to set a new research direction to overcome the limitations of AI federated learning by utilizing blockchain technology and maximizing the interaction between the two technologies. To promote the convergence between blockchain and AI federated learning, this study proposes a blockchain platform design centered on a consensus algorithm to satisfy the consensus efficiency optimization represented by Proof of Useful Work (PoUW). Existing blockchain-based federated learning has not sufficiently addressed the limitations of TPS (Transaction Per Second), block data capacity constraints, and data privacy and reliability issues arising from centralized server-based federated learning. To overcome these limitations, blockchain and federated learning need to be integrated at the system level, beyond mere record storage. To this end, researchers need to explore a new approach that organically connects the consensus algorithm of blockchain and the learning contribution mechanism of federated learning. This provides a direction to solve the problem of high resource consumption of existing blockchain consensus algorithms by utilizing the learning contribution data generated in the process of federated learning in the consensus process. Such a consensus process can maximize the efficiency between learning and block generation and enhance the scalability of blockchain networks. Furthermore, these algorithms need to be designed to fairly evaluate learning contributions, protect data privacy, and increase system-wide transparency. 

In addition, researchers should work on improving the stability and scalability of large-scale AI data processing environments by introducing a multi-chain structure of mainchain and sidechains. A structure in which the main chain is responsible for updating global models, and sidechains can distribute large-scale data to ease the load on the main chain will be an important technical foundation for creating an efficient and reliable data management and learning ecosystem. Further research is needed on data synchronization 

_Electronics_ **2025** , _14_ , 84 

30 of 35 

and sharing protocols to optimize the interaction between blockchain and federated learning and maximize data transparency, privacy-preserving access control techniques, and mechanisms to increase the efficiency of global model updates. To successfully implement the convergence of blockchain and AI, researchers need to come up with novel designs and technical solutions to increase the reliability and efficiency of federated learning. This will enable blockchain and AI to work complementarily in the data-centric era and build a secure and efficient ecosystem, even in large-scale data processing environments. 

This approach maximizes the efficient utilization of resources and optimizes the energy consumption of the system through the integration of blockchain and AI. It can also serve as an optimized framework for managing AI model updates, data training, and model version tracking within blockchain blocks and transactions. This enables efficient management of continuous AI model improvements and updates within a blockchain network and provides a transparent mechanism for tracking changes to each model version. Such a framework will contribute to improving the performance and accuracy of AI models while maintaining the trustworthiness and transparency of the blockchain. 

## _6.2. Discussion_ 

Table 3 shows the papers that have investigated the combination of blockchain and AI. Many of the existing studies that investigate the convergence of blockchain and AI are narrowly focused on one specific technology or limited to specific applications, lacking a comprehensive approach. Most existing studies either focus on the development of either blockchain or AI or only examine limited examples of their integration, failing to provide a thorough analysis of how these two technologies can complement each other and maximize synergies. This fragmented approach fails to adequately assess the broader possibilities of convergence or the potential synergies that can be achieved through a more systematic combination of the two technologies. 

To fill this research gap, our study adopts a balanced and comprehensive view of the convergence of blockchain and AI. Our research explores how the strengths and weaknesses of each technology can be complemented and strengthened and proposes various integration possibilities and potential development paths for blockchain and AI. Specifically, we extensively review real-world examples from various industries where blockchain and AI are being integrated to systematically identify the benefits and technical challenges arising from this convergence, including ensuring data reliability, overcoming scalability issues, protecting data privacy, and achieving cross-platform compatibility. Unlike previous studies that focus on specific application cases or technical aspects, this research aims to explore how these technologies can be integrated more broadly and how to address the complex challenges that may arise during the integration process. 

With this comprehensive approach, our research goes beyond simple technology integration to explore how the two technologies can complement each other by addressing each other’s weaknesses and maximizing their strengths. Specifically, we propose a new framework that overcomes the current limitations of blockchain and improves data processing performance by integrating the learning and prediction capabilities of AI while maintaining the scalability and security of blockchain. The framework introduces a new consensus structure to manage AI model training data and processes more efficiently and securely, enabling the convergence of blockchain and AI in a more effective and innovative way. This provides a clear direction for future research, with a focus on moving beyond the simple technical integration of blockchain and AI technologies to address complex challenges and explore how the two technologies can work more organically and complementarily, providing researchers with new pathways. 

_Electronics_ **2025** , _14_ , 84 

31 of 35 

**Table 3.** Comparison Table of Blockchain and AI Integration Studies. 

|**Research Paper**|**Research Focus**|**Integration Approach**|**Research Methodology**|**Key Contributions**|
|---|---|---|---|---|
|Tyagi et al. [87]|Blockchain and AI<br>Integration in Supply<br>Chain Management|Synergy between<br>Blockchain and AI|Literature Review|Enhanced Security and<br>Performance in Supply<br>Chain|
|Ahamed et al. [88]|Blockchain and AI<br>Integration in<br>Autonomous Vehicles|Integration of AI and<br>Blockchain|Case Study|Enhanced Data<br>Traceability and Safety in<br>Supply Chain|
|Rickardo et al. [89]|AI and Blockchain<br>Integration in Supply<br>Chain Management|Innovative Integration of<br>AI and Blockchain|Comprehensive Review|Enhanced Transparency<br>and Effciency in Supply<br>Chain|
|Oriekhoe et al. [90]|Comprehensive Review<br>of Blockchain in Supply<br>Chain Management|Integration of Blockchain<br>and AI|Literature Review|Enhanced Data Privacy<br>and Scalability|
|Kashem et al. [91]|Integration of AI and<br>Blockchain in Supply<br>Chain Management|Integration of AI and<br>Blockchain|Comprehensive Review|Enhanced Optimization<br>and Effciency in Supply<br>Chain Operations|
||Blockchain and AI|||Enhanced Effciency and|
|Emrouznejad et al. [92]|Integration in<br>Operations and Supply|Integration of Blockchain<br>and AI|Experimental Study|Transparency in<br>Operations and Supply|
||Chain Management|||Chain|
|Tsolakis et al. [93]|AI and Blockchain<br>Integration in Business|Integration of AI and<br>Blockchain|Conceptual Review|Enhanced Sustainability<br>and Data Monetization<br>in Business|
||Blockchain and AI|||Enhanced Data|
|Zawish et al. [94]|Integration in<br>Agricultural Supply|Integration of Blockchain<br>and AI|Case Study|Traceability and<br>Effciency in Agricultural|
||Chain Management|||Supply Chain|
|||||Proposal of|
|Our Study|Comprehensive<br>Integration of Blockchain<br>and AI|Systematic Integration<br>Analysis|Blockchain Scalability, AI<br>Data Management,<br>Consensus Algorithm<br>Energy Effciency|Comprehensive<br>Framework, Addressing<br>Technical Challenges,<br>Suggesting Future|
|||||Research Directions|



## **7. Conclusions** 

The convergence of blockchain and artificial intelligence (AI) represents a major turning point in the evolution of technology. This paper comprehensively reviews studies and projects from various fields that propose the technological integration of blockchain and AI to identify the potential and future prospects of this convergence. Through this extensive analysis, we propose a novel framework that maximizes the synergistic effects of these two technologies, providing a clear direction to predict and guide future technological advancements. Future research should focus on validating the practical feasibility of the proposed framework and applying it to various industrial settings. In particular, experimental studies to evaluate the performance improvement and resource utilization optimization between blockchain and AI, cryptographic techniques to enhance data privacy, and verification of the stability of large-scale data processing using multi-chain structures are required. This will help put the convergence of blockchain and AI on a more reliable technological footing and contribute to promoting data-driven industrial innovation. 

The decentralized structure of blockchain ensures the integrity and security of data, while the analytical capabilities of AI enable efficient data processing and decision-making. The convergence of these two technologies has the potential to enable innovations that neither technology alone can achieve. However, challenges remain, including data protocol constraints and the need for comprehensive regulatory measures. This study provides an 

_Electronics_ **2025** , _14_ , 84 

32 of 35 

in-depth analysis of how the integration of blockchain and AI can be utilized in various industries, showing a broader and deeper scope than previous studies. By identifying, addressing, and proposing solutions to key technical challenges, we build a foundation for more organic and complementary operations of these technologies. Future research will focus on evaluating the feasibility of the proposed framework, which will enable the convergence of blockchain and AI to be achieved more effectively, paving the way for industrial transformation and redefining digital interactions. 

**Author Contributions:** Conceptualization, N.C. and H.K.; investigation, N.C.; writing—original draft preparation, N.C.; project administration, H.K.; supervision, H.K.; validation, H.K. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This work was supported by Kyonggi University‘s Graduate Research Assistantship 2024. This research was supported by Basic Science Research Program through the National Research Foundation of Korea (NRF) funded by the Ministry of Education (No. 2020R1A6A1A03040583). 

**Data Availability Statement:** No new data were created or analyzed in this study. 

**Conflicts of Interest:** The authors declare no conflicts of interest. 

## **References** 

1. Anton, D. ARTIFICIAL INTELLIGENCE IN BLOCKCHAIN-PROVIDE DIGITAL TECHNOLOGY. _Int. J. Innov. Technol. Econ._ **2022** , _4_ , 1–12. [CrossRef] 

2. Hickman, C.F.L.; Alshubbar, H.; Chambost, J.; Jacques, C.; Pena, C.A.; Drakeley, A.; Freour, T. Data sharing: Using blockchain and decentralized data technologies to unlock the potential of artificial intelligence: What can assisted reproduction learn from other areas of medicine? _Fertil. Steril._ **2020** , _114_ , 927–933. [CrossRef] [PubMed] 

3. Kumar, Y. AI techniques in blockchain technology for fraud detection and prevention. In _Security Engineering for Embedded and Cyber-Physical Systems_ ; CRC Press: Boca Raton, FL, USA, 2022; pp. 207–224. 

4. Ali, A.; Abd Razak, S.; Othman, S.H.; Eisa, T.A.E.; Al-Dhaqm, A.; Nasser, M.; Elhassan, T.; Elshafie, H.; Saif, A. Financial fraud detection based on machine learning: A systematic literature review. _Appl. Sci._ **2022** , _12_ , 9637. [CrossRef] 

5. Marwala, T.; Xing, B. Blockchain and Artificial Intelligence. _arXiv_ **2018** , arXiv:1802.04451. 

6. Muheidat, F.; Tawalbeh, L. Artificial Intelligence and Blockchain for Cybersecurity Applications. In _Artificial Intelligence and Blockchain for Future Cybersecurity Applications_ ; Series Title: Studies in Big Data; Maleh, Y., Baddi, Y., Alazab, M., Tawalbeh, L., Romdhani, I., Eds.; Springer International Publishing: Berlin/Heidelberg, Germany, 2021; Volume 90, pp. 3–29. [CrossRef] 

7. Nakamoto, S. Bitcoin: A Peer-to-Peer Electronic Cash System. Decentralized Business Review. 2008. Available online: https://www.klausnordby.com/bitcoin/Bitcoin_Whitepaper_Document_HD.pdf (accessed on 19 November 2024). 

8. Castor, A. A (Short) Guide to Blockchain Consensus Protocols. 2017. Available online: https://www.coindesk.com/markets/20 17/03/04/a-short-guide-to-blockchain-consensus-protocols/ (accessed on 19 November 2024) 

9. Khan, S.N.; Loukil, F.; Ghedira-Guegan, C.; Benkhelifa, E.; Bani-Hani, A. Blockchain smart contracts: Applications, challenges, and future trends. _Peer-Netw. Appl._ **2021** , _14_ , 2901–2925. [CrossRef] 

10. Zikratov, I.; Kuzmin, A.; Akimenko, V.; Niculichev, V.; Yalansky, L. Ensuring data integrity using blockchain technology. In Proceedings of the 2017 20th Conference of Open Innovations Association (FRUCT), Saint-Petersburg, Russia, 3–7 April 2017; pp. 534–539. ISSN: 2305-7254. [CrossRef] 

11. Narayanan, A.; Bonneau, J.; Felten, E.; Miller, A.; Goldfeder, S. _Bitcoin and Cryptocurrency Technologies: A Comprehensive Introduction_ ; Princeton University Press: Princeton, NJ, USA, 2016. 

12. Yli-Huumo, J.; Ko, D.; Choi, S.; Park, S.; Smolander, K. Where is current research on blockchain technology?—A systematic review. _PLoS ONE_ **2016** , _11_ , e0163477. [CrossRef] 

13. Buterin, V.; Conner, E.; Dudley, R.; Slipper, M.; Norden, I.; Bakhta, A. EIP-1559: Fee Market Change for ETH 1.0 Chain. 2019. Available online: https://eips.ethereum.org/EIPS/eip-1559 (accessed on 19 November 2024). 

14. Androulaki, E.; Barger, A.; Bortnikov, V.; Cachin, C.; Christidis, K.; De Caro, A.; Enyeart, D.; Ferris, C.; Laventman, G.; Manevich, Y.; et al. Hyperledger fabric: A distributed operating system for permissioned blockchains. In Proceedings of the Thirteenth EuroSys Conference, Porto, Portugal, 1–15 April 2018; EuroSys ’18, pp. 1–15. [CrossRef] 

15. Hyperledger Foundation. Hyperledger Besu. Available online: https://www.hyperledger.org/projects/besu (accessed on 19 November 2024) 

16. Stuart, R.; Peter, N. _Artificial Intelligence A Modern Approach_ , 3rd ed.; PEARSON: London, UK, 2010; ISSN: 978-0-13-604259-4. 

_Electronics_ **2025** , _14_ , 84 

33 of 35 

17. Li, T.; Sahu, A.K.; Talwalkar, A.; Smith, V. Federated learning: Challenges, methods, and future directions. _IEEE Signal Process. Mag._ **2020** , _37_ , 50–60. [CrossRef] 

18. Chalapathy, R.; Chawla, S. Deep learning for anomaly detection: A survey. _arXiv_ **2019** , arXiv:1901.03407. 19. Salah, K.; Rehman, M.H.U.; Nizamuddin, N.; Al-Fuqaha, A. Blockchain for AI: Review and Open Research Challenges. _IEEE Access_ **2019** , _7_ , 10127–10149. [CrossRef] 

20. Team, N.A. _Nebula ai (Nbai)—Decentralized ai Blockchain Whitepaper_ ; Academic Press: Cambridge, MA, USA, 2018. 21. Wang, T.; Liew, S.C.; Zhang, S. When blockchain meets AI: Optimal mining strategy achieved by machine learning. _Int. J. Intell. Syst._ **2021** , _36_ , 2183–2207. [CrossRef] 

22. Alrubei, S.M.; Ball, E.; Rigelsford, J.M. A Secure Blockchain Platform for Supporting AI-Enabled IoT Applications at the Edge Layer. _IEEE Access_ **2022** , _10_ , 18583–18595. [CrossRef] 

23. Khan, A.A.; Laghari, A.A.; Li, P.; Dootio, M.A.; Karim, S. The collaborative role of blockchain, artificial intelligence, and industrial internet of things in digitalization of small and medium-size enterprises. _Sci. Rep._ **2023** , _13_ , 1656. [CrossRef] [PubMed] 

24. Zheng, Z.; Dai, H.N.; Wu, J. Blockchain Intelligence: When Blockchain Meets Artificial Intelligence. _arXiv_ **2020** , arXiv:1912.06485. [CrossRef] 

25. Chen, T.; Li, Z.; Zhu, Y.; Chen, J.; Luo, X.; Lui, J.C.S.; Lin, X.; Zhang, X. Understanding Ethereum via Graph Analysis. _ACM Trans. Internet Technol._ **2020** , _20_ , 18:1–18:32. [CrossRef] 

26. Chen, W.; Wu, J.; Zheng, Z.; Chen, C.; Zhou, Y. Market Manipulation of Bitcoin: Evidence from Mining the Mt. Gox Transaction Network. In Proceedings of the IEEE INFOCOM 2019—IEEE Conference on Computer Communications, Paris, France, 29 April–2 May 2019; pp. 964–972. ISSN: 2641-9874. [CrossRef] 

27. Deepa, M.; Akila, D. Cost-Effective Anomaly Detection for Blockchain Transactions Using Unsupervised Learning. In _Intelligent Computing and Innovation on Data Science_ ; Lecture Notes in Networks and Systems; Peng, S.L., Hsieh, S.Y., Gopalakrishnan, S., Duraisamy, B., Eds.; Springer: Singapore, 2021; pp. 445–453. [CrossRef] 

28. Shayegan, M.J.; Sabor, H.R.; Uddin, M.; Chen, C.L. A Collective Anomaly Detection Technique to Detect Crypto Wallet Frauds on Bitcoin Network. _Symmetry_ **2022** , _14_ , 328. [CrossRef] 

29. Deebak, B.D.; AL-Turjman, F. Privacy-preserving in smart contracts using blockchain and artificial intelligence for cyber risk measurements. _J. Inf. Secur. Appl._ **2021** , _58_ , 102749. [CrossRef] 

30. Liu, T.; Sabrina, F.; Jang-Jaccard, J.; Xu, W.; Wei, Y. Artificial Intelligence-Enabled DDoS Detection for Blockchain-Based Smart Transport Systems. _Sensors_ **2022** , _22_ , 32. [CrossRef] 

31. Mansour, R.F. Artificial intelligence based optimization with deep learning model for blockchain enabled intrusion detection in CPS environment. _Sci. Rep._ **2022** , _12_ , 12937. [CrossRef] [PubMed] 

32. Ray, R.R.; Agar, Z.; Dutta, P.; Ganguly, S.; Sah, P.; Roy, D. MenGO: A Novel Cloud-based Digital Healthcare Platform for Andrology Powered by Artificial Intelligence, Data Science & Analytics, Bioinformatics and Blockchain. _Biomed Sci Instrum_ **2021** , _57_ , 4. [CrossRef] 

33. Griewing, S.; Lingenfelder, M.; Wagner, U.; Gremke, N. Use Case Evaluation and Digital Workflow of Breast Cancer Care by Artificial Intelligence and Blockchain Technology Application. _Healthcare_ **2022** , _10_ , 2100. [CrossRef] [PubMed] 

34. Raja, G.; Manaswini, Y.; Vivekanandan, G.D.; Sampath, H.; Dev, K.; Bashir, A.K. AI-Powered Blockchain—A Decentralized Secure Multiparty Computation Protocol for IoV. In Proceedings of the IEEE INFOCOM 2020—IEEE Conference on Computer Communications Workshops (INFOCOM WKSHPS), Toronto, ON, Canada, 6–9 July 2020; pp. 865–870. [CrossRef] 

35. Xiao, W.; Liu, C.; Wang, H.; Zhou, M.; Hossain, M.S.; Alrashoud, M.; Muhammad, G. Blockchain for Secure-GaS: Blockchain-Powered Secure Natural Gas IoT System With AI-Enabled Gas Prediction and Transaction in Smart City. _IEEE Internet Things J._ **2021** , _8_ , 6305–6312. [CrossRef] 

36. Mao, D.; Wang, F.; Hao, Z.; Li, H. Credit Evaluation System Based on Blockchain for Multiple Stakeholders in the Food Supply Chain. _Int. J. Environ. Res. Public Health_ **2018** , _15_ , 1627. [CrossRef] [PubMed] 

37. Kamble, S.S.; Gunasekaran, A.; Kumar, V.; Belhadi, A.; Foropon, C. A machine learning based approach for predicting blockchain adoption in supply Chain. _Technol. Forecast. Soc. Chang._ **2021** , _163_ , 120465. [CrossRef] 

38. Kim, S.K. Blockchain Smart Contract to Prevent Forgery of Degree Certificates: Artificial Intelligence Consensus Algorithm. _Electronics_ **2022** , _11_ , 2112. [CrossRef] 

39. Li, D.; Han, D.; Weng, T.H.; Zheng, Z.; Li, H.; Liu, H.; Castiglione, A.; Li, K.C. Blockchain for federated learning toward secure distributed machine learning systems: A systemic survey. _Soft Comput._ **2022** , _26_ , 4423–4440. [CrossRef] [PubMed] 

40. Mugunthan, V.; Rahman, R.; Kagal, L. BlockFLow: An Accountable and Privacy-Preserving Solution for Federated Learning. _arXiv_ **2020** , arXiv:2007.03856. [CrossRef] 

41. Toyoda, K.; Zhao, J.; Zhang, A.N.S.; Mathiopoulos, P.T. Blockchain-Enabled Federated Learning With Mechanism Design. _IEEE Access_ **2020** , _8_ , 219744–219756. [CrossRef] 

42. Kim, H.; Park, J.; Bennis, M.; Kim, S.L. Blockchained On-Device Federated Learning. _IEEE Commun. Lett._ **2020** , _24_ , 1279–1283. [CrossRef] 

_Electronics_ **2025** , _14_ , 84 

34 of 35 

43. Zhang, Q.; Palacharla, P.; Sekiya, M.; Suga, J.; Katagiri, T. Demo: A Blockchain Based Protocol for Federated Learning. In Proceedings of the 2020 IEEE 28th International Conference on Network Protocols (ICNP), Madrid, Spain, 13–16 October 2020; pp. 1–2. ISSN: 2643-3303. [CrossRef] 

44. Liang, W.; Xiao, L.; Zhang, K.; Tang, M.; He, D.; Li, K.C. Data Fusion Approach for Collaborative Anomaly Intrusion Detection in Blockchain-Based Systems. _IEEE Internet Things J._ **2022** , _9_ , 14741–14751. [CrossRef] 

45. Lu, Y.; Huang, X.; Dai, Y.; Maharjan, S.; Zhang, Y. Blockchain and Federated Learning for Privacy-Preserved Data Sharing in Industrial IoT. _IEEE Trans. Ind. Informatics_ **2020** , _16_ , 4177–4186. [CrossRef] 

46. Durga, R.; Poovammal, E. FLED-Block: Federated Learning Ensembled Deep Learning Blockchain Model for COVID-19 Prediction. _Front. Public Health_ **2022** , _10_ , 892499. [CrossRef] 

47. Pokhrel, S.R.; Choi, J. Federated Learning With Blockchain for Autonomous Vehicles: Analysis and Design Challenges. _IEEE Trans. Commun._ **2020** , _68_ , 4734–4746. [CrossRef] 

48. Kumar, R.; Khan, A.A.; Kumar, J.; Zakria; Golilarz, N.A.; Zhang, S.; Ting, Y.; Zheng, C.; Wang, W. Blockchain-Federated-Learning and Deep Learning Models for COVID-19 Detection Using CT Imaging. _IEEE Sensors J._ **2021** , _21_ , 16301–16314. [CrossRef] [PubMed] 

49. You, J. Blockchain Framework for Artificial Intelligence Computation. _Softw. Impacts_ **2022** , _13_ , 100314. [CrossRef] 

50. Chen, X.; Wang, X.; Yang, K. Asynchronous Blockchain-based Privacy-preserving Training Framework for Disease Diagnosis. In Proceedings of the 2019 IEEE International Conference on Big Data (Big Data), Los Angeles, CA, USA, 9–12 December 2019; pp. 5469–5473. [CrossRef] 

51. Rahman, M.A.; Hossain, M.S.; Islam, M.S.; Alrajeh, N.A.; Muhammad, G. Secure and Provenance Enhanced Internet of Health Things Framework: A Blockchain Managed Federated Learning Approach. _IEEE Access_ **2020** , _8_ , 205071–205087. [CrossRef] [PubMed] 

52. Hathaliya, J.; Sharma, P.; Tanwar, S.; Gupta, R. Blockchain-Based Remote Patient Monitoring in Healthcare 4.0. In Proceedings of the 2019 IEEE 9th International Conference on Advanced Computing (IACC), Tiruchirapalli, India, 13–14 December 2019; pp. 87–91. ISSN: 2473-3571, [CrossRef] 

53. Tagde, P.; Tagde, S.; Bhattacharya, T.; Tagde, P.; Chopra, H.; Akter, R.; Kaushik, D.; Rahman, M.H. Blockchain and artificial intelligence technology in e-Health. _Environ. Sci. Pollut. Res._ **2021** , _28_ , 52810–52831. [CrossRef] [PubMed] 

54. Jabarulla, M.Y.; Lee, H.N. A Blockchain and Artificial Intelligence-Based, Patient-Centric Healthcare System for Combating the COVID-19 Pandemic: Opportunities and Applications. _Healthcare_ **2021** , _9_ , 1019. [CrossRef] [PubMed] 

55. Zhang, X.; Shi, X.; Pan, W. Big Data Logistics Service Supply Chain Innovation Model Based on Artificial Intelligence and Blockchain. _Mob. Inf. Syst._ **2022** , _2022_ , e4794190. [CrossRef] 

56. Hua, G.; Zhu, L.; Wu, J.; Shen, C.; Zhou, L.; Lin, Q. Blockchain-Based Federated Learning for Intelligent Control in Heavy Haul Railway. _IEEE Access_ **2020** , _8_ , 176830–176839. [CrossRef] 

57. Zhan, T. Trustworthy Artificial Intelligence for Blockchain-based Cryptocurrency. In Proceedings of the 8th International Conference on Artificial Intelligence and Applications (AI 2022), Tianjin, China, 18–21 March 2022; Academy and Industry Research Collaboration Center (AIRCC): Chennai, India, 2022; pp. 63–68. ISBN: 9781925953787. [CrossRef] 

58. Du, Y.; Miao, S.; Tong, Z.; Lemieux, V.; Wang, Z. Blockchain-Empowered Mobile Edge Intelligence, Machine Learning and Secure Data Sharing. In _Advances in the Convergence of Blockchain and Artificial Intelligence_ ; IntechOpen: Rijeka, Croatia, 2021. 

59. Ruf, P.; Reich, C.; Ould-Abdeslam, D. Flexibility of Modular and Accountable MLOps Pipelines for CPS. In Proceedings of the IARIA Congress 2022: International Conference on Technical Advances and Human Consequences, Nice, France, 24–28 July 2022; pp. 69–75. 

60. Subramanya, R.; Sierla, S.; Vyatkin, V. From DevOps to MLOps: Overview and application to electricity market forecasting. _Appl. Sci._ **2022** , _12_ , 9851. [CrossRef] 

61. Battina, D.S. An intelligent devops platform research and design based on machine learning. _Training_ **2019** , _6_ , 68–75. 

62. Kuˇcera, J.; Bruckner, T. Blockchain and ethics: A brief overview of the emerging initiatives. In Proceedings of the BIR Workshops, Katowice, Poland, 23–25 September 2019; Volume 2443, pp. 129–139. 

63. Sharif, M.M.; Ghodoosi, F. The ethics of blockchain in organizations. _J. Bus. Ethics_ **2022** , _178_ , 1009–1025. [CrossRef] 

64. Asif, R.; Hassan, S.R.; Parr, G. Integrating a blockchain-based governance framework for responsible AI. _Future Internet_ **2023** , _15_ , 97. [CrossRef] 

65. Di Prisco, D. Blockchain and AI: The technological revolution’s impact on corporate governance relationships. In Proceedings of the New Challenges in Corporate Governance: Theory and Practice, Naples, Italy, 3–4 October 2019; pp. 368–381. 

66. El Khatib, M.; Al Mulla, A.; Al Ketbi, W. The role of blockchain in e-governance and decision-making in project and program management. _Adv. Internet Things_ **2022** , _12_ , 88–109. [CrossRef] 

67. M., S.; Chattu, V.K. A Review of Artificial Intelligence, Big Data, and Blockchain Technology Applications in Medicine and Global Health. _Big Data Cogn. Comput._ **2021** , _5_ , 41. [CrossRef] 

68. Rao, K.P.N.; Manvi, S. Survey on Electronic Health Record Management Using Amalgamation of Artificial Intelligence and Blockchain Technologies. _Acta Inform. Pragensia_ **2023** , _12_ , 179–199. [CrossRef] 

_Electronics_ **2025** , _14_ , 84 

35 of 35 

69. Ul Hassan, M.; Rehmani, M.H.; Chen, J. Anomaly Detection in Blockchain Networks: A Comprehensive Survey. _IEEE Commun. Surv. Tutorials_ **2023** , _25_ , 289–318. [CrossRef] 

70. Harris, J.D.; Waggoner, B. Decentralized and Collaborative AI on Blockchain. In Proceedings of the 2019 IEEE International Conference on Blockchain (Blockchain), Atlanta, GA, USA, 14–17 July 2019; pp. 368–375. [CrossRef] 

71. Hussain, A.A.; Al-Turjman, F. Artificial intelligence and blockchain: A review. _Trans. Emerg. Telecommun. Technol._ **2021** , _32_ , e4268. [CrossRef] 

72. Kumar, S.; Lim, W.M.; Sivarajah, U.; Kaur, J. Artificial Intelligence and Blockchain Integration in Business: Trends from a Bibliometric-Content Analysis. _Inf. Syst. Front._ **2023** , _25_ , 871–896. [CrossRef] 

73. Polas, M.R.H.; Afshar Jahanshahi, A.; Kabir, A.I.; Sohel-Uz-Zaman, A.S.M.; Osman, A.R.; Karim, R. Artificial Intelligence, Blockchain Technology, and Risk-Taking Behavior in the 4.0IR Metaverse Era: Evidence from Bangladesh-Based SMEs. _J. Open Innov. Technol. Mark. Complex._ **2022** , _8_ , 168. [CrossRef] 

74. Yang, Q.; Zhao, Y.; Huang, H.; Xiong, Z.; Kang, J.; Zheng, Z. Fusing Blockchain and AI With Metaverse: A Survey. _IEEE Open J. Comput. Soc._ **2022** , _3_ , 122–136. [CrossRef] 

75. Azzaoui, A.E.; Singh, S.K.; Pan, Y.; Park, J.H. Block5GIntell: Blockchain for AI-Enabled 5G Networks. _IEEE Access_ **2020** , _8_ , 145918–145935. [CrossRef] 

76. Plasma Chains. Available online: https://ethereum.org/en/developers/docs/scaling/plasma/ (accessed on 19 November 2024) 77. Ethereum Org State Channels. Available online: https://github.com/ethereum/ethereum-org-website/blob/dev/src/content/ developers/docs/scaling/state-channels/index.md (accessed on 19 November 2024) 

78. What Are Layer-2 Scaling Solutions. Available online: https://crypto.com/university/what-are-layer-2-scaling-solutions (accessed on 19 November 2024) 

79. Adi Ben, A. _Blockchain Bridges Have Failed Spectacularly. Can the Faith and Trust in Them be Restored_ ? YAHOO Finance: New York, NY, USA, 2022; Section: Opinion. 

80. Fiege, U.; Fiat, A.; Shamir, A. Zero knowledge proofs of identity. In Proceedings of the Nineteenth Annual ACM Symposium on Theory of Computing, New York, NY, USA, 25–27 May 1987; pp. 210–217. 

81. Yi, X.; Paulet, R.; Bertino, E.; Yi, X.; Paulet, R.; Bertino, E. _Homomorphic Encryption_ ; Springer: Berlin/Heidelberg, Germany, 2014. 82. Inter-Blockchain Communication. Available online: https://ibcprotocol.org/ (accessed on 19 November 2024) 

83. Interoperable, Scalable, Public Blockchain|Polkadot. Available online: https://www.polkadot.network/features/technology/ (accessed on 19 November 2024) 

84. Chainlink: The Industry-Standard Web3 Services Platform. Available online: https://chain.link/ (accessed on 19 November 2024) 

85. What Are Blockchain Rollups? Available online: https://www.ledger.com/academy/what-are-blockchain-rollups (accessed on 19 November 2024) 

86. Williams, S.; Diordiiev, V.; Berman, L.; Uemlianin, I. Arweave: A Protocol for Economically Sustainable Information Permanence. Arweave Yellow Paper. 2019. Available online: https://www.semanticscholar.org/paper/Arweave%3A-A-Protocolfor-Economically-Sustainable-Williams-Diordiiev/c7c62789ca397106e3df50b300bcdd494fecd27b (accessed on 19 November 2024). 

87. Tyagi, A.; Aswathy, S.; Abraham, A. Integrating blockchain technology and artificial intelligence: Synergies perspectives challenges and research directions. _J. Inf. Assur. Secur._ **2020** , _15_ , 78–85. 

88. Ahamed, N.; Karthikeyan, P. A reinforcement learning integrated in heuristic search method for self-driving vehicle using blockchain in supply chain management. _Int. J. Intell. Networks_ **2020** , _1_ , 100–110. 

89. Rickardo, G.; Gladson, V. Disruptive technologies in supply chain management such as artificial intelligence and blockchain. _World J. Adv. Res. Rev._ **2023** , _16_ , 22–34. [CrossRef] 

90. Oriekhoe, O.; Ashiwaju, B.; Ihemereze, K. Blockchain technology in supply chain management: A comprehensive review. _Int. J. Manag. Enterp. Res._ **2024** , _12_ , 50–65. 

91. Kashem, M.; Shamsuddoha, M.; Nasir, T.; Chowdhury, A. Supply chain disruption versus optimization: A review on artificial intelligence and blockchain. _Knowledge_ **2023** , _3_ , e100. [CrossRef] 

92. Emrouznejad, A.; Chowdhury, S.; Dey, P. Blockchain in operations and supply chain management. _Ann. Oper. Res._ **2023** , _315_ , 125–140. [CrossRef] 

93. Tsolakis, N.; Schumacher, R.; Dora, M. Artificial intelligence and blockchain implementation in supply chains: A pathway to sustainability and data monetisation? _Ann. Oper. Res._ **2023** , _312_ , 45–60. [CrossRef] [PubMed] 

94. Zawish, M.; Ashraf, N.; Ansari, R.; Davy, S. Toward on-device AI and blockchain for 6G-enabled agricultural supply chain management. _IEEE Internet Things Mag._ **2022** , _5_ , 100–115. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

