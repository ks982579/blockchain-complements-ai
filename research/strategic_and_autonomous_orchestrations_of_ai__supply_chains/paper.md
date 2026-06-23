## _Article_ 

## **Strategic and Autonomous Orchestration of Artificial Intelligence and Blockchain Integration for Supply Chains** 

**Funlade Sunmola[1,] * and George Baryannis[2]** 

- 1 School of Physics, Engineering and Computer Science, University of Hertfordshire, Hatfield AL10 9AB, UK 2 School of Computing and Engineering, University of Huddersfield, Huddersfield HD1 3DH, UK; g.bargiannis@hud.ac.uk 

- Correspondence: f.sunmola@herts.ac.uk 

## **Abstract** 

Global supply chains face intensifying pressures from disruption, regulatory complexity, and sustainability mandates, requiring a shift toward more resilient and adaptive coordination. While artificial intelligence (AI) and blockchain have been recognised as complementary enablers, their implementation remains largely fragmented, existing as isolated tools linked by manual data exchange rather than integrated, programmable logic. This paper addresses this orchestration gap by proposing the Dynamic Resource Orchestration Framework for AI-Blockchain Integrated Supply Chains (DROF-AIBC). Grounded in Resource Orchestration Theory (ROT) and Dynamic Capabilities Theory (DCT), the framework provides a theoretical foundation for the strategic and autonomous orchestration of digital resources. Unlike classic supply chain orchestration, which focuses on the linear coordination of physical assets and legacy systems, DROF-AIBC conceptualises an “intelligent conductor” as a coordination mechanism combining AI-driven analytics and smart contract-based execution. This mechanism supports the configuration, optimisation, and monitoring of resources in response to changing external signals, effectively closing the loop between analytical insights and verifiable execution. The paper further substantiates how this autonomous capability serves as a foundational roadmap for the Industry 5.0 paradigm, embedding human-centricity through Explainable AI (XAI) to provide a “provenance of logic”, promoting circular economy sustainability, and fostering systemic resilience in turbulent environments. The framework aims to provide both a theoretical foundation and a practical roadmap for orchestrating AI and blockchain to advance resilient, sustainable and adaptive supply chains. 

**Keywords:** autonomous orchestration; Industry 5.0; artificial intelligence (AI); blockchain; resource orchestration; dynamic capabilities; supply chain resilience; sustainability; explainable AI 

Academic Editor: Jianhua Zhu Received: 7 February 2026 Revised: 19 March 2026 Accepted: 27 March 2026 Published: 30 March 2026 

**Copyright:** © 2026 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license. 

## **1. Introduction** 

- Global supply chains are increasingly characterised by multi tier complexity, volatile demand, and frequent disruptions, increasing the need for transparency, responsive- ness and long term viability. Recent operations and supply chain research frames these challenges through resilience and viability lenses, emphasising the ability of supply networks to sense, absorb and adapt to shocks while sustaining performance over time [1,2]. Within this environment, digital transformation has accelerated, with artificial intelligence 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

_Systems_ **2026** , _14_ , 363 

2 of 33 

(AI) and blockchain frequently highlighted as core enablers of visibility, coordination and data-driven decision-making across organisational boundaries [3,4]. 

Blockchain provides a decentralised, immutable ledger and programmable coordi- nation via smart contracts, enabling verifiable provenance, auditability and cross party process automation. Studies show that permissioned ledgers and smart contracts can - support multi party workflows in sectoral settings such as construction and logistics, delivering trusted data sharing and transaction integrity [5]. Recent prototypes also explore blockchain-backed messaging and eventing to decouple participants, with latency/throughput trade-offs depending on network design [6]. In practice, implementers weigh permissioned versus permissionless options, consensus choices and on-chain versus off-chain data placement, alongside interoperability with prevailing supply-chain data standards. 

Complementing this, AI contributes predictive, prescriptive and optimisation capabilities for forecasting, quality assessment, anomaly detection and policy selection. Applications span planning, sourcing, manufacturing, logistics and risk, though barriers remain around data quality, integration and change management [3,7]. Advances in reinforcement learning and graph-based learning are enabling more adaptive decisions; prescriptive analytics is particularly important for resilience, linking scenario generation and policy evaluation [8]. In addition, causal machine learning enables scenario-based intervention planning beyond pattern recognition [9]. These developments introduce requirements for robustness, monitoring and explainability in operational use of AI. 

Emerging literature indicates that integrating AI and blockchain requires coupling intelligence (in the form of modelling, analytics, learning and optimisation) with trust (integrity, traceability, and explainability). However, early implementations typically focus on only a limited form of integration: AI models inform forecasts, inspections or risk scores while blockchain records traceability events and proofs, linked only through data exchange. - In such a setting, automation is limited and governance remains largely off chain, which constrains scalability and the ability to embed policies consistently across stakeholders [10]. 

The true transformative potential arises when model outputs are bound to programmable business logic, closing the loop between analytics and execution. A common motif is an off-chain analytics layer that emits signals to on-chain contracts via oracles and event streams, enabling auditable actions (e.g., releasing holds, adjusting contractual - terms, or triggering conditional payments), often alongside privacy preserving learning for compliance [11]. 

However, a significant orchestration gap persists in current literature and practice. While AI and blockchain have been recognised as complementary, their implementation remains largely fragmented and lacks architectural coherence. In many settings, AI models inform forecasts while blockchain records events, but these systems are linked only by manual data exchange rather than autonomous, programmable logic. This fragmentation prevents the “intelligent conductor” capability required for governance and real-time adaptation in turbulent environments. 

Furthermore, as industrial paradigms shift toward Industry 5.0, there is a growing need to embed human-centricity, sustainability, and systemic resilience into digital architectures. This paper addresses these gaps by proposing the Dynamic Resource Orchestration Framework for AI-Blockchain Integrated Supply Chains (DROF-AIBC). Grounded in Resource Orchestration Theory (ROT) and Dynamic Capabilities Theory (DCT), the framework provides a theoretical foundation for moving beyond linear coordination toward the strategic and autonomous orchestration of digital resources. 

These gaps motivate a focus on orchestration: an architectural and managerial layer - aligning data flows, decision policies and smart contract execution across heterogeneous systems, while supporting resilience, viability and governance at scale [2]. In operational 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

3 of 33 

terms, orchestration enables interoperability and workflow automation across diverse AI and blockchain services and existing systems (e.g., enterprise resource planning). Beyond integration, orchestration coordinates resource allocation and scheduling across components, supporting efficient execution of AI training and inference tasks. It also supports governance and compliance by enabling consistent policy enforcement across integrated systems, facilitating cross-system audit trails for regulatory scrutiny, and coordinating identity and access management mechanisms. 

Against this backdrop, this paper investigates how AI and blockchain can be strategically orchestrated to enhance supply chain performance and governance. To guide this investigation, the following key research questions are posed: 

- RQ1: What are the specific applications and underlying rationales for integrating AI and blockchain in supply chains, and what reported benefits and limitations characterise their current integration landscape? 

- RQ2: How do emerging themes in AI–blockchain integration redefine or contribute to the paradigm of supply chain orchestration? 

- RQ3: What robust conceptual framework can be proposed for the orchestrated integration of artificial intelligence and blockchain within supply chains to guide future research and practical implementation? 

We answer these questions by: (1) conducting a systematic literature review of AI– blockchain integrations in supply chains to map technologies, applications, rationales, benefits and limitations; (2) deriving and synthesising emerging themes that inform supply chain orchestration needs; and (3) proposing a Dynamic Resource Orchestration Framework - for AI-Blockchain Integrated Supply Chains (DROF AIBC) that links orchestration logics with adaptive capabilities, providing a theoretical model to guide both future research and practical implementation efforts. 

A fundamental contribution of this research is the distinction between classic supply chain orchestration and the orchestration of AI and blockchain resources. While traditional orchestration focuses on the linear coordination of physical assets and enterprise systems, the integration of AI and blockchain demands a more adaptive orchestration approach. This approach conceptualises an “intelligent conductor”: a coordination capability that configures resource bundles, such as AI models and smart contracts, to support automated governance and near real-time adaptation. By shifting the focus from manual data exchange to the orchestration of digital intelligence and decentralised trust, firms can achieve the transparency and resilience required for the Industry 5.0 era. Unlike existing AI–blockchain integration approaches, which predominantly focus on data interoperability or isolated use cases, this study advances an orchestration-centric perspective that explicitly links analytical outputs to programmable execution, grounded in Resource Orchestration Theory and Dynamic Capabilities Theory. 

The rest of this paper is organised as follows. Section 2 outlines the research methodology, including the systematic literature review and thematic analysis processes. Section 3 presents the results and analysis derived from the review while Section 4 synthesises emerging themes in relation to supply chain orchestration. Section 5 introduces the DROF-AIBC framework, including its theoretical underpinnings, components, and contribution to AI-blockchain focused orchestration. Section 6 reflects on the implications of the framework for research and practice. Finally, Section 7 concludes and highlights directions for future research. 

## **2. Methodology** 

This study adopts a three-phase methodology designed to explore the landscape of AI–blockchain integration, perform thematic analysis and formulate a novel orchestration approach. The methodology is depicted in Figure 1 and presented in this section. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

4 of 33 

**==> picture [378 x 55] intentionally omitted <==**

**----- Start of picture text -----**<br>
Phase 1: Phase 3:<br>Phase 2:<br>Systematic Novel Orchestration<br>Thematic Analysis<br>Literature Review Framework<br>**----- End of picture text -----**<br>


**Figure 1.** Three-phase methodology for exploring AI-blockchain integration. 

## _2.1. Systematic Literature Review_ 

To establish a strong theoretical foundation for the proposed orchestration approach, we first conducted a rigorous systematic literature review. This enables a structured understanding of how integrated AI–blockchain approaches have been considered and applied in supply chain management. 

The review was conducted following a predefined protocol aligned with established guidelines for systematic reviews outlined in the PRISMA statement [12] and in relevant software engineering, management and information systems research [13,14]. The process involves defining research questions, developing search and review strategies, extracting and analysing relevant data and information and synthesising findings. Research questions have already been discussed in the introduction; the systematic review aims to provide answers to RQ1 and RQ2 and inform the development of the novel orchestration framework for RQ3. 

## 2.1.1. Search Strategy 

The articles reviewed were identified and retrieved using automated search against databases of peer-reviewed literature. In determining the appropriate online databases to use, we considered the somewhat orthogonal criteria of coverage and rigour. According to Gusenbauer [15], Google Scholar has the largest disciplinary coverage across 56 bibliographic databases. However, Google Scholar, as well as other databases ranked highly in this survey also index non-peer-reviewed material, such as technical reports, preprints, and various forms of grey literature. Scopus was selected, instead, as one of the most highly ranked databases in terms of coverage that also allows filtering out non-peer-reviewed content. 

The search strategy closely follows recent systematic reviews in relation to supply chains [16]. We opted for a simple keyword combination, consisting of a conjunction of the terms “artificial intelligence”, “blockchain” and “supply chain”. The first keyword is chosen instead of a more specific term, such as “machine learning” to ensure that all AI approaches are included, not just those involving machine learning. The second keyword is chosen instead of the broader “distributed ledger technology” as the former is preferred to the latter in supply chain literature. Finally, the third keyword ensures that only works relevant to supply chains are returned. 

To determine the most appropriate year range, we conducted trial searches to determine the earliest point in which research implementing AI and blockchain integration in the context of supply chains was introduced. This was determined to be 2018, hence our search range was from 2018 to 2025. 

## 2.1.2. Review Strategy 

To determine which articles form part of the review corpus, a set of inclusion and exclusion criteria was applied consistently in a multi-step process. In the first step, we included only studies that (1) are written in English and (2) their document type in Scopus is article. The latter criterion is to maintain a consistent standard of peer-reviewed, comprehensive, original research published in journal articles, thereby excluding reviews, book chapters and conference papers. While we acknowledge that conference papers are important in rapidly evolving fields such as AI and blockchain, they were excluded to prioritise mature and fully developed contributions. 

In the second step, we excluded studies that are not relevant to the defined research questions: (1) studies that mention supply chains only in passing or as a potential appli- 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

5 of 33 

cation area; and (2) studies that do not involve both AI and blockchain in the proposed approach. At this point, we also considered to exclude studies that only offer a theoretical perspective on the integration without implementing an actual AI-blockchain integration solution but decided against this, to ensure that we gain insights into the theoretical underpinnings of the proposed approaches. Filtering during this second step was conducted based primarily on inspecting abstracts, only looking into full text if abstract inspection is not enough to make a decision. 

In the final step of the review strategy, the remaining studies were read in full to finalise the review corpus and also to prepare for the data and information extraction process that follows. Studies were divided between the authors, with one author validating the other author’s work to ensure accuracy and consistency. 

## _2.2. Data Extraction, Analysis and Synthesis_ 

The selected studies are grouped into two categories, technical and non-technical, based on whether they include an implementation of the proposed AI-blockchain integration approach. From the selected studies, relevant data and information were systematically extracted using a pre-agreed set of bibliographic information, as well as comparison criteria. The comparison criteria are as follows: 

- AI methodology used: Identifies the AI technique or algorithm applied (e.g., machine learning or deep learning algorithms, knowledge graphs, large language models). 

- Blockchain type and features: Specifies the nature of the blockchain implemented (e.g., public, private, permissioned) and any key features used, such as smart contracts, consensus mechanisms, or tokenisation. 

- Integration approach: Describes how AI and blockchain are technically or functionally integrated (e.g., AI operating on blockchain-secured data, blockchain recording AIgenerated outputs, AI triggering smart contracts). 

- Data type and source: Indicates the nature of data used in the study (e.g., real-world, simulated, or synthetic) and the dataset or source used. 

- Domain or sector: Identifies the supply chain domain in which the solution is applied (e.g., agriculture, pharmaceutical, humanitarian, logistics, food, manufacturing). 

- Use case: Specifies the particular supply chain problem addressed (e.g., traceability, risk detection, demand forecasting, quality control, sustainability, logistics optimisation). 

- • Rationale for AI-blockchain integration: Explains the justification for combining AI and blockchain, typically focusing on complementary strengths such as intelligence and trust, or prediction and traceability. 

- Benefits of integration: Summarises the reported advantages of the integrated system (e.g., enhanced transparency, improved decision-making, automation, fraud reduction). 

- Challenges or limitations: Notes any technical, organisational, or contextual barriers to implementation, such as data standardisation, infrastructure readiness, or interoperability. 

Note that, depending on the technical or non-technical nature of a paper, not all of the comparison criteria are applicable. Extracted data were analysed and compiled in the form of two comparison tables, one for non-technical and one for technical papers (Tables 1 and 2, respectively). Finally, the information compiled in the comparison tables was used as a basis for a thematic analysis following the six-phase method outlined by Braun and Clarke [17], conducted manually and collaboratively between the two co-authors. Specifically, phase 1 of this method involved familiarisation with the relevant data during the systematic review process. Phase 2 involved generating initial codes from the Benefits and Rationale data extracted in Tables 1 and 2, and the Underlying Rationale data from Table 3. In phases 3 through 5, these codes were collated into potential themes, reviewed against the dataset, and refined into the five definitive themes presented in Section 4. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

6 of 33 

**Table 1.** Conceptual Studies on AI-blockchain Integration. 

|**Citation**|**Integration**|**Domain**|**Use Case**|**Rationale**|**Benefts**|**Limitations**|
|---|---|---|---|---|---|---|
|Villegas-Ch et al. [18]|AI analyses IoT data, BC<br>logs events|Food logistics|Cold-chain monitoring|AI detects anomalies, BC<br>ensures traceability|Improved safety,<br>transparency and waste<br>reduction|Scalability, connectivity<br>and energy constraints|
|Wu et al. [19]|Not applicable|Not specifed|None|Data-driven visibility<br>and risk response|Improved agility,<br>coordination and<br>resilience|Integration complexity|
|Jha et al. [20]|AI predictions recorded<br>on BC|Agriculture|Traceability and yield<br>prediction|AI insights, BC<br>transparent transactions|Improved effciency,<br>trust and traceability|Infrastructure cost<br>and interoperability|
|Wang et al. [21]|AI decisions supported<br>by BC|Agriculture|Production management|AI forecasting with<br>secure decentralised data|Improved transparency,<br>sustainability and<br>effciency|Infrastructure cost and<br>scalability challenges|
|Xue et al. [22]|Not applicable|Not specifed|None|Improves automation<br>and data integrity|Combines data-driven<br>decision making and<br>transparency|Resource competition<br>and different data<br>requirements|
|Gao et al. [23]|Not applicable|Cross-sector supply<br>chains|Supply chain effciency|GenAI enhances AI–BC<br>synergy|Improves decisions and<br>transparency|Limited geographic<br>scope|
|Boudouaia and Ouchani<br>[24]|Not applicable|Not specifed|Multiple|AI enhanced by BC<br>security|Improves security and<br>transparency|Complexity and<br>interoperability issues|
|Adamashvili et al. [25]|AI supports decisions,<br>BC supports traceable<br>data recording|Wine|Inventory control, fraud<br>prevention|AI for data-driven<br>insights, BC for<br>auditability|Logistics optimisation<br>and fraud mitigation|Investment needs,<br>interoperability issues|
|Vijayapriya et al. [26]|AI forecasts, BC<br>validates transactions|Manufacturing|Multiple|AI enables decisions, BC<br>for traceability|Boosts security and<br>traceability|Single-frm perspective<br>only|
|Albaaji and Chandra[27]|AI analytics feed into BC<br>records|Agrifood|Multiple|Enhanced forecasting<br>and coordination|Increases trust and<br>accountability|Infrastructure and policy<br>gaps|
|Vanmathi et al. [28]|Conceptual only|Not specifed|Multiple|Reliable, trusted<br>decision-making|Enhances transparency<br>and traceability|Scalability,<br>interoperability,<br>legislation|
|Zulkarnain [29]|Conceptual only|Fish and seafood|Multiple|Data-driven,<br>tamper-proof monitoring|Enhances trust and<br>traceability|Cost, digital literacy and<br>capacity building|
|Chandra et al. [30]|AI for automation, BC<br>for secure storage|Agrifood|Multiple|Foster self-sustaining<br>farming ecosystems|Farmer empowerment<br>through traceable data|Infrastructure, digital<br>literacy, data privacy|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

7 of 33 

**Table 1.** _Cont._ 

|**Citation**|**Integration**|**Domain**|**Use Case**|**Rationale**|**Benefts**|**Limitations**|
|---|---|---|---|---|---|---|
|Ramachandran [31]|Conceptual layered<br>framework|Healthcare|Electronic Health<br>Records Management|AI enables data-driven<br>insights, BC ensures<br>immutability|Auditability, compliance,<br>transparency, reliability,<br>interoperability|Not specifed|
||AI learns from|||AI: forecasting and|Improved effciency,||
|Hong and Xiao [32]|blockchain-timestamped|Cross-sector|Not specifed|optimisation, BC:|transparency, traceability|Not specifed|
||data|||integrity and traceability|and accuracy||
|||||BC enables secure,|Enhanced farmer trust,||
|Chen et al. [33]|Not specifed|Agriculture|Food safety, fraud<br>prevention|transparent data, XAI<br>ensures decision|food traceability,<br>decision transparency,|No framework or<br>implementation|
|||||accountability|data governance||
|Wang and Yu [34]|AI generates data that is<br>processed and recorded<br>on BC|Not specifed|Financial and<br>operational effciency|AI supports predictive<br>analytics, BC guarantees<br>immutability|Improved transparency,<br>responsiveness, security|Not specifed|
|Tsolakis et al. [35]|AI data processing and<br>validation, BC ensures<br>security and<br>decentralisation|Food|Traceability,<br>transparency,<br>sustainability|AI ensures data<br>consistency, BC boosts<br>auditability and trust|Enables sustainability,<br>operational effciency<br>and coordination,<br>consumer trust|No real-world<br>deployment, focus on<br>operations-wise<br>implications only|
|Bechtsis et al. [36]|AI algorithms and data<br>stored securely in BC|Organic Food|Traceability, data<br>fragmentation,<br>certifcation|BC provides trustable<br>data infrastructure for AI<br>models|Improved data visibility,<br>traceability and<br>governance, enhanced<br>resilience|No real-world<br>implementation|
||AI for forecasting and|||BC ensures integrity and|Prevent data tampering,|No real-world|
|Das et al. [37]|anomaly detection, BC<br>for secure data|Covid-19 vaccine|Distribution monitoring,<br>logging and tracking|availability of sensitive<br>data, AI provides data|enable real-time<br>monitoring, support|implementation,<br>assumes existence of|
||provenance|||prediction|automated audit trails|cloud infrastructure|
||AI for demand|||BC enhances trust,|Reduced congestion and|Lack of real-time data|
|Rodríguez-Espíndola<br>et al. [10]|forecasting,<br>prioritisation, BC for|Humanitarian SCs|Disaster response|traceability, AI supports<br>real-time and|delays, improved donor<br>accountability, supplier|integration, technology<br>awareness, digital|
||traceability|||transparent decisions|coordination|infrastructure|
|Chidepatil et al. [38]|AI for data process, BC<br>for smart contract<br>automation|Plastics|Recycling and reuse,<br>waste management|AI improves sorting<br>precision, BC enables<br>secure, transparent B2B<br>coordination|Encourage industrial<br>uptake of recycled<br>plastics, enhance<br>visibility and<br>transparency, reduce cost|Only pilot-scale testing,<br>integration with legacy<br>recycling systems<br>challenging|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

8 of 33 

**Table 2.** Technically Implemented Studies on AI-blockchain Integration. 

|**Citation**|**AI**|**Blockchain**|**Integration**|**Dataset**|**Domain**|**Use Case**|**Rationale**|**Benefts**|**Limitations**|
|---|---|---|---|---|---|---|---|---|---|
|Zhu and Liu<br>[39]|AIoT sensors,<br>optimisation<br>models|Unspecifed|AIoT tracks, BC<br>verifes claims|Simulated/<br>numerical|Music|Sustainable<br>product<br>planning|Transparency<br>for eco-claims<br>and trust|Better pricing,<br>transparency<br>and effciency|No<br>real-world de-<br>ployment|
|Lv et al. [40]|AIoT,<br>behavioural<br>authentication|Consortium<br>multichain,<br>smart contracts|AIoT data<br>managed<br>through<br>multichain BC|Experimental<br>workloads|Healthcare|Hierarchical<br>access control|Privacy,<br>transparency,<br>monitoring|Faster access,<br>lower energy,<br>higher resilience|Scalability,<br>governance and<br>resource over-<br>head|
|Dahbi et al. [41]|GenAI, DRL<br>hyperheuristics|Blockchain<br>reputation<br>system, smart<br>contracts|BC reputation<br>guides AI<br>matching|Real-world<br>SFSC scenarios|Food|Supplier-<br>customer<br>matching|Trust-aware<br>dynamic<br>matching|Higher<br>satisfaction,<br>fewer failures,<br>moregains|Scalability and<br>computa-<br>tional overhead|
|Shelke et al. [42]|DMN, CNN|Dual-layer<br>blockchain,<br>smart contracts|AI key<br>generation<br>secures BC<br>sharing|Not specifed|Not specifed|Secure data<br>sharing and<br>retrieval|Stronger key<br>management<br>and traceability|Lower latency,<br>higher accuracy<br>and trust|Needs extensive<br>training data|
|Chen et al. [43]|RL, FL, GNN,<br>NSGA-II|Permissioned<br>semantic<br>blockchain|BC coordinates<br>AI scheduling<br>and resilience|Bosch, textile,<br>demand<br>forecasting|Manufacturing|Scheduling and<br>disruption<br>forecasting|Effciency,<br>resilience,<br>secure<br>coordination|Better<br>coordination,<br>transparency<br>and resilience|High tuning and<br>resource de-<br>mands|
|||Public Binance|AdaBoost|Indian||Blood|Secure,|High accuracy,|Interoperability,|
|Ch et al. [44]|AdaBoost|Smart Chain,|predicts, BSC|Government|Healthcare|availability and|transparent|low fees, strong|gas fees, lim-|
|||smart contracts|logs transactions|blood bank data||traceability|blood allocation|traceability|ited deployment|
|Khanna et al.<br>[45]|MARL, LLM,<br>GNN, DDPG|Private<br>Ethereum, smart<br>contracts|Agents<br>negotiate, BC<br>enforces SLAs|Synthetic<br>shipment data|Food|Fruit cold-chain<br>logistics|Dynamic<br>resilient<br>coordination|Lower spoilage,<br>energy use and<br>emissions|Synthetic data,<br>no deployment|
|||||||||Better||
|Fatorachian and<br>Kazemi [46]|Predictive<br>analytics, linear<br>regression|Unspecifed|AI predicts, BC<br>secures<br>emissions data|Company A<br>records, Carbon<br>Cloud|Cold chain<br>logistics|Emissions<br>tracking|Data integrity,<br>optimisation<br>and compliance|monitoring,<br>transparency<br>and|Single-case<br>study only|
|||||||||optimisation||
|Soy and<br>Balkrishna [47]|GAN, CNN|Unclear|CNN<br>classifcation<br>output logged<br>in BC|MedMNIST|Pharma|Drug<br>counterfeiting|High accuracy,<br>secure<br>provenance|Traceability and<br>secure audit<br>trail|GAN training<br>complexity and<br>scalability|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

9 of 33 

**Table 2.** _Cont._ 

|**Citation**|**AI**|**Blockchain**|**Integration**|**Dataset**|**Domain**|**Use Case**|**Rationale**|**Benefts**|**Limitations**|
|---|---|---|---|---|---|---|---|---|---|
|Sunmola et al.<br>[48]|KG, ML, LLM|Permissioned,<br>with smart<br>contracts|KG stored on<br>BC, ML-based<br>decisions|LODHalal,<br>Halal<br>ingredients|Food|Halal<br>certifcation and<br>compliance|data-driven<br>insights,<br>transparency|Reduce manual<br>burden, increase<br>trust|Limited<br>validation of<br>proposed<br>framework|
|Alhazmi et al.<br>[49]|LSTM, SHAP|Permissioned,<br>with smart<br>contracts|SHAP to<br>explain, BC for<br>logging|COVID-19<br>World<br>Vaccination|Healthcare|Crisis<br>management|AI: prediction,<br>BC:<br>transparency|High accuracy,<br>improved trust|Local<br>interpretation,<br>scalability|
|Ismail et al. [50]|Various ML<br>classifers|Public Ethereum<br>with smart<br>contracts|Data verifed on<br>chain, ML to<br>identify species,<br>quality|Real data<br>collected<br>through<br>bespoke device|Fish and<br>seafood|Traceability,<br>authenticity,<br>fraud<br>prevention|AI provides<br>verifcation, BC<br>ensures<br>traceability|Enhances trust,<br>compliance,<br>consumer safety|Scalability, cost<br>variability,<br>privacy|
|Abdelhamid<br>et al. [51]|Multi-Agent<br>System, PSO|Bespoke, using<br>agents instead<br>of traditional<br>miners|Tight<br>integration with<br>AI agents<br>operating<br>within BC|RT-IoT2022<br>synthetic<br>dataset|Not specifed|BC scalability in<br>data-intensive<br>IoT SCs|Context-aware,<br>autonomous<br>decision-<br>making|Scalable,<br>AI-managed BC<br>with faster data<br>access|No user<br>validation,<br>requires<br>fne-tuning|
|Wang [52]|CGAN,<br>Q-Learning,<br>Genetic<br>Algorithm|Private,<br>with smart<br>contracts and<br>PoA consensus|AI handles<br>forecasting, BC<br>secures<br>transactions|Synthetic|Healthcare|Demand<br>forecasting,<br>inventory<br>management|AI boosts<br>operational<br>effciency, BC<br>ensures trust|Higher<br>inventory<br>effciency, better<br>demand<br>estimation|No real-world<br>deployment,<br>resource<br>demands|
|Ismail et al. [50]|7 supervised ML<br>algorithms|Not specifed|Architecture<br>embedding ML<br>and BC|WUSTL-IIoT-<br>2021|Not specifed|Detecting and<br>mitigating<br>cyber-attacks|AI: threat<br>detection, BC:<br>immutability|Effective<br>detection of<br>low-frequency<br>attacks|Partial<br>unsuitability for<br>resource-<br>constraint IIoT|
|Dillenberger<br>et al. [53]|Various ML<br>regression<br>models|Permissioned,<br>with smart<br>contracts|BC hosts data<br>that AI is<br>trained on|Synthetic|Logistics|Delay<br>prediction,<br>compliance<br>checking|BC and AI<br>ensure privacy,<br>provenance,<br>trust|Enhanced data<br>integrity,<br>security and<br>traceability|BC platforms<br>not optimised<br>for complex<br>queries|
|Nasurudeen<br>and<br>Karthikeyan<br>[54]|RL, A*<br>heuristics|Public,<br>distributed<br>consensus,<br>smart contracts|AI for learning<br>policies, BC to<br>store|Simulated|Logistics|Delivery delays,<br>routing<br>optimisation|AI enables<br>learning, BC<br>ensures secure<br>records|Reduced service<br>time, data traffc,<br>full traceability|No real-world<br>deployment|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

10 of 33 

**Table 2.** _Cont._ 

|**Citation**|**AI**|**Blockchain**|**Integration**|**Dataset**|**Domain**|**Use Case**|**Rationale**|**Benefts**|**Limitations**|
|---|---|---|---|---|---|---|---|---|---|
|Eluubek kyzy<br>et al. [55]|ACO|Permissioned,<br>with smart<br>contracts|AI for<br>optimisation,<br>BC to<br>enforce/verify|Simulated|Agriculture|Order allocation,<br>quality<br>assurance|Security, trust,<br>fairness,<br>effciency|Fairer market<br>participation,<br>trusted data|No deployment,<br>infrastructure<br>needs|
|Dey et al. [56]|RL (Q-learning)|Customised,<br>with smart<br>contracts|BC records food<br>data, AI<br>optimises waste|Data collected<br>from food<br>management<br>app|Food waste|Waste<br>minimisation|BC ensures<br>traceability, AI<br>for adaptive<br>optimisation|Distributed<br>optimisation,<br>traceable food<br>provenance|Privacy and<br>security<br>concerns,<br>Q-learning costs|
|Zawish et al.<br>[57]|CNN|Private<br>blockchain,<br>using smart<br>contracts|AI for biomass<br>estimation, BC<br>for logging and<br>validation|Real biomass<br>estimation<br>dataset|Agriculture|Crop<br>traceability,<br>fraud detection|Real-time<br>decision making,<br>auditability|Improved<br>scalability,<br>privacy,<br>responsiveness|Mobility and<br>communication<br>constraints, no<br>deployment|
|Wang et al. [58]|CNN<br>(EffcientNet)|Customised<br>multilayer<br>architecture|AI analyses<br>signals, BC<br>certifes results|Real feld trial<br>data|Seafood|Fish provenance<br>verifcation|Automated<br>quality<br>assessment,<br>immutability|Increased<br>consumer trust<br>and<br>transparency|Limited<br>cryptographic<br>capacity<br>on-device|
|Zhang et al. [59]|ACO, Genetic<br>Algorithm|Customised<br>six-layer<br>architecture|AI: optimisation,<br>BC: certifcation|Simulated|Logistics|Ineffciencies,<br>limited<br>traceability|Optimisation,<br>security,<br>auditability|Improved<br>effciency,<br>increased<br>transparency|No real-world<br>deployment|
|Karamchandani<br>et al. [60]|Case-based<br>reasoning,<br>Fuzzy sets|Permissioned,<br>with smart<br>contracts|AI: analysis, BC:<br>storage|Simulated|Not specifed|Agility and<br>transparency|Accountability,<br>traceability and<br>trust|Customised<br>decision-<br>making,<br>immutability|No real-world<br>deployment|
|Liu et al. [61]|Genetic<br>Algorithm|Private<br>blockchain,<br>using smart con|AI optimises,<br>BC logs and<br>verifes|Simulated|Not specifed|Bullwhip effect,<br>uncoordinated<br>costing|Intelligent<br>optimisation,<br>secure data|Reduces<br>enterprise cost,<br>improves<br>traceability|No real-world<br>deployment|
|Bhatia and<br>Albarrak [62]|Faster R-CNN,<br>SHAP|Customised<br>with smart<br>contracts|AI analyses<br>images, BC logs<br>information|Synthetic|Food|Quality<br>detection, origin<br>traceability|Detect, trace and<br>explain risks|Improved<br>consumer trust<br>and food safety|No real-world<br>deployment|
|Lee et al. [63]|LSTM, Deep<br>Q-Learning|Public,<br>with smart<br>contracts|BC stores AI<br>output|Real sensor data|Agriculture|High fruit losses|Transparent<br>decision-<br>making|Improves<br>forecasting<br>accuracy|Dataset limited<br>to one type of<br>fruit|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

11 of 33 

**Table 3.** Applications of and rationale for AI-blockchain integration in supply chains, based on literature in Tables 1 and 2. 

|**Category**|**Specifc Applications**|**Underlying Rationale**|
|---|---|---|
|Transparency<br>& Traceability|- End-to-end product tracking (origin to consumer) [58,62]<br>- Tamper-proof audit trails [55,57]<br>- Real-time visibility across stakeholders [25,51]<br>- Compliance tracking in regulated sectors [18,20,47,48]|- Blockchain provides a decentralised, immutable ledger for traceability [20,57,58]<br>- AI supports real-time decision-making, prediction, and monitoring [51,63]<br>- The combination enhances transparency and synchronised visibility [25,62]|
|Fraud Detection<br>& Prevention|- Identifying suspicious activities (e.g., counterfeiting) [18,47,50]<br>- Secure audit logs for compliance [26,36]<br>- Authenticity verifcation at various points [27,33]|- AI detects anomalies, fraud, and verifes authenticity [33,47]<br>- Blockchain ensures tamper-proof provenance and auditability [36,50]<br>- Combined use increases compliance and reduces counterfeit risk [26,27]|
||- Real-time inventory optimisation [52,63]||
|Inventory<br>& Demand Forecasting|- Forecasting demand shifts [25,52]<br>- Reducing wastage and overproduction [56]<br>- Adaptive logistics and route reconfguration [54,59]|- AI enables forecasting and real-time optimisation [43,44,52,63]<br>- Blockchain secures planning and inventory data [54,59]<br>- Integration improves responsiveness and reduces waste [25,56]|
||- Demand prediction and resource planning [43,44]||
|Automated Processes<br>& Smart Contracts|- Automated payments based on delivery/quality verifcation [45,57,63]<br>- Self-executing compliance checks [48,49]<br>- Workfow automation across actors [51,53,54]<br>- Access control and data sharing automation [40]|- Smart contracts enable automatic execution of supply chain logic [40,45,48,49]<br>- AI triggers events based on data insights [52,53]<br>- Integration facilitates automated coordination and trust [51,54]|
||- Identifying potential disruptions (e.g., natural disasters) [23,37]||
|Risk Management<br>& Resilience|- Adaptive planning via real-time data [56,63]<br>- Strengthening network resilience [10,54]<br>- Explainable risk-driven decisions [48,62]|- AI forecasts disruptions and supports adaptive planning [19,23,37,43]<br>- Blockchain provides secure, traceable records for events [10]<br>- Explainable AI improves stakeholder trust and risk understanding [48,62]|
||- Dynamic coordination and disruption mitigation [19,43,45]||
|Sustainability<br>& Circular Economy|- Traceability for eco-compliance and conservation [29,38,39,46]<br>- AI-driven waste and resource optimisation [55,56]|- AI models support waste reduction and effciency gains [55,56]<br>- Blockchain documents lifecycle and eco-compliance data [29,38,39,46]<br>- Integration aligns digital infrastructure with circular goals [38]|
|Cybersecurity<br>& Data Integrity|- Detecting cyber-attacks and intrusions [35,64]<br>- Enhancing security in industrial IoT [35,51]<br>- Secure data sharing and access control [40,42]|- AI detects cyber-threats in real time [64]<br>- Blockchain enhances data integrity and traceability [35,40,42,51]<br>- Together, they improve security in decentralised environments [35]|



https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

12 of 33 

The outcomes of this process are presented in the following section, which analyses the selected studies and sets the foundation for thematic synthesis. 

## **3. Results and Analysis** 

The search was conducted on 16 March 2026 and yielded 1419 results. After following the review strategy described in the previous section, which is visualised in the PRISMA flow diagram of Figure 2, 48 studies remained. In this section, we present a detailed analysis of these studies. 

**==> picture [328 x 396] intentionally omitted <==**

**----- Start of picture text -----**<br>
Records identified from<br>Scopus search<br>(n = 1419)<br>Records screened by<br>Language (English) & Records excluded<br>Doc Type (Article) (n = 1057)<br>(n = 1419)<br>Records screened by Records excluded<br>Title & Abstract (irrelevant to SC)<br>(n = 362) (n = 183)<br>Full-text articles Records excluded<br>assessed for eligibility (missing AI/BC integration)<br>(n = 179) (n = 131)<br>Studies included in<br>qualitative synthesis<br>(n = 48)<br>**----- End of picture text -----**<br>


**Figure 2.** PRISMA flow diagram of the literature selection process. 

## _3.1. Conceptual Studies_ 

Table 1 compares conceptual contributions that explore the rationale for integrating AI and blockchain in supply chains. These studies span a range of domains, from agrifood, pharmaceuticals and healthcare to manufacturing. The studies present a variety of anticipated benefits, including improved traceability, data security, operational efficiency, and supply chain transparency. 

Several papers emphasise the importance of integration for addressing systemic challenges in agri-food and rural economies. For instance, Albaaji and Chandra [27], Zulkarnain [29] and Chandra et al. [30] propose conceptual frameworks for using AI to enhance prediction and automation, while blockchain ensures data immutability and provenance verification. Similar goals are considered by Adamashvili et al. [25] for the 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

13 of 33 

wine industry and Vijayapriya et al. [26] for manufacturing, where AI is positioned as an operational optimiser and blockchain as a trust enabler. Related ideas are explored by Jha et al. [20], who propose a blockchain-supported agricultural supply chain architecture where AI-driven analytics support yield prediction and operational decisions while blockchain maintains transparent traceability across supply chain actors. 

Other studies focus on sector-specific constraints. Xue et al. [22], Gao et al. [23], and Vanmathi et al. [28] explore readiness challenges in Chinese and Indian SMEs, noting the barriers to adoption due to digital infrastructure and legal uncertainties. Boudouaia and Ouchani [24], Hong and Xiao [32], and Ramachandran [31] propose integration frameworks tailored to data governance, interoperability, and legal traceability, respectively. Similarly, Wu et al. [19] present a conceptual framework integrating AI, IoT, big data analytics, and blockchain to enhance supply chain resilience, highlighting the importance of interoperability and coordinated data governance across stakeholders. 

Sustainability and resilience are also recurring themes. Bechtsis et al. [36], Das et al. [37], and Chidepatil et al. [38] argue that AI–blockchain integration can support circular economy models and ethical sourcing. Rodríguez-Espíndola et al. [10] focus on humanitarian logistics, proposing blockchain as a means to ensure transparency and accountability in relief operations. In food logistics, Villegas-Ch et al. [18] demonstrate how AI-based anomaly detection combined with blockchain-based event logging can improve cold-chain monitoring and reduce waste in perishable supply chains. 

Finally, several contributions, such as Wang and Yu [34], Chen et al. [33], and Tsolakis et al. [35], raise concerns about governance, explainability, and stakeholder coordination. Related governance and coordination challenges are also highlighted by Wang et al. [21], who discuss blockchain-enabled data infrastructures for smart agriculture where AI-driven decision support relies on trusted and decentralised data sharing mechanisms. All studies in Table 1 stop short of implementation, yet provide valuable insights into architecture, data strategy, and system design principles that inform later technical work. 

## _3.2. Technically Implemented Studies_ 

Table 2 compares studies that have implemented AI–blockchain integration to varying degrees, either through simulations, prototypes, or field trials. These studies demonstrate how AI models and blockchain infrastructures can be operationally combined to address traceability, fraud prevention, automation, and coordination in supply chains. We discuss these studies in groups, based on the type of AI method employed, revealing patterns in the types of problems addressed and the integration strategies adopted. 

Supervised machine learning and classification models form the foundation of several studies. Ismail et al. [64] use a suite of seven supervised ML classifiers to detect product anomalies, with outputs stored on blockchain for traceability. Dillenberger et al. [53] apply ML regression models to predict operational outcomes, linking model outputs to blockchain-based record-keeping. Karamchandani et al. [60] integrate case-based reasoning with rough fuzzy set theory, demonstrating the potential of hybrid symbolic-statistical approaches in secure pharmaceutical coordination. Similarly, Bhatia and Albarrak [62] employ Faster R-CNN in combination with SHAP explainability methods to support AIpowered inspection with blockchain audit trails. Related predictive modelling is also explored by Fatorachian and Kazemi [46], who use regression-based analytics to estimate emissions and operational performance in cold-chain logistics, and Ch et al. [44] who use AdaBoost to support blood availability prediction and allocation in blood supply chains. 

Deep learning and computer vision approaches are common in quality control and perishable goods tracking. Soy and Balkrishna [47] and Zawish et al. [57] use convolutional neural networks (CNNs) to automate visual inspection of food and meat products, linking 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

14 of 33 

results to blockchain for verification. Wang et al. [58] adopt EfficientNet CNNs for image classification of fish quality, pairing this with Hidden Markov Models and smart contract integration to coordinate downstream decisions. More specialised neural architectures are explored by Shelke et al. [42], who propose a deep learning-based framework for secure data sharing and retrieval in blockchain-enabled supply chains, using neural models to support encryption and data integrity mechanisms within the blockchain infrastructure. 

Reinforcement learning and deep Q-learning approaches are used for dynamic optimisation. Dey et al. [56] deploy Q-learning for anomaly detection in supply chain logistics, while Lee et al. [63] use Deep Q-Learning with LSTM to predict mango shelf life and coordinate export timing using blockchain voting mechanisms. Nasurudeen and Karthikeyan [54] extend this logic by combining reinforcement learning with A* heuristic search for adaptive logistics reconfiguration in disaster scenarios. Wang [52] go further by integrating CGAN, Q-learning, and genetic algorithms to optimise inventory allocation and blockchainencoded logistics decisions. Similarly, Chen et al. [43] employ reinforcement learning and federated learning techniques to improve manufacturing scheduling and supply chain resilience, with blockchain providing trusted coordination among distributed production actors. 

Evolutionary algorithms and swarm intelligence are used for optimisation tasks in multi-agent settings. Abdelhamid et al. [51] integrate a multi-agent system with Particle Swarm Optimisation (PSO) to coordinate logistics in decentralised IoT supply chains. Zhang et al. [59] use ant colony optimisation and genetic algorithms to optimise routing and transport. Eluubek kyzy et al. [55] apply standalone ant colony optimisation for path planning in edge-based sensor networks with blockchain coordination. Genetic algorithms are also employed by Liu et al. [61] for production planning in a blockchain-secured manufacturing scenario. Additional optimisation-driven integration is proposed by Dahbi et al. [41], who combine generative heuristics and reinforcement learning to dynamically match suppliers and buyers in short food supply chains while blockchain maintains trusted reputation records. 

Hybrid AI and knowledge-based systems are explored in more recent work. Sunmola et al. [48] combine knowledge graphs, machine learning, and large language models (LLMs) to generate explanations for AI decisions, integrating these into blockchain-based compliance records. Alhazmi et al. [49] use a blend of LSTM for temporal prediction and SHAP for model interpretability, aiming to increase trust in AI-supported decision-making recorded on-chain. More complex hybrid architectures are proposed by Khanna et al. [45], who integrate multi-agent reinforcement learning, graph neural networks, and large language models to support coordinated decision-making in blockchain-enabled cold-chain logistics systems. A related AIoT-driven integration is presented by Zhu and Liu [39], where IoT sensor data and AI optimisation models are combined with blockchain-based verification to support sustainable production planning and traceable supply chains. Similarly, Lv et al. [40] combine AIoT sensing, behavioural authentication, and multichain blockchain design to support hierarchical access control in green medical supply chain systems. 

This diversity of approaches reflects the flexibility of AI methods to complement blockchain in different use cases. However, it also highlights a lack of standardised integration strategies or architectural patterns. The majority of works in Table 2 follow an integration strategy that uses Blockchain as an immutable storage infrastructure for data fed into AI algorithms or outputs of these algorithms (which operate as analytical and predictive engines), with smart contracts used primarily for data validation. In a few cases (Sunmola et al. [48], Ismail et al. [50], Eluubek kyzy et al. [55], Lee et al. [63]), smart contracts are an integral part of a decision making process that is supported by AI, using data-driven insights to adapt smart contract behaviour accordingly. Finally, Abdelhamid et al. [51] propose a tighter integration, with AI agents operating within the 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

15 of 33 

blockchain infrastructure. This is akin to what is termed as Decentralised AI and Agentic AI in literature, distributing AI processing across multiple interconnected agents and using blockchain as the network that connects these agents, providing benefits such as data privacy, and increased robustness and resilience due to not having a single point of failure. 

Finally, it should be added that most implementations remain one-off prototypes or simulations, with limited discussion of generalisability, lifecycle maintenance, or multistakeholder deployment. This further motivates the need for orchestration frameworks that can guide integration across a variety of technical, sectoral, and organisational settings. 

## _3.3. Summary_ 

The comparative analysis presented in Tables 1 and 2 reveals a vibrant but fragmented research landscape. Conceptual papers articulate ambitious visions for AI and blockchain integration across domains such as agrifood, pharmaceuticals, healthcare, and manufacturing. These visions highlight anticipated benefits including transparency, security, and automation, and are often supported by high-level frameworks or scenario-based illustrations. However, few offer concrete mechanisms for operationalising integration or managing its ongoing adaptation in real-world settings. 

In contrast, technically implemented studies present a diverse array of AI methods, ranging from CNNs and LSTMs to reinforcement learning, ant colony optimisation, and multi-agent systems, often paired with blockchain-based provenance or contract enforcement. These papers provide valuable insights into what is technically feasible, but typically focus on narrow use cases or single-layer integration. End-to-end architectures, deployment in complex multi-actor environments, and attention to long-term governance and explainability are frequently lacking. 

Several overarching research gaps emerge from this synthesis. First, there is little convergence around integration architectures: different papers adopt bespoke data pipelines, model coupling mechanisms, and smart contract logic, with limited discussion of reusability or modularity and limited exploration of the range of capabilities that an AI-blockchain integration offers. Second, although some studies point toward sustainability, resilience, or human-centred design, these elements are not systematically embedded in integration logic. Third, most studies fail to address deployment barriers in a structured or proactive way, including but not limited to sectoral readiness, digital capability, or regulatory alignment. Finally, the lack of orchestration mechanisms that can coordinate AI and blockchain resources dynamically across operational, analytical, and governance layers is a persistent limitation. 

These observations motivate the need for a structured thematic synthesis that can guide future research and system design. In the following section, we identify five interrelated themes that represent both opportunities for innovation and critical requirements for integration. These themes provide a bridge between the descriptive insights of the literature and the prescriptive needs of orchestration. They also lay the groundwork for the proposed DROF-AIBC framework introduced in Section 5, which responds directly to the orchestration challenges identified in this systematic review. 

To consolidate recurring integration patterns, Table 3 summarises the main application areas of AI–blockchain integration in supply chains. Each entry is grounded in the literature reviewed in this section and categorised to reflect common themes such as transparency, risk management, and automation. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

16 of 33 

## **4. Emerging Themes** 

_4.1. Enhancing Transparency, Traceability, and Operational Intelligence Through AI–Blockchain Integration_ 

One of the most widely recognised opportunities in integrating AI and blockchain within supply chains lies in their combined potential to enhance transparency, traceability, and operational intelligence. These capabilities are especially important in complex, global, and multi-tier supply networks where a lack of real-time visibility, fragmented data systems, and inconsistent reporting contribute to inefficiencies, fraud, and regulatory risk. The literature shows that AI–blockchain integration enables more verifiable and data-driven decision-making across supply chain functions. 

In many studies, AI is employed to generate actionable insights from diverse data sources, supporting functions such as quality control, demand forecasting, and routing. For example, Soy and Balkrishna [47] apply convolutional neural networks (CNNs) for real-time quality inspection of agricultural products, reducing reliance on manual grading and improving consistency. Similarly, Wang et al. [58] integrates image classification (EfficientNet) with a Hidden Markov Model to assess product freshness. Across these applications, AI supports localised, data-driven decisions. 

Blockchain complements this intelligence layer by serving as a decentralised, tamperproof ledger that securely stores provenance, handling, and transactional data. For instance, Bhatia and Albarrak [62] link AI-based classification with QR-enabled traceability, while Zawish et al. [57] record IoT-based hygiene and temperature data on-chain. More integrated architectures combine AI-driven anomaly detection with blockchain-based event recording to support real-time coordination and verification [51,55]. These implementations are particularly evident in sectors such as food, agriculture, and pharmaceuticals, where traceability is closely tied to safety and compliance [10,47]. 

However, the literature also reveals that many AI-blockchain integrations remain isolated, with AI outputs often isolated from contractual logic or logistics workflows. Across studies, this reflects a consistent pattern of partial integration, where AI and blockchain are combined at a technical level but rarely coordinated at the architectural or decision-making level. Challenges such as data interoperability, technical infrastructure gaps, and lack of standardisation across platforms remain significant, as highlighted by Chidepatil et al. [38] and Albaaji and Chandra [27]. These constraints underscore the need for coordinated orchestration mechanisms that bridge the divide between AI intelligence and blockchain infrastructure, ensuring that insights are not only generated but acted upon in a verifiable and timely manner. 

Taken together, the reviewed studies demonstrate strong functional potential but limited architectural coherence, pointing to a growing demand for integration strategies that can synchronise distributed analytics and data governance functions across the supply chain. As such, any future orchestration approach will need to support the seamless flow of real-time insights into secure, auditable systems of coordination—offering the transparency and responsiveness required by modern, interconnected supply chains. 

## _4.2. Strengthening the Architecture of AI-Blockchain Integration_ 

The integration of AI and blockchain within supply chain contexts requires a carefully designed architectural foundation. Rather than simply co-deploying two technologies, effective integration demands interoperable layers that can facilitate real-time data exchange, distributed inference, secure provenance, and intelligent automation. The literature presents diverse architectural patterns, but most remain context-specific and lack generalisable integration models. For example, Adamashvili et al. [25] propose a wine supply chain architecture where AI supports forecasting and optimisation while blockchain 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

17 of 33 

ensures traceability, structured through layered data sharing and rule-based triggers. Similarly, Vijayapriya et al. [26] describe a manufacturing scenario where machine learning evaluates supplier performance and blockchain records transactions. However, across studies, these architectures are typically designed for specific use cases and lack generalisable integration patterns. 

From a more technical standpoint, Abdelhamid et al. [51] propose a multi-agent system (MAS) architecture in which AI agents and blockchain nodes coordinate within decentralised IoT environments, enabling autonomy, scalability, and modularity. Other work combines advanced AI techniques, such as CGANs and Q-learning, with blockchainbased execution layers [52]. Despite their sophistication, these approaches remain largely vertical-specific, reinforcing a pattern of bespoke rather than reusable architectural design. 

A key limitation across architectural proposals is the lack of standardisation. Many papers propose isolated solutions, often without reference to reusable components or reference models. This absence of architectural convergence is particularly visible in the variation in data pipelines, consensus mechanisms, and AI model deployment strategies. For instance, some studies integrate AI with permissioned blockchains [48], while others explore public Ethereum-based infrastructures [50], each bringing distinct trade-offs in terms of speed, scalability, and regulatory compliance. Coupling between AI and blockchain also varies widely, from off-chain inference with on-chain logging to tighter integration with smart contracts. 

Moreover, data synchronisation between AI and blockchain layers remains underaddressed. Real-time applications such as predictive routing or adaptive supplier selection require near-synchronous coordination between inference and validation processes. Without mechanisms for latency management, message coordination, and model traceability, system reliability is difficult to guarantee. While Abdelhamid et al. [51] hint at some of these capabilities through agent coordination logic, broader architectural blueprints remain underdeveloped. As a result, few studies demonstrate how AI–blockchain integration can be reliably sustained in real-time, multi-stakeholder environments. 

Collectively, the literature demonstrates significant architectural diversity but limited convergence toward unified integration approaches. This lack of convergence highlights the need for orchestration mechanisms that enable modular integration, abstraction of complexity, and alignment between analytical outputs and trusted execution. Future systems must support plug-and-play AI models, abstract blockchain configuration layers, and provide middleware services for aligning inference and trust workflows. 

## _4.3. Sustainability and Circular Economy as Strategic Drivers_ 

The integration of AI and blockchain in supply chains is increasingly shaped by sustainability and circular economy goals. These strategic drivers reflect a shift in focus—from solely optimising efficiency and responsiveness, to embedding long-term environmental, social, and ethical considerations into the design and operation of supply chain systems. The literature positions AI–blockchain integration as enabling traceability, accountability, and lifecycle awareness in high-impact sectors such as agrifood, seafood, and manufacturing. However, across studies, sustainability is often treated as an application outcome rather than a core architectural design principle. 

Zulkarnain [29] present a model for seafood supply chains where AI monitors sustainability metrics and blockchain ensures traceable documentation of sourcing practices. Similarly, Albaaji and Chandra [27] show how predictive AI supports resource efficiency in agrifood systems while blockchain provides auditable records of production and distribution. Across these studies, AI enables environmental monitoring and decision-making, while blockchain provides verifiable accountability. In more technical settings, Abdelhamid 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

18 of 33 

et al. [51] propose a decentralised architecture using multi-agent systems to coordinate sustainability goals, and Eluubek kyzy et al. [55] demonstrate provenance tracking combined with AI-based anomaly detection. Despite these advances, such architectures remain largely conceptual or context-specific, with limited evidence of scalable deployment across multi-tier supply chains. 

Despite this potential, several barriers persist. Energy consumption associated with blockchain consensus mechanisms remains a concern, particularly in public or hybrid infrastructures. While some studies propose lightweight or permissioned alternatives, systematic evaluation of sustainability trade-offs is limited. Additionally, explainability of AI-driven sustainability decisions is often insufficient, which may constrain adoption in regulated or high-stakes contexts. 

A common thread is the importance of data integrity and interoperability in enabling circular economy models. Closed-loop systems require reliable lifecycle data spanning sourcing, production, reuse, and recycling. AI supports lifecycle prediction and anomaly detection, while blockchain ensures traceability across actors. However, fully circular implementations remain rare, with most studies focusing on partial lifecycle stages rather than end-to-end circular flows [32,35]. 

Taken together, the literature highlights strong potential for sustainability integration but limited systematic embedding within architectural design. This gap highlights the need for orchestration mechanisms that integrate sustainability objectives directly into system design, rather than treating them as downstream outcomes. Such approaches must support coordination across environmental data, decentralised governance, and predictive predictive analytics to enable genuinely circular and sustainable supply chain systems. 

## _4.4. Barriers and Enablers of Real-World, Sector-Specific Implementation_ 

While AI–blockchain integration has generated considerable academic interest, translating these innovations into operational systems remains a complex challenge. The literature identifies sector-specific barriers and enablers that shape real-world implementation, reflecting differences in regulation, infrastructure, digital literacy, and supply chain complexity. These factors are highly contextual and unevenly distributed across industries and geographies. Understanding these factors is essential to designing AI–blockchain systems that are not only technically feasible but also socially and organisationally sustainable [65]. Across studies, these barriers are consistently identified but rarely addressed through coordinated implementation strategies. 

Several studies point to infrastructural and human-capital barriers in lower- and middleincome contexts, particularly in agrifood and manufacturing. Albaaji and Chandra [27] emphasise limited ICT infrastructure and digital literacy in agrifood supply chains, a barrier also emphasised in broader digital supply chain studies [3,5]. Similarly, Xue et al. [22] identify cost, talent shortages, and system fragmentation as barriers in SMEs. Across these cases, technical feasibility does not equate to implementability, viability or suitability. This highlights a recurring gap between proposed technical solutions and their practical applicability across different organisational contexts. 

In terms of regulatory and legal constraints, Vanmathi et al. [28] and Gao et al. [23] identify challenges in data-sharing rules, interoperability standards, and liability attribution, particularly in cross-border and multi-stakeholder environments. While some studies advocate formal alignment between smart contracts and legal frameworks [31], others propose participatory, pilot-based approaches. However, few studies provide concrete mechanisms for aligning legal, technical, and organisational requirements within a single implementation framework. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

19 of 33 

Enablers are similarly context-dependent. Established traceability practices and strong consumer demand support adoption in sectors such as seafood and wine [25,50], while IoT infrastructure provides critical data inputs for integrated systems [29]. These enablers, however, are highly context-dependent and not easily transferable across sectors with different infrastructural conditions. 

A consistent limitation across the literature is the lack of end-to-end implementation evidence. Most studies remain at proof-of-concept stage, with limited attention to long-term adoption or socio-technical integration. Integration between AI and blockchain components is often ad hoc, lacking standardised data models, interfaces, and governance structures. Albaaji and Chandra [27] caution that without a systemic approach to integration, these technologies may reinforce existing silos rather than dissolve them. This suggests that current approaches do not sufficiently address integration at the organisational and ecosystem level. 

In summary, the literature highlights significant socio-technical barriers and fragmented implementation approaches. This points to the need for orchestration mechanisms that support adaptive, context-aware deployment, modular integration, and alignment between technical, organisational, and regulatory dimensions. 

## _4.5. Risk, Resilience and Explainable Adaptive Decision-Making_ 

As supply chains face increasingly complex and interconnected risks, including geopolitical shocks, pandemics, cyber threats, and climate-related disruptions, there is growing demand for systems that can both detect risks and adapt in real time while maintaining stakeholder trust [66]. The literature suggests that the integration of AI and blockchain technologies can support the development of such systems by combining predictive analytics, secure coordination, and explainable decision-making [67]. These capabilities enhance resilience, but are rarely developed as part of integrated orchestration mechanisms. 

AI contributes to resilience by enabling early warning signals and dynamic adjustments. Das et al. [37] apply machine learning for cyber-attack detection, while Gao et al. [23] use AI to improve forecasting and production planning in vaccine logistics. These approaches demonstrate strong predictive capability but are typically confined to specific risk domains. 

Some studies go further by incorporating learning-based models that support adaptive decision-making. Nasurudeen and Karthikeyan [54] combine reinforcement learning with heuristic search for dynamic logistics reconfiguration, while Lee et al. [63] integrate federated learning and deep Q-learning with blockchain-based governance for distributed decision-making. These examples highlight the potential for decentralised, adaptive coordination, but remain largely experimental and not fully operationalised. 

Across these cases, blockchain is positioned as a foundational layer for trust and auditability. Rodríguez-Espíndola et al. [10] demonstrate its role in humanitarian logistics, while Zawish et al. [57] show how it secures real-time sensor data for risk monitoring. Across these applications, blockchain ensures verifiable records of events and decisions, but is rarely used for active decision execution. 

A distinctive feature of this theme is the emphasis on explainability. As AI-driven systems increasingly take on high-consequence decisions, the ability to understand and justify model outputs becomes essential for trust, adoption, and regulatory compliance. Bhatia and Albarrak [62] use SHAP explanations in food quality assessment, while Sunmola et al. [48] and Tsolakis et al. [35] emphasise human-centred design. Other studies advocate hybrid approaches combining AI with structured decision models [68,69]. Nevertheless, explainability is typically treated as an add-on rather than embedded within decision and execution workflows. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

20 of 33 

A key limitation across the literature is the lack of tightly coupled integration between risk detection, explainability, and coordinated execution. While AI is often capable of detecting risk, and blockchain can preserve records, the orchestration between these components, particularly in terms of real-time smart contract triggering, feedback loops, and cross-actor decision governance, remains limited. Furthermore, few systems explicitly tailor their explainability mechanisms to the cognitive and operational needs of supply chain decision-makers. 

Collectively, these themes reveal a consistent limitation across the literature: the absence of a unified orchestration logic capable of integrating adaptive capabilities, architectural requirements, and socio-technical considerations. This gap motivates the development of the DROF-AIBC framework, which is introduced in the following section. 

## **5. A Novel Autonomous Orchestration Approach** 

Section 4 highlighted five interrelated themes in the AI–blockchain literature: the - imperative to improve transparency and traceability across multi tier networks, the need for coherent integration architectures, the role of sustainability and the circular economy as strategic drivers, the socio-technical barriers and enablers shaping implementation in specific sectors, and the emerging focus on risk, resilience and explainable adaptive decision-making. Together, these themes point to a dual challenge: firms must simultaneously exploit the complementary capabilities of AI and blockchain and develop the organisational routines to adapt these technologies to turbulent environments. 

Building on these insights, we propose the Dynamic Resource Orchestration Framework for AI–Blockchain-integrated supply chains (DROF-AIBC). The framework provides a holistic and actionable model for understanding how firms can strategically leverage AI and blockchain to improve supply chain performance. It moves beyond a static view of technology adoption, emphasising the continuous and adaptive processes required to orchestrate these technologies in dynamic environments. 

DROF-AIBC is derived from the thematic findings presented in Section 4 and relies on established theoretical foundations that explain how resources can be managed and how organisations adapt to change. These foundations are introduced in the next subsection, followed by a detailed presentation of the DROF-AIBC framework and its distinctive contributions. 

## _5.1. Theoretical Underpinnings_ 

To ground the proposed DROF-AIBC framework, this subsection introduces two complementary perspectives from literature: Resource Orchestration Theory (ROT) and Dynamic Capabilities Theory (DCT). ROT provides insight into how resources are deliberately managed to create value, while DCT explains how organisations adapt these resources in dynamic environments. Together, they form the dual foundation for the orchestration of AI–blockchain systems. 

## 5.1.1. Resource Orchestration Theory (ROT) 

Resource Orchestration Theory (ROT) extends the Resource-Based View by emphasising that competitive advantage derives not simply from owning valuable resources, but from the managerial actions taken to configure them effectively [70,71]. ROT treats resources as dynamic assets whose value emerges through managerial action rather than static possession. These processes are typically described in three stages. First, structuring involves acquiring, accumulating, and divesting resources to align the firm’s portfolio with strategic goals. Second, bundling refers to integrating and combining resources to form capabilities, which may be stabilised, enriched, or newly created. Finally, leveraging is the 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

21 of 33 

deployment of these capabilities to exploit market opportunities and generate superior performance. Together, these stages frame orchestration as a continuous process rather than a one-off activity. 

Within supply chains, ROT has been used to explain resilience, innovation, and performance under uncertainty. Empirical studies show that structuring and leveraging digital resources enhances firms’ ability to withstand and recover from disruptions [72–74]. Similarly, the alignment of information technology capabilities with supply chain integration processes has been shown to improve operational and financial outcomes [75]. ROT has also been applied to the study of supply chain innovation, demonstrating how orchestrating information exchange and knowledge resources can lead to new product and service development [76]. These findings highlight ROT’s relevance in coordinating interdependent resources across organisational boundaries. 

ROT is particularly relevant for digital technologies such as AI and blockchain. AI supports resource reconfiguration through prediction and optimisation, while blockchain enables traceability, transparency, and trust [77]. Their value arises not in isolation but through integration into capability bundles that can be strategically deployed. ROT therefore provides a lens for understanding how firms acquire, combine, and leverage AI and blockchain resources within supply chains. 

However, ROT has limitations. It focuses on resource configuration but gives limited attention to how orchestration adapts under rapid change. In complex and volatile supply chains, there is a need to view orchestration similarly to coordination among autonomous, dynamic agents [78]. This limitation motivates the integration of Dynamic Capabilities Theory, which explicitly addresses adaptation and renewal in changing environments. 

## 5.1.2. Dynamic Capabilities Theory (DCT) 

Dynamic Capabilities Theory (DCT) complements ROT by focusing on how firms sustain competitive advantage in turbulent environments [79,80]. While operational capabilities support efficiency, dynamic capabilities enable continuous renewal and reconfiguration of resources. DCT is commonly articulated through three activities: sensing, seizing, and transforming. Sensing involves identifying opportunities and threats, seizing concerns mobilising resources to capture them, and transforming entails reconfiguring structures and routines to embed new capabilities. This cycle emphasises adaptation as the primary source of sustained advantage. 

In supply chain contexts, DCT is closely associated with agility, adaptability, and resilience. Agility reflects the ability to respond rapidly to short-term shocks, while adaptability enables more fundamental redesign of supply chain structures over time [81,82]. These together underpin supply chain ambidexterity, the balance between exploiting current efficiencies and exploring new opportunities [83]. Dynamic capabilities are also linked to resilience, as firms that sense disruptions early and reconfigure resources effectively maintain continuity and recover performance [84,85]. Such capabilities are particularly vital as global supply chains face technological, regulatory, and geopolitical uncertainty. 

Applied to AI and blockchain, DCT frames integration as an iterative and adaptive process. Firms must sense technological developments, seize opportunities through investment and capability building, and transform orchestration practices accordingly. For example, For example, adopting explainable AI or transitioning between blockchain architectures requires changes to governance, partnerships, and compliance structures. DCT therefore ensures that the orchestration of AI and blockchain remains responsive to evolving technical and institutional conditions. 

Despite its strengths, DCT is less specific than ROT about the actual managerial processes through which resources are arranged and exploited. It highlights the need for 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

22 of 33 

renewal and flexibility, but does not explain in detail how resources should be structured, bundled, or leveraged. This makes DCT highly complementary to ROT: the former provides the adaptive, evolutionary logic, while the latter specifies the micro-level managerial actions. 

Taken together, ROT and DCT form a dual theoretical foundation for orchestrating AI and blockchain in supply chains. ROT explains how resources are structured and deployed, while DCT captures how these processes are continuously renewed. This integration positions digital technologies as dynamic resources whose value depends on both effective configuration and ongoing adaptation, directly underpinning the DROF-AIBC framework, introduced next. 

## _5.2. The DROF-AIBC Framework: Linking Orchestration with Adaptation_ 

DROF-ABIS conceptualises digital transformation as a continuous interplay between strategic adaptation and resource management. At the strategic level, DCT explains how firms sense opportunities and threats, seize them through investment, and transform their resource base to remain aligned with changing environments. Empirical research shows that digital technologies such as AI and blockchain enhance supply chain resilience when embedded within this adaptive cycle [86]. At the operational level, ROT focuses on structuring, bundling and leveraging resources to create value. DROF-ABIS integrates these perspectives by embedding resource orchestration within the dynamic capabilities cycle, framing AI–blockchain integration as a continuous process rather than a one-off adoption. 

The framework therefore extends existing models by recognising that AI and blockchain are mutually reinforcing. Blockchain provides decentralised storage, smart contracts and immutable records that enable visibility, transparency and security across dispersed networks [5], while AI contributes analytical and decision-support capabilities that make sense of large volumes of data and trigger intelligent actions [3]. 

DROF-AIBC, as visualised in Figure 3 comprises five nested layers that translate adaptation and resource orchestration into actionable elements. The framework is described from the external environment inward, with each layer progressively translating environmental signals into coordinated resource orchestration and performance outcomes. 

The outermost layer is the External Dynamic Supply Chain Environment, representing volatile conditions such as market shifts, competitive pressures, geopolitical events, and regulatory changes, alongside increasing sustainability and resilience demands. This layer acts as a trigger that initiates the sensing–seizing–transforming cycle. 

The second layer is the Dynamic Capabilities Cycle, an overarching adaptive loop that guides a firm’s strategic response to the external environment. It begins with Sensing, where AI is used to detect and interpret signals such as emerging technologies, market changes, and risks. The cycle then moves to Seizing, where the firm makes strategic decisions and commits resources to address the opportunities or threats that were identified. Finally, Transforming, involves reconfiguring existing resources and capabilities, including AI models and blockchain architectures, to ensure the orchestration remains agile and effective over time. Crucially, this cycle functions as an iterative feedback loop in which the outcomes of prior orchestration are monitored and used to inform subsequent resource reconfiguration. 

The Strategic Enablers and Oversight Layer provides the foundational support for core orchestration activities. Within this layer, Coordination acts as the “intelligent conductor” of the framework, synchronising AI systems and blockchain networks through rules and optimisation strategies to support efficient resource allocation. This ensures that computational demands for AI are met efficiently across the decentralised infrastructure. Parallel to this, Governance ensures accountability by linking AI-generated insights to smart contract execution, enabling auditable and policy-compliant actions. It also supports 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

23 of 33 

transparency and regulatory alignment through Explainable AI (XAI) mechanisms that establish a “provenance of logic”. Finally, Digital Capabilities & Infrastructure Readiness addresses requirements related to investment, skills, and organisational preparedness. 

**Figure 3.** Dynamic Resource Orchestration Framework for AI–Blockchain-integrated supply chains (DROF-AIBC). 

At the heart of the framework lies the Resource Orchestration Core, which functions as the primary engine for autonomous orchestration by transforming static digital assets into self-configuring resource bundles. Structuring involves acquiring and organising AI and blockchain resources, while bundling integrates them into functional capabilities. This enables AI-driven logic to interact with programmable rules (in formal languages such as predicate or fluent calculus [87]), allowing automated, data-driven execution. The system analyses blockchain-secured data and triggers smart contracts based on verified events, embedding XAI to ensure traceable decision rationales. Leveraging then deploys these capabilities to achieve adaptive, efficient, and resilient operations. 

The innermost core is Supply Chain Performance & Competitive Advantage. This represents the ultimate goals of this process: improved operational performance, enhanced resilience, sustainability and competitive advantage. These outcomes emerge from the continuous alignment of adaptive capabilities, orchestration processes, and enabling mechanisms, demonstrating how AI–blockchain integration can deliver both operational and strategic value. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

24 of 33 

## _5.3. Mapping Themes to Framework Components_ 

The relationship between the themes identified in Section 4 and the framework components is summarised in Table 4, illustrating how the framework is grounded in the empirical insights from literature. 

**Table 4.** Mapping of Literature-Derived Themes to DROF-AIBC Framework Design Requirements and Components. 

|**Theme**|**Identifed Gap in Literature**|**Design Requirement for**<br>**Framework**|**Resulting Framework**<br>**Component**|
|---|---|---|---|
||Fragmented implementations;|Close the loop between|Governance Layer: Policy|
|Transparency|AI-Blockchain often linked only|analytical insights and|enforcement via|
||bymanual data exchange.|verifable execution.|smart contracts.|
||Lack of standardised patterns;|Need for harmonisation|Coordination Layer: AI acting|
|Integration|most studies focus on narrow|between heterogeneous digital|as a conductor for dynamic|
||use cases.|resources.|resource optimisation.|
||Often discussed conceptually|Environmental and ethical|Innermost Core: Competitive|
|Sustainability|but not systematically|goals must be key|advantage defned by|
||embedded in integration logic.|requirements.|sustainabilityand resilience.|
|Barriers|Limited implementability due<br>to infrastructure and digital<br>literacy gaps.|Must address socio-technical<br>readiness and investment costs.|Infrastructure Readiness:<br>Explicit support for digital<br>literacyand specialised talent.|
|Resilience|Lack of real-time adaptation<br>and trust.|An adaptive and transparent<br>feedback loop needed.|Dynamic Capabilities Cycle:<br>Sensing-Seizing-Transforming<br>powered by XAI.|



Each theme is interpreted as a design requirement and mapped to a corresponding framework component. In particular, the identified “orchestration gap” motivates the need for coordinated integration between analytical outputs and executable logic, reflected in the governance and coordination layers. Similarly, limitations in existing integration architectures motivate the structuring and bundling of digital resources, while sustainability, socio-technical constraints, and explainability requirements inform the definition of core performance objectives, enabling conditions, and adaptive capabilities. 

This mapping provides an explicit link between the literature and the proposed architecture, ensuring that the framework components are grounded in identified gaps. The resulting design captures the need for coordinated, transparent, and adaptable integration of AI and blockchain in supply chain contexts. 

## _5.4. Enabling Industry 5.0 Capabilities_ 

The transition from Industry 4.0 to Industry 5.0 reflects a shift from technical efficiency towards human-centric, sustainable, and resilient systems. While Industry 4.0 prioritised the technical “how” of automation, Industry 5.0 emphasises “to what end”, human wellbeing and environmental sustainability at the core of industrial performance. DROF-AIBC supports this transition by introducing autonomous orchestration as a mechanism for aligning digital capabilities with these objectives. 

The framework enables data-driven adaptation rather than static automation, allowing systems to adjust resource allocation and workflows based on evolving operational and environmental requirements. This capability directly underpins the Industry 5.0 resilience requirement. By linking AI-driven sensing with blockchain-enabled execution, the framework supports closed-loop responses to disruption. For instance, the system may use the sensing capability of the Dynamic Capabilities Cycle to interpret market shifts or geopoliti- 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

25 of 33 

cal risks. Instead of requiring human intervention for every adjustment, the autonomous orchestration layer can trigger smart contracts to re-route shipments, update insurance terms, or initiate conditional payments based on verified data insights, thereby closing the loop between analytics and execution. 

Human-centricity is addressed through the integration of explainability within orchestration processes. XAI is leveraged to generate decision rationales that are recorded on the blockchain, creating a verifiable “provenance of logic”. This enables human oversight at the level of validation rather than execution, ensuring that AI-supported decisions remain transparent, interpretable, and aligned with organisational and regulatory requirements. As a result, the framework supports a human-in-the-loop model consistent with Industry 5.0 principles. 

Sustainability is embedded through the integration of environmental monitoring and verifiable data management. AI supports the analysis of sustainability metrics across supply chain networks, while blockchain ensures traceability of lifecycle data. This enables systems to identify and respond to sustainability deviations, supporting resource reconfiguration toward compliant and efficient practices. In this way, the framework moves beyond passive reporting to actively reinforcing sustainable and circular supply chain behaviours. 

Overall, DROF-AIBC operationalises Industry 5.0 by linking adaptive capabilities with orchestrated resource management, ensuring that AI and blockchain integration supports resilience, human oversight, and sustainability in a unified manner. 

## _5.5. Propositions for Future Empirical Testing_ 

While DROF-AIBC is conceptual and architectural in nature, its components can be operationalised into a set of testable propositions to guide future empirical research. 

Proposition 1 (Integration and Coordination). Firms that adopt integrated AIblockchain architectures with autonomous orchestration mechanisms will exhibit higher levels of supply chain transparency and data consistency compared to those relying on loosely coupled or manual integration approaches. 

Proposition 2 (Governance and Trust). The use of XAI in conjunction with blockchainbased audit trails will positively influence stakeholder trust and perceived accountability in supply chain decision-making processes. 

Proposition 3 (Adaptive Capabilities and Resilience). Supply chains that implement autonomous orchestration supported by AI-driven sensing and rule-based execution will demonstrate improved responsiveness and resilience to external disruptions. 

Proposition 4 (Smart Contract-Enabled Execution). The alignment of AI-generated insights with smart contract-based execution mechanisms will enhance operational efficiency by reducing delays, manual intervention, and coordination overhead. 

Proposition 5 (Sustainability and Resource Optimisation). The integration of AIdriven monitoring with blockchain-enabled provenance tracking will support more effective sustainability practices, including resource optimisation and compliance with environmental standards. 

## **6. Discussion and Implications** 

This section discusses the implications of the proposed framework for both research and practice, followed by its limitations. 

## _6.1. Strategic Integration of AI, Blockchain, and Orchestration Theory_ 

Integrating ROT and DCT within DROF-AIBC addresses the challenge of coordinating digital technologies in complex and dynamic supply chains. The combination moves 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

26 of 33 

beyond theoretical alignment to provide a structured approach for achieving adaptive and sustained advantage under technological and environmental uncertainty. 

To understand this strategic integration, it is necessary to recognise complementary strengths and limitations of AI and blockchain. Blockchain provides decentralised trust, immutability, and smart contract automation, supporting provenance and accountability, but remains largely a passive infrastructure, unable to generate actionable insights [37,47,64]. AI enables prediction, optimisation, and decision support, but depends on data quality and lacks inherent auditability [23,32,52]. 

When combined, these technologies compensate for each other’s limitations. Blockchain strengthens AI by providing verifiable data provenance that enhances explainability, while AI extends blockchain functionality by enabling dynamic decision-making and automated contract execution [24,25,48]. However, without an overarching orchestration layer, such integrations remain limited in scope [22]. DROF-AIBC addresses this by providing the conceptual and operational mechanisms required to coordinate and align AI and blockchain resources in a systematic way. 

ROT provides the logic for structuring, bundling, and leveraging resources to create value [36], while DCT explains how firms must sense, seize, and transform to remain adaptive in rapidly changing contexts [63]. Within DROF-AIBC, ROT offers the operational logic of deployment, while DCT supplies the adaptive logic of continuous alignment. Together, they position orchestration as a continuous and adaptive process that links resource management with strategic responsiveness. The framework also provides a generalisable structure that can be adapted across sectors with different technological and regulatory constraints. 

## _6.2. Addressing Research Objectives and Gaps_ 

In line with the research questions in Section 1, this study achieves its objectives by systematically mapping the state of AI-blockchain integration in supply chains, extracting emerging themes, and developing a robust orchestration framework. Each research question is addressed as follows. 

For RQ1, the review identifies applications, rationales, benefits, and limitations of existing approaches. The evidence shows a fragmented landscape characterised by isolated implementations, weak integration, and limited consideration of sustainability and resilience. 

For RQ2, the synthesis of emerging themes highlights transparency, integration architecture, sustainability, socio-technical barriers, and resilience as key dimensions. Together, these shift orchestration from a technical integration problem to a strategic and adaptive capability. 

For RQ3, DROF-AIBC framework provides a structured response by integrating resource orchestration with dynamic capabilities. The framework establishes a continuous cycle linking resource configuration with adaptation, addressing fragmentation, lack of architectural coherence, and weak alignment with strategic objectives. 

## _6.3. Comparative Analysis of Orchestration Paradigms_ 

In the context of DROF-AIBC, autonomous orchestration can be formally defined as the coordinated configuration and continuous adaptation of AI and blockchain resources, where analytical outputs are systematically linked to programmable execution under predefined governance policies and auditable logic. Table 5 provides a comparison of autonomous orchestration in DROF-AIBC and existing orchestration approaches. 

Unlike existing orchestration approaches, DROF-AIBC uniquely integrates analytical decision-making, programmable execution, and explainability within a unified coordination mechanism, rather than treating them as loosely coupled or sequential components. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

27 of 33 

**Table 5.** Comparison of DROF-AIBC with existing orchestration approaches. 

|**Feature**|**Conventional**<br>**Orchestration**|**Cyber-Physical**<br>**Orchestration**|**Big Data**<br>**Orchestration**|**DROF-AIBC**|
|---|---|---|---|---|
|Primary Focus|Linear coordination of<br>physical assets and<br>legacy enterprise<br>systems|Integration of sensing<br>hardware with<br>operational control<br>systems|Managing<br>large-volume data<br>pipelines and<br>storage fows|Orchestration of<br>digital intelligence and<br>decentralised trust|
|Governance|Centralised and<br>often manual|Automated but<br>typically siloed within<br>specifc hardware|Rule-based data<br>management|Decentralised<br>self-executing<br>governance via<br>smart contracts|
|Adaptability|Reactive, dependent<br>on manual<br>reconfguration|Real-time response to<br>physical sensor<br>deviations|Scalable data<br>processing,<br>but limited<br>execution logic|Proactive<br>self-optimisation<br>through a Dynamic<br>Capabilities Cycle|
|Trust Mechanism|Relies on relational<br>contracts and<br>manual audits|Centralised system<br>security|Data integrity checks<br>within the pipeline|Immutable<br>“provenance of logic”<br>through XAI<br>and blockchain|
|Human Role|Operational manager<br>of physical assets|Supervisor of<br>automated<br>hardware loops|Data scientist or<br>systems architect|Strategic validator in a<br>human-centric<br>Industry 5.0 ecosystem|



## _6.4. Managerial Implications and Implementation Considerations_ 

The shift toward the autonomous orchestration defined by the DROF-AIBC framework provides a structured reference point for managers seeking to move beyond fragmented digital tools. By linking AI-driven insights with blockchain-based execution, firms can improve coordination, transparency, and responsiveness in supply chain operations. However, implementation introduces operational and governance challenges that require careful planning and capability development. 

A staged adoption pathway can support implementation. First, firms should focus on foundational readiness, including upgrading legacy infrastructure, improving data quality, and developing digital literacy. Second, integration and alignment involves connecting AI models with blockchain infrastructures through well-defined interfaces and governance rules. Third, operational scaling focuses on embedding these capabilities into end-to-end processes while monitoring performance, cost, and organisational impact. This staged approach helps reduce implementation risk and supports gradual capability development. 

Implementation also requires explicit consideration of trade-offs. While public blockchain infrastructures may enhance transparency, they can introduce latency and higher computational costs, making permissioned architectures more suitable in timesensitive contexts. Similarly, the energy consumption of blockchain consensus mechanisms must be evaluated against sustainability objectives, particularly in high-volume supply chain environments. Managers must therefore balance transparency, performance, and environmental impact when selecting architectural configurations. 

Finally, as AI–blockchain systems become more integrated, governance complexity increases, particularly regarding accountability and compliance. The framework addresses this by emphasising XAI and auditable data flows, enabling decision processes to be monitored and interpreted. This supports trust and facilitates alignment with regulatory and organisational requirements, while maintaining human oversight of AI-supported processes. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

28 of 33 

## _6.5. Limitations_ 

Several limitations should be acknowledged. We first discuss limitations of the systematic literature review approach, followed by limitations of the proposed framework. 

The literature search was restricted to the Scopus database. While Scopus is recognised for its high coverage of peer-reviewed literature [15], it is possible that relevant niche studies indexed solely in Web of Science or IEEE Xplore were excluded. Second, the thematic analysis, although conducted following the rigorous Braun and Clarke [17] protocol, inherently involves subjective interpretation by the researchers; no formal inter-coder reliability metric was calculated, although thematic coding was conducted iteratively with cross-checking to ensure consistency. Third, only journal articles were included, excluding conference proceedings that may capture emerging developments. Finally, no bibliometric or network-based analysis was performed, as the focus was primarily on thematic analysis. 

The framework is derived from systematic literature synthesis and remains conceptual, requiring empirical validation across different supply chain contexts. Moreover, the framework is conceptualised at the inter-organisational supply chain level, focusing on multi-actor coordination across distributed networks rather than intra-firm optimisation. Its applicability is most relevant to digitally mature or digitally transitioning supply chains where AI and blockchain capabilities can be meaningfully integrated. The framework does not assume full automation but instead defines a continuum of implementation, where decision support, semi-automated execution, and policy-driven automation may coexist depending on organisational readiness, regulatory constraints, and infrastructural conditions. 

## **7. Conclusions and Future Work** 

This paper has investigated the strategic integration of AI and blockchain in supply chains and proposed the Dynamic Resource Orchestration Framework for AI-Blockchainintegrated supply chains (DROF-AIBC). The framework synthesises ROT and DCT, providing both a theoretical foundation and a practical roadmap for orchestrating digital intelligence and decentralised trust infrastructures. By addressing the persistent orchestration gap, DROF-AIBC moves beyond siloed implementations and offers a structured, adaptive model that embeds sustainability, resilience, and explainability as primary concerns. Crucially, the study introduces the paradigm of autonomous orchestration, where AI-driven analytics and rule-based execution via smart contracts support the configuration and optimisation of supply chain processes within defined governance constraints. 

The study contributes theoretically by extending ROT and DCT to encompass digital resources and practically by offering managers a staged roadmap for integrating AI and blockchain into cohesive, autonomous workflows. In doing so, it positions these technologies not as auxiliary tools but as strategic assets central to the evolution of robust, resilient, and sustainable supply chains. The shift from manual data exchange to integrated, data-driven coordination provides the necessary technical and managerial infrastructure to realise the Industry 5.0 ideal of a human-centric yet highly autonomous industrial environment. 

While DROF-AIBC provides a comprehensive conceptual foundation, further empirical work is needed to validate and refine the framework. Potential future research directions include: 

- Empirical validation: Longitudinal case studies and simulation-based stress tests could examine how the proposed relationships (e.g., between orchestration, transparency, resilience, and efficiency) manifest in practice, providing evidence for the formulated propositions. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

29 of 33 

- Middleware and intelligent agents: Deeper exploration is required into orchestration layers that mediate between AI, blockchain, and legacy systems, enabling adaptive routing, trust negotiation, and decentralised decision-making. 

- Socio-organisational adoption: Future work should investigate how behavioural, cultural, and incentive-related factors shape the adoption of autonomous AI-blockchain infrastructures across multi-stakeholder supply networks. 

- Technical challenges: Research into privacy-preserving mechanisms and mechanisms for Explainable AI (XAI) within autonomous machine-led decisions remains critical for trust and compliance. 

Addressing these remaining gaps will be pivotal for refining the framework and significantly advancing the broader agenda of orchestrated, intelligent, and trustworthy supply chains. 

**Author Contributions:** Conceptualization, F.S. and G.B.; Formal analysis, F.S. and G.B.; Methodology, F.S. and G.B.; Project administration, F.S. and G.B.; Software, F.S. and G.B.; Supervision, F.S. and G.B.; Visualization, F.S. and G.B.; Writing—original draft, F.S. and G.B.; Writing—review & editing, F.S. and G.B. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This research received no external funding. 

**Data Availability Statement:** No new data were created or analyzed in this study. Data sharing is not applicable to this article. 

**Conflicts of Interest:** The authors declare no conflicts of interest. 

## **References** 

1. Ivanov, D.; Dolgui, A. Viability of intertwined supply networks: Extending the supply chain resilience angles towards survivability. A position paper motivated by COVID-19 outbreak. _Int. J. Prod. Res._ **2020** , _58_ , 2904–2915. [CrossRef] 

2. Ivanov, D.; Dolgui, A.; Blackhurst, J.V.; Choi, T.M. Toward supply chain viability theory: From lessons learned through COVID-19 pandemic to viable ecosystems. _Int. J. Prod. Res._ **2023** , _61_ , 2402–2415. [CrossRef] 

3. Sharma, R.; Shishodia, A.; Gunasekaran, A.; Min, H.; Munim, Z.H. The role of artificial intelligence in supply chain management: mapping the territory. _Int. J. Prod. Res._ **2022** , _60_ , 7527–7550. [CrossRef] 

4. Tiwari, M.K.; Bidanda, B.; Geunes, J.; Fernandes, K.; Dolgui, A. Supply chain digitisation and management. _Int. J. Prod. Res._ **2024** , _62_ , 2918–2926. [CrossRef] 

5. Wang, Y.; Chen, C.H.; Zghari-Sales, A. Designing a blockchain enabled supply chain. _Int. J. Prod. Res._ **2021** , _59_ , 1450–1475. [CrossRef] 

6. Lau, C.W.; Liu, J.; Ma, X.; Talluri, S. Performance analysis of a blockchain-based messaging system implementation for air cargo supply chains. _Int. J. Prod. Res._ **2025** , _63_ , 5428–5451. [CrossRef] 

7. Cannas, V.G.; Ciano, M.P.; Saltalamacchia, M.; Secchi, R. Artificial intelligence in supply chain and operations management: A multiple case study research. _Int. J. Prod. Res._ **2024** , _62_ , 3333–3360. [CrossRef] 

8. Smyth, C.; Dennehy, D.; Wamba, S.F.; Scott, M.; Harfouche, A. Artificial intelligence and prescriptive analytics for supply chain resilience: A systematic literature review and research agenda. _Int. J. Prod. Res._ **2024** , _62_ , 8537–8561. [CrossRef] 

9. Wyrembek, M.; Baryannis, G.; Brintrup, A. Causal machine learning for supply chain risk prediction and intervention planning. _Int. J. Prod. Res._ **2025** , _63_ , 5629–5648. [CrossRef] 

10. Rodríguez-Espíndola, O.; Chowdhury, S.; Beltagui, A.; Albores, P. The potential of emergent disruptive technologies for humanitarian supply chains: The integration of blockchain, Artificial Intelligence and 3D printing. _Int. J. Prod. Res._ **2020** , _58_ , 4610–4630. [CrossRef] 

11. Zheng, G.; Ivanov, D.; Brintrup, A. An adaptive federated learning system for information sharing in supply chains. _Int. J. Prod. Res._ **2025** , _63_ , 3938–3960. [CrossRef] 

12. Page, M.J.; McKenzie, J.E.; Bossuyt, P.M.; Boutron, I.; Hoffmann, T.C.; Mulrow, C.D.; Shamseer, L.; Tetzlaff, J.M.; Akl, E.A.; Brennan, S.E.; et al. The PRISMA 2020 statement: An updated guideline for reporting systematic reviews. _BMJ_ **2021** , _372_ , n71. [CrossRef] 

13. Kitchenham, B.A.; Budgen, D.; Brereton, P. _Evidence-Based Software Engineering and Systematic Reviews_ ; CRC Press: Boca Raton, FL, USA, 2015; Volume 4. 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

30 of 33 

14. Tranfield, D.; Denyer, D.; Smart, P. Towards a methodology for developing evidence-informed management knowledge by means of systematic review. _Br. J. Manag._ **2003** , _14_ , 207–222. [CrossRef] 

15. Gusenbauer, M. Search where you will find most: Comparing the disciplinary coverage of 56 bibliographic databases. _Scientometrics_ **2022** , _127_ , 2683–2745. [CrossRef] 

16. Kosasih, E.E.; Papadakis, E.; Baryannis, G.; Brintrup, A. A review of explainable artificial intelligence in supply chain management using neurosymbolic approaches. _Int. J. Prod. Res._ **2024** , _62_ , 1510–1540. [CrossRef] 

17. Braun, V.; Clarke, V. Using thematic analysis in psychology. _Qual. Res. Psychol._ **2006** , _3_ , 77–101. 

18. Villegas-Ch, W.; Gutierrez, R.; Govea, J.; García-Ortiz, J. Integrated AI, IoT, and Blockchain for Enhancing Security and Traceability in Perishable Logistics. _Emerg. Sci. J._ **2025** , _9_ , 2471–2496. [CrossRef] 

19. Wu, H.; Li, G.; Zheng, H. How Does Digital Intelligence Technology Enhance Supply Chain Resilience? Sustainable Framework and Agenda. _Ann. Oper. Res._ **2025** , _355_ , 901–923. [CrossRef] 

20. Jha, A.K.; Raj, A.; Jha, A.K.; Shetty, S.D. Agricultural supply chain management using hyperledger and AIOT. _J. Ambient Intell. Humaniz. Comput._ **2025** , _16_ , 471–485. [CrossRef] 

21. Wang, X.; Wu, Q.; Zeng, H.; Yang, X.; Cui, H.; Yi, X.; Piran, M.J.; Luo, M.; Que, Y. Blockchain-Empowered H-CPS Architecture for Smart Agriculture. _Adv. Sci._ **2025** , _12_ , 2503102. [CrossRef] 

22. Xue, J.; Li, G.; Ivanov, D. Digital transformation in the blockchain era: Balancing efficiency and resilience in operations management. _Int. J. Prod. Econ._ **2025** , _282_ , 109525. [CrossRef] 

23. Gao, C.; Keoy, K.H.; Lim, A.F. Adoption and impact of generative artificial intelligence on blockchain-enabled supply chain efficiency. _J. Syst. Inf. Technol._ **2025** , _27_ , 173–196. [CrossRef] 

24. Boudouaia, M.A.; Ouchani, S. Designing Secure and Smart Supply Chains: A Roadmap. _Computer_ **2024** , _57_ , 45–55. [CrossRef] 

25. Adamashvili, N.; Zhizhilashvili, N.; Tricase, C. The Integration of the Internet of Things, Artificial Intelligence, and Blockchain Technology for Advancing the Wine Supply Chain. _Computers_ **2024** , _13_ , 72. [CrossRef] 

26. Vijayapriya, R.; Arun, S.L.; Vengatesan, K.; Samee, S. Smart manufacturing supply chain process strategy using intelligent computation techniques. _Int. J. Interact. Des. Manuf. (IJIDeM)_ **2025** , _19_ , 681–694. [CrossRef] 

27. Albaaji, G.F.; Chandra, S.S.V. Blockchain technology in agriculture: Digitizing the Iraqi agricultural environment. _Environ. Dev. Sustain._ **2025** , _27_ , 17741–17752. [CrossRef] 

28. Vanmathi, C.; Farouk, A.; Alhammad, S.M.; Mangayarkarasi, R.; Bhattacharya, S.; Kasyapa, M.S.B. The Role of Blockchain in Transforming Industries Beyond Finance. _IEEE Access_ **2024** , _12_ , 148845–148867. [CrossRef] 

29. Zulkarnain, S. Development of Blockchain-Based System for Tracking Fish Species’ Taxonomy and Conservation Status. _FishTaxa_ **2024** , _31_ , 22–31. 

30. Chandra, V.S.S.; Hareendran, A.S.; Albaaji, G.F. Precision farming for sustainability: An agricultural intelligence model. _Comput. Electron. Agric._ **2024** , _226_ , 109386. [CrossRef] 

31. Ramachandran, M. AI and Blockchain Framework for Healthcare Applications. _Facta Univ.–Ser. Electron. Energet._ **2024** , _37_ , 169–193. [CrossRef] 

32. Hong, Z.; Xiao, K. Digital economy structuring for sustainable development: The role of blockchain and artificial intelligence in improving supply chain and reducing negative environmental impacts. _Sci. Rep._ **2024** , _14_ , 3912. [CrossRef] [PubMed] 

33. Chen, H.Y.; Sharma, K.; Sharma, C.; Sharma, S. Integrating explainable artificial intelligence and blockchain to smart agriculture: Research prospects for decision making and improved security. _Smart Agric. Technol._ **2023** , _6_ , 100350. [CrossRef] 

34. Wang, D.; Yu, A. Supply Chain Resources and Economic Security Based on Artificial Intelligence and Blockchain Multi-Channel Technology. _Int. J. Inf. Technol. Syst. Approach_ **2022** , _16_ , 1–15. [CrossRef] 

35. Tsolakis, N.; Schumacher, R.; Dora, M.; Kumar, M. Artificial intelligence and blockchain implementation in supply chains: A pathway to sustainability and data monetisation? _Ann. Oper. Res._ **2023** , _327_ , 157–210. [CrossRef] 

36. Bechtsis, D.; Tsolakis, N.; Iakovou, E.; Vlachos, D. Data-driven secure, resilient and sustainable supply chains: Gaps, opportunities, and a new generalised data sharing and data monetisation framework. _Int. J. Prod. Res._ **2022** , _60_ , 4397–4417. [CrossRef] 

37. Das, A.K.; Bera, B.; Giri, D. AI and Blockchain-Based Cloud-Assisted Secure Vaccine Distribution and Tracking in IoMT-Enabled COVID-19 Environment. _IEEE Internet Things Mag._ **2021** , _4_ , 26–32. [CrossRef] 

38. Chidepatil, A.; Bindra, P.; Kulkarni, D.; Qazi, M.; Kshirsagar, M.; Sankaran, K. From Trash to Cash: How Blockchain and Multi-Sensor-Driven Artificial Intelligence Can Transform Circular Economy of Plastic Waste? _Adm. Sci._ **2020** , _10_ , 23. [CrossRef] 

39. Zhu, Q.; Liu, Z. AIoT-Enabled Blockchain Integration in Dual-Product Music Supply Chains: Optimizing Sustainable Production Decisions Under Consumer Eco-Awareness Heterogeneity. _IEEE Internet Things J._ **2025** , _12_ , 39572–39583. [CrossRef] 

40. Lv, J.; Kim, B.G.; Xiao, Z. AIoT-Enhanced Blockchain Framework for Hierarchical Access Control in Green Medical Supply Chain Systems. _IEEE Internet Things J._ **2025** , _12_ , 48405–48415. [CrossRef] 

41. Dahbi, H.; Benabdellah, A.C.; Belhadi, A.; Kamble, S.; Zouadi, T. Generative heuristics-driven for blockchain-enhanced reputation systems and dynamic optimization in short food supply chains. _J. Oper. Res. Soc._ **2025** , 1–23. [CrossRef] 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

31 of 33 

42. Shelke, K.; Pawar, C.; Kalantri, R. DMCFHNet-based blockchain-enabled secure data sharing and retrieval in the supply chain management system. _J. Control Decis._ **2025** , 1–22. [CrossRef] 

43. Chen, J.; Lu, L.; Chen, M.; Qu, T.; Li, Z. Optimization of Artificial Intelligence Algorithms for Intelligent Manufacturing: Enhancing Production Efficiency and Supply Chain Stability. _J. Organ. End User Comput._ **2025** , _37_ , 1–28. [CrossRef] 

44. Ch, R.; Sai, V.G.; Divya, D.; Gadekallu, T.R.; Abidi, M.H.; Alasim, F. Security and privacy for Binance-based integrated blockchain for blood supply chain management systems. _PeerJ Comput. Sci._ **2025** , _11_ , e3123. [CrossRef] 

45. Khanna, A.; Jain, S.; Sah, A.; Dangi, S.; Sharma, A.; Tiang, S.S.; Wong, C.H.; Lim, W.H. Generative AI and Blockchain-Integrated Multi-Agent Framework for Resilient and Sustainable Fruit Cold-Chain Logistics. _Foods_ **2025** , _14_ , 3004. [CrossRef] 

46. Fatorachian, H.; Kazemi, H. Secure digital frameworks for cold chain emissions tracking: leveraging AI and blockchain for robust data integrity. _Comput. Ind. Eng._ **2025** , _209_ , 111467. [CrossRef] 

47. Soy, A.; Balkrishna, S.M. AI Predictive Analytics for Verifying Pharmaceutical Authenticity and Combating Drug Counterfeiting. _Commun. Appl. Nonlinear Anal._ **2025** , _32_ , 76–86. [CrossRef] 

48. Sunmola, F.; Baryannis, G.; Tan, A.; Co, K.; Papadakis, E. Holistic Framework for Blockchain-Based Halal Compliance in Supply Chains Enabled by Artificial Intelligence. _Systems_ **2025** , _13_ , 21. [CrossRef] 

49. Alhazmi, S.; Rahmani, M.K.I.; Arif, M.; Nafis, M.T. Developing Intelligent and Immutable Vaccine Supply and Operation Platform Using Blockchain and Artificial Intelligence Technologies. _IEEE Access_ **2024** , _12_ , 88189–88201. [CrossRef] 

50. Ismail, S.; Nouman, M.; Reza, H.; Vasefi, F.; Zadeh, H.K. A Blockchain-Based Fish Supply Chain Framework for Maintaining Fish Quality and Authenticity. _IEEE Trans. Serv. Comput._ **2024** , _17_ , 1877–1886. [CrossRef] 

51. Abdelhamid, M.M.; Sliman, L.; Ben Djemaa, R. AI-Enhanced Blockchain for Scalable IoT-Based Supply Chain. _Logistics_ **2024** , _8_ , 109. [CrossRef] 

52. Wang, F. The Application of Intelligent Information Systems Driven by 6G Big Data in Product Sales Traceability. _Wirel. Pers. Commun._ **2024** . [CrossRef] 

53. Dillenberger, D.N.; Novotny, P.; Zhang, Q.; Jayachandran, P.; Gupta, H.; Hans, S.; Verma, D.; Chakraborty, S.; Thomas, J.J.; Walli, M.M.; et al. Blockchain analytics and artificial intelligence. _IBM J. Res. Dev._ **2019** , _63_ , 5:1–5:14. [CrossRef] 

54. Nasurudeen, N.; Karthikeyan, P. A Reinforcement Learning Integrated in Heuristic search method for self-driving vehicle using blockchain in supply chain management. _Int. J. Intell. Netw._ **2020** , _1_ , 92–101. [CrossRef] 

55. Eluubek kyzy, I.; Song, H.; Vajdi, A.; Wang, Y.; Zhou, J. Blockchain for consortium: A practical paradigm in agricultural supply chain system. _Expert Syst. Appl._ **2021** , _184_ , 115425. [CrossRef] 

56. Dey, S.; Saha, S.; Singh, A.K.; McDonald-Maier, K. SmartNoshWaste: Using Blockchain, Machine Learning, Cloud Computing and QR Code to Reduce Food Waste in Decentralized Web 3.0 Enabled Smart Cities. _Smart Cities_ **2022** , _5_ , 162–176. [CrossRef] 

57. Zawish, M.; Ashraf, N.; Ansari, R.I.; Davy, S.; Qureshi, H.K.; Aslam, N.; Hassan, S.A. Toward On-Device AI and Blockchain for 6G-Enabled Agricultural Supply Chain Management. _IEEE Internet Things Mag._ **2022** , _5_ , 160–166. [CrossRef] 

58. Wang, X.; Yu, G.; Liu, R.P.; Zhang, J.; Wu, Q.; Su, S.W.; He, Y.; Zhang, Z.; Yu, L.; Liu, T.; et al. Blockchain-Enabled Fish Provenance and Quality Tracking System. _IEEE Internet Things J._ **2022** , _9_ , 8130–8142. [CrossRef] 

59. Zhang, X.; Shi, X.; Pan, W. Big Data Logistics Service Supply Chain Innovation Model Based on Artificial Intelligence and Blockchain. _Mob. Inf. Syst._ **2022** , _2022_ , 4794190. [CrossRef] 

60. Karamchandani, A.; Srivastava, S.K.; Abha; Srivastava, A. A lower approximation based integrated decision analysis framework for a blockchain-based supply chain. _Comput. Ind. Eng._ **2023** , _177_ , 109092. [CrossRef] 

61. Liu, T.; Yuan, Y.; Yu, Z. An Intelligent Optimization Control Method for Enterprise Cost Under Blockchain Environment. _IEEE Access_ **2023** , _11_ , 3597–3606. [CrossRef] 

62. Bhatia, S.; Albarrak, A.S. A Blockchain-Driven Food Supply Chain Management Using QR Code and XAI-Faster RCNN Architecture. _Sustainability_ **2023** , _15_ , 2579. [CrossRef] 

63. Lee, C.A.; Chow, K.M.; Chan, H.A.; Lun, D.P.K. Decentralized governance and artificial intelligence policy with blockchain-based voting in federated learning. _Front. Res. Metr. Anal._ **2023** , _8_ , 1035123. [CrossRef] 

64. Ismail, S.; Dandan, S.; Dawoud, D.W.; Reza, H. A Comparative Study of Lightweight Machine Learning Techniques for Cyber-Attacks Detection in Blockchain-Enabled Industrial Supply Chain. _IEEE Access_ **2024** , _12_ , 102481–102491. [CrossRef] 

65. Shidpour, H.; Karimi, N.; Baryannis, G.; Shidpour, M. A multi-phase analytics framework for supply chain supplier selection and order allocation with delay risks and Industry 4.0 readiness. _Supply Chain Anal._ **2025** , _12_ , 100172. [CrossRef] 

66. Brintrup, A.; Baryannis, G.; Tiwari, A.; Ratchev, S.; Martinez-Arellano, G.; Singh, J. Trustworthy, responsible and ethical artificial intelligence in manufacturing and supply chains: Synthesis and emerging research questions. _Data-Centric Eng._ **2025** , _6_ , e53. [CrossRef] 

67. Sunmola, F.; Baryannis, G. Artificial Intelligence Opportunities for Resilient Supply Chains. _IFAC-PapersOnLine_ **2024** , _58_ , 813–818. [CrossRef] 

68. Wyrembek, M.; Baryannis, G. Using MCDM methods to optimise machine learning decisions for supply chain delay prediction: A Stakeholder-centric approach. _Logforum_ **2024** , _20_ , 175–189. [CrossRef] 

https://doi.org/10.3390/systems14040363 

_Systems_ **2026** , _14_ , 363 

32 of 33 

69. Malik, S.; Islam, S.M.R.; Akram, T.; Naqvi, S.R.; Alghamdi, N.S.; Baryannis, G. A novel hybrid meta-heuristic contrast stretching technique for improved skin lesion segmentation. _Comput. Biol. Med._ **2022** , _151_ , 106222. [CrossRef] [PubMed] 

70. Sirmon, D.G.; Hitt, M.A.; Ireland, R.D. Managing firm resources in dynamic environments to create value: Looking inside the black box. _Acad. Manag. Rev._ **2007** , _32_ , 273–292. [CrossRef] 

71. Sirmon, D.G.; Hitt, M.A.; Ireland, R.D.; Gilbert, B.A. Resource orchestration to create competitive advantage: Breadth, depth, and life cycle effects. _J. Manag._ **2011** , _37_ , 1390–1412. 

72. El-Baz, K.A.; El-Midany, T.T.; Ghattas, M.S.; AbouEleaz, M.A. The Impact of Enterprise Resource Planning (ERP) Implementation on Performance of Firms: A Case to Support Production Process Improvement. _Mansoura Eng. J._ **2023** , _48_ , 10. 

73. Queiroz, M.M.; Wamba, S.F.; Jabbour, C.J.C.; Machado, M.C. Supply chain resilience in the UK during the coronavirus pandemic: A resource orchestration perspective. _Int. J. Prod. Econ._ **2022** , _245_ , 108405. [CrossRef] [PubMed] 

74. Lu, Q.; Qin, W.; Yan, R.; Zhang, S.; Ma, L. The effect of SMEs’ digitalization on supply chain financing performance: Based on the resource orchestration theory. _J. Theor. Appl. Electron. Commer. Res._ **2025** , _20_ , 20. [CrossRef] 

75. Liu, Z.; Prajogo, D.; Oke, A. Supply chain technologies: Linking adoption, utilization, and performance. _J. Supply Chain Manag._ **2016** , _52_ , 22–41. [CrossRef] 

76. Tajeddini, K.; Hussain, M.; Gamage, T.C.; Papastathopoulos, A. Effects of resource orchestration, strategic information exchange capabilities, and digital orientation on innovation and performance of hotel supply chains. _Int. J. Hosp. Manag._ **2024** , _117_ , 103645. [CrossRef] 

77. Gligor, D.M.; Davis-Sramek, B.; Tan, A.; Vitale, A.; Russo, I.; Golgeci, I.; Wan, X. Utilizing blockchain technology for supply chain transparency: A resource orchestration perspective. _J. Bus. Logist._ **2022** , _43_ , 140–159. [CrossRef] 

78. Ahmad, M.A.; Baryannis, G.; Hill, R. Defining Complex Adaptive Systems: An Algorithmic Approach. _Systems_ **2024** , _12_ , 45. [CrossRef] 

79. Teece, D.J.; Pisano, G.; Shuen, A. Dynamic capabilities and strategic management. _Strateg. Manag. J._ **1997** , _18_ , 509–533. [CrossRef] 

80. Teece, D.J. Explicating dynamic capabilities: The nature and microfoundations of (sustainable) enterprise performance. _Strateg. Manag. J._ **2007** , _28_ , 1319–1350. [CrossRef] 

81. Giannakis, M.; Louis, M. A multi-agent based system with big data processing for enhanced supply chain agility. _J. Enterp. Inf. Manag._ **2016** , _29_ , 706–727. [CrossRef] 

82. Iranmanesh, M.; Maroufkhani, P.; Asadi, S.; Ghobakhloo, M.; Dwivedi, Y.K.; Tseng, M.L. Effects of supply chain transparency, alignment, adaptability, and agility on blockchain adoption in supply chain among SMEs. _Comput. Ind. Eng._ **2023** , _176_ , 108931. [CrossRef] 

83. Aslam, H.; Blome, C.; Roscoe, S.; Azhar, T.M. Dynamic supply chain capabilities: How market sensing, supply chain agility and adaptability affect supply chain ambidexterity. _Int. J. Oper. Prod. Manag._ **2018** , _38_ , 2266–2285. [CrossRef] 

84. Hosseini, E.; Tajpour, M.; Salamzadeh, A.; Demiryurek, K.; Kawamorita, H. Resilience and knowledge-based firms performance: The mediating role of entrepreneurial thinking. _J. Entrep. Bus. Resil._ **2021** , _4_ , 7–29. 

85. Kaneberg, E.; Piotrowicz, W.D.; Jensen, L.M.; Hertz, S.; Kedziora, D. Supply chain resilience and critical dynamic capabilities: A balanced scorecard approach. _Prod. Manuf. Res._ **2025** , _13_ , 2523957. [CrossRef] 

86. Zhao, N.; Hong, J.; Lau, K.H. Impact of supply chain digitalization on supply chain resilience and performance: A multi-mediation model. _Int. J. Prod. Econ._ **2023** , _259_ , 108817. [CrossRef] 

87. Baryannis, G.; Plexousakis, D. WSSL: A Fluent Calculus-Based Language for Web Service Specifications. In _25th International Conference on Advanced Information Systems Engineering (CAiSE 2013)_ ; Lecture Notes in Computer Science; Salinesi, C., Norrie, M.C., Pastor, Ó., Eds.; Springer: Berlin/Heidelberg, Germany, 2013; Volume 7908, pp. 256–271. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

https://doi.org/10.3390/systems14040363 

