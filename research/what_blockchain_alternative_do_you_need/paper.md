## **What Blockchain Alternative Do You Need?** 

Tommy Koens and Erik Poll 

Radboud University, Nijmegen, The Netherlands `tkoens@cs.ru.nl E.Poll@cs.ru.nl` 

**Abstract.** With billions of dollars spent on blockchain, there clearly is a need to determine if this technology should be used, as demonstrated by the many proposals for decision schemes. In this work we rigorously analyze 30 existing schemes. Our analysis demonstrates contradictions between these schemes – so clearly they cannot all be right – and also highlights what we feel is a more structural flaw of most of them, namely that they ignore alternatives to blockchain-based solutions. To remedy this, we propose an improved scheme that does take alternatives into account, which we argue is more useful in practice to decide an optimal solution for a particular use case. 

## **1 Introduction** 

Ever since the invention of blockchain in 2008 [46], this technology has piqued the interest of industry, and many blockchain initiatives have arisen. Over a 1000 patents [1] in this technology were filed, and it is estimated that blockchain global spending reaches 2 billion US dollar in 2018 [15]. 

Following the blockchain hype [48], many initiatives discovered that blockchain as it is used in for example Bitcoin and Ethereum is not a panacea. Instead, alternative blockchain technologies have been proposed that fit better. To be able to determine if, and if so which blockchain is needed in a particular scenario, various decision models have been proposed. However, there are significant differences between such schemes. In fact, some schemes provide different answers for the same scenario. This raises the question: Which decision scheme should you use? This paper addresses that question and makes the following contributions: 

- We perform a critical analysis of decision schemes in Section 3. Our analysis demonstrates some contradictions between schemes and suggests that none of the schemes is complete, in that they do not take current limitations of blockchain technology into account and ignore what alternative database technologies besides blockchain there are. 

**–** To repair this omission, we propose a new scheme in Section 4, which does take these alternatives into account. With our scheme the need for blockchain or alternative technologies can be determined. We discuss our scheme in Section 5. Given the global interest and financial resources spend on blockchain, our scheme can be used as a sanity check for blockchain initiatives. 

Section 6 discusses future work and we summarize our conclusions in Section 7. 

2 

## **2 Background** 

Blockchain technology underlying cryptocurrencies such as Bitcoin and Ethereum offers a unique property. Namely, it allows for reaching agreement on a single state of a shared ledger by a consortium of unknown participants [50]. Transaction sets, called blocks, are proposed at frequent time intervals, where each block includes the cryptographic hash of its predecessor block. This creates a chain of blocks, which explains the term blockchain. Two important characteristics of this technology, as it is originally used in e.g. Bitcoin, are that the blockchain is _permissionless_ and _public_ . Permissionless means that anyone may join or leave the network at will. Public means that anyone, in principle, may propose a new state of the ledger. 

However, there currently are several issues with this technology. First, it performs poorly regarding transaction scalability. For example, Ripple, a technology that is not blockchain-based [9,28], claims to be able to scale to 50.000 transactions per second (tps) [27], whereas Bitcoin can handle 7 tps and Ethereum 14 tps. Second, although a blockchain is a form of database, it is currently not suited to store large amounts of transaction data. 

Furthermore, some authors [51,52,55] claim that blockchain is an immutable ledger. However, this is a misconception, as blockchains are mutable. First of all, an important purpose of this technology is that state changes are made possible. Therefore, state stored on the blockchain is by default mutable. Mutability may also refer to the stored transactions on the ledger. Again, these transactions are also mutable, although they are much harder to change than state. For example, anyone with over half of Bitcoin’s network resources can rewrite the ledger’s history [35], which is also called mutable-by-hashing-power [38]. Recent work suggests that even a quarter of the resources is sufficient to ultimately achieve the same goal [39]. Another example that further illustrates the mutability of blockchains is the hard fork of Ethereum after the DAO hack, where 50 million dollars worth of Ether was stolen through a bug in a smart contract. The current ledger called Ethereum Classic left the funds stolen. However, a new ledger (called Ethereum) was created, returning the stolen funds, thus undoing the hack and rewriting history. Although blockchains are mutable, in most cases it is hard to rewrite history. However, there are scenarios where easy mutability is a requirement, for example, because of the need to correct accidental mistakes. 

To overcome these blockchain issues (scalability, performance, hard to alter history), alternative database technologies may be more useful. For example, permissioned and public database technology can be found in Ripple [27], which uses a distributed ledger [28]. Here, anyone may join the network and read from the ledger, but only a limited set of participants may propose new ledger states. Also, permissioned and private database technologies have been proposed, for example in R3 Corda [25]. Here, participation in the network is by invitation only, and also a limited number of the participants may propose new states. 

Following these types of database technologies, initiatives have to decide which technology is appropriate for a particular scenario. To support this decision making process, decision schemes for database technologies, and in par- 

3 

ticular blockchain, have been proposed. However, the decision schemes are not always clear in what is meant with blockchain. 

## **2.1 Blockchain terminology** 

We observed that the term blockchain is used arbitrarily in the schemes. Indeed, Birch et al. [34] and Maull [45] also state that many authors use the term blockchain in different ways. Interestingly, in the original work by Nakamoto [46] the term blockchain is not used, but the term distributed time-stamp server is used. Pahl [47], Birch et al. [34] and Lin [43] state that blockchain is a distributed ledger. Pahl [47] also calls blockchain a distributed database, while Birch et al. [34] use the term ’shared ledger’. W¨ust and Gervais distinguish permissionless and permissioned blockchains, and provide examples for each type. Their Corda example, however, can be considered a decentralized database [22]. Although Corda is heavily inspired by blockchain systems [26], Corda does not use a chain of blocks. These examples show that, indeed, various terms are used interchangeably and are not always correctly. 

The terminology for the different solutions we use in this paper is illustrated in Figure 1 and explained below. We distinguish two types of databases: central databases (DBs) and distributed databases. In a central database, data is centrally stored. Following this, a central ledger is a central database with the inclusion of transaction interaction. Transaction interaction [12] refers to the interdependency of transactions of different participants. For example, a Bitcoin account with a balance of 0 can only create a valid transaction after it receives a transaction that increases its current balance. Additionally, a shared central ledger can be used when multiple writers are present. 

A distributed database stores data across multiple locations, and provides read and write access to participants. Following this, a distributed ledger is a distributed database with the inclusion of transaction interaction. We consider blockchain (BC) to be a particular form of distributed ledger technology (DLT), as here _unknown_ participants can read from and write to the ledger, and reach consensus on the state of the ledger. 

**==> picture [325 x 120] intentionally omitted <==**

**----- Start of picture text -----**<br>
Databases<br>Central databases Distributed databases<br>Central ledgers Distributed ledgers<br>Shared ledger Blockchain<br>**----- End of picture text -----**<br>


**Fig. 1.** Our classification of database technologies 

4 

## **3 Evaluation of Decision Schemes** 

In this section we analyze 30 blockchain decision schemes, listed in Table 1. We classify the schemes by type, based on the question(s) they answer. We also classify the choices that the schemes involve, listed in Table 2, and we investigate contradictions between some of the schemes. 

## **3.1 An overview of schemes** 

We found 30 decision schemes in the literature and on the web, and included all schemes found; see Table 1. Five schemes are represented as a questionnaire, indicated by a ’Q’ in Table 1. The remaining 25 schemes are represented by a flow diagram, indicated by an ’F’ in Table 1, where a sequence of binary choices lead to an end state that provides the optimal solution for a given scenario. 

**Table 1.** An overview of decision schemes 

|**No. **|**DS name**|**Ref. **|**F/Q **|**Model **|**#ES **|**End states**|
|---|---|---|---|---|---|---|
|1|CapGemini|[7]|Q|1|2|A.1.a; A.2.a|
|2|Cooke|[19]|F|1|2|A.1.a; A.2.a|
|3|Gardner|[16]|F|1|2|A.1.a; A.2.a|
|4|Lixar|[20]|F|1|2|A.1.a; A.2.a|
|5|Meunier|[29]|Q|1|2|A.1.a; A.2.a|
|6|Nandwani|[18]|F|1|1|A.1.b|
|7|PWC|[24]|Q|1|2|A.1.a; A.2.a|
|8|Verslype|[17]|F|1|2|A.1.b; A.2.a|
|9|Birch|[34]|F|2|4|B.1; B.2; B.3; B.4|
|10|Saiko|[8]|F|2|3|B.1.a; B.3.c; B.4.a|
|11|Bico|[5]|F|1,2|4|A.2.a; B.4.a; B.3.c; B.1.a|
|12|Chand|[21]|F|1,2|3|A.2.a; B.4.a; B.1.a|
|13|Hyperledger|[13]|F|1,2|3|A.2.a; B.1.a; B.3.a|
|14|Ico|[32]|F|1,2|3|A.2.a; B.3.a; B.1.a|
|15|Lin|[43]|F|1,2|4|A.2.a; B.4.a; B.3.c; B.1.a|
|16|Meuller|[31]|F|1,2|3|A.2.a; B.1.a; B.3.a|
|17|Pahl|[47]|F|1,2|5|A.1.a; A.2.a; B.1.c; B.3.b; B.4.b|
|18|Peck|[49]|F|1,2|3|A.2.a; B.3.a; B.1.a|
|19|Suichies|[4]|F|1,2|4|A.2.a; B.1.a; B.3.c; B.4.a|
|20|WEF|[33]|F|1,2|5|A.2.a; A.3 (x2); A.1.a (x2)|
|21|W¨ust|[56]|F|1,2|4|A.2.a; B.1.b; B.3.b; B.4.b;|
|22|DHS|[29]|F|1,3|7|C.1.d; C.1.a (x3); C.1.b; C.1.c; A.1.a|
|23|Greenspan|[12]|Q|1,3|3|A.1.a; A.2.a; C.1.a|
|24|IBM|[14]|F|1,3|2|A.1.b; C.1|
|25|Lewis|[2]|F|1,3|3|A.2.b; A.1.a; C.1.a|
|26|Xu|[57]|F|1,3|2|C.1; A.1.a|
|27|Deloitte|[10]|Q|1,2,3|4|A.1.a; C.1; B.1.b; B.3.a|
|28|Henkel|[6]|F|1,2,3|5|A.1.a; A.2.a; B.1.a; B.3.a; B.4.a|
|29|Maull|[45]|F|1,2,3|4|A.2; B.1; B.3; B.4|
|30|Quindazzi|[23]|F|1,2,3|5|A.2.a; C.1; B.1; B.3; B.4|



5 

We observe that all schemes can be classified in (a combination of) three models, where each model addresses a primary question: 

- **Model 1: Determine if blockchain should be used.** Schemes that aim to determine if you should use a distributed ledger or, more specific, blockchain. 

- **– Model 2: Determine blockchain type.** These schemes aim to determine which type of blockchain fits best to a particular problem. 

- **Model 3: Determine alternative technologies.** The third model suggests alternative technologies such as traditional databases. 

A classification of each scheme towards these three models can be found in Table 1 (column: Model). Additionally, we counted the number of end states (column: #ES) for each decision scheme. This already shows that there exists a difference between similar scheme types and number of end states. Furthermore, we grouped the various end state descriptions (column: End states), according to our terminology definition in Section 2, in the columns below. Typically, in the literature blockchains are classified in three categories: 

- Permissionless (anyone may write to the ledger) and public (anyone may read from the ledger). 

- Permissioned and private (only a limited set of participants may read from the ledger). 

- Permissioned and public. 

From these columns we also note various levels of granularity of end state descriptions. For example, it is not clear if end state B.1.a (public BC) is permissionless (similar to B.1.c) or permissioned (similar to B.3.b). Also, Birch et al. [34] introduce new terminology, such as the public double permissionless DLT (B.2). This includes the reward mechanism of writing to the ledger which, when intrinsic to the consensus process, is called double permissionless. An extrinsic mechanism where a writer receives a physical reward (e.g. cash) is called permissionless [34]. 

## **A. Model 1 end states** 

1. DLT is a good fit. (a) Use BC. 

   - (b) Let’s talk. 

2. DLT is not a good fit. 

   - (a) Don’t use BC. 

   - (b) Problem of standards. 

3. BC may be a good solution. 

## **B. Model 2 end states** 

1. Public permissionless DLT. 

   - (a) Public BC. 

   - (b) Permissionless BC. 

   - (c) Public permissionless BC. 

2. Public double permissionless DLT. 

3. Public permissioned DLT. 

   - (a) Permissioned BC. 

## **C. Model 3 end states** 

1. Consider alternative approaches. 

   - (a) Central database. 

   - (b) Encrypted DB. 

   - (c) Managed DB. 

   - (d) Consider email / spreadsheets. 

   - (b) Public permissioned BC. 

   - (c) Hybrid BC. 

4. Private permissioned DLT. 

   - (a) Private BC. 

   - (b) Private permissioned BC. 

5. Private double permissioned DLT. 

6 

## **3.2 Model 1 scheme end states** 

Model 1 schemes aim to determine if you should use a blockchain. Several schemes, for example Pahl, Gardner, and Greenspan, give a clear yes-or-no answer whether a blockchain should be used or not. Other schemes are more conservative. For example Peck, Meuller, and DHS, only say that blockchain may be an option. Typically, these schemes do not elaborate what further conditions have to be met to determine if blockchain should (or should not) be used. 

## **3.3 Model 2 scheme end states** 

Model 2 schemes aim to determine which type of blockchain is needed. Typically, these schemes also answer the question whether you should use a blockchain or not, so they are also model 1 schemes. 

Both Saiko and Birch et al. propose a type 2 scheme only. Interestingly, Saiko considers three types of blockchains, although uses the terms blockchain and ledger interchangeably. In contrast, Birch et al. consider four distributed ledger types, although in their work they do provide examples that include blockchain. The main difference between the two schemes is that Birch et al. suggest two types of public ledgers and two types of private ledgers, whereas Saiko suggests a single public ledger and two types of private ledgers. Here, again, we observe a difference in schemes, similar to model 1 schemes. 

However, we consider blockchain variants not a viable option, as better, alternative technologies are available. We will discuss this further in Section 5. 

## **3.4 Model 3 scheme end states** 

Model 3 schemes also consider alternative technologies other than blockchain. One of the outcomes of IBM’s scheme is ’consider alternative approaches’, but it does not say what these alternatives might be. The scheme by DHS does suggest some concrete alternatives, such as a database or a managed database. Quindazzi refers to the traditional ledger (as in the current banking system) as an alternative to other types of ledgers. However, these suggestions are generic and do not point out which type of database should be used. Clearly, the end states of which type of database to use can be refined in these models. 

## **3.5 Scheme questions** 

In this section we analyze all schemes, and group and classify the questions that are used to determine an end state; see Table 2. To be able to reach any of the three model end states (as discussed in the previous section), each question should lead to an answer which holds a (database) technology property. In particular, we are interested in questions that differentiate between technologies [41], which we label ‘T’. For our scheme we currently consider the remaining questions as not relevant. We classified the questions as follows: 

7 

1. Our first question type refers to determining which database type is needed. We label these as ‘T’. 

2. Also, there exist questions that address the current limitations of blockchain, which we label as ‘L’. 

3. A particular set of questions focus on the system design, instead of technological properties. For example ‘(do you need) censorship resistance’ and ’where is consensus determined’ are design questions. These scheme questions consider this to be a prerequisite for the use of a technology. We do not consider these questions for our scheme, as they do not distinguish between technologies from a technical perspective. We label these questions as ‘E’. 

4. We label our fourth question type as process questions, ‘P’. The answers to these questions also do not in particular differentiate between technologies. Therefore, these questions types in the schemes are irrelevant for determining if, and if so, which database technology can be used. For example, the questions ‘aiming to remove third parties?’, ‘looking to reduce costs?’, and ‘can participants adopt?’ are process related questions. We do not include these questions in our scheme, see Section 4. 

   - Also, some schemes (e.g. Cooke, Suichies, WEF) include the question ‘Are writers interests unified?’ to determine the appropriateness of blockchain, and consider that if this is indeed true, no blockchain is needed. However, the interests of the honest participants may be aligned, but not the interests of a malicious participant. The point here being that when choosing a particular technology, the basic issues (such as the double spend attack in blockchain) should be considered as part of the system. Therefore, the interests of participants by default are not aligned, which is why we consider this question not to be relevant for our scheme. 

5. Two questions stand out because these are the questions that we aim to answer, namely if, and if so which blockchain is needed in a particular scenario. These two questions, which we label ‘D’, are ‘Traditional approach results in consistency loss?’, and ‘Can other technologies offer a solution?’. Again, we exclude these particular questions from our scheme, as they do not differentiate between technologies. 

**Including the questionnaires.** The questionnaires consist of a list of questions that must be answered affirmatively to determine if blockchain may be a suitable solution However, only the schemes by Greenspan and Deloitte state that all questions must be answered affirmatively for this technology to be useful. Therefore, because of the schemes boolean end states, these can be considered a flow diagram, too. The questionnaires by Capgemini, PWC and Meunier provide an approximation of the number of questions that must be answered affirmative, making it unclear when exactly blockchain is useful. 

From all questionnaires we can conclude that there are two end states, similar to scheme model 1. Although the questionnaires do not follow a particular flow, their questions can be classified, similar to the questions made in the flow diagrams. Therefore, we include their questions in Table 2, too. 

8 

**Summary.** From Table 2 we observe that most questions are process questions. Moreover 25 out of the 30 schemes contain questions that do not contribute to the overall question the scheme aims to answer. Furthermore, none of the schemes address all tech type questions. This suggests the need for a new scheme. 

## **3.6 Inconsistency between schemes** 

There are clear contradictions between some of the schemes: these schemes come to different conclusions based on identical answers to the questions used in the schemes. Below we give some examples of contradictions we observed. 

**Comparison 1: Cooke vs. Gardner.** We present our results in Table 3. From this table we observe that making similar decisions in the schemes may lead to different answers. The difference can be explained by the additional question by Cooke, namely ‘are writers interests unified?’. Cooke considers this a relevant question, whereas Gardner’s scheme omits this question. As discussed in the previous paragraph, we consider this question not to be relevant for deciding which scheme to use as one must assume that writers interest always are misaligned. 

**Comparison 2: W¨ust vs. Hyperledger.** In Table 4 we compare the two schemes of Hyperledger and W¨ust in deciding which type of blockchain could be used. In this comparison a difference in terminology appears, as the scheme by W¨ust is more fine-grained. Whereas W¨ust uses a combination of two axis (permissionless/permissioned, and, public/private) to describe blockchain, Hyperledger uses only two terms (either permissioned, or public). Here, the Hyperledger scheme could be improved by using similar end states as W¨ust. 

**Comparison 3: IBM vs. Verslype.** The IBM scheme suggests that working with complex business logic may be an argument for using a blockchain. In contrast, Verslype suggests that simple business rules may be an argument for using a blockchain. Clearly, these two schemes contradict each other. It is not clear which scheme is correct, as there is a lack of description of what this specific question means. A possible explanation for the apparent contradiction is that the two schemes consider different types of blockchain. Complex business rules can be, to some extent, captured by smart contracts. Therefore, the IBM scheme is probably considering a blockchain similar to Ethereum that supports smart contracts. However, not all blockchains can deal with complex smart contracts; for instance, Bitcoin does not. Therefore, the scheme by Verslype is probably considering a blockchain as used in Bitcoin. 

**Summary.** These comparisons show that inconsistencies between schemes may be explained by several factors. First, the comparison between the schemes of Cooke and Gardner show clear contradictions. Second, the comparison between W¨ust and Hyperledger shows that there is a difference in granularity of the end state description. Finally, some inconsistencies between schemes may be ex- 

9 

**Table 2.** Scheme questions classification 

|**No. **|**Question**|**Class.**|
|---|---|---|
|1|Traditional approach is insufcient?[6,10,17,21]|D|
|2|Can other technologies ofer a solution?[2,6,31,49]|D|
|3|Aiming to remove third parties?[24,33,57]|P|
|4|Are you working with digital assets?[7,33]|P|
|5|Where is consensus determined?[4,5,8,19,23,24,29,43,45]|P|
|6|Do you need censorship resistance?[49]|E|
|7|How is the incentive structure determined?[34]|E|
|8|Are there contractual relations?[6,14,31,33,45]|P|
|9|Rules of tx do not change frequently?[13]|P|
|10|Sensitive identifers stored?[2,11,14,21]|P|
|11|Requires a market approach?[14]|P|
|12|Looking to reduce costs?[14]|P|
|13|Looking to improve discoverability?[14]|P|
|14|Is there a real (business) problem?[2,6,31]|P|
|15|Can participants adapt?[2,6,29]|P|
|16|Do the benefts justify the cost of adoption?[2]|P|
|17|Is this a ’blockchains are free’ play?[2]|P|
|18|Need an immutable log?[11,13,29,32]|E|
|19|Are there relative simple business rules?[14,17,29]|P|
|20|Many participants transacting?[29,31]|P|
|21|Is data integrity required?[7]|P|
|22|Do you need to share operational data?[7]|P|
|23|Are there transaction rules set?[12]|P|
|24|Who stands behind the assets?[12]|P|
|25|Can the project be open sourced?[21]|P|
|26|Participants trust each other?[4,5,7,8,12,16,20,23,29,33,45,49,56]|P|
|27|Participants interests aligned?[4,5,8,13,18,23,31,32,33,43]|P|
|28|Need a database?[4,5,7,8,10,11,12,13,16,18,20,23,24,32,33,43,45,47,56]|T|
|29|Can you use a TTP?[2,4,5,8,10,11,12,16,17,20,21,23,29,31,33,34,43,45,56,57]|T|
|30|Shared write access?[4,5,8,10,11,12,13,14,16,17,18,24,33,34,43,47,49,56]|T|
|31|Participants known?[4,5,6,8,12,18,23,31,33,34,45,47,56]|T|
|32|Need to control functionality?[4,8,19,23,33,43,45,49]|T|
|33|Public transactions?[2,4,5,7,8,17,18,19,14,21,29,32,33,43,45,47,49]|T|
|34|Is there transaction interaction?[10,12,21,24]|T|
|35|Do you need high transaction throughput?[14,17,21,24,29,31,33,45]|L|
|36|Do you need to store large transactional data?[21,33]|L|



10 

**Table 3.** Comparing scheme choices of Cooke, and Gardner 

||**Cooke**|**Gardner**|
|---|---|---|
|**Question**|**Answer**||
|Do you need a database?|Yes|Yes|
|Are there multiple writers?|Yes|Yes|
|Are writers trusted?|Yes|Yes|
|**Conclusion**|Undetermined.|You don’t need a blockchain|



**Table 4.** Scheme end state comparison between W¨ust and Hyperledger 

|**W¨ust**<br>**Hyperledger**|
|---|
|Transaction visibility: Yes<br>Yes|
|Leads to:<br>Public permissioned blockchain Public blockchain|
|Transaction visibility: No<br>No|
|Leads to:<br>Private permissioned blockchain Permissioned blockchain|



plained by the schemes considering different types of blockchain solutions, as we assume is the case in the contradiction between IBM and Verslype. 

## **4 A New Scheme** 

In this section we propose a new scheme that is based on the three scheme models identified in Section 3. Our scheme aims to answer three questions: 

1. Should you use a blockchain? (scheme model 1). 

2. If so, which type of blockchain is best? (scheme model 2). 

3. If not, which alternative database technology is best? (scheme model 3). 

We include alternative technologies in our scheme, and we focus only on the questions that differentiate between technologies. Because of this, our scheme aims to replace all 30 schemes. 

## **4.1 Scheme questions and end states** 

In explaining our scheme, we use our terminology from Section 2. Our new scheme starts with the need for storing state (1, see Figure 2). If indeed a database is needed and there exists only a single writer (2) that performs state updates, a central database (end state II, see Figure 2) can be used. 

If, however, there are multiple writers (2) and there exists the need to control functionality (3) by a specific party a shared central database (III) should be used. Here we assume that there exists such a specific party, and that the writers trust this party. Controlling database functionality may include setting the rules on how database permissions are set (such as create, store, delete), how the data is stored in the database (a relational database or an object oriented database), or how the database can be queried (e.g. ServerSQL, or MySQL). Similarly, if all participants agree that a third party (4) provides states updates, also a 

11 

shared central database should be used. Note that we omit the question ‘Is public verifiability required?’, in contrast to, for example, W¨ust and Gervais [56], as we consider this to be a design question. In particular for blockchain, this question is inherent to the technology. For all other technologies some form of public verifiability could be present, for example by giving auditors access to the ledger. 

Thus, so far we consider that there is a need to store state and multiple participants are present that do not wish to use a single party for state updates. 

The next question is about transaction interaction. If no transaction interaction (5) is required, a distributed database could be used, for example the cloud storage network Storj (IV) [30]. 

If transaction interaction is required, participants are known (e.g. through a certificate authority) (6), and anyone can join the network (7), again a distributed ledger could be used, for example Ripple (V). When a form of access control (7) is in place, still, a distributed ledger can be used, for example Corda (VI). Note that, in principle, a blockchain _could_ be used in these cases (IV, V, VI). However, other technologies are present that do not lack the current drawbacks of blockchain. As one of the anonymous reviewers pointed out: “Blockchains are often sufficient but not often necessary”. 

If participants are unknown, then blockchain may provide a solution. Here, our scheme is in line with Perlman [50] who states that a blockchain can achieve consensus amongst a consortium of unknown participants. Our scheme also takes some of the current limitations of blockchain into account. Currently, blockchain is limited in processing a large number (a ball park figure is greater than 2000 transactions per second) of transactions (8). and is not fit for storing large amounts (e.g. Tera-bytes) of transactional data (9). Although current research in scalability has shown significant improvements, for example Omniledger [42], there are currently no real life implementations on a global scale. Then, according to our scheme, there is currently no solution available (VII). However, if these two properties do not matter, then a public permissionless blockchain (VIII) should be used. 

## **5 Discussion** 

Following our scheme, blockchain is only needed where there exists a group of unknown participants that wish to reach consensus. Blockchain _could_ be used in any case where there exists a need for a database. This may give rise to the notion of public permissioned blockchains and private permissioned blockchains, which are in essence a shared database [3][45]. However, using blockchain in those cases where alternative technologies are suggested in our scheme may not be the best choice, considering the issues blockchain currently has, as discussed in Section 2. This is why our scheme includes only one type of blockchain, namely the ‘classic’ public and permissionless blockchain. 

Schemes closely related to our work, for example W¨ust and Gervais, Peck, Pahl, and Lin et al., address the question ‘do you need a blockchain?’. Their 

12 

**==> picture [325 x 469] intentionally omitted <==**

**----- Start of picture text -----**<br>
1.Need to store state? I. Don't use a DB<br>No<br>      Yes<br>2.Is there a single writer? Yes II. Central DB<br>      No<br>3.Need to control<br>III. Shared central DB<br>functionality?       Yes<br>      No<br>4.Can you use<br>a third party?<br>      No<br>IV. Distributed DB<br>5.Transaction interact.?<br>      No (e.g. Storj)<br>V. Distributed ledger<br>      Yes        Yes (e.g. Ripple)<br>7.Can anyone join VI. Distributed ledger<br>6.Participants known? Yes the network? No (e.g. Corda)<br>      No<br>8.Tx throughput VII. Currently no<br>matters?       Yes solutions available<br>      No<br>9.Store large<br>amount of data?<br>      No VIII. Public permissionless<br>blockchain<br>**----- End of picture text -----**<br>


**Fig. 2.** Scheme for determining which type of database is appropriate 

schemes suggest either to use a type of blockchain, or not to use a blockchain. This, however, is misleading as the scheme suggests that blockchain is needed, whereas other technologies are available. Such technologies do not have the cur- 

13 

rent limitations of blockchain. In fact, these technologies have been tested over time and have proven to provide a functionality that is desired. We argue that the end states of decision schemes should at least include technologies that provide the desired functionality, and where possible without the limitations of blockchain. Therefore, we argue that the schemes that do not include alternative technologies are incomplete and hence wrong. 

Also, in our analyses we labeled a large number of scheme questions as ‘process’, as these questions do not contribute to the overall question the scheme aims to answer, as discussed in Section 3. The questions labeled ‘process’, therefore, should not be included in these decision making schemes. Additionally, we labeled 9 questions as ‘tech’ meaning that these in fact do contribute to any of the scheme goals. We used these 9 questions and created a new scheme, together with the end states of alternative technologies, see Figure 2. As our scheme includes all relevant questions of the identified schemes and questionnaires, includes end states that suggest alternative technologies, and our scheme determines if blockchain should be used, we argue that our scheme can replace the identified 30 schemes. 

## **6 Future Work** 

Our scheme can be used for determining if blockchain is needed from a technical perspective. Our scheme can be extended with non-technical questions that drive the adoption of blockchain, for example philosophical beliefs and economic incentives. Furthermore, our scheme provides an overview of various types of distributed ledger technologies. This could be expanded with more distributed ledger technologies. Additionally, our scheme could be expanded by including current issues of distributed ledgers. Additionally, a further analysis on the consensus between the schemes can be made. 

The concept of trust also merits further research. Trust is a important concept which is not really considered in our scheme (except in the question ‘can you use a third party?’). It is clear that trust shifts with the introduction of blockchain. Indeed, replacing trust with cryptographic proofs was one of the motivations behind Bitcoin. Still, introducing a blockchain does not remove all need for trust, as it may also introduce new types of trust. 

## **7 Conclusion** 

With a growing global interest in blockchain, many decision schemes have been proposed to determine if blockchain is suitable, and if so which type. This paper analyzed 30 of such schemes. We classified these schemes based on which of the following three questions they try to answer: Should you use a blockchain? If so, which blockchain variant is best? If not, which alternative is best? 

Our analysis of these schemes shows that over half of the schemes contain questions that do not contribute to the goal of the scheme. Furthermore, many schemes are biased in favor of blockchain-based solutions, as their end states only 

14 

consider some type of blockchain. Such schemes seem to disregard alternative solutions and suggest that blockchain is needed in most scenarios – incorrectly in our opinion, if one takes into account that these alternatives lack some of the drawbacks and limitations of blockchain-based solutions. Of course, we are not the first to argue that for many proposed applications blockchain-based solutions are not the best solution, or not even suitable at all [12,36,37,40,44,49,50,53,54]. 

Furthermore, like Birch et al. [34] and Maull [45], we observe that there exists a Babylonian confusion with regards to the term blockchain. This is why we put the term blockchain into perspective alongside other database technologies, before our analysis of the schemes. 

Our analysis shows that there are inconsistencies between the schemes, where the same decisions lead to different outcomes, or, conversely, similar outcomes can be reached with opposing decisions. There clearly is a need to improve these decision schemes. 

We argued that if one uses a blockchain-based solution, only a public permissionless blockchain really makes sense. Although other blockchain types _could_ be used in some scenarios, alternative technological solutions are then always a better choice as they lack some of the downsides and limitations of blockchain. Finally, our scheme is a practical guide for blockchain initiatives that need to determine which technology is suitable for a particular scenario. 

## **Acknowledgements** 

We would like to thank the anonymous reviewers for their constructive feedback. 

## **References** 

1. Alex Lee. Blockchain patent filings dominated by financial services industry. `http://patentvue.com/2018/01/12/blockchain-patent-filings-dominatedby-financial-services-industry/` . 

2. Antony Lewis. Blockchain cheat sheet v0.1. `https://bitsonblocks.files. wordpress.com/2016/01/2016-01-26-fintech-finals-hk.pdf?lipi=urn%3Ali% 3Apage%3Ad_flagship3_pulse_read%3BQH2LGbmzRKy9rZY9N4nfsA%3D%3D` . 

3. Arvind Narayanan. ”Private blockchain” is just a confusing name for a shared database. `https://freedom-to-tinker.com/2015/09/18/private-blockchainis-just-a-confusing-name-for-a-shared-database/` . 

4. Bart Suichies. Why blockchain must die in 2016. `https://medium.com/ @bsuichies/why-blockchain-must-die-in-2016-e992774c03b4` . 

5. Best of ICOs. The blockchain test. `https://hackernoon.com/should-we-ico51964dccadbe` . 

6. Brian Henkel. Beginning blockchain: Key questions to getting started. `https://www.ca.com/en/blog-mainframeai/beginning-blockchain-keyquestions-to-getting-started.html` . 

7. Cap Gemini - SAI trends. `https://sai.be/UserContent/FPAX7KUQ45F7B9C4KPZQ_ SAI%20trends%20-%20December%202017%20-%20handout.pdf` . 

8. Daniil Saiko. Blockchain technology. `http://www.cambridgefx.com/blog/ blockchain-technology/` . 

15 

9. David Schwartz. The Ripple protocol consensus algorithm. `https://ripple.com/ files/ripple_consensus_whitepaper.pdf` . 

10. Deloitte. I. Blockchain A new model for health information exchanges. `https: //www.semanticscholar.org/paper/I.-Blockchain%E2%80%94A-New-Modelfor-Health-Information-I../b99277c3eecfe6d3dd784fe572a45780ffd040e2` . 

11. DHS. Most companies don’t need blockchain. `https://medium.com/@sbmeunier/ when-do-you-need-blockchain-decision-models-a5c40e7c9ba1` . 

12. Gideon Greenspan. Avoiding the pointless blockchain project. `https://www. multichain.com/blog/2015/11/avoiding-pointless-blockchain-project/` . 

13. Hyperledger. Blockchain decision path. `https://steemit.com/ethereum/@whd/ fast-method-to-rate-ico-basing-on-hyperledger-course-at-edx` . 

14. IBM. How to decide when to use blockchain. `https://www.ibm.com/ developerworks/community/blogs/gcuomo/resource/BLOGS_UPLOADED_IMAGES/ HowToDecideWhenToUseBlockchain.jpg` . 

15. IDC. New IDC spending guide sees worldwide blockchain spending growing to $9.7 billion in 2021. `https://www.idc.com/getdoc.jsp?containerId=prUS43526618` . 

16. Jeremy Gardner. Do you need a blockchain? `https://twitter.com/ Disruptepreneur/status/755857596423077888/photo/1?ref_src=twsrc%5Etfw` . 

17. Kristof Verslype. Beslissingsmodel: Wanneer blockchain gebruiken? `https://www. smalsresearch.be/beslissingsmodel-wanneer-blockchain-gebruiken/` . 

18. Kunal Nandwani . Do you really need to use blockchain for your application? `https://www.linkedin.com/pulse/when-use-b-word-can-blockchainactually-help-kunal-nandwani/` . 

19. Laurence Cooke. Blockchain technology. `http://www.cambridgefx.com/blog/ blockchain-technology/` . 

20. Lixar. Blockchain part 2. `https://lixar.com/lixar-blog/tech/blockchainpart-2/` . 

21. Mahesh Chand. Do you need a blockchain. `https://www.c-sharpcorner.com/ article/do-you-need-a-blockchain2/` . 

22. Mike Hearn. Corda: A distributed ledger. `https://docs.corda.net/_static/ corda-technical-whitepaper.pdf` . 

23. Mike Quindazzi. Do you really need a blockchain? `https://twitter.com/ mikequindazzi/status/787760892783894528` . 

24. PWC. Blockchain: The $5 billion opportunity for reinsurers. `https://www. pwc.com/gx/en/financial-services/publications/assets/blockchain-forreinsurers.pdf` . 

25. R3. Corda. `https://docs.corda.net/` . 

26. Richard Gendal Brown, James Carlyle, Ian Grigg, Mike Hearn. Corda: An introduction. `https://docs.corda.net/_static/corda-introductory-whitepaper.pdf` . 

27. Ripple. One frictionless experience to send money globally. `https://ripple.com/` . 28. Ripple. Reaching consensus in the XRP ledger. `https://ripple.com/build/ reaching-consensus-xrp-ledger/` . 

29. Sebastien Meunier. When do you need blockchain? Decision models. `https://medium.com/@sbmeunier/when-do-you-need-blockchain-decisionmodels-a5c40e7c9ba1` . 

30. Shawn Wilkinson, Tome Boshevski,Josh Brandoff, James Prestwich, Gordon Hall, Patrick Gerbes, Philip Hutchins, Chris Pollard, Vitalik Buterin. Storj. A peer-topeer cloud storage network. `https://storj.io/storj.pdf` . 

31. Thomas Mueller. Will blockchain solve my business problem? `https: //medium.com/contractus/do-i-need-a-blockchain-for-my-businessproject-8f8cada7f3ac` . 

16 

32. Verified ICOs. Is a blockchain really required? `https://medium.com/ @VerifiedICOs/is-a-blockchain-really-required-1a68c7791fa1` . 

33. World Economic Forum. Blockchain beyond the hype. `http://www3.weforum.org/ docs/48423_Whether_Blockchain_WP.pdf` . 

34. David Birch, Richard G Brown, and Salome Parulava. Towards ambient accountability in financial services: Shared ledgers, translucent transactions and the technological legacy of the great financial crisis. _Journal of Payments Strategy & Systems_ , 10(2):118–131, 2016. 

35. Joseph Bonneau, Andrew Miller, Jeremy Clark, Arvind Narayanan, Joshua A Kroll, and Edward W Felten. Sok: Research perspectives and challenges for bitcoin and cryptocurrencies. In _Security and Privacy (SP), 2015 IEEE Symposium on_ , pages 104–121. IEEE, 2015. 

36. Mark Buitenhek. Understanding and applying blockchain technology in banking: Evolution or revolution? _Journal of Digital Banking_ , 1(2):111–119, 2016. 

37. Lemuria Carter and Jolien Ubacht. Blockchain applications in government. In _Proceedings of the 19th Annual International Conference on Digital Government Research: Governance in the Data Age_ , page 126. ACM, 2018. 

38. Daniel Conte de Leon, Antonius Q Stalick, Ananth A Jillepalli, Michael A Haney, and Frederick T Sheldon. Blockchain: properties and misconceptions. _Asia Pacific Journal of Innovation and Entrepreneurship_ , 11(3):286–300, 2017. 

39. Ittay Eyal and Emin G¨un Sirer. Majority is not enough: Bitcoin mining is vulnerable. In _International conference on financial cryptography and data security_ , pages 436–454. Springer, 2014. 

40. David Gerard. _Attack of the 50 Foot Blockchain: Bitcoin, Blockchain, Ethereum & Smart Contracts_ . CreateSpace Independent Publishing Platform, 2017. 

41. Tommy Koens and Erik Poll. What Blockchain Alternative Do You Need? 2018. Manuscript submitted for publication. 

42. Eleftherios Kokoris-Kogias, Philipp Jovanovic, Linus Gasser, Nicolas Gailly, and Bryan Ford. Omniledger: A secure, scale-out, decentralized ledger. _IACR Cryptology ePrint Archive_ , 2017:406, 2017. 

43. Yu-Pin Lin, Joy R Petway, Johnathen Anthony, Hussnain Mukhtar, Shih-Wei Liao, Cheng-Fu Chou, and Yi-Fong Ho. Blockchain: The evolutionary next step for ICT E-agriculture. _Environments_ , 4(3):50, 2017. 

44. Juri Mattila, Timo Sepp¨al¨a, and Jan Holmstr¨om. Product-centric information management: A case study of a shared platform with blockchain technology. 2016. 

45. Roger Maull, Phil Godsiff, Catherine Mulligan, Alan Brown, and Beth Kewell. Distributed ledger technology: Applications and implications. _Strategic Change_ , 26(5):481–489, 2017. 

46. Satoshi Nakamoto. Bitcoin: A Peer-to-Peer Electronic Cash System. 2008. `https: //bitcoin.org/bitcoin.pdf` . 

47. Claus Pahl, Nabil El Ioini, and Sven Helmer. A decision framework for blockchain platforms for iot and edge computing. In _International Conference on Internet of Things, Big Data and Security_ , 2018. 

48. Kasey Panetta. Top trends in the gartner hype cycle for emerging technologies, 2017. `https://www.gartner.com/smarterwithgartner/top-trendsin-the-gartner-hype-cycle-for-emerging-technologies-2017/` . 

49. Morgen E Peck. Blockchain world. Do you need a blockchain? This chart will tell you if the technology can solve your problem. _IEEE Spectrum_ , 54(10):38–60, 2017. 

50. Radia Perlman. Blockchain: Hype or hope? _;login:_ , 42(2):68–72, 2017. 

17 

51. Gareth W Peters and Efstathios Panayi. Understanding modern banking ledgers through blockchain technologies: Future of transaction processing and smart contracts on the internet of money. In _Banking Beyond Banks and Money_ , pages 239–278. Springer, 2016. 

52. Marc Pilkington. 11 blockchain technology: principles and applications. _Research handbook on digital transformations_ , page 225, 2016. 

53. Michael Pisa and Matt Juden. Blockchain and economic development: Hype vs. reality. _Center for Global Development Policy Paper_ , 107, 2017. 

54. Deepak Puthal, Nisha Malik, Saraju P Mohanty, Elias Kougianos, and Chi Yang. The blockchain as a decentralized security framework. _IEEE Consumer Electronics Magazine_ , 7(2):18–21, 2018. 

55. Don Tapscott and Alex Tapscott. _Blockchain revolution: how the technology behind bitcoin is changing money, business, and the world_ . Penguin, 2016. 

56. Karl W¨ust and Arthur Gervais. Do you need a blockchain? _IACR Cryptology ePrint Archive_ , 2017:375, 2017. 

57. Xiwei Xu, Ingo Weber, Mark Staples, Liming Zhu, Jan Bosch, Len Bass, Cesare Pautasso, and Paul Rimba. A taxonomy of blockchain-based systems for architecture design. In _Software Architecture (ICSA), 2017 IEEE International Conference on_ , pages 243–252. IEEE, 2017. 

