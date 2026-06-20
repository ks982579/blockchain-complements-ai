CORRECTED

1. Key Points — shared initialization bullet:
   - Original: "averaging models from different random initializations can diverge"
   - Issue: The paper reserves "diverge" for the large-E training instability case. For different-initialization averaging, the paper says the result "could produce an arbitrarily bad model" (Section 2, Figure 1 left).
   - Fixed: "averaging models from different random initializations can produce arbitrarily bad models"

All other sections (Overview, Background, Conclusion): PASS
