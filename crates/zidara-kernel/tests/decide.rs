mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use support::*;
use zidara_kernel::activity::{ActivityKind, Response};
use zidara_kernel::decide::{Command, Envelope, KernelError, decide};
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::evidence::EvidenceKind;
use zidara_kernel::governance::{Assessment, Decision};
use zidara_kernel::ids::*;
use zidara_kernel::lesson::Lesson;
use zidara_kernel::roles::{Actor, Permission, Role};
use zidara_kernel::rules::NotifyReason;
use zidara_kernel::score::Score;
use zidara_kernel::state::{ProposalStatus, fold};

/// A cohort under test: a log, an id counter and the fixtures.
struct Class {
    lesson: Lesson,
    registry: TestRegistry,
    framework: FrameworkId,
    log: Vec<Event>,
    ids: SequentialIds,
    now: i64,
}

impl Class {
    fn new() -> Self {
        Self {
            lesson: lesson(),
            registry: TestRegistry::standard(),
            framework: FrameworkId::new("test"),
            log: vec![],
            ids: SequentialIds::default(),
            now: 1_000,
        }
    }

    fn run(&mut self, actor: &Actor, command: Command) -> Result<Vec<Event>, KernelError> {
        self.now += 1;
        let env = env(&self.lesson, &self.registry, &self.framework, self.now);
        let events = decide(actor, command, &self.log, &env, &mut self.ids)?;
        self.log.extend(events.clone());
        Ok(events)
    }

    fn ok(&mut self, actor: &Actor, command: Command) -> Vec<Event> {
        self.run(actor, command).unwrap()
    }

    /// The version of `activity` due for learner 1.
    fn due(&self, activity: &str) -> u32 {
        zidara_kernel::decide::next_variant(
            &self.log,
            user(1),
            &ActivityId::new(activity),
            &self.lesson,
        )
    }

    /// Learner 1 answers the due version of quiz-f2, `right` of its three questions correctly.
    fn quiz_f2(&mut self, right: usize) -> Vec<Event> {
        let quiz = answer_due(&self.log, 1, "quiz-f2", right);
        self.ok(&learner(1), quiz)
    }

    fn answer(&mut self, text: &str) -> EventId {
        let events = self.ok(
            &learner(1),
            Command::SubmitAnswer {
                activity: ActivityId::new("explain-f2"),
                answer: text.into(),
            },
        );
        events[0].id
    }
}

fn frac(num: u32, den: u32) -> Response {
    Response::Fraction { num, den }
}

fn bodies(events: &[Event]) -> Vec<&EventBody> {
    events.iter().map(|e| &e.body).collect()
}

fn assessment(score: u16, confidence: u16) -> Assessment {
    Assessment {
        score: Score::of(score),
        confidence: Score::of(confidence),
        rationale: "r".into(),
        met: None,
    }
}

fn f2() -> OutcomeId {
    OutcomeId::new("F2")
}

#[test]
fn a_quiz_records_evidence_per_outcome_as_the_system() {
    let mut class = Class::new();
    let events = class.quiz_f2(3);
    assert!(matches!(events[0].body, EventBody::QuizSubmitted { .. }));
    assert_eq!(events[0].actor, learner(1));
    let EventBody::EvidenceRecorded { evidence, .. } = &events[1].body else {
        panic!("{events:?}")
    };
    assert_eq!(events[1].actor, Actor::System);
    assert_eq!(
        (evidence.outcome.clone(), evidence.score, evidence.kind),
        (f2(), Score::FULL, EvidenceKind::Formative)
    );
    assert_eq!(events.len(), 2);
    assert!(
        events
            .iter()
            .all(|e| e.at == Timestamp(1_001) && e.cohort == CohortId(Uuid::from_u128(2)))
    );
}

#[test]
fn two_weak_tries_bring_remediation_and_a_notice_once() {
    let mut class = Class::new();
    // The first weak try already flags the band; remediation waits for the second.
    let first = class.quiz_f2(1);
    assert_eq!(first.len(), 3, "{first:?}");
    assert!(
        !first
            .iter()
            .any(|e| matches!(e.body, EventBody::ActivityAssigned { .. }))
    );
    let second = class.quiz_f2(1);
    assert!(bodies(&second).contains(&&EventBody::ActivityAssigned {
        learner: user(1),
        outcome: f2(),
        activity: ActivityId::new("remedial-f2"),
    }));
    assert!(bodies(&second).contains(&&EventBody::TeacherNotified {
        learner: user(1),
        outcome: f2(),
        reason: NotifyReason::Struggling,
    }));
    // A third weak try adds nothing new: help and notices are given once.
    let third = class.quiz_f2(0);
    assert_eq!(third.len(), 2, "{third:?}");
}

#[test]
fn a_double_tap_is_two_attempts() {
    let mut class = Class::new();
    class.quiz_f2(3);
    class.quiz_f2(3);
    assert_eq!(fold(&class.log).evidence_for(user(1), &f2()).len(), 2);
}

#[test]
fn no_remediation_activity_means_only_a_notice() {
    let mut class = Class::new();
    let wrong = || vec![Response::Choice { index: 2 }, frac(0, 1)];
    class.ok(
        &learner(1),
        Command::SubmitQuiz {
            activity: ActivityId::new("quiz-f1"),
            variant: 0,
            responses: wrong(),
        },
    );
    let second = class.ok(
        &learner(1),
        Command::SubmitQuiz {
            activity: ActivityId::new("quiz-f1"),
            variant: class.due("quiz-f1"),
            responses: wrong(),
        },
    );
    assert!(
        !second
            .iter()
            .any(|e| matches!(e.body, EventBody::ActivityAssigned { .. }))
    );
    assert!(second.iter().any(|e| matches!(
        e.body,
        EventBody::TeacherNotified {
            reason: NotifyReason::Struggling,
            ..
        }
    )));
}

#[test]
fn band_notices_are_given_once() {
    let mut class = Class::new();
    let first = class.quiz_f2(0);
    assert!(first.iter().any(|e| matches!(
        e.body,
        EventBody::TeacherNotified {
            reason: NotifyReason::BelowExpected,
            ..
        }
    )));
    let second = class.quiz_f2(0);
    assert!(!second.iter().any(|e| matches!(
        e.body,
        EventBody::TeacherNotified {
            reason: NotifyReason::BelowExpected,
            ..
        }
    )));
}

#[test]
fn quizzes_check_the_activity_and_responses() {
    let mut class = Class::new();
    let quiz = |activity: &str, responses| Command::SubmitQuiz {
        activity: ActivityId::new(activity),
        variant: 0,
        responses,
    };
    assert_eq!(
        class.run(&learner(1), quiz("nope", vec![])),
        Err(KernelError::UnknownActivity {
            activity: ActivityId::new("nope")
        })
    );
    assert_eq!(
        class.run(&learner(1), quiz("explain-f2", vec![])),
        Err(KernelError::WrongActivityKind {
            activity: ActivityId::new("explain-f2")
        })
    );
    assert_eq!(
        class.run(&learner(1), quiz("quiz-f2", vec![])),
        Err(KernelError::Responses {
            expected: 3,
            got: 0
        })
    );
}

#[test]
fn only_learners_submit_work_and_only_as_people() {
    let mut class = Class::new();
    let command = || Command::SubmitAnswer {
        activity: ActivityId::new("explain-f2"),
        answer: "x".into(),
    };
    assert_eq!(
        class.run(&teacher(), command()),
        Err(KernelError::Forbidden {
            role: Role::Teacher,
            permission: Permission::SubmitWork
        })
    );
    let robot = Actor::Agent {
        agent: AgentId::new("bot"),
        role: Role::Learner,
    };
    assert_eq!(class.run(&robot, command()), Err(KernelError::NotAPerson));
    assert_eq!(
        class.run(
            &learner(1),
            Command::SubmitAnswer {
                activity: ActivityId::new("quiz-f1"),
                answer: "x".into()
            }
        ),
        Err(KernelError::WrongActivityKind {
            activity: ActivityId::new("quiz-f1")
        })
    );
    assert_eq!(
        class.run(
            &learner(1),
            Command::SubmitAnswer {
                activity: ActivityId::new("nope"),
                answer: "x".into()
            }
        ),
        Err(KernelError::UnknownActivity {
            activity: ActivityId::new("nope")
        })
    );
}

#[test]
fn teachers_record_evidence_and_learners_cannot() {
    let mut class = Class::new();
    let record = |kind, outcome: &str, activity: &str| Command::RecordEvidence {
        learner: user(1),
        outcome: OutcomeId::new(outcome),
        activity: ActivityId::new(activity),
        score: Score::of(4_000),
        kind,
    };
    let events = class.ok(
        &teacher(),
        record(EvidenceKind::Summative, "F3", "checkpoint"),
    );
    assert_eq!(events[0].actor, teacher());
    assert_eq!(
        class.run(
            &learner(1),
            record(EvidenceKind::Formative, "F3", "checkpoint")
        ),
        Err(KernelError::Forbidden {
            role: Role::Learner,
            permission: Permission::RecordFormative
        })
    );
    assert_eq!(
        class.run(
            &agent(),
            record(EvidenceKind::Summative, "F3", "checkpoint")
        ),
        Err(KernelError::Forbidden {
            role: Role::AssessmentAgent,
            permission: Permission::RecordSummative
        })
    );
    assert_eq!(
        class.run(
            &teacher(),
            record(EvidenceKind::Formative, "F9", "checkpoint")
        ),
        Err(KernelError::UnknownOutcome {
            outcome: OutcomeId::new("F9")
        })
    );
    assert_eq!(
        class.run(&teacher(), record(EvidenceKind::Formative, "F3", "nope")),
        Err(KernelError::UnknownActivity {
            activity: ActivityId::new("nope")
        })
    );
}

#[test]
fn a_confident_formative_mark_is_applied_by_the_agent() {
    let mut class = Class::new();
    let answer = class.answer("multiply both by the same");
    let events = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: assessment(10_000, 9_000),
        },
    );
    let EventBody::ProposalMade { proposal } = &events[0].body else {
        panic!()
    };
    assert_eq!(
        (
            proposal.decision,
            proposal.learner,
            proposal.outcome.clone()
        ),
        (Decision::Apply, user(1), f2())
    );
    assert_eq!(events[1].actor, agent());
    assert!(
        matches!(&events[1].body, EventBody::EvidenceRecorded { evidence, .. } if evidence.score == Score::FULL)
    );
}

#[test]
fn an_unsure_mark_waits_for_the_teacher_who_accepts_it() {
    let mut class = Class::new();
    let answer = class.answer("it is the same");
    let made = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: assessment(5_000, 6_000),
        },
    );
    assert_eq!(made.len(), 1);
    let proposal = made[0].id;
    assert_eq!(
        fold(&class.log).proposals[&proposal].1,
        ProposalStatus::Queued
    );
    let decided = class.ok(
        &teacher(),
        Command::DecideProposal {
            proposal,
            accept: true,
        },
    );
    assert_eq!(decided[1].actor, teacher());
    assert!(
        matches!(&decided[1].body, EventBody::EvidenceRecorded { evidence, .. } if evidence.score == Score::of(5_000))
    );
    assert_eq!(
        fold(&class.log).proposals[&proposal].1,
        ProposalStatus::Accepted
    );
    // A decided proposal cannot be decided again.
    assert_eq!(
        class.run(
            &teacher(),
            Command::DecideProposal {
                proposal,
                accept: false
            }
        ),
        Err(KernelError::NotOpen { proposal })
    );
}

#[test]
fn a_rejected_proposal_records_nothing() {
    let mut class = Class::new();
    let answer = class.answer("it is the same");
    let proposal = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: assessment(5_000, 6_000),
        },
    )[0]
    .id;
    let decided = class.ok(
        &teacher(),
        Command::DecideProposal {
            proposal,
            accept: false,
        },
    );
    assert_eq!(decided.len(), 1);
    assert!(fold(&class.log).evidence_for(user(1), &f2()).is_empty());
}

#[test]
fn proposals_need_a_real_unmarked_answer_and_an_agent() {
    let mut class = Class::new();
    let ghost = EventId(Uuid::from_u128(777));
    let propose = |answer| Command::ProposeAssessment {
        answer,
        assessment: assessment(10_000, 9_000),
    };
    assert_eq!(
        class.run(&agent(), propose(ghost)),
        Err(KernelError::UnknownAnswer { answer: ghost })
    );
    let answer = class.answer("multiply both by the same");
    assert_eq!(
        class.run(&teacher(), propose(answer)),
        Err(KernelError::Forbidden {
            role: Role::Teacher,
            permission: Permission::ProposeAssessment
        })
    );
    class.ok(&agent(), propose(answer));
    assert_eq!(
        class.run(&agent(), propose(answer)),
        Err(KernelError::AlreadyProposed { answer })
    );
    assert_eq!(
        class.run(
            &agent(),
            Command::DecideProposal {
                proposal: answer,
                accept: true
            }
        ),
        Err(KernelError::Forbidden {
            role: Role::AssessmentAgent,
            permission: Permission::DecideProposal
        })
    );
}

#[test]
fn proposals_for_answers_the_lesson_no_longer_knows_are_refused() {
    let mut class = Class::new();
    let submitted = |activity: &str| {
        event(
            500,
            1,
            learner(1),
            EventBody::AnswerSubmitted {
                learner: user(1),
                activity: ActivityId::new(activity),
                answer: "x".into(),
            },
        )
    };
    let answer = EventId(Uuid::from_u128(500));
    class.log = vec![submitted("gone")];
    assert_eq!(
        class.run(
            &agent(),
            Command::ProposeAssessment {
                answer,
                assessment: assessment(1, 1)
            }
        ),
        Err(KernelError::UnknownActivity {
            activity: ActivityId::new("gone")
        })
    );
    class.log = vec![submitted("quiz-f1")];
    assert_eq!(
        class.run(
            &agent(),
            Command::ProposeAssessment {
                answer,
                assessment: assessment(1, 1)
            }
        ),
        Err(KernelError::WrongActivityKind {
            activity: ActivityId::new("quiz-f1")
        })
    );
}

#[test]
fn teachers_undo_evidence_and_assignments_only() {
    let mut class = Class::new();
    let answer = class.answer("multiply both by the same");
    let applied = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: assessment(10_000, 9_000),
        },
    );
    let evidence = applied[1].id;
    assert_eq!(
        class.run(&agent(), Command::Undo { target: evidence }),
        Err(KernelError::Forbidden {
            role: Role::AssessmentAgent,
            permission: Permission::Undo
        })
    );
    class.ok(&teacher(), Command::Undo { target: evidence });
    assert!(fold(&class.log).evidence_for(user(1), &f2()).is_empty());
    assert_eq!(
        class.run(&teacher(), Command::Undo { target: evidence }),
        Err(KernelError::NotUndoable { target: evidence })
    );
    assert_eq!(
        class.run(&teacher(), Command::Undo { target: answer }),
        Err(KernelError::NotUndoable { target: answer })
    );

    class.quiz_f2(0);
    let assigned = class
        .quiz_f2(0)
        .into_iter()
        .find(|e| matches!(e.body, EventBody::ActivityAssigned { .. }))
        .unwrap();
    class.ok(
        &teacher(),
        Command::Undo {
            target: assigned.id,
        },
    );
    assert!(fold(&class.log).assignments.is_empty());
}

#[test]
fn teachers_switch_frameworks_and_unknown_ones_are_refused() {
    let mut class = Class::new();
    let switch = |id: &str| Command::ChangeFramework {
        framework: FrameworkId::new(id),
    };
    assert_eq!(
        class.run(&learner(1), switch("quiet")),
        Err(KernelError::Forbidden {
            role: Role::Learner,
            permission: Permission::ChangeFramework
        })
    );
    assert_eq!(
        class.run(&teacher(), switch("nope")),
        Err(KernelError::UnknownFramework {
            framework: FrameworkId::new("nope")
        })
    );
    class.ok(&teacher(), switch("quiet"));
    // The quiet framework has no rules, so weak tries bring nothing.
    class.quiz_f2(0);
    assert_eq!(class.quiz_f2(0).len(), 2);
}

#[test]
fn a_missing_default_framework_is_an_error_not_a_panic() {
    let mut class = Class::new();
    class.framework = FrameworkId::new("missing");
    assert_eq!(
        class.run(
            &learner(1),
            Command::SubmitQuiz {
                activity: ActivityId::new("quiz-f2"),
                variant: 0,
                responses: vec![frac(0, 1), frac(0, 1), frac(0, 1)],
            }
        ),
        Err(KernelError::UnknownFramework {
            framework: FrameworkId::new("missing")
        })
    );
}

#[test]
fn commands_and_errors_travel_as_json() {
    let envelope = Envelope {
        id: Uuid::from_u128(5),
        at: Timestamp(9),
        command: Command::Undo {
            target: EventId(Uuid::from_u128(1)),
        },
    };
    let json = serde_json::to_string(&envelope).unwrap();
    assert!(json.contains(r#""type":"undo""#), "{json}");
    assert_eq!(serde_json::from_str::<Envelope>(&json).unwrap(), envelope);
    let error = serde_json::to_string(&KernelError::NotAPerson).unwrap();
    assert_eq!(error, r#"{"type":"not_a_person"}"#);
    assert_eq!(
        KernelError::NotAPerson.to_string(),
        "only a person can submit work"
    );
}

fn quiz_f1(variant: u32, responses: Vec<Response>) -> Command {
    Command::SubmitQuiz {
        activity: ActivityId::new("quiz-f1"),
        variant,
        responses,
    }
}

#[test]
fn quiz_versions_rotate_and_the_kernel_keeps_the_turn() {
    use zidara_kernel::decide::next_variant;
    let mut class = Class::new();
    let f1 = ActivityId::new("quiz-f1");
    let turn =
        |class: &Class, learner: u128| next_variant(&class.log, user(learner), &f1, &class.lesson);
    assert_eq!(turn(&class, 1), 0);
    class.ok(
        &learner(1),
        quiz_f1(0, vec![Response::Choice { index: 0 }, frac(3, 8)]),
    );
    assert_eq!(turn(&class, 1), 1);
    // The memorised version is refused: the next turn is version 1.
    assert_eq!(
        class.run(
            &learner(1),
            quiz_f1(0, vec![Response::Choice { index: 0 }, frac(3, 8)])
        ),
        Err(KernelError::WrongVariant { expected: 1 })
    );
    class.ok(
        &learner(1),
        quiz_f1(1, vec![Response::Choice { index: 1 }, frac(2, 5)]),
    );
    class.ok(
        &learner(1),
        quiz_f1(2, vec![Response::Choice { index: 2 }, frac(4, 7)]),
    );
    // Three versions, so the fourth try starts again at version 0. Each learner has their own turn.
    assert_eq!(turn(&class, 1), 0);
    assert_eq!(turn(&class, 2), 0);
    assert_eq!(
        class.run(
            &learner(1),
            quiz_f1(3, vec![Response::Choice { index: 0 }, frac(3, 8)])
        ),
        Err(KernelError::WrongVariant { expected: 0 })
    );
}

#[test]
fn answers_are_marked_against_the_version_asked() {
    let mut class = Class::new();
    class.ok(
        &learner(1),
        quiz_f1(0, vec![Response::Choice { index: 0 }, frac(3, 8)]),
    );
    // Version 1's right answers; version 0's answers would score nothing here.
    let events = class.ok(
        &learner(1),
        quiz_f1(1, vec![Response::Choice { index: 1 }, frac(2, 5)]),
    );
    let EventBody::QuizSubmitted { variant, .. } = &events[0].body else {
        panic!()
    };
    assert_eq!(*variant, 1);
    assert!(
        matches!(&events[1].body, EventBody::EvidenceRecorded { evidence, .. } if evidence.score == Score::FULL)
    );
    class.ok(
        &learner(1),
        quiz_f1(2, vec![Response::Choice { index: 0 }, frac(3, 8)]),
    );
    let state = fold(&class.log);
    let scores: Vec<u16> = state
        .evidence_for(user(1), &OutcomeId::new("F1"))
        .iter()
        .map(|e| e.score.bp())
        .collect();
    assert_eq!(scores, [10_000, 10_000, 0]);
}

#[test]
fn quizzes_without_versions_always_use_the_first() {
    use zidara_kernel::decide::next_variant;
    let mut class = Class::new();
    let checkpoint = ActivityId::new("checkpoint");
    let full = || vec![frac(2, 5), frac(4, 10), Response::Choice { index: 1 }];
    class.ok(
        &learner(1),
        Command::SubmitQuiz {
            activity: checkpoint.clone(),
            variant: 0,
            responses: full(),
        },
    );
    assert_eq!(
        next_variant(&class.log, user(1), &checkpoint, &class.lesson),
        0
    );
    assert_eq!(
        next_variant(
            &class.log,
            user(1),
            &ActivityId::new("reading-halves"),
            &class.lesson
        ),
        0
    );
    assert_eq!(
        next_variant(&class.log, user(1), &ActivityId::new("nope"), &class.lesson),
        0
    );
}

#[test]
fn old_commands_without_a_version_mean_the_first() {
    let json = r#"{"type":"submit_quiz","activity":"quiz-f1","responses":[]}"#;
    let command: Command = serde_json::from_str(json).unwrap();
    assert_eq!(
        command,
        Command::SubmitQuiz {
            activity: ActivityId::new("quiz-f1"),
            variant: 0,
            responses: vec![]
        }
    );
}

/// The teacher has the last word: remediation she undid is not assigned again by the rules.
#[test]
fn remediation_the_teacher_undid_stays_undone() {
    let mut class = Class::new();
    class.quiz_f2(0);
    let assigned = class
        .quiz_f2(0)
        .into_iter()
        .find(|e| matches!(e.body, EventBody::ActivityAssigned { .. }))
        .expect("two weak tries bring remediation");
    class.ok(
        &teacher(),
        Command::Undo {
            target: assigned.id,
        },
    );
    let third = class.quiz_f2(0);
    assert!(
        !third
            .iter()
            .any(|e| matches!(e.body, EventBody::ActivityAssigned { .. })),
        "{third:?}"
    );
    assert!(fold(&class.log).assignments.is_empty());
}

#[test]
fn an_undone_mark_leaves_its_proposal_undone() {
    let mut class = Class::new();
    let answer = class.answer("multiply both by the same");
    let applied = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer,
            assessment: assessment(10_000, 9_000),
        },
    );
    let (proposal, evidence) = (applied[0].id, applied[1].id);
    assert!(matches!(
        applied[1].body,
        EventBody::EvidenceRecorded { proposal: Some(p), .. } if p == proposal
    ));
    class.ok(&teacher(), Command::Undo { target: evidence });
    assert_eq!(
        fold(&class.log).proposals[&proposal].1,
        ProposalStatus::Undone
    );

    // A mark the teacher accepted, then undid, reads undone too.
    let second = class.answer("you multiply them");
    let queued = class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer: second,
            assessment: assessment(10_000, 1_000),
        },
    )[0]
    .id;
    let accepted = class.ok(
        &teacher(),
        Command::DecideProposal {
            proposal: queued,
            accept: true,
        },
    );
    let recorded = accepted
        .iter()
        .find(|e| matches!(e.body, EventBody::EvidenceRecorded { .. }))
        .unwrap()
        .id;
    class.ok(&teacher(), Command::Undo { target: recorded });
    assert_eq!(
        fold(&class.log).proposals[&queued].1,
        ProposalStatus::Undone
    );
}

#[test]
fn the_rubric_marks_each_open_answer_that_has_no_proposal_yet() {
    let mut class = Class::new();
    let first = class.answer("multiply both by the same number");
    let second = class.answer("I add one");
    // Answers to other kinds of activity, and answers already proposed, are not marked again.
    class.quiz_f2(3);
    class.ok(
        &agent(),
        Command::ProposeAssessment {
            answer: first,
            assessment: assessment(10_000, 9_000),
        },
    );
    let marks = zidara_kernel::governance::rubric_marks(&class.log, &class.lesson);
    assert_eq!(marks.len(), 1);
    assert_eq!(marks[0].0, second);
    let ActivityKind::OpenResponse { rubric, .. } = &class
        .lesson
        .activity(&ActivityId::new("explain-f2"))
        .unwrap()
        .kind
    else {
        panic!("explain-f2 is an open response");
    };
    assert_eq!(
        marks[0].1,
        zidara_kernel::governance::assess(rubric, "I add one")
    );
    // Answers to an activity the lesson does not have, or to one that is not an open
    // response, are skipped, not marked.
    let stray = |id, activity: &str| {
        event(
            id,
            5_000,
            learner(1),
            EventBody::AnswerSubmitted {
                learner: user(1),
                activity: ActivityId::new(activity),
                answer: "anything".into(),
            },
        )
    };
    let log = vec![stray(900, "gone"), stray(901, "quiz-f1")];
    assert!(zidara_kernel::governance::rubric_marks(&log, &class.lesson).is_empty());
}

fn start(class: &mut Class, quiz: &str) -> EventId {
    let events = class.ok(
        &teacher(),
        Command::StartChallenge {
            activity: ActivityId::new(quiz),
        },
    );
    assert_eq!(
        bodies(&events),
        [&EventBody::ChallengeStarted {
            activity: ActivityId::new(quiz)
        }]
    );
    assert_eq!(events[0].actor, teacher());
    events[0].id
}

fn live(question: u32, response: Response) -> Command {
    Command::AnswerChallenge { question, response }
}

fn choice(index: u32) -> Response {
    Response::Choice { index }
}

fn scores(events: &[Event]) -> Vec<(UserId, OutcomeId, Score, EvidenceKind)> {
    events
        .iter()
        .filter_map(|e| match &e.body {
            EventBody::EvidenceRecorded { evidence, .. } => {
                assert_eq!(e.actor, Actor::System);
                Some((
                    evidence.learner,
                    evidence.outcome.clone(),
                    evidence.score,
                    evidence.kind,
                ))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_live_round_goes_question_by_question_and_ends_in_practice_evidence() {
    let mut class = Class::new();
    let round = start(&mut class, "quiz-f1");
    // quiz-f1: a choice (the first option is right), then 3/8 exactly.
    let right = class.ok(&learner(1), live(0, choice(0)));
    assert_eq!(
        bodies(&right),
        [&EventBody::ChallengeAnswered {
            challenge: round,
            learner: user(1),
            question: 0,
            response: choice(0),
            correct: true,
        }]
    );
    assert_eq!(right[0].actor, learner(1));
    let wrong = class.ok(&learner(2), live(0, choice(1)));
    assert!(matches!(
        wrong[0].body,
        EventBody::ChallengeAnswered { correct: false, .. }
    ));
    let next = class.ok(&teacher(), Command::NextQuestion);
    assert_eq!(
        bodies(&next),
        [&EventBody::ChallengeAdvanced {
            challenge: round,
            question: 1
        }]
    );
    class.ok(&learner(1), live(1, frac(3, 8)));
    let end = class.ok(&teacher(), Command::EndChallenge);
    assert_eq!(end[0].body, EventBody::ChallengeEnded { challenge: round });
    assert_eq!(end[0].actor, teacher());
    let f1 = OutcomeId::new("F1");
    assert_eq!(
        scores(&end),
        [
            (user(1), f1.clone(), Score::FULL, EvidenceKind::Formative),
            (user(2), f1.clone(), Score::ZERO, EvidenceKind::Formative),
        ]
    );
    assert!(end.iter().all(|e| match &e.body {
        EventBody::EvidenceRecorded { evidence, .. } =>
            evidence.activity == ActivityId::new("quiz-f1"),
        _ => true,
    }));
    // Not an attempt at the quiz: the version due is unchanged, and the round is over.
    assert_eq!(class.due("quiz-f1"), 0);
    assert_eq!(fold(&class.log).challenge, None);
}

#[test]
fn a_live_round_on_a_test_is_still_practice() {
    let mut class = Class::new();
    start(&mut class, "checkpoint");
    // The checkpoint asks 2/5 exactly first.
    class.ok(&learner(1), live(0, frac(2, 5)));
    let end = class.ok(&teacher(), Command::EndChallenge);
    assert_eq!(
        scores(&end),
        [(
            user(1),
            OutcomeId::new("F1"),
            Score::FULL,
            EvidenceKind::Formative
        )]
    );
}

#[test]
fn nobody_answered_means_no_evidence() {
    let mut class = Class::new();
    start(&mut class, "quiz-f1");
    let end = class.ok(&teacher(), Command::EndChallenge);
    assert_eq!(end.len(), 1);
}

#[test]
fn a_live_round_refuses_what_does_not_fit() {
    let mut class = Class::new();
    let refused =
        |class: &mut Class, actor: &Actor, command: Command| class.run(actor, command).unwrap_err();
    let start_on = |quiz: &str| Command::StartChallenge {
        activity: ActivityId::new(quiz),
    };
    // Only teachers run rounds; only learners answer, as people.
    assert_eq!(
        refused(&mut class, &learner(1), start_on("quiz-f1")),
        KernelError::Forbidden {
            role: Role::Learner,
            permission: Permission::RunChallenge
        }
    );
    for command in [Command::NextQuestion, Command::EndChallenge] {
        assert!(matches!(
            refused(&mut class, &learner(1), command),
            KernelError::Forbidden { .. }
        ));
    }
    // Nothing to advance, end or answer yet.
    for command in [Command::NextQuestion, Command::EndChallenge] {
        assert_eq!(
            refused(&mut class, &teacher(), command),
            KernelError::NoChallenge
        );
    }
    assert_eq!(
        refused(&mut class, &learner(1), live(0, choice(0))),
        KernelError::NoChallenge
    );
    // Only a quiz can be played.
    assert_eq!(
        refused(&mut class, &teacher(), start_on("reading-halves")),
        KernelError::WrongActivityKind {
            activity: ActivityId::new("reading-halves")
        }
    );
    assert_eq!(
        refused(&mut class, &teacher(), start_on("nope")),
        KernelError::UnknownActivity {
            activity: ActivityId::new("nope")
        }
    );
    start(&mut class, "quiz-f1");
    assert_eq!(
        refused(&mut class, &teacher(), start_on("quiz-f2")),
        KernelError::ChallengeRunning
    );
    assert!(matches!(
        refused(&mut class, &teacher(), live(0, choice(0))),
        KernelError::Forbidden { .. }
    ));
    assert_eq!(
        refused(&mut class, &agent(), live(0, choice(0))),
        KernelError::Forbidden {
            role: Role::AssessmentAgent,
            permission: Permission::SubmitWork
        }
    );
    let system = Actor::System;
    assert!(matches!(
        refused(&mut class, &system, live(0, choice(0))),
        KernelError::Forbidden { .. }
    ));
    // Not ahead of the board, not twice.
    assert_eq!(
        refused(&mut class, &learner(1), live(1, frac(3, 8))),
        KernelError::NotThisQuestion
    );
    class.ok(&learner(1), live(0, choice(2)));
    assert_eq!(
        refused(&mut class, &learner(1), live(0, choice(0))),
        KernelError::AlreadyAnswered
    );
    // Past the last question there is nothing to show.
    class.ok(&teacher(), Command::NextQuestion);
    assert_eq!(
        refused(&mut class, &teacher(), Command::NextQuestion),
        KernelError::NotThisQuestion
    );
    // Not behind it either.
    assert_eq!(
        refused(&mut class, &learner(2), live(0, choice(0))),
        KernelError::NotThisQuestion
    );
    assert_eq!(
        refused(&mut class, &learner(2), live(2, choice(0))),
        KernelError::NotThisQuestion
    );
    // Once it ends, a new round starts fresh: last round's answers do not count in it.
    class.ok(&teacher(), Command::EndChallenge);
    start(&mut class, "quiz-f1");
    class.ok(&learner(1), live(0, choice(0)));
}

#[test]
fn live_answers_belong_to_their_round_whatever_the_clock_says() {
    let started = event(
        10,
        100,
        teacher(),
        EventBody::ChallengeStarted {
            activity: ActivityId::new("quiz-f1"),
        },
    );
    let round = started.id;
    let other = EventId(Uuid::from_u128(77));
    // The phone's clock was behind: its answer sorts before the round started.
    let early = event(
        11,
        50,
        learner(1),
        EventBody::ChallengeAnswered {
            challenge: round,
            learner: user(1),
            question: 0,
            response: choice(0),
            correct: true,
        },
    );
    // An answer, a step and an end for another round change nothing here.
    let stray = event(
        12,
        120,
        learner(2),
        EventBody::ChallengeAnswered {
            challenge: other,
            learner: user(2),
            question: 0,
            response: choice(0),
            correct: true,
        },
    );
    let step = event(
        13,
        130,
        teacher(),
        EventBody::ChallengeAdvanced {
            challenge: other,
            question: 1,
        },
    );
    let end = event(
        14,
        140,
        teacher(),
        EventBody::ChallengeEnded { challenge: other },
    );
    let state = fold(&[started, early, stray, step, end]);
    let running = state.challenge.unwrap();
    assert_eq!((running.id, running.question), (round, 0));
    assert_eq!(
        running.answers.into_iter().collect::<Vec<_>>(),
        [((user(1), 0), (choice(0), true))]
    );
}
