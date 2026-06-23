# Factcheck report

Result: **CORRECTED**

- Overview: CORRECTED (1 issue)
- Background: CORRECTED (1 issue)
- Key Points: PASS
- Conclusion: PASS

## Issue 1 — Overview, inline arXiv identifier removed

- Original: "...(Queralta & Westerlund, arXiv:1912.01711, University of Turku)..."
- The arXiv ID 1912.01711 is genuine (from the directory's RIS metadata) but does not appear in paper.md's prose, so it was removed from the summary body; the identifier is retained in the summary frontmatter (doi/url).

## Issue 2 — Background, yellow-paper attribution

- Original: "the Ethereum yellow paper and Solidity/EVM smart contracts (Wood/Buterin)"
- The yellow paper (ref [24]) is by Gavin Wood; Buterin is cited only for Casper/sharding, not the yellow paper. Corrected to "the Ethereum yellow paper (Wood) and Solidity/EVM smart contracts".

## Verified correct (no change)

PoW-as-compute-estimation and "data reward" reframing; partial-PoW and fixed-difficulty recommendations; data-stamp / Proof-of-Quality and validation-graph collusion detection; hybrid PoS+PoW; both genesis models; four compute platforms (Intel Up, Up Gateway, NVIDIA Jetson TX2, Intel i5-6200U); hashing-power-vs-CNN-latency correlation; std dev below 3%; Jetson TX2 GPU outlier; CIFAR-10 and KITTI datasets; lidar stamp-size geometry (linear for corner/tree, nonlinear for car); optimization weights α/β; gas-grows-with-payload; all Background author attributions.
