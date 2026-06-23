1 

## Web3 × AI Agents: Landscape, Integrations, and Foundational Challenges 

Yiming Shen, Jiashuo Zhang, Zhenzhe Shao, Wenxuan Luo, Yanlin Wang, Ting Chen, Zibin Zheng, Jiachi Chen* 

_**Abstract**_ **—The convergence of Web3 technologies and AI agents represents a rapidly evolving frontier poised to reshape decentralized ecosystems. This paper presents** _**the first and most comprehensive analysis**_ **of the intersection between Web3 and AI agents, examining five critical dimensions: landscape, economics, governance, security, and trust mechanisms. Through an analysis of 133 existing projects, we first develop a taxonomy and systematically map the current market landscape (RQ1), identifying distinct patterns in project distribution and capitalization. Building upon these findings, we further investigate four key integrations: (1) the role of AI agents in participating in and optimizing decentralized finance (RQ2); (2) their contribution to enhancing Web3 governance mechanisms (RQ3); (3) their capacity to strengthen Web3 security via intelligent vulnerability detection and automated smart contract auditing (RQ4); and (4) the establishment of robust reliability frameworks for AI agent operations leveraging Web3’s inherent trust infrastructure (RQ5). By synthesizing these dimensions, we identify key integration patterns, highlight foundational challenges related to scalability, security, and ethics, and outline critical considerations for future research toward building robust, intelligent, and trustworthy decentralized systems with effective AI agent interactions.** 

_**Index Terms**_ **—Web3, Blockchain, Artificial Intelligence (AI), LLM, Agent.** 

## I. INTRODUCTION 

The rapid evolution of Web3 technologies has created expansive decentralized ecosystems spanning multiple blockchain networks, with Total Value Locked (TVL) exceeding $100 billion across protocols [1]. However, these ecosystems continue to face persistent challenges, including complex user interfaces, inefficient decision-making processes, and limited autonomous capabilities, which hinder their mainstream adoption [2]. Concurrently, the advancement of large language models (LLMs) and AI agent technologies has enabled sophisticated reasoning and autonomous task execution. Nevertheless, such AI systems currently lack inherent mechanisms for trustless operation and decentralized coordination [3], [4]. 

Fortunately, the integration of Web3 and AI technologies can address mutual limitations: Web3 provides cryptographic 

Y. Shen, Z. Shao, Y. Wang, Z. Zheng is with the School of Software Engineering, Sun Yat-sen University, Zhuhai, Guangdong, China (e-mail: shenym7@mail2.sysu.edu.cn; shaozhzh3@mail2.sysu.edu.cn; wangylin36@mail.sysu.edu.cn; zhzibin@mail.sysu.edu.cn) J. Zhang is with the School of Computer Science, Peking University, Beijing, China (e-mail: zhangjiashuo@pku.edu.cn) W. Luo, T. Chen is with the School of Computer Science and Engineering (School of Cyber Security), University of Electronic Science and Technology of China, Chengdu 611731, China (e-mail: luowx2000@outlook.comb; brokendragon@uestc.edu.cn) 

J. Chen is the with The State Key Laboratory of Blockchain and Data Security, Zhejiang University, Hangzhou, Zhejiang, China (e-mail: chenjch86@mail.sysu.edu.cn) 

J. Chen is the corresponding author. 

security and decentralized infrastructure for AI agents, while AI agents enhance Web3 accessibility, efficiency, and intelligent automation [5], [6], [7]. Current Web3-AI Agent implementations demonstrate significant market traction and technical sophistication [8]. Projects range from simple chatbot interfaces that assist with blockchain queries to autonomous agents managing multi-million dollar DeFi portfolios, executing complex cross-chain transactions, and actively participating in decentralized governance [9]. 

To systematically explore this emerging landscape, we examine five research questions (RQs) representing critical aspects of Web3-AI Agent integration. Firstly, we develop a taxonomy and map the existing market landscape (RQ1), revealing distinct patterns in both project distribution and capitalization. Building upon this landscape analysis, we explore four key integrations. We investigate how AI agents participate in and optimize decentralized finance (RQ2) and enhance Web3 governance mechanisms (RQ3). We examine how AI agents strengthen Web3 security through intelligent vulnerability detection and automated smart contract auditing (RQ4). Furthermore, we assess how Web3’s native trust infrastructure creates robust reliability frameworks for AI agent operations (RQ5). 

Through the analysis of 133 existing projects collectively valued at over $6.9 billion in market capitalization, we establish that the intersection of Web3 and AI technologies creates new integrations that mitigate fundamental challenges in both domains while introducing novel capabilities for autonomous decentralized operations. The analysis reveals that although infrastructure projects constitute a smaller number overall, they dominate market capitalization, indicating substantial investor confidence in foundational technologies. Meanwhile, the growth of AI agent incubation platforms and financial services applications demonstrates substantial developer activity and practical deployment across diverse use cases. 

Our findings highlight bidirectional enhancements facilitated by the integration of Web3 and AI technologies. AI agents leverage Web3’s trustless infrastructure to operate autonomously with cryptographic guarantees, while Web3 systems benefit from AI-enhanced accessibility, intelligent automation, and sophisticated decision-making capabilities. However, significant challenges remain in scalability, security, ethics, and technical integration that require continued research and development. 

Our analysis offers the following key contributions: 

- We develop the first and the most comprehensive taxonomy organizing Web3 AI agent projects into four primary categories and ten subcategories shown in Figure 1. 

2 

Fig. 1. The Ecosystem Landscape of Web3 AI Agent Projects 

- We demonstrate specific mechanisms through which AI agents enhance Web3 operations across three critical domains: decentralized finance through autonomous strategy execution and intelligent portfolio optimization, governance through automated analysis and community engagement, and security through intelligent vulnerability detection and automated smart contract auditing. 

- We examine how Web3 trust mechanisms enable reliable AI agent operations through cryptographic guarantees, decentralized verification, and transparent accountability systems. 

- We provide open-source access to our comprehensive dataset of 133 Web3 AI agent projects to support future research in this domain: https://github.com/shenyimings/ Web3-AI-Agents 

**==> picture [224 x 175] intentionally omitted <==**

**----- Start of picture text -----**<br>
Observe Blockchain<br>Goal<br>Oriented Wallet<br>YE Smart Contract<br>Memory<br>Z\<br>feeft tee DeFi Protocol<br>Agents<br>Orchestration Market Data<br>Planning<br>Action ...<br>AI Agents Interact Web3<br>Model Context Protocol (MCP)<br>**----- End of picture text -----**<br>


## II. BACKGROUND 

Fig. 2. A typical architecture of Web3 AI Agents 

## _A. Web3 Ecosystem_ 

Web3 represents the decentralized evolution of the internet, built on blockchain infrastructure where users maintain ownership of their digital assets and identities through cryptographic protocols and smart contracts [6]. The ecosystem spans multiple blockchain networks, including Ethereum [10], Solana [11], and layer-2 solutions [12], [13], each supporting a diverse range of decentralized applications (DApps) that span decentralized finance (DeFi) protocols, gaming, and content creation platforms [14]. Currently, the Total Value Locked (TVL) across all Web3 protocols exceeds $100 billion [1], demonstrating the significant scale and adoption of this emerging digital economy. 

## _B. Large Language Model (LLM)_ 

LLMs represent advanced AI systems trained on vast text corpora to understand and generate human-like text, enabling 

sophisticated reasoning and interaction capabilities [15]. Modern LLMs like GPT [16], Claude [17], and Gemini [18] demonstrate remarkable proficiency in natural language understanding, code generation, and complex problem-solving tasks that form the cognitive foundation for autonomous agent systems [19]. 

## _C. Web3-AI Agent_ 

Figure 2 illustrates a typical Web3-AI Agent architecture where LLMs serve as the core reasoning engine, equipped with essential agent capabilities including planning, memory, and multi-agent orchestration. These agents interact with blockchain infrastructure through the Model Context Protocol (MCP) [20] - a standardized interface that enables seamless communication between LLMs and external tools or resources. 

3 

**==> picture [500 x 142] intentionally omitted <==**

**----- Start of picture text -----**<br>
Data Collection Taxonomic Categorization<br>Open Card Sorting on 133 projects DeFi (RQ2)<br>- MarketCap<br>- Networks Explore Validate<br>- Website Governance<br>Search Classify<br>- Whitepaper (RQ3)<br>& Filter * About Findings<br>* Tokenomics (RQ1)<br>Security<br>* Pathways Market Analysis<br>(RQ4)<br>Quantitative analysis on launched 77 projects<br>Data Source<br>Category Network Trust (RQ5)<br>Project Information<br>7<br>**----- End of picture text -----**<br>


Fig. 3. Research methodology 

This architecture enables AI agents to function as intelligent front-ends for Web3 interactions, handling complex tasks such as navigating multiple blockchains, managing gas fees, interfacing with various protocols and tokens, connecting to decentralized applications, and securing sensitive information like seed phrases. 

Web3-AI Agent currently manifests in diverse forms, including autonomous trading bots [21], DeFi portfolio managers [22], cross-chain transaction assistants [23], and intelligent wallet interfaces [24]. The ecosystem demonstrates rapid growth with projects ranging from simple chatbot interfaces for blockchain queries to sophisticated autonomous agents capable of executing complex multi-step DeFi strategies and managing entire crypto portfolios without human intervention [25]. 

## III. METHODOLOGY 

This study employs a mixed-methods approach, combining systematic data collection, qualitative categorization, and quantitative market analysis, to investigate the Web3-AI Agent landscape. The overall methodology is illustrated in Figure 3. 

## _A. Data Collection_ 

We establish a comprehensive dataset of Web3-AI agent projects through systematic collection from multiple sources, i.e., CoinMarketCap [9], Product Hunt [26], and GitHub [27]. Our data collection strategy combines keyword-based filtering with snowball sampling [28] to ensure coverage of the emerging Web3-AI Agent ecosystem. Complete data sources, keyword specifications, and results are available in our online repository. 

_1) Data Sources:_ We collect project data from three primary sources: CoinMarketCap [9] for market-listed projects with verifiable trading activity, Product Hunt [26] for emerging technology platforms and developer tools, and GitHub [27] for open-source projects demonstrating technical implementation. This multi-source approach captures both established projects with market presence and early-stage developments with active codebases. 

Our initial keyword set includes “Web3 AI”, “blockchain AI”, “Web3 agent”, “Crypto agent”, and “DeFi agent”. During data collection, we implement snowball sampling [28] by recording additional relevant keywords encountered in project documentation and applying them to discover related projects. This iterative process expands our keyword list to 15 terms, including “Web3 AI Agent”, “Crypto AI Agent”, “Crypto LLM Agent”, “Crypto LLM”, “Crypto AI”, “Blockchain Agent”, “Chain Agent”, “Decentralized AI Agent”, “Blockchain LLM”, and “DeFi AI”, ensuring comprehensive coverage of terminology variations in this emerging domain. 

_2) Project Filtering:_ We apply the following verification criteria to ensure data quality and relevance. _First_ , projects must demonstrate meaningful integration of both Web3 and AI agent technologies, verified through at least one of the following: a public GitHub repository containing relevant implementation code, a published white paper detailing technical architecture, or verifiable on-chain activity demonstrating operational deployment. _Second_ , we exclude projects that merely use AI or Web3 terminology without substantive integration, such as traditional machine learning applications in cryptocurrency trading that predate modern large language model capabilities. _Finally_ , we filter out projects lacking sufficient documentation or evidence of active development. This verification process yields 133 projects meeting our inclusion criteria. 

For each validated project, we collect structured metadata including project name, primary category, technical description, market capitalization (where available), on-chain smart contract addresses, GitHub repository URLs, official website links, and associated white papers. All of the data can be found at our online repositories. 

## _B. Taxonomic Categorization_ 

We develop a comprehensive taxonomy through structured qualitative analysis using open card sorting methodology adapted from established software engineering research practices [29], [30]. This approach enables systematic organization of the diverse Web3-AI Agent landscape while maintaining flexibility to capture emerging patterns. 

4 

TABLE I 

CONSOLIDATED WEB3 AI AGENT TAXONOMY WITH MARKET METRICS[1] 

|**Primary Category**<br>**Avg. MCap ($)**|**Subcategory**<br>**Count**<br>**Representative Projects**|
|---|---|
|AI Agent Incubation (56)<br>88,368,535.8|Builder & Marketplace<br>46<br>Singularity, Eliza OS, Alchemist.ai|
||Monetization & Launchpad<br>10<br>Virtuals-Protocol, Clanker|
|Infrastructure (34)<br>187,702,496.4|Agent Protocol & DePin<br>22<br>NEAR, Fetch.ai, Delysium|
||Trust Layer<br>6<br>OriginTrail, Phala Network|
||AI-powered Development<br>6<br>ChainGPT, Reploy|
|Financial Services (55)<br>56,869,408.6|DeFAI Agents<br>28<br>WayFinder, Hey Anon, Griffain|
||Investment Analytics<br>27<br>Nexo, Aixbt, Numer.ai|
|Creative & Virtual (28)<br>84,564,822.5|Game & Metaverse<br>14<br>Freysa, Luna, Hytopia|
||Content Creation<br>10<br>Botto, Zerebro, Bad Idea AI|
||AI-powered RWA<br>4<br>OriginTrail, Propy, Rentality|



_1) Categorization Process:_ Our categorization follows a three-phase process designed to ensure reliability and consistency. In the initial exploration phase [31], two researchers independently examine 40% of the collected projects (53 projects), reviewing their white papers, technical documentation, and stated objectives to identify core functionalities and initial grouping patterns. This phase involves iterative category refinement, where emerging patterns prompt revisiting initial assignments to ensure consistency with evolving category definitions. 

During the classification phase, the same two researchers independently categorize the remaining 60% of projects (80 projects) according to the established framework. Each researcher applies the defined categories without consultation to test inter-rater reliability. In the validation phase, a third researcher reviews all classifications, identifies discrepancies between the initial categorizers, and facilitates consensus discussions to resolve conflicts and finalize project assignments. 

_2) Taxonomy Framework:_ This process produces a taxonomy comprising four primary categories and ten subcategories, as detailed in Table I, which capture the functional diversity of Web3-AI Agent integration : 

- **AI Agent Incubation** (56 projects): Platforms providing infrastructure for creating, deploying, and monetizing AI agents within Web3 environments. This category includes _Builder & Marketplace platforms_ (46 projects) offering development tools and agent distribution systems, and _Monetization & Launchpad services_ (10 projects) facilitating agent tokenization and funding mechanisms. 

- **Infrastructure** (34 projects): Foundational technologies enabling decentralized AI agent operations. Subcategories include _Agent Protocol & DePIN systems_ (22 projects) providing core networking and computational resources, _Trust Layer protocols_ (6 projects) establishing verification and reputation mechanisms, and _AI-powered Development Tools_ (6 projects) offering specialized programming environments for agent creation. 

- **Financial Services** (55 projects): Applications leveraging AI agents for decentralized finance (DeFi) operations and 

   - market analysis. This encompasses _DeFAI Agents_ (28 projects) performing autonomous trading and portfolio management, and _Investment Analytics tools_ (27 projects) providing market intelligence and risk assessment capabilities. 

- **Creative & Virtual** (28 projects): Projects utilizing AI agents in gaming, content creation, and asset management contexts. Subcategories include Game & Metaverse applications (14 projects), Content Creation platforms (10 projects), and AI-powered Real World Asset (RWA) management systems (4 projects). 

## _C. Market Analysis_ 

We conduct quantitative market analysis using market capitalization data to understand the economic significance and distribution patterns across our taxonomy. Market capitalization data provides insights into investor confidence and project maturity within each category. _All valuation data was sourced from CoinMarketCap [9], reflecting market capitalizations recorded as of 6 April 2025._ 

Our analysis explores the relationship between project count and aggregate market value across taxonomy categories to highlight areas of concentrated investment versus broader development activity. This approach reveals patterns in market validation and resource allocation within the Web3-AI Agent ecosystem [6]. The methodology ensures systematic coverage of the Web3-AI Agent landscape while maintaining rigorous standards for project inclusion and categorization accuracy. This framework enables a comprehensive analysis of current market patterns and technological integration approaches across the identified project categories. 

## IV. ANSWER TO RQ1: LANDSCAPE OF WEB3 AI AGENT 

This section presents findings from the systematic analysis of 133 Web3-AI Agent projects, revealing distinct patterns in project distribution and market capitalization. These insights provide a foundation for understanding the four key integration domains discussed in subsequent research questions. 

## _A. Distribution and Market Patterns_ 

> 127 projects are classified under multiple categories due to their multifaceted nature, and their full market capitalization is attributed to each category they belong to in this analysis. 

_1) Taxonomic:_ Our taxonomic analysis reveals significant differences between the number of projects and their market 

5 

**==> picture [516 x 210] intentionally omitted <==**

**----- Start of picture text -----**<br>
28 $187.7M Number of Projects 50 $134.4M Number of Projects $140.0M<br>25 25 Avg Market Cap $175.0M 45 Avg Market Cap $120.0M<br>40<br>$150.0M<br>20 20 19 $100.0M<br>$125.0M<br>30<br>28 $80.0M<br>15 $100.0M<br>$88.4M $84.6M 22<br>$60.0M<br>$75.0M 20<br>$48.8M<br>10<br>$56.9M $39.2M 15 $40.0M<br>$50.0M<br>10 $25.3M<br>5<br>$25.0M $20.0M<br>4<br>$4.0M<br>0 $0.0M 0 $0.0M<br>Incubation Infrastructure Financial Creative Ethereum Solana Base Binance Others<br>Number of Projects Number of Projects<br>Avg Market Cap per Project Avg Market Cap per Project<br>**----- End of picture text -----**<br>


Fig. 4. Distribution and Market Capitalization by Category 

Fig. 5. Distribution and Market Capitalization by Blockchain Network 

valuation across the four identified categories. The _AI Agent Incubation_ category demonstrates the highest development activity with 56 projects (42.1% of total), indicating substantial entrepreneurial focus on creating platforms and tools for agent development. These projects encompass builder platforms, marketplaces, and monetization mechanisms that facilitate the entire lifecycle of AI agent creation and deployment. 

Our market analysis leverages capitalization data available for 77 of the 133 identified projects, representing a collective market capitalization of approximately $6.92 billion at the time of analysis. An observation from Figure 4 is the dominance of the Infrastructure category. While this category, comprising only 34 projects (25.6% of total), accounts for $4.69 billion, or 67.8% of the analyzed market capitalization. This category includes foundational protocols, decentralized physical infrastructure networks (DePIN) [32], and trust layer mechanisms that provide the technological backbone for decentralized AI agent operations. The Financial Services category contains 55 projects (41.4%), demonstrating significant interest in applying AI agents to decentralized finance and investment analytics. The Creative & Virtual category includes 28 projects (21.1%), focusing on gaming, content creation, and real-world asset management applications. 

**Observation 1:** The Web3-AI Agent ecosystem exhibits a distinctive dual concentration pattern, with market capitalization concentrated in _Infrastructure_ while development activity concentrates in _AI Agent Incubation_ projects. 

_2) Blockchain Network:_ Analysis of the underlying blockchain networks (Figure 5), based on data from 114 projects (as some projects deploy on multiple networks), highlights Ethereum’s dominance. Ethereum hosts 45 projects (39.5% of network instances), which collectively represent $6.05 billion (87.4%) of the market capitalization analyzed by 

the network. This dominance is also reflected in the highest average market cap per project on Ethereum ($134.4 million). Solana (28 projects, 24.6%) and Base (22 projects, 19.3%) emerge as the next most popular platforms, indicating growing multi-chain development. However, their associated market capitalizations ($1.10 billion and $1.07 billion, respectively) and average project market caps ($39.2 million and $48.8 million, respectively) are considerably lower than Ethereum’s. Binance Smart Chain (BSC) follows closely behind Solana, hosting 15 projects (13.2%) with an average market cap of $25.3 million per project. Other networks like Arbitrum [33], Sui [34], and Polygon [35] currently host fewer projects in this specific domain. 

**Observation 2:** Ethereum demonstrates clear dominance in the Web3-AI Agent landscape despite emerging multi-chain development trends. 

## _B. Key Ecosystem Patterns_ 

Our landscape analysis, integrating project counts, categorization, and market capitalization data, identifies several significant findings regarding the current state of the Web3-AI Agent ecosystem. 

_1) Market Concentration Dynamics:_ The market exhibits a significant power-law distribution. The top 10% of projects with available market capitalization data (7 out of 77) collectively command $5.13 billion, representing /74.2% of the total analyzed market capitalization. This concentration, exemplified by leading projects such as NEAR Protocol [36] ($2.44B) and Fetch.ai [37] ($1.05B), suggests winner-takesmost dynamics. 

_2) Cross-Category Integration Trends:_ A notable trend is the integration of functionalities across categories. Approximately 20.3% of projects (27 out of 133) span multiple taxonomic classifications. These multi-category projects account for a substantial portion of the market, representing 

6 

$1.93 billion (27.9%) of the analyzed capitalization. Frequent combinations like _Financial + Incubation_ (9 projects) and _Infrastructure + Incubation_ (8 projects) suggest a move towards building more comprehensive platforms and integrated service offerings that blur common categorical boundaries. 

_3) Emergence of Multi-Chain Deployment:_ The Ethereum network remains the dominant platform for Web3-AI Agent development, hosting the largest share of projects (39.5% of network instances) and capturing the majority of the associated market capitalization (87.4%). However, the presence of projects on Solana (24.6%) and Base (19.3%) indicates a trend towards multi-chain deployment and exploration of alternative layer-1 and layer-2 solutions. 

**Observation 3:** The ecosystem demonstrates three patterns: 1) market concentration with winner-takes-most dynamics, 2) increasing cross-category integration, and 3) multi-chain deployment 

## _C. Growing Focus on Foundational Infrastructure_ 

The disproportionate market capitalization concentration in Infrastructure projects ($187.7 million average vs. $88.4 million for Incubation) reflects investor prioritization of foundational technologies. This pattern indicates market recognition that scalable, secure, and efficient infrastructure represents a prerequisite for broader Web3-AI agent adoption. The substantial valuation commanded by infrastructure projects further demonstrates that the Web3-AI agent ecosystem remains in its beginning stages, where foundational building blocks command greater market confidence than applicationlayer innovations [38]. The high project count in _AI Agent Incubation_ (56 projects), combined with lower average market capitalization, indicates substantial grassroots development activity. This pattern suggests a vibrant ecosystem of builders creating tools, platforms, and services to lower barriers for Web3-AI agent development and deployment, positioning the ecosystem for significant expansion as infrastructure maturity enables more sophisticated applications. 

**Observation 4:** The Web3-AI Agent ecosystem remains in early development stages with infrastructure commanding dominant market capitalization, while the incubation platform development activity demonstrates strong investor interest. 

Our findings reveal a rapidly evolving Web3-AI Agent landscape characterized by concentrated investment in foundations, expanding multi-chain deployment, and emerging technological diversification. Building upon this landscape foundation, our analysis identifies four primary synergistic domains where Web3-AI Agent implementations demonstrate the most significant integration potential: decentralized finance, governance mechanisms, security frameworks, and trust infrastructure. We examine these synergistic effects in depth through our subsequent research questions (RQ2-RQ5). 

**Answer to RQ1** : The Web3-AI Agent landscape comprises 133 projects across four main categories, with Infrastructure projects dominating 67.81% of the $6.92 billion total market capitalization despite representing only 24.8% of projects. The ecosystem exhibits pronounced market concentration, substantial incubation activity, and increasing cross-category convergence with Ethereum hosting 39.5% of projects and capturing 87.4% of market value. 

## V. ANSWER TO RQ2: AI AGENTS IN DECENTRALIZED FINANCE 

The integration of AI agents into the Decentralized Finance (DeFi) ecosystem marks a significant advancement, enabling more dynamic, intelligent, and automated financial systems. AI agents leverage Web3’s core components—smart contracts for execution, blockchain transparency for data verification, and native assets for value transfer—to participate actively in DeFi. This section explores the key roles that AI agents play in transforming DeFi operations by analyzing current implementations and case studies from our dataset. 

## _A. Autonomous Trading Strategy Implementation_ 

A primary role of AI agents in DeFi is the autonomous execution of transactions and complex financial strategies based on user intent or predefined rules. These agents interact directly with blockchain protocols and smart contracts, translating high-level goals into concrete on-chain actions. For instance, platforms like _Griffain_ [39] empower users to deploy agents that perform tasks such as setting up recurring token purchases, executing sales when specific market cap targets are met, or automatically depositing idle assets into yield-bearing protocols. Similarly, _Wayfinder_ [21] acts as an omnichain protocol where user-owned agents navigate diverse blockchain ecosystems and decentralized applications (DApps). These agents can independently execute transactions ranging from simple token swaps to more intricate strategies like liquidity provision or lending, utilizing dedicated Web3 wallets under user control. This capability moves beyond manual interaction, allowing for persistent, automated management of DeFi activities based on sophisticated logic or real-time triggers, operating with an efficiency often unachievable manually. 

**Observation 5:** AI agents enable autonomous DeFi strategy execution by translating high-level user intents into complex on-chain actions, moving beyond manual interaction toward automated real-time management with sophisticated logic 

## _B. Intelligent Portfolio Construction and Optimization_ 

AI agents significantly enhance portfolio management within DeFi by offering personalized and adaptive strategies. They analyze a user’s transaction history, risk tolerance, and market conditions to construct and dynamically manage DeFi portfolios. _One Click Crypto_ [40], positioning itself as a 

7 

“Wealthfront for DeFi”, exemplifies this by using AI to analyze a user’s blockchain history and risk profile to recommend and deploy a tailored DeFi portfolio with a single click. Agents can continuously monitor market volatility, protocol risks (e.g., smart contract vulnerabilities, impermanent loss), and yield opportunities. Based on this ongoing analysis, they autonomously rebalance assets, harvest rewards, and shift capital between different protocols or liquidity pools to optimize risk-adjusted returns. 

While not explicitly focused solely on portfolio construction, the agents within ecosystems like _Sharpe AI_ [41] can execute complex, multi-step strategies involved in sophisticated portfolio optimization across different chains and protocols. These AI agents autonomously monitor market conditions, identify optimal entry and exit points, and orchestrate intricate DeFi operations—from yield farming and liquidity provision to cross-chain arbitrage—based on user-defined parameters and risk preferences. 

These agents serve as intermediaries that translate high-level investment goals into precise technical execution across fragmented DeFi protocols, continuously learning from transaction histories to optimize pathways and strategies while adapting to evolving market dynamics. 

**Observation 6:** AI agents enhance DeFi portfolio management through personalized analysis of user profiles and continuous optimization, autonomously rebalancing assets and harvesting rewards. 

## _C. AI-Driven Market Analysis and Intelligence_ 

AI agents serve as powerful tools for aggregating, processing, and interpreting the vast amount of data generated within the DeFi and broader crypto markets. They provide users with real-time insights, predictive analytics, and sentiment analysis, enabling more informed decision-making. 

Projects like _Aixbt_ [42] offer a Bloomberg-style intelligence dashboard driven by AI, delivering real-time research on tokenomics, narrative tracking, and smart money flows faster than human analysts. _Assemble AI_ ’s agent, NS3 [43], focuses on Web3 journalism, using advanced AI reasoning to analyze crypto and economic trends, market psychology, and potential ripple effects, delivering insights in multiple languages. _DexCheck AI_ [44] leverages AI and machine learning for real-time DEX and NFT market analysis, featuring an AI-powered on-chain search engine and integration with Telegram trading bots for automated solutions. Furthermore, _Hey Anon_ [23] combines conversational AI with real-time data aggregation, allowing users to query project updates and analyze trends across multiple platforms. These intelligence agents sift through noise, identify patterns, and deliver actionable information, making the complex and fast-moving DeFi market more understandable. 

**Observation 7:** AI agents process vast DeFi market data to deliver real-time insights and predictive analytics that enable faster, more informed decision-making than human analysis. 

## _D. Improving DeFi Accessibility and Interaction_ 

A crucial role for AI agents is simplifying the complex user experience associated with DeFi and Web3 interactions. By reducing technical complexity, agents make DeFi more accessible to a broader audience. _Hey Anon_ [23], for example, uses conversational AI to allow users to manage DeFi operations through natural language prompts, simplifying tasks and information retrieval. _Wayfinder_ explicitly aims to improve blockchain accessibility via its natural language interface, enabling both novice and experienced users to execute complex cross-chain transactions (like bridging assets) through generalized instructions, which the AI agent translates into the necessary multi-step on-chain actions. These agents act as intelligent interfaces, managing wallet interactions, gas fees, and protocol specifics behind the scenes. Wayfinder’s concept of agents learning from past interactions and leveraging shared knowledge (”Wayfinding Paths,” network memory) further points towards systems that adapt to user needs and continuously improve the interaction experience within decentralized ecosystems [21]. This focus on usability is critical for driving wider adoption of DeFi technologies. 

**Observation 8:** AI agents improve DeFi accessibility through natural language interfaces that simplify complex operations and reduce technical barriers for mainstream users. 

In conclusion, AI agents are becoming integral participants in the DeFi ecosystem, functioning as autonomous executors, intelligent portfolio managers, sophisticated market analysts, and user-friendly interfaces. Their ability to process vast data, execute complex strategies, and simplify interactions, all underpinned by Web3’s trustless and programmable infrastructure, significantly enhances the efficiency, sophistication, and accessibility of decentralized finance. 

**Answer to RQ2** : AI agents transform DeFi through four key roles: autonomous trading strategy implementation, intelligent portfolio construction and optimization, AIdriven market analysis and intelligence, and improving accessibility through natural language interfaces. These agents enable persistent automation, personalized strategies, real-time insights, and simplified interactions, enhancing DeFi efficiency and accessibility. 

## VI. ANSWER TO RQ3: AI AGENTS ENHANCING WEB3 GOVERNANCE 

AI agents enhance Web3 governance by addressing critical challenges throughout the governance lifecycle [45]. Building upon Ostrom’s established governance theory [46], we 

8 

analyze AI agent integration across three fundamental governance phases: _decision-making processes_ (proposal analysis and community engagement), _implementation and enforcement_ (automated monitoring and execution), and _system evolution_ (adaptive mechanism design [47]). 

## _A. Proposal Analysis and Community Engagement_ 

AI agents streamline governance processes by automating proposal analysis and enhancing community engagement capabilities. These agents automatically parse complex proposals, provide key summaries, and identify implications across economic, security, and community alignment dimensions. For example, agents developed on platforms like _Fetch.ai_ [37] or utilizing analytical capabilities similar to _ChainGPT_ [48] analyze smart contract modifications within proposals and flag potential security vulnerabilities before voting commences. 

AI agents enhance deliberation quality by facilitating sophisticated voting strategies and community engagement. Users delegate voting power to AI agents programmed with specific preferences or ethical frameworks, leveraging ecosystems like _Autonolas (Olas)_ [49] for co-owned agent services or _Artificial Liquid Intelligence (ALI)_ [50] where agents execute decisions based on token-holder defined rules. For instance, a DAO focused on environmental sustainability deploys AI agents that automatically vote against proposals conflicting with predefined environmental criteria while providing detailed rationale to the community. 

Community accessibility improves through AI agents that translate complex technical terminologies into simplified language and provide personalized governance notifications. Agents built using frameworks from _MyShell_ [51] create interfaces that lower barriers for less technical community members. These agents monitor discussion forums, summarize key arguments, track community sentiment, and identify emerging consensus points. While dedicated governance agent categories were not explicitly identified in our landscape analysis (RQ1), existing platforms within _AI Agent Incubation_ and _Content Creation_ categories adapt to serve proposal analysis and deliberation-support functions within DAO governance contexts. 

**Observation 9:** AI agents simplify governance processes by automating proposal analysis, providing security monitoring, and enhancing community engagement through natural language interfaces and personalized notifications. 

_B. Automated Monitoring and Enforcement of Governance Decisions_ 

AI agents ensure faithful execution of approved governance proposals through continuous monitoring of on-chain activities and relevant off-chain data sources. These agents verify that decisions encoded in passed proposals are correctly enacted by tracking smart contract interactions and balance changes. For example, when a proposal mandates fee structure modifications or treasury disbursements, AI agents could leverage 

_OriginTrail_ ’s Decentralized Knowledge Graph to monitor relevant smart contract executions to confirm compliance. 

Automated oversight mechanisms flag deviations from approved proposals and trigger appropriate responses. Monitoring agents automatically alert communities or designated oversight bodies when violations are detected, and can even trigger pre-defined contingency actions via smart contracts. This capability enhances accountability and transparency in post-voting governance phases. The reliability of such agents depends on robust _Agent Protocol & DePIN_ infrastructure, such as that developed by _Autonolas (Olas)_ , and _Trust Layer_ mechanisms to ensure operational integrity and data veracity. AI agents within the _Humans.ai_ ecosystem exemplify this approach by emphasizing ethical AI governance with transparent and accountable operations. 

**Observation 10:** AI agents ensure governance compliance through continuous monitoring of on-chain activities, automated verification of proposal execution, and real-time alerting of deviations. 

## _C. Adaptive Governance and Mechanism Design_ 

AI agents contribute to dynamic and resilient governance models by analyzing historical governance data, participation patterns, and network performance metrics. These agents identify inefficiencies in existing governance mechanisms and simulate potential impacts of proposed changes to voting rules, quorum thresholds, or incentive structures. For instance, AI tools specialized from _ChainGPT_ ’s code analysis capabilities analyze governance patterns and model outcome scenarios for proposed mechanism modifications [48]. 

Iterative governance refinement occurs through AI-assisted analysis and recommendation systems. AI agents suggest adjustments to delegation incentives for improved voter participation or propose modifications to proposal lifecycles that enhance decision-making speed without compromising deliberation quality. 

Advanced AI models supported by _Infrastructure_ projects like _NEAR Protocol_ and _Fetch.ai_ ’s Open Economic Framework enable sophisticated governance mechanism design. These systems become more responsive and better aligned with longterm community objectives through continuous learning and adaptation. AI agents from _Virtuals Protocol_ [52], designed for human-curated AI, incorporate community feedback into learning processes to develop adaptive governance models that evolve with community needs. 

**Observation 11:** AI agents enable adaptive governance by analyzing historical patterns, identifying mechanism inefficiencies, and recommending iterative improvements to voting rules and incentive structures. 

AI agents possess significant potential to revolutionize Web3 governance through automated analysis, inclusive deliberation, diligent decision monitoring, and adaptive mechanism design. These capabilities address prevalent challenges in 

9 

current decentralized systems while introducing new considerations, including AI decision-making transparency, algorithmic bias risks, and agent security requirements. The foundational tools and agent capabilities across various categories (RQ1) provide a strong basis for this governance evolution, though careful research and robust solutions remain necessary as these systems mature. 

**Answer to RQ3** : AI agents enhance Web3 governance through intelligent proposal analysis and community engagement, automated monitoring and enforcement of governance decisions, and adaptive governance mechanism design. These agents address low participation, information asymmetry, and complex decision-making challenges while enabling more dynamic and responsive governance systems. 

## VII. ANSWER TO RQ4: AI AGENTS FOR WEB3 SECURITY 

The integration of AI agents into Web3 security represents a transformative shift from traditional static analysis tools to intelligent, adaptive vulnerability detection systems. AI agents leverage natural language processing, machine learning, and automated reasoning to identify complex security vulnerabilities that conventional tools often miss [53]. This section examines the evolution of Web3 security through a comparative analysis of traditional approaches and AI agentenhanced solutions. 

## _A. Traditional Web3 Security Approaches_ 

Before the emergence of LLM-driven agents, automated security auditing in Web3 ecosystems primarily relied on rulebased analysis methods [54], [55], [56]. These approaches, while foundational, exhibit significant limitations that prevent them from replacing human auditors [57]. 

**Rule-Based Security Techniques.** Static analysis tools, such as Slither [56], examine code without execution to detect common vulnerabilities, including reentrancy attacks and integer overflows, through predefined pattern matching. However, they suffer from high false positive rates and struggle to comprehend complex business logic [58]. Dynamic analysis frameworks, such as Mythril [59] and Manticore [60], utilize symbolic execution to explore runtime behavior and identify security flaws; however, they face scalability challenges and state explosion problems with complex contracts [61]. Formal verification approaches, such as Certora [62] and KEVM [63], mathematically prove contract correctness through theorem proving and model checking, providing rigorous security guarantees but requiring extensive manual specification writing and struggling to adapt to evolving vulnerability patterns [57]. **Data-Driven Analysis Techniques.** Previous Web3 vulnerability detection efforts extensively utilized machine learning (ML) and deep learning (DL) approaches [64]. These methods typically train models on pre-constructed, manually annotated Web3 vulnerability datasets to detect known vulnerability patterns. Tools like VulDeePecker [65] and DR-GCN [66] apply neural networks to identify code patterns associated 

with security flaws. However, these approaches consistently face generalization limitations and maintenance challenges, struggling to detect vulnerabilities outside their training distributions and failing to adapt to emerging Web3 attack vectors without significant retraining efforts [67], [68]. 

**Observation 12:** Traditional Web3 security approaches face limitations in generalization, adaptability to emerging threats, and comprehensive business logic analysis. 

## _B. AI Agent-Enabled Web3 Security Technologies_ 

AI agents enhance smart contract auditing through intelligent analysis that identifies sophisticated vulnerabilities and adapts to evolving attack patterns. LLMs analyze smart contract code with human-like reasoning, extending beyond surface-level vulnerabilities to identify complex logical issues through business logic comprehension [3]. 

**Agent-Based Detection Systems.** _GPTScan_ [69] and its commercial implementation, MetaScan [70], address the critical gap where approximately 80% of Web3 security bugs cannot be audited by existing tools through a three-phase methodology: vulnerability decomposition, GPT-based matching, and static confirmation. The system achieves over 90% precision for token contracts while detecting nine previously unknown vulnerabilities missed by human auditors, completing analysis in 14.39 seconds per 1,000 lines of Solidity code at $0.01 USD per scan. 

Advanced AI security systems employ multiple specialized agents working collaboratively to enhance vulnerability detection accuracy. For example, _iAudit_ [71] demonstrates this approach through its two-stage fine-tuning framework using specialized LLM agents: a Detector model for initial vulnerability identification and a Reasoner model for generating explanations, with additional Ranker and Critic agents iteratively selecting vulnerability causes and debating explanations, achieving 91.21% F1 score and 91.11% accuracy on real smart contract vulnerabilities. This collaborative approach mimics human expert auditing while maintaining consistency and eliminating fatigue-related errors. 

**AI-Powered Security Intelligence Platforms.** AI agents extend beyond individual vulnerability detection to create comprehensive security intelligence platforms providing realtime threat monitoring across Web3 ecosystems. ChainGPT’s integrated platform demonstrates this evolution through multiple AI-powered tools: Smart Contract Auditor delivers automated 6-step auditing within 1-2 minutes, CryptoGuard browser extension provides real-time phishing detection, and ChainAware.ai achieves 98% accuracy in fraud detection through continuous transaction monitoring [72]. These platforms integrate with multiple threat intelligence providers, including Forta [73] and GoPlus [74], enabling comprehensive cross-platform security analysis while maintaining real-time monitoring capabilities across various blockchain architectures. 

10 

**Observation 13:** Agent-driven Web3 security overcomes traditional limitations by understanding real business logic, collaborative multi-agent systems that achieve high accuracy, and monitoring platforms that adapt to evolving threats. 

**Answer to RQ4** : AI agents enhance Web3 security through automated smart contract auditing and security intelligence platforms. These agents address traditional auditing limitations by identifying complex logic vulnerabilities and providing cost-effective, scalable security coverage across decentralized ecosystems. 

## VIII. ANSWER TO RQ5: WEB3 TRUST MECHANISMS FOR AI AGENTS 

Web3’s native trust infrastructure enables reliable AI agent operations through cryptographic guarantees, decentralized verification, and transparent execution environments. Unlike traditional centralized systems, Web3-AI Agent leverages blockchain’s trustlessness properties to establish verifiable behavior and secure computation. This section examines how Web3 trust mechanisms enable robust AI agent operations across three dimensions: **cryptographic security** , **decentralized consensus systems** , and **transparent governance mechanisms** . 

## _A. Cryptographic Security and Privacy-Preserving Computation_ 

Trusted Execution Environments (TEE) and advanced cryptographic protocols form the foundation for secure AI agent operations in Web3 ecosystems [7]. Projects like _Mind Network_ utilize Fully Homomorphic Encryption (FHE) to enable AI agents to perform computations on encrypted data while preserving privacy, allowing agents to process sensitive information without exposing underlying data to external parties. This capability is critical for AI agents managing financial portfolios or personal data, where privacy and security are paramount concerns. 

_METAVERSE_ demonstrates a pioneering implementation of AI-driven fundraising within a Trusted Execution Environment, where an AI agent autonomously raised over 30,000 SOL and created liquidity pools with cryptographically verifiable execution. The integration of _ai16z (Eliza)_ framework [75] with _Phala Network_ ’s TEE infrastructure [76] ensures that all AI actions are cryptographically secured and verifiable in real-time, eliminating risks associated with human error or malicious manipulation. 

Multi-party cryptography and game theory mechanisms, as implemented in _Fetch.ai_ ’s blockchain architecture [37], enable secure coordination between multiple AI agents without requiring trust between participants. The platform’s Digital Twin Metropolis maintains immutable records of inter-agent agreements through WebAssembly virtual machines, ensuring that collaborative AI operations remain transparent and verifiable across distributed networks. 

**Observation 14:** Cryptographic security protocols, including TEE and FHE technologies, enable AI agents to perform secure computations on sensitive data while maintaining privacy and verifiable execution in Web3 environments. 

## _B. Decentralized Consensus and Verification Systems_ 

Blockchain consensus mechanisms provide distributed verification for AI agent decisions and actions, creating trustless environments where agent behavior can be verified without centralized authorities. _OriginTrail_ [37] exemplifies this approach through its Decentralized Knowledge Graph (DKG) infrastructure, which enables AI agents to access and contribute to a shared, verified knowledge base while maintaining provenance and authenticity of information sources. 

_NEAR Protocol_ ’s consensus mechanism, _Doomslug_ [36], provides the security foundation for AI-native transactions, ensuring that AI agent operations achieve fast finality while maintaining network security. The platform’s sharded architecture enables AI agents to operate across different network segments while benefiting from the overall security guarantees of the blockchain consensus. 

Smart contract-based verification systems enable autonomous validation of AI agent behaviors and outcomes. _Trias_ focuses on creating trustworthy and reliable intelligent autonomous systems through blockchain-enabled verification mechanisms that ensure AI agents operate according to predefined parameters and security requirements. The platform’s native-application-compatible smart contract execution enables AI agents to interact with traditional computing environments while maintaining blockchain-level security guarantees. Decentralized oracle networks and data verification systems ensure that AI agents receive authentic external data for decision-making. Projects utilizing _OriginTrail_ ’s protocol combine blockchain technology with knowledge graph structures to enable trusted AI applications based on W3C standards [77], providing agents with verifiable real-world data inputs. 

**Observation 15:** Blockchain consensus mechanisms provide distributed verification for AI agent decisions through trustless validation systems, ensuring agent behavior authenticity without requiring centralized authorities. 

## _C. Transparent Governance and Accountability Mechanisms_ 

Web3’s transparent governance enables community oversight of AI agent behavior through blockchain-based systems. Blockchain infrastructure provides immutable audit trails for comprehensive tracking of AI agent actions. Every transaction and decision is permanently recorded on-chain, creating unforgeable histories that enable accountability analysis. This transparency proves crucial for AI agents managing financial assets or making autonomous decisions with economic implications. Specifically, _Commune AI_ ’s protocol [78] 

11 

demonstrates community-driven validation through permissionless frameworks that enable stakeholder oversight while maintaining operational efficiency. The platform’s token-based incentive system rewards community members for contributing to agent validation processes. 

**Observation 16:** Web3’s transparency creates immutable audit trail mechanisms that enable comprehensive accountability for AI agent actions and decisions. 

In conclusion, Web3 trust mechanisms provide a comprehensive foundation for reliable AI agent operations through cryptographic security, decentralized verification, and transparent governance. The integration of TEE technologies, blockchain consensus, and community oversight creates an environment where AI agents operate autonomously while maintaining verifiability and accountability. These mechanisms address fundamental AI deployment challenges by replacing centralized trust with cryptographic guarantees and community governance, enabling truly trustworthy autonomous AI systems in decentralized environments. 

**Answer to RQ5** : Web3 trust mechanisms facilitate reliable AI agent operations by leveraging cryptographic security and privacy-preserving computation, decentralized consensus and verification systems, as well as transparent governance and accountability mechanisms. These mechanisms replace centralized trust with cryptographic guarantees and community governance, creating trustworthy autonomous AI systems in decentralized environments. 

## IX. DISCUSSION 

Our research highlights substantial progress in the integration of Web3 and AI agents, while also identifying critical gaps that require further development. The analysis demonstrates bidirectional integrations between Web3 and AI agent capabilities, yet significant challenges remain in realizing the full potential of this convergence. 

## _A. Emerging Research Directions_ 

Several promising research directions emerge from our analysis, representing underexplored integration points between Web3 and AI agents, which offer opportunities for significant technical and practical advances. 

**Agent Memory and Context Persistence.** Current AI agents face limitations in maintaining long-term memory and contextual understanding across extended interactions [79], [80]. Web3 infrastructure offers novel solutions through decentralized storage and blockchain-based state persistence. Decentralized storage networks, such as IPFS [81], provide tamper-proof memory systems that enable agents to maintain coherent, long-term contexts while ensuring data integrity and availability. Research opportunities include developing Web3native memory architectures that enable agents to maintain verifiable interaction histories, learn from past experiences, and build persistent knowledge graphs. 

**Portable AI Agent Digital Assets.** The integration of AI agents with Web3 digital asset infrastructure enables truly portable agent capabilities. Research directions include developing frameworks that enable agents to own, transfer, and manage digital assets across various platforms and networks. This includes creating agent-specific wallet architectures, cross-chain asset management protocols, and standardized interfaces for agent-asset interactions [75]. Such systems would enable agents to accumulate value, maintain economic relationships, and participate in decentralized economies autonomously. Future work should establish interoperability standards for agent-asset interactions and develop universal interfaces for cross-platform agent operations. 

**Agent Decentralized Identity (Agent DID).** Currently, AI agents operate through user-delegated permissions, which fundamentally limit their autonomy and create a dependency on human wallet infrastructure [82]. There is a need for native identity systems enabling agents to maintain sovereign digital identities on blockchain networks [83]. Research opportunities include creating agent-specific identity protocols, autonomous key management systems, and reputation mechanisms that allow agents to build independent economic and social relationships within Web3 ecosystems. Future research priorities include developing certification processes for autonomous agent identities and establishing liability frameworks for agentcontrolled operations. 

**Decentralized Multi-Agent Coordination.** Web3 infrastructure provides unique opportunities for developing truly decentralized multi-agent coordination systems. Research directions include creating agent-to-agent (A2A) [84] communication protocols, decentralized task allocation mechanisms, and consensus systems specifically designed for agent coordination. This could enable complex collaborative behaviors where multiple agents coordinate directly through blockchain-based protocols without centralized orchestration, opening possibilities for decentralized autonomous organizations (DAOs) composed entirely of AI agents. Future research should focus on developing regulatory frameworks for agents’ coordination mechanisms for cross-border agent operations. 

**Real World Assets (RWA) Integration with AI Agents.** The convergence of RWA tokenization and AI agents represents an underexplored frontier in Web3 finance. Despite the RWA market reaching $23 billion in the first half of 2025 with 260% growth [85], AI agent integration remains limited. Current implementations face challenges including centralization risks in off-chain oracle systems [86], low efficiency in asset verification processes, and interoperability issues between RWA protocols and AI agents. These limitations significantly increase development costs and complexity. Research opportunities include developing autonomous asset management systems, creating AI-driven risk assessment frameworks for tokenized assets, and establishing protocols for agent-controlled real-world asset operations. 

## _B. Technical Challenges and Limitations_ 

Current AI agent implementations in Web3 environments face several fundamental limitations that constrain their effectiveness and adoption. 

12 

**AI Agent Reliability Issues.** Contemporary AI agents exhibit significant reliability challenges that limit their suitability for autonomous Web3 operations. Hallucination remains a critical problem, where agents generate false information or make incorrect decisions based on fabricated data [80]. Additionally, limited context memory restricts agents’ ability to maintain coherent, long-term interactions, particularly in complex financial or governance operations that require extended reasoning chains [3]. High computational costs make real-time agent operations expensive and potentially unsustainable for resource-constrained applications [15]. 

**Security Vulnerabilities.** AI agents face increasing security threats that pose substantial risks for Web3 integration. Prompt injection attacks enable malicious actors to manipulate agent behavior by crafting inputs that override intended instructions [87]. Jailbreaking techniques enable attackers to bypass security measures and cause agents to perform unauthorized actions [88]. These vulnerabilities are particularly concerning in Web3 contexts where agents may control significant financial assets or participate in irreversible blockchain transactions. **Trust and Adoption Barriers.** User trust in AI agents remains a significant barrier to adoption, particularly in highstakes financial applications. Recent research [87] indicates that users express significant reluctance to delegate cryptocurrency asset management to AI agents due to concerns about reliability, security, and accountability. This trust deficit creates a fundamental barrier to the widespread adoption of AI agents, as users prefer to maintain direct control over financial decisions rather than rely on autonomous systems. 

## X. RELATED WORK 

## _A. Blockchain and AI Integration_ 

The convergence of blockchain and artificial intelligence technologies has become a significant research focus across multiple domains. Bhumichai _et al._ [89] provide a comprehensive review of AI-blockchain convergence, identifying key research directions and challenges in combining these technologies. Choi _et al._ [90] present a comprehensive survey categorizing research into two main scenarios: using blockchain to enhance AI capabilities and applying AI to advance blockchain technology. Kayikci _et al._ [5] focus specifically on machine learning and blockchain integration, examining how blockchain provides secure and transparent transaction recording while machine learning enables datadriven decision-making through large-scale data analysis. 

Our work differs from these general surveys in that it specifically examines AI agents as autonomous entities operating within Web3 ecosystems, rather than treating AI as a complementary technology to blockchain. 

## _B. AI Agents in Decentralized Systems_ 

Research on AI agents operating within decentralized systems has explored multi-agent coordination, autonomous economic systems [25], and Decentralized Autonomous Organizations (DAOs). Karim _et al._ [4] survey secure and scalable collaboration among multi-agents in blockchain environments, focusing on GenAI and LLM-based agents that represent the 

forefront of intelligent systems in decentralized environments. Ante [91] investigates autonomous AI agents specifically within decentralized finance, developing a typology based on a qualitative analysis of 306 major crypto AI agents and introducing a quadrant-based framework that distinguishes four archetypal system configurations. Chaffer _et al._ [45] examine decentralized governance frameworks for autonomous AI agents, while Ballandies _et al._ [92] investigate collective intelligence mechanisms in decentralized autonomous organizations. 

While previous work primarily focuses on theoretical frameworks or limited application scenarios, our research extends by providing a systematic market analysis of real-world Web3AI Agent implementations across diverse application domains, revealing actual deployment patterns and market dynamics that existing literature has not captured. 

## XI. CONCLUSION 

This paper presents the first comprehensive systematic analysis of Web3-AI agent integration, examining 133 active projects with $6.9 billion collective market capitalization to reveal how AI agents fundamentally reshape decentralized ecosystems across the landscape, finance, governance, security, and trust dimensions. Our analysis demonstrates that AI agents create bidirectional complementarity by leveraging Web3’s trustless infrastructure for autonomous operations while enhancing Web3 systems through intelligent automation, sophisticated decision-making, and improved accessibility. The ecosystem exhibits pronounced infrastructure dominance, substantial incubation activity, and increasing cross-category convergence, with AI agents transforming DeFi through autonomous trading and portfolio optimization, enhancing governance through intelligent proposal analysis and automated enforcement, strengthening security through advanced vulnerability detection and smart contract auditing, and operating reliably through cryptographic guarantees and decentralized verification mechanisms. While significant challenges remain in scalability, security, ethics, and technical integration, future research directions encompass agent memory persistence, portable digital assets, decentralized identity systems, and multi-agent coordination frameworks. 

## REFERENCES 

- [1] DefiLlama, “DefiLlama,” 2025. [Online]. Available: https://defillama. com/ 

- [2] L. Zhou, X. Xiong, J. Ernstberger, S. Chaliasos, Z. Wang, Y. Wang, K. Qin, R. Wattenhofer, D. Song, and A. Gervais, “SoK: Decentralized Finance (DeFi) Attacks,” in _2023 IEEE Symposium on Security and Privacy (SP)_ . IEEE Computer Society, May 2023, pp. 2444–2461. 

- [3] Z. Zheng, K. Ning, Q. Zhong, J. Chen, W. Chen, L. Guo, W. Wang, and Y. Wang, “Towards an understanding of large language models in software engineering tasks,” _Empirical Software Engineering_ , vol. 30, no. 2, p. 50, Dec. 2024. 

- [4] M. M. Karim, D. H. Van, S. Khan, Q. Qu, and Y. Kholodov, “AI Agents Meet Blockchain: A Survey on Secure and Scalable Collaboration for Multi-Agents,” _Future Internet_ , vol. 17, no. 2, p. 57, 2025. 

- [5] S. Kayikci and T. M. Khoshgoftaar, “Blockchain meets machine learning: A survey,” _Journal of Big Data_ , vol. 11, no. 1, p. 9, 2024. 

- [6] R. Huang, J. Chen, Y. Wang, T. Bi, L. Nie, and Z. Zheng, “An overview of Web3 technology: Infrastructure, applications, and popularity,” _Blockchain: Research and Applications_ , vol. 5, no. 1, p. 100173, Mar. 2024. 

13 

- [7] A. M. S. Saleh, “Blockchain for secure and decentralized artificial intelligence in cybersecurity: A comprehensive review,” _Blockchain: Research and Applications_ , 2024. 

- [8] G. Kassis, “Web3 meets AI: The Rise of the Agentic Web Economy _|_ LinkedIn,” Jan. 2025. [Online]. Available: https://www.linkedin.com/ pulse/web3-meets-ai-rise-agentic-web-economy-george-kassis-9lrvf/ 

- [9] CoinMarketCap, “Top AI & Big Data Tokens by Market Capitalization,” 2025. [Online]. Available: https://coinmarketcap.com/view/ai-big-data/ 

- [10] V. Buterin, “A next-generation smart contract and decentralized application platform,” _whitepaper_ , pp. 3(37):2–1, 2014. 

- [11] Solana, “Solana,” 2025. [Online]. Available: https://solana.com 

- [12] Coinbase, “Base,” 2025. [Online]. Available: https://base.org [13] Optimism, “Optimism,” 2025. [Online]. Available: https://www. optimism.io/ 

- [14] Z. Zheng, S. Xie, H.-N. Dai, W. Chen, X. Chen, J. Weng, and M. Imran, “An overview on smart contracts: Challenges, advances and platforms,” _Future Generation Computer Systems_ , vol. 105, pp. 475–491, Apr. 2020. 

- [15] W. X. Zhao, K. Zhou, J. Li, T. Tang, X. Wang, Y. Hou, Y. Min, B. Zhang, J. Zhang, Z. Dong, Y. Du, C. Yang, Y. Chen, Z. Chen, J. Jiang, R. Ren, Y. Li, X. Tang, Z. Liu, P. Liu, J.-Y. Nie, and J.-R. Wen, “A Survey of Large Language Models,” Mar. 2025. 

- [16] OpenAI, “GPT-4o,” 2024. [Online]. Available: https://openai.com/index/ hello-gpt-4o/ 

- [17] Anthropic, “Claude 3.7 Sonnet,” 20250224. [Online]. Available: https://www.anthropic.com/claude/sonnet 

- [18] Google, “Gemini 2.5 Pro,” Mar. 2025. [Online]. Available: https://blog.google/technology/google-deepmind/ gemini-model-thinking-updates-march-2025/ 

- [19] L. Wang, C. Ma, X. Feng, Z. Zhang, H. Yang, J. Zhang, Z. Chen, J. Tang, X. Chen, Y. Lin, W. X. Zhao, Z. Wei, and J. Wen, “A survey on large language model based autonomous agents,” _Frontiers of Computer Science_ , vol. 18, no. 6, p. 186345, Mar. 2024. 

- [20] Anthropic, “Model Context Protocol,” 2025. [Online]. Available: https://modelcontextprotocol.io/introduction 

- [21] Wayfinder, “Home - Wayfinder,” 2025. [Online]. Available: https: //www.wayfinder.ai/ 

- [22] Nexo, “Nexo: Your Wealth Platform for Digital Assets,” 2025. [Online]. Available: https://nexo.com 

- [23] HeyAnon, “HeyAnon,” 2025. [Online]. Available: https://heyanon.ai/ welcome 

- [24] Armor, “Armor Wallet - AI powered web3 wallet,” 2025. [Online]. Available: https://armorwallet.ai 

- [25] J. Nartey, “Decentralized Finance (DeFi) and AI: Innovations at the Intersection of Blockchain and Artificial Intelligence,” 2024, sSRN Electronic Journal. 

- [26] P. Hunt, “Product Hunt – The best new products in tech.” 2025. [Online]. Available: https://www.producthunt.com 

- [27] I. GitHub, “Github,” 2025. [Online]. Available: https://github.com/home 

- [28] L. A. Goodman, “Snowball Sampling,” _The Annals of Mathematical Statistics_ , vol. 32, no. 1, pp. 148–170, Mar. 1961. 

- [29] C. Wohlin, M. H¨ost, and K. Henningsson, “Empirical Research Methods in Software Engineering,” in _Empirical Methods and Studies in Software Engineering: Experiences from ESERNET_ , R. Conradi and A. I. Wang, Eds. Berlin, Heidelberg: Springer, 2003, pp. 7–23. 

- [30] Z. Wan, X. Xia, D. Lo, J. Chen, X. Luo, and X. Yang, “Smart Contract Security: A Practitioners’ Perspective,” in _Proceedings of the 43rd International Conference on Software Engineering_ , ser. ICSE ’21. IEEE Press, 2021, pp. 1410–1422. 

- [31] Z. Wan, X. Xia, A. E. Hassan, D. Lo, J. Yin, and X. Yang, “Perceptions, Expectations, and Challenges in Defect Prediction,” _IEEE Transactions on Software Engineering_ , vol. 46, no. 11, pp. 1241–1266, Nov. 2020. 

- [32] Wikipedia, “Decentralized physical infrastructure network,” _Wikipedia_ , Jul. 2025. 

- [33] Arbitrum, “Arbitrum — Build anything today,” 2025. [Online]. Available: https://arbitrum.io/ 

- [34] Sui, “Sui _|_ Deliver the Benefits of Web3 with the Ease of Web2,” 2025. [Online]. Available: https://sui.io/ 

- [35] Polygon, “Polygon,” 2025. [Online]. Available: https://polygon. technology/ 

- [36] NEAR, “The Blockchain for AI NEAR,” 2025. [Online]. Available: https://near.org// 

- [37] Fetch.ai, “Fetch.ai - Build. Discover. Transact.” 2025. [Online]. Available: https://fetch.ai/ 

- [38] S. Lee, “We Aren’t Ready For AI Agent Computing Demands.” [Online]. Available: https://www.forbes.com/sites/digital-assets/2025/ 04/17/we-arent-ready-for-ai-agent-computing-demands/ 

- [39] Griffain, “Griffain,” 2025. [Online]. Available: https://griffain.com 

- [40] oneclick, “Home for the best DeFi yields,” 2025. [Online]. Available: https://defi.oneclick.fi/ 

- [41] S. AI, “Sharpe AI: Smart Crypto Trading Platform _|_ AI-Powered Analysis,” 2025. [Online]. Available: https://www.sharpe.ai/ 

- [42] aixbt, “AIXBT,” 2025. [Online]. Available: https://aixbt.tech/ 

- [43] A. AI, “Assemble AI: AI Agent Revolutionizing Web3 Journalism.” 2025. [Online]. Available: https://about.ns3.ai/ 

- [44] D. AI, “DexCheck AI: AI Powered Crypto Analytics Terminal & Web3 Infrastructure,” 2025. [Online]. Available: https://dexcheck.ai/ 

- [45] T. J. Chaffer, C. v. G. II, B. Okusanya, D. Cotlage, and J. Goldston, “Decentralized Governance of Autonomous AI Agents,” 2025. 

- [46] E. Ostrom, _Governing the commons: The evolution of institutions for collective action_ . Cambridge university press, 1990. 

- [47] M. S. Harre, “From Firms to Computation: AI Governance and the Evolution of Institutions,” Jul. 2025. 

- [48] ChainGPT, “Explore AI Agents Ecosystem for Web3 _|_ ChainGPT,” 2025. [Online]. Available: https://www.chaingpt.org/ai-agents 

- [49] Olas, “Olas _|_ Co-own AI,” 2025. [Online]. Available: https: //olas.network 

- [50] A. L. Intelligence, “Introduction _|_ AI News Swarm,” Mar. 2025. [Online]. Available: https://ainews.aliagents.ai 

- [51] MyShell, “MyShell AI _|_ Build, Share, and Own AI Agent,” 2025. [Online]. Available: https://myshell.ai/ 

- [52] VIRTUAL, “Virtuals Protocol _|_ Society of AI Agents,” 2025. [Online]. Available: https://virtuals.io/ 

- [53] S. Hu, T. Huang, F. Ilhan, S. F. Tekin, and L. Liu, “ Large Language Model-Powered Smart Contract Vulnerability Detection: New Perspectives ,” in _2023 IEEE Conference on Trust, Privacy and Security in Intelligent Systems and Applications (TPS)_ . IEEE Computer Society, 2023, pp. 297–306. [Online]. Available: https: //doi.ieeecomputersociety.org/10.1109/TPS-ISA58951.2023.00044 

- [54] I. Nikoli´c, A. Kolluri, I. Sergey, P. Saxena, and A. Hobor, “Finding The Greedy, Prodigal, and Suicidal Contracts at Scale,” in _Proceedings of the 34th Annual Computer Security Applications Conference_ , ser. ACSAC ’18. New York, NY, USA: Association for Computing Machinery, Dec. 2018, pp. 653–663. 

- [55] P. Tsankov, A. Dan, D. Drachsler-Cohen, A. Gervais, F. B¨unzli, and M. Vechev, “Securify: Practical Security Analysis of Smart Contracts,” in _Proceedings of the 2018 ACM SIGSAC Conference on Computer and Communications Security_ . Toronto Canada: ACM, Oct. 2018, pp. 67– 82. 

- [56] J. Feist, G. Greico, A. Groce, and ACM, “Slither: A Static Analysis Framework For Smart Contracts,” in _2019 IEEE/ACM 2ND INTERNATIONAL WORKSHOP ON EMERGING TRENDS IN SOFTWARE ENGINEERING FOR BLOCKCHAIN (WETSEB 2019)_ , 2019, pp. 8–15. 

- [57] C. Chen, J. Su, J. Chen, Y. Wang, T. Bi, J. Yu, Y. Wang, X. Lin, T. Chen, and Z. Zheng, “When ChatGPT Meets Smart Contract Vulnerability Detection: How Far Are We?” _ACM Trans. Softw. Eng. Methodol._ , Nov. 2024. 

- [58] J. Chen, Y. Shen, J. Zhang, Z. Li, J. Grundy, Z. Shao, Y. Wang, J. Wang, T. Chen, and Z. Zheng, “FORGE: An LLM-driven Framework for LargeScale Smart Contract Vulnerability Dataset Construction,” 2025. 

- [59] Consensys, “Consensys/mythril: Mythril is a symbolic-execution-based securty analysis tool for EVM bytecode. It detects security vulnerabilities in smart contracts built for Ethereum and other EVM-compatible blockchains.” 2018. 

- [60] M. Mossberg, F. Manzano, E. Hennenfent, A. Groce, G. Grieco, J. Feist, T. Brunson, and A. Dinaburg, “Manticore: A User-Friendly Symbolic Execution Framework for Binaries and Smart Contracts,” in _2019 34th IEEE/ACM International Conference on Automated Software Engineering (ASE)_ , Nov. 2019, pp. 1186–1189. 

- [61] S. Chaliasos, M. A. Charalambous, L. Zhou, R. Galanopoulou, A. Gervais, D. Mitropoulos, and B. Livshits, “Smart Contract and DeFi Security Tools: Do They Meet the Needs of Practitioners?” in _Proceedings of the IEEE/ACM 46th International Conference on Software Engineering_ , ser. ICSE ’24. New York, NY, USA: Association for Computing Machinery, Feb. 2024, pp. 1–13. 

- [62] Certora, “Certora Prover Documentation — Certora Prover Documentation 0.0 documentation,” 2025. [Online]. Available: https://docs.certora.com/en/latest/ 

- [63] runtimeverification, “KEVM _|_ evm-semantics,” Jul. 2025. 

- [64] Z. Wang, H. Jin, W. Dai, K.-K. R. Choo, and D. Zou, “Ethereum smart contract security research: Survey and future research opportunities,” _Frontiers of Computer Science_ , vol. 15, no. 2, p. 152802, Apr. 2021. 

- [65] D. Zou, S. Wang, S. Xu, Z. Li, and H. Jin, “VulDeePecker: A Deep Learning-Based System for Multiclass Vulnerability Detection,” _IEEE_ 

14 

_Transactions on Dependable and Secure Computing_ , vol. 18, no. 5, pp. 2224–2236, Sep. 2021. 

- [66] Y. Zhuang, Z. Liu, P. Qian, Q. Liu, X. Wang, and Q. He, “Smart Contract Vulnerability Detection using Graph Neural Network,” in _Twenty-Ninth International Joint Conference on Artificial Intelligence and Seventeenth Pacific Rim International Conference on Artificial Intelligence {IJCAIPRICAI-20}_ . Yokohama, Japan: International Joint Conferences on Artificial Intelligence Organization, Jul. 2020, pp. 3283–3290. 

- [67] Y. Li, S. Wang, and T. N. Nguyen, “Vulnerability detection with finegrained interpretations,” in _Proceedings of the 29th ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software Engineering_ , ser. ESEC/FSE 2021. New York, NY, USA: Association for Computing Machinery, Aug. 2021, pp. 292– 303. 

- [68] Y. Shen, K. Li, L. Mao, W. Li, and X. Li, “IntelliCon: Confidence-Based Approach for Fine-Grained Vulnerability Analysis in Smart Contracts,” in _Blockchain and Trustworthy Systems_ , J. Chen, B. Wen, and T. Chen, Eds. Singapore: Springer Nature, 2024, pp. 45–59. 

- [69] Y. Sun, D. Wu, Y. Xue, H. Liu, H. Wang, Z. Xu, X. Xie, and Y. Liu, “GPTScan: Detecting Logic Vulnerabilities in Smart Contracts by Combining GPT with Program Analysis,” in _Proceedings of the IEEE/ACM 46th International Conference on Software Engineering_ , ser. ICSE ’24. Association for Computing Machinery, 2024, pp. 3048– 3060. [Online]. Available: https://doi.org/10.1145/3597503.3639117 

   - [87] H. Song, Y. Shen, W. Luo, L. Guo, T. Chen, J. Wang, B. Li, X. Zhang, and J. Chen, “Beyond the Protocol: Unveiling Attack Vectors in the Model Context Protocol Ecosystem,” 2025. 

   - [88] J. Chen, Q. Zhong, Y. Wang, K. Ning, Y. Liu, Z. Xu, Z. Zhao, T. Chen, and Z. Zheng, “RMCBench: Benchmarking Large Language Models’ Resistance to Malicious Code,” in _Proceedings of the 39th IEEE/ACM International Conference on Automated Software Engineering_ , ser. ASE ’24. Association for Computing Machinery, 2024, pp. 995–1006. 

   - [89] D. Bhumichai, C. Smiliotopoulos, R. Benton, G. Kambourakis, and D. Damopoulos, “The Convergence of Artificial Intelligence and Blockchain: The State of Play and the Road Ahead,” _Information_ , vol. 15, no. 5, p. 268, 2024. 

   - [90] N. Choi and H. Kim, “Technological Convergence of Blockchain and Artificial Intelligence: A Review and Challenges,” _Electronics_ , vol. 14, no. 1, p. 84, 2024. 

   - [91] L. Ante, “Autonomous AI Agents in Decentralized Finance: Market Dynamics, Application Areas, and Theoretical Implications,” 2024, sSRN Electronic Journal. 

   - [92] M. C. Ballandies, D. Carpentras, and E. Pournaras, “DAOs of Collective Intelligence? Unraveling the Complexity of Blockchain Governance in Decentralized Autonomous Organizations,” 2024. 

- [70] MetaTrust, “MetaScan - Automated & AI-Powered Security Assessment for Smart Contracts,” 2025. [Online]. Available: https://metatrust.io/ metascan 

- [71] W. Ma, D. Wu, Y. Sun, T. Wang, S. Liu, J. Zhang, Y. Xue, and Y. Liu, “ Combining Fine-Tuning and LLM-Based Agents for Intuitive Smart Contract Auditing with Justifications ,” in _2025 IEEE/ACM 47th International Conference on Software Engineering (ICSE)_ . Los Alamitos, CA, USA: IEEE Computer Society, May 2025, pp. 1742– 1754. [Online]. Available: https://doi.ieeecomputersociety.org/10.1109/ ICSE55347.2025.00027 

- [72] ChainAware.ai, “ChainAware.ai - The Web3 AI Agents,” 2025. [Online]. Available: http://chainaware.ai 

- [73] Forta, “Forta,” 2025. [Online]. Available: https://www.forta.org/ 

- [74] GoPlus, “Overview _|_ GoPlus Security,” Jun. 2025. [Online]. Available: https://whitepaper.gopluslabs.io/goplus-network 

- [75] S. Walters, S. Gao, S. Nerd, F. Da, W. Williams, T.-C. Meng, H. Han, F. He, A. Zhang, M. Wu, T. Shen, M. Hu, and J. Yan, “Eliza: A Web3 friendly AI Agent Operating System,” 2025. 

- [76] Phala, “Phala Network,” 2025. [Online]. Available: https://docs.phala. network/phala-cloud/getting-started/overview 

- [77] OriginTrail, “OriginTrail: Powering the shift to human-centric AI - OriginTrail,” 2025. [Online]. Available: https://origintrail.io/ 

- [78] C. AI, “Commune AI,” 2025. [Online]. Available: https://communeai. org/docs/getting-started/intro 

- [79] N. F. Liu, K. Lin, J. Hewitt, A. Paranjape, M. Bevilacqua, F. Petroni, and P. Liang, “Lost in the Middle: How Language Models Use Long Contexts,” _Transactions of the Association for Computational Linguistics_ , vol. 12, pp. 157–173, 2024. 

- [80] ZhangZiyao, WangChong, WangYanlin, ShiEnsheng, MaYuchi, ZhongWanjun, ChenJiachi, MaoMingzhi, and ZhengZibin, “LLM Hallucinations in Practical Code Generation: Phenomena, Mechanism, and Mitigation,” _Proceedings of the ACM on Software Engineering_ , 2025. 

- [81] J. Benet, “IPFS - Content Addressed, Versioned, P2P File System,” 2014. [82] K. Nagware, “Verifiable AI - How Decentralized Identity (DID) Empowers Agentic AI _|_ LinkedIn,” Apr. 2025. [Online]. Available: https://www.linkedin.com/pulse/ verifiable-ai-how-decentralized-identity-did-empowers-nagware-dnx5f/ 

- [83] O. Avellaneda, A. Bachmann, A. Barbir, J. Brenan, P. Dingle, K. H. Duffy, E. Maler, D. Reed, and M. Sporny, “Decentralized identity: Where did it come from and where is it going?” _IEEE Communications Standards Magazine_ , vol. 3, no. 4, pp. 10–13, 2019. 

- [84] Google, “Announcing the Agent2Agent Protocol (A2A)Google Developers Blog,” 2025. [Online]. Available: https://developers. googleblog.com/en/a2a-a-new-era-of-agent-interoperability/ 

- [85] Z. Vardai, “RWA token market grows 260% in 2025 as firms embrace regulating crypto,” 2025. [Online]. Available: https://cointelegraph.com/ news/rwa-token-market-bitcoin-adoption-2025 

- [86] S. Chen, M. Jiang, and X. Luo, “Exploring the Security Issues of Real World Assets (RWA),” in _Proceedings of the Workshop on Decentralized Finance and Security_ , ser. DeFi ’24. Association for Computing Machinery, 2024, pp. 31–40. 

