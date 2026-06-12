_**2020 IEEE Pune Section International Conference (PuneCon) Vishwakarma Institute of Technology, Pune India. Dec 16-18, 2020**_ 

## Machine Learning Model Cards Transparency Review 

## Using model card toolkit 

Abhishek Wadhwani _Dept. of Information Technology Oriental College of Technology,_ Bhopal Madhya Pradesh, India abhishekwadhwani2@gmail.com 

Priyank Jain _Dept. of CSE MANIT Bhopal_ Madhya Pradesh, India priyankjain1984@gmail.com 

_**Abstract**_ **—In our day to day life, we rely on information that is provided by product makers to make rightful choices such as the nutritional content of food, warnings in medications, strength parameters of a constructed road, etc but when it comes to AI there’s has not been any such provided information. The machine learning models are very often distributed without a proper clear understanding of how it functions, i.e. under what conditions would it perform the best and most consistently, whether or not it has blind spots, and, if so, then where are they.** 

**Model cards are a very recent and hot topic of research. In Machine Learning (ML), transparency with model cards is relevant as it affects a wide range of domains, from health care to finance and jobs, etc. This research paper presents the importance of model cards and transparency issues.** 

_**Index Terms—Model documentation, disaggregated evaluation, fairness in AI, Machine Learning model evaluation, ethics, datasheets.**_ 

## I. INTRODUCTION 

Model Cards are a vision proposed by googling this year 2020 in their research that organizes these essential facts about an ML model in a structured manner. 

In ML, Model Card's transparency is important because it affects a wide variety of domains from health care to finance and employment, etc. The information needed by the downstream users can vary and often it does than what was intended by the model makers. If transparency is maintained by listing essential facts of the model the user can choose the appropriate one for their use case. 

Over the year these cards are coming more and more in use as the MediaPipe team which creates state of art models on computer vision has included model cards in their repositories. Creating these sorts of cards take substantial time and effort and often require detailed analysis of the performance and data the model is trained on which is a tedious task. Also, model creators may want to specify the model’s limitations or use cases in a form understandable to the users. 

In order to streamline the creation of these model cards google has shared a model card toolkit (MCT) which is collection of tools that aid developers in compiling the information that goes into the model card [15]. 

Increased transparency for a machine learning model can benefit experts and non-experts alike. 

Developers may use them in designing applications that emphasize a model’s strengths while avoiding or informing end- users of its weaknesses. Users like analysts, journalists might provide insights to make some complex technology easier to explain to a general audience and also help advocacy groups to better understand AI’s impact on themselves. 

The model card toolkit (MCT) provides a schema in JSON with the fields to be included in model cards shown in figure 

1. The model’s provenance information is stored within ML Metadata (MLMD) which is automatically populated in JSON with the relevant information. An API for the reverse process is also provided that is it fetches this stored information in JSON and visualize it in the form of a card whose syntax is in figure 

2. The model card creator can choose what graphs and metrics to display in the final Card and can also create supplementary information such as limitations, tradeoffs, intended use case, etc. This sort of information is crucial for the developers to choose from. Standard User Interface (UI) templates are provided in the MCT but a user can also create their template in HTML if wishes to visualize the information differently. 

At present MCT is available to TensorFlow extended (TFX) 

users as open source and also to google cloud platform users. Those who are not using TensorFlow can also leverage model cards by custom HTML template creation. 

## II. BACKGROUND 

Standardized methods for benchmarking numerous differing systems under various conditions have been developed by most mature industries. An example being, as mentioned in [1], the electronic hardware industry provides datasheets along with the performances of  the  components under  differing  test conditions, characterized in detail. Contrastingly, even with the  wide reach  and  influence  of machine  learning  models. There exist no standardized stress tests which are performed on systems based on machine learning, neither are the results of these tests reported in standardized formats. Normalized structures have been proposed by specialists for correspondence of datasets attributes utilized in AI [2,1,3] to help clients in understanding the setting in which the datasets should be utilized. We propose a normalized technique, focusing on 

**978-1-7281-9600-8/20/$31.00 ©2020 IEEE** 

133 

Fig. 1. Model Card Toolkit 

corresponding   pieces   of   AI   models,   for   assessing   the exhibition of human-driven models. Partitioned into unitary and intersectional gatherings, for instance, social, segment, or phenotypic  populace  and  so  on.  A  system  alluded  to as "Model Cards" could introduce a comparable assessment notwithstanding certain  conditions like expected  use.  Other than AI, the requirement for such announcing of results dependent on the populace, as proposed here, has gotten continuously clear. For instance, in accident tests for vehicles, fakers having prototypical female qualities were presented simply after specialists found that ladies were more inclined to enduring genuine head wounds in certifiable side effects than men [4]. Likewise, drugs that are created dependent on the aftereffects of clinical preliminaries on male subjects solely have caused overdosing in ladies [5, 6]. In 1998, the U.S. Food and Drug Administration requested that consequences of clinical preliminaries should be recognized by gatherings, as age,   race,   and   sex   [7]. While  such   a  populace  based examination of triumphs and mistakes can be accommodated "men", "ladies" and other such unitary gatherings and "non- parallel" sex gatherings, they ought to likewise be given of an intersectional gathering, taking a gander at least two qualities immediately, for example, sexual orientation and age simultaneously. Intersectional examinations are related to the intersectionality hypothesis; it portrays how particular encounters associated with qualities like race, sex and so on independently don't precisely mirror their communications [8]. Kimberl'e Crenshaw, pioneer of intersectional research basic race hypothesis, talked about the account of Emma DeGraffenreid, who had been essential for a bombed claim against   General   engines   in   1976,   guaranteed   that   the organization victimized individuals of color in their recruiting rehearses.  As  they  would  see  it,  the  appointed authorities noticed that since numerous ladies were being recruited for secretarial jobs, and many individuals of color for production line jobs  were employed  by  General Motors, they couldn't have oppressed individuals of color. The judges in their court of opinion noted that since for secretarial positions many women were hired and for factory roles, many black men were hired hence General motors could not have discriminated against black women. What these judges missed was that only white women were hired for those roles and thus black women like Emma DeGraffenreid got no opportunity of being utilized at General Motors. The significance of intersectional investigations is featured in this model that is observational examinations that underscore the cooperation between different segment classes including race, sexual orientation, and age. 

Fig. 2. Model Card Sections 

## III. MOTIVATION 

With the rapid increase in ML technology, there is a growing divide among those who are looking to use trained machine learning models in a particular context. 

The model cards are the first step with an aim to standardize the ethical practice and reporting for AI models that allowed- users or stakeholders to compare models before deployment for their specific use case. This helps in decision making saves money and also helps to find naive users the right product for themselves in the AI market. To guarantee that model cards are factually exact and unquestionable, the assessment datasets ought not exclusively to be illustrative of the model's run of the mill use cases yet in addition foreseen test situations and testing cases. For example, if a model is proposed for use in a work environment that is phenotypically and demographically homogeneous, and prepared on a dataset that is illustrative of the normal use case, it might be 

134 

important to assess that model on two assessment sets: one that coordinates the work environment's populace, and another set that contains people that may be all the more trying for the model, (for example, youngsters, the old, and individuals from outside the common working environment populace). This system can feature obsessive issues that may not be obvious in more normal testing. Outlined below are some use cases for different kinds of stakeholders: 

1) AI and ML practitioners may better understand how well the model could operate and monitor its excellence for the expected use cases. 

2) System developers can compare the results of the experiment with other models in the same space and decide to train their system. 

3) Software engineers chipping away at the administrations should utilize the model expectations to educate their plan and usage choices. 

4) Policymakers ought to see how a program of AI would fizzle or prevail in manners that influence individuals. 

5) Companies will educate choices about the execution concerning innovation with incorporating AI. 

6) ML-proficient people can brief on the different choices for calibrating, model mix, or extra standards and imperatives to help clergyman models for expected use cases without requiring specialized mastery. 

7) Individuals influenced who can experience the ill effects of a model can help clarify how this works or use card subtleties to look for cures. 

This approach not only strengthens product awareness and helps streamline decision-making processes for the stakeholders involved, but also facilitates forward-looking model research Methodologies. For example, slicing the assessment through classes works to expose errors that could fall unfairly on other categories of individuals, which is compatible with other recent conceptions of mathematical justice (discussed more often in the example model card in figure 3). Including community feedback as part of the monitoring  process would allow members to continue evaluating the consistency and equality of potential machine learning performance. Therefore, beyond helping decisionmaking processes to assess the suitability of a specified ML algorithm in a particular environment, application reporting is an approach to responsible, open, and accountable activities in ML. Product issuing individuals and groups can also be required to include information on the model card because it allows future application consumers to be more aware of which versions are suitable for their particular purposes. If monitoring of model cards becomes Standard, potential users can well informedly evaluate various models. Furthermore, findings on many separate test databases can benefit prospective consumers, while examination Datasets fit for analyzed assessments are not yet common. Future studies may involve the development of reliable assessment databases and protocols for the forms of quantified assessment that we recommend in this review, for example, through incorporating differential privacy mechanisms [9] such that individuals can not be identified uniquely through their characteristics. Preferably, the model card would contain as much data about the preparation information as the assessment information. Notwithstanding, there may be situations where it isn't plausible to give this degree of point 

by point data about the preparation information. For instance, the information might be exclusive  or  require a non-revelation  arrangement. In these cases, we advocate for fundamental insights concerning the dispersions over gatherings in the information, just as whatever other subtleties that could advise partners on the sorts regarding inclinations the model may have encoded. 

## IV. TYPES OF MODEL CARDS 

This work present types of model cards into two categories: a classification system based on images, and a scoring system based on text. 

## _A. Smiling Classifier_ 

To show an illustration of a model card for an image classification issue, research work uses the public CelebA dataset [10] to analyze a qualified ”smiling” classifier’s success across both age and gender. Such findings suggest a few possible problems: the false discovery rate is also much higher for older adults than that for other categories. That implies several predictions classify improperly older men, even if they’re not smiling. Males, on the other side (overall average), have a higher false-negative score, which implies that a number of the people genuinely smiling in the pictures are wrongly identified as getting no smile. The outcomes of these analyzes provide the model with an overview into context not well fit for that. The eqution1 represents the linear regression method for smiling classifier. 

**==> picture [205 x 32] intentionally omitted <==**

This could not be recommended, for example, applying the concept to a broad audience community, and can be the most effective when detecting the intensity of a smile relevant than detecting its absence (in an example application which recognizes the ’fun moments’ automatically in images). For example, additional fine tuning of photos of older people may help build a more inclusive representation between categories. 

## _B. Toxicity Scoring_ 

The second illustration gives the Perspective API layout card TOXICITY classifier built to identify ’toxicity’ in the text [11] and shown in figure 3. To validate the model, this work uses an open-source, synthetically generated test collection of identity phrase templates published in [12], intersectional edition. This work presenting two iterations of the predictive analysis: one for TOXICITY v. 1, this model’s original iteration, and another for TOXICITY v. 5, the newest version. This model card highlights the dramatic changes models will evolve with time and the value of keeping a model card updated for each new product launches. For many words, TOXICITY v. 1 has low efficiency, especially ”lesbian,” ”gay” and ”homosexual”. This is consistent with what some users found in the initial TOXICITY model, as reported in [13] by the team behind Perspective API. In order to build a more balanced outcome in TOXICITY v. 5, the Insight API team discusses the prejudice reduction methods added to the TOXICITY v. 1 model. By making model cards a regular part of API launches, teams like the Perspective API team that earlier 

135 

identify and reduce some of those biases. Equation 2, 3,4 and 5 represents SVM model for TOXICITY scoring. 

**==> picture [106 x 9] intentionally omitted <==**

## V. CONCLUSIONS 

This research paper addresses major issues of model cards and their transparency. Transparency in itself bring challenges such as the one of intellectual property. There is a need to ensure the delivery of meaningful lucid data without exposing sensitive, proprietary data or trade secrets. The model creators must adapt to federated learning or differential privacy to avoid leaks. Also, since model cards express qualitative information as well which presents communication challenges such as the specification of the type of input, limitations etc. 

- Fig. 4. Model Card type for two versions of Perspective API’s toxicity detector. 

## REFERENCES 

   - [1] Emily M. Bender and Batya Friedman. 2018.  “Data  Statements for NLP: Toward Mitigating System Bias and Enabling Better Science”. Transactions of the ACL (TACL) (2018). 

   - [2] Timnit Gebru, Jamie Morgenstern, Briana Vecchione, Jennifer  Wortman Vaughan, Hanna M. Wallach, Hal Daume´ III, and Kate Crawford. 2018. Datasheets for Datasets. CoRR abs/1803.09010 (2018). http://arxiv.org/abs/1803.09010. 

   - [3] Sarah Holland, Ahmed Hosny, Sarah Newman,  Joshua  Joseph,  and Kasia Chmielinski. 2018. The Dataset Nutrition Label: A Framework To Drive Higher Data Quality Standards. CoRR abs/1805.03677 (2018). http://arxiv.org/abs/1805.03677 

   - [4] Food and Drug Administration. 1989. Guidance for the Study of Drugs Likely to Be Used in the Elderly. (1989). 

   - [5] U.S. Food and Drug Administration. 2013. FDA Drug Safety Communication: Risk of  next-morning  impairment  after  use  of  insomnia  drugs;  FDA  requires  lower  recommended  doses  for  certain  drugs  containing  zolpidem  (Ambien,  Ambien   CR,   Edluar, and Zolpimist). (2013). https://web.archive.org/web/20170428150213/ https://www.fda.gov/drugs/drugsafety/ucm352085.htm. 

   - [6] IHS (Insurance Institute for Highway Safety: Highway Loss Data Institute). 2003. Special Issue: Side Impact Crashworthiness. Status Report 38, 7 (2003). 

- Fig. 3. Model Card type for a trained and validated smile detector on the CelebA datasets. 

- [7] Amy Westervelt. 2018. The medical research gender gap: how excluding women from clinical trials is hurting our health. (2018) 

- [8] Kimberle Crenshaw. 1989. Demarginalizing the intersection of race and sex: A black feminist critique of antidiscrimination doctrine, feminist theory and antiracist politics. U. Chi. Legal F. (1989), 139 

- [9] Cynthia Dwork. 2008. Differential Privacy: A Survey of Results. In Theory and Applications of Models of Computation, Manindra 

136 

Agrawal, Dingzhu Du, Zhenhua Duan, and Angsheng Li (Eds.). Springer Berlin Heidelberg, Berlin, Heidelberg, 1–19. 

- [10] Ziwei Liu, Ping Luo, Xiaogang Wang, and Xiaoou Tang. 2015. Deep Learning Face Attributes in the Wild. In Proceedings of International Conference on Computer Vision (ICCV). 

- [11] Jigsaw. 2017. Perspective API. (2017). https://www.perspectiveapi.com/ [12]Lucas Dixon, John Li, Jeffrey Sorensen, Nithum Thain, and Lucy 

- [12] Vasserman. 2018. Measuring and Mitigating Unintended Bias in Text Classification. Proceedings of the AAAI/ACM Conference on AI, Ethics, and Society (2018). 

- [13] Lucy Vasserman, John Li, CJ Adams, and Lucas Dixon. 2018. Unintended bias and names of frequently targeted groups. Medium (2018). https://medium.com/the-false-positive/ unintended-bias-andnames-of-frequently-targeted-groups-8e0b81f80a23 

- [14] https://github.com/tensorflow/model-cardtoolkit/blob/master/modelcardtoolkit/documentation/examples/ MLMDModelCardToolkitDemo.ipynb 

- [15] https://ai.googleblog.com/2020/07/introducing-model-card-toolkitfor.html 

137 

