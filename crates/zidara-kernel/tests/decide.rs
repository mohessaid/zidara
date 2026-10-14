mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use support::*;
use zidara_kernel::activity::Response;
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

    fn quiz_f2(&mut self, right: usize) -> Vec<Event> {
        let answers = [frac(2, 4), frac(4, 6), frac(6, 8)];
        let responses = (0..3)
            .map(|i| {
                if i < right {
                    answers[i].clone()
                } else {
                    frac(0, 1)
                }
            })
            .collect();
        self.ok(
            &learner(1),
            Command::SubmitQuiz {
                activity: ActivityId::new("quiz-f2"),
                responses,
            },
        )
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
    let EventBody::EvidenceRecorded { evidence } = &events[1].body else {
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
            responses: wrong(),
        },
    );
    let second = class.ok(
        &learner(1),
        Command::SubmitQuiz {
            activity: ActivityId::new("quiz-f1"),
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
        matches!(&events[1].body, EventBody::EvidenceRecorded { evidence } if evidence.score == Score::FULL)
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
        matches!(&decided[1].body, EventBody::EvidenceRecorded { evidence } if evidence.score == Score::of(5_000))
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
