## SoK: Bridging Trust into the Blockchain A Systematic Review on On-Chain Identity 

Awid Vaziry _Service-centric Networking Technische Universit¨at Berlin_ Berlin, Germany vaziry@tu-berlin.de 

Kaustabh Barman _Service-centric Networking Technische Universit¨at Berlin_ Berlin, Germany kaustabh.barman@tu-berlin.de 

Patrick Herbke _Service-centric Networking Technische Universit¨at Berlin_ Berlin, Germany p.herbke@tu-berlin.de 

_**Abstract**_ **—Identifying users who are issuing transactions on the blockchain is required to adhere to the ongoing regulation of blockchain-based services and applications. This systematic review explores the current status, identifies research gaps, and outlines future research directions for establishing trusted and privacy-compliant identities on the blockchain (onchain identity). A systematic search term was applied across various scientific databases, collecting 2232 potentially relevant research papers. These papers were narrowed down in two methodologically executed steps to 98 and finally to 13 relevant sources. The relevant articles were then systematically analyzed based on a set of screening questions. The results of the selected studies provide insightful findings on the mechanisms of onchain identities. On-chain identities are established using zeroknowledge proofs, public key infrastructure/certificates, and web of trust approaches. The technologies and architectures used by the authors are also highlighted. Trust has emerged as a key research gap, manifesting in two ways: firstly, a gap in how to trust the digital identity representation of a physical human; secondly, a gap in how to trust identity providers that issue identity confirmations on-chain. Potential future research avenues are suggested to help fill the current gaps in establishing trust and on-chain identities.** 

_**Index Terms**_ **—Blockchain, Distributed Ledger Technology, On-Chain, Trust, Identity Management, Systematic Literature Review, SoK** 

## I. INTRODUCTION 

With the increasing adoption and institutionalization of blockchain applications and decentralized finance services, regulators require proof of user identities to prevent fraud and maintain the integrity of financial markets. Beyond privacy and data protection concerns, there is the broader challenge of bridging identity representations and making them available on-chain [1], [2]. 

Blockchain transactions and consumer-facing decentralized finance applications have gained interest in recent years. However, current blockchain applications often do not comply with existing financial regulations, such as antimoney laundering (AML), know-your-customer (KYC), and counter-terrorist financing (CTF) rules. Alignment with these regulatory frameworks requires blockchain-based verification of user identities. Such alignment is crucial to prevent potential bans on blockchain applications and foster broader adoption and innovation [1], [3]. This study aims to work towards the development of a trusted and usable on-chain identity 

solution. These on-chain identity solutions typically combine and integrate a variety of technologies, resulting in a hybrid of on-chain (blockchain-based) and off-chain (internet- and physical-world-based) components [4]. 

In this paper, we conduct a structured, systematic literature review using the _Guidelines for performing Systematic Literature Reviews in Software Engineering_ [5] and the _Preferred Reporting Items for Systematic Reviews and MetaAnalyses_ [6] (PRISMA) framework. The objective is to synthesize and critically evaluate the existing body of knowledge on blockchain-based on-chain identity solutions. This involves aggregating the technologies and architectures in use and identifying common patterns and shortcomings. Based on this analysis, we identify research gaps and suggest future avenues of investigation. Although there are existing reviews in the general context of using blockchain for identity management, this is, to the best of our knowledge, the first structured literature review with a specific focus on on-chain identities. 

## II. BACKGROUND 

This section introduces the preliminary concepts essential for comprehending the review and its outcomes. Consequently, blockchain, identity, and related technologies, like public key infrastructure and zero-knowledge proofs, are presented. Furthermore, other surveys are introduced, and this work is situated within the context of existing literature reviews on blockchain and identity. 

## _A. Blockchain_ 

A blockchain is a specific instance of a distributed ledger, a technological concept that enables the maintenance of appendonly transactional databases in a decentralized manner. Bitcoin first introduced the blockchain concept for securing a virtual currency without a middleman and without a central point of failure [7]. Nodes in the network bundle transactions into a block and agree on including a block by adhering to a consensus algorithm. In 2015, _Buterin et al._ introduced Ethereum, which offers Turing complete code execution, socalled smart contracts [8]. Blockchains can be categorized into two types based on network governance - permissionless and permissioned networks. In _permissionless blockchains_ , 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

anyone can run a node and secure the network. In most cases, honest behaving nodes are rewarded by economic incentives; dishonest behavior is usually punished or economically disadvantageous. Fault tolerance mechanisms ensure that a minority of nodes do not possess sufficient power to impair execution. To prevent spam and to reward node operators for computation and storage, users are typically required to pay to use the blockchain. In Ethereum, payment is denominated in a unit called gas, which limits how much can be spent in a transaction. This leads to resource constraints and limited throughput. _Permissioned blockchains_ are secured by a single entity or a consortium, and access to the network is usually restricted. Hyperledger-based distributed ledger technologies (DLTs) like Fabric and Aries are examples of permissioned blockchains [9]. 

This work will refer to both on-chain and off-chain approaches. In this discussion, we define these two terms as follows: 

- _On-Chain:_ refers to operations carried out or recorded on the blockchain [10]. Data is stored in the nodes of the distributed ledger network, can be read by anyone, and can be used by on-chain smart contracts. 

- _Off-Chain:_ describes any data, application, or computations that occur outside of the blockchain [10]. For example on a web server or a local machine. Off-chain data can usually not be accessed by a smart contract. 

## _B. Identity and Self Sovereign Identity_ 

The International Organization for Standardization defines identity as a ”set of attributes related to an entity.” Identity attributes are characteristic properties of an entity, such as a personal ID number, address, or MAC address. An identity can be represented by a credential (e.g., digital record, password, physical ID card) [11]. 

The shift to web3, with its decentralized and user-centric applications, has pushed the adoption of Self-Sovereign Identity (SSI) systems. SSI enables individuals to control their identities without a central authority, such as using a blockchain or a DLT for secure, tamper-proof identity verification [12]. 

The issuer-holder-verifier framework is commonly used for SSI and blockchain-based identity management and verification. The issuer creates and issues credentials to the holder. The holder controls the credentials and determines when and with whom to share them, while maintaining privacy and autonomy. The verifier validates the credentials, ensuring their legitimacy without requiring direct interaction with the issuer. [13]. 

## _C. Definition of on-chain identity_ 

The term ”on-chain identity” is not formally defined in academia and was first used by _Azouvi et al._ [14] in 2017 within a context that aligns with the scope of our work. To comply with data protection regulations (e.g. GDPR), our definition of on-chain identity explicitly excludes the storage 

of personal data on the blockchain, even in encrypted form. We define on-chain identity as the possession of an attestation (e.g., a token or claim) by an externally owned account (a user), which can be presented to an on-chain entity (a smart contract) to demonstrate that a trusted authority has verified the user’s identity. In simpler terms, to provide a blockchainbased proof that one’s identity has been verified in advance. 

## _D. Public Key Infrastructure_ 

Public Key Infrastructure (PKI) is a framework designed to manage digital keys and certificates, ensuring electronic communications security and authenticity. PKI is built upon several key components, such as trusted entities named Certificate Authorities (CAs), Registration Authorities (RAs), Digital Certificates, and Certificate Revocation Lists (CRLs). A certificate is a document that binds public keys to the identities of individuals or entities. X.509 certificates are the defined standard format for public key certificates within the PKI, containing the public key of the certificate owner, identifying information, and the digital signature of the issuing CA [15]. 

## _E. Zero-Knowledge Proofs_ 

A zero-knowledge proof (zkp) is a cryptographic technique that enables one party (the prover) to prove to another party (the verifier) that a statement is true without revealing any additional information. Some basic principles of zkps as laid down by _Goldwasser et al._ [16] are as follows: 

- _Completeness:_ A truthful prover can convince the verifier if the statement is true. 

- _Soundness:_ If the statement is false, no cheating prover can convince the verifier of its truth. 

_• Zero-Knowledge:_ If the statement is true, the verifier learns nothing other than the statement is true. A zero-knowledge succinct non-interactive argument of knowledge (zk-SNARK) is a zkp with the additional properties of non-interactivity and succinctness. A non-interactive zeroknowledge proof (NIZK) eliminates the need for interaction between the prover and the verifier by generating a concise proof that the verifier can verify independently. The concept of succinctness implies that proofs are smaller in size than the propositions they prove [17]. 

## _F. Related Work_ 

Related literature reviews on the intersection of blockchain and identity can be divided into two categories. First, specialized surveys focusing on specific areas, such as blockchain and identity for healthcare [18] or for the Internet of Things [19]. Second, general surveys aimed at collecting all available literature. _Liu et al._ [2] and _Ahmed et al._ [20] provide such general surveys in 2020 and 2023, respectively. _Ahmed et al._ concludes that blockchain technology can significantly improve identity management systems by enhancing security, privacy, and user control. However, it also notes that integration is still in its early stages. Our work is unique due to the focus on on-chain verifiable 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

identities, i.e., identities made for use on the blockchain. To the best of our knowledge, this is the first review of its kind. 

transactions, initiating trust is challenging. The question arises: Has the real-world owner of this identity approved its use on the blockchain? 

## III. METHOD OF LITERATURE REVIEW 

This review employs a systematic approach to retrieve, filter, and select relevant publications. This approach is primarily based on the updated PRISMA guidelines by _Page et al._ [6]. PRISMA is renowned for its efficiency in enhancing the quality and transparency of review reporting. To refine the method, the eight-step _Guide to Conducting a Systematic Literature Review of Information Systems Research_ , as proposed by _Okoli et al._ [21] is integrated as well as the _Guidelines for performing Systematic Literature Reviews in Software Engineering_ as proposed by _Kitchenham et al._ [5]. This set of guidelines covers the planning, conducting, and reporting phases of systematic literature reviews (SLRs). Including the development of a review protocol, the identification and selection of studies, the assessment of study quality, the extraction and synthesis of data, and the reporting of findings. Both guidelines highlight the importance of pre-planning the review, primarily through creating a thorough review protocol. The protocol ensures the accuracy and significance of the results by minimizing ad-hoc decisions. 

The review methodology is structured to provide a comprehensive understanding of the procedure. The subchapters are organized according to the PRISMA framework, which details the ( _i_ ) search process, ( _ii_ ) selection processes, ( _iii_ ) eligibility check, and ( _iv_ ) screening procedures. The systematic methodology will be critically evaluated. The results are presented at a high level in the following chapter IV. These results are thoroughly analyzed and discussed in the discussion chapter V. 

## _A. Objective definition and planning of the Review_ 

The primary goal of this systematic literature review is to determine the current state of on-chain verifiable identities. Therefore, this work systematically collects, evaluates, and synthesizes the findings from existing research on blockchainbased identity solutions. The analysis emphasizes the technologies used to identify and categorize components to establish identities, focusing on whether these components are implemented on-chain or off-chain. Three research questions, which serve as a framework for the review, are developed based on the current state of the art and questions deemed relevant for the review. Currently known blockchain identity protocols have at least some logic being computed or stored off-chain. The first research question [ **RQ1** ] explores the current state of knowledge regarding the realization of onchain identities concerning implementing on-chain logic. The second research question [ **RQ2** ] further investigates the use of additional algorithms or technologies, aiming to identify patterns of approaches deemed useful by different researchers. The final research question [ **RQ3** ] explores establishing trust in the system. Since the blockchain operates on data already present on the blockchain or new data published during 

- **RQ1** Which components of blockchain-based identity solutions are currently implemented on-chain versus off-chain, and what are the underlying reasons for their respective implementations? 

- **RQ2** What additional technologies, algorithms, and patterns are frequently used for on-chain and off-chain components of blockchain-based identity solutions, and what criteria or considerations guide their selection? 

- **RQ3** How is trust established within a solution, specifically addressing credential issuance and the integration of realworld data/identities into on-chain environments? 

## _B. Step 1 - Search Process_ 

The initial step in the PRISMA process is the identification of relevant literature. The primary search terms are _‘blockchain’_ and _‘identity management’_ . These are expanded by including synonyms, abbreviations, and domain-specific concepts. The case-insensitive search term employed is as follows: 

**Search Term** (" **blockchain** " OR " **distributed ledger** " OR " **on-chain** " OR " **decentralized finance** " OR " **defi** " OR " **smart contract** ") AND (" **identity management** " OR " **verifiable credentials** " OR " **self-sovereign identity** " OR " **SSI** " OR " **decentralized identifiers** " OR " **know your customer** " OR " **KYC** ") 

The researchers conducted a literature search across six databases recognized as comprehensive computer science and technical research repositories. One of those databases (arXiv) contains grey literature, which is acceptable by our guidelines and due to the multi-step filtering and quality check process. The search was executed on February 26, 2024. When the option to filter the search was available, the search was conducted on titles and abstracts, thereby enhancing the relevance of retrieved documents. The search was limited to literature published in 2017 and subsequent years, which aligns with significant advancements in the blockchain and Ethereum ecosystem [22]. The combined use of all databases yielded a total of **2,232** publications. For the most recent outcomes of the query, please consult the following hyperlinks: 

- IEEE Xplore: IEEE-1 

- SpringerLink: Springer-1 Springer-2 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

- ACM Digital Library: ACM-1 ACM-2 

- Wiley Online Library: Wiley-1 

- ScienceDirect: SD-1 SD-2 SD-3 SD-4 

- arXiv e-Print archive: arXiv-1 arXiv-2 

The search results from the _IEEE Xplore_ and _ACM Digital Library_ were exported as a spreadsheet (.csv file). Outcomes from the _SpringerLink_ and _ScienceDirect_ databases were exported as a BibTeX library (.bib file). The _ArXiv_ and _Wiley_ results were added manually. All results were processed with a data analysis, and manipulation library called pandas[1] . The data was cleaned and aggregated, and **31** duplicates were removed. 

All aggregated and cleaned results were exported into a new spreadsheet and subsequently enriched by ( _i_ ) document title, ( _ii_ ) abstract, ( _iii_ ) author names, ( _iv_ ) publication year, and ( _v_ ) Digital Object Identifier (DOI). 

**==> picture [237 x 295] intentionally omitted <==**

**----- Start of picture text -----**<br>
Records identified trough database search  (Total records = 2232)<br>SpringerLink Database (n = 1541)<br>IEEE Xplore Digital Library (n = 382)<br>ScienceDirect Database (n = 135)<br>arXiv e-Print archive (n = 118)<br>ACM Digital Library (n = 47)<br>Wiley Online Library (n = 9)<br>Records after duplicates removed<br>(n = 2201)<br>Titles and abstracts screened Records excluded<br>(n = 2201) (n = 2103)<br>Excluded with reasons<br>(Total = 83)<br>Full-text articles Quality / Short (n = 25)<br>assessed (n = for 98) eligibility SuperficialMinimalScopeBlockchain/ SolutionFocus (n(n(n= ==9)21)16)<br>No on-chain (n = 5)<br>Other reason (n = 7)<br>Excluded during in-<br>Eligible studies found<br>depth screening<br>(n = 15) (n = 2)<br>Studies included for quantitative and qualitative analysis<br>(n = 13)<br>Searching<br>Inclusion<br>Eligibility Check<br>Screening<br>**----- End of picture text -----**<br>


Fig. 1. Flowchart of the systematic literature review process: identification, screening, eligibility, and inclusion of articles. A white box implies inclusion and a grey box exclusion. 

## _C. Step 2 - Selection process_ 

The initial stage of the selection process serves to identify which literature should be included or excluded. Each of the 2,232 records is subjected to an individual screening based on its title and abstract. Following this, the literature is classified as either ”inclusion” or ”exclusion.” Papers marked 

**==> picture [87 x 10] intentionally omitted <==**

for ”inclusion” advance to the subsequent filtering phase, whereas those designated for ”exclusion” are removed from the review. In the event of uncertainty, the paper is included and subjected to further examination in the subsequent stage. The criteria for inclusion and exclusion at this stage are as follows: 

- **Inclusion Criteria** (must satisfy all) 

- 1) **Study Focus:** Papers that specifically focus on blockchain-based identity solutions with significant parts of the solution implemented on-chain. 

- 2) **Application Scope:** Works discussing or developing solutions applicable to on-chain applications within contexts such as decentralized finance (DeFi) or where a significant portion of the operational logic is blockchainbased, such as through smart contracts. 

- 3) **Publication Years:** Studies published earliest in 2017 up until today. 

- 4) **Language:** Papers written in English. 

## **Exclusion Criteria** (may not match one) 

- 1) **Beyond Scope:** Papers that do not address blockchainbased identity solutions. 

- 2) **Minimal Blockchain:** Papers that utilize the blockchain as a medium for realizing off-chain identities 

- 3) **No on-chain:** Solutions without on-chain applicability. 

- 4) **Superficial Solutions:** Papers that lack explanations for how the proposed solution is implemented technologically or architecturally. 

- 5) **Quality Issues / Short Papers:** Abstracts, posters, presentations, or without detailed methodology and results. 

- 6) **Language Limitations:** Studies not available in English. 

By applying these criteria to screen the titles and abstracts, a total of **98** papers are identified as relevant and marked for inclusion. 

## _D. Step 3 - Eligibility Check_ 

The objective of the eligibility check is to identify highquality and relevant articles. First, the full texts of all **98** advanced articles are obtained. Each article undergoes a thorough evaluation, including reading the title, abstract, and conclusion. If necessary, additional sections or the complete article are reviewed. The final decision on inclusion is made based on the criteria defined in the preceding step. The article will be flagged if a researcher is unsure whether to include or exclude it. In the next quality control step, all flagged articles and a randomly chosen subset will be reviewed and categorized again to confirm their eligibility (test-retest). After completing step 3, ”Eligibility Check,” 15 articles remain and are moved to the final step. The results are documented in the spreadsheet. 

## _E. Step 4 - Screening_ 

Each article is methodically analyzed during the screening process to extract relevant information. A set of screening questions (SQ) ensures a comprehensive and systematic 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

information retrieval. These screening questions are designed to address the initial research questions and make the included articles more comparable. The existing spreadsheet is extended with columns for the screening questions. Each publication undergoes multiple readings during the screening process to extract information and populate the spreadsheet with responses to the SQs. In some cases, the screening may be conducted by various researchers. 

- _F. Screening Questions_ 

## **Technical Aspects:** 

- **SQ1 DLT Technology:** Which Distributed Ledger Technology is used? 

- **SQ2 Additional Concepts:** Which supplementary concepts, standards, protocols, tools, or technologies are integrated, and for which components are these utilized? 

- **SQ3 Main Entities:** What are the main entities that make up the solution? 

- **SQ4 On-chain versus Off-chain:** Which components are onchain and off-chain? 

- **SQ5 Solution Maturity:** Is the solution theoretical, conceptually implemented, or deployed in a production environment? 

- **SQ6 Data Bridging Challenge:** What strategies are employed to address the challenge of making on-chain attestations based on real-world user data? 

During the screening process, two additional articles are marked for exclusion, leaving **13** articles relevant for the review. 

**==> picture [225 x 212] intentionally omitted <==**

**----- Start of picture text -----**<br>
13<br>11<br>9<br>7<br>5<br>3<br>1<br>2017 2018 2019 2020 2021 2022 2023<br>Year<br>Cumulative of Publications<br>Pulications<br>of<br>Number<br>**----- End of picture text -----**<br>


Fig. 2. Cumulative number of publications on on-chain verifiable identities over the years, showing increasing research interest. 

solutions on different blockchain platforms. This results in some articles being counted for multiple technologies. As an example, _Tadjik et al._ [23] uses a combination of Hyperledger Fabric and Hyperledger Indy in their protocol. Another example is _Eres et al._ [24], who utilize a combination of a public Ethereum network (Kovan testnet) and an additional permissioned Ethereum network (Hyperledger Besu). This article will be counted only once for ”Ethereum”, as both technologies are Ethereum networks. 

## _G. Outcome_ 

TABLE I 

After completing all the steps described above, 13 articles remain. These articles align with the initially defined scope of this structured literature review and are being further processed to address the initial research questions. During the filtration process, various observations are made. The wide subject of identity and blockchain is an area of active research with over 2,232 publications matching the search term, and 98 articles are relevant to our specific criteria. However, only sparse literature exists in the specific subfield of on-chain verifiable identities. Figure 2 illustrates the growth in research interest in on-chain verifiable identities over time. 

## IV. RESULTS 

The results of the systematic literature review are presented stepwise, with each previously introduced screening question addressed successively. The review data are presented, and the findings are assessed qualitatively. 

## **SQ1 DLT Technology.** _Which Distributed Ledger Technology is used?_ 

Table I provides an overview of the technology selected by different researchers. Some researchers combine two or more DLTs to realize their systems or implement their 

A SUMMARY OF THE DLTS UTILIZED OR PROPOSED IN THE EXAMINED RESEARCH ARTICLES. 

||**Technology**<br>Ethereum<br>(_9 articles_)|**References**<br>[24], [14], [25], [26],<br>[27], [28], [29], [30],<br>[31]|
|---|---|---|
||Hyperledger Indy|[23], [29]|
||(_2 articles_)||
||Hyperledger Fabric|[23], [32]|
||(_2 articles_)||
||Bitcoin|[14], [33]|
||(_2 articles_)||
||Not specifed<br>(_1 articles_)|[34]|



Ethereum is used in nine proposed architectures. The technologies Hyperledger Indy and Fabric are each used twice. Two of the solutions are based on Bitcoin. It is worth noting that these two Bitcoin-based solutions were among 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

the earliest publications in our review, having been released in 2017 and 2018. One proposed solution did not specify the type of blockchain technology used. 

## **SQ2 Additional Concepts.** _Which supplementary concepts, standards, protocols, tools, or technologies are integrated, and for which components are these utilized?_ 

This screening question examines the approaches researchers employ to realize on-chain identities. This analysis should provide insights and guidance for future researchers seeking to build upon these concepts. 

## TABLE II 

ADDITIONAL PROTOCOLS, STANDARDS, TOOLS, AND TECHNOLOGIES INTEGRATED INTO ON-CHAIN IDENTITY SOLUTIONS. 

|**Technology**<br>zk-proofs|**References**<br>[24], [27], [28],|
|---|---|
|(_7 articles_)|[29], [30], [31],|
|PKI / X.509 certifcates<br>(_3 articles_)|[34]<br>[14], [25], [26]|
|WoT / Trust propagation|[24], [14], [32]|
|(_3 articles_)<br>Other<br>(_4 articles_)|[27], [29], [31],<br>[34]|



The most frequently utilized technical concepts are zeroknowledge proofs (zkps) in the form of zk-SNARKs, which have been employed in seven published works. Four of the seven aforementioned publications, [27], [28], [30], [31] make use of the ZoKrates[2] library for proof generation. Three solutions employ PKI and/or X.509 certificates. A comparison of the two approaches by publication date reveals that PKI/X.509 papers were published between 2017 and 2021, while zkp publications were released between 2021 and 2023. Three proposed solutions employ trust propagation or web of trust-related concepts. Four papers explicitly reference the use of additional cryptographic schemes. Namely, Shamirs Secret Sharing by _Luong et al._ [31] and _Bruschi et al._ [27]. Camenisch-Lysyanskaya signatures are used by _Damgard et al._ [34] and _Muth et al._ [29]. In addition, the PointchevalSanders signature scheme and Pedersen commitments are employed by _Damgard et al._ [34] and MiMc block ciphers are utilized by _Bruschi et al._ [27]. 

## **SQ3 Main Entities.** _What are the main entities that make up the solution?_ 

The majority of the authors refer to an issuer-holder-verifier model. All mention a ”user”, ”holder”, ”account”, or a similar form of the subject of the credentials, who holds and controls 

**==> picture [86 x 10] intentionally omitted <==**

them. However, depending on the proposed framework, there are some modifications to this model. Similarly, every examined work mentions an issuer of a credential. The issuer acts as a trusted entity that verifies the subject’s identity and creates the digital credentials. Depending on the context, the issuer is referred to as a registrar, a trusted entity, an identity provider, an attribute manager, or sometimes a trust anchor. All proposed solutions involve some verifier or verification. However, not all researchers explicitly mention the verifier or use new entity names, such as a ”service provider” acting as a verifier. Other entities of interest are higher-level instances with extended rights within the system. Six frameworks [24]–[27], [32], [34] rely on the concept of trust anchors. Also referred to as a ”registry”, ”governance authority”, ”maintainer”, ”trusted third party”, or ”credible entity”. Other mentioned entities are a ”Judicial Authority” [27] and an ”Anonymity Revoker” [34], which are privileged entities able to reveal a user’s real-world identity. 

## **SQ4 On-chain versus Off-chain.** _Which components are onchain and off-chain?_ 

This screening question is answered by first categorizing the solutions into two main groups: those presenting off-chaincentric identity verification architectures and those presenting on-chain identity verification. Subsequently, the approaches taken by the researchers are grouped according to these categories. Off-chain-centric identity verification is proposed by five authors, while eight solutions propose partial or full on-chain verification of identities. 

**Off-chain-centric:** _Mukta et al._ [32] and _Azouvi et al._ [14] build their architecture on WoT based solutions with on-chain trust registries. In both solutions, trust evaluation and verification are handled by off-chain logic. _Buccafurri_ et al. [33] briefly discusses the verification process, which involves querying past registration transactions and validating the corresponding transaction submitter keys offchain. _Tavares et al._ [25] presents a certificate-based approach, where certificates are stored on-chain but validated off-chain. _Tadjik et al._ [23] uses Hyperledger Indy, which is a blockchain designed for the sole purpose of holding identities on-chain. However, the resulting data must be accessed and utilized offchain. Consequently, this approach is considered off-chaincentric verification. 

**On-chain-centric:** _Heiss_ [28], _Luong_ [31], _Bruschi_ [27], _Di Francesco Maesa_ [30], _Muth_ [29] and their respective colleagues present zk-SNARK-based on-chain verification methods. Their approach involves calculating an attributed/identity proof off-chain and submitting it to a smart contract for on-chain verification. Once the proof is successfully verified, the user may be added to a list of verified entities or marked as verified. _Jun et al._ [24] employ a combination of a public and a private ledger. They propose a WoT-based approach, storing a trust graph on-chain in their private, access-restricted blockchain. Trust calculations, identity verification, and most other 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

calculations are performed off-chain, with a non-interactive zero-knowledge proof published on the public ledger to enable on-chain credential verification. The system designed by _Damgard et al._ [34] issues each account holder with an off-chain certificate. This certificate is obtained from an identity provider and can be used by account holders to generate new, publicly unlinkable accounts independently. The creation information is submitted on-chain after the off-chain generation of a new account. This information contains a zkp verifying the correctness of the account and confirming that an identity provider signed the attributes published by the user. The number of attributes the user wishes to include is at their discretion. The off-chain collaboration of a designated number of anonymity-revokers may revoke the anonymity of accounts. _Gallersdorfer et al._ [26] employs a certificate-based approach, wherein X.509 certificates are stored and verified on the blockchain. The logic involves creating, modifying, retrieving, and revoking certificates and managing the relationships between certificate chains. 

## **SQ5 Solution Maturity.** _Is the solution theoretical, conceptually implemented, or deployed in a production environment?_ 

All proposed systems except one were designed and deployed as a conceptual implementation (proof-of-concept). Only _Damgard et al._ [34] proposed a theoretical system without an actual implementation. Some researchers tested the deployment time. However, deployment time on public ledgers is of minor relevance due to the direct correlation with the gas price set by the deployer. In addition, most systems do not face a time constraint but rather a cost constraint. For example, the solutions of _Bruschi et al._ [27] and _Muth et al._ [29] consume several million gas units for on-chain proof verification. The solution of _Heiss et al._ [28] consumes about 600k[3] gas for proof verification, which is achieved by off-chain pre-processing of the identity. However, the deployment costs of the verifier smart contract are not mentioned. _Gallersdorfer et al._ [26] consumes 500k - 1.5M gas units for on-chain PKI certificate submission. 

**SQ6 Data Integration Challenge.** _What strategies are_ 

## _employed to address the challenge of making on-chain attestations based on real-world user data?_ 

Implementing on-chain identity for real-world actors necessarily involves using real-world data for on-chain attestations (e.g., ensuring an identity) based on this data. Consequently, there is a need for trust in the data. This raises the question of how identities are verified and how the root trust of blockchain actors is guaranteed. Three distinct approaches could be observed, researchers: ( _i_ ) do not address this question at all [25], [28]. ( _ii_ ) introduce some trust anchor or trusted authority without further mentioning the implementation [23], [31]. ( _iii_ ) use a governmental digital 

> 3About 30 USD with a gas price of 15 GWei and an Ethereum price of 3500 USD 

id as a root of trust [25]. Nevertheless, the establishment of trust for identity documents and on-chain entities within those protocols remains an unresolved question. 

## V. DISCUSSION 

The systematic screening of the literature revealed patterns and trends in establishing on-chain identities. It can be seen that a majority of solutions are implemented using Ethereum-based ledgers. The initial identity verification of a new user and most cryptographic computations, such as key exchange, hashing, proof generation, certificate creation, and similar setup steps, are conducted off-chain. The most widely used on-chain components involve some level of identity or credential registration. Entities participating in the system may also be registered on-chain. Eight solutions allow onchain, smart contract-based verification of an identity claim [ **RQ1** ]. All but one implemented a proof of concept for their solution, and various researchers tested their deployment. The most prominent technologies for realizing some form of onchain identity verification are non-interactive zero-knowledge proofs, which were employed by seven researchers, as well as PKI or WoT solutions, which were present in three architectures, respectively [ **RQ2** ]. Most solutions follow a model with the issuer, holder, and verifier as the primary actors within the system. The most common gap identified is a lack of explanation in incorporating initial trust into the system. This lack of trust manifests in two different ways. First, it is necessary to determine how the issuer of an on-chain identity can be trusted. Second, it is essential to investigate how the user can verify their ”real-world identity” to the issuer [ **RQ3** ]. 

Ethereum was chosen for 70% of all solutions, possibly due to its established smart contract functionality, comprehensive developer tools, and mainstream adoption. However, due to the high cost of on-chain computation and storage, all authors outsource most of their protocol off-chain. The most notable distinction between the solutions under consideration can be observed in the on-chain credential verification phase, where zkps, WoT, or PKIs are employed on-chain. However, no solution would be suitable for a live Ethereum mainnet implementation. The reasons for this are as follows: ( _i_ ) high gas costs that make a solution economically infeasible ( _ii_ ) unrealistic trust assumptions, where the question of initial trust in the system is unsatisfactorily answered, and ( _iii_ ) the missing consideration of how real-world identities are bridged into the blockchain. 

The research field of blockchain combined with identity management is an area of interest, with a total of 98 articles generally corresponding to the specific topic. However, only 13 articles were suitable for this research, indicating a niche with increasing publications over the last years. A comprehensive overview of the state of the art, technologies used, and potential future developments in on-chain identity solutions was presented. For future research, several distinct research directions can be proposed. First, zkps proof sizes and computations are currently infeasible for on-chain applications. Therefore, further research is needed to make 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

zkps more efficient at the algorithmic level. An alternative approach would be to enhance the integration of zkps within the blockchain protocol. Secondly, the trust issue represents a significant challenge that requires further investigation. The lack of trusted mechanisms on public blockchains has resulted in the absence of trusted anchors that can verify new entities. Third, the next trust issue is the privacycompliant bridging of real-world human identities into the digital space and possibly on-chain. Most researchers have not explicitly explained how trust or identities are transferred to the blockchain. Therefore, we recommend additional research focusing on the certification of blockchain actors, on-chain trusted entities, or other novel approaches to introduce trust anchors on permissionless distributed ledgers. 

## VI. CONCLUSION 

This systematic literature review investigated the establishment of blockchain-based (on-chain) identities and synthesized existing research to identify prevailing trends, gaps, and implications. On-chain identity was defined as the on-chain attestation of a verified identity without storing personal data on the blockchain. Articles were included in the review when a significant part of the logic was implemented on the blockchain or when identities could be verified by the on-chain logic. The research area of on-chain identity is crucial due to the ongoing regulation and institutionalization of blockchain services and applications. This necessitates the identification of entities issuing transactions on the blockchain to comply with current laws and work towards a decentralized but legally compliant blockchain space. 

This literature review utilized a rigorous and structured approach, following the PRISMA framework, and using _Okoli’s_ , and _Kitchenham’s_ guidelines to minimize subjectivity and bias. Quality checks were incorporated to ensure the integrity of the data. It should be noted that this study has some limitations. Firstly, subjectivity was introduced during the formation of the search term, the definition of the inclusion criteria, and the decision of which studies to include. Secondly, the literature included is insufficient for quantitative claims but well-suited for qualitative analysis. 

Most included articles follow an issuer-holder-verifier model for establishing identities. Ethereum and Hyperledger (Fabric and Indy) are the blockchain technologies most commonly used. Further utilized technologies are zkps, PKI / X.509 certificates, and WoT approaches. A notable absence in the literature is an examination of the mechanisms by which trust of on-chain actors is established. Another significant gap in the literature is the identification and digitization of human identity data, which must be trusted and compliant. 

Future research should investigate methods to reduce the deployment and usage costs of technologies deemed useful for the establishment of on-chain identities. For example, enhancing the efficacy of zero-knowledge proofs and on-chain certificates could prove beneficial in this regard. Potential avenues for exploration include enhancing the algorithms or extending the blockchain protocol with dedicated zkp or 

certificate management functionality. The most prominent gap is the lack of trust in on-chain entities and, therefore, in digital data and attestations published on the blockchain. Further research is required to address the dual lack of on-chain trust. It is necessary to investigate mechanisms to identify and verify real-world humans and legal bodies in order to establish trust in their on-chain representations, particularly in their on-chain identity. Additionally, research on algorithms, patterns, and architectures is needed to establish trusted on-chain actors that can act as a root of trust. 

## REFERENCES 

- [1] C. Wronka, “Financial crime in the decentralized finance ecosystem: new challenges for compliance,” _Journal of Financial Crime_ , vol. 30, no. 1, pp. 97–113, Jan. 2023. 

- [2] Y. Liu, D. He, M. S. Obaidat, N. Kumar, M. K. Khan, and K.-K. Raymond Choo, “Blockchain-based identity management systems: A review,” _Journal of Network and Computer Applications_ , vol. 166, p. 102731, Sep. 2020. 

- [3] T. Yuyama, K. Katayama, and P. Brigner, “Proposal of principles of defi disclosure and regulation,” in _Financial Cryptography and Data Security. FC 2023 International Workshops_ . Springer Nature Switzerland, 2024, pp. 141–164. 

- [4] L. Lesavre, “A Taxonomic Approach to Understanding Emerging Blockchain Identity Management Systems.” 

- [5] B. A. Kitchenham and S. Charters, “Guidelines for performing Systematic Literature Reviews in Software Engineering,” Keele University, Tech. Rep. EBSE 2007-001, Jul. 2007, backup Publisher: Keele University and Durham University Joint Report. 

- [6] M. J. Page, J. E. McKenzie, P. M. Bossuyt, I. Boutron, T. C. Hoffmann, C. D. Mulrow, L. Shamseer, J. M. Tetzlaff, E. A. Akl, S. E. Brennan, R. Chou, J. Glanville, J. M. Grimshaw, A. Hr´objartsson, M. M. Lalu, T. Li, E. W. Loder, E. Mayo-Wilson, S. McDonald, L. A. McGuinness, L. A. Stewart, J. Thomas, A. C. Tricco, V. A. Welch, P. Whiting, and D. Moher, “The PRISMA 2020 statement: an updated guideline for reporting systematic reviews,” _BMJ_ , p. n71, Mar. 2021. 

- [7] S. Nakamoto, “Bitcoin: A peer-to-peer electronic cash system,” 2008. 

- [8] V. Buterin _et al._ , “A next-generation smart contract and decentralized application platform,” _white paper_ , vol. 3, no. 37, pp. 2–1, 2014. 

- [9] S. D. Palma, R. Pareschi, and F. Zappone, “What is your Distributed (Hyper)Ledger?” in _2021 IEEE/ACM 4th International Workshop on Emerging Trends in Software Engineering for Blockchain (WETSEB)_ . Madrid, Spain: IEEE, May 2021, pp. 27–33. 

- [10] J. Eberhardt and S. Tai, “On or off the blockchain? insights on off-chaining computation and data,” in _Service-Oriented and Cloud Computing: 6th IFIP WG 2.14 European Conference, ESOCC 2017, Oslo, Norway, September 27-29, 2017, Proceedings 6_ . Springer, 2017. 

- [11] “IT Security and Privacy — A framework for identity management Part 1: Terminology and concepts,” 2019. 

- [12] P. Dunphy and F. A. Petitcolas, “A first look at identity management schemes on the blockchain,” _IEEE security & privacy_ , vol. 16, no. 4, 2018. 

- [13] A. M¨uhle, A. Gr¨uner, T. Gayvoronskaya, and C. Meinel, “A survey on essential components of a self-sovereign identity,” _Computer Science Review_ , vol. 30, pp. 80–86, 2018. 

- [14] S. Azouvi, M. Al-Bassam, and S. Meiklejohn, “Who Am I? Secure Identity Registration on Distributed Ledgers,” in _Data Privacy Management, Cryptocurrencies and Blockchain Technology_ , 2017, vol. 10436, pp. 373–389. 

- [15] D. Cooper, S. Santesson, S. Farrell, S. Boeyen, R. Housley, and W. Polk, “Internet x. 509 public key infrastructure certificate and certificate revocation list (crl) profile,” Tech. Rep., 2008. 

- [16] S. Goldwasser, S. Micali, and C. Rackoff, “The knowledge complexity of interactive proof-systems,” in _Providing sound foundations for cryptography: On the work of shafi goldwasser and silvio micali_ , 2019. 

- [17] E. Ben-Sasson, A. Chiesa, E. Tromer, and M. Virza, “Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture,” _Proceedings of the 23rd USENIX Security Symposium_ , 2014. 

- [18] B. Houtan, A. S. Hafid, and D. Makrakis, “A Survey on BlockchainBased Self-Sovereign Patient Identity in Healthcare,” _IEEE Access_ , 2020. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

- [19] R. Huo, S. Zeng, Z. Wang, J. Shang, W. Chen, T. Huang, S. Wang, F. R. Yu, and Y. Liu, “A Comprehensive Survey on Blockchain in Industrial Internet of Things: Motivations, Research Progresses, and Future Challenges,” _IEEE Communications Surveys & Tutorials_ , 2022. 

- [20] M. R. Ahmed, A. K. M. M. Islam, S. Shatabda, and S. Islam, “Blockchain-based identity management system and self-sovereign identity ecosystem: A comprehensive survey,” _IEEE Access_ , vol. 10, pp. 113 436–113 481, 2022. 

- [21] C. Okoli, “A Guide to Conducting a Standalone Systematic Literature Review,” _Communications of the Association for Information Systems_ , vol. 37, 2015. 

- [22] F. Sch¨ar, “Decentralized Finance: On Blockchain- and Smart Contractbased Financial Markets,” Mar. 2020. 

- [23] H. Tadjik, J. Geng, M. G. Jaatun, and C. Rong, “Blockchain Empowered and Self-sovereign Access Control System,” in _2022 IEEE International Conference on Cloud Computing Technology and Science (CloudCom)_ . IEEE, Dec. 2022, pp. 74–82. 

- [24] K. J. Eer, J. Diaz, and M. Kohlweiss, “Bottom-Up Trust Registry in Self Sovereign Identity,” in _Blockchain and Applications, 4th International Congress_ , 2023, vol. 595, pp. 423–433. 

- [25] M. Tavares, A. Guerreiro, C. Coutinho, F. Veiga, and A. Campos, “WalliD: Secure your ID in an Ethereum Wallet,” in _2018 International Conference on Intelligent Systems (IS)_ . IEEE, Sep. 2018, pp. 714–721. 

- [26] U. Gallersd¨orfer, F. Groschupp, and F. Matthes, “Mirroring Public Key Infrastructures to Blockchains for On-Chain Authentication,” in _Financial Cryptography and Data Security. FC 2021 International Workshops_ , vol. 12676. Springer Berlin Heidelberg, 2021. 

- [27] F. Bruschi, T. Paulon, V. Rana, and D. Sciuto, “A privacy preserving identification protocol for smart contracts,” in _2021 IEEE Symposium on Computers and Communications (ISCC)_ . IEEE, Sep. 2021. 

- [28] J. Heiss, R. Muth, F. Pallas, and S. Tai, “Non-disclosing Credential Onchaining for Blockchain-Based Decentralized Applications,” in _ServiceOriented Computing_ . Springer Nature Switzerland, 2022, vol. 13740. 

- [29] R. Muth, T. Galal, J. Heiss, and F. Tschorsch, “Towards Smart Contract-Based Verification of Anonymous Credentials,” in _Financial Cryptography and Data Security. FC 2022 International Workshops_ . Springer International Publishing, 2023, vol. 13412, pp. 481–498. 

- [30] D. Di Francesco Maesa, A. Lisi, P. Mori, L. Ricci, and G. Boschi, “Self sovereign and blockchain based access control: Supporting attributes privacy with zero knowledge,” _Journal of Network and Computer Applications_ , vol. 212, p. 103577, Mar. 2023. 

- [31] D. A. Luong and J. H. Park, “Privacy-Preserving Identity Management System on Blockchain Using Zk-SNARK,” _IEEE Access_ , vol. 11, pp. 1840–1853, 2023. 

- [32] R. Mukta, H.-Y. Paik, Q. Lu, and S. S. Kanhere, “CredTrust: Credential Based Issuer Management for Trust in Self-Sovereign Identity,” in _2022 IEEE International Conference on Blockchain (Blockchain)_ , Aug. 2022. 

- [33] F. Buccafurri, G. Lax, A. Russo, and G. Zunino, “Integrating Digital Identity and Blockchain,” in _On the Move to Meaningful Internet Systems. OTM 2018 Conferences_ , vol. 11229. Springer International Publishing, 2018, pp. 568–585. 

- [34] I. Damg˚ard, C. Ganesh, H. Khoshakhlagh, C. Orlandi, and L. Siniscalchi, “Balancing Privacy and Accountability in Blockchain Identity Management,” in _Topics in Cryptology_ , 2021, pp. 552–576. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 07:45:52 UTC from IEEE Xplore.  Restrictions apply. 

