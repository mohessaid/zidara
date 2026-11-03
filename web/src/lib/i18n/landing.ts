// The landing page's words (ADR 0014), in the app's three languages. Every figure comes from a
// row marked "verified" in docs/impact-facts.md, and each one names its source on the page.
// House style for this page: no em or en dashes; plain sentences; one label per intent.
import type { Locale } from "./i18n.ts";

export type Source = { label: string; href: string };
export type Why = { q: string; a: string; cite: number[] };
export type Tile = { title: string; body: string };
export type Qa = { q: string; a: string };

export type LandingCopy = {
  nav: { why: string; how: string; proof: string; play: string; menu: string };
  hero: { before: string; accent: string; after: string; lead: string; more: string };
  live: {
    label: string;
    story: string;
    level: string;
    state: Record<"achieved" | "in_progress" | "not_started" | "needs_support", string>;
    open: string;
    after: string;
    on: string;
    verdict: Record<"mastery" | "cambridge-style", string>;
    engine: string;
    band: string;
    loading: string;
    failed: string;
  };
  facts: string[];
  why: { title: string; lead: string; steps: Why[]; rootTitle: string; root: string; sources: string };
  sources: Source[];
  how: {
    title: string;
    lead: string;
    blocks: Tile[];
    blocksTitle: string;
    layers: Tile[];
    layersTitle: string;
    tiles: Tile[];
    flow: string[];
  };
  made: { title: string; lead: string; shots: { src: string; alt: string; caption: string }[] };
  proof: { title: string; lead: string; stats: { value: string; label: string }[]; code: string };
  faq: { title: string; items: Qa[] };
  end: { title: string; lead: string; classes: string };
  footer: { line: string; made: string; code: string };
};

const SOURCES: Source[] = [
  { label: "World Bank, UNESCO, UNICEF et al., The State of Global Learning Poverty: 2022 Update", href: "https://www.unicef.org/media/122921/file/StateofLearningPoverty2022.pdf" },
  { label: "IEA, TIMSS 2023 International Results in Mathematics, Exhibit 1.1.4", href: "https://timss2023.org/?p=5644" },
  { label: "W3Techs, Usage statistics of content languages for websites (18 October 2026)", href: "https://w3techs.com/technologies/overview/content_language" },
  { label: "GSMA, The State of Mobile Internet Connectivity 2025", href: "https://www.gsma.com/somic/wp-content/uploads/2025/09/The-State-of-Mobile-Internet-Connectivity-2025-Trends-in-Mobile-Internet-Connectivity.pdf" },
  { label: "UNESCO and Teacher Task Force, Global Report on Teachers (UN News, 29 February 2024)", href: "https://news.un.org/en/story/2024/02/1147067" },
];

const en: LandingCopy = {
  nav: { why: "Why", how: "How it works", proof: "Proof", play: "Play a lesson", menu: "Sections" },
  hero: {
    before: "The learning game that follows ",
    accent: "your school's rules",
    after: ".",
    lead: "Zidara turns a school's own way of teaching into a game children play on the phones they have, even offline.",
    more: "How it works",
  },
  live: {
    label: "School rules",
    story: "One learner answered the first fractions quiz once, and got both questions right. Switch the rules:",
    level: "Level {n}",
    state: { achieved: "Mastered", in_progress: "In progress", not_started: "Not started", needs_support: "Needs support" },
    open: "Open",
    after: "Opens after level {n}",
    on: "Opens on {date}",
    verdict: {
      mastery: "Mastery: two right answers prove it. Level 1 is mastered and level 2 opens.",
      "cambridge-style": "Cambridge-style: the same answers earn a top band, but practice is not an assessment, and level 3 waits for the scheme of work.",
    },
    engine: "Live: the real Zidara engine, compiled to WebAssembly, is judging this in your browser.",
    band: "band {n} of {of}",
    loading: "Starting the engine…",
    failed: "The engine could not start in this browser. The lesson still works.",
  },
  facts: ["One Rust engine for web, Android and iOS", "Works offline", "Arabic, French and English", "WCAG 2.2 AA", "Public source code"],
  why: {
    title: "Five whys, down to the root",
    lead: "We asked why so many children are not learning, and kept asking.",
    steps: [
      { q: "Why are so many children not learning?", a: "An estimated 7 in 10 ten-year-olds in low- and middle-income countries cannot read and understand a simple text. In maths, over half of fourth graders in Kuwait and Morocco fall below the lowest international benchmark.", cite: [1, 2] },
      { q: "Why doesn't digital learning close the gap?", a: "Most of it is made for someone else. Arabic is 0.6% of the web's content, under half of people in the Middle East and North Africa use mobile internet, and 3 in 10 of those who do still use 3G or basic phones.", cite: [3, 4] },
      { q: "Why don't schools adapt the tools they have?", a: "Because the way of teaching is built in. A platform carries one idea of progress, and changing it means a new product. Teachers fill the gap by hand, and the world needs 44 million more of them by 2030.", cite: [5] },
      { q: "Why is the way of teaching built in?", a: "Because nobody wrote pedagogy down as something software can run. Its rules live in documents and in teachers' heads, so every app rebuilds a simplified copy.", cite: [] },
      { q: "Why does that matter for a child?", a: "Software that does not know how teaching works rewards taps instead of learning, and AI that does not know the rules guesses.", cite: [] },
    ],
    rootTitle: "So we built the missing piece: pedagogy as software.",
    root: "Zidara makes a school's rules something the engine runs, tests and can swap. The game, the rewards and the AI all follow them.",
    sources: "Sources",
  },
  sources: SOURCES,
  how: {
    title: "One engine. Any pedagogy. Every audience.",
    lead: "Every way of teaching we studied reduces to six building blocks. Only the rules change.",
    blocksTitle: "Six building blocks",
    blocks: [
      { title: "Outcomes", body: "What a learner should be able to do" },
      { title: "Activities", body: "What the learner does" },
      { title: "Evidence", body: "What the work proves" },
      { title: "Judgement", body: "How evidence becomes a level" },
      { title: "Progression", body: "What opens next" },
      { title: "Roles", body: "Who teaches, assesses and helps" },
    ],
    layersTitle: "Three layers",
    layers: [
      { title: "Experience", body: "How it looks, for each audience and device" },
      { title: "Framework", body: "What learning means: Mastery, Cambridge-style, a school's own" },
      { title: "Engine", body: "Built once, tested hard, no network or clock inside" },
    ],
    tiles: [
      { title: "Not N × M", body: "Frameworks and experiences mix freely. Adding a pedagogy costs weeks, not a new product." },
      { title: "AI that follows the rules", body: "Agents are actors with roles and permissions. An assessment agent pre-marks; anything unsure goes to the teacher. Every action is logged and can be undone." },
      { title: "Built for the phones children have", body: "Works with no network and syncs later without conflicts. Holds 60 frames a second on an economy-phone profile." },
      { title: "Each class keeps its own data", body: "The database itself keeps every class apart, so no screen, query or agent can cross over." },
    ],
    flow: ["Agent proposes", "Rules check", "Teacher decides"],
  },
  made: {
    title: "Everything here is made with Zidara",
    lead: "The courses, the game, the teacher's studio and the live demo above all run on the same engine. One course even teaches Zidara, with Zidara.",
    shots: [
      { src: "/landing/path.webp", alt: "The fractions path under Mastery: level 1 gold with a palm beside it, level 2 open, level 3 locked.", caption: "The path is the framework's progression, drawn as a game." },
      { src: "/landing/lens.webp", alt: "The same path under Cambridge-style rules: level 1 still open with three stars, levels 2 and 3 open.", caption: "Switch the school's rules and the same answers are judged again." },
      { src: "/landing/right.webp", alt: "A fractions question answered right: Bizra cheers and ten points rise.", caption: "Feedback at once, marked by the engine." },
      { src: "/landing/finish.webp", alt: "The finish screen: You can do this now, a palm grew in your oasis, two stars.", caption: "Rewards come only from evidence of learning." },
    ],
  },
  proof: {
    title: "Proof, not promises",
    lead: "One command checks every claim before anything ships.",
    stats: [
      { value: "100%", label: "of lines, functions and branches covered by tests" },
      { value: "369", label: "deliberate bugs planted in the code, every one caught" },
      { value: "60 fps", label: "on an economy-phone profile, CPU slowed four times" },
      { value: "100", label: "Lighthouse accessibility on every screen, and 90 or more for performance" },
      { value: "18", label: "user stories written in plain words, run as tests" },
    ],
    code: "Read the code",
  },
  faq: {
    title: "Questions",
    items: [
      { q: "Who is Zidara for?", a: "Children in school and the teachers who run their classes. The prototype starts with primary maths, fractions for ages 9 to 11, in Arabic, French and English." },
      { q: "Is it a product or a prototype?", a: "A working prototype. Two courses, the teacher's studio, a live class game, offline sync and the governed AI assessor run today. It has not yet been piloted in a school." },
      { q: "Does AI grade children?", a: "No. The engine never calls a model. An assessment agent may pre-mark an open answer with a confidence score; the school's policy applies it or sends it to the teacher, and anything summative always goes to a person." },
      { q: "Does it need the internet?", a: "Only for the first visit. Learners keep playing offline, and their work syncs when the connection returns, checked again by the same rules on the server." },
      { q: "Can a school bring its own pedagogy?", a: "Yes, that is the point. Outcomes and thresholds are data; judgement and progression are code behind one interface, held to the same tests. Mastery and Cambridge-style run today." },
    ],
  },
  end: { title: "Play the first lesson", lead: "Two minutes, no account, in your language.", classes: "Teachers: open the demo classes" },
  footer: { line: "A learning engine that knows how teaching works.", made: "Made with Zidara", code: "Source code" },
};

const fr: LandingCopy = {
  nav: { why: "Pourquoi", how: "Comment ça marche", proof: "Preuves", play: "Jouer une leçon", menu: "Sections" },
  hero: {
    before: "Le jeu d'apprentissage qui suit ",
    accent: "les règles de votre école",
    after: ".",
    lead: "Zidara transforme la façon d'enseigner d'une école en un jeu que les enfants jouent sur leur téléphone, même hors ligne.",
    more: "Comment ça marche",
  },
  live: {
    label: "Règles de l'école",
    story: "Un élève a répondu une fois au premier quiz de fractions, et juste aux deux questions. Changez les règles :",
    level: "Niveau {n}",
    state: { achieved: "Maîtrisé", in_progress: "En cours", not_started: "Pas commencé", needs_support: "Besoin d'aide" },
    open: "Ouvert",
    after: "S'ouvre après le niveau {n}",
    on: "S'ouvre le {date}",
    verdict: {
      mastery: "Maîtrise : deux bonnes réponses le prouvent. Le niveau 1 est maîtrisé et le niveau 2 s'ouvre.",
      "cambridge-style": "Style Cambridge : les mêmes réponses obtiennent la meilleure mention, mais un entraînement n'est pas une évaluation, et le niveau 3 attend la progression annuelle.",
    },
    engine: "En direct : le vrai moteur de Zidara, compilé en WebAssembly, juge ceci dans votre navigateur.",
    band: "mention {n} sur {of}",
    loading: "Démarrage du moteur…",
    failed: "Le moteur n'a pas pu démarrer dans ce navigateur. La leçon fonctionne quand même.",
  },
  facts: ["Un moteur Rust pour le web, Android et iOS", "Marche hors ligne", "Arabe, français et anglais", "WCAG 2.2 AA", "Code source public"],
  why: {
    title: "Cinq pourquoi, jusqu'à la racine",
    lead: "Nous avons demandé pourquoi tant d'enfants n'apprennent pas, et nous avons continué à demander.",
    steps: [
      { q: "Pourquoi tant d'enfants n'apprennent-ils pas ?", a: "Environ 7 enfants de dix ans sur 10 dans les pays à revenu faible et intermédiaire ne savent pas lire et comprendre un texte simple. En maths, plus de la moitié des élèves de quatrième année au Koweït et au Maroc sont sous le plus bas repère international.", cite: [1, 2] },
      { q: "Pourquoi le numérique ne comble-t-il pas l'écart ?", a: "Il est surtout fait pour d'autres. L'arabe représente 0,6 % des contenus du web, moins de la moitié des habitants du Moyen-Orient et de l'Afrique du Nord utilisent l'internet mobile, et 3 sur 10 d'entre eux ont encore un téléphone 3G ou basique.", cite: [3, 4] },
      { q: "Pourquoi les écoles n'adaptent-elles pas leurs outils ?", a: "Parce que la façon d'enseigner y est figée. Une plateforme porte une seule idée du progrès, et la changer revient à faire un autre produit. Les enseignants comblent le manque à la main, et le monde en manquera de 44 millions d'ici 2030.", cite: [5] },
      { q: "Pourquoi la façon d'enseigner est-elle figée ?", a: "Parce que personne n'a écrit la pédagogie sous une forme qu'un logiciel peut exécuter. Ses règles vivent dans des documents et dans la tête des enseignants, alors chaque appli en refait une copie simplifiée.", cite: [] },
      { q: "Pourquoi est-ce important pour un enfant ?", a: "Un logiciel qui ne sait pas comment on enseigne récompense les clics au lieu de l'apprentissage, et une IA qui ignore les règles devine.", cite: [] },
    ],
    rootTitle: "Alors nous avons construit la pièce qui manquait : la pédagogie en logiciel.",
    root: "Zidara fait des règles d'une école quelque chose que le moteur exécute, teste et peut remplacer. Le jeu, les récompenses et l'IA les suivent tous.",
    sources: "Sources",
  },
  sources: SOURCES,
  how: {
    title: "Un moteur. Toute pédagogie. Chaque public.",
    lead: "Chaque façon d'enseigner que nous avons étudiée se ramène à six briques. Seules les règles changent.",
    blocksTitle: "Six briques",
    blocks: [
      { title: "Objectifs", body: "Ce qu'un élève doit savoir faire" },
      { title: "Activités", body: "Ce que fait l'élève" },
      { title: "Preuves", body: "Ce que le travail démontre" },
      { title: "Jugement", body: "Comment les preuves deviennent un niveau" },
      { title: "Progression", body: "Ce qui s'ouvre ensuite" },
      { title: "Rôles", body: "Qui enseigne, évalue et aide" },
    ],
    layersTitle: "Trois couches",
    layers: [
      { title: "Expérience", body: "L'apparence, pour chaque public et chaque appareil" },
      { title: "Cadre", body: "Ce qu'apprendre veut dire : maîtrise, style Cambridge, celui d'une école" },
      { title: "Moteur", body: "Construit une fois, testé à fond, sans réseau ni horloge" },
    ],
    tiles: [
      { title: "Pas N × M", body: "Cadres et expériences se combinent librement. Ajouter une pédagogie prend des semaines, pas un nouveau produit." },
      { title: "Une IA qui suit les règles", body: "Les agents sont des acteurs avec des rôles et des droits. Un agent d'évaluation pré-corrige ; le moindre doute va à l'enseignant. Chaque action est tracée et réversible." },
      { title: "Fait pour les téléphones des enfants", body: "Marche sans réseau et se synchronise ensuite sans conflit. Tient 60 images par seconde sur un profil de téléphone d'entrée de gamme." },
      { title: "Chaque classe garde ses données", body: "La base de données sépare elle-même chaque classe : aucun écran, requête ou agent ne peut passer de l'une à l'autre." },
    ],
    flow: ["L'agent propose", "Les règles vérifient", "L'enseignant décide"],
  },
  made: {
    title: "Tout ce que vous voyez est fait avec Zidara",
    lead: "Les cours, le jeu, l'atelier de l'enseignant et la démo en direct ci-dessus tournent sur le même moteur. Un cours enseigne même Zidara, avec Zidara.",
    shots: [
      { src: "/landing/path.webp", alt: "Le chemin des fractions en maîtrise : niveau 1 doré avec un palmier, niveau 2 ouvert, niveau 3 verrouillé.", caption: "Le chemin est la progression du cadre, dessinée comme un jeu." },
      { src: "/landing/lens.webp", alt: "Le même chemin selon les règles du style Cambridge : niveau 1 encore ouvert avec trois étoiles, niveaux 2 et 3 ouverts.", caption: "Changez les règles de l'école : les mêmes réponses sont jugées à nouveau." },
      { src: "/landing/right.webp", alt: "Une question de fractions bien répondue : Bizra applaudit et dix points montent.", caption: "Un retour immédiat, corrigé par le moteur." },
      { src: "/landing/finish.webp", alt: "L'écran de fin : tu sais le faire maintenant, un palmier a poussé dans ton oasis, deux étoiles.", caption: "Les récompenses viennent seulement des preuves d'apprentissage." },
    ],
  },
  proof: {
    title: "Des preuves, pas des promesses",
    lead: "Une seule commande vérifie chaque affirmation avant toute mise en ligne.",
    stats: [
      { value: "100 %", label: "des lignes, fonctions et branches couvertes par des tests" },
      { value: "369", label: "bugs volontairement glissés dans le code, tous détectés" },
      { value: "60 i/s", label: "sur un profil de téléphone d'entrée de gamme, processeur ralenti quatre fois" },
      { value: "100", label: "en accessibilité sur Lighthouse pour chaque écran, et 90 ou plus en performance" },
      { value: "18", label: "scénarios écrits en langage clair, exécutés comme tests" },
    ],
    code: "Lire le code",
  },
  faq: {
    title: "Questions",
    items: [
      { q: "À qui s'adresse Zidara ?", a: "Aux enfants scolarisés et aux enseignants qui font leur classe. Le prototype commence par les maths du primaire, les fractions de 9 à 11 ans, en arabe, en français et en anglais." },
      { q: "Produit ou prototype ?", a: "Un prototype qui fonctionne. Deux cours, l'atelier de l'enseignant, un jeu de classe en direct, la synchronisation hors ligne et l'évaluateur IA encadré tournent aujourd'hui. Il n'a pas encore été testé dans une école." },
      { q: "L'IA note-t-elle les enfants ?", a: "Non. Le moteur n'appelle jamais de modèle. Un agent d'évaluation peut pré-corriger une réponse ouverte avec un indice de confiance ; la règle de l'école l'applique ou l'envoie à l'enseignant, et toute évaluation sommative revient toujours à une personne." },
      { q: "Faut-il internet ?", a: "Seulement pour la première visite. Les élèves continuent hors ligne, et leur travail se synchronise au retour de la connexion, revérifié par les mêmes règles sur le serveur." },
      { q: "Une école peut-elle apporter sa pédagogie ?", a: "Oui, c'est tout l'intérêt. Objectifs et seuils sont des données ; jugement et progression sont du code derrière une seule interface, soumis aux mêmes tests. La maîtrise et le style Cambridge tournent aujourd'hui." },
    ],
  },
  end: { title: "Jouez la première leçon", lead: "Deux minutes, sans compte, dans votre langue.", classes: "Enseignants : ouvrir les classes de démo" },
  footer: { line: "Un moteur d'apprentissage qui sait comment on enseigne.", made: "Fait avec Zidara", code: "Code source" },
};

const ar: LandingCopy = {
  nav: { why: "لماذا", how: "كيف تعمل", proof: "الإثبات", play: "العب درسًا", menu: "الأقسام" },
  hero: {
    before: "لعبة التعلّم التي تتّبع ",
    accent: "قواعد مدرستك",
    after: ".",
    lead: "تحوّل زيدارا طريقة المدرسة في التعليم إلى لعبة يلعبها الأطفال على هواتفهم، حتى دون إنترنت.",
    more: "كيف تعمل",
  },
  live: {
    label: "قواعد المدرسة",
    story: "أجاب متعلّم مرة واحدة عن أول اختبار في الكسور، وأصاب في السؤالين. بدّل القواعد:",
    level: "المستوى {n}",
    state: { achieved: "أتقنه", in_progress: "قيد التعلّم", not_started: "لم يبدأ", needs_support: "يحتاج دعمًا" },
    open: "مفتوح",
    after: "يُفتح بعد المستوى {n}",
    on: "يُفتح في {date}",
    verdict: {
      mastery: "الإتقان: إجابتان صحيحتان تكفيان دليلًا. أُتقن المستوى 1 وفُتح المستوى 2.",
      "cambridge-style": "على طريقة كامبريدج: الإجابات نفسها تنال أعلى درجة، لكن التمرين ليس تقييمًا، والمستوى 3 ينتظر موعده في خطة السنة.",
    },
    engine: "مباشر: محرّك زيدارا الحقيقي، مترجمًا إلى WebAssembly، يحكم على هذا داخل متصفّحك.",
    band: "الدرجة {n} من {of}",
    loading: "نشغّل المحرّك…",
    failed: "تعذّر تشغيل المحرّك في هذا المتصفّح. الدرس يعمل مع ذلك.",
  },
  facts: ["محرّك واحد بلغة Rust للويب وأندرويد وiOS", "يعمل دون إنترنت", "العربية والفرنسية والإنجليزية", "WCAG 2.2 AA", "شيفرة مصدرية علنية"],
  why: {
    title: "خمسة أسئلة «لماذا»، حتى الجذر",
    lead: "سألنا لماذا لا يتعلّم كثير من الأطفال، وواصلنا السؤال.",
    steps: [
      { q: "لماذا لا يتعلّم كثير من الأطفال؟", a: "يُقدَّر أن 7 من كل 10 أطفال في العاشرة في البلدان المنخفضة والمتوسطة الدخل لا يستطيعون قراءة نص بسيط وفهمه. وفي الرياضيات، أكثر من نصف تلاميذ السنة الرابعة في الكويت والمغرب دون أدنى معيار دولي.", cite: [1, 2] },
      { q: "لماذا لا يسدّ التعلّم الرقمي الفجوة؟", a: "لأن معظمه صُنع لغيرهم. العربية 0.6% فقط من محتوى الويب، وأقل من نصف سكان الشرق الأوسط وشمال أفريقيا يستعملون الإنترنت عبر الهاتف، و3 من كل 10 منهم ما زالوا على هواتف 3G أو هواتف بسيطة.", cite: [3, 4] },
      { q: "لماذا لا تكيّف المدارس أدواتها؟", a: "لأن طريقة التعليم مبنية داخلها. تحمل المنصّة فكرة واحدة عن التقدّم، وتغييرها يعني منتجًا جديدًا. يسدّ المعلّمون النقص بأيديهم، والعالم يحتاج 44 مليون معلّم إضافي بحلول 2030.", cite: [5] },
      { q: "لماذا طريقة التعليم مبنية داخل الأدوات؟", a: "لأن أحدًا لم يكتب البيداغوجيا بصيغة يستطيع البرنامج تشغيلها. قواعدها في الوثائق وفي أذهان المعلّمين، فيعيد كل تطبيق بناء نسخة مبسّطة منها.", cite: [] },
      { q: "لماذا يهمّ هذا الطفل؟", a: "البرنامج الذي لا يعرف كيف يكون التعليم يكافئ النقرات بدل التعلّم، والذكاء الاصطناعي الذي لا يعرف القواعد يخمّن.", cite: [] },
    ],
    rootTitle: "فبنينا القطعة الناقصة: البيداغوجيا برنامجًا.",
    root: "تجعل زيدارا قواعد المدرسة شيئًا يشغّله المحرّك ويختبره ويستبدله. واللعبة والمكافآت والذكاء الاصطناعي كلّها تتّبعها.",
    sources: "المصادر",
  },
  sources: SOURCES,
  how: {
    title: "محرّك واحد. أي بيداغوجيا. لكل جمهور.",
    lead: "كل طريقة تعليم درسناها ترجع إلى ست لبنات. القواعد وحدها تتغيّر.",
    blocksTitle: "ست لبنات",
    blocks: [
      { title: "الأهداف", body: "ما ينبغي أن يقدر عليه المتعلّم" },
      { title: "الأنشطة", body: "ما يفعله المتعلّم" },
      { title: "الأدلّة", body: "ما يثبته العمل" },
      { title: "الحكم", body: "كيف تصير الأدلّة مستوى" },
      { title: "التدرّج", body: "ما الذي يُفتح بعد ذلك" },
      { title: "الأدوار", body: "من يعلّم ويقيّم ويساعد" },
    ],
    layersTitle: "ثلاث طبقات",
    layers: [
      { title: "التجربة", body: "الشكل، لكل جمهور ولكل جهاز" },
      { title: "الإطار", body: "معنى التعلّم: الإتقان، طريقة كامبريدج، طريقة المدرسة نفسها" },
      { title: "المحرّك", body: "يُبنى مرة ويُختبر بصرامة، بلا شبكة ولا ساعة بداخله" },
    ],
    tiles: [
      { title: "لا N × M", body: "الأطر والتجارب تتركّب بحرّية. إضافة بيداغوجيا تكلّف أسابيع، لا منتجًا جديدًا." },
      { title: "ذكاء اصطناعي يتّبع القواعد", body: "الوكلاء فاعلون لهم أدوار وصلاحيات. وكيل التقييم يصحّح مبدئيًا، وكل ما فيه شكّ يذهب إلى المعلّم. كل فعل مسجَّل ويمكن التراجع عنه." },
      { title: "مصنوع لهواتف الأطفال", body: "يعمل دون شبكة ويتزامن لاحقًا بلا تعارض. يحافظ على 60 صورة في الثانية على ملف هاتف اقتصادي." },
      { title: "لكل قسم بياناته", body: "قاعدة البيانات نفسها تفصل الأقسام، فلا شاشة ولا استعلام ولا وكيل يعبر من قسم إلى آخر." },
    ],
    flow: ["الوكيل يقترح", "القواعد تفحص", "المعلّم يقرّر"],
  },
  made: {
    title: "كل ما تراه هنا مصنوع بزيدارا",
    lead: "الدروس واللعبة واستوديو المعلّم والعرض المباشر أعلاه كلّها تعمل على المحرّك نفسه. وأحد الدروس يعلّم زيدارا، بزيدارا.",
    shots: [
      { src: "/landing/path.webp", alt: "طريق الكسور بقواعد الإتقان: المستوى 1 ذهبي وبجانبه نخلة، المستوى 2 مفتوح، المستوى 3 مقفل.", caption: "الطريق هو تدرّج الإطار، مرسومًا لعبة." },
      { src: "/landing/lens.webp", alt: "الطريق نفسه بقواعد طريقة كامبريدج: المستوى 1 ما زال مفتوحًا بثلاث نجوم، والمستويان 2 و3 مفتوحان.", caption: "بدّل قواعد المدرسة فتُقيَّم الإجابات نفسها من جديد." },
      { src: "/landing/right.webp", alt: "سؤال في الكسور أُجيب عنه صحيحًا: بذرة تهتف وتصعد عشر نقاط.", caption: "ملاحظة فورية، يصحّحها المحرّك." },
      { src: "/landing/finish.webp", alt: "شاشة النهاية: أصبحت تستطيع هذا، نبتت نخلة في واحتك، نجمتان.", caption: "المكافآت تأتي من أدلّة التعلّم وحدها." },
    ],
  },
  proof: {
    title: "إثبات، لا وعود",
    lead: "أمر واحد يفحص كل ادّعاء قبل أن يُنشر أي شيء.",
    stats: [
      { value: "100%", label: "من الأسطر والدوال والفروع تغطّيها الاختبارات" },
      { value: "369", label: "خطأً زُرع عمدًا في الشيفرة، وكُشف كلّها" },
      { value: "60", label: "صورة في الثانية على ملف هاتف اقتصادي، والمعالج أبطأ أربع مرات" },
      { value: "100", label: "في إتاحة Lighthouse لكل شاشة، و90 أو أكثر في الأداء" },
      { value: "18", label: "قصة استخدام بكلام واضح، تُشغَّل اختبارات" },
    ],
    code: "اقرأ الشيفرة",
  },
  faq: {
    title: "أسئلة",
    items: [
      { q: "لمن زيدارا؟", a: "للأطفال في المدرسة وللمعلّمين الذين يديرون أقسامهم. يبدأ النموذج برياضيات الابتدائي، الكسور من 9 إلى 11 سنة، بالعربية والفرنسية والإنجليزية." },
      { q: "منتج أم نموذج أولي؟", a: "نموذج أولي يعمل. درسان واستوديو المعلّم ولعبة القسم المباشرة والمزامنة دون إنترنت والمقيّم الآلي المحكوم بالقواعد تعمل اليوم. لم يُجرَّب بعد في مدرسة." },
      { q: "هل يمنح الذكاء الاصطناعي العلامات للأطفال؟", a: "لا. المحرّك لا يستدعي نموذجًا أبدًا. قد يصحّح وكيل التقييم إجابة مفتوحة مبدئيًا مع درجة ثقة، فتطبّقها قاعدة المدرسة أو ترسلها إلى المعلّم، وكل تقييم ختامي يعود دائمًا إلى إنسان." },
      { q: "هل يحتاج إلى الإنترنت؟", a: "في الزيارة الأولى فقط. يواصل المتعلّمون اللعب دون اتصال، ويتزامن عملهم حين يعود الاتصال، ويُفحص من جديد بالقواعد نفسها على الخادم." },
      { q: "هل تستطيع مدرسة أن تأتي ببيداغوجيتها؟", a: "نعم، وهذا هو المقصود. الأهداف والعتبات بيانات، والحكم والتدرّج شيفرة خلف واجهة واحدة تخضع للاختبارات نفسها. الإتقان وطريقة كامبريدج يعملان اليوم." },
    ],
  },
  end: { title: "العب الدرس الأول", lead: "دقيقتان، بلا حساب، بلغتك.", classes: "للمعلّمين: الأقسام التجريبية" },
  footer: { line: "محرّك تعلّم يعرف كيف يكون التعليم.", made: "صُنع بزيدارا", code: "الشيفرة المصدرية" },
};

export const LANDING: Record<Locale, LandingCopy> = { en, fr, ar };
