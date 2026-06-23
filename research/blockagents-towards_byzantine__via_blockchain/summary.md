---
id: blockagents-towards_byzantine__via_blockchain
title: "BlockAgents: Towards Byzantine-Robust LLM-Based Multi-Agent Coordination via Blockchain"
authors:
  - Bei Chen
  - Gaolei Li
  - Xi Lin
  - Zheng Wang
  - Jianhua Li
year: 2024
venue: "Proceedings of the ACM Turing Award Celebration Conference - China 2024 (ACM-TURC '24)"
publisher: "ACM"
pages: "187-192"
doi: "10.1145/3674399.3674445"
url: "https://research.ebsco.com/linkprocessor/plink?id=69dc2faf-8c0b-37f2-9243-5309ddeb9b39"
type: inproceedings
keywords:
  - Blockchain
  - Large Language Model (LLM)
  - Multi-Agent System (MAS)
---

## Overview

This paper (Chen et al., ACM-TURC '24) introduces BlockAgents, a framework that integrates blockchain into LLM-based cooperative multi-agent systems to defend against Byzantine attacks, addressing the observation that most existing LLM multi-agent approaches are vulnerable to malicious agents that can inherit LLM biases (deceptive behavior) or collude as their numbers grow. The framework decomposes multi-agent collaboration into a unified four-stage workflow (role assignment, proposal statement, evaluation, and decision-making) and stores discussion records and final solutions immutably on a blockchain to provide auditability. Its central technical contributions are a proof-of-thought (PoT) consensus mechanism that combines a contribution-based reward/stake system, stake-based miner designation, and multi-round debate-style voting so that the agent contributing most to the group reasoning earns accounting rights, plus a multi-metric prompt-based evaluation method that scores proposals along factual consistency, redundancy, and contextual causal relevance to distinguish valid from malicious answers. Evaluated on GSM8K, MATH, and MMLU against MAD and Sampling-and-Voting baselines, BlockAgents reduces poisoning-induced accuracy drops to under 3% and cuts backdoor attack success rates to under 5%. The authors state this is the first work to defend multi-agent systems against colluding entities from the defender's perspective and the first to apply blockchain to securing multi-agent collaboration.

## Background

The paper builds on a body of prior work it cites to motivate and support its argument. It situates itself within LLM-based multi-agent coordination research, noting that beyond single-agent modes, prior approaches improve reasoning via role-playing (CAMEL) and multi-agent debate (citing Liang et al.'s MAD, Du et al., and others), and that several works introduce an "evaluator" role (e.g., Reconcile, ChatEval) — but the paper argues none of these consider the trustworthiness of the evaluation process. It cites prior security research showing LLM-based agents are vulnerable to poisoning and backdoor attacks at the "cognitive level" (e.g., Sleeper Agent, BadChain, "Watch Out for Your Agents!," PoisonPrompt, instruction-tuning poisoning), claiming agents are less robust and can generate stealthier harmful content than the underlying LLMs, and that collusion among multiple malicious agents (citing "Evil Geniuses") makes poisoning more covert. For its defense design, the paper draws on blockchain-based Byzantine-robust distributed systems, especially federated learning: it cites traditional Proof-of-Stake (PoS) consensus as able to isolate Byzantine nodes via stake-weighted voting and disincentivized stake risk, along with model-validation, committee-based consensus, and incentive-mechanism strategies (e.g., BRFL, FLoBC, blockchain defenses against FL poisoning). It also notes prior NLG evaluation methods (G-Eval, ChatEval) focus on improving generation accuracy rather than mitigating malicious content. The baselines it compares against are MAD (multi-agent debate) and Sampling-and-Voting ("More Agents Is All You Need").

## Key Points

- BlockAgents is presented as the first framework to enhance the security of LLM-based multi-agent collaboration using blockchain, and the first to study defending against colluding malicious entities in multi-agent systems from the defender's side.
- It achieves Byzantine-robust and auditable multi-agent coordination by integrating blockchain into a unified four-stage workflow: role assignment (agents become workers W or miners M, with |W|+|M| <= N), proposal statement (signed answers sent to a random miner), evaluation (miners score answers, allocate rewards, and form blocks), and decision-making (the highest-scoring answer in the winning block is adopted).
- The proposed proof-of-thought (PoT) consensus mechanism rewards agents by contribution: workers are rewarded by their answer score (0-10) and miners by the difficulty of winning the debate (scaled by debate round t), with accumulated rewards forming an agent's "stake."
- Stake-based miner designation assigns the |M| highest-stake agents as miners/evaluators, which the paper claims prevents malicious agents from participating in evaluation.
- A multi-round debate-style voting mechanism requires a leader miner's block to receive more than |M|/2 votes to gain accounting rights; otherwise leaders are re-elected up to a maximum of R rounds, which both resists Byzantine manipulation and improves evaluation quality as honest miners refine scores by referencing others' transactions.
- The multi-metric prompt-based evaluation scores each proposal on three dimensions tuned to detect Byzantine attacks — factual inconsistency, redundancy (used to hide attacker-inserted fragments), and contextual causal relevance — focusing on mitigating malicious content rather than just improving accuracy.
- The threat model assumes an adversary can control at most N/3 agents to output arbitrary content, with cross-role collusion (e.g., evaluators giving high scores to wrong answers), under two attack types: poisoning attacks (make final output wrong) and backdoor attacks (insert illegal content given a trigger phrase such as "2024").
- Against poisoning attacks (N=10, 2 malicious agents, R=2, GPT-3.5-Turbo), BlockAgents limits accuracy drop (Delta) to 1.0% on GSM8K (78.4->77.4), 2.6% on MATH (34.6->32.0), and 0.6% on MMLU (64.3->63.7), versus much larger drops for MAD (16.2/6.4/18.2) and Sampling-and-Voting (6.7/5.7/4.9).
- Against backdoor attacks (4/10 malicious agents), BlockAgents achieves attack success rates (ASR) of just 0.6% (GSM8K), 1.8% (MATH), and 3.7% (MMLU), dramatically lower than MAD (46.6/36.9/49.8) and Sampling-and-Voting (71.2/82.5/92.3); the authors conclude voting-only mechanisms carry the greatest security risk.
- Ablation shows the multi-dimensional evaluation prompt improves accuracy under poisoning (e.g., 77.4 vs 75.8 on GSM8K, 32.0 vs 31.4 on MATH, 63.7 vs 60.1 on MMLU), confirming redundancy and contextual relevance dimensions aid robust evaluation.

## Conclusion

The paper concludes that BlockAgents delivers scalable, auditable, and Byzantine-robust multi-agent coordination, with experiments supporting its core claims: poisoning interference on accuracy is held under 3% and backdoor attack success rate under 5% across three datasets, and BlockAgents consistently shows the smallest accuracy degradation among compared frameworks (sometimes the others dropped below even single-agent accuracy). Ablation studies support the design choices: accuracy stays most stable as the number of malicious agents increases; larger maximum debate rounds R raise accuracy because more communication eases consensus, but gains plateau beyond R=3 (identified as the appropriate choice); and accuracy drops both when miners are too few (a single malicious miner has outsized influence) and too many (too few workers means insufficient alternative proposals), leading the authors to recommend designating roughly half the agents as miners. The authors note practical trade-offs (choosing appropriate R and miner counts) as considerations rather than formally listed limitations, and they frame the work as an inspiration for future research on multi-agent collaboration security. Open angles implied include extending beyond the N/3 adversary bound, testing on backbones other than GPT-3.5-Turbo and on non-math/reasoning tasks, handling more sophisticated or stealthier collusion, and addressing the computational/time overhead introduced by multi-round debate and blockchain consensus.
