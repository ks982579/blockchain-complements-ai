www.nature.com/scientificreports 

## **OPEN Blockchain-enhanced incentivecompatible mechanisms for multiagent reinforcement learning systems** 

## **Ke Tian** 

**Ensuring trust, fairness, and long-term efficiency in multi-agent systems poses significant challenges, particularly under partially competitive and decentralized settings where strategic manipulation and collusion can arise. This paper proposes a blockchain-enhanced framework that integrates smart contracts with multi-agent reinforcement learning (MARL) to design incentive-compatible mechanisms for strategic agent coordination. The framework utilizes the decentralized and tamper-resistant nature of blockchain to record agent behaviors on-chain, enforce transparency, and implement automated penalty and reward mechanisms through smart contracts. We embed these mechanisms into a Multi-Agent Soft Actor-Critic (MASAC) algorithm, aligning local decision-making with global system objectives. Experimental validation in two representative domains—automated market bidding and intelligent traffic control—demonstrates that the proposed approach significantly improves social welfare, reduces collusion success rates, enhances fairness, and increases behavioral robustness under noise. Ablation studies further reveal the complementary contributions of each system component. This work lays the foundation for scalable, transparent, and incentive-aligned coordination in decentralized intelligent agent systems.** 

**Keywords** Blockchain, Multi-agent reinforcement learning, Incentive-compatible mechanism, Smart contracts, Decentralized coordination 

In recent years, Distributed Artificial Intelligence (Distributed AI) and Multi-Agent Reinforcement Learning (MARL) have gained significant traction in a wide variety of applications, often involving complex environments where multiple intelligent agents must make decisions concurrently, either in competition or cooperation[1][,][2] . Typical examples include automated market bidding, intelligent traffic signal control, and drone formation tasks; however, as the number of agents and the complexity of interactions increase, problems such as “cheating” or “collusion” become more prevalent, degrading efficiency, triggering trust issues, and even destabilizing the entire system[3][–][5] . An emerging technology that holds promise for mitigating these concerns is the blockchain, whose decentralized, tamper-resistant, and auditable features present a new avenue for establishing trust among agents without relying on a central authority[6][,][7] . By recording transactions (or agent actions) in an immutable ledger and automatically enforcing rules through smart contracts, blockchain-based systems provide transparent and verifiable mechanisms for interactions among agents, allowing on-chain implementation of incentive structures and mechanism design to ensure that malicious strategies yield lower long-term utility compared to truthful participation[8][,][9] . 

Motivated by these advantages, this paper proposes a blockchain-supported incentive-compatible mechanism for multi-agent game environments, integrating mechanism design principles with MARL and leveraging blockchain’s inherent properties to create an environment in which each self-interested agent, while maximizing its own rewards, is disincentivized from dishonest or collusive strategies[10][,][11] . We present a novel blockchain-based architecture that records agents’ actions in a decentralized ledger, employs smart contracts to ensure transparent decision-making and reward allocation, and formalizes an incentive mechanism wherein immutable records and consensus protocols eliminate tampering or deception, thereby consistently favoring honest behavior over manipulative alternatives. Furthermore, we embed mechanism design concepts into multiagent reinforcement learning algorithms, enabling agents to learn optimal policies under both competitive and cooperative conditions while harnessing the blockchain’s transparent reward system to promote compliance 

Department of Electrical and Computer Engineering, University of Illinois at Urbana-Champaign, Champaign, IL, USA. email: ketian2@illinois.edu 

**Scientific Reports** |        (2025) 15:42841 

1 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

and enhance overall social welfare. To validate the proposed approach, we conduct extensive simulations in two representative scenarios—automated market bidding and intelligent traffic signal control—and demonstrate that our method improves fairness, boosts system-wide efficiency, and maintains robustness against malicious or collusive attempts, ultimately illustrating how rational agents can be guided to act in ways that benefit both themselves and the entire system. 

## **Related work** 

## **Multi-agent reinforcement learning** 

Multi-Agent Reinforcement Learning (MARL) has shown significant promise in addressing complex tasks that involve multiple interacting agents, each with individual objectives and varying degrees of cooperation or competition. However, applying traditional single-agent reinforcement learning methods directly to multiagent settings often proves inadequate due to a range of distinctive challenges, including non-stationary environments, intertwined policy updates, and limited observability of other agents’ states and actions[12][,][13] . As a result, researchers have proposed specialized MARL frameworks and algorithms to ensure more stable learning, efficient coordination, and robust performance against adversarial strategies. One of the most straightforward yet foundational approaches involves treating each agent as a single-agent learner, independently applying algorithms such as Q-Learning or Policy Gradient without explicit consideration of other agents’ policies[14] . Although conceptually simple, this “Independent Learning” strategy suffers when the environment becomes highly non-stationary, since each agent’s behavior changes over time, thus leading to convergence instability and limiting overall performance in domains that require tight coordination or adaptive responses to opponents. To address these limitations, a common strategy is to adopt a “Centralized Training and Decentralized Execution (CTDE)” paradigm[15] , in which algorithms have access to global information during training (e.g., all agents’ observations, actions, and rewards) but only employ localized observations at execution time. Classic examples include Multi-Agent Deep Deterministic Policy Gradient (MADDPG) and Counterfactual Multi-Agent Policy Gradients (COMA), wherein a centralized critic evaluates joint actions while each agent’s decentralized actor uses local information in real-time decisions[16] . By separating training from execution, CTDE methods effectively balance coordination efficiency and scalability in mixed cooperative-competitive settings. When agents engage in adversarial or partially cooperative scenarios, researchers further integrate game-theoretic constructs—such as Nash Equilibria, correlated equilibria, or cooperative game formulations—into MARL, thus enhancing learning outcomes by addressing strategic uncertainties[17] . In zero-sum games, for instance, MARL methods leverage iterative techniques to approximate Nash policies, enabling agents to adapt optimally to competitive strategies; in cooperative or general-sum settings, additional concepts like Shapley values and reward shaping encourage stable and fair joint solutions. Overall, modern MARL research highlights coordination, stability, and scalability in non-stationary multi-agent environments, with each class of methods—ranging from Independent Learning to CTDE and game-theoretic approaches—offering a different balance of simplicity, performance, and theoretical guarantees, ultimately forming the basis for applications such as robotics swarms, autonomous vehicles, and other domains where strategic interactions strongly influence system-level outcomes. 

## **Blockchain and smart contracts** 

Blockchain technology, first popularized by Nakamoto (2008), offers decentralized governance, tamperresistance, and traceability, providing a novel foundation of trust for multiple parties without the need for a central authority. By storing data in an immutable ledger and facilitating programmatic control via smart contracts[18] , it enables autonomous enforcement of predefined rules while minimizing human intervention. These features naturally complement multi-agent systems, where distributed entities must coordinate, exchange information, and reach consensus under potential adversarial conditions. Indeed, initial explorations in areas such as supply chain finance and distributed energy trading highlight the capacity of blockchain to enhance transparency and efficiency in multi-agent collaborations[19][,][20] . Nevertheless, most existing efforts focus primarily on data storage, simplistic token-based incentives, or basic consensus mechanisms, offering limited insight into the complexities of multi-agent strategic interactions and long-term cooperative or competitive behaviors[21] . Consequently, there remains a pressing need to investigate advanced mechanism design that exploits blockchain’s immutable records and executable smart contracts for incentive alignment and conflict resolution in intricate multi-agent settings. Addressing this gap requires more sophisticated modeling at the intersection of decentralized ledger technology, game theory, and reinforcement learning, thereby ensuring that blockchain-based multi-agent systems can transition beyond rudimentary data-sharing applications and tackle complex, dynamic, and potentially largescale environments in a secure and trustless manner. 

## **Mechanism design and incentive compatibility** 

Mechanism design is a branch of game theory that focuses on constructing rules and incentive structures to align individual agents’ self-interested behaviors with system-wide objectives[22] . By specifying how actions map to outcomes and how rewards (or costs) are distributed, mechanism design aims to achieve socially desirable equilibria, typically maximizing collective welfare or enforcing fairness while still allowing each participant to pursue its own utility. Central to this paradigm is incentive compatibility, the requirement that rational agents have no motivation to misreport private information or collude in ways that could distort the mechanism’s intended outcome[23] . If a mechanism is incentive-compatible, then truth-telling or honest participation emerges as the optimal strategy for each agent, minimizing manipulative practices that might degrade system performance. The integration of mechanism design principles with blockchain smart contracts significantly augments the reliability of such frameworks, as the immutable and decentralized nature of the ledger ensures transparent enforcement of rules while reducing the risk of tampering or collusion[24] . In essence, smart contracts can automatically execute the prescribed allocation and payment schemes, thereby removing trusted third-party 

**Scientific Reports** |        (2025) 15:42841 

2 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

dependencies and diminishing operational overhead[25][–][27] . Nevertheless, implementing advanced mechanism design in a blockchain setting demands careful attention to scalability, consensus latency, and potential strategic attacks on the network layer. These issues underscore the need for robust protocols and incentive-compatible frameworks that can thrive in distributed, adversarial, and large-scale environments—areas where multi-agent learning and mechanism design intersect to create opportunities for novel, automated solutions. 

## **Problem definition and system architecture** 

## **Multi-agent game environment definition** 

In this work, we consider a multi-agent game environment with _N_ autonomous agents, each denoted by _i ∈ I_ = _{_ 1 _,_ 2 _, . . . , N }_ . The environment’s global state space is represented by _S_ , and each state _s ∈ S_ captures the system’s configuration at a given time. Every agent _i ∈ I_ has an associated action set _Ai_ , and a joint action is defined as _a_ = ( _a_ 1 _, a_ 2 _, . . . , aN_ ) _∈ A_ = _A_ 1 _× A_ 2 _× · · · × AN_ , where _ai ∈ Ai_ for _i ∈ I_ . The combined action space is thus _A_ = _A_ 1 _× A_ 2 _× · · · × AN_ . Upon execution of a joint action _a ∈ A_ , the environment transitions to a new state _s[′] ∈ S_ and provides each agent _i ∈ I_ with an immediate reward _ri_ , which typically encompasses both individual benefits and collective gains. Due to the mixed cooperative-competitive nature of many real-world applications, we assume the possibility of “cheating” behaviors, such as falsifying reported actions, concealing relevant information, or colluding against specific agents. These actions can undermine fairness and overall efficiency, especially when the agents are driven by self-interest. Consequently, our objective is to devise an incentive-compatible mechanism that guarantees rational agents will, under the assumption of self-interested maximization, adopt rule-abiding or cooperative strategies rather than exploit the system for short-term advantage. 

In this work the joint action is generated by having each agent independently sample an action from its local policy based on its own observation, after which the individual actions are combined into the joint action vector a that represents the system-wide decision at that time step. The environment transition mechanism P then maps the current global state together with the joint action to a distribution over the next global state, thereby capturing the stochastic dynamics induced by decentralized decision-making. The long-term return for each agent is defined as the discounted cumulative reward, where future rewards are weighted by a discount factor γ to reflect the balance between immediate and delayed outcomes. Throughout training we adopt the centralized training and decentralized execution paradigm, under which the centralized critic or learner has access to global state and action information during training to improve stability and coordination, while during execution each agent relies only on its local observations to select actions in a scalable and decentralized manner. 

## **Blockchain and smart contract layer** 

To establish a secure and transparent decision-making process in the multi-agent setting, we integrate a blockchain network that operates as follows. First, each agent or its designated proxy can function as a node participating in the consensus procedure, thereby ensuring a decentralized ledger that records every critical interaction. In private or consortium blockchain scenarios, a subset of “witness nodes” may be authorized to uphold network security and consistency. Second, smart contracts formally encode the rules and incentives of the game, enabling automated verification of agent actions, execution of reward-and-penalty computations, and publication of both current system states and historical activity logs on-chain. By codifying these elements into smart contracts, the framework can systematically check for irregularities—such as deceptive bidding, unauthorized strategy changes, or unexpected coordination failures—and respond through preset sanctions or token redistributions. This eliminates the need for a third-party arbitrator and considerably reduces administrative overhead, as all agreements and transactions are self-enforced once written into the contract’s code. Finally, all key information, including bidding prices or traffic control policies, is maintained in the blockchain ledger to facilitate future auditing or incentive distribution. Because the ledger is tamper-resistant and visible to all authorized stakeholders, it supports an incontrovertible source of truth that fosters accountability and reduces disputes. Consequently, agents face increased pressure to comply with established rules and to adopt “honest” behavior over exploitative tactics, while simultaneously retaining the system’s decentralized advantages. 

## **Incentive compatibility mechanism principle** 

In designing the incentive mechanism, we assume that each agent aims to maximize its own expected return (i.e., cumulative reward), denoted by _Ui_ . Formally, the mechanism must satisfy: _Ui_ (honest strategy) _≥ Ui_ (dishonest strategy), indicating that no agent should find it profitable to deviate from truthful or cooperative participation. Achieving this condition relies on three key elements: immutable data recording, transparent incentive distribution, and long-term reward design. First, the immutable data recording capability provided by the blockchain infrastructure ensures that no single agent can retroactively alter past actions or transaction details for personal gain. Since all critical interactions, from bidding information to resource allocations, are logged in a shared ledger that is collectively maintained by the network’s participants, any attempt at retroactive tampering would be immediately detected or rendered computationally infeasible. As a result, dishonest agents face a diminished probability of extracting value from hidden manipulations, and the deterrent effect increases with the cost of mounting a successful attack on the blockchain. Second, transparent incentive distribution guarantees that each agent’s reward is determined solely by verifiable information stored on-chain, insulating the payout mechanism from unilateral tampering. By codifying the reward function and penalty rules within smart contracts, the system automatically calculates and distributes payoffs based on actions and outcomes visible to all authorized parties. This eliminates clandestine deals or back-channel negotiations, while also precluding any single point of failure or collusion arising from centralized bookkeeping. 

In competitive-cooperative scenarios where agents might otherwise conspire to inflate joint returns or ostracize a subset of participants, the blockchain records serve as a public adjudicator, significantly reducing the 

**Scientific Reports** |        (2025) 15:42841 

3 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

incentive to commit fraud. Finally, long-term reward design plays a pivotal role by explicitly favoring extended compliance and discouraging short-term exploitation. Through either reinforcement learning or game-theoretic equilibrium analysis, the mechanism can weight future rewards heavily enough that agents perceive greater gains from sustained honest engagement than from a momentary breach of protocol. Over repeated rounds of interaction, agents learn that abiding by the mechanism’s rules yields stable and high expected returns, while cheating or colluding generates only transient advantages—if any—followed by sanctions or lost trust. By integrating these three pillars into a coherent incentive-compatibility framework, the system fosters rational cooperation and reduces the strategic payoff of deceptive practices. This approach is particularly well-suited for environments where agents dynamically adapt to each other’s strategies and where the cost of information asymmetry can be high, as it realigns short-term interests with collective long-term efficiency. 

In this work incentive compatibility is defined as the condition that under the specified smart contract parameters and detection rules, neither a unilateral deviation by a single agent nor a coordinated deviation by a coalition yields a higher expected long-term return than following the prescribed compliant strategy. This definition rests on three essential elements: the immutable nature of blockchain records that prevents retroactive manipulation, the transparent settlement and redistribution rules codified in the smart contract that guarantee verifiable payoffs, and the explicit weighting of future rewards that ensures sustained compliance is more profitable than short-term exploitation. Concretely, when a deviation is detected by the contract’s monitoring logic, the redistribution coefficient applied to the deviating agent’s future rewards is reduced, the penalty is permanently logged on-chain, and this adjustment directly feeds back into the agent’s subsequent learning signals. 

## **Technical details** 

## **Reinforcement learning methodology** 

To facilitate efficient multi-agent collaboration and competition, we employ a Centralized Training and Decentralized Execution (CTDE) paradigm. During the training phase, a centralized component has access to global state information and can gather each agent’s actions and rewards, thus enabling more stable and informed gradient updates. Conversely, in the execution phase, each agent relies solely on locally available observations, preserving scalability and reducing communication overhead. Under this framework, several well-established MARL algorithms may be applied: 

- MADDPG (Multi-Agent Deep Deterministic Policy Gradient). This method extends the deterministic policy gradient approach to multi-agent settings by maintaining a centralized critic for each agent, which conditions on the global state and the actions of all agents. 

- MASAC (Multi-Agent Soft Actor-Critic). Particularly well-suited for mixed cooperative-competitive tasks, MASAC inherits the benefits of Soft Actor-Critic’s entropy augmentation, rendering it more robust to strategy uncertainties and better able to handle continuous action spaces. 

- QMIX, QTRAN, and Value Decomposition Methods. When cooperative objectives predominate, value decomposition can explicitly factor the joint action-value function into per-agent components, simplifying credit assignment and convergence. 

In practice, our system leverages MASAC in scenarios that require both cooperative coordination (e.g., managing shared resources) and competitive decision-making (e.g., strategic bidding), balancing adaptability with robust performance. Nonetheless, alternative algorithms may be adopted where discrete action spaces dominate or purely cooperative behavior is prioritized. 

## **Blockchain network and smart contract design** 

The blockchain layer underpins a secure, immutable record of agent interactions, anchoring our incentive mechanisms: 

- Network layer. We implement a consortium (permissioned) blockchain configuration to balance decentralization and performance. Frameworks like Hyperledger Fabric or specialized sidechains can be employed to achieve efficient consensus without sacrificing data integrity. This design choice ensures a controlled participant set (e.g., recognized stakeholders or agent proxies) while maintaining cryptographic security against external tampering. 

- Smart contract logic. At the heart of our system lies a set of smart contracts that codify the rules of the multi-agent game and govern reward distribution. Key operations include: 

   1. Data on-chain. Periodically or event-triggered, each agent’s observed local state, performed action, and immediate reward are submitted to the blockchain. Hashes of critical parameters or policy updates may also be stored for verifiability. 

   2. Reward/penalty computation. A dedicated function, rewardDistribution(), aggregates relevant data at the end of each round or phase, calculating each agent’s total payoff or penalty. The contract subsequently adjusts on-chain account balances accordingly. 

   3. Incentive compatibility safeguards. Embedded within the contract are rules for detecting anomalous behavior and penalizing proven violations. If consensus nodes or a predefined detection mechanism identify misconduct, the offending agent(s) receive reduced future rewards or face confiscation of on-chain tokens. 

**Scientific Reports** |        (2025) 15:42841 

4 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

By automating these processes through smart contracts, the system removes the need for manual oversight, strengthens trust via transparent execution, and ensures that any violations or questionable transactions are promptly surfaced and penalized. 

## **Mechanism design implementation** 

To fully operationalize our incentive-compatible framework within the blockchain-based multi-agent setting, we incorporate the following critical steps: 

1. Initial setup. Each agent _ai ∈ A_ is assigned a unique on-chain identity and an initial token balance, representing its stake or “budget” of potential rewards. These tokens may be transferred, awarded, or forfeited based on the agent’s subsequent behavior. 

2. Policy evaluation and submission. After each round, agents perform localized updates to their strategies (e.g., policy gradients or value-function refinements). To preserve traceability, the resulting model parameters or their cryptographic hashes are recorded on-chain, providing an immutable history of every agent’s policy evolution and facilitating accountability in case of disputes. 

3. Collusion and anomaly detection. Leveraging the chain’s historical data, we apply offline or near-real-time detection mechanisms to spot suspicious activity. In an automated bidding example, multiple agents consistently offering synchronized bids that stifle competition may be flagged for collusion; upon verification, they are subject to penalty assignment as per the smart contract’s terms. By contrast, an agent acting alone to manipulate prices without corroborating evidence from other agents might be categorized differently and subjected to alternative sanctions. 

4. Long-term reward maximization. Because our multi-agent reinforcement learning loops extend over multiple rounds, the underlying reward structure is carefully designed to prioritize sustained compliance and fair engagement over immediate, exploitative gains. Hyperparameters balancing short-term returns against future benefits ensure that agents discover and uphold cooperative (or at least non-manipulative) strategies yielding higher expected payoffs over repeated interactions. This long-horizon incentive alignment is reinforced by on-chain penalty mechanisms for misbehavior, adding the potential of future reward forfeiture to the immediate costs of dishonesty or collusion. 

We characterize the blockchain performance by measuring two key quantities: the end-to-end confirmation latency, defined as the elapsed time from transaction submission to final confirmation, and the transaction throughput, defined as the number of confirmed transactions per unit of wall-clock time. For experimental load mapping, in the automated market bidding domain we simulated 50 agents interacting over 200 rounds, while in the traffic signal coordination domain we simulated 5 intersections operating over 300 steps. 

In both cases we explicitly mapped each contract call frequency to the corresponding on-chain record per interaction step so that the measured throughput and latency faithfully represent the actual execution profile. For sampling and statistics, the first 10% of interaction steps were treated as a warm-up phase and excluded from analysis. During the main runs, timestamps of all transaction confirmations were recorded, and we computed mean and standard deviation using wall-clock time. Each reported value represents the average over three independent runs. The measured results showed that in the automated market bidding domain the system achieved an average throughput of 15 transactions per second with a consensus delay of 2.3 s per transaction, while in the traffic signal coordination domain the throughput was 12 transactions per second with a consensus delay of 1.9 s. These values were obtained under a permissioned blockchain configuration with four validator nodes running a Practical Byzantine Fault Tolerance consensus protocol. 

## **Algorithmic framework** 

To illustrate the training and execution pipeline, we adopt Multi-Agent Soft Actor-Critic (MASAC) as a representative algorithm. The following steps outline how the process is orchestrated in conjunction with a blockchain-based contract interface. 

In this description, let: 

- _ε_ be the multi-agent environment, 

- _C_ be the blockchain contract interface, 

- _T_ be the total number of training episodes. 

1.  Initialization. 

Each agent _ai ∈/ A_ begins with parameterized policy and value networks. Specifically, agent _ai_ has a policy _πθi_ and a value network _Qϕi_ . The parameters _θi_ and _ϕi_ are randomly initialized within stable bounds (e.g., using Xavier or orthogonal initialization). In parallel, the blockchain smart contract is configured with the reward distribution logic, including detection thresholds for anomalous behaviors (e.g., potential collusion) and predefined penalty rules. This setup ensures that upon completion of every episode, the chain can automatically validate actions, distribute incentives, and penalize misconduct. 

2. Iterative training (for _t_ = 1 : _t_ = 1 to _T_ ). 

Each training cycle is composed of an execution phase and a centralized training phase, followed by an on-chain reward settlement. 

3. Environment reset. 

**Scientific Reports** |        (2025) 15:42841 

5 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

At the start of each episode replay buffers) are prepared to record subsequent transitions. _t_ , the environment _ε_ is reset to an initial global state _s_ 0. All agent memories (e.g., 

## 4. Execution phase. 

For each time step _k_ within an episode, agent _ai ∈ A_ observes its local partial state _o[k] i_[ and samples an action ] _[a][k] i k_ from its current policy _πθi_ ( _a[k] i_ �� _oi_[)][. The environment then transitions to a new global state ] _[s][k]_[+1][ and furnishes ] each agent with an immediate reward _ri[k]_[. Alongside these transitions, the quadruples ][(] _[o][k] i[, a][k] i[, r] i[k]_[)][ (plus a ] timestamp) are recorded on-chain via _C_ to provide a transparent, immutable ledger of interactions. 

## 5. Centralized training. 

After collecting trajectories from all time steps of the episode, a centralized learner aggregates the experience tuples 

_{_ ( _sk, ak, rk, sk_ +1) _}_ = _{_ ( _sk,_ _**a** k,_ _**r** k, sk_ +1) _}_ 

for all agents. These experiences are used to update the value and policy parameters via batch optimization: 

**==> picture [233 x 25] intentionally omitted <==**

where _JQ_ ( _ϕi_ ) and _Jπ_ ( _θi_ ) represent the objective functions for updating the Q-value network and the policy network, respectively. The exact forms of these losses follow the Soft Actor-Critic or MADDPG frameworks, tailored to multi-agent settings[28][,][29] . Typically, the centralized critic or global state information can be leveraged in the gradient computations to enhance training stability and address non-stationarity. 

## 6. Incentive/penalty settlement. 

Upon completion of an episode, the blockchain contract automatically executes its rewardDistribution() function. Rewards (or punishments) are computed based on aggregated performance metrics and consistency checks. If collusion or other policy infractions are flagged by the on-chain detection logic, the offending agent(s) receive partial or total forfeiture of tokens. Conversely, rule-abiding agents maintain or increase their token balances. These token adjustments reinforce the incentive-compatible nature of the system, where cooperative or lawful strategies align with long-term returns. 

7. Progression to the next episode. 

Updated agent networks are retained for the next iteration, while any penalties or credits are permanently recorded on-chain. Agents thus face evolving conditions in which past misbehavior influences future earning potential, further encouraging compliant strategies over multiple episodes. 

## 8. Output. 

After TTT training episodes, each agent _ai ∈ A_ possesses an updated policy _πθi_ = _π_ ( _θi_ ) that ideally satisfies the designed incentive-compatibility criteria, balancing its own cumulative reward with on-chain constraints. Additionally, a comprehensive record of every transaction, penalty, and reward distribution is accessible on the blockchain, certifying that the final multi-agent mechanism is transparent, traceable, and resistant to manipulative behaviors. 

During the data collection phase, at each interaction step we continue to store the tuple ⟨local observation, action, raw environment reward, timestamp, contract call handle⟩. At the end of an episode the rewardDistribution contract is triggered, and the verified reward R_i together with the detection flags is synchronously written back into the replay buffer. In the training phase, the critic updates are performed using R_i instead of the raw environment reward, with the explicit rationale that this verified reward already incorporates redistribution and penalties, thereby correcting for short-term gains that would otherwise be obtained from improper behavior. In the policy update objective we introduce an additional weighting term to capture the impact of future settlement rules, described textually as “reducing preference for action–state pairs that may trigger penalties and increasing preference for long-term compliant behaviors,” while the entropy regularization remains unchanged. The settlement and update sequence follows a strict “end-of-episode synchronous settlement” policy, and asynchronous write-backs are deliberately avoided to prevent signal drift. We also specify that the order of random seed initialization, replay buffer sampling, and target network updates is fixed across runs, which ensures full reproducibility of the training dynamics. 

## **Experimental design and evaluation** 

To validate the effectiveness of our proposed framework, we conducted experiments in two representative multiagent game environments, each reflecting a distinct combination of competitive and cooperative interactions. All trials were carried out on a permissioned blockchain testbed, with a set of validator nodes responsible for consensus and execution of smart contracts. Data were analyzed in Python using the numpy and matplotlib libraries to produce quantitative metrics and visualizations. 

**Scientific Reports** |        (2025) 15:42841 

6 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

|**Configuration**|**Social welfare (avg)**|**Individual profit (avg)**|**Collusion success Rate (%)**|**Incentive compatibility index**|
|---|---|---|---|---|
|Baseline (no blockchain)|1200 (± 40)|12 (± 0.6)|15.3 (± 2.1)|0.72 (± 0.05)|
|Blockchain (no detection)|1250 (± 35)|12.5 (± 0.7)|10.8 (± 1.8)|0.78 (± 0.04)|
|Full blockchain mechanism|1320 (± 50)|13.2 (± 0.4)|3.4 (± 0.9)|0.85 (± 0.03)|



**Table 1** . Performance metrics of different configurations in the automated market bidding environment. 

**Fig. 1** . Comparison of social welfare and collusion success rate under different market bidding configurations. The Full Blockchain Mechanism achieves the highest social welfare while reducing collusion success rate to under 4%. In contrast, the absence of blockchain results in lower welfare and a significantly higher rate of successful collusion. 

## **Automated market bidding** 

In this scenario, _N_ agents represent bidders competing for a shared resource, such as advertising slots, across a series of auction rounds. Each round follows a format akin to Vickrey or multi-price mechanisms, determining the ultimate payment based on the second-highest or averaged bids. We deployed a consortium blockchain (with five validator nodes) to record all bidding and settlement transactions. Smart contracts embedded collusion-detection logic, automatically penalizing patterns that indicated coordinated underbidding or price manipulation. 

We ran simulations with _N_ = 100 bidders, each participating in 200 consecutive auction rounds. Agents used the MASAC algorithm described in “Technical details”, with hyperparameters fine-tuned to balance exploration and exploitation. Table 1 summarizes key metrics—Social Welfare, Individual Profit, Collusion Success Rate, and Incentive Compatibility—in three configurations: 

1. Baseline (no blockchain): Standard multi-agent RL without on-chain enforcement. 

2. Blockchain without collusion detection: Bidding recorded on-chain, but no penalty for detected collusion. 

3. Full blockchain mechanism: All records on-chain plus the automated detection and penalty contract. 

From the table, the presence of blockchain alone improved social welfare by around 4% relative to the baseline, while the introduction of explicit collusion detection and automated penalties further raised the overall welfare by roughly 10% compared to the baseline. Similarly, individual profits increased as fewer “price wars” and manipulative tactics occurred, while the Collusion Success Rate (i.e., instances of effective underbidding coalitions) dropped substantially. The Incentive Compatibility Index, measured as the ratio of honest-strategy returns to potential cheating-strategy returns over 200 rounds, rose significantly under the full mechanism, indicating that rational bidders found greater benefit in transparent, rule-abiding behavior (see Fig. 1). 

In addition to the existing baselines, we have implemented MADDPG and COMA under settings aligned with our proposed method. For both, the training duration, learning rate, network width, and exploration temperature were matched to the values used in our framework to ensure fair comparison. Beyond MARL baselines, we also included two non-blockchain mechanisms: the centralized trust authority baseline, in which a single coordinator collects agent actions, computes rewards, and distributes outcomes while maintaining an internal log; and the cryptographic commitment scheme baseline, in which agents first commit to their actions 

**Scientific Reports** |        (2025) 15:42841 

7 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

using hash-based commitments and later reveal them to enable verification without a full blockchain. All baselines shared identical state and reward interfaces with our method to isolate the effect of the coordination and enforcement mechanisms rather than input design. 

The results demonstrate clear relative differences among the three categories of mechanisms. In terms of social welfare, the blockchain-based MASAC consistently achieved the highest values, followed by the centralized trust authority, with the commitment scheme trailing slightly above the no-blockchain condition. For collusion success rates, the blockchain setup maintained the lowest values, the commitment scheme achieved moderate reduction, while the centralized authority remained vulnerable to manipulation due to its single point of control. For the incentive compatibility index, the blockchain configuration again scored highest, while the commitment scheme reached intermediate levels and the centralized authority achieved only marginal improvements. Importantly, the centralized authority baseline exposed the risk of single-point failure and limited transparency, as it could be both a bottleneck and a collusion target, while the commitment scheme lacked automated enforcement and thus weakened long-term compliance. In contrast, the blockchain-enhanced framework provided auditability and traceability by design, ensuring that rule violations were both recorded and penalized without reliance on trust in a central party. 

## **Intelligent traffic coordination** 

Here, multiple agents each control the traffic signals at an intersection within a simulated city grid of five major crossroads, aiming to reduce overall congestion during peak flow periods. Although each agent strives to minimize queue length at its own intersection, overly extended green phases can increase wait times for connecting roads, thus establishing a hybrid cooperative-competitive dynamic. 

We integrated a lightweight blockchain system with three validator nodes, recording the timing of signal changes, average vehicle queue lengths, and any emergent anomalies. The smart contract specified traffic-flow rules and a congestion-penalty scheme wherein an intersection excessively favoring its own traffic would incur demerit points. If a node’s penalty points exceeded a threshold, its subsequent token rewards were automatically reduced. 

In our experiments, each simulation ran for 300 time steps under varying traffic inflow rates. We contrasted a baseline multi-agent RL approach (QMIX) with our blockchain-enhanced MASAC strategy. Table 2 highlights three metrics: 

The Avg Wait Time was reduced by around 8% when blockchain-based cooperation was introduced, dropping from 45.3 s to 41.8 s. Fairness, measured as the variance of average wait times across intersections, improved significantly, suggesting that intersections no longer attempted to “offload” their congestion at the expense of neighboring roads. Finally, the Stability Index, computed as the proportion of time steps in which the system remained at or near a locally optimal traffic distribution (within a certain threshold of queue lengths), increased from 0.72 to 0.83 under the blockchain model. 

We further assessed the system’s resilience to abrupt changes—such as a lane closure at one intersection— where blockchain records enabled rapid detection and reallocation of signal priorities (see Fig. 2). Traffic recovered to its steady state in under 15 steps (on average), compared to over 25 steps for the baseline. A timeseries comparison of queue lengths is illustrated by the Python code snippet below, generating a line plot: 

To further illustrate the system’s dynamic response and temporal-spatial coordination effectiveness, we visualized three aspects of the experiment in Fig. 3. Figure 3a presents the recovery trajectory of average queue length across all intersections following a simulated lane closure at time step $t = 0$. The Blockchain-MASAC model rapidly reduces congestion, stabilizing queue lengths below 30 units within 15 steps, while the baseline QMIX system takes more than 25 steps to reach comparable conditions. This supports the earlier observation of improved resilience and faster convergence. Figure 3b,c provide heatmap visualizations of per-intersection vehicle wait times over a 50-step simulation window, contrasting the baseline and blockchain settings. In the baseline scenario, prolonged localized congestion can be observed, especially in intersection 2 and 4, which persist throughout the episode. Conversely, the Blockchain-MASAC approach results in visibly lower wait times with reduced temporal-spatial variance, indicating more balanced signal allocation across intersections. Together, these visualizations offer strong evidence that the proposed blockchain-enhanced coordination framework not only improves macro-level traffic efficiency but also enhances fine-grained fairness and responsiveness in localized decision-making. It further confirms that decentralized, incentive-driven control policies can outperform centralized heuristics under high-stakes, dynamic congestion environments. 

We further implemented QMIX-V2 and a centralized critic variant without blockchain enforcement as comparison points. QMIX-V2 captures value decomposition learning under cooperative settings, while the centralized critic variant retains a shared evaluator but omits any on-chain redistribution or penalty enforcement. Both methods achieved reasonable fairness and partial stability but consistently underperformed compared to the blockchain-enhanced MASAC. Specifically, fairness scores improved relative to plain QMIX but lagged behind the blockchain variant, and stability remained sensitive to agents adopting extreme strategies without 

|**Confguration**|**Avg wait time (s)**|**Fairness (var of wait)**|**Stability index**|
|---|---|---|---|
|Baseline (QMIX)|45.3 (± 3.2)|104.1 (± 12.0)|0.72 (± 0.05)|
|Blockchain-based MASAC|41.8 (± 2.5)|76.7 (± 10.4)|0.83 (± 0.04)|



**Table 2** . Traffic coordination performance under two control strategies in a multi-agent intersection environment. 

**Scientific Reports** |        (2025) 15:42841 

8 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

**Fig. 2** . Average queue length over time under a simulated traffic disruption event. The blockchain-enhanced MASAC model demonstrates faster recovery and lower congestion levels compared to the baseline QMIX approach. This reflects superior resilience in dynamic urban traffic conditions. 

**Fig. 3** . Spatiotemporal performance comparison of traffic control strategies under disruption. ( **a** ) Recovery curve showing the reduction in average queue length after a lane closure event. ( **b** ) Heatmap of intersectionlevel wait times using baseline QMIX, with visible localized congestion. ( **c** ) Corresponding heatmap under Blockchain-MASAC, exhibiting lower queue lengths and more balanced traffic distribution. 

effective penalty mechanisms. In contrast, the introduction of on-chain redistribution directly improved both fairness and stability, highlighting the unique contribution of blockchain enforcement beyond the baseline cooperative learning architectures. 

The observed improvement in stability index is directly attributable to the effect of on-chain penalties, which suppress extreme signal extension strategies that otherwise destabilize coordination cycles. The reduction in variance across runs reflects enhanced cross-intersection balance, since redistribution discourages opportunistic exploitation by single agents and promotes more even outcomes across all controlled nodes. 

## **Policy robustness under noisy rewards** 

To further examine the resilience of our blockchain-enhanced multi-agent framework, we evaluated its robustness under stochastic perturbations applied to the reward signals during training. In real-world decentralized systems such as market bidding or distributed traffic control, agents may encounter noisy observations or delayed, imprecise reward feedback due to latency, sensor errors, or adversarial interference. Therefore, an effective mechanism should not only optimize incentives in ideal environments but also remain stable and effective under imperfect conditions. 

We simulated an auction-based multi-agent system as described in “Automated market bidding”, involving _N_ = 100 bidding agents over 200 auction rounds. Three configurations were tested: 

**Scientific Reports** |        (2025) 15:42841 

9 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

- Baseline (no blockchain): Standard multi-agent reinforcement learning without any blockchain infrastructure. 

- Blockchain (no detection): All bidding data recorded on-chain, but without collusion penalty. 

- Full blockchain mechanism: Complete blockchain integration including on-chain record-keeping, automated collusion detection, and incentive-compatible reward redistribution. 

To simulate environmental uncertainty, we introduced Gaussian noise _N_ (0 _, σ_[2] ) to the reward signals during policy updates, with _σ ∈_ 0 _._ 0 _,_ 0 _._ 1 _,_ 0 _._ 2 _,_ 0 _._ 3 _,_ 0 _._ 4. For each noise level, we repeated the training process over five runs and measured the average cumulative reward across agents at convergence. 

4 The results are illustrated in Fig. , which plots the average reward performance for each configuration under increasing levels of reward noise. 

From the plot, it is evident that the Full Blockchain Mechanism consistently yields higher average returns 

than the other two baselines, across all noise levels. Specifically: 

- Under zero noise, the Full Blockchain agents achieve an average cumulative reward of approximately 1300, compared to 1250 and 1200 for the partial and no-blockchain cases, respectively. 

- As noise increases to _σ_ = 0 _._ 4, all configurations experience some degradation in performance. However, the performance drop is most severe in the No Blockchain setup, where rewards fall below 1020. In contrast, the Full Blockchain mechanism maintains an average above 1170, demonstrating greater resistance to perturbation. 

- The gap between configurations becomes more pronounced under higher noise. For example, at _σ_ = 0 _._ 3, the Full Blockchain mechanism outperforms the baseline by approximately 14% in average reward. 

These results validate the robustness of on-chain enforcement, particularly the role of immutable records and automated smart contract-based redistribution in stabilizing learning dynamics. The system’s ability to preserve incentive compatibility even in the presence of noisy or distorted feedback makes it suitable for deployment in real-world decentralized and adversarial environments. 

## **Cross-agent behavioral correlation analysis** 

While prior experiments have demonstrated improvements in system-level efficiency, fairness, and incentive compatibility, it is equally critical to assess how agents’ behavioral strategies evolve and interact over time. In multi-agent environments, particularly those with mixed cooperative-competitive dynamics, agents may exhibit convergence toward similar policies—sometimes driven by rational learning, but other times reflecting implicit coordination or emergent collusion. Therefore, we extend our analysis to examine whether the proposed blockchain-based mechanism can promote behavioral diversity while suppressing unintended convergence or homogenization. In this analysis, the total agent population consisted of 100, and we selected 20 agents solely for readability. The subset was chosen by uniform random sampling, and the procedure was repeated across different random seeds. In all cases, the correlation structures and clustering trends were consistent, indicating robustness of the conclusions to sampling choices. 

**Fig. 4** . Reward robustness under noisy environment. 

**Scientific Reports** |        (2025) 15:42841 

10 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

We collected post-training behavior vectors from agents in the market bidding environment, where each vector encodes an agent’s action distribution, bidding entropy, temporal deviation, and win-loss patterns over the final 50 auction rounds. Using these representations, we conducted three complementary analyses: 

1. Pairwise correlation analysis to assess behavioral alignment between agents. 

2. Hierarchical clustering to evaluate global strategy grouping. 

3. Principal component analysis (PCA) to visualize high-dimensional behavioral embeddings. 

All analyses were performed under the three experimental configurations—No Blockchain, Blockchain without Detection, and Full Blockchain Mechanism. The goal was to investigate how incentive mechanisms influence the evolution and diversity of agent behaviors. Figure 5 presents a heatmap of pairwise action correlations among 20 randomly sampled agents in the No Blockchain setting. 

The results show multiple agent pairs with correlation values exceeding 0.90, indicating highly similar bidding behaviors. In the absence of explicit penalty mechanisms, agents tend to gravitate toward “safe” strategies, which in competitive settings can result in coordinated underbidding or reduced market exploration. This emergent homogenization undermines both competition and long-term social welfare. 

To further understand behavioral grouping structures, Fig. 6 displays a strategy clustering heatmap based on hierarchical linkage. 

Under the Full Blockchain Mechanism, agents are more evenly distributed across the strategy space, and distinct clusters reflect individualized policy trajectories. In contrast, the baseline configuration leads to fewer, tighter clusters, again pointing to over-converged behaviors in the absence of decentralized enforcement. 

Lastly, Fig. 7 provides a visual projection of agent strategies using PCA over the learned behavior embeddings. 

The PCA projection shows that agents trained under the Full Blockchain Mechanism occupy a broader region of the latent behavior space, forming well-separated clusters. This suggests the incentive-aligned design encourages exploration and discourages collusion-driven convergence. Conversely, agents under the No Blockchain configuration are concentrated in a narrow region, consistent with homogenized or risk-averse behavior. 

## **Ablation studies** 

To better understand the contributions of individual components in our blockchain-enhanced multi-agent reinforcement learning framework, we conducted ablation studies by selectively removing key elements from the full architecture. These experiments allow us to isolate the impact of critical modules—including on-chain record-keeping, collusion detection, and smart contract-based incentive redistribution—on both system performance and behavioral alignment. 

We defined four variants of the overall system: 

**Fig. 5** . Agent action correlation heatmap. Each entry in the matrix reflects the Pearson correlation coefficient between the behavior vectors of two agents. High off-diagonal values indicate strong behavioral alignment, which may suggest implicit collusion or policy convergence. 

**Scientific Reports** |        (2025) 15:42841 

11 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

## y Uustering neatmap 

**Fig. 6** . Strategy clustering heatmap. Agents are clustered using Ward’s linkage on their normalized behavior vectors. The heatmap encodes the intensity of specific behavior features (e.g., bid frequency, deviation). The dendrogram reveals emergent strategy families. 

Full blockchain mechanism (FBM): Our proposed complete framework, including on-chain recording, incentive-compatible smart contracts, and automated collusion detection. 

FBM—detection module: Full mechanism with the collusion detection logic disabled; all actions are still recorded on-chain, but no penalization is enforced. 

FBM—incentive redistribution: Smart contracts log data and detect collusion, but the dynamic redistribution of rewards based on behavior is removed. 

FBM—blockchain logging: All on-chain functions are disabled; smart contract logic and detection modules exist but rely on off-chain state, compromising transparency and traceability. 

Each variant was evaluated in the automated market bidding environment using the same MASAC backbone. Metrics measured include Social Welfare, Incentive Compatibility Index, Collusion Rate, and Reward Robustness under Noise (σ = 0.3). All experiments were averaged over 5 randomized runs. 

The Full Blockchain Mechanism consistently outperforms the ablated variants across all evaluation criteria. Disabling the collusion detection module leads to a noticeable increase in coordinated underbidding (from 3.4% to 10.5%), which directly impacts social welfare and fairness. The absence of reward redistribution further reduces the incentive compatibility index, indicating that while detection alone discourages blatant collusion, it is insufficient for long-term behavioral alignment without corresponding financial disincentives (see Table 3). Removing the blockchain logging layer results in the most significant performance degradation. Without a verifiable, tamper-resistant record of agent behavior, even well-designed off-chain mechanisms are vulnerable to information asymmetry and manipulation. This finding reinforces the importance of transparency and auditability in achieving trustworthy multi-agent coordination. 

Furthermore, when noise is injected into the reward function (σ = 0.3), the Full Blockchain Mechanism shows the highest resilience, maintaining over 1200 in average reward, while the no-logging version drops below 1100. This demonstrates the stabilizing effect of smart contracts as deterministic policy anchors in stochastic environments. 

**Scientific Reports** |        (2025) 15:42841 

12 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

**Fig. 7** . PCA projection of learned agent behaviors. Each dot represents an agent in the 2D PCA space. Distinct color groups correspond to different configurations. Larger spatial separation implies greater behavioral diversity. 

|**Configuration**|**Social welfare**|**Incentive index**|**Collusion rate (%)**|**Noise-robust reward**|
|---|---|---|---|---|
|Full blockchain mechanism (FBM)|1320 (± 50)|0.85 (± 0.03)|3.4 (± 0.9)|1210 (± 35)|
|FBM—detection module|1265 (± 45)|0.77 (± 0.05)|10.5 (± 1.5)|1160 (± 40)|
|FBM—incentive redistribution|1248 (± 50)|0.74 (± 0.06)|8.3 (± 1.3)|1145 (± 42)|
|FBM—blockchain logging|1202 (± 60)|0.69 (± 0.04)|14.6 (± 2.1)|1095 (± 50)|



**Table 3** . Ablation study results across key performance dimensions in the automated bidding environment. 

The ablation results confirm that each component of our framework plays a complementary and essential 

role: 

- Blockchain logging provides the immutable substrate for verifiability; 

- Collusion detection enforces behavioral discipline; 

- Incentive redistribution aligns agent utility with system objectives. 

Omitting any of these elements leads to measurable drops in performance, fairness, and robustness. Thus, the integrated architecture is key to sustaining scalable and ethical multi-agent reinforcement learning systems in decentralized settings. 

We further support the ablation analysis with two additional visualizations provided in Fig. 8. These help quantify the trade-offs and variance across ablated configurations beyond average performance. Figure 8a plots the relationship between average social welfare and the incentive compatibility index. The Full Blockchain Mechanism clearly dominates, achieving both higher reward and stronger incentive alignment. The baseline (No Logging) configuration clusters at the bottom left, indicating both poor efficiency and weak incentive adherence. The partial configurations (No Detection and No Redistribution) occupy intermediate positions, forming a visual Pareto frontier of performance degradation. Figure 8b presents an error bar chart of social welfare across five experimental runs. The Full Blockchain system not only delivers the highest mean reward but also exhibits the smallest variance. In contrast, the No Logging configuration shows significant fluctuation, reinforcing concerns about its reliability in stochastic environments. 

These visualizations collectively reinforce the conclusion that all three architectural elements—blockchain transparency, behavioral monitoring, and dynamic incentive redistribution—are essential for achieving robust, fair, and scalable multi-agent coordination in decentralized systems. 

The training complexity of the blockchain-enhanced MASAC framework can be decomposed into learning updates and blockchain settlement. For the learning component, the critic update scales quadratically with the number of agents, since it must condition on the joint action space, while the actor updates scale linearly with agent count. For the blockchain component, both transaction logging and contract settlement costs increase 

**Scientific Reports** |        (2025) 15:42841 

13 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

**Fig. 8** . Ablation study comparisons across key dimensions. ( **a** ) Scatter plot comparing social welfare and incentive compatibility index across ablation configurations. The Full Blockchain setup achieves the highest balance. ( **b** ) Error bar plot showing standard deviation in social welfare performance over 5 trials per configuration. The complete mechanism yields both higher and more stable outcomes. 

linearly with the number of agents and the number of interactions per episode, with an additional consensus delay that remains constant regardless of population size. Empirically, we measured that in the bidding domain with 50 agents and 200 rounds, each training episode required approximately 2.1 s with peak memory of 1.8 GB, while in the traffic domain with 5 intersections and 300 steps, the per-episode training step averaged 1.2 s with peak memory of 1.1 GB. All measurements were repeated across three independent runs under identical configurations, and results remained stable, confirming the reproducibility of the computational cost estimates. 

Memory consumption is dominated by the replay buffer, which scales linearly with the number of agents, episode length, and stored trajectories. Blockchain storage overhead is relatively modest because only hashed summaries of model updates and settlement outcomes are logged on-chain rather than full parameter vectors. This strategy reduces long-term storage requirements while still ensuring auditability and transparency. The measured values indicate that memory usage scales predictably and that persistent blockchain storage remained well within practical limits under all tested configurations. 

## **Discussion** 

The preceding experimental results demonstrate the potential and efficacy of integrating blockchain-based incentive-compatible mechanisms into multi-agent reinforcement learning (MARL) systems. By embedding smart contracts and decentralized logging into the learning loop, we successfully enhanced system-wide metrics—such as social welfare, fairness, and robustness—while suppressing harmful behaviors like collusion and policy convergence. This architecture creates a transparent and verifiable interaction substrate that reconfigures the strategic landscape: agents are systematically disincentivized from manipulative strategies, and instead guided toward compliance and cooperation, not through centralized enforcement, but via on-chain economic alignment. Despite these promising results, several limitations remain and motivate the following research directions: 

## **Scalability and latency in blockchain infrastructure** 

While our system performed effectively in medium-scale environments (e.g., 100 bidding agents or 5 traffic intersections), real-world applications often demand much higher throughput and lower response latency— especially in high-frequency auction markets, real-time traffic management, or drone swarm coordination. Traditional blockchain protocols (e.g., Proof-of-Work or PBFT-based permissioned ledgers) struggle under such loads, leading to bottlenecks in transaction finalization and smart contract execution. 

Future work should explore the integration of Layer-2 solutions (e.g., rollups, state channels) and asynchronous cross-shard communication to offload high-frequency interactions while preserving the tamper-resistance and traceability of on-chain commitments. Additionally, emerging consensus models such as HotStuff, DAG-based ledgers, or zero-knowledge rollups may offer promising directions to support large-scale, low-latency multiagent ecosystems. 

In this pathway, transactions are aggregated into batches off-chain and submitted to the base chain with a fraud-proof window that allows disputes to be raised. The mechanism introduces a delay between submission and finality, but it substantially improves throughput by allowing many interactions to be processed in parallel before final settlement. We describe how the fraud-proof window translates into a bounded finality delay that still aligns with the interaction frequencies in both the automated market bidding and intelligent traffic control domains, where auction rounds and signal updates can tolerate deferred finality while benefiting from higher 

**Scientific Reports** |        (2025) 15:42841 

14 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

throughput. Zero-Knowledge Rollups: In this pathway, succinct validity proofs are generated for each batch of off-chain transactions and verified on-chain with a constant cost independent of the batch size. This property makes the approach highly suitable for domains requiring very frequent and lightweight interactions, such as small bids or rapid intersection updates, because throughput can grow with batching while verification overhead remains fixed. We emphasize that the validity proofs preserve transparency and auditability by ensuring that all on-chain states are cryptographically guaranteed to be correct, maintaining the system’s accountability even under very high interaction rates. State Channels: This mechanism allows a fixed group of agents to conduct an unbounded number of interactions off-chain by maintaining a signed state channel, with only the channel opening and closing recorded on-chain. This bidirectional settlement approach is well-suited for repetitive micro-interactions such as recurring bids among a stable set of participants or signal adjustments among adjacent intersections, where the overhead of continuous on-chain logging is unnecessary. The use of state channels in these contexts can drastically reduce latency and blockchain load while still preserving enforceability upon disputes. Asynchronous Sharded Consensus: This pathway partitions the overall agent population into clusters, each operating on a separate shard with its own consensus process, while cross-shard verification and secure message bridges maintain global consistency. We describe how clustering agents into shards aligns with natural subdomains such as different auction markets or separate traffic regions. The system enforces crossshard security assumptions to prevent inconsistencies, and mechanisms for cross-cluster collusion detection ensure that coordinated deviations across shards are still penalized. 

## **Formal verification and safety of smart contracts** 

The core enabler of our incentive mechanism lies in the correct and secure execution of smart contracts. However, any logical flaw, coding vulnerability, or misalignment in contract semantics can lead to erroneous rewards, false-positive collusion penalties, or catastrophic systemic manipulation. These risks are further amplified in adversarial MARL settings, where agents may actively probe for exploits. 

To address this, future research must incorporate formal methods and symbolic analysis into the smart contract development pipeline, possibly through tools like Solidity verification (e.g., Oyente, Mythril) or _formal proof assistants (e.g., Coq, F)._ * Moreover, the design of multi-agent-aware verification tools—capable of modeling game-theoretic constraints and policy-dependent interactions—remains an open challenge. Ensuring safety in dynamic, evolving agent environments calls for adaptive contract monitors and secure rollback mechanisms to mitigate cascading failures. 

To complement the implementation details provided earlier in “Blockchain network and smart contract design”, here we summarize the complete verification and assurance pipeline, the corresponding verification objectives, and the conclusions established. The toolchain we employed consisted of Mythril for static vulnerability detection, Oyente for symbolic execution cross-checks, Solidity SMTChecker for automated property proofs, and Coq for higher-level logical verification. The verification objectives were threefold: first, to guarantee the absence of low-level vulnerabilities such as reentrancy, overflow, or unbounded gas consumption; second, to prove safety properties including balance preservation after reward redistribution and immutability of transaction integrity; and third, to formally establish incentive compatibility at the logical contract layer by showing that redistribution and penalty functions always terminate deterministically and enforce compliance under any admissible input set. The outcomes of this pipeline confirmed that no exploitable low-level bugs exist in the deployed contracts, that all safety invariants hold under symbolic and SMT-based reasoning, and that the Coq proofs guarantee the correctness and incentive alignment of redistribution rules. 

## **Adaptive incentive design under strategy evolution** 

Our current incentive-compatible framework is based on fixed smart contract rules that detect collusion patterns and redistribute rewards accordingly. However, as agents adapt their behaviors through reinforcement learning, the original detection heuristics may become obsolete or circumventable. Similarly, over-penalizing benign correlations could discourage natural cooperation, reducing system efficiency. 

This motivates the exploration of adaptive or meta-learned mechanism design, where the detection thresholds, penalty functions, and reward redistribution logic evolve in tandem with the agents’ strategy distributions. Techniques such as inverse reinforcement learning (IRL) or meta-gradient optimization may be leveraged to learn dynamic rule sets that remain robust and fair across temporal shifts. Furthermore, game-theoretic regret analysis may help in bounding the worst-case incentive misalignments over long horizons. 

## **Generalizability to cross-domain multi-agent systems** 

The proposed framework was validated in two domains: automated market bidding and urban traffic signal control. Both feature clear reward externalities and mixed cooperative-competitive dynamics. However, decentralized systems span a much wider landscape—ranging from energy trading, decentralized logistics, supply chain coordination, to UAV swarm deployment and distributed cybersecurity. 

Future work should systematically investigate how the proposed blockchain-MARL framework generalizes across such domains, particularly those with partial observability, heterogeneous agents, or asymmetric objectives. Moreover, multi-layer scenarios—such as hierarchical multi-agent systems or federated learning with incentive misalignment—present exciting but complex targets for extending our mechanism. The design of domain-specific smart contracts, combined with scalable policy-sharing architectures, will be central to realizing this generalization. 

## **Ethical, legal, and societal implications** 

Finally, as blockchain-augmented decision-making systems increasingly influence real-world infrastructure (e.g., traffic systems, financial exchanges, energy grids), questions of auditability, accountability, and social 

**Scientific Reports** |        (2025) 15:42841 

15 

| https://doi.org/10.1038/s41598-025-20247-8 

www.nature.com/scientificreports/ 

acceptability emerge. While smart contracts offer transparency, they also risk codifying biases or errors at scale. Likewise, economic incentives may unintentionally disadvantage certain agent groups or lead to resource monopolization. 

Thus, future research must engage in interdisciplinary dialogue across law, economics, and human–computer interaction to ensure that incentive-compatible systems also meet ethical and societal standards. This includes incorporating differential privacy, algorithmic fairness, and human-in-the-loop verification into the blockchainMARL stack. 

## **Conclusion** 

This paper presents a novel framework for designing incentive-compatible mechanisms in multi-agent game environments by integrating blockchain infrastructure with reinforcement learning and smart contract technologies. The proposed system leverages the tamper-resistant, decentralized, and transparent properties of blockchain to enforce rule adherence and enable verifiable interaction records, while embedding incentive structures directly into the agents’ learning process through on-chain reward redistribution and automated behavior monitoring. By combining principles from mechanism design with multi-agent soft actor-critic learning, the framework ensures that agents acting in partially cooperative and competitive environments are systematically incentivized to adopt honest, rule-abiding strategies that align individual interests with collective system efficiency. 

To evaluate the effectiveness of the proposed framework, we conducted extensive experiments in two representative domains: automated market bidding and intelligent traffic coordination. The results demonstrate that the full blockchain-enhanced mechanism significantly improves social welfare, reduces the prevalence of collusive behaviors, and enhances both fairness and operational stability. These improvements are attributed to the transparent reward structures, immutable behavior records, and the enforcement of penalties via smart contracts, all of which contribute to fostering robust and ethically-aligned multi-agent interactions. 

This work contributes a foundational architecture for bridging distributed artificial intelligence and blockchain technology, offering new insights into how decentralized mechanism design can scale to complex, dynamic, and potentially adversarial multi-agent systems. As both blockchain platforms and multi-agent reinforcement learning algorithms continue to evolve, the integrated approach proposed here is expected to show increasing promise across a wide range of real-world applications, including smart cities, decentralized energy markets, supply chain finance, and beyond. Looking forward, future advances in scalability, contract verification, and adaptive incentive optimization will further enhance the feasibility and trustworthiness of blockchain-driven coordination frameworks, paving the way for a new era of transparent, self-organizing intelligent systems. 

## **Data availability** 

The data that support the findings of this study are available from the corresponding author, KE Tian, upon reasonable request. 

Received: 18 July 2025; Accepted: 12 September 2025 

## **References** 

1. Hu, K. _et al._ A review of research on reinforcement learning algorithms for multi-agents. _Neurocomputing_ . 128068 (2024). 

2. Feriani, A. & Hossain, E. Single and multi-agent deep reinforcement learning for AI-enabled wireless networks: A tutorial. _IEEE Commun. Surv. Tutor._ **23** , 1226–1252 (2021). 

3. Wang, Y., Yang, X., Liang, H. & Liu, Y. A review of the self-adaptive traffic signal control system based on future traffic environment. _J. Adv. Transp._ **2018** , 1096123 (2018). 

4. Vasirani, M. & Ossowski, S. A market-inspired approach for intersection management in urban road traffic networks. _J. Artif. Intell. Res._ **43** , 621–659 (2012). 

5. Fzrachi, A. & Stucke, M. E. Sustainable and unchallenged algorithmic tacit collusion. _Nw. J. Tech. Intell. Prop._ **17** , 217 (2019). 

6. Mishra, R., Kshetri, N. & Jha, S. K. In _Blockchain Technology for Cyber Defense, Cybersecurity, and Countermeasures_ 17–33 (CRC Press, 2025). 

7. Shi, D., Li, A. & Yang, B. In _International Conference on Database Systems for Advanced Applications._ 171–179 (Springer). 

8. Hewa, T. M., Hu, Y., Liyanage, M., Kanhare, S. S. & Ylianttila, M. Survey on blockchain-based smart contracts: Technical aspects and future research. _IEEE Access_ **9** , 87643–87662 (2021). 

9. Han, R., Yan, Z., Liang, X. & Yang, L. T. How can incentive mechanisms and blockchain benefit with each other? A survey. _ACM Comput. Surv._ **55** , 1–38 (2022). 

10. Zu, X., Hu, F., Liang, D. & Liu, Y. In _2024 4th International Conference on Electronic Information Engineering and Computer Communication (EIECC)._ 990–996 (IEEE). 

11. Abadi, J. & Brunnermeier, M. _Blockchain Economics_ . (National Bureau of Economic Research, 2018). 

12. Busoniu, L., Babuska, R. & De Schutter, B. A comprehensive survey of multiagent reinforcement learning. _IEEE Trans. Syst. Man Cybern. Part C Appl. Rev._ **38** , 156–172 (2008). 

13. Shoham, Y. & Leyton-Brown, K. _Multiagent Systems: Algorithmic, Game-Theoretic, and Logical Foundations_ (Cambridge University Press, 2008). 

14. Tan, M. In _Proceedings of the Tenth International Conference on Machine Learning,_ 330–337. 

15. Oliehoek, F. A. & Amato, C. _A Concise Introduction to Decentralized POMDPs_ , vol. 1 (Springer, 2016). 

16. Lowe, R. _et al._ Multi-agent actor-critic for mixed cooperative-competitive environments. _Adv. Neural Inf. Process. Syst._ **30** (2017). 17. Whiteson, S. Counterfactual Multi− Agent Policy Gradients. (2018). 

18. Nakamoto, S. Bitcoin: A peer-to-peer electronic cash system. (2008). 

19. Buterin, V. A next-generation smart contract and decentralized application platform. _White Paper._ **3** , 2–1 (2014). 

20. Zheng, Z., Xie, S., Dai, H.-N., Chen, X. & Wang, H. Blockchain challenges and opportunities: A survey. _Int. J. Web Grid Serv._ **14** , 352–375 (2018). 

21. Chen, J. _et al._ Heterogeneous Riemannian few-shot learning network. _IEEE Trans. Neural Netw. Learn. Syst._ (2025). 

22. Nisan, N., Roughgarden, T., Tardos, E. & Vazirani, V. Combinatorial auctions. In _Algorithmic Game Theory_ , 267–298 (2007). 

~~ON~~ **Scientific Reports** |        (2025) 15:42841 | https://doi.org/10.1038/s41598-025-20247-8 natureportfolio 16 

www.nature.com/scientificreports/ 

23. Stiglitz, J. E. & Kosenko, A. _The Economics of Information in a World of Disinformation: A Survey Part 1: Indirect Communication_ (National Bureau of Economic Research, 2024). 

24. Wang, Y. _et al._ Meta-DDA: Meta-learning with diffusion and dual augmentation for few-shot text classification. _Knowl.-Based Syste._ 114179 (2025). 

25. Myerson, R. B. Optimal auction design. _Math. Oper. Res._ **6** , 58–73 (1981). 

26. Feigenbaum, J. & Shenker, S. In _Current Trends in Theoretical Computer Science: The Challenge of the New Century Vol 1: Algorithms and Complexity Vol 2: Formal Models and Semantics_ , 403–434 (World Scientific, 2004). 

27. Wang, Z., Hu, Q., Li, R., Xu, M. & Xiong, Z. Incentive mechanism design for joint resource allocation in blockchain-based federated learning. _IEEE Trans. Parallel Distrib. Syst._ **34** , 1536–1547 (2023). 

28. OroojlooyJadid, A. & Hajinezhad, D. A review of cooperative multi-agent deep reinforcement learning. arXiv preprintarXiv:1908.03963 (2019). 

29. Hua, M. _et al._ Multi-agent reinforcement learning for connected and automated vehicles control: Recent advancements and future prospects. arXiv preprintarXiv:2312.11084 (2023). 

## **Author contributions** 

K.T. conceived the research idea; designed the blockchain-enhanced incentive-compatible framework; developed the smart contracts and the MASAC integration; implemented all software and simulation environments; designed and ran the automated market bidding and intelligent traffic control experiments; performed ablation and robustness analyses; created all figures and tables; interpreted results; wrote and revised the manuscript; and approved the final version. K.T. solely conceived and designed the study, performed the experiments, analyzed the data, and wrote the manuscript. 

## **Funding** 

This research received no specific grant from any funding agency in the public, commercial, or not-for-profit sectors. 

## **Declarations** 

## **Competing interests** 

The authors declare no competing interests. 

## **Additional information** 

**Correspondence** and requests for materials should be addressed to K.T. 

**Reprints and permissions information** is available at www.nature.com/reprints. 

**Publisher’s note** Springer Nature remains neutral with regard to jurisdictional claims in published maps and institutional affiliations. 

**Open Access** This article is licensed under a Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License, which permits any non-commercial use, sharing, distribution and reproduction in any medium or format, as long as you give appropriate credit to the original author(s) and the source, provide a link to the Creative Commons licence, and indicate if you modified the licensed material. You do not have permission under this licence to share adapted material derived from this article or parts of it. The images or other third party material in this article are included in the article’s Creative Commons licence, unless indicated otherwise in a credit line to the material. If material is not included in the article’s Creative Commons licence and your intended use is not permitted by statutory regulation or exceeds the permitted use, you will need to obtain permission directly from the copyright holder. To view a copy of this licence, visit  h t t p : / / c r e a t i v e c o m m o n s . o r g / l i c e n s e s / b y - n c - n d / 4 . 0 / . 

© The Author(s) 2025 

**Scientific Reports** |        (2025) 15:42841 

17 

| https://doi.org/10.1038/s41598-025-20247-8 

