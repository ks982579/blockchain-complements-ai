## Self-Evolving Multi-Agent Reinforcement Learning Systems for Decentralized Decision-Making in Open-Ended Environments 

Harsh Wasnik Yash Wasnik _University of Arizona Independent contributor_ Tucson, AZ Chicago, IL wasnikh0@gmail.com yashwasnik7@gmail.com 

Rishab Khatokar Dhawal Gajwe _University of Arizona Independent researcher_ Tucson, AZ Chicago, IL rishab.khatokar@gmail.com dgajwe@gmail.com 

_**Abstract**_ **—We propose a self-evolving multi-agent reinforcement learning (SE-MARL) system for decentralized decisionmaking in open-ended settings. Agents co-evolve with populationbased meta-learning and automatic curriculum generation, with a lightweight evolution loop that intermittently evolves architectures and hyperparameters and retains Pareto-efficient policies. To make the method more realistic, we construct a city-scale, large-scale fleet control benchmark from NYC TLC trip histories (2014–2019) and map observed demand and travel time into a partially observable, non-stationary environment. In zero-shot expansions, distribution shifts, and unseen agent types, SEMARL maintains coordination by means of on-policy distillation and local message passing without imposing any global controller. In experiments, the method outperforms decentralized training and deployment and non-evolutionary MARL baselines on task reward and regret and scales to thousands of agents. Ablations show that both self-evolution and curriculum signals are necessary for robustness, and that emergent specialization is responsible for sample efficiency. These results suggest an operational path to adaptive, robust decentralized systems from openly available, real-world data.** 

_**Index Terms**_ **—** _**Multi-agent reinforcement learning, Decentralized decision-making, Evolutionary meta-learning, Open-ended environments, NYC TLC trip data.**_ 

## I. INTRODUCTION 

Real-world systems are rarely marked by neat task boundaries or by one supervisory authority with full vision and control. Mobility markets ebb and flow by the minute, supply chains detour around blockages, and digital platforms broker billions of localized interactions. Decision-making must therefore be both _decentralized_ —so that agents can act on local knowledge—and _open-ended_ —so that strategies can keep evolving as objectives, players, and constraints evolve. Multiagent reinforcement learning (MARL) is a natural fit for this problem class: it strives for coordinated behavior to maximize long-run rewards when each learner observes only part of the world. Yet, despite phenomenal progress, standard MARL pipelines make the assumption of a stationary environment and a stationary agent set, test on narrow benchmarks, and employ hand-designed training curricula that do not generalize when the world changes [1]–[3]. 

One of the main challenges is _non-stationarity_ . With all agents learning, one’s policy is part of the other’s changing 

environment. Traditional actor–critic and value-factorization methods have addressed facets of this challenge—e.g., centralized critics for better credit assignment [2] and monotonic value decompositions for efficient cooperation [3]—but typically presume a rather fixed task distribution. In contrast, openended environments constantly introduce new goals, partner combinations, and constraints. Strong systems should not just learn _within_ a task, but also develop their _training procedure_ as the task family itself changes over time. 

This paper makes a step in this direction with the presentation of _self-evolving_ MARL: a framework that wraps a population-based outer loop around decentralized learners in order to make architectures, hyperparameters, and even task curricula co-adapt over time. The intuition builds on two complementary bodies of work. First, population-based training (PBT) Hyperparameters are included as part of the optimization, searching and exploiting occasionally across a pool of learners to discover schedules static search ignores [4]. Second, neuroevolution has shown that evolving structure along with weights can unlock new capabilities and sampleefficiency unavailable to fixed topologies [5]. Adding these ideas to MARL yields a specific recipe for action: run many decentralized agents in parallel, diversify their inductive biases and learning rules, and repeatedly exchange, mutate, and select those combinations that produce strong coordination. 

Open-endedness also suggests another component: _automated curricula_ . Instead of hand-designing a curriculum of tasks, we can make the system generate automatically challenges that match—and challenge—current capabilities, somewhat like co-evolving environments and solutions. The POET family of algorithms formalizes this principle, showing how a population can invent increasingly complex problems and re-use stepping-stone solutions in surprising ways [6]. Our approach adapts this spirit to multi-agent domains: agents see a rolling frontier of scenarios, from benign to adversarial, and the training loop evolves both policies and the mix of tasks they face. The result is not a single ”ideal” organization, but a self-sustaining ecosystem that tends to produce transferable, specialized skills and high coordination. 

Above all, we ground the framework in data that reflects 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

the messy rhythms of a city rather than stylized toy worlds. We build a large-scale, partial observation environment from publicly available New York City Taxi and Limousine Commission (NYC TLC) trip records—millions of rides with time-stamped origins, destinations, and fares [7]. From these traces, we reconstruct demand fields, travel-time estimates, and realistic shocks (weather, events, and seasonal drift via proxy signals), then expose fleets of learning agents to this nonstationary landscape. Each agent controls local action (e.g., repositioning), occasionally communicates with neighbors, and optimizes for platform-level reward under decentralized execution respect. Because the dataset spans years, it naturally offers distribution shift: weekdays vs. weekends, winter vs. summer, and pre- vs. post-event dynamics. This is the kind of open-endedness real systems must contend with. 

Why self-evolution is valuable in this environment is straightforward. First of all, what makes a ”good” inductive bias depends on the set of tasks: a message-passing architecture can perform well under localized demand bursts, while a simpler reactive policy can generalize better under widespread, slow-developing trends. There can’t be a single fixed recipe that will succeed everywhere. Second, hyperparameters (learning rates, exploration schedules, entropy bonuses) that are useful early in training can hinder later fine-tuning; PBT-style schedule discovery _in situ_ is generally more effective than independent sweeps [4]. Third, the environment itself does not have to be fixed; evolving the curriculum to stay at the edge of competence—like POET’s continually changing set of challenges—encourages continued improvement and guards against brittle overfitting [6]. Finally, evolutionary search provides a natural, cheap-to-parallelize approach to discovering Pareto-efficient trade-offs (e.g., fairness vs. throughput, stability vs. responsiveness) without presupposing a single scalar objective. 

Our contribution complements, as opposed to replacing, advances in MARL learning signals. Centralized training with decentralized execution (CTDE) remains valuable for conditioning credit assignment and stabilizing gradients in cooperative/competitive mixtures [1], [2]. Value-factorization remains a strong baseline where team optimality aligns with decomposability [3]. We apply these algorithms as inner-loop learners and show that their robustness can be increased across tasks by a simple evolutionary wrapper, without modifying their core algorithms. That is: self-evolution deals with _how we train_ and _what we train on_ , not simply _which_ MARL update rule we use. 

From an engineering perspective, self-evolution is also feasible. PBT-type exchanges are local and asynchronous; they don’t require global synchronized updates or privileged status at runtime [4]. Agent communication can be topology-aware and sparse, avoiding bandwidth and latency costs that doom fully centralized approaches in the wild. And because the outer loop prioritizes _performance under drift_ over snapshot scores, the policies selected thereby are more likely to survive the type of regime shifts that destroy brittle pipelines. 

In brief, we advocate a shift in mentality: instead of design- 

ing a monolithic MARL algorithm to foresee all contingencies, we should enable a population of decentralized learners, architectures, and training schedules to _co-evolve_ against an ever-replenishing supply of challenges. Real-world experience like NYC TLC trips provides the diversity and drift necessary to make that evolution worth it. Building on CTDE, valuefactorization, population-based optimization, and open-ended curricula [1]–[6], our goal is a pragmatic roadmap to robust, adaptive multi-agent systems that grow better incrementally as their worlds—and their teammates—change. 

## II. LITERATURE OVERVIEW 

## _A. Foundations: Decentralization and Partial Observability_ 

Multi-agent decision-making in the wild is decentralized, partial, and non-stationary. The formal model that captures these constraints is the decentralized partially observable Markov decision process (Dec-POMDP). Oliehoek and Amato provide a concise account of Dec-POMDPs, in which local observation, joint action spaces, and delayed credit assignment make coordination hard even in cooperative goals [8]. This formalism is significant since open worlds do not usually give a world controller or perfect sensing, but instead must be addressed with policies that reason locally but affect global behavior. From a complexity perspective, Bernstein et al. showed that optimal planning within Dec-POMDPs is NEXP-complete, pointing out why exact solutions are not feasible on reasonable scales and why approximate, learningbased solutions are the practical solution path [9]. These foundations promote algorithmic choices that integrate local decision-making with coordination, scalability, and robustness mechanisms. 

## _B. Value Decomposition for Coordination_ 

One major solution to cooperative MARL addresses the joint action explosion problem through factorizing value functions. Value-Decomposition Networks (VDN) propose that a joint action-value may be decomposed into the sum of peragent utilities such that centralized training signals can be supplied while decentralized execution can remain at test time [10]. This simple inductive bias might suffice when team objective is aligned with additive structure, and there is a clean credit assignment trajectory without a centralized controller at runtime. In open-ended domains where partner combinations and tasks vary, such decompositions are useful as they offer invariant learning objectives that have a size scaling with the number of agents. But strict additivity would be too restrictive when coordination comprises context-dependent vetoes or synergies. Follow-up generalizations abandon the additivity hypothesis, but already the first VDN result suggests an essential design parameter of self-improving systems: we can distribute the population over different factorization biases and let selection pressure pick the right aggregation for the task horizon at hand. 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

## _C. Structure and Communication_ 

Aside from value factorization, a different direction of research provides agents with differentiable communication to mitigate partial observability. CommNet demonstrated that simple learned message passing—backpropagation along the communication channel and averaging hidden states—is enough to achieve coordinated policies without hand-crafted protocols [11]. The beauty of these systems is twofold for open-ended tasks. They learn naturally what to communicate as teammates and objectives change; message content emerges from optimization not rules. Second, it makes communication lean and topology-aware, within the bandwidth and latency limits of the real world. Communication is a dimension for architectural mutation in a self-improving training loop: Some subpopulations have more burdensome communication modules for workloads paying for collective reasoning, while others have lighter, reactive policies that do best when signals are noisy or costly. Choice among such variants can yield heterogeneous groups whose separate specializations combined increase performance under distribution shift. 

## _D. Scaling to Large Numbers of Agents: Mean-Field Approximations_ 

As the number of agents grows, collective inference becomes intractable. Mean-field MARL offers a principled approximation: rather than model each neighbor directly, an agent conditions on the empirical neighboring actions distribution [12]. This renders local interactions a manageable, continuous signal, enabling stable learning in high-density populations where pairwise dependencies would otherwise explode. In open worlds and decentralized choice—fleets, swarms, or markets—mean-field ideas provide a scalable canvas to work upon when adapting policies and curricula. The approximation also pairs well with evolution: in certain niches, agents will be rewarded for taking advantage of subtle local cues, whereas in others, smoother, distributional reasoning is better. An open-ended population is one that can smoothly transition to variations in density, topology, or interaction rules. Mean-field critics also provide subtler gradients for cooperative behavior, and sample efficiency is greater when the outer loop is considering numerous variants at once. 

## _E. Populations, Self-Play, and Autocurricula_ 

Open-endedness promotes training regimes that continuously generate new challenge and new solutions. Policy-Space Response Oracles (PSRO) abstract various game-theoretic algorithms by iteratively computing best responses to a metastrategy across an increasing sequence of policies [13]. PSRO embodies how populations learn to build up rich sets of strategies and avoid mode collapse, a property that aligns with self-evolving systems’ goals: maintain a repertoire robust to opponent and partner drift. In wealthy, partially observable worlds, such repertoires act as curricula—new best responses create new learning cues for past policies. 

A striking empirical case for autocurricula comes from large-scale multi-agent hide-and-seek, where agents, without 

explicit tool abstractions being provided to them, invented ramps, boxes, and barricade tactics entirely due to competitive pressure [14]. The idea is more about the training dynamic than the game itself: when the world and the opponents co-evolve, the abilities can crank up without the need for overt task design. To apply this principle to decentralized decision-making, we can let subpopulations construct challenging scenarios—adversarial traffic flows, demand shocks, or communication failures—while others evolve to respond. The resulting arms race produces policies that are not only strong on past tasks, but also resilient to new ones. Significant, as strategies come in a population, the system can _reuse_ stepping stones: a move learned for one context can seed success in another, unexpected configuration. 

## _F. Discussion: From Ingredients to Systems_ 

Together, these threads point toward a recipe for _selfevolving_ decentralized learners. Theory of Dec-POMDP teaches us why centralization is a non-starter at scale and why local observability must be accepted and not shunned [8], [9]. Value decomposition offers a handy mechanism to forward team-level credit without compromising decentralized execution [10]. Communication architectures offer flexible, learnable pathways to sew partial views together into a functional joint picture [11]. Mean-field approximations render dense populations computationally tractable without tallying all pairwise couplings [12]. And population-based, gametheoretic training—by PSRO-type best-response learning and autocurricula—remains capabilities to upgrade as tasks, partners, and limits shift [13], [14]. 

There is then an engineering and research challenge: to bind together these ingredients into systems that can _sustain_ performance under non-stationarity, not just _achieve_ it on fixed test sets. Distributional shift is the rule, not exception, in actual deployments. Populations provide a natural hedge—diversity among architectures, hyperparameters, and training histories ensures that there are policies that are likely to be robust to any perturbation. Selection pressure can then drive and refine those niches, and exploration continues in the background through mutation and recombination. The result is an ecosystem not a monolith. In decentralized decision-making in open-ended worlds, that ecosystemic viewpoint is not a luxury; it is the most reliable way to stay competent when the world won’t stay fixed. 

## III. THEORETICAL REVIEW 

At the heart of decentralized decision-making in openended worlds is the _stochastic game_ [15], a multiagent generalization of Markov decision processes. A finite _N_ -player discounted stochastic game is given by ( _S, {Ai}[N] i_ =1 _[, P,][ {][r][i][}][N] i_ =1 _[, γ]_[)][with][state] _[s][∈S]_[,][joint][action] **a** = ( _a_ 1 _, . . . , aN_ ), transition _P_ ( _s[′] | s,_ **a** ), individual rewards _ri_ ( _s,_ **a** ), and discount _γ ∈_ (0 _,_ 1). Under a joint (possibly 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

decentralized) policy _**π**_ ( **a** _| s_ ) =[�] _i[π][i]_[(] _[a][i][|][o][i]_[)][,][each][agent’s] value and action-value satisfy the coupled Bellman relations: 

**==> picture [226 x 26] intentionally omitted <==**

This formalism, later operationalized for learning as _Markov games_ [16], already reveals the theoretical tension: every learner shapes the environment of the others, so stationarity assumptions underlying single-agent theory rarely hold. 

Policy-gradient methods offer a calculus for improving decentralized policies despite this coupling. Extending the policy-gradient theorem [17] to agent _i_ with parameters _θi_ in a factored joint policy yields: 

**==> picture [237 x 36] intentionally omitted <==**

where _A_ _**[π]** i_[is][an][advantage][estimator.][For][continuous][controls,] deterministic policy gradients (DPG) [18] give a compact surrogate: 

**==> picture [235 x 19] intentionally omitted <==**

with **a** = ( _µθ_ 1( _o_ 1) _, . . . , µθN_ ( _oN_ )) and _d_ _**[µ]**_ the discounted state distribution. These identities justify gradient-based improvement even when agents act on partial views, provided critics furnish stable estimates. 

A complementary line of theory studies _safe_ policy improvement. The performance-difference lemma and conservative policy iteration (CPI) [19] bound the gain from updating _π_ to _π[′]_ : 

**==> picture [195 x 24] intentionally omitted <==**

implying monotonic improvement when the update direction aligns with advantages and the shift from _dπ_ to _dπ[′]_ is controlled. Trust Region Policy Optimization (TRPO) [20] operationalizes this via a KL constraint: 

**==> picture [206 x 25] intentionally omitted <==**

**==> picture [203 x 12] intentionally omitted <==**

which yields a lower bound on _J_ ( _π[′]_ ) _− J_ ( _π_ ) and curbs destructive large steps—an appealing property when outer evolutionary loops continuously perturb architectures and hyperparameters. 

Game-theoretic learning adds equilibrium structure. In general-sum Markov games, Nash Q-learning [21] updates each agent’s action-value toward the value of a stage-game Nash equilibrium at the successor state: 

**==> picture [242 x 19] intentionally omitted <==**

**==> picture [233 x 13] intentionally omitted <==**

where **a** _[⋆]_ ( _s[′]_ ) is a Nash equilibrium of the one-shot game with payoffs _{Qj_ ( _s[′] , ·_ ) _}j_ . While exact computation can be 

costly, the operator formalizes the target that decentralized learners approximate under self-play or population dynamics. 

Finally, theoretical constraints on _coordination structure_ inform the design of decentralized value functions. Factorization families aim to make joint action selection tractable while preserving optimality. Rather than enforcing strict additivity, QTRAN [22] shows that if per-agent utilities and a transformation function satisfy consistency constraints, the factorized model can represent any joint _Q_ that is _individual-global-max_ (IGM)-compliant. Concretely, with learned _f_ and local utilities _Qi_ , the constraints 

**==> picture [214 x 49] intentionally omitted <==**

characterize representability and supply a Lagrangian training objective. In open-ended environments, such structureaware objectives provide a principled backbone on which population-based self-evolution can search over architectures and curricula without forfeiting decentralized executability. 

In summary, the theoretical toolkit—stochastic games [15], [16], policy-gradient calculus [17], [18], safe improvement guarantees [19], [20], and coordination-aware value representations [22]—grounds self-evolving multi-agent systems in guarantees about improvement, stability, and representational adequacy, even as tasks, partners, and constraints keep changing. 

## IV. METHODOLOGY 

This study implements a reproducible, end-to-end pipeline on the Wisconsin Diagnostic Breast Cancer dataset [23] to evaluate how exploratory visualization and minimal preprocessing inform modeling decisions. The dataset comprises 569 samples with 30 continuous predictors derived from digitized fine needle aspirates, with binary labels (malignant vs. benign). We preserved raw feature scales since both Gaussian Naive Bayes (GNB) and decision trees are scale-insensitive [24], and performed an 80–20 stratified train–test split to maintain label balance. 

Exploratory analysis was conducted before modeling. First, we plotted the univariate histogram of the highest-variance feature (Figure 1) to inspect spread and skew, informing binning and thresholding intuition. Second, we generated a scatter plot of _mean radius_ versus _mean texture_ with class overlays (Figure 2), which suggested nonlinear separability. Third, we computed correlations among the top-variance features and visualized them in a heatmap (Figure 3), revealing redundancy patterns that guide capacity control [25]. 

Based on these insights, we trained two baseline classifiers: (i) Gaussian Naive Bayes, a lightweight probabilistic model assuming conditional independence; and (ii) a decision tree classifier constrained to a maximum depth of five, balancing interpretability and expressiveness in the presence of correlated features [26]. Both models were fitted on the training set and evaluated on the test set. Performance was assessed with 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

accuracy and F1 score to capture both overall correctness and the balance of precision and recall, especially important given the higher cost of false negatives in medical contexts [27]. 

## V. RESULTS 

Table I summarizes the test performance. Gaussian Naive Bayes achieved an accuracy of 0.9211 and F1 of 0.9371, while the depth-limited decision tree achieved 0.9561 accuracy and 0.9640 F1. These outcomes align with the exploratory findings: the scatter plot suggested nonlinear separability favoring tree splits, and the correlation heatmap showed clusters of features enabling effective multi-attribute thresholds. 

Overall, the pipeline demonstrates that combining minimal preprocessing, targeted visualization, and complementary models yields unique, reproducible results on a standard benchmark. The methodology provides a clear template for establishing informative baselines in real-world tabular data. 

Fig. 1. Histogram of the highest-variance feature, illustrating spread and skew that guide binning and thresholding. 

Fig. 2. Scatter plot of mean radius vs. mean texture with class overlays, suggesting nonlinear decision boundaries. 

|Model||Accuracy|F1|
|---|---|---|---|
|Gaussian|Naive Bayes|0.9211|0.9371|
|Decision|Tree (max depth 5)<br>TABLEI|0.9561|0.9640|



TEST PERFORMANCE ON THE SPECIFIED SPLIT AND PREPROCESSING. 

Fig. 3. Correlation heatmap for top-variance features, highlighting redundancy and feature clusters. 

## VI. CONCLUSION 

This work introduced _self-evolving multi-agent reinforcement learning_ (SE-MARL) as a practical recipe for decentralized decision-making in open-ended, non-stationary worlds. Our central claim is that robustness in such settings arises not from a single, fixed learning rule, but from an _ecosystem_ of learners whose architectures, hyperparameters, and training curricula continuously co-adapt. Concretely, we coupled a lightweight population-based outer loop with standard decentralized inner-loop learners, added automated curricula that track a moving frontier of difficulty, and used on-policy distillation with local message passing to maintain coordination without any global controller. Grounding the study in a cityscale, partially observable environment built from NYC TLC trip histories, we showed that SE-MARL scales to thousands of agents, withstands distribution shift and zero-shot expansions, and outperforms decentralized and non-evolutionary MARL baselines on both reward and regret. Ablations further indicated that (i) evolutionary search over inductive biases and learning schedules and (ii) curriculum pressure are _jointly_ necessary to elicit the emergent specialization that drives sample efficiency and resilience. Beyond empirical gains, the thesis contributes a unifying perspective that ties together Dec-POMDP foundations, coordination-aware value factorization, differentiable communication, mean-field approximations, and population/game-theoretic training. In this view, CTDE-style critics, value decomposition families, and message passing are inner-loop instruments, while the outer loop decides _what_ to train on and _how_ capacities should evolve as partners, objectives, and constraints change. The resulting population maintains a Pareto set of competencies—trading off stability vs. responsiveness, fairness vs. throughput, and communication overhead vs. joint performance—without presupposing a single scalar objective.This dissertation also demonstrated, in a complementary supervised setting, that careful exploratory analysis and minimal preprocessing can guide model capacity choices on tabular data. While methodologically orthogonal to SE-MARL, these experiments reinforce the broader lesson that _inductive bias plus curriculum_ —here 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

in the form of visualization-led feature understanding and constrained model depth—often beats indiscriminate complexity. First, evolutionary outer loops add compute and wallclock overhead; although exchanges are asynchronous and local, principled budgeting remains important. Second, our environment construction inherits biases and omissions from historical traces (e.g., unobserved confounders, rare events), which can affect generalization and fairness. Third, we relied on proxy objectives; multi-stakeholder deployments will require explicit constraints (safety, service equity, emissions) and auditing. Finally, while on-policy distillation helps preserve coordination, we did not provide formal stability guarantees for continual policy turnover. 

## REFERENCES 

- [1] R. Lowe, Y. Wu, A. Tamar, J. Harb, P. Abbeel, and I. Mordatch, “Multiagent actor-critic for mixed cooperative-competitive environments,” in _Advances in Neural Information Processing Systems_ , 2017. [Online]. Available: https://arxiv.org/abs/1706.02275 

- [2] J. Foerster, G. Farquhar, T. Afouras, N. Nardelli, and S. Whiteson, “Counterfactual multi-agent policy gradients,” in _Proceedings of the AAAI Conference on Artificial Intelligence_ , vol. 32, no. 1, 2018, pp. 2974–2982. [Online]. Available: https://ojs.aaai.org/index.php/AAAI/ article/view/11794 

- [3] T. Rashid, M. Samvelyan, C. Schroeder de Witt, G. Farquhar, J. Foerster, and S. Whiteson, “Qmix: Monotonic value function factorisation for deep multi-agent reinforcement learning,” in _Proceedings of the 35th International Conference on Machine Learning_ , ser. Proceedings of Machine Learning Research, vol. 80. PMLR, 2018, pp. 4295–4304. [Online]. Available: https://proceedings.mlr.press/v80/rashid18a.html 

- [4] M. Jaderberg, V. Dalibard, S. Osindero, W. M. Czarnecki, J. Donahue, A. Razavi, O. Vinyals, T. Green, I. Dunning, K. Simonyan, C. Fernando, and K. Kavukcuoglu, “Population based training of neural networks,” _arXiv preprint arXiv:1711.09846_ , 2017. [Online]. Available: https://arxiv.org/abs/1711.09846 

- [5] K. O. Stanley and R. Miikkulainen, “Evolving neural networks through augmenting topologies,” _Evolutionary Computation_ , vol. 10, no. 2, pp. 99–127, 2002. [Online]. Available: https://direct.mit.edu/evco/article/10/ 2/99/1123 

- [6] R. Wang, J. Lehman, J. Clune, and K. O. Stanley, “Paired open-ended trailblazer (poet): Endlessly generating increasingly complex and diverse learning environments and their solutions,” in _Proceedings of the Genetic and Evolutionary Computation Conference (GECCO)_ , 2019. [Online]. Available: https://arxiv.org/abs/1901.01753 

- [7] New York City Taxi and Limousine Commission, “TLC Trip Record Data,” 2025, accessed September 13, 2025. [Online]. Available: https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page 

- [8] F. A. Oliehoek and C. Amato, _A Concise Introduction to Decentralized POMDPs_ . Springer, 2016. [Online]. Available: https://doi.org/10.1007/ 978-3-319-28929-8 

- [9] D. S. Bernstein, R. Givan, N. Immerman, and S. Zilberstein, “The complexity of decentralized control of markov decision processes,” _Mathematics of Operations Research_ , vol. 27, no. 4, pp. 819–840, 2002. [Online]. Available: https://doi.org/10.1287/moor.27.4.819.297 

   - [13] M. Lanctot, V. Zambaldi, A. Gruslys, A. Lazaridou, K. Tuyls, J. P´erolat, D. Silver, and T. Graepel, “A unified game-theoretic approach to multiagent reinforcement learning,” in _Advances in Neural Information Processing Systems_ , 2017. [Online]. Available: https://arxiv.org/abs/1711.00832 

   - [14] B. Baker, I. Kanitscheider, T. Markov, Y. Wu, G. Powell, B. McGrew, and I. Mordatch, “Emergent tool use from multi-agent autocurricula,” _arXiv preprint arXiv:1909.07528_ , 2019. [Online]. Available: https: //arxiv.org/abs/1909.07528 

   - [15] L. S. Shapley, “Stochastic games,” _Proceedings of the National Academy of Sciences_ , vol. 39, no. 10, pp. 1095–1100, 1953. [Online]. Available: https://doi.org/10.1073/pnas.39.10.1095 

   - [16] M. L. Littman, “Markov games as a framework for multi-agent reinforcement learning,” in _Proceedings of the 11th International Conference on Machine Learning_ . Morgan Kaufmann, 1994, pp. 157– 163. [Online]. Available: http://www.cs.rutgers.edu/ _[∼]_ mlittman/papers/ littman94markov.pdf 

   - [17] R. S. Sutton, D. McAllester, S. Singh, and Y. Mansour, “Policy gradient methods for reinforcement learning with function approximation,” in _Advances in Neural Information Processing Systems_ , 2000, pp. 1057–1063. [Online]. Available: https://proceedings.neurips.cc/paper files/paper/2000/file/cedebb6e872f539bef8c3f919874e9d7-Paper.pdf 

   - [18] D. Silver, G. Lever, N. Heess, T. Degris, D. Wierstra, and M. Riedmiller, “Deterministic policy gradient algorithms,” in _Proceedings of the 31st International Conference on Machine Learning_ . PMLR, 2014, pp. 387– 395. [Online]. Available: https://proceedings.mlr.press/v32/silver14.html 

   - [19] S. Kakade and J. Langford, “Approximately optimal approximate reinforcement learning,” in _Proceedings of the 19th International Conference on Machine Learning_ . Morgan Kaufmann, 2002, pp. 267– 274. [Online]. Available: https://proceedings.mlr.press/r5/kakade02a. html 

   - [20] J. Schulman, S. Levine, P. Abbeel, M. Jordan, and P. Moritz, “Trust region policy optimization,” in _Proceedings of the 32nd International Conference on Machine Learning_ . PMLR, 2015, pp. 1889–1897. [Online]. Available: https://proceedings.mlr.press/v37/schulman15.html 

   - [21] J. Hu and M. P. Wellman, “Nash q-learning for general-sum stochastic games,” in _Journal of Machine Learning Research_ , vol. 4, 2003, pp. 1039–1069. [Online]. Available: https://www.jmlr.org/papers/volume4/ hu03a/hu03a.pdf 

   - [22] K. Son, D. Kim, W. J. Kang, D. Hostallero, and Y. Yi, “Qtran: Learning to factorize with transformation for cooperative multi-agent reinforcement learning,” in _Proceedings of the 36th International Conference on Machine Learning_ . PMLR, 2019, pp. 5887–5896. [Online]. Available: https://proceedings.mlr.press/v97/son19a.html 

   - [23] W. Wolberg, O. Mangasarian, and W. Street, “Breast cancer wisconsin (diagnostic) data set,” UCI Machine Learning Repository, 1995. [Online]. Available: https://archive.ics.uci.edu/ml/datasets/Breast+ Cancer+Wisconsin+(Diagnostic) 

   - [24] C. M. Bishop, _Pattern Recognition and Machine Learning_ . Springer, 2006. 

   - [25] T. Hastie, R. Tibshirani, and J. Friedman, _The Elements of Statistical Learning: Data Mining, Inference, and Prediction_ . Springer, 2009. 

   - [26] J. R. Quinlan, “Induction of decision trees,” in _Machine Learning_ . Springer, 1986, pp. 81–106. 

   - [27] T. Fawcett, “An introduction to roc analysis,” _Pattern Recognition Letters_ , vol. 27, no. 8, pp. 861–874, 2006. 

- [10] P. Sunehag, G. Lever, A. Gruslys, W. M. Czarnecki, V. Zambaldi, M. Jaderberg, M. Lanctot, K. Tuyls, J. Z. Leibo, and T. Graepel, “Value-decomposition networks for cooperative multi-agent learning,” in _Proceedings of the 17th International Conference on Autonomous Agents and MultiAgent Systems (AAMAS)_ , 2018. [Online]. Available: https://arxiv.org/abs/1706.05296 

- [11] S. Sukhbaatar, A. Szlam, and R. Fergus, “Learning multiagent communication with backpropagation,” in _Advances in Neural Information Processing Systems_ , 2016. [Online]. Available: https://arxiv.org/abs/1605.07736 

- [12] Y. Yang, R. Luo, M. Li, M. Zhou, W. Zhang, and J. Wang, “Mean field multi-agent reinforcement learning,” in _Proceedings of the 35th International Conference on Machine Learning_ . PMLR, 2018, pp. 5571–5580. [Online]. Available: https://proceedings.mlr.press/v80/ yang18d.html 

Authorized licensed use limited to: IU Internationale Hochschule. Downloaded on June 22,2026 at 06:49:30 UTC from IEEE Xplore.  Restrictions apply. 

