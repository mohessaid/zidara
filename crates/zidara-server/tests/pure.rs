//! The server's pure parts: no database, so these also feed mutation testing quickly.

use std::collections::BTreeMap;

use serde_json::json;
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::evidence::{Evidence, EvidenceKind};
use zidara_kernel::governance::{Assessment, Decision, Proposal};
use zidara_kernel::ids::*;
use zidara_kernel::roles::{Actor, Role};
use zidara_kernel::rules::NotifyReason;
use zidara_kernel::score::Score;
use zidara_server::ids::UuidV7;
use zidara_server::visibility::learner_of;
use zidara_server::xapi::{rfc3339, statement};

fn user(n: u128) -> UserId {
    UserId(Uuid::from_u128(n))
}

fn eid(n: u128) -> EventId {
    EventId(Uuid::from_u128(n))
}

fn evidence() -> Evidence {
    Evidence {
        learner: user(1),
        outcome: OutcomeId::new("F2"),
        activity: ActivityId::new("quiz-f2"),
        score: Score::of(8_000),
        kind: EvidenceKind::Formative,
    }
}

fn proposal() -> Proposal {
    Proposal {
        answer: eid(9),
        learner: user(1),
        outcome: OutcomeId::new("F2"),
        activity: ActivityId::new("explain-f2"),
        kind: EvidenceKind::Formative,
        assessment: Assessment {
            score: Score::FULL,
            confidence: Score::of(9_000),
            rationale: "r".into(),
        },
        decision: Decision::Queue,
    }
}

#[test]
fn learner_bodies_name_their_learner() {
    let known = BTreeMap::new();
    let a = ActivityId::new("x");
    let o = OutcomeId::new("F1");
    let bodies = [
        EventBody::QuizSubmitted {
            learner: user(1),
            activity: a.clone(),
            responses: vec![],
        },
        EventBody::AnswerSubmitted {
            learner: user(1),
            activity: a.clone(),
            answer: "a".into(),
        },
        EventBody::EvidenceRecorded {
            evidence: evidence(),
        },
        EventBody::ActivityAssigned {
            learner: user(1),
            outcome: o.clone(),
            activity: a.clone(),
        },
        EventBody::TeacherNotified {
            learner: user(1),
            outcome: o,
            reason: NotifyReason::Struggling,
        },
        EventBody::ProposalMade {
            proposal: proposal(),
        },
    ];
    for body in bodies {
        assert_eq!(learner_of(&body, &known), Some(user(1)), "{body:?}");
    }
}

#[test]
fn cohort_wide_bodies_have_no_learner() {
    let body = EventBody::FrameworkChanged {
        framework: FrameworkId::new("mastery"),
    };
    assert_eq!(learner_of(&body, &BTreeMap::new()), None);
}

#[test]
fn undo_and_decisions_belong_to_their_target_learner() {
    let known = BTreeMap::from([(eid(5), Some(user(2))), (eid(6), None)]);
    assert_eq!(
        learner_of(&EventBody::Undone { target: eid(5) }, &known),
        Some(user(2))
    );
    assert_eq!(
        learner_of(
            &EventBody::ProposalDecided {
                proposal: eid(5),
                accept: true
            },
            &known
        ),
        Some(user(2))
    );
    assert_eq!(
        learner_of(&EventBody::Undone { target: eid(6) }, &known),
        None
    );
    assert_eq!(
        learner_of(&EventBody::Undone { target: eid(7) }, &known),
        None
    );
}

#[test]
fn v7_ids_rise_and_never_repeat() {
    let mut ids = UuidV7;
    let all: Vec<EventId> = (0..1_000).map(|_| ids.next_id()).collect();
    assert!(all.windows(2).all(|w| w[0] < w[1]));
    assert_eq!(all[0].0.get_version_num(), 7);
}

#[test]
fn timestamps_are_rfc3339_in_utc() {
    assert_eq!(rfc3339(0), "1970-01-01T00:00:00.000Z");
    assert_eq!(rfc3339(1_791_158_400_000), "2026-10-05T00:00:00.000Z");
    assert_eq!(rfc3339(1_709_210_096_789), "2024-02-29T12:34:56.789Z");
    assert_eq!(rfc3339(951_782_400_000), "2000-02-29T00:00:00.000Z");
    assert_eq!(rfc3339(4_107_542_399_999), "2100-02-28T23:59:59.999Z");
    assert_eq!(rfc3339(-1), "1969-12-31T23:59:59.999Z");
    // Century edges, where the algorithm's 100- and 400-year corrections matter
    // (found by mutation testing: the earlier vectors never reached them).
    assert_eq!(rfc3339(4_107_542_400_000), "2100-03-01T00:00:00.000Z");
    assert_eq!(rfc3339(13_574_649_599_999), "2400-02-29T23:59:59.999Z");
}

fn event(body: EventBody) -> Event {
    Event {
        id: eid(42),
        at: Timestamp(1_791_158_400_000),
        tenant: TenantId(Uuid::from_u128(1)),
        cohort: CohortId(Uuid::from_u128(2)),
        actor: Actor::Agent {
            agent: AgentId::new("assessor"),
            role: Role::AssessmentAgent,
        },
        body,
    }
}

#[test]
fn evidence_becomes_an_xapi_statement() {
    let s = statement(
        &event(EventBody::EvidenceRecorded {
            evidence: evidence(),
        }),
        "equivalent-fractions",
    )
    .unwrap();
    assert_eq!(
        s,
        json!({
            "id": "00000000-0000-0000-0000-00000000002a",
            "actor": { "objectType": "Agent", "account": { "homePage": "urn:zidara", "name": "00000000-0000-0000-0000-000000000001" } },
            "verb": { "id": "http://adlnet.gov/expapi/verbs/scored", "display": { "en-US": "scored" } },
            "object": { "objectType": "Activity", "id": "urn:zidara:activity:equivalent-fractions/quiz-f2" },
            "result": { "score": { "scaled": 0.8, "raw": 8000, "min": 0, "max": 10000 } },
            "context": { "extensions": {
                "urn:zidara:outcome": "F2",
                "urn:zidara:kind": "formative",
                "urn:zidara:recorded-by": { "kind": "agent", "agent": "assessor", "role": "assessment_agent" }
            } },
            "timestamp": "2026-10-05T00:00:00.000Z"
        })
    );
}

#[test]
fn only_evidence_becomes_a_statement() {
    let body = EventBody::FrameworkChanged {
        framework: FrameworkId::new("mastery"),
    };
    assert_eq!(statement(&event(body), "l"), None);
}

mod assessor_parts {
    use serde_json::json;
    use zidara_kernel::governance::{Criterion, MIXED, Rubric};
    use zidara_kernel::score::Score;
    use zidara_server::assessor::{OpenItem, fallback, read_reply, request_body, uses_fallbacks};

    fn rubric() -> Rubric {
        Rubric {
            criteria: vec![Criterion {
                any_of: vec!["multiply".into(), "times".into()],
                weight: 2,
            }],
            min_words: 1,
        }
    }

    #[test]
    fn only_models_with_server_fallbacks_ask_for_them() {
        for model in [
            "claude-opus-5-5",
            "claude-opus-5",
            "claude-fable-5-1",
            "claude-sonnet-5-5",
        ] {
            assert!(uses_fallbacks(model), "{model}");
        }
        for model in ["claude-haiku-5-5", "claude-sonnet-5", "anything"] {
            assert!(!uses_fallbacks(model), "{model}");
        }
        let r = rubric();
        let item = OpenItem {
            prompt: "Q?",
            rubric: &r,
        };
        assert!(
            request_body("claude-haiku-5-5", &item, "a")
                .get("fallbacks")
                .is_none()
        );
        let body = request_body("claude-opus-5-5", &item, "my answer");
        assert_eq!(body["fallbacks"], "default");
        assert_eq!(body["output_config"]["effort"], "low");
        let content = body["messages"][0]["content"].as_str().unwrap();
        assert!(content.contains("Question:\nQ?"), "{content}");
        assert!(
            content.contains("- weight 2: multiply | times"),
            "{content}"
        );
        assert!(
            content.contains("<answer>\nmy answer\n</answer>"),
            "{content}"
        );
    }

    fn text(t: &str) -> serde_json::Value {
        json!({ "stop_reason": "end_turn", "content": [ { "type": "text", "text": t } ] })
    }

    #[test]
    fn replies_are_validated() {
        let good = read_reply(&text(
            r#"{"score_bp":0,"confidence_bp":10000,"rationale":"r"}"#,
        ))
        .unwrap();
        assert_eq!(
            (good.score, good.confidence, good.rationale.as_str()),
            (Score::ZERO, Score::FULL, "r")
        );
        for bad in [
            json!({ "stop_reason": "end_turn" }),
            json!({ "stop_reason": "end_turn", "content": [ { "type": "thinking", "thinking": "" } ] }),
            text(r#"{"score_bp":-1,"confidence_bp":1,"rationale":"r"}"#),
            text(r#"{"score_bp":70000,"confidence_bp":1,"rationale":"r"}"#),
            text(r#"{"score_bp":1,"confidence_bp":10001,"rationale":"r"}"#),
            text(r#"{"score_bp":1,"confidence_bp":1}"#),
        ] {
            assert_eq!(read_reply(&bad), None, "{bad}");
        }
    }

    #[test]
    fn fallbacks_keep_the_score_and_cap_confidence() {
        let r = rubric();
        let unsure = fallback(&r, "you multiply", "down");
        assert_eq!((unsure.score, unsure.confidence), (Score::FULL, MIXED));
        assert!(
            unsure.rationale.starts_with("rubric fallback (down): "),
            "{}",
            unsure.rationale
        );
        // Already below the cap: kept as it is.
        let short = fallback(&Rubric { min_words: 5, ..r }, "multiply", "down");
        assert!(short.confidence < MIXED);
    }
}
