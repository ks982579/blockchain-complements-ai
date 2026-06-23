**==> picture [35 x 35] intentionally omitted <==**

## _Article_ 

## **The Paradox of AI Knowledge: A Blockchain-Based Approach to Decentralized Governance in Chinese New Media Industry** 

**Jing Wu[1] and Yaoyi Cai[2,] *** 

- 1 School of Humanities and Communication, Xiamen University Tan Kah Kee College, Zhangzhou 363123, China; julywu@xujc.com 

- 2 School of Digital Media, Shenzhen Polytechnic University, Shenzhen 518055, China ***** Correspondence: caiyaoyi@szpu.edu.cn 

## **Abstract** 

AI text-to-video systems, such as OpenAI’s Sora, promise substantial efficiency gains in media production but also pose risks of biased outputs, opaque optimization, and deceptive content. Using the Orientation–Stimulus–Orientation–Response (O-S-O-R) model, we conduct an empirical study with 209 Chinese new media professionals and employ structural equation modeling to examine how information elaboration relates to AI knowledge, perceptions, and adoption intentions. Our findings reveal a knowledge paradox: higher objective AI knowledge negatively moderates elaboration, suggesting that centralized information ecosystems can misguide even well-informed practitioners. Building on these behavioral insights, we propose a blockchain-based governance framework that operationalizes five mechanisms to enhance oversight and trust while maintaining efficiency: Expert Assessment DAOs, Community Validation DAOs, real-time algorithm monitoring, professional integrity protection, and cross-border coordination. While our study focuses on China’s substantial new media market, the observed patterns and design principles generalize to global contexts. This work contributes empirical grounding for Web3-enabled AI governance, specifies implementable smart-contract patterns for multi-stakeholder validation and incentives, and outlines a research agenda spanning longitudinal, cross-cultural, and implementation studies. 

Academic Editor: Qiang Qu Received: 15 September 2025 Revised: 14 October 2025 Accepted: 16 October 2025 Published: 20 October 2025 

**Citation:** Wu, J.; Cai, Y. The Paradox of AI Knowledge: A Blockchain-Based Approach to Decentralized Governance in Chinese New Media Industry. _Future Internet_ **2025** , _17_ , 479. https:// doi.org/10.3390/fi17100479 

**Copyright:** © 2025 by the authors. Licensee MDPI, Basel, Switzerland. This article is an open access article distributed under the terms and conditions of the Creative Commons Attribution (CC BY) license (https://creativecommons.org/ licenses/by/4.0/). 

**Keywords:** artificial intelligence; Sora; blockchain governance; decentralized autonomous organizations; new media industry 

## **1. Introduction** 

The COVID-19 pandemic accelerated digital transformation across media industries worldwide. Previously, media industries operated through established production workflows and centralized governance structures. The pandemic accelerated digital transformation as professionals rapidly adopted ICT tools and resilience-oriented practices under crisis conditions, often bypassing systematic governance frameworks [1–7]. As a result, the post-pandemic era presents the challenge of institutionalizing these adaptive behaviors into durable community-driven oversight mechanisms. The emergence of advanced generative AI technologies exemplifies this governance challenge. 

In February 2024, OpenAI released Sora, its first large-scale text-to-video generative model, representing a watershed moment for media professionals globally. The development of AI technology profoundly impacts media professionals by fundamentally 

https://doi.org/10.3390/fi17100479 

_Future Internet_ **2025** , _17_ , 479 

_Future Internet_ **2025** , _17_ , 479 

2 of 31 

transforming content production processes and reshaping the entire professional ecosystem. Sora’s ability to generate up to 60-s videos directly from text prompts demonstrates AI’s potential to dramatically enhance efficiency in content creation, enabling professionals to produce high-quality content with unprecedented speed and minimal resource requirements. However, alongside these efficiency gains, AI technologies introduce critical challenges that threaten professional integrity and public trust [8]. AI’s potential to mislead media professionals emerges as a central concern, manifesting through (1) Algorithmic bias that skews content recommendations and production decisions, (2) Generated misinformation that professionals may unknowingly incorporate into their work, (3) Manipulated training data that embeds harmful stereotypes or false information into AI outputs, and (4) Black-box decision-making processes that obscure how AI systems reach their conclusions, making it difficult for professionals to verify accuracy or appropriateness. 

Existing research on AI adoption has predominantly employed technology-centric models that treat governance as an external constraint rather than a behavioral driver. While COVID-era studies [2,3] documented how organizational resilience and ICT adoption shaped operational outcomes in supply chains and micro, small, and medium enterprises (MSMEs), these macro-level analyses cannot explain how individual professionals process governance-related information when deciding whether to adopt AI tools with potential to mislead them. This study addresses this gap by examining whether individual orientations (AI knowledge, institutional trust) moderate the relationship between information elaboration and adoption intentions—a moderation claim with profound implications for decentralized governance design. 

According to global industry reports, video content dominates digital consumption across all major markets, with professionals in Europe, North America, Latin America, and Asia-Pacific facing similar pressures to adopt AI tools while managing associated risks [9]. The fundamental tension between AI’s efficiency benefits and its potential to mislead exists regardless of geographical or cultural context. China is a global leader in both video production and consumption. According to the [10], by December 2023 the country had 1.074 billion online audiovisual users, representing 98.3% of all internet users, making it the most widely used internet application. Video apps also dominate usage time, with short-video platforms ranking highest at 151 min per person per day, followed by long-video platforms at 112 min [11]. This massive market sustains the Chinese video industry and provides livelihoods for millions of content creators. By 2023, China hosted 1.55 billion short-video accounts and 15.08 million full-time live-streamers. On short-video platforms alone, roughly 80 million new videos are uploaded daily, alongside over 3.5 million live-stream sessions [11]. This massive scale provides an ideal empirical setting for examining professional responses to GenAI. However, specific institutional arrangements and regulatory frameworks do not transfer directly to other national contexts with different governance traditions and media industry structures. This urgency reflects a post-pandemic transition: pandemic disruptions accelerated digitization of media production as professionals rapidly adopted new ICT tools under crisis conditions, often bypassing established governance frameworks in favor of immediate operational continuity. 

The rapid spread of AI across industries is irreversible, and public attitudes toward such technologies are more complex than ever. On the one hand, AI sparks [12], particularly fears of technological replacement of human labor [13,14]. For content creators, the possibility that Sora may trigger waves of unemployment is especially concerning. Indeed, in May 2023 Hollywood witnessed its first industry-wide strike in 63 years, as writers and actors protested unregulated AI use by streaming platforms, warning that creative jobs like screenwriting could be replaced [15]. China’s short-video industry is arguably even more vulnerable: while AI can already generate scripts, Sora demonstrates that actors, 

_Future Internet_ **2025** , _17_ , 479 

3 of 31 

videographers, and other roles might also be displaced, making the risks of disruption far more immediate and tangible for Chinese creators. On the other hand, AI also opens new opportunities. As one popular Reddit post noted, “Having ChatGPT is like carrying a PhD in your pocket”. AI tools are designed not to replace humans but to augment human knowledge and decision-making capacity [16]. This technological optimism complicates how content creators view Sora. Rapid adoption of AI tools may help them boost efficiency, reduce workload, and gain competitive advantage, an especially compelling prospect in China’s fiercely competitive video industry, where Sora’s commercial potential cannot be ignored. 

In terms of China’s new media sector, Sora embodies both an imminent threat and a possible breakthrough: a technology that could eliminate creative jobs, yet also one that may allow creators to outpace peers in an “evolutionary” environment. This paradox underscores the value of examining how Chinese content creators and media professionals perceive and accept Sora, and critically, how to govern such technologies to counteract their potential to mislead professionals. Effective governance requires mechanisms that can balance oversight with innovation. Existing governance approaches face critical limitations. Traditional regulatory frameworks suffer from regulatory lag that cannot keep pace with rapidly evolving AI technologies, jurisdictional constraints that struggle with globally deployed systems, and limited technical expertise to assess complex algorithms [17–19]. Industry self-regulation encounters fundamental conflicts of interest where profit motives override public protection and lacks binding enforcement mechanisms [20,21]. Multistakeholder governance bodies experience decision-making paralysis, vulnerability to capture by dominant players, and absence of enforcement authority [22–24]. 

In contrast, blockchain-based decentralized autonomous organizations (DAOs) offer unique advantages that directly address these limitations. DAOs provide algorithmic transparency and auditability through immutable on-chain records of governance decisions and algorithmic parameters [25,26]. Distributed consensus mechanisms aggregate expert knowledge while preventing single-entity dominance [27,28]. Programmable enforcement via smart contracts automatically executes community decisions without centralized authorities [29,30]. Economic incentive alignment through tokenomics rewards quality oversight and long-term community benefit [31,32]. Furthermore, cross-border coordination enables global participation while maintaining regulatory interoperability [33,34]. These blockchain-based communities can develop collective norms and rules that guide AI algorithm behavior through transparent governance protocols, implement real-time auditing systems where community members continuously monitor AI outputs for bias, misinformation, or harmful content, create accountability frameworks with smart contracts that automatically penalize AI systems producing misleading content and reward accurate ethical outputs, and establish professional standards through community consensus that ensures AI tools enhance rather than compromise professional integrity. 

The community governance frameworks we propose based on our behavioral findings apply universally across different media markets and regulatory environments. DAOs can operate across jurisdictional boundaries while respecting local regulations, enabling international cooperation in AI oversight. Media professionals worldwide can participate in these decentralized governance systems regardless of their geographic location, contributing to collective intelligence that benefits the global media ecosystem. Although scholarship on AI adoption is expanding, most studies rely on technology-centered models while neglecting how behavioral insights can inform community-based governance design for addressing AI’s potential to mislead professionals. Our study addresses this gap by applying the O-S-O-R model to understand behavioral patterns that inform the design of DAO-based governance systems. Specifically, how the knowledge paradox (where higher AI expertise 

_Future Internet_ **2025** , _17_ , 479 

4 of 31 

correlates with increased skepticism through information elaboration) necessitates governance mechanisms that properly weight expert skepticism through token-weighting, how strong benefit perception effects require transparent verification of AI benefits and risks through blockchain’s immutable audit trails, and how variation in information processing patterns demands multi-token architectures with differentiated participation mechanisms for technical experts, community validators, and affected stakeholders. 

Based on our empirical findings, we propose a multi-layered DAO structure: 

- Technical Assessment DAOs composed of AI experts who evaluate algorithm design and identify potential bias sources; 

- Content Verification DAOs, where media professionals collectively verify AI-generated content accuracy and appropriateness; 

- Ethics Oversight DAOs that establish and enforce professional standards for AI use in media; 

- Community Impact DAOs that assess how AI deployment affects different stakeholder groups and adjust governance policies accordingly. 

To address these gaps, this study applies the Orientation–Stimulus–Orientation– Response (O-S-O-R) model to investigate how Chinese content creators and media professionals perceive and respond to Sora. Specifically, it examines how they interpret this disruptive tool, why they may resist or embrace it, and what their responses reveal about broader debates over AI governance. Our findings inform the design of blockchain-based governance mechanisms that could address the identified behavioral patterns through decentralized decision-making structures. 

## **2. Theoretical Framework** 

## _2.1. The O-S-O-R Model and Decentralized Information Processing_ 

The Orientation–Stimulus–Orientation–Response (O-S-O-R) model is widely applied to explain how media messages influence individual behavioral intentions. Within this framework, _O1_ represents pre-orientation variables, _S_ denotes stimuli, _O2_ refers to postorientation variables influenced by those stimuli, and _R_ stands for behavioral response. In essence, the model illustrates how individuals’ pre-existing orientations shape their exposure to certain stimuli, which in turn affects their subsequent orientations and ultimately determines their behavioral outcomes [35]. 

Crucially, the O-S-O-R framework maps directly onto blockchain-based governance mechanisms, enabling a behavioral foundation for decentralized AI oversight. Individual orientations ( _O1_ ) correspond to stakeholder characteristics that determine participation roles in decentralized autonomous organizations (DAOs): professionals with high AI knowledge (measured through technical assessments) hold EXPERT tokens and evaluate algorithm design, while professionals with domain expertise in content creation hold COMMUNITY tokens and validate outputs. Stimuli ( _S_ ) represent information processing activities enhanced through blockchain-based transparency: algorithm audit reports, content flagging events, and governance proposals qualify as stimuli that trigger community review when they exceed predefined risk thresholds encoded in smart contracts. Postorientation variables ( _O2_ ), such as credibility assessments, risk perceptions, and benefit evaluations, align with collective intelligence formation that DAOs facilitate through smart contract-mediated consensus mechanisms: when individual _O2_ evaluations aggregate to form collective risk scores above threshold levels, smart contracts automatically trigger on-chain checks including mandatory third-party audits, temporary algorithm pauses, or escalated review procedures. Finally, behavioral responses ( _R_ ) translate into governance participation and token-based decision-making: adoption intentions manifest as on-chain 

_Future Internet_ **2025** , _17_ , 479 

5 of 31 

votes, staking behaviors, and participation in validation activities that characterize Web3 ecosystems [25]. 

Specifically, pre-orientation variables ( _O1_ ) typically capture internal personal characteristics such as personality traits or values, which influence how individuals engage with stimuli ( _S_ ). Stimuli represent environmental factors that trigger emotional or cognitive reactions, often in the form of mediated information. Post-orientation variables ( _O2_ ) describe the cognitive processes through which individuals interpret and evaluate stimuli prior to forming a behavioral response ( _R_ ) [36]. Although the O-S-O-R framework has been less frequently applied to studies of technology adoption, it is particularly suitable for examining how content creators and professionals in the Chinese media sector perceive _Sora_ . The model helps to reveal how individual characteristics and information-processing behaviors shape acceptance intentions toward this disruptive AI technology. 

First, the O-S-O-R model emphasizes the role of information environments in shaping attitudes and behavioral intentions. Since _Sora_ has not yet been officially released, most content creators’ knowledge and perceptions of the technology derive from their information environments, especially social media platforms. These platforms provide ample opportunities for encountering and discussing technology-related topics [37,38] and have become a primary source of AI-related information. Moreover, the ways in which AI technologies are presented in news and media coverage strongly influence and reflect public attitudes [39]. Using the O-S-O-R framework therefore allows this study to examine how information environments, via mediating mechanisms, shape intentions to adopt _Sora_ . 

Second, the model highlights the importance of personal characteristics ( _O1_ ) in shaping responses to technology. Given the uncertainty and opacity of AI, individuals who are more risk-tolerant, more willing to learn, or more motivated by curiosity are more likely to adopt AI tools [40]. By emphasizing _O1_ , the O-S-O-R model offers a useful lens for understanding how differences among Chinese content creators and professionals affect their engagement with information about _Sora_ , and how these differences ultimately shape their acceptance intentions [41]. 

This behavioral governance approach builds on resilience mechanisms that emerged during the COVID-19 pandemic, institutionalizing effective crisis adaptations into durable oversight structures. Recent COVID-era studies [2,6,7] reveal organizational adaptations that parallel the DAO mechanisms we propose. These mechanisms align with our benefitperception pathways (H4, H7), where transparent DAO governance enhances professionals’ perceived benefits by providing algorithmic accountability that centralized systems lack. 

## _2.2. Blockchain-Based Governance Theory_ 

The behavioral patterns captured by the O-S-O-R model have direct applications in blockchain governance system design. Traditional centralized AI development creates information asymmetries where technology companies control algorithm design, training data, and deployment policies while communities bear the consequences. These asymmetries manifest as the behavioral paradoxes we observe in adoption studies. 

**Smart Contract Governance Mechanisms** : Blockchain networks enable programmable governance through smart contracts that can automate decision-making based on community consensus. Unlike centralized systems where corporate executives make unilateral decisions about AI deployment, smart contracts can implement multi-stakeholder voting mechanisms that weight different perspectives based on expertise, stake, and community contribution [27,29]. However, smart contract governance faces notable limitations including: (1) code vulnerabilities that can be exploited—as demonstrated by The DAO hack in 2016, where $60 million was stolen due to a recursive call vulnerability [42], 

_Future Internet_ **2025** , _17_ , 479 

6 of 31 

(2) immutability challenges when bugs are discovered post-deployment, and (3) high technical barriers that limit broad participation [43]. 

**Decentralized Autonomous Organizations (DAOs)** : DAOs represent organizational structures where governance rights are distributed among token holders who participate in transparent, blockchain-recorded decision-making processes. For AI governance, DAOs can implement (1) technical assessment committees with domain experts, (2) community impact evaluation groups, (3) risk monitoring DAOs that continuously audit AI system behavior, and (4) benefit distribution mechanisms that ensure fair value capture from AI-generated content [26]. Recent advances in multi-agent blockchain systems demonstrate how these governance structures can be scaled and secured through collaborative AI agents that facilitate automated consensus mechanisms [44]. Nevertheless, empirical evidence from existing DAOs reveals persistent challenges. Polkadot’s governance system, despite sophisticated design, experiences voter participation rates below 15% for most proposals, with token concentration enabling governance by a small number of whale addresses [45]. MakerDAO similarly struggles with governance capture concerns, where technical complexity and voter apathy result in effective control by a minority of highly engaged token holders. These patterns suggest that DAO governance may replicate rather than resolve the concentration of power observed in traditional corporate structures. 

**Token Economics for Participation Incentives** : Blockchain systems implement sophisticated incentive mechanisms through tokenomics that address the participation barriers revealed by behavioral research. Governance tokens reward quality information processing, expert knowledge contribution, and long-term community benefit rather than short-term adoption optimization [31]. However, token-based governance introduces plutocratic tendencies where economic stake translates directly into voting power, potentially marginalizing stakeholders who are most affected by decisions but hold fewer tokens [46]. Additionally, token markets create speculative dynamics that misalign governance incentives, as short-term traders influence long-term protocol decisions [47]. 

**Information Processing in Blockchain Ecosystems** : The O-S-O-R model’s emphasis on information elaboration ( _S → O2_ ) closely aligns with blockchain’s transparency requirements. On-chain governance creates immutable records of all discussions, proposals, and decisions, enabling community members to engage in deep information processing about AI technologies. Unlike social media platforms where algorithms curate information exposure, blockchain governance platforms implement community-controlled information curation that addresses the expert skepticism we observe in our findings. Yet blockchain’s transparency creates trade-offs: 

- Privacy concerns, as all governance activities are publicly visible [48]; 

- Potential for vote-buying or coordinated manipulation attacks [49]; 

- Deterrence of candid deliberation since participants recognize that all statements are immutably recorded [50]. 

**Multi-Token Governance Architecture** : Based on behavioral insights, we propose differentiated token systems: 

- Expertise Tokens earned through demonstrated technical knowledge and awarded to participants who provide accurate risk assessments; 

- Community Tokens distributed to stakeholders affected by AI deployment decisions; 

- _•_ Validation Tokens earned through quality information processing and peer verification activities. 

This multi-token approach addresses the knowledge paradox by ensuring expert skepticism is properly weighted in governance outcomes. However, multi-token systems introduce additional complexity that exacerbates participation barriers and creates new 

_Future Internet_ **2025** , _17_ , 479 

7 of 31 

forms of inequality if certain stakeholder groups systematically struggle to earn particular token types [51]. The interoperability and exchange rates between token types also create design challenges without clear optimal solutions [52]. 

## _2.3. Smart Contract Implementation for AI Governance_ 

**Automated Policy Execution** : Smart contracts implement community decisions about AI usage policies without requiring centralized enforcement. For example, if a DAO votes to restrict certain types of AI-generated content, smart contracts automatically flag and filter such content based on cryptographic verification of AI model signatures and content provenance [53]. Advanced AI agents enhance these systems by providing intelligent data annotation and transformation capabilities that improve policy enforcement accuracy [54]. However, automated policy execution through smart contracts faces practical limitations: 

- The oracle problem where smart contracts cannot directly access off-chain data about AI model behavior without trusted intermediaries [55]; 

- Gas costs and scalability constraints that make continuous monitoring economically prohibitive on current blockchain networks; 

- Challenges in encoding nuanced policy decisions into deterministic code, potentially leading to rigid enforcement that cannot adapt to edge cases [56]. 

**Content Authenticity Verification** : Blockchain systems create immutable provenance records for AI-generated content, addressing trust concerns revealed in our study. Every AIgenerated video includes cryptographic signatures indicating the model used, parameters applied, and human oversight involved, with this information permanently recorded onchain for community verification. While this approach offers enhanced transparency, it also encounters limitations: 

- Provenance systems only verify that content metadata matches blockchain records, not whether the content itself is truthful or appropriate [57]. 

- Adversaries create authentic records of malicious content, making verification of technical provenance distinct from content quality assurance [58]. 

- Widespread adoption requires industry coordination and standardization that proves difficult to achieve given competitive dynamics among AI providers [57]. 

**Economic Incentive Alignment** : Smart contracts implement automated benefit distribution mechanisms where AI-generated revenue is distributed to community stakeholders based on their governance participation, risk assessment accuracy, and long-term value contribution rather than simple capital investment. This approach addresses economic concerns about AI displacing human workers by ensuring communities capture value from AI deployment. Nevertheless, economic mechanism design for AI governance involves unresolved challenges: 

- Measuring risk assessment accuracy or long-term value contribution objectively onchain remains technically difficult [59]. 

- Automated distribution mechanisms are gamed by participants who optimize for measurable metrics rather than genuine contribution [60]. 

- Legal and tax uncertainty around token-based compensation creates barriers to mainstream adoption, particularly for professional media organizations operating under traditional employment frameworks [61]. 

In addition, this study introduces AI knowledge as a moderating variable in the S _→_ O2 pathway in order to explore whether individuals’ actual knowledge levels condition how social media information elaboration influences their cognitive evaluations of Sora. Understanding these moderation effects has direct implications for designing inclusive 

_Future Internet_ **2025** , _17_ , 479 

8 of 31 

governance mechanisms that account for different expertise levels within communities. Thus, the O-S-O-R model in this study is operationalized as follows: 

- _O1_ (Pre-orientation variable): Interest in science and technology (IST). 

- _S_ (Stimulus): Social media information elaboration (SMIE), defined as the extent to which individuals actively interpret Sora-related content on social platforms. 

- _O2_ (Post-orientation variables): Confidence in AI knowledge (AIcon), risk perception (RP), and benefit perception (BP). 

- _R_ (Response): Intention to adopt Sora. 

- Moderator: Objective AI knowledge level (AIK). 

## **3. Research Hypotheses** 

Guided by the O-S-O-R framework and prior literature, we propose the following hypotheses linking information elaboration, knowledge, perceptions, and Sora adoption. 

_3.1. From Interest in Science and Technology to Social Media Information Elaboration (O1 → S): Blockchain Information Curation_ 

Accordingly, this study introduces _social media information elaboration (SMIE)_ as the stimulus (S) and also as the outcome predicted by _interest in science and technology (IST)_ (O1). Information elaboration refers to the cognitive process of integrating new information with existing knowledge to achieve effective learning [62]. In blockchain governance contexts, this process becomes community-validated information processing where multiple stakeholders contribute to collective intelligence formation. 

Blockchain-based information systems can enhance traditional social media information processing by implementing: (1) Cryptographic verification of information sources, (2) Community-driven fact-checking with economic incentives for accuracy, and (3) Transparent curation algorithms that are controlled by DAOs rather than corporate platforms. Users with stronger interest in science and technology are more likely to engage with this enhanced information processing mechanism, making them ideal participants in blockchain-based AI governance systems. 

Therefore, Hypothesis 1 is proposed: 

**H1.** _Interest in science and technology positively influences social media information elaboration about Sora among new media content creators and industry professionals, with implications for their potential participation in blockchain-based AI governance systems._ 

_3.2. From Social Media Information Elaboration to AI Knowledge Confidence, Risk Perception, and Benefit Perception (S → O2): Community Validation Mechanisms_ 

In blockchain governance systems, individual information processing becomes collective intelligence through community validation mechanisms. Smart contracts can aggregate individual assessments into community consensus, weighing contributions based on demonstrated expertise and past accuracy. This addresses the limitations of current social media environments where information quality varies significantly. 

**Blockchain-Enhanced Knowledge Confidence** : Traditional knowledge confidence suffers from information source opacity and lack of verification mechanisms. Blockchain systems can implement _reputation-based knowledge scoring_ where confidence is validated by community consensus and tracked immutably over time. Contributors who consistently provide accurate assessments earn higher reputation scores, creating sustainable incentives for quality information processing [25]. 

**Decentralized Risk Assessment** : Rather than relying on corporate risk assessments, blockchain governance can implement _distributed risk evaluation protocols_ where multiple stakeholders contribute to comprehensive risk matrices. Smart contracts can automati- 

_Future Internet_ **2025** , _17_ , 479 

9 of 31 

cally aggregate these assessments, weighting them based on contributor expertise and historical accuracy. This addresses the risk perception biases observed in centralized information environments. 

**Community-Driven Benefit Evaluation** : Blockchain systems enable _transparent benefit tracking_ where the actual outcomes of AI deployment are recorded on-chain and automatically inform future benefit assessments. Unlike traditional systems where benefit claims cannot be verified, blockchain governance creates auditable records of AI system performance and community impact. 

Therefore, Hypotheses 2, 3, and 4 are proposed: 

**H2.** _Social media information elaboration about Sora is positively associated with confidence in AI knowledge, with this relationship enhanced in blockchain-based governance systems through community validation mechanisms._ 

**H3.** _Social media information elaboration about Sora is negatively associated with risk perception of Sora, with blockchain governance enabling more accurate collective risk assessment through distributed evaluation protocols._ 

**H4.** _Social media information elaboration about Sora is positively associated with benefit perception of Sora, with blockchain systems providing transparent verification of benefit claims through immutable outcome tracking._ 

_3.3. From AI Knowledge Confidence, Risk Perception, and Benefit Perception to Intention to Adopt Sora (O2 → R)_ 

Confidence in one’s own knowledge exerts a significant influence on judgment and decision-making [63–72]. Subjective confidence in one’s computer security knowledge has a greater impact on user behavior than actual knowledge, as what people believe they know influences adoption more than what they truly know [64]. For example, individuals with IT backgrounds may be overconfident about their ability to detect deepfakes [73]. Evidence links subjective knowledge confidence with more positive views of technology [74]. Thus, if content creators are more confident in their AI knowledge, they are more inclined to adopt AI tools at work, such as Sora. Hence, Hypothesis 5 is proposed. 

**H5.** _Confidence in AI knowledge positively predicts intention to adopt Sora._ 

Research also shows that risk perception (RP) and benefit perception (BP) are key predictors of new-technology acceptance [75–77]. When evaluating new technologies, risk and benefit perceptions tend to be negatively correlated [78,79], perhaps because people automatically down-weight risk and up-weight benefits to avoid cognitive dissonance when they see a technology as advantageous [78]. Humans generally seek high benefit/low-cost options; lower risk perception and higher benefit perception positively predict acceptance across technologies [80–82]. For content creators, introducing AI tools into work is not risk-free. Fear of AI-induced unemployment is widely noted across countries [83]. Yet even in the face of risks, perceived benefits exert a stronger positive effect on AI-adoption intentions than perceived risks [81]. Hence, Hypotheses 6 and 7 are proposed. 

**H6.** _Risk perception of Sora negatively predicts intention to adopt Sora._ 

**H7.** _Benefit perception of Sora positively predicts intention to adopt Sora._ 

_Future Internet_ **2025** , _17_ , 479 

10 of 31 

## _3.4. Moderating Role of AI Knowledge_ 

Existing research on _AI knowledge (AIK)_ mainly relies on surveys to assess respondents’ AI knowledge levels [84–87]. Some studies examine relationships between AI knowledge and attitudes/trust toward AI, but findings are inconsistent. For instance, a study found that knowledge about autonomous vehicles correlates positively with willingness to ride in them—consistent with the intuition that knowledge helps people grasp small risks and large benefits, thus facilitating acceptance [88]. By contrast, a study on AI in healthcare work reported no direct relationship between physicians’ AI knowledge and their adoption of AI tools; even doctors with limited knowledge still tended to adopt AI in practice [89]. These contradictions may stem from multiple sources: (a) heterogeneous measurement approach—some employ objective tests and others utilizing subjective questionnaires; (b) domain specificity—AI applications span many sectors, so a single standardized scale is difficult; and (c) indirect effects—insufficient attention to mediating or moderating roles of AI knowledge may contribute to divergent conclusions. Therefore, this study not only assesses objective AI knowledge among Chinese new media workers in their professional domain but also explores the mechanisms by which AI knowledge operates in the adoption process. Therefore, Research Question 1 is posed. 

**RQ1.** _Does AI knowledge (AIK) moderate the effects of social media information elaboration (SMIE) on AI knowledge confidence (AIcon), risk perception (RP), and benefit perception (BP)?_ 

## _3.5. Demographic Correlates of Adoption Intentions_ 

In technology adoption research, demographics—such as gender, age, income, and education—are long-standing predictors. For example, gender and age significantly influence mobile technology adoption [90,91]. In autonomous-vehicle research, higher-income, younger, and more educated individuals are more willing to pay, while gender effects are sometimes null [92], though other studies find men more likely to adopt [93]. Higher education is associated with greater technological knowledge [94,95] and faster adoption and is also considered an important moderator in autonomous-vehicle acceptance [96]. Given the work-domain focus of this study, we include industry tenure in the new media sector as a factor. Although prior research has not directly linked tenure to AI adoption, studies in the hospitality sector show significant moderating effects of industry tenure on technology acceptance [97]. Thus, Research Question 2 is posed. 

**RQ2.** _Do age, gender, education, industry tenure, and income significantly relate to intention to adopt Sora among China’s new media content creators and industry professionals?_ 

The theoretical model is presented in Figure 1. 

**==> picture [350 x 144] intentionally omitted <==**

**----- Start of picture text -----**<br>
Confidence in<br>AI<br>Interest in Sci- Social Media  Risk Percep- Intention<br>ence &Tech- Information<br>nology Elaboration tion to Adopt<br>= 4 4‘ 7 >}<br>aw,<br>a7 W<br>a7? oe<br>1/47if > Benefit Per-<br>{1,77, ception<br>uy?<br>AI Knowledge<br>**----- End of picture text -----**<br>


**Figure 1.** Theoretical Framework. 

_Future Internet_ **2025** , _17_ , 479 

11 of 31 

## **4. Research Design** 

We describe the sampling strategy, measurement instruments, and modeling procedures used to test the hypotheses. 

## _4.1. Sampling Design_ 

We implemented a cross-sectional online survey targeting China’s new media workforce, with an emphasis on video content creators on major platforms. Data were collected via an online questionnaire link (supported by an online survey platform wjx.cn) between April and May 2024 using snowball sampling through industry referrals and creator communities. 

Current employment in the new media sector was required for participants to be included in the study. A screening question at survey entry filtered out ineligible respondents. In total, 290 responses were received, 79 were excluded by the screener as non-industry, and 2 additional cases were removed for low-quality/patterned responding, yielding a final analytic sample of N = 209. Participation was voluntary and informed consent was obtained prior to participation. 

Among these 209 respondents, the mean age was 32.1, and 40.7% were women. Over half (56.94%) reported annual income below RMB 200,000. Nearly all (99%) held an undergraduate degree or above, reflecting the sector’s educational profile. Additional characteristics are summarized in Table 1. 

**Table 1.** Descriptive statistics of the sample (N = 209). 

|**Characteristics**|**Category**|**N**|**%**|**M**|**SD**|
|---|---|---|---|---|---|
|Gender|Male|124|59.30%|1.41|0.492|
||Female|85|40.70%|||
|Age|18–24|38|18.18%|2.23|0.812|
||25–34|97|46.41%|||
||35–44|62|29.67%|||
||45–64|12|5.74%|||
||Over 64|0|—|||
|Education|High school or below|2|1.00%|2.22|0.435|
||Undergraduate or college|160|76.56%|||
||Postgraduate or above|47|22.49%|||
|Industry tenure|3 years|71|33.97%|1.92|0.771|
||3–10 years|84|40.19%|||
||Over 10 years|54|25.84%|||
|Annual Income|Under 99,999 Yuan|53|25.36%|2.64|1.523|
||100,000–199,999 Yuan|66|31.58%|||
||200,000–299,999 Yuan|42|20.10%|||
||300,000–399,999 Yuan|20|9.57%|||
||400,000–499,999 Yuan|8|3.83%|||
||Over 500,000 Yuan|20|9.57%|||



## _4.2. Measurement_ 

All items used five-point Likert-type scales unless otherwise specified. Scale reliabilities and descriptive statistics are reported with each construct. 

**Interest in Science and Technology (IST).** Five items adapted from [98] assessed respondents’ interest in science and technology (e.g., “For me science is an exciting topic”, 1 = strongly disagree, 5 = strongly agree). Higher scores indicate stronger interest (M = 3.91, SD = 0.91, α = 0.92). 

**Social Media Information Elaboration (SMIE).** Three items adapted from [99] measured depth of processing of Sora-related content on social media (e.g., “After getting 

_Future Internet_ **2025** , _17_ , 479 

12 of 31 

information about Sora from social media, I stop and think about it”). Higher scores indicate greater elaboration (M = 3.47, SD = 1.04, α = 0.89). 

**AI Knowledge (AIK) and Confidence in AI Knowledge (AIcon).** Following [100], AIK was measured by nine domain-relevant true/false items tailored to media contexts (e.g., “Using deepfake, videos of famous people can be faked”, 1 = correct, 0 = incorrect). Total correct reflects objective knowledge (M = 4.66, SD = 0.69; mean accuracy = 66.6%). After confirmatory analysis, Items 5 and 9 (loadings 0.51 and 0.56) were dropped. AIcon captured respondents’ confidence in their judgments for each AIK item (1 = guess, 6 = certain), higher scores indicate greater confidence (M = 4.88, SD = 0.98, α = 0.90). 

**Risk Perception (RP) and Benefit Perception (BP) of Sora.** Based on [100], both constructs used six items. RP items asked, for example, “When you think about this use of Sora, to what extent are you troubled?” (higher = greater perceived risk, M = 2.72, SD = 0.93, α = 0.90). BP items asked, for example, “How optimistic, if at all, are you when you think about this use of Sora” (higher = greater perceived benefits, M = 3.37, SD = 0.88, α = 0.93). 

**Intention to Adopt Sora (IAS).** Two items adapted from [101] gauged adoption intention in work settings (e.g., “I support adopting Sora and related technologies in my job”, 1 = strongly disagree, 5 = strongly agree). Higher scores indicate stronger intention (M = 3.62, SD = 1.03, α = 0.93). 

## **5. Results** 

We report measurement quality and structural relations in sequence. First, we assess reliability, convergent validity, and discriminant validity (loadings, CR, AVE, and VIF). Next, we test the hypothesized paths and moderation effects in the structural model, with emphasis on the knowledge paradox (objective AI knowledge negatively moderating elaboration). Finally, we summarize overall model fit and robustness checks. 

## _5.1. Measurement Model_ 

This study evaluates the measurement and structural models in Smart PLS 4.0, and reports PLS-SEM results following the recommendations of [102]. 

Item factor loadings are assessed. Except that Items 5 and 9 of the latent variable Confidence in AI Knowledge are removed, the remaining loadings range from 0.722 to 0.966, meeting the recommended threshold (>0.708, see Table 2). Next, variance inflation factors (VIFs) are examined for multicollinearity, results indicate no collinearity concerns, with all item VIFs < 5.0 [102,103]. 

We compute Cronbach’s α for latent variables with two or more items to assess reliability. For the two-item latent variable (Intention to Adopt Sora), we report the Spearman– Brown coefficient [104]. All reliability coefficients fall within an acceptable range, with α and Spearman–Brown between 0.887 and 0.932 (see Table 2). 

Convergent validity is assessed using average variance extracted (AVE) and composite reliability (CR). AVE values should be _≥_ 0.50 [105], and CR values _≥_ 0.70 [106]. Results show that all latent variables meet these standards (see Table 2). 

_Future Internet_ **2025** , _17_ , 479 

13 of 31 

**Table 2.** Constructs, items, and reliability and validity assessments. 

|**Constructs**|**Items**|**FL**|**VIF**|**α**|**SpearmanBrown**|**AVE**|**CR**|
|---|---|---|---|---|---|---|---|
|Interest in|IST1|0.751|1.798|0.92||0.761|0.941|
|Science and|IST2|0.881|3.009|||||
|Technology|IST3|0.889|3.059|||||
||IST4|0.934|4.855|||||
||IST5|0.895|3.546|||||
|Social Media|SMIE1|0.877|2.062|0.887||0.816|0.93|
|Information|SMIE2|0.922|3.364|||||
|Elaboration|SMIE3|0.91|3.051|||||
|Risk|RP1|0.722|2.098|0.898||0.64|0.914|
|Perception|RP2|0.829|3.305|||||
||RP3|0.802|2.576|||||
||RP4|0.824|3.146|||||
||RP5|0.817|1.479|||||
||RP6|0.799|2.819|||||
|Confdence in|AIcon1|0.783|2.515|0.896||0.617|0.918|
|AI|AIcon2|0.87|3.343|||||
|Knowledge|AIcon3|0.776|2.102|||||
||AIcon4|0.805|2.293|||||
||AIcon6|0.728|2.369|||||
||AIcon7|0.759|2.476|||||
||AIcon8|0.768|1.881|||||
|Beneft|BP1|0.81|2.759|0.932||0.746|0.946|
|Perception|BP2|0.839|3.036|||||
||BP3|0.851|3.122|||||
||BP4|0.894|3.585|||||
||BP5|0.906|4.903|||||
||BP6|0.877|4.3|||||
|Intention to|IAS1|0.966|4.006|—|0.928|0.933|0.965|
|Adopt Sora|IAS2|0.966|4.006|||||



To address increasing criticism of the Fornell–Larcker criterion [102,107,108], this study uses the HTMT (Heterotrait–Monotrait ratio) as the basis for discriminant validity [108]. As shown in Table 3, all HTMT values are clearly below the 0.90 threshold, indicating adequate discriminant validity. 

**Table 3.** The HTMT discriminant validity correlation matrix. 

||**(1)**|**(2)**|**(3)**|**(4)**|**(5)**|
|---|---|---|---|---|---|
|(1) Intention to Adopt Sora|—|—|—|—|—|
|(2) Confdence in AI Knowledge|0.402|—|—|—|—|
|(3) Beneft Perception|0.778|0.317|—|—|—|
|(4) Risk Perception|0.148|0.12|0.26|—|—|
|(5) Social Media Information<br>Elaboration|0.518|0.37|0.543|0.22|—|
|(6) Interest in Science and<br>Technology|0.381|0.259|0.418|0.111|0.657|



## _5.2. Structural Modeling_ 

We evaluate the hypothesized paths using PLS-SEM, reporting standardized coefficients, _t_ statistics, _p_ values, and _R_[2] , and test moderation via AIK _×_ SMIE interaction terms. 

## 5.2.1. Path Testing 

Path coefficients are tested using bias-corrected bootstrapped confidence intervals (5000 subsamples). Detailed PLS-SEM results are reported in Table 4, and the visualized 

_Future Internet_ **2025** , _17_ , 479 

14 of 31 

paths are provided in Figure 2. As shown in Figure 2, the model follows these pathways: Interest in Science and Technology significantly and positively predicts Social Media Information Elaboration (β = 0.600, _t_ = 11.573, _p_ < 0.001, _f_[2] = 0.564). Information elaboration shows significant positive effects on Confidence in AI Knowledge (β = 0.294, _t_ = 4.884, _p_ < 0.001, _f_[2] = 0.121), Benefit Perception (β = 0.486, _t_ = 8.478, _p_ < 0.001, _f_[2] = 0.323), and Risk Perception (β = 0.231, _t_ = 2.414, _p_ < 0.05, _f_[2] = 0.057). Confidence in AI Knowledge (β = 0.169, _t_ = 3.800, _p_ < 0.001, _f_[2] = 0.058) and Benefit Perception (β = 0.051, _t_ = 13.119, _p_ < 0.001, _f_[2] = 0.854) are positively associated with Intention to Adopt Sora. These results indicate that H1, H2, H3, H4, H5, and H7 are supported, demonstrating explanatory power of the O–S–O–R model for Sora adoption intention; H6 is not supported, suggesting that Risk Perception does not affect intention to adopt Sora among new media workers. 

**Table 4.** Results of PLS-SEM. 

|**Path**|**Std β**|**_t_ Statistics**|**_p_ Values**|**_f_ 2**|**_R_2**|
|---|---|---|---|---|---|
|IST -> SMIE|0.600|11.573|0.000|0.564|0.357|
|SMIE -> AIcon|0.294|4.884|0.000|0.121|0.292|
|SMIE -> BP|0.486|8.478|0.000|0.323|0.277|
|SMIE -> RP|0.231|2.414|0.016|0.057|0.079|
|AIcon -> IAS|0.169|3.800|0.000|0.058|0.580|
|BP -> IAS|0.051|13.119|0.000|0.854|-|
|RP -> IAS|0.068|0.072|0.942|0.000|-|
|Age -> IAS|0.106|2.047|0.041|0.018|-|
|Education -> IAS|_−_0.057|1.221|0.222|0.007|-|
|Gender -> IAS|_−_0.053|0.579|0.563|0.002|-|
|Annual salary -> IAS|0.015|0.327|0.744|0.000|-|
|Industry tenure -> IAS|_−_0.035|0.706|0.481|0.002|-|
|AIK x SMIE -> AIcon|_−_0.251|3.824|0.000|0.103|-|
|AIK x SMIE -> BP|_−_0.135|2.329|0.020|0.029|-|
|AIK x SMIE -> RP|_−_0.067|0.674|0.501|0.006|-|



Note: IST = Interest in Science and Technology, SMIE = Social Media Information Elaboration, AIcon = Confidence in AI knowledge, BP = Benefit Perception, RP = Risk perception, AIK = AI Knowledge. 

**Figure 2.** Structural Equation Modeling. 

_Future Internet_ **2025** , _17_ , 479 

15 of 31 

Regarding RQ1, results indicate a reverse (negative) moderation of AI knowledge on the associations between social media information elaboration (SMIE) and both confidence in AI knowledge (AIcon) and benefit perception (BP). Without the moderator, SMIE is positively associated with AIcon (β = 0.294, _p_ < 0.001) and BP (β = 0.486, _p_ < 0.001). When AI knowledge is introduced as a moderator, the path coefficients become negative: the interaction with SMIE negatively predicts AIcon (β = _−_ 0.251, _p_ < 0.001) and BP (β = _−_ 0.135, _p_ < 0.05). This pattern suggests that among respondents with higher objective AI knowledge, more intensive elaboration of Sora-related information on social media is associated with lower confidence in their AI knowledge and less optimistic benefit appraisals. 

Among demographic factors, only age shows a significant positive effect on intention to adopt Sora (IAS) (β = 0.106, _t_ = 2.047, _p_ < 0.05, _f_[2] = 0.018). This may reflect that over 75% of respondents are in the career advancement age range (25–44 years). Education (β = _−_ 0.057, _t_ = 1.221, _p_ > 0.05, _f_[2] = 0.007), gender (β = _−_ 0.053, _t_ = 0.579, _p_ > 0.05, _f_[2] = 0.002), annual income (β = 0.015, _t_ = 0.327, _p_ > 0.05, _f_[2] = 0.000), and industry tenure (β = _−_ 0.035, _t_ = 0.706, _p_ > 0.05, _f_[2] = 0.002) are all non-significant. 

## 5.2.2. Model Fit 

The model explains 58% of the variance in Intention to Adopt Sora. Stone–Geisser’s Q[2] values are computed via PLS blindfolding (D = 5) to evaluate predictive relevance. As shown in Table 5, except for Interest in Science and Technology, all constructs have Q[2] > 0; the relatively small sample size may affect the stability of blindfolding [103]. 

**Table 5.** Results of Q2 level assessment. 

|**Construct**||||**Construct**|||
|---|---|---|---|---|---|---|
|**Cross-Validated**||||**Cross-Validated**|||
||Redundancy||||Communality||
||SSO|SSE|Q2 (= 1_−_SSE/SSO)|SSO|SSE|Q2 (= 1_−_SSE/SSO)|
|AIcon|1463|1218.53|0.167|1463|751.689|0.486|
|BP|1254|1004.239|0.199|1254|452.598|0.639|
|IAS|418|204.586|0.511|418|140.118|0.665|
|IST|1045|1045|0.000|1045|375.968|0.64|
|RP|1254|1201.693|0.042|1254|661.386|0.473|
|SMIE|627|450.385|0.282|627|249.33|0.602|



Model fit is assessed through the standardized root mean square residual (SRMR) [109,110] and Goodness-of-Fit (GoF) [111] which are widely used for reporting PLS-SEM in current practice. The present model yields SRMR = 0.080, meeting Hu and Bentler’s _≤_ 0.08 criterion. GoF = 0.488 (>0.36) exceeds the benchmark recommended by [112], indicating good overall fit. 

## **6. Discussion** 

The findings of this study reveal critical behavioral patterns that inform the design of blockchain-based governance systems to address AI’s potential to mislead media professionals globally. Our empirical results demonstrate the need for decentralized community governance mechanisms that can establish norms, rules, and strategies to guide AI algorithms and audit their outputs, ensuring proper conduct for media professionals worldwide. 

## _6.1. The Knowledge Paradox: Evidence for Community-Driven AI Oversight_ 

Most critically, our findings reveal a “knowledge paradox” (β = _−_ 0.251, _p_ < 0.001) that provides empirical evidence for why centralized AI governance fails media professionals. Those with higher AI knowledge become more skeptical as they engage more deeply with available information, while those with less knowledge show increased confidence. This paradox demonstrates how current centralized information environments may mislead even knowledgeable professionals by creating systematic exclusion of expert perspectives. 

_Future Internet_ **2025** , _17_ , 479 

16 of 31 

**DAO-Based Solution** : This knowledge paradox provides strong empirical justification for implementing multi-tiered DAO governance, where 

- **Expert Assessment DAOs** capture technical knowledge through specialized governance tokens that weight expert skepticism as valuable input rather than an obstacle to adoption; 

- **Community Validation DAOs** ensure broader stakeholder perspectives are included in AI oversight decisions; 

- **Cross-DAO Consensus Mechanisms** use smart contracts to reconcile different perspectives and prevent any single viewpoint from dominating AI governance decisions. 

**Universal Applicability** : This knowledge paradox pattern likely exists among media professionals globally, not just in China, making our DAO governance solutions applicable across different countries and media markets. A joined mapping from these empirical patterns to concrete DAO governance components is provided in Table 6. 

**Table 6.** Empirical Findings to DAO Governance Mapping. 

|**Empirical Finding**|**Statistical Evidence**|**DAO Governance Application**|**Blockchain Implementation**|
|---|---|---|---|
|Knowledge Paradox|β=_−_0.251,_p_< 0.001|Expert Assessment DAOs weight<br>skepticism as valuable input|Smart contracts adjust voting<br>power based on expertise levels|
|Information Elaboration Effect|β= 0.294 (confdence),β= 0.486<br>(beneft),β= 0.231 (risk)|Multi-perspective<br>validation requirements|Consensus mechanisms reconcile<br>different viewpoints|
|Confdence-Adoption Link|β= 0.169,_p_< 0.001,_f_ 2 = 0.058|Reputation-based<br>credibility systems|Professional integrity tokens track<br>accuracy over time|
|Beneft Perception Impact|β= 0.051,_p_< 0.001,_f_ 2 = 0.854|Community-defned<br>quality standards|Automated reward systems for<br>guideline compliance|
|High Explained Variance|_R_2 = 58%|Comprehensive<br>governance framework|Multi-tiered DAO architecture<br>covers all behavioral patterns|
|Knowledge Test Results|66.6% accuracy|Global applicability evidence|Cross-border DAO<br>network viability|



## _6.2. Community-Driven AI Algorithm Auditing Systems_ 

Our findings on information elaboration patterns (β = 0.294 for confidence, β = 0.486 for benefit perception, β = 0.231 for risk perception) reveal how media professionals process AI-related information. These behavioral patterns inform the design of DAO-based auditing systems that can prevent AI from misleading professionals. 

**Real-Time Algorithm Monitoring DAOs** : Implement blockchain-based systems where community members continuously audit AI outputs through scalable multi-agent architectures that ensure secure collaboration between human validators and automated systems [54]: 

- **Bias Detection Protocols** : Smart contracts that automatically flag AI-generated content showing systematic bias against specific demographics, viewpoints, or content types. 

- **Misinformation Verification Networks** : DAO members cross-reference AI outputs against verified information sources, with consensus mechanisms determining content accuracy through advanced node classification algorithms that can evolve with largescale attributed network data [113]. 

- **Professional Standards Enforcement** : Community-established rules about appropriate AI use in different media contexts (news vs. entertainment vs. advertising). 

- **Algorithmic Transparency Requirements** : Mandate that AI systems provide explainable outputs that DAO members can evaluate for appropriateness. 

**Global Implementation Strategy** : These auditing systems can be deployed across different countries and media markets, with local DAOs adapting global standards to regional contexts while maintaining interoperability through blockchain networks. 

_Future Internet_ **2025** , _17_ , 479 

17 of 31 

## _6.3. DAO-Established Norms and Rules for AI Guidance_ 

The strong positive effects of benefit perception on adoption intentions (β = 0.051, _p_ < 0.001, _f_[2] = 0.854) suggest that media professionals need clear frameworks for realizing AI benefits while avoiding misleading outcomes. DAOs can establish these frameworks through. 

**Community Governance Protocols** : Community Governance Protocols outline DAOgoverned rules to ensure responsible AI use, such as 

- **Ethical AI Use Standards** : DAO-voted guidelines on when and how AI should be used in different media production contexts. 

- **Quality Assurance Requirements** : Community-defined minimum standards for AIgenerated content before publication. 

- **Disclosure Mandates** : Rules requiring transparent labeling of AI-generated or AIassisted content. 

- **Professional Development Programs** : DAO-funded training initiatives to help media professionals use AI tools responsibly. The multi-tier DAO architecture that operationalizes these norms is specified in Table 7. 

**Table 7.** Multi-Tiered DAO Architecture Specification. 

|**DAO Tier**|**Primary Function**|**Governance Token**<br>**Requirements**|**Decision Authority**|**Behavioral Evidence**|
|---|---|---|---|---|
|**Expert Assessment DAOs**|Technical AI evaluation and<br>skepticism integration|High expertise threshold<br>(PhD, 5+ years’ experience)|Algorithm bias detection,<br>technical standards|Knowledge paradox<br>(β=_−_0.251) shows expert<br>skepticism value|
|**Community Validation DAOs**|Broader stakeholder<br>perspective inclusion|Medium participation<br>threshold (2+ years media<br>experience)|Content quality assessment,<br>ethical guidelines|Information elaboration effects<br>(β= 0.294–0.486)|
|**Professional Standards DAOs**|Industry-specifc rule<br>establishment|Professional certifcation<br>required|Sector-specifc AI use<br>protocols|Confdence-adoption link<br>(β= 0.169)|
|**Cross-Border Coordination DAOs**|International governance<br>harmonization|Representatives from<br>regional DAOs|Global incident response,<br>resource sharing|66.6% knowledge accuracy<br>suggests universal challenges|
|**Audit Enforcement DAOs**|Real-time monitoring and<br>compliance|Staking requirements for<br>audit participation|Violation fagging, penalty<br>implementation|Risk perception patterns<br>(β= 0.231) inform monitoring<br>priorities|



**Smart Contract Implementation** : These norms and rules can be automatically enforced through smart contracts that 

- Block publication of content violating community standards; 

- Automatically reward media professionals who follow established guidelines; 

- Provide real-time feedback to help professionals improve their AI use practices; 

- Create reputation systems that build trust among community members. 

Figure 3 illustrates the knowledge paradox solution: compared with traditional arrangements, the DAO design increases professional confidence, especially at higher knowledge levels, aligning with the expert-weighting and community validation logic detailed in Table 7. Figure 3 provides the knowledge paradox solution by comparing professional confidence levels under traditional centralized governance versus the proposed DAO-based governance system. Three knowledge groups were constructed based on objective AI knowledge test scores: Low (0–7 correct answers out of 15, _n_ = 71), Medium (8–11 correct, _n_ = 68), and High (12–15 correct, _n_ = 70). Mean confidence scores were calculated for each group under two governance scenarios. Under Traditional System governance, confidence declines sharply as knowledge increases: 0.80 for low-knowledge professionals, 0.60 for medium-knowledge professionals, and only 0.30 for high-knowledge professionals. This declining pattern reflects the knowledge paradox identified in our empirical results (β = _−_ 0.251, _p_ < 0.001), where greater AI expertise amplifies skepticism toward centralized governance systems. Under DAO System governance, confidence instead increases with knowledge: 0.70 for low-knowledge professionals, 0.80 for medium-knowledge profession- 

_Future Internet_ **2025** , _17_ , 479 

18 of 31 

als, and 0.90 for high-knowledge professionals. The crossover pattern is particularly striking: low-knowledge professionals show slightly higher confidence in traditional systems (0.80 vs. 0.70), while high-knowledge professionals exhibit dramatically higher confidence in DAO systems (0.90 vs. 0.30). This reversal demonstrates that decentralized blockchainbased governance addresses the transparency deficits and expert-exclusion concerns that generate skepticism among knowledgeable professionals, transforming expert awareness from a barrier to adoption into support for properly designed governance mechanisms. 

**Figure 3.** Knowledge Paradox Solution. 

## _6.4. Blockchain Platform Selection and Technical Architecture_ 

Implementing the multi-tiered DAO architecture specified in Table 7 requires careful selection among available blockchain platforms, each offering distinct trade-offs relevant to AI governance requirements. Literature on blockchain governance systems identifies several platform options with varying characteristics [8]. 

**Ethereum-Based Implementation** : Ethereum remains the dominant platform for DAO deployment, offering mature smart contract infrastructure through Solidity programming language and established governance frameworks including Compound Governor and OpenZeppelin contracts [114]. Ethereum’s advantages for AI governance include 

- Extensive developer ecosystem and audited contract libraries that reduce security risks; 

- Proven track record with major DAOs. including MakerDAO and Uniswap governance; 

- Robust decentralization with over 500,000 validator nodes post-Merge. 

However, Ethereum faces limitations including relatively high gas costs that make frequent governance transactions expensive (average $5–50 per transaction depending on network congestion), and throughput constraints of approximately 15–30 transactions per second that limit scalability for real-time content monitoring. For the multi-token architecture proposed in Table 8, Ethereum’s ERC-20 token standard provides well-established infrastructure, though managing multiple token types introduces complexity in voting weight calculations [115]. 

_Future Internet_ **2025** , _17_ , 479 

19 of 31 

**Table 8.** Token Economics Design Matrix. 

|**Activity Type**|**Token Reward**|**Behavioral Justifcation**|**Governance Function**|**Staking Requirements**|
|---|---|---|---|---|
|**Expert Algorithm Assessment**|EXPERT tokens (weighted<br>voting power)|Knowledge paradox<br>(β=_−_0.251) values expert<br>skepticism|Technical standard setting,<br>bias detection|PhD + 5 years’ experience<br>verifcation|
|**Community Content Validation**|COMMUNITY tokens<br>(broad participation)|Information elaboration<br>effects (β= 0.294–0.486)|Quality assurance, ethical<br>guidelines|2+ years’ media experience|
|**Professional Integrity Tracking**|REPUTATION tokens<br>(credibility scoring)|Confdence-adoption link<br>(β= 0.169) builds legitimate<br>confdence|Accuracy tracking, trust<br>building|Performance history<br>verifcation|
|**Real-Time Audit Participation**|AUDIT tokens (monitoring<br>incentives)|Risk perception patterns<br>(β= 0.231) inform priorities|Violation detection,<br>compliance monitoring|Minimum stake + audit<br>training|
|**Cross-Border Coordination**|GLOBAL tokens<br>(international governance)|66.6% knowledge accuracy<br>shows universal challenges|Resource sharing, incident<br>response|Regional DAO representation|
|**Training Program Contribution**|EDUCATION tokens<br>(knowledge sharing)|Model’s 58% explanatory<br>power shows learning needs|Professional development,<br>skill building|Certifed educator credentials|



**Polkadot Parachain Architecture** : Polkadot offers an alternative approach through its parachain model, which enables specialized blockchain customization while maintaining cross-chain interoperability [116]. For AI governance, Polkadot’s advantages include 

- Substrate framework allowing custom governance logic tailored to media industry requirements; 

- Shared security model where parachains benefit from relay chain validators without maintaining separate validator sets; 

- Cross-chain message passing (XCMP) that facilitates the cross-border coordination DAOs specified in Table 7. 

However, Polkadot introduces trade-offs including parachain slot acquisition costs (recent auctions exceeded $100 million in bonded DOT tokens), governance complexity as the platform itself undergoes rapid evolution through its own referendum system, and limited smart contract maturity compared to Ethereum. The technical complexity that contributed to low participation rates in Polkadot governance (below 15% as noted in Section 2.2) raises concerns about whether media professionals would effectively engage with parachain governance [45]. 

**Layer-2 Scaling Solutions** : Given Ethereum’s gas cost and throughput limitations, Layer-2 solutions including Optimistic Rollups (Optimism, Arbitrum) and Zero-Knowledge Rollups (zkSync, StarkNet) offer promising alternatives that maintain Ethereum’s security guarantees while dramatically reducing costs and increasing throughput [117]. For the real-time audit enforcement DAOs specified in Table 7, Layer-2 platforms provide 

- Transaction costs reduced by 10–100x compared to Ethereum mainnet, making continuous monitoring economically viable; 

- Throughput increases to hundreds or thousands of transactions per second supporting large-scale content verification; 

- Eventual settlement to Ethereum mainnet, providing security for high-value governance decisions [118]. 

However, Layer-2 solutions involve trade-offs including withdrawal delays (typically 7 days for Optimistic Rollups due to fraud proof windows), additional technical complexity in bridging assets between layers, and fragmented liquidity across multiple Layer-2 networks [119]. 

**Blockchain Feature Selection Rationale** : Based on the behavioral findings and governance requirements, we propose a hybrid architecture that leverages different blockchain layers for distinct functions. Core governance decisions including expert assessment DAO votes on algorithm standards and professional standards DAO policy setting utilize Ethereum mainnet, where security and decentralization outweigh gas cost concerns given infrequent but high-stakes decisions [118]. Real-time content monitoring and audit enforce- 

_Future Internet_ **2025** , _17_ , 479 

20 of 31 

ment DAOs operate on Layer-2 solutions where high throughput and low costs enable continuous operation. Cross-border coordination DAOs leverage cross-chain bridges or Polkadot XCMP to facilitate international cooperation while respecting jurisdictional boundaries [120]. This hybrid approach addresses the blockchain trilemma by optimizing different components for security, scalability, or decentralization based on functional requirements rather than seeking a single platform that compromises all three dimensions [121]. 

**Smart Contract Language and Security Considerations** : The choice between Solidity (Ethereum), Rust (Polkadot/Substrate), or other smart contract languages involves trade-offs between developer availability, security tooling, and expressiveness. Given the security-critical nature of AI governance where flawed smart contracts create vulnerabilities demonstrated by The DAO hack (Section 2.2), we prioritize platforms with mature auditing infrastructure. Solidity benefits from extensive security tools including Slither, Mythril, and formal verification frameworks, alongside established audit firms specializing in DAO contracts [122]. However, Solidity’s design enables reentrancy attacks and other vulnerabilities that require careful defensive programming. Rust-based smart contracts (used in Substrate/Polkadot and Solana) offer memory safety guarantees that prevent entire classes of vulnerabilities but involve steeper learning curves and less mature auditing tools [123]. For the proposed AI governance system, we recommend Solidity-based implementation on Ethereum/Layer-2 given current developer ecosystem maturity, with gradual exploration of Rust-based contracts as tooling improves [116]. 

**Consensus Mechanism Implications** : The choice of consensus mechanism affects governance participation patterns and energy efficiency. Ethereum’s transition from Proof-ofWork to Proof-of-Stake reduces energy consumption by 99.95% while maintaining security, addressing sustainability concerns relevant for media organizations with environmental commitments. Proof-of-Stake’s requirement that validators stake capital creates potential plutocratic tendencies (noted in Section 2.2) where wealthy participants disproportionately influence consensus, though this operates at the blockchain layer rather than DAO governance layer where token-weighted voting already introduces similar dynamics [124]. Alternative consensus mechanisms including Delegated Proof-of-Stake (used by EOS and Tron) offer higher throughput but greater centralization with typically 21–101 validators, trading decentralization for performance in ways that may undermine the governance transparency goals motivating blockchain adoption. For AI governance DAOs, the consensus mechanism primarily affects transaction finality speed and costs rather than governance outcomes, making Ethereum’s Proof-of-Stake a reasonable default choice balanced between decentralization and efficiency [120]. 

## _6.5. Implementation Preconditions and Practical Challenges_ 

While the DAO architecture outlined above addresses the behavioral paradoxes identified in our empirical findings, successful implementation requires establishing critical preconditions and confronting practical limitations that blockchain governance inherits or introduces. 

**Identity and Reputation Systems** : Effective DAO governance depends on verifiable identity without compromising privacy. Traditional blockchain pseudonymity enables Sybil attacks where single actors create multiple identities to manipulate voting outcomes. For the Expert Assessment DAOs specified in Table 7, verifying credentials such as PhD qualifications and five years of professional experience requires identity infrastructure that links on-chain addresses to real-world credentials. Zero-knowledge proof systems offer promising solutions, enabling participants to cryptographically prove credential possession without revealing personal information [125]. Our findings on the knowledge paradox suggest that reputation must capture not just participation frequency but also demonstrated 

_Future Internet_ **2025** , _17_ , 479 

21 of 31 

expertise, requiring sophisticated on-chain metrics that reward quality skepticism and accurate risk assessment rather than mere activity levels. 

**Anti-Capture Safeguards** : Token-weighted voting creates plutocratic risks where wealthy participants accumulate disproportionate governance power, potentially replicating the centralized control that DAO governance aims to overcome. Empirical evidence from existing DAOs supports these concerns: MakerDAO governance concentrates among large token holders, with top 10 addresses controlling over 50% of voting power [117], while Polkadot referenda consistently show participation below 15% with outcomes determined by whale addresses [126]. For AI governance where professional standards affect livelihoods, token concentration enables capture by technology companies, investors, or other actors whose interests diverge from working media professionals. Several anti-capture mechanisms warrant consideration. However, each safeguard introduces complexity and potential gaming vectors that require careful mechanism design informed by ongoing empirical study of DAO governance in practice. 

**On-Chain Auditing Costs** : Real-time algorithm monitoring DAOs specified in Section 6.2 require frequent transactions for content flagging, validation voting, and smart contract execution. Layer-2 solutions reduce costs by 10–100x as noted in Section 6.4, but still impose non-trivial expenses that accumulate across large-scale operations. Furthermore, computational costs extend beyond transaction fees to include node operation expenses, data storage costs (particularly for content hashes and audit trails), and oracle costs for importing off-chain information about AI outputs into smart contracts. These economic realities necessitate hybrid architectures where only high-stakes decisions such as algorithm standard changes and professional sanctions utilize expensive on-chain governance, while routine content validation employs off-chain coordination with periodic on-chain checkpoints [127]. 

**Centralization Through Token Accumulation** : Even with anti-capture safeguards, token economics create accumulation dynamics where early participants, successful contributors, and financially privileged actors amass governance power over time. The REPUTATION token system in Table 8 rewards accurate assessments and quality contributions, but professionals who participate full-time accumulate more tokens than those balancing governance with content creation responsibilities, potentially creating governance classes divorced from practical production realities. Addressing these dynamics requires ongoing mechanism adjustment informed by empirical monitoring of token distribution patterns, complemented by non-token-based governance elements such as randomly selected review juries or mandatory participation rotation that ensure governance remains representative despite economic inequalities [117]. 

## _6.6. Blockchain-Based Professional Integrity Protection_ 

Our finding that confidence in AI knowledge positively predicts adoption intentions (β = 0.169, _p_ < 0.001) reveals the importance of building legitimate confidence rather than false confidence that could lead professionals to be misled by AI. DAO governance systems can protect professional integrity through 

**Reputation-Based Verification Systems** : Reputation-Based Professional Integrity Protection establishes blockchain-anchored systems that verify credibility, incentivize accountability, and support continuous learning through the following components: 

- **Professional Credibility Tokens** : Blockchain-based credentials that track media professionals’ accuracy and ethical standards over time. 

- **Peer Review Networks** : DAO members evaluate each other’s AI-assisted work, building collective quality assurance. 

_Future Internet_ **2025** , _17_ , 479 

22 of 31 

- **Accountability Mechanisms** : Smart contracts that automatically track the outcomes of AI-assisted content, rewarding professionals whose work maintains high standards. 

- _•_ **Continuous Learning Protocols** : DAO-organized training programs that help professionals stay current with AI capabilities and limitations. 

**Global Professional Standards** : These protection mechanisms can be implemented across different media markets while respecting local professional norms and regulatory requirements. The supporting token-economics design is summarized in Table 8. 

Figure 4 represents the activity-to-token reward matrix (Table 8) as an intensity heatmap showing which tokens most strongly incentivize different governance activities. Six governance activity types (Algorithm Assessment, Content Validation, Integrity Tracking, Audit Participation, Cross-Border Coordination, Training Contribution) form the rows, while six token types (EXPERT, COMMUNITY, REPUTATION, AUDIT, GLOBAL, EDUCATION) constitute the columns. Cell colors indicate reward intensity on a scale from 10 (minimal reward) to 90 (maximum reward). Intensity values reflect behavioral justifications from our empirical findings and functional alignment between activities and tokens. Algorithm Assessment activities receive maximum rewards (90) for EXPERT and AUDIT tokens, as the knowledge paradox (β = _−_ 0.251) demonstrates that expert technical knowledge is essential for identifying algorithmic bias. Content Validation shows high rewards (80–85) for COMMUNITY and AUDIT tokens, reflecting information elaboration effects (β = 0.294–0.486) where broad stakeholder engagement improves quality assessment. Integrity Tracking receives maximum REPUTATION token rewards (90), since the confidence-adoption link (β = 0.169) indicates that tracking professional credibility builds legitimate confidence. Cross-Border Coordination shows peak rewards (85) for GLOBAL tokens, aligning with the finding that 66.6% knowledge accuracy suggests universal challenges requiring international cooperation. Training Contribution reaches maximum intensity (90) for EDUCATION tokens, as the model’s 58% explanatory power reveals substantial learning needs. The heatmap pattern reveals functional specialization: bright regions along diagonal-like positions show where token types align with their primary intended activities, while moderate intensities (50–60) in off-diagonal positions indicate secondary incentives that encourage cross-functional participation without diluting specialized expertise. This intensity distribution ensures that participants receive the strongest rewards when contributing to activities matching their demonstrated capabilities, while maintaining sufficient incentives for broader engagement that prevents governance silos. 

**Figure 4.** Activity-token Reward Matrix. 

_Future Internet_ **2025** , _17_ , 479 

23 of 31 

## _6.7. Universal Implementation Framework for Global Media Industries_ 

Our knowledge test showing 66.6% accuracy among Chinese media professionals suggests similar challenges likely exist globally. The knowledge paradox we identify transcends cultural boundaries, making our DAO governance solutions applicable worldwide and across adjacent creative industries facing parallel AI governance challenges. 

**Generalization Beyond Media to Creative Industries** : The behavioral mechanisms and governance design principles derived from our media professional study extend naturally to adjacent creative sectors confronting generative AI disruption. Music production faces AI composition tools like Suno and Udio that generate complete tracks from text prompts, raising authenticity and attribution challenges analogous to video generation. Visual arts and illustration communities confront Midjourney, DALL-E, and Stable Diffusion systems that generate images competing with human artists while training on copyrighted works without consent [128]. Game development encounters AI systems generating code, 3D assets, and narrative content that transform production pipelines [129]. Across these domains, professionals exhibit similar behavioral patterns: expertise creates awareness of AI limitations and governance gaps that reduce adoption confidence under centralized control, while benefit perceptions drive adoption when appropriate oversight exists. The multi-token DAO architecture in Table 8 generalizes directly: EXPERT tokens for domain specialists (music producers, illustrators, game designers), COMMUNITY tokens for broader practitioner participation, REPUTATION tokens tracking contribution quality, AUDIT tokens incentivizing real-time output monitoring, GLOBAL tokens coordinating cross-border standards, and EDUCATION tokens rewarding skill development. Industry-specific adaptations require only adjusting threshold criteria and audit priorities while maintaining core governance structure, enabling federated implementation where music DAOs, visual arts DAOs, and game development DAOs operate independently while sharing anti-capture mechanisms and identity infrastructure through cross-chain interoperability protocols. 

**Post-Pandemic Resilience into Scalable Protocols** : The DAO governance framework we propose institutionalizes organizational adaptations that emerged during COVID-19 disruptions, transforming temporary crisis responses into permanent governance infrastructure. Recent empirical studies [2,5,7] document resilience mechanisms that parallel our DAO design. By encoding these crisis-tested resilience behaviors into immutable smart contracts and token economics, blockchain governance transforms adaptive responses that organizations adopted under pandemic pressure into durable institutional features that persist beyond crisis conditions. This addresses the post-pandemic governance challenge identified in our introduction: whereas crisis-driven ICT adoption bypassed systematic oversight in favor of operational continuity, the current phase demands institutionalizing effective practices into frameworks that balance innovation with professional protection. 

**Cross-Border DAO Networks** : Cross-Border DAO Networks enable coordinated international governance, local adaptation, shared resources, and joint incident response across jurisdictions through the following components: 

- **International Coordination Protocols** : Blockchain systems that enable cooperation between DAOs in different countries while respecting local regulations, leveraging advanced cross-blockchain data migration techniques to ensure seamless information sharing [130]. 

- **Cultural Adaptation Mechanisms** : Smart contracts that automatically adjust governance rules based on regional contexts while maintaining core professional standards. 

- _•_ **Resource Sharing Systems** : DAOs can share best practices, training materials, and auditing tools across international networks using content-centric software-defined approaches that optimize information distribution [131]. 

_Future Internet_ **2025** , _17_ , 479 

24 of 31 

- **Global Incident Response** : Coordinated DAO response to AI systems that mislead professionals across multiple jurisdictions. 

**Implementation Roadmap** : The Implementation Roadmap sequences the rollout from pilot DAOs to interoperable, globally scaled, and regulator-aligned governance as follows: 

- **Phase 1** : Establish pilot DAOs in major media markets (North America, Europe, Asia-Pacific) and adjacent creative industries (music, visual arts, game development). 

- _•_ **Phase 2** : Develop interoperability protocols between regional and sector-specific DAOs, sharing identity infrastructure and anti-capture mechanisms. 

- **Phase 3** : Scale governance systems to cover global creative economies while federating authority to respect jurisdictional and cultural differences. 

- **Phase 4** : Integrate existing regulatory frameworks while maintaining decentralized oversight capabilities, clarifying complementary roles between blockchain governance and statutory enforcement. 

**Necessary Role of Statutory Regulation** : While blockchain-based DAO governance addresses transparency, participation, and accountability challenges that centralized systems create, decentralized mechanisms cannot replace statutory regulation entirely. Legal enforcement of criminal violations such as fraud, defamation, and copyright infringement requires state authority with coercive power that smart contracts lack [132]. Cross-border disputes involving conflicting national laws exceed DAO jurisdiction, necessitating international legal frameworks and treaties. Consumer protection regulations including data privacy requirements (GDPR, CCPA) and accessibility standards demand governmental oversight with audit authority and penalty mechanisms beyond blockchain capabilities [133]. Systemic risks where AI-generated misinformation threatens public health, election integrity, or national security warrant centralized intervention when decentralized coordination proves insufficient. The optimal governance architecture therefore combines decentralized DAO oversight for algorithmic transparency, professional standards, and community coordination with statutory regulation for legal enforcement, consumer protection, and crisis response. This hybrid model positions DAOs as a complement to rather than a replacement for traditional regulation, addressing the governance gaps and participation deficits that centralized approaches create while preserving state capacity for functions requiring legal authority. Future research should examine integration mechanisms that enable information flow between DAO governance systems and regulatory agencies, specify escalation protocols for transferring issues from blockchain to legal jurisdiction when appropriate, and assess how decentralized oversight can inform statutory rulemaking by surfacing practitioner knowledge and community norms. 

## **7. Theoretical Contributions and Future Research Directions** 

This section articulates the work’s conceptual advances and outlines next steps. We contribute evidence for a knowledge paradox in AI adoption, link it to information processing within the O-S-O-R model, and translate these insights into a blockchain governance blueprint with implementable smart-contract patterns. We then identify research priorities to validate generalizability, refine mechanisms, and assess integration with regulatory regimes. 

## _7.1. Contributions to Web3 and Behavioral Literature_ 

This study makes several significant contributions to the intersection of behavioral research and decentralized governance: 

_Future Internet_ **2025** , _17_ , 479 

25 of 31 

- **Novel Application of O-S-O-R to Blockchain Governance** : We demonstrate how traditional behavioral models can inform the design of decentralized systems, providing an empirical foundation for DAO architecture decisions. 

- **Knowledge Paradox Discovery** : The identification of negative moderation effects provides crucial insights for designing inclusive governance mechanisms that properly integrate expert knowledge while addressing participation barriers. 

- **Empirical Foundation for Token Economics** : Our findings on the relationships between confidence, benefit perception, and behavioral intentions provide empirical guidance for designing sustainable token incentive systems. 

## _7.2. Implications for Smart Contract Design_ 

Our results inform specific design principles for blockchain-based AI governance: 

- **Multi-stakeholder Validation Requirements** : Smart contracts should mandate assessment from both expert and community perspectives before implementing AIrelated decisions. 

- **Adaptive Incentive Mechanisms** : Governance tokens should be distributed based on comprehensive models that account for the knowledge paradox and other behavioral patterns revealed in our study. 

- **Temporal Governance Protocols** : Smart contracts should implement time-delayed decision mechanisms that allow for thorough information processing and expert review before final implementation. 

Together, these contract patterns enable transparent, auditable, and incentivecompatible AI governance deployable across heterogeneous media contexts. 

## _7.3. Future Research Priorities_ 

Future research should validate and scale this framework through four complementary directions: 

- **Longitudinal Analysis** : Future studies should examine how the knowledge paradox evolves over time as AI technologies mature and information environments change. This research is crucial for designing adaptive governance mechanisms. 

- **Cross-Cultural Validation** : Our findings from Chinese media professionals should be validated across different cultural contexts to ensure the generalizability of proposed blockchain governance frameworks. 

- **Implementation Studies** : Empirical research is needed on actual blockchain-based governance systems to validate the theoretical frameworks proposed based on our behavioral findings. 

- **Regulatory Integration Research** : Future work should examine how decentralized AI governance mechanisms can complement and integrate with traditional regulatory approaches across different jurisdictions. 

Therefore, if AI technologies are ultimately to benefit everyone, then cultivating public interest in science and technology, implementing transparent decentralized governance mechanisms, and openly acknowledging AI-related risks through community-driven processes are all essential for encouraging broader, more sustainable adoption of AI applications. The behavioral patterns revealed in this study provide empirical foundations for designing such systems, moving beyond centralized models toward community-governed approaches that address the complex psychological and social dynamics inherent in AI adoption decisions. 

_Future Internet_ **2025** , _17_ , 479 

26 of 31 

## **8. Conclusions** 

This study examines how to prevent AI-driven misguidance of media professionals while ensuring equitable access to AI benefits. Using the O-S-O-R framework and data from 209 Chinese media professionals, we identify a knowledge paradox where higher AI expertise amplifies skepticism toward adoption under centralized governance, alongside strong information elaboration effects through confidence, benefit perception, and risk perception pathways. Building on these behavioral insights, we propose a blockchain-based governance framework with five integrated mechanisms: Expert Assessment DAOs, Community Validation DAOs, real-time algorithm monitoring, professional integrity protection, and cross-border coordination. These mechanisms address transparency deficits and participation barriers that generate skepticism among knowledgeable professionals, transforming expertise from a barrier to adoption into support for properly designed oversight. 

This study has important limitations that future research should address. The crosssectional design limits causal inference about how behavioral dynamics evolve over time. The single-country sample in China constrains generalizability given distinct regulatory environments and platform ecosystems. Construct operationalization choices require validation across varied settings. Future research should pursue four complementary directions. First, longitudinal tracking before and after key AI policy or model releases can examine how the knowledge paradox and information elaboration patterns change as technologies mature. Second, multi-country replications with varied platform regimes can assess whether findings generalize across institutional contexts. Third, field pilots of DAO modules with pre-registered outcomes including audit accuracy rates, time-to-correction metrics, and participation sustainability can validate the theoretical governance architecture through real-world implementation. Fourth, economic analyses comparing token incentive efficacy versus traditional quality assurance approaches can assess cost-effectiveness and long-run viability. This research agenda addresses whether decentralized governance successfully institutionalizes adaptive coordination and transparency practices or whether organizations revert to centralized patterns once adoption pressures stabilize. 

**Author Contributions:** Conceptualization, J.W. and Y.C.; methodology, J.W.; software, J.W.; validation, J.W. and Y.C.; formal analysis, J.W.; investigation, Y.C..; resources, Y.C.; data curation, J.W.; writing— original draft preparation, J.W.; writing—review and editing, Y.C.; visualization, J.W.; supervision, Y.C.; project administration, Y.C.; funding acquisition, Y.C. All authors have read and agreed to the published version of the manuscript. 

**Funding:** This work was supported by 2025 Fujian Social Sciences Fund Project No. FJ2025C124. 

**Data Availability Statement:** Data will be made available upon request. 

**Conflicts of Interest:** Authors declare no conflicts of interest. 

## **References** 

1. Kumar, V.; Verma, P.; Mittal, A.; Panduro, J.A.T.; Singh, S.; Paliwal, M.; Sharma, N.K. Adoption of ICTs as an emergent business strategy during and following COVID-19 crisis: Evidence from Indian MSMEs. _Benchmarking Int. J._ **2022** , _30_ , 1850–1883. [CrossRef] 

2. Qrunfleh, S.; Vivek, S.; Merz, R.; Mathivathanan, D. Mitigation themes in supply chain research during the COVID-19 pandemic: A systematic literature review. _Benchmarking Int. J._ **2022** , _30_ , 1832–1849. [CrossRef] 

3. Gupta, A.; Kumar Singh, R. Managing resilience of micro, small and medium enterprises (MSMEs) during COVID-19: Analysis of barriers. _Benchmarking Int. J._ **2022** , _30_ , 2062–2084. [CrossRef] 

4. Santos, S.C.; Liguori, E.W.; Garvey, E. How digitalization reinvented entrepreneurial resilience during COVID-19. _Technol. Forecast. Soc. Change_ **2023** , _189_ , 122398. [CrossRef] 

5. Sreenivasan, A.; Suresh, M. Readiness of financial resilience in start-ups. _J. Saf. Sci. Resil._ **2023** , _4_ , 241–252. [CrossRef] 

6. Fares, N.; Lloret, J.; Kumar, V.; Frederico, G.F.; Kumar, A.; Garza-Reyes, J.A. Enablers of post-COVID-19 customer demand resilience: Evidence for fast-fashion MSMEs. _Benchmarking Int. J._ **2023** , _30_ , 2012–2039. [CrossRef] 

_Future Internet_ **2025** , _17_ , 479 

27 of 31 

7. Varma, D.; Dutta, P. Restarting MSMEs and start-ups post COVID-19: A grounded theory approach to identify success factors to tackle changed business landscape. _Benchmarking Int. J._ **2022** , _30_ , 1912–1941. [CrossRef] 

8. Liu, Y.; Lu, Q.; Zhu, L.; Paik, H.-Y.; Staples, M. A systematic literature review on blockchain governance. _J. Syst. Softw._ **2022** , _197_ , 111576. [CrossRef] 

9. Zirar, A.; Ali, S.I.; Islam, N. Worker and workplace Artificial Intelligence (AI) coexistence: Emerging themes and research agenda. _Technovation_ **2023** , _124_ , 102747. [CrossRef] 

10. State Council of the PRC. China Records Over 1 Billion Online Audiovisual Users. Available online: https://english.www.gov. cn/archive/statistics/202403/28/content_WS6604c172c6d0868f4e8e58ce.html (accessed on 28 March 2024). 

11. ChinaNews.com. The Number of Professional Online Livestreamers in China Reaches 15.08 Million. Available online: https: //www.chinanews.com.cn/cj/2024/03-27/10188073.shtml (accessed on 27 March 2024). 

12. Oliver, D. Americans Are Concerned About Rise of AI and Human Enhancements, Survey Finds. Available online: https: //edition.cnn.com/2022/03/17/health/ai-human-enhancements/index.html (accessed on 17 March 2022). 

13. Ipsos. Revolution@Work: Fears and Expectations. Available online: https://www.ipsos.com/en/revolutionwork-fears-andexpectations (accessed on 21 November 2017). 

14. Dafoe, A.; Future, A. Artificial Intelligence: American Attitudes and Trends. Governance AI. 2019. Available online: https: //governanceai.github.io/US-Public-Opinion-Report-Jan-2019/ (accessed on 15 October 2025). 

15. Grohmann, R.; Rocha, A.C.; Guilherme, G. Worker-led AI governance: Hollywood writers’ strikes and the worker power. _Inf. Commun. Soc._ **2025** , 1–9. [CrossRef] 

16. Jarrahi, M.H. Artificial Intelligence and the Future of work: Human-AI Symbiosis in Organizational Decision Making. _Bus. Horizons_ **2018** , _61_ , 577–586. [CrossRef] 

17. Bradford, A. _The Brussels Effect: How the European Union Rules the World_ ; Oxford University Press: Oxford, UK, 2020. 18. Smuha, N.A. From a ‘race to AI’to a ‘race to AI regulation’: Regulatory competition for artificial intelligence. _Law Innov. Technol._ **2021** , _13_ , 57–84. [CrossRef] 

19. Yeung, K. Algorithmic regulation: A critical interrogation. _Regul. Gov._ **2018** , _12_ , 505–523. [CrossRef] 20. Guihot, M.; Matthew, A.F.; Suzor, N.P. Nudging robots: Innovative solutions to regulate artificial intelligence. _Vanderbilt J. Entertain. Technol. Law_ **2017** , _20_ , 385. 

21. Calo, R. Artificial intelligence policy: A primer and roadmap. _UCDL Rev._ **2017** , _51_ , 399. 

22. Metcalf, J.; Moss, E.; Boyd, D. Owning ethics: Corporate logics, silicon valley, and the institutionalization of ethics. _Soc. Res. Int. Q._ **2019** , _86_ , 449–476. [CrossRef] 

23. Helberger, N. The political power of platforms: How current attempts to regulate misinformation amplify opinion power. _Digit. Journal._ **2020** , _8_ , 842–854. [CrossRef] 

24. Hagendorff, T. The ethics of AI ethics: An evaluation of guidelines. _Minds Mach._ **2020** , _30_ , 99–120. [CrossRef] 

25. Hassan, S.; De Filippi, P. Decentralized autonomous organization. _InterGuonet Policy Rev._ **2021** , _10_ , 1–10. [CrossRef] 26. Wright, A.; De Filippi, P. Decentralized blockchain technology and the rise of lex cryptographia. _SSRN Electron. J._ **2015** . [CrossRef] 27. De Filippi, P.; Hassan, S. Blockchain technology as a regulatory technology: From code is law to law is code. _arXiv_ **2018** , arXiv:1801.02507. [CrossRef] 

28. Buterin, V. A next-generation smart contract and decentralized application platform. _Ethereum Whitepaper_ **2014** , _3_ , 1–36. 29. Szabo, N. Formalizing and securing relationships on public networks. _First Monday_ **1997** , _2_ . [CrossRef] 30. Werbach, K. Trust, but verify: Why the blockchain needs the law. _Berkeley Technol. Law J._ **2018** , _33_ , 487–550. 31. Beck, R.; Müller-Bloch, C.; King, J.L. Governance in the blockchain economy: A framework and research agenda. _J. Assoc. Inf. Syst._ **2018** , _19_ , 1020–1034. [CrossRef] 

32. Chen, Y.; Bellavitis, C. Blockchain disruption and decentralized finance: The rise of decentralized business models. _J. Bus. Ventur. Insights_ **2020** , _13_ , e00151. [CrossRef] 

33. Atzori, M. Blockchain technology and decentralized governance: Is the state still necessary? _SSRN Electron. J._ **2015** . [CrossRef] 34. Hsieh, Y.-Y.; Vergne, J.-P.; Anderson, P.; Lakhani, K.; Reitzig, M. Bitcoin and the rise of decentralized autonomous organizations. _J. Organ. Des._ **2018** , _7_ , 14. [CrossRef] 

35. Markus, H.; Zajonc, R.B. The cognitive perspective in social psychology. In _The Handbook of Social Psychology_ ; Random House: New York, NY, USA, 1985; pp. 137–229. 

36. Jiang, S. The Roles of Worry, Social Media Information Overload, and Social Media Fatigue in Hindering Health Fact-Checking. _Soc. Media+ Soc._ **2022** , _8_ , 205630512211130. [CrossRef] 

37. Huber, B.; Barnidge, M.; Gil de Zúñiga, H.; Liu, J. Fostering public trust in science: The role of social media. _Public Underst. Sci._ **2019** , _28_ , 759–777. [CrossRef] [PubMed] 

38. Yeo, S.K.; Su, L.Y.-F.; Cacciatore, M.A.; McKasy, M.; Qian, S. Predicting Intentions to Engage With Scientific Messages on Twitter: The Roles of Mirth and Need for Humor. _Sci. Commun._ **2020** , _42_ , 481–507. [CrossRef] 

_Future Internet_ **2025** , _17_ , 479 

28 of 31 

39. Weber Shandwick. AI-Ready or Not: Artificial Intelligence Here We Come! Weber Shandwick. 2019. Available online: https://www.webershandwick.com/news/ai-ready-or-not-artificial-intelligence-here-we-come/ (accessed on 15 October 2025). 

40. Venkatesh, V. Adoption and use of AI tools: A research agenda grounded in UTAUT. _Ann. Oper. Res._ **2021** , _308_ , 641–652. [CrossRef] 

41. Yoo, J.H. No Clear Winner: Effects of The Biggest Loser on the Stigmatization of Obese Persons. _Health Commun._ **2012** , _28_ , 294–303. [CrossRef] 

42. Atzei, N.; Bartoletti, M.; Cimoli, T. A survey of attacks on ethereum smart contracts (sok). In Proceedings of the International Conference on Principles of Security and Trust, Uppsala, Sweden, 22–29 April 2017; Springer: Berlin/Heidelberg, Germany, 2017; pp. 164–186. 

43. Voskobojnikov, A.; Wiese, O.; Mehrabi Koushki, M.; Roth, V.; Beznosov, K. The u in crypto stands for usable: An empirical study of user experience with mobile cryptocurrency wallets. In Proceedings of the 2021 CHI Conference on Human Factors in Computing Systems, Online Virtual Conference, 8–13 May 2021; pp. 1–14. 

44. Karim, M.; Khan, S.; Van, D.H.; Liu, X.; Wang, C.; Qu, Q. Transforming Data Annotation with AI Agents: A Review of Architectures, Reasoning, Applications, and Impact. _Future Internet_ **2025** , _17_ , 353. [CrossRef] 

45. Boehmer, N.; Brill, M.; Cevallos, A.; Gehrlein, J.; Sánchez-Fernández, L.; Schmidt-Kraepelin, U. Approval-based committee voting in practice: A case study of (over-) representation in the Polkadot blockchain. In Proceedings of the AAAI Conference on Artificial Intelligence, Philadelphia, PA, USA, 25 February–4 March 2025; Volume 38, pp. 9519–9527. 

46. Sockin, M.; Xiong, W. A model of cryptocurrencies. _Manag. Sci._ **2023** , _69_ , 6684–6707. [CrossRef] 47. Fritsch, R.; Müller, M.; Wattenhofer, R. Analyzing voting power in decentralized governance: Who controls DAOs? _Blockchain Res. Appl._ **2024** , _5_ , 100208. [CrossRef] 

48. Nasrulin, B.; Muzammal, M.; Qu, Q. Chainmob: Mobility analytics on blockchain. In Proceedings of the 2018 19th IEEE International Conference on Mobile Data Management (MDM), Aalborg, Denmark, 26–28 June 2018; IEEE: New York, NY, USA, 2018; pp. 292–293. 

49. Breckenridge, S.; Vilardell, D.; Fábrega, A.; Zhao, A.; McCorry, P.; Solari, R.; Juels, A. B-Privacy: Defining and Enforcing Privacy in Weighted Voting. _arXiv_ **2025** , arXiv:2509.17871. [CrossRef] 

50. Stasavage, D. Open-door or closed-door? Transparency in domestic and international bargaining. _Int. Organ._ **2004** , _58_ , 667–703. [CrossRef] 

51. Dimitri, N. The Economic Value of Dual-Token Blockchains. _Mathematics_ **2023** , _11_ , 3757. [CrossRef] 52. Harris, C.G. Cross-chain technologies: Challenges and opportunities for blockchain interoperability. In Proceedings of the 2023 IEEE International Conference on Omni-Layer Intelligent Systems (COINS), Berlin, Germany, 23–25 July 2023; IEEE: New York, NY, USA, 2023; pp. 1–6. 

53. Zhang, P.; Schmidt, D.C. Decentralizing the cloud: A survey of decentralized applications and their blockchain implementations. _IEEE Access_ **2019** , _7_ , 114813–114834. 

54. Karim, M.; Van, D.H.; Khan, S.; Qu, Q.; Kholodov, Y. Ai agents meet blockchain: A survey on secure and scalable collaboration for multi-agents. _Future Internet_ **2025** , _17_ , 57. [CrossRef] 

55. Eskandari, S.; Salehi, M.; Gu, W.C.; Clark, J. Sok: Oracles from the ground truth to market manipulation. In Proceedings of the 3rd ACM Conference on Advances in Financial Technologies, Cambridge, MA, USA, 19–21 September 2022; pp. 127–141. 

56. Li, C. Gas estimation and optimization for smart contracts on ethereum. In Proceedings of the 2021 36th IEEE/ACM International Conference on Automated Software Engineering (ASE), Melbourne, Australia, 15–19 November 2021; IEEE: New York, NY, USA, 2021; pp. 1082–1086. 

57. Chandra, B.; Dunietz, J.; Roberts, K. Reducing Risks Posed by Synthetic Content an Overview of Technical Approaches to Digital Content Transparency. 2024. Available online: https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=959123 (accessed on 14 October 2025). 

58. Feng, K.J.K.; Ritchie, N.; Blumenthal, P.; Parsons, A.; Zhang, A.X. Examining the impact of provenance-enabled media on trust and accuracy perceptions. _Proc. ACM Hum.-Comput. Interact._ **2023** , _7_ , 1–42. [CrossRef] 

59. Kiayias, A.; Lazos, P. SoK: Blockchain governance. In Proceedings of the 4th ACM Conference on Advances in Financial Technologies, Cambridge, MA, USA, 19–21 September 2022; pp. 61–73. 

60. Thomas, R.L.; Uminsky, D. Reliance on metrics is a fundamental challenge for AI. _Patterns_ **2022** , _3_ , 100476. [CrossRef] 61. Martin, W.M.; Avdul, D.; Lopez, Y. Cryptocurrency compensation: Look before you leap. _Compens. Benefits Rev._ **2023** , _55_ , 68–75. [CrossRef] 

62. Eveland, J.R.; William, P. The Effect of Political Discussion in Producing Informed Citizens: The Roles of Information, Motivation, and Elaboration. _Political Commun._ **2004** , _21_ , 177–193. [CrossRef] 

63. Rae, C.L.; Ahmad, A.; Larsson, D.E.O.; Silva, M.; van Praag, C.D.G.; Garfinkel, S.N.; Critchley, H.D. Impact of cardiac interoception cues and confidence on voluntary decisions to make or withhold action in an intentional inhibition task. _Sci. Rep._ **2020** , _10_ , 4184. [CrossRef] 

_Future Internet_ **2025** , _17_ , 479 

29 of 31 

64. Sawaya, Y.; Sharif, M.; Christin, N.; Kubota, A.; Nakarai, A.; Yamada, A. Self-Confidence Trumps Knowledge. In Proceedings of the 2017 CHI Conference on Human Factors in Computing Systems, Denver, CO, USA, 6–11 May 2017. [CrossRef] 

65. Cassam, Q. Diagnostic error, overconfidence and self-knowledge. _Palgrave Commun._ **2017** , _3_ , 17025. [CrossRef] 66. Malmendier, U.; Tate, G. Behavioral CEOs: The Role of Managerial Overconfidence. _J. Econ. Perspect._ **2015** , _29_ , 37–60. [CrossRef] 67. Mannes, A.E.; Moore, D.A. A Behavioral Demonstration of Overconfidence in Judgment. _Psychol. Sci._ **2013** , _24_ , 1190–1197. [CrossRef] 

68. McKenzie, C.R.; Liersch, M.J.; Yaniv, I. Overconfidence in interval estimates: What does expertise buy you? _Organ. Behav. Hum. Decis. Process._ **2008** , _107_ , 179–191. [CrossRef] 

69. Naguib, M.; Brull, S.J.; Hunter, J.M.; Kopman, A.F.; Fülesdi, B.; Johnson, K.B.; Arkes, H.R. Anesthesiologists’ Overconfidence in Their Perceived Knowledge of Neuromuscular Monitoring and Its Relevance to All Aspects of Medical Practice. _Anesthesia Analg._ **2019** , _128_ , 1118–1126. [CrossRef] 

70. Pikulina, E.; Renneboog, L.; Tobler, P.N. Overconfidence and investment: An experimental approach. _J. Corp. Financ._ **2017** , _43_ , 175–192. [CrossRef] 

71. Yang, H.; Thompson, C. Nurses’ risk assessment judgements: A confidence calibration study. _J. Adv. Nurs._ **2010** , _66_ , 2751–2760. [CrossRef] [PubMed] 

72. Yang, H.; Thompson, C.; Bland, M. The effect of clinical experience, judgment task difficulty and time pressure on nurses’ confidence calibration in a high fidelity clinical simulation. _BMC Med. Inform. Decis. Mak._ **2012** , _12_ , 113. [CrossRef] 

73. Sütterlin, S.; Lugo, R.G.; Ask, T.F.; Veng, K.; Eck, J.; Fritschi, J.; Özmen, M.T.; Bärreiter, B.; Knox, B.J. The role of IT background for metacognitive accuracy, confidence and overestimation of deep fake recognition skills. In _Proceedings of the International Conference on Human-Computer Interaction_ ; Springer International Publishing: Cham, Switzerland, 2022; pp. 103–119. 

74. Wildavsky, A.; Dake, K. _Theories of Risk Perception: Who Fears What and Why?_ Routledge: Abingdon, UK, 1990; pp. 41–60. 75. Midden, C.J.H.; Huijts, N.M.A. The Role of Trust in the Affective Evaluation of Novel Risks: The Case of CO2Storage. _Risk Anal._ **2009** , _29_ , 743–751. [CrossRef] [PubMed] 

76. Siegrist, M.; Keller, C.; Kastenholz, H.; Frey, S.; Wiek, A. Laypeople’s and Experts’ Perception of Nanotechnology Hazards. _Risk Anal._ **2007** , _27_ , 59–69. [CrossRef] 

77. Wallquist, L.; Visschers, V.H.M.; Siegrist, M. Impact of Knowledge and Misconceptions on Benefit and Risk Perception of CCS. _Environ. Sci. Technol._ **2010** , _44_ , 6557–6562. [CrossRef] 

78. Alhakami, A.S.; Slovic, P. A Psychological Study of the Inverse Relationship Between Perceived Risk and Perceived Benefit. _Risk Anal._ **1994** , _14_ , 1085–1096. [CrossRef] 

79. Frewer, L.J.; Howard, C.; Shepherd, R. Understanding public attitudes to technology. _J. Risk Res._ **1998** , _1_ , 221–235. [CrossRef] 80. Bearth, A.; Siegrist, M. Are risk or benefit perceptions more important for public acceptance of innovative food technologies: A meta-analysis. _Trends Food Sci. Technol._ **2016** , _49_ , 14–23. [CrossRef] 

81. Ward, C.; Raue, M.; Lee, C.; D’aMbiosio, L.; Coughlin, J.F. Acceptance of Automated Driving Across Generations: The Role of Risk and Benefit Perception, Knowledge, and Trust. In _Human-Computer Interaction. User Interface Design, Development and Multimodality_ ; Springer International Publishing: Cham, Switzerland, 2017; pp. 254–266. [CrossRef] 

82. Siegrist, M. The Influence of Trust and Perceptions of Risks and Benefits on the Acceptance of Gene Technology. _Risk Anal._ **2000** , _20_ , 195–204. [CrossRef] 

83. Kelley, P.G.; Yang, Y.; Heldreth, C.; Moessner, C.; Sedley, A.; Kramm, A.; Newman, D.T.; Woodruff, A. Exciting, Useful, Worrying, Futuristic: Public Perception of Artificial Intelligence in 8 Countries. In Proceedings of the 2021 AAAI/ACM Conference on AI, Ethics, and Society, Virtual Event, 19–21 May 2021. [CrossRef] 

84. Bao, L.; Krause, N.M.; Calice, M.N.; Scheufele, D.A.; Wirz, C.D.; Brossard, D.; Newman, T.P.; Xenos, M.A. Whose AI? How different publics think about AI and its social impacts. _Comput. Hum. Behav._ **2022** , _130_ , 107182. [CrossRef] 

85. Rathi, D.S.; Rathi, S.K. Knowledge on Artificial Intelligence and Related Fields Among Engineering Students. _Int. J. Eng. Sci._ **2019** , _8_ , 3–10. 

86. Selwyn, N.; Cordoba, B.G. Australian public understandings of artificial intelligence. _AI Soc._ **2021** , _37_ , 1645–1662. [CrossRef] 87. Selwyn, N.; Gallo Cordoba, B.; Andrejevic, M.; Campbell, L. _AI for Social Good: Australian Attitudes Toward AI and Society_ ; Monash University: Melbourne, Australia, 2020. [CrossRef] 

88. Evans, G.; Durant, J. The relationship between knowledge and attitudes in the public understanding of science in Britain. _Public Underst. Sci._ **1995** , _4_ , 57–74. [CrossRef] 

89. Ahmed, Z.; Bhinder, K.K.; Tariq, A.; Tahir, M.J.; Mehmood, Q.; Tabassum, M.S.; Malik, M.; Aslam, S.; Asghar, M.S.; Yousaf, Z. Knowledge, attitude, and practice of artificial intelligence among doctors and medical students in Pakistan: A cross-sectional online survey. _Ann. Med. Surg._ **2022** , _76_ , 103493. [CrossRef] 

90. Chawla, D.; Joshi, H. The Moderating Effect of Demographic Variables on Mobile Banking Adoption: An Empirical Investigation. _Glob. Bus. Rev._ **2018** , _19_ (Suppl. S3), S90–S113. [CrossRef] 

_Future Internet_ **2025** , _17_ , 479 

30 of 31 

91. Hu, S.; Laxman, K.; Lee, K. Exploring factors affecting academics’ adoption of emerging mobile technologies—an extended UTAUT perspective. _Educ. Inf. Technol._ **2020** , _25_ , 4615–4635. [CrossRef] 

92. Liu, H.; Yang, R.; Wang, L.; Liu, P. Evaluating Initial Public Acceptance of Highly and Fully Autonomous Vehicles. _Int. J. Hum.-Comput. Interact._ **2019** , _35_ , 919–931. [CrossRef] 

93. Hudson, J.; Orviska, M.; Hunady, J. People’s attitudes to autonomous vehicles. _Transp. Res. Part A Policy Pract._ **2019** , _121_ , 164–176. [CrossRef] 

94. Chang, C.-Y.; Lai, C.-L.; Hwang, G.-J. Trends and research issues of mobile learning studies in nursing education: A review of academic publications from 1971 to 2016. _Comput. Educ._ **2018** , _116_ , 28–48. [CrossRef] 

95. Gerosa, T.; Gui, M.; Hargittai, E.; Nguyen, M.H. (Mis)informed during COVID-19: How education level and information sources contribute to knowledge gaps. _Int. J. Commun._ **2021** , _15_ , 2196–2217. [CrossRef] 

96. Park, J.; Hong, E.; Le, H.T. Adopting autonomous vehicles: The moderating effects of demographic variables. _J. Retail. Consum. Serv._ **2021** , _63_ , 102687. [CrossRef] 

97. Sun, S.; Lee, P.C.; Law, R.; Hyun, S.S. An investigation of the moderating effects of current job position level and hotel work experience between technology readiness and technology acceptance. _Int. J. Hosp. Manag._ **2020** , _90_ , 102633. [CrossRef] 

98. Retzbach, J.; Retzbach, A.; Maier, M.; Otto, L.; Rahnke, M. Effects of Repeated Exposure to Science TV Shows on Beliefs About Scientific Evidence and Interest in Science. _J. Media Psychol._ **2013** , _25_ , 3–13. [CrossRef] 

99. Li, W.; Xu, S.; Zheng, X.; Sun, R. Bridging the Knowledge Gap in Artificial Intelligence: The Roles of Social Media Exposure and Information Elaboration. _Sci. Commun._ **2024** , _46_ , 399–430. [CrossRef] 

100. Said, N.; Potinteu, A.E.; Brich, I.; Buder, J.; Schumm, H.; Huff, M. An artificial intelligence perspective: How knowledge and confidence shape risk and benefit perception. _Comput. Hum. Behav._ **2023** , _149_ , 107855. [CrossRef] 

101. Andrews, J.E.; Ward, H.; Yoon, J. UTAUT as a Model for Understanding Intention to Adopt AI and Related Technologies among Librarians. _J. Acad. Libr._ **2021** , _47_ , 102437. [CrossRef] 

102. Hair, J.F.; Astrachan, C.B.; Moisescu, O.I.; Radomir, L.; Sarstedt, M.; Vaithilingam, S.; Ringle, C.M. Executing and interpreting applications of PLS-SEM: Updates for family business researchers. _J. Fam. Bus. Strat._ **2021** , _12_ , 100392. [CrossRef] 

103. Hair, J.; Black, W.C.; Babin, B.J.; Anderson, R.E. _Multivariate Data Analysis: A Global Perspective_ , 7th ed.; Pearson Education: Upper Saddle River, NJ, USA, 2010. 

104. Eisinga, R.; Grotenhuis, M.T.; Pelzer, B. The reliability of a two-item scale: Pearson, Cronbach, or Spearman-Brown? _Int. J. Public Health_ **2012** , _58_ , 637–642. [CrossRef] 

105. Fornell, C.; Larcker, D.F. Evaluating Structural Equation Models with Unobservable Variables and Measurement Error. _J. Mark. Res._ **1981** , _18_ , 39–50. [CrossRef] 

106. Nunnally, J.C.; Bernstein, I.H. _Psychometric Theory_ , 3rd ed.; Tata McGraw-Hill Education: New Delhi, India, 1994. 

107. Franke, G.; Sarstedt, M. Heuristics versus statistics in discriminant validity testing: A comparison of four procedures. _Internet Res._ **2019** , _29_ , 430–447. [CrossRef] 

108. Henseler, J.; Ringle, C.M.; Sarstedt, M. A new criterion for assessing discriminant validity in variance-based structural equation modeling. _J. Acad. Mark. Sci._ **2015** , _43_ , 115–135. [CrossRef] 

109. Henseler, J.; Hubona, G.; Ray, P.A. Using PLS path modeling in new technology research: Updated guidelines. _Ind. Manag. Data Syst._ **2016** , _116_ , 2–20. [CrossRef] 

110. Hu, L.T.; Bentler, P.M. Cutoff Criteria for Fit Indexes in Covariance Structure Analysis: Conventional Criteria versus New Alternatives. _Struct. Equ. Model. Multidiscip. J._ **1999** , _6_ , 1–55. [CrossRef] 

111. Al-Fraihat, D.; Joy, M.; Masa’Deh, R.; Sinclair, J. Evaluating E-learning systems success: An empirical study. _Comput. Hum. Behav._ **2020** , _102_ , 67–86. [CrossRef] 

112. Wetzels, M.; Odekerken-Schröeder, G.; Van Oppen, C. Using PLS Path Modeling for Assessing Hierarchical Construct Models: Guidelines and Empirical Illustration. _MIS Q._ **2009** , _33_ , 177. [CrossRef] 

113. Abbasi, F.; Muzammal, M.; Qu, Q.; Riaz, F.; Ashraf, J. SNCA: Semi-Supervised Node Classification for Evolving Large Attributed Graphs. _Big Data Min. Anal._ **2024** , _7_ , 794–808. [CrossRef] 

114. Zhang, L.; Ma, X.; Liu, Y. Sok: Blockchain decentralization. _arXiv_ **2022** , arXiv:2205.04256. 

115. Anusha, S.; Vasundhara, D.N. Prediction of Bitcoin price for decision making using artificial intelligence. In Proceedings of the 2024 2nd International Conference on Computer, Communication and Control (IC4), Indore, India, 8–10 February 2024; IEEE: New York, NY, USA, 2024; pp. 1–6. 

116. Abbas, H.; Caprolu, M.; Di Pietro, R. Analysis of polkadot: Architecture, internals, and contradictions. In Proceedings of the 2022 IEEE International Conference on Blockchain (Blockchain), Espoo, Finland, 22–25 August 2022; IEEE: New York, NY, USA, 2022; pp. 61–70. 

117. Motepalli, S.; Freitas, L.; Livshits, B. Sok: Decentralized sequencers for rollups. _arXiv_ **2023** , arXiv:2310.03616. [CrossRef] 

118. Thibault, L.T.; Sarry, T.; Hafid, A.S. Blockchain scaling using rollups: A comprehensive survey. _IEEE Access_ **2022** , _10_ , 93039–93054. [CrossRef] 

_Future Internet_ **2025** , _17_ , 479 

31 of 31 

119. Moosavi, M.; Salehi, M.; Goldman, D.; Clark, J. Fast and furious withdrawals from optimistic rollups. In Proceedings of the 5th Conference on Advances in Financial Technologies (AFT 2023), Princeton, NJ, USA, 23–25 October 2023; Schloss DagstuhlLeibniz-Zentrum für Informatik: Wadern, Germany, 2023; p. 22-1. 

120. Liu, Y.; He, J.; Li, X.; Chen, J.; Liu, X.; Peng, S.; Cao, H.; Wang, Y. An overview of blockchain smart contract execution mechanism. _J. Ind. Inf. Integr._ **2024** , _41_ , 100674. [CrossRef] 

121. Fekete, D.L.; Kiss, A. Trust-minimized optimistic cross-rollup arbitrary message bridge. _J. Netw. Comput. Appl._ **2023** , _221_ , 103771. [CrossRef] 

122. Feist, J.; Grieco, G.; Groce, A. Slither: A static analysis framework for smart contracts. In Proceedings of the 2019 IEEE/ACM 2nd International Workshop on Emerging Trends in Software Engineering for Blockchain (WETSEB), Montréal, QC, Canada, 27 May 2019; IEEE: New York, NY, USA, 2019; pp. 8–15. 

123. Wei, Z.; Sun, J.; Zhang, Z.; Zhang, X.; Yang, X.; Zhu, L. Survey on quality assurance of smart contracts. _ACM Comput. Surv._ **2024** , _57_ , 1–36. [CrossRef] 

124. Woitschig, P.; Uddin, G.S.; Xie, T.; Härdle, W.K. The energy consumption of the Ethereum-ecosystem. _SSRN Electron. J._ **2023** . [CrossRef] 

125. Lavin, R.; Liu, X.; Mohanty, H.; Norman, L.; Zaarour, G.; Krishnamachari, B. A survey on the applications of zero-knowledge proofs. _arXiv_ **2024** , arXiv:2408.00243. [CrossRef] 

126. Messias, J.; Pahari, V.; Chandrasekaran, B.; Gummadi, K.P.; Loiseau, P. Understanding blockchain governance: Analyzing decentralized voting to amend defi smart contracts. _arXiv_ **2023** , arXiv:2305.17655. 

127. Khan, S.; Qiming, H. GPU-accelerated homomorphic encryption computing: Empowering federated learning in IoV. _Neural Comput. Appl._ **2025** , _37_ , 10351–10380. [CrossRef] 

128. Murray, M.D. Generative ai art: Copyright infringement and fair use. _SMU Sci. Technol. Law Rev._ **2023** , _26_ , 259. [CrossRef] 

129. Ternar, A.; Denisova, A.; Cunha, J.M.; Kultima, A.; Guckelsberger, C. Generative AI in Game Development: A Qualitative Research Synthesis. _arXiv_ **2025** , arXiv:2509.11898. [CrossRef] 

130. Zhang, M.; Qu, Q.; Ning, L.; Fan, J. On time-aware cross-blockchain data migration. _Tsinghua Sci. Technol._ **2024** , _29_ , 1810–1820. [CrossRef] 

131. Karim, M.M.; Sharif, K.; Biswas, S.; Latif, Z.; Qu, Q.; Li, F. CIC-SIoT: Clean-Slate Information-Centric Software-Defined Content Discovery and Distribution for Internet-of-Things. _IEEE Internet of Things J._ **2024** , _11_ , 37140–37153. 

132. Emmert, F. Blockchain and private international law—The perspective of the United States of America. In _Blockchain and Private International Law_ ; Brill Nijhoff: Leiden, The Netherlands, 2023; pp. 709–726. 

133. Akanfe, O.; Lawong, D.; Rao, H.R. Blockchain technology and privacy regulation: Reviewing frictions and synthesizing opportunities. _Int. J. Inf. Manag._ **2024** , _76_ , 102753. [CrossRef] 

**Disclaimer/Publisher’s Note:** The statements, opinions and data contained in all publications are solely those of the individual author(s) and contributor(s) and not of MDPI and/or the editor(s). MDPI and/or the editor(s) disclaim responsibility for any injury to people or property resulting from any ideas, methods, instructions or products referred to in the content. 

Copyright of Future Internet is the property of MDPI and its content may not be copied or emailed to multiple sites without the copyright holder's express written permission. Additionally, content may not be used with any artificial intelligence tools or machine learning technologies. However, users may print, download, or email articles for individual use. 

