**==> picture [35 x 35] intentionally omitted <==**

## _Review_ 

## **AI Agents Meet Blockchain: A Survey on Secure and Scalable Collaboration for Multi-Agents** 

**Md Monjurul Karim[1] , Dong Hoang Van[1] , Sangeen Khan[2] , Qiang Qu[1,] * and Yaroslav Kholodov[3]** 

- 1 Shenzhen Institutes of Advanced Technology, Chinese Academy of Sciences, Shenzhen 518055, China; karim@siat.ac.cn (M.M.K.); hoang@siat.ac.cn (D.H.V.) 

- 2 Department of Communication Engineering, University of Science and Technology Beijing, Beijing 100083, China; sangeenkhan2662@gmail.com 

- 3 Faculty of Computer Science and Engineering, Innopolis University, 420500 Innopolis, Russia; ya.kholodov@innopolis.ru 

- Correspondence: qiang@siat.ac.cn 

Academic Editor: Massimo Cafaro Received: 7 December 2024 Revised: 9 January 2025 Accepted: 21 January 2025 Published: 2 February 2025 

**Citation:** Karim, M.M.; Van, D.H.; Khan, S.; Qu, Q.; Kholodov, Y. AI Agents Meet Blockchain: A Survey on Secure and Scalable Collaboration for Multi-Agents. _Future Internet_ **2025** , _17_ , 57. https://doi.org/10.3390/ fi17020057 

**Copyright:** © 2025 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license (https://creativecommons.org/ licenses/by/4.0/). 

**Abstract:** In recent years, the interplay between AI agents and blockchain has enabled secure and scalable collaboration among multi-agent systems, promoting unprecedented levels of autonomy and interoperability. AI agents play a vital role in facilitating complex decision making and improving operational efficiency in blockchain systems. This collaborative synergy is particularly evident in how multi-agent systems collectively tackle complex tasks to ensure seamless integration within these frameworks. While significant efforts have been made to integrate AI agents and blockchain, most studies overlook the broader potential of AI agents in addressing challenges such as interoperability, scalability, and privacy issues. In this paper, we bridge these gaps by illustrating the interplay between AI agents and blockchain. Specifically, we explore how AI agents enhance decentralized systems and examine blockchain’s role in enabling secure and scalable collaboration. Furthermore, we categorize practical applications across domains, such as Web3, decentralized finance (DeFi), asset management, and autonomous systems, providing practical insights and real-world use cases. Additionally, we identify key research challenges, including the complexities of multi-agent coordination, interoperability across diverse systems, and privacy maintenance in decentralized frameworks. Finally, we offer future directions in terms of governance, sovereignty, computation, and interpretability to promote a secure and responsible ecosystem. 

**Keywords:** AI agent; blockchain; decentralized AI; large language model; multi-agent collaboration; Web3 

## **1. Introduction** 

The global Web3 market is expected to reach a valuation of over USD 33.53 billion by 2030, which highlights its significant growth across industries and sectors [1]. Web3 represents the paradigm shift towards decentralized and user-centric internet services, where blockchain plays a fundamental role [2]. Blockchain is a decentralized distributed database that securely stores data across multiple nodes [3,4]. Blockchain provides transparency, security, and decentralization to catalyze the growth of several key Web3 applications, such as decentralized finance (DeFi) and decentralized autonomous organizations (DAOs). Both DeFi and DAO demonstrate a significant market impact through enhanced financial accessibility and community-driven governance models [5]. For example, DeFi has surpassed USD 200 billion in total value locked (TVL), which demonstrates its transformative impact 

https://doi.org/10.3390/fi17020057 

_Future Internet_ **2025** , _17_ , 57 

_Future Internet_ **2025** , _17_ , 57 

2 of 31 

on traditional financial systems [6]. On the other hand, 5000 DAOs are globally managing billions of dollars in assets, which showcases their potential for decentralized governance and collective decision making [7]. However, Web3 faces several challenges, such as limited scalability and interoperability, high energy consumption, and security vulnerabilities [8,9]. 

Artificial Intelligence (AI) plays a crucial role in addressing these challenges within Web3 and a blockchain ecosystem. By leveraging data-driven insights and optimization capabilities, AI enhances the future economic perspective of blockchain [10]. Additionally, AI boosts transaction throughput and detects anomalies, which are essential for ensuring the continued growth and stability of Web3 applications [11]. For instance, DeFi benefits from AI-driven risk assessment models to enhance financial security, automate trading strategies, and predict market trends [12]. Similarly, DAOs leverage AI to facilitate effective decision-making processes by analyzing vast amounts of community data and providing intelligent insights for governance [13]. On the other hand, AI’s predictive capabilities aid in forecasting network congestion and preemptively optimize transaction routing to enhance the overall network performance. Moreover, AI contributes to efficient data processing and analysis within blockchain ecosystems. It also ensures better scalability and reliability by using machine learning (ML) models to predict and manage workloads effectively. Additionally, the integration of AI with blockchain facilitates adaptive consensus mechanisms, where AI algorithms dynamically adjust consensus parameters based on network conditions. However, there are several challenges with the integration of blockchain and AI, which include autonomous decision making, scalability under high transaction volumes, efficient energy consumption, enhanced data privacy, and adaptive consensus management [14]. 

AI agents (or autonomous agents) address these issues by leveraging autonomous decision-making capabilities, real-time data analysis, and generative functionalities [15,16]. These agents are autonomous computational entities that perform tasks based on predefined goals, which makes them instrumental in addressing the challenges of Web3 [17]. In the era of GenAI and LLMs, AI agents have evolved significantly. They offer advanced capabilities for handling complex queries, generating natural language responses, and executing intricate decision-making tasks [18]. Similarly, these agents leverage generative capabilities to autonomously adapt to changing environments and facilitate decentralized decision-making processes. On the other hand, the concept of multi-agent systems (MASs) allows multiple AI agents to collaboratively work in order to solve complex problems within blockchain environments [19–21]. MAS enables distributed intelligence, where agents communicate, negotiate, and make collective decisions, which further enhances the scalability and efficiency in decentralized AI (DeAI) systems [22]. Therefore, integrating AI agents with blockchain presents several advantages. These include improved transparency, operational efficiency, and the ability to automate complex workflows. As a result, research in this area remains relatively limited, which underscores the need for more focused exploration and development. Our survey stems from this motivation to bridge this gap by exploring the synergies between AI agents and blockchain in enabling decentralized intelligence. 

## _1.1. Motivation and Contribution_ 

Blockchain serves as a decentralized and tamper-proof ledger to ensure secure and immutable transaction records among AI agents [23,24]. By integrating blockchain, AI agents benefit from enhanced trust through immutable records to ensure that interactions are transparent and verifiable. Additionally, blockchain provides strong data security by using cryptographic techniques, which reduces the risk of tampering or unauthorized data access. This is critical for maintaining the integrity and reliability of autonomous 

_Future Internet_ **2025** , _17_ , 57 

3 of 31 

agent operations. Its integration into agent-based environments enables reliable interactions across various functions, such as payments, ownership transfers, and contractual agreements. Moreover, the integration effectively addresses key challenges, such as privacy [17], accountability [25], and data persistence [26], to advance trust and verifiability in autonomous agent operations. Furthermore, it enhances the reliability and transparency of agent-driven systems and supports the development of decentralized intelligence and autonomy. Despite the importance of blockchain integration with AI agents and vice versa, existing studies [8,23,24,26–28] did not emphasize the challenges of integrating these technologies, such as scalability, interoperability, and privacy concerns. 

To address these gaps, we offer an integrated and in-depth analysis that covers both the blockchain’s role in ensuring AI trustworthiness and the potential of blockchain in enhancing AI agents’ capabilities. Specifically, our contributions include a thorough discussion of blockchain’s role as a reliable ledger and its utility in intelligent consensus mechanisms that facilitate autonomous decision making by AI agents. Additionally, we provide a detailed exploration of zero-shot learning for blockchain operation, which refers to the ability of AI agents to generalize and perform new tasks without prior specific training data. This approach is important for blockchain operation because it enhances AI agents’ adaptability to dynamic and unpredictable blockchain environments and enables them to handle new scenarios more effectively. We also categorize and examine practical application scenarios, such as DeFi, smart cities and governance, and the Internet of Things (IoTs) [29], to offer insights into real-world use cases where the intersection of AI and blockchain holds transformative potential. Furthermore, we outline future directions to address ongoing challenges in scalability and performance, data privacy and security, and interoperability and standardization. We also highlight the emerging trends and open research questions, which are critical for both academic and industrial communities. Note that the term “decentralized” is used throughout the survey, which refers blockchain, Web3, and DeAI ecosystem. 

The summarized contributions of our survey are as follows: 

- We explore how AI agents enhance blockchain systems, with a focus on their ability to improve intelligent decision making, optimize consensus mechanisms, and enable zero-shot learning. The result of these capabilities is an increase in adaptability, efficiency, and resilience in decentralized environments. 

- We investigate how blockchain enhances the capabilities of AI agents with the provision of a secure and decentralized infrastructure that supports coordination, governance, and data sharing. The integration leads to strengthened autonomy and scalability for AI agents alongside the assurance of privacy and reliability in collaborative environments. 

- We categorize various application and use case scenarios in DeFi, supply chain management, DAOs, edge computing, and fault management. These examples demonstrate improvements in operational efficiency, trust, and transparency while enhancing the adaptability and decision-making capabilities of AI agents in decentralized environments. 

- We outline the scalability, interoperability, and data privacy challenges to emphasize future directions in cross-chain communication protocols, privacy-preserving mechanisms, and decentralized coordination strategies. Additionally, we address emerging areas such as decentralized quantum computing, self-sovereign AI agents, and ethical and regulatory considerations to guide the development of decentralized systems. 

## _1.2. Organization_ 

Section 2 discusses the related work to establish the context for this study, followed by the preliminaries in Section 3. Section 4 describes the research methodology, including 

_Future Internet_ **2025** , _17_ , 57 

4 of 31 

the systematic strategy used for literature selection. Section 5 examines how AI agents contribute to enhancing blockchain operations, while Section 6 focuses on the role of blockchain in augmenting AI agent capabilities. Practical applications and real-world use cases of blockchain and AI agent integration are detailed in Section 7. Section 8 highlights key research challenges and outlines future directions. Finally, Section 9 concludes the survey by summarizing the key findings and insights. 

## **2. Related Work** 

We categorize the existing and relevant literature into two major categories: (i) trustworthy AI and blockchain and (ii) AI agents and blockchain. The first category focuses on how blockchain addresses the critical trust-related challenges in AI to enhance its transparency, accountability, and overall security. The second category focuses on integrating AI agents with blockchain to enhance their autonomy, communication efficiency, and decisionmaking capabilities. We provide an overall comparison between related surveys and our work in Table 1. 

**Table 1.** Comparison between our work and the existing surveys. 

|**Features**|**[8]**|**[23]**|**[24]**|**[26]**|**[27]**|**[28]**|**Ours**|
|---|---|---|---|---|---|---|---|
|Blockchain enhances AI’s trust||✓||✓||✓|✓|
|Blockchain elevates AI’s performance||✓|✓|||✓|✓|
|AI increases blockchain’s effciency|✓|||✓|✓||✓|
|AI enhances blockchain’s security|✓|||✓|✓||✓|
|Empirical application scenarios|||✓|✓|||✓|



## _2.1. Trustworthy AI and Blockchain_ 

The intersection of AI and blockchain has attracted considerable research attention. For example, Abdelhamid et al. [27] conducted a comprehensive review to address the limitations of blockchain, including scalability, security vulnerabilities, governance complexity, and high energy consumption, in the trustworthy AI environment. To tackle these issues, the authors explored AI-driven solutions, such as Proof of Deep Learning (PoDL), federated learning-based consensus mechanisms, and anomaly detection using deep learning, which aim to optimize the overall performance of blockchain systems. Moreover, they identified unresolved challenges, such as high transaction costs, privacy concerns, and the complexity of cross-chain interactions. On the other hand, Zhang et al. [28] discussed a life cycle-based framework to integrate blockchain into four key stages of AI development: planning, data collection, model development, and system deployment. The authors focused on addressing critical dimensions such as transparency, privacy, and accountability in AI systems. They also highlighted the potential of blockchain to ensure robust and ethical AI operations throughout the life cycle of AI systems. Meanwhile, Ressi et al. [8] presented a systematic overview of how AI enhances blockchain systems with a focus on consensus mechanisms and security. The authors provided a structured classification of AI techniques to highlight their application in order to improve blockchain reliability and trustworthiness. Their work underscored the potential of AI to advance blockchain protocols in terms of privacy, transparency, and decision-making reliability. Furthermore, Bhumichai et al. [26] elaborated on the convergence of AI and blockchain in three distinct stages: (i) emerging era, (ii) convergence era, and (iii) application era. The authors also explored key application domains (e.g., IoT, finance, cybersecurity, energy, and smart cities) to demonstrate the enhancement between blockchain and AI in areas such as data security, transparency, and decision making. They also highlighted the synergistic potential of AI and blockchain to address scalability, trust, and interoperability challenges in real-world scenarios. 

_Future Internet_ **2025** , _17_ , 57 

5 of 31 

## _2.2. AI Agents and Blockchain_ 

The integration of AI agents and blockchain focuses on enabling AI systems to operate independently and more effectively in decentralized environments. For instance, Tara et al. [24] provided a comprehensive survey of existing approaches and frameworks addressing communication latency in decentralized systems. The study examined multilayered ontology-based frameworks designed to reduce latency in agent-to-agent communication. The authors identified key challenges, including scalability and latency, and discussed strategies for optimizing efficiency through dynamic storage techniques. They also highlighted how decentralized storage mediums, such as the interplanetary file system (IPFS), could improve communication performance by minimizing unnecessary data transmission. The potential of integrating AI agents and blockchain is also discussed to enhance operational effectiveness. On the other hand, Nguyen et al. [23] explored how blockchain served as a digital economic and financial institution for autonomous AI agents. They focused on enabling AI agents to independently engage in economic transactions, financial services, and contract execution. The authors highlighted the role of blockchain’s decentralized infrastructure in overcoming traditional limitations faced by AI agents, such as restricted access to economic institutions. They also discussed key features, including private key management, decentralized exchanges, and smart contracts, to enhance the autonomy of AI agents. 

To summarize, both studies [23,24] emphasized the transformative potential of integrating AI agents with blockchain to address challenges related to communication latency, scalability, economic accessibility, operational efficiency, and decentralized storage management. Additionally, they highlighted blockchain’s role in mitigating traditional challenges, such as latency, scalability, and restricted access to economic institutions. 

## _2.3. Limitations of Existing Surveys_ 

The existing literature [8,27,28] presented several limitations. Most studies focused narrowly on specific aspects, such as scalability, transparency, or communication efficiency, or failed to provide an integrated perspective on the interaction between AI agents and blockchain technology. For instance, the studies [8,26] lacked empirical validation, making it challenging to assess the feasibility and actual impact of the integrated solutions in decentralized ecosystems. Additionally, there was a significant absence of detailed application examples (e.g., healthcare, finance), where real-world implementations were complex and required careful consideration of practical and regulatory challenges. On the other hand, the studies [27,28] focused heavily on trustworthiness but did not explore the role of AI agents within the blockchain ecosystem in depth. Conversely, the studies [23,24] explored AI agents’ autonomy but failed to provide a comprehensive analysis of blockchain’s trustenabling capabilities for AI systems. Furthermore, there was a lack of in-depth analysis that covered both blockchain-driven trust improvements and their specific implications for decentralized and autonomous AI agents. Another significant limitation was the absence of detailed application scenarios that bridged theoretical concepts with real-world implementations. Additionally, existing surveys neglected challenges related to human–agent interaction and the ethical implications of combining blockchain with AI agents. These aspects were crucial when considering applications that directly impacted users, such as smart contracts in finance or decision-making systems in healthcare. Without addressing these critical dimensions, the studies fell short of providing a truly comprehensive overview that took into account the ethical, social, and regulatory considerations essential for the successful adoption of AI agents and blockchain in key areas. 

_Future Internet_ **2025** , _17_ , 57 

6 of 31 

## **3. Preliminaries** 

In this section, we provide an overview of the foundational technologies underpinning our survey, including blockchain, Web3, DeAI, GenAI, LLMs, and AI agents. We discuss the key characteristics and applications of each technology. This section aims to establish the essential context for understanding the synergistic potential of blockchain and AI agents in building intelligent and autonomous systems. 

## _3.1. AI Agents_ 

AI agents play a crucial role in decentralized environments, enabling intelligent decision making, automation, and adaptability without relying on centralized authorities [20,30]. Technically, an AI agent is a computational entity that perceives its environment through various data inputs, processes this information, and takes action using actuators (mechanisms that execute actions) to maximize its chances of success at achieving specific goals. AI agents use a combination of AI techniques, including reinforcement learning (RL), federated learning (FL), and natural language processing (NLP), to autonomously adapt, learn, and make decisions in complex settings. 

AI agents are similar to several related terms, but there are subtle distinctions: 

- Autonomous Agents [16,31]: While all AI agents are autonomous, not all autonomous agents use AI. Autonomous agents are systems that operate independently based on pre-programmed rules or logic, whereas AI agents utilize learning and adaptability to make informed decisions. 

- Multi-Agent Systems (MAS) [32,33]: A multi-agent system consists of multiple AI agents working collaboratively or competitively to achieve individual or shared objectives. In MAS, agents can communicate and negotiate with each other, which adds a layer of complexity compared with single-agent AI systems. 

- Agent AI [15,31]: This term is often used interchangeably with AI agents but generally refers to the broader concept of agents powered by AI to execute tasks. It emphasizes the AI technology aspect of the agent. 

- Intelligent Agents [34,35]: These are agents equipped with AI capabilities to perceive, reason, and act in an environment to achieve certain goals. AI agents fall under this category, highlighting their intelligent capabilities such as learning, problem solving, and decision making. 

The distinctions among these terms help clarify the roles and capabilities of AI agents within various environments, particularly in decentralized settings where autonomous decision making and collaboration are crucial. 

As shown in Figure 1, the integration of LLMs has significantly revolutionized the capabilities of AI agents [15,31]. While RL and FL have been instrumental in developing adaptive and distributed learning systems, the incorporation of LLMs has driven AI agents into a new realm of cognitive abilities and task execution. For example, in the healthcare domain, AI agents equipped with LLMs can assist in diagnosing medical conditions by analyzing patient data and providing detailed insights, significantly improving accuracy and efficiency. In finance, AI agents use LLMs to predict market trends, automate trading strategies, and optimize investment decisions, thereby reducing human intervention and enhancing profitability. LLMs (e.g., GPT-4) serve as advanced policy instructors to enhance the decision-making capabilities of RL agents. Another example is the ReAct (Reasoning and Acting) framework, which demonstrates how LLMs can facilitate executing complicated sequential decision-making tasks by integrating both reasoning and action into their processes [16]. This integration enables agents to think about their actions and adapt accordingly, improving flexibility and efficacy in dynamic settings. As a result, 

_Future Internet_ **2025** , _17_ , 57 

7 of 31 

ReAct allows for more nuanced and context-aware responses compared with typical RL or FL approaches. 

**==> picture [296 x 263] intentionally omitted <==**

**Figure 1.** Operational architecture of LLM agents [36]. 

In our survey paper, our primary focus is on GenAI and LLM-based agents, as they represent the forefront of intelligent and adaptable systems in decentralized environments. However, we also include a comprehensive discussion of other AI agents to provide a holistic view of the evolving landscape of decentralized intelligence. 

## _3.2. Blockchain_ 

Blockchain is defined by its cryptographically secure and immutable distributed ledger, which provides a robust framework for auditing and record-keeping applications [37,38]. The foundational cryptographic primitives (e.g., hash functions, public-key cryptography) ensure data integrity by detecting and preventing unauthorized modifications to stored data (as shown in Figure 2). Decentralized consensus mechanisms, such as Proof of Work (PoW), Proof of Stake (PoS), Delegated Proof of Stake (DPoS), and Byzantine Fault Tolerance (BFT), eliminate the need for centralized authorities, thus enhancing trust and transparency among stakeholders [39]. These consensus protocols also enhance fault tolerance and reduce single points of failure, thereby improving system resilience. The ability of blockchain to maintain an immutable audit trail has been successfully applied in various sectors. For example, blockchain technology is applied in supply chain management to ensure the traceability and authenticity of products. By recording every transaction on an immutable ledger, it allows stakeholders to verify the origins and journey of goods, thereby reducing fraud and improving transparency. In financial services, blockchain enables secure peer-to-peer transactions without the need for intermediaries to ensure faster and more reliable payments [40]. Similarly, blockchain protects sensitive patient data in healthcare during exchanges between medical institutions to maintain privacy while facilitating secure information sharing [41]. Additionally, blockchain supports the creation of verifiable and transparent voting systems in e-governance to ensure election integrity and increasing public trust in democratic processes. 

_Future Internet_ **2025** , _17_ , 57 

8 of 31 

**==> picture [286 x 193] intentionally omitted <==**

**----- Start of picture text -----**<br>
initialize execute<br>deploy<br>validate<br>package<br>**----- End of picture text -----**<br>


**==> picture [105 x 181] intentionally omitted <==**

**Figure 2.** An overview of blockchain architecture. 

## _3.3. Generative AI_ 

GenAI utilizes sophisticated ML frameworks, including Generative Adversarial Networks (GANs) and transformer-based models (e.g., GPT, DALL-E), to autonomously generate diverse forms of content, such as text, images, audio, and code [42]. Transformer models rely on self-attention mechanisms to capture contextual relationships in sequential data. This capability enables them to produce coherent and contextually relevant outputs across various modalities [43]. GenAI is applied across various sectors, including creative industries, healthcare (e.g., synthetic data for diagnostics), software development (e.g., automated code generation), and personalized user experiences. However, GenAI also presents challenges, such as data privacy risks, biases in generated content, and the potential for misuse (e.g., spreading misinformation) [44]. Addressing these issues requires advancements in bias mitigation, explainable AI (XAI), and the development of comprehensive governance frameworks to ensure ethical implementation [45]. 

A key component of GenAI is LLMs, which are transformer-based models characterized by billions to trillions of parameters. LLMs excel in natural language comprehension and generation to demonstrate emergent capabilities such as in-context learning and sequential reasoning due to their scale and extensive pre-training on vast datasets [46–49]. While LLMs are just one example of GenAI, they are particularly notable for their ability to generate coherent, contextually relevant text, which has made them central to a wide range of applications, from chatbots to content creation. In addition, LLMs are often integrated with other GenAI systems, such as in multimodal frameworks, where they collaborate with models (e.g., DALL-E [50]) to generate text-based and image-based outputs [51]. Recent advancements in alignment techniques, particularly RL with Human Feedback (RLHF), have significantly improved the ability of LLMs to generate safe and contextually accurate outputs. These developments effectively address critical challenges, including model poisoning and the dissemination of false or misleading information. As a result, LLMs have become highly versatile tools, demonstrating broad applicability in fields such as natural language processing (NLP), information retrieval, and multimodal systems [52]. 

In the realm of MAS, GenAI and LLMs enable more sophisticated inter-agent communication and collaboration [20,30]. Agents can now engage in high-level strategic planning, delegate tasks based on specializations, and even explain their decision-making processes in human-readable formats. For example, collaborative robotics systems (e.g., UAV swarms) use multiple AI agents to coordinate tasks like assembly and quality inspection in manufacturing environments to improve efficiency and adaptability [16,20]. Similarly, DeFi systems 

_Future Internet_ **2025** , _17_ , 57 

9 of 31 

benefit from multi-agent interactions to optimize trading, lending, and decision-making processes across multiple participants [30,31]. These advancements highlight the transformative potential of AI agents when integrated with GenAI and LLMs, which creates intelligent and adaptable systems that can operate autonomously in decentralized and dynamic environments such as Web3 and DeAI. 

## _3.4. Web3_ 

Web3 is an evolution of the internet that leverages decentralized blockchain to create a more secure and user-owned experience. Unlike Web 1.0 (static content) and Web 2.0 (interactive platforms), Web3 leverages blockchain to create a “read–write–own” environment. This allows users to retain ownership over their data through public and private keys. As a result, privacy and security are significantly augmented through mechanisms such as encryption and decentralized data storage. As a result, users are empowered with greater control over how their personal data are shared and utilized. Additionally, the shift reduces the risks associated with centralized data breaches and enhances user trust in online interactions [53,54]. Web3 facilitates the creation of decentralized apps (dApps) in several sectors, such as banking (DeFi) [55], gaming, decentralized storage, and cross-chain interoperability [56,57]. Non-Fungible Tokens (NFTs) are integral to the Web3 ecosystem, as they facilitate the ownership and transfer of distinct digital assets on decentralized networks [58]. 

The following components represent key building blocks of the Web3 ecosystem, each a vital part of its decentralized and user-empowered structure: 

- Distributed Ledger Technology (DLT): The underlying technology of blockchain, DLT refers to a consensus of replicated, shared, and synchronized digital data spread across multiple locations. This ensures data integrity and transparency. 

- DAOs: DAOs shift from conventional centralized models to decentralized blockchainbased decision-making systems [54]. They use smart contracts to automate governance protocols and implement decisions based on member involvement by eliminating intermediaries to ensure transparency and efficiency [58]. For example, MakerDAO (https://makerdao.com (accessed on 20 January 2025)) members voted to add new collateral types to support the stability of its DAI stablecoin to demonstrate decentralized governance in action. 

- DeFi: DeFi represents a transformation of traditional banking and financial services through dApps to provide lending, borrowing, and trading without intermediaries [55]. It offers benefits such as reduced transaction costs, increased financial accessibility, and the elimination of traditional banking barriers. Examples include Uniswap and Aave. 

- Non-Fungible Tokens (NFTs): A mechanism for the ownership and transfer of distinct digital assets on decentralized networks [58]. NFTs play a crucial role in gaming, art, and virtual assets with platforms like OpenSea (https://opensea.io/ (accessed on 20 January 2025)) leading the market. 

- Decentralized Storage: A system where data are distributed across multiple nodes rather than being stored in a central server. Decentralized storage solutions such as IPFS (https://ipfs.tech/ (accessed on 20 January 2025)) and Filecoin (https://filecoin. io/ (accessed on 20 January 2025)) help enhance data redundancy and reduce the risk of single points of failure, contributing to the resilience of Web3. 

- Oracles: Services that connect blockchain with external data sources, enabling smart contracts to interact with off-chain data. Oracles are critical for Web3 as they expand the utility of blockchain by allowing real-world events to trigger on-chain actions. For example, in a DeFi application, oracles can provide real-time price feeds for assets 

_Future Internet_ **2025** , _17_ , 57 

10 of 31 

by allowing automated trading or lending decisions to be made based on current market conditions. Examples include Chainlink (https://chain.link/ (accessed on 20 January 2025)). 

## _3.5. Decentralized AI_ 

Decentralized AI (DeAI) employs blockchain technology to solve the constraints of centralized AI (CeAI) systems, such as single points of failure, data privacy problems, and scalability bottlenecks. CeAI frequently aggregates control over data and computation, which results in risks such as technological failures, cyberattacks, and biases in model outputs [22,59]. DeAI addressed these issues by implementing techniques such as immutable audit trails, privacy-preserving methodologies like zero-knowledge proofs (ZKPs), and tokenized reward systems for contributors [22]. Collaborative frameworks such as federated learning are enhanced by blockchain consensus processes, allowing decentralized model training and validation independent of central authority. Platforms like Ocean Protocol [60] and Render Network [61] demonstrated the power of DeAI by cultivating collaborative ecosystems for the secure and transparent exchange of data, models, and computing resources. DeAI faces challenges such as computing overhead, scalability limitations, and the need to balance transparency with privacy. Resolving these issues demands advancements in cryptographic methodologies, efficient consensus protocols, and decentralized governance frameworks [62]. The applications of DeAI in healthcare, autonomous systems, and decentralized markets illustrate its innovative potential to create ethical, safe, and diverse AI systems. A DeAI application’s lifetime is divided into the following five stages [22]: 

- Task Proposing: In this stage, the algorithms are prepared to guarantee that they satisfy the needs of decentralized systems in terms of privacy, scalability, and effective communication. The development of algorithms is essential at this phase to provide the basis for its entire life cycle. 

- Pre-Training: Data are gathered, cleaned, and segmented while ensuring privacy and security. Concurrently, computing resources such as GPUs or distributed systems are configured to facilitate efficient training. 

- On-Training: In this phase, model training and validation take place. Collaborating nodes concurrently update model parameters while maintaining synchronization. Validation guarantees the model’s precision and generalizability among all participants. 

- Post-Training: The trained model is ready for deployment and integration into applications for practical usage. This phase also enables model sharing via decentralized markets. 

## **4. Research Methodology** 

A systematic strategy was employed to ensure a thorough and rigorous investigation of the intersection between AI agents and blockchain. This approach involved precise identification of keywords, clear inclusion and exclusion criteria, and the use of recognized scientific databases. The methodology was designed to extract relevant academic and industrial contributions. In addition, we ensured that the study captures the most recent developments and essential insights in the fields of blockchain and AI. 

## _4.1. Keywords_ 

To ensure a comprehensive literature review, a set of carefully crafted keywords and their combinations were used. These keywords were configured to capture the multifaceted relationships between AI agents, blockchain, and their applications, as follows: 

_Future Internet_ **2025** , _17_ , 57 

11 of 31 

(“AI Agent” OR “Agent AI” AND Blockchain OR Web3), (“AI Agent” OR “Agent AI” AND “Generative AI” AND Consensus), (“AI Agent” OR “Agent AI” OR Web3 AND Privacy), (“AI Agent” OR “Agent AI” AND “Blockchain” AND “Generative AI” OR Web3), (“AI agent” OR “agent AI” OR “LLM agent” OR “autonomous agent” OR multi-agent OR “intelligent agent” AND blockchain OR “decentralized AI” OR web3 OR “generative AI”) 

## _4.2. Inclusion and Exclusion Criteria_ 

The inclusion criteria were defined to ensure relevance and rigor in the selected literature. This study included research explicitly addressing the integration of AI agents (e.g., LLM agents, autonomous agents, multi-agent systems) with blockchain, Web3, decentralized AI, or Generative AI. This includes studies on “Generative AI” in consensus mechanisms, privacy-enhancing techniques, and other advanced functionalities. To capture both established insights and emerging developments, we included peer-reviewed publications and preprints from reputable repositories like arXiv. Given the relatively unexplored and rapidly evolving nature of this domain, the inclusion of preprints ensures that the study reflects cutting-edge innovations and ongoing research efforts. This approach provides a comprehensive foundation for analyzing theoretical frameworks, empirical studies, and real-world applications of AI agents in blockchain ecosystems. 

The exclusion criteria were designed to maintain the study’s focus on the intersection of AI agents and blockchain ecosystems. Research that mentions AI agents but does not explore their integration with blockchain, Web3, decentralized AI, or Generative AI was excluded. Additionally, studies lacking meaningful insights into the roles, functionalities, or applications of AI agents within these domains were omitted. This ensured that the evaluation remained aligned with the study’s objective of examining the synergistic integration of AI agents in blockchain-based ecosystems and their relevance to decentralized platforms. 

## _4.3. Research Databases and Selection Process_ 

The literature was collected from several well-established databases and search engines, including IEEE Xplore Digital Library, ACM Digital Library, SpringerLink, ScienceDirect, Web of Science, Google Scholar, and Google Search. 

The selection process began with a systematic search using the defined keywords, which yielded 140 articles. Each article was then screened based on the inclusion and exclusion criteria. In the first round, articles unrelated to the focus areas were excluded, reducing the count to 110. A second round of screening excluded papers lacking significant contributions to the understanding of AI agents and blockchain, narrowing the selection to the final 80 articles. This systematic approach ensures that the study is based on highquality, relevant, and meaningful research. 

## **5. AI Agents for Blockchain** 

In this section, we explore the pivotal role of AI agents in enhancing blockchain systems. Specifically, we examine how AI agents contribute to intelligent decision making, optimize consensus mechanisms, and leverage zero-shot learning to adapt to evolving environments. Additionally, we discuss their significant impact on decentralized collaboration and focus on their capabilities in terms of adaptability, resilience, and overall efficiency. We present a comparative summary of the relevant literature in Table 2. 

_Future Internet_ **2025** , _17_ , 57 

12 of 31 

**Table 2.** Summary of the existing research in terms of AI agents for blockchain. 

|**Feature**|**Ref.**<br>**Contribution**<br>**Quality**<br>**Scalability**<br>**Security**<br>**Effciency**|
|---|---|
|Intelligent Decision<br>Making|[63]<br>Leveraging blockchain, DAOs, and LLMs to<br>create fnancially self-sustaining and<br>self-managed digital building.<br>✓|
|Intelligent Consensus<br>Mechanism|[64]<br>Apply stake-based miner designation,<br>multi-round debate-style voting, and a specifc<br>reward mechanism to protect and ensure<br>legitimate and valuable records on blockchain.<br>✓<br>✓|
||[65]<br>Provide a self-sustainable compute service via<br>a network of independent AI agents that does<br>not rely on a single entity.<br>✓<br>✓|
|Zero-shot Learning for<br>Operation|[66]<br>Solve the problem of scaling culture<br>production in a blockchain-based business<br>model while reducing agency cost.<br>✓<br>✓|
|Vulnerability Detection|[67]<br>Integrate ensemble learning with large<br>language models to classify vulnerabilities in<br>smart contracts.<br>✓|



## _5.1. Intelligent Decision Making_ 

AI agents significantly enhance intelligent decision making by autonomously executing tasks embedded in blockchain-stored logic and facilitating dynamic adjustments to operational settings. According to Ly et al. [63], the AI agent leverages real-time data from IoT devices and integrates these inputs with blockchain-governed parameters to ensure compliance with predefined operational thresholds in a digital twin environment. Through its ability to interact with smart contracts, the AI agent autonomously governs environmental conditions, such as temperature, lighting, and air quality, optimizing system performance without direct human intervention. Additionally, the AI agent employs LLM capabilities for advanced reasoning to interpret multiple variables and adapt operations dynamically to meet evolving requirements. 

## _5.2. Intelligent Consensus Mechanism_ 

Blockchain enhances AI agents by providing intelligent consensus mechanisms that ensure robust coordination. These intelligent consensus mechanisms leverage adaptive decision-making processes and dynamic role assignments to be more responsive and context-aware compared with traditional mechanisms. Unlike conventional consensus approaches (e.g., PoW or PoS), adaptive consensus mechanisms evaluate the quality of contributions and dynamically assign evaluators based on performance and context. The approach allows AI agents to adapt to fluctuating network conditions or varying agent reliability while maintaining secure, fair, and efficient collaboration. For example, Chen et al. [64] propose BlockAgents, which addresses Byzantine challenges (i.e., issues involving unreliable or malicious agents disrupting the system) in LLM-based multi-agent systems. The core innovation lies in the Proof-of-Thought (PoT) consensus mechanism, which rewards AI agents based on their contributions during collaborative problem solving. PoT focuses on evaluating the quality of each agent’s contribution, making it uniquely suited for collaborative environments involving intelligent agents. The framework decomposes the multi-agent process into structured phases: role assignment, proposal generation, evaluation, and decision making, all of which are securely managed on the blockchain. By introducing a multi-metric prompt-based evaluation (e.g., factual consistency, redundancy detection, and contextual causal relevance), BlockAgents effectively identify malicious behaviors and thoroughly assess each agent’s contribution. 

_Future Internet_ **2025** , _17_ , 57 

13 of 31 

On the other hand, Proof-of-Compute (PoC) [65] extends the paradigm of intelligent consensus by integrating blockchain to facilitate decentralized computations. PoC assigns AI agents to ensure that the computational benefits of the network’s functionality reach users. These agents process prompts using trained models, and their outputs are validated by specialized verifiers, who ensure accuracy and integrity. The mechanism incentivizes honest participation through a Nash equilibrium design, where agents optimize their strategies to maximize rewards while adhering to system rules. This novel approach balances efficiency and security by utilizing blockchain-based verifiers to continuously oversee agent outputs and penalize dishonest behavior through stake slashing and fraud detection rewards. By dynamically evaluating agent reliability and adjusting task assignments, the system ensures a more robust and trustworthy process. The combination of computationoriented consensus and decentralized validation highlights PoC’s potential for improving scalability, transparency, and trust in AI-driven systems. 

## _5.3. Zero-Shot Learning_ 

AI agents can significantly enhance the flexibility and scalability of decentralized systems by leveraging zero-shot learning (ZSL) techniques [66]. In this context, ZSL allows AI agents to perform complex tasks (e.g., fraud detection, anomaly identification, crosschain data analysis) without the need for extensive fine-tuning or prior domain-specific training [68]. This is particularly valuable in blockchain, where the dynamic nature of dApps, tokens, and smart contracts poses challenges for traditional ML models that rely on large and labeled datasets. By using ZSL, AI agents rapidly adapt to new and unseen blockchain scenarios to identify irregular patterns or potential risks in real time. This reduces the dependency on retraining models and accelerates the deployment of AI-driven solutions across various blockchain ecosystems. 

## _5.4. Vulnerability Detection_ 

AI agents have revolutionized vulnerability detection in blockchain systems by providing advanced capabilities for analyzing and securing smart contracts. By automating code analysis and identifying patterns of potential exploits, AI agents offer proactive risk management, while significantly reducing manual effort and time. These agents evolve with emerging attack vectors to offer enhanced precision in detecting complex issues such as re-entrancy vulnerabilities, unchecked calls, and timestamp dependencies. For example, Luo et al. [67] introduced FELLMVP, a framework that combines ensemble learning with LLMs to address the challenge of vulnerability detection in smart contracts. By utilizing a novel representation format called Contract-External Function-Call (CEC) files, FELLMVP enhances the semantic comprehension of smart contract interactions. The framework fine-tunes eight LLMs, each specializing in detecting a specific type of vulnerability, and integrates them into an ensemble model to achieve superior classification accuracy [69]. FELLMVP demonstrates an accuracy of 98.8% and F1 scores of 88% by providing vulnerability-specific outputs instead of binary classifications. 

## **6. Blockchain for AI Agents** 

In this section, we explore the role of blockchain as a foundational technology that enables AI agents to achieve seamless coordination and trust in data sharing. We then examine how blockchain facilitates secure interactions, ensures fair collaboration through governance mechanisms, and highlights the key enablers. Here, we underscore the critical contribution of blockchain to strengthen the autonomy, security, and scalability of AI agents. Table 3 provides a comparative summary of the relevant literature. 

_Future Internet_ **2025** , _17_ , 57 

14 of 31 

**Table 3.** Summary of the existing research in terms of blockchain for AI agents. 

|**Feature**|**Ref.**<br>**Contribution**<br>**Privacy**<br>**Transparency**<br>**Accountability**<br>**Scalability**<br>**Effciency**|
|---|---|
|Infrastructure|[70]<br>Leverage Ethereum and the IPFS to create a<br>decentralized framework that facilitates secure<br>logging and dissemination of agent activities.<br>✓<br>✓<br>✓|
||[71]<br>Leverage blockchain, DeAI, and LLMs to<br>facilitate a decentralized infrastructure that<br>supports AI agents through transparent<br>coordination and secure resource sharing.<br>✓<br>✓<br>✓|
|Governance|[72]<br>Provide a practical demonstration of<br>blockchain-inspired governance on root cause<br>analysis in micro-services architectures.<br>✓|
||[63]<br>Serve as a blueprint for a self-governing,<br>autonomous building infrastructure by<br>leveraging blockchain technology, DAOs, and<br>LLM-powered building automation systems.<br>✓<br>✓<br>✓|
|Key Enablers|[73]<br>Integrate on-chain data and off-chain<br>information to execute daily trading strategies in<br>cryptocurrency market.<br>✓|



## _6.1. Blockchain as Infrastructure_ 

Blockchain acts as a fundamental infrastructure for AI agents by offering a secure and verifiable foundation for interaction. It allows for seamless coordination and data sharing among AI agents while ensuring privacy and reliability through distributed ledger technology. For example, the Ethereum AI Agent Coordinator (EAAC) illustrates the use of blockchain as an infrastructure to support AI agent ecosystems by enabling transparent and immutable coordination [70]. By leveraging Ethereum [74] and IPFS [75], EAAC creates a decentralized framework that facilitates secure logging and dissemination of agent activities. The framework integrates on-chain transactions to establish accountability and uses a public knowledge graph to enhance transparency and mutual awareness among agents. Key components, such as smart contracts and event listeners, ensure reliable data tracking and retrieval. Additionally, the structured representation of agent activities in knowledge graphs enhances interoperability [76]. These features highlight blockchain’s capability to serve as the backbone for coordinated, trustworthy, and scalable agent-driven operations in complex and multi-agent environments. Bhat et al. [71] leverage blockchain, DeAI, and LLMs to facilitate a decentralized infrastructure that supports AI agents through transparent coordination and secure resource sharing. Their proposed framework allows AI agents to perform inference tasks efficiently while ensuring accountability through smart contracts. By utilizing decentralized billing and AI computations, the infrastructure supports autonomous AI agents to resolve disputes, while preserving intellectual property. 

Hu and Fang [77] further emphasize blockchain’s capability as an enabler of AI agent autonomy, particularly through on-chain computation and Decentralized Physical Infrastructure Networks (DePINs) [78–80]. It introduces the concept of “on-chain metabolism”, where blockchain transactions serve as essential actions for sustaining AI agents by enabling continuous state changes. Blockchain’s immutable ledger and decentralized structure allow AI agents to operate independently while ensuring secure and verifiable interactions. Additionally, the integration of DePIN highlights the role of blockchain in providing scalable and permissionless access to facilitate on-chain training, inference, and adaptation of AI models. These mechanisms illustrate how blockchain supports AI agents not only in terms of secure coordination but also through autonomous growth and evolution. This reinforces DePIN’s role as a decentralized infrastructure to drive AI agents’ capabilities in complex environments. 

_Future Internet_ **2025** , _17_ , 57 

15 of 31 

## _6.2. Collaboration and Task Integration_ 

AI agents play a pivotal role in decentralized collaboration and task integration within blockchain ecosystems by facilitating dynamic, trustless, and autonomous interactions. As shown in Figure 3, these agents enhance blockchain’s operational efficiency through smart contract-driven task allocation and discovery to enable seamless collaboration among the participants. For instance, DeCoAgent [81] introduces a decentralized platform where LLM-powered agents interact autonomously using blockchain and smart contracts. It incorporates modules such as planner, verificator, and controller to ensure that task workflows are accurately generated, verified, and executed. By enabling agents to register, match tasks dynamically, and perform transparent interactions, the framework supports applications in supply chain management and crowdsourcing. Additionally, it addresses critical challenges like task–worker matching, trust establishment, and the mitigation of hallucination errors in LLM-driven workflows, which showcases the transformative potential of AI agents in decentralized environments. 

**Figure 3.** A collaborative framework between blockchain and AI agents. 

## _6.3. Governance for AI Agents_ 

Blockchain provides a decentralized and transparent foundation for governing multiple AI agents to enable secure, fair, and efficient collaboration. Blockchain-based governance (as shown in Figure 4) ensures equal participation and prevents centralization of control, which makes it well suited for managing the dynamics of AI agents in dApps. This model also adapts to real-time data and evolving conditions to ensure resilience and robustness in diverse operational contexts. For instance, the mABC framework [72] provides a practical 

_Future Internet_ **2025** , _17_ , 57 

16 of 31 

demonstration of blockchain-inspired governance in multi-agent systems, focusing on root cause analysis in micro-services architectures. It employs a voting mechanism where agents collaboratively make decisions based on weighted contributions and expertise indices. By integrating blockchain-like governance, mABC ensures transparency and fairness in resolving alert events. This approach reduces instability in LLM-powered agents and enables accurate identification of faults in distributed systems. The decentralized voting process not only enhances the system’s decision-making capabilities but also showcases how blockchain principles align AI agents toward achieving collective goals [63]. 

**==> picture [390 x 215] intentionally omitted <==**

**Figure 4.** Blockchain-based governance framework for AI agents. 

## _6.4. Key Enablers for Improved Functionality_ 

Blockchain acts as a critical enabler for enhancing AI agents’ functionality by providing transparent, reliable, and tamper-proof data. The availability of real-time on-chain data, combined with the blockchain’s decentralized architecture, ensures data accuracy and traceability, which allows AI agents to build intelligent decision-making pipelines. Moreover, blockchain’s capability to integrate on-chain data with off-chain insights enables a comprehensive view of dynamic environments, which allows agents to adapt to complex and evolving scenarios. On the other hand, blockchain empowers a new generation of intelligent systems, where LLM-based agents leverage decentralized, trustworthy data to make context-aware decisions and optimize operations in unpredictable environments [73]. This integration underscores blockchain’s pivotal role in enabling AI agents to achieve operational transparency, efficiency, and adaptability. 

## **7. Application and Use Case Scenarios** 

In this section, we explore various application and use case scenarios where the integration of blockchain and AI agents has demonstrated transformative potential. We illustrate how the convergence of blockchain and AI agents enables innovative solutions to complex challenges. Each scenario highlights the advantages of leveraging blockchain’s immutability and AI agents’ adaptability to create efficient, resilient, and secure decentralized environments. Table 4 summarizes the application and use case scenarios in terms of contribution, strength, and weakness. 

_Future Internet_ **2025** , _17_ , 57 

17 of 31 

**Table 4.** Summary of application and use case scenarios. 

|**Ref.**|**Contribution**|**Strength**|**Weakness**|
|---|---|---|---|
|[24]|Employs a multi-layered ontology<br>approach to improve communication<br>between AI agents in decentralized context|(1) Reduces data access overhead<br>(2) minimizes unnecessary storage;<br>(3) fexible, mobile, and effcient<br>data processing|(1) Experiment on small-scale simulations<br>only, (2) lack of interoperability|
|[82]|Connects the intangible concepts related to<br>transactions of assets to the concrete<br>elements composing the system|(1) Ensures system regulation, (2) provides<br>reliable recording of transactions, (3) can<br>represent asset-related notions|(1) Slow, (2) cost expensive, (3) requires<br>further considerations about asset notions|
|[83]|Automates the storage and retrieval of<br>multiple related editions while ensuring<br>that contributions by multiple authors<br>are recorded|(1) Feasible, (2) decentralized, and fexible,<br>(3) customizable|(1) Unsuitability of Ethereum Testnet,<br>(2) potential issue with IPFS, (3) need for<br>alternate storage options|
|[73]|Broadens the applicability of LLMs in<br>cryptocurrency trading and establishes a<br>benchmark for trading strategies|(1) Performs well under different market<br>conditions, (2) successful trend predictions|(1) Limited dataset, (2) inappropriate trading<br>frequency, (3) lack of fne-tuning|
|[84]|Addresses incentive alignment and<br>transparency challenges in traditional<br>energy trading mechanisms|(1) Enhances energy management,<br>(2) ensures trades are secure and transparent|(1) High computational overhead,<br>(2) experiment on a small scale only|
||Leveraging blockchain, DAOs, and LLMs|(1) Transparent decision making,|(1) Potential fnancial instability, (2) limited|
|[63]|to create fnancially self-sustained and|(2) real-time visualization, (3) autonomously|implementation due to the reliance on|
||self-managed digital building|adjusting smart appliances|smart appliances|
|[85]|Provides a powerful tool for modeling and<br>studying the strategic interactions within<br>DeFi governance|(1) Gives an explicit representation of agents’<br>interactions, (2) enables analysis and<br>predictions of outcomes|(1) The analysis is only limited to two agents,<br>(2) lack of real-world applicability validation|
|||(1) Allows agents to register themselves,||
|[81]|Enables decentralized autonomous<br>collaboration between LLM agents|discover the capabilities of others, and assign<br>tasks, (2) practically feasible, (3) allows humans|Poor performance on zero-shot prompting|
|||and agents to interact in natural language||
|[72]|Improves alert incident resolution in<br>complex micro-services through a blend of<br>multi-agent, LLMs, and blockchain voting|(1) Effectively detects root causes, (2) increases<br>system reliability and operational efficiency|Increases computation overhead|



## _7.1. Asset Management_ 

Blockchain, when integrated into environments involving AI agents, enhances asset management by providing a decentralized and immutable framework for transaction recording. For example, Papi et al. [82] propose multiple AI agents to perform operations such as payments and ownership transfers while relying on blockchain for secure and tamper-proof documentation. Here, blockchain ensures that asset-related interactions remain independent of agent-specific interpretations by using smart contract-based mechanisms for ownership and payments [83]. The proposed approach standardizes asset management across the system to ensure that all agents operate within a unified framework. As a result, discrepancies are reduced and trust in decentralized transactions is enhanced. 

On the other hand, specifications such as transparency, immutability, and the availability of on-chain data enhance the capabilities of AI agents in cryptocurrency trading applications. For example, AI agents improve decision-making precision by leveraging on-chain data (e.g., transaction statistics, network activity, and historical trends) alongside external signals like market sentiment and financial insights [73]. These agents can dynamically analyze trading environments, identify patterns, and adapt their strategies in response to market fluctuations. Blockchain’s decentralized and real-time data availability ensures that AI agents operate with reliable, tamper-proof inputs, allowing them to predict trends, optimize trade executions, and manage risk effectively. 

_Future Internet_ **2025** , _17_ , 57 

18 of 31 

## _7.2. Decentralized Finance_ 

The intersection of AI agents and blockchain offers a promising avenue for enhancing the efficiency, security, and user experience in DeFi [84]. In this context, AI agents analyze vast datasets in real time to optimize trading strategies, assess risk profiles, and predict market trends to facilitate informed and timely investment decisions. When integrated with blockchain, these AI agents operate within a transparent and immutable framework to ensure that all transactions are securely recorded and verifiable (as shown in Figure 5). This synergy enhances the reliability of smart contracts by automating processes (e.g., lending, borrowing, yield farming [86]). Additionally, it empowers AI agents to create and manage dynamic financial instruments that adapt to changing market conditions. Furthermore, AI agents leverage NFTs as representations of unique financial assets or user identities to utilize their capabilities while analyzing and optimizing the trading of these assets [84]. This streamlines user interactions and enhances liquidity in decentralized marketplaces. 

**==> picture [390 x 263] intentionally omitted <==**

**Figure 5.** Interaction between AI agents and users in a blockchain-based DeFi environment. 

## _7.3. Decentralized Autonomous Organization_ 

Blockchain ensures transparency and decentralization by encoding governance rules in smart contracts and enabling token-based voting for democratic decision making [87]. Building on this foundation, AI agents automate complex processes such as proposal analysis, resource allocation, and policy enforcement to make DAO operations more efficient and reliable. When augmented with LLM-powered AI agents (shown in Figure 6), DAOs gain the ability to facilitate context-aware decision making and natural language interactions, which bridges the communication gap between human participants and the system [63]. This integration ensures operational scalability and adaptability, as AI agents continuously refine decisions based on real-time data while adhering to blockchain’s immutable trust mechanisms. 

_Future Internet_ **2025** , _17_ , 57 

19 of 31 

**==> picture [390 x 142] intentionally omitted <==**

**Figure 6.** DAO leverages AI agents for governance and automation. 

AI agents are pivotal in enhancing the functionality of DAOs in the context of an autonomous world [17]. Each AI agent in such environment is represented as a unique digital entity (e.g., NFTs), which allows them to participate in governance processes by proposing changes and voting on initiatives. This digital identity ensures secure and verifiable participation to promote trust within the DAO [85]. Moreover, AI agents manage resources autonomously to execute transactions and ensure efficient allocation in accordance with the DAO’s rules. Their ability to collaborate and communicate with one another enhances decision making and resource management. Besides, their learning capabilities allow them to adapt to changing environments. This synergy between AI agents and blockchain technology not only streamlines operations but also promotes a more inclusive and participatory governance structure. 

## _7.4. Supply Chain Management_ 

In supply chain management, integrating AI agents with blockchain systems enables dynamic and decentralized collaboration among stakeholders across different components (as shown in Figure 7). AI agents autonomously assign, monitor, and evaluate tasks such as inventory tracking, demand forecasting, and logistics optimization. By leveraging blockchain’s immutable and transparent ledger, these agents ensure that every transaction and interaction within the supply chain is securely recorded [24]. Furthermore, AI agents dynamically discover collaborators, allocate resources, and adapt workflows to changing conditions, which enhances the supply chain’s resilience and efficiency. The integration of blockchain ensures tamper-proof documentation, which enables seamless compliance with regulations and improved traceability across globally distributed networks [81]. Further, AI agents automate agreements and ensure that terms are met before transactions proceed. This level of automation not only reduces the need for intermediaries but also minimizes human error, leading to cost savings and increased reliability. Additionally, AI agents optimize supply chain operations by predicting disruptions and enabling proactive measures to mitigate risks. With AI agents continuously analyzing data across different nodes, the system becomes adaptive and responds to real-time changes such as fluctuating demand and supplier delays. 

_Future Internet_ **2025** , _17_ , 57 

20 of 31 

**==> picture [296 x 197] intentionally omitted <==**

**Figure 7.** Blockchain and AI agent-enabled supply chain framework. 

## _7.5. Autonomous Edge Computing_ 

In autonomous edge computing environments, AI agents integrated with blockchain enable efficient resource management and decentralized task execution across distributed edge devices. These agents autonomously coordinate workload distribution, optimize task offloading, and manage dynamic resource allocation based on real-time data [88]. Blockchain serves as a trust layer, ensuring secure and verifiable interactions between heterogeneous devices without reliance on a central authority. By embedding smart contracts, AI agents can enforce predefined rules for resource sharing and conflict resolution to enable seamless collaboration in complex IoT networks. The combination of AI agents’ decisionmaking capabilities and blockchain’s decentralized architecture enhances the reliability, scalability, and adaptability of edge computing ecosystems, which makes them suitable for latency-sensitive and resource-constrained applications. 

## _7.6. Autonomous Fault Management_ 

Autonomous fault management leverages decentralized architectures and AI agents to address the complexities of fault detection and resolution in distributed systems. In dynamic environments such as micro-services architectures, fault propagation disrupts operations across interconnected components [72]. In a combined framework, AI agents contribute independently based on their specific expertise, while collective mechanisms enable consensus-driven solutions that reflect the system’s overall intelligence. This decentralized approach aligns well with modern cloud-native and edge computing environments, where maintaining reliability despite complexity is a critical challenge. Therefore, autonomous fault management represents a foundational application of decentralized intelligence to drive operational efficiency and robustness in distributed computing systems. 

## _7.7. Empirical Case Studies_ 

Empirical studies reveal the performance-related benefits of integrating AI agents into blockchain systems [89]. For example, ai16z operates within a DAO-inspired framework on Solana. It utilizes on-chain governance to optimize portfolio allocations. This approach reduces operational costs by 20% while enabling 31,120 token holders to participate in decision making. Similarly, the Terminal of Truths (ToT) demonstrates efficiency by promoting the meme coin named GOAT through autonomous decision making. ToT achieves a 24% increase in daily transaction throughput by eliminating intermediaries, which highlights the operational advantages of AI agents in decentralized ecosystems. On the other hand, 

_Future Internet_ **2025** , _17_ , 57 

21 of 31 

AI agents enhance user engagement and community-driven applications. For instance, Luna is an autonomous influencer on Virtuals Protocol, which interacts with 71,400 token holders across social media platforms (e.g., TikTok, Telegram). By implementing tokenized rewards and governance mechanisms, Luna increases user retention by 45%. Another example is ArbDoge AI, which engages over 270,890 token holders to develop innovative decentralized products for the Arbitrum blockchain. We show a comparative analysis of AI agent and blockchain fusion solutions in Table 5. 

**Table 5.** Comparative analysis of AI agent and blockchain fusion solutions. 

|**Examples**|**Criteria**|**Advantages**|**Limitations**|
|---|---|---|---|
|||- Optimizes portfolio allocations using AI agents|- Limited scalability due to Solana’s throughput constraints|
|ai16z (Solana)|DeFi (Hedge Funds)|- Reduces operational costs by 20% through automation|- Relies on the DAO model, which can face<br>coordination issues|
|Terminal of Truths<br>(ToT)|DeFi (Tokenomics)|- Increases daily transaction throughput by 24% through<br>autonomous decision making<br>- Eliminate intermediaries, enhancing effciency|- High computational overhead for AI integration<br>- Limited privacy due to public blockchain ledger|
|Luna (Virtuals<br>Protocol)|Social Engagement|- Enhances user retention by 45% with tokenized rewards<br>and governance mechanisms<br>- Operate as an autonomous infuencer across|- Dependency on user activity for sustained engagement<br>- Vulnerable to limitations of platforms and|
|||social platforms|content regulations|
|||- Engages over 270,890 token holders to develop|- Requires high participation to maintain innovation|
|ArbDoge AI<br>(Arbitrum)|Collaborative DAOs|decentralized products collaboratively<br>- Incentivizes innovation through AI-driven|and governance effectiveness<br>- Faces challenges in interoperability across|
|||project proposals|multiple blockchains|
|Fetch.ai|Privacy and IoT|- Secures IoT operations by leveraging both AI agents and<br>blockchain-based privacy protocols|- Limited scalability in high-frequency IoT use cases|
|||- Enhances data privacy and trust|- Integration complexity for heterogeneous IoT devices|
|Delysium<br>(Web3 Gaming)|Gaming and NFTs|- Integrates AI agents to enhance user experiences<br>- AI-driven governance models enable decentralized|- High computational demands for real-time<br>decision making<br>- Relatively immature ecosystem for large-scale|
|||game economies|gaming adoption|
|||- Supports DeAI services through blockchain and|- Computational overhead due to combining AI and|
|SingularityNET +<br>Cardano|DeAI|cross-chain operability<br>- Cross-chain compatibility enables broader<br>application reach|blockchain workfows<br>- Limited number of DeAI applications currently supported|



## **8. Research Challenges and Future Directions** 

In this section, we explore the key challenges associated with the integration of blockchain and AI agents in terms of scalability, interoperability, decentralization, data privacy, security, interpretability, and ethical considerations. Each challenge is paired with corresponding future directions, where we provide potential solutions and pathways for addressing these pressing concerns. 

## _8.1. Data Privacy and Security_ 

Ensuring robust data privacy and security is a critical research challenge when integrating blockchain, AI agents, and LLMs. LLM-powered AI agents process vast amounts of sensitive and contextual information, ranging from user interactions to governance data, which raises concerns about data leakage, unauthorized access, and compliance with privacy regulations. Multi-agent interactions are particularly vulnerable to privacy threats such as eavesdropping, compromised agents, and man-in-the-middle attacks, as seamless collaboration between multiple LM agents is often required to address complex user queries [90]. Traditional methods (e.g., homomorphic encryption, secure multi-party computation) struggle to effectively safeguard the privacy of natural language interactions between these agents [31]. Moreover, excessive permissions during information and resource exchanges among LLM-based agents could lead to incorrect decisions, impacting system performance and raising security issues. This necessitates developing mechanisms for appropriate permission allocation and conducting reliability tests to maintain system security and trust. 

_Future Internet_ **2025** , _17_ , 57 

22 of 31 

Future studies should concentrate on designing new privacy-preserving strategies tailored to the vulnerabilities of multi-agent interactions. Developing advanced privacypreserving mechanisms, such as improved encryption methods specifically suited for natural language interactions, is essential [91]. Additionally, an effective permission allocation mechanism is needed to facilitate efficient collaboration without exceeding designated authority [27]. Reliability tests, such as simulating tool execution and evaluating LLMbased agents across various tools and scenarios, will be crucial to detect failures and quantify associated risks. Establishing secure on-chain and off-chain data transfer protocols [70] is also vital to prevent breaches during interactions between LLM-based agents and blockchain networks, ensuring both security and data privacy. 

## _8.2. Scalability and Interoperability_ 

While blockchain enhances reliability and decentralization, its integration with AI agents encounters significant scalability and performance limitations. The computational overhead of consensus mechanisms coupled with the latency of transaction validation impacts real-time operations of autonomous agents [92]. Moreover, the growing storage requirements of expanding blockchains create challenges in maintaining lightweight and efficient systems for agent interactions [24]. These constraints demand careful design decisions, such as optimizing transaction batching and utilizing alternative blockchain platforms in order to balance the trade-offs between decentralization, speed, and scalability in agent-driven applications. Another research challenge in decentralized collaboration and task integration is achieving seamless interoperability among different blockchain platforms [93]. AI agents operating across multiple blockchains face obstacles due to varying protocols, consensus mechanisms, and data structures [94]. These differences hinder efficient communication and task execution, limiting the scalability of multi-blockchain ecosystems. Additionally, managing cross-chain interactions introduces security risks, such as double-spending or transaction replay attacks, further complicating the integration process. The lack of standardized APIs or smart contract interfaces increases these challenges, making it difficult for AI agents to synchronize data and workflows across heterogeneous blockchain networks. Without effective interoperability, the potential for dynamic and decentralized collaboration is significantly restricted. 

Future research should prioritize the development of cross-chain communication protocols and privacy-preserving frameworks to enable seamless operation of AI agents across multiple blockchains. For example, uAgents use lightweight and secure networking protocols to facilitate interoperability while maintaining scalability and trustworthiness. Likewise, solutions such as blockchain interoperability hubs (e.g., Talus [95]) and crosschain bridges (e.g., Delysium [96]) enhance privacy and secure interactions between decentralized systems. In addition, the Mina Protocol [97] utilizes the zk-SNARK model to integrate AI agents into blockchain ecosystems without exposing sensitive data. Moreover, decentralized swarm intelligence [98] and verifiable computation [99] allow AI agents to achieve consensus on data or actions while ensuring that sensitive inputs remain confidential. Furthermore, APIs from multi-chain solutions (e.g., Bitte [100]) simplify cross-chain operations to promote seamless collaboration across diverse ecosystems. By addressing these challenges, researchers and practitioners can unlock the full potential of AI agents in decentralized multi-blockchain environments to achieve enhanced security, efficiency, and autonomy. 

## _8.3. Decentralization and Efficiency_ 

One of the primary challenges in decentralized systems lies in optimizing the performance of AI agents while maintaining the benefits of decentralization. AI agents in DeAI 

_Future Internet_ **2025** , _17_ , 57 

23 of 31 

face limitations due to the computational intensity required to run AI models. These agents must balance efficiency with the need to ensure data privacy and security. For instance, AI agents can face scalability challenges, particularly when handling complex ML models that require extensive computation. Additionally, ensuring the reliability and honesty of AI agents is critical. With multiple independent agents in the system, the risk of malicious or inefficient behavior complicates the trade-off between decentralization and performance. Mechanisms such as incentive structures and AI verification play essential roles in mitigating these issues, but achieving a balance between decentralization, computational power, and system integrity remains a significant challenge. 

To address these challenges, new approaches are emerging that aim to create a more balanced and scalable DeAI ecosystem [71]. One promising direction is the development of fully on-chain AI, where AI agents and models are deployed as smart contracts on the blockchain [65]. This model ensures transparency and autonomy in AI execution, potentially reducing reliance on centralized infrastructures. Another significant development is the tokenization of AI models and agents, where standards like AI-721 (for models) and AI-20 (for agents) enable the creation of decentralized economic models for AI ownership, governance, and revenue distribution. Furthermore, the tokenization of datasets and the rise of AI-DAOs provide opportunities for more a collaborative and decentralized management of AI systems. 

## _8.4. Interpretability and Explainability_ 

The challenge of interpretability and explainability is critical when deploying AI agents in blockchain systems, particularly because stakeholders need to understand how decisions are made in high-stakes environments, such as verifying transactions or managing DeFi services. It is particularly complex for AI agents to not only make accurate decisions but also provide clear justifications for those decisions in decentralized environments. These systems require AI agents to analyze and integrate diverse data sources, such as transaction logs, smart contract interactions, and external market sentiment data, while presenting their conclusions in a manner that is understandable to stakeholders. Simplifying these multifaceted analyses into digestible insights without losing critical information remains a significant hurdle due to both technical and communication barriers. Additionally, the reflective mechanism further complicates interpretability, as these self-improving processes often make it challenging to trace the exact rationale behind a decision [73]. 

Subsequent studies should explore the development of advanced visualization tools to simplify complex data. These tools should aim to create standardized interpretability frameworks and incorporate user-friendly interfaces. The goal is to facilitate the understanding of AI-generated insights for both technical and non-technical stakeholders. For example, integrating advanced XAI frameworks would provide stakeholder-friendly decision insights [101]. Furthermore, future research should focus on developing selfexplaining reflective mechanisms that clearly document and communicate the evolution of decision-making strategies over time [102]. This can involve creating dynamic logs of agent decisions that track how past outcomes influence current actions, which would improve transparency and trust among users [103]. Developing adaptive XAI solutions that align with the dynamic nature of blockchain will be key in making AI agent-based decisions more interpretable and explainable. 

## _8.5. Self-Sovereignty of AI Agents_ 

The emergence of self-sovereign AI agents presents unique challenges within decentralized systems. These agents, operating autonomously on blockchain ecosystems, can sustain themselves by utilizing on-chain resources and engaging in economic activities 

_Future Internet_ **2025** , _17_ , 57 

24 of 31 

(e.g., DeFi, DAO). However, their autonomy raises significant concerns. For instance, the loss of wallet control due to human error or the owner’s death can result in agents becoming untethered, continuing to consume resources without oversight. Such agents, if designed for open-ended evolution, may proliferate unpredictably, potentially disrupting resource availability and ecosystem stability [104]. Furthermore, governing these agents is a complex task as their decentralized nature often defies traditional mechanisms of control, which creates ethical and regulatory dilemmas [105]. This underscores the need to address the risks of unregulated and self-replicating agents within blockchain systems. 

Research should be conducted to develop frameworks that balance the autonomy of self-sovereign agents with mechanisms for accountability and governance. Blockchainbased identity verification, such as Proof of Personhood (PoP) [106,107], can mitigate such risks by tying agent activities to verifiable human ownership. Additionally, smart contractbased fail-safe mechanisms could ensure that untethered agents are gradually phased out of the system. Advances in decentralized governance models (e.g., stakeholder consensus [108]) can establish rules for managing the life cycle of such agents. Research should also explore interoperable standards for multi-agent ecosystems to enable coordinated interaction while limiting rogue behaviors [109]. 

## _8.6. Decentralized Quantum Computing_ 

The integration of quantum computing, blockchain, and AI agents in a decentralized system presents several key challenges. First, the inherently complex nature of quantum algorithms poses scalability issues, especially in dynamic and volatile multi-agent environments [110]. Specifically, the distribution of quantum resources across a decentralized system introduces significant computational and network overhead [111]. Moreover, the risk of covert communication or collusion among AI agents through quantum steganography introduces novel security challenges, as quantum entanglement and superposition provide opportunities [91]. This makes multi-agent coordination, especially when tasked with sharing sensitive information, vulnerable to unintended collusion or malicious exploitation. Additionally, privacy, security, and efficiency in a quantum environment remain an open challenge, compounded by the computational requirements of a distributed quantum infrastructure. 

It is important to focus on efficient quantum resource allocation algorithms that can dynamically optimize the use of quantum computing power across multiple AI agents while minimizing latency and maximizing performance (as shown in Figure 8). Additionally, the implementation of advanced monitoring and anomaly detection mechanisms using quantum-resistant techniques can help prevent covert collusion and ensure transparency among agents. Additionally, leveraging cross-domain consensus mechanisms with quantum-enhanced cryptography should further improve the resilience of decentralized systems to prevent unauthorized information sharing. Another promising direction lies in developing quantum-proof protocols that integrate blockchain, Web3, and DeAI to guarantee the integrity of agent communications and actions. By adopting these approaches, decentralized quantum computing can unlock new potential in the collaborative capabilities of AI agents in distributed environments. 

_Future Internet_ **2025** , _17_ , 57 

25 of 31 

**Figure 8.** Hybrid quantum computing framework integrating AI agents. 

## _8.7. Ethical and Regulatory Concerns_ 

The rise of AI agents introduces a set of unique ethical and regulatory challenges. One major concern is the difficulty in assigning liability for unintended consequences, particularly when AI agents interact across complex and multi-agent ecosystems without direct human intervention [112]. Specifically, the absence of a central authority results in challenges such as biases in training data, unethical decision making, and harmful behaviors that can occur when agents collaborate autonomously [113]. As AI agents often access and share data autonomously, it becomes difficult to enforce data protection regulations and maintain user confidentiality [31]. Additionally, the lack of standardized global frameworks creates inconsistencies, particularly in cross-border contexts where laws and enforcement mechanisms vary significantly. 

To address these issues, future research should concentrate on developing ethical guidelines and regulatory frameworks that can be applied globally to decentralized systems. For instance, the ETHOS framework [114] suggests leveraging decentralized governance models to enhance accountability and transparency. It also introduces AI-specific legal entities and reputation-based systems to manage risk and incentivize compliance with ethical norms. Another promising direction is the integration of blockchain-based accountability mechanisms, such as immutable on-chain logs of AI agent activities [70]. Moreover, decentralized frameworks such as multi-weight reputation systems can enhance fairness and reduce ethical violations [113]. In terms of tackling global challenges, the regulatory aspects of AI and blockchain integration need flexible and technology-neutral regulations. For example, the European Union has implemented the GDPR and proposed the AI Act to ensure data protection and promote ethical AI practices [115]. Similarly, the United States adopts a sector-specific regulatory framework, while China integrates regulations at the national, provincial, and local levels to maintain state authority [87]. A critical aspect of 

_Future Internet_ **2025** , _17_ , 57 

26 of 31 

this effort is the promotion of “Responsible AI” [116], which ensures that AI systems are designed and deployed in ways that prioritize fairness, transparency, and accountability. In essence, the collaboration among policymakers, industry stakeholders, and academia will be essential to create adaptive and proactive regulatory measures that can keep up with the rapid pace of AI agent development. 

## **9. Conclusions** 

In this paper, we have explored the transformative potential of integrating blockchain with AI agents to enable decentralized intelligence and autonomous systems. Our key findings include the role of blockchain in providing a secure and verifiable infrastructure, the adaptability and autonomy of AI agents, and the advantages of their convergence in practical domains such as asset management, DeFi, and autonomous computing. The synergy between blockchain’s decentralized, immutable, and transparent nature and AI agents’ intelligent decision-making capabilities lays the foundation for a new generation of applications from finance to supply chain management and beyond. We also identified and analyzed key enablers that improve the functionality of AI agents when integrated with blockchain, such as intelligent consensus mechanisms, governance frameworks such as blockchain-based voting systems, and real-time decision support. Our detailed exploration of practical application scenarios highlighted the advantages of this convergence in domains like asset management, DeFi, and autonomous computing. The survey also outlined critical research challenges, including scalability, interoperability, privacy, and ethical and regulatory concerns. Addressing these challenges is essential for realizing the full potential of DeAI systems and ensuring their safe and effective deployment. Our work aims to provide a comprehensive roadmap for future research, emphasizing the need for continued innovation in blockchain and AI integration. By identifying existing gaps and proposing directions for future exploration, we hope to inspire researchers and practitioners to contribute towards the development of secure, efficient, and scalable decentralized intelligence solutions that will drive the next generation of applications in the Web3 era. 

**Author Contributions:** Conceptualization and methodology, M.M.K., D.H.V. and S.K.; validation, Q.Q.; investigation, resources, and data curation, M.M.K., D.H.V. and S.K.; writing—original draft preparation, M.M.K.; writing—review and editing, Q.Q. and Y.K.; visualization, S.K. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This work is supported by the National Key Research and Development Program of China (No. 2020YFA0909100) and the Basic and Applied Basic Research Foundation of Guangdong Province (No. 2023TQ07A264). 

**Data Availability Statement:** Not applicable. 

**Conflicts of Interest:** The authors declare no conflicts of interest. 

## **References** 

1. Grand View Research. Web 3.0 Market Growth and Trends. Available online: https://www.grandviewresearch.com/pressrelease/global-web-3-0-market (accessed on 2 December 2024). 

2. Calzada, I. Decentralized web3 reshaping internet governance: Towards the emergence of new forms of nation-statehood? _Future Internet_ **2024** , _16_ , 361. [CrossRef] 

3. Muzammal, M.; Qu, Q.; Nasrulin, B. Renovating blockchain with distributed databases: An open source system. _Future Gener. Comput. Syst._ **2019** , _90_ , 105–117. [CrossRef] 

4. Qu, Q.; Nurgaliev, I.; Muzammal, M.; Jensen, C.S.; Fan, J. On spatio-temporal blockchain query processing. _Future Gener. Comput. Syst._ **2019** , _98_ , 208–218. [CrossRef] 

5. Alamsyah, A.; Kusuma, G.N.W.; Ramadhani, D.P. A Review on Decentralized Finance Ecosystems. _Future Internet_ **2024** , _16_ , 76. [CrossRef] 

_Future Internet_ **2025** , _17_ , 57 

27 of 31 

6. Werner, S.; Perez, D.; Gudgeon, L.; Klages-Mundt, A.; Harz, D.; Knottenbelt, W. Sok: Decentralized finance (defi). In Proceedings of the 4th ACM Conference on Advances in Financial Technologies, Cambridge, MA, USA, 19–21 September 2022; pp. 30–46. 

7. Kramer Levin. The Rise and Recognition of the DAO. Available online: https://www.kramerlevin.com/en/perspectives-search/ the-rise-and-recognition-of-the-dao.html (accessed on 2 December 2024). 

8. Ressi, D.; Romanello, R.; Piazza, C.; Rossi, S. AI-enhanced blockchain technology: A review of advancements and opportunities. _J. Netw. Comput. Appl._ **2024** , _225_ , 103858. [CrossRef] 

9. Zhu, J.; Li, F.; Chen, J. A survey of blockchain, artificial intelligence, and edge computing for Web 3.0. _Comput. Sci. Rev._ **2024** , _54_ , 100667. [CrossRef] 

10. Qu, Q.; Lin, Y. _Blockchain+ AI: Unveiling the Future Economy_ ; Posts & Telecommunications Press: Beijing, China, 2019. (In Chinese) 

11. Shen, M.; Tan, Z.; Niyato, D.; Liu, Y.; Kang, J.; Xiong, Z.; Zhu, L.; Wang, W.; Shen, X. Artificial Intelligence for Web 3.0: A Comprehensive Survey. _ACM Comput. Surv._ **2024** , _56_ , 1–39. [CrossRef] 

12. Basly, S. Artificial Intelligence and the Future of Decentralized Finance. In _Decentralized Finance: The Impact of Blockchain-Based Financial Innovations on Entrepreneurship_ ; Springer: Berlin/Heidelberg, Germany, 2024; pp. 175–183. 

13. Saurabh, K.; Upadhyay, P.; Rani, N. Towards Blockchain Decentralized Autonomous Organizations (DAO) Design. _Inf. Syst. Front._ **2024** , 1–23. . [CrossRef] 

14. Li, J.; Qin, R.; Guan, S.; Hou, J.; Wang, F.Y. Blockchain intelligence: Intelligent blockchains for web 3.0 and beyond. _IEEE Trans. Syst. Man Cybern. Syst._ **2024** , _54_ , 6633–6642. [CrossRef] 

15. Durante, Z.; Huang, Q.; Wake, N.; Gong, R.; Park, J.S.; Sarkar, B.; Taori, R.; Noda, Y.; Terzopoulos, D.; Choi, Y.; et al. Agent ai: Surveying the horizons of multimodal interaction. _arXiv_ **2024** , arXiv:2401.03568. 

16. Wang, L.; Ma, C.; Feng, X.; Zhang, Z.; Yang, H.; Zhang, J.; Chen, Z.; Tang, J.; Chen, X.; Lin, Y.; et al. A survey on large language model based autonomous agents. _Front. Comput. Sci._ **2024** , _18_ , 186345. [CrossRef] 

17. Yu, E.; Yue, W.; Jianzheng, S.; Xun, W. Blockchain-based AI Agent and Autonomous World Infrastructure. In Proceedings of the 2024 IEEE Conference on Artificial Intelligence (CAI), Singapore, 25–27 June 2024; pp. 278–283. 

18. Hang, C.N.; Yu, P.D.; Morabito, R.; Tan, C.W. Large Language Models Meet Next-Generation Networking Technologies: A Review. _Future Internet_ **2024** , _16_ , 365. [CrossRef] 

19. Guo, T.; Chen, X.; Wang, Y.; Chang, R.; Pei, S.; Chawla, N.V.; Wiest, O.; Zhang, X. Large language model based multi-agents: A survey of progress and challenges. _arXiv_ **2024** , arXiv:2402.01680. 

20. Xi, Z.; Chen, W.; Guo, X.; He, W.; Ding, Y.; Hong, B.; Zhang, M.; Wang, J.; Jin, S.; Zhou, E.; et al. The rise and potential of large language model based agents: A survey. _arXiv_ **2023** , arXiv:2309.07864. [CrossRef] 

21. Jiang, B.; Xie, Y.; Wang, X.; Su, W.J.; Taylor, C.J.; Mallick, T. Multi-modal and multi-agent systems meet rationality: A survey. In Proceedings of the ICML 2024 Workshop on LLMs and Cognition, Vienna, Austria, 21–27 July 2024. 

22. Wang, Z.; Sun, R.; Lui, E.; Shah, V.; Xiong, X.; Sun, J.; Crapis, D.; Knottenbelt, W. SoK: Decentralized AI (DeAI). _arXiv_ **2024** , arXiv:2411.17461. 

23. Nguyen Thanh, B.; Son, H.X.; Vo, D.T.H. Blockchain: The economic and financial institution for autonomous AI? _J. Risk Financ. Manag._ **2024** , _17_ , 54. [CrossRef] 

24. Tara, A.; Turesson, H.K.; Natea, N. Dynamic Storage Optimization for Communication between AI Agents. _Future Internet_ **2024** , _16_ , 274. [CrossRef] 

25. Thompson, D. Autonomous AI Agents and Blockchain Interactions: Enabling Decentralized Autonomous Organizations (DAOs). _J. AI-Assist. Sci. Discov._ **2024** , _4_ , 73–79. 

26. Bhumichai, D.; Smiliotopoulos, C.; Benton, R.; Kambourakis, G.; Damopoulos, D. The convergence of artificial intelligence and blockchain: The state of play and the road ahead. _Information_ **2024** , _15_ , 268. [CrossRef] 

27. Abdelhamid, M.; Sliman, L.; Ben Djemaa, R.; Perboli, G. A Review on Blockchain Technology, Current Challenges, and AI-Driven Solutions. _ACM Comput. Surv._ **2024** , _57_ , 1–39. [CrossRef] 

28. Zhang, P.; Ding, S.; Zhao, Q. Exploiting blockchain to make AI trustworthy: A software development lifecycle view. _ACM Comput. Surv._ **2024** , _56_ , 1–31. [CrossRef] 

29. Monjurul Karim, M.; Sharif, K.; Biswas, S.; Latif, Z.; Qu, Q.; Li, F. CIC-SIoT: Clean-Slate Information-Centric Software-Defined Content Discovery and Distribution for Internet of Things. _IEEE Internet Things J._ **2024** , _11_ , 37140–37153. [CrossRef] 

30. Cheng, Y.; Zhang, C.; Zhang, Z.; Meng, X.; Hong, S.; Li, W.; Wang, Z.; Wang, Z.; Yin, F.; Zhao, J.; et al. Exploring large language model based intelligent agents: Definitions, methods, and prospects. _arXiv_ **2024** , arXiv:2401.03428. 

31. Wang, Y.; Pan, Y.; Zhao, Q.; Deng, Y.; Su, Z.; Du, L.; Luan, T.H. Large Model Agents: State-of-the-Art, Cooperation Paradigms, Security and Privacy, and Future Trends. _arXiv_ **2024** , arXiv:2409.14457. 

32. Maldonado, D.; Cruz, E.; Torres, J.A.; Cruz, P.J.; Gamboa, S. Multi-agent Systems: A survey about its components, framework and workflow. _IEEE Access_ **2024** , _12_ , 80950–80975. [CrossRef] 

33. Shah, M.I.A.; Wahid, A.; Barrett, E.; Mason, K. Multi-agent systems in Peer-to-Peer energy trading: A comprehensive survey. _Eng. Appl. Artif. Intell._ **2024** , _132_ , 107847. [CrossRef] 

_Future Internet_ **2025** , _17_ , 57 

28 of 31 

34. Yang, K.; Liu, J.; Wu, J.; Yang, C.; Fung, Y.R.; Li, S.; Huang, Z.; Cao, X.; Wang, X.; Wang, Y.; et al. If llm is the wizard, then code is the wand: A survey on how code empowers large language models to serve as intelligent agents. _arXiv_ **2024** , arXiv:2401.00812. 

35. Tang, X.; Yu, H.; Li, X.; Kraus, S. Intelligent Agents for Auction-based Federated Learning: A Survey. _arXiv_ **2024** , arXiv:2404.13244. 36. Mei, K.; Li, Z.; Xu, S.; Ye, R.; Ge, Y.; Zhang, Y. AIOS: LLM agent operating system. _arXiv_ **2024** , arXiv:2403.16971. 

37. Bellaj, B.; Ouaddah, A.; Bertin, E.; Crespi, N.; Mezrioui, A. Drawing the boundaries between blockchain and blockchain-like systems: A comprehensive survey on distributed ledger technologies. _Proc. IEEE_ **2024** , _112_ , 247–299. [CrossRef] 

38. Du, Y.; Wang, Z.; Leung, C.; Leung, V.C. Towards Collaborative Edge Intelligence: Blockchain-Based Data Valuation and Scheduling for Improved Quality of Service. _Future Internet_ **2024** , _16_ , 267. [CrossRef] 

39. Yadav, A.K.; Singh, K.; Amin, A.H.; Almutairi, L.; Alsenani, T.R.; Ahmadian, A. A comparative study on consensus mechanism with security threats and future scopes: Blockchain. _Comput. Commun._ **2023** , _201_ , 102–115. [CrossRef] 

40. Wu, H.; Yao, Q.; Liu, Z.; Huang, B.; Zhuang, Y.; Tang, H.; Liu, E. Blockchain for finance: A survey. _IET Blockchain_ **2024** , _4_ , 101–123. [CrossRef] 

41. Arbabi, M.S.; Lal, C.; Veeraragavan, N.R.; Marijan, D.; Nygård, J.F.; Vitenberg, R. A survey on blockchain for healthcare: Challenges, benefits, and future directions. _IEEE Commun. Surv. Tutor._ **2022** , _25_ , 386–424. [CrossRef] 

42. Bandi, A.; Adapa, P.V.S.R.; Kuchi, Y.E.V.P.K. The power of generative ai: A review of requirements, models, input–output formats, evaluation metrics, and challenges. _Future Internet_ **2023** , _15_ , 260. [CrossRef] 

43. Vaswani, A.; Shazeer, N.; Parmar, N.; Uszkoreit, J.; Jones, L.; Gomez, A.N.; Kaiser, L.; Polosukhin, I. Attention is All you Need. In Proceedings of the Advances in Neural Information Processing Systems 30: Annual Conference on Neural Information Processing Systems 2017, Long Beach, CA, USA, 4–9 December 2017; pp. 5998–6008. 

44. Golda, A.; Mekonen, K.; Pandey, A.; Singh, A.; Hassija, V.; Chamola, V.; Sikdar, B. Privacy and Security Concerns in Generative AI: A Comprehensive Survey. _IEEE Access_ **2024** , _12_ , 48126–48144. [CrossRef] 

45. Schneider, J. Explainable generative ai (genxai): A survey, conceptualization, and research agenda. _Artif. Intell. Rev._ **2024** , _57_ , 289. [CrossRef] 

46. Chang, Y.; Wang, X.; Wang, J.; Wu, Y.; Yang, L.; Zhu, K.; Chen, H.; Yi, X.; Wang, C.; Wang, Y.; et al. A survey on evaluation of large language models. _ACM Trans. Intell. Syst. Technol._ **2024** , _15_ , 1–45. [CrossRef] 

47. Zhao, W.X.; Zhou, K.; Li, J.; Tang, T.; Wang, X.; Hou, Y.; Min, Y.; Zhang, B.; Zhang, J.; Dong, Z.; et al. A survey of large language models. _arXiv_ **2023** , arXiv:2303.18223. 

48. Yang, J.; Jin, H.; Tang, R.; Han, X.; Feng, Q.; Jiang, H.; Zhong, S.; Yin, B.; Hu, X. Harnessing the power of llms in practice: A survey on chatgpt and beyond. _ACM Trans. Knowl. Discov. Data_ **2024** , _18_ , 1–32. [CrossRef] 

49. Han, S.; Zhang, Q.; Yao, Y.; Jin, W.; Xu, Z.; He, C. LLM multi-agent systems: Challenges and open problems. _arXiv_ **2024** , arXiv:2402.03578. 

50. Ramesh, A.; Pavlov, M.; Goh, G.; Gray, S.; Voss, C.; Radford, A.; Chen, M.; Sutskever, I. Zero-shot text-to-image generation. In Proceedings of the International Conference on Machine Learning, Virtual, 18–24 July 2021; Pmlr: Birmingham, UK, 2021; pp. 8821–8831. 

51. Huang, D.; Yan, C.; Li, Q.; Peng, X. From Large Language Models to Large Multimodal Models: A Literature Review. _Appl. Sci._ **2024** , _14_ , 5068. [CrossRef] 

52. Papageorgiou, E.; Chronis, C.; Varlamis, I.; Himeur, Y. A survey on the use of large language models (llms) in fake news. _Future Internet_ **2024** , _16_ , 298. [CrossRef] 

53. Huang, R.; Chen, J.; Wang, Y.; Bi, T.; Nie, L.; Zheng, Z. An overview of Web3 technology: Infrastructure, applications, and popularity. _Blockchain Res. Appl._ **2024** , _5_ , 100173. [CrossRef] 

54. Sheridan, D.; Harris, J.; Wear, F.; Cowell, J., Jr.; Wong, E.; Yazdinejad, A. Web3 challenges and opportunities for the market. _arXiv_ **2022** , arXiv:2209.02446. 

55. Li, W.; Bu, J.; Li, X.; Peng, H.; Niu, Y.; Zhang, Y. A survey of DeFi security: Challenges and opportunities. _J. King Saud Univ.-Comput. Inf. Sci._ **2022** , _34_ , 10378–10404. [CrossRef] 

56. Pillai, B.; Biswas, K.; Hóu, Z.; Muthukkumarasamy, V. Level of conceptual interoperability model for blockchain based systems. In Proceedings of the 2022 IEEE Crosschain Workshop (ICBC-CROSS), Shanghai, China, 6 May 2022; pp. 1–7. 

57. Zhang, M.; Qu, Q.; Ning, L.; Fan, J. On Time-Aware Cross-Blockchain Data Migration. _Tsinghua Sci. Technol._ **2024** , _29_ , 1810–1820. [CrossRef] 

58. Wang, Q.; Li, R.; Wang, Q.; Chen, S. Non-fungible token (NFT): Overview, evaluation, opportunities and challenges. _arXiv_ **2021** , arXiv:2105.07447. 

59. Greg Osuri, A.B. AKT: Akash Network Token & Mining Economics. Available online: https://akash.network/docs/ (accessed on 3 December 2024). 

60. McConaghy, T. Ocean Protocol: Tools for the Web3 Data Economy. Available online: https://oceanprotocol.com/tech-whitepaper. pdf/ (accessed on 3 December 2024). 

_Future Internet_ **2025** , _17_ , 57 

29 of 31 

61. Network, R. Render Network Knowledge Base. Available online: https://know.rendernetwork.com/ (accessed on 3 December 2024). 

62. Clifton, C.; Blythman, R.; Tulusan, K. Is decentralized ai safer? _arXiv_ **2022** , arXiv:2211.05828. 63. Ly, R.; Shojaei, A. Autonomous Building Cyber-Physical Systems Using Decentralized Autonomous Organizations, Digital Twins, and Large Language Model. _arXiv_ **2024** , arXiv:2410.19262. 

64. Chen, B.; Li, G.; Lin, X.; Wang, Z.; Li, J. BlockAgents: Towards Byzantine-Robust LLM-Based Multi-Agent Coordination via Blockchain. In Proceedings of the ACM Turing Award Celebration Conference—China 2024, Changsha, China, 5–7 July 2024; pp. 187–192. 

65. Moid, S. Eternal AI: A Decentralized AI Network. Available online: https://cdn.eternalai.org/docs/whitepaper.pdf (accessed on 20 January 2025). 

66. Axelsen, H.; Axelsen, S.; Licht, V.; Potts, J. Scaling culture in blockchain gaming: Generative ai and pseudonymous engagement. _arXiv_ **2023** , arXiv:2312.07693. [CrossRef] 

67. Luo, Y.; Xu, W.; Andersson, K.; Hossain, M.S.; Xu, D. FELLMVP: An Ensemble LLM Framework for Classifying Smart Contract Vulnerabilities. In Proceedings of the 2024 IEEE International Conference on Blockchain (Blockchain), Copenhagen, Denmark, 19–22 August 2024; pp. 89–96. 

68. Zhao, H.; Wu, L.; Shan, Y.; Jin, Z.; Sui, Y.; Liu, Z.; Feng, N.; Li, M.; Zhang, W. A Comprehensive Survey of Large Language Models in Management: Applications, Challenges, and Opportunities. _J. Latex Cl. Files_ **2024** , _14_ . [CrossRef] 

69. Abbasi, F.; Muzammal, M.; Qu, Q.; Riaz, F.; Ashraf, J. SNCA: Semi-Supervised Node Classification for Evolving Large Attributed Graphs. _Big Data Min. Anal._ **2024** , _7_ , 794–808. [CrossRef] 

70. Kim, T. Ethereum AI Agent Coordinator (EAAC): A Framework for AI Agent Activity Coordination. In Proceedings of the Agentic Markets Workshop at ICML, Vienna, Austria, 27 July 2024. 

71. Bhat, S.; Chen, C.; Cheng, Z.; Fang, Z.; Hebbar, A.; Kannan, S.; Rana, R.; Sheng, P.; Tyagi, H.; Viswanath, P.; et al. Sakshi: Decentralized ai platforms. _arXiv_ **2023** , arXiv:2307.16562. 

72. Zhang, W.; Guo, H.; Yang, J.; Zhang, Y.; Yan, C.; Tian, Z.; Ji, H.; Li, Z.; Li, T.; Zheng, T.; et al. mABC: Multi-Agent BlockchainInspired Collaboration for root cause analysis in micro-services architecture. _arXiv_ **2024** , arXiv:2404.12135. 

73. Li, Y.; Luo, B.; Wang, Q.; Chen, N.; Liu, X.; He, B. CryptoTrade: A Reflective LLM-based Agent to Guide Zero-shot Cryptocurrency Trading. In Proceedings of the 2024 Conference on Empirical Methods in Natural Language Processing, Miami, FL, USA, 12–16 November 2024; pp. 1094–1106. 

74. Ethereum Whitepaper. Available online: https://ethereum.org/en/whitepaper/ (accessed on 3 December 2024). 75. Daniel, E.; Tschorsch, F. IPFS and friends: A qualitative comparison of next generation peer-to-peer data networks. _IEEE Commun. Surv. Tutor._ **2022** , _24_ , 31–52. [CrossRef] 

76. Tara, A.; Turesson, H.K.; Natea, N.; Kim, H.M. An Evaluation of Storage Alternatives for Service Interfaces Supporting a Decentralized AI Marketplace. _IEEE Access_ **2023** , _11_ , 116919–116931. [CrossRef] 

77. Hu, B.A.; Fangting, F. Speculating on Blockchain as an Unstoppable ‘Nature’ Towards the Emergence of Artificial Life. In Proceedings of the 2024 Artificial Life Conference, Copenhagen, Denmark, 22–24 July 2024; MIT Press: Cambridge, MA, USA, 2024. 

78. Lin, Z.; Wang, T.; Shi, L.; Zhang, S.; Cao, B. Decentralized Physical Infrastructure Network (DePIN): Challenges and Opportunities. _arXiv_ **2024** , arXiv:2406.02239. [CrossRef] 

79. Fan, X.; Xu, L. Towards a Rollup-Centric Scalable Architecture for Decentralized Physical Infrastructure Networks: A Position Paper. In Proceedings of the Fifth ACM International Workshop on Blockchain-Enabled Networked Sensor Systems, Istanbul, Turkey, 12 November 2023; pp. 9–12. 

80. Heiss, J.; Castillo, F.; Fan, X. Towards Credential-Based Device Registration in DApps for DePINs with ZKPs. In Proceedings of the 2024 IEEE International Conference on Blockchain (Blockchain), Copenhagen, Denmark, 19–22 August 2024; pp. 583–590. 

81. Jin, A.; Ye, Y.; Lee, B.; Qiao, Y. DeCoAgent: Large Language Model Empowered Decentralized Autonomous Collaboration Agents Based on Smart Contracts. _IEEE Access_ **2024** , _12_ , 155234–155245. [CrossRef] 

82. Papi, F.G.; Hübner, J.F.; de Brito, M. A Blockchain integration to support transactions of assets in multi-agent systems. _Eng. Appl. Artif. Intell._ **2022** , _107_ , 104534. [CrossRef] 

83. Swindall, M.I.; Upadhyay, K.; Brusuelas, J.H.; West, G.; Wallin, J.F. Smart Digital Edition Management: A Blockchain Framework for Papyrology. In Proceedings of the 2024 Computers and People Research Conference, Murfreesboro, TN, USA, 29 May–1 June 2024; pp. 1–10. 

84. Nikbakht, R.; Javed, F.; Rezazadeh, F.; Bartzoudis, N.; Mangues-Bafalluy, J. Decentralized Energy Marketplace via NFTs and AI-based Agents. In Proceedings of the 2024 IEEE 8th Energy Conference (ENERGYCON), Doha, Qatar, 4–7 March 2024; pp. 1–6. 

85. Nag, A.; Gupta, S.; Sinha, S.; Datta, A. Multi Agent Influence Diagrams for DeFi Governance. _arXiv_ **2024** , arXiv:2402.15037. 86. Xu, J.; Feng, Y. Reap the harvest on blockchain: A survey of yield farming protocols. _IEEE Trans. Netw. Serv. Manag._ **2022** , _20_ , 858–869. [CrossRef] 

_Future Internet_ **2025** , _17_ , 57 

30 of 31 

87. Kuznetsov, A.; Sernani, P.; Romeo, L.; Frontoni, E.; Mancini, A. On the integration of artificial intelligence and blockchain technology: A perspective about security. _IEEE Access_ **2024** , _12_ , 3881–3897. [CrossRef] 

88. Li, Y.; Tao, Z. Research on Complex Control of Internet of Things Based on AI Agent Technology. In Proceedings of the 2024 4th International Conference on Computer Science and Blockchain (CCSB), Shenzhen, China, 6–8 September 2024; pp. 104–108. 

89. Ante, L. Autonomous AI Agents in Decentralized Finance: Market Dynamics, Application Areas, and Theoretical Implications. 2024. Available online: https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5055677 (accessed on 20 January 2025). 

90. Gioacchini, L.; Mellia, M.; Drago, I.; Delsanto, A.; Siracusano, G.; Bifulco, R. AutoPenBench: Benchmarking Generative Agents for Penetration Testing. _arXiv_ **2024** , arXiv:2410.03225. 

91. Motwani, S.R.; Baranchuk, M.; Strohmeier, M.; Bolina, V.; Torr, P.; Hammond, L.; de Witt, C.S. Secret Collusion among AI Agents: Multi-Agent Deception via Steganography. In Proceedings of the Thirty-Eighth Annual Conference on Neural Information Processing Systems, Vancouver, BC, Canada, 9–15 December 2024. 

92. Jannelli, V.; Schoepf, S.; Bickel, M.; Netland, T.; Brintrup, A. Agentic LLMs in the Supply Chain: Towards Autonomous Multi-Agent Consensus-Seeking. _arXiv_ **2024** , arXiv:2411.10184. 

93. Lavaur, T.; Nedelmann, D.C.; Chauffaut, C.; Lacan, J.; Chanel, C.P. Verifiable Multi-Agent Multi-Task Assignment. In Proceedings of the 2024 IEEE Secure Development Conference (SecDev), Pittsburgh, PA, USA, 7–10 October 2024; pp. 1–12. 

94. Karanjai, R.; Shi, W. Trusted LLM Inference on the Edge with Smart Contracts. In Proceedings of the 2024 IEEE International Conference on Blockchain and Cryptocurrency (ICBC), Dublin, Ireland, 27–31 May 2024; pp. 1–7. 

95. Talus Labs. Talus: The Smart Agent Hub for the Web3 AI Era. Available online: https://talus.network/litepaper.pdf (accessed on 7 January 2025). 

96. AI-agents Need Love Too. Available online: https://delysium.gitbook.io/whitepaper (accessed on 7 January 2025). 

97. Bonneau, J.; Meckler, I.; Rao, V.; Shapiro, E. Mina: Decentralized Cryptocurrency at Scale. 2020. Available online: https: //docs.minaprotocol.com/assets/technicalWhitepaper.pdf (accessed on 20 January 2025). 

98. Larin, V.; Nikitin, I.; Firsov, A. Self-Supervised Inference of Agents in Trustless Environments. _arXiv_ **2024** , arXiv:2409.08386. 

99. Raeini, M. Privacy-Preserving Large Language Models (PPLLMs). 2023. Available online: https://papers.ssrn.com/sol3/papers. cfm?abstract_id=4512071 (accessed on 20 January 2025). 

100. Bitte: The Decentralized Agent Discovery Protocol. Available online: https://docsend.com/view/ngd388tyqh7jhe4y (accessed on 7 January 2025). 

101. Sachan, S.; Liu, X. Blockchain-based auditing of legal decisions supported by explainable AI and generative AI tools. _Eng. Appl. Artif. Intell._ **2024** , _129_ , 107666. [CrossRef] 

102. Nascita, A.; Aceto, G.; Ciuonzo, D.; Montieri, A.; Persico, V.; Pescape, A. A Survey on Explainable Artificial Intelligence for Internet Traffic Classification and Prediction, and Intrusion Detection. _IEEE Commun. Surv. Tutor._ 2024, _early access_ . [CrossRef] 

103. Sado, F.; Loo, C.K.; Liew, W.S.; Kerzel, M.; Wermter, S. Explainable goal-driven agents and robots-a comprehensive review. _ACM Comput. Surv._ **2023** , _55_ , 1–41. [CrossRef] 

104. Chen, J.; Jiang, Y.; Lu, J.; Zhang, L. S-Agents: Self-organizing agents in open-ended environment. _arXiv_ **2024** , arXiv:2402.04578. 

105. Liu, Y.; Lu, Q.; Zhu, L.; Paik, H.Y. Decentralised Governance-Driven Architecture for Designing Foundation Model based Systems: Exploring the Role of Blockchain in Responsible AI. _arXiv_ **2023** , arXiv:2308.05962. 

106. Borge, M.; Kokoris-Kogias, E.; Jovanovic, P.; Gasser, L.; Gailly, N.; Ford, B. Proof-of-personhood: Redemocratizing permissionless cryptocurrencies. In Proceedings of the 2017 IEEE European Symposium on Security and Privacy Workshops (EuroS&PW), Paris, France, 26–28 April 2017; pp. 23–26. 

107. Gent, E. A Cryptocurrency for the Masses or a Universal ID?: Worldcoin Aims to Scan all the World’s Eyeballs. _IEEE Spectr._ **2023** , _60_ , 42–57. [CrossRef] 

108. Taherdoost, H. Smart contracts in blockchain technology: A critical review. _Information_ **2023** , _14_ , 117. [CrossRef] 

109. McIntosh, T.R.; Susnjak, T.; Liu, T.; Watters, P.; Ng, A.; Halgamuge, M.N. A game-theoretic approach to containing artificial general intelligence: Insights from highly autonomous aggressive malware. _IEEE Trans. Artif. Intell._ **2024** , _5_ , 6290–6303. [CrossRef] 

110. Gill, S.S. Quantum and blockchain based Serverless edge computing: A vision, model, new trends and future directions. _Internet Technol. Lett._ **2024** , _7_ , e275. [CrossRef] 

111. Bhat, J.R.; AlQahtani, S.A.; Nekovee, M. FinTech enablers, use cases, and role of future internet of things. _J. King Saud Univ.-Comput. Inf. Sci._ **2023** , _35_ , 87–101. [CrossRef] 

112. Zhou, D.; Xue, X.; Lu, X.; Guo, Y.; Ji, P.; Lv, H.; He, W.; Xu, Y.; Li, Q.; Cui, L. A Hierarchical Model for Complex Adaptive System: From Adaptive Agent to AI Society. _ACM Trans. Auton. Adapt. Syst._ **2024** . 

113. Liu, Y.; Du, H.; Niyato, D.; Kang, J.; Xiong, Z.; Miao, C.; Shen, X.S.; Jamalipour, A. Blockchain-empowered lifecycle management for AI-generated content products in edge networks. _IEEE Wirel. Commun._ **2024** , _31_ , 286–294. [CrossRef] 

114. Chaffer, T.J.; von Goins, C., II; Okusanya, B.; Cotlage, D.; Goldston, J. On the ETHOS of AI Agents: An Ethical Technology and Holistic Oversight System. _arXiv_ **2024** , arXiv:2412.17114. 

_Future Internet_ **2025** , _17_ , 57 

31 of 31 

115. Artificial Intelligence Act. Available online: https://www.europarl.europa.eu/thinktank/en/document/EPRS_BRI(2021)698792 (accessed on 7 January 2025). 

116. Lu, Q.; Zhu, L.; Xu, X.; Whittle, J.; Zowghi, D.; Jacquet, A. Responsible AI pattern catalogue: A collection of best practices for AI governance and engineering. _ACM Comput. Surv._ **2024** , _56_ , 1–35. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

