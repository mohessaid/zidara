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
            met: None,
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
            variant: 0,
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
            json!({ "stop_reason": "end_turn", "content": [ { "type": "text", "text": 5 } ] }),
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

mod auth_tokens {
    use uuid::Uuid;
    use zidara_kernel::ids::Timestamp;
    use zidara_server::auth::{AuthError, Keys, TOKEN_LIFETIME_MS, issue, verify};

    const NOW: i64 = 1_791_158_400_000;

    #[test]
    fn a_token_names_its_user_until_it_expires() {
        let keys = Keys::from_secret(b"a test secret that is long enough");
        let user = Uuid::from_u128(7);
        let token = issue(&keys, user, Timestamp(NOW));
        assert_eq!(verify(&keys, &token, Timestamp(NOW)), Ok(user));
        assert_eq!(
            verify(&keys, &token, Timestamp(NOW + TOKEN_LIFETIME_MS - 1_000)),
            Ok(user)
        );
        assert_eq!(
            verify(&keys, &token, Timestamp(NOW + TOKEN_LIFETIME_MS)),
            Err(AuthError::Expired)
        );
        assert_eq!(TOKEN_LIFETIME_MS, 12 * 60 * 60 * 1000);
    }

    #[test]
    fn forged_and_broken_tokens_are_refused() {
        let keys = Keys::from_secret(b"a test secret that is long enough");
        let token = issue(&keys, Uuid::from_u128(7), Timestamp(NOW));
        let other = Keys::from_secret(b"another secret, also long enough");
        assert_eq!(
            verify(&other, &token, Timestamp(NOW)),
            Err(AuthError::Invalid)
        );
        let mut tampered = token.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'A' { 'B' } else { 'A' });
        assert_eq!(
            verify(&keys, &tampered, Timestamp(NOW)),
            Err(AuthError::Invalid)
        );
        assert_eq!(
            verify(&keys, "not a token", Timestamp(NOW)),
            Err(AuthError::Invalid)
        );
        assert_eq!(format!("{keys:?}"), "Keys(..)");
    }
}

mod config {
    use std::collections::BTreeMap;

    use zidara_server::config::{AssessorConfig, Config};

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: BTreeMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn the_minimum_is_a_database_and_a_secret() {
        let config = Config::from_env(env(&[
            ("DATABASE_URL", "postgres://x"),
            ("ZIDARA_SECRET", "s3cret-s3cret-s3cret-s3cret-s3cret"),
        ]))
        .unwrap();
        assert_eq!(config.database_url, "postgres://x");
        assert_eq!(config.addr, "127.0.0.1:8080");
        assert!(!config.demo_login);
        assert_eq!(config.web_dir, None);
        assert_eq!(config.assessor, AssessorConfig::Rubric);
        assert_eq!(config.agent, zidara_server::seed::Seed::DEMO.agent);
    }

    #[test]
    fn missing_or_weak_settings_are_errors() {
        assert_eq!(
            Config::from_env(env(&[(
                "ZIDARA_SECRET",
                "s3cret-s3cret-s3cret-s3cret-s3cret"
            )]))
            .unwrap_err(),
            "DATABASE_URL is not set"
        );
        assert_eq!(
            Config::from_env(env(&[("DATABASE_URL", "x")])).unwrap_err(),
            "ZIDARA_SECRET is not set"
        );
        assert_eq!(
            Config::from_env(env(&[
                ("DATABASE_URL", "x"),
                ("ZIDARA_SECRET", &"s".repeat(31))
            ]))
            .unwrap_err(),
            "ZIDARA_SECRET must be at least 32 bytes"
        );
        assert!(
            Config::from_env(env(&[
                ("DATABASE_URL", "x"),
                ("ZIDARA_SECRET", &"s".repeat(32))
            ]))
            .is_ok()
        );
        assert_eq!(
            Config::from_env(env(&[
                ("DATABASE_URL", "x"),
                ("ZIDARA_SECRET", "s3cret-s3cret-s3cret-s3cret-s3cret"),
                ("ZIDARA_AGENT_ID", "nope")
            ]))
            .unwrap_err(),
            "ZIDARA_AGENT_ID is not a UUID"
        );
    }

    #[test]
    fn claude_is_used_when_a_key_is_set() {
        let config = Config::from_env(env(&[
            ("DATABASE_URL", "x"),
            ("ZIDARA_SECRET", "s3cret-s3cret-s3cret-s3cret-s3cret"),
            ("ANTHROPIC_API_KEY", "k"),
            ("ZIDARA_ASSESSOR_MODEL", "claude-haiku-5-5"),
            ("ZIDARA_DEMO_LOGIN", "1"),
            ("ZIDARA_ADDR", "0.0.0.0:9000"),
            ("ZIDARA_WEB_DIR", "/srv/zidara/web"),
            ("ZIDARA_AGENT_ID", "00000000-0000-0000-0000-000000000007"),
        ]))
        .unwrap();
        assert_eq!(
            config.assessor,
            AssessorConfig::Claude {
                base_url: "https://api.anthropic.com".into(),
                api_key: "k".into(),
                model: "claude-haiku-5-5".into()
            }
        );
        assert!(config.demo_login);
        assert_eq!(config.web_dir.as_deref(), Some("/srv/zidara/web"));
        assert_eq!(config.addr, "0.0.0.0:9000");
        assert_eq!(config.agent, uuid::Uuid::from_u128(7));
        let default_model = Config::from_env(env(&[
            ("DATABASE_URL", "x"),
            ("ZIDARA_SECRET", "s3cret-s3cret-s3cret-s3cret-s3cret"),
            ("ANTHROPIC_API_KEY", "k"),
        ]))
        .unwrap();
        assert!(
            matches!(default_model.assessor, AssessorConfig::Claude { ref model, .. } if model == "claude-opus-5-5")
        );
        assert_eq!(
            format!("{config:?}"),
            "Config { addr: \"0.0.0.0:9000\", demo_login: true, agent: 00000000-0000-0000-0000-000000000007, \
             assessor: Claude { base_url: \"https://api.anthropic.com\", model: \"claude-haiku-5-5\" }, web_dir: Some(\"/srv/zidara/web\"), .. }"
        );
    }
}

mod clock_and_debug {
    use zidara_server::config::AssessorConfig;
    use zidara_server::school::{Clock, SystemClock};

    #[test]
    fn the_system_clock_reads_real_time() {
        let now = SystemClock.now().0;
        // After 1 January 2026 and before 2100: the clock is wired to real time, in milliseconds.
        assert!(now > 1_767_225_600_000 && now < 4_102_444_800_000, "{now}");
    }

    #[test]
    fn the_rubric_assessor_prints_its_name() {
        assert_eq!(format!("{:?}", AssessorConfig::Rubric), "Rubric");
    }
}

mod agent_passes {
    use uuid::Uuid;
    use zidara_server::agent::Passes;

    #[test]
    fn one_marking_pass_per_class_at_a_time() {
        let passes = Passes::default();
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        assert!(passes.request(a), "the first request starts a pass");
        assert!(
            !passes.request(a),
            "a second one, during the pass, does not"
        );
        assert!(!passes.request(a), "nor a third");
        assert!(passes.request(b), "another class has its own pass");
        // Answers arrived during the pass, so exactly one more pass runs, then none.
        assert!(passes.finished(a));
        assert!(!passes.finished(a));
        assert!(
            passes.request(a),
            "after that, a new answer starts a new pass"
        );
        assert!(!passes.finished(a));
        assert!(!passes.finished(b));
    }
}
