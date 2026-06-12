International Journal of Accounting Information Systems 48 (2023) 100598 

## Contents lists available at ScienceDirect International Journal of Accounting Information Systems journal homepage: www.elsevier.com/locate/accinf 

## Accounting and auditing with blockchain technology and artificial Intelligence: A literature review 

Hongdan Han[*] , Radha K. Shiwakoti, Robin Jarvis, Chima Mordi, David Botchie 

_Brunel Business School, Brunel University London, UB8 3PH, United Kingdom_ 

|A R T I C L E I N F O|A B S T R A C T|
|---|---|
|_Keywords:_<br>Accounting<br>Auditing<br>Artificial intelligence<br>Blockchain<br>Review|This paper surveys the published work on how blockchain technology will impact accounting in<br>general, but AI-enabled auditing specifically. The purpose is to investigate how blockchain<br>technology can improve transparency and trust in accounting practice and how professionals can<br>use blockchain data to improve decision-making, based on the qualities of immutability, append-<br>only, shared, verified, and agreed-upon (i.e., consensus-driven) blockchain data. The multi-party|
||validation of blockchain protocols adds real-time trusted data for the AI systems used by auditors<br>to improve assurance and efficiency. This review summarizes four themes emerging from the|
||literature focusing on how blockchain technology has changed record-keeping in accounting:|
||event approach to accounting; real-time accounting; triple entry-accounting and continuous<br>auditing. The research interprets the findings using agency theory and stakeholder theory to|
||advance how using blockchain to mitigate information asymmetry and improve stakeholder<br>collaborations is understood. The investigation also summarizes the challenges and clarifies or-|
||ganizations’reasons to be cautious about adopting blockchain. Lastly, the study suggests that<br>future researchers use this study in two ways that enrich blockchain literature: first, to apply the<br>themes and answer the questions identified within this review to improve the business methods of|
||practitioners and policymakers; and second, to encourage stakeholders such as practitioners,|
||system designers/developers, and policymakers to collaborate in designing blockchain ecosys-|
||tems that suit accounting and auditing as they transform digitally.|



## **1. Introduction** 

Making companies’ systems digital has enabled them to adopt new technological tools to simplify business processes and transform business models to innovate their operations (Gomber et al., 2018) because they can increasingly access advanced computing power and large databases (de Sousa et al., 2019). Today, the world’s most valuable businesses are Internet-driven and platform-based (Iansiti and Lakhani, 2017). Academics, social media, industries, and governments spend much time attending to the digital forms of technology: blockchain, artificial intelligence (AI), big data, the Internet of things (loT), and cloud computing. These innovations change organizations and individuals greatly (Benlian et al., 2018), with blockchain specifically providing the foundation for what Tapscott and Euchner (2019) call an Internet of value that will fundamentally reshape society and its business. Blockchain, now regarded to be the fifth pillar of the IT revolution (Thakkar, 2019) is expected to become the foundational technology as the next-generation Internet 

- Corresponding author. 

_E-mail addresses:_ Hongdan.han@brunel.ac.uk (H. Han), Radha.Shiwakoti@brunel.ac.uk (R.K. Shiwakoti), Robin.Jarvis@brunel.ac.uk (R. Jarvis), Chima.Mordi@brunel.ac.uk (C. Mordi), David.Botchie@brunel.ac.uk (D. Botchie). 

https://doi.org/10.1016/j.accinf.2022.100598 

Received 3 March 2021; Received in revised form 25 April 2022; Accepted 14 November 2022 Available online 24 November 2022 

1467-0895/© 2022 The Authors. Published by Elsevier Inc. This is an open access article under the CC BY license (http://creativecommons.org/licenses/by/4.0/). 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

(Iansiti and Lakhani, 2017; Shermin, 2017). Since Nakamoto (2008) set the groundwork for what would become blockchain technology in 2008, the banking, financial, insurance, education, health care, and government sectors have been using blockchain technology to the extent that 10 % of the global GDP will be recorded and stored on blockchain by 2027 (World Economic Forum, 2015). PricewaterhouseCoopers (PwC) (2020) estimates blockchain could boost global GDP by 1.76 trillion US dollars by 2030. Deloitte’s, 2020 global blockchain survey indicates that organizations are more committed than ever to implementing blockchain in their business (Deloitte, 2020). With blockchain’s growing maturity, innovators are discovering new opportunities to create value and enhance trust and resilience to digital transformation by combining blockchain with other technology forms, notably, AI, IoT or cloud computing (Cuomo, 2020). 

## _1.1. Purpose of the paper_ 

This study reviews how blockchain technology can improve transparency and trust in accounting practice, and how its professionals can improve decision-making by using blockchain’s ability to provide immutable, append-only, shared, verified, and agreedupon (i.e., consensus-driven) data. This investigation involves determining how blockchain will impact accounting, particularly related to AI-enabled auditing and identifying themes that mark how blockchain will impact record-keeping and its validity in accounting. 

## _1.2. Motivations for this review_ 

Three observations motivated this review. First, much of blockchain literature remains focused on financial aspects in Bitcoin and its valuation/volatility, and how supply-chain management enables its transparency and traceability. There is little scholarly publication in the field of accounting and auditing relative to the growing body of blockchain research to identify emerging themes in what is being said about blockchain and how it will impact the accounting profession. Few researchers study how it is applied to both accounting and assurance since blockchain has gained mainstream attention in 2017. 

Second, the literature widely acknowledges that current accounting and auditing practices face challenges. For example, accounting is currently far from automated and entails labour-intensive tasks wherein data is manually added to spreadsheets or accounting software (Deloitte, 2016; Du et al., 2019; Martin, 2018; Vasarhelyi, 2012). Since accounting divisions centrally authorize Tan and what is written to accounting information system (AIS) databases, they are responsible for the validity of financial reports ( Low, 2019). Delays in financial reporting can particularly be subject to the age-old problem of “cooking the books” by some managers to fabricate or manipulate financial information to gain an unfair advantage over creditors or investors (Dong et al., 2018). Different organizations maintain their ledgers in different formats, thus needing to be extensively reconciled (Brown et al., 2016). Because accounting is opaque, auditors often spend much time gathering and validating transactions. Such paper-based procedures to check signed supporting documents are tedious, and thus potentially prone to human error and fraud (Hughes et al., 2019). Further, current enterprise resource planning systems (ERPs) lack multi-party validation (Cong et al., 2018). Therefore, current accounting and auditing work needs to improve but also respond to the digital transformation. 

Third, emergent blockchain applications in financial services, health care, supply chains, and government services motivate accounting academics and practitioners to become familiar with blockchain-based accounting transactions. Additionally, blockchain technology has the potential to address present challenges in accounting because it provides better transparency, traceability, timeliness, and evidence of tampering than other existing financial record-keeping systems (Yu et al., 2018). The Institute of Chartered Accountants in England and Wales (ICAEW, 2018) refers to blockchain as an accounting technology. Deloitte (2016) argues that blockchain is a game-changer in accounting because it secures the integrity of records by providing completely traceable audit trails that enable fully automated audits. Yermack (2017) asserts that blockchain has progressed financial record-keeping more than any other facility since double-entry bookkeeping was introduced centuries ago. Its innovative technology enables an accounting ecosystem that inherently works to validate transactions (Dai and Vasarhelyi, 2017). Because many industries are currently exploring different forms in which blockchain can be applied, a set of acceptable models and standards for its use will eventually emerge (Kokina et al., 2017). 

Therefore, this study poses two research questions: 1) what are the emerging themes in blockchain applications in accounting that explain how accounting will change under blockchain approach? 2) What points of caution do organizations need to note when adopting blockchain technology? 

## _1.3. Contributions of the study_ 

This research reviews the literature on blockchain applications relating to accounting, thus contributing to it in three primary ways. First, the literature review provides four categories of blockchain use (i.e., event approach to accounting; real-time accounting; tripleentry accounting, and continuous auditing). These categories relate to how blockchain’s impact on accounting improves trust in the industry generally and on auditing specifically using AI technology to improve business decision-making and efficiency. 

Second, this review interprets our findings using agency and stakeholder theories so that using blockchain technology can be better understood. We argue that blockchain technology can be a new means for organizations to mitigate business agency problems by revising power and control because it encourages those involved to collaborate using secure, shared, verified, and consensus-driven financial data. 

Third, this study also summarizes the challenges and elucidates the points of caution for blockchain adoption. Further, this review 

2 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

suggests future research to promote stakeholder collaboration to design blockchain systems that suit accounting and auditing, as blockchains are not one-size-fits-all solutions and can be contextual. 

## _1.4. Overview of the study_ 

The paper first presents its purpose and background motivation, before clarifying how the study contributes academically. This is followed by reviewing blockchain literature in Section 2 that defines blockchain, its main characteristics, different types, how they are currently applied, and the potential benefits in accounting practice. Section 2 also introduces the theories that underlie the study. Section 3 outlines the scope of the review and methods. Following on from this, Section 4 summarizes the review’s findings while Section 5 discusses the themes and how they contribute theoretically to the agency and stakeholder literature, and practically to the work of accountants and auditors. Lastly, this study outlines the points of caution regarding using blockchain applications and suggests how future research can contribute further to the accounting profession with advanced technology literature. This study aims to be useful to both scholars and practitioners. 

## **2. Literature review** 

This review overviews using blockchain as a new accounting tool to achieve secure, shared, verified, and agreed-upon (i.e., consensus-driven) record-keeping to improve transparency and trust in accounting. 

## _2.1. Definition of blockchain_ 

Blockchain is described as a type of distributed ledger technology (DLT) (Christie, 2018; Ølnes et al., 2017) or a type of financial technology (FinTech) (FRC, 2018; Chen et al., 2019; Goldstein et al., 2019; Gomber et al., 2018). Others view blockchain as a sequential database or a giant spreadsheet that surpasses the classical financial ledger by recording transactional information, secured by cryptography, and governed by a consensus mechanism (Hinings et al., 2018; Yermack, 2017). The variety of definitions of blockchain reflect how different disciplines interpret it from various perspectives (Xu et al., 2019), indicating that a single definition is distant (Sheldon, 2019). 

This study adopts the definition of blockchain from the Institute of Chartered Accountants in England and Wales (ICAEW): Blockchain is not a single technology, but rather a protocol— a way of doing things—for recording transactions. Unlike the Internet, in which data is shared, in a blockchain ownership can be transferred from one party to another. Blockchain is a desirable model for several reasons. For example, in a market with many transacting parties, it could remove the need to reconcile disparate ledgers. Being distributed between all users also eliminates outages and removes the cost of having to pay a central authority to maintain the accuracy of the ledger. Any participant in the ledger can trace all previous transactions, allowing for increased transparency and blockchain to self-audit. 

(ICAEW, 2018). 

In sum, the ICAEW (2018) refers to blockchain as an accounting technology for transferring the ownership of assets and maintaining a ledger of accurate financial information, where the constancy of a ledger derives from trust in the system that drives the record-keeping. 

## _2.2. Blockchain characteristics and typology_ 

The primary characteristics of blockchains are transparency, decentralization, immutability, tamper resistance, strong authentication, synchronized networks, and consensus (Iansiti and Lakhani, 2017; Pattison, 2017; Swan, 2015; Yermack, 2017). In other words, blockchain technology allows anything of value, not just finance, but also assets like intellectual property, health data, votes, and ideas to be transferred (Tapscott and Tapscott, 2016). 

Based on the intended audience, different generations of blockchain technology have been identified (Casino et al., 2019; Swan, 2015; Xu et al., 2019). Blockchain 1.0 enables digital cryptocurrency transactions. Blockchain 2.0 uses smart contracts. Blockchain 3.0 extends applications to areas beyond cryptocurrency and finance into government, health care, and supply chains. Blockchain 4.0 is for the joint use of blockchain and AI (Angelis and Ribeiro da Silva, 2019). 

Emphasizing the rights to access, and permission to validate transactions, blockchain technology can be set up differently so that different users are given different access rights, and to record, update and validate transaction records. Usually, public and private blockchains are distinguished by the scale of the network (i.e., how many nodes are involved), the permissions to join the network, and the approach to validating transactions (Coyne and McMickle, 2017; O’Leary, 2017). Ølnes et al. (2017) suggest that control, data ownership, privacy, and access describe dominant key design decisions that help organizations to understand which type of blockchain works best for their needs. 

A public and permissionless blockchain, like Bitcoin, includes many nodes, allows anyone to participate as a miner/validator, and allows anyone to view the underlying ledger. However, it is costly and takes time to reach a consensus because of the enormity of the distributed network. On the other hand, a private and permissioned blockchain (like those being developed now by consortia) has far fewer participating nodes, and potential users require approval to participate in mining/validating transactions and to view the underlying ledger. Because identities are known, less blind trust is required in private blockchains and private blockchains are thus comparatively faster and more cost-effective (lower risk of ex-post opportunism). This design can protect the privacy and 

3 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

## confidentiality of business data. 

Additionally, smart contracts are the most transformative blockchain application (Iansiti and Lakhani, 2017). They define the terms of a contract between parties using technical codes, and are self-enforcing and tamperproof because digital contracts are automatically executed (Cong and He, 2019). Smart contracts increase transparency for all participants within the network and lower the costs of contracting between parties (Moll and Yigitbasioglu, 2019). However, notably, it remains unclear who is responsible when smart contracts encounter problems or execute in unexpected ways. 

## _2.3. Emergent literature on blockchain_ 

In recent years, studies have examined uses of blockchain in FinTech, supply chains, and corporate governance (Goldstein et al., 2019; Gomber et al., 2018; Kumar et al., 2019; Yermack, 2017; Zachariadis et al., 2019; Hastig and Sodhi, 2020; Lumineau et al., 2021; Ziolkowski et al., 2020). Some have explored topics such as blockchain, smart contracts, cryptocurrency regulation and Bitcoin (Biais et al., 2019; Cong and He, 2019; Holub and Johnson, 2018; Yin et al., 2019). This is because blockchain, cloud computing, AI, and loT can support businesses to build dynamic capabilities for digital transformation (Warner and Wager, 2019¨ ). 

The following cases exemplify the advantages of blockchain’s various broad technological applications whereby it can: a) transform AI (Salah et al., 2019); b) technologically develop digital currencies, privacy assurance, cybersecurity, and smart contract security; c) achieve the scalability of data management and consensus protocols; d) develop the interoperability of incentives, governance, and legal ethics of ecosystems (Jie et al., 2020); e) enable business economically (Xu et al., 2019); f) generate value for most types of FinTech innovations including AI (Hua et al., 2019); g) enhance auditable and verifiable data management to simplify and secure the management of trusted information; and h) improve business, healthcare, loT, privacy, and data management within the multiple domains of supply chains (Casino et al., 2019). 

Risius and Spohrer (2017) propose a blockchain research framework based on three groups of activities (i.e., designing and featuring, measuring and evaluating, managing and organizing). Schmitz and Leoni (2019) identify the emerging themes: governance, transparency, trust, smart contract, blockchain-enabled audits, and the changing roles of accountants and auditors. Brennan et al. (2019) overview the implications of disruptive technology on three fundamental corporate governance activities (i.e., financial, external, and internal auditing). Moll and Yigitbasioglu (2019) describe the role of Internet-related technologies: cloud, big data, blockchain, and AI in shaping the work of accountants. 

## _2.3.1. Blockchain in accounting_ 

Accountancy organizations, namely, ICAEW, the Association of Chartered Accountants (ACCA), the Chartered Institute of Management Accountants (CIMA), the Chartered Institute of Public Finance and Accountancy (CIPFA), and the International Federation of Accountants (IFAC), all publish reports on their websites relevant to blockchain technology. For example, Deloitte, EY, KPMG, and PwC lead the initiative to incorporate blockchain into their businesses to cater for changing customer demands for blockchain transactions (O’Neal, 2019). Accordingly, Deloitte formed its Rubix division and launched a blockchain plug-and-play product (Leung, 2016; Palmer, 2019); EY introduced a blockchain analyzer platform to support audit teams’ reconciling data; PwC released cryptocurrency auditing software and updated its Halo tool for auditing, and KPMG has been working with Guardtime, Microsoft, _R_ 3, and Tomia to produce blockchain-based services (O’Neal, 2019). 

Tan and Low (2019) argue that blockchain technology affects the database engine of the accounting information system (AIS) by digitalizing current paper-based validations. The technology can securely store accounting data such as accounts payable and accounts receivable (Dai and Vasarhelyi, 2017) and can increase the efficiency of accounting for transactions (ICAEW, 2018). Deloitte (2017) and McWaters et al. (2016) identify ways in which blockchain technology addresses current accounting challenges. It can simplify operations, reduce transaction-settlement time and counterpart risk, minimize fraud, and improve regulation and capital liquidity (see Table 1). 

## **Table 1** 

Benefits of Blockchain-based Accounting. 

|Current accounting challenges|Value driver|Blockchain benefts|
|---|---|---|
|Manual documents|Operational<br>simplifcation/effciency|Digitize documents, increase effciency, reduce costs, reduce human error, automate<br>reconciliation.|
|Time-consuming process|Transaction settlement|Blockchain-powered smart contract enables contracts to execute automatically once pre-|
||time reduction|set conditions are met and facilitates real-time transactions.|
|Lack of mechanism to track|Counterpart risk reduction|Agreements are codifed and executed in a shared, immutable environment, forming an|
|transactions from different ledgers||audit trail.|
|Prone to fraud|Fraud minimization|Blockchain provides transparency, visibility, provenance, and immutable records, which|
|||enhances security. Any suspicious fund transfer will be observed and detected in real-|
|||time.|
|Regulatory complexity, costly to|Regulatory effciency|Provides faster and more accurate reporting by automating compliance processes through|
|organizations|improvement|a smart contract. It permits real-time monitory between regulators and regulated entities.|
|Intermediaries are involved in many|Liquidity and capital|Blockchain eliminates imbalance of information among market participants, increases|
|processes|improvement|transparency|



Sources: World Economic Forum (Deloitte, 2017:15; McWaters et al., 2016:19). 

4 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

## _2.4. Blockchain adds trust for AI-enabled auditing_ 

The prominent purpose of using blockchain technology for keeping accounting records is to create trust (Smith, 2019) and a trust network either with or without a trusted party involved (FRC, 2018). Blockchain collects validated pieces of information about the amount of a transaction, who it was paid to, and by whom, then hashes and adds the block to the existing chain (Fanning and Centers, 2016). Combining hash algorithms, private and public keys, and decentralized ledgers are what make blockchain powerful in modern Internet use (Hughes et al., 2019) because its immutability, traceability and visibility enable participants to view fully encrypted synchronized transactions (Deloitte, 2016; PwC, 2018). The distributed network, digital signature, and consensus validation rules have made blockchain secure and reliable (Boillet, 2017). According to the Financial Reporting Council, the trust derived from blockchain occurs because records are tamper-resistant and immutable, stemming from their being distributed and hashed. Like fingerprints, hashes are unique since every change, regardless of how minor, when adding information causes the hash to change from one unique identity to another because such changes mean its block is no longer the same. The consensus mechanism makes gaming blockchain difficult (FRC, 2018). The unique blockchain hash features offer a source of trust to create resilience (Deloitte, 2016). Additionally, the technology can boost information auditability and transparency (Ølnes et al., 2017). 

## _2.4.1. Infusing blockchain into AI_ 

A recent article published by IBM suggests, 

“If decisions and associated data points are recorded via transactions on a blockchain, the inherent attributes of blockchain will make auditing them much simpler. Blockchain is a key technology that brings trust to transactions in a network; therefore, infusing blockchain into AI decision-making processes could be the element needed to achieve the transparency necessary to fully trust the decisions and outcomes derived from AI” 

## (Cuomo, 2020, para 16). 

ICAEW (2018) argues that blockchain and other automation trends like machine learning will lead to more and more transactionallevel accounting being done automatically. In this study, AI refers to machine learning that offers auditors a new means of augmenting their analyses facility adding to their computer-assisted auditing tools and techniques (Shimamoto, 2018). Machine learning can assist auditors to recognize and apply patterns, deriving algorithms based on those patterns, and refining them based on feedback. AI can help auditors review documents efficiently by detecting anomalies and better evaluating risk. Auditors can use AI technologies to review general ledgers, tax compliance, audit work papers, data analytics, fraud detection, and decision-making (Munoko et al., 2020). Blockchain delivers trust and confidence in AI-based processes by enriching trust in their data, models, and analytics (Cuomo, 2020), and enables a more agile and precise auditing model that automates assurance (Dai and Vasarhelyi, 2017) and thus strengthens auditing. Auditing professions can benefit from AI technology by comparing corresponding accounting entries in blockchains recorded by each of the trading parties. This procedure is relatively easy because blockchain data is traceable and auditable. 

## _2.5. Theoretical perspectives of the study_ 

This review studies the impact of blockchain-enabled accounting generally and AI-enabled auditing specifically using both agency and stakeholder theories. This study relied on two conditions: that network participants are honest, and that raw data from either public or private chains is put into blockchains. It then explores how blockchain and AI tools were adopted in accounting and auditing to ensure trust in financial information. This involved lessening information asymmetry and enabling value to be created for CEOs, managers, shareholders, accountants, auditors, investors, policymakers, and other stakeholders to make commitments collectively and collaborate about them. 

## _2.5.1. Agency theory perspective_ 

Since Jensen and Meckling (1976) extended Berle and Means’s (1932) work and theorized the principal-agent relationships, corporate governance scholars have explored various internal and external monitoring and control mechanisms. The internal governance mechanisms primarily focus on boards of directors, board composition, board size, board independence, ownership structure, and managerial incentive mechanisms, whereas the external governance mechanisms cover issues related to the external market, laws and regulations. 

In accounting research, agency theory seems to be the standard approach to emphasize “the conditions of uncertainty that leads to potential information asymmetries between the executives who manage the firm and external investors” (Walker, 2013:448). Information asymmetry exists when information is held privately, withheld, or revealed strategically by managers to influence the outcome of a decision or transaction (Williamson, 1979). Internal managers are in the position of having more information that can be manipulated to maximize their interest at the expense of the principal (Godfrey et al., 2003). Information asymmetry has created ethical risks that triggered many global financial scandals (Lubatkin, 2005; Cuevas-Rodríguez et al., 2012). 

In this context, accounting and auditing practices need to mitigate information asymmetry in the interest of transparency and accountability. Therefore, this review argues that using blockchain and AI can provide a new technological means of controlling and monitoring accounting information to reduce information asymmetry and agency problems. This occurs because blockchain enables shared, verified and agreed-upon data and AI detects anomalies. Additionally, smart contracts automate procedures that can also reduce managerial manipulation and opportunistic behaviour. 

5 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

## _2.5.2. Stakeholder theory perspective_ 

Stakeholder theory recognizes that firms are part of a greater social system, and decisions cannot be made in isolation. The theory promotes an open and inclusive relationship with all stakeholders consisting of managers, directors, investors, employees, other companies, service providers, government, and society at large (Freeman, 1994; Freeman, 1984). 

Under current accounting practice, external financial information users are unable to observe the true transactions and accounting processes of a firm (Yu et al., 2018). Vasarhelyi (2012) argues that the accounting literature has largely focused on how users interpret financial reporting standards and market effects but fail to include the diverse stakeholders with various information needs under the current model. He suggests that accounting changes to a more disaggregated approach to disclosing information, a view with wide support (Moll and Yigitbasioglu, 2019; Yermack, 2017). Dai and Vasarhelyi (2017) propose a blockchain-enabled accounting ecosystem wherein managers, accountants, business partners, and investors can actively collaborate to verify transactions and enable organizations to serve broader interests. Because the distributed nature of blockchain technology can be more inclusive, this review argues that it can be a worthwhile means of promoting collaboration and interaction by diverse people within its extensive networks. Together with the use of AI technology, firms can promote an open and inclusive corporate culture to sharpen decision-making using multi-party verified and shared blockchain data. 

The literature previously reviewed shows that blockchain technology can address the challenges faced by current accounting practice by offering a new way of recording, updating, validating, and sharing data that includes auditing boosted by using advanced AI tools. This review investigated the emerging themes and findings relevant to blockchain applications in accounting to explain how it will change under blockchain approach and what points of caution organizations need to note when adopting the technology. The next section details our scope of review and method. 

## **3. Methodology** 

This study followed the review method used by de Bakker et al. (2019) that starts with manually screening a list of related articles from quality academic journals, followed by systematically searching relevant literature using keywords related to the study. Further, because blockchain technology is emerging, this study also searched relevant industry literature such as industry reports, blogs, and news on blockchain technology. This approach offered broad coverage of the blockchain literature, focusing particularly on how it might be applied to new ways of record-keeping in accounting and its auditing component. Fig. 1 presents the review steps involved with guidance from the Prisma diagram. 

**Fig. 1.** Structured systematic review steps. 

6 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

First, this review used the Academic Journal Guide 2018 and identified 4* and 3* quality journals from the accounting, finance, information management, and innovation fields. We searched “blockchain” inside each of the journals from 2017 to 2019 because blockchain began to gain mainstream attention in 2017 (Murray, 2018). Our initial search identified 11 A* journals that provided 24 papers (see Table 2). 

Following this, we systematically searched the relevant literature in Scopus, the largest abstract and citation database of peerreviewed literature, using the search terms, “blockchain” and “accounting”, “blockchain” and “auditing”, “blockchain” and “AI”, “AI” and “auditing”, “AI” and “accounting”. The search generated 1052 results. We added AI to the search because blockchain can add trust in data for AI systems through multi-party validations. AI has been used in the existing accounting information systems. These two technologies will yield substantial value. However, this investigation focused on the impact of blockchain in keeping accounting records. We refined the search results to include source type (Journals); language (English); and subject areas (Business, Management, Accounting, Economics, Econometrics and Finance). This stage refined the search results from 1052 to 317 articles. Combining both sets of papers (i.e., 24 & 317) yielded 341 articles that reflect a corpus of potentially relevant articles. 

Subsequently, the 341 search results were imported into Mendeley to examine each in more detail by looking at the title, keywords, and abstracts to determine whether they should remain in the dataset. We removed duplicates before each entry was assessed on its relevance to our investigation thus leaving a final dataset of 179. We used Nvivo software to code the reviewed literature into different categories, namely, blockchain benefits, prospects, implications, points of caution, and impact on accounting and auditing. We also categorized the type of analysis for each article before summarizing the themes (Table 3 overviews the journal papers). 

## **4. Results** 

The studies contained in our dataset (N = 179) are predominantly conceptual in nature, which is expected as blockchain research is still in an early stage, similar to the business, energy, and industrial strategy literature (e.g., BEIS, 2020; Toufaily et al., 2021). 

Additionally, this review found some accounting or information systems journals beginning to publish blockchain-related research frequently. For example, the International Journal of Accounting Information Systems (Alles and Gray, 2020; McCallig et al., 2019; Sogaard, 2021; Vincent et al., 2020; Wang and Kogan, 2018; Yen and Wang, 2021); Australian Accounting Review (Carlin, 2019; Karajovic et al., 2019; Schmitz and Leoni, 2019; Tan and Low, 2019); British Accounting Review (Brennan et al., 2019; Moll and Yigitbasioglu, 2019); Current Issues in Auditing (Sheldon, 2018, 2019); Journal of Emerging Technologies in Accounting (Coyne et al., 2016; Coyne and McMickle, 2017; Issa et al., 2016; Kokina et al., 2017; Marshall and Lambert, 2018; Raschke et al., 2018; Rozario and Thomas, 2019; Cong et al., 2018); Accounting Today (Antoinette, 2018; Boillet, 2017; Hood, 2018; Ketz, 2017; Pitter, 2018); Journal of Accountancy (Carlozo, 2017; Drew, 2018, 2019; Drew and Tysiac, 2020; Tysiac and Drew, 2018; Vetter, 2019); Journal of Information Systems (Dai and Vasarhelyi, 2017; Appelbaum and Nehmer, 2020; Sheldon, 2020), etc. 

Further, Fig. 2 summarizes four main themes in the literature derived from blockchain-enabled accounting and lists points of caution in adopting it. 

These four emerging themes also relate to how blockchain will impact accounting and auditing that uses AI tools. These themes include the event approach to accounting, real-time accounting, triple-entry accounting, and continuous auditing in accounting using blockchain tools. Auditing can be more efficient when using traceable and auditable blockchain data. 

## _4.1. Event approach to accounting_ 

Blockchain provides research opportunities for the event approach to accounting, as the technology improves access to real-time accounting data (Wu et al., 2019). Using blockchain technology to keep accounting records makes them more traceable and visible so 

**Table 2** 

Manually screened list of quality academic journals. 

|Journal|ABS|No. of|Keywords|Illustrative references|
|---|---|---|---|---|
|||Articles|||
|Review of Financial Studies|4*|4|Blockchain, FinTech, innovation|Biais et al. (2019),Chen et al. (2019),Cong and He (2019),|
|||||Goldstein et al. (2019)|
|Review of Finance|4|1|Blockchain, corporate governance|Yermack (2017)|
|Journal of Management|4|5|Information based transformation,|Benlian et al. (2018),Clemons et al. (2017),Giboney et al.|
|Information Systems|||blockchain, Fintech|(2019),Gomber et al. (2018),Yin et al. (2019)|
|Technovation|3|2|Innovation, disintermediation|Linton (2018),Urbinati et al. (2019)|
|Information and Organization|3|2|Digital innovation and transformation,|Hinings et al. (2018),Zachariadis et al. (2019)|
||||blockchain||
|Information Society<br>Journal of Strategic|3<br>3|2<br>3|Bitcoin, data governance<br>Blockchain, platform, digital|Holub and Johnson (2018),Winter and Davidson (2019)<br>Du et al. (2019),Shafei et al. (2019),Vial (2019)|
|Information Systems|||transformation||
|British Accounting Review|3|2|Cloud, big data, blockchain, and<br>artifcial intelligence (AI)|Brennan et al. (2019),Moll and Yigitbasioglu (2019)|
|Long Range Planning|3|1|Digital transformation (blockchain)|Warner and W¨ager (2019)|
|Decision Sciences|3|1|Blockchain|Kumar et al. (2019)|
|Harvard Business Review|3|1|Blockchain|Iansiti and Lakhani (2017)|
|**Total**||**24**|||



7 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

## **Table 3** 

Overview of Journal Papers In Our Dataset (N = 179). 

|**Table 3**<br>Overview of Journal Papers In Our Dataset (N=179).||
|---|---|
|Academic journals<br>Overall No. of articles|Types of analysis|
||Conceptual<br>Qualitative<br>Quantitative<br>Mixed|
|Technovation<br>2<br>Review of Financial Studies<br>4<br>Journal of Strategic Information Systems<br>3<br>Journal of Management Information Systems<br>5<br>Information Society<br>2<br>Information and Organization<br>2<br>Harvard Business Review<br>1<br>British Accounting Review<br>2<br>Long Range Planning<br>1<br>Decision Sciences<br>1<br>Review of Finance<br>1<br>Applied Sciences (Switzerland)<br>1<br>Australian Accounting Review<br>6<br>Business Lawyer<br>1<br>Computer Journal<br>2<br>Computer Law and Security Review<br>2<br>Current Issues in Auditing<br>4<br>Cyprus Review<br>1<br>Digital Communications and Networks<br>1<br>Foresight (Cambridge)<br>1<br>Future Generation Computer Systems<br>4<br>Future Internet<br>3<br>Government Information Quarterly<br>3<br>IEEE Access<br>4<br>IEEE Engineering Management Review<br>1<br>IEEE Network<br>1<br>IEEE Transactions on Computational Social Systems<br>1<br>Intelligent Systems in Accounting, Finance and Management<br>5<br>International Journal of Accounting Information Systems<br>7<br>International Journal of Communication Networks and Information Security<br>1<br>International Journal of Computers, Communications and Control<br>1<br>International Journal of Disclosure and Governance<br>1<br>International Journal of Distributed Sensor Networks<br>1<br>International Journal of Environmental Research and Public Health<br>1<br>International Journal of Recent Technology and Engineering<br>2<br>International Journal of Scientifc and Technology Research<br>1<br>Journal of Data and Information Quality<br>1<br>Journal of Emerging Technologies in Accounting<br>13<br>Journal of Information Systems<br>4<br>Journal of Network and Computer Applications<br>1<br>Journal of Advances in Management Research<br>1<br>Journal of Business Ethics<br>1<br>Journal of Corporate Accounting&Finance<br>1<br>Journal of Entrepreneurship and Public Policy<br>1<br>Journal of the Association for Information Systems<br>1<br>Journal of Strategic Information Systems<br>1<br>The Journal of Financial Perspectives<br>1<br>Technological Forecasting and Social Change<br>3<br>Accounting and Finance<br>2<br>Accounting today<br>13<br>Armed Forces Comptroller<br>1<br>Australasian Accounting, Business and Finance Journal<br>1<br>Business Horizons<br>6<br>Computer Law and Security Review<br>2<br>CPA Journal<br>1<br>Electronic Commerce Research and Applications<br>2<br>European Research Studies Journal<br>3<br>Financial Innovation<br>4<br>MIT Sloan Management Review<br>2<br>MIT Technology Review<br>2<br>Research in International Business and Finance<br>1<br>Research Policy<br>2<br>SSRN Electronic Journal<br>4<br>Strategic Change<br>6<br>Telematics and Informatics<br>6<br>Journal of Accountancy<br>12<br>**Total**<br>**179**|1<br>1<br>3<br>1<br>2<br>1<br>4<br>1<br>2<br>2<br>1<br>2<br>1<br>1<br>1<br>1<br>6<br>1<br>2<br>2<br>4<br>1<br>1<br>1<br>4<br>3<br>2<br>1<br>4<br>1<br>1<br>1<br>3<br>1<br>1<br>7<br>1<br>1<br>1<br>1<br>1<br>1<br>1<br>1<br>1<br>12<br>1<br>4<br>1<br>1<br>1<br>1<br>1<br>1<br>1<br>1<br>3<br>1<br>1<br>13<br>1<br>1<br>6<br>2<br>1<br>2<br>3<br>4<br>2<br>2<br>1<br>2<br>4<br>6<br>4<br>1<br>1<br>12<br>162<br>11<br>5<br>1<br>(_continued on next page_)|



8 

_International Journal of Accounting Information Systems 48 (2023) 100598_ 

_H. Han et al._ 

**Table 3** ( _continued_ ) 

**==> picture [459 x 58] intentionally omitted <==**

**----- Start of picture text -----**<br>
Academic journals  Overall No. of articles  Types of analysis<br>Conceptual  Qualitative  Quantitative  Mixed<br>Conceptual:  90.50 %<br>Qualitative:  6.15 %<br>Quantitative:  2.79 %<br>Mixed:  0.56 %<br>**----- End of picture text -----**<br>


**==> picture [167 x 8] intentionally omitted <==**

**----- Start of picture text -----**<br>
Fig. 2. Summarized topics covered in this review.<br>**----- End of picture text -----**<br>


that all interested parties who are afforded such rights can view less aggregated transaction data in real-time within the network based on their individual needs for decision-making (Moll and Yigitbasioglu, 2019; Yermack, 2017). For example, Sorter (1969) proposes an event approach to accounting theory by comparing it with the value approach in accounting. His example is an investor attempting to ’s future values based forecast the value of the firm using the two different approaches. He suggests that investors may forecast the firm on the trend, size, and variability of current income or other aggregated values, which is more consistent with a value approach. Alternatively, investors may use accounting data to predict future sales, cost of sales, and taxes (Sorter, 1969). The real difference between the two approaches lies in the degree of accounting information aggregation. The event approach emphasizes using raw data and less aggregated information for decision-making while the value approach uses aggregated information. The question of how to aggregate and share accounting information among various users has always challenged accounting professions (Sheldon, 2018). The excitement and interest in blockchain-based technologies have raised awareness about the lack of financial standards (Singh, 2020) on the emerging FinTech products such as crypto-assets. However, “any aggregation generally involves loss of information” (Ijiri, 1967:120). 

Recently, the availability of data from social media to the government explains why investor decision-making is more likely to be influenced by disaggregated data (Dai and Vasarhelyi, 2017). In other words, business partners and lenders prefer disaggregated data to help them better understand a business and to better guide their decisions. Financial advisory boards typically wish to receive raw data, not those manipulated by companies according to their flexible accounting standards (Cong et al., 2018: 7). Further, developing machine learning will provide better tools to improve the capabilities to make decisions using disaggregated data. According to Cong et al. (2018), while businesses will still provide aggregated numbers to satisfy financial reporting obligations, many businesses can openly offer disaggregated data on demand. For example, many jurisdictions around the world currently require public companies to produce financial reports using extensible business reporting language (XBRL). It is a de-facto standard for the digital exchange of financial information, allowing financial information to be aggregated, transmitted, and analyzed. XBRL used in the UK for company reporting is known as Inline XBRL or iXBRL. XBRL enables data to be processed automatically by software because XBRL can provide a computer-readable tag for each item of business data based on taxonomies used (Gov.UK, 2020). Whereas blockchain is a distributed 

9 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

ledger system, XBRL is a data standard. Because blockchain will need data standards to work well, system designers, accounting practitioners and policy makers need to collaborate and identify whether existing standards like XBRL can be applied or new standards need to be developed as needed (Singh, 2020). 

## _4.2. Real-time accounting_ 

Blockchain is a cutting-edge technology that can transform invoicing, payment processing, contracts, and documentation (Kokina and Davenport, 2017). While items like cash, receivables, payables, and just-in-time manufacturing inventories are already kept up-todate in ERPs, records in ERPs are centralized and lack multi-party validation. Blockchain allows for the public display of encrypted transactions that benefit from multi-party validation, thus enabling companies to provide real-time balance sheets, income statements, cash statements, inventory records and capital investments relevant to some business partners, clients, auditors, and regulators in the value chain (Cong et al., 2018, Smith, 2018). Because blockchain allows essential information to be instantly shared, it could enable a real-time, verifiable, and transparent accounting ecosystem wherein managers, accountants, business partners and investors can collaborate to verify transactions and provide reliable evidence for multi-party validation (Dai and Vasarhelyi, 2017). Blockchainenabled, real-time accounting would significantly reduce opportunistic managerial behaviours to engage in accounting gimmicks and value-destroying actions to manipulate reported earnings. This is because such accounting can allow participants instantly to spot suspicious asset transfers and other transactions that risk conflicts of interests (Buterin, 2014; Tapscott and Tapscott, 2017; Yermack, 2017; Yu et al., 2018). Sheldon (2018) proposes a new way to apply blockchain for accounting professions that share cases of misconduct among parties in real-time. Rozario and Vasarhelyi (2018) suggest using real-time accounting data recorded on blockchains to inform audit procedures and real-time reporting. Wang and Kogan (2018) present a design for a blockchain-based transaction processing system (TPS). It entails developing a prototype to demonstrate the functionality of blockchain-based TPS in real-time accounting for continuous monitoring and fraud prevention. Sogaard (2021) introduces the real-time settlement of value-added tax (VAT) using real-time accounting information recorded in blockchains. Now, financial institutions can deliver in real-time so that every settlement is certain. For example, Ripple offers simpler and faster cross-border payments using blockchains in the global networks (Fanning and Centers, 2016). Blockchain will not replace the XBRL standard; it will become more efficient if XBRL provides high-quality structured data. XBRL combined with a blockchain can enable real-time reporting and real-time accounting (XBRL, 2019). 

## _4.3. Triple entry accounting_ 

Triple entry accounting using blockchain technology creates a shared ledger that can be viewed within business networks. Blockchain’s shared ledger can fundamentally improve transparency and trust using multi-party validated records (Cai, 2019; Carlin, 2019; Dai and Vasarhelyi, 2017; Faccia and Mosteanu, 2019; Karajovic et al., 2019; Schmitz and Leoni, 2019; Simoyama et al., 2017; Tapscott and Euchner, 2019). 

Also available is a framework of triple entry accounting whereby a third layer of entries called “trebit” are added to explain additional income to the current debit and credit entries (Ijiri, 1967). This differs from the triple entry accounting because it adds digitally signed receipts shared by each agent, to provide powerful evidence by sharing the records (Grigg, 2005) in which a digitally signed receipt is the third transaction, thus ensuring trust and transparency of accounting records (Cai, 2019). Triple-entry accounting can also follow an independent and secure pattern to improve the reliability of financial statements, thus increasingly assuring that financial information can be shared with participants in blockchain networks (Dai and Vasarhelyi, 2017). This shared ledger can automate reconciliation to streamline the procedure and provide greater confidence in decision-making (ICAEW, 2018). McCallig et al. (2019) suggest faithfully representing financial reporting can be increased by using the shared data from independent entities, a transparent system, and open-access immutable storage provided by a blockchain. Of course, different businesses will have different needs for triple entry accounting systems. For example, banks have legal requirements to track individual transactions, while other enterprises have more aggregated demands. The design of triple entry accounting systems must fit the purpose of a long-term business strategy. 

## _4.4. Continuous auditing_ 

Traditional auditing methods have now become inadequate to support current and future business needs in the digital economy (Chiu et al., 2018). 

Continuous auditing refers to the use of advanced technology to automate audit activities on a continuous basis to test controls, analyse risks, identify exceptions or anomalies, analyse patterns, and review trends. It will likely advance towards the integration of artificial intelligence and blockchain to form a coherent ecosystem to improve assurance (Cong et al., 2018). Deloitte, Ernst & Young, PwC and KPMG (often referred to as the Big 4 accounting firms) report they are planning to use AI in audit planning, risk assessment, tests of transactions, analytics, and to prepare audit-work papers to benefit from saving time, faster data analysis, increased accuracy, in-depth knowledge, and better client service (Munoko et al., 2020). The Big 4 seek to use AI systems, especially machine learning, allow a system to learn from data to recognize/apply patterns and develop how new data is presented (Shaw, 2019; Shimamoto, 2018). Adopting machine learning has paved the way for advanced auditing, which can be enhanced by blockchain (Casino et al., 2019). For example, Ernst & Young launched a “plug-in, always-on” audit to use real-time blockchain data to replace current sampling practices. Blockchain can transform current auditing to be a more precise and timely automatic assurance system (Dai and Vasarhelyi, 2017) and add trust in AI systems (Salah et al., 2019; Cuomo, 2020; Foote, 2019). Once records have been approved, validated, and stored in a 

10 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

blockchain, the records are immutable. Fanning and Centers (2016) suggest that blockchain can offer a real-time audit trail because information recorded on blockchain can improve audit efficiency and reduce human error (Kokina et al., 2017). The advantages evolve as follows: continuous auditing using blockchain reduces manual data extraction and audit preparation tasks (Schmitz and Leoni, 2019 ), and, in CPA firms, being well connected to blockchain makes it easier for auditors to collect audit evidence and provide assurance services while the firms maintain confidentiality and security of their data (Vincent et al., 2020). 

While AI and blockchain offer technological tools for auditors, they still need data standards to provide a meaningful report, although whether the current XBRL reporting standards will be sufficient to be used by AI and blockchain remains a question. Whether XBRL can provide the quality machine-readable, unambiguous data that are necessary for better AI and blockchain applications, is, as yet, unknown. 

In addition to the four themes, we also note points of caution for organizations to consider when adopting this technology. Blockchains do not provide one-size-fits-all solutions in that they can be specifically applied to different situations and are not the only possible answer, nor even the best. However, blockchain can solve current accounting challenges by changing to multi-party validation of transactions, thus increasing trust and being conducive to digital corporate reporting. While no technology is completely reliable, blockchain can only be altered so false information is added or previous information is deleted if someone manages to obtain 51 % computing power. However, doing so is challenging in public blockchains owing to their distribution networks. The risk of hacking is even greater in private blockchains managed by administrators because a hacker can penetrate their networks or they can modify the operating rules and the contents of private blockchains. Further, believing that organizations will completely abandon their existing IT infrastructures and replace these with a blockchain is unreasonable. Rather, it will be one component of an IT infrastructure, and companies will start implementing blockchain in certain parts of their business to co-exist with their existing legacy systems since not all the data might reside on blockchains. 

## **5. Discussion** 

The above four themes indicate how accounting and auditing could be changed with blockchain and AI technologies to improve transparency and trust in accounting practice. Accounting professionals can improve their decision-making with blockchain’s capacity to provide immutable, append-only, shared, verified, and agreed-upon (i.e., consensus-driven) data, and AI’s ability to learn from data that inform successful decision-making. 

Blockchain-enabled real-time accounting can improve efficiency, reduce settlement time in payments, and mitigate earnings manipulation. Triple entry accounting can reduce the cost of maintaining and reconciling the ledger. The event approach to accounting can provide clarity over ownership of assets. It can allow auditors to check the details of a transaction on how it is recorded and classified. For example, the event approach would enable auditors to check whether the inflow of cash is from sales or receivables, or new investments. Blockchain alongside AI technology will make continuous auditing possible. These advanced technologies will automate many labour-intensive accounting and auditing processes. This will improve the efficiency of accounting and auditing functions. Simultaneously, and this will also have a profound change in the way practitioners work. 

This next section discusses the theoretical and practical implications of the technological impact of blockchain and AI in accounting and auditing. 

## _5.1. Theoretical implications_ 

In theory, to some extent, blockchain’s specific capacity to provide transparency, accurate distribution, immutability, and the computer logic of technology can help companies to overcome information asymmetry and reduce ethical hazards through smart contracts or automation. Further, the technology can also facilitate a new way of collaborating financially for managers, accountants, business partners, investors, and auditors to achieve both cooperation and coordination in blockchain networks or ecosystems. 

From the agency theory perspective, blockchain technology will increase the difficulty for managers to manipulate accounting data because it provides smart contracts and records data precisely. Data recorded on a blockchain is validated through multi-party consensus. This makes it harder to tamper with the data. Additionally, many processes can be automated. For example, a shared blockchain ledger in the triple entry accounting automates reconciliation. Bill payment, expense reporting, auditing sampling, and compliance processes can be automated using blockchain-enabled smart contracts. This technology will make it easier for organizations to control and monitor accounting information. Financial fraud will thus be harder to hide when used together with AI to detect anomalies. In theory, suspicious fund transfers can also be detected in real time. 

However, it does not mean that the use of blockchain and artificial intelligence in accounting can eliminate fraud. This assertion of mitigating agency problems by reducing information asymmetry assumes that people are not manipulating the source or raw data in blockchains. It is worth noting that there are still incentives for firms to cheat by faking the source data if the potential benefits are large enough. If managers obtain 51 % of the computing power, they will be able to manipulate a blockchain’s ledger by adding false information or deleting/modifying historical data. 

From the stakeholder theory perspective, blockchain technology can be an effective mechanism to promote an open and inclusive environment. Interested parties like accountants, business partners, and investors can join and collaborate in blockchain ecosystems to view, update or validate transactions based on their access rights. Organizations can promote stakeholder inclusion and expand business opportunities in blockchain networks. The event approach to accounting with real-time data recorded on blockchains can meet the unique interests and objectives of different accounting information users, who then can use AI to recognize patterns and predict trends. Real-time accounting enables different users who have access to blockchain network to view transaction data as it 

11 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

occurs. The triple entry accounting provides a unique shared ledger that can be viewed by permitted users as the single source of truth. Continuous auditing provides enhanced assurance to improve trust. However, it is critical to balance stakeholders’ conflicts of interest. Companies need to ensure the design of blockchain ecosystem to maximize its capacities to facilitate collaboration. 

## _5.2. Practical implications_ 

This section interprets the practical implications of blockchain-enabled accounting covering four aspects: the changing role of accountants and auditors, challenges, points of caution for those using blockchain technology, choices between different types of blockchains, and implications for small and medium practices (SMPs). 

## _5.2.1. Changing role of accountants and auditors_ 

Blockchain changes the traditional accounting processes by further digitalizing contemporary paper-based validation. It provides a better tool for accountants and auditors to focus on more valuable activities like strategy and in-depth analysis. Blockchain will not completely replace accountants or auditors because their expertise is needed to judge what is fair-value accounting, evaluate intangible assets, assess depreciation, and distinguish types of leases (Hughes et al., 2019). More jobs will be created for accounting practitioners regarding assuring the authenticity of source documents and the worth of smart contracts (Yu et al., 2018). Accounts will play a vital role in generating, executing, and controlling smart contracts (Schmitz and Leoni, 2019). The changing roles will give accountants more capabilities and more time to concentrate on planning and evaluation to increase the scope of accounting (ICAEW, 2018 ). When they use AI technology, it would be easier and more efficient for auditors to check and validate accounting transactions using data recorded on blockchains. The change will increase auditors’ value as they undertake more complex tasks such as smart contract reviews, risk assessments, predictive audits, real-time fraud detection, signature verification, software/algorithm audits, and audit compliance analyses (Boillet, 2017; Tan and Low, 2019). 

## _5.2.2. Challenges and points of caution for blockchain technology_ 

Blockchain is a developing technology. Currently, firms face technical, organizational, and legal challenges for blockchain adoption. For example, the most talked-about challenges are energy consumption, storage capacity, privacy, scalability, interoperability, cybersecurity (Bertino et al., 2019; Buterin, 2014; Gilbert, 2016; O’Leary, 2019), top management support, organizational readiness, access to funds, technical competence, governance issues (Clohessy and Acton, 2019; Coyne and McMickle, 2017), and a lack of blockchain standardization (Guo and Liang, 2016; Hughes et al., 2019). 

Possibly, the technical challenges will be addressed as the technology evolves. For example, Intel is actively collaborating with _R_ 3, using silicon-based technologies like Intel® Software Guard Extensions (Intel® SGX), to help improve the privacy and security of blockchain solutions. The real challenge is with the change management concerning the people, culture, and processes. It involves process change, workflow change, and cultural change. Reaching the benefits might be more cumbersome than thought. As previously mentioned, blockchain can be quite situation specific and is not one-size-fits-all solutions to all the business problems. The use of the technology needs to align with the organizational purpose. Not all the data might reside on a blockchain. The choice between permissionless or permissioned blockchain has quite significant implications in terms of security and throughput. 

## _5.2.3. Choices between different types of blockchains_ 

When choosing blockchain applications, companies need to ensure that blockchain systems are suitably designed and configured, and use processes that are supported by their internal controls (FRC, 2018). An important aspect is whether blockchain can do things cheaper, better, or faster. Currently, the most common examples of public blockchains are Bitcoin (BTC) and Ethereum (ETH), and Litecoin (LTC) while the most common private examples are Ripple (XRP) and Hyperledger. The most common examples of consortium blockchains are Quorum, Hyperledger, and Corda. Whether a blockchain is permissionless or permissioned significantly influences it regarding security and throughput. Permissioned blockchains usually have higher throughput because security needs are reduced with known validators, while permissionless blockchains have lower throughput because security needs are high and more rigorous consensus algorithms are required. Currently, private and permissioned or consortium blockchains are likely to be first adopted into the mainstream business environment for six reasons: 1) to safeguard company’s sensitive information; 2) to differentiate access for stakeholders according to their needs for accounting information to better control who can view/update the ledger; 3) to lower the cost of validation since the consensus mechanism is less expensive because identifiers are known and fewer nodes are needed to achieve consensus; 4) to share certain accounting records with internal departments or external suppliers, customers, investors, regulators and auditors; 5) to increase control of participants so transactions can be more rapidly verified; and 6) to fix or reverse transactions more flexibly (Coyne and McMickle, 2017; Sheldon, 2019; Yermack, 2017). 

## _5.2.4. Implications for small and medium practices (SMPs)_ 

The Big 4 firms are pioneering blockchain and AI applications in their businesses, which concerns SMPs, who may not have sufficient funds or relevant competence to implement blockchain-related services. SMPs should not underestimate the difficulties associated with implementing complex technology. The large firms would capture more market power while SMPs may be left with poorer accountancy/audit options. Ultimately, accounting professions may face an increased inequality gap between big firms and SMPs. Professional accountancy organizations (PAOs) can strive to support SMPs in improving their technical competence by conducting SMP forums to keep practitioners updated with blockchain applications in accounting and auditing. Companies, accountants, auditors, system developers, and regulators must collaborate to invest their knowledge and skills in designing and implementing blockchains in 

12 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

the world of financial record-keeping, together with the use of AI to improve business operations and resilience to meet the needs of the digital economy. 

## **6. Conclusion and future research** 

This study reviews and summarizes four themes marking the changes in recordkeeping in accounting with blockchain technology. This technology can provide shared, verified, and agreed-upon (i.e., consensus-driven) auditable data. Auditing can enhance audit effectiveness with AI tools by using traceable and auditable blockchain data. This review interprets the results using agency and stakeholder theories to explain how blockchain-enabled accounting can avoid information asymmetry and include all stakeholders because blockchain offers new ways of organizing collaboration. However, it is a new, evolving technology that will challenge organizations faced with possible risks from embracing blockchain in accounting. Therefore, more research is needed to explore more cases of using blockchain-enabled accounting. Lastly, this study suggests some questions that future research could seek to answer and thus broaden blockchain literature with empirical research. They could ask: What types of accounting transactions can be recorded on a blockchain, and at what cost? How can blockchain data be synchronized in AI-enabled auditing? What are the data standards that blockchain and AI can reach? How can businesses govern blockchain-based accounting information systems? How can regulations be adjusted to guide and support innovation in blockchain-based accounting information systems and AI-enabled auditing? What different responses and challenges face large accounting and auditing practices and SMPs from adopting blockchain? Blockchain needs to be developed, standardized, and improved to overcome technical, organizational, and regulatory challenges to become truly an integral part of the financial system. 

## **Declaration of Competing Interest** 

The authors declare that they have no known competing financial interests or personal relationships that could have appeared to influence the work reported in this paper. 

## **Acknowledgments** 

We would like to thank the International Journal of Accounting Information Systems editors and three anonymous reviewers for their invaluable and insightful comments and suggestions which immensely helped to improve this manuscript. 

## **References** 

Alles, M., Gray, G. L., 2020. “The first mile problem”: Deriving an endogenous demand for auditing in blockchain-based business processes. International Journal of Accounting Information Systems. 38(September 2020, 100465), 1–15. Angelis, J., Ribeiro da Silva, E., 2019. Blockchain adoption: A value driver perspective. Bus. Horiz. 62 (3), 307–314. Antoinette, A., 2018. Being the bookkeeper of the future. Account. Today 32 (7), 1. 

Appelbaum, D., Nehmer, R.A., 2020. Auditing cloud-based blockchain accounting systems. J. Inform. Syst. 34 (2), 5–21. 

BEIS.,2020. Use of distributed ledger technologies to verify the provenance of goods. Gov.UK (Department for Business, Energy & Industrial Strategy). Available at. https://assets.publishing.service.gov.uk/government/uploads/system/uploads/attachment_data/file/923608/use-distributed-ledgers-verify-provenance-goods. pdf (Accessed: 21 June 21). Benlian, A., Kettinger, W.J., Sunyaev, A., Winkler, T.J., 2018. Special section: The transformative value of cloud computing: A decoupling, platformization, and recombination theoretical framework. J. Manag. Inf. Syst. 35 (3), 719–739. Berle, A., Means, C.G., 1932. The Modern Corporation and Private Property. Macmillan, New York. Bertino, E., Kundu, A., Sura, Z., 2019. Data transparency with Blockchain and AI ethics. J. Data Inform. Qual. 11 (4), 1Biais, B., Bisi`ere, C., Bouvard, M., Casamatta, C., 2019. The blockchain folk theorem. Rev. Financ. Stud. 32 (5), 1662–1715–8. . Boillet, J., 2017. Are auditors ready for blockchain? The audit profession is eyeing blockchain. Account. Today. 31 (9), 34. 

Brennan, N.M., Subramaniam, N., van Staden, C.J., 2019. Corporate governance implications of disruptive technology: An overview. Brit. Account. Rev. 51 (6), 100860. Brown, R. G., Carlyle, J., Grigg, I., Hearn, M., 2016. Corda: An Introduction. R3. Available at. https://docs.corda.net/en/pdf/corda-introductory-whitepaper.pdf (Accessed: 3 July 2020). Buterin, V., 2014. Ethereum White Paper: A Next Generation Smart Contract & Decentralized Application Platform. Etherum, January, 1–36. Available at. https:// github.com/ethereum/wiki/wiki/White-Paper (Accessed: 6 June 2020). Cai, C.W., 2019. Triple-entry accounting with blockchain: How far have we come? Account. Fin. 1–23. Carlin, T., 2019. Blockchain and the journey beyond double entry. Aust. Account. Rev. 29 (2), 305–311. Carlozo, L., 2017. What is blockchain? J. Account. 224 (1), 1–2. Casino, F., Dasaklis, T.K., Patsakis, C., 2019. A systematic literature review of blockchain-based applications: Current status, classification and open issues. Telemat. Inform. 36, 55–81. Chen, M.A., Wu, Q., Yang, B., 2019. How valuable Is FinTech innovation? Rev. Financ. Stud. 32 (5), 2062–2106. Chiu, V., Chan, D.Y., Vasarhelyi, M.A., 2018. Introduction. Continuous Audit. 1–6. Christie, L., 2018. Distributed Ledger Technology. POSTbrief Houses of Parliament. Available at. https://researchbriefings.parliament.uk/ResearchBriefing/ Summary/POST-PB-0028 (Accessed: 26 November 2019). Clemons, E. K., Dewan, R. M., Kauffman, R. J., Weber, T. A., 2017. Understanding the Information-Based Transformation of Strategy and Society. Journal of Management Information Systems. 34(2), 425–456. 1334474. Clohessy, T., Acton, T., 2019. Investigating the influence of organizational factors on blockchain adoption: An innovation theory perspective. Ind. Manag. Data Syst. 119 (7), 1457–1491. Cong, Y., Du, H., Vasarhelyi, M.A., 2018. Technological disruption in accounting and auditing. J. Emerg. Technol. Account. 15 (2), 1–10. Cong, L.W., He, Z., 2019. Blockchain disruption and smart contracts. Rev. Financ. Stud. 32 (5), 1754–1797. Coyne, J.G., Coyne, E.M., Walker, K.B., 2016. A model to update accounting curricula for emerging technologies. J. Emerg. Technol. Acc. 13 (1), 161–169. Coyne, J.G., McMickle, P.L., 2017. Can blockchains serve an accounting purpose? J. Emerg. Technol. Acc. 14 (2), 101–111. 

13 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

Cuevas-Rodríguez, G., Gomez-Mejia, L.R., Wiseman, R.M., 2012. Has agency theory run its course?: Making the theory more flexible to inform the management of reward systems. Corp. Gov. 20 (6), 526–546. 

Cuomo, J., 2020. How blockchain adds trust to AI and IoT. IBM. Available at. https://www.ibm.com/blogs/blockchain/2020/08/how-blockchain-adds-trust-to-aiand-iot/ (Accessed: 17 July 2021). 

Dai, J., Vasarhelyi, M.A., 2017. Toward blockchain-based accounting and assurance. J. Inf. Syst. 31 (3), 5–21. 

de Bakker, F.G., Rasche, A., Ponte, S., 2019. Multi-stakeholder initiatives on sustainability: A cross-disciplinary review and research agenda for business ethics. Bus. Ethics Q. 29 (3), 343–383. de Sousa, W.G., de Melo, E.R.P., Bermejo, P.H.D.S., Farias, R.A.S., Gomes, A.O., 2019. How and where is artificial intelligence in the public sector going? A literature review and research agenda. Gov. Inf. Q. 36 (4), 101392. 

Deloitte., 2016. Blockchain Technology A game-changer in accounting? Available at. https://www2.deloitte.com/content/dam/Deloitte/de/Documents/Innovation/ Blockchain_A game-changer in accounting.pdf (Accessed: 7 July 2020). 

Deloitte., 2017. Blockchain in banking While the interest is huge, challenges remain for large scale adoption. Available at. https://www2.deloitte.com/content/dam/ Deloitte/in/Documents/strategy/in-strategy-innovation-blockchain-in-banking-noexp.pdf (Accessed: 7 July 2020). 

Deloitte., 2020. Thriving in the era of pervasive AI Deloitte’s State of AI in the Enterprise, 3rd Edition. Available at. https://www2.deloitte.com/content/dam/ Deloitte/nl/Documents/innovatie/deloitte-nl-exec-deck-state-of-ai-in-the-enterprise-3rd-edition-final.pdf (Accessed: 19 February 2021). Dong, W., Liao, S., Zhang, Z., 2018. Leveraging financial social media data for corporate fraud detection. J. Manag. Inf. Syst. 35 (2), 461Drew, J., 2018. Merging accounting with “big data” science. J. Account. 226 (1), 47–52. –487. 

Drew, J., 2019. A different approach to applying blockchain. J. Account. 228 (4), 1–4. 

Drew, J., Tysiac, K., 2020. 2020s vision: Tech transformation on tap. J. Account. 229 (1), 23–33. 

Du, W. (Derek), Pan, S. L., Leidner, D. E., Ying, W., 2019. Affordances, experimentation and actualization of FinTech: A blockchain implementation study. Journal of Strategic Information Systems. 28(1), 50–65. 

Faccia, A., Mosteanu, N.R., 2019. Accounting and blockchain technology: From double-entry to triple-entry. Business Manage. Rev. 10 (2), 108–116. 

Fanning, K., Centers, D., 2016. Blockchain and its coming impact on financial services. J. Corp. Acc. Financ. 53–57. Foote, K. D., 2019. Blockchain and Artificial Intelligence: Driving the Fourth Industrial Revolution. Dataversity. Available at. https://www.dataversity.net/ blockchain-and-artificial-intelligence-driving-the-fourth-industrial-revolution/ (Accessed: 2 January 2020). 

FRC., 2018. Blockchain and the future of corporate reporting How does it measure up? Financial Reporting Council. Available at. https://www.frc.org.uk/ getattachment/58866565-ab3b-44d3-93e1-1ef7158968d5/Blockchain-and-the-future-of-corporate-reporting-how-does-it-measure-up-(June-2018).pdf (Accessed: 3 December 2019). 

Freeman, R.E., 1984. Strategic Management: A Stakeholder Approach. Pitman, Boston. 

Freeman, R.E., 1994. Strategic Management: A Stakeholder Approach. Prentice-Hall, Englewood Cliffs, NJ. 

Giboney, J.S., Briggs, R., Nunamaker, J., 2019. Special section: Engineering artifacts and processes of information systems. J. Manag. Inf. Syst. 36 (1), 11–13. Gilbert, D., 2016. Bitcoin’s Big Problem: Transaction Delays Renew Blockchain Debate. International Business Times. Available at. http://www.ibtimes.com/bitcoinsbig-problem-transaction-delays-renew-blockchain-debate-2330143 (Accessed: 7 November 2019). 

Godfrey, J., Hodgson, A., Holmes, S., 2003. Accounting Theory. 5th ed, 5th Ed. Wiley, Milton. 

Goldstein, I., Jiang, W., Karolyi, G.A., 2019. To FinTech and beyond. Rev. Financ. Stud. 32 (5), 1647–1661. 

Gomber, P., Kauffman, R.J., Parker, C., Weber, B.W., 2018. On the fintech revolution: Interpreting the forces of innovation, disruption, and transformation in financial services. J. Manag. Inf. Syst. 35 (1), 220–265. 

Gov.UK.,2020. _Guidance XBRL guide for businesses_ . https://www.gov.uk/government/publications/xbrl-guide-for-uk-businesses/xbrl-guide-for-uk-businesses (Accessed: 03/11/2021). 

Grigg, I.,2005. _Triple Entry Accounting_ . https://www.researchgate.net/publication/308640258_Triple_Entry_Accounting (Accessed: 18 November 2019. Guo, Y., Liang, C., 2016. Blockchain application and outlook in the banking industry. Fin. Innov. 2 (1). 

Hastig, G.M., Sodhi, M.S., 2020. Blockchain for supply chain traceability: Business requirements and critical success factors. Prod. Oper. Manag. 29 (4), 935–954. Hinings, B., Gegenhuber, T., Greenwood, R., 2018. Digital innovation and transformation: An institutional perspective. Inf. Organ. 28 (1), 52–61. Holub, M., Johnson, J., 2018. Bitcoin research across disciplines. Information Society. 34(2), 114–126. 1414094. 

Hood, D., 2018. Brace yourself for AI & blockchain: There’s less threat and more opportunity in emerging technologies than many think. Account. Today 32 (1), 1–31. Hua, X., Huang, Y., Zheng, Y., 2019. Current practices, new insights, and emerging trends of financial technologies. Ind. Manag. Data Syst. 119 (7), 1401–1410. Hughes, A., Park, A., Kietzmann, J., Archer-Brown, C., 2019. Beyond Bitcoin: What blockchain and distributed ledger technologies mean for firms. Bus. Horiz. 62 (3), 273–281. 

Iansiti, M., Lakhani, K.R., 2017. It will take years to transform business, but the journey begins now. Harv. Bus. Rev. 95 (1), 172. 

ICAEW, 2018. Blockchain and the future of accountancy. Available at. https://www.icaew.com/technical/technology/blockchain/blockchain-articles/blockchainand-the-accounting-perspective (Accessed: 3 December 2019). 

Ijiri, Y., 1967. The Foundations of Accounting Measurement. Prentice-Hall Inc, pp. 12–19. 

Issa, H., Sun, T., Vasarhelyi, M.A., 2016. Research ideas for artificial intelligence in auditing: The formalization of audit and workforce supplementation. J. Emerg. Technol. Account. 13 (2), 1–20. 

Jensen, M.C., Meckling, W.F., 1976. Theory of the firm: Managerial behavior, agency costs, and ownership structure. J. Financ. Econ. 3, 305–360. Jie, I. H. D., Li-Yan, A. C., Teng, J. G. W., 2020. _2020_ Singapore Blockchain Ecosystem Report. Opennodes. Available at. https://opennodes.com/SingaporeEcosystem-Report-2020.pdf (Accessed: 8 July 2021). 

Karajovic, M., Kim, H.M., Laskowski, M., 2019. Thinking outside the block: Projected phases of blockchain integration in the accounting industry. Aust. Account. Rev. 29 (2), 319–330. 

Ketz, J.E., 2017. Your future in the accounting profession. Account. Today 31 (6), 14. 

Kokina, J., Davenport, T.H., 2017. The emergence of artificial intelligence: How automation is changing auditing. J. Emerg. Technol. Account. 14 (1), 115–122. Kokina, J., Mancha, R., Pachamanova, D., 2017. Blockchain: Emergent industry adoption and implications for accounting. J. Emerg. Technol. Account. 14 (2), 91–100. 

Kumar, A., Liu, R., Shan, Z., 2019. Is blockchain a silver bullet for supply chain management? Technical challenges and research opportunities. Decis. Sci. 1–30. Leung, A., 2016. Deloitte Sets Good Example, Installs Bitcoin ATM in Toronto Office. Cointelegraph. Available at. https://cointelegraph.com/news/deloitte-sets-goodexample-installs-bitcoin-atm-in-toronto-office (Accessed: 6 November 2019). 

Linton, J.D., 2018. Open innovation/integration versus disintermediation/disintegration. Technovation 78, 1–3. Lubatkin, M.H., 2005. A theory of the firm only a microeconomist could love. J. Manag. Inq. 14 (2), 213–226. 

Lumineau, F., Wang, W., Schilke, O., 2021. Blockchain governance - A new way of organizing collaborations? Organ. Sci. 32 (2), 500–521. 

Marshall, T.E., Lambert, S.L., 2018. Cloud-based intelligent accounting applications: Accounting task automation using IBM Watson cognitive computing. J. Emerg. Technol. Account. 15 (1), 199–215. 

Martin, R., 2018. How Blockchain Will Impact Accounting. Ignite. Available at. https://igniteoutsourcing.com/blockchain/blockchain-accounting-applications (Accessed: 3 December 2019). McCallig, J., Robb, A., Rohde, F., 2019. Establishing the representational faithfulness of financial accounting information using multiparty security, network analysis and a blockchain. Int. J. Account. Inf. Syst. 33, 47–58. 

McWaters, R. J., Bruno, G., Galaski, R., Chatterjee, S., 2016. The future of financial infrastructure An ambitious look at how blockchain can reshape financial services. World Economy Forum. Available at. http://www3.weforum.org/docs/WEF_The_future_of_financial_infrastructure.pdf (Accessed: 7 July 2020). 

Moll, J., Yigitbasioglu, O., 2019. The role of Internet-related technologies in shaping the work of accountants: New directions for accounting research. Brit. Account. Rev. 51 (6), 100833. 

14 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

Munoko, I., Brown-Liburd, H.L., Vasarhelyi, M., 2020. The ethical implications of using artificial intelligence in auditing. J. Bus. Ethics 167 (2), 209–234. Murray, J., 2018. The coming world of blockchain: A primer for accountants and auditors. CPA J. 88 (6), 20–27. Nakamoto, S., 2008. Bitcoin: A peer-to-peer electronic cash system decentralized. Business Rev. 21260. O’Leary, D.E., 2017. Configuring blockchain architectures for transaction information in blockchain consortiums: The case of accounting and supply chain systems. Intell. Syst. Account. Fin. Manage. 24 (4), 138–147. O’Leary, D.E., 2019. Some issues in blockchain for accounting and the supply chain, with an application of distributed databases to virtual organizations. Intell. Syst. Account. Fin. Manage. 26 (3), 137–149. O’Neal, S., 2019. Big Four and Blockchain: Are Auditing Giants Adopting Yet? Available at. https://cointelegraph.com/news/big-four-and-blockchain-are-auditinggiants-adopting-yet (Accessed: 6 November 2019). Ølnes, S., Ubacht, J., Janssen, M., 2017. Blockchain in Government: Benefits and implications of distributed ledger technology for information sharing. Gov. Inf. Q. 34 (3), 355–364. Palmer, D., 2019. Deloitte ‘Blockchain in a Box’ to Help Enterprises Showcase Tech. Available at. https://www.coindesk.com/deloitte-launches-blockchain-in-a-boxto-help-enterprises-showcase-tech (Accessed: 6 November 2019). Pattison, I., 2017. 4 characteristics that set blockchain apart. Available at. https://www.ibm.com/blogs/cloud-computing/2017/04/11/characteristics-blockchain/ (Accessed: 23 November 2019). Pitter, A, 2018. Job disruption is quickly coming to accounting. _Accounting Today_ , _32_ (4), 1–1. PWC, 2018. Blockchain is here. What’s your next move? PwC’s 2018 Survey. Available at. https://www.pwc.com/gx/en/industries/technology/blockchain/ blockchain-in-business.html (Accessed: 16 July 2020). PWC, 2020. Time for trust The trillion-dollar reasons to rethink blockchain. PwC. Available at. https://image.uk.info.pwc.com/lib/fe31117075640475701c74/m/2/ 434c46d2-a889-4fed-a030-c52964c71a64.pdf (Accessed: 16 June 2021). Raschke, R.L., Saiewitz, A., Kachroo, P., Lennard, J.B., 2018. Ai-enhanced audit inquiry: A research note. J. Emerg. Technol. Account. 15 (2), 111–116. Risius, M., Spohrer, K., 2017. A blockchain research framework: What We (don’t) Know, where we go from here, and how we will get there. Bus. Inf. Syst. Eng. 59 (6), 385–409. Rozario, A.M., Thomas, C., 2019. Reengineering the audit with blockchain and smart contracts. J. Emerg. Technol. Account. 16 (1), 21–35. Rozario, A.M., Vasarhelyi, M.A., 2018. Auditing with smart contracts. Int. J. Digit. Account. Res. 18, 1–27. Salah, K., Rehman, M.H.U., Nizamuddin, N., Al-Fuqaha, A., 2019. Blockchain for AI: Review and open research challenges. IEEE Access 7, 10127–10149. Schmitz, J., Leoni, G., 2019. Accounting and auditing at the time of blockchain technology: A research agenda. Aust. Account. Rev. 29 (2), 331–342. Shafiei, G., E., Stein, M. K., Avital, M., 2019. Crowdwork platform governance toward organizational value creation. Journal of Strategic Information Systems. 28(2), 175–195. Shaw, J., 2019. Artificial Intelligence and Ethics. Harvard Magazine. Available at. https://www.harvardmagazine.com/2019/01/artificial-intelligence-limitations (Accessed: 27 July 2021). Sheldon, M.D., 2018. Using blockchain to aggregate and share misconduct issues across the accounting profession. Curr. Issues Audit. 12 (2), A27–A35. Sheldon, M.D., 2019. A primer for information technology general control considerations on a private and permissioned blockchain audit. Curr. Issues Audit. 13 (1), A15–A29. Sheldon, M.D., 2020. Auditing the blockchain oracle problem. J. Inf. Syst. 35 (1), 121–133. Shermin, V., 2017. Disrupting governance with Blockchains and smart contracts. Strateg. Chang. 26 (5), 499–509. Shimamoto, D. C., 2018. Why Accountants Must Embrace Machine Learning. IFAC. Available at. https://www.ifac.org/knowledge-gateway/preparing-future-readyprofessionals/discussion/why-accountants-must-embrace-machine-learning (Accessed: 22 July 2021). Simoyama, F. D. O., Grigg, I., Bueno, R. L. P., De oliveira, L. C., 2017. Triple entry ledgers with blockchain for auditing. Int. J. Auditing Technology. 3(3), 163–183. Singh, M. (2020). _Blockchain and XBRL: The Myth_ . CFA Institute. https://blogs.cfainstitute.org/marketintegrity/2020/10/19/blockchain-and-xbrl-the-myth/ (Accessed: 02 November 2021). Smith, S.S., 2018. Implications of next step blockchain applications for accounting and legal practitioners: A case study. Austr. Account. Business Fin. J. 12 (4), 77–90. Smith, P., 2019. Blockchain could bring a new lease of life to audit. ACCA. Available at. https://www.accaglobal.com/uk/en/member/discover/cpd-articles/auditassurance/blockchain-audit.html (Accessed: 21 July 2021). Sogaard, J. S., 2021. A blockchain-enabled platform for VAT settlement. International Journal of Accounting Information Systems. 40(March 2021, 100502), 1–18. Sorter, G.H., 1969. An “Events” approach to basic accounting theory. Account. Rev. 44 (1), 12–19. Swan, M., 2015. Blockchain: Blueprint for a new economy, First Edition. O’Reilly Media Inc. Tan, B.S., Low, K.Y., 2019. Blockchain as the database engine in the accounting system. Aust. Account. Rev. 29 (2), 312–318. Tapscott, D., Tapscott, A., 2016. The Impact of the Blockchain Goes Beyond Financial Services. Harvard Business Review. Available at. https://hbr.org/2016/05/theimpact-of-the-blockchain-goes-beyond-financial-services (Accessed: 25 October 2019). Tapscott, D., Euchner, J., 2019. Blockchain and the internet of value. Res. Technol. Manage. 62 (1), 12–19. Tapscott, D., Tapscott, A., 2017. How blockchain will change organizations. MIT Sloan Manag. Rev. 58 (2), 10–13. Thakkar, P., 2019. _BOSS Magazine | How Blockchain is Redefining the Rules of Supply Chain_ . Boss Magazine. Available at. https://thebossmagazine.com/blockchainsupply-chain/ (Accessed: 16 January 2020). Toufaily, E., Zalan, T., Dhaou, S.B., 2021. A framework of blockchain technology adoption: An investigation of challenges and expected value. Inf. Manag. 58 (103444), 1–17. Tysiac, K., Drew, J., 2018. Accounting firms: The next generation. J. Account. 225 (6), 3–9. Urbinati, A., Bogers, M., Chiesa, V., Frattini, F., 2019. Creating and capturing value from Big Data: A multiple-case study analysis of provider companies. Technovation. 84–85(July 2018), 21–36. Vasarhelyi, M.A., 2012. Financial accounting standards should not matter: It’s just a layer. J. Inf. Syst. 26 (2), 1–11. Vetter, A., 2019. What CPAs must do to capitalize on disruption. J. Account. 228 (4), 1–7. Vial, G., 2019. Understanding digital transformation: A review and a research agenda. J. Strateg. Inf. Syst. 28 (2), 118–144. Vincent, N. E., Skjellum, A., Medury, S., 2020. Blockchain architecture: A design that helps CPA firms leverage the technology. International Journal of Accounting Information Systems. 38(September 2020, 100466), 1–13. Walker, M., 2013. How far can we trust earnings numbers? What research tells us about earnings management. Account. Bus. Res. 43 (4), 445–481. Wang, Y., Kogan, A., 2018. Designing confidentiality-preserving Blockchain-based transaction processing systems. International Journal of Accounting Information Warner, K.S.R., WSystems. 30(September 2018), 1ager, M., 2019. Building dynamic capabilities for digital transformation: An ongoing process of strategic renewal. Long Range Plan. 52 (3), 326¨ –18. –349. Williamson, O.E., 1979. Transaction-cost economics: The governance of contractual relations. J. Law Econ. 22, 233–261. Winter, J.S., Davidson, E., 2019. Big data governance of personal health information and challenges to contextual integrity. Inform. Soc. 35 (1), 36–51. WorldEconomicForum., 2015. Deep Shift Technology Tipping Points and Societal Impact. Available at. http://www3.weforum.org/docs/WEF_GAC15_Technological_ Tipping_Points_report_2015.pdf (Accessed: 4 December 2019). Wu, J., Xiong, F., Li, C., 2019. Application of internet of things and blockchain technologies to improve accounting information quality. IEEE Access 7, 100090–100098. Xbrl, 2019. Could Blockchain or AI Replace XBRL? Accessed: 30 October 2021. https://www.xbrl.org/news/could-blockchain-or-ai-replace-xbrl/#. Xu, M., Chen, X., Kou, G., 2019. A systematic review of blockchain. Fin. Innov. 5 (1). Yen, J.C., Wang, T., 2021. Stock price relevance of voluntary disclosures about blockchain technology and cryptocurrencies. International Journal of Accounting Information Systems. 40(March 2021, 100499), 1–21. Yermack, D., 2017. Corporate governance and blockchains. Eur. Finan. Rev. 21 (1), 7–31. 

15 

_H. Han et al.                                                                                                                                                                                                            International Journal of Accounting Information Systems 48 (2023) 100598_ 

Yin, H.H.S., Langenheldt, K., Harlev, M., Mukkamala, R.R., Vatrapu, R., 2019. Regulating cryptocurrencies: A supervised machine learning approach to deanonymizing the bitcoin blockchain. J. Manag. Inf. Syst. 36 (1), 37–73. 

Yu, T., Lin, Z., Tang, Q., 2018. Blockchain: The introduction and its application in financial accounting. J. Corpor. Account. Fin. 29 (4), 37–47. Zachariadis, M., Hileman, G., Scott, S.V., 2019. Governance and control in distributed ledgers: Understanding the challenges facing blockchain technology in financial services. Inf. Organ. 29 (2), 105–117. 

Ziolkowski, R., Miscione, G., Schwabe, G., 2020. Decision problems in blockchain governance: Old wine in new bottles or walking in someone else’s shoes? J. Manag. Inf. Syst. 37 (2), 316–348. 

16 

