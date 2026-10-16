//! Identity and time. The kernel never makes either up: shells pass them in.

use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

macro_rules! text_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        #[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

macro_rules! uuid_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        #[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
        pub struct $name(pub Uuid);
    };
}

text_id!(/// An outcome, e.g. `F2`.
    OutcomeId);
text_id!(/// An activity in a lesson, e.g. `quiz-f1`.
    ActivityId);
text_id!(/// An AI agent acting in the system, e.g. `assessor`.
    AgentId);
text_id!(/// A framework, e.g. `mastery`.
    FrameworkId);
uuid_id!(/// An event in the log.
    EventId);
uuid_id!(/// A person.
    UserId);
uuid_id!(/// A cohort: the unit of isolation.
    CohortId);
uuid_id!(/// A tenant: a school or organisation.
    TenantId);

/// Milliseconds since the Unix epoch, UTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(transparent)]
pub struct Timestamp(pub i64);

/// Hands out event ids. Shells use UUIDv7; tests and replays count.
pub trait IdGen {
    fn next_id(&mut self) -> EventId;
}

/// Ids 1, 2, 3, … for tests and deterministic replays.
#[derive(Debug, Default)]
pub struct SequentialIds(u128);

impl IdGen for SequentialIds {
    fn next_id(&mut self) -> EventId {
        self.0 += 1;
        EventId(Uuid::from_u128(self.0))
    }
}
