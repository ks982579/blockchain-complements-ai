---
id: bias_in_data-driven_ai_systems
title: "Bias in data-driven artificial intelligence systems—An introductory survey"
authors:
  - Eirini Ntoutsi
  - Pavlos Fafalios
  - Ujwal Gadiraju
  - Vasileios Iosifidis
  - Wolfgang Nejdl
  - Maria-Esther Vidal
  - Salvatore Ruggieri
  - Franco Turini
  - Symeon Papadopoulos
  - Emmanouil Krasanakis
  - Ioannis Kompatsiaris
  - Katharina Kinder-Kurlanda
  - Claudia Wagner
  - Fariba Karimi
  - Miriam Fernandez
  - Harith Alani
  - Bettina Berendt
  - Tina Kruegel
  - Christian Heinze
  - Klaus Broelemann
  - Gjergji Kasneci
  - Thanassis Tiropanis
  - Steffen Staab
year: 2020
venue: "WIREs Data Mining and Knowledge Discovery"
publisher: Wiley
volume: 10
issue: 3
pages: "e1356"
doi: "10.1002/widm.1356"
url: "https://wires.onlinelibrary.wiley.com/doi/abs/10.1002/widm.1356"
type: article
keywords:
  - fairness
  - fairness-aware AI
  - fairness-aware machine learning
  - interpretability
  - responsible AI
---

## Overview

This is a broad, multidisciplinary survey of bias and fairness in data-driven AI
systems, aimed at unifying technical, legal, and ethical perspectives. It organizes the
field into three categories: understanding bias (how bias arises and is manifested in
data, and how fairness is formally defined), mitigating bias (pre-processing,
in-processing, and post-processing technical approaches), and accounting for bias
(proactive bias-aware data collection and retroactive explanation of AI decisions). Its
contribution is a comprehensive map connecting these technical approaches to their
legal grounding (primarily EU law/GDPR), and an articulation of open research
directions toward AI that is fair "by design" within a legal framework.

## Background

The survey explicitly positions itself relative to prior surveys with narrower scope:
machine-ethics-focused work (Yu et al., 2018), explainability/black-box-model surveys
(Atzmueller, 2017; Guidotti et al., 2019), and Web-specific bias work (Baeza-Yates,
2018) — distinguishing itself by covering technical solutions AND their legal grounding
together. It draws on real-world documented bias incidents (e.g., the COMPAS recidivism
tool's racial bias reported by Angwin et al., 2016; Google Ads' gender-biased job ad
delivery, Datta et al., 2015; facial recognition performing worse on darker-skinned
women, Buolamwini & Gebru, 2018) as motivating evidence. On the legal side, it builds on
EU regulatory instruments (GDPR Articles 5, 9, 22; the EU Charter of Fundamental Rights;
Directive 2004/113) and cites over 20 existing formal fairness definitions (Verma &
Rubin, 2018; Zliobaite, 2017) as the technical foundation it organizes and critiques.

## Key Points

- Bias in AI is not new but can be amplified by AI systems via complex socio-technical
  feedback loops (e.g., institutional bias, "pernicious feedback loops" from
  under/over-representation of social groups in training data).
- Bias enters data through sensitive features and their causal correlations (including
  proxy/redundant encodings), and through under- or over-representation of groups
  (selection, reporting, and detection biases).
- More than 20 formal fairness definitions exist, falling into five categories:
  predicted-outcome, predicted-and-actual-outcome, predicted-probabilities-and-actual-
  outcome, similarity-based, and causal-reasoning definitions — but no consensus exists
  on which is "correct," and the problem of formalizing fairness remains open.
- Bias-mitigation techniques fall into three stages: pre-processing (modifying training
  data, e.g., reweighting/relabeling instances), in-processing (modifying the learning
  algorithm via constraints/regularization, e.g., AdaFair, fair-PCA, fair clustering),
  and post-processing (adjusting a trained model's internals or outputs).
- Existing EU law (GDPR Art. 5, 9, 22; EU Charter Art. 20/21) addresses discrimination
  in decisions and data accuracy but provides little concrete guidance on "debiasing"
  data or models, and its real-world effectiveness for automated decision-making (Art.
  22) remains uncertain.
- "Accounting for bias" requires both proactive bias-aware data collection (which is
  itself prone to bias, even among expert assessors) and retroactive explanation of AI
  decisions, supported by formal description/ontology frameworks for documenting bias
  provenance and context.
- Fairness is not a purely statistical/mathematical property — it is dynamic, social,
  context- and culture-dependent ("fair is not fair everywhere"), and requires
  multidisciplinary (technical + legal + social) solutions rather than technical fixes
  alone.

## Conclusion

The survey concludes that, while substantial technical progress has been made on
detecting and mitigating bias in AI systems, there is no conclusive evidence about which
mitigation methods are best (a systematic, standardized evaluation across fairness
notions and AI models is still missing), and formal fairness definitions remain
contested and context-dependent. The authors do not test or validate a specific
method themselves — as a survey, its "findings" are the state-of-the-field assessment
and gap analysis. They explicitly flag several open research directions: (1) systematic
benchmarking/evaluation of existing bias-mitigation methods against standardized
fairness measures; (2) developing realistic, domain- and culture-specific fairness
definitions; (3) extending fairness research beyond supervised learning into
unsupervised and reinforcement learning settings; (4) addressing fairness in
generative/synthetic data and adversarial robustness; (5) improving developer awareness
and diversity to reduce unconscious bias in system design; and (6) closing legal/
regulatory gaps (e.g., binding data-quality standards), particularly noting that EU-wide
consensus on algorithmic fairness regulation is still lacking. The paper frames bias as
a deeply embedded societal problem that technology alone cannot eliminate, requiring
ongoing collaboration between technologists, legal scholars, and policymakers.
