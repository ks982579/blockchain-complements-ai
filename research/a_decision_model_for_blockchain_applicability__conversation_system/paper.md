Knowledge-Based Systems 220 (2021) 106791 

Contents lists available at ScienceDirect 

# Knowledge-Based Systems 

journal homepage: www.elsevier.com/locate/knosys 

# A decision model for blockchain applicability into knowledge-based conversation system 

Wenli Yang , Saurabh Garg, Zhiqiang Huang, Byeong Kang 

_Discipline of ICT, School of Technology, Environments and Design, University of Tasmania, Australia_ 

https://doi.org/10.1016/j.knosys.2021.106791 

0950-7051/© 2021 Elsevier B.V. All rights reserved. 

_W. Yang, S. Garg, Z. Huang et al._ 

### Article Info

_Article history:_ Conversation Received 1 October 2020 human experts. Received in revised form 19 January 2021 based on Accepted 19 January 2021 Blockchain Available online 25 February 2021 

_Keywords:_ Blockchain platform Conversation system Knowledge-based Decision model Multiple measurements 

## Abstract

Conversation systems usually suffer from the challenge of knowledge management from multiple human experts. The current mechanism used in knowledge-based conversation system is always based on centralized servers, which may be problematic in terms of transparency and security. Blockchain solutions are currently being proposed improve the security and efficiency in different domains. However, there are various blockchain platforms with different characteristics, and conversation system implemented using blockchain platform is not in place yet. In this paper, we clearly identified the requirement analysis of knowledge-based conversation system and present a decision model for identify the best fitting blockchain platform for knowledge-based conversation system. In the proposed method, multiple measurements including Analytical Hierarchy Process (AHP), Fuzzy analytical hierarchy process (FAHP), and Fuzzy Technique for Order Preference by Similarity to Ideal Solution (FTOPSIS) are utilized to analyze and create consistent result together, which can be used for the selection of blockchain platforms and improve the efficiency of the decision-making process. © 2021 Elsevier B.V. All rights reserved. 

## **1. Introduction** 

Conversation systems mainly involves natural language understanding, conversation management, knowledge query and natural language generation [1]. The ability to query Knowledge Base (KB) is essential in conversation systems and the KB interpretation requires human inputs. To ensure the quality of knowledge base, it is better to use multiple experts to share the individual knowledge together [2]. However, the current mechanism of knowledge management from multiple human experts in the conversation system is based on centralized servers, and the main challenges of the centralized mechanism include knowledge sharing and contribution assignment [3]. 

- Knowledge sharing: Traditional knowledge exchange methods are always based on the central server or a third party to collect and transfer knowledge between experts. But this strategy can only support opensource information and for many conventional scenarios, such as medical training, psychological consultation, and travel services, etc., they may include the user’s identity information and privacy task solutions with different authorities and permissions. Therefore, it is difficult to manage multiple experts and create knowledge acquisition in a trusted and secure manner by using a centralized sharing mechanism. 

- ∗ Corresponding author. _E-mail address:_ yang.wenli@utas.edu.au (W. Yang). 

- Contribution assignment: The contributions for each expert, such as the number of successful transactions, and the average customer interaction time of added scenarios, should be stored in a trustworthy and secure manner. The information in traditional mechanisms is always stored in log files that can provide an audit trail but are easily erased or alterable without a trace. Thus, centralized control is not ideal for contribution assignment. 

Contrary to traditional centralized mechanisms, blockchain technology is based on decentralized transaction and data management, which can provide anonymity, safety, and data integrity [4–6]. Blockchain combines with multiple technologies to ensure an immutable, irrevocable, and traceable blockchain ledger [7,8], making this field a vast research area to deal with the limitations and enhancements in current knowledge-based conversation system [9]. Once established a blockchain-based solution is the right underlying technology, we face a significant challenge of selecting one suitable blockchain platform available for the conversation system. The selection process of blockchain has to consider and fit to the demands and problems of knowledge-based conversation system. In addition, the number of blockchain platforms keeps increasing and the features of blockchain platform also keep improving, therefore, the blockchain platform selection must be adapted keeping collecting and updating. If there are some new blockchain platforms in the market, the knowledge regarding new blockchain platform can be quickly organized and evaluated into selection process when it needs to be applied into conversation system. At present, there are no feasible decision model to support the selection of the most suitable blockchain platforms when applied into knowledge-based conversation system. 

In this paper, a decision model with multiple measurements is presented to identify the best fitting blockchain platform for knowledge-based conversation system. First, the requirement analysis and design aims are clearly identified when applying blockchain into knowledge-based conversation systems. Subsequently, the selection of blockchain platform applied into knowledge-based conversation system is modeled as a decision making problem. Finally, a blockchain applicability evaluation model using multiple measurements was built, which provided an efficient method of selecting the most suitable decentralized and transparent platform for multiple experts to maintain and share knowledge in a manner of trust and security. This paper has the following contributions: 

- (1) Clearly identified the requirements of knowledge-based conversation system and design aims using blockchain based mechanism. 

- (2) Modeling the blockchain platform selection as a decision making problem with multiple criteria based on the identified requirements as well as design aims. 

- (3) A decision model using multiple measurements was built to evaluate the feasibility of blockchain and get final consistent decision. 

The remainder of this paper is organized as following structure. Section 2 describes the related work about selection of blockchain platforms and existing decision-making methods. Section 3 outlines how to formulate decision-making for blockchain platform selection as a standardized model. Section 4 presents a decision model to address the blockchain platform selection problem. Section 5 illustrates an evaluation and analysis with four popular blockchain platforms. Section 6 summarizes the proposed approach, defends its novelty, and offers limitations as well as directions for future studies. 

## **2. Related work** 

In this part, we mainly review some state-of-art works in two primary areas: evaluating and selecting for blockchain platforms and decision-making methods. 

_**Evaluating and selecting of blockchain platforms:**_ several studies point out evaluation and comparison of different blockchain platforms [10–14]. Dinh et al. [10] proposed a benchmarking framework for evaluating private blockchain systems, which contains workloads for measuring the data processing performance, and workloads for understanding the performance at different layers of the blockchain. Hileman and Rauchs [11] presented a global benchmarking study to provide better understand current areas of focus, attitudes toward the blockchain technology and challenges that need to be answered. Maple and Jackson [12] focused on the assessment of different types of blockchain to provide guidance to choose best blockchain solution, which includes analyzing blockchain essential technical features, outlining blockchain building blocks and comparing multiple blockchain platforms. Kuo et al. [13] introduced a comparison of popular blockchain platforms using a systematic review method, and provided a reference for selection of a suitable blockchain platform given requirements and technical features that are common in healthcare and biomedical research applications. Siamak et al. [14] designed a decision support method for blockchain platform selection and used three industry case including ShareCompany BIQH, DUO and Veris Foundation which focused on financial area, education and healthcare respectively. 

_**Decision-making techniques:**_ a variety of decision-making methods have applied into many research areas recently. The popular selected decision-making methods are presented as below: 

Analytic hierarchy process (AHP): is a well-known theory of measurement through pairwise comparisons and relies on the judgments of experts to derive priority scales [15]. Maček and Alagić [16] described a AHP model to evaluate Bitcoin cryptocurrency in the context of information security risks related to the existing most common online payment systems like e-banking, m-banking, and e-commerce. 

Fuzzy-based methods: uses linguistic variables to express the comparative judgments given by decision makers, and linguistic variables are expressed qualitatively by linguistic terms and quantitatively by a fuzzy set in the universe of discourse and respective membership function [17]. Ferhat et al. [18] proposed a multi-criteria decision model to assist a global logistics company on the blockchain software selection problem using Buckley’s Fuzzy Analytical Hierarchy Process (Fuzzy AHP). 

Technique for Order Preference by Similarity to Ideal Solution (TOPSIS): defined the positive ideal matrix and negative ideal matrix and calculated the distance between expert and the decision matrix to determine the weights of decision makers [19]. Mohammad et al. presented an assessment method to evaluate the impact of different blockchain models using TPOSIS for healthcare management. 

Table 1 summarized the selected studies that discuss the blockchain platform selection problem. The studies using benchmarking are based on documentations and reports, which are often out of date soon and should keep updating continuously. And the majority of decision model using AHP, Fuzzy based and TOPSIS are appropriate for specific case studies. However, it may not be suitable for knowledge-based conversation applications. In addition, the current decision model always focused on the selection of various decision-making methods and choose the most efficient one applied into domain area, but no matter which decision-making method selected, it still has some limitations and choosing different methods may have inconsistent results. It is better to use multiple measurements to solve conflict and get consistent evaluation result. In our proposal, we propose a standardized framework to guide decision-making process for blockchain-based conversation, which can be used for keeping updating, and a decision model with multiple measurements will be used to evaluate different blockchain platforms to select the best fitting blockchain platform. 

## **3. Modeling decision-making steps for blockchain platforms** 

## _3.1. Requirement analysis_ 

The knowledge-based conversation system is composed of multiple experts who can share and construct a knowledge database collaboratively to address many complex tasks efficiently. The user’s request may be related to different scenarios and these questions should be solved by multiple experts based on a predefined cooperation strategy. Thus, the knowledge-based conversation system is an integrated system for implementing conversation scenario management, decision support, and intelligent search, which has characteristics shown in Table 2. 

Based on the analysis shown in Table 2, we can see that the main requirements of the knowledge-based conversation system include content reliability and confidentiality, immediacy response and open-ended and extensible. 

**Table 1** 

The summarization of related studies regarding selection of blockchain platforms problem. 

|Decision-making methods|Domain specific|References|
|---|---|---|
|Benchmarking|Not defined|[3–5]|
|AHP|Financial area, education, and healthcare;|[7,9]|
||e-banking, m-banking, e-commerce||
|Fuzzy based|Supply chain|[11,20]|
|TOPSIS|Healthcare|[21,22]|
||Internet of vehicle||



## **Table 2** 

The requirement analysis of the knowledge-based conversation system. 

> [!warning]
> Table does not match article format exactly - markdown conversion issue.

|**Table 2**<br>The requirement analysis|of the knowledge-based conversation system.| |
|---|---|---|
|Key requirements|Detailed description|Design aim|
|Content reliability|Content may be related to the user, e.g., personal|(1) Identifier content should have secure|
|and confidentiality|information, so that conversational content|storage.|
||maintained by each participating expert should be|(2) Shared knowledge should be transferred|
||kept secure and be identified clearly. This enables|and stored safely.|
||participants to make their contributions in a|(3) Contributions should be assigned without|
||trustworthy and secure manner [23,24]. Expert|central control that can reduce the possibility|
||knowledge should also be protected during sharing.|of insecurity caused by human factors.|
|Immediacy and|Responses should be immediate and users’ requests|(1) Agreement between multiple experts|
|accurate response|should be replied to accurately [25].|should be reached to get a consensus.|
|||(2) Support fast searching and matching in the|
|||knowledge database to get an accurate|
|||response.|
|Open-ended and|The more open the system is, the more efficient|(1) Design a fair incentive scheme to|
|extensible|the knowledge base, which should also expand in|encourage communities of participants to|
||the future [26,27].|cooperate and create value.|
|||(2) The handling capacity of the system should|
|||have a certain redundancy to expand the|
|||functional module.|



Content reliability and confidentiality: Conversation content is very important for users to protect their privacy and for each expert to evaluate their contributions during sharing and maintenance [28,29]. Thus, this content should be clearly identified and ensure data integrity. The design aims including security storage, security knowledge sharing and contributions assignment without central control are identified to meet this requirement. 

Immediacy and accurate response: to solve tasks, multiple experts should find the best solutions based on an efficient consensus mechanism. This assurance can allow the system to reach an agreement over the whole network and to have global collaboration between multiple experts. Current knowledge-based conversation systems lack consensus methods among multiple experts to support immediate and accurate responses [23,24]. Therefore, reaching agreement between multiple experts as well as fast searching and matching knowledge base are design aims to satisfy immediate and accurate response. 

Open-ended and extensible: to support multiple experts sharing and maintaining knowledge base together, an incentive scheme must be built to encourage multiple experts to promote good content and restrict bad content. And the knowledge base should keep expanding in the future. Based on these requirements, a fair incentive scheme and the capacity of expansion are necessary design aims. 

## _3.2. Modeling concepts_ 

Based on the requirement analysis of a knowledge-based conversation system, we modeled the selection of blockchain platform as a decision-making problem. The modeling of decisionmaking steps for blockchain platform selection are shown as Fig. 1. 

As shown in Fig. 1, based on the identified design aims, we can formulate the modeling concepts into three steps. First, decision items are selected according to the design aims. Then, the multiple evaluating criteria are identified for decision items. 

Finally, applicability levels of blockchain applied in conversation systems are assigned based the opinions from experts or related documentations. 

## _3.2.1. Decision items for evaluating blockchain platforms_ 

Based on the requirement analysis of knowledge-based conversation system, four group decision items, including decentralized architecture, storage and sharing, computing performance, and scalability are selected to match our design aim, then the detailed items are identified as well. Then for each group of decision item, corresponding blockchain configurations are chosen as evaluation criteria for evaluating blockchain platforms. 

**Decentralized architecture:** there are three architectures regarding trust mechanism: centralized, partial centralized and total decentralized [30]. The centralized architecture is relied on a single or a few entities that control the entire network, and the partial centralized architecture is used to manage authority allocation of nodes, such as read–write access, transaction authority, etc. The decentralized architecture is based on decentralized transaction and data management which can provide anonymity, safety, and data integrity. There is no need of a thirdparty organization to control the transactions, making this field a vast research area to deal with limitations and enhancements in the centralized and partial architectures. Different architectures may cause different blockchain types and chain structures. Public chain is fully decentralized where any node can join and leave the system with better transaction transparency and security but affect the network performance. The consortium chain provides the data sharing between organizations, while the private chain is used to manage access permission within single organization. 

> [!warning]
> Figure omitted in markdown version

**Fig. 1.** The overall modeling concepts. 

**Storage and sharing:** The content in the knowledge-based conversation system includes different types of scenarios and permissions so that how to design the storage and sharing mode is also an important point. Blockchain has two storage modes: the on-chain and off-chain. On-chain is used to storage metadata, key data and hash values, while off-chain can be considered as private cloud or third-party storage [31,32]. In addition, the contents may be related to identical information and unimportant information, we should decide whether it should be stored permanently or not. Thus, it not only considers the reliability and availability of service, but also considers flexibility and how to reduce the deployment cost. 

**Computing performance:** Regarding the computing and processing in the knowledge-based conversation system, it needs to consider the blockchain configuration such as consensus protocol, incentive scheme, and to decide if we need to design the new method or not [5]. Consensus algorithms are the core technology for blockchain, since it determines who keeps records, which would influence the security and reliability for the whole system. So far, there have been many consensus algorithms, with Proof of Work, Proof of Stake, and Delegated Proof of Stake, the most popular ones. However, we should consider the processing speed and fault tolerance of protocols, and if there needs to be some improvements based on the existing methods, the deployment cost and flexibility should be also considered. 

**Scalability:** To support variety conversation scenario and multiexpert decision, the blockchain configurations should consider on-chain scaling and off-chain scaling, many on-chain scaling such as Segregated Witness and off-chain scaling such as Lightning Network are used to increase blockchain size and storage efficiency [33,34]. Different blockchain platforms use different scaling methods including space recovery, parallel verification, deployment cost, etc. 

## _3.2.2. Levels of applicability_ 

According to the satisfaction degree matched with design aims, applicability levels are defined in Table 3 as very inappropriate, inappropriate, appropriate, and very appropriate. 

## _3.2.3. Formulation the blockchain platform selection with multiple criteria_ 

For each group of decision item, the main flow is presented to formulate the selection of blockchain platform with multiple criteria based on expert opinions and literature reviews. The arrows are used to represent the possible order for decision making (see Fig. 2). 

The design flow starts from deciding whether it is needed to use decentralization architecture or not. Total centralization architecture has a higher network performance and is easy to deploy but has lower attack-resistance that provides attackers with a single target to hack. Partial centralization also has appropriate network performance, attack-resistance, and fault tolerance because of limited node access and transactional authority to perform refinement operations. Decentralized architecture has the highest attack-resistance but lowest network performance, and it is also hard to deploy compared to centralization because it is open to the public [30] (see Table 4). 

**Table 3** 

Applicability levels of blockchain applied in conversation systems. 

| Applicability levels | Applicability statement                                                   |
| -------------------- | ------------------------------------------------------------------------- |
| Very inappropriate   | Does not satisfy the design aim using blockchain-based mechanisms.        |
| Inappropriate        | Satisfies the design aim using blockchain-based mechanisms but only       |
|                      | fewer blockchain configurations are acceptable.                           |
| Appropriate          | Satisfies the design aim using blockchain-based mechanisms and most       |
|                      | blockchain configurations are acceptable.                                 |
| Very appropriate     | Satisfies the design aim using blockchain-based mechanisms and almost all |
|                      | the blockchain configurations have a high performance.                    |

> [!warning]
> Figure omitted in markdown version

**Fig. 2.** The main workflow to guide formulation models. 

**Table 4** 

Design decisions regarding decentralization architecture. +: very inappropriate, ++: inappropriate, + + +: appropriate, + + ++: very appropriate. 

| Design decisions regarding<br>veryy appropriate.ppropriate.ropriate.priate.riate. | regarding decentralization architecture. | architecture. +:: very inappropriate, | inappropriate, ++:: inappropriate, | inappropriate, + + +:: appropriate, | appropriate, + + ++: |
| --------------------------------------------------------------------------------- | ---------------------------------------- | ------------------------------------- | ---------------------------------- | ----------------------------------- | -------------------- |
| Decision item                                                                     | Description                              | Network                               | Deployment                         | Attack-                             | Fault                |
|                                                                                   |                                          | performance                           | cost benefits                      | resistant                           | tolerance            |
| Total centralization                                                              | Single/multipoint service                | + + ++                                | + + ++                             | +                                   | + + +                |
| Partial centralization                                                            | Permission chain                         | + + +                                 | + + +                              | + + +                               | + + +                |
| Decentralization                                                                  | Public chain                             | ++                                    | ++                                 | + + ++                              | ++                   |

The second design decision is the on-chain and off-chain division regarding the data classification, storage and sharing. The on-chain mechanism can use transaction constraints and smart contracts to provide storage and sharing, which is more reliable. Transaction constraints have some limitations on transaction type and size, while a smart contract uses variables and log events to support more flexible storage and sharing. At present, Bitcoin only provides simple on-chain storage, while the functions in Ethereum are more powerful with smart contracts. The off-chain mechanism can use the local private cloud or a third-party platform, which is easy to deploy and supports the flexible availability of the service but has low reliability because it is easily erased or alterable without a trace [32] (see Table 5). 

**Table 5** 

Design decisions regarding storage and sharing. +: very inappropriate, ++: inappropriate, + + +: appropriate, + + ++: very appropriate. 

| **Table 5**<br>Design decisions regarding | storage and sharing. +: very inappropriate, ++: inappropriate, + + +: appropriate, + + ++: very appropriate.                                                          |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Decision item                             | Description<br>Reliability<br>Availability<br>of service<br>Flexibility<br>and opening<br>Deployment<br>cost benefits                                                 |
| Data classification                       | On-chain<br>Transaction constraints<br>+ + ++<br>++<br>++<br>++<br>Smart contract<br>+ + +                                                                            |
|                                           | Off-chain<br>Localization/ third-party platform<br>+ + +<br>+ + +<br>+ + +<br>+ + ++                                                                                  |
| Data storage                              | On-chain<br>Embedded transaction (Bitcoin)<br>+ + ++<br>+<br>+<br>++<br>Embedded transaction (Ethereum)<br>++<br>+<br>++<br>Smart contract<br>+ + +<br>+ + +<br>+ + + |
|                                           | Off-chain<br>Localization<br>++<br>+ + +<br>+ + +<br>+ + ++<br>Third-party platform<br>+ + +<br>+ + +<br>+ + +                                                        |
| Data sharing                              | On-chain<br>Transaction constraints<br>+ + ++<br>++<br>+ + +<br>++<br>Smart contract<br>+ + +                                                                         |
|                                           | Off-chain<br>Localization, third-party platform<br>+<br>+ + +<br>+ + +<br>+ + ++                                                                                      |

Then, the design decisions regarding computing performance include searching and matching, consensus protocol, incentive scheme will be processed. Searching and matching are related to the chain structure. Single chain has lower processing speeds and flexibility but is easy to deploy, while side chain and multi-chain have high processing speeds to support more conversation scenarios. The consensus protocols can be divided into proof-based and voting-based. The agreement base of proof-based consensus algorithms, such as PoW and PoS, is following nodes to perform enough proof, and most nodes can join freely. It always has high decentralized and low processing speeds, while voting-based verifies the network from a majority of node decisions under limited executing nodes, such as Paxos and practical Byzantine fault tolerance (PBFT), which have higher processing speeds and fault tolerance. Additionally, the incentive system includes an economic-based model and a non-economic model that compensates individuals with money or other rewards. The token economic model is easy to compute with high speed but has less flexibility and stability. The non-economic model is not related to financial factors and for the conversation system, the design of the non-economic model should focus on how to provide participants with more opportunities to receive good content, which may need more flexible computing processes [5] (see Table 6). 

**Table 6** 

Design decisions regarding computing performance. +: very inappropriate, ++: inappropriate, + + +: appropriate, + + ++: very appropriate. 

|Decision item|Description|Processing speed|Deployment cost benefits|Flexibility and opening|Fault tolerance|
|---|---|---|---|---|---|
|Searching and matching|Single chain|++|+ + ++|++|++|
||Side chain|+ + +|+ + +|+ + +|+ + +|
||Multi-chain|+ + ++|+ + +|+ + ++|+ + +|
|Consensus protocol|Proof-based|++|++|+ + +|++|
||Voting-based|+ + +|+ + +|+ + +|+ + +|
|Incentive scheme|Economic model|+ + +|++|++|+ + +|
||Non-economic model|++|+ + +|+ + ++|+ + +|

With more experts and uses being involved, the popularity has brought the network scalability problem to light. There are two ways to scale a system to handle millions of transactions: on-chain scaling and off-chain scaling. With on-chain scaling, all transactions are made in the blockchain. At present, one way is to increase the blocksize directly to process more transactions in a short time. Another way is to remove the overhead from the block to increase data storage. These methods can improve the little space available; however, it is still an utterly inadequate method of dealing with the scalable problem. Off-chain scaling uses extra layers on top of the blockchain to deal with most of the transactions, which will be bundled as one and saved on the blockchain, such as the Lightning Network. This can create channels across peers using a two-layer network without any limitations in blocksize. Compared to on-chain scaling, off-chain scaling is more flexible and has better concurrency [34] (see Table 7). 

Based on the evaluation criteria selected in Table 9, we set the evaluation criteria U = {U1, _U_ 2, _U_ 3, _U_ 4, _U_ 5, _U_ 6} as below to build the membership matrix for each alternative blockchain platform. 

- Access authority: private chain, consortium chain, public chain. 

- Chain structure: single chain, side chain, multi-chain. 

- Storage and sharing mechanism: on-chain and off-chain. 

- Consensus protocol: PoW, PoS, Byzantine fault tolerance (BFT), PBFT. 

- Incentive scheme: token, no financial factors, both. 

- Scale mode: on-chain, off-chain, others. 

Then experts’ opinions are collected to identify the criterion and sub-criterion using DELPHI method with a 0.1–0.9 rating scale [35]. 

## _4.2. A weighting method using multiple measurements for decision making_ 

In this section, to overcome the limitations and inconsistent decisions making by single measurement, we aim to identify the consistent criteria weighting for selecting blockchain platforms and rank such platforms using multiple measurement methods. We will compare the results from these three methods and provide the most appropriate options for blockchain platform selection. The detailed steps are introduced how to use these three methods, respectively. 

## _4.2.1. Multiple measurement methods_ 

## **4. A decision model for blockchain platform selection** 

In general, a decision model contains three basic layers: target, criteria, and alternatives as shown in Fig. 3. For our research, the target is to analyze and select the most suitable blockchain platform for knowledge-based conversation system. Criteria are used to make decisions based on the identified decision items. The alternative platform options to be evaluated are Ethereum, Fabric, Corda, Multichain and even more. 

In our proposed decision model, first, weighted membership matrixes of each blockchain platform are built by using multiple criteria. Then the evaluation results are composed and synthesized using multiple measurements. Finally, the final decision result will be judged by consistent checking and get the best fitting blockchain platform. 

## _4.1. Weighted membership matrix_ 

Based on the established model, the multiple evaluation criteria for a blockchain-based conversation system is presented in Table 8. 

In our paper, the three most popular weighting methods including AHP, Fuzzy based AHP and TOPSIS are utilized to combine and evaluate the criteria weighting. 

## **(1) Analytical hierarchy process (AHP)** 

The analytical hierarchy process is a multiple criteria decisionmaking method that was presented by Prof. Thomas L. Saaty [15]. AHP simplifies preference ratings in decision criteria using pairwise comparisons. By checking the consistency of attribute values during measurements, AHP can eliminate bias and conflicts in decision making. Using AHP normally involves four main steps: Step 1: Decomposing the decision-making problem into a hierarchy structure with general levels. 

Step 2: Developing a pairwise comparison matrix, establishing priorities between criteria in the hierarchy using the nine-point scale presented by Saaty and Vargas [36], and normalizing the resulting matrix. 

Step 3: Synthesizing judgments to calculate percentages for weight attributes, which includes normalizing the comparison matrix and computing the weights. 

Step 4: Calculating the consistency ratio to measure the above judgments, which are consistent, and obtaining the set of final 


**Table 7**

Design decisions regarding scalability. +: very inappropriate, ++: inappropriate, + + +: appropriate, + + ++: very appropriate.

| Decision item     | Description    | Deployment cost benefits  | Flexibility and opening | Concurrent capacity | Space recovery|
| ----------------- | ------------------------ | ------------- | ----------- | ---------- | -------- |
| On-chain scaling  | Increasing the blocksize | ++            | ++          | ++         | ++       |
|                   | Removing the overhead    | + + +         | + + +       | ++         | ++       |
|                   | Shading                  | + + +         | + + +       | ++         | ++       |
| Off-chain scaling | Lightning Network        | + + +         | + + +       | + + +      | + + +    |
|                   | Raiden Network           | + + +         | + + +       | + + +      | + + +    |
|                   | Plasma                   | + + +         | + + +       | + + +      | + + +    |



**Fig. 3.** Basic Hierarchy model of process design. 

**Table 8** 

Evaluation criteria and sub-criteria for a blockchain-based conversation system. 

|Criterion|Sub-criterion|Detailed items|
|---|---|---|
|Decentralization architecture|Access authority|Private chain, Consortium chain, Public chain|
||Chain structure|Single chain, Side chain, Multi-chain|
|Storage and sharing|Data classification|On-chain, Off-chain|
||Data storage|On-chain, Off-chain|
||Data sharing|On-chain, Off-chain|
|Computing performance|Consensus protocol|Proof-based: PoW, PoS,|
|||Voting-based: PBFT, Raft,|
|||Others: Notary|
||Incentive scheme|Token economic model,|
|||Non-economic model with no financial factors,|
|||Both|
|Scalability|Scale mode|On-chain: increasing blocksize, removing overhead,|
|||shading.|
|||Off-chain: Lightning Network, Raiden Network,|
|||Plasma.|



weights. The consistency criteria (CI) is calculated by CI = ( _λ_ max − n)/n − 1, where _λ_ max is the maximum eigenvalue of the judgment’s matrix, and the consistency ratio is CR = CI/RI. 

## **(2) Fuzzy analytical hierarchy process (FAHP)** 

The FAHP method is an updated analytical method developed from the classic AHP. It was difficult to set uncertain attributes from the crisp values using AHP; therefore, FAHP was proposed to resolve the uncertainty of the AHP approach by performing fuzzy comparisons, which makes decisions for multiple criteria using weight derivations from a fuzzy pairwise comparison. Chang (1992) proposed a creative algorithm for dealing with pairwise comparison scales by using triangular fuzzy numbers. In 1996, he introduced a new analysis algorithm for simulated values of pairwise comparisons. So far, FAHP has been used to make decisions, such as the selection of cryptographic algorithms for blockchain, evaluating the risks of blockchain, and other issues. The process of FAHP for blockchain platform selection has the following four steps [37]: 

Step 1: Building the evaluation hierarchy structure for selecting total n blockchain platforms. 

Step 2: Determining the evaluation dimension weights using a 0.1–0.9 scale to build the judgment matrix _A_ =[(] _aij_ ) _n_ × _n_[.] Step 3: Establishing the fuzzy consistent matrix _R_ =[(] _rij_ ) _n_ × _n_[.] whose elements have degrees of membership. 

**==> picture [252 x 28] intentionally omitted <==**

Step 4: Calculating the weight vector of the elements in all dimensions of the hierarchy system by using the root-squaring method. 

**==> picture [252 x 80] intentionally omitted <==**

## **(3) Technique for Order Preference by Similarity to Ideal Solution (TOPSIS)** 

TOPSIS is a popular multiple attribute decision-making (MADM) method first proposed by Hwang and Yoon in 1981, further refined by Yoon in 1987, and then updated again by Hwang et al. (1993). TOPSIS is a type of compensatory aggregation method that compares all substitutes by determining weights for every attribute, normalizing the scores for every attribute, and calculating the geometric distance between each alternative and the ideal alternative. The ideal alternative is the one with the best score in every attribute. The attributes of TOPSIS alternatives are assumed to be monotonically increasing or decreasing. Trade-offs could exist between TOPSIS attributes, which means that a wrong result in any one attribute could be denied by a correct result in any other attribute [38]. 

The short TOPSIS process for our BaaS selection is based on the following steps: Step 1: Take m alternatives and n criteria to create an evaluation matrix, using _A_ =[(] _aij_ ) _n_ × _n_[to get the intersection of each criterion] and alternative. 

Step 2: The matrix, A, is then normalized to form a matrix. 

**==> picture [252 x 14] intentionally omitted <==**

Step 3: Define the weight of each criterion in the evaluation matrix: _w_ = [ _w_ 1 _, w_ 2 _, . . . , wn_ ], then get the weighting normalized matrix _C_ =[(] _cij_ ) _n_ ∗ _n_[.] Step 4: Determine the worst alternative _C_[−] and the best alternative. 

max _cij, where criteria j is positive criteria Cj_[+][=] {min _cij,where criteria j is negative criteria_ 

- min _cij, where criteria j is positive criteria_ 

- _Cj_[−][=] {max _cij,where criteria j is negative criteria_ (4) 

Step 5: Calculate the L2-norm distances _D_[−] and _D_[+] between the target alternative, i and the worst condition, _C_[−] , and the distance between the alternative, i, and the best condition, _C_[+] , respectively. 

**==> picture [252 x 35] intentionally omitted <==**

Step 6: Calculate the similarity to the worst condition then rank the alternatives according to the results. 

## _4.2.2. Composing and synthesizing_ 

Based on the membership matrix R = { _Re, Rf , Rc, Rm_ } of blockchain platforms as well as calculated weighting W = { _WAHP , WFAHP , WTOPSIS_ } from three measurements, the weighted average method by B = W ◦ R is used to get the three groups of evaluation results for each alternative blockchain platforms: 

**==> picture [121 x 37] intentionally omitted <==**

## _4.2.3. Consistent checking_ 

Based on the above results, Eq. (7) was used to judge and determine the selection of blockchain platforms. 

**==> picture [252 x 29] intentionally omitted <==**

When |W − n| ≤ 0 _._ 5 _,_ (n = 1 _,_ 2 _,_ 3 _,_ 4), the final evaluation level will be defined in the _n_ level. The results with same level using all three methods will be finalized as the consistent result, which can be candidates to select the most suitable blockchain platforms. 

## **5. Evaluation results and analysis** 

## _5.1. Implementation stages_ 

From the decision model described in Section 4, the following stages were conducted in the research. 

Stage 1. Determine the weighted membership matrix of each alternative blockchain platform 

- Collect the experts’ opinions to identify the criterion and sub-criterion using expert DELPHI method with 0.1–0.9 rating scale. 

- Build the membership matrix of blockchain platforms based on their technical features. 

Stage 2 Implementation of AHP for weighting decisions and ranking blockchain platforms 

- Build the comparative matrix of attributes based on hierarchical levels of the sub-criterion. 

- Calculate the consistency ratio and verify AHP consistency. 

- Get the relative attribute weights of each sub-criterion. 

Stage 3 Implementation of FAHP for weighting decisions and ranking blockchain platforms 

- Build the comparative matrix of attributes based on hierarchical levels of the sub-criterion. 

- Build the fuzzy consistent matrix. 

- Get the fuzzy synthesis values. 

- Calculate the attribute weights of each sub-criterion. 

Stage 4 Implementation of TOPSIS for weighting decisions and ranking blockchain platforms 

8 

_W. Yang, S. Garg, Z. Huang et al._ 

_Knowledge-Based Systems 220 (2021) 106791_ 

**Table 9** 

The results of applicability levels for all criteria. 

> [!warning]
> Tables have issues due to markdown conversion.

| **Table 9**<br>The results of applicability levels for all criteria.                                                                                                                     | **Table 9**<br>The results of applicability levels for all criteria.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Criterion<br>Sub-criterion<br>Very<br>inappropriate<br>Inappropriate<br>Appropriate<br>Very<br>appropriate                                                                               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_1<br>Private chain<br>0.5<br>0.3<br>0.1<br>0.1<br>Consortium chain<br>0.1<br>0.2<br>0.2<br>0.5<br>Public chain<br>0.0<br>0.2<br>0.2<br>0.6                                            |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_2<br>Single chain<br>0.6<br>0.2<br>0.1<br>0.1<br>Side chain<br>0.4<br>0.4<br>0.2<br>0.2<br>Multi-chain<br>0.1<br>0.1<br>0.2<br>0.6                                                    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_3<br>On-chain<br>0.3<br>0.2<br>0.2<br>0.3<br>Off-chain<br>0.0<br>0.0<br>0.8<br>0.2                                                                                                    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_4<br>PoW<br>0.9<br>0.1<br>0.0<br>0.0<br>PoS<br>0.6<br>0.2<br>0.1<br>0.0<br>PBFT<br>0.0<br>0.2<br>0.3<br>0.5<br>Raft<br>0.0<br>0.2<br>0.2<br>0.6<br>Notary<br>0.1<br>0.2<br>0.2<br>0.5 |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_5<br>Token<br>0.3<br>0.2<br>0.2<br>0.3<br>Non-financial<br>0.0<br>0.3<br>0.3<br>0.4<br>Both<br>0.0<br>0.0<br>0.1<br>0.9                                                               |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| _U_6<br>On-chain<br>0.1<br>0.2<br>0.2<br>0.5<br>Off-chain<br>0.0<br>0.1<br>0.2<br>0.7                                                                                                    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|                                                                                                                                                                                          | **Table 10**<br>Comparative judgment matrix for criterions.<br>Blockchain platforms<br>Access<br>authority<br>Chain<br>structure<br>Storage and<br>sharing<br>Consensus<br>Incentive<br>Scale mode<br>Access authority<br>1<br>1/4<br>1/3<br>1/5<br>3<br>2<br>Chain structure<br>4<br>1<br>1/2<br>1/3<br>4<br>3<br>Storage and sharing<br>3<br>2<br>1<br>1/4<br>4<br>3<br>Consensus<br>5<br>3<br>4<br>1<br>7<br>6<br>Incentive<br>1/3<br>1/4<br>1/4<br>1/7<br>1<br>1/2<br>Scale mode<br>1/2<br>1/3<br>1/3<br>1/6<br>2<br>1 |



- Build the comparative matrix of attributes based on hierarchical levels of the sub-criterion. 

- Normalize the membership matrix. 

- Establish the worst alternative and best alternative. 

**Table 11** 

||Random-generated consistency|Random-generated consistency|Random-generated consistency|Random-generated consistency|Random-generated consistency|index.||||||
|---|---|---|---|---|---|---|---|---|---|---|---|
||n|1|2|3|4|5|6|7|8|9|10|
||RI|0|0|0.58|0.9|1.12|1.24|1.32|1.41|1.45|1.49|



- Calculate the similarity value and rank the alternatives’ scores. 

Stage 5 Compose and synthesize to evaluate the results of each blockchain platform. 

- Calculate the averaged weighting of each blockchain platform by using AHP, FAFP, TOPSIS respectively. 

Stage 6 Consistent checking to determine the evaluation results. 

- Get the final level of each blockchain platform by using AHP, FAFP, TOPSIS respectively. 

- Choose candidates with same levels. 

- Determine the best fitting decision with the highest level in the candidates. 

## _5.2. Research limitations_ 

The main limitations of this research can be regarded as the obtaining opinion of insufficient experts, and they may lack knowledge to determine the applicability of all the criterions. Furthermore, the selection of designing factors in this study is based on the requirement analysis of multi-expert conversation system, and it is possible to consider and include more criterions as well as sub-criterions. 

## _5.3. Evaluation results_ 

**(1) Results regarding the weighted membership matrix** 

For building the AHP, FAHP, and TOPSIS, the opinions of ten artificial intelligence experts were obtained and used for 

applicability in evaluating each criterion and sub-criterion. We used a 0.1–0.9 rating scale based on the expert DELPHI method using four levels: very inappropriate, inappropriate, appropriate, and very appropriate. The results of all criteria regarding the membership matrix are shown in Table 9. 

Based on the results of applicability for all criteria, we can build the membership matrix of Ethereum, Fabric, Corda, and MultiChain according to the technical features of these blockchain platforms. 

|Ethereum is a decentralized platform using public chain. Its<br>storage and sharing mechanism are both on-chain. It is used to<br>build<br>a<br>single-chain<br>structure.<br>The<br>consensus<br>protocol<br>of|
|---|
|Ethereum is PoW and the incentive scheme is a token-based|
|model. It can use the Raiden Network or Plasma to reduce net-|
|work congestion and facilitate the speed of processing.|
|_Re_ =<br>⎡<br>⎢⎢⎢⎢⎢⎣<br>0_._0<br>0_._2<br>0_._2<br>0_._6<br>0_._6<br>0_._2<br>0_._1<br>0_._1<br>0_._3<br>0_._2<br>0_._2<br>0_._3<br>0_._9<br>0_._1<br>0_._0<br>0_._0<br>0_._3<br>0_._2<br>0_._2<br>0_._3<br>0_._0<br>0_._1<br>0_._2<br>0_._7<br>⎤<br>⎥⎥⎥⎥⎥⎦|



Fabric is partial centralization architecture with consortium chains and its original transaction and computing are used offchain (InterPlanetary file system) for storage. The consensus protocol also supports multi-chain. Fabric can provide a second layer solution to allow for off-chain scalability using both Fabric 

**Table 12** 

Fuzzy consistent matrix for criteria. 

|Blockchain platforms|Access authority|Chain structure|Storage and sharing|Consensus|Incentive|Scale mode|
|---|---|---|---|---|---|---|
|Access authority|[1,1,1]|[0.5000,0.6667,1]|[0.6667,1,1.5000]|[0.4000,0.5000,0.6667]|[0.6667,1,1.5000]|[0.5000,0.7500,1]|
|Chain structure|[1,1.5000,2]|[1,1,1]|[1,1.3333,2]|[0.6667,1,1.5000]|[1,1.5000,2]|[0.6667,1,1.5000]|
|Storage and sharing|[0.6667,1,1.5000]|[0.5000,0.7500,1]|[1,1,1]|[0.5000,0.6667,1]|[1,1.5000,2]|[0.6667,1,1.5000]|
|Consensus|[1.5000,2,2.5000]|[0.6667,1,1.5000]|[1,1.5000,2]|[1,1,1]|[2.5000,3,3.5000]|[2,2.5000,3]|
|Incentive|[0.6667,1,1.5000]|[0.5000,0.6667,1]|[0.5000,0.6667,1]|[0.2857,0.3333,0.4000]|[1,1,1]|[1,1.3333,2]|
|Scale mode|[1,1.3333,2]|[0.6667,1,1.5000]|[0.6667,1,1.5000]|[0.3333,0.4000,0.5000]|[0.5000,0.7500,1]|[1,1,1]|



token and user-defined non-economic incentive schemes. 

|_Rf_|=|⎡<br>⎢⎢⎢⎢⎢⎣<br>0_._1<br>0_._1<br>0_._0<br>0_._0<br>0_._0<br>0_._0|0_._2<br>0_._1<br>0_._0<br>0_._2<br>0_._0<br>0_._1|0_._2<br>0_._2<br>0_._8<br>0_._2<br>0_._1<br>0_._2|0_._5<br>0_._6<br>0_._2<br>0_._6<br>0_._9<br>0_._7<br>⎤<br>⎥⎥⎥⎥⎥⎦|
|---|---|---|---|---|---|



Corda is also a consortium blockchain platform based on partial centralization architecture and it supports off-chain computing. The multi-chain structure can be also designed in Corda. It has several notary clusters and each cluster can be deployed to a different consensus algorithm. We do not suggest building a digital currency or token when using Corda as it supports layer two to scale blockchain. 

**==> picture [115 x 64] intentionally omitted <==**

MultiChain is a platform that helps users build certain private chains used by organizations. The consensus protocol is PoW. Multichain is compatible with Bitcoin, which also uses on-chain storage and sharing, and supports many different tokens, including Lightning Network. 

**==> picture [117 x 64] intentionally omitted <==**

## **(2) Results regarding the implementation of AHP methodology** 

Step 1: Building the comparative matrix of attributes 

Based on the proposed AHP process, we constructed our selection hierarchy architecture of blockchain platforms as shown in Table 9. According to the requirement analysis of the knowledgebased conversation system collected from experts, the comparative matrix of attributes was established, as shown in Table 10, according to the nine-point scale. 

Step 2: Calculating the consistency ratio and verify AHP consistency 

To check the consistency of the matrix, first, we calculated the largest eigenvalue _**λ** max_ of the comparative matrix. Then, Eq. (7) was used to calculate the CI = 0 _._ 0621 and CR = 0 _._ 0501, with the random-generated consistency index, as shown in Table 11. 

**==> picture [250 x 45] intentionally omitted <==**

If CR _<_ 0.1, then the comparative matrix processes a better consistency. Otherwise, we need to adjust the comparative matrix processes a satisfied consistency. 

Step 3: Get the relative attribute weights 

## **Table 13** 

The L, M and U values for triangular fuzzy number. 

|Blockchain platforms|L|M|U|
|---|---|---|---|
|Access authority|0.0710|0.1240|0.2209|
|Chain structure|0.1015|0.1850|0.3313|
|Storage and sharing|0.0824|0.1492|0.2650|
|Consensus|0.1649|0.2774|0.4472|
|Incentive|0.0752|0.1261|0.2286|
|Scale mode|0.0793|0.1383|0.2485|



The relative contribution of each attribute to the target is determined by calculations made using the Eigenvector V = (0.1565, 0.3276,0.3732, 0.8427,0.0762,0.1139). Then each attribute weight is _wi_ = _vk/_[∑] _[k] i_ =1 _[v][k][,][wherek]_[ =][6][and][the][final][attribute][weight][is] _WAHP_ = ( _w_ 1 _, w_ 2 _, . . . , wk_ ). The result is _WAHP_ = _(_ 0 _._ 0828 _,_ 0 _._ 1733 _,_ 0 _._ 1975 _,_ 0 _._ 4458 _,_ 0 _._ 0403 _,_ 0 _._ 0602 _)[T]_ . **(3) Results regarding the implementation of FAHP methodology** 

Step 1: Building the comparative matrix of attributes 

This was the same as the AHP method and we established the comparative matrix of attributes as shown in Table 10. Step 2: Building the fuzzy consistent matrix 

The original comparative matrix of experts’ opinions was converted to a fuzzy consistent matrix using triangular fuzzy number. Table 12 shows the results of the fuzzy consistent matrix for the criteria. 

Step 3: Find the sum of every lowest value (L), middle value (M) and Upper value U values to be fuzzy synthesis values for triangular fuzzy number. 

Step 4: Get the normalized weight calculation after the comparison of fuzzy synthesis values (see Table 13) using the following degree of possibility calculation 

**==> picture [252 x 43] intentionally omitted <==**

Then we normalize of vector weight to get the final attribute weights. 

_W_ FAHP = (0 _._ 0885 _,_ 0 _._ 2128 _,_ 0 _._ 1452 _,_ 0 _._ 3311 _,_ 0 _._ 09810 _._ 1243) _[T] ._ 

## **(4) Results regarding the implementation of TOPSIS methodology** 

Step 1: Building the comparative matrix of attributes 

This was the same as the AHP and FAHP methods and we established the comparative matrix of attributes as shown in Table 10. 

Step 2: Normalizing the membership matrix 

We established the normalized evaluation membership matrix for all criteria, as shown in Table 14. 

Step 3: Establishing the worst alternative and best alternative 

Based on the above weighted normalized value, the highest and lowest values were considered as the best and worst solutions for each criterion. 

## **Table 14** 

The normalized matrix for criteria. 

|Blockchain platforms|Access|Chain|Storage and|Consensus|Incentive|Scale mode|
|---|---|---|---|---|---|---|
||authority|structure|sharing||||
|Access authority|0.1395|0.0663|0.0796|0.1780|0.3078|0.2598|
|Chain structure|0.5581|0.2650|0.1194|0.2967|0.4104|0.3897|
|Storage and sharing|0.4186|0.5301|0.2388|0.2226|0.4104|0.3897|
|Consensus|0.6977|0.7951|0.9552|0.8902|0.7182|0.7795|
|Incentive|0.0465|0.0663|0.0597|0.1272|0.1026|0.0650|
|Scale mode|0.0698|0.0883|0.0796|0.1484|0.2052|0.1299|



## **Table 15** 

|**Table 15**||||
|---|---|---|---|
|The evaluation|results by using three measurements.|||
|Blockchain|AHP|FAHP|TOPSIS|
|Platforms||||
|_Be_|[0_._58_,_0_._15_,_0_._09_,_0_._18]|[0.50,0.15,0.11,0.23]|[0.62,0.15,0.08,0.15]|
|_Bf_|[0_._03_,_0_._13_,_0_._31_,_0_._53]|[0.03,0.12,0.28,0.58]|[0.03,0.14,0.32,0.52]|
|_Bc_|[0_._07_,_0_._14_,_0_._32_,_0_._47]|[0.06,0.15,0.30,0.49]|[0.08,0.14,0.32,0.47]|
|_Bm_|[0_._54_,_0_._15_,_0_._10_,_0_._21]|[0.45,0.15,0.12,0.27]|[0.57,0.14,0.09,0.20]|



## **Table 16** 

The candidates with same evaluated levels. 

|Blockchain|platforms|AHP|FAHP|TOPSIS|
|---|---|---|---|---|
|_Be_||1.87|2.05|1.76|
|_Bf_||3.34|3.39|3.35|
|_Bc_||3.19|3.22|3.2|
|_Bm_||1.98|2.19|1.92|



Best solution _C_[+] = (0 _._ 6977 _,_ 0 _._ 7951 _,_ 0 _._ 9552 _,_ 0 _._ 8902 _,_ 

0 _._ 7182 _,_ 0 _._ 7795) 

Worst solution _C_[−] = (0 _._ 1395 _,_ 0 _._ 0663 _,_ 0 _._ 0597 _,_ 0 _._ 1272 _,_ 

0 _._ 1026 _,_ 0 _._ 0650) 

Step 4: Calculating the similarity value and ranking the alternatives’ scores 

The distance of each normalized weighted value from the best and worst solutions was calculated according to Eq. (5). Then rank the alternatives’ scores were determined by the sum of the distance values from the best and worst solutions. Finally, we normalized the alternatives’ scores and obtained the final result: 

_WFTOPSIS_ = (0 _._ 0799 _,_ 0 _._ 1836 _,_ 0 _._ 1999 _,_ 0 _._ 5013 _,_ 0 _,_ 0 _._ 0353) _[T] ._ 

## **(5) Results regarding the implementation of Composing and synthesizing** 

By using AHP, FAHP and TOPSIS, the evaluation results of Ethereum, Fabric, Corda and Multichain are calculated by weighted average method respectively as Table 15: 

## **(6) Results regarding the implementation of Consistent Checking** 

Based on the above calculated results, the Eq. (7) is used to judge and determine the selection of blockchain platforms, and the final evaluation levels will be show in Table 16. 

Thus, based on the above results, Ethereum and Multi-chain gets all same level 2 while Fabric and Corda gets all same level 3. Both can be candidates to be selected. 

## _5.4. Discussion and summary_ 

Based on the results of AHP, FAHP, and FTPOSIS, the comparison of the final weight for each criterion is shown in Fig. 4. In short, the results summarized in Table 17 are consistent with the rankings of AHP and FTOPSIS; however, there are some differences between the obtained results by AHP and FAHP, and these differences can be explained by the following points: 

**==> picture [167 x 119] intentionally omitted <==**

**Fig. 4.** Comparison of the final weight for each criterion using AHP, FAHP, and FTOPSIS. 

1. The calculation mechanism was different between AHP and FAHP and FTOPSIS. In classical AHP and FTOPSIS, the numerical values of variables are used for evaluating criteria; however, in the FAHP method, the decision-making of criteria was determined by fuzzy numbers. 

2. In classical AHP, the consistency process is used to measure the judgments, while fuzzy AHP does not require any consistency mechanism because of fuzziness. 

3. The characteristic of evaluations is another factor. Since probable deviation is used to integrate the decision-making process in FAHP, the evaluation results are a more natural process considering the uncertain characteristics of information, compared to the AHP method. 

In summary, from comparing the final weights for each criterion using AHP, FAHP, and FTOPSIS, it seems that the top three criteria are consistent with these three methods. Hence, the consensus protocol, chain structure, and storage and sharing are the most important considerations when selecting blockchain platforms for a conversation system. 

Furthermore, based on the evaluation results show in Table 16, there are no differences between the rankings of alternatives using these three methods. The results show that Hyperledger Fabric is the first choice for use in a conversation system compared to Ethereum, Corda, and Multichain. However, the blockchain platform can also be changed according to future requirements. 

**Table 17** 

## **Declaration of competing interest** 

The results of the difference between rankings of criteria. 

|Criterions|AHP|FAHP|FTOPSIS|dAHP−FAHP|dAHP−FTOPSIS|
|---|---|---|---|---|---|
|Access authority<br>Chain structure<br>Storage and sharing|4<br>3<br>2|6<br>2<br>3|4<br>3<br>2|−2<br>+1<br>−1|0<br>0<br>0|
|Consensus protocol|1|1|1|0|0|
|Incentive scheme|6|5|6|+1|0|
|Scale mode|5|4|5|+1|0|



The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **References** 

- [1] E. Black, A. Hunter, An inquiry dialogue system, Auton. Agents Multi-Agent Syst. 19 (2) (2009) 173–209. 

## _5.5. Future work_ 

For the knowledge-based conversation system, the domain knowledge maintenance is a collaborative process by multiple experts, which will be built through selected blockchain platform to make our conversation system more efficiently. Based on the above discussion and analysis, the Hyperledger Fabric blockchain ledger is used to establish as a knowledge base for our conversation system. To implement this, the following components can be explored in the future works. First, the Fabric multi-chain structure can be used to store different domain knowledge data with various membership, which provides flexible functions to identify authorities. Second, knowledge rules and afterwards can be generated and validated through blockchain smart contracts to guarantee transparency and non-tampering. Last but not the least, the reward scheme based on expert reputation can be utilized to motivate experts for knowledge base maintenance. In this way, the knowledge base can be implemented as a Fabric wrapper and can support more reliable conversation service. 

## **6. Conclusion** 

To assist decision-makers with selecting the best fitting blockchain platforms based on the requirement analysis of knowledge-based conversation system, the selection of blockchain platforms for knowledge-based conversation system was modeled as a decision-making problem with formulated multiple criteria regarding the identified requirement analysis and design aims. Moreover, a decision model using multiple measurements was designed to address this decision-making problem. By determining the consistent weighting of each criterion, AHP, FAHP, and FTOPSIS were combined to provide appropriate options for blockchain platform selection. In our study, we conducted four blockchain platforms to apply our established model and evaluate the results. From the compared and analyzed results, we can see the Hyperledger Fabric was recommended as the consistent best fitting blockchain platform for knowledgebased conversation system. However, this result may be updated when some new blockchain platforms in the market, and the knowledge regarding new blockchain platform can be quickly organized into our proposed decision-making steps modeling to be evaluated. And new measurement method can be also added into the presented decision model for further study in decisionmaking problems for blockchain platform selection. Furthermore, based on the selection of best fitting blockchain platform, the detailed implementation plan including chain structure, smart contract, and reward scheme, etc. will be designed and evaluated in the future. 

## **CRediT authorship contribution statement** 

**Wenli Yang:** Conception and design of study, Data acquisition and analysis, Drafting the manuscript, Revising the manuscript critically for important intellectual content. **Saurabh Garg:** Conception and design of study, Drafting the manuscript, Revising the manuscript critically for important intellectual content. **Zhiqiang Huang:** Data acquisition and analysis. **Byeong Kang:** Conception and design of study. 

- [2] J. Pei, P. Ren, M. de Rijke, A modular task-oriented dialogue system using a neural mixture-of-experts, 2019, arXiv preprint arXiv:1907.05346. 

- [3] A. Styhre, Knowledge Sharing in Professions: Roles and Identity in Expert Communities, Gower Publishing, Ltd, 2011. 

- [4] S. Wurster, et al., Born global market dominators and implications for the blockchain avantgarde, in: Corporate and Global Standardization Initiatives in Contemporary Society, IGI Global, 2018, pp. 86–115. 

- [5] L. Bach, B. Mihaljevic, M. Zagar, Comparative analysis of blockchain consensus algorithms, in: 2018 41st International Convention on Information and Communication Technology, Electronics and Microelectronics (MIPRO), IEEE, 2018. 

- [6] W. Yang, et al., A survey on blockchain-based internet service architecture: requirements, challenges, trends, and future, IEEE Access 7 (2019) 75845–75872. 

- [7] W. Viriyasitavat, D. Hoonsopon, Blockchain characteristics and consensus in modern business processes, J. Ind. Inf. Integr. 13 (2019) 32–39. 

- [8] M. Sáez, Blockchain-Enabled Platforms: Challenges and recommendations, Int. J. Interact. Multimedia Artif. Intell. 6 (3) (2020). 

- [9] A. Baldominos, F. Mochon, Towards Blockchain Intelligence, UNIV INT RIOJA-UNIR RECTORADO, AVENIDA DE LA PAZ, 137, LOGRONO, 26006, SPAIN, 2020. 

- [10] T.T.A. Dinh, et al., Blockbench: A framework for analyzing private blockchains, in: Proceedings of the 2017 ACM International Conference on Management of Data, 2017. 

- [11] G. Hileman, M. Rauchs, Global Blockchain Benchmarking Study, Vol. 122, Cambridge Centre for Alternative Finance, University of Cambridge, 2017. 

- [12] C. Maple, J. Jackson, Selecting effective blockchain solutions, in: European Conference on Parallel Processing, Springer, 2018. 

- [13] T.-T. Kuo, H. Zavaleta Rojas, L. Ohno-Machado, Comparison of blockchain platforms: a systematic review and healthcare examples, J. Amer. Med. Inform. Assoc. 26 (5) (2019) 462–478. 

- [14] S. Farshidi, et al., Decision support for blockchain platform selection: Three industry case studies, IEEE Trans. Eng. Manage. (2020). 

- [15] T.L. Saaty, Decision making with the analytic hierarchy process, Int. J. Serv. Sci. 1 (1) (2008) 83–98. 

- [16] D. Maček, D. Alagić, Comparisons of Bitcoin cryptosystem with other common internet transaction systems by AHP technique, J. Inf. Organ. Sci. 41 (1) (2017) 69–87. 

- [17] C. Kahraman, U. Cebeci, Z. Ulukan, Multi-criteria supplier selection using fuzzy AHP, Logist. Inf. Manage. 16 (6) (2003) 382–394. 

- [18] F. Karayazi, I. Bereketli, Criteria Weighting for Blockchain Software Selection using fuzzy AHP, in: International Conference on Intelligent and Fuzzy Systems, Springer, 2020. 

- [19] H.-S. Shih, H.-J. Shyur, E.S. Lee, An extension of TOPSIS for group decision making, Math. Comput. Modelling 45 (7–8) (2007) 801–813. 

- [20] M. Çolak, et al., A multi-criteria evaluation model based on hesitant fuzzy sets for blockchain technology in supply chain management, J. Intell. Fuzzy Systems 38 (1) (2020) 935–946. 

- [21] S. Liu, et al., Blockchain service provider selection based on an integrated BWM-entropy-TOPSIS Method under an intuitionistic fuzzy environment, IEEE Access 8 (2020) 104148-104164. 

- [22] G. Rathee, et al., CRT-BIoV: A cognitive radio technique for Blockchainenabled Internet of Vehicles, IEEE Trans. Intell. Transp. Syst. (2020). 

- [23] Y. Zheng, J. Ma, L. Wang, Consensus of hybrid multi-agent systems, IEEE Trans. Neural Netw. Learn. Syst. 29 (4) (2017) 1359–1365. 

- [24] J. Qin, et al., Recent advances in consensus of multi-agent systems: A brief survey, IEEE Trans. Ind. Electron. 64 (6) (2016) 4972–4983. 

- [25] R. Vaculin, et al., Real-Time Conversation Analysis System, Google Patents, 2019. 

- [26] M. Shibata, T. Nishiguchi, Y. Tomiura, Dialog system for open-ended conversation using web documents, Informatica 33 (3) (2009). 

- [27] J. Tang, et al., Target-guided open-domain conversation, 2019, arXiv preprint arXiv:1905.11553. 

- [28] P. Henderson, et al., Ethical challenges in data-driven dialogue systems, in: Proceedings of the 2018 AAAI/ACM Conference on AI, Ethics, and Society, 2018. 

- [29] H. Chen, et al., A survey on dialogue systems: Recent advances and new frontiers, Acm Sigkdd Explor. Newsl. 19 (2) (2017) 25–35. 

- [30] S. Rahmadika, D.R. Ramdania, M. Harika, Security analysis on the decentralized energy trading system using blockchain technology, J. Online Inform. 3 (1) (2018) 44–47. 

- [31] J. Eberhardt, S. Tai, On or off the blockchain? Insights on off-chaining computation and data, in: European Conference on Service-Oriented and Cloud Computing, Springer, 2017. 

- [32] T. Hepp, et al., On-chain vs. off-chain storage for supply-and blockchain integration, it-Inf. Technol. 60 (5–6) (2018) 283–291. 

- [33] A. Chauhan, et al., Blockchain and scalability, in: 2018 IEEE International Conference on Software Quality, Reliability and Security Companion (QRS-C), IEEE, 2018. 

- [34] S. Kim, Y. Kwon, S. Cho, A survey of scalability solutions on blockchain, in: 2018 International Conference on Information and Communication Technology Convergence (ICTC), IEEE, 2018. 

- [35] H.A. Linstone, M. Turoff, The Delphi Method, Addison-Wesley Reading, MA, 1975. 

- [36] K.D. Goepel, B. Performance, Comparison of Judgment Scales of the Analytical Hierarchy Process-A new approach, Int. J. Inf. Technol. Decis. Mak. 18 (2) (2019) 445–463. 

- [37] S. Aydin, C. Kahraman, A new fuzzy analytic hierarchy process and its application to vendor selection problem, J. Mult.-Valued Logic Soft Comput. 20 (2013). 

- [38] S. Fordoni, R. Melit, Multiple selections of Kashmir Tunnel risk by fuzzy multiple-criteria decision analysis, J. Bioinform. Intell. Control 4 (2015). 
