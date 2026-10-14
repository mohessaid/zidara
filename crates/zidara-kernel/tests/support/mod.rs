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
