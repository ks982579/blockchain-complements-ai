## **A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

_Many organizations are looking at blockchain technologies. However, the drawbacks of blockchain databases (e.g., scalability, capacity, latency, privacy) mean that the technology is not always appropriate. This article presents a ten-step decision path that can help determine whether the application of blockchain is justified and, if so, which kind of blockchain technology to use. We describe how this decision path was used to develop a blockchain prototype for the Danish maritime shipping industry.[1]_ 

**Asger B. Pedersen[2] Marten Risius[2] Roman Beck** Netcompany Group University of Queensland IT University at Copenhagen (Denmark) (Australia) (Denmark) 

## **The Promises and Challenges of Blockchains[12]** 

Recently, blockchain technologies have attracted considerable attention. A distributed blockchain application performs the vital functions of a trusted third party by using computer algorithms and cryptography to confirm asset authenticity, authenticate asset ownership, and validate transactions. Blockchains enable organizations to transact directly with each other. With a blockchain application, every participating organization has an exact copy of the same digital ledger. Furthermore, transactions on the shared ledger are immutable, which means every party can be confident they are dealing with the same data. With one version of the truth transparently available to all parties, there are no reconciliations, which enables faster settlement times and lower transaction costs.[3] (More information about blockchain technology is included in Appendix A.) 

Initial research focused on the application of blockchain technology in the financial industry, but more recently supply chains have emerged as the most promising sector. Market forecasters estimate that between $1.5 billion and $2.1 billion was spent on blockchain technologies in 2018 to enhance traceability and transparency in supply chains and to save costs.[4,5] A recent study found that blockchain technology was being deployed to realize value from provenance 

> 1 Mary Lacity is the accepting senior editor for this article. 

> 2 Authors contributed equally. 

> 3 Lacity M. C. “Addressing Key Challenges to Making Enterprise Blockchain Applications a Reality,” _MIS Quarterly Executive_ (17:3), September 2018, pp. 201-222. 

> 4 “Does blockchain hold the key to a new age of supply chain transparency and trust? How organizations have moved from blockchain hype to reality,” _Capgemini Research Institute_ , 2018, available at https://www.capgemini.com/wp-content/uploads/2018/10/ Digital-Blockchain-in-Supply-Chain-Report.pdf. 

> 5 Schatsky, D., Arora, A. and Dongre, A. “Blockchain and the five vectors of progress,” _Deloitte Insights_ , November 24, 2018, available at https://www2.deloitte.com/insights/us/en/focus/signals-for-strategists/value-of-blockchain-applications-interoperability. html. 

**Forthcoming** | MIS Quarterly Executive **1** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

tracking in supply chains at a faster rate than in banking and financial services.[6] 

One supply-chain-related industry that is exploring blockchain technology is maritime shipping. CIOs in this industry have high hopes for the potential of blockchains to reshape the sector to potentially reduce transaction costs.[7] Moreover, maritime shipping executives see blockchains as a feasible technology for facilitating operations, avoiding financial penalties, and improving regulatory compliance. Leading shipping companies such as Mærsk A/S, APL Ltd., Hyundai Merchant Marine Co., and Samsung SDS Co. have already invested heavily in blockchain technology to replace their paper-intensive processes; they hope that using blockchains will generate an additional $1 trillion in global trade.[8] 

However, determining what kind of blockchain and which configuration to use has thus far presented a major obstacle for decision makers and system architects. While there are frameworks to address these issues and to comprehensively explain the technical design[9] and business application considerations,[10] they fail to address decision makers’ common questions about whether a blockchain solution is feasible, and if so, what kind of blockchain system should be implemented. Each blockchain implementation requires a carefully considered decision based on the characteristics of the individual application.[11] 

In this article, we describe a managerial framework for addressing these questions. Using 

> 6 Gupta, S. “HFS Top 10 Blockchain Platforms 2018,” _HFS Research,_ November 7, 2018, available at https://www.hfsresearch.com/ top-10-reports/hfs-top-10-blockchain-platforms-2018. 

> 7 Iansiti, M., and Lakhani, K. R. “The Truth About Blockchain,” _Harvard Business Review_ , January-February 2017, pp. 118-127. 8 Park, K. “Blockchain Is About to Revolutionize the Shipping Industry”, _Bloomberg,_ April 18, 2018, available at https://www. bloomberg.com/news/articles/2018-04-18/drowning-in-a-sea-ofpaper-world-s-biggest-ships-seek-a-way-out. 

9 Xu, X., Weber, I., Staples, M., Zhu, L., Bosch, J., Bass, L., Pautasso, C. and Rimba, P. “A Taxonomy of Blockchain-Based Systems for Architecture Design,” 2017 _IEEE International Conference on Software Architecture (ICSA)_ , IEEE, April 2017, pp. 243-252. 10 Glaser, F. “Pervasive Decentralisation of Digital Infrastructures: A Framework for Blockchain enabled System and Use Case Analysis,” _Proceedings of the 50th Hawaii International Conference on System Sciences,_ IEEE, 2017. 

11 Risius, M. and Spohrer, K. “A Blockchain Research Framework: What We (don’t) Know, Where We Go from Here, and How We Will Get There,” _Business & Information Systems Engineering_ (59:6), December 2017, pp. 385-409. 

a design science research approach (see Appendix B), we have created this framework based on our several years’ experience working as blockchain consultants on projects in both the private and public sectors, as well as on the knowledge and understanding gained through heading the European Blockchain Center. 

The framework comprises a ten-step decision path for determining if a blockchain is applicable and deciding what kind of blockchain solution would be most suitable. To illustrate this approach, we have developed a blockchain prototype for the Danish maritime shipping industry. The development of the prototype involved interviews with representatives of DanPilot (a Danish pilotage company) and with representatives of the Danish Maritime Authority, the government agency that regulates maritime affairs. DanPilot handles the public pilotage through Danish territorial waters from any destination in Denmark to all ports in the Baltic Sea. It employs about 160 pilots and 90 boatmen who manage approximately 20,000 pilotages a year. 

By describing the framework and prototype, we are able to provide practitioners in other business sectors with an urgently needed decision path that considers the unique attributes of different types of blockchain (see Appendix C). The framework will help decision makers not only to decide whether or not to use blockchain technology, but also which type of blockchain to deploy. Although the blockchain decision path will help organizations to systematically assess the feasibility of a potential blockchain solution, practitioners will still have to take account of the specific circumstances of each situation. Practitioners will often be faced with the challenge of resolving complex and potentially paradoxical business and design trade-offs.[12] 

## **Using Blockchain Technology to Address Business Inefficiencies** 

Financially challenging times are often a powerful driver for companies to reevaluate their processes, and to identify inefficiencies 

> 12 Andriopoulos, C. and Lewis, M. W. “Managing Innovation Paradoxes: Ambidexterity Lessons from Leading Product Design Companies,” _Long Range Planning_ (43:1), February 2010, pp. 104-122. 

**2** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

and missed opportunities. In the aftermath of the 2008 financial crisis, the maritime shipping industry, like many others, has had some extremely difficult years financially, resulting from shrinking demand, excessive shipping capacity and expensive credit.[13] Out of necessity, shipping companies have sought larger, better and more price-sensitive solutions while at sea to improve both their operational efficiencies and their financial performance.[14] 

Challenging circumstances like these are not unique to maritime shipping. Supply chains, in general, are a prominent use case for blockchain technologies as businesses struggle with legacy systems and paper-based processes, experience strong price pressure, and rely heavily on integrated systems and information. Thus, the blockchain decision path described in this article is not limited to the maritime shipping industry but is applicable to other business contexts. 

The Danish maritime shipping industry is representative of the general maritime industry for several reasons. Denmark is among the world’s leading shipping nations in terms of owned and operating tonnage.[15] As the sixthlargest shipping nation, Denmark has devoted significant effort to overcoming economic obstacles and staying competitive by investing heavily in IT-based solutions.[16] 

A comprehensive analysis of operational processes in the Danish maritime shipping industry identified a major inefficiency: most of the data relating to ships’ maintenance, logs, crews, machinery and monitoring is gathered and stored locally onboard the ships for insurance purposes. To make matters worse, most of this data is stored in paper documents. For administrative and regulatory purposes these documents are then duplicated multiple times and distributed to crews, ships, shipping 

13 “The global shipping industry’s woes,” _The Economist,_ September 9, 2012, available at https://www.economist.com/graphicdetail/2016/09/09/the-global-shipping-industrys-woes. 

14 Sanders, U., Faeste, L., Riedl, J., Egloff, C., Lee, D., Kloppsteck, L., Kolind, J. and Italino, J. _Battling Overcapacity in Container Shipping_ , March 19, 2015, Boston Consulting Group. 

companies, and maritime authorities. These processes rely heavily on manual labor in disparate organizations, leading to incoherent and dispersed data storage systems. 

Shipping companies like Mærsk and DanPilot, as well as national and international administrative authorities, like the International Maritime Organization of the United Nations, complain about these inefficient processes. In particular, any compliance failures that arise within these processes frequently lead to multimillion-dollar claims, resulting, for example, from delays in unloading cargo, additional docking fees, or immobilized cargo ships. Furthermore, the fragmented and often inconsistent databases need to be available for public access because they are often consulted during legal disputes. As insufficient as they are, the documents in the authorities’ databases are considered to be most reliable and most important in legal disputes. 

In short, the essence of the major problem identified in the Danish maritime shipping industry was that there was no single _source of truth_ , which was the root cause of substantial legal problems and financial losses. However, inefficient business processes resulting from outdated IT infrastructure provide fertile grounds for substantial change and improvement.[17] To address these issues, we developed a blockchain prototype, which illustrates the decision path necessary for selecting the proper system solution. These types of issues are clearly evident in, but not limited to, the maritime shipping industry. Thus, the general decision steps we describe can also be applied in other industries. 

## **The Ten-Step Blockchain Decision Path** 

We developed our ten-step blockchain decision path by first reviewing different blockchain decision paths from public media and 

15 “Maritime Denmark a global power hub,” _The Danish Government Ministry of Industry, Business and Financial Affairs_ , January 2018, available at https://eng.em.dk/publications/2018/marts/maritime-denmark-a-global-power-hub/. 

16 “Strategy forDenmark’s Digital Growth,” _The Danish Government Ministry of Industry, Business and Financial Affairs_ , 2018, available at https://eng.em.dk/media/10566/digital-growth-strategyreport_uk_web-2.pdf. 

17 Grover, V., Jeong, S. R., Kettinger, W. J. and Teng, J. T. C. “The Implementation of Business Process Reengineering,” _Journal of Management Information Systems_ (12:1), Summer 1995, pp. 109144. 

| MIS Quarterly Executive **3** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Figure 1: Overview of the Blockchain Decision Path** 

practitioners.[18,19] We then discussed blockchain design decisions with a potential system user (an experienced DanPilot pilot) and a potential system owner who works for the Danish Maritime Authority (and is also an experienced ship inspector at the Nautical Institute). We then integrated these inputs with our professional experience in blockchain consulting. 

We articulate our ten-step decision path as a series of questions (see Figure 1). The first seven are increasingly specific questions about whether the use of blockchain technology would be useful and feasible; the last three help to determine which blockchain type would be appropriate for the particular application (see Appendix C). Below, we describe each step, outline potential alternatives to blockchain solutions, and illustrate 

> 18  Meunier, S. “When Do You Need Blockchain? Decision Models”, _Medium_ , August 4, 2016, available at https://medium. com/@sbmeunier/when-do-you-need-blockchain-decision-modelsa5c40e7c9ba1. 

> 19 Zubko, H. and Bohner, T. “Lessons Learned from Hyperledger Fabric PoC Projects,” _Hyperledger,_ April 19, 2018, available at https://cn.hyperledger.org/blog/2018/04/19/lessons-learned-fromhyperledger-fabric-poc-projects. 

the individual decisions by applying them to the case of the Danish maritime shipping industry. 

## **Step 1. Is There a Need for a Shared Common Database?** 

It is important to remember that, despite all its various fields of application, a blockchain is essentially a shared database.[20] Thus, the first step when considering the feasibility of a blockchain solution is to ask if a database is needed to provide the required service, and then consider if a traditional database can adequately meet the need. If so, established technologies, rather than a blockchain, should be used to store data and manage transactions. 

In the case of the maritime shipping industry, each vessel weighing over 100 gross tons is issued a unique seven-digit international identification number (IMO-code) by the International Maritime Organization (IMO). This number was introduced in 1987 to increase safety and reduce fraud. As a UN agency, the IMO sets the standards for safety and acceptable 

> 20 Glaser, F., op. cit., 2017. 

**4** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Figure 2: Overview of the Diverse Information Associated with a Vessel** 

levels of shipping pollution.[21] The frequent need to exchange data among multiple parties, and the long history of different types of data storage relating to a ship’s IMO-code, are all factors indicating a strong need for a database. In essence, various parties hold, edit, and access different kinds of data about each vessel (see Figure 2). This data is of various types and is stored in various physical and digital formats. Moreover, the data is owned by different organizations, whose operations depend on the exchange of the data. 

Maritime shipping therefore provides a clear example of an industry that needs a shared common blockchain-based database to prevent data inconsistency across multiple databases, as illustrated by the following quote: 

_“Every time they do something related to the registry, whether it’s here at the Danish Maritime Authority or whether its shipping companies, or brokers, or agents, etc. it’s sent through a blockchain so everything is updated at once.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

During this first step, organizations should closely consider scalability issues relating to 

> 21 “IMO What It Is,” _International Maritime Organization_ , 2013, available at http://www.imo.org/en/About/Documents/What%20 it%20is%20Oct%202013_Web.pdf. 

the amount of data that would be stored on a blockchain and the rate of change of the data. At present, storing and exchanging a lot of data on blockchains can be very slow and expensive due to prolonged verification periods and transaction fees. To prevent scalability issues, designers may consider integrating the blockchain system with an off-chain database, or simply using a conventional database instead of a blockchain. In the case of the maritime shipping industry, frequent database updates are not required, so there are no scalability concerns about using a blockchain solution. 

## **Step 2. Are Multiple Parties Involved?** 

Step 2 of the decision path considers whether the application requires the essential blockchain functionality of a decentralized transactional database.[22] Such a database implies that multiple parties engage with and interact through the system. In a blockchain, engagement means that more than one party contributes, writes, and updates the data. Therefore, the second question that needs to be addressed is whether more than one party is involved with the database. A blockchain system only makes sense if there are multiple parties. Otherwise, a centralized database will provide a more efficient service. 

> 22 Xu, X., et al., op. cit., 2017. 

| MIS Quarterly Executive **5** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Figure 3: Key Players in the Maritime Shipping Industry** 

In the maritime shipping case, a blockchain is appropriate, as shown by the following quote: 

_“Blockchain can help us obtain better security when handling documents between different parties. So that those who are in this chain handling documents concerning ships would be in the loop all the time on these documents.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

There are multiple parties involved in the maritime shipping industry. One is the Nautical Institute (a nongovernmental international representative body for maritime professionals involved in the control of sea-going ships), which works alongside the IMO in a consulting role. The institute classifies ships based on the IMO publication that determines rules for each class for “dynamic positioning.”[23] Another is the Danish Maritime Authority, which is the national governmental organization responsible for ensuring shipping companies’ documentation compliance and their certification requirements for cargo, safety, and medical restrictions. 

The nongovernmental classification companies are responsible for technical standards and the maintenance of vessels, and, according to their class, for conducting checks to ensure that the requirements for machinery and equipment are kept up to date. If there are accidents, a classification company functions as an insurance company. By underwriting a vessel, the classification company demonstrates the quality of a vessel to the owner and the authorities. The 

> 23 “Strategic Plan 2016-2020,” _The Nautical Institute,_ 2016, available at https://www.nautinst.org/download.cfm?docid=2DE93AEB0A7F-41A3-9B53D95DF786E051. 

owner of a vessel is often a shipping company, but a vessel might also be personally owned; the service area[24] of the vessel determines which certificates and legal requirements it must abide by. In addition, the governmental authorities seek to improve transparency by providing the general public with open access to information about a vessel. 

Given the number and diversity of players in the maritime shipping industry (see Figure 3), a blockchain system is a feasible solution. 

## **Step 3. Do the Involved Parties Have Conflicting Interests and/or Are they Trusted?** 

When deciding whether blockchain technology is appropriate for a use case that involves multiple parties, it is necessary to assess how those parties relate to each other. If all parties can completely rely on each other to provide accurate and reliable information, blockchain databases are not necessary. However, a blockchain is appropriate if there are trust issues or conflicts of interest between the parties. 

The unique advantage of a blockchain is that it creates a trust-free ecosystem once the data is uploaded.[25] Trust in a blockchain is established by decentralizing data storage and control across participating nodes,[26] which enables autonomously running trust-free services in the 

> 24 A ship might be owned, for example, by a company in the Philippines but service/dock in Denmark. Such a ship would then be subject to the legal requirements from both the Philippines and Denmark (not just Denmark or the Philippines). 

> 25 Beck, R., et al., op. cit., 2016. 

> 26 Xu, X., et al., op. cit., 2017. 

**6** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

**Figure 4. Current Public Access to a Vessel’s Dynamic Positioning Information** 

form of “smart contracts.”[27] The tamper-resistant character of blockchains means that participants can trust the validity of the stored data, rather than trusting other participants. If the parties have conflicting interests or the data from each party cannot be absolutely trusted, blockchains enable automatic data verification and storage, which means the parties can reliably transact. Although the trust-free nature of blockchains begins to break down when it becomes necessary for blockchain systems to link digital value to physical value through trusted interfaces, absence of trust between the parties and the immutable log of transactions are very strong reasons for using blockchain technology.[28] 

Step 3 of the blockchain decision path asks whether there are trust issues or conflicting interests between the parties. If there are no trust issues, multiple copies of a centralized database or a managed database with assigned “CreateRead-Update-Delete” (CRUD) rights may offer a better solution than a blockchain. The answer to this question in the maritime shipping case is “Yes,” because, as shown in the quote below, 

> 27 Smart contracts are pieces of code that implement business logic by transitioning the current database state to the next state. 

there are several stakeholders with different and potentially conflicting interests: 

_“Of course, there [will] always be … shipping companies [that] might not sail [to] the highest standards, where the ship does not live up to the best quality. They might not have an interest in open data that is accessible, because then we could simply [rate the] ships. [Making all the data public] would [mean we could choose] more secure ships over less secure ships.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

A database system for the marine shipping industry must be able to provide different functionalities for the various stakeholders involved (see Figure 3). At the most basic level, international laws require that any system must provide data access to the general public (see Figure 4). Also the Nautical Institute must be capable of verifying any vessel’s dynamic position class. Furthermore, the IMO and the Danish Maritime Authority require certification and documentation to confirm vessels’ compliance with (inter)national medical and safety laws. To match the data with a vessel, these organizations require the data on technical standards and 

> 28 Glaser, F., op. cit., 2017. 

| MIS Quarterly Executive **7** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

maintenance from the classification company to be linked with the IMO-code. Furthermore, the Danish Maritime Authority and the shipping companies both have an interest in supporting the competitiveness of the shipping companies. 

Currently, however, shipping companies are dissatisfied with the data-access processes. They complain about conflicting information from the various sources, untimely communications with authorities, which delay their operations (e.g., booking pilots), and the need to report the same information back to multiple databases. There are frequent cases of conflicting information—for example, when both the operator and “vendor” (i.e., the company that charters a vessel) are listed as the owners. Communication delays stem from the fact that the Danish Maritime Authority requires all documents to be physically duplicated in the register to minimize the impact of hacking, and requires that at least two people manually check all documents. However, thorough document validation is necessary in this industry because some foreign shipping companies attempt to circumvent certain expensive legal requirements by sailing under different countries’ flags (see quote below): 

_“a blockchain means that] those … handling documents concerning ships would be in the loop all the time … and would not be able to change or fake anything without everyone else knowing it.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

Individual governments can levy multi-milliondollar fines for these kinds of violations. 

Thus, potential conflicts of interest, as well as erroneous and conflicting data, demonstrate that information from the different parties involved in the maritime shipping industry cannot be trusted. This indicates that a blockchain solution is feasible. 

## **Step 4. Can or Do the Participants Want to Avoid a Trusted Third Party?** 

In situations where there is a lack of trust between parties, a trusted third party is often used to manage transactions. One advantage of blockchain systems is that they enable peer-topeer transactions without relying on a trusted third-party service, such as an escrow service, data —feed provider, licensing authority, or 

public notary. This characteristic of blockchains eliminates the need for a central integration point, which can be a single point of failure that could be exploited to control and manipulate a database.[29] The autonomous operating nature of blockchains, together with the trust-free setup of smart contracts, removes the need for third-party trust intermediaries.[30] However, a trusted thirdparty service provider could be used to process and secure transactions in situations where all participants have no problems using such a provider. 

At present, there is no third-party service provider capable of integrating all sources of information in the maritime shipping industry and making the data publicly accessible. Indeed, there are considerable trust issues that would preclude the use of a third party. Moreover, there is considerable interest in building a system that does not require such an intermediary, as illustrated by the two quotes below: 

_“… you need to understand … that … banks have … little trust in each other, so for anything they do they need to have an intermediary. [This even applies to] banks in the same company. It could be [that] banks within Danske Bank [have so little] trust [in] each other that they always use an intermediary. So, there could be a trusted [central] agent …. [But] by using blockchain they can … eliminate this middle man ….”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

_“The challenge occurs when we have international stakeholders, where we need to validate their identity, and [need to validate] who these people are. It sometimes happens that the person who is employed by a company needs to go to a notary to prove that … he is the correct person. But [even] this is not enough. [The employee and the notary] … can … be asked [to go together] to the embassy, which confirms that the notary is … reputable … and that everything is ok.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

From the above discussion, the answer to the Step 4 question in the case of the maritime shipping industry indicates that a blockchainbased solution is feasible and desirable. 

29 Xu, X., et al., op. cit., 2017. 

> 30 Glaser, F., op. cit., 2017. 

**8** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Figure 5: Input Requirements for the Blockchain Prototype’s System’s Smart Contract** 

## **Step 5. Do the Rules Governing System Access Differ Between Participants?** 

After establishing that multiple parties with potentially conflicting interests are involved and that there are trust and compliance issues, the next step is to consider whether individuals require different access rights to the system. The blockchain architectural design allows for differing rights for data reading and writing, as well as access validation rights.[31] Furthermore, at the application layer, smart contracts can govern different privileges in terms of asset issuers (e.g., releasing tokens), account managers (e.g., controlling and exchanging tokens) or observers (e.g., receiving and viewing transactions).[32] If all participants have the same access rights, a relational database offers a more feasible solution than a blockchain. 

Different participants in the maritime shipping industry play different roles and therefore require different access rights. For example, the IMO issues vessel IMO-codes, while the Nautical Institute issues the license given to each vessel and the Danish Maritime Authority 

is responsible for maintaining the registry. The general public also needs to be able to access information about a vessel. The different rights of the various participants mean that the rules governing system access are not uniform, which indicates that using a blockchain system would be beneficial (see quote below): 

_“Mærsk, for example, may be allowed to … do some things in [the] blockchain, using some governance rights. That’s one of the things [among others like the guarantee of information validity and source identity].”_ Project Manager and Nautical Advisor, Danish Maritime Authority. 

## **Step 6. Do the Rules for Transacting Remain Largely Unchanged?** 

The next step in determining whether a blockchain solution is feasible is to consider how frequently the rules for transacting change. It is difficult to accommodate changes in blockchains 

> 31 Xu, X., et al., op. cit., 2017. 

> 32 “Participating in a Blockchain”, _Chain_ , 2018, available at https://chain.com/docs/1.2/core/learn-more/blockchain-participants. 

| MIS Quarterly Executive **9** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

because of their consensus-based decisionmaking procedures.[33,34] 

Smart contracts that provide blockchainbased services are autonomously executed,[35] making them very difficult to change or update.[36] So, for systems where transacting rules change frequently, it would not be advisable to use blockchains. 

In the case of the maritime shipping industry, the basic information requirements do not change (see quote below): 

_“The register of ships can be compared to the [stability of the] land register. In our register it is just registration of ships instead of houses, where [an entry] depends on [a vessel’s] tonnage.”_ Project Manager and Nautical Advisor, Danish Maritime Authority. 

Thus, the data necessary for smart contracts to update and retrieve vessel information could be standardized (see Figure 5), and a blockchain solution would be feasible. 

## **Step 7. Is There a Need for an Objective, Immutable Log?** 

The common benefit of all types of blockchain is the immutability and integrity of a nonrepudiable log of transparent transactions.[37] The tamper-proof log of historical transactions is particularly helpful for auditing purposes.[38] A blockchain not only stores current information but also maintains a log of its history. 

In contrast, creating an auditable history for paper-based records is much more difficult. 

> 33 However, there are examples of changes in blockchain decisionmaking. For example, following the siphoning of decentralized autonomous organization (DAO) tokens through the exploitation of a coding bug, there was a DAO “hard fork” in Ethereum, leading to a heavily disputed but ultimately successful change to decisionmaking. (A hard fork is a radical change to the protocol that makes previously invalid blocks/transactions valid (or vice versa); this requires all nodes or users to upgrade to the latest version of the protocol software.) In the case of Bitcoin, however, the stakeholders’ inability to agree on a hard fork to solve scaling issues has led to a so-called governance crisis. 

- 34 Murck, P. “Who Controls the Blockchain?,” _Harvard Business Review_ (19:1), April 19, 2017, available at https://hbr.org/2017/04/ who-controls-the-blockchain. 

- 35 Glaser, F., op. cit., 2017. 

36 Grincalaitis, M. “Can a Smart Contract Be upgraded/modified? Is CPU mining even worth the Ether? The Top Questions Answered Here…,” _Medium_ , February 6, 2018, available at https://medium. com/@merunasgrincalaitis/can-a-smart-contract-be-upgraded-modified-1393e9b507a. 

- 37 Xu, X., et al., op. cit., 2017. 

38 Glaser, F., op. cit., 2017. 

Not only must the authenticity of records be guaranteed by physical seals and signs—which can never be entirely trustworthy—but paper records (and databases) that rely on manual input are also prone to human error, especially when transactions must be manually handled on a regular basis.[39] If a system does not require the guaranteed validity of transactions, and does not need a definitive validation of transaction details, such as timestamps and parties involved, then a regular database may be a simpler solution than a blockchain. 

This is not the situation in the maritime shipping industry. Since 1987, international maritime law has required that all relevant information about any vessel above 100 gross tons is stored in a way that is auditable in order to increase safety and prevent fraud (see quote below): 

_“[by using blockchain technology] we also have the benefits of the entire audit trail and the document flow, or at least the philosophy behind it.”_ Project Manager and Nautical Advisor, Danish Maritime Authority. 

DanPilot provides an example of the administrative challenge of providing access to historical data. With approximately 20,000 pilotages a year, DanPilot has to respond to about 55 obligatory data searches each day. It has had to hire 50 administrative staff members (approximately 20% of its workforce) to manage the legal requirements relating to data in its current system. However, because all international authorities have their own disconnected databases and individual specifications, pilots also have to double-check all information so they can reasonably demonstrate that the company is obeying all the many laws. The Danish Maritime Authority decrees that, even though a pilot may be given wrong, invalid, or incomplete data, it is his or her personal responsibility if something goes wrong—and the pilot’s license is at risk. The occasional administrative violations that inevitably occur frequently lead to multi-million-dollar costs 

> 39 Bauerle, N., and Kuznetsov, M. “Why Use a Blockchain?,” _CoinDesk_ , 2018, available at https://www.coindesk.com/information/ why-use-a-blockchain/. 

**10** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Figure 6: Blockchain Prototype’s Mode of Operation** 

resulting from delayed cargo clearing, additional docking fees and contract penalties.[40] 

In addition, the Danish Maritime Authority makes its vessel register publicly accessible through a separate database on its website. However, this database is not the official register and may contain outdated, altered, or missing information. Moreover, searching this public register requires specific knowledge of a vessel— for example, call-sign, ship name or IMO-code. All three are unique identifiers of a vessel and, depending on the flag or organization, the identifier may change (see quote below): 

_“... if you dig into the data, trying to figure out why only the Danish Maritime Authority has [a vessel listed] as a cargo ship and not a standby ship, in relation to how it is built from the classification companies, and in relation to how it is operated, it becomes very confusing. This is where I think a blockchain can offer the absolute truth.”_ Pilot and Expert Judge, DanPilot. 

Given the fluctuating and highly documented nature of shipping operations—which 

40 Pittalis, E., Sleiman, T., Washington, T. and Leech, J. “No exceptions from insurers in 2020 for IMO non-compliance,” _S&P Global_ , 2018, available at https://www.platts.com/latest-news/shipping/ london/feature-no-exceptions-from-insurers-in-2020-for-26884390. 

often depend on not entirely trustworthy information—an objective and immutable log provided by a blockchain would be highly beneficial, as illustrated by the following quote: 

_“[In terms of meeting the legal requirements], I am not for one second in doubt that this [blockchain prototype] could be used to exchange information easily and smoothly.”_ Project Manager and Nautical Advisor, Danish Maritime Authority. 

## **Step 8. Is Public Access Required?** 

For the Danish maritime shipping industry, the answers to the questions in Steps 1 to 7 indicate there is a valid use case for a blockchain database. Step 8 (and the next two) determine which type of blockchain should be used. As described in Appendix C, there are three main types—permissionless and public and private permissioned. The choice depends on whether a governance mechanism for controlling access to and participation in the network is needed. Control functions in a blockchain depend on whether there is a need to manage writing rights. For a permissionless blockchain, new users can join at any time; they can validate and transmit 

| MIS Quarterly Executive **11** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

transactions, as well as append or mine[41] blocks. Permissioned blockchains only allow preregistered nodes to validate transactions.[42,43] Permissioned blockchains are more suitable for regulated industries or use cases that have “know-your-customer” regulations. The permission information can be stored either onor off-chain.[44] 

As the quote below shows, different stakeholders in the maritime shipping industry have different rights within the system: _“We have all the different stakeholders segmented into categories, which will require some kind of access control to get into the system, like a protected [permissioned] blockchain.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

While the general public only needs to read the data, other stakeholders have various writing rights for their respective data responsibilities. These various rights indicate that the maritime shipping industry requires a permissioned blockchain. To increase the system’s ease of use, we decided to make the prototype available to “heavy” and “light” nodes.[45] 

## **Step 9. Are Transactions Public?** 

After considering what writing rights will be required, the next step is to decide who will be allowed to read blockchain data,[46] which will determine whether a public or private blockchain should be used. If the transactions can be viewed by the general public, a public blockchain like 

41 For an explanation of the term “mine” in relation to block chains, see https://en.bitcoin.it/wiki/Mining. 

42 Beck, R., Müller-Bloch, C. and King, J. L. “Governance in the Blockchain Economy: A Framework and Research Agenda,” _Journal of the Association for Information System_ (19:10), December 2018, pp. 1020-1034. 

43 Peters, G. W. and Panayi, E. “Understanding Modern Banking Ledgers through Blockchain Technologies: Future of Transaction Processing and Smart Contracts on the Internet of Money,” chapter in _Banking Beyond Banks and Money A Guide to Banking Services in the Twenty-First Century_ , Springer, 2016, pp. 239-278. 

44 Swanson, T. “Consensus-as-a-service: a brief report on the emergence of permissioned, distributed ledger systems,” April 6, 2015, available at http://www.ofnumbers.com/wp-content/uploads/2015/04/ Permissioned-distributed-ledgers.pdf. 

45 Clients using heavy nodes download the entire blockchain platform and need to download every new block before a correct updated output is reliable. This enables the maritime authorities and shipping companies to prevent fraudulent database manipulations. Light nodes do not store the entire blockchain but enable efficient reading access to the blockchain system. 

46 Beck, R., et al., op. cit., 2018. 

Bitcoin or Ethereum should be chosen. A system that regulates reading access requires a private blockchain such as IBM’s Hyperledger Fabric. 

As mentioned earlier (and reinforced by the quote below), in the case of the maritime shipping industry, the general public only requires reading access, while the remaining stakeholders have different writing rights: 

_“In the ship registry there is no confidential information—everything is publicly available— which is one of the things a new [blockchainbased] ship registry should provide as open data.”_ Project Manager and Nautical Advisor, Danish Maritime Authority 

The industry, therefore, requires a permissioned public blockchain that checks individual rights when logging a transaction (i.e., creating or updating data) or a call request (i.e., reading data). As shown in Figure 6, our prototype was designed to operate in this way. Such a system disentangles the reading rights (i.e., general citizen) and writing rights (i.e., shipping companies, classification companies, flag-states, IMO, Nautical Institute) of the different stakeholders. 

## **Step 10. Where is Consensus Determined?** 

A permissioned blockchain is required if reading and/or writing access needs to be limited, with one or more authorities acting as the gatekeeper for participation. These authorities determine who may join a network (and read information), initiate transactions, or mine blocks.[47] Whether to use a private or public permissioned blockchain depends on how consensus for the validity of transactions is determined. 

Private permissioned blockchains determine the validity of transactions within the organization. Examples of this type of blockchain include IBM’s Hyperledger Fabric and R3’s Corda. Hyperledger Fabric, for example, does not require computationally intensive mining, but relies on a consensus mechanism of trusted validating peer nodes that multicast transaction requests to all 

> 47 Xu, X., et al., op. cit., 2017. 

**12** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

other validating peers to reach consensus and ultimately execute transactions.[48] 

Public permissioned blockchains (sometimes referred to as hybrid blockchains) have more finely differentiated rights,[49,50] where consensus is established between participating organizations. Examples include Ripple, Multichain, Eris, and a private iteration of Ethereum. It should be noted that public permissioned blockchains also have a consensus mechanism. The two types of permissioned blockchain differ only in terms of how they determine consensus. 

## **Concluding Comments** 

In this article, we have provided a step-bystep decision path that managers can follow to identify whether they have a valid case for adopting a blockchain solution, the alternatives to blockchain they should consider, and which type of blockchain to use. The series of simple yes-no questions will help practitioners decide when to use a blockchain and which type of blockchain technology to deploy. In reality, however, design requires much more than binary decisions, and involves complex and possibly paradoxical trade-offs. These trade-offs can be localized to the actual design characteristics, but they also need to take account of the broader business requirements and constraints.[51] 

When assessing the practicality of a blockchain solution, practitioners therefore need to carefully evaluate the feasibility of design solutions being able to meet different business requirements. For example, if there are pressing regulatory requirements for an auditable and immutable log (Step 7), a blockchain solution might be the best option regardless of the other decision steps. As a rule of thumb, we usually advise that a blockchain is feasible if at least five out of the first seven questions are answered with “Yes.” Even so, practitioners need to carefully balance various potentially paradoxical business and design requirements for each individual case. 

We have illustrated the 10-step decision path by applying it to the case of the Danish maritime 

shipping industry, where we have developed a blockchain-based prototype to overcome the “no single source of truth” problem. This problem frequently leads to substantial operational inefficiencies and costs resulting from delayed discharge of cargo, additional docking fees, or tied-up resources (e.g., immobilized vessels). A shipping company employee who participated in our research estimates that the costs of developing and rolling out a blockchain system would be well below the fees and costs associated with just one of the frequently occurring cargoclearance delays. 

Inefficient processes using partly paperbased documentation, redundant data storage, and inefficient communications are not confined to the maritime shipping industry. We believe, therefore, that managers in other industries pondering whether to adopt blockchain solutions will also benefit from the decision path described in this article. 

## **Appendix A: Brief Description of Blockchain Technology** 

In essence, a blockchain is a distributed transactional database that is shared among multiple parties (see figure). To perform a transaction, users reference each other through their public keys and use their private keys to cryptographically sign transactions.[52] Each successful transaction on the blockchain indicates an update to the database that is replicated and stored by each participant. Transactions are aggregated and appended to the database in blocks,[53] and can be automatically managed through smart contracts. Services that are based on one or more smart contracts are called _decentralized applications_ (DApps).[54,55] The essential blockchain benefits derived from these functionalities include immutability, nonrepudiation, data integrity, transparency, and the potential for equal rights of participants.[56] 

52 Glaser, F., op. cit., 2017. 

48 Castro, M. and Liskov, B. “Practical Byzantine Fault Tolerance,” _Proceedings of the Third Symposium on Operating Systems Design and Implementation_ , February 1999, pp. 173-186. 

- 49 Xu, X., et al., op. cit., 2017. 

- 50 Glaser, F., op. cit., 2017. 

> 51 Andriopoulos, C., and Lewis, M. W., op. cit., 2010. 

53 Xu, X., et. al., op. cit., 2017. 

> 54 Lacity, M. C. “Addressing Key Challenges to Making Enterprise Blockchain Applications a Reality,” _MIS Quarterly Executive_ (17:3), September 2018, pp. 201-222. 

> 55 Glaser, F., op. cit., 2017. 

> 56 Xu, X., et al., op. cit., 2017. 

| MIS Quarterly Executive **13** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Core Components of a Blockchain System** 

Blockchain systems are commonly distinguished in terms of public or private access to reading blockchain data and the permissioned or permissionless rights to validate data (see Appendix C). However, despite a common view that blockchains have the potential for revolutionizing the economy at large,[57] the technology also has limitations compared with other distributed databases (including capacity, latency, and privacy) that must be taken into account when considering a blockchain solution.[58,59] 

## **Appendix B: Research Approach** 

We applied a problem-centered design science research approach to develop a proof-ofconcept prototype that addresses the needs of the maritime shipping industry. The stages of this 

> 57 Beck, R., Czepluch, S. J., Lollike, N. and Malone, S. “Blockchain: The Gateway to Trust-free Cryptographic Transactions,” _24th European Conference on Information Systems (ECIS)_ , Association for Information Systems, June 2016. 

> 58 Glaser, F., op. cit., 2017. 

> 59 Xu, X., et. al., op. cit., 2017. 

approach are depicted in the figure and described below.[60] 

## **The Problem-Centered Solution** 

The maritime shipping industry is currently experiencing significant financial challenges. This has encouraged stakeholders across the industry to reconsider their processes and identify opportunities for improving them. Most of the current administrative processes require a lot of human attention, which causes inefficiencies, errors, and delay that can lead to considerable costs and financial penalties. The root cause of the problems is the lack of a single source of truth. The goal of our research project was to demonstrate a solution that could overcome the problems. 

**Identify Problem and Motivation.** The main problems arising from the lack of a single source of truth in the maritime shipping industry are financial penalties resulting from delays in unloading cargo, and shipping companies forging documentation to avoid proper occupational health regulations and to circumvent legal requirements. 

> 60 Peffers, K., Tuunanen, T., Rothenberger, M. A. and Chatterjee, S. “A Design Science Research Methodology for Information System Research,” _Journal of Management Information Systems_ (24:3), 2007, pp. 45-77. 

**14** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

## **Problem-Centered Design Science Approach** 

**==> picture [462 x 151] intentionally omitted <==**

**----- Start of picture text -----**<br>
Identify Problem Define Objectives  Design and<br>Evaluation Communication<br>and Motivation of a Solution Development<br>Ensure<br>Economically  Efficient, reliable  stakeholders’  Assess the  The prototype was<br>suffering industry  and easily  rights and  prototype’s  publicly presented<br>lacking single  verifiable  permissions are  performance  as part of the<br>source of truth documentation managed and  against  Danish Digital<br>can be executed requirements Growth Strategy<br>Problem-<br>oriented<br>Approach<br>**----- End of picture text -----**<br>


**Objective of the Solution.** To explore the potential use of a blockchain solution, we first conducted interviews with representatives of various stakeholders in the Danish maritime shipping industry. Next, we used the 10-step decision path to confirm that a blockchain solution was feasible and to identify which type of blockchain should be used. We then decided to develop a proof-of-concept blockchain prototype. The main objectives of the prototype were to improve the efficiency of administrative processes, to make them more reliable and trustworthy, and to make documents more easily verifiable. By conducting interviews before developing the prototype, we were able to ensure that we chose the most appropriate and most beneficial solution (see table in Appendix C). 

**Design and Development.** The solution needs to allow the general public to access the data, and also to manage the specific permissions required by participants from the different industry stakeholders for logging changes. To accommodate the variety of reading and writing rights, we opted to use a public permissioned blockchain on the Ethereum private net to develop the prototype.[61] Initially, the Danish Maritime Authority was the “super user” who managed permission rights. 

Stakeholders access the blockchain either through a heavy or light node depending on whether writing rights are required or permitted. 

**Demonstration.** We demonstrated the design, development, and benefits of the prototype 

blockchain solution to the Danish Maritime Authority. The demonstration showed the feasibility of applying a blockchain solution across the entire maritime shipping industry. Moreover, the prototype included signed and executed smart contracts, which allowed the general public to follow changes in the ship registry. The smart contracts were written in the Solidity programming language. 

The insights gained from the design decisions during the development of the prototype enabled us to validate the 10-step blockchain decision path. 

**Evaluation.** A further interview with a representative of the Danish Maritime Authority evaluated the prototype solution against the qualities and requirements gathered in the earlier interviews. This evaluation confirmed the blockchain prototype was consistent with the actual needs and constraints of the industry. 

**Communication.** We discussed the prototype results, and the use of the blockchain decision path, with blockchain consultants. Additionally, in early 2018, the Danish government published _Strategy for Denmark’s Digital Growth_ ,[62] which included a commitment to use a blockchainpowered solution for the ship register. 

## **Outcome of the Research Project** 

The prototype demonstrated that blockchain technology can provide a suitable and effective solution for the identified problem-centered issue 

> 61 The interactive prototype can be accessed at https://projects. invisionapp.com/share/7KBLNHNTZ#/screens/232477611. 

> 62 Available at https://eng.em.dk/media/10566/digital-growthstrategy-report_uk_web-2.pdf. 

| MIS Quarterly Executive **15** 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

in the maritime shipping industry by establishing a single source of truth. 

## **Appendix C: The Three Types of Blockchain[63]** 

The last three steps of the 10-step blockchain decision path determine which type of blockchain should be used. The three types are summarized in the table and described below. 

## **The Three Blockchain Types** 

- _A permissionless public blockchain_ is an open network that enables anyone to join (examples include Bitcoin and Ethereum). With this type of blockchain, all users can read, write, and verify transactions. This type of blockchain can replace the role of a trusted third party. Trust is built between peers in the network because they all have to abide by the established consensus mechanism. The most popular consensus mechanisms are Proof of Work (PoW) and Proof of Stake (PoS). In a PoW system, miners in the network compete computation-wise by solving the hash function of the next block. PoS defines the next valid block in a more deterministic 

way, depending on the stake that different miners hold (e.g., number of tokens). 

- _A permissioned public blockchain_ is a closed network, where only verified and trusted nodes can participate (examples are Ripple, Multichain, Eris and Hyperledger Fabric). This type is also called a “hybrid blockchain,” because all participants can view the data, but only authorized users can validate transactions.[64] Users are authorized through a network consensus after providing the necessary proof of eligibility. However, if there are no trust issues among users, the only reason for using a hybrid blockchain would be the immutable logging of historical transactions for auditability purposes.[65] 

- _A permissioned private blockchain_ is a closed network that allows only authorized users to read, submit, and validate transactions[66] (examples include Hyperledger Fabric and Corda). Transactions are verified or the blockchain’s consensus is determined within an organization. Commonly, a Practical Byzantine Fault Tolerance (pBFT) protocol is used, which requires a certain percentage of previously verified nodes to confirm the transactions. This makes 

**==> picture [469 x 207] intentionally omitted <==**

**----- Start of picture text -----**<br>
The Three Blockchain Types<br>Blockchain Type Properties<br>Anyone can join the network, and read, write and verify<br>Permissionless public blockchain<br>transactions through Proof of Work or Proof of Stake.<br>Only trusted and validated peer nodes may join the network.<br>Permissioned public blockchain<br>Consensus is determined between participating organizations.<br>Only trusted and validated peer nodes may join the network.<br>Permissioned private blockchain Consensus is determined within an organization through, for<br>example, a Practical Byzantine Fault Tolerance [63]  algorithm.<br>**----- End of picture text -----**<br>


63 Castro, M. and Liskov, B., op. cit., 1999. 

64 Beck, R., et al., op. cit., 2018. 65 Glaser, F., op. cit., 2017. 66 Beck, R., et. al., op. cit., 2018. 

**16** MIS Quarterly Executive | 

**misqe.org** | © 2019 University of Minnesota 

**A Ten-Step Decision Path to Determine When to Use Blockchain Technologies** 

the pBFT model more efficient than PoW as the miners are not competing and only doing the computations to benefit the network. 

& Distributed Ledger Technology standardization group on Blockchain Governance. He is featured in the blockchain documentary film, _Blockchain City._ 

Note that the same blockchain application can operate as more than one type. For example, Hyperledger Fabric can be used to provide a permissioned public or permissioned private blockchain. 

## **About the Authors** 

## **Asger B. Pedersen** 

Asger Pedersen (ASGER1989@GMAIL.COM) is a consultant at Netcompany Group and runs the Blockchain Center of Excellence. He has a master’s degree in software development and technology, with a specialization in blockchain, from IT University at Copenhagen, and a bachelor’s degree in technology, management, and marine engineering from Svendborg International Maritime Academy. He has won nine hackathons in the EU, most recently “ID at Work,” on the subject of digital identity. 

## **Marten Risius** 

Marten Risius (m.risius@business.uq.edu.au) is a senior lecturer in information systems at the University of Queensland, Brisbane, Australia. His research focuses on managerial and societal issues relating to social media and blockchain technologies. His work has been published in various international journals and presented at peer-reviewed conferences. Marten has been internationally recognized with various academic and industry awards as well as research grants. His work has also been featured in public media—for example, _The Boston Globe._ 

## **Roman Beck** 

Roman Beck (romb@itu.dk) is a professor at IT University of Copenhagen and head of the European Blockchain Center. His research focuses on how blockchain technology is changing the nature of work, with an emphasis on governance and value creation in decentralized systems. His research has been published in over 120 journal and peer-reviewed conference articles. Roman is the conference co-chair for the International Conference of Information Systems (ICIS) 2022, and convener of the ISO TC 307 WG 5 Blockchain 

| MIS Quarterly Executive **17** 

Copyright of MIS Quarterly Executive is the property of MIS Quarterly Executive and its content may not be copied or emailed to multiple sites or posted to a listserv without the copyright holder's express written permission. However, users may print, download, or email articles for individual use. 

