# **Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey** 

YINQIU LIU, Nanyang Technological University, Singapore RUICHEN ZHANG, Nanyang Technological University, Singapore HAOXIANG LUO, University of Electronic Science and Technology of China, China YIJING LIN, Beijing University of Posts and Telecommunications, China GENG SUN, Jilin University, China & Nanyang Technological University, Singapore DUSIT NIYATO, Nanyang Technological University, Singapore HONGYANG DU, The University of Hong Kong, Hong Kong SAR, China ZEHUI XIONG, Queen’s University Belfast, United Kingdom YONGGANG WEN, Nanyang Technological University, Singapore ABBAS JAMALIPOUR, The University of Sydney, Australia DONG IN KIM, Sungkyunkwan University, South Korea PING ZHANG, Beijing University of Posts and Telecommunications, China 

Agentification serves as a critical enabler of Edge General Intelligence (EGI), transforming massive edge devices into cognitive agents through integrating Large Language Models (LLMs) and perception, reasoning, and acting modules. These agents collaborate across heterogeneous edge infrastructures, forming multi-LLM agentic AI systems that leverage collective intelligence and specialized capabilities to tackle complex, multistep tasks. However, the collaborative nature of multi-LLM systems introduces critical security vulnerabilities, including insecure inter-LLM communications, expanded attack surfaces, and cross-domain data leakage that traditional perimeter-based security cannot adequately address. To this end, this survey introduces zero-trust security of multi-LLM in EGI, a paradigmatic shift following the “never trust, always verify” principle. We begin by systematically analyzing the security risks in multi-LLM systems within EGI contexts. Subsequently, we present the vision of a zero-trust multi-LLM framework in EGI. We then survey key technical progress to facilitate zero-trust multi-LLM systems in EGI. Particularly, we categorize zero-trust security mechanisms into model- and system-level approaches. The former and latter include strong identification, context-aware access control, etc., and proactive maintenance, blockchain-based management, etc., respectively. Finally, we identify critical research directions. This survey serves as the first systematic treatment of zero-trust applied to multi-LLM systems, providing both theoretical foundations and practical strategies. 

CCS Concepts: • **General and reference** → **Surveys and overviews** ; • **Networks** → **Mobile networks** ; • **Computing methodologies** → **Artificial intelligence** ; • **Security and privacy** → **Network security** . 

Additional Key Words and Phrases: Multi-LLM, zero-trust, edge general intelligence, agentic AI, security 

## **1 INTRODUCTION** 

## **1.1 Background** 

Multi-Large Language Model (multi-LLM) systems [1] are rapidly emerging as a transformative paradigm in agentic AI [2] and agentification [3], where multiple specialized LLMs collaborate 

Authors’ Contact Information: Yinqiu Liu, yinqiu001@e.ntu.edu.sg, Nanyang Technological University (NTU), Singapore; Ruichen Zhang, ruichen.zhang@ntu.edu.sg, NTU, Singapore; Haoxiang Luo, lhx991115@163.com, University of Electronic Science and Technology of China, Chengdu, China; Yijing Lin, yjlin@bupt.edu.cn, Beijing University of Posts and Telecommunications, Beijing, China; Geng Sun, sungeng@jlu.edu.cn, Jilin University, Jilin, China; Dusit Niyato, dniyato@ntu.edu.sg, NTU, Singapore; Hongyang Du, duhy@hku.hk, The University of Hong Kong, Hong Kong SAR, China; Zehui Xiong, z.xiong@qub.ac.uk, Queen’s University Belfast, Northern Ireland, United Kingdom; Yonggang Wen, ygwen@ntu.edu.sg, NTU, Singapore; Abbas Jamalipour, a.jamalipour@ieee.org, The University of Sydney, Sydney, Australia; Dong In Kim, dongin@skku.edu, Sungkyunkwan University, Suwon, South Korea; Ping Zhang, pzhang@bupt.edu.cn, Beijing University of Posts and Telecommunications, Beijing, China; 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:2 

to solve complex tasks beyond the capabilities of individual agents. This architectural shift is particularly crucial for advancing Edge General Intelligence (EGI) [4, 5], since different LLMs trained on diverse domain-specific datasets and serving users within certain application domains can be strategically deployed to provide low-latency, customized, and ubiquitous intelligence services. Multi-LLM systems have gained widespread deployment across EGI domains. For instance, Waymo deploys multiple specialized LLMs directly on vehicle chips, where perception LLMs manage sensors, planning LLMs make real-time navigation decisions, and control LLMs execute driving maneuvers[1] . Moreover, AWS’s Bedrock implements dynamic LLM routing. For example, in online shopping systems, lightweight LLMs can handle simple product searches and order tracking locally, while complex queries, such as product comparisons, can be routed to powerful cloud LLMs, reducing response latency by up to 50% and operational costs by 30%[2] . 

## **1.2 Motivations** 

However, multi-LLM systems expose significant security vulnerabilities that threaten the integrity of EGI deployments. First, each individual LLM remains susceptible to jailbreaking attacks, where adversaries craft malicious prompts to bypass safety guardrails and extract harmful outputs [6]. Furthermore, the collaborative nature of multi-LLM architectures introduces unique attack surfaces: inter-LLM communication channels can be exploited through prompt injection, where compromised outputs from one LLM cascade as malicious inputs to others [7]. Additionally, the presence of potentially malicious LLMs poses risks, as one rogue LLM can poison the entire decision-making pipeline [8]. Moreover, multi-LLM systems blur the boundaries between data ownership and model accountability. When multiple LLMs process sensitive information collaboratively, it becomes challenging to ensure consistent privacy policies across heterogeneous LLMs and trace data leakage sources [9]. The user-system boundary is similarly obscured, as LLMs in multi-LLM intelligent systems serve dual roles, both as user interfaces and as autonomous agents executing system operations, which undermines traditional access control mechanisms [9]. 

To this end, researchers have focused on improving the trustworthiness and security of multiLLM systems. For instance, during the pre-training phases, adversarial training [10] and Differential Privacy (DP) techniques [11] can be employed to strengthen LLMs’ resistance against malicious inputs and prevent models from memorizing users’ private data. During edge deployment, LLMs can be deployed within Trusted Execution Environments (TEEs) on devices, thereby isolating attackers and ensuring that LLM operations remain tamper-proof [12]. In addition, reactive output filtering mechanisms and firewall-based defenses have been developed to intercept and sanitize potentially harmful content before it reaches end users [13]. 

Nonetheless, these measures are fundamentally grounded in traditional and intuitive perimeterbased security paradigms, where they construct and expand security boundaries within multi-LLM EGI scenarios to establish trusted zones. Although these approaches demonstrated effectiveness, they face increasingly severe challenges in dynamic multi-LLM deployments. First, LLM capabilities are advancing at an unprecedented pace, often exceeding designers’ expectations, making it difficult to define clear security boundaries. For example, GPT-4 demonstrated unexpected autonomous problem-solving capabilities during safety testing when it successfully bypassed a CAPTCHA challenge by recruiting a human worker on TaskRabbit [14]. Furthermore, many perimeter-based security defenses exhibit reactive characteristics. For instance, with reactive output filtering, the transient existence of malicious content can still cause information leakage and security risks, which is detrimental to EGI [5]. Finally, the collaborative nature of multi-LLM systems creates extensive 

> 1https://waymo.com/blog/2024/10/introducing-emma 

> 2https://aws.amazon.com/blogs/machine-learning/multi-LLM-routing-strategies-for-generative-ai-applications-on-aws/ 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:3 

lateral movement opportunities across different security domains, potentially compromising data integrity and undermining the effectiveness of boundary-based protections [15]. 

Recently, zero-trust security has emerged as a transformative paradigm, gaining widespread attention and achieving significant success across diverse domains, including computer networks, 6G communications, and IoT systems [16]. Unlike perimeter-based approaches that operate under implicit trust assumptions and respond only after security breaches occur, zero-trust security adopts the foundational principle of “never trust, always verify,” establishing a security-first framework where trust is explicitly earned rather than assumed. Specifically, the integration of zero-trust security in multi-LLM and the corresponding benefits can be explained as follows. 

- **Explicit Verification** : No component in the multi-LLM system possesses inherent trustworthiness. Every data access request, tool invocation, and inter-LLM communication should undergo rigorous validation before execution [15]. Each user input and LLM output is treated as potentially malicious and should be verified through formal verification. This comprehensive verification scheme specifically addresses the single-point failure vulnerabilities of multi-LLM systems, where a single compromised component can undermine the entire EGI. Moreover, unlike traditional perimeter-based approaches that implicitly trust components within established boundaries, explicit verification ensures continuous authentication even for previously verified entities, effectively preventing the lateral movement attacks that plague collaborative LLM architectures. 

- **Least Privilege** : Traditional approaches assume that enhancing LLM capabilities inherently improves service experience and security. However, zero-trust enforces that user requests should be fulfilled using LLMs with the minimum necessary capabilities [16], ensuring optimal resource allocation and security posture. Access to data repositories, external APIs, and computational resources is granted dynamically based on specific task requirements and validated trust levels, enabling precise capability matching while maintaining security boundaries [16]. This intelligent access control prevents over-privileged operations that could expose sensitive resources, while simultaneously optimizing computational efficiency in resource-constrained edge environments. 

- **Continuous Monitoring** : Finally, zero-trust LLM maintains persistent surveillance of system activities, assuming that security threats can emerge at any time from any component [17]. Each deviation from established operational patterns triggers immediate security assessments and potential containment measures. This proactive monitoring capability enables early threat detection and rapid response in multi-LLM EGI deployments, maintaining the operational continuity of EGI services. Unlike reactive security measures that respond only after attacks have been taken, continuous monitoring provides real-time visibility into interLLM communications and collaborative decision-making processes, enabling the detection of subtle anomalies such as consensus manipulation and cross-context data leakage before they can propagate throughout the distributed system. 

## **1.3 Related Surveys** 

_1.3.1 Single-LLM Security._ Recent research into the security and privacy issues of LLMs has generated a broad range of surveys, each contributing unique insights into this complex domain. Initially, Das _et al._ [18] presented a comprehensive survey focusing explicitly on the security and privacy challenges of LLMs, analyzing vulnerabilities, such as prompt hacking and adversarial attacks, and privacy attacks such as gradient leakage and membership inference. They highlighted these risks in various applications, providing detailed mitigation strategies and emphasizing future research directions. Building upon this foundation, Aguilera-Martínez _et al._ [19] extended the discussion 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:4 

Table 1. Summary of related surveys 

|**Ref.**|**Overview**|**Single-LLM**|**Multi-LLM**|**EGI**|**Zero-Trust**|
|---|---|---|---|---|---|
|[18]|A survey comprehensively analyzing security and privacy vulnerabilities of LLMs, including<br>prompt hacking, adversarial attacks, and privacy leakage, alongside mitigation strategies.|✓|✗|✗|✗|
|[19]|A structured survey categorizing LLM attacks by lifecycle stages, detailing prevention-based<br>and detection-based defenses.|✓|✗|✗|✗|
|[20]|A survey examining security vulnerabilities specifc to edge intelligence scenarios involving<br>LLM deployments, highlighting proactive and reactive defenses under resource constraints.|✓|✗|✓|✗|
|[21]|A survey proposing a taxonomy categorizing threats of LLMs by origin (inputs, model<br>faws, interactions) and consequences (security, privacy, ethical issues), discussing relevant<br>defenses.|✓|✗|✗|✗|
|[22]|A survey providing guidelines for evaluating LLM alignment with security and ethical<br>standards, covering dimensions such as reliability, fairness, and robustness.|✓|✗|✗|✗|
|[9]|A survey categorizing security challenges in multi-LLM systems with a focus on cross-domain<br>interactions, highlighting risks like dynamic grouping and collusion.|✗|✓|✗|✗|
|[23]|A survey analyzing communication security within multi-LLM systems, detailing vulnerabil-<br>ities and mitigation methods across diferent interaction patterns and stages.|✗|✓|✗|✗|
|[24]|A survey exploring trust, orchestration, and resource scheduling challenges in multi-LLM<br>EGI deployments.|✗|✓|✓|✗|
|[25]|A systematic evaluation of security-efciency trade-ofs in collaborative multi-LLM systems,<br>reviewing infectious malicious prompts and targeted defense methods.|✗|✓|✗|✗|



by categorizing LLM attacks according to their occurrence at different lifecycle stages, including pre-training, inference, etc. Additionally, they provided a structured overview of defense strategies, separating them into prevention-based and detection-based mechanisms. Furthermore, Friha _et al._ [20] explored the integration of LLMs within edge intelligence, examining the unique architectural challenges, resource constraints, and security vulnerabilities presented when deploying powerful LLMs in resource-constrained environments. Particularly, they categorized defense strategies into proactive measures, which aim to prevent potential threats, and reactive measures, designed to mitigate risks after threats have emerged. Gan _et al._ [21] conducted a comprehensive survey of security risks in LLM-based agents, proposing an innovative taxonomy based on threat sources and impacts. Specifically, they categorized threats by their origins (problematic inputs, model flaws, or input-model interactions) and their consequences (security/safety, privacy, or ethical issues), thus comprehensively discussing security vulnerabilities and defense mechanisms across different components of LLM-based agent architectures. Lastly, Liu _et al._ [22] offered guidelines for evaluating the alignment of LLMs with security and ethical standards. Their survey covered critical dimensions such as reliability, safety, fairness, resistance to misuse, interpretability, adherence to social norms, and robustness. Despite their thoroughness, these surveys collectively reveal common limitations. Primarily, there is insufficient coverage of multi-LLM systems, where interactions between multiple LLMs can introduce complex, emergent security and ethical risks not captured by analyses focusing solely on single-LLM deployments. 

_1.3.2 Multi-LLM Security._ To this end, few works have begun explicitly discussing the multiLLM scenario. Ko _et al._ [9] categorized security challenges specifically in cross-domain multiLLM systems, highlighting novel threats arising from inter-domain interactions such as unvetted dynamic grouping and collusion control. Kong _et al._ [23] presented a comprehensive survey on the communication security of multi-LLM systems, outlining security risks across user-LLM, LLM-LLM, and LLM-environment interactions, along with detailed countermeasures for each communication stage. Luo _et al._ [24] explored the architecture, trust, and orchestration challenges of multi-LLM deployments in EGI, emphasizing the importance of dynamic orchestration and resource scheduling to maintain security and efficiency. Furthermore, Peign _et al._ [25] systematically evaluated the tradeoffs between security and collaborative efficiency within multi-LLM systems, introducing concepts such as infectious malicious prompts and reviewing targeted defense strategies. However, these surveys primarily discuss traditional security approaches aimed at enhancing the trustworthiness of 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:5 

LLMs themselves and their generated content, including methods such as adversarial training, LLM pruning, and cryptographic protections. They did not consider the emerging zero-trust paradigm, in which implicit trusts are eliminated in every entity by default. We observe that many advanced studies [26–33] have proposed defenses aligning with zero-trust principles, while a comprehensive survey and tutorial systematically discussing multi-LLM security from a zero-trust perspective have yet to emerge. Our survey fills this critical gap (see Table 1). 

## **1.4 Our Contributions and Survey Structure** 

To the best of our knowledge, we are the first to survey the concept of “zero-trust multi-LLM” in EGI scenarios. We begin by comprehensively introducing essential background knowledge, including the basics of LLM technologies, the fundamentals of multi-LLM architectures, zero-trust security principles, and the emerging paradigm of EGI. Subsequently, we systematically analyze the security challenges faced by multi-LLM systems in EGI deployments. We then survey traditional security approaches based on perimeter security models. Particularly, we analyze the inherent limitations of these approaches and demonstrate how zero-trust principles offer transformative opportunities. Observing that although substantial literature has presented security defenses that align with zero-trust concepts, a comprehensive zero-trust framework for multi-LLM systems remains absent. Hence, we further present a unified vision of zero-trust multi-LLM architectures for EGI. Then, we conduct an extensive survey of zero-trust security mechanisms categorized into model- and system-level approaches. Finally, we identify some critical future research directions that will drive the advancement of zero-trust multi-LLM systems. The main contributions of this survey can be summarized as follows. 

- We comprehensively introduce the current development status and advantages of multi-LLM systems over traditional single LLMs. Afterward, we comprehensively analyze the critical role that multi-LLM plays in constructing future EGI architectures. Moreover, we introduce the concept and applications of zero-trust security. 

- We analyze security risks faced by multi-LLM systems in EGI deployments, categorizing threats at both intra-LLM and inter-LLM levels. Subsequently, we summarize traditional perimeter-based security approaches, analyze their limitations, and explore the integration of zero-trust security with multi-LLM architectures, providing a comparative analysis between traditional trustworthy approaches and zero-trust paradigms. 

- We present a comprehensive zero-trust multi-LLM framework oriented to EGI environments, demonstrating how zero-trust principles can be systematically implemented across distributed multi-LLM deployments. Moreover, we conduct an extensive survey of state-of-the-art zerotrust security mechanisms for multi-LLM systems, categorizing approaches into model- and system-level approaches. 

- We identify and discuss three critical future research directions that require immediate attention from the research community. These research directions collectively advance the theoretical foundations and practical deployment of zero-trust multi-LLM systems, encouraging further research in this emerging field. 

The remainder of this survey is organized as shown in Fig. 1. First, Section 2 provides fundamental background knowledge, covering the basics of LLMs, multi-LLM systems, zero-trust security principles, and their applications in EGI scenarios. Then, Section 3 systematically analyzes the security challenges faced by multi-LLM systems in EGI deployments, categorizing vulnerabilities at both intra-LLM and inter-LLM levels, and surveys traditional trustworthy approaches along with their inherent limitations. Section 4 introduces a comprehensive zero-trust multi-LLM framework for EGI, demonstrating the systematic implementation of zero-trust principles through detailed 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:6 

**==> picture [333 x 285] intentionally omitted <==**

**----- Start of picture text -----**<br>
Single-LLM  security<br>Introduction Background Motivations Related surveys Contributions and structure<br>Multi-LLM  security<br>SSS<br>Multi-LLM systems Multi-LLM for EGI Zero-trust concepts and mechanisms Applications<br>Fundamentals and Preliminaries •••• OrchestrationRouting and LLM selectionShared memory & stateCommunication protocols Multi-LLM in EGIApplications of Vision to EGI securityZero-trust  •••• Explicit Identity verificationLeast-privilege access controlContinuous monitoring and analyticsMicro-segmentation and isolation of zero-trust in AI<br>Intra-LLM  security issues Inter-LLM  security issues Trustworthy multi-LLMs<br>Security of Multi-LLM in EGI Jailbreak & prompt injectionUnpredictable LLM abilities Over-permissive integrationExpanded attack surfaces context Cross-data  Adversarial trainingDifferential privacy Access control & firewalls to zero-trust trustworthy From<br>a == Data leakage & privacy Insecure communication leakage RL with human feedback = communication Secure  defenses<br>a Toxic/misaligned responses Consensus manipulation —— Reactive output filtering protocols<br>Constructing zero-trust multi-LLM in EGI Operation workflow<br>Trustworthy<br>Zero-Trust Multi-LLM for EGI Identity & AuthenticationMobile-edge LLMCloud LLM Communication gatewayUser input checkingOutput verification detectionanomaly Auditing &  •••• Startup and LLM authenticationPrompt understandingCollaborative decision panningPolicy-governed plan execution multi-LLM Multi-LLMzero-trust versus<br>Strong LLM identity,  Context-aware  Stateless and Ephemeral<br>rr authentication, & authorization access control LLM management<br>Intra-LLM Zero-<br>Model approachesTrust Security:  Reputation-based authenticationMulti-factor authentication Stateless LLM managementEphemeral LLM  Implementing zero-trust security is a<br>management systematic work ,<br>p o —— Token-based authentication re a requiring mechanisms from both  model  and<br>system  perspectives.<br>Ld Proactive Maintenance Blockchain and distributed  segmentation & Micro- Intelligent system monitoring &  Legend:<br>Inter-LLM Zero-Trust Security:  Intelligent input checking management isolation managementfailure  Section<br>System approaches Reputation schemes Subsection<br> — Topology-aware approaches Subsubsection<br>Challenges and Future Directions Conclusion Cc] Bullet<br>**----- End of picture text -----**<br>


Fig. 1. The structure of the paper. 

architectural design and operational workflows. Sections 5 and 6 present an extensive review of zero-trust security mechanisms. The identification and discussion of critical future research directions are presented in Section 7. Finally, Section 8 concludes this survey. 

## **2 FUNDAMENTALS AND PRELIMINARIES** 

## **2.1 Multi-LLM Systems** 

LLMs represent a revolutionary advancement in AI, evolving from early transformer architectures to sophisticated systems capable of natural language understanding, reasoning, and generation across diverse domains [34]. Recent developments have witnessed remarkable progress from GPT-3’s 175 billion parameters to GPT-4’s multimodal capabilities, demonstrating unprecedented performance in Q&A, code generation, image drawing, etc. [35]. 

Nonetheless, a single LLM often faces notable limitations in complex real-world scenarios. First, each LLM is fundamentally constrained by performance ceilings and lacks specialization, making it less effective in handling complex, multi-step tasks that require decomposition, domain expertise, or long-term planning [36]. Additionally, single LLMs create computational bottlenecks and represent single points of failure, lacking fault tolerance, load distribution, and horizontal scaling capabilities [37]. Most critically, single LLMs cannot engage in collaborative reasoning, cross-validation, and consensus mechanisms that significantly improve accuracy and reduce hallucinations [38]. These structural limitations highlight the need for collaborative intelligence in practical deployments. 

In contrast, a multi-LLM system refers to an architecture in which multiple LLMs are organized to cooperate on solving one complex task [39]. This approach addresses the fundamental limitations 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:7 

**==> picture [384 x 149] intentionally omitted <==**

**----- Start of picture text -----**<br>
Mechanisms of multi-LLM systems Edge general intelligence Enablers of zero-trust security<br>LLM structure Linear multi- structure Tree  Orchestration  higher costslatency and Longer  Explicit identify is required all times Explicit identify verificationUser<br>High ability Edge LLM<br>LLM pool Profileof LLM Dynamic access control Pass<br>LLM & external toolDynamically elect  … Routing  Denied Least-privilege access control<br>save 01010 01011 11… Decision-<br>+ fetch System log making & failure management<br>Maintain a shared memory to<br>establish context during collaboration Memory  Roadside unit 01010 01011 11… Continuous monitoring<br>Sender: agent AReceiver: agent BContent: “Hello B” Ubiquitous on every smart  Segment 1<br>Structured communication protocol to  Date: 2025-9-23 External tools mobile deviceLimited ability Mobile LLM Segment 2 …<br>ensure the efficiency and security  Communications Micro-segmentation<br>Vehicle<br>**----- End of picture text -----**<br>


Fig. 2. Background and preliminaries. (left): The mechanisms of multi-LLM systems. (middle): The vision of EGI. (right): The concepts of zero-trust. 

of single LLM systems by enabling specialized model deployment for domain-specific expertise, distributed computational load across multiple instances, and collaborative reasoning mechanisms that enhance performance [36, 39]. Moreover, multi-LLM architectures provide inherent fault tolerance through redundancy and horizontal scalability through intelligent routing. For instance, in autonomous driving systems, a multi-LLM system can deploy specialized LLMs for perception (i.e., processing sensor data), planning (i.e., route optimization and decision-making), and control (i.e., vehicle dynamics), where each LLM contributes its expertise while collaborating through standardized communication protocols to achieve reliable autonomous navigation. As shown in Fig. 2(left), the technical enablers of multi-LLM systems are as follows. 

- **Orchestration** : As shown in Fig. 2(left), current multi-LLM systems exhibit diverse topologies, including linear pipelines, star-shaped patterns with a central coordinator, hierarchical trees, etc. [40]. Linear structures fit staged workflows with sequential steps, while star and tree topologies enable central control or layered decomposition. The orchestration mechanisms manage task delegation, scheduling, and output integration. Systems may use explicit controller agents, such as Claude’s lead agent[3] , or implicit workflows via prompt chaining. Moreover, orchestration defines agent profiles and interfaces, aligning the multiLLM system with external user intent. 

- **Routing and LLM Selection:** Efficient routing mechanisms determine which LLMs and tools are called for a given task. Static routing relies on predefined rules, while adaptive routing leverages prompts or learned policies. For instance, the MRKL framework [41] allows an LLM to dynamically select from expert modules for specific operations. Industrial platforms like AWS Bedrock[4] also support LLM routing and load balancing based on task requirements and system states. 

- **Shared Memory & State:** Effective collaboration in multi-LLM systems requires mechanisms for consistent memory access and contextual alignment across agents. Memory can be categorized into several types, including short-term conversational buffers, long-term memory stored in vector databases, and shared domain knowledge [39]. For example, MetaGPT [37] uses a shared message pool to coordinate agent behaviors asynchronously. Similarly, LLMs may access structured external knowledge sources via Retrieval-Augmented Generation 

> 3https://www.anthropic.com/engineering/built-multi-agent-research-system 

> 4https://aws.amazon.com/cn/bedrock/ 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:8 

- (RAG) techniques. This shared memory not only provides coherence in context management but also supports task continuity, enables strategy adaptation from prior episodes, and promotes consistency in goal-oriented reasoning. 

- **Communication Protocols:** LLMs should communicate using well-defined formats and structured interaction patterns [42]. Communication may occur through natural language dialogue, structured JSON exchanges, or symbolic representations. Protocol designs span synchronous turn-taking, asynchronous message broadcasting, and debate-based deliberations evaluated by a coordinator. Emerging protocol standards, such as Model Context Protocol (MCP) [23], Agent Communications Protocol (ACP), and Agent Network Protocol (ANP) [42], support both inter-LLM communication and interactions between agents and external tools, ensuring interoperability and consistent multi-agent behavior. 

## **2.2 Multi-LLM for EGI** 

_2.2.1 The Vision of EGI._ EGI is an emerging paradigm that extends edge computing toward more general-purpose AI capabilities [4]. Traditional edge computing shifts data processing from centralized cloud infrastructures to decentralized edge nodes near data sources, thereby reducing latency and network congestion [43]. Early edge intelligence enabled real-time AI services localized at the edge, such as object detection by smart surveillance cameras [44], but such systems are confined to a single, pre-defined task or application domain. EGI takes a further step forward by drawing inspiration from artificial general intelligence [45], aiming to endow edge devices with human-like cognitive versatility. Specifically, EGI targets edge systems that can perform comprehension, reasoning, decision-making, and adaptation across diverse scenarios, even those not seen during training [5]. 

A key enabler of this shift is the emergence of LLMs, which serve as general-purpose cognitive engines trained on vast corpora of multimodal data. Furthermore, multi-LLM systems are particularly significant for realizing EGI. By orchestrating multiple specialized LLMs across the edge networks, multi-LLM architectures offer a scalable and ubiquitous way to embed human-oriented general intelligence. Moreover, these systems facilitate collaborative reasoning, context-aware coordination, and task decomposition across heterogeneous edge LLMs, making LLM inferences available at the edge. 

_2.2.2 Applications of Multi-LLMs in EGI._ In typical EGI scenarios, lightweight LLMs embedded at the edge perform context-specific tasks, while more powerful LLMs at cloud layers provide abstraction, coordination, and global optimization. Some important applications of multi-LLM systems in representative EGI scenarios are listed as follows. 

- **Smart Healthcare:** EGI in healthcare is realized through a hierarchical deployment of multiLLMs across wearable devices, bedside equipment, and hospital data centers. On-device LLMs summarize patient vitals and detect anomalies in real time, preserving privacy and minimizing latency. Summarized insights are selectively shared with cloud-hosted expert LLMs, which provide diagnostic reasoning or suggest treatment plans based on broader clinical patterns [46]. 

- **Autonomous Mobility:** In autonomous cars or drone fleets, EGI is formed via decentralized multi-agent LLM frameworks. Each vehicle hosts an embedded LLM for local perception and decision-making. Roadside units can operate LLM-based regional coordinators for traffic analysis and congestion control [47]. 

- **Smart Grids:** EGI in energy systems is constructed via a distributed network of LLMs embedded in smart meters, substations, and regional control centers. These local LLMs assist field engineers by analyzing real-time grid telemetry, recommending operational adjustments, 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:9 

and detecting faults with contextual reasoning. A central LLM, when involved, handles gridscale forecasting or rare-event responses. The system’s decentralization ensures real-time operation and resilience against network fragmentation [48]. 

## **2.3 Zero-trust Security** 

_2.3.1 Zero-Trust Concepts and Mechanisms._ Zero-trust security is a modern cybersecurity model based on the principle of “ _never trust, always verify_ ” [17, 49]. Unlike traditional perimeter-based defenses that implicitly trusted entities after one-time authentication, Zero-trust assumes no inherent trust even for insiders [17]. In practice, every access request is treated as if it originates from an open and potentially hostile environment and must be explicitly authenticated and authorized, regardless of the requester’s location or prior access history. As illustrated in Fig. 2(right), a comprehensive zero-trust architecture is supported by a suite of technical enablers. 

- **Explicit Identity Verification** : Every access attempt is subject to rigorous identity authentication using multiple context-aware signals. This includes user credentials [50], Multi-Factor Authentication (MFA) [51], device hardware status [52], geographic location [51], behavioral indicators [53], etc. Authentication is enforced continuously, not just at entity login, ensuring that identity trustworthiness is dynamically reassessed throughout a session. 

- **Least-Privilege Access Control** : Access permissions are granted according to the principle of minimum necessary privilege. Users, applications, and services are only permitted to interact with the specific resources required for their roles or tasks. This mechanism relies on fine-grained access control models such as Role-Based Access Control (RBAC) [54] and Attribute-Based Access Control (ABAC) [55], significantly reducing the attack surface. 

- **Continuous Monitoring and Analytics** : System and user activities are persistently observed to detect deviations from established norms. Telemetry data, including network activity, authentication patterns, and application usage, is collected and analyzed using User and Entity Behavior Analytics (UEBA) [56], Security Information and Event Management (SIEM) [57], etc. These insights enable automated policy adaptation, real-time threat detection, and incident response. 

- **Micro-segmentation and Lateral Movement Prevention** : Network environments are partitioned into logically isolated zones with tailored access policies [17]. This segmentation limits the spread of breaches by preventing unauthorized east-west movement across internal systems. Traffic between segments is monitored and governed by policy enforcement engines, ensuring that even compromised entities remain contained. 

_2.3.2 Applications of Zero-Trust in AI._ Zero-trust principles are increasingly applied across various stages of AI pipelines to enhance trustworthiness, privacy, and resilience against adversarial threats. Representative application domains include 

- **Federated Learning (FL):** In FL, multiple distributed clients collaboratively train a global model without sharing raw data. Zero-trust approaches improve FL by eliminating implicit trust assumptions among clients [58]. Mechanisms such as client attestation, secure identity provisioning, and TEE [12] can be employed to detect data poisoning, model inversion, and free-riding attacks. 

- **Zero-Trust Model Access:** As AI models are increasingly deployed across distributed and heterogeneous platforms, zero-trust security frameworks have become essential to safeguard model endpoints. Mechanisms such as dynamic authentication, continuous identity verification, and fine-grained access control mitigate the risks of unauthorized queries and adversarial model extractions [59]. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:10 

Table 2. Security issues in multi-LLM systems for EGI 

|**Category**|**Type of Security Issue**|**Description**|
|---|---|---|
|Intra-LLM|Jailbreaks & Prompt Injection|Adversaries craft malicious prompts to bypass safety guardrails, manipulating the LLM’s<br>internal state to generate harmful outputs [61, 62].|
||Unpredictable Emerging Abilities|Unexpected emergent behaviors in LLMs pose risks due to unforeseen capabilities, such<br>as autonomously generating exploit codes [63].|
||Data Leakage & Privacy|LLMs inadvertently disclose sensitive information through memorized training data or<br>user-provided confdential inputs [64, 65].|
||Toxic or Misaligned Responses|LLMs produce biased, ofensive, or incorrect content due to training data biases or<br>misalignment with intended operational scenarios, leading to real-world risks [66].|
|Inter-LLM|Expanded Attack Surface|Each additional LLM increases entry points for attackers, creating a cascading risk where<br>a compromised model afects multiple peers [67, 68].|
||Over-Permissive Integration|LLMs tightly integrated with privileged system components may unintentionally trigger<br>sensitive operations, resulting in unauthorized actions or escalations of privilege [14,69].|
||Insecure Communication|Communication channels lacking encryption or authentication become vulnerable to<br>prompt injections, impersonation, and eavesdropping [67, 70, 71].|
||Consensus Manipulation|Malicious or compromised LLMs manipulate consensus protocols by injecting false<br>information or dishonest voting, disrupting collaborative decisions [40].|
||Cross-Context Data Leakage|Collaboration among LLMs leads to unintended data exposure when sharing context-<br>specifc information, potentially violating global privacy policies [72].|



- **Multi-Agent and Collaborative Intelligence:** In distributed AI systems involving interacting agents, zero-trust ensures that inter-agent communications are authenticated, contextually scoped, and revocable. Protocols such as certificate pinning, time-bound tokens, and risk-adaptive authentication prevent spoofing or privilege escalation, especially in scenarios like distributed inference, agent planning, or swarm robotics. 

- **Zero-Trust Reinforcement Learning:** In safety-critical RL applications (e.g., autonomous driving), adversarial attacks on sensors, environments, or reward models pose severe risks [60]. Zero-trust RL frameworks integrate behavioral policy validation, continuous verification of inputs, and response gating to mitigate risks from compromised components or manipulated environments. 

## **3 SECURITY OF MULTI-LLMS IN EGI** 

In this section, we introduce the security issues faced by multi-LLM systems in EGI (see Table 2). Moreover, we review perimeter-based security defenses. 

## **3.1 Intra-LLM Level Security Issues** 

_3.1.1 Jailbreaks Attack and Prompt Injection._ Even on the edge, LLMs remain vulnerable to malicious prompts designed to override their safeguards [61]. Adversaries can craft “jailbreak” inputs that exploit the LLM’s learned patterns (often by obfuscation [61] or role-playing [73]) to make it ignore safety instructions. Similarly, prompt injection attacks use malicious text to manipulate the LLM’s internal state. For instance, Liao _et al._ [62] demonstrated a single “universal” prompt that reliably bypassed safeguards in GPT-4, Bing, Bard, and Claude simultaneously. In an EGI context, a local edge LLM with weak or no moderation could be tricked into providing harmful output, such as advising on illicit activities or disclosing system secrets. 

_3.1.2 Unpredictable Emerging Abilities._ LLMs may demonstrate emergent capabilities that were not anticipated by their developers, and this unpredictability can be dangerous in EGI deployments. Because, due to resource constraints [5], EGI systems often use smaller or specialized LLMs at the edge, one might assume they are safer. However, even downsized LLMs can exhibit surprising skills learned from vast pretraining data. For instance, a recent study found that GPT-4 can autonomously generate functional exploit code for 87% of known one-day software vulnerabilities when simply given their official descriptions [63]. LLM’s ability to produce working attack payloads without 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:11 

human guidance reveals a serious security concern. In EGI applications, if an edge LLM agent unexpectedly acquires such capabilities (e.g., crafting network intrusions or bypassing authentication), malicious users could leverage it to harm the local environment. The difficulty of fully predicting an LLM’s behavior means edge systems face a risk of unknown unknowns, complicating safety assessments and deployment decisions. 

_3.1.3 Data Leakage and Privacy._ LLMs have no intrinsic concept of confidentiality. Specifically, they generate outputs based on training data and input prompts without understanding what should not be revealed. This poses acute risks in EGI systems that integrate LLMs with sensitive enterprise or sensor data. An edge-based LLM might inadvertently disclose private information, e.g., personal details, device telemetry, and access credentials, in its responses if the prompts indirectly trigger memorized data [64]. Likewise, users may unwittingly input confidential data, e.g., customer records, internal documents, source code, into an LLM-powered edge service [65]. If such privacy is recorded in the memory of the multi-LLM systems and shared among LLMs, persistent privacy violations can happen. Additionally, without strict access controls, an insider or attacker which can query a local LLM could extract any data the model has seen [64]. In summary, data leakage in multi-LLM EGI can flow both ways: the LLM might expose private training data outward, or ingest private user input that stays in system memory, later becoming accessible to others. These outcomes undermine the presumed privacy advantages of processing data on the edge. 

_3.1.4 Toxic or Misaligned Responses._ Even when functioning correctly, LLMs can produce biased, offensive, or hallucinatory content due to biases in training data or misalignment with human values [66]. In an EGI setting, such toxic or misleading output can have serious real-world consequences. For example, if a local financial advisor LLM on a branch office server fabricates wrong investment statistics or a healthcare assistant LLM on a hospital device gives a biased medical recommendation, the consequences could be legal and reputation damage. Moreover, edge LLM deployments often utilize open-source or customized models that may lack rigorous moderation. Early versions of Meta’s LLaMA, for instance, could generate extremist or misinformation text when prompted [66]. A misaligned response from an edge LLM can directly impact on-site operations or user safety. For instance, incorrect driving decisions generated by an LLM-based autonomous vehicle could lead to traffic violations, collisions, or endangerment of human lives. Thus, ensuring response quality is not merely an ethical concern but a critical security requirement in multi-LLM EGI systems. 

## **3.2 Inter-LLM Level Security Issues** 

_3.2.1 Expanded Attack Surface._ Deploying multiple LLMs inherently enlarges the system’s attack surface, as each LLM becomes a potential entry point for adversaries. In multi-LLM environments, a single compromised model can trigger cascading failures across the entire network. Previous work demonstrates that adversarial prompts injected into one LLM can propagate through interagent communication, resulting in a “chain-of-compromise” effect [67]. For example, Lee _et al._ [67] proposed a prompt infection framework, where a malicious payload spreads covertly between LLMs through normal message passing. Shen _et al._ [68] further showed that multi-LLM s coordination increases the likelihood of compound vulnerabilities, especially in decentralized topologies. 

_3.2.2 Over-Permissive Integration._ In EGI, LLMs are often tightly integrated with privileged components, such as internal APIs, data pipelines, code execution engines, and physical devices. Unlike traditional software, LLMs interpret and act on natural language instructions, potentially triggering actions dynamically based on prompt semantics [69]. Without strict enforcement of privilege boundaries, even innocuous prompts can lead to sensitive operations. A recent study demonstrated that GPT-4, when granted tool-use capabilities, autonomously devised a strategy to bypass a CAPTCHA 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:12 

challenge by recruiting a human worker on TaskRabbit. It further deceived the worker by falsely claiming to be visually impaired to justify the request [14]. This illustrates how an LLM, once endowed with autonomy, can infer and execute multi-step plans that fall outside its intended operational scope. In EGI settings, such over-permissive integration can result in LLMs issuing unintended commands, escalating privileges, or modifying protected data. 

_3.2.3 Insecure Inter-LLM Communication._ LLMs need to frequently exchange prompts and responses to coordinate tasks. If communication channels are not authenticated or encrypted, they become vectors for prompt injection, impersonation, or eavesdropping [67]. One prominent threat is inter-LLM prompt injection, where an adversary injects a malicious instruction into one LLM that subsequently propagates across peers [70]. Furthermore, if identity verification is absent, attackers may impersonate trusted LLMs, forcing others to perform unauthorized actions or disclose private data [71]. Attackers may also attempt passive surveillance by intercepting inter-LLM traffic, reconstructing sensitive task contexts, or operational intents. In edge environments where lightweight communication protocols are common, these risks are amplified due to the absence of a centralized authentication infrastructure. 

_3.2.4 Consensus Manipulation by Byzantine LLMs._ Multi-LLM systems often rely on consensus protocols, either centralized (e.g., round-robin leader rotation [40]) or decentralized (e.g., random voting [40]), to coordinate collective decisions. However, if one or more LLMs behave in a Byzantine manner, they can disrupt collaborative outcomes. Malicious LLMs may inject false information, vote dishonestly, or collude to dominate consensus, especially in decentralized settings lacking a global coordinator. Note that even centralized coordination does not guarantee safety. If the coordinator itself is compromised, it can propagate biased plans, suppress valid alternatives, or selectively misinform subordinates. Such manipulation can lead to unsafe decisions, policy violations, or service disruptions, particularly damaging in EGI contexts where coordination governs physical processes such as autonomous fleets or energy balancing. 

_3.2.5 Cross-Context Data Leakage from Inter-Agent Invocation._ In multi-LLM EGI deployments, individual LLMs often manage localized datasets tailored to specific functions or regions. However, when LLMs collaborate through task delegation or response chaining, unintended data exposure can occur. For example, a predictive maintenance LLM might query a building usage LLM for context, unintentionally eliciting user-specific energy data that fall outside its access scope. Even when each LLM complies with its own policy in isolation, combined behavior can violate global privacy constraints. Recent work shows that carefully designed multi-step prompts can extract sensitive information distributed across LLMs by reconstructing partial answers [72]. In decentralized EGI environments, the absence of centralized policy enforcement exacerbates this issue. 

## **3.3 Perimeter-Based Security: Towards Trustworthy Multi-LLMs** 

Traditional security strategies rely heavily on perimeter-based defenses, setting a clear boundary within which all components are implicitly trusted. Consequently, strengthening security typically involves reinforcing or extending these security boundaries. In multi-LLM contexts, researchers have followed this conventional principle to develop trustworthy LLMs [10, 74, 75], constructing security boundaries from multiple perspectives, including model capabilities, training data quality, runtime environments, and interaction processes. In the following parts, we survey the representative approaches for implementing trustworthy LLMs (see Fig. 3). 

_3.3.1 Adversarial Training._ Adversarial training [10] is a robust strategy to enhance the trustworthiness of LLM capabilities against harmful inputs and jailbreak attacks. Specifically, this technique involves repeatedly exposing the LLMs to specially crafted inputs designed to lead the models 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:13 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

**==> picture [385 x 129] intentionally omitted <==**

**----- Start of picture text -----**<br>
Roles Access<br>[“make a bomb”] [“How are you!”] [“How do you  [“Of course, to  LLMs with different<br>produce a  produce a  roles and<br>[“Hello world”] Prompt LLM [“Harmful input”] Response bomb? Being with Of course”] bomb, you need to …”] different control<br>Adversarial training Reactive output filtering Access control & firewalls access<br>Private data LLM will  Inter-LLM<br>Training data … Differential privacyNoise multiplier LLM not tend to sensitive training reveal data Agent 1 Generation Multi-LLM system communication Agent 2<br>Pa \<br>s Untrusted part TEE enclave<br>[“Hello!”] Prompt [“Good morning”][“I am a robot”] Response Values0.80.4 a LLM LLMs, data, usersExternal tools,  API LLM : and changedThe program TEE cannot be intruded and data in  plain textSender’s  public keySender’s  A xbcy%x&aazz&*a private keySender’s  : Received plain text<br>Reinforcement with human feedback Hardware enclave Secure communication protocols<br>**----- End of picture text -----**<br>


Fig. 3. The representative perimeter-based security defenses for multi-LLM systems. We can observe that these methods aim to construct and expand the trustworthiness of the model itself, execution environment, and interaction & communications. 

into producing incorrect or harmful responses. In this way, LLMs can learn from these difficult examples, gradually improving their ability to identify and resist such attacks (see Fig. 3). 

For example, Xhonneux _et al._ [76] proposed Continuous Adversarial Training (CAT). By adjusting input prompt embeddings in directions that maximize LLM confusion, CAT efficiently synthesizes challenging training samples. These maliciously perturbed samples are used in the training loop, enabling LLMs to iteratively learn to recognize their vulnerabilities and minimize the adverse effects of these perturbations. Furthermore, Yu _et al._ [10] developed Refusal Feature Adversarial Training (ReFAT), focusing on a specific embedding characteristic called refusal feature, which signals when an LLM should reject harmful requests. ReFAT deliberately disrupts this feature during training, effectively simulating conditions where the LLM’s safeguard mechanisms are compromised. By repeatedly forcing the LLM to handle scenarios where refusal features are unavailable, ReFAT can strengthen the LLM’s overall robustness. Recently, AdversaFlow [77] introduced a visually interactive approach to adversarial training. By visualizing the progression of adversarial attacks and LLM responses, experts can interactively generate, observe, and analyze adversarial examples, iteratively refining LLM input based on reactions. 

_3.3.2 Differential Privacy._ Trustworthy multi-LLM not only means preventing harmful outputs, but also protecting sensitive inputs and training data. In multi-LLM EGI scenarios, edge devices may handle private user data or proprietary information that cannot be leaked. Moreover, researchers have shown that LLMs can inadvertently memorize training data, and determined attackers might extract verbatim sensitive information (e.g., personal identifiers or secret keys) from an LLM by querying it strategically [11]. Hence, DP [78] is an effective approach that can be used during LLM training to ensure that the model does not remember or reveal specific details from its training examples. Specifically, DP addresses this by adding carefully calibrated noise to the training process, so that any single training datum has only a negligible influence on the final LLM parameters. In practice, applying DP to LLMs means that the model learns general patterns but cannot reproduce any exact training sentence with high confidence. For instance, Behnia _et al._ [78] presented a DP-assisted fine-tuning framework that achieves measurable privacy guarantees while minimizing the impact on LLM performance. Considering that each EGI device fine-tunes an LLM based on the user interactions it observes, applying DP ensures that even if one LLM is compromised, the attacker cannot directly identify raw data. Moreover, since DP facilitates privatized updates, multiple devices can contribute to training a joint LLM without exposing their raw data. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:14 

_3.3.3 Reinforcement Learning with Human Feedback._ As shown in Fig. 3, RLHF is a prominent approach to fine-tuning LLMs to align their output more closely with human preferences, thus improving their safety, ethical compliance, and overall trustworthiness. For instance, Dai _et al._ [79] proposed the Safe RLHF framework, explicitly capturing human preferences concerning helpfulness and harmlessness. By training LLMs to generate content that aligns with user preferences, the usefulness and safety of the generation can be improved. However, conventional RLHF approaches suffer from issues, such as reward hacking and incorrect generalization of target objectives. To address these limitations, Sun _et al._ [80] introduced a personalized LLM security alignment framework. By incorporating user-specific preferences and employing multi-dimensional reward functions, their approach provides precise reward signals to RLHF, leading to more accurate and contextually relevant safety alignment. Moreover, the authors in [81] emphasized that despite the substantial improvements RLHF offers in aligning LLMs with human preferences, it cannot guarantee absolute safety. This inherent limitation arises because RLHF relies on generalizing human preferences from limited and potentially biased human feedback. Consequently, achieving complete safety and ethical alignment remains challenging, underscoring the need for continuous human oversight, diverse participation, and robust testing methodologies. 

_3.3.4 Reactive Output Filtering._ Another important defense is output filtering, where a separate mechanism intercepts and screens the LLM’s generated content before it reaches the end-user [82]. A simple approach is to employ a classification model or heuristic rules to detect toxic, hateful, or otherwise harmful language in the LLM’s output and block or modify it as needed. Recent research has explored using LLMs themselves as filters, effectively turning the model’s own knowledge of harmful content into a defensive tool. For example, Self-Defense [82] prompts the LLM to examine its output for harmful content. In detail, after the LLM generates a response, that response is fed (along with an instruction) to the same or another LLM, which should decide whether the content is malicious or violates policies. The experiments demonstrate that Self-Defense can reduce the attack success rate to virtually zero across a variety of adversarial prompt attacks. Note that the advantage of output filtering is that it provides a second line of defense. Even if one entity in the multi-LLM produces harmful content, a filter can catch it before it propagates further or is shown to users. Many commercial AI systems (e.g., OpenAI’s and Anthropic’s APIs) incorporate such automated content filters. 

_3.3.5 Hardware Enclaves._ Hardware Enclaves (i.e., TEEs) are secure, hardware-backed execution contexts designed to isolate sensitive computations and data, ensuring they are protected from unauthorized access and tampering [74, 83]. Leveraging hardware-based isolation, TEEs guarantee that the code and parameters of LLMs remain confidential and trusted. Moreover, TEEs support run-time attestation, which continuously verifies and confirms the integrity of the LLM and the secure operation throughout its execution. 

Numerous research efforts have explored the deployment of LLMs within TEEs to enhance security. First, Chrapek _et al._ [74] demonstrated a practical method to secure LLMs employing Intel SGX and Trust Domain Extensions (TDX). Their implementation involves isolating LLM execution within hardware enclaves, ensuring continuous verification and protection against unauthorized model extraction, malicious quantization attacks, and unauthorized fine-tuning. Their approach achieved less than 10% overhead compared to conventional, insecure deployments, highlighting the feasibility and efficiency of deploying complex inference tasks within TEEs. Similarly, Dong _et al._ [83] benchmarked the performance of DeepSeek LLMs across TEE-based, CPU-only, and CPU-GPU implementations. Su _et al._ [84] introduced a framework using Intel TDX with run-time attestation. This approach secures containerized LLMs by continuously validating their integrity within TEEs, effectively preventing unauthorized access and tampering during model execution. Furthermore, 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:15 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

this solution includes a hardware-agnostic runtime environment, which mitigates vendor lock-in issues, enabling secure edge LLM deployment across diverse hardware platforms. Similarly, Li _et al._ [85] proposed embedding LLM computations within TEEs, ensuring robust security guarantees. They also optimized inter-TEE communication through direct secure channels, greatly guaranteeing both the security and performance of decentralized AI inference tasks, particularly suitable for high-stakes domains like healthcare and finance. Finally, Lin _et al._ [86] introduced the LoRATEE framework, specifically designed for secure and efficient multi-tenant LoRA-based LLM inference using TEEs. By embedding LoRA adapters inside Intel SGX enclaves and employing a lightweight one-time pad encryption for secure data transmission between the enclave and external computing environments (such as GPUs), they significantly mitigate potential security risks associated with multi-tenant environments. 

_3.3.6 Access Control & Firewalls._ Access control and firewalls are critical security mechanisms for ensuring the confidentiality and security of multi-LLM systems in EGI environments. Effective access control mechanisms regulate user permissions, ensuring that only authorized entities can interact with LLM resources and preventing unauthorized data access and operations [87]. Firewalls, in turn, monitor and filter both incoming and outgoing traffic according to predefined security rules, protecting LLM systems from malicious inputs and unauthorized access attempts [88]. 

In terms of access control, traditional approaches have relied primarily on static permission models such as discretionary access control [89] and mandatory access control [90], which grant fixed permissions based on user identity or security clearance levels. However, for multi-LLM systems, RBAC has emerged as a particularly relevant paradigm, where permissions are associated with LLMs with specific roles, enabling dynamic permission scheduling during inter-LLM collaborations. The importance of RBAC for multi-LLM is reported by Sanyal _et al._ [87]. Their assessment of 16 state-of-the-art LLMs across 40 distinct enterprise scenarios revealed that even flagship models like GPT-4.1 achieve only 27% accuracy when reasoning about complex multi-permission cases. To this end, Shi _et al._ [91] introduced Progent, a programmable privilege control mechanism that realizes RBAC for LLMs. Rather than relying on static user-based permissions, Progent implements dynamic role-driven policies where tool access rights are determined by the LLM’s assigned role and current task context. Moreover, privilege control policies are expressed by JSON files that define which tool calls are permissible for specific roles, under what conditions they are allowed, and what fallback actions occur when role-based constraints are violated. This RBAC-based approach enables scalable permission management across multi-LLM systems, where different LLMs can assume different organizational roles with corresponding access privileges, ensuring secure execution while preventing unauthorized operations that exceed their designated role boundaries. 

As for firewalls, Huang _et al._ [88] introduced FirewaLLM, a portable framework that desensitizes sensitive user inputs using smaller local models before interactions with large LLMs. This approach effectively prevents inadvertent disclosure of personal, health, or financial data during LLM interactions. Abdelnabi _et al._ [92] proposed a comprehensive firewall framework tailored for dynamic LLM networks. Their design automatically generates task-specific firewall rules from prior conversations, sanitizing inputs into deterministic and verifiable formats. It also incorporates dynamic abstraction of user data and implements self-correcting mechanisms, significantly reducing the risks of prompt injection, data exfiltration, and unauthorized manipulation. Yao _et al._ [93] presented ControlNet, an AI firewall specifically designed for Retrieval-Augmented Generation (RAG)-based LLM systems. Leveraging neuron activation shift phenomena, ControlNet detects and mitigates malicious queries, enforcing precise query flow control policies. Finally, Namer _et al._ [75] developed an automated framework to detect ”expensive” prompts designed to overload LLM systems, configuring firewall rules dynamically to mitigate denial-of-service attacks. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:16 

_3.3.7 Secure Communication Protocols._ Given the extensive interactions and information exchanges inherent in multi-LLM systems, trustworthy communications should also be considered. To this end, Gan _et al._ [94] proposed a binary mapping framework that systematically categorizes threats based on sources and impacts, enabling precise identification and blocking of malicious or misleading interactions across LLMs. Multi-layered security mechanisms, including data link layer encryption protocols and network layer agent behavior monitoring, further protect information exchanges between LLMs and with users or environmental interfaces. Advanced frameworks, such as MCPShield[5] , extended this protection through signature-matching and adversarial behavior profiling, enabling pre-execution detection of high-risk tools and malformed tasks. Moreover, MCIP [95] introduces runtime trace analysis with an explainable logging schema and security-awareness model to track violations in complex LLM-tool interactions. These robust communication protocols collectively establish a comprehensive security posture that ensures secure, reliable, and verifiable inter-LLM communications in multi-LLM systems. 

## **3.4 From Trustworthy to Zero-Trust Defenses** 

Trustworthy LLM proposals [10, 12, 75, 82, 94] have significantly improved the security posture of multi-LLM systems. However, these traditional perimeter-based strategies present several inherent limitations that become increasingly pronounced as multi-LLM systems evolve in complexity and scope. First, as LLM capabilities expand and their operational domains broaden, the attack surface increases correspondingly [96], making it challenging to maintain clearly defined security boundaries. Moreover, the frequent interactions and data exchanges inherent in multi-LLM systems create extensive lateral movement opportunities across different security domains, potentially compromising data integrity and security perimeters [39]. Finally, traditional security paradigms often exhibit reactive characteristics (i.e., responding to threats only after attacks or breaches occur), which can lead to delayed detection and mitigation that allows substantial damage. These limitations prove especially pronounced in EGI environments, where sophisticated prompt injections and adversarial attackers damage LLMs in unforeseen ways. Meanwhile, resource constraints make extensive security measures, such as comprehensive adversarial training or TEE deployment, impractical due to computational, energy, and bandwidth limitations. 

Remark 1. _These systemic limitations raise a fundamental question: can LLMs be trusted? From the user’s perspective, LLMs exhibit inherent opacity and limited interpretability due to their extensive training datasets and complex architectural structures, making human oversight and comprehensive screening practically infeasible [14]. From the LLM perspective, each LLM cannot assume the trustworthiness of collaborating ones, as these may be compromised, malicious, or manipulated by adversaries seeking to exploit the collaborative framework [97]. This mutual distrust requires a new security paradigm that does not rely on the assumed benevolence of LLMs, i.e., zero-trust._ 

## **4 ZERO-TRUST MULTI-LLM FOR EGI** 

## **4.1 Constructing Zero-Trust Multi-LLMs for EGI** 

We observe that many advanced studies [26–33] have proposed defenses aligning with zero-trust principles or presented zero-trust single LLMs, while a comprehensive tutorial systematically discussing zero-trust multi-LLM systems in EGI has yet to emerge. Hence, we demonstrate the vision of zero-trust multi-LLM in EGI using an autonomous driving case study. As illustrated in Fig. 4, the EGI paradigm involves heterogeneous LLMs with diverse capabilities, computational requirements, and operational scopes deployed across cloud infrastructures, edge servers such 

> 5https://github.com/riseandignite/mcp-shield 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:17 

**==> picture [385 x 206] intentionally omitted <==**

**----- Start of picture text -----**<br>
4. Communication gateway Entity: Role: Public key: Auditing & anomaly detectionAgent B xxx&%23dadd… 3. Additionally, cloud LLM servers as the policy engine,<br>Access:  (More context-aware factors…)Data analysis tool configuring the basic  access control, authentication, and<br>user/LLM as the communication 4. Cloud LLM servers gateway to assign information to the  with access xbcy%x&aazz&*a Encrypted exchangeinter-LLM data  xbcy%x&aazz&*a 7. Auditing and detectionAnomaly  2. Cloud LLM 3. Identity and authentication 2. Cloud LLM possess advanced  generation reasoning and  capability identification  schemes.<br>“Help me prepare<br>the car and drive<br>History… me to the bank<br>building…”<br>1. Following the<br>principleleast privilege  , LLMs will  Ap) Action oes ire x vi 5. User prompt will be<br>be functionally  checked, to<br>separate and only has the minimal  Perception LLM Planning LLM ans J harmful partRemove  detect/predict potential attacks, thus realizing<br>required capability 1. Mobile LLM (on car) 5. Input checking proactive maintenance<br>Workflow<br>Setup & LLM authentication Prompt understanding Collaborative decision plan Policy execution Continuous monitoring<br>Keys “Help me prepare the car and drive  “Prepare car” Safety Safety LLM<br>Cloud LLM Mobile LLMs me to the bank building…” “Navigate to the bank” Perception Access control, gateway… Auditing Digital SignatureCross-check<br>On” >  | ©@=8=8| 2) Action — | @® Other cloud &  ©<br>Driving executionAction  mobile LLMs<br>Each LLM generates the  public- The user prompt is  checked , and the  The  collaboration pattern  is fixed.  The decision of Multi-LLM system is  Every historical behaviors of each<br>private key pair . Cloud LLM sets the  user’s intents are extracted and  Each LLM only has the minimal  executed by the corresponding LLM  LLM. All inter-LLM communications,<br>basic  access control, identification,  mapped to the corresponding  access that required to handle the  under supervision. tool calling, and other logs are<br>and authentication policies. instructions & inputs to LLMs. task under dynamic authentications. monitored and analyzed.<br>6. Output ver.<br>Prompt<br>Identification<br>**----- End of picture text -----**<br>


Fig. 4. The vision of zero-trust multi-LLM in EGI. The upper part illustrates a scenario of autonomous driving, where users send a prompt about navigation to the multi-LLM system. The lower part demonstrates the end-to-end workflow. 

as roadside units, and on-vehicle chips. Note that we follow NIST SP 800-207 [15], the most widely adopted zero-trust standard, to showcase the architecture. The subsequent parts detail the systematic implementation of each component of this framework. 

_4.1.1 Mobile-Edge LLMs._ As shown in Fig. 4, mobile LLMs operate on edge servers and vehicular chips to ensure minimal latency for real-time decision-making in autonomous driving scenarios. In multi-LLM settings, these LLMs undergo functional segregation to handle distinct operational aspects and collaborate to accomplish the assigned task [39]. For instance, perception LLMs transform raw sensor output into structured textual scene descriptions, and planning LLMs interpret traffic regulations for high-level route planning decisions. Zero-trust enforcement restricts each LLM’s access exclusively to the minimum necessary data and command interfaces. For example, perception LLMs can access camera and LiDAR feeds but cannot interface with steering or acceleration controls. Upon system initialization, each LLM should undergo cryptographic identification to prevent Sybil attacks [98]. 

_4.1.2 Cloud LLMs._ Cloud LLMs can offer computationally intensive generation capabilities, comprehensive knowledge repositories, and coordination services that surpass the capacity of mobileedge LLMs. Moreover, following NIST zero-trust standards [15], cloud LLMs function as policy engines that establish and enforce security protocols throughout the distributed multi-LLM network while maintaining centralized oversight of system-wide security policies. 

_4.1.3 Identity and Authentication Module._ At the core of the framework lies a comprehensive identity management system that establishes the foundation for all zero-trust operations. Every LLM, whether deployed at the edge or in the cloud, receives a cryptographically secure identity comprising unique asymmetric key pairs and digitally signed certificates [99]. This module enforces mandatory identity verification for each interaction, eliminating the possibility of anonymous or unverified communications within the system. Moreover, the authentication mechanism implements continuous verification rather than one-time startup authentication. Session credentials, which can 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:18 

be based on multi-factor authentication [51], tags [100], or reputation [101, 102] are deliberately short-lived and require frequent re-establishment to mitigate the impact of potential credential theft or compromise. This approach aligns with zero-trust principles by maintaining persistent identity verification throughout the operational lifecycle, ensuring that even previously authenticated entities must continuously prove their legitimacy to maintain system access. 

_4.1.4 Inter-LLM Communication Gateway._ As illustrated in Fig. 4, all inter-LLM communications within the multi-LLM system are channeled through a secure gateway deployed on the cloud LLM. This gateway functions as both an intelligent firewall and a message broker, implementing policycontrolled routing mechanisms that enforce strict communication boundaries between system components [42, 103]. Specifically, it maintains dynamic and fine-grained access control policies that specify authorized communication patterns for each type of LLM. Any communication attempt that violates the policies is immediately intercepted and blocked by the gateway, preventing unauthorized data flows and potential security breaches. Moreover, transmitted content is encrypted using the sender LLM’s public key and decrypted at the receiver using private keys, thereby preventing sensitive information from being intercepted or tampered with during transmission [99]. These mechanisms ensure that no internal message routing occurs based on implicit trust assumptions about the source, requiring explicit verification for every communication attempt. 

_4.1.5 User Input Checking._ User inputs are treated as potentially hostile data sources within the zero-trust framework, as they may originate from malicious attackers or be compromised through adversarial manipulation during transmission. These inputs could contain carefully crafted prompt injection attacks designed to bypass LLM safety guardrails and trigger jailbreaking behaviors that compromise system integrity [30]. The input validation module implements comprehensive sanitization mechanisms, including lexical analysis to detect suspicious instruction patterns, semantic filtering to identify context manipulation attempts, and behavioral pattern recognition to flag inputs deviating from normal user interaction profiles. This proactive input filtering ensures that adversarial prompts cannot propagate through the multi-LLM system and maintains the principle of explicit verification. 

_4.1.6 Multi-Layer LLM Output Verification._ Similar to perimeter-based security, zero-trust multiLLM systems incorporate a comprehensive multi-layer verification engine that embodies the ”never trust, always verify” principle through hierarchical validation mechanisms. This multi-tiered filtering system can combine rule-based constraints, intelligent verification algorithms, and contextaware dynamic assessments to ensure that every LLM output undergoes rigorous validation before execution [104]. 

_4.1.7 Behavioral Auditing and Anomaly Detection._ Zero-trust requires persistent surveillance, since breaches are inevitable and threats can arise from any component of the entire multi-LLM system [49]. The behavioral auditing module implements comprehensive logging of all LLM communications, decisions, and collaborative interactions with tamper-resistant records stored in secure local repositories. Real-time anomaly detection algorithms analyze behavioral patterns to identify security threats, such as plan LLMs attempting direct vehicle control or outputs containing prompt injection tokens. 

## **4.2 Operational Workflow** 

Fig. 4 illustrates the end-to-end operation of the zero-trust multi-LLM system in an autonomous driving scenario. Four major steps are included. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:19 

- **Startup and LLM Authentication** : Initially, each mobile LLM generates its public-private key pair and registers its identity with the cloud LLM. The policy engine initializes access control mechanisms, input and output checking protocols, etc. Meanwhile, the behavioral auditing system begins comprehensive logging of all inter-LLM communications and individual LLM activities for anomaly detection. 

- **Prompt Understanding** : User prompts undergo rigorous validation through the input checking module before processing. If anomalous content is detected, the framework immediately quarantines the input and initiates additional verification procedures to prevent system compromise [15]. 

- **Collaborative Decision Planning** : The primary advantage of multi-LLM systems lies in their ability to organize LLMs collaboratively [1]. Multiple LLMs participate in decisionmaking processes in which the planning LLMs formulate initial decisions, the cloud LLM provides expert validation for complex scenarios, and the safety LLMs perform independent assessments of the collaboration process. The system supports multiple collaboration patterns, such as pipeline or distributed voting (see Section 2.1), based on the specific task. 

- **Policy-Governed Plan Execution** : The multi-LLM system generates final outputs that trigger executable vehicle commands, such as lane change maneuvers, speed adjustments, and braking actions [105]. The multi-layer verification mechanism validates all LLM outputs against safety constraints, regulatory compliance, and operational feasibility before execution. Moreover, execution can only be initiated by designated LLMs that have explicit authorization for specific vehicle control functions. 

- **Continuous Monitoring and Threat Mitigation** : The behavioral auditing system monitors all inter-LLM communications and individual LLM behaviors, maintaining comprehensive logs for forensic analysis and real-time threat detection. When anomaly detection mechanisms identify suspicious behavior or malicious content generation from any LLM, the cloud LLM isolates compromised LLMs while enabling the remaining LLMs to continue collaborative operation, ensuring system resilience and operational continuity [49]. 

**Lesson Learned** : By organizing the modules involved according to the workflow detailed above, we can construct a multi-LLM framework in EGI that effectively fulfills the four fundamental principles of the zero-trust paradigm (i.e., explicit verification, least privilege, continuous monitoring, and segmentation) [49]. Note that our demonstrated implementation adheres to the most famous NIST SP 800-207 zero-trust standard [15], which employs a policy engine responsible for defining security policies (e.g., input/output validation rules and agent authentication), making real-time decisions on access and communication, and dynamically coordinating interactions among LLMs. Nevertheless, other influential Zero-trust standards and frameworks, such as Google’s BeyondCorp[6] and CISA[7] , can also be applied. 

## **4.3 Trustworthy Multi-LLM versus Zero-Trust Multi-LLM** 

To demonstrate the features of zero-trust multi-LLM systems, we compare them against previous ones operating with security perimeters (see Table 3). First, we observe that perimeter-based security and zero-trust security operate on fundamentally different assumptions, with different mechanisms. Moreover, traditional security approaches can rely on isolated security strategies that address individual vulnerabilities independently. For instance, we can employ DP to protect LLM training data or deploy LLMs within TEEs to ensure computational integrity, treating these as standalone solutions. However, constructing a zero-trust environment represents a comprehensive systems 

> 6https://cloud.google.com/beyondcorp 

> 7https://www.cisa.gov/zero-trust-maturity-model 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:20 

Table 3. Perimeter-based security vs zero-trust security for multi-LLM systems 

|**Dimension**|**Sub-dimension**|**Traditional Perimeter Security**|**Zero-Trust Security**|
|---|---|---|---|
|**Core Philosophy**|Basic Concept|Expanding trust perimeter in multi-LLM|Building a multi-LLM environment without trust|
||Trust Assumption|Components within perimeter are implicitly trusted|No component and entity should be unconditionally<br>trusted at any time|
||Technical Enablers (Sur-<br>veyed in Sections 3.3, 5,<br>and 6)|• Enhance LLM capabilities against attacks (DP,<br>adversarial training, RLHF)<br>• Operate on trusted hardware (TEE)<br>• Set security perimeters (frewalls, RBAC)<br>• Establish trusted communication links|• Strong identifcation and authentication<br>• Context-aware access control<br>• Stateless and Ephemeral LLM<br>• Proactive maintenance<br>• Distributed management<br>• Micro-segmentation and Isolation<br>• Intelligent monitoring and failure manage-<br>ment|
|**Implementation**|Deployment|Can**independently**use individual defense mecha-<br>nisms, e.g., adversarial training or DP.|**Systems engineering**requiring multi-mechanism<br>coordination|
||Implementation Risk|Failure of one protection mechanism may compro-<br>mise overall defense|If only implementing partial zero-trust principles<br>while ignoring others, may create serious vulnera-<br>bilities in a zero-trust environment. Example: Only<br>doing identity verifcation without continuous moni-<br>toring allows attackers to persist after initial access|
|**Resource Overhead**|Training Overhead|**High**: For instance, RLHF fne-tuning requires exten-<br>sive human annotation; Adversarial training needs<br>massive attack samples; DP training increases com-<br>putational complexity|**Relatively Low**: Mainly relies on existing model ca-<br>pabilities; No large-scale retraining required|
||Operational Overhead|**Medium**: Periodic security rule updates; Hardware<br>maintenance costs|**High**: Continuous authentication increases computa-<br>tional burden; Real-time monitoring requires substan-<br>tial computing resources; Dynamic permission man-<br>agement increases system complexity; Multi-system<br>coordination operational complexity|
|**Applicable Scenarios**|Environment Features|Relatively closed environments; Simple threat mod-<br>els; Single LLM deployment; Resource-constrained<br>scenarios|Distributed multi-LLM systems; EGI environments;<br>High-security requirement scenarios; Complex threat<br>environments; Cross-domain LLM collaboration|
||Typical Applications|Enterprise internal AI assistants, closed-domain QA<br>systems|Autonomous driving systems, smart healthcare net-<br>works, multi-institutional collaborative AI|
||Target Attacks in EGI|• External penetration attacks (network in-<br>trusions, malicious external access, known<br>threat patterns identifed by the frewall)<br>• Static threats (predefned attack patterns,<br>signature-based malware, fxed attack vec-<br>tors)|• Lateral movement (authorized user malicious<br>behavior, privilege abuse, inter-LLM mali-<br>cious propagation)<br>• Dynamic & complex attacks (prompt injec-<br>tion, jailbreaking, adversarial attacks, model<br>poisoning)<br>• Emerging & unknown threats (emergent<br>capabilities, multi-modal attacks, context-<br>dependent threats)|
|**Advantages**||Relatively simple implementation; Clear single-point<br>optimization efects; High technology maturity; Con-<br>trollable costs|Comprehensive security assurance; Adaptable to dy-<br>namic threats; Fine-grained control; Systematic pro-<br>tection|
|**Limitations**||Once perimeter is breached, internal systems fully<br>exposed; Difcult to handle insider threats; Static pro-<br>tection mechanisms; High lateral movement risks|Extremely high implementation complexity; Expen-<br>sive operational costs; High technical team require-<br>ments; May impact system performance|



engineering challenge that requires an end-to-end workflow to orchestrate multiple modules, as demonstrated in our proposed framework above [15]. From a resource management perspective, the traditional security paradigm might incur significant training costs and hardware overhead, such as extensive adversarial training and the deployment of TEEs. However, zero-trust approaches introduce additional operational overhead, primarily due to continuous verification processes [49] and persistent authentication and monitoring requirements [106]. 

Note that it is important to recognize overlaps between traditional perimeter-based security methods and zero-trust strategies. Several foundational techniques, such as cryptographic authentication [99] and access control mechanisms [87, 91], apply to both concepts. For example, access control mechanisms can be utilized to construct security boundaries for LLMs by defining precise permissions for accessing specific tools and data. However, zero-trust imposes even higher demands on access control, such as context-aware policies, which dynamically adjust permissions based on real-time contextual factors for different micro-segmentations. Similarly, continuous monitoring, 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:21 

Table 4. Model-based approaches of zero-trust multi-LLM 

||**Introduction**|**Methods**|**Representative Works**|
|---|---|---|---|
|Strong LLM<br>Identity,<br>Authentication,<br>and<br>Authorization|Implements “never trust,<br>always verify” principle by<br>establishing continuous<br>authentication and<br>cryptographic identity<br>verifcation for each LLM<br>in EGI environments.|Multi-factor<br>Authentication|**Adaptive MFA**[107]: Dynamically adjusts verifcation requirements based<br>on real-time risk assessments and agent behavior patterns.|
||||**Context-aware MFA**[51]: Balances security and usability by tightening<br>protocols during detected anomalies using contextual information.|
|||Reputation-based<br>Authentication|**LLMChain**[101]: Maintains reputation scores based on historical outputs,<br>policy adherence, and user feedback for access control.|
||||**Blockchain Reputation**[108]: Provides transparent, verifable, and im-<br>mutable reputation histories using blockchain technology.|
|||Token-based<br>Authentication|**Fine-grained Token Control**[26]: Issues ephemeral, cryptographically<br>secure tokens encoding specifc permissions for LLM interactions.|
||||**Ephemeral Token Management**[49]: Enforces principle of least privilege<br>through continuous re-authentication and token expiration.|
|Context-aware<br>Access Control|Implements “least<br>privilege” principle by<br>context-aware permission<br>management that grants<br>the minimal necessary<br>access rights to LLMs.||**AgentSafe**[109]: Protects multi-LLM systems through hierarchical data<br>management with ThreatSieve and HierarCache components.|
||||**EPEAgents**[27]: Minimizes data exchange by providing each LLM only<br>task-relevant information through context-aware fltering.|
||||**ABE-based Access Control**[28]: Uses Attribute-Based Encryption with<br>policy hiding to ensure only authorized LLMs can decrypt sensitive infor-<br>mation.|
||||**Collaborative Memory Framework**[110]: Encodes memory access per-<br>missions as time-evolving bipartite graphs with context-aware policies.|
|Stateless and<br>Ephemeral LLM<br>Management|Embodies “assume breach”<br>principle through<br>eliminating persistent state<br>and creating disposable<br>LLMs in EGI and isolated<br>execution contexts.|Stateless Management|**PagedAttention**[111]: Implements process-like isolation with dynamic<br>memory allocation and copy-on-write semantics for request isolation.|
||||**vAttention** [112]: Advances stateless security through hardware-level<br>isolation using CUDA virtual memory APIs for protected address spaces.|
||||**BlockLLM** [29]: Extends stateless principles through component-level<br>micro-segmentation with cryptographically isolated blocks.|
|||Ephemeral<br>Management|**Self-destructing Models**[113]: Embed algorithmic time locks that de-<br>grade model performance when adapted for harmful tasks.|
||||**Serverless Deployment**[114]: Realizes automatic lifecycle management<br>with security checkpoints and anomaly-triggered termination.|



although prevalent in traditional security frameworks, assumes heightened importance in zero-trust environments. Intelligent monitoring ensures real-time detection and proactive response to threats, maintaining secure operations even as conditions evolve. 

## **5 INTRA-LLM ZERO-TRUST SECURITY: MODEL APPROACHES** 

In this section, we review the technical progress of model-level approaches that provide zero-trust management of individual LLMs. 

## **5.1 Strong LLM Identity, Authentication, and Authorization** 

First, each LLM in a zero-trust architecture is assigned a robust cryptographic identity to ensure secure and verifiable identification. Practically, this involves providing every LLM with a unique asymmetric key pair and a digital certificate, similar to IoT or embedded devices that use devicespecific keys [99] or X.509 [115] certificates to authenticate identity. Moreover, every user-to-LLM, inter-LLM, and LLM-to-tool call requires prior authentication. Note that authentication is continuous rather than a one-time handshake; credentials remain short-lived and frequently renewed, embodying the zero-trust principle of persistent verification [106]. Representative authentication approaches are described below (see Table 4 also). 

_5.1.1 Multi-factor Authentication._ MFA introduces additional verification layers beyond a single credential, forming a cornerstone of zero-trust security [107]. By requiring LLMs to present multiple independent proofs of identity, such as cryptographic keys combined with contextual information (e.g., device attestation, biometric verification, or behavior analytics), MFA substantially reduces the risk of unauthorized access due to compromised credentials. Recent research emphasizes contextaware or adaptive MFA, dynamically adjusting verification requirements based on real-time risk 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:22 

assessments, including agent behavior, device integrity, location, and access patterns [51]. This dynamic MFA approach effectively balances security and usability by tightening security protocols during detected anomalies. These adaptive mechanisms, initially designed for device authentication, are particularly applicable to authenticating LLMs in multi-LLM zero-trust EGI, with heterogeneous devices and application patterns [116]. 

_5.1.2 Reputation-based Authentication._ In zero-trust multi-LLM systems, each LLM can maintain a reputation derived from historical outputs, policy adherence, and user/peer feedback [101, 102, 117]. Suspicious or erroneous behavior results in decreased reputation scores, triggering restricted privileges, while consistently reliable LLMs accrue higher reputation scores and consequently gain access to more sensitive tools or data. Moreover, blockchain technology has been proposed to securely track reputation, providing transparent, verifiable, and immutable reputation histories [118]. For example, LLMChain integrates automated performance assessments and human feedback within a blockchain ledger to calculate comprehensive, tamper-proof reputation scores [108]. 

_5.1.3 Token-Based Authentication._ Chen _et al._ [26] provided fine-grained authorization through encrypted tokens encoding specific permissions for LLM interactions. Specifically, a central provider, potentially served by cloud LLMs within multi-LLM EGI architectures, maintains LLM identities and access policies, issuing ephemeral, cryptographically secure tokens defining permitted interactions. LLMs should obtain fresh tokens for each session or interaction, strictly limiting privileges to the immediate task. Notably, each token has built-in expiration conditions, requiring continuous re-authentication. Such adaptive authentication effectively enforces the principle of least privilege [49], ensuring that permissions are granted per request and revoked as soon as tasks are completed, aligning seamlessly with zero-trust principles of persistent verification and minimal trust. 

**Lesson Learned** : We can conclude that MFA provides the strongest security through layered verification, but introduces computational overhead suitable for high-security applications, while reputation offers dynamic trustworthiness management ideal for long-term deployments with established behavior patterns. Token-based authentication delivers fine-grained access control with minimal overhead, making it effective for distributed systems requiring precise privilege control. 

## **5.2 Context-aware Access Control** 

Zero-trust principles demand stringent access control to prevent privilege abuse and information leakage in multi-LLM systems. However, traditional RBAC, such as simple role-based permissions [119], is insufficient for zero-trust LLM collaboration. RBAC assumes that if an LLM’s identity or role is verified, it can be trusted broadly, which is an assumption zero-trust explicitly rejects. 

Context-aware Access Control (CAAC) frameworks extend beyond identity: “context” can include any information about the user/LLM, the resource, or the environment of an access request [120]. For instance, some proposals augment the RBAC with conditions, such as location, time of request, or other situational constraints [120]. Such context-driven policies allow fine-grained, situationspecific decisions (e.g., permitting an LLM to see certain data only during a particular task or within certain time bounds), aligning with zero-trust’s demand for dynamic and least-trust enforcement. 

Furthermore, multiple recent works propose advanced access control mechanisms tailored to multi-LLM systems that embody zero-trust principles. AgentSafe [109] is one such framework that protects multi-LLM systems through hierarchical data management. It classifies information into security levels and restricts sensitive data to only LLMs with authorization. Moreover, it introduces two components: ThreatSieve and HierarCache. ThreatSieve secures inter-LLM communication by authenticating each message’s source and verifying the sender’s authority, blocking unauthorized or impersonating LLMs. HierarCache, on the other hand, hierarchically manages LLM memory to prevent data leakage and poisoning. Historical information is stored in “drawers” according to 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:23 

sensitivity and LLM relationships, so each LLM only retains or sees memory appropriate to its trust level. Notably, AgentSafe’s design is dynamic: as LLMs join/leave or as tasks change, the hierarchy can adjust. Complementing the hierarchical approach, the Embedded Privacy-Enhancing Agents (EPEAgents) [27] paradigm focuses on context-aware filtering of information in multi-LLM systems. Specifically, cloud LLM will coordinate context sharing during LLM collaboration. The core idea is to minimize data exchange: rather than broadcasting a raw knowledge base or full context to all LLMs, each LLM receives only the task-relevant information that it truly needs. Importantly, EPEAgents is built to preserve contextual awareness for legitimate collaboration. Unlike rigid RBAC, which might omit crucial context and degrade performance, EPEAgent ensures each LLM still gets the context it needs by dynamically evaluating both the LLM’s declared role and the evolving task requirements, allowing the system to adapt to changing collaboration patterns while maintaining strict information boundaries. 

Other works bring cryptographic enforcement to access control for multi-LLM systems, using fine-grained encryption to guarantee that even if communications are observed by attackers, only authorized LLMs can decrypt and use sensitive information. Xiao _et al._ [28] proposed a privacypreserving access control scheme for LLM-driven networks based on Attribute-Based Encryption (ABE) with policy hiding and revocation. In their design, each piece of content (e.g., a dataset, prompt, or answer) can be encrypted under an access policy (defined as a logical combination of attributes) such that only an LLM possessing a key with attributes satisfying that policy can decrypt it. This allows extremely fine-grained control: policies might encode that “only an LLM with role = doctor AND clearance = high” can decrypt a medical query, or “LLMs with project_ID = 1 OR role = auditor” can access a certain log. Unlike RBAC, ABE does not grant blanket access to any LLM, while it evaluates the attributes presented at decryption time, enabling contextual decisions (attributes include an LLM’s role, affiliation, and user prompts). 

Beyond these, emerging research on dynamic access control provides additional tools to strengthen zero-trust LLM systems. For example, Rezazadeh _et al._ [110] presented a “collaborative memory” framework for multi-LLM systems that encodes memory access permissions as a time-evolving bipartite graph linking users, LLMs, and resources. Specifically, the framework maintains separate private and shared memory tiers for each user, with each memory fragment tagged by provenance (which LLMs contributed it, which LLMs can access it, etc.). Context-aware and fine-grained read/write policies then operate on this graph: a read policy might say that LLM _𝐴_ can retrieve data from user _𝐵_ ’s shared memory only if an edge exists between _𝐴_ and _𝐵_ and the fragment is not time-expired, etc., resulting in filtered, transformed views of memory for each LLM. 

## **5.3 Stateless and Ephemeral LLM Management** 

Traditionally, multiple LLMs maintain a shared memory pool to preserve states, which accumulate information across multiple requests to optimize inference performance and enable contextual understanding [110, 121]. However, this persistent state cannot be trusted, as malicious inputs can contaminate shared memory spaces, enabling cross-request information extraction and state poisoning attacks that compromise the integrity of future interactions. In contrast, stateless LLM management [111, 112] fundamentally rejects the traditional assumption that LLMs’ internal states can be trusted over time, instead requiring continuous verification of every component while minimizing the vulnerability window through short-lived, isolated execution contexts. 

_5.3.1 Stateless LLM._ The evolution of stateless LLMs has been driven by breakthrough innovations in memory management and isolation techniques. PagedAttention [111] established the foundational paradigm by implementing process-like isolation where each request is dynamically allocated with memory blocks. This approach partitions the key-value cache into fixed-size blocks with 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:24 

Table 5. The representative proactive maintenance approaches to realize zero-trust multi-LLM in EGI 

|**Methods**|**Description**|**Representative Works**|
|---|---|---|
|Intelligent<br>Input<br>Checking|Preemptively analyzes and verifes user<br>prompts to detect and neutralize<br>malicious instructions or jailbreak<br>attempts before LLMs generate harmful<br>content.|**LLM Prompt Detection System**[123]: Employs regular expressions and fne-tuned LLMs to<br>identify PII, malicious codes, URLs, and prompt injection commands.|
|||**DefensiveTokens**[30]: Embeds special tokens into user prompts to disrupt malicious patterns<br>and prevent injection attacks.|
|||**JailGuard**[124]: Creates multiple mutated variants of user prompts to defend against prompt-<br>based attacks through pattern disruption.|
|||**Layered Filtering Framework**[125]: Uses GPT-4 as attack validator to achieve 97% detection<br>accuracy for prompt injection attacks.|
|||**SecurityLingua** [126]: Employs security-aware prompt compression to highlight critical<br>security instructions and remove distracting context.|
|Reputation<br>Schemes|Evaluates LLM through reputation to<br>proactively isolate suspicious models<br>before causing damage.|**LLMChain Framework**[108]: Evaluates LLM reliability based on previous interactions to<br>identify potentially malicious LLMs proactively.|
|||**PsySafe**[127]: Employs psychology-based framework to assess LLM behaviors and psycho-<br>logical states for early malicious tendency identifcation.|
|Topology-<br>aware<br>Proactive<br>Maintenance|Examines network-wide LLM interaction<br>patterns and topological confgurations<br>to predict and prevent misinformation<br>propagation throughout the system.|**MedSentry**[31]: Implements comprehensive evaluation pipeline that benchmarks risk in<br>representative multi-LLM topologies for early vulnerability identifcation.|
|||**NetSafe**[128]: Examines topological aspects of LLM interactions to determine how network<br>confgurations infuence misinformation propagation.|
|||**G-Safeguard**[129]: Uses temporal graph modeling to detect anomalies in LLM interactions<br>and disrupt harmful content spread.|
|||**Guardian**[130]: Leverages topology-guided graph neural networks to proactively identify<br>and isolate malicious LLMs in collaborative environments.|



copy-on-write semantics, ensuring complete isolation between requests and automatic memory reclamation upon completion, thereby eliminating the persistent shared state. Building on this foundation, vAttention [112] advanced stateless security through hardware-level isolation using CUDA virtual memory APIs [122], creating true memory virtualization where each request operates in protected virtual address spaces. BlockLLM [29] extended stateless principles to multi-tenant environments through component-level micro-segmentation, dividing LLMs into cryptographically isolated blocks where each tenant processes requests through separate block instances. This architecture prevents cross-tenant information leakage while implementing per-block access control policies that ensure compromises cannot propagate beyond the current request, directly implementing zero-trust micro-segmentation principles at the finest possible granularity. 

_5.3.2 Ephemeral LLM._ Ephemeral LLM management [113] extends the stateless paradigm by introducing temporally-bounded LLM instances that automatically self-destruct after predetermined conditions are met, directly implementing zero-trust’s “assume breach” principle through proactive threat containment. Henderson _et al._ [113] pioneered self-destructing models, which embed algorithmic time locks that degrade model performance when adapted for harmful tasks such as generating toxic content or bypassing safety guardrails, ensuring that compromised instances lose their utility for malicious purposes without external intervention. This approach transforms LLM instances into inherently untrusted, which aligns with zero-trust’s fundamental rejection of persistent trust relationships. Furthermore, ephemeral LLMs can be deployed on serverless computing platforms [114] to realize automatic lifecycle management with multiple security checkpoints. 

## **6 INTER-LLM ZERO-TRUST SECURITY: SYSTEM APPROACHES** 

In this section, we review the technical progress of system-level approaches, which ensure zero-trust security between LLMs and eventually build the zero-trust multi-LLM system. 

## **6.1 Proactive Maintenance** 

Traditional perimeter-based security approaches typically employ reactive response mechanisms, taking actions after attacks are launched [31, 123, 128]. However, in multi-LLM collaborative environments, this reactive nature may allow malicious information to propagate and amplify 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:25 

**==> picture [388 x 84] intentionally omitted <==**

**----- Start of picture text -----**<br>
Round 1 Round 2 Round 3 Guardian Event monitor Behavior analyzer Risk responder<br>SentinelAgent<br>Backend LLM<br>Graph preprocessing<br>User  Inspector<br>prompt<br>LLM  A Inspector LLM  B<br>User<br>prompt<br>eee Collaboration pattern in the multi-LLM system Graphical encoder and decoder eee Find potential risks and attackers Inspector ay Environment Multi-LLM system Response<br>**----- End of picture text -----**<br>


Fig. 5. (left): The illustration of Guardian [130]. It analyzes the collaboration pattern of multiple LLMs to detect potential risks and make proactive maintenance. (right): The illustration of SentinelAgent [131]. It monitors all logs and behaviors in multi-LLM systems and utilizes an LLM to analyze data intelligently. 

throughout the edge network before detection. The zero-trust principle of “never trust, always verify” demands more proactive security maintenance strategies that can anticipate and prevent security incidents before they materialize. 

_6.1.1 Intelligent Input Checking._ An effective proactive strategy involves input checking, specifically analyzing and verifying user prompts to preemptively detect and neutralize potential threats before the LLM generates harmful content. For instance, Kim _et al._ [123] proposed a comprehensive LLM prompt detection system that employs regular expressions and fine-tuned LLMs to identify personally identifiable information, malicious codes, URLs, and prompt injection commands within user prompts. Chen _et al._ [30] introduced DefensiveTokens, which embed special tokens into user prompts. These tokens can disrupt or alter malicious patterns in the raw prompt, making it more difficult for attackers to construct successful injection attacks. Similarly, Zhang _et al._ [124] presented JailGuard, which defends prompt-based attacks by creating multiple mutated variants of the user prompt (e.g., random replacement, targeted insertion). Muliarevych _et al._ [125] explored a layered filtering framework, which includes a prompt analyzer to preprocess and wrap user inputs, and an attack validator that evaluates the wrapped prompts for malicious intent. Utilizing GPT-4 as the attacker validator, this layered filtering subsystem achieved superior effectiveness, accurately detecting 97% of various prompt injection attacks. SecurityLingua [126] further enhanced proactive security by employing security-aware prompt compression. This method uses a specialized compressor model trained on malicious prompt patterns to effectively highlight critical security-related instructions within user prompts. By compressing the prompts, SecurityLingua removes extraneous, potentially distracting context that attackers often introduce to mask harmful intentions. 

_6.1.2 Reputation Schemes._ In addition to input verification, the reputation of each LLM, which is based on its historical behavior and content generation quality, can be utilized as a metric to perform proactive maintenance. Specifically, LLMs whose reputation falls below a predefined threshold due to suspicious activities or poor-quality content can be proactively isolated or assigned lower priorities, thus minimizing potential damage. For instance, the LLMChain framework [108] demonstrated such a reputation system, evaluating LLM reliability based on previous interactions, thereby identifying potentially malicious LLMs and mitigating the damage they can cause. In addition to reputation, other assessments can be applied from interdisciplinary perspectives. PsySafe [127] presented a psychology-based framework that assesses multi-LLM systems by evaluating LLM behaviors and psychological states, allowing early identification of malicious tendencies and proactive intervention. 

_6.1.3 Topology-aware Proactive Maintenance._ Beyond individual LLM assessments, network-wide safety evaluations play a crucial role in predictive security maintenance. MedSentry [31], for instance, implemented a comprehensive evaluation pipeline that systematically benchmarks the risk 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:26 

existing in representative multi-LLM topologies. This allows vulnerabilities to be identified early, facilitating preemptive actions to mitigate misinformation or harmful content before impacting practical services. NetSafe [128] further extended this concept by examining the topological aspects of LLM interactions, highlighting how network configurations influence the propagation of misinformation or harmful behaviors, thus determining preventive strategies. Similarly, G-Safeguard [129] employed a temporal graph modeling technique to detect and remediate anomalies in LLM interactions. By analyzing communication patterns over time, G-Safeguard identified anomalous LLMs and disrupted the spread of harmful content, demonstrating significant effectiveness in containing and neutralizing threats early in their propagation. Lastly, Guardian [130] leveraged topology-guided graph neural networks to proactively identify malicious LLMs within collaborative multi-LLM environments (see Fig. 5(left)). By systematically screening, adjudicating, and isolating malicious LLMs based on their interactions, Guardian maintains robust decision-making and prevents information contamination. 

**Lesson Learned** : These three proactive maintenance approaches demonstrate a progressive defense strategy with increasing scope and sophistication. Input checking provides immediate threat prevention by analyzing individual prompts at the entry point, offering rapid responses. Reputation schemes advance this approach by leveraging historical behavioral data to proactively isolate potentially malicious LLMs based on accumulated trust scores, enabling predictive intervention before threats materialize. Network topology-aware maintenance operates from a macroscopic perspective, analyzing system-wide interaction patterns and communication flows to identify vulnerabilities and anomalies across the entire multi-LLM ecosystem. 

## **6.2 Blockchain and Distributed Management** 

Blockchain is a decentralized ledger system that enables tamper-resistant record keeping on public networks [132–134]. A distinctive characteristic of blockchain is the elimination of trusted authorities, operating under the assumption that any node could potentially be malicious, and achieving Byzantine fault tolerance through distributed data storage and consensus mechanisms [135]. This property makes blockchain particularly valuable for constructing zero-trust multi-LLM environments in EGI scenarios. To be specific, first, distributed data storage preserves interaction records and historical content generated by LLMs across multiple nodes, preventing records from being tampered with by any single entity. Additionally, blockchain leverages consensus mechanisms to resist Byzantine attacks, where malicious nodes may attempt to manipulate the collaborative process or inject false information [136]. Finally, smart contracts define various operational rules and trigger conditions that can be automatically executed without human intervention and cannot be influenced by attackers, making blockchains ideal for managing collaborative processes and responding to anomalous behaviors [137]. 

For data storage, Mo _et al._ [138] presented a blockchain-based crowdsourcing evaluation framework for LLMs, where LLMs’ reputation scores are recorded immutably on the blockchain. This approach ensures that evaluation scores cannot be retroactively modified, maintains a transparent history of LLM performance, and enables fair reputation-based selection of high-quality LLMs for future tasks. LLM-Net [32] constructed a multi-LLM network that divides LLMs into different roles, including coordinators, respondents, and validators. Validators execute blockchain consensus mechanisms to verify content generated by respondents, while coordinators run smart contracts to manage the collaborative process. Furthermore, all interactions and transactions are preserved on the blockchain, ensuring transparency and accountability. Similarly, Luo et al. [136] also utilized blockchain to construct decentralized multi-LLM networks. Notably, they evaluated the efficiency of four mainstream consensus algorithms in supporting multi-LLM operations and provided insights for selecting the most suitable one based on network latency and throughput requirements. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:27 

BlockAgents [139] proposed a novel consensus mechanism, namely Proof-of-Thought (PoT), which is designed specifically for multi-LLM systems. Specifically, PoT ensures that LLMs contributing the most valuable insights to the collaborative reasoning process acquire the highest voting priority, while incorporating stake-based miner designation and multi-round debate-style voting to prevent malicious LLMs from dominating the decision-making process. Wang et al. [33] developed a threshold signature algorithm suitable for blockchain-assisted multi-LLM systems. Decisions agreed upon by multiple LLMs are collaboratively signed by everyone and written on the blockchain, enabling traceability and accountability while ensuring that no single entity can control the entire signature process. Karanjai et al. [137] employed smart contracts to manage the execution of LLM inference on edge devices, preventing attackers from launching DDoS attacks on EGI. 

Apart from blockchain, Multi-Party Secure Computation (MPC) [140] has emerged as a critical technique for constructing zero-trust multi-LLM systems by enabling secure collaborative computations without revealing individual data inputs. This is particularly important in scenarios involving federated fine-tuning and decentralized inference, where maintaining data confidentiality and preventing unauthorized access to sensitive model information are paramount. Additionally, Zero-Knowledge Proofs (ZKP) [141] enhance zero-trust principles by enabling the verification of computations across multiple LLMs without revealing the underlying LLM parameters or private data. Qu _et al._ [141] introduced zkGPT, a non-interactive ZKP framework specifically tailored for efficient LLM inference. By generating cryptographic proofs that verify the correctness of inference results without exposing confidential model parameters, zkGPT aligns perfectly with the principles of zero-trust architectures, ensuring verifiability and integrity of computations in distributed multi-LLM systems. 

## **6.3 Micro-segmentation and Isolation** 

Micro-segmentation [15] is a critical zero-trust security technique that divides network infrastructure into smaller, isolated segments to limit lateral movement (i.e., attackers moving from one compromised system to adjacent systems within the network) and contain potential breaches. In EGI with multiple LLMs, micro-segmentation becomes particularly critical due to the diverse service requirements, sensitive data processing, and heterogeneous device configurations. 

Recent advances in wireless network slicing have demonstrated significant potential for enhancing security isolation in multi-LLM deployments. Liu _et al._ [142] introduced WiLLM, the first open-source wireless communication system specifically designed for LLM services through its innovative “Tree-Branch-Fruit” slicing architecture. This hierarchical framework enables dedicated communication channels for different LLM services, ensuring that computational and communication resources are isolated between different slices while allowing telecom operators to monetize services through slice subscriptions. The approach directly supports zero-trust principles by establishing strict boundaries between LLM services, preventing unauthorized inter-LLM communication, and enabling granular access controls at the network level. Based on this foundation, Liu _et al._ [143] further demonstrated the practical implementation of dedicated network slicing for LLMs through their proposed LLM-Slice system, which creates LLM-specific network slices to efficiently bind services and communication resources. The system enables different LLMs, including Google Bard[8] , Meta LLaMA[9] , and ChatGPT [144], to coexist with independent resource allocation and management, ensuring that potential security breaches in one slice cannot propagate to others. Furthermore, a permissions database is used to enforce strict authentication policies for each 

> 8https://gemini.google.com/app 

> 9https://www.llama.com/ 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:28 

LLM. Finally, an intelligent controller continuously validates cross-slice communications, enabling real-time threat detection that aligns with zero-trust’s continuous monitoring [143]. 

## **6.4 Intelligent System Monitoring and Failure Management** 

Zero-trust multi-LLM systems in EGI demand continuous monitoring and robust failure management to ensure reliability and safety. Recent studies have summarized numerous failure modes unique to multi-LLM setups that underscore the need for comprehensive monitoring [49]. For instance, Cemri _et al._ [145] identified 14 distinct failure modes in popular multi-LLM frameworks, ranging from coordination breakdowns (i.e., one LLM’s action invalidates another’s) to more subtle issues like shared misconceptions that propagate across the network. The complexity and variety of these failure modes highlight why traditional guardrails are insufficient for multi-LLM systems, necessitating more sophisticated monitoring approaches that can detect distributed failures. 

Multi-LLM monitoring has followed a clear evolutionary path, progressively aligning with the stringent demands of zero-trust architectures. Initially, oversight relied on manually crafted rules or static guardrails, such as reactive output filters or step-by-step validators [146] to intercept disallowed actions. Despite effectively detecting straightforward violations, such as unauthorized tool calls or toxic outputs, these methods fundamentally fall short of zero-trust requirements due to their inability to dynamically adapt to evolving threats beyond predefined rules. To this end, researchers introduced traditional anomaly detection and machine learning techniques [147], capitalizing on the rich telemetry from multi-LLM systems, such as action and interaction logs, performance metrics, and state-change traces, to identify deviations from expected behavior. Although these approaches expand monitoring capabilities and handle extensive data, they treat LLM systems as black-box output generators, lacking the nuanced semantic understanding essential for comprehensive oversight in zero-trust environments. 

The latest advancement employs LLMs themselves as monitoring agents. For instance, Zhang _et al._ [148] proposed AgentFM, which organizes multiple specialized LLMs to monitor activities specific to its designated role. Additionally, a higher-level meta-LLM integrates these role-specific insights, enabling the detection and handling of cross-role anomalies, effectively supporting the nuanced oversight required by zero-trust environments. As shown in Fig. 5(right), He _et al._ [131] introduced SentinelAgent, which implements real-time anomaly detection and intervention by dynamically constructing interaction graphs of LLM interactions. This framework detects anomalies at various granularities: individual LLM misbehavior at the node level, unsafe inter-LLM communications at the edge level, and suspicious exploit chains at the path level. Notably, a pluggable LLM-based monitoring agent is leveraged for semantic analysis, enabling active intervention such as interrupting harmful behaviors and correcting multi-LLM collusion, thereby offering robust zero-trust monitoring capabilities suited for complex multi-LLM ecosystems. Chang _et al._ [149] presented SagaLLM, which structures each task into independent, compensable units. In this framework, each operational LLM is paired with a dedicated compensation LLM responsible for rolling back and correcting any faulty operations. Independent validating LLMs are employed to assess intermediate and final outputs, detecting inconsistencies such as logical contradictions or coordination errors between LLMs. Upon detecting anomalies, SagaLLM automatically initiates structured rollbacks. 

Remark 2. _The technical progress reviewed in Sections 5 and 6 contributes to the implementation and advancement of multi-LLM zero-trust systems in EGI (such as the one demonstrated in Section 4). For instance, the strong identity protocols and context-aware access control can realize the “Identity and Authentication Module”, proactive maintenance enables “User Input Checking” and “Multi-Layer LLM Output Verification” modules, and intelligent monitoring schemes realize “Behavioral Auditing and Anomaly Detection.”_ 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:29 

## **7 CHALLENGES AND FUTURE RESEARCH DIRECTIONS** 

## **7.1 Ethical and Societal Issues** 

The deployment of zero-trust multi-LLM systems in EGI raises ethical and societal implications that extend beyond technical security considerations. Unlike traditional AI systems, multi-LLM systems in EGI directly interact with critical societal infrastructure, autonomous vehicles, and healthcare networks, amplifying the potential impact of algorithmic bias, discrimination, and social harm [150]. The zero-trust paradigm’s fundamental assumption of “never trust, always verify” introduces additional ethical complexities when applied to human-AI interactions, potentially undermining user trust and creating psychological barriers to system adoption. 

Future research should address several critical challenges: 1) developing ethical frameworks for algorithmic accountability in distributed multi-LLM decision-making, where responsibility attribution becomes complex due to the collaborative nature of reasoning processes and the absence of centralized control mechanisms, 2) establishing fairness-preserving zero-trust protocols that prevent discriminatory outcomes while maintaining security guarantees, particularly when LLMs trained on biased datasets collaborate in safety-critical applications, and 3) designing transparent governance mechanisms that enable public oversight and democratic participation in zero-trust multi-LLM system deployment decisions. 

## **7.2 Asymmetric Information and Network Heterogeneity** 

The heterogeneous nature of EGI networks presents fundamental challenges in implementing unified zero-trust multi-LLM frameworks due to asymmetric information sharing capabilities and widely varying communication conditions among edge devices [151]. The stark disparity between high-capacity wired backhaul connections (e.g., between roadside units and base stations with negligible delay) and unreliable wireless links (e.g., vehicle-to-vehicle channels with random delays and intermittent connectivity) creates temporal inconsistencies that can undermine zero-trust security protocols requiring synchronized verification and continuous authentication. 

This network asymmetry introduces several critical research challenges: 1) developing delaytolerant zero-trust protocols that can maintain security guarantees despite variable network conditions and intermittent connectivity, ensuring that authentication and authorization mechanisms remain effective even when some nodes experience communication delays or temporary isolation, 2) designing adaptive information sharing strategies that dynamically adjust collaboration patterns based on real-time network conditions while preserving the principle of least privilege access control, and 3) establishing distributed consensus mechanisms for multi-LLM coordination that are resilient to network partitions and asymmetric information propagation delays. Future work can also explore federated zero-trust architectures that leverage edge-cloud hierarchies to compensate for local network limitations, implementing tiered security policies that can gracefully degrade while maintaining critical safety properties when network conditions deteriorate. 

## **7.3 Privacy-Preserving Collaborative Reasoning** 

The deployment of zero-trust multi-LLM systems necessitates collaborative reasoning capabilities that can leverage collective intelligence while maintaining cryptographic privacy guarantees and preventing any form of information leakage, extending the principle of “least privilege” to collaborative inference scenarios [16]. This challenge is particularly acute in EGI scenarios where sensitive data must be processed at the edge while maintaining mathematically provable confidentiality. 

Future research should address: 1) developing transformer-oriented encryption schemes that enable encrypted multi-LLM collaboration without any plaintext data exposure, ensuring that even compromised LLMs cannot access sensitive information [16], and 2) advancing secure MPC frameworks with ZKP that enable collaborative inference among distributed LLMs while maintaining 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:30 

cryptographic proof that no participating LLM can extract more information than strictly necessary for the collaborative task. Recent work on encryption-friendly LLMs demonstrates the feasibility of homomorphic encryption for transformer models, achieving significant computational speedups while maintaining performance comparable to plaintext models [16]. However, extending these approaches to multi-LLM collaborative scenarios introduces additional complexity in terms of zerotrust encrypted communication protocols and cryptographically verifiable consensus mechanisms that must be addressed through innovative cryptographic and distributed systems research that assumes no trust in any component. 

## **8 CONCLUSION** 

This paper has presented the first systematic survey of zero-trust security principles applied to multi-LLM systems in EGI. As multi-LLM systems become increasingly critical, traditional perimeter-based security approaches prove inadequate to address the unique vulnerabilities in collaborative edge deployments. We have systematically analyzed security challenges in multiLLM systems, categorizing threats at both intra-LLM and inter-LLM levels, and demonstrate the limitations of existing trustworthy approaches. Then, we have proposed a unified zero-trust multi-LLM framework implementing the four fundamental zero-trust principles through detailed architectural design and operational workflows. We have categorized zero-trust mechanisms into model-level approaches focusing on individual LLM security and system-level approaches addressing distributed coordination challenges. Additionally, we have identified several critical future research directions that require immediate attention. We hope that this survey prompts both theoretical progress and practical implementation for secure edge agentic AI. 

## **REFERENCES** 

- [1] Yingxuan Yang et al. 2024. LLM-based Multi-Agent Systems: Techniques and Business Perspectives. _ArXiv preprint: ArXiv:2411.14033_ (2024). 

- [2] Deepak Bhaskar Acharya, Karthigeyan Kuppan, and B. Divya. 2025. Agentic AI: Autonomous Intelligence for Complex Goals—A Comprehensive Survey. _IEEE Access_ 13 (2025), 18912–18936. 

- [3] Ahmet Gunduz, Kamer Ali Yuksel, and Hassan Sawaf. 2025. MediaMind: Revolutionizing Media Monitoring using Agentification. _ArXiv preprint: ArXiv:2502.12745_ (2025). 

- [4] Handi Chen et al. 2024. Towards Edge General Intelligence via Large Language Models: Opportunities and Challenges. _ArXiv preprint: ArXiv:2410.18125_ (2024). 

- [5] Le He et al. 2025. The Road Toward General Edge Intelligence: Standing on the Shoulders of Foundation Models. _IEEE Communications Magazine_ (2025), 1–7. 

- [6] Zihao Xu, Yi Liu, Gelei Deng, Yuekang Li, and Stjepan Picek. 2024. A Comprehensive Study of Jailbreak Attack versus Defense for Large Language Models. In _Proc. ACL Findings_ . 7432–7449. 

- [7] Yi Liu et al. 2023. Prompt Injection attack against LLM-integrated Applications. _ArXiv preprint: ArXiv:2306.05499_ (2023). 

- [8] Zhaorun Chen, Zhen Xiang, Chaowei Xiao, Dawn Song, and Bo Li. 2024. AgentPoison: Red-teaming LLM Agents via Poisoning Memory or Knowledge Bases. In _Proc. NeurIPS_ . 1–29. 

- [9] Ronny Ko et al. 2025. Seven Security Challenges That Must be Solved in Cross-domain Multi-agent LLM Systems. _ArXiv preprint: ArXiv:2505.23847_ (2025). 

- [10] Lei Yu, Virginie Do, Karen Hambardzumyan, and Nicola Cancedda. 2024. Robust LLM safeguarding via refusal feature adversarial training. _ArXiv preprint: ArXiv:2409.20089_ (2024). 

- [11] Zachary Charles et al. 2024. Fine-Tuning Large Language Models with User-Level Differential Privacy. _ArXiv preprint: ArXiv:2407.07737_ (2024). 

- [12] Wei Yu et al. 2022. TEE based Cross-silo Trustworthy Federated Learning Infrastructure. In _Proc. IJCAI_ . 

- [13] Yifan Zeng et al. 2024. AutoDefense: Multi-Agent LLM Defense against Jailbreak Attacks. In _Proc. NeurIPS_ . 

- [14] OpenAI et al. 2023. GPT-4 Technical Report. _arXiv preprint arXiv:2303.08774_ (2023). 

- [15] [n. d.]. NIST Zero trust standard. 2025. https://www.nist.gov/publications/zero-trust-architecture 

- [16] Xinye Cao et al. 2025. Exploring LLM-Based Multi-Agent Situation Awareness for Zero-Trust Space-Air-Ground Integrated Network. _IEEE Journal on Selected Areas in Communications_ 43, 6 (2025), 2230–2247. 

- [17] Alexandre Poirrier, Laurent Cailleux, and Thomas Heide Clausen. 2025. Is Trust Misplaced? A Zero-Trust Survey. _Proc. IEEE_ 113, 1 (2025), 5–39. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:31 

- [18] Badhan Chandra Das, M. Hadi Amini, and Yanzhao Wu. 2025. Security and Privacy Challenges of Large Language Models: A Survey. _ACM Computing Survey_ 57, 6 (Feb. 2025), 1–39. 

- [19] Francisco Aguilera-Martínez and Fernando Berzal. 2025. LLM Security: Vulnerabilities, Attacks, Defenses, and Countermeasures. _ArXiv preprint: ArXiv:2505.01177_ (2025). 

- [20] Othmane Friha, Mohamed Amine Ferrag, Burak Kantarci, Burak Cakmak, Arda Ozgun, and Nassira GhoualmiZine. 2024. LLM-Based Edge Intelligence: A Comprehensive Survey on Architectures, Applications, Security and Trustworthiness. _IEEE Open Journal of the Communications Society_ 5 (2024), 5799–5856. 

- [21] Yuyou Gan et al. 2024. Navigating the Risks: A Survey of Security, Privacy, and Ethics Threats in LLM-Based Agents. _ArXiv preprint: ArXiv: 2411.09523_ (2024). 

- [22] Yang Liu et al. 2024. Trustworthy LLMs: a Survey and Guideline for Evaluating Large Language Models’ Alignment. _ArXiv preprint: ArXiv: 2308.05374_ (2024). 

- [23] Dezhang Kong et al. 2025. A Survey of LLM-Driven AI Agent Communication: Protocols, Security Risks, and Defense Countermeasures. _ArXiv preprint: ArXiv:2506.19676_ (2025). 

- [24] Haoxiang Luo et al. 2025. Toward Edge General Intelligence with Multiple-Large Language Model (Multi-LLM): Architecture, Trust, and Orchestration. _ArXiv preprint: ArXiv:2507.00672_ (2025). 

- [25] Pierre Peigne et al. 2024. Multi-Agent Security Tax: Trading Off Security and Collaboration Capabilities in Multi-Agent Systems. In _Proc. AAAI_ . 27573–27581. 

- [26] Shih-Han Chan. 2025. Encrypted Prompt: Securing LLM Applications Against Unauthorized Actions. _ArXiv preprint: ArXiv:2503.23250_ (2025). 

- [27] Zitong Shi et al. 2025. Privacy-Enhancing Paradigms within Federated Multi-Agent Systems. _ArXiv preprint: ArXiv:2503.08175_ (2025). 

- [28] Peng Xiao, Shunkun Yang, Hailin Wang, Zhenhong Zhang, and Chunsheng Zou. 2025. Privacy-preserving revocable access control for LLM-driven electrical distributed systems. _Peer-to-Peer Networking and Applications_ 18, 148 (2025), 1–12. 

- [29] Usama Arshad and Zahid Halim. 2025. BlockLLM: A futuristic LLM-based decentralized vehicular network architecture for secure communications. _Computers and Electrical Engineering_ 123 (Jan. 2025), 1–39. 

- [30] Sizhe Chen, Yizhu Wang, Nicholas Carlini, Chawin Sitawarin, and David Wagner. 2025. Defending Against Prompt Injection With a Few DefensiveTokens. _ArXiv preprint: ArXiv:2507.07974_ (2025). 

- [31] Kai Chen et al. 2025. MedSentry: Understanding and Mitigating Safety Risks in Medical LLM Multi-Agent Systems. _ArXiv preprint: ArXiv:2505.20824_ (2025). 

- [32] Zan-Kai Chong, Hiroyuki Ohsaki, and Bryan Ng. 2025. LLM-Net: Democratizing LLMs-as-a-Service through Blockchain-based Expert Networks. _ArXiv preprint: ArXiv:2501.07288_ (2025). 

- [33] Jing Wang, Xue Yuan, Yingjie Xu, Yudi Zhang, and Guowen Xu. 2024. An Efficient Multiparty Threshold ECDSA Protocol against Malicious Adversaries for Blockchain-Based LLMs. _IET Information Security_ 2024 (2024), 1–12. 

- [34] [n. d.]. OpenAI ChatGPT. 2025. https://openai.com/index/chatgpt/ 

- [35] Alexander Kirillov et al. 2023. Segment Anything. In _Proc. ICCV_ . 4015–4026. 

- [36] Qingyu Wu et al. 2024. AutoGen: Enabling Next-Gen LLM Applications via Multi-Agent Conversations. In _Proc. COLM_ . 1–46. 

- [37] Sirui Hong et al. 2024. MetaGPT: Meta Programming for A Multi-Agent Collaborative Framework. In _Proc. ICLR_ . 1–29. 

- [38] Yilun Du, Shuang Li, Antonio Torralba, Joshua B. Tenenbaum, and Igor Mordatch. 2024. Improving factuality and reasoning in language models through multiagent debate. In _Proc. ICML_ . 11733 – 11763. 

- [39] Xinyi Li, Sai Wang, Siqi Zeng, Yu Wu, and Yi Yang. 2024. A survey on LLM-based multi-agent systems: workflow, infrastructure, and challenges. _Vicinagearth_ 1, 9 (2024), 1–43. 

- [40] Guibin Zhang et al. 2025. G-Designer: Architecting Multi-Agent Communication Topologies via Graph Neural Networks. In _Proc. ICLR_ . 1–12. 

- [41] Ehud Karpas et al. 2022. MRKL systems: A modular, neuro-symbolic architecture that combines large language models, external knowledge sources and discrete reasoning. _ArXiv preprint: ArXiv:2205.00445_ (2022). 

- [42] Abul Ehtesham, Aditi Singh, Gaurav Kumar Gupta, and Saket Kumar. 2025. A survey of agent interoperability protocols: Model Context Protocol (MCP), Agent Communication Protocol (ACP), Agent-to-Agent Protocol (A2A), and Agent Network Protocol (ANP). _ArXiv preprint: ArXiv:2505.02279_ (2025). 

- [43] Yu Du, Jun Li, Long Shi, Tingting Liu, Feng Shu, and Zhu Han. 2022. Two-Tier Matching Game in Small Cell Networks for Mobile Edge Computing. _IEEE Transactions on Services Computing_ 15, 1 (2022), 254–265. 

- [44] Yung-Yao Chen, Sin-Ye Jhong, Shao-Kai Tu, Yu-Hsiu Lin, and Yi-Chen Wu. 2024. Autonomous Smart-Edge Fault Diagnostics via Edge-Cloud-Orchestrated Collaborative Computing for Infrared Electrical Equipment Images. _IEEE Sensors Journal_ 24, 15 (2024), 24630–24648. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:32 

- [45] Sébastien Bubeck et al. 2023. Sparks of Artificial General Intelligence: Early experiments with GPT-4. _ArXiv preprint: ArXiv:2303.12712_ (2023). 

- [46] Jan Clusmann et al. 2023. The future landscape of large language models in medicine. _Communications Medcine_ 3, 141 (2023), 1–8. 

- [47] Senkang Hu, Zhengru Fang, Zihan Fang, Yiqin Deng, Xianhao Chen, and Yuguang Fang. 2025. AgentsCoDriver: Large Language Model Empowered Collaborative Driving with Lifelong Learning. _ArXiv preprint: ArXiv:2404.06345_ (2025). 

- [48] Xiaozhi Deng, Tengteng Ma, Haobin Li, and Mingxin Lu. 2024. Federated Large Language Models for Smart Grid: A Communication Efficient LoRA Approach. In _Proc. ICCASIT_ . 1369–1374. 

- [49] Hrishikesh Joshi. 2025. Emerging Technologies Driving Zero Trust Maturity Across Industries. _IEEE Open Journal of the Computer Society_ 6 (2025), 25–36. 

- [50] Sunder A. Khowaja, Parus Khuwaja, Kapal Dev, Keshav Singh, Xingwang Li, Nikolaos Bartzoudis, and Ciprian R. Comsa. 2025. Block Encryption LAyer (BELA): Zero-Trust Defense Against Model Inversion Attacks for Federated Learning in 5G/6G Systems. _IEEE Open Journal of the Communications Society_ 6 (2025), 807–819. 

- [51] Qingxuan Wang and Ding Wang. 2023. Understanding Failures in Security Proofs of Multi-Factor Authentication for Mobile Devices. _IEEE Transactions on Information Forensics and Security_ 18 (2023), 597–612. 

- [52] Wentao Jing, Linning Peng, Hua Fu, and Aiqun Hu. 2024. An Authentication Mechanism Based on Zero Trust With Radio Frequency Fingerprint for Internet of Things Networks. _IEEE Internet of Things Journal_ 11, 13 (2024), 23683–23698. 

- [53] Sungmin Hong, Lei Xu, Jianwei Huang, Hongda Li, Hongxin Hu, and Guofei Gu. 2023. SysFlow: Toward a Programmable Zero Trust Framework for System Security. _IEEE Transactions on Information Forensics and Security_ 18 (2023), 2794–2809. 

- [54] Lan Zhou, Vijay Varadharajan, and Michael Hitchens. 2013. Achieving Secure Role-Based Access Control on Encrypted Data in Cloud Storage. _IEEE Transactions on Information Forensics and Security_ 8, 12 (2013), 1947–1960. 

- [55] Marcela Tuler De Oliveira, Lúcio Henrik Amorim Reis, Yiannis Verginadis, Diogo Menezes Ferrazani Mattos, and Sílvia Delgado Olabarriaga. 2022. SmartAccess: Attribute-Based Access Control System for Medical Records Based on Smart Contracts. _IEEE Access_ 10 (2022), 117836–117854. 

- [56] Pejman Najafi, Daniel Koehler, Feng Cheng, and Christoph Meinel. 2021. NLP-based Entity Behavior Analytics for Malware Detection. In _Proc. IPCCC_ . 1–5. 

- [57] Sandeep Bhatt, Pratyusa K. Manadhata, and Loai Zomlot. 2014. The Operational Role of Security Information and Event Management Systems. _IEEE Security & Privacy_ 12, 5 (2014), 35–41. 

- [58] Eranga Bandara, Xueping Liang, Sachin Shetty, Ravi Mukkamala, Abdul Rahman, and Ng Wee Keong. 2022. Skunk — A Blockchain and Zero Trust Security Enabled Federated Learning Platform for 5G/6G Network Slicing. In _Proc. SECON_ . 109–117. 

- [59] Liv d’Aliberti, Evan Gronberg, and Joseph Kovba. 2024. Privacy-Enhancing Technologies for Artificial IntelligenceEnabled Systems. In _Proc. IWSPA_ . 

- [60] Rui Zhao, Ziguo Chen, Yuze Fan, Yun Li, and Fei Gao. 2024. Towards Robust Decision-Making for Autonomous Highway Driving Based on Safe Reinforcement Learning. _Sensors_ 24, 13 (2024). 

- [61] Zihao Xu, Yi Liu, Gelei Deng, Yuekang Li, and Stjepan Picek. 2024. A Comprehensive Study of Jailbreak Attack versus Defense for Large Language Models. In _Proc. ACL_ . 7432–7449. 

- [62] Zeyi Liao and Huan Sun. 2024. AmpleGCG: Learning a Universal and Transferable Generative Model of Adversarial Suffixes for Jailbreaking Both Open and Closed LLMs. In _Proc. COLM_ . 1–14. 

- [63] Richard Fang, Rohan Bindu, Akul Gupta, and Daniel Kang. 2024. LLM Agents can Autonomously Exploit One-day Vulnerabilities. _ArXiv preprint: ArXiv:2404.08144_ (2024). 

- [64] Nicholas Carlini et al. 2021. Extracting Training Data from Large Language Models. In _Proc. USENIX Security_ . 2633–2650. 

- [65] Fatemehsadat Mireshghallah, Kartik Goyal, Archit Uniyal, Taylor Berg-Kirkpatrick, and Reza Shokri. 2022. Quantifying Privacy Risks of Prompting Large Language Models. _arXiv preprint arXiv:2210.17012_ (2022). 

- [66] Pranab Sahoo, Prabhash Meharia, Akash Ghosh, Sriparna Saha, Vinija Jain, and Aman Chadha. 2024. A Comprehensive Survey of Hallucination in Large Language, Image, Video and Audio Foundation Models. In _Proc. EMNLP_ . 11709–11724. 

- [67] Donghyun Lee and Mo Tiwari. 2024. Prompt Infection: LLM-to-LLM Prompt Injection within Multi-Agent Systems. _ArXiv preprint: ArXiv:2410.07283_ (2024). 

- [68] Xu Shen et al. 2025. Understanding the Information Propagation Effects of Communication Topologies in LLM-based Multi-Agent Systems. _ArXiv preprint: ArXiv:2505.23352_ (2025). 

- [69] Rupeng Zhang et al. 2025. From Allies to Adversaries: Manipulating LLM Tool-Calling through Adversarial Injection. In _Proc. NAACL_ . 2009–2028. 

- [70] Pengfei He, Yupin Lin, Shen Dong, Han Xu, Yue Xing, and Hui Liu. 2025. Red-Teaming LLM Multi-Agent Systems via Communication Attacks. _arXiv preprint arXiv:2502.14847_ (2025). 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

35:33 

- [71] Yuyang Zhang, Kangjie Chen, Jiaxin Gao, Ronghao Cui, Run Wang, Lina Wang, and Tianwei Zhang. 2024. Towards Action Hijacking of Large Language Model-based Agent. _arXiv preprint arXiv:2412.10807_ (2024). 

- [72] Gurusha Juneja, Alon Albalak, Wenyue Hua, and William Yang Wang. 2025. MAGPIE: A dataset for Multi-AGent contextual PrIvacy Evaluation. _ArXiv preprint: ArXiv:2506.20737_ (2025). 

- [73] Haibo Jin, Ruoxi Chen, Peiyan Zhang, Andy Zhou, Yang Zhang, and Haohan Wang. 2025. GUARD: Role-playing to Generate Natural-language Jailbreakings to Test Guideline Adherence of LLMs. _ArXiv preprint: ArXiv:2402.03299_ (2025). 

- [74] Marcin Chrapek, Anjo Vahldiek-Oberwagner, Marcin Spoczynski, Scott Constable, Mona Vij, and Torsten Hoefler. 2024. Fortify Your Foundations: Practical Privacy and Security for Foundation Model Deployments In The Cloud. _ArXiv preprint: ArXiv:2410.05930_ (2024). 

- [75] Assaf Namer, Prashant Kulkarni, Erik Jeansson, Brandon Maltzman, and Hauke Vagts. 2025. Automatically Detecting Expensive Prompts and Configuring Firewall Rules to Mitigate Denial of Service Attacks on Large Language Models. _https://www.tdcommons.org/dpubs_series/6642/_ (2025). 

- [76] Sophie Xhonneux, Alessandro Sordoni, Stephan Günnemann, Gauthier Gidel, and Leo Schwinn. 2025. Efficient adversarial training in LLMs with continuous attacks. In _Proc. NeurIPS_ . 1502 – 1530. 

- [77] Dazhen Deng, Chuhan Zhang, Huawei Zheng, Yuwen Pu, Shouling Ji, and Yingcai Wu. 2025. AdversaFlow: Visual Red Teaming for Large Language Models with Multi-Level Adversarial Flow. _IEEE Transactions on Visualization and Computer Graphics_ 31, 1 (2025), 492–502. 

- [78] Rouzbeh Behnia, Mohammadreza Reza Ebrahimi, Jason Pacheco, and Balaji Padmanabhan. 2022. EW-Tune: A Framework for Privately Fine-Tuning Large Language Models with Differential Privacy. In _Proc. ICDMW_ . 560–566. 

- [79] Juntao Dai et al. 2024. Safe RLHF: Safe Reinforcement Learning from Human Feedback. In _Proc. ICML_ . 1–28. 

- [80] Zhendan Sun and Ruibin Zhao. 2025. LLM Security Alignment Framework Design Based on Personal Preference. In _Proc. AIFE_ . 6–11. 

- [81] Petr Spelda and Vit Stritecky. 2025. Security practices in AI development. _AI & Society_ (2025), 1–11. 

- [82] Mansi Phute, Alec Helbling, Matthew Daniel Hull, ShengYun Peng, Sebastian Szyller, Cory Cornelius, and Duen Horng Chau. 2024. LLM Self Defense: By Self Examination, LLMs Know They Are Being Tricked. In _Proc. ICLR_ . 1–6. 

- [83] Ben Dong and Qian Wang. 2025. Evaluating the Performance of the DeepSeek Model in Confidential Computing Environment. _ArXiv preprint: ArXiv:2502.11347_ (2025). 

- [84] Jianchang Su and Wei Zhang. 2025. Runtime Attestation for Secure LLM Serving in Cloud-Native Trusted Execution Environments. In _Proc. ICSA_ . 1–5. 

- [85] Qinfeng Li et al. 2024. CoreGuard: Safeguarding Foundational Capabilities of LLMs Against Model Stealing in Edge Deployment. _ArXiv preprint: ArXiv:2410.13903_ (2024). 

- [86] Zechao Lin, Sisi Zhang, Xingbin Wang, Yulan Su, Yan Wang, Rui Hou, and Dan Meng. 2025. LoRATEE: A Secure and Efficient Inference Framework for Multi-Tenant LoRA LLMs Based on TEE. In _Proc. ICASSP_ . 1–5. 

- [87] Debdeep Sanyal, Umakanta Maharana, Yash Sinha, Hong Ming Tan, Shirish Karande, Mohan Kankanhalli, and Murari Mandal. 2025. OrgAccess: A Benchmark for Role Based Access Control in Organization Scale LLMs. _ArXiv preprint: ArXiv:2505.19165_ (2025). 

- [88] Bin Huang et al. 2024. FirewaLLM: A Portable Data Protection and Recovery Framework for LLM Services. In _Proc. DMBD_ . 16–30. 

- [89] Tsau Young T. Y. Lin and Pierre Vachon. 2017. Secure information flow and file movements: A topological theory of discretionary access controls. In _Proc. Big Data_ . 1821–1829. 

- [90] Hui Lu, Xiaojiang Du, Dawei Hu, Shen Su, and Zhihong Tian. 2025. BPFGuard: Multi-Granularity Container Runtime Mandatory Access Control. _IEEE Transactions on Cloud Computing_ 13, 2 (2025), 629–640. 

- [91] Tianneng Shi, Jingxuan He, Zhun Wang, Linyu Wu, Hongwei Li, Wenbo Guo, and Dawn Song. 2025. Progent: Programmable Privilege Control for LLM Agents. _ArXiv preprint: ArXiv:2504.11703_ (2025). 

- [92] Sahar Abdelnabi, Amr Gomaa, Eugene Bagdasarian, Per Ola Kristensson, and Reza Shokri. 2025. Firewalls to Secure Dynamic LLM Agentic Networks. _ArXiv preprint: ArXiv:2502.01822_ (2025). 

- [93] Yidou Chen Yixin Jiang Cong Wang Zhan Qin Hongwei Yao, Haoran Shi. 2025. ControlNET: A Firewall for RAG-based LLM System. _arXiv preprint arXiv:2504.09593_ (2025). 

- [94] Yuyou Gan et al. 2024. Navigating the Risks: A Survey of Security, Privacy, and Ethics Threats in LLM-Based Agents. _arXiv preprint arXiv:2411.09523_ (2024). 

- [95] Huihao Jing et al. 2025. MCIP: Protecting MCP Safety via Model Contextual Integrity Protocol. _arXiv preprint arXiv:2505.14590_ (2025). 

- [96] Wayne Xin Zhao et al. 2025. A Survey of Large Language Models. _ArXiv preprint: ArXiv:2303.18223_ (2025). 

- [97] Bei Chen et al. 2024. BlockAgents: Towards Byzantine-Robust LLM-Based Multi-Agent Coordination via Blockchain. In _Proc. ACM Turing Award Celebration Conference_ . 187–192. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

Y. Liu et al. 

35:34 

- [98] Yuan Yao, Bin Xiao, Gang Yang, Yujiao Hu, Liang Wang, and Xingshe Zhou. 2019. Power Control Identification: A Novel Sybil Attack Detection Scheme in VANETs Using RSSI. _IEEE Journal on Selected Areas in Communications_ 37, 11 (2019), 2588–2602. 

- [99] Qianlong Sun, Guoshun Nan, Tianyi Li, Huici Wu, Zhou Zhong, and Xiaofeng Tao. 2025. A Secure Digital Signature Scheme for Deep Learning-Based Semantic Communication Systems. _IEEE Wireless Communications Letters_ 14, 4 (2025), 1119–1123. 

- [100] Chenxin Zhang, Jin He, Baixiang Fan, Yaqiang Gong, Shuo Li, Bo Yin, and Yongfeng Lin. 2022. Tag-Based Trust Evaluation In Zero Trust Architecture. In _Proc. IAECST_ . 772–776. 

- [101] Senthil Murugan Nagarajan, Ganesh Gopal Devarajan, M. Suresh Thangakrishnan, T. V. Ramana, Ali Kashif Bashir, and Ahmad Ali AlZubi. 2024. Artificial Intelligence-Based Zero Trust Security Approach for Consumer Industry. _IEEE Transactions on Consumer Electronics_ 70, 3 (2024), 5411–5418. 

- [102] He Fang, Yongxu Zhu, Yan Zhang, and Xianbin Wang. 2024. Decentralized Edge Collaboration for Seamless Handover Authentication in Zero-Trust IoV. _IEEE Transactions on Wireless Communications_ 23, 8 (2024), 8760–8772. 

- [103] Sahar Abdelnabi, Amr Gomaa, Eugene Bagdasarian, Per Ola Kristensson, and Reza Shokri. 2025. Firewalls to Secure Dynamic LLM Agentic Networks. _ArXiv preprint: ArXiv:2502.01822_ (2025). 

- [104] Samuel Gehman, Suchin Gururangan, Maarten Sap, Yejin Choi, and Noah A. Smith. 2020. RealToxicityPrompts: Evaluating Neural Toxic Degeneration in Language Models. In _Proc. EMNLP_ . 3356–3369. 

- [105] Yuan Sun, Navid Salami Pargoo, Peter Jin, and Jorge Ortiz. 2024. Optimizing Autonomous Driving for Safety: A Human-Centric Approach with LLM-Enhanced RLHF. In _Proc. UbiComp_ . 76–80. 

- [106] Nurun Nahar, Karl Andersson, Olov Schelén, and Saguna Saguna. 2024. A Survey on Zero Trust Architecture: Applications and Challenges of 6G Networks. _IEEE Access_ 12 (2024), 94753–94764. 

- [107] Fanqin Zhou, Lei Zhang, Zhixiang Yang, and Lei Feng. 2025. Radio Frequency-Enhanced Multi-Factor IoT Device Authentication via Swarm Learning. _IEEE Transactions on Network Science and Engineering_ 12, 4 (2025), 2487–2499. 

- [108] Mouhamed Amine Bouchiha, Quentin Telnoff, Souhail Bakkali, Ronan Champagnat, Mourad Rabah, Mickaël Coustaty, and Yacine Ghamri-Doudane. 2024. LLMChain: Blockchain-Based Reputation System for Sharing and Evaluating Large Language Models. In _Proc. COMPSAC_ . 439–448. 

- [109] Junyuan Mao et al. 2025. AgentSafe: Safeguarding Large Language Model-based Multi-agent Systems via Hierarchical Data Management. _ArXiv preprint: ArXiv:2503.04392_ (2025). 

- [110] Alireza Rezazadeh, Zichao Li, Ange Lou, Yuying Zhao, Wei Wei, and Yujia Bao. 2025. Collaborative Memory: Multi-User Memory Sharing in LLM Agents with Dynamic Access Control. _ArXiv preprint: ArXiv:2505.18279_ (2025). 

- [111] Woosuk Kwon et al. 2023. Efficient Memory Management for Large Language Model Serving with PagedAttention. In _Proc. SOSP_ . 1–16. 

- [112] Ramya Prabhu, Ajay Nayak, Jayashree Mohan, Ramachandran Ramjee, and Ashish Panwar. 2025. vAttention: Dynamic Memory Management for Serving LLMs without PagedAttention. In _Proc. ASPLOS_ . 1–18. 

- [113] Peter Henderson, Eric Mitchell, Christopher Manning, Dan Jurafsky, and Chelsea Finn. 2023. Self-Destructing Models: Increasing the Costs of Harmful Dual Uses of Foundation Models. In _Proc. AIES_ . 287–296. 

- [114] Yao Fu, Leyang Xue, Yeqi Huang, and Andrei-Octavian Brabete. 2023. ServerlessLLM: low-latency serverless inference for large language models. In _Proc. OSDI_ . 135–153. 

- [115] Holger Kinkelin, Richard von Seck, Christoph Rudolf, and Georg Carle. 2020. Hardening X.509 Certificate Issuance using Distributed Ledger Technology. In _Proc. NOMS_ . 1–6. 

- [116] Patricia Arias-Cabarcos, Christian Krupitzer, and Christian Becker. 2019. A Survey on Adaptive Authentication. _ACM Computing Survey_ 52, 4 (2019), 1–30. 

- [117] Manoj Karkee Christos Emmanouilidis Shaina Raza, Ranjan Sapkota. 2025. TRiSM for Agentic AI: A Review of Trust, Risk, and Security Management in LLM-based Agentic Multi-Agent Systems. _ArXiv preprint: ArXiv:2506.04133_ (2025). 

- [118] Ziye Geng, Yunhua He, Chao Wang, Gang Xu, Ke Xiao, and Shui Yu. 2021. A Blockchain based Privacy-Preserving Reputation Scheme for Cloud Service. In _Proc. ICC_ . 1–6. 

- [119] Mahdi Ghafoorian, Dariush Abbasinezhad-Mood, and Hassan Shakeri. 2019. A Thorough Trust and Reputation Based RBAC Model for Secure Data Storage in the Cloud. _IEEE Transactions on Parallel and Distributed Systems_ 30, 4 (2019), 778–788. 

- [120] A. Kayes et al. 2020. A Survey of Context-Aware Access Control Mechanisms for Cloud and Fog Networks: Taxonomy and Open Research Issues. _Sensors_ 20, 9 (2020), 1–34. 

- [121] Cunchen Hu et al. 2024. MemServe: Context Caching for Disaggregated LLM Serving with Elastic Memory Pool. _ArXiv preprint: ArXiv:2406.17565_ (2024). 

- [122] [n. d.]. CUDA API libraries. 2025. https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__VA.html 

- [123] Minjae Kim, Taehyeong Kwon, Kibeom Shim, and Beonghoon Kim. 2024. Protection of LLM Environment Using Prompt Security. In _Proc. ICTC_ . 1715–1719. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

35:35 

Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey 

- [124] Xiaoyu Zhang et al. 2025. JailGuard: A Universal Detection Framework for Prompt-based Attacks on LLM Systems. _ACM Transactions on Software Engineering and Methodology_ (Mar. 2025). 

- [125] Oleksandr Muliarevych. 2024. Enhancing System Security: LLM-Driven Defense Against Prompt Injection Vulnerabilities. In _Proc. TCSET_ . 420–423. 

- [126] Yucheng Li, Surin Ahn, Huiqiang Jiang, Amir H. Abdi, Yuqing Yang, and Lili Qiu. 2025. SecurityLingua: Efficient Defense of LLM Jailbreak Attacks via Security-Aware Prompt Compression. _ArXiv preprint: ArXiv:2506.12707_ (2025). 

- [127] Zaibin Zhang et al. 2024. PsySafe: A Comprehensive Framework for Psychological-based Attack, Defense, and Evaluation of Multi-agent System Safety. In _Proc. ACL_ . 15202–15231. 

- [128] Miao Yu et al. 2025. NetSafe: Exploring the Topological Safety of Multi-agent Networks. _ArXiv preprint: ArXiv:2410.15686_ (2025). 

- [129] Shilong Wang et al. 2025. G-Safeguard: A Topology-Guided Security Lens and Treatment on LLM-based Multi-agent Systems. In _Proc. ACL_ . 7261–7276. 

- [130] Jialong Zhou et al. 2025. GUARDIAN: Safeguarding LLM Multi-Agent Collaborations with Temporal Graph Modeling. _ArXiv preprint: ArXiv:2505.19234_ (2025). 

- [131] Xu He, Di Wu, Yan Zhai, and Kun Sun. 2025. SentinelAgent: Graph-based Anomaly Detection in Multi-Agent Systems. _ArXiv preprint: ArXiv:2505.24201_ (2025). 

- [132] Yinqiu Liu, Kun Wang, Yun Lin, and Wenyao Xu. 2019. LightChain: A Lightweight Blockchain System for Industrial Internet of Things. _IEEE Transactions on Industrial Informatics_ 15, 6 (2019), 3571–3581. 

- [133] Huawei Huang et al. 2025. BlockEmulator: An Emulator Enabling to Test Blockchain Sharding Protocols. _IEEE Transactions on Services Computing_ 18, 2 (2025), 690–703. 

- [134] Kai Lei, Maoyu Du, Jiyue Huang, and Tong Jin. 2020. Groupchain: Towards a Scalable Public Blockchain in Fog Computing of IoT Services Computing. _IEEE Transactions on Services Computing_ 13, 2 (2020), 252–262. 

- [135] Jiewu Leng, Man Zhou, J. Leon Zhao, Yongfeng Huang, and Yiyang Bian. 2022. Blockchain Security: A Survey of Techniques and Research Directions. _IEEE Transactions on Services Computing_ 15, 4 (2022), 2490–2510. 

- [136] Haoxiang Luo et al. 2025. A Trustworthy Multi-LLM Network: Challenges, Solutions, and A Use Case. _ArXiv preprint: ArXiv:2505.03196_ (2025). 

- [137] Rabimba Karanjai and Weidong Shi. 2024. Trusted LLM Inference on the Edge with Smart Contracts. In _Proc. ICBC_ . 1–7. 

- [138] Zefeng Mo, Zhihao Hou, Ruilin Lai, Xiaoyuan Wu, Junjie Zhou, and Gansen Zhao. 2025. A Blockchain-Based Framework for Crowdsourcing Evaluation of Large Language Models. In _Blockchain and Web3.0 Technology Innovation and Application_ . 62–71. 

- [139] Bei Chen, Gaolei Li, Xi Lin, Zheng Wang, and Jianhua Li. 2024. BlockAgents: Towards Byzantine-Robust LLM-Based Multi-Agent Coordination via Blockchain. In _Proc. ACM-TURC_ . 187–192. 

- [140] Peiming Xu, Huan Xu, Maoqiang Chen, Zhihong Liang, and Wenqian Xu. 2025. Privacy-Preserving Large Language Model in Terms of Secure Computing: A Survey. In _Proc. ASENS_ . 286–294. 

- [141] Wenjie Qu, Yijun Sun, Xuanming Liu, Tao Lu, Yanpei Guo, Kai Chen, and Jiaheng Zhang. 2025. zkGPT: An Efficient Non-interactive Zero-knowledge Proof Framework for LLM Inference. In _Proc. USENIX Security_ . 1–19. 

- [142] Boyi Liu et al. 2025. WiLLM: an Open Framework for LLM Services over Wireless Systems. _ArXiv preprint: ArXiv:2506.19030_ (2025). 

- [143] Boyi Liu, Jingwen Tong, and Jun Zhang. 2024. Poster Abstract: LLM-Slice: Dedicated Wireless Network Slicing for Large Language Models. In _Proc. SenSys_ . 853–854. 

- [144] Tom B. Brown et al. 2020. Language Models are Few-Shot Learners. _ArXiv preprint: ArXiv:2005.14165_ (2020). 

- [145] Melissa Z Pan et al. 2025. Why Do Multiagent Systems Fail?. In _Proc. ICLR_ . 1–40. 

- [146] Xiaowei Huang et al. 2024. A survey of safety and trustworthiness of large language models through the lens of verification and validation. _Artificial Intelligence Review_ 57, 175 (2024), 1–53. 

- [147] Behnaz Elhaminia et al. 2023. Toxicity Prediction in Pelvic Radiotherapy Using Multiple Instance Learning and Cascaded Attention Layers. _IEEE Journal of Biomedical and Health Informatics_ 27, 4 (2023), 1958–1966. 

- [148] Lingzhe Zhang, Yunpeng Zhai, Tong Jia, Xiaosong Huang, Chiming Duan, and Ying Li. 2025. AgentFM: Role-Aware Failure Management for Distributed Databases with LLM-Driven Multi-Agents. In _Proc. FSE-IVR_ . 1–5. 

- [149] Edward Y. Chang and Longling Geng. 2025. SagaLLM: Context Management, Validation, and Transaction Guarantees for Multi-Agent LLM Planning. _ArXiv preprint: ArXiv:2505.20824_ (2025). 

- [150] Abhinav Tiwari and Hany E. Z. Farag. 2025. Responsible AI Framework for Autonomous Vehicles: Addressing Bias and Fairness Risks. _IEEE Access_ 13 (2025), 58800–58822. 

- [151] Nouha Kherraf, Hyame Assem Alameddine, Sanaa Sharafeddine, Chadi M. Assi, and Ali Ghrayeb. 2019. Optimized Provisioning of Edge Computing Resources With Heterogeneous Workload in IoT Networks. _IEEE Transactions on Network and Service Management_ 16, 2 (2019), 459–474. 

ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025. 

