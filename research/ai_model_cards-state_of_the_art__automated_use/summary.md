---
id: ai_model_cards-state_of_the_art__automated_use
title: "AI Model Cards: State of the Art and Path to Automated Use"
authors:
  - Ali Mehraj
  - An Cao
  - Kari Systä
  - Tommi Mikkonen
  - Pyry Kotilainen
  - David Hästbacka
  - Niko Mäkitalo
year: 2025
venue: "International Conference on Web Information Systems and Technologies (WEBIST 2025)"
publisher: "SCITEPRESS"
pages: "71-82"
doi: "10.5220/0013706600003985"
url: "https://www.scitepress.org/Link.aspx?doi=10.5220/0013706600003985"
type: inproceedings
keywords:
  - Model Cards
  - AI Model
  - Privacy
  - Transparency
  - Ethical AI
  - Machine Learning
  - ML
  - Artificial Intelligence
  - AI
---

## Overview

This paper presents a state-of-the-art review of "model cards" — standardized documentation introduced by Google in 2019 to report ML model transparency, performance, and ethical considerations — motivated by emerging regulations such as the EU AI Act, the Data Act, and GDPR that require AI developers to disclose training data, intended use, biases, and risks. The authors' long-term goal is to automate risk analysis and regulatory compliance checks for software systems that incorporate ML components, and this study assesses whether existing model cards can serve as a reliable data source for that vision. To this end, they analyzed 90 model cards sampled from four major repositories (50 from Hugging Face, 20 from Kaggle, 10 from Nvidia NGC Catalog, and 10 from Google Model Garden), examining what information is reported (RQ1), how structurally consistent the cards are across sections and naming conventions (RQ2), and how well they cover ethical/regulatory aspects such as bias, privacy, safety, security, and risk (RQ3). The findings reveal substantial inconsistency: while most cards report basic model details and intended use, only 39 of 90 document training data, only 18 cover factors/metrics, and ethical reporting is sparse — e.g., risk mitigation appears in just 4 cards, bias mitigation in 5, and explicit regulatory compliance declarations in none, with section naming varying widely (e.g., "Quantitative Analyses" never appears verbatim, instead replaced by terms like "Evaluation Results" or "Benchmarks"). Based on these gaps, the paper's main contribution is a unified, more granular model card template — extending Mitchell et al.'s original nine sections with new subsections (e.g., Primary/Secondary Intended Use, a consolidated Ethical Considerations section with Bias, Privacy, Safety and Security, Risk, Misuse, and Regulatory Compliance subsections) — explicitly mapped against the transparency requirements of EU AI Act Annex IV, intended as a foundation for future machine-interpretable model cards usable in automated regulatory compliance and risk-analysis tooling.

## Background

This paper situates its work within a broader regulatory and technical context, drawing on several strands of prior work and external sources.

**Regulatory drivers.** The paper frames the motivation for model cards around emerging EU/EU-adjacent regulations: the EU AI Act (citing Edwards, 2021, "The EU AI Act: a summary of its significance and scope"), the EU Data Act (citing Perarnaud et al., 2022, a Centre for European Policy Studies technical report), and the General Data Protection Regulation / GDPR (citing Voigt and Von dem Bussche, 2017, "The EU General Data Protection Regulation (GDPR): A Practical Guide"). These are presented as the compliance pressures that make transparent AI model documentation increasingly necessary. The paper also references Annex IV of the EU AI Act specifically, which sets out transparency obligations for AI systems, and uses it as a benchmark against which to compare its proposed model card template — a researcher following up on this work should look closely at the actual Annex IV text (linked via EUR-Lex) for the precise transparency requirements being mapped.

**Origin of model cards.** The core concept of "Model Cards" is attributed to Google, introduced by Mitchell et al. (2019), "Model Cards for Model Reporting" (FAccT 2019). This paper treats Mitchell et al.'s original nine-section structure (Model Details, Intended Use, Factors, Metrics, Evaluation Data, Training Data, Quantitative Analyses, Ethical Considerations, Caveats and Recommendations) as the canonical baseline against which it compares the 90 model cards it surveys, and as the starting point for its own proposed unified template. Anyone wanting to understand the "ground truth" structure being evaluated should read this original paper.

**Claims about the purpose and value of model cards.** The paper cites several secondary sources to support claims about why model cards matter:
- Dan and Seeuws (2024) is cited for the claim that model cards facilitate comprehension, assessment, and reproducibility of ML research results (in a clinical/medical journal context).
- Nunes et al. (2022), "Using model cards for ethical reflection: a qualitative exploration" (Brazilian Symposium on Human Factors in Computing Systems), is cited for the claim that model cards aim to increase transparency around intended use, performance, limitations, and developer considerations.
- Wadhwani and Jain (2020), "Machine learning model cards transparency review: Using model card toolkit" (IEEE PuneCon 2020), is cited for the claim that model cards help stakeholders compare models, reduce costs, and make better selection decisions.

**Adoption and tooling ecosystem.** The paper notes that model cards have gained traction across major organizations (Google, Meta, Nvidia, OpenAI, Anthropic) and that Hugging Face hosts over 1.6 million models with associated cards. It points to existing toolkits and templates as evidence of an established (if inconsistent) ecosystem: Google's Model Card Toolkit, TensorFlow's model-card-toolkit (GitHub), the PyPI model-card-toolkit package, and Hugging Face's annotated model card documentation. It also references Amazon's "AI Service Cards" as a related, possibly inspired-by, documentation format. These are all potentially useful artifacts for a researcher to examine directly when assessing the practical state of model card tooling.

**Domain-specific extensions of the model card concept.** The paper cites three lines of prior work that adapted or extended the model card idea:
- Mongan, Moy, and Kahn (2020), "Checklist for Artificial Intelligence in Medical Imaging (CLAIM)," described as a checklist whose required information corresponds closely to model card content, applied in the medical imaging domain.
- Booth and Ghosh (2023), "Machine learning model cards toward model-based system engineering analysis of resource-limited systems" (SPIE), cited for work incorporating empirically derived sensor-fusion RNN performance and cost data into machine-readable model cards.
- Amith et al. (2022), "Toward a standard formal semantic representation of the model card report" (BMC Bioinformatics), cited for an approach using biomedical/oncology terminology and semantic web technology to make model cards machine-readable.

The paper frames these three as evidence that model cards have been adapted for various purposes and that there are existing attempts at machine interpretability, but claims that none of this prior work addresses making model cards useful for system-level software architecture design — the gap this paper's broader research program aims to fill.

**The authors' own prior work.** The paper builds directly on the authors' earlier research: Kotilainen et al. (2024), "The Programmable World and Its Emerging Privacy Nightmare" (International Conference on Web Engineering), cited for the observation that the model card format is continuously evolving; and Kotilainen et al. (2025), "Allocating Distributed AI/ML Applications to Cloud–Edge Continuum Based on Privacy, Regulatory, and Ethical Constraints" (Journal of Systems and Software), cited as prior work where the authors used a model-card-like "metadata card" concept for safe, ethical orchestration and deployment of ML models, and as an example of analyzing system-level risk using model-provided metadata. This 2025 paper is presented as the immediate predecessor whose vision (automated regulatory compliance and risk analysis in software systems) motivates the present state-of-the-art review.

**Repositories surveyed.** Although not "prior work" in the citation sense, the paper's empirical analysis is grounded in publicly accessible model repositories — Hugging Face, Kaggle, Nvidia NGC Catalog, and Google Model Garden — which a researcher may want to inspect directly to understand the source material (90 model cards) underlying the paper's findings.

## Key Points

- This paper presents a state-of-the-art review of 90 AI model cards sampled from four major model repositories (Hugging Face, Kaggle, Nvidia NGC Catalog, and Google Model Garden), collected between 04.04.2025 and 10.04.2025, to assess their structure, content, and ethical reporting.
- The study finds significant variance in the information reported across model cards: 85 of 90 included basic model details, but only 69 reported intended use, 39 reported training data, 36 reported evaluation/performance data, and only 18 reported factors/metrics-related data.
- The paper documents that ethical considerations are inconsistently and often only partially reported: bias-related information appeared in 23 model cards, privacy in 8, safety in 13, security in 7, risk in 13, and limitations in 38, with only 13 model cards using a dedicated "Ethical Considerations" (or similarly named) section.
- The study shows that mitigation measures for identified issues are rarely reported—risk mitigation appeared in only 4 of 34 model cards reporting risk factors, and bias mitigation appeared in only 5 of 22 model cards reporting bias.
- The paper finds that no model card in the sample contained an explicit regulatory compliance declaration, and only 6 model cards (all from Nvidia) demonstrated partial adherence to GDPR-related data practices without explicitly declaring GDPR compliance.
- The study identifies substantial inconsistency in section naming and structure across model cards (Table 3), noting that sections such as Factors, Metrics, Evaluation Data, and Quantitative Analyses from the original Mitchell et al. (2019) model card template are rarely used verbatim and are instead merged or renamed (e.g., as "Evaluation Results," "Benchmark," or "Performance").
- The paper reports repository-level differences in model card quality and structure: Hugging Face cards generally cover model details, intended use, evaluation, training data, and limitations; Kaggle cards mostly contain only model details and intended use; Nvidia NGC Catalog cards are the most comprehensive, including detailed ethical considerations (privacy, safety, security, risk); and Google Model Garden cards are largely limited to model details, with notable inconsistency between its Gemma and Gemini model families.
- The paper proposes a new unified model card template (Table 5) that extends the original Mitchell et al. (2019) structure with explicit subsections for Primary/Secondary Intended Use, Hardware/Software Factors, consolidated Evaluation, Training Procedure, and an expanded Ethical Considerations section covering Bias, Privacy, Safety and Security, Risk, Misuse, and a new Regulatory Compliance subsection.
- The paper maps its proposed model card template against the transparency requirements of Annex IV of the EU AI Act (Table 6), arguing that the template's sections (Model Details, Intended Use, Factors, Training Data, Evaluation, and Ethical Considerations subsections) cover the corresponding Annex IV obligations.
- The paper argues that, in their current loosely structured and inconsistently reported form, most existing model cards are insufficient to support automated risk analysis and regulatory compliance checking, and that a well-structured, standardized model card is a necessary first step toward future machine-interpretable model cards for system-level design and regulatory-compliance automation.

## Conclusion

The paper set out to assess whether existing model cards, as practiced across major ML model repositories, could serve as a foundation for the authors' longer-term goal of automating risk analysis and regulatory compliance checks for AI-enabled software systems. To do this, the authors reviewed 90 model cards drawn from Hugging Face, Kaggle, Nvidia's NGC Catalog, and Google's Model Garden, organized around three research questions covering the information content, structural consistency, and ethical/regulatory coverage of these documents.

The results largely confirm the paper's motivating concern rather than its hopeful framing: model cards in practice are inconsistent and, for the purposes the authors care about, insufficient as-is.

- **Information content (RQ1):** Basic model details and intended-use information are reasonably common (present in 85 and 69 of 90 cards, respectively), but training data, factors/metrics, and evaluation-related information are reported far less consistently (39, 18, and 36 cards respectively). Coverage varies sharply by repository — Hugging Face and Nvidia's NGC Catalog provide relatively richer documentation, Kaggle cards are largely minimal, and even Google's own Gemini-family cards deviate from its Gemma-family template and from Google's own pioneering model card concept.

- **Structural consistency (RQ2):** Section naming and organization vary widely across cards, even for semantically equivalent content (e.g., "Factors," "Metrics," "Evaluation Data," and "Quantitative Analyses" from the original Mitchell et al. 2019 template are frequently merged, renamed, or omitted entirely — "Quantitative Analyses" as a label appears in none of the 90 cards). This structural heterogeneity is identified as a major obstacle to automated extraction and machine-interpretability.

- **Ethical/regulatory coverage (RQ3):** This is where the gap is most pronounced. Only 13 of 90 cards have a dedicated "Ethical Considerations" section (though related content is often scattered into other sections). Risk mitigation was reported in only 4 cards (despite risk factors appearing in 34), bias mitigation in only 5 (despite bias being mentioned in 22), and safety/security policies in zero cards. No card made an explicit regulatory compliance declaration, though six Nvidia cards showed partial, implicit alignment with GDPR-adjacent practices (e.g., data correction/removal).

**Support for the paper's claims:** The authors' initial premise — that model cards are a promising but currently underdeveloped vehicle for automated compliance/risk tooling — is supported. The data clearly show that the *concept* has been widely adopted (all major repositories provide some form of model card) but that current practice falls well short of what would be needed for automated, machine-interpretable regulatory analysis, particularly given the near-total absence of explicit regulatory compliance statements and risk/bias mitigation reporting. The paper does not claim model cards are unsuitable in principle; rather, it concludes they could become suitable if standardized and expanded.

**Main contribution / response to the gap:** Based on these findings, the authors propose a unified, more granular model card template (Table 5) that restructures and extends the original Mitchell et al. (2019) sections — notably consolidating evaluation-related content, adding explicit subsections for bias, privacy, safety/security, risk, and misuse under "Ethical Considerations," and introducing a new "Regulatory compliance" section with subsections for adherence and declaration. They map this template against the EU AI Act's Annex IV transparency requirements (Table 6) to argue it would help close the compliance-documentation gap.

**Limitations acknowledged by the authors:**
- The "Threats to Validity" section notes that some model repositories may have been overlooked, and that the four selected repositories lacked a common sorting/selection mechanism (downloads vs. likes vs. "trending" vs. "popular"), which may have introduced selection bias affecting comparability across repositories.
- The sample (90 cards) is acknowledged as a snapshot of top/popular models rather than a comprehensive or random sample, and the authors explicitly call for future work with a larger pool of model cards to validate and generalize the findings.
- The proposed template itself is explicitly framed as a *starting point*, not a finished solution: the authors state it "is not yet machine-interpretable and would require further refinement and proper tooling" before it could support automated extraction.

**Open questions / future research directions:**
- Extending the state-of-the-art review to a larger and more representative set of model cards.
- Operationalizing the proposed template into a machine-interpretable format (e.g., via semantic web technologies, as hinted at by related prior work cited in the paper) and building tooling to automatically extract risk and regulatory information from model cards.
- Assessing the *quality* (not just presence) of ethical/transparency reporting in model cards.
- Integrating model-card-derived information into system-level architecture design, risk analysis, and regulatory compliance checking for software systems that embed ML components — the authors' overarching long-term research agenda, of which this paper is an early, foundational step.
- Anticipating that enforcement of the EU AI Act will likely push developers toward more complete regulatory disclosures in model cards, which the authors see as both a future opportunity and a partial validation of the need for their proposed template.
