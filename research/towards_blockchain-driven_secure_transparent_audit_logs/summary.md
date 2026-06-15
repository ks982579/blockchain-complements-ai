---
id: towards_blockchain-driven_secure_transparent_audit_logs
title: "Towards Blockchain-Driven, Secure and Transparent Audit Logs"
authors:
  - Ashar Ahmad
  - Muhammad Saad
  - Mostafa Bassiouni
  - Aziz Mohaisen
year: 2018
venue: "Proceedings of the 15th EAI International Conference on Mobile and Ubiquitous Systems: Computing, Networking and Services (MobiQuitous '18)"
publisher: "Association for Computing Machinery"
pages: "443-448"
doi: "10.1145/3286978.3286985"
url: "https://doi.org/10.1145/3286978.3286985"
type: inproceedings
keywords:
  - Hyperledger
  - Distributed Systems
  - Blockchain
  - Audit Log
---

## Overview

This paper presents BlockAudit, a system that integrates conventional audit logs from enterprise online transaction processing (OLTP) systems with a permissioned blockchain (Hyperledger Fabric) to make those logs tamper-proof, replicated, and verifiable. The motivating problem is that conventional audit logs rely on a client-server database model and are therefore vulnerable to a single point of trust: an attacker who gains physical access (stolen credentials) or exploits a remote software vulnerability can corrupt both the underlying data and the audit trail that would otherwise expose the intrusion. BlockAudit addresses this by extending the Object-Relational Mapping (ORM) layer of an existing application (using NHibernate's insert/update/delete event listeners) to serialize audit log entries as JSON, broadcast them as blockchain transactions to a peer-to-peer network, and reach consensus on their ordering via Hyperledger's BFT-based protocol — all with minimal modification to the existing application stack. The authors implement and evaluate the design using a real eGovernment application provided by ClearVillage Inc., measuring latency as a function of network size (number of peers) and payload size (2–20MB), and argue the approach offers a practical, "plug and play" path for existing audit-log systems to gain blockchain-grade integrity, transparency, and fault tolerance.

## Background

The paper situates audit logs as a regulatory and operational necessity in enterprise/OLTP systems (e.g., order entry, retail, financial transactions). It notes that enterprise and corporate organizations maintain audit logs to enable continuous monitoring and transparent auditing of system events (citing Olivier and von Solms; Wee), and separately notes that federal laws and regulations, including the Code of Federal Regulations of the FDA and HIPAA, require organizations to maintain audit logs for data auditing, compliance, and insurance (citing Ringelstein and Staab). It draws on prior characterizations of audit log vulnerabilities, noting that conventional protections such as append-only WORM devices or continuous-feed printers rest on the weak assumption that the logging site itself cannot be compromised — an assumption attackers have repeatedly defeated (citing Lee, Zhang, and Xu's "LogGC" work and Margulies' "A Developer's Guide to Audit Logging"). On the blockchain side, the paper frames blockchains generally as enabling secure, transparent, immutable, intermediary-free record keeping, citing broader blockchain research on cryptocurrencies, healthcare, and IoT applications (Bonneau et al.; Guo et al.; Jesus et al.) and on consensus/scalability tradeoffs among PoW, PoS, and BFT protocols (citing Croman et al. "On Scaling Decentralized Blockchains" and Saad/Mohaisen's work on blockchain cryptocurrency characterization). For the specific design choice of Hyperledger Fabric, the paper relies on Androulaki et al.'s description of Hyperledger Fabric as a modular, permissioned distributed ledger supporting pluggable consensus and smart contracts in general-purpose languages. It also explicitly positions itself against prior secure-logging schemes — Schneier and Kelsey's forward-secure audit log cryptography, Snodgrass et al.'s trusted-notary tamper detection for RDBMS logs, and blockchain-based logging proposals by Sutton and Samavi, Castaldo and Cinque, and Cucurull and Puiggali — describing each as a point of comparison for BlockAudit's design.

## Key Points

- BlockAudit proposes a formal threat model for audit-log systems comprising two adversary classes: a "physical access attack" (attacker obtains legitimate user credentials and can directly corrupt both database and audit log) and a "remote vulnerability attack" (attacker exploits software bugs/malware to contaminate database and logs without full credential access).
- The paper's core technical contribution is a design that converts existing ORM-layer audit log generation (via NHibernate's IPostInsertEventListener, IPostUpdateEventListener, and IPostDeleteEventListener events) into blockchain transactions, requiring only minor modifications to existing enterprise applications rather than a system redesign.
- BlockAudit serializes audit log entries (session ID, entity name, event type, timestamps, user ID, URL, and old/new property value pairs) into JSON and submits them via a "createAudit" REST web service as blockchain transactions broadcast to a peer-to-peer network.
- The system uses Hyperledger Fabric's pluggable BFT-based consensus protocol, chosen specifically because audit-log applications favor low-latency consensus over the large-network scalability that PoW-style protocols provide.
- The architecture is described as "application agnostic" and "plug and play," decoupling the searchable/queryable audit log (kept in the relational database for fast queries) from the tamper-evident copy (replicated on the blockchain).
- The authors implemented BlockAudit using a real-world eGovernment application from ClearVillage Inc. and deployed it on Hyperledger Composer with a Node.js wrapper to ingest JSON transactions from the application.
- Empirical evaluation shows that consensus latency remains negligible across payload sizes (2–20MB) as long as the network has fewer than approximately 30 peers; beyond ~30 peers, latency increases considerably, and a sharp latency jump occurs specifically between 5MB and 10MB payloads, while 15–20MB shows little additional change.
- The paper argues qualitatively that BlockAudit defends against both threat-model attacks: in the physical-access case, an attacker's actions are still recorded immutably on the blockchain (since logging happens at the ORM layer, independent of direct database writes), enabling detection and recovery; in the remote-vulnerability case, the pre-attack blockchain state is preserved tamper-proof, allowing auditors to diff current data against the immutable log to identify the attack's effects.
- The paper contrasts BlockAudit with prior blockchain-based logging systems (Sutton and Samavi's Bitcoin-based integrity digests, Castaldo and Cinque's cross-border eHealth logging, Cucurull and Puiggali's immutable log integrity proofs), claiming its distinguishing advantage is localization of changes to the ORM layer, leaving other application layers untouched and easing adoption by existing systems.

## Conclusion

The paper concludes that BlockAudit successfully demonstrates a feasible transition path from conventional audit logs to a blockchain-backed, distributed, append-only, tamper-proof design, achieved by extending the ORM layer rather than re-architecting the application. The latency evaluation supports the claim that the system is practical at moderate network sizes (under ~30 peers) across a range of realistic payload sizes (2–20MB), though latency degrades noticeably as peer count grows beyond that threshold and when payload size crosses roughly the 5–10MB boundary — indicating a scalability limitation that the authors acknowledge implicitly through their choice of a low-latency but less scalable BFT consensus protocol. The defense arguments against the physical-access and remote-vulnerability attacks are presented largely qualitatively/conceptually rather than through adversarial experiments, so the security claims rest on the design's properties (replication, immutability, ORM-layer independence from direct DB writes) rather than empirical attack demonstrations. The authors explicitly flag future work: deploying BlockAudit in a production environment, identifying performance bottlenecks and optimizations, and using the latency/network-size/payload-size relationships established here to derive optimal block size and average block time for audit-log workloads specifically — leaving open questions about real-world scalability beyond the tested network sizes and about formal/adversarial security validation.
