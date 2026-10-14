#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::activity::{Activity, ActivityKind, ActivityRole, Expected, Question};
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
    assert_eq!(lesson.activities().len(), 8);
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
    };
    assert_eq!(
        unknown(with_kind(
            "q",
            ActivityKind::Quiz {
                evidence: EvidenceKind::Formative,
                questions: vec![question]
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
