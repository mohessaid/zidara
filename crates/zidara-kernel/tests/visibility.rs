mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use support::*;
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::ids::*;
use zidara_kernel::roles::Role;
use zidara_kernel::visibility::visible_to;

fn ids(events: &[&Event]) -> Vec<u128> {
    events.iter().map(|e| e.id.0.as_u128()).collect()
}

#[test]
fn learners_see_their_own_events_and_cohort_wide_ones_teachers_and_agents_see_all() {
    let log = vec![
        event(
            1,
            10,
            teacher(),
            EventBody::FrameworkChanged {
                framework: FrameworkId::new("test"),
            },
        ),
        event(
            2,
            11,
            teacher(),
            EventBody::EvidenceRecorded {
                evidence: evidence(1, "F1", 9_000),
                proposal: None,
            },
        ),
        event(
            3,
            12,
            teacher(),
            EventBody::EvidenceRecorded {
                evidence: evidence(2, "F1", 9_000),
                proposal: None,
            },
        ),
        // Undoing points at an earlier event: it belongs to that event's learner.
        event(
            4,
            13,
            teacher(),
            EventBody::Undone {
                target: EventId(Uuid::from_u128(3)),
            },
        ),
        event(
            5,
            14,
            teacher(),
            EventBody::Undone {
                target: EventId(Uuid::from_u128(2)),
            },
        ),
    ];
    assert_eq!(
        ids(&visible_to(&log, user(1), Role::Learner)),
        vec![1, 2, 5]
    );
    assert_eq!(
        ids(&visible_to(&log, user(2), Role::Learner)),
        vec![1, 3, 4]
    );
    for role in [Role::Teacher, Role::AssessmentAgent, Role::System] {
        assert_eq!(ids(&visible_to(&log, user(9), role)), vec![1, 2, 3, 4, 5]);
    }
}

#[test]
fn a_live_answer_is_its_learners_and_the_round_is_everyones() {
    let round = EventId(Uuid::from_u128(1));
    let answer = |id, n| {
        event(
            id,
            12,
            learner(n),
            EventBody::ChallengeAnswered {
                challenge: round,
                learner: user(n),
                question: 0,
                response: zidara_kernel::activity::Response::Choice { index: 0 },
                correct: true,
            },
        )
    };
    let log = vec![
        event(
            1,
            10,
            teacher(),
            EventBody::ChallengeStarted {
                activity: ActivityId::new("quiz-f1"),
            },
        ),
        answer(2, 1),
        answer(3, 2),
        event(
            4,
            13,
            teacher(),
            EventBody::ChallengeAdvanced {
                challenge: round,
                question: 1,
            },
        ),
        event(
            5,
            14,
            teacher(),
            EventBody::ChallengeEnded { challenge: round },
        ),
    ];
    assert_eq!(
        ids(&visible_to(&log, user(1), Role::Learner)),
        vec![1, 2, 4, 5]
    );
    assert_eq!(
        ids(&visible_to(&log, user(2), Role::Learner)),
        vec![1, 3, 4, 5]
    );
    assert_eq!(
        ids(&visible_to(&log, user(9), Role::Teacher)),
        vec![1, 2, 3, 4, 5]
    );
}
