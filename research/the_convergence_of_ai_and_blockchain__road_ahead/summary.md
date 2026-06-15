---
id: the_convergence_of_ai_and_blockchain__road_ahead
title: "The Convergence of Artificial Intelligence and Blockchain: The State of Play and the Road Ahead"
authors:
  - Dhanasak Bhumichai
  - Christos Smiliotopoulos
  - Ryan Benton
  - Georgios Kambourakis
  - Dimitrios Damopoulos
year: 2024
venue: "Information"
publisher: "MDPI"
volume: 15
issue: 5
pages: "268"
doi: "10.3390/info15050268"
url: "https://www.mdpi.com/2078-2489/15/5/268"
type: article
keywords:
  - blockchain
  - artificial intelligence
  - smart contract
  - survey
---

## Overview

This paper is a systematic meta-survey examining the convergence of artificial intelligence (AI) and blockchain technology, two disruptive technologies whose combination remains in a nascent but rapidly growing stage of exploration. Using a methodology grounded in Systematic Literature Reviews (SLRs) combined with bibliometric analysis, the authors selected and analyzed sixteen recent survey/research articles (2018-2023) to extract fourteen recurring features of AI-blockchain integration, including data security, data privacy, data encryption, data sharing, decentralized intelligent systems, efficiency, automated decision systems, collective decision making, scalability, system security, transparency, sustainability, device cooperation, and mining hardware design. From this analysis, the paper constructs a three-era timeline (emerging, convergence, application) to organize how the two technologies have evolved together, and it categorizes convergence-era features into three groups (data manipulation, potential system improvements, and hardware issues), while application-era findings are organized around five focus areas (IoT, cybersecurity, energy, smart cities, and finance). The paper's main contribution is to consolidate fragmented prior literature into a structured taxonomy and roadmap, identify ten axes of open challenges (e.g., data operation, privacy, scalability, smart contract security, governance, interoperability), and propose this as a reference resource for researchers and practitioners entering the AI-blockchain space.

## Background

The paper situates blockchain's origins in Nakamoto's 2008 Bitcoin paper [1], but traces its conceptual roots to Chaum's 1983 "Blind signatures for untraceable payments" [3] and notes blockchain's broader evolution into an umbrella term for distributed ledger technologies (DLTs) used across finance, healthcare, and government [27-30]. It cites Castro and Liskov's Practical Byzantine Fault Tolerance (PBFT) [36] and subsequent consensus mechanism research [37,52,53] as foundational to blockchain's scalability and energy-efficiency debates, and references historical governance events like Ethereum's DAO fork [45] and Bitcoin's SegWit [46] as evidence that blockchains are not fully immutable in practice. For AI, the paper grounds its discussion in Turing's 1950 "Computing Machinery and Intelligence" [4] and standard AI definitions from Russell & Norvig [60], Winston [61], and Rich & Knight [62], while noting the recent rise of LLMs (GPT, Gemini/MMLU) and techniques like fine-tuning [63], prompt engineering [64], and reinforcement learning from human feedback [65]. Market-growth claims are drawn from external sources: the global blockchain market is projected to grow from USD 564.01M (2024) to USD 2475.35M (2030) at a 27.9% CAGR [13], the broader crypto ecosystem has a USD 2.66T market cap with over 2.4M cryptocurrencies [31], and the McKinsey Global Institute forecasts AI could add USD 13T to global economic output by 2030 [16]. The paper also leans heavily on eight prior survey works (e.g., Dinh & Thai's "AI and blockchain: A disruptive integration" [19], Salah et al.'s "Blockchain for AI: Review and open research challenges" [18], Karger's "Combining Blockchain and Artificial Intelligence" [79], and Bertino et al.'s "Data transparency with blockchain and AI ethics" [81]) as the basis for its feature taxonomy, and cites concrete prior systems such as Zyskind et al.'s blockchain-based personal data framework [96], Omohundro et al.'s work on AI/smart-contract safety [95], and various IoT/blockchain/AI frameworks (NeuRoNt [129], BlockDeepNet [131], DAI platforms [132,134]).

## Key Points

- The paper systematically reviewed sixteen recent (2018-2023) survey/research articles using SLR guidelines (Kitchenham et al. [77]) and bibliometric analysis (Ellegaard & Wallin [78]) to extract fourteen cross-cutting features of AI-blockchain convergence.
- It proposes a three-era timeline of AI-blockchain convergence: an emerging era (pre-2018, isolated applications), a convergence era (technical feature integration), and an application era (sector-specific deployment), tracing the earliest integration concepts to 2014-2015.
- For the convergence era, the paper groups the fourteen extracted features into three categories: data manipulation (data security, privacy, encryption, sharing), potential system improvements (decentralized intelligent systems, scalability, efficiency, automated/collective decision making, system security, sustainability, transparency), and hardware issues (device cooperation, mining hardware design).
- For the application era, the paper analyzes five focus areas in depth—IoT applications, cybersecurity, energy, smart cities, and finance—citing specific systems such as NeuRoNt (Ethereum-based multi-agent edge problem-solving) [129], BlockDeepNet (DL + blockchain for IoT security) [131], PPSC-BCAI (privacy-preserving smart contracts with AI) [141], and the Learning Markets model for decentralized AI collaboration [151].
- The paper compiles a table of real-world AI-blockchain cryptocurrency/token projects (e.g., AGI, RNDR, FET, FIL, LINK, TAO, ICP) categorized by market cap, blockchain type/layer, services, and consensus mechanism, illustrating the practical breadth of current AI-blockchain integration.
- The paper identifies ten distinct axes of open research challenges: data operation/quality, privacy vs. transparency trade-offs, system scalability, blockchain security (including consensus centralization risks), smart contract security (citing the 2016 DAO hack), decentralized oracle reliance, fog computing integration, system governance, cryptocurrency transaction fees, and the lack of standards/interoperability/regulation.
- The paper explicitly frames blockchain's "trustlessness" as a matter of degree rather than an absolute property, arguing that blockchain shifts—rather than eliminates—trust requirements (e.g., the 51% honest-majority assumption in PoW).
- The paper distinguishes "decentralization" (no central controlling authority) from "distribution" (data/processing spread across nodes) as conceptually separate properties often conflated in the literature.
- The paper argues that AI can optimize blockchain operations (e.g., resource estimation, intrusion detection, mining-hardware energy efficiency via ML-managed data centers, potentially reducing energy consumption by 40%+ per cited work [119]), while blockchain can provide AI with secure, verifiable, and shareable training data and decentralized governance/coordination infrastructure.

## Conclusion

The paper concludes that the convergence of blockchain and AI is reshaping multiple sectors (healthcare, finance, cybersecurity, energy, smart cities, IoT) by combining blockchain's security/transparency/decentralization with AI's predictive and automation capabilities, and that this convergence is still in an early, rapidly evolving phase. Its core findings are summarized as four takeaways: identification of major adoption trends across sectors, emergence of novel categorized features (data manipulation, potential system, hardware issues), broad cross-sector application potential (especially for smart contracts), and a set of unresolved challenges and future research directions. The paper does not claim to test hypotheses empirically—it is a literature synthesis—so its "findings" are descriptive/taxonomic rather than experimentally validated. It explicitly flags numerous open research questions as future work: developing privacy-preserving mechanisms that still support transparency, designing more scalable consensus mechanisms (citing Graphchain and Algorand as promising but underexplored [168,169]), improving smart contract security tooling (especially for languages like Solidity), reducing reliance on centralized oracles (e.g., via Chainlink-like solutions), establishing formal governance models for multi-party blockchain-AI systems, and creating industry standards/regulatory frameworks (citing ongoing IEEE, ITU, and NIST efforts [181,182]). The authors also call for empirical studies on AI-designed blockchains, AI-agent-based smart contracts, DAO-based governance models, and interdisciplinary collaboration that integrates legal frameworks with technical development, framing this as a roadmap rather than a closed research program.
