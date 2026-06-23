# Pipeline Progress

This is the durable index. Read this first in any new session to see what's done and
what's next.

## Steps

- [x] 001-init: Plan written, memory structure created.
- [x] 002-pdf-conversion: `apps/pdf_to_md.py` written, run across all `research/` dirs (48/48 converted, see 002-pdf-conversion/NOTES.md).
- [~] 003-summarization: `summarize-paper` skill built; summaries written per-paper
      (see checklist below; resumable across sessions). 2026-06-23: 41 new paper
      directories added. 39 still need summaries; 2 are deferred (see "Papers deferred"
      below). Original 60 remain summarized.
- [x] 004-vector-db: `apps/surreal_core/` (Rust workspace, SurrealDB + fastembed) +
      `apps/surreal_cli.py` (Python wrapper, init/update/query/related — revised
      2026-06-12: ChromaDB -> SurrealDB -> Rust core w/ Python wrapper) built and run.
      Details + tests: `.claude-memory/004-vector-db/PROGRESS.md`.
- [x] 005-query-agent: `.claude/skills/research-query/SKILL.md` built and smoke-tested
      (query + citation synthesis, with Background-section flagging).

## Known issues / decisions

- `research/ConvergenceOfAIandBlockchain` duplicate directory: already removed, no
  action needed. Only `research/the_convergence_of_ai_and_blockchain__road_ahead`
  remains.
- `research/blockchain_for_ai-review_and_open_research/` has 2 PDFs: a 10.9MB
  `Blockchain_for_AI_Review_and_Open_Research_Challenges.pdf` and a 820KB
  `Final__Blockchain_for_AI__Review.pdf`. Different files (not duplicates) - the
  conversion script picks the larger one by default; smaller one noted as extra.
- `research/blockchain_for_deep_learning-review_open_challenges/` has 2 PDFs that are
  byte-identical (`BlockchainForDeepLearning-ReviewAndChallenges.pdf` ==
  `s10586-022-03582-7.pdf`) - conversion script picks one, no data loss either way.

## Papers deferred (not summarizing now) — 2026-06-23

These are intentionally NOT being summarized in the current pass:

- `research/autonomous_agents_on_blockchains-standards__trust_boundaries/` — paper is
  **121 pages**, over the 50-page limit (too many tokens to summarize now). paper.md
  exists; revisit if/when we decide to handle long papers.
- `research/a_multi-agent_framework_for__information_systems/` — source **could not be
  converted to markdown** (see that dir's `README.md`). No paper.md exists, so no
  summary possible until the conversion issue is resolved.

## Per-paper checklist (101 directories)

| # | Directory | paper.md | summary.md |
|---|-----------|----------|------------|
| 1 | accounting_and_auditing_with_blockchain_tech_and_AI__review | [x] | [x] |
| 2 | ai_agents_meet_blockchain__multi-agents | [x] | [x] |
| 3 | AI_and_BC_integration_in_business__content_analysis | [x] | [x] |
| 4 | ai-driven_optimization_of_blockchain_scalability__privacy_protection | [x] | [x] |
| 5 | ai_model_cards-state_of_the_art__automated_use | [x] | [x] |
| 6 | a_marketplace_for_trading_AI__IoT_data | [x] | [x] |
| 7 | an_explainable_federated_blockchain__healthcare_data | [x] | [x] |
| 8 | a_review_on_blockchain_technology__AI-driven_solutions | [x] | [x] |
| 9 | artificial_intelligence_and_blockchain-how_should_emerging_tech_be_gov | [x] | [x] |
| 10 | a_survey_on_blockchain-based_federated_learning | [x] | [x] |
| 11 | a_survey_on_blockchain-enabled_federated_learning_and__twin | [x] | [x] |
| 12 | bassa-ML-a_blockchain_and_model_card__provenance_platform | [x] | [x] |
| 13 | bias_in_data-driven_ai_systems | [x] | [x] |
| 14 | blockchain_and_AI_integration__transparency_in_finance | [x] | [x] |
| 15 | blockchain_and_explainable-AI__PCOS_detection | [x] | [x] |
| 16 | blockchain-as-a-platform-for-ai-transparency | [x] | [x] |
| 17 | blockchain-based_auditing_of_legal_decisions__AI_tools | [x] | [x] |
| 18 | blockchain-based_federated_learning-a_survey_and_new_perspectives | [x] | [x] |
| 19 | blockchain-based_federated_learning_systems__choices | [x] | [x] |
| 20 | blockchain-based_health_and_usage_monitoring__aerospace_structures | [x] | [x] |
| 21 | blockchain-enabled_federated_learning-a_survey | [x] | [x] |
| 22 | blockchain_federated_learning_for_IOT__comp_survey | [x] | [x] |
| 23 | blockchain_for_ai-review_and_open_research | [x] | [x] |
| 24 | blockchain_for_deep_learning-review_open_challenges | [x] | [x] |
| 25 | blockchain_for_explainable_and_trustworhty_AI | [x] | [x] |
| 26 | blockchain_for_federated_learning_toward_ml__systemic_survey | [x] | [x] |
| 27 | blockchain_NFT_federated_learning_and_model_cards__sliced_env | [x] | [x] |
| 28 | CounterweightsAndComplementarities-ConvergeAIAndBlockchain | [x] | [x] |
| 29 | decentralised_gov-driven_architecture__responsible_AI | [x] | [x] |
| 30 | enhancing_data_provenance_in_AI_with_BC__quality_model | [x] | [x] |
| 31 | establishing_data_provenance_for_responsible_AI_systems | [x] | [x] |
| 32 | exploiting_BL_to_make_AI_trustworthy__lifecycle_view | [x] | [x] |
| 33 | generative-AI_BC_NFT_federated_learning__6G_environment | [x] | [x] |
| 34 | greenthread-blockchain_non-fungible_token__fashion_platform | [x] | [x] |
| 35 | indy528-federated_learning__model_cards | [x] | [x] |
| 36 | influence_of_blockchain_and_AI__from_Turkey | [x] | [x] |
| 37 | kaputa-blockchain_NFT_and_model_card__and_marketplace | [x] | [x] |
| 38 | luunu-blockchain_MISP_model_cards__sharing_platform | [x] | [x] |
| 39 | machine_learning_model_cards_transparency_review | [x] | [x] |
| 40 | rethink_blockchain_gov_with_AI-the_VOPPA_framework | [x] | [x] |
| 41 | security_and_privacy_on_blockchain | [x] | [x] |
| 42 | technological_convergence_BL_AI__review_and_challenges | [x] | [x] |
| 43 | the_applications_of_BL_in_AI | [x] | [x] |
| 44 | the_convergence_of_ai_and_blockchain__road_ahead | [x] | [x] |
| 45 | towards_blockchain-driven_secure_transparent_audit_logs | [x] | [x] |
| 46 | towards_semantic_versioning_of_open__hugging_face | [x] | [x] |
| 47 | toward_trustworthy_AI-BC-based__learning_systems | [x] | [x] |
| 48 | wedaGPT-generative-AI_Blockchain_self_sovereign__medicine_platform | [x] | [x] |
| 49 | federated_learning-strategies_for_improving_comm_efficiency | [x] | [x] |
| 50 | do_you_need_a_blockchain | [x] | [x] |
| 51 | communication-efficient_learning_of__decentralized_data | [x] | [x] |
| 52 | a_systematic_lit_review__GDPR_vs_blockchain | [x] | [x] |
| 53 | blockchain_scaling_using_rollup | [x] | [x] |
| 54 | on_scaling_decentralized_blockchains | [x] | [x] |
| 55 | resolving_the_trilemma_challenge_in_blockchain | [x] | [x] |
| 56 | the_bitcoin_lightning_network-scalable_off-chain_instant_payments | [x] | [x] |
| 57 | x402-an_open_standard_or_internet-native_payments | [x] | [x] |
| 58 | a_blockchain_integration_to_support__multi-agent_systems | [x] | [x] |
| 59 | a_decision_model_for_blockchain_applicability__conversation_system | [x] | [x] |
| 60 | a_survey_of_multi-agent_deep_reinforcement_learning_w_comm | [x] | [x] |
| 61 | a_decision_framework_for_blockchain_platforms__edge_computing | [x] | [ ] (9 pp) |
| 62 | ai-based_crypto_tokens-the_illusion_of_decentralized_ai | [x] | [ ] (14 pp) |
| 63 | ai_oracle-a_blockchain-powered__ai_agents | [x] | [ ] (10 pp) |
| 64 | a_multi-agent_framework_for__information_systems | — unconvertible | — DEFERRED (no md) |
| 65 | an_extensive_multivocal_lit_review_of_blockchain__and_interoperability | [x] | [ ] (50 pp) |
| 66 | a_ten-step_decision_path_to_determine_when__blockchain_technologies | [x] | [ ] (18 pp) |
| 67 | autonomous_agents_on_blockchains-standards__trust_boundaries | [x] | — DEFERRED (121 pp >50) |
| 68 | betaweb-towards_a_blockchain-enabled_trustworthy_agentic_web | [x] | [ ] (21 pp) |
| 69 | blockagents-towards_byzantine__via_blockchain | [x] | [ ] (6 pp) |
| 70 | blockchain_and_the_future_of_the_internet__comprehensive_review | [x] | [ ] (25 pp) |
| 71 | blockchain-based_coordination-assessing__smart_contracts | [x] | [ ] (21 pp) |
| 72 | blockchain-enhanced_incentive-campatible_mechanisms__learning_systems | [x] | [ ] (17 pp) |
| 73 | blockchain_for_large_language_model_security__holistic_survey | [x] | [ ] (20 pp) |
| 74 | blockchain_intelligence-when_blockchain_meets_ai | [x] | [ ] (5 pp) |
| 75 | blockchain_meets_llms-a_living__bidirectional_integration | [x] | [ ] (17 pp) |
| 76 | blockchain-powered_collaboration_in_heterogeneous_swarms_of_robots | [x] | [ ] (18 pp) |
| 77 | blockchain_solutions_for_multi-agent_robotic_systems__open_questions | [x] | [ ] (5 pp) |
| 78 | byzantine-robust_decentralized_coordination_of_llm_agents | [x] | [ ] (9 pp) |
| 79 | decentralized_multi-agent_system_with_trust-aware_communication | [x] | [ ] (7 pp) |
| 80 | erc-8004-trustless_agents | [x] | [ ] (~12 pp) |
| 81 | evaluating_blockchain_technology_implementation_risks__making_approach | [x] | [ ] (29 pp) |
| 82 | evaluating_suitability_of_applying_blockchain | [x] | [ ] (4 pp) |
| 83 | federated_multi-agent_reinforcement_learning__and_challenges | [x] | [ ] (23 pp) |
| 84 | framework_for_determining_the_suitability__to_consider | [x] | [ ] (23 pp) |
| 85 | multi-agent_coordination_across_diverse_applications-a_survey | [x] | [ ] (23 pp) |
| 86 | multi-agent_systems_and_blockchain__literature_review | [x] | [ ] (15 pp) |
| 87 | secure_multi-llm_agentic_AI_and_agentification__zero-trust-a_survey | [x] | [ ] (35 pp) |
| 88 | securing_federated_learning_with_blockchains-a_systematic_literature_review | [x] | [ ] (35 pp) |
| 89 | self-evolving_multi-agent_reinforcement_learning__open-ended_environments | [x] | [ ] (6 pp) |
| 90 | sok-blockchain_agent-to-agent_payments | [x] | [ ] (21 pp) |
| 91 | sok-bridging_trust_into_the_blockchain__on-chain_identity | [x] | [ ] (9 pp) |
| 92 | sok-security_and_privacy_of_ai_agents_for_blockchain | [x] | [ ] (13 pp) |
| 93 | sok-security_of_auto_llm_agents_in_agentic_commerce | [x] | [ ] (17 pp) |
| 94 | strategic_and_autonomous_orchestrations_of_ai__supply_chains | [x] | [ ] (32 pp) |
| 95 | the_agent_economy-a_blockchain-based_foundation_for_autonomous_ai_agents | [x] | [ ] (17 pp) |
| 96 | the_paradox_of_ai_knowledge-a_blockchain-based__media_industry | [x] | [ ] (32 pp) |
| 97 | the_trust_paradox_in_llm-based_multi-agent_systems | [x] | [ ] (18 pp) |
| 98 | towards_transparent_and_incentive-compatible__blockchain-driven_approach | [x] | [ ] (15 pp) |
| 99 | web3_x_ai_agents-landscape_integrations_and_foundational_challenges | [x] | [ ] (14 pp) |
| 100 | what_blockchain_alternative_do_you_need | [x] | [ ] (17 pp) |
| 101 | when_not_to_use_machine_learning-a__and_limitations | [x] | [ ] (7 pp) |
