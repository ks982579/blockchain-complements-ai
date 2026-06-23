# Factcheck report

Result: **CORRECTED**

- Overview: CORRECTED (1 issue)
- Background: PASS
- Key Points: PASS
- Conclusion: PASS

Verified correct: intra-LLM vs inter-LLM risk taxonomy; NIST SP 800-207 alignment; MODEL-level placements (AgentSafe, EPEAgents, PagedAttention, BlockLLM) and SYSTEM-level placements (DefensiveTokens, JailGuard, MedSentry, NetSafe, G-Safeguard, WiLLM, LLM-Slice, AgentFM, SentinelAgent, SagaLLM) — none mislevelled; blockchain specifics (LLMChain, LLM-Net coordinators/respondents/validators, BlockAgents Proof-of-Thought, Karanjai DDoS); MPC and ZKP/zkGPT; three future directions; risk demonstrations (GPT-4 CAPTCHA via TaskRabbit, 87% one-day vulnerabilities, Cemri et al. 14 failure modes). Note: paper body carries an ACM Computing Surveys citation (Vol 9, Article 35, Sept 2025), used as the frontmatter venue.

## Issue 1

1. Overview — inline arXiv identifier not present in paper body.
   - Problematic text: "This survey (Liu et al., arXiv 2508.19870, 2025) is the first systematic treatment..."
   - Source: paper.md body contains no arXiv id; only "ACM Comput. Surv., Vol. 9, No. 9, Article 35. Publication date: September 2025." appears. The string "2508.19870" appears nowhere in paper.md.
   - Replacement: "This survey (Liu et al., 2025) is the first systematic treatment..."
