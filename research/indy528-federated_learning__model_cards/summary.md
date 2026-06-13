---
id: indy528-federated_learning__model_cards
title: "Indy528 — Federated Learning Model Tokenization with Non-Fungible Tokens(NFT) and Model Cards"
authors:
  - Eranga Bandara
  - Xueping Liang
  - Sachin Shetty
  - Ravi Mukkamala
  - Abdul Rahman
  - Ng Wee Keong
year: 2022
venue: "2022 IEEE 19th International Conference on Mobile Ad Hoc and Smart Systems (MASS)"
pages: "195-201"
doi: "10.1109/MASS56207.2022.00033"
url: "https://api.semanticscholar.org/CorpusID:254917805"
type: inproceedings
keywords:
  - NFT
  - Blockchain
  - Machine Learning
  - Federated Learning
  - Model Card
---

## Overview

This paper presents Indy528, a blockchain-based platform that represents federated machine learning (FML) models as Non-Fungible Tokens (NFTs) using a novel token schema called i528, an extension of the ERC721 standard. The work addresses three persistent problems in federated learning: reliance on a centralized coordinator, vulnerability to attacks and privacy breaches, and poor model transparency/provenance. Indy528 combines a coordinator-less FML architecture with Model Cards (structured records of data provenance, training participants, aggregation history, and quantitative evaluation) and encodes model ownership, off-chain storage locations, and Model Card data into i528 NFTs stored on the Rahasak blockchain ledger. As a demonstration use case, the authors build an NFT-based decentralized marketplace where model creators can publish trained FML models as tradeable NFTs and buyers can inspect a model's provenance and performance via its attached Model Card before purchase. The platform is implemented and evaluated using a five-peer federated GRU-based spam-detection model, with measurements of local/global model accuracy, training/validation loss, and block generation time.

## Background

- The paper situates NFTs within prior characterizations of non-fungibility and uniqueness (citing Wang et al.'s NFT overview, arXiv:2105.07447), and notes prior blockchain-based tokenization work such as tokenized index funds (Ciriello, 2021).
- Federated learning is framed using foundational FL literature: Konečný et al. (2016) on communication-efficient FL strategies, and Yang et al. (2018) on Google's applied FL for keyboard query suggestions, which the paper uses to define the standard FL workflow (local training, parameter sharing, global aggregation).
- The paper cites known weaknesses of typical FL systems from other sources: vulnerability to attacks/privacy breaches (Kuo and Ohno-Machado's ModelChain work on decentralized privacy-preserving healthcare modeling) and insufficient transparency (Strobel's work on transparency in machine learning).
- The coordinator-less, blockchain-enabled FML approach builds on Qu et al. (2020), who proposed decentralized privacy-preserving federated learning in fog computing using blockchain.
- Model Cards are adopted from Google's TensorFlow Model Card Toolkit (Wadhwani and Jain, 2020), which the paper uses as the standardized framework for reporting model provenance, usage, ethics considerations, and limitations.
- ERC721 is described per Chirtoaca et al.'s framework for deployable NFT smart contracts on Ethereum, and ERC20 token standards are referenced via Victor and Lüders' measurement study.
- The security model explicitly aligns with goals from Miao et al. (2015) on secure cloud deduplication incentive mechanisms, and the blockchain trust model is grounded in Völter et al.'s (2021) evaluation of trust signals in blockchain applications.
- The underlying infrastructure—Rahasak blockchain, its Aplos smart contract platform, and the Tikiri lightweight IoT blockchain—comes from the same author group's earlier publications (Bandara et al., 2021), as does the self-sovereign identity platform used for peer identity registration.
- The global model is created using a model averaging function (Nguyen et al., 2021, on FL meeting blockchain in edge computing) implemented with the Stochastic Gradient Descent (SGD) approach; Kamp et al. (2018) on dynamic model averaging is cited separately as an example of model data converted into Model Card objects.
- The GRU neural network architecture used in the evaluation use case is drawn from Dey and Salem's (2017) work on gated RNN variants.
- The related work section surveys six other NFT application domains for comparison: energy trading (NFT-Energy, Karandikar et al.), smart parking on real estate (Parkchain, Jennath et al.), museum fundraising (NFT-OpenGLAM, Valeonti et al.), virtual land pricing (Fertile LAND, Dowling), gaming assets (Tokenfication, Fowler and Pirker), GAN-generated digital art (NFTGAN, Shahriar and Hayawi), and patents/IP (Bamakan et al.).

## Key Points

- Indy528 is presented as the first system to represent machine learning models—specifically federated learning models—as NFTs.
- The paper proposes a blockchain-enabled, coordinator-less FML system in which each peer runs its own blockchain node and off-chain storage, removing the need for a centralized aggregator.
- The platform defines i528, a novel NFT token schema extending ERC721, encoding model name, description, model URI, owner, and embedded Model Card objects as JSON.
- Model Cards are generated for both local and global models and stored as transactions on the blockchain ledger, capturing participating clients, local model generators, aggregator peers, model generation times, model URIs, and quantitative evaluation metrics.
- The architecture is organized into five layers: blockchain ledger, smart contract, FML, Model Card service, and NFT service layers, with four key smart contracts (Identity, FML, Model Card, NFT) implementing the system's functionality.
- A leader peer (selected via consensus) coordinates federated pipeline creation, initial model parameter distribution, and global model aggregation via SGD-based model averaging, then issues the i528 NFT for the resulting global model.
- The paper proposes an NFT-based decentralized marketplace where model creators/trainers publish FML models as NFTs, buyers purchase them with fiat currency, and revenue is shared among contributors based on ownership recorded in the NFT.
- Buyers can inspect a model's information, accuracy, training process, and quantitative evaluation through the Model Card attached to its NFT before purchase, supporting a transparent and auditable trading system.
- The system was implemented using the Rahasak blockchain with its Aplos smart contract platform (Scala), PyTorch/PySyft for FML, TensorFlow Model Card Toolkit for Model Cards, Apache Cassandra for off-chain storage, Apache Kafka for peer-to-peer communication, and Docker/Kubernetes for deployment.
- The platform was evaluated with a five-peer federated GRU-based spam message detection use case, measuring local model accuracy (ROC AUC), training/validation loss over iterations, federated model accuracy across peers, and block generation time as a function of transaction count.
- A comparative table positions Indy528 against six other NFT platforms (NFT-Energy, NFT-OpenGLAM, Fertile LAND, Tokenfication, NFTGAN, Parkchain) across dimensions including application domain, blockchain, smart contract language, token format, storage, trading support, and ERC721 compliance—Indy528 is the only one targeting machine learning models with a custom i528 token format on Cassandra storage with trading support.

## Conclusion

The paper concludes that Indy528 successfully demonstrates representing FML models—including their ownership, off-chain storage locations, and Model Card-based provenance information—as i528 NFTs on the Rahasak blockchain, and that this provides enhanced transparency and auditability for federated learning processes while enabling a functional model-sharing/trading marketplace. The evaluation results show that local model accuracy increases (and training/validation loss decreases) as training iterations increase, that federated model accuracy improves across iterations when aggregating peer models, and that block generation time increases with the number of transactions per block due to leader election, replication/broadcast, Merkle proof generation, and transaction validation overhead—establishing baseline performance characteristics for the system. The paper does not report any failed hypotheses, but it is primarily a system/architecture proposal with a single proof-of-concept use case (spam detection via GRU across five peers), so claims about generalizability to other ML model types, larger peer networks, or adversarial conditions remain untested. The authors explicitly identify integrating additional NFT token schemes into the Indy528 platform as future work, leaving open questions about interoperability with other NFT standards, scalability of the marketplace and blockchain ledger under larger numbers of peers/models, and more rigorous security/privacy evaluation beyond the stated alignment with prior trust and security models.
