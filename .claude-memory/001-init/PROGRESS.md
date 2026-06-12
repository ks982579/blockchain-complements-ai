# Pipeline Progress

This is the durable index. Read this first in any new session to see what's done and
what's next.

## Steps

- [x] 001-init: Plan written, memory structure created.
- [x] 002-pdf-conversion: `apps/pdf_to_md.py` written, run across all `research/` dirs (48/48 converted, see 002-pdf-conversion/NOTES.md).
- [ ] 003-summarization: `summarize-paper` skill built; summaries written per-paper
      (see checklist below; resumable across sessions).
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

## Per-paper checklist (48 directories)

| # | Directory | paper.md | summary.md |
|---|-----------|----------|------------|
| 1 | accounting_and_auditing_with_blockchain_tech_and_AI__review | [x] | [x] |
| 2 | ai_agents_meet_blockchain__multi-agents | [x] | [x] |
| 3 | AI_and_BC_integration_in_business__content_analysis | [x] | [x] |
| 4 | ai-driven_optimization_of_blockchain_scalability__privacy_protection | [x] | [x] |
| 5 | ai_model_cards-state_of_the_art__automated_use | [x] | [x] |
| 6 | a_marketplace_for_trading_AI__IoT_data | [x] | [ ] |
| 7 | an_explainable_federated_blockchain__healthcare_data | [x] | [ ] |
| 8 | a_review_on_blockchain_technology__AI-driven_solutions | [x] | [ ] |
| 9 | artificial_intelligence_and_blockchain-how_should_emerging_tech_be_gov | [x] | [ ] |
| 10 | a_survey_on_blockchain-based_federated_learning | [x] | [ ] |
| 11 | a_survey_on_blockchain-enabled_federated_learning_and__twin | [x] | [ ] |
| 12 | bassa-ML-a_blockchain_and_model_card__provenance_platform | [x] | [ ] |
| 13 | bias_in_data-driven_ai_systems | [x] | [x] |
| 14 | blockchain_and_AI_integration__transparency_in_finance | [x] | [ ] |
| 15 | blockchain_and_explainable-AI__PCOS_detection | [x] | [ ] |
| 16 | blockchain-as-a-platform-for-ai-transparency | [x] | [ ] |
| 17 | blockchain-based_auditing_of_legal_decisions__AI_tools | [x] | [ ] |
| 18 | blockchain-based_federated_learning-a_survey_and_new_perspectives | [x] | [ ] |
| 19 | blockchain-based_federated_learning_systems__choices | [x] | [ ] |
| 20 | blockchain-based_health_and_usage_monitoring__aerospace_structures | [x] | [ ] |
| 21 | blockchain-enabled_federated_learning-a_survey | [x] | [ ] |
| 22 | blockchain_federated_learning_for_IOT__comp_survey | [x] | [ ] |
| 23 | blockchain_for_ai-review_and_open_research | [x] | [ ] |
| 24 | blockchain_for_deep_learning-review_open_challenges | [x] | [ ] |
| 25 | blockchain_for_explainable_and_trustworhty_AI | [x] | [ ] |
| 26 | blockchain_for_federated_learning_toward_ml__systemic_survey | [x] | [ ] |
| 27 | blockchain_NFT_federated_learning_and_model_cards__sliced_env | [x] | [ ] |
| 28 | CounterweightsAndComplementarities-ConvergeAIAndBlockchain | [x] | [ ] |
| 29 | decentralised_gov-driven_architecture__responsible_AI | [x] | [ ] |
| 30 | enhancing_data_provenance_in_AI_with_BC__quality_model | [x] | [ ] |
| 31 | establishing_data_provenance_for_responsible_AI_systems | [x] | [ ] |
| 32 | exploiting_BL_to_make_AI_trustworthy__lifecycle_view | [x] | [ ] |
| 33 | generative-AI_BC_NFT_federated_learning__6G_environment | [x] | [ ] |
| 34 | greenthread-blockchain_non-fungible_token__fashion_platform | [x] | [ ] |
| 35 | indy528-federated_learning__model_cards | [x] | [ ] |
| 36 | influence_of_blockchain_and_AI__from_Turkey | [x] | [ ] |
| 37 | kaputa-blockchain_NFT_and_model_card__and_marketplace | [x] | [ ] |
| 38 | luunu-blockchain_MISP_model_cards__sharing_platform | [x] | [ ] |
| 39 | machine_learning_model_cards_transparency_review | [x] | [ ] |
| 40 | rethink_blockchain_gov_with_AI-the_VOPPA_framework | [x] | [ ] |
| 41 | security_and_privacy_on_blockchain | [x] | [ ] |
| 42 | technological_convergence_BL_AI__review_and_challenges | [x] | [ ] |
| 43 | the_applications_of_BL_in_AI | [x] | [ ] |
| 44 | the_convergence_of_ai_and_blockchain__road_ahead | [x] | [ ] |
| 45 | towards_blockchain-driven_secure_transparent_audit_logs | [x] | [ ] |
| 46 | towards_semantic_versioning_of_open__hugging_face | [x] | [ ] |
| 47 | toward_trustworthy_AI-BC-based__learning_systems | [x] | [ ] |
| 48 | wedaGPT-generative-AI_Blockchain_self_sovereign__medicine_platform | [x] | [ ] |
