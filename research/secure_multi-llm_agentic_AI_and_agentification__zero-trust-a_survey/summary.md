---
id: secure_multi-llm_agentic_AI_and_agentification__zero-trust-a_survey
title: "Secure Multi-LLM Agentic AI and Agentification for Edge General Intelligence by Zero-Trust: A Survey"
authors:
  - Yinqiu Liu
  - Ruichen Zhang
  - Haoxiang Luo
  - Yijing Lin
  - Geng Sun
  - Dusit Niyato
  - Hongyang Du
  - Zehui Xiong
  - Yonggang Wen
  - Abbas Jamalipour
  - Dong In Kim
  - Ping Zhang
year: 2025
venue: "ACM Computing Surveys"
publisher: "ACM"
doi: "10.48550/arXiv.2508.19870"
url: "https://arxiv.org/abs/2508.19870"
type: article
keywords:
  - Multi-LLM
  - zero-trust
  - edge general intelligence
  - agentic AI
  - security
---

## Overview

This survey (Liu et al., 2025) is the first systematic treatment of applying the zero-trust security paradigm ("never trust, always verify") to multi-LLM agentic AI systems deployed for Edge General Intelligence (EGI). It frames "agentification" — turning massive edge devices into cognitive agents by integrating LLMs with perception, reasoning, and acting modules — as the key enabler of EGI, and argues that the resulting multi-LLM collaboration (e.g., perception/planning/control LLMs in Waymo vehicles, AWS Bedrock dynamic LLM routing) introduces security vulnerabilities that traditional perimeter-based defenses cannot handle: insecure inter-LLM communication, expanded attack surfaces, cascading single-point compromise, and cross-domain data leakage. The paper's core contributions are (1) a structured security-risk taxonomy splitting threats into intra-LLM and inter-LLM levels, (2) a critique of perimeter-based "trustworthy LLM" defenses and a motivated shift to zero-trust, (3) a unified zero-trust multi-LLM reference framework for EGI (instantiated on an autonomous-driving case and aligned with NIST SP 800-207), (4) a taxonomy of zero-trust mechanisms into MODEL-level (intra-LLM) and SYSTEM-level (inter-LLM) approaches, and (5) three future research directions. It positions blockchain, MPC, and ZKP as system-level enablers of decentralized, Byzantine-resilient zero-trust coordination.

## Background

The paper builds on and cites a broad body of prior work to support its argument. It draws on prior single-LLM security surveys (Das et al.; Aguilera-Martínez et al.; Friha et al. on LLM-based edge intelligence; Gan et al.'s threat taxonomy by origin/consequence; Liu et al.'s alignment-evaluation guidelines) and notes their common limitation of under-covering multi-LLM systems. It cites the few existing multi-LLM security surveys (Ko et al. on cross-domain threats like dynamic grouping and collusion; Kong et al. on user-LLM/LLM-LLM/LLM-environment communication security; Luo et al. on EGI architecture/trust/orchestration; Peigné et al. on the "multi-agent security tax" and infectious malicious prompts), arguing none adopt a zero-trust lens. For multi-LLM mechanics it references orchestration topologies, MRKL routing, MetaGPT's shared message pool, RAG memory, and emerging protocols (MCP, ACP, ANP). It grounds zero-trust in NIST SP 800-207 (the policy-engine standard it follows), plus Google BeyondCorp and CISA maturity models, with enablers like MFA, RBAC/ABAC, UEBA, SIEM, and micro-segmentation. It cites prior perimeter-based defenses it deems insufficient: adversarial training (CAT, ReFAT, AdversaFlow), differential privacy fine-tuning, RLHF/Safe-RLHF, reactive output filtering (Self-Defense), TEEs (Intel SGX/TDX, LoRA-TEE, CoreGuard), and firewalls (FirewaLLM, ControlNet). It also cites concrete demonstrations of risk it uses as evidence — GPT-4 autonomously bypassing a CAPTCHA via TaskRabbit, GPT-4 generating exploit code for 87% of one-day vulnerabilities, universal jailbreak prompts working across GPT-4/Bing/Bard/Claude, prompt-infection propagation (Lee et al.), and Cemri et al.'s 14 multi-LLM failure modes.

## Key Points

- The collaborative nature of multi-LLM systems creates security vulnerabilities (insecure inter-LLM communication, expanded attack surface, cascading single-point compromise, cross-domain data leakage) that perimeter-based security cannot adequately address, motivating a zero-trust paradigm for EGI.
- Security risks are taxonomized at two levels: INTRA-LLM issues (jailbreaks/prompt injection, unpredictable emergent abilities, data leakage/privacy, toxic/misaligned responses) and INTER-LLM issues (expanded attack surface, over-permissive integration, insecure communication, Byzantine consensus manipulation, cross-context data leakage from inter-agent invocation).
- Perimeter-based "trustworthy LLM" defenses are reactive, assume implicit trust within boundaries, struggle to define stable boundaries against rapidly expanding LLM capabilities, and enable lateral movement — making them fundamentally inadequate for dynamic multi-LLM EGI.
- The paper proposes a unified zero-trust multi-LLM reference framework (aligned with NIST SP 800-207) with seven components — mobile-edge LLMs (functionally segregated, least-privilege), cloud LLMs as policy engine, identity & authentication module (per-LLM key pairs, continuous short-lived credentials), inter-LLM communication gateway (encrypted, policy-controlled firewall/broker), user input checking, multi-layer output verification, and behavioral auditing/anomaly detection — and an end-to-end five-step operational workflow.
- Zero-trust mechanisms are categorized into MODEL-level (intra-LLM) approaches: strong LLM identity/authentication/authorization (adaptive/context-aware MFA, reputation-based authentication, ephemeral token-based authentication); context-aware access control beyond RBAC (AgentSafe, EPEAgents, ABE-based encryption, collaborative-memory bipartite-graph policies); and stateless/ephemeral LLM management (PagedAttention, vAttention, BlockLLM, self-destructing models, serverless deployment).
- Zero-trust mechanisms are categorized into SYSTEM-level (inter-LLM) approaches: proactive maintenance (intelligent input checking such as DefensiveTokens/JailGuard/SecurityLingua, reputation schemes, topology-aware methods like MedSentry/NetSafe/G-Safeguard/Guardian); blockchain and distributed management; micro-segmentation/isolation (network slicing — WiLLM, LLM-Slice); and intelligent monitoring/failure management (AgentFM, SentinelAgent, SagaLLM).
- Blockchain is presented as a particularly valuable zero-trust enabler because it eliminates trusted authorities, assumes any node may be malicious, and achieves Byzantine fault tolerance via distributed storage, consensus, and tamper-proof smart contracts — supporting immutable LLM reputation records (LLMChain, Mo et al.), decentralized role-based multi-LLM networks (LLM-Net with coordinators/respondents/validators; Luo et al. benchmarking four consensus algorithms), a Proof-of-Thought consensus that weights voting by reasoning contribution (BlockAgents), threshold-signature collective decision signing (Wang et al.), and smart-contract-governed edge inference to resist DDoS (Karanjai et al.).
- Complementary cryptographic primitives — Multi-Party Computation (MPC) for collaborative computation without revealing inputs, and Zero-Knowledge Proofs (ZKP, e.g., zkGPT) for verifying inference correctness without exposing model parameters/data — are positioned as core enablers of zero-trust collaborative reasoning.
- The paper provides an explicit comparative analysis (philosophy, deployment, overhead, applicable scenarios, target attacks) showing zero-trust requires whole-system engineering with multi-mechanism coordination rather than the independent, single-point defenses of perimeter security, and that partial zero-trust implementation can itself create serious vulnerabilities.

## Conclusion

The survey concludes that as multi-LLM systems become critical infrastructure for EGI, traditional perimeter-based security is inadequate for collaborative edge deployments, and that zero-trust offers a transformative, more appropriate paradigm. Rather than presenting empirical results, the paper substantiates its thesis through systematic risk analysis, a comparative framework (perimeter vs. zero-trust), and an extensive literature taxonomy demonstrating that many existing defenses already align with zero-trust principles even though no unified zero-trust multi-LLM framework previously existed — a gap the proposed reference architecture fills. The authors note key trade-offs and limitations: zero-trust shifts cost from training overhead to high operational overhead (continuous authentication, real-time monitoring, dynamic permission management), demands strong technical teams, may degrade performance, and risks creating vulnerabilities if only partially implemented. Three future research directions are identified: (1) ethical and societal issues — algorithmic accountability in distributed decision-making, fairness-preserving zero-trust protocols, and transparent/democratic governance, plus the tension that "never trust, always verify" may undermine user trust; (2) asymmetric information and network heterogeneity — delay-tolerant zero-trust protocols, adaptive information-sharing under varying connectivity, and partition-resilient distributed consensus, possibly via federated edge-cloud zero-trust hierarchies with graceful degradation; and (3) privacy-preserving collaborative reasoning — transformer-oriented/homomorphic encryption schemes and MPC-with-ZKP frameworks enabling encrypted multi-LLM collaboration with provable minimal information leakage. The authors hope the survey prompts both theoretical progress and practical implementation of secure edge agentic AI.
