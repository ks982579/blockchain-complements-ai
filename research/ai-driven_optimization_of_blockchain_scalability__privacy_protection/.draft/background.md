## Background

This paper situates its review within a broad body of prior work on blockchain technology, artificial intelligence (AI), and earlier attempts to combine the two. The following summarizes the external sources, claims, and technologies the paper draws on as foundation for its own classification and discussion.

### Origins and evolution of blockchain

The paper traces blockchain's history back to its introduction in 2008 (citing reference [1]), noting that Bitcoin's market capitalization surpassing USD 10 billion in 2016 is treated (via [2]) as a milestone that shifted public perception of blockchain from a cryptocurrency-only technology toward a general-purpose decentralized infrastructure. It draws on other sources ([3–5]) to support the idea of "Blockchain+" — blockchain's spread into finance, supply chain, healthcare, and public services. The paper also adopts a commonly cited (though informally standardized) three-generation framework for blockchain evolution — Blockchain 1.0 (cryptocurrencies, citing [8]), Blockchain 2.0 (smart contracts, citing [9]), and Blockchain 3.0 (IoT, industrial and privacy applications, citing [10–12]) — which a researcher could investigate further to understand how this periodization is used elsewhere in the literature.

It also cites domain-specific blockchain applications from other papers: finance [13], agriculture [14], food [15], and supply chain management [16], as background examples of blockchain's practical reach, particularly food-safety traceability via consortium blockchains.

### Foundational claims about blockchain architecture

Much of the technical background (Section 2.1) is attributed to other sources describing core blockchain components:
- Distributed ledger structure and immutability via encryption [33, 34].
- Consensus algorithms and smart contracts as core mechanisms [35].
- Block header/body structure, Merkle trees, and hash chaining [36–38].
- Public/private/consortium chain classifications and their tradeoffs [40–43].
- General consensus algorithm properties [44, 45].
- Energy-consumption figures for PoW, PoS, DPoS, PBFT, Kafka, Raft, and Paxos are drawn from external benchmarks, notably the **Cambridge Bitcoin Electricity Consumption Index (2024)** for PoW (~120 TWh/year, ~707 kWh per transaction) and the **IBM Blockchain Whitepaper (2022)** for PBFT energy use, plus claims about Ethereum's 2022 "Merge" reducing energy consumption by over 99.95% [46–48].
- Smart contract execution model and "if/when...then" logic, citing [49–51].
- Cryptographic foundations — symmetric vs. asymmetric encryption, RSA/ECC, and their role in blockchain security — drawn from [52–62], including claims about secure transmission infrastructure (Wi-Fi/5G) as the typical deployment context for symmetric encryption [60].

### Foundational claims about AI/ML/DL

Section 2.2 similarly builds on external descriptions of AI subfields: machine learning, deep learning, NLP, and computer vision [63–67], and traces AI's evolution from early pattern-recognition applications to "high intelligence" systems exemplified by GPT-4o [71] and DeepSeek [72]. Specific ML/DL models discussed (decision trees [80, 81], SVM [82, 83], random forests [84], XGBoost [85–87], CNNs, RNNs, LSTM, GRU, Transformers, GANs, VAEs, and MLPs [88–104]) are all introduced via citations to foundational or survey-style sources on these algorithms, which a researcher could consult for deeper technical grounding.

### Prior surveys on AI + blockchain integration

A central piece of background is the paper's positioning relative to **existing reviews** on AI-blockchain convergence. It cites several prior integration studies as motivating context (not as part of its own contribution):
- He et al. [17] on blockchain's suitability for edge-centric IoT and AI-edge integration.
- Islam et al. [18] on blockchain-AI drone-based pandemic surveillance.
- D'Souza et al. [19] on a blockchain+AI pharmaceutical supply chain system using the Rasa chatbot.
- Suhail et al. [20] survey on blockchain-based digital twins.
- Guergov et al. [21] on IoT-AI-blockchain convergence and hybrid models.
- Yang et al. [22] on blockchain, AI, and the metaverse.
- Kumar et al. [23] on integrated AI-blockchain platforms across business verticals (ten key application areas).
- Junaid et al. [24] on IoT/AI/blockchain in healthcare management.
- Xu et al. [25] on digital twins, AI, and blockchain in Industrial IoT.

The paper also explicitly compares itself (Table 3) against a set of prior domain-specific surveys, framing each as having gaps the current paper intends to address:
- Verma et al. [26] (2022) — blockchain + AI in plastic recycling, noted as lacking discussion of privacy/security and practical cases.
- Badidi et al. [27] (2022) — edge AI and blockchain for smart sustainable cities, noted as lacking blockchain performance optimization and privacy solutions.
- Haddad et al. [28] (2022) — blockchain + AI for electronic health records, noted as not addressing blockchain scalability.
- Kumar et al. [29] (2023) — AI-driven blockchain in public health, noted as lacking technical/deployment depth.
- Uddin et al. [30] (2024) — Metaverse, blockchain, and AI integration, noted as lacking industry application cases.
- Omidian et al. [31] (2024) — blockchain-AI synergy in healthcare, noted as lacking implementation detail.
- Zhou et al. [32] (2022) — AI, big data, and blockchain in food safety, noted as lacking privacy/optimization discussion.

This comparison table is useful background for understanding how the current survey's four-part classification (consensus, smart contracts, privacy protection, data retrieval) was framed as a response to gaps in this prior literature.

### Real-world application precedent

The paper cites the **Mobility Open Blockchain Initiative**, a collaboration involving Honda, General Motors, PGE and others, as an existing real-world example (reference [106]) of an AI-blockchain integration — specifically the first blockchain-based Electric Vehicle Grid Integration (EVGI) standard, used to motivate the practical relevance of AI-driven optimization for energy/transportation applications.

### Specific prior technical proposals cited as building blocks

For each of its four classification categories (especially consensus algorithms), the paper draws heavily on specific prior technical proposals, which represent the actual state-of-the-art systems a researcher would want to investigate directly:
- **PoAh / PoAh 2.0** (Dutta et al. [116]) — Proof-of-Authentication consensus for resource-constrained IoT devices, enhanced with AI-determined transaction metadata.
- AI-assisted consensus for vehicular networks using BP neural networks and the Informer model (Sun et al. [117]).
- **FGADL-DEVCA** (Alruwaili et al. [118]) — deep-learning-based data edge verification/consensus for IoT-cloud platforms.
- **BDEV-CAML** (Sasikumar et al. [119]) — blockchain-assisted data edge validation with ML for IoT fault detection.
- Off-chain smart contract consensus architecture for distributed computing (Zhang et al. [120]).
- Improved DPoS for IIoT big-data analytics (Sasikumar et al. [121]).
- **C-PoA** with PSLSTM + multi-head attention for healthcare threat detection (Kumar et al. [122]).
- LSTM-based anomaly detection for automotive/Internet-of-Vehicles security (Kim et al. [123]).
- **BCFL** — blockchain-enabled federated learning with differential privacy (DP-SGD) for IoV (Cui et al. [124]).
- **PF-PoFL** — decentralized AI task outsourcing with user-level differential privacy (Wang et al. [125]).
- **DPoEV/DABFT** dual consensus economic model for AIBC (Deng et al. [126]).
- Distributed Artificial Intelligence (DAI) over IoT with combined PoA+PoW consensus (Alrubei et al. [127]).
- **BCDDL** — blockchain consensus based on distributed deep learning proof-of-workload (Zhi et al. [128]).
- Blockchain-AI integration framework for carbon emissions trading (Jogunola et al. [129]).
- **TDCB-D3P** — Deep Reinforcement Learning (Double DQN with Priority Experience Replay) for trust-delegated consensus (Goh et al. [130]).
- Multi-layer ANN-based blockchain consensus and NLP-based degree certificate verification (Kim et al. [131]).
- DRL-based PoW task offloading to edge servers in Mobile Crowd Sensing systems (Chen et al. [132]).
- **XG-PBFT** — XGBoost-based node grouping/reputation for PBFT, reducing communication complexity from O(n²) to O(n) (Xiaowei et al. [133]).
- **CE-PBFT** — Credit Evaluation-based PBFT using decision trees for node behavior analysis (Xiao et al. [134]).

Additionally, the paper cites aggregate performance claims attributed to references [124, 127, 129] — approximately 25% reduction in consensus latency, 15% reduction in transaction processing costs, and 20% reduction in energy consumption compared to traditional blockchain systems — which a researcher may want to verify against the original sources.

### Smart contract vulnerability background

The discussion of smart contract security vulnerabilities (arithmetic overflow/underflow, re-entrancy, unchecked calls) references well-known historical incidents as background context, including the **DAO attack** (re-entrancy leading to a hard fork of Ethereum) and the **Parity Wallet vulnerability** (unchecked low-level call return values), both of which are widely documented elsewhere and serve as motivating real-world cases for AI-driven vulnerability detection research.
