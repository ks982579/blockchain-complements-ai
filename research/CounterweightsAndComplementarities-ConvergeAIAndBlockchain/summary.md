---
id: CounterweightsAndComplementarities-ConvergeAIAndBlockchain
title: "Counterweights and Complementarities: The Convergence of AI and Blockchain Powering a Decentralized Future"
authors:
  - Yibai Li
  - Zhiye Jin
  - Xiaobing (Emily) Li
  - K. D. Joshi
  - Xuefei (Nancy) Deng
year: 2025
venue: "SIGMIS Database: The DATABASE for Advances in Information Systems"
publisher: "Association for Computing Machinery"
volume: 56
issue: 2
pages: "6-12"
doi: "10.1145/3733612.3733614"
url: "https://doi.org/10.1145/3733612.3733614"
type: article
keywords:
  - ai
  - blockchain
  - decentralized intelligence
  - fintech
  - large language model
  - transparency
---

## Overview

This editorial, published in The DATA BASE for Advances in Information Systems (Vol. 56, No. 2, May 2025), examines the relationship between artificial intelligence (AI) and blockchain technology, framing them as opposing yet complementary forces in the technology landscape. The paper's central problem is that AI—especially since the rise of large language models (LLMs)—exerts a strong centralizing pull, concentrating data, computational resources, and decision-making power in a handful of large corporations, while blockchain operates on a fundamentally decentralizing logic. Rather than treating these as separate domains, the authors argue AI and blockchain can serve as "counterweights" to each other's weaknesses: blockchain can decentralize AI's data, computation, and governance, while AI can improve blockchain's efficiency and security (e.g., smart contract management, content moderation, fraud detection). The paper's primary contribution is to define and call for a new interdisciplinary research agenda called "decentralized intelligence" (DI)—intelligent systems designed to operate without centralized control—and to propose six concrete research initiatives plus a table of specific research questions to guide future scholarship. As an editorial, it is not an empirical study but a position/agenda-setting piece intended to orient the Information Systems (IS) research community toward this convergence space.

## Background

The paper situates AI's centralization problem within a body of work on the risks of large language models and foundation models, citing Bender et al. (2021) on "stochastic parrots" and the dangers of scaling language models, and Bommasani et al. (2021) on the opportunities and risks of foundation models—both used to support the claim that AI development is becoming concentrated among a small set of powerful actors. It draws on Weidinger et al. (2021) for the argument that data monopolization introduces bias and limits competitive access to high-quality datasets, and cites OpenAI's (2023) GPT-4 technical report and Floridi & Chiriatti (2020) to support the claim that training costs for state-of-the-art models (reportedly over $100 million for GPT-4) create a "winner-takes-all" dynamic favoring well-resourced firms. On the blockchain side, the paper builds on Asif et al. (2024) and Bassey et al. (2024) for blockchain's decentralization, trust, and user-control properties, and on the NIST blockchain overview (Yaga et al., 2018) for the caveat that blockchain does not automatically guarantee decentralization—it depends on design and governance choices. The paper also references the historical lineage of decentralized computing it sees DI as continuing: distributed/parallel computing (1950s-1980s), multi-agent systems (1980s-1990s, citing Mueller 2025), peer-to-peer systems like Napster and BitTorrent and grid computing (1990s-2000s), Bitcoin's introduction of trustless decentralized systems (2009), and Google's Federated Learning (2017, citing Yang et al., 2019 and McMahan et al., 2017). For the AI-blockchain complementarity argument, it points to zero-knowledge machine learning (ZKML) as detailed by Peng et al. (2025) as a mechanism for verifying AI computations without revealing underlying data, AI-assisted smart contract generation and vulnerability detection (Barbàra et al., 2024; Rahman et al., 2024), and blockchain-based NFTs/provenance tracking for distinguishing human-created content from AI-generated media (Hasan et al., 2024; Wang et al., 2021).

## Key Points

- This paper frames AI and blockchain as having fundamentally opposed structural tendencies—AI toward centralization and blockchain toward decentralization—and characterizes this opposition using the framing "AI is communist, blockchain is libertarian" (attributed to Ali Yahya/a16z, 2023).
- This paper identifies three specific, interrelated problems caused by AI's centralizing tendency: data monopolization (large tech firms control the datasets needed for competitive LLMs), resource monopolization (the high cost of compute for training state-of-the-art models restricts development to a few wealthy corporations), and concentration of power and control (a small set of firms—OpenAI, Google, Meta, Microsoft—dominate AI development, deployment, and policy influence).
- This paper argues that AI functions as a "sustaining" innovation that reinforces incumbent tech companies' market positions, whereas blockchain functions as a "disruptive" innovation that challenges existing power structures and redistributes control.
- This paper claims AI's data-hungry nature pushes toward erosion of individual privacy (a "panopticon" dynamic), while blockchain's emphasis on user sovereignty over data pushes in the opposite direction, toward greater privacy and user control.
- This paper argues that AI's generative capabilities create an abundance of media that makes it difficult to distinguish human-created from AI-generated content, while blockchain mechanisms such as NFTs and provenance tracking can help establish authenticity and preserve the value of human-created content.
- This paper proposes that blockchain can mitigate AI's centralization risks through decentralized data management, distributed model training/inference, immutable audit trails for transparency and accountability, zero-knowledge machine learning (ZKML) for privacy-preserving verification, and proof-of-humanity mechanisms to combat AI-generated misinformation.
- This paper proposes that AI can, in turn, enhance blockchain systems through automated smart contract code generation and vulnerability detection, content curation/spam filtering on decentralized platforms, transaction monitoring and defense against miner extractable value (MEV) attacks, and detection of deepfakes/misinformation.
- This paper introduces and defines "decentralized intelligence" (DI) as a new interdisciplinary research area focused on methodologies, algorithms, and architectures enabling intelligent systems to function without centralized control, and traces its conceptual lineage from 1950s distributed computing through multi-agent systems, peer-to-peer networks, blockchain, and federated learning.
- This paper proposes six concrete research/policy initiatives to build a DI ecosystem: government-funded open-AI systems, research consortia, regulatory frameworks tailored to decentralized AI, decentralized data cooperatives, standardization bodies for interoperability, and open-source collaborative development platforms (a "decentralized GitHub for AI").
- This paper provides a structured table of five specific research questions for blockchain-powered decentralized AI, covering blockchain-enabled AI marketplaces, mitigation of data monopolization via data cooperatives/DAOs, lowering economic/competitive barriers via tokenized compute access, mitigating privacy erosion via decentralized identity (DID) and zero-knowledge proofs, and using NFTs to establish provenance for human-created content.

## Conclusion

The editorial concludes that AI centralization is fundamentally a socio-technical problem rather than a purely technical one, and explicitly cautions (citing NIST's Yaga et al., 2018) that merely using blockchain does not guarantee decentralization—outcomes depend heavily on system design, governance, and implementation choices. As a position paper rather than an empirical study, it does not test hypotheses with data; instead, its "findings" are a set of conceptual arguments and a research agenda. The authors present the convergence of AI and blockchain as both an opportunity (mutual mitigation of each technology's weaknesses) and an open challenge requiring sustained interdisciplinary effort spanning technology, ethics, and governance. The paper explicitly identifies open research questions (summarized in its Table 1) around blockchain-enabled AI marketplaces, data cooperatives/DAOs, tokenized access to compute resources, decentralized identity and zero-knowledge proofs for privacy, and NFT-based provenance for human-created content—all flagged as directions for future IS scholarship. The authors also point to a specific institutional venue for this research community to coalesce: the mini-track on Responsible Approaches to Blockchain, Cryptocurrency, and Fintech at the Hawaii International Conference on System Sciences (2024-2026), signaling that the paper's main "output" is an agenda-setting call to action for the IS research community rather than a resolved set of findings.
