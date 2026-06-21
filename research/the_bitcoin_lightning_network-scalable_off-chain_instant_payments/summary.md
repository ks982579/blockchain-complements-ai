---
id: the_bitcoin_lightning_network-scalable_off-chain_instant_payments
title: "The Bitcoin Lightning Network: Scalable Off-Chain Instant Payments"
authors:
  - Joseph Poon
  - Thaddeus Dryja
year: 2016
url: "https://lightning.network/lightning-network-paper.pdf"
type: misc/preprint
keywords:
  - Bitcoin
  - Lightning Network
  - payment channels
  - scalability
  - micropayments
---

## Overview

This paper by Joseph Poon and Thaddeus Dryja proposes the Bitcoin Lightning Network, a second-layer protocol that addresses Bitcoin's fundamental scalability limitation -- its inability to process more than approximately 7 transactions per second on-chain -- by constructing a network of off-chain bidirectional micropayment channels. The core contribution is a trustless mechanism using Hashed Timelock Contracts (HTLCs) and Revocable Sequence Maturity Contracts (RSMCs) that enables parties to transact without broadcasting every transaction to the blockchain, while retaining the ability to enforce correct balances on-chain if a counterparty acts maliciously. The paper demonstrates that by moving transactions off-chain, Bitcoin could theoretically support billions of users making unlimited transactions, requiring only 133 MB blocks (for channel open/close operations) rather than the 24 GB blocks that would be needed if all transactions occurred on-chain. The design eliminates custodial counterparty risk, as funds remain controlled by cryptographic contracts enforceable through the Bitcoin scripting system, and additionally enables use cases such as near-instant payments, true micropayments, cross-chain payments, and financial smart contracts.

## Background

The paper builds on Bitcoin's existing scripting and transaction system as defined by Satoshi Nakamoto's original Bitcoin whitepaper [1], and situates its scalability argument against Visa's reported peak throughput of 47,000 transactions per second [2]. It draws on prior work in micropayment channels as documented on the Bitcoin Wiki [3] and in bitcoinj [4], as well as Leslie Lamport's foundational work on distributed consensus ("The Part-Time Parliament" [5]) and event ordering in distributed systems ("Time, Clocks, and the Ordering of Events" [6]) to justify using time-based commitments as a component of decentralized consensus. The paper references earlier hub-and-spoke micropayment channel proposals by Alex Akselrod [7][8] and Peter Todd [9], as well as trusted payment channel networks by Plooy (combining Bitcoin and Ripple concepts) [10] and BitPay's Impulse [11]. Key Bitcoin Improvement Proposals (BIPs) are cited as prerequisites: BIP 0068 by Mark Friedenbach for consensus-enforced relative lock-time via sequence numbers [12], BIP 0112 for OP_CHECKSEQUENCEVERIFY [13][14], and BIP 0032 for Hierarchical Deterministic Wallets used in efficient key storage [17]. Greg Maxwell's timestop proposal [15] is cited as a mitigation for flood attacks. The paper also references Nick Szabo's work on formalizing smart contracts [19] and Gerhardt and Hanke's pay-to-contract protocol [18] for cryptographic proof of payment. Critically, the paper notes that a new sighash type (SIGHASH_NOINPUT) is required via soft-fork to resolve transaction malleability, which is a prerequisite for the entire construction to work securely, building on the precedent of earlier soft-forks like P2SH (BIP 0016) [16].

## Key Points

- Bitcoin's on-chain capacity (less than 7 tps with 1 MB blocks) is fundamentally insufficient for global-scale payments; achieving Visa-equivalent throughput on-chain would require approximately 8 GB blocks every 10 minutes, which would centralize the network and undermine its security model.
- Bidirectional micropayment channels can be constructed using 2-of-2 multisignature Funding Transactions, where both parties hold off-chain Commitment Transactions that represent the current balance and can be broadcast to the blockchain at any time for enforcement.
- The Revocable Sequence Maturity Contract (RSMC) enables old channel states to be invalidated: if a party broadcasts a revoked Commitment Transaction, the counterparty can claim all funds in the channel as a penalty via a Breach Remedy Transaction, provided they act within a predefined confirmation window.
- Each party holds a distinct version of the Commitment Transaction (e.g., C1a for Alice, C1b for Bob), enabling blame attribution -- the blockchain can determine which party broadcast a particular transaction, making penalty enforcement possible.
- Hashed Timelock Contracts (HTLCs) enable conditional payments across a network of channels: payment is contingent on the recipient disclosing a preimage R to a known hash H within a specified timeframe, and funds are refunded to the sender if the timeframe expires without disclosure.
- Multi-hop payments across the network use decrementing timelocks: each successive hop in the payment path has a shorter HTLC expiry, ensuring that each intermediary can claim funds from the prior hop before their own obligation expires to the next hop.
- The SIGHASH_NOINPUT sighash type (requiring a Bitcoin soft-fork) is essential to the construction, as it allows spending from transactions whose signatures have not yet been exchanged, resolving the transaction malleability problem that would otherwise invalidate chained contract states.
- A timestop mechanism is proposed whereby miners flag blocks as "congested," pausing the relative timelock countdown (nSequence) during periods of blockchain spam, preventing attackers from exploiting full blocks to force timelock expirations.
- Key storage efficiency is achieved through BIP 0032 HD Wallet trees: millions of prior channel states can be invalidated by disclosing a single parent private key in the merkle tree, requiring only kilobytes of storage per channel regardless of transaction volume.
- Lightning Network fees are distinct from blockchain fees, derived primarily from the time-value of locking up funds along a route and counterparty risk of non-communication, and are expected to be significantly lower than on-chain fees, potentially approaching negligibility.
- Cooperative channel closure bypasses the RSMC timelock entirely: both parties sign a single Exercise Settlement Transaction spending directly from the Funding Transaction, allowing immediate fund recovery without waiting for confirmation maturity.
- Cross-chain payments are possible if different blockchains share similar hash functions, enabling payments to be routed across chains where neither the sender nor receiver needs to understand the other chain's consensus rules.
- Payment routing in the network can be constructed similarly to BGP or correspondent banking networks, with well-connected core nodes maintaining full routing tables and edge nodes (end-users) connecting intermittently through partial routes.

## Conclusion

The paper concludes that a network of micropayment channels using HTLCs and RSMCs can enable Bitcoin to scale to billions of users with near-instant, near-zero-fee transactions while preserving decentralization and eliminating custodial counterparty risk. Its scalability estimates show that with 7 billion people each opening two channels per year, only 133 MB blocks would be needed -- manageable on current consumer hardware with storage pruning. However, the authors identify several significant risks and open issues: forced expiration spam (mass simultaneous channel closures overwhelming block space) is cited as the greatest systemic risk; timelocks must be carefully calibrated to avoid enabling coin theft; online key storage for intermediary nodes creates theft risk via system compromise; data loss by one party can enable counterparty theft; and failure to broadcast time-sensitive transactions can result in fund loss. The entire system depends on Bitcoin soft-forks that had not yet been implemented at time of writing -- particularly SIGHASH_NOINPUT for transaction malleability resolution and OP_CHECKSEQUENCEVERIFY for relative timelocks. The paper acknowledges that while Lightning Network may reduce block size pressure in the short term, achieving true global scale will still eventually require block size increases. The paper is explicitly marked as a draft (Version 0.5.9.2), and the authors note that future versions will include stop-gap constructions by Rusty Russell using OP_CHECKSEQUENCEVERIFY or OP_CHECKLOCKTIMEVERIFY as alternatives to the full malleability fix.
