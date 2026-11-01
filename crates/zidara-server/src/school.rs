//! What the server knows besides the database: lessons, frameworks, the agent policy, time.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use zidara_frameworks::Registry;
use zidara_kernel::governance::Policy;
use zidara_kernel::ids::Timestamp;
use zidara_kernel::lesson::Lesson;

pub use zidara_frameworks::TERM_START;

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
    packs: BTreeMap<String, &'static str>,
    registry: Registry,
    pub policy: Policy,
}

impl School {
    /// The demo: two lessons (fractions, and how Zidara itself works), both frameworks, the
    /// Cambridge calendar opening one outcome a week.
    pub fn demo() -> Self {
        let packs: [&'static str; 2] = [
            include_str!("../../../content/lessons/equivalent-fractions.json"),
            include_str!("../../../content/lessons/how-zidara-works.json"),
        ];
        let lessons: Vec<(Lesson, &'static str)> = packs
            .into_iter()
            .map(|pack| {
                (
                    serde_json::from_str(pack).expect("the demo lessons are valid"),
                    pack,
                )
            })
            .collect();
        Self {
            packs: lessons.iter().map(|(l, p)| (l.id.clone(), *p)).collect(),
            lessons: lessons
                .into_iter()
                .map(|(l, _)| (l.id.clone(), l))
                .collect(),
            registry: zidara_frameworks::demo_registry(),
            policy: Policy::STANDARD,
        }
    }

    pub fn lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    /// The lesson pack exactly as authored, for clients to cache offline.
    pub fn pack(&self, id: &str) -> Option<&'static str> {
        self.packs.get(id).copied()
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}
