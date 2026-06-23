---
id: ai-based_crypto_tokens-the_illusion_of_decentralized_ai
title: "AI-Based Crypto Tokens: The Illusion of Decentralized AI?"
authors:
  - Rischan Mafrur
year: 2025
venue: "IET Blockchain"
publisher: "Wiley"
volume: 5
issue: 1
pages: "e70015"
doi: "10.48550/arXiv.2505.07828"
url: "https://research.ebsco.com/linkprocessor/plink?id=8f4baca5-bb3d-3045-9546-7fdfac714c37"
type: article
keywords:
  - Distributed, Parallel, and Cluster Computing
  - Artificial Intelligence
  - Cryptography and Security
  - Databases
---

## Overview

This paper by Rischan Mafrur (Macquarie University) is a critical review of "AI-based crypto tokens" — cryptographic assets that purport to power decentralized AI platforms for computation, data sharing, and model deployment. The work examines leading projects (RENDER, Bittensor/TAO, Fetch.ai/FET, SingularityNET/AGIX, Ocean Protocol/OCEAN, Numerai/NMR, Story/IP, Cortex/CTXC, DeepBrain Chain/DBC, Alethea AI/ALI) along four axes: blockchain infrastructure, token utility, consensus mechanism, and business model. Its central question is whether these tokens deliver genuine decentralization and technological utility or merely an "illusion of decentralized AI" that leverages AI narratives for speculative financial gain. The paper's core finding is that most platforms depend heavily on off-chain computation, have weak or no on-chain verifiability, face severe scalability limits, and largely replicate centralized AI service structures while adding token-based payment and governance layers without delivering novel value. It then surveys emerging directions (zkML/zk proofs, TEEs, AI oracles, modular blockchains, blockchain-enabled federated learning, improved tokenomics) and offers a practical decision framework for whether an AI token should be launched at all.

## Background

The paper builds on and cites a substantial body of prior work to frame its argument. It draws on Keršič and Turkanović's systematic review of decentralized AI (DEAI) building blocks, noting (citing them) that DEAI envisions AI models, agents, and datasets registered, discovered, and executed via blockchain registries, marketplaces, and token incentives. It situates itself relative to prior surveys of blockchain–AI convergence in specific sectors (e-Health, food safety, supply chains, finance, metaverse) and broader cross-industry reviews. On the DeFi side it cites work showing AI is already used within blockchain applications — e.g., Raun et al. on machine-learning bidding for MEV auctions in Flashbots private pools (>50% prediction accuracy), and Arkham Intelligence's "Ultra"/AI Entity Predictions engine for wallet attribution. To support its skepticism, the paper cites empirical studies of speculative token economies: Silberholz and Wu's analysis of 891 Ethereum tokens showing a ~90% decline in average utility usage since 2017 amid rising speculation; theory of speculation in community assets (Mei and Sockin); the ICP token's >95% collapse; Filecoin's low storage utilization (3.8% filled vs. 40–70% for centralized providers); and Helium's minimal IoT revenue. It also cites Ante/Demir and Saggu/Ante documenting abnormal returns (>41% within two weeks) for AI-themed crypto after ChatGPT's late-2022 launch. For decentralization-illusion claims it draws on Web3 governance critiques (Calzada; Barbereau et al. on timocratic DeFi voting). Legal/accountability context cites the Ooki DAO ruling and the DAO-as-general-partnership scholarship, plus the EU AI Act. Future-direction discussion cites zkML, zkPoT, TEEs, Chainlink AI oracles, Proof-of-Useful-Work and Coin.AI, modular chains (Cosmos, Celestia), and federated-learning systems (FLCoin, Biscotti, FabricFL, DeepChain), as well as Data Shapley Value and proof-of-quality incentive proposals.

## Key Points

- AI-based tokens are presented as a distinct category whose stated goal is to decentralize both the data and the computational infrastructure underlying AI services — not merely to apply AI within existing blockchain apps.
- The paper provides a structured comparative taxonomy of major AI tokens by blockchain base, AI role, computation location, token utility, consensus mechanism, and business model (Tables 1–2), and a classification into AI service marketplaces, compute networks, data marketplaces, agent/automation, AI-enhanced Web3 apps, and on-chain execution.
- Nearly all surveyed platforms rely on off-chain computation for AI inference/training, using the blockchain only as a coordination, payment, and metadata layer; genuine on-chain AI inference (e.g., Cortex's CVM) remains experimental and confined to non-mainstream chains.
- The stateless, reset-per-block nature of most smart contracts prevents on-chain learning or stateful AI behavior, making "learning contracts" practically elusive.
- Decentralized AI networks cannot match centralized providers on throughput, latency, and compute scale; blockchains support far fewer transactions per second than commercial AI services require, and large-model training exceeds decentralized coordination capacity.
- Quality control and trust are unsolved: there is a "verification dilemma" where AI output correctness often cannot be confirmed without re-execution, and incentive schemes (Numerai staking, Bittensor peer evaluation) introduce risks of collusion, sabotage, and gaming.
- AI-token platforms suffer a two-sided market bootstrapping problem and compete poorly against free/open alternatives (Hugging Face, Kaggle) and well-resourced centralized incumbents (OpenAI, AWS, Google, Azure), often at higher cost due to cryptoeconomic overhead and worse usability.
- Decentralized governance creates slow, friction-laden protocol evolution (analogous to ETH/ETC and BTC/BCH forks) and difficulty calibrating tokenomics, compounded by regulatory uncertainty around token classification, GDPR, and content liability.
- Decentralized AI creates an accountability/legal "liability vacuum"; token governance does not confer legal personhood, and participants may face partnership-style liability (illustrated by the Ooki DAO ruling) while struggling to meet EU AI Act obligations.
- The 2024 merger of Fetch.ai, SingularityNET, Ocean Protocol, and CUDOS into the Artificial Superintelligence Alliance (ASI), consolidating FET/AGIX/OCEAN into a unified ASI token, is documented as a signal of consolidation in the space.
- The paper argues token underperformance is a systemic Web3 issue (Helium, Filecoin, ICP) of weak product–market fit and network effects, not something intrinsic to AI.
- It identifies frontiers where decentralization could offer differentiated (not merely equivalent) value: privacy-preserving training on sensitive data (Ocean's compute-to-data) and collectively owned/governed foundational models (Bittensor).
- It surveys concrete future-enabling technologies: zkML/zkPoT verifiable computation, TEEs, quorum-based AI oracles, AI-dedicated Layer-1s and zk-rollup Layer-2s, modular blockchain architectures, blockchain-coordinated federated learning, data DAOs/cooperatives, and enhanced tokenomics (Data Shapley Value, proof-of-quality, fee burning/redistribution).
- It offers a six-point practical framework for launching a decentralized AI token: validate the need for decentralization, establish product–market fit, define clear token utility, align incentives/network effects, assess technical readiness, and consider regulatory/governance implications.

## Conclusion

The paper concludes that, despite real technical creativity, current AI-token ecosystems largely fall short of the promise of decentralized, trustless AI — and in some cases function chiefly as speculative financial instruments rather than engines of genuine innovation. The author's skeptical thesis (the "illusion of decentralized AI") is supported by the project-level evidence summarized in Table 4, which pairs each project's achievements with persistent shortcomings (off-chain execution, low real demand, scalability and verification gaps, narrow or speculative use cases, foundation-dominated governance). The paper frames the observed limitations as symptomatic of broader structural problems in Web3 token design — weak product–market fit, lack of genuine decentralization, and misaligned incentives — rather than AI-specific failings. It explicitly leaves open whether these ecosystems can overcome their foundational limitations or remain hype-driven illusions. Identified open research questions and future angles include: making zkML/verifiable off-chain computation practical for non-trivial models; designing collusion-resistant, game-resistant quality incentives; building AI-specialized and modular blockchain infrastructure; maturing blockchain-coordinated federated learning for sectors like healthcare and finance; resolving the legal/accountability "liability vacuum" against frameworks like the EU AI Act; and developing ethical, equitable governance (e.g., quadratic staking, capped rewards, "red-team DAOs") to prevent re-centralization. The author stresses that realizing the vision requires interdisciplinary advances across cryptography, distributed systems, tokenomics, and AI safety, plus proactive regulatory engagement.
