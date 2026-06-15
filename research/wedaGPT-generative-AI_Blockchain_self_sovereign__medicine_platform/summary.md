---
id: wedaGPT-generative-AI_Blockchain_self_sovereign__medicine_platform
title: "WedaGPT — Generative-AI (with Custom-Trained Meta's Llama2 LLM), Blockchain, Self Sovereign Identity, NFT and Model Card Enabled Indigenous Medicine Platform"
authors:
  - Eranga Bandara
  - Peter Foytik
  - Sachin Shetty
  - Ravi Mukkamala
  - Abdul Rahman
  - Xueping Liang
  - Ng Wee Keong
  - Kasun De Zoysa
year: 2024
venue: "2024 IEEE Symposium on Computers and Communications (ISCC)"
publisher: "IEEE"
pages: "1-6"
url: "https://api.semanticscholar.org/CorpusID:273754067"
type: inproceedings
keywords:
  - GPT
  - LLM
  - Llama2
  - Blockchain
  - NFT
  - Model Card
---

## Overview

This paper presents "WedaGPT," a platform designed to preserve, share, and economically empower traditional/indigenous medicine knowledge by combining a custom fine-tuned Llama2-13B large language model with blockchain, self-sovereign identity (SSI), Non-Fungible Tokens (NFTs), and Model Cards. The core problem addressed is that indigenous medical knowledge (such as palm-leaf "puskola" manuscripts) is at risk of being lost, practitioners' expertise is undervalued and unrecognized, and there is no mechanism to verify practitioners' credentials or the authenticity of treatments, enabling fraud. WedaGPT's contribution is a six-layer architecture (Resource Provider, Blockchain/Smart Contract, LLM, Data Provenance, NFT, and Customer layers) that digitizes ancient medical texts, fine-tunes an LLM on this corpus using Qlora 4-bit quantization with LoRA adapters, exposes the model via a chat interface in an SSI-enabled mobile wallet, encodes recipes and treatment provenance as NFTs and Model Cards on a blockchain (built on the authors' own "Rahasak" blockchain), and distributes NFT marketplace revenue among contributors (doctors, translators, manufacturers, therapists). The platform was prototyped in collaboration with practitioners from "Dibulagala Ceylon" and evaluated for blockchain transaction scalability, search throughput, block creation time, and qualitative LLM response quality for treatments documented in palm manuscripts (e.g., gastritis).

## Background

The paper situates itself within a body of literature on the value and fragility of traditional/indigenous medicine: it notes that these systems play a crucial role in global healthcare and cultural/community identity (citing Kurien et al.), and cites work showing traditional regimens can effectively treat conditions such as cancers, coma, and bone injuries that lack effective remedies in Western medicine, as well as a Sri Lankan traditional medicine regimen for knee osteoarthritis (citing De Silva et al.). It also notes the risk of knowledge loss from oral/local-only transmission and undigitized manuscripts, as well as marginalization of practitioners and barriers to credential verification (citing Abdullahi and Redvers et al.). On the technical side, it builds directly on the authors' prior work on the "Rahasak" scalable blockchain architecture (used as the underlying ledger), a "Tikiri" lightweight IoT blockchain, the "Mystiko" big-data-on-blockchain system, and a prior SSI-empowered digital identity platform — all reused here for identity proofs and data provenance. The LLM component builds on Meta's Llama2 (Touvron et al.), the few-shot learning paradigm of GPT-style models (Brown et al.), and Qlora's 4-bit quantized LoRA fine-tuning approach (Dettmers et al.), served via Ollama, LlamaIndex, and LangChain. The Model Card concept is drawn from Wadhwani and Jain's work on ML model card transparency. The NFT-for-medical-data approach follows prior work on NFT-based medical device traceability (Gebreab et al.), federated learning with NFT marketplaces for health diagnosis (Sai et al.), blockchain-based drug traceability (Turki et al.), SSI-empowered patient tokenization (Zhuang et al.), and NFT-based electronic medical records (Mohammed and Abdul Wahab).

## Key Points

- WedaGPT proposes an integrated six-layer architecture (Resource Provider, Blockchain/Smart Contract, LLM, Data Provenance, NFT, Customer layers) that combines generative AI, blockchain, SSI, NFTs, and Model Cards specifically for the indigenous/traditional medicine domain — a combination the authors claim is novel relative to prior blockchain/NFT/SSI/AI medical platforms (per their comparison in Table I).
- The platform constructs an AI-powered knowledge base by translating ancient medical recipe books (palm-leaf "puskola" manuscripts) into English/PDF and using this corpus to fine-tune a Llama2-13B LLM via Qlora 4-bit quantization with Low-Rank Adapters (LoRA), served through Ollama with LlamaIndex/LangChain integration.
- The platform introduces a custom NFT schema ("w-528") to represent ancient medicinal recipes as tradeable digital assets, encapsulating recipe details, contributor identities, links to Model Cards, and LLM information, sold on NFT marketplaces for fiat currency.
- Revenue from NFT sales is algorithmically/contractually shared among all contributors (doctors, translators, recipe owners, therapists), which the paper presents as a mechanism for equitable recognition and economic incentivization of traditional knowledge holders.
- Medical recipe and treatment provenance (patient status, prescription dates, treatment history, physician details) is encapsulated in Model Card objects stored on-chain via a dedicated Model Card smart contract, enabling NFT purchasers to independently verify data provenance.
- The system uses SSI-enabled mobile wallets and an Identity smart contract so that participants' actual identities remain private in their wallets while verifiable SSI-Proofs are recorded on-chain, intended to reduce fraud and enable credential verification of doctors and treatments.
- Four core smart contracts (Identity, Recipe, Model Card, NFT) underpin the system, running on the authors' Rahasak blockchain with each organization operating its own node.
- The platform was implemented as a proof of concept in collaboration with "Dibulagala Ceylon" practitioners, fine-tuning Llama2-13B on translated palm-manuscript content covering treatments such as blood sugar regulation and gastritis.
- Blockchain evaluation demonstrated linear transaction throughput scalability with increasing peer count (for both invoke and query transactions), block generation time measured up to 7 peers, and a Lucene-index-based search capable of querying 2 million records in approximately 4 milliseconds.
- LLM evaluation was qualitative, demonstrated via an example chatbot response about gastritis treatment derived from the palm manuscripts.

## Conclusion

The paper concludes that WedaGPT successfully demonstrates a working integration of a custom-trained Llama2 LLM, blockchain, SSI, NFTs, and Model Cards to digitize, preserve, and monetize traditional medical knowledge while improving transparency and reducing fraud risk through verifiable identity proofs and on-chain provenance records. The blockchain evaluation supports claims of scalability (linear throughput growth with peer count) and fast search performance over large transaction volumes, while the LLM evaluation is presented only as a single illustrative example response rather than a rigorous quantitative benchmark (e.g., no accuracy, hallucination-rate, or medical-correctness evaluation against expert ground truth is reported). The paper does not address regulatory, legal, or medical-safety validation of AI-generated treatment advice, nor does it discuss how NFT-based monetization interacts with intellectual property or benefit-sharing norms for indigenous communities. As future work, the authors state they intend to integrate additional open-source LLMs to expand the platform's capabilities, leaving open questions about model selection criteria, multilingual support beyond English translation, and broader clinical validation of the LLM's medical recommendations.
