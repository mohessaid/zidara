#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::activity::{
    Activity, ActivityKind, ActivityRole, Expected, Input, Look, Part, Question, Toy, Visual,
};
use zidara_kernel::evidence::EvidenceKind;
use zidara_kernel::governance::Rubric;
use zidara_kernel::ids::{ActivityId, OutcomeId};
use zidara_kernel::lesson::{Lesson, LessonError};
use zidara_kernel::locale::Localized;
use zidara_kernel::outcome::{GraphError, Outcome};

const PACK: &str = include_str!("../../../content/lessons/equivalent-fractions.json");

fn outcome(id: &str) -> Outcome {
    Outcome {
        id: OutcomeId::new(id),
        title: Localized::default(),
        prerequisites: vec![],
    }
}

fn reading(id: &str, outcomes: &[&str]) -> Activity {
    Activity {
        id: ActivityId::new(id),
        title: Localized::default(),
        outcomes: outcomes.iter().map(|o| OutcomeId::new(*o)).collect(),
        role: ActivityRole::Core,
        kind: ActivityKind::Reading {
            body: Localized::default(),
            audio: None,
        },
    }
}

fn with_kind(id: &str, kind: ActivityKind) -> Activity {
    Activity {
        kind,
        ..reading(id, &[])
    }
}

#[test]
fn the_demo_pack_is_valid() {
    let lesson: Lesson = serde_json::from_str(PACK).unwrap();
    assert_eq!(lesson.id, "equivalent-fractions");
    assert_eq!(lesson.title.get("ar"), "الكسور المتكافئة");
    assert_eq!(lesson.graph.len(), 3);
    assert_eq!(lesson.activities().len(), 10);
    assert!(lesson.activity(&ActivityId::new("quiz-f1")).is_some());
    assert!(lesson.activity(&ActivityId::new("nope")).is_none());
}

#[test]
fn remediation_is_found_by_outcome() {
    let lesson: Lesson = serde_json::from_str(PACK).unwrap();
    let f2 = lesson.remediation_for(&OutcomeId::new("F2")).unwrap();
    assert_eq!(f2.id, ActivityId::new("remedial-f2"));
    // reading-halves covers F1 but is core, not remediation.
    assert!(lesson.remediation_for(&OutcomeId::new("F1")).is_none());
}

#[test]
fn rejects_graph_errors() {
    let err = Lesson::new(
        "x",
        Localized::default(),
        vec![outcome("F1"), outcome("F1")],
        vec![],
    )
    .unwrap_err();
    assert_eq!(
        err,
        LessonError::Graph(GraphError::Duplicate(OutcomeId::new("F1")))
    );
}

#[test]
fn rejects_duplicate_activities() {
    let err = Lesson::new(
        "x",
        Localized::default(),
        vec![outcome("F1")],
        vec![reading("a", &["F1"]), reading("a", &["F1"])],
    )
    .unwrap_err();
    assert_eq!(err, LessonError::DuplicateActivity(ActivityId::new("a")));
}

#[test]
fn rejects_unknown_outcomes_wherever_they_appear() {
    let unknown = |activity: Activity| {
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![activity],
        )
        .unwrap_err()
    };
    let expected = |a: &str| LessonError::UnknownOutcome {
        activity: ActivityId::new(a),
        outcome: OutcomeId::new("F9"),
    };
    assert_eq!(unknown(reading("r", &["F9"])), expected("r"));
    let question = Question {
        id: "q".into(),
        outcome: OutcomeId::new("F9"),
        prompt: Localized::default(),
        expected: Expected::Exactly { num: 1, den: 2 },
        look: Look::default(),
    };
    assert_eq!(
        unknown(with_kind(
            "q",
            ActivityKind::Quiz {
                evidence: EvidenceKind::Formative,
                questions: vec![question],
                variants: vec![]
            }
        )),
        expected("q")
    );
    let open = ActivityKind::OpenResponse {
        evidence: EvidenceKind::Formative,
        outcome: OutcomeId::new("F9"),
        prompt: Localized::default(),
        rubric: Rubric {
            criteria: vec![],
            min_words: 0,
        },
    };
    assert_eq!(unknown(with_kind("o", open)), expected("o"));
}

#[test]
fn rejects_choices_without_the_correct_option() {
    let question = |correct| Question {
        id: "q".into(),
        outcome: OutcomeId::new("F1"),
        prompt: Localized::default(),
        look: Look::default(),
        expected: Expected::Choice {
            options: vec![Localized::default(); 2],
            correct,
        },
    };
    let quiz = |correct| {
        with_kind(
            "q",
            ActivityKind::Quiz {
                evidence: EvidenceKind::Formative,
                questions: vec![question(correct)],
                variants: vec![],
            },
        )
    };
    assert!(
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![quiz(1)]
        )
        .is_ok()
    );
    assert_eq!(
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![quiz(2)]
        )
        .unwrap_err(),
        LessonError::BadChoice {
            activity: ActivityId::new("q"),
            question: "q".into(),
            correct: 2
        }
    );
}

#[test]
fn a_bad_pack_fails_to_deserialize() {
    let pack = r#"{"id":"x","title":{},"outcomes":[{"id":"F1","title":{}}],
                   "activities":[{"id":"r","title":{},"outcomes":["F2"],
                                  "kind":{"type":"reading","body":{}}}]}"#;
    let err = serde_json::from_str::<Lesson>(pack).unwrap_err();
    assert!(err.to_string().contains("unknown outcome"), "{err}");
}

#[test]
fn every_version_of_a_quiz_is_checked() {
    let choice = |correct| Question {
        id: "q".into(),
        outcome: OutcomeId::new("F1"),
        prompt: Localized::default(),
        look: Look::default(),
        expected: Expected::Choice {
            options: vec![Localized::default(); 2],
            correct,
        },
    };
    let quiz = |variant: Question| {
        with_kind(
            "q",
            ActivityKind::Quiz {
                evidence: EvidenceKind::Formative,
                questions: vec![choice(0)],
                variants: vec![vec![variant]],
            },
        )
    };
    assert!(
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![quiz(choice(1))]
        )
        .is_ok()
    );
    assert_eq!(
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![quiz(choice(5))]
        )
        .unwrap_err(),
        LessonError::BadChoice {
            activity: ActivityId::new("q"),
            question: "q".into(),
            correct: 5
        }
    );
    let elsewhere = Question {
        outcome: OutcomeId::new("F9"),
        ..choice(0)
    };
    assert_eq!(
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![quiz(elsewhere)]
        )
        .unwrap_err(),
        LessonError::UnknownOutcome {
            activity: ActivityId::new("q"),
            outcome: OutcomeId::new("F9")
        }
    );
}

fn pack_with(expected: Expected, look: Look) -> Result<Lesson, LessonError> {
    let question = Question {
        id: "q".into(),
        outcome: OutcomeId::new("F1"),
        prompt: Localized::default(),
        look,
        expected,
    };
    Lesson::new(
        "x",
        Localized::default(),
        vec![outcome("F1")],
        vec![with_kind(
            "a",
            ActivityKind::Quiz {
                evidence: EvidenceKind::Formative,
                questions: vec![],
                variants: vec![vec![question]],
            },
        )],
    )
}

fn looks(visual: Option<Visual>, input: Input) -> Look {
    Look {
        visual,
        input,
        explain: None,
    }
}

const fn exactly(num: u32, den: u32) -> Expected {
    Expected::Exactly { num, den }
}

const fn equal_to(num: u32, den: u32) -> Expected {
    Expected::EquivalentTo { num, den }
}

fn ok(expected: Expected, look: Look) -> bool {
    match pack_with(expected, look) {
        Ok(_) => true,
        Err(LessonError::BadLook { activity, question }) => {
            assert_eq!((activity, question.as_str()), (ActivityId::new("a"), "q"));
            false
        }
        Err(other) => panic!("{other}"),
    }
}

#[test]
fn pictures_must_be_drawable() {
    let bar = |parts, shaded| Some(Visual::Bar { parts, shaded });
    let keypad = |v| ok(exactly(1, 2), looks(v, Input::Keypad));
    assert!(keypad(None));
    assert!(keypad(bar(1, 0)));
    assert!(keypad(bar(24, 24)));
    assert!(!keypad(bar(0, 0)));
    assert!(!keypad(bar(25, 1)));
    assert!(!keypad(bar(4, 5)));
    assert!(keypad(Some(Visual::Circle {
        parts: 8,
        shaded: 3
    })));
    assert!(!keypad(Some(Visual::Circle {
        parts: 3,
        shaded: 4
    })));
    let pair = |top, bottom| Some(Visual::Pair { top, bottom });
    let (good, bad) = (
        Part {
            parts: 3,
            shaded: 2,
        },
        Part {
            parts: 2,
            shaded: 3,
        },
    );
    assert!(keypad(pair(good, good)));
    assert!(!keypad(pair(bad, good)));
    assert!(!keypad(pair(good, bad)));
}

#[test]
fn a_shading_input_must_be_able_to_give_the_answer() {
    let shade = |expected, parts| ok(expected, looks(None, Input::Shade { parts }));
    // Exactly 3/8 needs a bar in eighths.
    assert!(shade(exactly(3, 8), 8));
    assert!(!shade(exactly(3, 8), 4));
    assert!(!shade(exactly(9, 8), 8));
    // Any fraction equal to 3/4: 6/8 works, sixths cannot.
    assert!(shade(equal_to(3, 4), 8));
    assert!(shade(equal_to(3, 4), 4));
    assert!(!shade(equal_to(3, 4), 6));
    assert!(!shade(equal_to(5, 4), 8));
    assert!(shade(equal_to(1, 1), 24));
    assert!(!shade(equal_to(1, 1), 25));
    assert!(!shade(equal_to(0, 1), 0));
    // No whole has zero parts.
    assert!(!shade(equal_to(0, 0), 2));
    assert!(!ok(exactly(0, 0), looks(None, Input::Cut { max: 4 })));
    // A choice is not shaded.
    let choice = Expected::Choice {
        options: vec![Localized::default()],
        correct: 0,
    };
    assert!(ok(choice.clone(), Look::default()));
    assert!(!ok(choice.clone(), looks(None, Input::Shade { parts: 2 })));
    assert!(!ok(choice, looks(None, Input::Cut { max: 2 })));
}

#[test]
fn a_cutting_input_must_be_able_to_give_the_answer() {
    let cut = |expected, max| ok(expected, looks(None, Input::Cut { max }));
    assert!(cut(exactly(2, 5), 5));
    assert!(cut(exactly(2, 5), 12));
    assert!(!cut(exactly(2, 5), 4));
    assert!(!cut(exactly(6, 5), 12));
    // 4/6 is 2/3 at its simplest: thirds are enough.
    assert!(cut(equal_to(4, 6), 3));
    assert!(!cut(equal_to(4, 6), 2));
    assert!(!cut(equal_to(7, 6), 12));
    assert!(cut(equal_to(0, 3), 1));
    assert!(cut(exactly(1, 24), 24));
    assert!(!cut(exactly(1, 1), 25));
    assert!(!cut(exactly(0, 1), 0));
}

#[test]
fn toys_must_be_drawable() {
    let explore = |toy| {
        Lesson::new(
            "x",
            Localized::default(),
            vec![outcome("F1")],
            vec![with_kind(
                "e",
                ActivityKind::Explore {
                    body: Localized::default(),
                    toy,
                },
            )],
        )
        .map(|_| ())
    };
    let bad = Err(LessonError::BadToy(ActivityId::new("e")));
    assert_eq!(explore(Toy::Fold { folds: 1 }), Ok(()));
    assert_eq!(explore(Toy::Fold { folds: 4 }), Ok(()));
    assert_eq!(explore(Toy::Fold { folds: 0 }), bad);
    assert_eq!(explore(Toy::Fold { folds: 5 }), bad);
    assert_eq!(explore(Toy::Wall { rows: vec![1, 12] }), Ok(()));
    assert_eq!(explore(Toy::Wall { rows: vec![] }), bad);
    assert_eq!(explore(Toy::Wall { rows: vec![2, 0] }), bad);
    assert_eq!(explore(Toy::Wall { rows: vec![13] }), bad);
}

#[test]
fn a_question_without_a_look_gets_the_plain_one() {
    let pack = r#"{"id":"x","title":{},"outcomes":[{"id":"F1","title":{}}],
                   "activities":[{"id":"a","title":{},"outcomes":["F1"],
                     "kind":{"type":"quiz","evidence":"formative","questions":[
                       {"id":"q","outcome":"F1","prompt":{},"expected":{"type":"exactly","num":1,"den":2}},
                       {"id":"r","outcome":"F1","prompt":{},"expected":{"type":"exactly","num":1,"den":2},
                        "look":{"visual":{"type":"bar","parts":2,"shaded":1},"input":{"type":"shade","parts":2},
                                "explain":{"en":"One of two equal parts."}}}]}}]}"#;
    let lesson: Lesson = serde_json::from_str(pack).unwrap();
    let ActivityKind::Quiz { questions, .. } = &lesson.activities()[0].kind else {
        panic!()
    };
    assert_eq!(questions[0].look, Look::default());
    assert_eq!(questions[0].look.input, Input::Keypad);
    assert_eq!(
        questions[1].look,
        Look {
            visual: Some(Visual::Bar {
                parts: 2,
                shaded: 1
            }),
            input: Input::Shade { parts: 2 },
            explain: Some(Localized::from_pairs(&[("en", "One of two equal parts.")])),
        }
    );
}

#[test]
fn a_live_round_asks_the_first_version_of_a_quiz_and_nothing_else() {
    let lesson: Lesson = serde_json::from_str(PACK).unwrap();
    let asked: Vec<&str> = lesson
        .questions(&ActivityId::new("quiz-f1"))
        .iter()
        .map(|q| q.id.as_str())
        .collect();
    let ActivityKind::Quiz { questions, .. } =
        &lesson.activity(&ActivityId::new("quiz-f1")).unwrap().kind
    else {
        panic!()
    };
    assert_eq!(
        asked,
        questions.iter().map(|q| q.id.as_str()).collect::<Vec<_>>()
    );
    assert!(!asked.is_empty());
    assert!(
        lesson
            .questions(&ActivityId::new("reading-halves"))
            .is_empty()
    );
    assert!(lesson.questions(&ActivityId::new("nope")).is_empty());
}
