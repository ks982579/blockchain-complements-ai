# Factcheck report

Result: **CORRECTED**

- Overview: PASS
- Background: CORRECTED (1 issue)
- Key Points: CORRECTED (1 issue)
- Conclusion: PASS

1. Background section — citation mis-attribution.
   Original problematic text:
   "It cites a UK Government Chief Scientific Adviser report (distributed ledger
   technology beyond blockchain) and Omohundro on cryptocurrencies and smart
   contracts to situate broader exploration by startups, enterprises, and
   governments across supply chain, health records, voting, energy, ownership,
   identity, and critical infrastructure. The authors note (citing Gartner) that
   90% of enterprise blockchain projects from 2015 were predicted to fail within
   18-24 months, ..."

   Conflicting source text (paper.md):
   - "Many startups, enterprises, and governments [3] are exploring blockchain
     applications in areas as diverse as supply chain, electronic health records,
     voting, energy supply, ownership management, identity management, and
     protecting critical civil infrastructure." (reference [3] = UK Government
     Chief Scientific Adviser report)
   - "Gartner estimated that 90% of enterprise blockchain projects launched in
     2015 would fail within 18 to 24 months [4]." (reference [4] = "S. Omohundro.
     Cryptocurrencies, smart contracts, and artificial intelligence")

   The summary attached Omohundro [4] to the startups/enterprises/governments
   exploration claim, which is actually cited to [3]. Omohundro [4] is in fact the
   citation for the Gartner 90% / 18-24-months statistic.

   Replacement text:
   "It cites a UK Government Chief Scientific Adviser report (distributed ledger
   technology beyond blockchain) to situate broader exploration by startups,
   enterprises, and governments across supply chain, health records, voting,
   energy, ownership, identity, and critical infrastructure. The authors note
   (citing Omohundro's work on cryptocurrencies, smart contracts, and artificial
   intelligence) that Gartner estimated 90% of enterprise blockchain projects
   from 2015 would fail within 18-24 months, ..."

2. Key Points, final bullet — unsupported/misleading distinguishing factor for EHRs.
   Original problematic text:
   "the distinguishing factors being EHRs' confidentiality/decentralized-authority
   needs and the stock market's confidentiality plus high-performance requirement."

   Conflicting source text (paper.md):
   In Table I, the EHR column shows B = "Decentralized" and D = "Confidential".
   For criterion B the paper states blockchain "is suitable for scenarios without
   any trusted authority or the current trusted authority has potential to be
   decentralized" — i.e., a "Decentralized" answer is blockchain-FAVORABLE, not a
   reason against blockchain. The paper's stated reason EHRs are unsuitable is the
   confidentiality requirement (Conclusion: "EHRs ... is not suitable yet due to
   the nature or limitation of blockchain"; EHR text emphasizes data must remain
   private/confidential). Listing "decentralized-authority needs" as a reason
   AGAINST blockchain for EHRs is unsupported and misleading.

   Replacement text:
   "the distinguishing factors being EHRs' confidentiality requirement and the
   stock market's confidentiality plus high-performance requirement."

NOTE: Both corrections could not be written into summary.md because the harness
blocks writes to that file. The intended corrected summary.md content is supplied
in the subagent's response message for the caller to apply.

Verified as accurate (no change needed):
- Framework has exactly SEVEN criteria, labeled A-G, matching the seven labels and
  ordering (A multi-party, B trusted-authority, C centralized operation,
  D transparency vs confidentiality, E data integrity, F immutability,
  G high performance). Source: paper.md line 29 ("mainly seven questions") and
  Section II subsections A-G plus Table I rows A-G.
- Four case studies and verdicts (Table I): Supply chain = Blockchain (suitable);
  EHR = Database (not suitable); Identity = Blockchain (suitable); Stock Market =
  Database (not suitable). Matches summary.
- Throughput: "3-20 transactions per second" (public blockchain) vs VISA "1,700
  transactions per second". Matches summary.
- Gartner: "90% of enterprise blockchain projects launched in 2015 would fail
  within 18 to 24 months". Matches summary.
- Deloitte: "42% of the companies in consumer goods and manufacturing plan to
  spend at least $5 million on blockchain technology in 2017". Matches summary.
