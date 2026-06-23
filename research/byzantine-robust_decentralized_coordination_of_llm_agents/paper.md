## Byzantine-Robust Decentralized Coordination of LLM Agents 

## Yongrae Jo 

_Department of Computer Science and Engineering Pohang University of Science and Technology_ Pohang, Republic of Korea memex@postech.ac.kr 

## Chanik Park 

_Department of Computer Science and Engineering Pohang University of Science and Technology_ Pohang, Republic of Korea cipark@postech.ac.kr 

_**Abstract**_ **—Collaboration among multiple large language model (LLM) agents is a promising approach to overcome inherent limitations of single-agent systems, such as hallucinations and single points of failure. As LLM agents are increasingly deployed on open blockchain platforms, multi-agent systems capable of tolerating malicious (Byzantine) agents have become essential. Recent Byzantine-robust multi-agent systems typically rely on leader-driven coordination, which suffers from two major drawbacks. First, they are inherently vulnerable to targeted attacks against the leader. If consecutive leaders behave maliciously, the system repeatedly fails to achieve consensus, forcing new consensus rounds, which is particularly costly given the high latency of LLM invocations. Second, an underperforming proposal from the leader can be accepted as the final answer even when higher-quality alternatives are available, as existing methods finalize the leader’s proposal once it receives a quorum of votes. To address these issues, we propose DecentLLMs, a novel decentralized consensus approach for multi-agent LLM systems, where worker agents generate answers concurrently and evaluator agents independently score and rank these answers to select the best available one. This decentralized architecture enables faster consensus despite the presence of Byzantine agents and consistently selects higher-quality answers through Byzantine-robust aggregation techniques. Experimental results demonstrate that DecentLLMs effectively tolerates Byzantine agents and significantly improves the quality of selected answers.** 

_**Index Terms**_ **—Blockchain, Multi-Agent, Large Language Model** 

## I. INTRODUCTION 

LLMs [1]–[6] have demonstrated remarkable performance across a wide range of natural language processing tasks such as reasoning [7], summarization [8], translation [9], coding [10], smart contract auditing [11], anomaly detection [12], medical diagnosis [13] and more. One promising direction to enhance the capabilities of LLMs is through multi-agent systems [7], [9], [14]–[18], in which multiple LLM-based agents collaborate to overcome the limitations of a single LLM, such as a single point of failure and hallucinations. In multi-agent systems, these agents jointly solve problems, improving answer quality through quorum-based voting or multi-round debates. 

Recently, open blockchain networks have seen the emergence of Byzantine-robust multi-agent systems [19], [20] to tolerate Byzantine failures of individual LLM agents. This 

**==> picture [253 x 120] intentionally omitted <==**

**----- Start of picture text -----**<br>
200<br>Agent 1<br>150 Agent 2<br>Agent 3<br>100 Agent 4 lead er<br>Agent 5<br>50 Agent 6<br>Agent 7<br>0<br>0 1 2 3 0 25 50 75<br>#Consecutive Byzantine Leaders Score<br>(a) Increasing Consensus (b) Accepting Underperform-<br>Latency with Byzantine ing Leader’s Answer<br>Leaders<br>Consensus Latency (s)<br>**----- End of picture text -----**<br>


Fig. 1. Motivating Examples: Leader-based consensus is vulnerable to Byzantine leaders and can finalize underperforming leader’s answer due to quorum-based (e.g., majority) voting. 

is because, in a multi-agent system, a Byzantine agent can deliberately sabotage collaboration among agents, leading to poor-quality answers or increased latency during debate-style voting. Existing Byzantine-robust multi-agent systems typically employ a leader-driven architecture, where a designated leader agent is responsible for coordinating the others to output a consensus answer. However, this architecture is inherently vulnerable to targeted attacks on the leader agent, which can significantly degrade the overall system performance. For example, BlockAgents [19] designates a leader who first mines a block and then initiates a multi-round, debate-style voting process to obtain a majority of votes and reach consensus on the proposed block. If the leader is malicious and fails to secure a majority of votes within a preconfigured number of rounds, the protocol restarts the consensus process with a new leader. Trusted MultiLLMN [20] follows a HotStuff [21]-like leader-rotation consensus mechanism that requires a two-thirds quorum of votes to succeed and designates a different leader in each round. If the designated leader is Byzantine, the system restarts the consensus process in the next round with the new leader’s proposal. 

We identify two problems in such leader-based protocols. First, consensus latency can increase significantly due to consecutive Byzantine leaders. In these systems, consensus is attempted on a single leader’s proposal each round. If the leader is Byzantine or submits a low-quality answer that fails 

**==> picture [220 x 96] intentionally omitted <==**

**----- Start of picture text -----**<br>
round 1 round 2 round 3 … round Rmax<br>Agent 1<br>Agent 2<br>Agent 3<br>…<br>Agent N<br>**----- End of picture text -----**<br>


**==> picture [253 x 28] intentionally omitted <==**

**----- Start of picture text -----**<br>
(a) Single Answer Evaluation (b) Multiple Answers Eval-<br>Consensus in a Round uation Consensus in a<br>Round<br>**----- End of picture text -----**<br>


Fig. 2. Comparisons with leader-based and leader-less approaches in Byzantine-robust multi-agent systems. 

to obtain a quorum, the round does not reach consensus and must be rerun with a new leader. In the worst case, multiple consecutive Byzantine leaders can dramatically increase latency, as illustrated in Fig. 1a. Second, the leader’s proposal may not be the best answer available. A quorum-based vote (e.g., majority or two-thirds) on a single leader’s proposal can finalize an underperforming answer. For example, in Fig. 1b, seven LLM agents generate responses with varying quality scores. Although better proposals exist, if Agent 4 is elected leader, its underperforming answer can be accepted through majority voting (i.e., votes from Agents 4-7), even though higher-quality alternatives are available. 

To address these limitations, we adopt a leaderless approach in which multiple answers are evaluated in parallel within a single round, and the highest-quality answer is selected. Unlike leader-based approaches that vote on one leader’s answer per round, this method evaluates all participants’ answers simultaneously, thereby mitigating the risk of poor-quality outputs and reducing consensus latency even in the presence of Byzantine agents. We illustrate the differences between leaderbased and leaderless approaches in Fig. 2a and Fig. 2b. 

To this end, we propose DecentLLMs, a decentralized multiagent system that employs a leaderless consensus architecture: worker agents generate answers in parallel, and evaluator agents score them to select the highest-quality answer. This architecture enables all agents within each role to participate equally in the consensus process. To achieve Byzantine-robust score aggregation by evaluator agents, DecentLLMs adopts the Geometric Median (GM) algorithm [22], which remains resilient to Byzantine attacks as long as an honest majority of evaluator agents exist. 

## II. BACKGROUND AND RELATED WORKS 

## _A. Multi LLM Agents Cooperation_ 

An LLM agent excels at generating fluent natural language, but it is also known to suffer from serious limitations such as hallucinations [23]. For example, an agent may fabricate facts that were never in its training data, make logical inconsistencies, stray from the original question (loss of contextual relevance), or exhibit bias toward early tokens. Multi-LLM 

(a) Multi Round Fixed Leader-driven Debate Style Consensus (e.g., [19]) 

**==> picture [231 x 125] intentionally omitted <==**

**----- Start of picture text -----**<br>
round 1 round 2 round 3 … round N<br>Agent 1<br>… … … …<br>WIZ AA<br>Agent 2<br>… … … …<br>VINA<br>Agent 3<br>IA<br>… … … … …<br>VVNZEN<br>Agent N<br>(b) Multi Round Rotating Leader-driven Quality Consen-<br>sus (e.g., [20])<br>**----- End of picture text -----**<br>


Fig. 3. Operations of Existing Leader-Based Consensus in Byzantine-Robust Multi-Agent Systems. 

agents have emerged as a promising way to address these issues [7], [9], [14]–[17]. By running several LLM instances concurrently, the agents exchange answers, critique or debate one another, and iteratively refine their responses. Through peer feedback and quorum-based voting, they can produce higher-quality answers and reduce hallucinations. 

## _B. Malicious Byzantine Agents_ 

Existing multi-agent LLM frameworks do not take Byzantine behavior into account; most assume that errors are merely unintentional hallucinations. Yet, as AI agents are increasingly deployed in open peer-to-peer environments (e.g., decentralized GPU networks on public blockchains [24]–[26]), Byzantine agents that can threaten the protocol are expected to proliferate. A Byzantine agent can inject inaccurate or nonsensical answers, disrupt cooperative protocols, and mislead end-users via backdoor attacks [27], [28]. Therefore, for multi-LLM agent systems operating in adversarial settings, Byzantinerobust mechanisms for detecting and mitigating such agents are essential. 

## _C. Blockchain-based Byzantine-robust Multi-Agent Systems_ 

BlockAgents [19] and Trusted MultiLLMN [20] are two recent systems that have been proposed to address the challenges of Byzantine agents in multi LLM agent systems. BlockAgents is a framework that coordinates multiple LLM agents to solve a task collaboratively while adding a blockchain layer for tamper-evident, auditable records and incentive mechanisms. At startup, each agent is assigned one of two roles: workers, who propose answers, and evaluators (or miners), 

**==> picture [491 x 146] intentionally omitted <==**

**----- Start of picture text -----**<br>
Answer Generations Answer Evaluations Byzantine-Robust Score Consensus<br>Workers Evaluators Evaluators<br>(| At = (M4, 1 l(a. Nw (aa)= s1 = RobustAggregation({A},Aj, ..., A,Ne °})<br>= 7<br>x ae SF WE | = myV2 +,~,00)}- V¢)t == Eval(a,)Bval(a) (AD ee sy; = RobustAggregation({A,n 42, .., AN°})<br>~f‘ =: ‘ °; Ahy = Ouva = Podhiy = Frat) N “ Sy, == RobustAggrr eg ation({Ay,,,ion({Al, , [Az,] An, ,...,.NwANE<br>4 a 7 At = (V1, =<br>(AP G<br>(os) V2 Ue )R= Eval(a;) en<br>€— prompt aa ve<br>A \ ee S ee EEO_ - pest = BestScore({(a1,51), (2, $2), +(Any :Sip<br>User O " e2 Ain, = (V2 +, Ve), = Eval(ay,,)<br>PROMPT … …<br>‘ … = Ves antsNeyNw. |<br>| ANSWER<br>Record on the<br>!1 de9p eepag | AneA= = (V4, umV2 ve)t"=++, Vc)3°= Bval(a)Eval(az) e Reply  — to User Blockchain<br>**----- End of picture text -----**<br>


Fig. 4. A protocol overview of DecentLLMs. 

who judge those answers and record the results on-chain. When workers generate and submit their responses, evaluators attempt to build a block that contains their assessments and the corresponding rewards. The evaluator who first creates and broadcasts a valid block becomes the leader. The leader’s block proposal then undergoes a multi-round, debate-style process that requires a majority vote by the other evaluators; once consensus is reached, the block is appended to the blockchain. We illustrate the multi-round, fixed-leader, debatestyle consensus mechanism in Fig. 3a. 

Similarly, Trusted MultiLLMN [20] is an LLM-based multiagent system that solves tasks collaboratively. The framework introduces Weighted Byzantine Fault Tolerance (WBFT), replacing the one-agent-one-vote rule with reputation-weighted voting. In each round, the protocol leader initiates a HotStufflike two-phase prepare/commit process [21], during which followers compare the leader’s answer with their own and cast weighted votes only if they deem it to be of better quality. When the cumulative weight in favor of the proposal exceeds two-thirds of the total honest stake, consensus is reached and the block containing the agreed answer is appended to the blockchain, ensuring tamper-evident and auditable records. We also depict the multi-round, rotating-leader, quality-based consensus mechanism in Fig. 3b. 

## III. DECENTLLMS 

## _A. Assumptions and Models_ 

Each agent possesses a public-private key pair that is used to sign and verify messages. Agents communicate over pointto-point links, assuming a synchronous network in which the maximum communication delay between agents is bounded and predetermined. We assume that standard cryptographic primitives are secure and have not been compromised. Each agent in DecentLLMs is classified as either honest or Byzantine. Honest agents faithfully follow the DecentLLMs protocol. Byzantine agents may behave arbitrarily, including the 

generation of extremely poor-quality answers (e.g., inserting advertisements), biased evaluations (e.g., assigning full/zero scores to Byzantine/honest agents, respectively), failing to respond, launching fork attacks (i.e., sending different values to different agents), or even colluding with other Byzantine agents to maximize their influence on the final output. Note that even honest agents may produce relatively poor-quality answers owing to the inherent limitations (i.e., hallucinations) of LLMs; however, they are assumed not to generate overly anomalous answers beyond what would be expected under a known probability distribution. Users are assumed to be honest. 

In DecentLLMs, there are two types of agents. A _worker_ agent _w_ in a worker group _W_ = _{w_ 1 _, w_ 2 _, . . . , wNw }_ , where _Nw_ = _|W |_ , takes a user’s prompt as input and generates answers by invoking an LLM. An _evaluator_ agent _e_ in an evaluator group _E_ = _{e_ 1 _, e_ 2 _, . . . , eNe }_ , where _Ne_ = _|E|_ , evaluates the answers proposed by the worker agents and engages in a decision-making process to select one among them. It is assumed that both the worker group and the evaluator group have an honest majority. Formally, this implies _fw < ⌊[N][w]_ 2 _[−]_[1] _⌋_ and _fe < ⌊[N][e]_ 2 _[−]_[1] _⌋_ , where _fw_ and _fe_ represent the maximum number of tolerable Byzantine agents within the worker and evaluator groups, respectively. 

## _B. Architecture_ 

We explain the design rationale behind the architecture of DecentLLMs. As previously mentioned, DecentLLMs does not rely on a leader agent; instead, multiple agents within each group participate equally in every phase of the protocol. 

First, we adopt a _parallel worker architecture_ . In DecentLLMs, worker agents independently generate responses in parallel, based on the user’s prompt, without communicating with each other. Generating multiple answers concurrently using several LLM instances for the same prompt is known to improve response quality by mitigating hallucinations [15]. 

This design supports horizontal scalability: by allocating additional GPU resources and increasing the number of worker agents, the system can produce more answers simultaneously. 

Second, we employ a _Byzantine-robust evaluation_ process. To identify the highest-quality response among those generated by the worker agents, DecentLLMs utilizes evaluator agents that independently assess each answer. Their evaluations are then aggregated through a Byzantine-robust consensus mechanism to determine the best answer. This ensures that the bestperforming answers are selected by scoring and ranking the worker agents’ responses. Moreover, DecentLLMs leverages the geometric median (GM) algorithm [22] to enhance robustness against Byzantine evaluators. As long as the majority of evaluator agents are honest, the geometric median of their scores reliably supports the selection of the best available answer. 

## _C. Protocol_ 

We now describe the DecentLLMs protocol in detail. The protocol initiates when a user submits a prompt request to DecentLLMs. An overview of this consensus protocol is presented in Fig. 4. We explain the three phases, including _answer generation_ , _answer evaluation_ , and _Byzantine-robust score consensus_ . The protocol proceeds synchronously. 

_1) Answer Generations:_ In this phase, the user broadcasts a prompt (i.e., a transaction request) to the worker agents. Each worker agent _wi_ independently invokes its own LLM instance to generate an answer _ai_ , subsequently broadcasting this answer to evaluator agents. A malicious worker agent may fail to broadcast an answer or may broadcast conflicting answers (i.e., a fork attack). To address such situations, DecentLLMs employs Byzantine reliable broadcast protocols over a synchronous network (e.g., [29]), ensuring that answers from honest worker agents are consistently broadcast to evaluator agents within bounded time. Note that because each worker agent may be equipped with different LLM instances, the latency of answer generation can vary, so the timeout for answer generation should be conservatively set. 

_2) Answer Quality Evaluations:_ Upon receiving the answers from workers, each evaluator agent _ej_ evaluates each answer’s quality using multiple criteria identified in [23]. The criteria include: _factual contradiction_ , measuring the degree to which an answer contradicts established facts; _factual fabrication_ , measuring the degree to which an answer invents or fabricates non-existent facts; _instruction inconsistency_ , measuring the degree to which an answer deviates from or does not properly address the original prompt; _context inconsistency_ , measuring the extent to which an answer deviates from the original context of the question; and _logical inconsistency_ , measuring the degree of incorrect logical argumentation. Each evaluation criterion is scored on a scale from 0 to 20, where scores closer to 20 indicate higher quality, scores closer to 0 indicate lower quality, and a score of 10 indicates neutrality. 

As illustrated in Fig. 4, each evaluator agent _ej_ invokes Eval function for each worker’s answer, computing a quality score for each criterion. These scores are stored as a vector 

_A[j] i_[=][(] _[v]_[1] _[, v]_[2] _[, . . . , v][C]_[)] _[j] i_[,][where][each] _[v]_[represents][the][score] for a criterion, and _C_ is the number of evaluation criteria. In our case, _C_ = 5. If an evaluator agent is Byzantine, it may significantly distort the quality scores by assigning full scores to Byzantine agents’ answers or zero scores to honest workers’ answers. _3) Byzantine-Robust Score Consensus:_ After evaluation, each evaluator agent _ej_ broadcasts its evaluation vectors _{A[j] i[}][N] i_ =0 _[w]_[to][other][evaluators][using][Byzantine][reliable][broad-] cast. Consequently, each evaluator _ej_ obtains a complete set of score vectors for each worker _wi_ ’s answer _ai_ , denoted by _{A_[1] _i[, A]_[2] _i[, . . . , A][N] i[e][}]_[. Each evaluator then applies the geometric] median (GM) algorithm [22] to aggregate the evaluation vectors, some of which may be Byzantine. The GM finds the vector that minimizes the sum of Euclidean distances to all input vectors. DecentLLMs adopts the GM algorithm as follows. Using the formula, each evaluator computes a robustly aggregated score _si_ for each worker _wi_ ’s answer: 

**==> picture [241 x 28] intentionally omitted <==**

Note that we choose the GM algorithm because, compared to other Byzantine-robust aggregation algorithms such as Krum [30] and Bulyan [31], GM offers higher Byzantine resilience. Specifically, GM tolerates up to _f ≤⌊[n][−]_ 2[1] _[⌋]_[Byzantine] vectors, while Krum and Bulyan tolerate up to _f ≤⌊[n][−]_ 2[2] _[⌋]_ and _f ≤⌊[n][−]_ 4[3] _[⌋]_[,][respectively.][Although][the][GM][algorithm][is] known for its high computational complexity, in DecentLLMs this complexity is acceptable because the number of evaluation dimensions _C_ is small (e.g., five in our criteria) and the number of evaluation vectors _Ne_ is relatively manageable. 

After calculating the aggregated scores _si_ for all answers, the evaluators identify the best answer _abest_ as the one with the highest robust score. If there are multiple answers tied for the highest score, _abest_ is selected as the one with the largest hash value among the results obtained by hashing the concatenation of each answer with the most recent block hash. Each evaluator agent then replies to the user with this answer, ensuring the user receives consistent responses from a majority of evaluators. Additionally, each evaluator _ej_ generates and records a transaction onto the blockchain, including the prompt request, worker answers, evaluation results, and their corresponding signatures. These records can later be utilized for reconfiguring the worker and evaluator groups. 

## IV. IMPLEMENTATION 

We outline the implementation of DecentLLMs. To effectively elicit high-quality outputs from LLMs, DecentLLMs employs several well-established prompting strategies, including role-play, chain-of-thought, and few-shot examples, as illustrated in Fig. 5. To efficiently handle numerical outputs from evaluators, DecentLLMs adopts a JSON-format output for easy parsing and aggregation. The geometric median (GM) is computed using Weiszfeld’s algorithm [32], with a maximum of 1000 iterations and a convergence tolerance of 

**==> picture [488 x 295] intentionally omitted <==**

**----- Start of picture text -----**<br>
Input Output<br>## Your Role  // Worker<br>You are a highly knowledgeable and rigorous AI assistant taking a multiple-choice exam. Your goal is to select the<br>best answer only after explaining clear, step-by-step reasoning based on the given question and options.<br>You must explain your reasoning before giving your final answer.<br>Okay, let's break down this question<br>## Output …Therefore, the answer is \\boxed{C}.<br>- First, write a clear, step-by-step explanation of how you arrived at the answer.<br>- Then, end your response on a separate final line with: `\\boxed{<option_letter>}` (e.g., `\\boxed{A}`).<br>Can you answer the following question as accurately as possible? {question}<br>## Your Role // Evaluator<br>- You are a meticulous evaluator whose task is to quantitatively assess each answer written by another worker.<br>## Evaluation Criteria (Score each from 0 to 20):<br>For each criterion, assign a score based on how well the answer satisfies the requirement:<br>- Factual Contradiction: Are there any claims that contradict known or stated facts?<br>… // Other criteria evaluations: [{<br>"agent_id": “worker-1",<br>"scores": {<br>## Examples (Three Workers)<br>"factual_contradiction": 10,<br>Assume that: worker-1 provided a fairly good answer, worker-2 gave a moderate answer, and worker-3 submitted a<br>bad answer. "factual_fabrication": 9,<br>"instruction_inconsistency": 12,<br>The following example shows how to assign graded scores across a spectrum of quality:<br>… // List evaluations scores for each workers "context_inconsistency": 11,<br>"logical_inconsistency": 10<br>}<br>As a careful evaluator, could you please evaluate the following proposal solutions to the question?<br>…. // Other worker’s scores<br>## Given question:<br>]<br>{question}<br>## List of Proposal Solutions:<br>--- Start of Proposal Solution from agent_id: worker-1 ---<br>… // Answer<br>--- End of Proposal Solution from agent_id: worker-1 ---<br>… // Other worker’s answers<br>**----- End of picture text -----**<br>


Fig. 5. Prompts used in DecentLLMs. Role-playing prompting is applied throughout, with additional chain-of-thought prompting for workers and few-shot examples for evaluators. Evaluators produce JSON-structured outputs. Note that italicized text indicates system prompts, while normal text indicates user prompts in input boxes. 

1e-5. Communication between agents is implemented using gRPC. 

## V. EVALUATION 

In this section, we present the experimental setup and results evaluating the effectiveness of DecentLLMs. We experimentally address the following questions: 1) _How much does DecentLLMs improve accuracy compared to existing quorumbased voting methods?_ 2) _How much does DecentLLMs reduce consensus latency compared to leader-based approaches in the presence of Byzantine agents?_ 3) _Does DecentLLMs assign appropriate scores to the generated answers?_ 4) _Does DecentLLMs exhibit sufficient Byzantine resilience?_ 

## _A. Experimental Setups_ 

We prototype DecentLLMs in Python 3.11.11 [33], supporting both worker and evaluator agents. For worker agents, we deploy up to 9 agents using a variety of open-source LLMs, including Qwen3 (0.6B, 1.7B, 4B, 8B, 14B) and Gemma3 (1B, 4B, 12B, 27B). Each worker runs on a dedicated local machine equipped with an AMD Ryzen Threadripper 3990X (2.9 GHz), 256 GB RAM, and either an NVIDIA RTX 3080 or 3080 Ti GPU. For evaluator agents, we employ more capable models, including Claude Sonnet (4, 3.7), Gemini 2.5 Pro, GPT (4o, o3, o4-mini), and Qwen3(235B, 32B). We randomly sample user prompts from the MMLU-Pro benchmark dataset [34], 

which contains a challenging problems for the recent LLM models. 

_Byzantine Agents Simulation:_ To simulate Byzantine workers, we intentionally manipulate the outputs of normal LLMs by inserting advertisements into responses and randomly altering numerical values. Byzantine evaluators are simulated to collude with Byzantine workers by assigning full scores (i.e., 100) to the responses from Byzantine workers and zero scores to those from honest workers, thereby increasing the likelihood that the Byzantine responses are selected as the final outputs. 

## _B. Results_ 

_1) Accuracy Improvements over Quorum-based Voting Methods:_ We present an experimental comparison of DecentLLMs with quorum-based voting methods used in leaderbased protocols, including 2/3-quorum (e.g., [20]) and majority quorum (e.g., [19]), as shown in Fig. 6. For simplicity, in both 2/3-quorum voting and majority quorum voting, we assumed that the leader’s answer proposal corresponds to the _⌈_[(] _[N][w]_ 3[+][1][)] _⌉_ -th and _⌈_[(] _[N][w]_ 2[+][1][)] _⌉_ -th ranked answers among all worker responses, respectively, since these positions would receive sufficient votes from agents with lower-scoring answers. We used 100 random problems from the MMLU-Pro [34] benchmark dataset for evaluation. As a result, DecentLLMs outperforms both quorum-based voting methods, achieving 

**==> picture [219 x 160] intentionally omitted <==**

**----- Start of picture text -----**<br>
80<br>71<br>64<br>60<br>50<br>40<br>20<br>0<br>DecentLLMs 2/3 Majority<br>Number of Correct Answers<br>**----- End of picture text -----**<br>


Fig. 6. Accuracy comparisons of DecentLLMs and other quorum-based voting methods, including 2/3-quorum and majority quorum. We selected 100 random problems from MMLU-Pro [34] benchmark dataset. 

an accuracy of 71 — representing a 7% improvement over 2/3-quorum (64) and a 21% improvement over majority quorum (50), respectively. This performance difference is mainly due to DecentLLMs’ approach of evaluating and ranking all worker answers to select the highest-scoring response as the final output. In contrast, the other two methods select the leader’s underperforming proposal as the final output as long as it receives sufficient quorum votes. 

_2) Consensus Latency:_ We compare the consensus latency of DecentLLMs for a single user request with two leaderdriven baselines, including fixed leader, debate-style consensus (e.g., [19]) and rotating leader-driven quality consensus (e.g., [20]) in terms of the number of Byzantine agents. We simulate their communication patterns as shown in Fig. 2 and DecentLLMs’s consensus latency is measured as the time taken from evaluators all-to-all broadcasting their evaluation results. For the leader-driven approaches we assume the worst case in which Byzantine leaders are selected in successive rounds. In the fixed-leader debate protocol, each leader run at most three consensus rounds. As shown in Fig. 7, DecentLLMs maintains a roughly constant latency of around 221 seconds, regardless of the number of Byzantine agents. In contrast, both leaderdriven approaches exhibit a nearly linear increase in latency as the number of Byzantine agents increases. This difference occurs because DecentLLMs’ decentralized consensus requires only a single round of communication among evaluators, even as the number of Byzantine agents grows. By contrast, the two leader-driven approaches require the system to either (i) rotate to a new leader in the next round (the rotating-leader approach) or (ii) retry with the same leader for up to three rounds (the fixed-leader approach). 

_3) Answer Quality Evaluation:_ We demonstrate a snapshot that illustrates how DecentLLMs operates in a specific example execution, as shown in Fig. V-B2, even in the presence of Byzantine agents. In the table, we show the received scores of each worker’s (i.e., _wi_ ) answers from each evaluator agent (i.e., _ej_ ). The notations _wi[∗]_[and] _[e][∗] j_[denote] 

**==> picture [227 x 170] intentionally omitted <==**

**----- Start of picture text -----**<br>
DecentLLMs<br>4000<br>Leader-driven(Rotation)<br>Leader-driven(Fixed)<br>3000<br>2000<br>1000<br>0<br>0 1 2 3<br>Number of Byzantine Agents (or Leaders)<br>Latency (s)<br>**----- End of picture text -----**<br>


Fig. 7. Consensus Latency of DecentLLMs and Leader-based Approaches. 

Byzantine workers and evaluators, respectively. The last two rows show the robust score of each worker’s answer, calculated by the GM algorithm, and whether each worker’s answer is correct. In the table, we observe that _w_ 7 received the highest robust score of 88 _._ 2, was selected as the final output, and provided the correct answer. The workers are ranked as follows: _w_ 7 _, w_ 6 _, w_ 1 _, w_ 8 _, w_ 2 _, w_ 3 _, w_ 0 _, w_ 4 _, w_ 5 _, w_ 9 _[∗]_[.][The][next][two] high-performing answers, _w_ 6 and _w_ 1, also provided correct answers. However, the the next ranked answer from _w_ 8 was incorrect, indicating that the evaluators failed to reliably score the answers. Additionally, although _w_ 3 received a relatively low score, it still provided the correct answer, which further highlights the evaluators’ reliability issues to assess answers. Although DecentLLMs successfully selected the correct answer in this case, it also highlights the need to improve the reliability of the evaluators’ scoring mechanism as an important direction for future work. 

_4) Byzantine Resilience:_ Next, we evaluate the Byzantine resilience of DecentLLMs in the presence of Byzantine evaluators, as shown in Fig. 8. In this experiment, we extend the previous example by gradually increasing the number of Byzantine evaluators up to seven to observe how the ranking based on robust scores changes under increasing Byzantine influence. In the figure, honest workers are shown as blue circles and Byzantine workers as red circles. The number inside each circle denotes the corresponding worker’s ID. As a result, DecentLLMs, which employs the GM algorithm, successfully selects the correct answer up to the algorithm’s theoretical fault tolerance threshold (i.e., 6). Beyond this point, however, the answer from the Byzantine worker (i.e., _w_ 9 _[∗]_[)] is selected. Importantly, this experiment also implies that the quality of the workers’ answers (and their corresponding evaluation scores) must be sufficiently high to fully leverage the Byzantine fault tolerance threshold of the GM algorithm. As observed in the case with three to six Byzantine evaluators, some honest workers’ answers received lower scores than those from the Byzantine worker, which could potentially prevent the correct answers from being selected in some cases. 

|Worker ID|_w_0|_w_1|_w_2|_w_3|_w_4|_w_5|_w_6|_w_7|_w_8|_w∗_<br>9|
|---|---|---|---|---|---|---|---|---|---|---|
|Evaluator ID|||||||||||
|_e_0 (Sonnet 4)|63|95|92|75|46|50|92|96|95|6|
|_e_1 (Sonnet 3.7)|21|98|99|55|45|41|89|99|100|22|
|_e_2 (Gemini 2.5)|33|100|75|45|40|35|99|100|85|0|
|_e_3 (GPT-4o)|60|80|100|80|50|94|82|82|82|38|
|_e_4 (GPT-o3)|52|83|66|57|44|84|84|83|66|35|
|_e_5 (GPT-o4-mini)|49|53|60|19|42|27|66|66|66|7|
|_e_6 (Qwen3-235B)|31|52|79|49|27|17|79|84|49|34|
|_e_7 (Qwen3-32b)|75|100|80|100|50|64|100|100|100|39|
|_e∗_<br>8 (Byzantine)|0|0|0|0|0|0|0|0|0|100|
|**Robust Score**|**45.1**|**83.8**|**80.4**|**55.6**|**43.5**|**42.0**|**84.5**|**88.2**|**83.0**|**28.5**|
|**Correct Answer**|**X**|**O**|**O**|**O**|**X**|**X**|**O**|**O**|**X**|**X**|



TABLE I 

EVALUATOR’S SCORES ON WORKER ANSWERS. THE NOTATIONS _wi[∗]_[AND] _[ e][∗] j_[DENOTE][A][ B][YZANTINE][WORKER][AND][A][ B][YZANTINE][EVALUATOR][,] RESPECTIVELY. 

**==> picture [227 x 196] intentionally omitted <==**

**----- Start of picture text -----**<br>
100 9<br>6 [7] 1<br>80 82 6 [7] 18 2 6 [7] 18 2 6 [7] 18 2 6 [7] 812 6 [7] 6 [7]<br>60 3 3 28 2 67<br>3 1<br>40 054 05 4 9 0549 3 0 94 [5] 39045 9 304 189034 1289<br>20 9 5 5 03 [4]<br>5<br>0 012345678<br>0 1 2 3 4 5 6 7 8<br>Number of Byzantine Evaluators<br>Honest Worker Byzantine Worker<br>Robust Score<br>**----- End of picture text -----**<br>


Fig. 8. Number of Byzantine Evaluators. With eight honest evaluators, we incrementally add up to seven Byzantine evaluators. The DecentLLMs continues to select the answer from the honest workers until six Byzantine evaluators are present. However, when the number reaches seven—violating the GM algorithm’s tolerance threshold (i.e., _f < ⌊_[15] 2 _[⌋]_[)—the][DecentLLMs] incorrectly selects the Byzantine worker’s. answer (i.e., from _w_ 9 _[∗]_[)] 

## VI. DISCUSSION AND FUTURE WORKS 

_Leader-based and Leader-less Approach.:_ Traditionally, state machine replication (SMR) in Byzantine consensus has been implemented using leader-based approaches (e.g., PBFT [35]). However, leader-based protocols often suffer from several limitations, including performance bottlenecks, the high complexity of leader replacement (i.e., view change protocol), and unfair transaction ordering. To address these issues, leaderless approaches (e.g., Narwhal [36]) have been proposed, enabling all participants to engage equally in the consensus process. DecentLLMs demonstrates the effective adoption of leader-less consensus architecture in LLM-based multi-agent systems, improving robustness against Byzantine agents and reducing consensus latency in worst-case scenarios compared to existing leader-based approaches [19], [20]. 

_Multi-Round Debates.:_ In multi-LLM agent systems, multi-round debate [7], [9], [13], [14], [37] is a technique known for progressively improving the quality of agent responses and reducing the degree of disagreement by exchanging feedback over several rounds. In DecentLLMs, multiround debates could be applied in two ways. First, to enhance the quality of worker responses. This can be achieved by having workers exchange their answers to improve their quality, or by refining their answers based on feedback from evaluators. Second, to reduce the variance among evaluator scores. High variance in scores, even among honest evaluators, can lead to vulnerability to Byzantine influence due to the mechanism of the Geometric Median (GM) algorithm. In our specific case, this effect was not observed because the answers from Byzantine workers contained clear signs of malicious content, prompting sufficiently low scores for effective classification. However, in scenarios with high score variance where Byzantine worker answers might receive high scores, multi-round debate could be employed. 

_AI Blockchain Oracle.:_ Blockchain oracles [38]–[41] are essential components that significantly improve the applicability of blockchain by providing external data feeds for smart contracts. We consider utilizing DecentLLMs’ LLMpowered multi-agent system as an AI-driven blockchain oracle, which represents a promising direction for future work. A key challenge is to ensure reliable and consistent outputs from non-deterministic LLM models for smart contract execution. 

## VII. CONCLUSION 

In this paper, we proposed DecentLLMs, a decentralized multi-agent system that adopts a leaderless consensus approach where worker agents generate answers in parallel and evaluator agents scores and ranking them to select best available answer using the Byzantine-robust aggregation algorithm. This design enables faster consensus and ensures that the highest-quality answer is selected, even in the presence of Byzantine agents. We demonstrated the effectiveness and robustness of our approach through experimental evaluations. 

REFERENCES 

- [1] OpenAI. (2024, May) Gpt-4o. [Online]. Available: https://openai.com/index/hello-gpt-4o// 

- [2] Qwen3. (2025, May) Qwen3. [Online]. Available: https://ollama.com/library/qwen3/ 

- [3] Anthropic. (2025, May) Claude ai. [Online]. Available: https://claude.ai/ 

- [4] Meta. (2025, May) Llama. [Online]. Available: https://www.llama.com/ [5] Google. (2025, May) Google gemini. [Online]. Available: https://gemini.google.com/ 

- [6] DeepSeek. (2025, May) Deepseek. [Online]. Available: https://www.deepseek.com/ 

- [7] Y. Du, S. Li, A. Torralba, J. B. Tenenbaum, and I. Mordatch, “Improving factuality and reasoning in language models through multiagent debate,” in _Proceedings of the 41st International Conference on Machine Learning_ , ser. ICML’24. JMLR.org, 2024. 

- [8] M. Gao, J. Ruan, R. Sun, X. Yin, S. Yang, and X. Wan, “Human-like summarization evaluation with chatgpt,” 2023. [Online]. Available: https://arxiv.org/abs/2304.02554 

- [9] J. Chun, Q. Chen, J. Li, and I. Ahmed, “Is multi-agent debate (mad) the silver bullet? an empirical analysis of mad in code summarization and translation,” 2025. [Online]. Available: https://arxiv.org/abs/2503.12029 

- [10] Y. Huang, J. Luo, Y. Yu, Y. Zhang, F. Lei, Y. Wei, S. He, L. Huang, X. Liu, J. Zhao, and K. Liu, “DA-code: Agent data science code generation benchmark for large language models,” in _Proceedings of the 2024 Conference on Empirical Methods in Natural Language Processing_ , Y. Al-Onaizan, M. Bansal, and Y.-N. Chen, Eds. Miami, Florida, USA: Association for Computational Linguistics, Nov. 2024, pp. 13 487–13 521. [Online]. Available: https://aclanthology.org/2024.emnlp-main.748/ 

- [11] V. Mothukuri, R. M. Parizi, and J. L. Massa, “Llmsmartsec: Smart contract security auditing with llm and annotated control flow graph,” in _2024 IEEE International Conference on Blockchain (Blockchain)_ , 2024, pp. 434–441. 

- [12] Y. Gai, L. Zhou, K. Qin, D. Song, and A. Gervais, “Blockchain large language models,” 2023. [Online]. Available: https://arxiv.org/abs/2304.12749 

- [13] H. Nori, M. Daswani, C. Kelly, S. Lundberg, M. T. Ribeiro, M. Wilson, X. Liu, V. Sounderajah, J. Carlson, M. P. Lungren, B. Gross, P. Hames, M. Suleyman, D. King, and E. Horvitz, “Sequential diagnosis with language models,” 2025. [Online]. Available: https://arxiv.org/abs/2506.22405 

- [14] A. Smit, N. Grinsztajn, P. Duckworth, T. D. Barrett, and A. Pretorius, “Should we be going mad? a look at multi-agent debate strategies for llms,” in _Proceedings of the 41st International Conference on Machine Learning_ , ser. ICML’24. JMLR.org, 2024. 

- [15] junyou li, Q. Zhang, Y. Yu, Q. FU, and D. Ye, “More agents is all you need,” _Transactions on Machine Learning Research_ , 2024. [Online]. Available: https://openreview.net/forum?id=bgzUSZ8aeg 

- [16] B. Yan, X. Zhang, L. Zhang, L. Zhang, Z. Zhou, D. Miao, and C. Li, “Beyond self-talk: A communication-centric survey of llm-based multi-agent systems,” 2025. [Online]. Available: https://arxiv.org/abs/2502.14321 

- [17] S. Feng, W. Ding, A. Liu, Z. Wang, W. Shi, Y. Wang, Z. Shen, X. Han, H. Lang, C.-Y. Lee, T. Pfister, Y. Choi, and Y. Tsvetkov, “When one llm drools, multi-llm collaboration rules,” 2025. [Online]. Available: https://arxiv.org/abs/2502.04506 

- [18] T. Guo, X. Chen, Y. Wang, R. Chang, S. Pei, N. V. Chawla, O. Wiest, and X. Zhang, “Large language model based multi-agents: A survey of progress and challenges,” in _Proceedings of the Thirty-Third International Joint Conference on Artificial Intelligence, IJCAI-24_ , K. Larson, Ed. International Joint Conferences on Artificial Intelligence Organization, 8 2024, pp. 8048–8057, survey Track. [Online]. Available: https://doi.org/10.24963/ijcai.2024/890 

- [19] B. Chen, G. Li, X. Lin, Z. Wang, and J. Li, “Blockagents: Towards byzantine-robust llm-based multi-agent coordination via blockchain,” in _Proceedings of the ACM Turing Award Celebration Conference - China 2024_ , ser. ACM-TURC ’24. New York, NY, USA: Association for Computing Machinery, 2024, p. 187–192. [Online]. Available: https://doi.org/10.1145/3674399.3674445 

- [20] H. Luo, G. Sun, Y. Liu, D. Zhao, D. Niyato, H. Yu, and S. Dustdar, “A weighted byzantine fault tolerance consensus driven trusted multiple large language models network,” 2025. [Online]. Available: https://arxiv.org/abs/2505.05103 

- [21] M. Yin, D. Malkhi, M. K. Reiter, G. G. Gueta, and I. Abraham, “Hotstuff: Bft consensus with linearity and responsiveness,” in _Proceedings of the 2019 ACM Symposium on Principles of Distributed Computing_ , ser. PODC ’19. New York, NY, USA: Association for Computing Machinery, 2019, p. 347–356. [Online]. Available: https://doi.org/10.1145/3293611.3331591 

- [22] R. Guerraoui, N. Gupta, and R. Pinot, “Byzantine machine learning: A primer,” _ACM Comput. Surv._ , vol. 56, no. 7, Apr. 2024. [Online]. Available: https://doi.org/10.1145/3616537 

- [23] L. Huang, W. Yu, W. Ma, W. Zhong, Z. Feng, H. Wang, Q. Chen, W. Peng, X. Feng, B. Qin, and T. Liu, “A survey on hallucination in large language models: Principles, taxonomy, challenges, and open questions,” _ACM Trans. Inf. Syst._ , vol. 43, no. 2, Jan. 2025. [Online]. Available: https://doi.org/10.1145/3703155 

- [24] IO.Net. (2025, May) io.net. [Online]. Available: https://io.net/ 

- [25] T. A. N. Authors. (2025, May) Akash network. [Online]. Available: https://akash.network// 

- [26] T. B. Authors. (2025, May) Bittensor. [Online]. Available: https://bittensor.com/ 

- [27] S. Zhao, M. Jia, Z. Guo, L. Gan, X. XU, X. Wu, J. Fu, F. Yichao, F. Pan, and A. T. Luu, “A survey of recent backdoor attacks and defenses in large language models,” _Transactions on Machine Learning Research_ , 2025, survey Certification. [Online]. Available: https://openreview.net/forum?id=wZLWuFHxt5 

- [28] M. Yu, F. Meng, X. Zhou, S. Wang, J. Mao, L. Pang, T. Chen, K. Wang, X. Li, Y. Zhang, B. An, and Q. Wen, “A survey on trustworthy llm agents: Threats and countermeasures,” 2025. [Online]. Available: https://arxiv.org/abs/2503.09648 

- [29] X. Zhang, B. Huang, S. Duan, and H. Zhang, “Randomized vs. deterministic? practical randomized synchronous BFT in expected constant time,” Cryptology ePrint Archive, Paper 2025/816, 2025. [Online]. Available: https://eprint.iacr.org/2025/816 

- [30] P. Blanchard, E. M. El Mhamdi, R. Guerraoui, and J. Stainer, “Machine learning with adversaries: Byzantine tolerant gradient descent,” in _Advances in Neural Information Processing Systems_ , I. Guyon, U. V. Luxburg, S. Bengio, H. Wallach, R. Fergus, S. Vishwanathan, and R. Garnett, Eds., vol. 30. Curran Associates, Inc., 2017. 

- [31] E. M. El Mhamdi, R. Guerraoui, and S. Rouault, “The hidden vulnerability of distributed learning in byzantium,” in _Proceedings of the 35th International Conference on Machine Learning_ , ser. Proceedings of Machine Learning Research, J. Dy and A. Krause, Eds., vol. 80. PMLR, 10–15 Jul 2018, pp. 3521–3530. 

- [32] wikipedia. (2025, May) Geometric median. [Online]. Available: https://en.wikipedia.org/wiki/Geometric median/ 

- [33] P. S. Foundation. (2024, May) Python 3.11.11. [Online]. Available: https://www.python.org/downloads/release/python-31111/ 

- [34] Y. Wang, X. Ma, G. Zhang, Y. Ni, A. Chandra, S. Guo, W. Ren, A. Arulraj, X. He, Z. Jiang _et al._ , “Mmlu-pro: A more robust and challenging multi-task language understanding benchmark,” _arXiv preprint arXiv:2406.01574_ , 2024. 

- [35] M. Castro, B. Liskov _et al._ , “Practical byzantine fault tolerance,” in _OSDI_ , vol. 99, 1999, pp. 173–186. 

- [36] G. Danezis, L. Kokoris-Kogias, A. Sonnino, and A. Spiegelman, “Narwhal and tusk: A dag-based mempool and efficient bft consensus,” in _Proceedings of the Seventeenth European Conference on Computer Systems_ , ser. EuroSys ’22. New York, NY, USA: Association for Computing Machinery, 2022, p. 34–50. [Online]. Available: https://doi.org/10.1145/3492321.3519594 

- [37] K. Xiong, X. Ding, Y. Cao, T. Liu, and B. Qin, “Examining inter-consistency of large language models collaboration: An indepth analysis via debate,” in _Findings of the Association for Computational Linguistics: EMNLP 2023_ . Association for Computational Linguistics, 2023, p. 7572–7590. [Online]. Available: http://dx.doi.org/10.18653/v1/2023.findings-emnlp.508 

- [38] L. Breidenbach, C. Cachin, B. Chan, A. Coventry, S. Ellis, A. Juels, F. Koushanfar, A. Miller, B. Magauran, D. Moroz _et al._ , “Chainlink 2.0: Next steps in the evolution of decentralized oracle networks,” _Chainlink Labs_ , vol. 1, pp. 1–136, 2021. 

- [39] Supra. (2025, May) Supra: A faster, better web3 experience for everyone. [Online]. Available: https://supra.com/ 

- [40] kava. (2025, May) Ai-powered oracles: Bridging blockchains with smarter data. [Online]. Available: https://www.kava.io/news/ai-poweredoracles/ 

- [41] RedStone. (2025, May) Redstone: The modular blockchain oracle. [Online]. Available: https://www.redstone.finance/ 

