// The front page's words (ADR 0014, ADR 0015), in the app's three languages. Zidara is pitched as
// what docs/proposal.md says it is: an adaptable learning engine (one kernel, any framework, any
// experience, AI agents inside approved rules). The demo course is one experience among many.
// Every figure comes from a row marked "verified" in docs/impact-facts.md and names its source.
// House style for this page: no em or en dashes; plain sentences; one label per intent.
import type { Locale } from "./i18n.ts";

export type Source = { label: string; href: string };
export type Why = { q: string; a: string; cite: number[] };
export type Tile = { title: string; body: string };
export type Qa = { q: string; a: string };
export type Agent = { name: string; body: string; today: boolean };
export type Made = { title: string; body: string; href: string; shot: string; alt: string };

export type LandingCopy = {
  nav: { why: string; how: string; engine: string; ai: string; made: string; play: string; menu: string };
  hero: { badge: string; before: string; accent: string; after: string; lead: string; primary: string; secondary: string };
  engine: { title: string; lead: string };
  live: {
    label: string;
    tag: string;
    frameworkAxis: string;
    experienceAxis: string;
    experiences: Record<"path" | "grid" | "studio" | "text", string>;
    story: string;
    level: string;
    state: Record<"achieved" | "in_progress" | "not_started" | "needs_support", string>;
    open: string;
    after: string;
    on: string;
    band: string;
    cols: { outcome: string; status: string; score: string; band: string };
    studioHead: string;
    attempts: string;
    smsNote: string;
    verdict: Record<"mastery" | "cambridge-style", string>;
    engine: string;
    loading: string;
    failed: string;
  };
  facts: string[];
  why: { title: string; lead: string; steps: Why[]; rootTitle: string; root: string; sources: string };
  sources: Source[];
  how: { title: string; lead: string; layersTitle: string; layers: Tile[]; cards: Tile[] };
  ai: { title: string; lead: string; flow: string[]; agents: Agent[]; today: string; designed: string };
  made: { title: string; lead: string; items: Made[]; open: string };
  proof: { title: string; lead: string; stats: { value: string; label: string }[]; code: string };
  faq: { title: string; items: Qa[] };
  end: { title: string; lead: string; primary: string; classes: string };
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
  nav: { why: "Vision", how: "Architecture", engine: "Live engine", ai: "AI agents", made: "Made with Zidara", play: "Try the demo", menu: "Sections" },
  hero: {
    badge: "Every demo on this page is made with Zidara",
    before: "The adaptable learning engine for ",
    accent: "the AI era",
    after: ".",
    lead: "Zidara is a learning platform built on one tested kernel. Bring any teaching framework, present it as any experience, and let AI agents work only inside the rules your educators approve.",
    primary: "See the engine live",
    secondary: "Try a demo course",
  },
  engine: {
    title: "One judgement, any framework, any experience",
    lead: "This is the real kernel, running in your browser. Change the framework and the judgement changes. Change the experience and only the drawing does.",
  },
  live: {
    label: "zidara / live engine",
    tag: "Running in your browser",
    frameworkAxis: "Framework",
    experienceAxis: "Experience",
    experiences: { path: "Children's path", grid: "Secondary grid", studio: "Teacher's studio", text: "Text message" },
    story: "One learner answered the first fractions quiz once and got both questions right.",
    level: "Level {n}",
    state: { achieved: "Mastered", in_progress: "In progress", not_started: "Not started", needs_support: "Needs support" },
    open: "Open",
    after: "Opens after level {n}",
    on: "Opens on {date}",
    band: "Band {n} of {of}",
    cols: { outcome: "Outcome", status: "Status", score: "Score", band: "Band" },
    studioHead: "Equivalent fractions, one learner",
    attempts: "attempts: {n}",
    smsNote: "For basic phones: the same view, sent as text.",
    verdict: {
      mastery: "Mastery: two right answers prove it. Level 1 is mastered and level 2 opens.",
      "cambridge-style": "Cambridge-style: the same answers earn a top band, but practice is not an assessment, and level 3 waits for the scheme of work.",
    },
    engine: "The real Zidara kernel, compiled to WebAssembly, decides this in your browser. Nothing is sent.",
    loading: "Starting the engine…",
    failed: "The engine could not start in this browser. The demo course still works.",
  },
  facts: ["One Rust kernel: server, browser, Android, iOS", "Any teaching framework", "Any experience", "AI agents with permissions", "Offline first", "Arabic, French, English"],
  why: {
    title: "Five whys, down to the root",
    lead: "We asked why so many children are still not learning, and kept asking until we reached something software could fix.",
    steps: [
      { q: "Why are so many children not learning?", a: "An estimated 7 in 10 ten-year-olds in low- and middle-income countries cannot read and understand a simple text. In maths, over half of fourth graders in Kuwait and Morocco fall below the lowest international benchmark.", cite: [1, 2] },
      { q: "Why hasn't digital learning closed the gap?", a: "Most of it is made for someone else. Arabic is 0.6% of the web's content, under half of people in the Middle East and North Africa use mobile internet, and 3 in 10 of those who do still use 3G or basic phones.", cite: [3, 4] },
      { q: "Why can't schools shape the platforms they have?", a: "Learning platforms carry one idea of a course and bolt the rest on as plugins. A teaching approach lives inside the application and cannot be lifted out, so a new pedagogy or a new audience means a new product, or work done by hand by teachers the world is already short of: 44 million more are needed by 2030.", cite: [5] },
      { q: "Why is the teaching locked inside?", a: "Because pedagogy was never written down as software. Its rules sit in documents and in teachers' heads, and every platform rebuilds a simplified copy for every audience: N frameworks times M experiences.", cite: [] },
      { q: "Why does it matter now?", a: "AI tutors and markers are arriving in every classroom. Without the school's rules inside the system, they guess, and nobody can check, govern or undo what they did.", cite: [] },
    ],
    rootTitle: "So we built the missing engine: pedagogy as software.",
    root: "Zidara turns any way of teaching into rules a tested kernel runs. Any experience and any AI agent then works on top of it, inside rules people approve.",
    sources: "Sources",
  },
  sources: SOURCES,
  how: {
    title: "One kernel. Any framework. Any experience.",
    lead: "Every teaching approach we compared, from Cambridge and mastery learning to IB criteria, competency-based education and spaced repetition, reduces to the same six building blocks. The blocks stay; only the rules change.",
    layersTitle: "Three layers, and automation through all of them",
    layers: [
      { title: "Experience", body: "How it looks and feels for each audience and device: a child's path, a secondary grid, a teacher's studio, a parent's summary, a tutor in chat." },
      { title: "Framework", body: "What learning means: outcomes, evidence and the rules that judge them. Mastery, Cambridge-style, or a school's own." },
      { title: "Kernel", body: "People, groups, content, activities, the evidence log, scheduling and security. Built once, tested hard, with no network or clock inside." },
      { title: "Automation", body: "Events, rules and AI agents as actors, running through all three layers." },
    ],
    cards: [
      { title: "Six building blocks", body: "Outcomes, activities, evidence, judgement, progression and roles. Every pedagogy is a set of rules over the same six." },
      { title: "Any framework", body: "Outcome trees and thresholds are data. Judgement and progression are compiled code behind one interface, held to the same tests. A new framework costs weeks, not a new product." },
      { title: "Any experience, not N × M", body: "A framework says what is true; an experience decides how to show it. N frameworks and M experiences cost N + M to build." },
      { title: "AI agents are actors", body: "They pass through the same permissions as people. Anything unsure or summative stops at a teacher, and every action can be undone." },
      { title: "Offline first, on every device", body: "The same kernel runs on the server, in the browser as WebAssembly and inside native apps. Work done offline replays through the same rules when it syncs." },
      { title: "Isolation in the database", body: "Every row carries its class and school, and PostgreSQL row-level security keeps them apart. No screen, query or agent can cross." },
    ],
  },
  ai: {
    title: "Built for the AI era, governed by people",
    lead: "The kernel never calls a model. Agents propose; the framework's policy decides what applies and what waits for a teacher. Every agent action is logged and can be undone.",
    flow: ["Agent proposes", "Rules check", "Teacher decides"],
    agents: [
      { name: "Assessment agent", body: "Pre-marks open answers with a confidence score. Only confident formative marks apply; summative work always goes to a teacher.", today: true },
      { name: "Tutor agent", body: "Reads a learner's evidence and suggests what to do next. It never grades summative work.", today: false },
      { name: "Operations agent", body: "Spots learners at risk and opens a case for a person to follow up.", today: false },
      { name: "Authoring agent", body: "Turns a curriculum document into a draft framework for a designer to approve.", today: false },
    ],
    today: "Running today",
    designed: "Designed",
  },
  made: {
    title: "Made with Zidara",
    lead: "Everything you can open from this page runs on the same kernel. The demo course is one experience among many, built to show how far the engine bends. One course even teaches Zidara, with Zidara.",
    items: [
      { title: "A learning game", body: "A course in the style of Duolingo and Brilliant for ages 9 to 11: films, toys and questions, marked by the kernel at once. Rewards come only from evidence.", href: "#play", shot: "/landing/path.webp", alt: "The fractions path after one session: level 1 mastered in blue with a palm of dots beside it, level 2 next, Bizra waiting." },
      { title: "Live rounds", body: "A round on the class board in the style of Kahoot, every phone answering. No timer and no ranking; the answers become practice evidence.", href: "#classes", shot: "/landing/board.webp", alt: "The teacher's studio during a live round: the question, three answers in colour, and buttons to show the answer or move on." },
      { title: "The teacher's studio", body: "The class's progress, the agent's queued marks and every undo, in one place.", href: "#classes", shot: "/landing/studio.webp", alt: "The teacher's studio: where the class is on each outcome, answers waiting for a decision, and the class's teaching approach." },
      { title: "A pedagogy switch", body: "A teacher moves the class from mastery to Cambridge-style, and every learner is judged again from the same evidence.", href: "#classes", shot: "/landing/lens.webp", alt: "The same path under Cambridge-style rules: level 1 is back to open, and a note says the engine, not the game, judges by the school's rules." },
    ],
    open: "Open",
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
      { q: "Is Zidara a game?", a: "No. Zidara is a learning engine and the platform around it. The game on this site is one experience built on it, to show how far the engine bends; the same kernel draws a secondary grid, a teacher's studio or a text message." },
      { q: "Who is it for?", a: "Schools, training centres and education systems that want their own way of teaching to run as software, and teams that want to build new learning experiences on a tested core. The prototype starts with primary maths in Arabic, French and English." },
      { q: "How is it different from other learning platforms?", a: "General platforms keep their teaching approach inside the application and add the rest as plugins. In Zidara, pedagogy is the core: frameworks are first class, experiences are swappable, and agents are governed actors." },
      { q: "Does AI grade learners?", a: "No. The engine never calls a model. An assessment agent may pre-mark an open answer with a confidence score; the framework's policy applies it or sends it to the teacher, and anything summative always goes to a person." },
      { q: "Can we bring our own pedagogy?", a: "Yes, that is the point. Outcomes and thresholds are data; judgement and progression are code behind one interface, held to the same tests. Mastery and Cambridge-style run today." },
      { q: "Is it a product or a prototype?", a: "A working prototype. Two frameworks, two courses, the teacher's studio, live rounds, offline sync and the governed AI assessor run today. It has not yet been piloted in a school." },
    ],
  },
  end: { title: "See what the engine can do", lead: "Open a demo course as a guest, or the teacher's studio. No account needed.", primary: "Try a demo course", classes: "Open the teacher's studio" },
  footer: { line: "The adaptable learning engine for the AI era.", made: "Made with Zidara", code: "Source code" },
};

const fr: LandingCopy = {
  nav: { why: "Vision", how: "Architecture", engine: "Moteur en direct", ai: "Agents IA", made: "Fait avec Zidara", play: "Essayer la démo", menu: "Sections" },
  hero: {
    badge: "Chaque démo de cette page est faite avec Zidara",
    before: "Le moteur d'apprentissage adaptable pour ",
    accent: "l'ère de l'IA",
    after: ".",
    lead: "Zidara est une plateforme d'apprentissage bâtie sur un seul noyau testé. Apportez n'importe quel cadre pédagogique, présentez-le sous n'importe quelle expérience, et laissez les agents IA agir seulement dans les règles que vos enseignants approuvent.",
    primary: "Voir le moteur en direct",
    secondary: "Essayer un cours de démo",
  },
  engine: {
    title: "Un jugement, n'importe quel cadre, n'importe quelle expérience",
    lead: "Voici le vrai noyau, qui tourne dans votre navigateur. Changez le cadre et le jugement change. Changez l'expérience et seul le dessin change.",
  },
  live: {
    label: "zidara / moteur en direct",
    tag: "Dans votre navigateur",
    frameworkAxis: "Cadre",
    experienceAxis: "Expérience",
    experiences: { path: "Chemin des enfants", grid: "Grille du secondaire", studio: "Atelier de l'enseignant", text: "Message texte" },
    story: "Un élève a répondu une fois au premier quiz de fractions, et juste aux deux questions.",
    level: "Niveau {n}",
    state: { achieved: "Maîtrisé", in_progress: "En cours", not_started: "Pas commencé", needs_support: "Besoin d'aide" },
    open: "Ouvert",
    after: "S'ouvre après le niveau {n}",
    on: "S'ouvre le {date}",
    band: "Mention {n} sur {of}",
    cols: { outcome: "Objectif", status: "État", score: "Score", band: "Mention" },
    studioHead: "Fractions équivalentes, un élève",
    attempts: "essais : {n}",
    smsNote: "Pour les téléphones basiques : la même vue, envoyée en texte.",
    verdict: {
      mastery: "Maîtrise : deux bonnes réponses le prouvent. Le niveau 1 est maîtrisé et le niveau 2 s'ouvre.",
      "cambridge-style": "Style Cambridge : les mêmes réponses obtiennent la meilleure mention, mais un entraînement n'est pas une évaluation, et le niveau 3 attend la progression annuelle.",
    },
    engine: "Le vrai noyau de Zidara, compilé en WebAssembly, décide ceci dans votre navigateur. Rien n'est envoyé.",
    loading: "Démarrage du moteur…",
    failed: "Le moteur n'a pas pu démarrer dans ce navigateur. Le cours de démo fonctionne quand même.",
  },
  facts: ["Un noyau Rust : serveur, navigateur, Android, iOS", "N'importe quel cadre pédagogique", "N'importe quelle expérience", "Des agents IA avec permissions", "Hors ligne d'abord", "Arabe, français, anglais"],
  why: {
    title: "Cinq pourquoi, jusqu'à la racine",
    lead: "Nous avons demandé pourquoi tant d'enfants n'apprennent toujours pas, et nous avons continué jusqu'à trouver ce qu'un logiciel peut réparer.",
    steps: [
      { q: "Pourquoi tant d'enfants n'apprennent-ils pas ?", a: "Environ 7 enfants de dix ans sur 10 dans les pays à revenu faible et intermédiaire ne savent pas lire et comprendre un texte simple. En maths, plus de la moitié des élèves de quatrième année au Koweït et au Maroc sont sous le plus bas repère international.", cite: [1, 2] },
      { q: "Pourquoi le numérique ne comble-t-il pas l'écart ?", a: "Il est surtout fait pour d'autres. L'arabe représente 0,6 % des contenus du web, moins de la moitié des habitants du Moyen-Orient et de l'Afrique du Nord utilisent l'internet mobile, et 3 sur 10 d'entre eux ont encore un téléphone 3G ou basique.", cite: [3, 4] },
      { q: "Pourquoi les écoles ne peuvent-elles pas façonner leurs plateformes ?", a: "Les plateformes d'apprentissage portent une seule idée du cours et ajoutent le reste en modules. L'approche pédagogique vit dans l'application et ne peut pas en sortir : une nouvelle pédagogie ou un nouveau public demande un nouveau produit, ou un travail fait à la main par des enseignants qui manquent déjà. Il en faudra 44 millions de plus d'ici 2030.", cite: [5] },
      { q: "Pourquoi l'enseignement est-il enfermé ?", a: "Parce que la pédagogie n'a jamais été écrite comme un logiciel. Ses règles vivent dans des documents et dans la tête des enseignants, et chaque plateforme en refait une copie simplifiée pour chaque public : N cadres fois M expériences.", cite: [] },
      { q: "Pourquoi est-ce urgent maintenant ?", a: "Les tuteurs et correcteurs IA arrivent dans chaque classe. Sans les règles de l'école dans le système, ils devinent, et personne ne peut vérifier, encadrer ou annuler ce qu'ils ont fait.", cite: [] },
    ],
    rootTitle: "Alors nous avons construit le moteur qui manquait : la pédagogie comme logiciel.",
    root: "Zidara transforme n'importe quelle façon d'enseigner en règles qu'un noyau testé exécute. Toute expérience et tout agent IA travaillent ensuite par-dessus, dans des règles que des personnes approuvent.",
    sources: "Sources",
  },
  sources: SOURCES,
  how: {
    title: "Un noyau. N'importe quel cadre. N'importe quelle expérience.",
    lead: "Chaque approche que nous avons comparée, de Cambridge et la pédagogie de la maîtrise aux critères de l'IB, à l'approche par compétences et à la répétition espacée, se ramène aux mêmes six briques. Les briques restent ; seules les règles changent.",
    layersTitle: "Trois couches, et l'automatisation à travers toutes",
    layers: [
      { title: "Expérience", body: "L'apparence pour chaque public et chaque appareil : le chemin d'un enfant, une grille du secondaire, l'atelier de l'enseignant, le résumé des parents, un tuteur en conversation." },
      { title: "Cadre", body: "Ce qu'apprendre veut dire : objectifs, preuves et règles qui les jugent. Maîtrise, style Cambridge, ou celui d'une école." },
      { title: "Noyau", body: "Personnes, groupes, contenus, activités, journal des preuves, calendrier et sécurité. Construit une fois, testé à fond, sans réseau ni horloge à l'intérieur." },
      { title: "Automatisation", body: "Événements, règles et agents IA en acteurs, à travers les trois couches." },
    ],
    cards: [
      { title: "Six briques", body: "Objectifs, activités, preuves, jugement, progression et rôles. Chaque pédagogie est un ensemble de règles sur les mêmes six." },
      { title: "N'importe quel cadre", body: "Arbres d'objectifs et seuils sont des données. Jugement et progression sont du code compilé derrière une seule interface, soumis aux mêmes tests. Un nouveau cadre coûte des semaines, pas un nouveau produit." },
      { title: "N'importe quelle expérience, pas N × M", body: "Un cadre dit ce qui est vrai ; une expérience décide comment le montrer. N cadres et M expériences coûtent N + M à construire." },
      { title: "Les agents IA sont des acteurs", body: "Ils passent par les mêmes permissions que les personnes. Tout ce qui est incertain ou sommatif s'arrête chez l'enseignant, et chaque action peut être annulée." },
      { title: "Hors ligne d'abord, sur tout appareil", body: "Le même noyau tourne sur le serveur, dans le navigateur en WebAssembly et dans les applis natives. Le travail fait hors ligne repasse par les mêmes règles à la synchronisation." },
      { title: "L'isolement dans la base de données", body: "Chaque ligne porte sa classe et son école, et la sécurité au niveau des lignes de PostgreSQL les sépare. Aucun écran, requête ou agent ne peut passer." },
    ],
  },
  ai: {
    title: "Conçu pour l'ère de l'IA, gouverné par des personnes",
    lead: "Le noyau n'appelle jamais de modèle. Les agents proposent ; la règle du cadre décide ce qui s'applique et ce qui attend l'enseignant. Chaque action d'agent est journalisée et peut être annulée.",
    flow: ["L'agent propose", "Les règles vérifient", "L'enseignant décide"],
    agents: [
      { name: "Agent d'évaluation", body: "Pré-corrige les réponses ouvertes avec un indice de confiance. Seules les notes formatives sûres s'appliquent ; le sommatif revient toujours à l'enseignant.", today: true },
      { name: "Agent tuteur", body: "Lit les preuves d'un élève et propose la suite. Il ne note jamais un travail sommatif.", today: false },
      { name: "Agent des opérations", body: "Repère les élèves en difficulté et ouvre un dossier pour qu'une personne s'en occupe.", today: false },
      { name: "Agent de conception", body: "Transforme un programme officiel en cadre provisoire qu'un concepteur approuve.", today: false },
    ],
    today: "Tourne aujourd'hui",
    designed: "Conçu",
  },
  made: {
    title: "Fait avec Zidara",
    lead: "Tout ce que vous pouvez ouvrir depuis cette page tourne sur le même noyau. Le cours de démo est une expérience parmi d'autres, faite pour montrer jusqu'où le moteur se plie. Un cours enseigne même Zidara, avec Zidara.",
    items: [
      { title: "Un jeu d'apprentissage", body: "Un cours dans le style de Duolingo et Brilliant pour les 9 à 11 ans : films, jouets et questions, corrigés aussitôt par le noyau. Les récompenses ne viennent que des preuves.", href: "#play", shot: "/landing/path.webp", alt: "Le chemin des fractions après une séance : le niveau 1 maîtrisé en bleu avec un palmier de points à côté, le niveau 2 ensuite, Bizra qui attend." },
      { title: "Des manches en direct", body: "Une manche sur le tableau de la classe dans le style de Kahoot, chaque téléphone répond. Pas de chrono ni de classement ; les réponses deviennent des preuves d'entraînement.", href: "#classes", shot: "/landing/board.webp", alt: "L'atelier de l'enseignant pendant une manche en direct : la question, trois réponses en couleur, et des boutons pour montrer la réponse ou continuer." },
      { title: "L'atelier de l'enseignant", body: "Les progrès de la classe, les notes proposées par l'agent et chaque annulation, au même endroit.", href: "#classes", shot: "/landing/studio.webp", alt: "L'atelier de l'enseignant : où en est la classe pour chaque objectif, les réponses en attente de décision, et l'approche pédagogique de la classe." },
      { title: "Changer de pédagogie", body: "Un enseignant passe la classe de la maîtrise au style Cambridge, et chaque élève est jugé à nouveau à partir des mêmes preuves.", href: "#classes", shot: "/landing/lens.webp", alt: "Le même chemin selon le style Cambridge : le niveau 1 redevient ouvert, et une note dit que c'est le moteur, pas le jeu, qui juge selon les règles de l'école." },
    ],
    open: "Ouvrir",
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
      { q: "Zidara est-il un jeu ?", a: "Non. Zidara est un moteur d'apprentissage et la plateforme qui l'entoure. Le jeu de ce site est une expérience construite dessus, pour montrer jusqu'où le moteur se plie ; le même noyau dessine une grille du secondaire, l'atelier de l'enseignant ou un message texte." },
      { q: "À qui s'adresse-t-il ?", a: "Aux écoles, centres de formation et systèmes éducatifs qui veulent que leur façon d'enseigner tourne comme un logiciel, et aux équipes qui veulent construire de nouvelles expériences sur un cœur testé. Le prototype commence par les maths du primaire en arabe, en français et en anglais." },
      { q: "En quoi est-il différent des autres plateformes ?", a: "Les plateformes générales gardent leur approche pédagogique dans l'application et ajoutent le reste en modules. Dans Zidara, la pédagogie est le cœur : les cadres sont de premier rang, les expériences sont interchangeables et les agents sont des acteurs encadrés." },
      { q: "L'IA note-t-elle les élèves ?", a: "Non. Le moteur n'appelle jamais de modèle. Un agent d'évaluation peut pré-corriger une réponse ouverte avec un indice de confiance ; la règle du cadre l'applique ou l'envoie à l'enseignant, et toute évaluation sommative revient toujours à une personne." },
      { q: "Pouvons-nous apporter notre pédagogie ?", a: "Oui, c'est tout l'intérêt. Objectifs et seuils sont des données ; jugement et progression sont du code derrière une seule interface, soumis aux mêmes tests. La maîtrise et le style Cambridge tournent aujourd'hui." },
      { q: "Produit ou prototype ?", a: "Un prototype qui fonctionne. Deux cadres, deux cours, l'atelier de l'enseignant, les manches en direct, la synchronisation hors ligne et l'évaluateur IA encadré tournent aujourd'hui. Il n'a pas encore été testé dans une école." },
    ],
  },
  end: { title: "Voyez ce que le moteur sait faire", lead: "Ouvrez un cours de démo en invité, ou l'atelier de l'enseignant. Aucun compte nécessaire.", primary: "Essayer un cours de démo", classes: "Ouvrir l'atelier de l'enseignant" },
  footer: { line: "Le moteur d'apprentissage adaptable pour l'ère de l'IA.", made: "Fait avec Zidara", code: "Code source" },
};

const ar: LandingCopy = {
  nav: { why: "الرؤية", how: "البنية", engine: "المحرّك مباشرة", ai: "وكلاء الذكاء الاصطناعي", made: "صُنع بزيدارا", play: "جرّب العرض", menu: "الأقسام" },
  hero: {
    badge: "كل عرض في هذه الصفحة مصنوع بزيدارا",
    before: "محرّك التعلّم القابل للتكيّف ",
    accent: "لعصر الذكاء الاصطناعي",
    after: ".",
    lead: "زيدارا منصّة تعلّم مبنية على نواة واحدة مختبَرة. أحضر أي إطار تعليمي، واعرضه بأي تجربة، ودع وكلاء الذكاء الاصطناعي يعملون داخل القواعد التي يوافق عليها معلّموك فقط.",
    primary: "شاهد المحرّك مباشرة",
    secondary: "جرّب درسًا تجريبيًا",
  },
  engine: {
    title: "حكم واحد، بأي إطار، وبأي تجربة",
    lead: "هذه هي النواة الحقيقية، تعمل في متصفّحك. غيّر الإطار فيتغيّر الحكم. غيّر التجربة فلا يتغيّر إلا الرسم.",
  },
  live: {
    label: "zidara / المحرّك مباشرة",
    tag: "يعمل في متصفّحك",
    frameworkAxis: "الإطار",
    experienceAxis: "التجربة",
    experiences: { path: "طريق الأطفال", grid: "شبكة الثانوي", studio: "استوديو المعلّم", text: "رسالة نصية" },
    story: "أجاب متعلّم مرة واحدة عن أول اختبار في الكسور، وأصاب في السؤالين.",
    level: "المستوى {n}",
    state: { achieved: "أتقنه", in_progress: "قيد التعلّم", not_started: "لم يبدأ", needs_support: "يحتاج دعمًا" },
    open: "مفتوح",
    after: "يُفتح بعد المستوى {n}",
    on: "يُفتح في {date}",
    band: "الدرجة {n} من {of}",
    cols: { outcome: "الهدف", status: "الحالة", score: "النتيجة", band: "الدرجة" },
    studioHead: "الكسور المتكافئة، متعلّم واحد",
    attempts: "المحاولات: {n}",
    smsNote: "للهواتف البسيطة: العرض نفسه، مرسلًا نصًّا.",
    verdict: {
      mastery: "الإتقان: إجابتان صحيحتان تكفيان دليلًا. أُتقن المستوى 1 وفُتح المستوى 2.",
      "cambridge-style": "على طريقة كامبريدج: الإجابات نفسها تنال أعلى درجة، لكن التمرين ليس تقييمًا، والمستوى 3 ينتظر موعده في خطة السنة.",
    },
    engine: "نواة زيدارا الحقيقية، مترجمة إلى WebAssembly، تقرّر هذا داخل متصفّحك. لا يُرسل شيء.",
    loading: "نشغّل المحرّك…",
    failed: "تعذّر تشغيل المحرّك في هذا المتصفّح. الدرس التجريبي يعمل مع ذلك.",
  },
  facts: ["نواة Rust واحدة: الخادم والمتصفّح وأندرويد وiOS", "أي إطار تعليمي", "أي تجربة", "وكلاء ذكاء اصطناعي بصلاحيات", "يعمل دون إنترنت أولًا", "العربية والفرنسية والإنجليزية"],
  why: {
    title: "خمسة أسئلة «لماذا»، حتى الجذر",
    lead: "سألنا لماذا لا يتعلّم كثير من الأطفال حتى اليوم، وواصلنا السؤال حتى وصلنا إلى ما يستطيع البرنامج إصلاحه.",
    steps: [
      { q: "لماذا لا يتعلّم كثير من الأطفال؟", a: "يُقدَّر أن 7 من كل 10 أطفال في العاشرة في البلدان المنخفضة والمتوسطة الدخل لا يستطيعون قراءة نص بسيط وفهمه. وفي الرياضيات، أكثر من نصف تلاميذ السنة الرابعة في الكويت والمغرب دون أدنى معيار دولي.", cite: [1, 2] },
      { q: "لماذا لا يسدّ التعلّم الرقمي الفجوة؟", a: "لأن معظمه صُنع لغيرهم. العربية 0.6% فقط من محتوى الويب، وأقل من نصف سكان الشرق الأوسط وشمال أفريقيا يستعملون الإنترنت عبر الهاتف، و3 من كل 10 منهم ما زالوا على هواتف 3G أو هواتف بسيطة.", cite: [3, 4] },
      { q: "لماذا لا تستطيع المدارس تشكيل منصّاتها؟", a: "تحمل منصّات التعلّم فكرة واحدة عن الدرس وتضيف الباقي إضافاتٍ. طريقة التعليم تعيش داخل التطبيق ولا يمكن إخراجها، فتعني بيداغوجيا جديدة أو جمهور جديد منتجًا جديدًا، أو عملًا يدويًا يقوم به معلّمون ينقصون العالم أصلًا: يحتاج العالم 44 مليون معلّم إضافي بحلول 2030.", cite: [5] },
      { q: "لماذا التعليم محبوس داخلها؟", a: "لأن البيداغوجيا لم تُكتب قطّ برنامجًا. قواعدها في الوثائق وفي أذهان المعلّمين، وكل منصّة تعيد بناء نسخة مبسّطة منها لكل جمهور: N إطارًا في M تجربة.", cite: [] },
      { q: "لماذا يهمّ هذا الآن؟", a: "يصل المعلّمون والمصحّحون الآليون إلى كل قسم. ومن دون قواعد المدرسة داخل النظام فإنهم يخمّنون، ولا يستطيع أحد أن يتحقّق ممّا فعلوه أو يحكمه أو يتراجع عنه.", cite: [] },
    ],
    rootTitle: "فبنينا المحرّك الذي كان ينقص: البيداغوجيا برنامجًا.",
    root: "تحوّل زيدارا أي طريقة في التعليم إلى قواعد تشغّلها نواة مختبَرة. ثم تعمل فوقها أي تجربة وأي وكيل ذكاء اصطناعي، داخل قواعد يوافق عليها الناس.",
    sources: "المصادر",
  },
  sources: SOURCES,
  how: {
    title: "نواة واحدة. أي إطار. أي تجربة.",
    lead: "كل طريقة تعليم قارنّاها، من كامبريدج والتعلّم بالإتقان إلى معايير البكالوريا الدولية والتعليم بالكفايات والتكرار المتباعد، تعود إلى ستّ لبنات بناء نفسها. اللبنات تبقى، والقواعد وحدها تتغيّر.",
    layersTitle: "ثلاث طبقات، والأتمتة تمرّ عبرها كلّها",
    layers: [
      { title: "التجربة", body: "الشكل والإحساس لكل جمهور وجهاز: طريق للطفل، وشبكة للثانوي، واستوديو للمعلّم، وملخّص للأولياء، ومعلّم في محادثة." },
      { title: "الإطار", body: "معنى التعلّم: الأهداف والأدلّة والقواعد التي تحكم عليها. الإتقان، أو طريقة كامبريدج، أو طريقة المدرسة نفسها." },
      { title: "النواة", body: "الأشخاص والمجموعات والمحتوى والأنشطة وسجلّ الأدلّة والجدولة والأمان. تُبنى مرة وتُختبر بصرامة، بلا شبكة ولا ساعة داخلها." },
      { title: "الأتمتة", body: "أحداث وقواعد ووكلاء ذكاء اصطناعي فاعلون، عبر الطبقات الثلاث." },
    ],
    cards: [
      { title: "ستّ لبنات بناء", body: "الأهداف والأنشطة والأدلّة والحكم والتدرّج والأدوار. كل بيداغوجيا مجموعة قواعد فوق اللبنات الستّ نفسها." },
      { title: "أي إطار", body: "أشجار الأهداف والعتبات بيانات. والحكم والتدرّج شيفرة مترجمة خلف واجهة واحدة، تخضع للاختبارات نفسها. الإطار الجديد يكلّف أسابيع، لا منتجًا جديدًا." },
      { title: "أي تجربة، لا N × M", body: "الإطار يقول ما هو صحيح، والتجربة تقرّر كيف تعرضه. N إطارًا وM تجربة تكلّف N + M في البناء." },
      { title: "وكلاء الذكاء الاصطناعي فاعلون", body: "يمرّون بالصلاحيات نفسها التي يمرّ بها الناس. كل ما هو غير مؤكَّد أو ختامي يتوقّف عند المعلّم، ويمكن التراجع عن كل فعل." },
      { title: "دون إنترنت أولًا، على كل جهاز", body: "النواة نفسها تعمل على الخادم، وفي المتصفّح بصيغة WebAssembly، وداخل التطبيقات الأصلية. العمل المنجز دون اتصال يمرّ بالقواعد نفسها عند المزامنة." },
      { title: "العزل داخل قاعدة البيانات", body: "كل سطر يحمل قسمه ومدرسته، وأمان مستوى الأسطر في PostgreSQL يفصل بينها. لا شاشة ولا استعلام ولا وكيل يستطيع العبور." },
    ],
  },
  ai: {
    title: "مصنوع لعصر الذكاء الاصطناعي، ويحكمه الناس",
    lead: "النواة لا تستدعي نموذجًا أبدًا. الوكلاء يقترحون، وسياسة الإطار تقرّر ما يُطبَّق وما ينتظر المعلّم. كل فعل لوكيل مسجَّل ويمكن التراجع عنه.",
    flow: ["الوكيل يقترح", "القواعد تتحقّق", "المعلّم يقرّر"],
    agents: [
      { name: "وكيل التقييم", body: "يصحّح الإجابات المفتوحة مبدئيًا مع درجة ثقة. لا يُطبَّق إلا التقييم التكويني المؤكَّد، والختامي يعود دائمًا إلى المعلّم.", today: true },
      { name: "الوكيل المعلّم", body: "يقرأ أدلّة المتعلّم ويقترح الخطوة التالية. لا يقيّم عملًا ختاميًا أبدًا.", today: false },
      { name: "وكيل العمليات", body: "يرصد المتعلّمين المعرّضين للتعثّر ويفتح ملفًّا ليتابعه شخص.", today: false },
      { name: "وكيل التأليف", body: "يحوّل وثيقة منهاج إلى إطار أوّلي يوافق عليه مصمّم.", today: false },
    ],
    today: "يعمل اليوم",
    designed: "مصمَّم",
  },
  made: {
    title: "صُنع بزيدارا",
    lead: "كل ما تستطيع فتحه من هذه الصفحة يعمل على النواة نفسها. الدرس التجريبي تجربة واحدة من تجارب كثيرة، صُنع ليُظهر إلى أي حدّ ينثني المحرّك. بل إن أحد الدروس يعلّم زيدارا، بزيدارا.",
    items: [
      { title: "لعبة تعلّم", body: "درس على طريقة Duolingo وBrilliant للأعمار من 9 إلى 11: أفلام وألعاب وأسئلة تصحّحها النواة فورًا. المكافآت لا تأتي إلا من الأدلّة.", href: "#play", shot: "/landing/path.webp", alt: "طريق الكسور بعد حصّة واحدة: المستوى 1 متقَن بالأزرق وبجانبه نخلة من النقاط، والمستوى 2 هو التالي، وبذرة تنتظر." },
      { title: "جولات مباشرة", body: "جولة على سبّورة القسم على طريقة Kahoot، وكل هاتف يجيب. لا مؤقّت ولا ترتيب، والإجابات تصير أدلّة تمرّن.", href: "#classes", shot: "/landing/board.webp", alt: "استوديو المعلّم أثناء جولة مباشرة: السؤال، وثلاث إجابات ملوّنة، وأزرار لإظهار الإجابة أو المتابعة." },
      { title: "استوديو المعلّم", body: "تقدّم القسم، وتقييمات الوكيل المنتظرة، وكل تراجع، في مكان واحد.", href: "#classes", shot: "/landing/studio.webp", alt: "استوديو المعلّم: أين وصل القسم في كل هدف، والإجابات التي تنتظر قرارًا، وطريقة التدريس في القسم." },
      { title: "تبديل البيداغوجيا", body: "ينقل المعلّم القسم من الإتقان إلى طريقة كامبريدج، فيُحكم على كل متعلّم من جديد بالأدلّة نفسها.", href: "#classes", shot: "/landing/lens.webp", alt: "الطريق نفسه بقواعد طريقة كامبريدج: يعود المستوى 1 مفتوحًا، وتقول ملاحظة إن المحرّك، لا اللعبة، يحكم بقواعد المدرسة." },
    ],
    open: "افتح",
  },
  proof: {
    title: "أدلّة، لا وعود",
    lead: "أمر واحد يتحقّق من كل ادّعاء قبل أي نشر.",
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
      { q: "هل زيدارا لعبة؟", a: "لا. زيدارا محرّك تعلّم والمنصّة المحيطة به. اللعبة في هذا الموقع تجربة واحدة مبنية عليه، لتُظهر إلى أي حدّ ينثني المحرّك، والنواة نفسها ترسم شبكة للثانوي أو استوديو للمعلّم أو رسالة نصية." },
      { q: "لمن هي؟", a: "للمدارس ومراكز التكوين والأنظمة التعليمية التي تريد أن تعمل طريقتها في التعليم برنامجًا، وللفرق التي تريد بناء تجارب تعلّم جديدة على نواة مختبَرة. يبدأ النموذج برياضيات الابتدائي بالعربية والفرنسية والإنجليزية." },
      { q: "بماذا تختلف عن منصّات التعلّم الأخرى؟", a: "المنصّات العامة تُبقي طريقة التعليم داخل التطبيق وتضيف الباقي إضافاتٍ. في زيدارا البيداغوجيا هي القلب: الأطر في المقام الأول، والتجارب قابلة للتبديل، والوكلاء فاعلون محكومون بالقواعد." },
      { q: "هل يمنح الذكاء الاصطناعي العلامات للمتعلّمين؟", a: "لا. المحرّك لا يستدعي نموذجًا أبدًا. قد يصحّح وكيل التقييم إجابة مفتوحة مبدئيًا مع درجة ثقة، فتطبّقها سياسة الإطار أو ترسلها إلى المعلّم، وكل تقييم ختامي يعود دائمًا إلى إنسان." },
      { q: "هل نستطيع أن نأتي ببيداغوجيتنا؟", a: "نعم، وهذا هو المقصود. الأهداف والعتبات بيانات، والحكم والتدرّج شيفرة خلف واجهة واحدة تخضع للاختبارات نفسها. الإتقان وطريقة كامبريدج يعملان اليوم." },
      { q: "منتج أم نموذج أولي؟", a: "نموذج أولي يعمل. إطاران ودرسان واستوديو المعلّم والجولات المباشرة والمزامنة دون إنترنت والمقيّم الآلي المحكوم بالقواعد تعمل اليوم. لم يُجرَّب بعد في مدرسة." },
    ],
  },
  end: { title: "شاهد ما يستطيعه المحرّك", lead: "افتح درسًا تجريبيًا ضيفًا، أو استوديو المعلّم. لا حاجة إلى حساب.", primary: "جرّب درسًا تجريبيًا", classes: "افتح استوديو المعلّم" },
  footer: { line: "محرّك التعلّم القابل للتكيّف لعصر الذكاء الاصطناعي.", made: "صُنع بزيدارا", code: "الشيفرة المصدرية" },
};

export const LANDING: Record<Locale, LandingCopy> = { en, fr, ar };
