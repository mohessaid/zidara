//! The rubric assessor and the agent, on the real database. Claude over HTTP: tests/claude.rs.
mod support;

use support::*;
use uuid::Uuid;
use zidara_kernel::decide::{Command, Envelope};
use zidara_kernel::event::EventBody;
use zidara_kernel::governance::{CLEAR, Criterion, Rubric};
use zidara_kernel::ids::{ActivityId, Timestamp};
use zidara_kernel::score::Score;
use zidara_kernel::state::{ProposalStatus, fold};
use zidara_server::agent::mark_pending;
use zidara_server::assessor::{Assessor, OpenItem};
use zidara_server::commands::apply;
use zidara_server::school::{FixedClock, School};
use zidara_server::store;

const NOW: i64 = 1_791_158_400_000 + 3_600_000;

fn rubric() -> Rubric {
    Rubric {
        criteria: vec![
            Criterion {
                any_of: vec!["multiply".into()],
                weight: 1,
            },
            Criterion {
                any_of: vec!["same".into()],
                weight: 1,
            },
        ],
        min_words: 3,
    }
}

fn item(rubric: &Rubric) -> OpenItem<'_> {
    OpenItem {
        prompt: "How do you make a fraction equal to 1/2?",
        rubric,
    }
}

#[tokio::test]
async fn the_rubric_assessor_is_the_kernels_rubric() {
    let rubric = rubric();
    let got = Assessor::Rubric
        .assess(&item(&rubric), "multiply by the same")
        .await;
    assert_eq!((got.score, got.confidence), (Score::FULL, CLEAR));
}

fn answer(text: &str) -> Envelope {
    Envelope {
        id: Uuid::now_v7(),
        at: Timestamp(NOW),
        command: Command::SubmitAnswer {
            activity: ActivityId::new("explain-f2"),
            answer: text.into(),
        },
    }
}

#[tokio::test]
async fn the_agent_marks_each_answer_once_and_the_policy_decides() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        answer("You multiply the top and the bottom by the same number"),
    )
    .await
    .unwrap();
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.yacine,
        db.seed.a,
        answer("it is the same"),
    )
    .await
    .unwrap();
    let marked = mark_pending(
        &db.app,
        &school,
        &clock,
        &Assessor::Rubric,
        db.seed.agent,
        db.seed.a,
    )
    .await
    .unwrap();
    assert_eq!(marked, 2);
    assert_eq!(
        mark_pending(
            &db.app,
            &school,
            &clock,
            &Assessor::Rubric,
            db.seed.agent,
            db.seed.a
        )
        .await
        .unwrap(),
        0
    );
    let mut karim = store::begin_as(&db.app, db.seed.karim).await.unwrap();
    let state = fold(&store::history(&mut karim, db.seed.a).await.unwrap());
    let mut statuses: Vec<(Uuid, ProposalStatus)> = state
        .proposals
        .values()
        .map(|(p, s)| (p.learner.0, *s))
        .collect();
    statuses.sort_by_key(|(learner, _)| *learner);
    let mut expected = vec![
        (db.seed.amina, ProposalStatus::Applied),
        (db.seed.yacine, ProposalStatus::Queued),
    ];
    expected.sort_by_key(|(learner, _)| *learner);
    assert_eq!(statuses, expected);
    // The applied mark was recorded by the agent, as evidence.
    let by_agent = store::history(&mut karim, db.seed.a)
        .await
        .unwrap()
        .into_iter()
        .filter(|e| {
            matches!(e.body, EventBody::EvidenceRecorded { .. })
                && matches!(e.actor, zidara_kernel::roles::Actor::Agent { .. })
        })
        .count();
    assert_eq!(by_agent, 1);
}

#[tokio::test]
async fn the_agent_only_marks_cohorts_it_serves() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let err = mark_pending(
        &db.app,
        &school,
        &clock,
        &Assessor::Rubric,
        db.seed.karim,
        db.seed.b,
    )
    .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn the_agent_skips_answers_it_cannot_mark_and_needs_a_lesson() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    // An answer recorded against a quiz (possible only by writing to the log directly) is skipped.
    let body = serde_json::to_value(EventBody::AnswerSubmitted {
        learner: zidara_kernel::ids::UserId(db.seed.amina),
        activity: ActivityId::new("quiz-f1"),
        answer: "x".into(),
    })
    .unwrap();
    sqlx::query("insert into events (id, tenant_id, cohort_id, learner_id, at_ms, actor, body) values ($1, $2, $3, $4, $5, '{\"kind\":\"system\"}', $6)")
        .bind(Uuid::now_v7())
        .bind(db.seed.tenant)
        .bind(db.seed.a)
        .bind(db.seed.amina)
        .bind(NOW)
        .bind(body)
        .execute(&db.owner)
        .await
        .unwrap();
    assert_eq!(
        mark_pending(
            &db.app,
            &school,
            &clock,
            &Assessor::Rubric,
            db.seed.agent,
            db.seed.a
        )
        .await
        .unwrap(),
        0
    );
    sqlx::query("update cohorts set lesson_id = 'gone' where id = $1")
        .bind(db.seed.a)
        .execute(&db.owner)
        .await
        .unwrap();
    let err = mark_pending(
        &db.app,
        &school,
        &clock,
        &Assessor::Rubric,
        db.seed.agent,
        db.seed.a,
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, zidara_server::error::ApiError::UnknownLesson(_)),
        "{err:?}"
    );
}
