//! Shared by every kernel test file; not every file uses every helper.
#![allow(dead_code)]

use proptest::strategy::Strategy;
use proptest::test_runner::{Config, RngAlgorithm, TestCaseError, TestRng, TestRunner};
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::evidence::{Evidence, EvidenceKind};
use zidara_kernel::ids::*;
use zidara_kernel::roles::{Actor, Role};
use zidara_kernel::score::Score;

/// Runs a property with a fixed seed: failures replay exactly, and wasm32 needs no file system.
pub fn check<S: Strategy>(strategy: S, test: impl Fn(S::Value) -> Result<(), TestCaseError>) {
    let config = Config {
        cases: 256,
        failure_persistence: None,
        ..Config::default()
    };
    let mut runner =
        TestRunner::new_with_rng(config, TestRng::deterministic_rng(RngAlgorithm::ChaCha));
    if let Err(failure) = runner.run(&strategy, test) {
        panic!("{failure}");
    }
}

pub fn user(n: u128) -> UserId {
    UserId(Uuid::from_u128(1_000 + n))
}

pub fn learner(n: u128) -> Actor {
    Actor::Human {
        user: user(n),
        role: Role::Learner,
    }
}

pub fn teacher() -> Actor {
    Actor::Human {
        user: user(99),
        role: Role::Teacher,
    }
}

pub fn agent() -> Actor {
    Actor::Agent {
        agent: AgentId::new("assessor"),
        role: Role::AssessmentAgent,
    }
}

pub fn event(id: u128, at: i64, actor: Actor, body: EventBody) -> Event {
    Event {
        id: EventId(Uuid::from_u128(id)),
        at: Timestamp(at),
        tenant: TenantId(Uuid::from_u128(1)),
        cohort: CohortId(Uuid::from_u128(2)),
        actor,
        body,
    }
}

pub fn evidence(learner: u128, outcome: &str, bp: u16) -> Evidence {
    Evidence {
        learner: user(learner),
        outcome: OutcomeId::new(outcome),
        activity: ActivityId::new("quiz"),
        score: Score::of(bp),
        kind: EvidenceKind::Formative,
    }
}
// --- appended to the support module from Task 8 ---

use zidara_kernel::decide::Env;
use zidara_kernel::framework::{
    Availability, Band, Framework, FrameworkRegistry, Judgement, OutcomeState, ProgressContext,
};
use zidara_kernel::governance::Policy;
use zidara_kernel::lesson::Lesson;
use zidara_kernel::rules::Rule;

pub const PACK: &str = include_str!("../../../../content/lessons/equivalent-fractions.json");

pub fn lesson() -> Lesson {
    serde_json::from_str(PACK).unwrap()
}

/// A deliberately simple framework: the kernel must work with any implementation.
/// Achieved at 50% mean; remediation below 50% after 2 tries; everything open.
#[derive(Debug)]
pub struct TestFramework {
    pub id: &'static str,
    pub rules: Vec<Rule>,
}

impl TestFramework {
    pub fn standard(id: &'static str) -> Self {
        Self {
            id,
            rules: vec![
                Rule::Remediate {
                    below: Score::of(5_000),
                    after_attempts: 2,
                },
                Rule::NotifyBelowRank { rank: 1 },
            ],
        }
    }
}

impl Framework for TestFramework {
    fn id(&self) -> FrameworkId {
        FrameworkId::new(self.id)
    }

    fn judge(&self, evidence: &[&Evidence]) -> Judgement {
        let scores: Vec<Score> = evidence.iter().map(|e| e.score).collect();
        let Some(score) = Score::mean(&scores) else {
            return Judgement::NOT_STARTED;
        };
        let achieved = score >= Score::of(5_000);
        Judgement {
            state: if achieved {
                OutcomeState::Achieved
            } else {
                OutcomeState::InProgress
            },
            score: Some(score),
            band: Some(Band {
                label: "t".into(),
                rank: u8::from(achieved),
                of: 2,
            }),
            attempts: evidence.len() as u32,
        }
    }

    fn availability(&self, ctx: &ProgressContext<'_>) -> Vec<Availability> {
        ctx.graph
            .iter()
            .map(|o| Availability {
                outcome: o.id.clone(),
                lock: None,
            })
            .collect()
    }

    fn rules(&self) -> &[Rule] {
        &self.rules
    }
}

#[derive(Debug)]
pub struct TestRegistry(pub Vec<TestFramework>);

impl TestRegistry {
    pub fn standard() -> Self {
        Self(vec![
            TestFramework::standard("test"),
            TestFramework {
                id: "quiet",
                rules: vec![],
            },
        ])
    }
}

impl FrameworkRegistry for TestRegistry {
    fn get(&self, id: &FrameworkId) -> Option<&dyn Framework> {
        self.0
            .iter()
            .find(|f| f.id == id.as_str())
            .map(|f| f as &dyn Framework)
    }
}

pub fn env<'a>(
    lesson: &'a Lesson,
    registry: &'a TestRegistry,
    framework: &'a FrameworkId,
    now: i64,
) -> Env<'a> {
    Env {
        now: Timestamp(now),
        tenant: TenantId(Uuid::from_u128(1)),
        cohort: CohortId(Uuid::from_u128(2)),
        lesson,
        frameworks: registry,
        default_framework: framework,
        policy: Policy::STANDARD,
    }
}

/// A quiz submission by `learner` for the version of `activity` that is due, with the first
/// `right` questions answered correctly and the rest wrongly.
pub fn answer_due(
    log: &[Event],
    learner: u128,
    activity: &str,
    right: usize,
) -> zidara_kernel::decide::Command {
    use zidara_kernel::activity::{ActivityKind, Expected, Response, version};
    let lesson = lesson();
    let id = ActivityId::new(activity);
    let variant = zidara_kernel::decide::next_variant(log, user(learner), &id, &lesson);
    let Some(ActivityKind::Quiz {
        questions,
        variants,
        ..
    }) = lesson.activity(&id).map(|a| a.kind.clone())
    else {
        panic!("{activity} is a quiz")
    };
    let questions = version(&questions, &variants, variant)
        .expect("the due version exists")
        .to_vec();
    let responses = questions
        .iter()
        .enumerate()
        .map(|(i, q)| match (&q.expected, i < right) {
            (Expected::Choice { correct, .. }, true) => Response::Choice { index: *correct },
            (Expected::Choice { correct, options }, false) => Response::Choice {
                index: (correct + 1) % options.len() as u32,
            },
            (Expected::Exactly { num, den } | Expected::EquivalentTo { num, den }, true) => {
                Response::Fraction {
                    num: *num,
                    den: *den,
                }
            }
            (_, false) => Response::Fraction { num: 0, den: 1 },
        })
        .collect();
    zidara_kernel::decide::Command::SubmitQuiz {
        activity: id,
        variant,
        responses,
    }
}
