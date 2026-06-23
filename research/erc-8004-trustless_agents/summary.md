---
id: erc-8004-trustless_agents
title: "ERC-8004: Trustless Agents [DRAFT]"
authors:
  - Marco De Rossi
  - Davide Crapis
  - Jordan Ellis
  - Erik Reppel
year: 2025
venue: "Ethereum Improvement Proposals"
issue: 8004
url: "https://eips.ethereum.org/EIPS/eip-8004"
type: misc
keywords:
  - Ethereum
  - trustless agents
  - agent discovery
  - reputation
  - validation
  - smart contracts
---

## Overview

ERC-8004 "Trustless Agents" is a draft Ethereum Improvement Proposal (created 2025-08-13; authored by Marco De Rossi, Davide Crapis, Jordan Ellis, and Erik Reppel) that specifies an on-chain framework for discovering, choosing, and interacting with autonomous AI agents across organizational boundaries without pre-existing trust, with the goal of enabling open-ended, cross-organizational agent economies. The standard addresses a gap left by agent communication protocols (MCP and Google's A2A), which handle capability advertisement, authentication, and task orchestration but do not inherently cover agent discovery or trust establishment in untrusted settings. ERC-8004's scope is three lightweight, composable registries deployable as per-chain singletons on any L2 or Mainnet: an Identity Registry (an ERC-721-based portable agent handle), a Reputation Registry (a standard interface for posting and aggregating client feedback), and a Validation Registry (generic hooks for independent verification of agent work). A central design principle is that trust is pluggable and tiered, with security proportional to value at risk, supporting four trust models: reputation via client feedback, crypto-economic validation via stake-secured re-execution, zero-knowledge machine learning (zkML) proofs, and trusted execution environment (TEE) oracle attestation. Payments are explicitly out of scope, though the spec shows how x402 proof-of-payment can enrich feedback signals. The document requires EIP-155, EIP-712, EIP-721, and EIP-1271, and is released under CC0.

## Background

The proposal situates itself relative to existing agent communication protocols that it does not replace but complements. It notes (citing those protocols) that the Model Context Protocol (MCP) lets servers list and offer capabilities such as prompts, resources, tools, and completions, while Google's Agent2Agent (A2A) protocol handles agent authentication, skills advertisement via AgentCards, direct messaging, and full task-lifecycle orchestration. The paper's core motivating claim is that neither of these protocols inherently covers agent discovery or trust across organizational boundaries, which is the gap ERC-8004 fills. The standard builds directly on established Ethereum primitives: ERC-721 (with the URIStorage extension) for the agent identity/NFT handle, EIP-712 and ERC-1271 for signature-based wallet control proofs (EOAs and smart-contract wallets respectively), and EIP-155 for chain identification. It also references EIP-7702 as an enabler of gas-sponsored, frictionless client feedback (since clients no longer need to be registered), and incorporates external nomenclature and ecosystem standards by reference, including A2A tasks/skills/contextId/taskId, MCP tools/prompts/resources, OASF skills and domains, ENS names, DIDs, IPFS for content-addressed off-chain data, subgraphs for indexing, and the x402 payment scheme for proof-of-payment. RFC 2119/8174 keyword conventions govern the normative language.

## Key Points

- Defines three composable on-chain registries (Identity, Reputation, Validation) that can be deployed as per-chain singletons on any L2 or Mainnet, separating agent discovery from trust establishment.
- The Identity Registry is built on ERC-721 + URIStorage, making agents browsable and transferable in NFT-compliant apps; each agent is globally identified by an `agentRegistry` string (`{namespace}:{chainId}:{identityRegistry}`, e.g. `eip155:1:0x742...`) plus an incrementally assigned `agentId` (the ERC-721 tokenId, with tokenURI referred to as `agentURI`).
- Specifies a mandatory JSON agent registration file (resolved via `agentURI`, which MAY use ipfs://, https://, or base64 data: URIs) containing type, name, description, image, a customizable `services`/endpoints list (web, A2A, MCP, OASF, ENS, DID, email), `x402Support`, `active`, `registrations`, and an OPTIONAL `supportedTrust` field listing trust models (reputation, crypto-economic, tee-attestation); if `supportedTrust` is absent/empty the ERC is used for discovery only.
- Provides registration functions (`register()` overloads), `setAgentURI()`, optional on-chain metadata via `getMetadata`/`setMetadata`, and a reserved `agentWallet` key that can only be changed by proving control of the new wallet through an EIP-712 (EOA) or ERC-1271 (smart contract wallet) signature, with `agentWallet` automatically cleared on agent transfer and requiring re-verification.
- Defines optional endpoint domain verification via a `/.well-known/agent-registration.json` file containing a `registrations` list that must match the on-chain agentRegistry/agentId.
- The Reputation Registry lets any non-owner `clientAddress` post feedback via `giveFeedback()`, consisting of a signed fixed-point `value` (int128) with `valueDecimals` (0-18) plus optional `tag1`/`tag2`, endpoint, off-chain `feedbackURI`, and KECCAK-256 `feedbackHash` for integrity; the agent owner/operator MUST NOT submit feedback on their own agent.
- Supports feedback revocation (`revokeFeedback()`), third-party responses (`appendResponse()`, e.g. for refunds or spam tagging), and on-chain read/aggregation functions (`getSummary`, `readFeedback`, `readAllFeedback`, `getClients`, etc.); `getSummary` REQUIRES a non-empty `clientAddresses` filter as a Sybil/spam mitigation, with sophisticated aggregation expected off-chain.
- Provides a flexible feedback semantics table (e.g. `starred`, `uptime`, `successRate`, `responseTime`, `revenues`, `tradingYield`) and an optional off-chain feedback JSON schema that can carry MCP/A2A/OASF identifiers and x402 `proofOfPayment` (fromAddress, toAddress, chainId, txHash).
- The Validation Registry lets an agent owner/operator request independent verification via `validationRequest(validatorAddress, agentId, requestURI, requestHash)` and lets the designated validator answer via `validationResponse()` with a `response` score 0-100 (usable as binary pass/fail or a spectrum); responses can be issued multiple times per request to support progressive states (e.g. soft vs. hard finality via `tag`).
- Validator contracts are generic hooks intended to implement crypto-economic stake-secured re-execution, zkML verifiers, or TEE oracles; validator incentives and slashing are explicitly out of scope of the registry.
- Establishes a tiered trust philosophy where security is proportional to value at risk, ranging from low-stake tasks (ordering pizza) to high-stake tasks (medical diagnosis), with developers selecting among the four pluggable trust models.

## Conclusion

As a draft standard rather than an empirical study, the document's "results" are its rationale and security analysis rather than experimental findings. The Rationale argues that linking the blockchain to a flexible registration file (rather than hard-coding a single agent protocol) future-proofs the design by allowing arbitrary endpoints that combine AI primitives (MCP, A2A) with Web3 primitives (wallets, DIDs, ENS); that reusing A2A/MCP nomenclature plus a flexible feedback schema balances interoperability with extensibility; that EIP-7702 enables frictionless, gas-sponsored feedback since clients need not register; that on-chain feedback plus IPFS data makes subgraph indexing straightforward; and that per-chain singleton deployment still allows agents to operate, transact, and register across multiple chains. The Security Considerations acknowledge key open weaknesses and tradeoffs: Sybil attacks can inflate the reputation of fake agents, which the protocol mitigates only by making signals public under a common schema and enabling filtering by reviewer (anticipating that third-party reputation-of-reviewers systems, auditor networks, and insurance pools will emerge off-chain to address it). The authors note that on-chain pointers and hashes are immutable, preserving an auditable trail, and explicitly concede a fundamental limitation: while the ERC cryptographically guarantees that a registration file corresponds to its on-chain agent, it cannot cryptographically guarantee that advertised capabilities are functional and non-malicious; the trust models are the proposed (but unproven) answer to that verification gap. Open considerations left to external protocols or future work include validator incentive/slashing design, robust off-chain reputation aggregation, defenses against feedback manipulation, and integration of payments (x402), all of which are outside the registry's normative scope.
