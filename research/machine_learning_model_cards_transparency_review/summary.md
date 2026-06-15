---
id: machine_learning_model_cards_transparency_review
title: "Machine Learning Model Cards Transparency Review: Using model card toolkit"
authors:
  - Abhishek Wadhwani
  - Priyank Jain
year: 2020
venue: "2020 IEEE Pune Section International Conference (PuneCon)"
pages: "133-137"
url: "https://api.semanticscholar.org/CorpusID:232152164"
type: inproceedings
keywords:
  - Model documentation
  - Disaggregated evaluation
  - Fairness in AI
  - Machine Learning model evaluation
  - Ethics
  - Datasheets
---

## Overview

This paper is a review-style discussion of "Model Cards," a documentation framework proposed by Google in 2020 to bring transparency to machine learning models, and of Google's accompanying Model Card Toolkit (MCT), which automates much of the process of compiling and rendering such cards. The core problem addressed is that, unlike mature industries (e.g., electronics, pharmaceuticals, automotive) that provide standardized datasheets describing performance under varying conditions, ML models are typically distributed with no standardized account of their performance characteristics, blind spots, or appropriate use conditions. The paper explains how the MCT uses a JSON schema backed by ML Metadata (MLMD) to store provenance and evaluation information, and an API to render that information into human-readable cards (with customizable HTML templates for non-TensorFlow users). It walks through two worked examples — an image-based "smiling" classifier evaluated on the CelebA dataset and the Perspective API's text-based TOXICITY classifier (versions 1 and 5) — to illustrate how disaggregated, intersectional performance reporting can reveal fairness problems such as differential false-positive/false-negative rates across age, gender, and identity-term groups. Its contribution to the field is largely expository/advocacy: synthesizing the rationale, mechanics, and stakeholder use cases for model cards while flagging open challenges (intellectual property exposure, qualitative-information communication) that remain for broader adoption of model-card-based transparency.

## Background

The paper situates model cards within a lineage of prior proposals for standardized AI/data documentation, citing "Data Statements for NLP" (Bender and Friedman, 2018), "Datasheets for Datasets" (Gebru et al., 2018), and the "Dataset Nutrition Label" (Holland et al., 2018) as earlier efforts to standardize how datasets are described so users understand appropriate contexts of use. It draws an analogy to the electronic hardware industry's practice of providing component datasheets characterizing performance under varying test conditions, arguing ML lacks an equivalent. To motivate the need for disaggregated and intersectional evaluation, the paper cites real-world examples from outside ML: FDA guidance requiring clinical trial results to be reported by demographic subgroup (age, race, sex) following cases where drugs tested mainly on men caused harms in women, and automotive crash testing that historically used male-only crash test dummies despite women's higher injury risk. It explicitly invokes Kimberlé Crenshaw's intersectionality theory (1989) and the Emma DeGraffenreid v. General Motors case to justify evaluating models across intersecting demographic categories (e.g., race and gender simultaneously) rather than single attributes in isolation. The two worked examples build directly on existing public work: the CelebA face-attributes dataset (Liu et al., 2015) for the smiling classifier, and the Perspective API TOXICITY classifier together with prior bias-measurement work by Dixon et al. (2018) and Vasserman et al. (2018) on unintended bias toward identity terms. The paper also references differential privacy (Dwork, 2008) as a potential mechanism for sharing aggregate training-data statistics without exposing individual-level proprietary data.

## Key Points

- The paper argues that model cards represent a first standardized step toward ethical reporting practices for ML models, enabling stakeholders to compare models before deployment for a specific use case.
- It contends that evaluation datasets used for model cards should represent not just typical use cases but also anticipated edge cases/test scenarios (e.g., evaluating a workplace-deployment model both on a population matching the workplace and on a more challenging population such as children or the elderly).
- It identifies and enumerates seven distinct stakeholder groups (ML practitioners, system developers, software engineers, policymakers, businesses, non-expert "ML-adjacent" decision-makers, and affected individuals) who each derive different value from model card information.
- Through the CelebA "smiling classifier" example, the paper claims that disaggregated evaluation reveals a higher false discovery rate for older adults and a higher false-negative rate for men, demonstrating concrete fairness blind spots that aggregate metrics would hide.
- Through the Perspective API TOXICITY example (v.1 vs. v.5), the paper claims model cards demonstrate how models evolve over time, showing that v.1 had low accuracy on identity terms like "lesbian," "gay," and "homosexual," and that subsequent bias-mitigation work improved v.5's behavior.
- The paper asserts that making model cards a routine part of model/API release processes (as done by the Perspective API team) helps teams systematically identify and reduce demographic biases across model versions.
- It proposes that future evaluation infrastructure could incorporate differential privacy mechanisms so that disaggregated subgroup reporting does not enable re-identification of individuals.
- It recommends that where full disclosure of training data characteristics is not feasible (due to proprietary constraints or non-disclosure agreements), model creators should at minimum report distributional statistics over demographic groups in the training data.
- The paper frames slicing evaluation results by demographic category as compatible with broader mathematical/algorithmic notions of fairness, and as a method for surfacing errors that fall disproportionately on specific subgroups.
- It describes the MCT's architecture: a JSON schema populated automatically from ML Metadata (MLMD), paired with an API that renders this JSON into a visual card, with default UI templates and support for custom HTML templates for non-TensorFlow users.

## Conclusion

The paper concludes that model cards address a genuine and previously unmet need for standardized, comparable performance reporting in ML, analogous to datasheets in other mature industries, and that the MCT lowers the barrier to producing such cards by automating data collection and rendering. However, it does not present new experimental results of its own beyond reproducing/discussing existing examples (CelebA smiling classifier, Perspective API TOXICITY v.1/v.5); its contribution is conceptual and synthetic rather than empirical. The paper explicitly flags unresolved challenges as open issues: (1) the tension between transparency and intellectual property/trade-secret protection, suggesting federated learning or differential privacy as possible mitigations; and (2) the difficulty of communicating qualitative information (intended use, limitations, input specifications) in a standardized way. It also notes, as a forward-looking research direction, the lack of common, fit-for-purpose evaluation datasets for disaggregated/intersectional analysis and calls for future work on developing reliable assessment databases and protocols for the kinds of quantified, subgroup-level evaluation the model card framework calls for.
