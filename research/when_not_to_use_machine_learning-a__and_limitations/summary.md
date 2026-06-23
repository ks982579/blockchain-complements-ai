---
id: when_not_to_use_machine_learning-a__and_limitations
title: "When not to use machine learning: a perspective on potential and limitations"
authors:
  - Matthew R. Carbone
year: 2022
venue: "arXiv"
doi: "10.48550/arXiv.2210.02666"
url: "https://arxiv.org/abs/2210.02666"
type: misc
keywords:
  - artificial intelligence
  - machine learning
  - modeling
  - data-driven modeling
  - limitations
---

## Overview

This perspective paper by Matthew R. Carbone (Brookhaven National Laboratory, 2022) tackles a question that is rarely addressed directly in the machine learning (ML) literature: when should one *not* use ML? Rather than cataloguing successful applications, it foregrounds the foundational principles of data-driven modeling — chiefly that supervised ML is, at its core, the minimization of *distance* between predicted and ground-truth values — and shows how the very principles that grant ML its "almost magical" predictive power simultaneously bound the class of problems it can solve. Aimed primarily at scientists in the materials, chemical, and physical sciences (though the arguments generalize to data-driven methods broadly), the paper identifies concrete failure conditions: poorly discriminative target distances, ill-posed inverse problems, low signal-to-noise ratios, training/deployment distribution mismatch, and the impossibility of reliable extrapolation beyond the union of training data and encoded prior belief. It frames these limitations against the historical backdrop of "AI Winters" caused by inflated expectations, arguing that understanding ML's limits is essential to avoid repeating that cycle and to empower (not replace) human researchers.

## Background

The paper situates itself in a long lineage the author cites from elsewhere: the origins of AI in the mid-1900s with Turing's "can machines think?" question (Turing 1950; Jordan & Mitchell 2015), the neural-network backbone tracing to McCulloch & Pitts (1943) and rooted in established statistics and applied-mathematics theory (Sarle 1994), with the computational machinery to realize ML arriving decades later (Bishop & Nasrabadi 2006). It draws on documented histories of the multiple "AI Winters" — periods of scarce funding and depleted expectations (Crevier 1993; Hendler 2008; Reyes & Maruyama 2019; Floridi 2020). The paper adopts the common framing (citing Holzinger et al. 2018) that AI is a superset of ML, while noting alternative views (Langley et al. 2011), and uses Mitchell's (2014) definition of ML as learning a task without task-specific heuristics. For its technical claims it leans on cited work covering: the supervised-learning-as-distance-minimization view and optimizers like SGD and Adam (Kingma & Ba 2014); reinforcement learning via Deep Q-networks (Mnih et al. 2015); variational autoencoders (Kingma & Welling 2014; Higgins et al. 2017); Gaussian Processes and kernels (Rasmussen & Williams 2006); SMILES molecular encodings (Weininger 1988) and latent-space molecular design (Sanchez-Lengeling & Aspuru-Guzik 2018; Gómez-Bombarelli et al. 2018); the Materials Project database (Jain et al. 2013); neural network potentials (Behler & Parrinello 2007; Behler 2011, 2021); adversarial examples (Goodfellow et al. 2015); uncertainty-aware/Bayesian deep learning (Srivastava et al. 2014; Wilson & Izmailov 2020; Jospin et al. 2022); transfer learning (Weiss et al. 2016); and best-practice ML tutorials for the materials community (Wang et al. 2020; Artrith et al. 2021). It also cites coherent diffraction imaging / phase retrieval (Zhang et al. 2016; Wu et al. 2021) as a concrete ill-posed inverse problem, and points to "data work" as a human, not technical, challenge (Sambasivan et al. 2021).

## Key Points

- Almost all supervised ML models are paradigmatically identical: they reduce to minimizing a *distance* (a metric like Euclidean/Manhattan, or a divergence/cross-entropy) between model predictions and ground truth; this unifying lens (extending to RL, clustering, dimensionality reduction, and VAEs) is the foundation for reasoning about when ML works.
- The length scales that let a model differentiate data points in feature space are set by the corresponding *target* values; large target changes can make a model "focus" on the inputs driving them at the expense of small changes (the bias-variance trade-off), so target/feature distance structure dictates effective use cases.
- Feature representations must be *mathematically rigorous*, not merely intuitive to a human; what appears intuitive to the experimenter may be meaningless to the machine.
- ML can only model *functions* (each input mapping to a unique output); inverse problems are often ill-posed/degenerate, so ML is inappropriate unless the problem is reformulated into a non-degenerate subspace expressible as a function.
- When signal-to-noise ratio is low or near 1, no data-driven technique will succeed because targets cannot be distinguished; even uncertainty-aware models may yield error bars too large to be useful.
- The testing set has two distinct roles that are not always the same: an unbiased disjoint sample for estimating generalization, *and* a representation of the real-world deployment scenario; conflating them invites failure.
- If training data come from a different distribution than the deployment data, the model is highly likely to fail; a practical sanity check is to recombine two datasets and sample randomly — if you can easily tell which set a sample came from, they are out-of-sample relative to each other.
- ML is *most* powerful when explicitly fit on the same type of data it will be deployed on; this narrow specialization is a strength, not a weakness (exemplified by system-specific neural network potentials).
- Proper data selection is a *human* challenge, not a technical one; if the model's performance cannot be validated on labeled deployment-distribution data, the model cannot be used.
- A data-driven "No Free Lunch" principle: models interpolate reliably only within the union of their training data *and* encoded prior belief — they do *not* extrapolate beyond it with any reliability; "data-driven" is arguably better understood as "information-driven."
- No amount of data can teach a model a structural property (e.g., periodicity) that is not encoded as prior belief; a Gaussian Process with a periodic kernel needs only a handful of points where a vanilla neural network fails even within the data's convex hull, because the optimizer has no incentive where ground truth is absent. Bayesian models revert to the prior mean far outside the fitted region.
- Active learning (using uncertainty-quantifying models like Gaussian Processes or ensembles) lets an algorithm detect when it is extrapolating and expand its interpolation window by sampling new high-uncertainty data.
- The way humans process data differs fundamentally from how ML algorithms do, and overlooking this is a recurring source of misapplication.

## Conclusion

The paper argues, largely through illustrative reasoning and worked 1-D examples (Gaussian Process vs. neural network fits) rather than new experiments, that ML's predictive power and its limitations stem from the same root — distance-based optimization over data and encoded priors — and that recognizing the boundary conditions (non-discriminative target distances, ill-posed inversions, low SNR, distribution shift, and the impossibility of true extrapolation) is as important as knowing how to apply the methods. Its central thesis is supported by these conceptual demonstrations: e.g., a neural network cannot infer periodicity from any quantity of data without a periodic prior, and models fit on one distribution should never be assumed performant on another. In its outlook, the paper warns that the original AI Winters were caused by outrageously inflated expectations tied to the "arrogant" goal of building a synthetic human-like thinker, and cautions the scientific community to keep expectations measured to avoid triggering the next one. It positions current AI/ML as a suite of data-driven tools meant to *empower* rather than replace human researchers, who remain responsible for formulating hypotheses, identifying appropriate use cases, and applying the tools correctly. Open/forward-looking threads it raises include: refining ill-posed inverse problems into learnable non-degenerate subspaces, broader adoption of uncertainty quantification and active learning to handle out-of-sample deployment, better encoding of prior belief to extend effective model reach, and the long-horizon (currently distant) prospect of genuinely sentient AI. The paper notes that simply trying ML on uncertain (e.g., inverse) problems carries no harm, but that possibility must never be mistaken for certainty.
