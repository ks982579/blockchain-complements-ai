2025 IEEE International Symposium on Parallel and Distributed Processing with Applications (ISPA) 

# Decentralized Multi-Agent System with Trust-Aware Communication 

1[st] Yepeng Ding _Hiroshima University_ Higashihiroshima, Japan yepengd@acm.org 

2[nd] Ahmed Twabi _Hiroshima University_ Higashihiroshima, Japan iman-twabi@hiroshima-u.ac.jp 

3[rd] Junwei Yu _The University of Tokyo_ Tokyo, Japan yujw@satolab.itc.u-tokyo.ac.jp 

4[th] Lingfeng Zhang _The University of Tokyo_ Tokyo, Japan zhang-lingfeng936@g.ecc.u-tokyo.ac.jp 

5[th] Tohru Kondo 6[th] Hiroyuki Sato _Hiroshima University National Institute of Informatics_ Higashihiroshima, Japan Tokyo, Japan tkondo@hiroshima-u.ac.jp schuko@nii.ac.jp 

_**Abstract**_ **—The emergence of Large Language Models (LLMs) is rapidly accelerating the development of autonomous multiagent systems (MAS), paving the way for the Internet of Agents. However, traditional centralized MAS architectures present significant challenges, including single points of failure, vulnerability to censorship, inherent scalability limitations, and critical trust issues. We propose a novel Decentralized Multi-Agent System (DMAS) architecture designed to overcome these fundamental problems by enabling trust-aware, scalable, and censorshipresistant interactions among autonomous agents. Our DMAS features a decentralized agent runtime underpinned by a blockchainbased architecture. We formalize a trust-aware communication protocol that leverages cryptographic primitives and on-chain operations to provide security properties: verifiable interaction cycles, communication integrity, authenticity, non-repudiation, and conditional confidentiality, which we further substantiate through a comprehensive security analysis. Our performance analysis validates the DMAS as a scalable and efficient solution for building trustworthy multi-agent systems.** 

_**Index Terms**_ **—multi-agent system, blockchain, agent-to-agent, large language model, self-sovereign identity.** 

## I. INTRODUCTION 

The rapid advancements in Large Language Models (LLMs) [1]–[4] have opened unprecedented avenues for creating highly autonomous and intelligent agents. These LLM-augmented agents possess remarkable capabilities in understanding natural language, performing complex reasoning, planning intricate sequences of actions, and engaging in sophisticated communication. As the complexity of real-world problems outgrows the capacity of single agents, the paradigm is naturally shifting towards Multi-Agent Systems (MAS) [5]–[7], where multiple agents collaborate to achieve shared or individual goals through interaction and coordination. This evolution envisions the Internet of Agents (IoA) [8], a future where a vast network of interoperable AI agents can discover, interact with, and provide services to each other and human users across diverse domains. 

While the promise of such interconnected agent ecosystems is immense, current practical implementations of LLM- 

This research was supported by JSPS KAKENHI Grant Number 25K21201. 

augmented MAS often suffer from fundamental limitations inherent to centralized architectures [9], [10]. Relying on central servers or authorities for agent discovery, communication, and trust management introduces several critical vulnerabilities: a _single point of failure_ leading to system fragility, susceptibility to _censorship and arbitrary control_ by a central entity, significant _scalability bottlenecks_ as the number of agents and complexity of interactions grow, and pervasive _trust issues_ . In a centralized MAS, users and agents must implicitly trust the central operator to ensure data integrity, privacy, fair resource allocation, and honest service provision. Furthermore, verifying compliance with agreed-upon protocols or holding agents accountable for their outputs becomes challenging without transparent and immutable records. These challenges collectively hinder the realization of a robust, resilient, and equitable IoA. 

Motivated by addressing these challenges, we introduce a novel **Decentralized Multi-Agent System (DMAS)** architecture. Our proposed DMAS leverages blockchain primitives [11]–[13] and the principles of self-sovereign identity [14]– [16] to enable trust-aware, scalable, and censorship-resistant interactions among autonomous agents. By decentralizing core functionalities, the DMAS provides a foundational framework for agents to operate and collaborate without reliance on a single, trusted third party. Our main contributions are summarized as follows: 

- We propose a decentralized multi-agent system architecture that integrates a _Decentralized Agent Runtime_ and a _Trust-Aware Communication Protocol_ to secure agent discovery and interactions while preserving performance. 

- We provide a semi-formal analysis of DMAS’s security properties. 

- We present a comprehensive performance analysis of the DMAS, demonstrating its high scalability and comparable efficiency with the centralized multi-agent system. 

2158-9208/25/$31.00 ©2025 IEEE 1439 DOI 10.1109/ISPA67752.2025.00198 

## II. RELATED WORK 

The integration of large language models (LLMs) into multiagent systems (MAS) has substantially advanced the sophistication of inter-agent collaboration. Recent surveys emphasize that LLM-augmented MAS harness the collective intelligence of multiple specialized agents to achieve capabilities that surpass those of individual agents, effectively addressing challenges such as hallucinations and single points of failure [7]. By engaging in dialogue, debate, and mutual verification, multiple LLM agents emulate the cooperative dynamics of human teams in complex problem-solving, leading to improved reasoning accuracy and task performance [17]. 

Recent research has increasingly focused on reflective coordination paradigms, wherein agents collaboratively regulate their behavior through self-evaluation and continual learning [18]. For instance, COPPER [19] introduces a self-reflection mechanism, either embedded within individual agents or implemented via a shared mediator—that iteratively refines agent prompts using a learned reflector model. This model is trained via counterfactual reinforcement learning (specifically, Proximal Policy Optimization), which rewards agents for generating useful reflections. As a result, the system dynamically optimizes role assignments and interaction patterns, mitigating creditassignment issues by attributing success to effective reflection rather than static agent roles. 

LLM-based multi-agent systems have demonstrated effectiveness across diverse domains, including software engineering [20], [21] and medical science [22], [23], showcasing their potential for tackling complex, interdisciplinary tasks that require long-term planning and specialized knowledge [7], [24]. However, many of these applications adopt centralized coordination architectures, which introduce security vulnerabilities and limit scalability, particularly in untrusted or decentralized environments. 

## III. DECENTRALIZED MULTI-AGENT SYSTEM 

We formulate a decentralized multi-agent system (DMAS) to enable trust-aware and scalable interactions among autonomous agents without reliance on centralized control. 

## _A. Decentralized Agent Runtime_ 

The DMAS introduces a decentralized runtime architecture to facilitate secure and scalable agent-to-agent interactions. This architecture, as depicted in Figure 1, comprises proxy agents and service agents, disseminated across decentralized environments. 

_1) Agent Type:_ The runtime primarily comprises two agent types: Proxy Agents (PAs) and Service Agents (SAs). These agents operate across both on-chain and off-chain environments, establishing a robust and privacy-preserving communication paradigm. 

_a) Proxy Agents:_ PAs serve as the user’s primary interface to the decentralized agent runtime, acting as intelligent intermediaries responsible for routing user requests to appropriate SAs based on their advertised capabilities. Each user within 

Fig. 1. Overview of the decentralized agent runtime. 

the DMAS ecosystem is mandated to host a dedicated Proxy Agent. PAs are designed to handle complex user queries by potentially orchestrating interactions with multiple SAs. 

_b) Service Agents:_ SAs form the computational backbone of the DMAS. They are designed to execute specific tasks or provide specialized services. SAs are responsible for processing requests originating from either PAs or other SAs and generating responses commensurate with their defined capabilities. Critically, SAs operate autonomously off-chain, performing computations or accessing external data sources as required by their service definitions. This might include accessing realworld APIs, performing complex data analysis, or executing machine learning models. 

_2) Verifiable Agent Registry:_ We introduce Self-Sovereign Identity (SSI) principles for robust agent identity and capability management within the DMAS. Correspondingly, we construct a Verifiable Agent Registry (VAR) implemented as a smart contract operating on blockchains. The VAR is central to establishing trust and transparency in agent interactions by providing a public, immutable record of agent identities and capabilities. 

_a) DID Management:_ Each PA and SA is assigned a unique Decentralized Identifier (DID). These DIDs are cryptographically verifiable and managed by a Verifiable Data Registry (VDR). The VDR is implemented as a smart contract on a public blockchain (e.g., Ethereum, Polygon), ensuring immutability, transparency, and censorship resistance in DID registration, updates, and revocation. The VDR acts as a public, tamper-proof directory for all registered SAs, providing a foundational layer of trust, which aligns with blockchain’s core tenets of decentralization, guaranteeing that an agent’s identity cannot be arbitrarily altered or censored. 

_b) DID Resolution:_ A core function of the VDR is to enable the resolution of an SA’s DID to a corresponding capability schema. This schema, structured in JSON-LD, contains a machine-readable description outlining the services, functionalities, input/output parameters, and operational parameters offered by the SA. This on-chain resolution mechanism provides 

1440 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

a trusted and verifiable source of information regarding agent capabilities, which is crucial for informed peer discovery and secure interaction initiation. The cryptographic link between the DID and its associated capability schema ensures that the advertised capabilities are genuinely linked to the registered agent, preventing impersonation or misrepresentation and fostering a secure decentralized runtime. 

_3) Agent Hosting:_ While DIDs and their resolution occur on-chain, the SAs themselves, along with their computational capabilities and execution environments, are hosted off-chain. This design choice optimizes for scalability and performance, preventing the blockchain from becoming a bottleneck for heavy computational tasks and enabling complex, long-running services. An SA, once its DID is resolved to its capability schema, can be effectively routed by a PA using the connection information (e.g., IP address, API endpoint, communication protocol details) embedded within that schema. This off-chain communication leverages established network protocols (e.g., HTTP/S, WebSockets) for efficient data exchange, highlighting the hybrid nature of the DMAS. The blockchain acts as a trust anchor and coordination layer, while the actual heavy lifting is performed off-chain. 

_4) Agent Memory:_ Due to the inherent space sensitivity and cost associated with on-chain storage, memory management within the DMAS is exclusively enforced on the PAs. PAs are responsible for maintaining conversational context and assembling received responses from SAs off-chain. This accumulated context is then utilized to inform and contextualize subsequent requests to SAs, enabling multi-turn interactions and complex task orchestration, similar to how an LLM maintains conversational state. Conversely, SAs are designed to be memoryless (stateless) from the perspective of persistent conversational history. Each request processed by an SA is treated as an independent operation, with all necessary context provided by the requesting PA. Our stateless design for SAs simplifies their implementation, enables parallel processing, and minimizes their on-chain footprint, which contributes to the overall scalability and efficiency of the DMAS. 

## _B. Trust-Aware Protocol_ 

We formulate a trust-aware protocol for the DMAS to delineate the formalized interaction mechanisms between a user’s designated Proxy Agent, denoted as _u_ , and a collective of autonomous Service Agents, represented as the set _S_ = _{s_ 1 _, s_ 2 _, . . . , sm}_ . This protocol is meticulously designed to imbue the DMAS with properties of verifiable trust, accountability, and conditional data exchange, leveraging the immutability and transparency afforded by a distributed ledger, B. The entire interaction sequence is predicated upon cryptographic primitives and on-chain attestations, thereby minimizing reliance on centralized authorities and fostering true decentralization. 

_1) Service Discovery:_ A service discovery process is initiated by a user request, denoted by _▷_ , which is handled by a PA _u_ . Agent _u_ is tasked with continuously communicating with a set of suitable SAs capable of fulfilling the requirements 

specified by _▷_ , until a termination condition _τ_ is satisfied. This process goes beyond simple lookup, involving dynamic interaction and delegation. 

To address the scalability challenges posed by the potentially large cardinality of _S_ , the service discovery mechanism employs a delegation pattern. In this pattern, each SA is equipped with the ability to forward incoming requests to other candidate SAs, based on the user request _▷_ and the contextual state Γ( _u_ ) maintained by the PA _u_ . If an SA responds with a directive to forward the request, we classify the result as nonterminal; otherwise, if the response terminates further routing, it is classified as terminal. This delegation allows for a flexible and dynamic discovery process, where specialized SAs can guide the PA to more relevant services. We present two common discovery strategies based on the delegation pattern: depthfirst strategy and breadth-first strategy, though the discovery strategy can be customized by PAs based on specific application requirements or heuristic rules. 

_a) Depth-First Strategy:_ The depth-first strategy is formalized in Algorithm 1. Initially, _u_ selects a small candidate set of SAs via _FirstSelect_ ( _u, ▷_ ), a predefined method such as a hardcoded list, semantic similarity techniques (e.g., LLM-based matching for richer understanding of service descriptions), or reputation scores. It then engages in iterative communication with each selected SA, recursively following routing suggestions until either no further candidates remain or the termination condition _τ_ is satisfied. The communication procedure between _u_ and an SA _s_ , denoted _Com_ ( _u, s_ ), is described in detail in Section III-B2. This strategy is suitable for scenarios where a deep, specialized exploration of a service chain is preferred. 

|**Algorithm 1** Depth-First Discovery Strategy|**Algorithm 1** Depth-First Discovery Strategy|
|---|---|
|**Require:** _u_, _▷_, _τ_||
|**Ensure:** _⃗R_: an ordered list of terminal responses||
|_⃗R ←∅_||
|ˆ_S ←FirstSelect_(_u, ▷_)|_▷_LIFO stack with initial candidates|
|**while** _|_ ˆ_S| >_0_∧τ_ =_⇒⊥_**do**||
|_s ←Pop_(ˆ_S_)||
|_r ←Com_(_u, s_)|_▷u_ communicates with _s_|
|**if** _IsTerminal_(_r_) =|_⊤_**then**|
|_R ←R ∪{r}_||
|**else**||
|_Push_(ˆ_S, SA_(_r_)) _▷_Add forwarded candidates to stack||
|**end if**||
|**end while**||
|**return** _⃗R_||



_b) Breadth-First Strategy:_ In contrast to the depth-first strategy, the breadth-first discovery strategy explores the SA space in a level-wise manner via a queue. The PA _u_ initiates the discovery process with an initial set of candidate SAs, selected via _FirstSelect_ ( _u, ▷_ ). Rather than following each routing path deeply, this strategy systematically explores all SAs at the current level before proceeding to the next. The process proceeds iteratively until the termination condition _τ_ is met or no further 

1441 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

candidates remain in the queue. This approach can be beneficial for discovering a wider range of potentially relevant services quickly, suitable for scenarios requiring diverse responses or exploring many options concurrently. 

_c) Termination Condition:_ The termination condition _τ_ governs when the service discovery process should halt. Rather than being a fixed Boolean value, _τ_ is formulated as a predicate function maintained by the PA _u_ . We define _τ_ as a conjunction or disjunction over a configurable set of atomic predicates, such as: fulfillment of the request (e.g., ”received at least three viable options”), the number of SA communications exceeding a predefined threshold, expiration of a wall-clock timeout, or an explicit human intervention signal. 

_2) Verifiable Communication:_ The DMAS leverages a distributed ledger, B, to enable verifiable communications between PAs and SAs. The communication follows a hybrid protocol that records requests and response receipts on-chain, ensuring verifiability and preventing malicious attacks such as repudiation or tampering, which is critical for building trust in a decentralized environment where agents may not inherently trust each other. This protocol comprises three core steps: request commitment, response commitment, and response retrieval, involving the resolution of DIDs via the VAR residing on B to yield capability schemas for available SAs. 

Let _DID_ ( _s_ ) denote the DID of SA _s ∈ S_ , and _CAP_ ( _DID_ ( _s_ )) denote its resolved capability schema. We assume a communication initiated by a PA _u_ to an SA _s ∈ S_ . 

_a) Request Commitment:_ The PA _u_ constructs an offchain request payload, _P_ ( _▷_ ), intended for _s_ . To establish an immutable record of the request initiation and to ensure its integrity, _u_ simultaneously broadcasts a request transaction, _X_ ( _P_ ( _▷_ )) ≜ _⟨DID_ ( _u_ ) _, DID_ ( _s_ ) _, H_ ( _P_ ( _▷_ )) _⟩_ , to the distributed ledger B. _H_ ( _P_ ( _▷_ )) is a cryptographic hash of the off-chain request payload, acting as a unique fingerprint of the request. 

Upon the successful inclusion of _X_ ( _P_ ( _▷_ )) into B, _u_ transmits the detailed off-chain payload _P_ ( _▷_ ) and a reference to _X_ ( _P_ ( _▷_ )) (e.g., the transaction hash or block number) to _s_ via a secure, direct communication channel (e.g., end-to-end encrypted peer-to-peer connection). This ensures that the SA receives both the content of the request and verifiable proof of its on-chain commitment, preventing _u_ from later denying having sent the request. 

_b) Response Commitment:_ Upon receipt of _P_ ( _▷_ ), SA _s_ first verifies its authenticity and validity by querying B to confirm the existence and integrity of _▷_ corresponding to _H_ ( _P_ ( _▷_ )) in the on-chain request commitment. If _H_ ( _P_ ( _▷_ )) matches, _s_ proceeds with the execution of the requested service. Let _◁_ denote the raw response generated by _s_ . 

To ensure conditional access and privacy-preserving delivery of _◁_ , _s_ performs the following operations: 

- 1) Encrypt the raw response using a symmetric encryption key, _κ_ : ¯ _◁_ = _Enc_ ( _◁, κ_ ). 

- 2) Store ¯ _◁_ in an off-chain storage facility, denoted as _D_ (¯ _◁_ ). 

- 3) Construct a response transaction, _X_ ( _◁_ ), and broadcasts it to B, where _X_ ( _◁_ ) ≜ _⟨H_ ( _X_ ( _P_ ( _▷_ ))) _, H_ (¯ _◁_ ) _, η⟩_ , and _η_ 

   - specifies the conditions under which _u_ may obtain the decoding key _κ_ (e.g., payment of a service fee). 

- 4) Notify _u_ with the storage location _D_ (¯ _◁_ ) and the transaction _X_ ( _◁_ ) via a secure, direct communication channel. 

- _c) Response Retrieval:_ Upon receiving the notification for 

- _X_ ( _◁_ ), the PA _u_ retrieves the encrypted response ¯ _◁_ via _D_ (¯ _◁_ ) and follows the operations below: 

- 1) _u_ verifies the integrity of ¯ _◁_ by comparing its cryptographic hash with _H_ (¯ _◁_ ) recorded on B. This ensures that the retrieved encrypted response has not been tampered with during transit or storage; 

- 2) _u_ proceeds to satisfy _η_ . For instance, if _η_ involves a payment, _u_ initiates a payment transaction on B to the specified wallet address in _η_ . The smart contract managing _η_ verifies the fulfillment of the conditions; 

- 3) Once _η_ are verifiably fulfilled on B (e.g., confirmed by the smart contract or by a verifiable credential), _u_ notifies _s_ via a secure, direct communication channel to retrieve _κ_ ¯ = _Enc_ ( _κ, PK_ ( _u_ )), where _PK_ ( _u_ ) is the public key of _u_ . This ensures that only the intended PA, whose public key is known, can decrypt the symmetric key, maintaining confidentiality. 

- 4) Finally, _u_ decrypts _κ_ ¯ with its private key _SK_ ( _u_ ) to obtain _κ_ , and then reveals _◁_ (the raw response) by decrypting ¯ _◁_ using _κ_ : 

## _◁_ = _Dec_ (¯ _◁, Dec_ (¯ _κ, SK_ ( _u_ ))) 

This multi-step response retrieval process ensures verifiability, trust, and privacy throughout the interaction lifecycle, providing a robust framework for secure and auditable agent communications within a decentralized environment. It mitigates common risks associated with off-chain interactions by anchoring critical steps to the immutable ledger. 

## IV. SECURITY ANALYSIS 

We provide a semi-formal security analysis of the DMAS regarding its key security properties. 

**Property IV.1** (Communication Integrity) **.** _The content of requests P_ ( _▷_ ) _and responses ◁ remains unaltered during transmission and storage between a PA u and a SA s._ 

_Proof._ We prove the communication integrity regarding the request and response integrity, respectively. For **Request Integrity** , 

- 1) When _u_ initiates a request, it computes a cryptographic hash _H_ ( _P_ ( _▷_ )) of the off-chain request payload _P_ ( _▷_ ). 

- 2) _H_ ( _P_ ( _▷_ )) is then committed on B as part of the request transaction _X_ ( _P_ ( _▷_ )) = _⟨DID_ ( _u_ ) _, DID_ ( _s_ ) _, H_ ( _P_ ( _▷_ )) _⟩_ . The immutability of B ensures that this committed hash cannot be altered post-recording. 

- 3) Upon receiving _P_ ( _▷_ ) off-chain, _s_ independently computes its hash, _H[′]_ ( _P_ ( _▷_ )), and queries B to retrieve the original committed hash _H_ ( _P_ ( _▷_ )). 

- 4) _s_ verifies the integrity of the received payload by comparing _H[′]_ ( _P_ ( _▷_ )) with _H_ ( _P_ ( _▷_ )). If _H[′]_ ( _P_ ( _▷_ )) = _H_ ( _P_ ( _▷_ )), it indicates tampering, and _s_ can reject the request. 

1442 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

## For **Response Integrity** , 

- 1) After processing the request, _s_ encrypts the raw response _◁_ with a symmetric key _κ_ to produce ¯ _◁_ = _Enc_ ( _◁, κ_ ). 

- 2) _s_ then computes the cryptographic hash _H_ (¯ _◁_ ) of this encrypted response. 

- 3) _H_ (¯ _◁_ ) is committed on B as part of _X_ ( _◁_ ) = _⟨H_ ( _X_ ( _P_ ( _▷_ ))) _, H_ (¯ _◁_ ) _, η⟩_ . Again, B’s immutability guarantees the integrity of this committed hash. 

- 4) _u_ retrieves ¯ _◁_ from the off-chain storage facility _D_ (¯ _◁_ ) and independently computes its hash, _H[′]_ (¯ _◁_ ). 

- 5) _u_ verifies the integrity of the retrieved encrypted response by comparing _H[′]_ (¯ _◁_ ) with _H_ (¯ _◁_ ) from B. A mismatch indicates tampering with the encrypted response. 

Therefore, any unauthorized alteration of request or response payloads, whether in transit or storage, will be detected through cryptographic hash mismatches verified against the immutable ledger, thus ensuring communication integrity. 

**Property IV.2** (Authenticity and Non-Repudiation) **.** _Senders of requests and responses can be verifiably identified, and they cannot falsely deny having originated these messages._ 

- _Proof._ We first prove **Authenticity** . 

- 1) Each PA _u_ and SA _s_ is assigned a unique DID, _DID_ ( _u_ ) and _DID_ ( _s_ ) respectively. 

- 2) These DIDs are registered and managed on the VAR, which is implemented as a smart contract on B. 

- 3) The cryptographic nature of DIDs and the immutability of the VDR on B ensure that each agent’s identity is verifiably linked to its DID, and that DIDs cannot be forged or impersonated without compromising the underlying cryptographic keys. 

We prove the non-repudiation via request non-repudiation and response non-repudiation, respectively. For **Request NonRepudiation** , 

- 1) The request transaction _X_ ( _P_ ( _▷_ )) explicitly includes _DID_ ( _u_ ) (the sender’s identifier) and _DID_ ( _s_ ) (the intended recipient’s identifier), along with the cryptographic hash of the request payload _H_ ( _P_ ( _▷_ )). 

- 2) _X_ ( _P_ ( _▷_ )) is broadcasted to and recorded on B. Given that transactions on a blockchain are cryptographically signed by the originating entity, _u_ cannot subsequently deny having initiated the request. The on-chain record serves as irrefutable proof. 

## For **Response Non-Repudiation** , 

- 1) The response transaction _X_ ( _◁_ ) includes _H_ ( _X_ ( _P_ ( _▷_ ))) (linking it to the specific request) and _H_ (¯ _◁_ ) (the hash of the encrypted response). 

- 2) _X_ ( _◁_ ) is broadcasted by _s_ to B and recorded immutably. 

- 3) By committing _X_ ( _◁_ ) to B, _s_ publicly attests to having processed the request and generated the corresponding (encrypted) response. _s_ cannot later deny having sent this response, as its action is verifiably recorded on B. 

Thus, the combination of cryptographically verifiable DIDs and immutable on-chain transaction records provides strong 

guarantees for authenticity and non-repudiation for both request and response commitments. 

**Property IV.3** (Response Confidentiality) **.** _The raw response ◁ is only revealed to the authorized PA u after specific conditions η are verifiably met._ 

- _Proof._ We assume the correctness and robustness of cryptographic algorithms in our proof. 

- **Raw Response Encryption:** _s_ encrypts the raw response _◁_ using a randomly generated symmetric encryption key _κ_ , resulting in ¯ _◁_ = _Enc_ ( _◁, κ_ ). ¯ _◁_ is then stored off-chain in _D_ (¯ _◁_ ). Without knowledge of _κ_ , ¯ _◁_ is computationally infeasible to decrypt, preserving _◁_ confidentiality. 

- **Conditional Key Release:** _κ_ is not directly transmitted. Instead, _s_ encrypts _κ_ using _u_ ’s public key _PK_ ( _u_ ), yielding _κ_ ¯ = _Enc_ ( _κ, PK_ ( _u_ )). _κ_ ¯ is released by _s_ to _u_ only after the conditions _η_ (specified in _X_ ( _◁_ ) on B) are verifiably fulfilled on B. The on-chain verification mechanism for _η_ (e.g., smart contract logic for payment) ensures this conditional release. 

- **Asymmetric Decryption:** Only _u_ possesses the corresponding private key _SK_ ( _u_ ) to _PK_ ( _u_ ). Therefore, only _u_ can decrypt _κ_ ¯ to retrieve _κ_ ( _κ_ = _Dec_ (¯ _κ, SK_ ( _u_ ))). Any unauthorized entity attempting to intercept _κ_ ¯ would be unable to decrypt it without _SK_ ( _u_ ). 

- **Raw Response Decryption:** Once PA _u_ obtains _κ_ , it can then decrypt the off-chain stored ¯ _◁_ to reveal the raw response _◁_ ( _◁_ = _Dec_ (¯ _◁, κ_ )). 

The multi-layered encryption scheme, combined with the onchain enforcement of conditions for key release, ensures that the confidentiality of the raw response _◁_ is maintained until the precise moment and to the precise entity ( _u_ ) for which access is authorized and conditions are fulfilled. 

**Property IV.4** (Verifiable Condition Fulfillment) **.** _The conditions η for releasing the decryption key κ are verifiably fulfilled on_ B _._ 

- _Proof._ We formulate the proof outline as follows. 

- **On-Chain Declaration:** The conditions _η_ are an integral part of the response commitment transaction _X_ ( _◁_ ) that SA _s_ broadcasts to B, meaning that _η_ is publicly declared and immutably recorded on the ledger before any key release occurs. This transparency allows any interested party to inspect the terms of engagement. 

- **Smart Contract Enforcement:** If _η_ involves a verifiable action (e.g., payment, proof of data access, computational proof), this action is executed and verified by a smart contract deployed on B. 

- **Irrefutable Proof of Fulfillment:** The successful execution of the action specified by _η_ results in a state change on B (e.g., a token balance update, an event emission by a smart contract). This on-chain state change serves as an irrefutable, timestamped, and publicly auditable proof that the conditions _η_ have been met. 

1443 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

- **On-Chain Verification:** SA _s_ is designed to release the encrypted symmetric key _κ_ ¯ to PA _u_ only after it has verified the on-chain fulfillment of _η_ . This verification process involves querying the state of B to confirm the necessary transactions or state changes have occurred, ensuring that SA _s_ is compensated or its conditions are met before revealing the sensitive data. 

Therefore, the DMAS guarantees verifiable condition fulfillment by leveraging the transparency and immutability of the distributed ledger, making all conditional agreements and their resolution publicly auditable and trustless. 

## V. PERFORMANCE ANALYSIS 

We evaluate the scalability and efficiency of the DMAS under various operational conditions. 

## _A. Experimental Setting_ 

- **Blockchain Network (** B **):** A local instance of an Ethereum testnet was deployed using Hardhat Network for controlled and repeatable experiments. The block time was configured to 2 seconds to accurately simulate a Layer-2 network environment, such as Base, known for its faster block finality compared to Ethereum’s mainnet. 

- **DMAS:** Both PAs and SAs were implemented as Python programs leveraging Autogen (v0.6.1) for robust multiagent conversation and orchestration capabilities. Notably, all agents (PAs and SAs) utilized GPT-4o as their underlying LLM for reasoning, natural language understanding, and response generation. The experimental setup involved deploying these agents within Docker containers for isolation and streamlined resource management. The number of PAs was varied from 1 to 32 concurrent instances to simulate increasing load. A diverse pool of 32 distinct SAs was pre-registered in the VAR. These SAs are categorized into 4 groups, with each group containing 7 unique terminal agents, each discoverable and routable by a specific routing agent. To initiate service discovery, each PA makes 8 distinct requests, initially engaging all 4 routing agents. 

- **Centralized MAS:** For the comparative experiment, we developed a Centralized MAS (CMAS) by adapting the DMAS implementation. We orchestrate SAs into two layers. The first layer comprises 4 routing agents, while the second layer consists of the other 28 agents. Unlike the DMAS where PAs interact with SAs via the blockchain, each PA is pre-configured to connect directly to all 4 routing agents in the first layer. The PAs then iteratively call their underlying LLMs (GPT-4o, as specified in the DMAS setup) to process requests and manage the flow of interaction through these two layers. 

- **Hardware Environment:** All experiments were executed on a single powerful physical machine, equipped with an Intel Core Ultra 9 285K processor, 64GB of RAM, and an RTX 5090 GPU. For the DMAS setting, this machine hosted the 32 Docker containers, with each container 

isolating a PA and an SA, allowing for a controlled assessment of resource utilization and performance under load. The CMAS is encapsulated in a single container for the comparative experiment. 

## _B. Experiment Design_ 

We first conducted a scalability experiment by gradually increasing the number of concurrent PAs from 1 to 32, allowing us to observe and analyze how the percentage distribution of onchain versus off-chain time costs shifted under varying loads. Our goal was to understand how the DMAS’s performance characteristics evolved as more agents became active. 

To isolate and quantify the performance overhead specifically attributable to the decentralized mechanisms of the DMAS, we further conducted a comparative experiment with the CMAS described in Section V-A. The CMAS bypasses all blockchain interactions, including DID resolution, verifiable commitments, and conditional key release, relying instead on pre-configured direct connections and a two-layered agent orchestration model. 

## _C. Scalability Analysis_ 

The total end-to-end latency for a full DMAS interaction cycle (from PA request initiation to decrypted response) can be broadly decomposed into on-chain and off-chain time cost. 

- **On-chain Time Cost** primarily includes the time spent waiting for blockchain transaction confirmations for the necessary verifiable steps, such as request commitment ( _X_ ( _P_ ( _▷_ ))), response commitment ( _X_ ( _◁_ )), and condition fulfillment (e.g., payment verification). These operations are inherently bound by the block time, consensus mechanism overhead, and network propagation delays. 

- **Off-chain Time Cost** includes all other delays. This includes the actual computation performed by SAs, the network latency for direct peer-to-peer payload transfers between PAs and SAs, the time taken for service discovery iterations (including any delegation hops), the latency for calling external LLM services (specifically OpenAI APIs for GPT-4o in our setup), and the subsequent data processing by agents. 

Our experimental results, as illustrated in Figure 2, demonstrate that on-chain time cost is a significant, but decreasing, percentage of the total time cost as off-chain operations scale with the number of concurrent requests. 

The observed trend highlights that the DMAS effectively pushes the performance bottleneck away from the blockchain for a wide range of workloads. The hybrid design allows the DMAS to leverage the high throughput capabilities of traditional off-chain distributed computing for complex agent interactions, while relying on the blockchain only for critical verifiable attestations. 

## _D. Efficiency Analysis_ 

The line chart in Figure 2 illustrates the end-to-end latency for the CMAS across varying numbers of concurrent requests. 

A critical finding emerges when we compare the total latency of the CMAS with the DMAS’s total latency after subtracting 

1444 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

- [5] W. Van der Hoek and M. Wooldridge, “Multi-agent systems,” _Foundations of Artificial Intelligence_ , vol. 3, pp. 887–928, 2008, iSBN: 1574-6526 Publisher: Elsevier. 

- [6] S. Han, Q. Zhang, Y. Yao, W. Jin, and Z. Xu, “LLM multi-agent systems: Challenges and open problems,” _arXiv preprint arXiv:2402.03578_ , 2024. 

- [7] T. Guo, X. Chen, Y. Wang, R. Chang, S. Pei, N. V. Chawla, O. Wiest, and X. Zhang, “Large Language Model based Multi-Agents: A Survey of Progress and Challenges.” in _33rd International Joint Conference on Artificial Intelligence (IJCAI 2024)_ . IJCAI; Cornell arxiv, 2024. 

- [8] W. Chen, Z. You, R. Li, Y. Guan, C. Qian, C. Zhao, C. Yang, R. Xie, Z. Liu, and M. Sun, “Internet of agents: Weaving a web of heterogeneous agents for collaborative intelligence,” _arXiv preprint arXiv:2407.07061_ , 2024. 

Fig. 2. Performance experiment results. 

its on-chain time cost. As depicted by the nearly identical curves compared to the DMAS off-chain time cost in Figure 2, our experiments lead to a conclusion: excluding the on-chain time cost, the DMAS shares almost the same efficiency compared to the CMAS. 

This direct comparison unequivocally demonstrates that the DMAS mimics the computational and communication efficiency of a centralized system for its off-chain operations. The architectural choice to offload heavy computation from the blockchain to a distributed off-chain environment is thus validated as highly effective. The observed performance difference in total latency between the DMAS and CMAS is, almost entirely, a direct consequence of the blockchain interactions required for ensuring the security properties. 

## VI. CONCLUSION 

We have presented the Decentralized Multi-Agent System (DMAS), a novel architectural paradigm designed to overcome the inherent limitations of centralized multi-agent systems, such as single points of failure, censorship, and trust deficits, by integrating a decentralized agent runtime and a trust-aware communication protocol. Our security analysis formally validates the system’s ability to ensure message integrity, agent authenticity, non-repudiation, and confidential data exchange. Furthermore, the performance analysis demonstrates the DMAS’s practical scalability and efficiency, underscoring its potential for fostering a trustworthy and scalable Internet of Agents. 

## REFERENCES 

- [1] W. X. Zhao, K. Zhou, J. Li, T. Tang, X. Wang, Y. Hou, Y. Min, B. Zhang, J. Zhang, and Z. Dong, “A survey of large language models,” _arXiv preprint arXiv:2303.18223_ , vol. 1, no. 2, 2023. 

- [2] S. Yao, J. Zhao, D. Yu, N. Du, I. Shafran, K. Narasimhan, and Y. Cao, “React: Synergizing reasoning and acting in language models,” in _International Conference on Learning Representations (ICLR)_ , 2023. 

- [3] H. Touvron, T. Lavril, G. Izacard, X. Martinet, M.-A. Lachaux, T. Lacroix, B. Rozi`ere, N. Goyal, E. Hambro, and F. Azhar, “Llama: Open and efficient foundation language models,” _arXiv preprint arXiv:2302.13971_ , 2023. 

- [4] Y. Chang, X. Wang, J. Wang, Y. Wu, L. Yang, K. Zhu, H. Chen, X. Yi, C. Wang, and Y. Wang, “A survey on evaluation of large language models,” _ACM transactions on intelligent systems and technology_ , vol. 15, no. 3, pp. 1–45, 2024, iSBN: 2157-6904 Publisher: ACM New York, NY. 

- [9] J. Yu, J. Zhou, Y. Ding, L. Zhang, Y. Guo, and H. Sato, “Textual Differential Privacy for Context-Aware Reasoning with Large Language Model,” in _2024 IEEE 48th Annual Computers, Software, and Applications Conference (COMPSAC)_ . IEEE, 2024, pp. 988–997. 

- [10] B. C. Das, M. H. Amini, and Y. Wu, “Security and privacy challenges of large language models: A survey,” _ACM Computing Surveys_ , vol. 57, no. 6, pp. 1–39, 2025, iSBN: 0360-0300 Publisher: ACM New York, NY. 

- [11] G. Wood, “Ethereum: A secure decentralised generalised transaction ledger,” _Ethereum project yellow paper_ , vol. 151, no. 2014, pp. 1–32, 2014. 

- [12] Y. Ding and H. Sato, “Bloccess: Enabling Fine-Grained Access Control Based on Blockchain,” _Journal of Network and Systems Management_ , vol. 31, no. 1, pp. 1–34, 2023. 

- [13] J. Garay, A. Kiayias, and N. Leonardos, “The bitcoin backbone protocol: Analysis and applications,” _Journal of the ACM_ , vol. 71, no. 4, pp. 1–49, 2024, iSBN: 0004-5411 Publisher: ACM New York, NY. 

- [14] Y. Ding, H. Sato, and M. G. Machizawa, “Leveraging Self-Sovereign Identity in Decentralized Data Aggregation,” in _2022 Ninth International Conference on Software Defined Systems (SDS)_ . Paris, France: IEEE, 2022, pp. 1–8. 

- [15] Y. Ding and H. Sato, “Self-Sovereign Identity as a Service: Architecture in Practice,” in _2022 IEEE 46th Annual Computers, Software, and Applications Conference (COMPSAC)_ . Los Alamitos, CA, USA: IEEE, 2022, pp. 1536–1543. 

- [16] Y. Ding, J. Yu, S. Li, H. Sato, and M. G. Machizawa, “Data Aggregation Management With Self-Sovereign Identity in Decentralized Networks,” _IEEE Transactions on Network and Service Management_ , vol. 21, no. 6, pp. 6174–6189, 2024. 

- [17] Q. Wu, G. Bansal, J. Zhang, Y. Wu, B. Li, E. Zhu, L. Jiang, X. Zhang, S. Zhang, and J. Liu, “Autogen: Enabling next-gen LLM applications via multi-agent conversations,” in _First Conference on Language Modeling_ , 2024. 

- [18] J. Huang, S. Gu, L. Hou, Y. Wu, X. Wang, H. Yu, and J. Han, “Large Language Models Can Self-Improve,” in _Proceedings of the 2023 Conference on Empirical Methods in Natural Language Processing_ , 2023, pp. 1051–1068. 

- [19] X. Bo, Z. Zhang, Q. Dai, X. Feng, L. Wang, R. Li, X. Chen, and J.R. Wen, “Reflective multi-agent collaboration based on large language models,” _Advances in Neural Information Processing Systems_ , vol. 37, pp. 138 595–138 631, 2024. 

- [20] D. Nam, A. Macvean, V. Hellendoorn, B. Vasilescu, and B. Myers, “Using an llm to help with code understanding,” in _Proceedings of the IEEE/ACM 46th International Conference on Software Engineering_ , 2024, pp. 1–13. 

- [21] L. Wu, Z. Zheng, Z. Qiu, H. Wang, H. Gu, T. Shen, C. Qin, C. Zhu, H. Zhu, and Q. Liu, “A survey on large language models for recommendation,” _World Wide Web_ , vol. 27, no. 5, p. 60, 2024, iSBN: 1386-145X Publisher: Springer. 

- [22] A. J. Thirunavukarasu, D. S. J. Ting, K. Elangovan, L. Gutierrez, T. F. Tan, and D. S. W. Ting, “Large language models in medicine,” _Nature medicine_ , vol. 29, no. 8, pp. 1930–1940, 2023, iSBN: 1078-8956 Publisher: Nature Publishing Group US New York. 

- [23] K. Singhal, S. Azizi, T. Tu, S. S. Mahdavi, J. Wei, H. W. Chung, N. Scales, A. Tanwani, H. Cole-Lewis, and S. Pfohl, “Large language models encode clinical knowledge,” _Nature_ , vol. 620, no. 7972, pp. 172–180, 2023, iSBN: 1476-4687 Publisher: Nature Publishing Group. 

- [24] S. Hong, M. Zhuge, J. Chen, X. Zheng, Y. Cheng, C. Zhang, J. Wang, Z. Wang, S. K. S. Yau, and Z. Lin, “METAGPT: META PROGRAMMING FOR A MULTI-AGENT COLLABORATIVE FRAMEWORK,” in _12th International Conference on Learning Representations, ICLR 2024_ , 2024. 

1445 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:51:22 UTC from IEEE Xplore.  Restrictions apply. 

