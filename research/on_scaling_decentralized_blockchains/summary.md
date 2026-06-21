---
id: on_scaling_decentralized_blockchains
title: "On Scaling Decentralized Blockchains (A Position Paper)"
authors:
  - Kyle Croman
  - Christian Decker
  - Ittay Eyal
  - Adem Efe Gencer
  - Ari Juels
  - Ahmed Kosba
  - Andrew Miller
  - Prateek Saxena
  - Elaine Shi
  - "Emin G\xFCn Sirer"
  - Dawn Song
  - Roger Wattenhofer
year: 2016
venue: "Financial Cryptography and Data Security"
publisher: "Springer Berlin Heidelberg"
pages: "106-125"
doi: "10.1007/978-3-662-53357-4_8"
type: inproceedings
keywords:
  - blockchain
  - scalability
  - Bitcoin
  - decentralization
  - reparameterization
---

## Overview

This position paper by Croman et al. (IC3, Cornell, ETH Zurich, and others) systematically analyzes the scalability limitations of Bitcoin and decentralized blockchains, combining empirical measurement of the live Bitcoin network with a structured exploration of the design space for next-generation protocols. The authors measure block propagation times, per-node bandwidth, and cost-per-confirmed-transaction (CPCT) to quantify how far simple parameter tuning (block size and block interval adjustments) can push Bitcoin's throughput, finding that the current peer-to-peer overlay limits throughput to roughly 27 transactions/sec at 90% node coverage -- far below Visa's 2,000 tx/sec average. They argue that fundamental protocol redesign, not merely reparameterization, is required and propose a five-plane decomposition (Network, Consensus, Storage, View, Side) as an organizing framework for evaluating and combining scaling techniques. The paper catalogs a range of existing and new ideas across these planes and identifies concrete open research challenges for building provably secure, high-performance decentralized blockchains.

## Background

The paper builds on Decker and Wattenhofer's 2012 measurement study of Bitcoin block propagation, which found that for blocks larger than roughly 20 KB, propagation time grew linearly with block size and that a full 1 MB block would have taken about 5 minutes to reach 90% of nodes in 2012. It references the ongoing Bitcoin community debate around Bitcoin Improvement Proposals (BIPs 100, 101, 102, 103) and the segregated witness proposal for modifying block size limits, noting the fragmentation into Core, XT, Classic, and Unlimited implementations. For consensus alternatives, the paper draws on the GHOST protocol by Sompolinsky and Zohar and the inclusive blockchain protocol by Lewenberg et al., which improve fairness and mining power utilization by accounting for off-main-chain forks. It cites Bitcoin-NG (Eyal et al.) as demonstrating that the inherent tradeoff between consensus speed, bandwidth, and security in Nakamoto consensus can be eliminated. The paper references proof-of-stake proposals such as NXT and PPCoin (King and Nadal), while noting that these lack formal convergence guarantees (citing Bentov et al.). For sidechains, it draws on Back et al.'s pegged sidechain proposal and the Lightning Network (Poon and Dryja) and duplex micropayment channels (Decker and Wattenhofer). On the cryptographic side, it references SNARKs (Ben-Sasson et al.; Parno et al.) as a mechanism for outsourcing view computation, and Kosba et al. for composable SNARK protocols. For network-layer improvements, the paper cites set reconciliation protocols, expander-graph-based overlay topologies, and prior work on scalable DHTs (Awerbuch and Scheideler) and Byzantine-resilient dynamic networks (Guerraoui et al.).

## Key Points

- Bitcoin's 90%-effective throughput on the current overlay network is only 55 Kbps, meaning a 1 MB block takes up to 2.4 minutes to reach the slowest 10% of nodes -- a significant fraction of the 10-minute block interval.
- Given the current overlay network and 10-minute block interval, block size should not exceed 4 MB (corresponding to at most 27 tx/sec) to maintain 90% node coverage; the block interval should not be reduced below roughly 12 seconds to fully utilize network bandwidth.
- The cost per confirmed transaction in Bitcoin is estimated at $1.4-$2.9 at maximum throughput and up to $6.2 at de facto throughput, with approximately 98% attributable to mining (proof-of-work electricity and hardware costs).
- Per-node bandwidth measurements of over 4,000 Bitcoin nodes show that individual nodes are provisioned with far more bandwidth than the overlay network actually achieves, indicating that Bitcoin's network-layer protocol (double transmission of transactions, lack of pipelining, multi-hop propagation delays) is the primary bottleneck -- not the underlying infrastructure.
- Reparameterization alone (adjusting block size and interval) is insufficient for meaningful scalability; fundamental protocol redesign is necessary to close the gap with mainstream payment processors.
- The paper introduces a five-plane decomposition of blockchain systems -- Network, Consensus, Storage, View, and Side Planes -- as a structured framework for reasoning about the design space of scalable blockchains.
- Network Plane improvements include set reconciliation protocols (to avoid transmitting each transaction twice), dedicated relay networks, and robust P2P overlays based on expander graphs with randomized peer placement.
- Consensus Plane alternatives include protocols like GHOST and Bitcoin-NG that decouple throughput from security loss due to forks, proof-of-stake systems (which lack formal convergence proofs), consortium/BFT-based consensus (which achieves 4,500 tx/sec with 64 nodes at 1.79-second latency but does not scale to hundreds of nodes), and consensus sharding across concurrent node sets.
- Sidechains introduce three technical challenges: independent security (mining power dilution), cross-chain transaction overhead that may burden the main chain, and high latency from sequential multi-chain confirmation requirements.
- The Storage Plane can be improved by sharding UTXO storage across nodes, though generalizing sharded storage to arbitrary smart contract state with authenticated reads remains an open problem.
- The View Plane can be optimized by outsourcing view computation via SNARKs, with experimental results showing an amortized SNARK cost as low as $0.0154 per transaction for a simple balance-tracking view.
- Payment networks (Lightning Network, duplex channels) form essentially separate Network and Consensus Planes backed by Bitcoin, and their performance depends on emergent graph properties (channel capacity, route discoverability, node availability), making it unclear whether they can outperform Bitcoin's native layers overall.

## Conclusion

The paper concludes that reparameterization of Bitcoin's block size and interval is at best a modest first step, capable of improving throughput to roughly 27 tx/sec while preserving 90% node participation, but falling far short of the thousands of transactions per second needed to compete with centralized payment processors. The authors' measurement data confirms that the peer-to-peer overlay protocol, not the underlying node infrastructure, is the binding constraint. Their five-plane framework reveals that no single technique suffices: meaningful scaling will require coordinated advances across the network, consensus, storage, and view layers. Key open challenges identified include developing better continuous monitoring and measurement techniques for decentralized networks (to answer questions like how far parameters can be pushed without sacrificing security), designing Byzantine-resilient overlay topologies for highly dynamic networks, sharding consensus and storage in adversarial settings, formally proving security properties of proof-of-stake protocols, and building end-to-end systems that combine these ideas with provable security guarantees. The paper also flags the need for incentive structures in the Network Plane and the practical viability of cryptographic tools like SNARKs for outsourcing computation. The authors explicitly frame this as a position paper intended to organize community research efforts rather than propose a single solution.
