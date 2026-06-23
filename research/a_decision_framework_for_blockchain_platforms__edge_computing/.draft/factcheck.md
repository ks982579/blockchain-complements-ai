# Factcheck report

Result: **CORRECTED**

- Overview: PASS
- Background: CORRECTED (1 issue)
- Key Points: PASS
- Conclusion: PASS

## Issue 1 — Background, citation/attribution error

Original problematic text:
> "Crucially, the framework is explicitly built atop three prior decision-oriented studies that it positions itself against: Wüst and Gervais (2017) ... Xu et al. (2016) ... and Xu et al. (2017) presenting a taxonomy ... plus Conoscenti et al. (2016) on blockchain for IoT."

Conflicting source text (paper.md): "Our framework builds on the knowledge gathered from existing studies and real use cases of blockchain applications, especially in the IoT context [Wüst and Gervais, 2017, Xu et al., 2016, Conoscenti et al., 2016]..."

The paper states the framework is built on three studies: Wüst and Gervais 2017, Xu et al. 2016, and **Conoscenti et al. 2016** — not Xu et al. 2017. Xu et al. 2017 (the taxonomy) appears only in the Related Work section, where it is positioned against, not used as a foundation. The summary wrongly placed Xu et al. (2017) among the three foundational studies and demoted Conoscenti to a "plus." Corrected in place.

All other claims, numbers, and citations across the four sections were verified against paper.md and are accurate (eight comparison properties, six platforms, Hyperledger ~100,000 tx/sec and Ripple ~1,000 tx/sec, Bitcoin 1MB block limit, OP_RETURN/whisper-message use-case details, and the Telit "no blockchain" verdict).
