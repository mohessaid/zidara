mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use proptest::prelude::*;
use support::*;
use zidara_kernel::event::EventBody;
use zidara_kernel::evidence::EvidenceKind;
use zidara_kernel::governance::{Assessment, Decision, Proposal};
use zidara_kernel::ids::*;
use zidara_kernel::roles::Actor;
use zidara_kernel::rules::NotifyReason;
use zidara_kernel::score::Score;
use zidara_kernel::state::{ProposalStatus, fold};

fn recorded(id: u128, at: i64, bp: u16) -> zidara_kernel::event::Event {
    event(
        id,
        at,
        Actor::System,
        EventBody::EvidenceRecorded {
            evidence: evidence(1, "F1", bp),
            proposal: None,
        },
    )
}

fn proposal(answer: u128, decision: Decision) -> Proposal {
    Proposal {
        answer: EventId(Uuid::from_u128(answer)),
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
        decision,
    }
}

fn eid(n: u128) -> EventId {
    EventId(Uuid::from_u128(n))
}

#[test]
fn an_empty_log_is_an_empty_state() {
    let state = fold(&[]);
    assert_eq!(state.framework, None);
    assert!(state.evidence.is_empty());
}

#[test]
fn evidence_is_ordered_by_time_not_arrival() {
    let state = fold(&[
        recorded(1, 30, 3_000),
        recorded(2, 10, 1_000),
        recorded(3, 20, 2_000),
    ]);
    let scores: Vec<u16> = state
        .evidence
        .iter()
        .map(|(_, _, e)| e.score.bp())
        .collect();
    assert_eq!(scores, [1_000, 2_000, 3_000]);
}

#[test]
fn events_at_the_same_time_keep_their_order() {
    let state = fold(&[recorded(9, 10, 1_000), recorded(1, 10, 2_000)]);
    let ids: Vec<EventId> = state.evidence.iter().map(|(id, _, _)| *id).collect();
    assert_eq!(ids, [eid(9), eid(1)]);
}

#[test]
fn undone_events_are_skipped() {
    let state = fold(&[
        recorded(1, 10, 1_000),
        recorded(2, 11, 2_000),
        event(3, 12, teacher(), EventBody::Undone { target: eid(1) }),
    ]);
    assert_eq!(state.evidence.len(), 1);
    assert_eq!(state.evidence[0].0, eid(2));
}

#[test]
fn evidence_is_found_per_learner_and_outcome() {
    let other = event(
        2,
        11,
        Actor::System,
        EventBody::EvidenceRecorded {
            evidence: evidence(2, "F1", 5_000),
            proposal: None,
        },
    );
    let elsewhere = event(
        3,
        12,
        Actor::System,
        EventBody::EvidenceRecorded {
            evidence: evidence(1, "F2", 5_000),
            proposal: None,
        },
    );
    let state = fold(&[recorded(1, 10, 1_000), other, elsewhere]);
    let found = state.evidence_for(user(1), &OutcomeId::new("F1"));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].score, Score::of(1_000));
}

#[test]
fn answers_and_proposals_move_through_their_states() {
    let state = fold(&[
        event(
            1,
            10,
            learner(1),
            EventBody::AnswerSubmitted {
                learner: user(1),
                activity: ActivityId::new("explain-f2"),
                answer: "same".into(),
            },
        ),
        event(
            2,
            11,
            agent(),
            EventBody::ProposalMade {
                proposal: proposal(1, Decision::Queue),
            },
        ),
        event(
            3,
            12,
            agent(),
            EventBody::ProposalMade {
                proposal: proposal(7, Decision::Apply),
            },
        ),
        event(
            4,
            13,
            agent(),
            EventBody::ProposalMade {
                proposal: proposal(8, Decision::Queue),
            },
        ),
        event(
            5,
            14,
            teacher(),
            EventBody::ProposalDecided {
                proposal: eid(2),
                accept: true,
            },
        ),
        event(
            6,
            15,
            teacher(),
            EventBody::ProposalDecided {
                proposal: eid(4),
                accept: false,
            },
        ),
        event(
            7,
            16,
            teacher(),
            EventBody::ProposalDecided {
                proposal: eid(99),
                accept: true,
            },
        ),
    ]);
    assert_eq!(state.answers[&eid(1)].text, "same");
    assert_eq!(state.proposals[&eid(2)].1, ProposalStatus::Accepted);
    assert_eq!(state.proposals[&eid(3)].1, ProposalStatus::Applied);
    assert_eq!(state.proposals[&eid(4)].1, ProposalStatus::Rejected);
    assert!(state.has_proposal_for(eid(1)));
    assert!(!state.has_proposal_for(eid(5)));
}

#[test]
fn assignments_notifications_and_framework_are_kept() {
    let state = fold(&[
        event(
            1,
            10,
            Actor::System,
            EventBody::ActivityAssigned {
                learner: user(1),
                outcome: OutcomeId::new("F2"),
                activity: ActivityId::new("remedial-f2"),
            },
        ),
        event(
            2,
            11,
            Actor::System,
            EventBody::TeacherNotified {
                learner: user(1),
                outcome: OutcomeId::new("F2"),
                reason: NotifyReason::Struggling,
            },
        ),
        event(
            3,
            12,
            teacher(),
            EventBody::FrameworkChanged {
                framework: FrameworkId::new("mastery"),
            },
        ),
        event(
            4,
            13,
            teacher(),
            EventBody::FrameworkChanged {
                framework: FrameworkId::new("cambridge-style"),
            },
        ),
        event(
            5,
            14,
            learner(1),
            EventBody::QuizSubmitted {
                learner: user(1),
                activity: ActivityId::new("quiz-f1"),
                variant: 0,
                responses: vec![],
            },
        ),
    ]);
    assert!(state.is_assigned(
        user(1),
        &OutcomeId::new("F2"),
        &ActivityId::new("remedial-f2")
    ));
    assert!(!state.is_assigned(
        user(2),
        &OutcomeId::new("F2"),
        &ActivityId::new("remedial-f2")
    ));
    assert!(!state.is_assigned(
        user(1),
        &OutcomeId::new("F1"),
        &ActivityId::new("remedial-f2")
    ));
    assert!(!state.is_assigned(user(1), &OutcomeId::new("F2"), &ActivityId::new("other")));
    assert!(state.notifications.contains(&(
        user(1),
        OutcomeId::new("F2"),
        NotifyReason::Struggling
    )));
    assert_eq!(state.framework, Some(FrameworkId::new("cambridge-style")));
}

#[test]
fn events_round_trip_through_json() {
    let original = recorded(1, 10, 1_000);
    let json = serde_json::to_string(&original).unwrap();
    assert!(json.contains(r#""type":"evidence_recorded""#), "{json}");
    assert_eq!(
        serde_json::from_str::<zidara_kernel::event::Event>(&json).unwrap(),
        original
    );
}

/// Late-synced work lands where it happened: arrival order never changes the state.
#[test]
fn arrival_order_does_not_matter_when_times_differ() {
    let log = proptest::collection::btree_set(0i64..1_000, 1..12).prop_flat_map(|times| {
        let events: Vec<_> = times
            .into_iter()
            .enumerate()
            .map(|(i, at)| recorded(i as u128 + 1, at, (i as u16 * 997) % 10_001))
            .collect();
        (Just(events.clone()), Just(events).prop_shuffle())
    });
    support::check(log, |(ordered, shuffled)| {
        prop_assert_eq!(fold(&ordered), fold(&shuffled));
        Ok(())
    });
}

/// Folding borrowed events (two slices, no copies) gives exactly what folding owned ones does.
#[test]
fn folding_references_matches_folding_values() {
    use zidara_kernel::state::fold_refs;
    let log = proptest::collection::btree_set(0i64..1_000, 1..12).prop_flat_map(|times| {
        let events: Vec<_> = times
            .into_iter()
            .enumerate()
            .map(|(i, at)| recorded(i as u128 + 1, at, (i as u16 * 997) % 10_001))
            .collect();
        (Just(events.clone()), 0..=events.len())
    });
    support::check(log, |(events, split)| {
        let (history, new) = events.split_at(split);
        prop_assert_eq!(fold_refs(history.iter().chain(new)), fold(&events));
        Ok(())
    });
}

#[test]
fn undoing_a_mark_touches_only_the_proposal_it_names() {
    let marked = |id, proposal| {
        event(
            id,
            20,
            teacher(),
            EventBody::EvidenceRecorded {
                evidence: evidence(1, "F2", 10_000),
                proposal,
            },
        )
    };
    let undo = |id, target| {
        event(
            id,
            30,
            teacher(),
            EventBody::Undone {
                target: eid(target),
            },
        )
    };
    let state = fold(&[
        event(
            1,
            10,
            agent(),
            EventBody::ProposalMade {
                proposal: proposal(7, Decision::Apply),
            },
        ),
        marked(2, None),
        marked(3, Some(eid(99))),
        undo(4, 2),
        undo(5, 3),
    ]);
    assert_eq!(state.proposals[&eid(1)].1, ProposalStatus::Applied);
    assert!(!state.proposals.contains_key(&eid(99)));
    assert!(state.evidence.is_empty());
}
