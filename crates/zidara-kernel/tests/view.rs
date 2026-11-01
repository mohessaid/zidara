mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use support::*;
use zidara_kernel::decide::{Command, decide};
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::framework::{
    Availability, Framework, FrameworkRegistry, Judgement, LockReason, OutcomeState,
    ProgressContext,
};
use zidara_kernel::governance::Assessment;
use zidara_kernel::ids::*;
use zidara_kernel::rules::{NotifyReason, Rule};
use zidara_kernel::score::Score;
use zidara_kernel::view::{learner_view, studio_view};

fn run(
    log: &mut Vec<Event>,
    ids: &mut SequentialIds,
    actor: &zidara_kernel::roles::Actor,
    command: Command,
) {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let env = env(&lesson, &registry, &framework, 10);
    let events = decide(actor, command, log, &env, ids).unwrap();
    log.extend(events);
}

#[test]
fn a_new_learner_sees_every_outcome_in_order_in_their_language() {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let view = learner_view(user(1), "ar", &[], &env(&lesson, &registry, &framework, 10)).unwrap();
    assert_eq!(view.framework, FrameworkId::new("test"));
    let titles: Vec<&str> = view.outcomes.iter().map(|o| o.title.as_str()).collect();
    assert_eq!(
        titles,
        [
            "أتعرّف على الكسر كأجزاء من كلّ",
            "أجد كسورًا متكافئة",
            "أقارن بين الكسور"
        ]
    );
    assert!(
        view.outcomes
            .iter()
            .all(|o| o.judgement == Judgement::NOT_STARTED && o.lock.is_none())
    );
    assert_eq!(view.next, Some(OutcomeId::new("F1")));
    assert!(view.assigned.is_empty());
}

#[test]
fn progress_moves_next_and_shows_assignments() {
    let (mut log, mut ids) = (vec![], SequentialIds::default());
    let quiz = answer_due(&log, 1, "quiz-f2", 0);
    run(&mut log, &mut ids, &learner(1), quiz);
    let quiz = answer_due(&log, 1, "quiz-f2", 0);
    run(&mut log, &mut ids, &learner(1), quiz);
    let quiz = answer_due(&log, 2, "quiz-f2", 3);
    run(&mut log, &mut ids, &learner(2), quiz);
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let env = env(&lesson, &registry, &framework, 10);
    let one = learner_view(user(1), "en", &log, &env).unwrap();
    assert_eq!(one.outcomes[1].judgement.state, OutcomeState::InProgress);
    assert_eq!(one.outcomes[1].judgement.attempts, 2);
    assert_eq!(one.assigned, vec![ActivityId::new("remedial-f2")]);
    let two = learner_view(user(2), "en", &log, &env).unwrap();
    assert_eq!(two.outcomes[1].judgement.state, OutcomeState::Achieved);
    assert_eq!(two.next, Some(OutcomeId::new("F1")));
    assert!(two.assigned.is_empty());
}

/// Locks every outcome but the first, to test that views show locks and skip them for `next`.
#[derive(Debug)]
struct Gated;

impl Framework for Gated {
    fn id(&self) -> FrameworkId {
        FrameworkId::new("gated")
    }
    fn judge(&self, evidence: &[&zidara_kernel::evidence::Evidence]) -> Judgement {
        if evidence.is_empty() {
            Judgement::NOT_STARTED
        } else {
            Judgement {
                state: OutcomeState::Achieved,
                score: Some(Score::FULL),
                band: None,
                attempts: 1,
            }
        }
    }
    fn availability(&self, ctx: &ProgressContext<'_>) -> Vec<Availability> {
        ctx.graph
            .iter()
            .enumerate()
            .map(|(i, o)| Availability {
                outcome: o.id.clone(),
                lock: (i > 0).then_some(LockReason::OpensAt { at: Timestamp(99) }),
            })
            .collect()
    }
    fn rules(&self) -> &[Rule] {
        &[]
    }
}

#[derive(Debug)]
struct OnlyGated;
impl FrameworkRegistry for OnlyGated {
    fn get(&self, id: &FrameworkId) -> Option<&dyn Framework> {
        (id.as_str() == "gated").then_some(&Gated as &dyn Framework)
    }
}

#[test]
fn locked_outcomes_are_shown_and_skipped_for_next() {
    let (lesson, unused) = (lesson(), TestRegistry::standard());
    let framework = FrameworkId::new("gated");
    let env = zidara_kernel::decide::Env {
        frameworks: &OnlyGated,
        ..env(&lesson, &unused, &framework, 10)
    };
    let achieved_f1 = event(
        1,
        1,
        zidara_kernel::roles::Actor::System,
        EventBody::EvidenceRecorded {
            evidence: evidence(1, "F1", 10_000),
            proposal: None,
        },
    );
    let view = learner_view(user(1), "en", &[achieved_f1], &env).unwrap();
    assert_eq!(
        view.outcomes[1].lock,
        Some(LockReason::OpensAt { at: Timestamp(99) })
    );
    assert_eq!(view.next, None);
}

#[test]
fn views_need_a_known_framework() {
    let (lesson, registry, framework) = (
        lesson(),
        TestRegistry::standard(),
        FrameworkId::new("missing"),
    );
    let env = env(&lesson, &registry, &framework, 10);
    assert!(learner_view(user(1), "en", &[], &env).is_err());
    assert!(studio_view(&[], "en", &[], &env).is_err());
}

#[test]
fn the_studio_counts_who_is_where_and_lists_the_agent_queue() {
    let (mut log, mut ids) = (vec![], SequentialIds::default());
    let quiz = answer_due(&log, 1, "quiz-f2", 0);
    run(&mut log, &mut ids, &learner(1), quiz);
    let quiz = answer_due(&log, 2, "quiz-f2", 3);
    run(&mut log, &mut ids, &learner(2), quiz);
    run(
        &mut log,
        &mut ids,
        &learner(1),
        Command::SubmitAnswer {
            activity: ActivityId::new("explain-f2"),
            answer: "the same".into(),
        },
    );
    let answer = log.last().unwrap().id;
    let unsure = Assessment {
        score: Score::of(5_000),
        confidence: Score::of(6_000),
        rationale: "r".into(),
        met: None,
    };
    run(
        &mut log,
        &mut ids,
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: unsure,
        },
    );
    run(
        &mut log,
        &mut ids,
        &learner(2),
        Command::SubmitAnswer {
            activity: ActivityId::new("explain-f2"),
            answer: "multiply both the same".into(),
        },
    );
    let answer = log.last().unwrap().id;
    let sure = Assessment {
        score: Score::FULL,
        confidence: Score::of(9_000),
        rationale: "r".into(),
        met: None,
    };
    run(
        &mut log,
        &mut ids,
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: sure,
        },
    );

    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let studio = studio_view(
        &[user(1), user(2), user(3)],
        "fr",
        &log,
        &env(&lesson, &registry, &framework, 10),
    )
    .unwrap();
    let f2 = &studio.outcomes[1];
    assert_eq!(f2.title, "Trouver des fractions équivalentes");
    assert_eq!((f2.achieved, f2.needs_support, f2.of), (1, 0, 3));
    assert_eq!(studio.queue.len(), 1);
    assert_eq!(studio.queue[0].answer, "the same");
    assert_eq!(studio.queue[0].proposal.learner, user(1));
    assert_eq!(studio.agent_actions.len(), 1);
    assert_eq!(studio.agent_actions[0].agent, AgentId::new("assessor"));
    assert_eq!(studio.flags[0].reason, NotifyReason::BelowExpected);
    assert_eq!(studio.flags[0].learner, user(1));
}

#[test]
fn an_empty_roster_reports_zeros() {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let studio = studio_view(&[], "en", &[], &env(&lesson, &registry, &framework, 10)).unwrap();
    assert!(
        studio
            .outcomes
            .iter()
            .all(|o| (o.achieved, o.needs_support, o.of) == (0, 0, 0))
    );
    assert!(studio.queue.is_empty() && studio.flags.is_empty() && studio.agent_actions.is_empty());
}

#[test]
fn needs_support_is_counted() {
    #[derive(Debug)]
    struct Struggling;
    impl Framework for Struggling {
        fn id(&self) -> FrameworkId {
            FrameworkId::new("s")
        }
        fn judge(&self, _: &[&zidara_kernel::evidence::Evidence]) -> Judgement {
            Judgement {
                state: OutcomeState::NeedsSupport,
                score: None,
                band: None,
                attempts: 2,
            }
        }
        fn availability(&self, _: &ProgressContext<'_>) -> Vec<Availability> {
            vec![]
        }
        fn rules(&self) -> &[Rule] {
            &[]
        }
    }
    #[derive(Debug)]
    struct One;
    impl FrameworkRegistry for One {
        fn get(&self, _: &FrameworkId) -> Option<&dyn Framework> {
            Some(&Struggling)
        }
    }
    let (lesson, unused) = (lesson(), TestRegistry::standard());
    let framework = FrameworkId::new("s");
    let env = zidara_kernel::decide::Env {
        frameworks: &One,
        ..env(&lesson, &unused, &framework, 10)
    };
    let studio = studio_view(&[user(1)], "en", &[], &env).unwrap();
    assert_eq!(studio.outcomes[0].needs_support, 1);
    // A framework that reports no availability leaves outcomes open.
    assert!(
        learner_view(user(1), "en", &[], &env)
            .unwrap()
            .outcomes
            .iter()
            .all(|o| o.lock.is_none())
    );
}

#[test]
fn views_serialize_for_the_bridge() {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    let view = learner_view(user(1), "en", &[], &env(&lesson, &registry, &framework, 10)).unwrap();
    let json = serde_json::to_string(&view).unwrap();
    assert!(json.contains(r#""state":"not_started""#), "{json}");
}

fn studio(log: &[Event], roster: &[UserId]) -> zidara_kernel::view::StudioView {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    studio_view(roster, "en", log, &env(&lesson, &registry, &framework, 10)).unwrap()
}

fn learner_sees(log: &[Event], n: u128) -> Option<zidara_kernel::view::LiveQuestion> {
    let (lesson, registry, framework) =
        (lesson(), TestRegistry::standard(), FrameworkId::new("test"));
    learner_view(user(n), "en", log, &env(&lesson, &registry, &framework, 10))
        .unwrap()
        .challenge
}

#[test]
fn the_board_counts_the_live_answers_and_each_phone_knows_its_own() {
    use zidara_kernel::activity::Response;
    use zidara_kernel::view::{LiveQuestion, LiveTally};
    let (mut log, mut ids) = (vec![], SequentialIds::default());
    let roster = [user(1), user(2), user(3)];
    assert_eq!(studio(&log, &roster).challenge, None);
    assert_eq!(learner_sees(&log, 1), None);
    let start = Command::StartChallenge {
        activity: ActivityId::new("quiz-f1"),
    };
    run(&mut log, &mut ids, &teacher(), start);
    let tally = |answered, correct, picks: Vec<u32>, question| LiveTally {
        activity: ActivityId::new("quiz-f1"),
        question,
        questions: 2,
        answered,
        correct,
        picks,
    };
    assert_eq!(
        studio(&log, &roster).challenge,
        Some(tally(0, 0, vec![0, 0, 0], 0))
    );
    let pick = |index| Command::AnswerChallenge {
        question: 0,
        response: Response::Choice { index },
    };
    run(&mut log, &mut ids, &learner(1), pick(0));
    run(&mut log, &mut ids, &learner(2), pick(2));
    run(&mut log, &mut ids, &learner(3), pick(0));
    assert_eq!(
        studio(&log, &roster).challenge,
        Some(tally(3, 2, vec![2, 0, 1], 0))
    );
    let phone = |answered| {
        Some(LiveQuestion {
            activity: ActivityId::new("quiz-f1"),
            question: 0,
            answered,
        })
    };
    assert_eq!(learner_sees(&log, 1), phone(Some(true)));
    assert_eq!(learner_sees(&log, 2), phone(Some(false)));
    assert_eq!(learner_sees(&log, 4), phone(None));
    // The next question is a fraction: no options to count, and nobody has answered it yet.
    run(&mut log, &mut ids, &teacher(), Command::NextQuestion);
    assert_eq!(
        studio(&log, &roster).challenge,
        Some(tally(0, 0, vec![], 1))
    );
    let fraction = Command::AnswerChallenge {
        question: 1,
        response: Response::Fraction { num: 3, den: 8 },
    };
    run(&mut log, &mut ids, &learner(1), fraction);
    assert_eq!(
        studio(&log, &roster).challenge,
        Some(tally(1, 1, vec![], 1))
    );
    assert_eq!(
        learner_sees(&log, 1),
        Some(LiveQuestion {
            activity: ActivityId::new("quiz-f1"),
            question: 1,
            answered: Some(true)
        })
    );
    run(&mut log, &mut ids, &teacher(), Command::EndChallenge);
    assert_eq!(studio(&log, &roster).challenge, None);
    assert_eq!(learner_sees(&log, 1), None);
}
