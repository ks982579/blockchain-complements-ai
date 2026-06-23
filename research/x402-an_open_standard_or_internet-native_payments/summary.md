---
id: x402-an_open_standard_or_internet-native_payments
title: "x402: An Open Standard for Internet-Native Payments — An HTTP-based protocol for agents, context retrieval, APIs, and more"
authors:
  - Erik Reppel
  - Ronnie Caspers
  - Kevin Leffew
  - Danny Organ
  - Dan Kim
  - Nemil Dalal
year: 2025
venue: "Coinbase Developer Platform Whitepaper"
publisher: "Coinbase Developer Platform"
url: "https://www.x402.org/x402-whitepaper.pdf"
type: misc
keywords:
  - x402
  - HTTP 402
  - internet-native payments
  - agentic commerce
  - stablecoins
  - USDC
  - micropayments
  - machine-to-machine payments
  - onchain settlement
  - EIP-712
---

## Overview

This Coinbase Developer Platform whitepaper introduces x402, an open, HTTP-native payment standard designed to let AI agents and web services autonomously pay for API access, data, compute, and digital content without human intervention. The core problem it targets is that legacy payment rails (credit cards, ACH, PayPal) were built for humans and impose account signups, API keys, subscriptions, high per-transaction fees, slow settlement, and chargeback/fraud risk—making them incompatible with machine-to-machine (M2M) and agentic commerce. x402's central technical move is to repurpose the long-reserved HTTP 402 "Payment Required" status code: when a request arrives without valid payment, the server returns a 402 with structured pricing/payment metadata, and the client retries with a cryptographically signed payment authorization (EIP-712), settled onchain in stablecoins such as USDC. The paper claims this enables near-instant (~200ms on Base), near-zero-fee (<$0.0001), chain- and token-agnostic micropayments integrable with a single line of middleware code. It positions x402 as foundational infrastructure for autonomous, pay-per-use digital economies and provides spec details, integration examples, and use cases rather than empirical evaluation.

## Background

The paper builds on several external claims and technologies. It cites figures that approximately 1.4 billion people remain unbanked and excluded from traditional financial rails (attributed to an external source). It references Base.org (2025) for a broader discussion of how onchain payments are transforming digital commerce. It relies on established blockchain and crypto infrastructure as prerequisites: stablecoins (primarily USDC, pegged to USD), Layer-2 rollups (Base, plus optimistic and ZK rollups) for low-cost high-throughput settlement, ERC-20 token standards, and the EIP-712 standard for structured, human-readable cryptographic message signing in wallets. It situates x402 against prior attempts at agentic payments—specifically "Browser Usage APIs"—arguing those remain tethered to human-oriented systems (credit cards, account verification, manual UX). The paper also notes interoperability with the Model Context Payment (MCP) protocol as an existing framework for monetizing contextual data, and mentions combining x402 with decentralized authorization solutions for granular access control (e.g., single-use URLs, IP-restricted streams). Comparative performance/cost figures for competing rails (credit card $0.30+2.9%, settlement in days, chargebacks up to 120d; Ethereum L1 $1-$5 + gas, 15-20 TPS) are presented as context for x402's claimed advantages.

## Key Points

- x402 is an open payment protocol that repurposes the HTTP 402 "Payment Required" status code so that AI agents and web services can pay for resources autonomously, eliminating the need for API keys, accounts, and subscriptions.
- The core payment flow is four steps: client requests a resource, server returns HTTP 402 with pricing/payment details, the agent retries with a signed payment authorization, and the server verifies, broadcasts, and fulfills the request.
- Payments use stablecoins (primarily USDC) settled onchain, with the paper claiming ~200ms settlement finality on Base, fees below $0.0001, no chargebacks, and throughput of hundreds to thousands of TPS.
- Integration is claimed to require as little as one line of middleware (e.g., paymentMiddleware(amount, address)), with reference implementations for Express.js, Next.js, and both browser and Node.js client libraries.
- Payment authorizations are cryptographically signed using the EIP-712 standard, with payment requests carrying fields such as maxAmountRequired, asset/assetAddress, network, expiresAt, nonce (replay protection), and paymentId.
- x402 enables pragmatic micropayments as low as ~$0.001 per request, making true pay-per-use pricing (per request, per second, per inference) economically viable where legacy ~$0.30 fees made it impractical.
- The protocol is designed to be chain- and token-agnostic, supporting multiple stablecoins, digital assets, and blockchain networks beyond the initial USDC/Base implementation.
- x402 reduces operational burden by eliminating chargebacks and disputes through instant onchain settlement and removing PCI compliance requirements for developers (unless a facilitator accepts card payments).
- Settlement is flexible, supporting direct onchain transactions, Layer-2 (optimistic/ZK rollup) settlement, payment channels for high-frequency micropayments between trusted parties, and batched settlement.
- x402 supports both autonomous agent use cases (API access, model inference monetization, paying for cloud compute/storage, context retrieval) and human use cases (per-article news, per-episode podcasts, per-play games, per-whitepaper downloads), and interoperates with the Model Context Payment (MCP) protocol.
- A full open-source reference implementation is provided, including protocol libraries, server middleware, client libraries, cryptographic signing/verification utilities, transaction-broadcasting services, and a local test environment with mock servers and pre-funded test wallets.

## Conclusion

The paper concludes that AI-driven systems demand payment infrastructure as autonomous and frictionless as the agents using it, and that x402 supplies this by standardizing pay-per-use payments at the HTTP protocol level—delivering instant low-cost transactions, no API keys/subscriptions/middlemen, and blockchain-agnostic flexibility. As a whitepaper, it is advocacy and specification rather than empirical study: its claims (e.g., 200ms settlement, sub-$0.0001 fees, hundreds-to-thousands of TPS) are presented as design properties and benchmark comparisons rather than tested results, and no independent evaluation, adoption data, or security analysis is offered. The paper does not foreground limitations explicitly, but several open questions are implicit: dependence on stablecoin stability and specific L2 infrastructure (Base), the trust assumptions of payment channels, the maturity and security of the EIP-712 signing flow for autonomous agents, regulatory/compliance treatment of agent-initiated crypto payments, and how access control and authorization integrate at scale. Future-research angles include real-world adoption measurement, interoperability across chains and with frameworks like MCP, agent-side key/wallet management and safety, and resistance to fraud or replay attacks beyond the nonce mechanism described.
