---
id: web3_x_ai_agents-landscape_integrations_and_foundational_challenges
title: "Web3 × AI Agents: Landscape, Integrations, and Foundational Challenges"
authors:
  - Yiming Shen
  - Jiashuo Zhang
  - Zhenzhe Shao
  - Wenxuan Luo
  - Yanlin Wang
  - Ting Chen
  - Zibin Zheng
  - Jiachi Chen
year: 2025
venue: "arXiv"
doi: "10.48550/arXiv.2508.02773"
url: "https://arxiv.org/abs/2508.02773"
type: misc
keywords:
  - Web3
  - Blockchain
  - Artificial Intelligence
  - LLM
  - Agent
---

## Overview

This paper (Shen et al., 2025) presents what it claims is the first and most comprehensive systematic analysis of the intersection between Web3 technologies and AI agents, examined across five critical dimensions: landscape, economics, governance, security, and trust mechanisms. Using a mixed-methods approach, the authors collect and filter 133 real-world Web3-AI agent projects (from CoinMarketCap, Product Hunt, and GitHub) and apply open card sorting to build a taxonomy of four primary categories and ten subcategories, then conduct quantitative market analysis on the 77 projects with available capitalization data (collectively ~$6.92 billion). Structured around five research questions, the work maps the market landscape (RQ1) and then investigates four bidirectional integration domains: AI agents participating in and optimizing DeFi (RQ2), enhancing Web3 governance (RQ3), strengthening Web3 security via intelligent vulnerability detection and automated smart-contract auditing (RQ4), and leveraging Web3's native trust infrastructure to build reliability frameworks for AI agent operations (RQ5). The central thesis is that Web3 and AI agents are mutually complementary—Web3 provides cryptographic, trustless, decentralized infrastructure for autonomous agents, while agents provide intelligent automation, accessibility, and sophisticated decision-making for Web3—and the paper synthesizes integration patterns alongside foundational challenges in scalability, security, and ethics, plus a research agenda. The authors release an open-source dataset of the 133 projects.

## Background

The paper builds on and cites a substantial body of prior work to frame its argument. It cites DefiLlama for the claim that Total Value Locked across Web3 protocols exceeds $100 billion, establishing the scale of the ecosystem, and cites prior SoK work (Zhou et al.) on DeFi attacks to motivate persistent security challenges. For the AI side, it draws on LLM surveys (Zhao et al.) and autonomous-agent surveys (Wang et al.), citing modern LLMs (GPT-4o, Claude 3.7 Sonnet, Gemini 2.5 Pro) as the cognitive foundation for agents, and Anthropic's Model Context Protocol (MCP) as the standardized interface connecting LLMs to blockchain tools. Methodologically, it adapts open card sorting and snowball sampling from established software-engineering empirical-research practices (Wohlin et al., Wan et al., Goodman). Its governance analysis is explicitly grounded in Ostrom's commons-governance theory. For security, it positions itself against traditional tooling it cites as limited: static analysis (Slither), symbolic execution (Mythril, Manticore), formal verification (Certora, KEVM), and ML/DL detectors (VulDeePecker, DR-GCN), then references LLM-driven security advances it draws on (GPTScan/MetaScan, iAudit, ChainGPT, Forta, GoPlus). The trust discussion references TEE/FHE technologies (Phala Network, Mind Network), Eliza/ai16z, NEAR's Doomslug consensus, OriginTrail's Decentralized Knowledge Graph, and IPFS. The paper distinguishes itself from prior surveys it cites—Bhumichai et al., Choi & Kim, and Kayikci & Khoshgoftaar on general AI-blockchain convergence; Karim et al. on multi-agent collaboration in blockchain; Ante's typology of 306 crypto AI agents in DeFi; and Chaffer et al. and Ballandies et al. on decentralized governance—arguing that, unlike these theoretical or narrower works, it provides a systematic market analysis of real-world deployments.

## Key Points

- The paper claims to deliver the first and most comprehensive taxonomy of Web3 AI agent projects, organizing 133 projects into four primary categories and ten subcategories: AI Agent Incubation (56 projects; Builder & Marketplace, Monetization & Launchpad), Infrastructure (34 projects; Agent Protocol & DePIN, Trust Layer, AI-powered Development), Financial Services (55 projects; DeFAI Agents, Investment Analytics), and Creative & Virtual (28 projects; Game & Metaverse, Content Creation, AI-powered RWA).
- The ecosystem exhibits a dual-concentration pattern: AI Agent Incubation has the most projects (56, 42.1%) indicating grassroots development activity, while Infrastructure dominates value—34 projects (25.6%) account for $4.69 billion, or 67.8% of analyzed market capitalization, with the highest average market cap per project (~$187.7M vs ~$88.4M for Incubation).
- The market shows a power-law / winner-takes-most distribution: the top 10% of projects with cap data (7 of 77) command $5.13 billion (74.2% of total analyzed cap), exemplified by NEAR Protocol (~$2.44B) and Fetch.ai (~$1.05B).
- Ethereum dominates the landscape: it hosts 45 projects (39.5% of network instances) capturing $6.05 billion (87.4%) of analyzed cap and the highest average per-project cap (~$134.4M), with Solana (28 projects, 24.6%), Base (22, 19.3%), and BSC (15, 13.2%) trailing, indicating an emerging but secondary multi-chain trend.
- Roughly 20.3% of projects (27 of 133) span multiple categories—accounting for $1.93 billion (27.9%) of cap—showing cross-category convergence, with common combinations being Financial+Incubation and Infrastructure+Incubation.
- (RQ2) AI agents transform DeFi through four roles: autonomous trading strategy execution (e.g., Griffain, Wayfinder), intelligent portfolio construction and optimization (e.g., One Click Crypto, Sharpe AI), AI-driven market analysis and intelligence (e.g., Aixbt, Assemble AI/NS3, DexCheck, Hey Anon), and improved accessibility via natural-language interfaces.
- (RQ3) AI agents enhance Web3 governance across Ostrom-derived phases: proposal analysis and community engagement, automated monitoring and enforcement of decisions, and adaptive governance/mechanism design (citing Fetch.ai, ChainGPT, Autonolas/Olas, ALI, MyShell, OriginTrail, Virtuals Protocol), addressing low participation, information asymmetry, and complex decision-making.
- (RQ4) Agent-based security overcomes traditional tooling limits: GPTScan/MetaScan achieve >90% precision on token contracts (finding nine previously unknown vulnerabilities, ~14.39s per 1,000 lines at ~$0.01/scan); iAudit's multi-agent fine-tuned framework reaches 91.21% F1 and 91.11% accuracy; ChainGPT/ChainAware.ai platforms deliver automated auditing and ~98% fraud-detection accuracy, addressing the claim that ~80% of Web3 bugs cannot be audited by existing tools.
- (RQ5) Web3 trust mechanisms enable reliable AI agent operations along three dimensions—cryptographic security and privacy-preserving computation (TEE, FHE; e.g., Mind Network, Phala+Eliza, Fetch.ai), decentralized consensus and verification (NEAR Doomslug, OriginTrail DKG, Trias), and transparent governance/accountability via immutable on-chain audit trails (e.g., Commune AI)—replacing centralized trust with cryptographic guarantees.
- The paper asserts bidirectional complementarity as its overarching contribution: agents leverage Web3's trustless infrastructure for autonomous, cryptographically verifiable operation, while Web3 gains intelligent automation, sophisticated decision-making, and improved accessibility.

## Conclusion

The paper concludes that its analysis of 133 active projects ($6.9B collective cap) supports its thesis of bidirectional complementarity: AI agents reshape decentralized ecosystems across landscape, finance, governance, security, and trust, while Web3 supplies the trust infrastructure agents lack. Its landscape findings (infrastructure value dominance, vibrant incubation activity, cross-category convergence, Ethereum dominance with emerging multi-chain deployment) are presented as evidence that the ecosystem remains in an early stage where foundational building blocks command the most investor confidence. The authors acknowledge significant open challenges in scalability, security, ethics, and technical integration. Specific technical limitations they flag include AI agent reliability issues (hallucination, limited context memory, high computational cost), security vulnerabilities (prompt injection and jailbreaking—especially dangerous when agents control irreversible financial transactions), and trust/adoption barriers (user reluctance to delegate crypto asset management to autonomous agents). They outline a forward research agenda: Web3-native agent memory and context persistence (e.g., IPFS-backed verifiable histories), portable AI agent digital assets and cross-chain asset management, Agent Decentralized Identity (Agent DID) to grant agents sovereign on-chain identities beyond user-delegated permissions, decentralized multi-agent coordination (A2A protocols, agent-only DAOs), and AI-agent integration with Real-World Asset (RWA) tokenization (noting the RWA market hit $23B in H1 2025 with 260% growth yet limited agent integration). The paper also notes governance-specific open questions around AI decision-making transparency, algorithmic bias, liability frameworks, and regulatory/cross-border coordination, framing these as priorities for building robust, intelligent, and trustworthy decentralized systems.
