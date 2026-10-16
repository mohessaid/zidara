//! Roles: who may do what. People and agents go through the same table.

use serde::{Deserialize, Serialize};

use crate::ids::{AgentId, UserId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Learner,
    Teacher,
    AssessmentAgent,
    System,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    SubmitWork,
    RecordFormative,
    RecordSummative,
    ProposeAssessment,
    DecideProposal,
    Undo,
    ChangeFramework,
}

/// The permission table. Agents may propose and record formative marks, nothing more.
pub fn allows(role: Role, permission: Permission) -> bool {
    use Permission::*;
    matches!(
        (role, permission),
        (Role::Learner, SubmitWork)
            | (
                Role::Teacher,
                RecordFormative | RecordSummative | DecideProposal | Undo | ChangeFramework
            )
            | (Role::AssessmentAgent, ProposeAssessment | RecordFormative)
            | (Role::System, RecordFormative | RecordSummative)
    )
}

/// Whoever caused an event.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Actor {
    Human { user: UserId, role: Role },
    Agent { agent: AgentId, role: Role },
    System,
}

impl Actor {
    pub fn role(&self) -> Role {
        match self {
            Actor::Human { role, .. } | Actor::Agent { role, .. } => *role,
            Actor::System => Role::System,
        }
    }

    pub fn can(&self, permission: Permission) -> bool {
        allows(self.role(), permission)
    }
}
