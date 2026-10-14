//! What the server knows besides the database: lessons, frameworks, the agent policy, time.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use zidara_frameworks::{Registry, cambridge::CambridgeStyle, mastery::Mastery};
use zidara_kernel::governance::Policy;
use zidara_kernel::ids::{OutcomeId, Timestamp};
use zidara_kernel::lesson::Lesson;

/// Monday 5 October 2026, 00:00 UTC: the demo term starts.
pub const TERM_START: i64 = 1_791_158_400_000;
const WEEK: i64 = 7 * 24 * 60 * 60 * 1000;

pub trait Clock: Send + Sync + Debug {
    fn now(&self) -> Timestamp;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        let since = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        Timestamp(i64::try_from(since.as_millis()).unwrap_or(i64::MAX))
    }
}

/// A clock tests can set.
#[derive(Debug)]
pub struct FixedClock(AtomicI64);

impl FixedClock {
    pub fn at(ms: i64) -> Self {
        Self(AtomicI64::new(ms))
    }

    pub fn set(&self, ms: i64) {
        self.0.store(ms, Ordering::SeqCst);
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        Timestamp(self.0.load(Ordering::SeqCst))
    }
}

#[derive(Debug)]
pub struct School {
    lessons: BTreeMap<String, Lesson>,
    registry: Registry,
    pub policy: Policy,
}

impl School {
    /// The demo: one lesson, both frameworks, the Cambridge calendar opening one outcome a week.
    pub fn demo() -> Self {
        let lesson: Lesson = serde_json::from_str(include_str!(
            "../../../content/lessons/equivalent-fractions.json"
        ))
        .expect("the demo lesson is valid");
        let opens_at = [("F1", 0), ("F2", 1), ("F3", 2)]
            .into_iter()
            .map(|(o, week)| (OutcomeId::new(o), Timestamp(TERM_START + week * WEEK)))
            .collect();
        let registry = Registry::new(vec![
            Box::new(Mastery::standard()),
            Box::new(CambridgeStyle::primary_checkpoint(opens_at)),
        ]);
        Self {
            lessons: BTreeMap::from([(lesson.id.clone(), lesson)]),
            registry,
            policy: Policy::STANDARD,
        }
    }

    pub fn lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}
