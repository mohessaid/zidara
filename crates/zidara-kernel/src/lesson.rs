//! Lessons: outcomes and activities, checked once when loaded.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::activity::{Activity, ActivityKind, ActivityRole, Expected};
use crate::ids::{ActivityId, OutcomeId};
use crate::locale::Localized;
use crate::outcome::{GraphError, Outcome, OutcomeGraph};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum LessonError {
    #[error(transparent)]
    Graph(#[from] GraphError),
    #[error("activity {0:?} appears twice")]
    DuplicateActivity(ActivityId),
    #[error("activity {activity:?} refers to unknown outcome {outcome:?}")]
    UnknownOutcome {
        activity: ActivityId,
        outcome: OutcomeId,
    },
    #[error("question {question} in {activity:?} has no option {correct}")]
    BadChoice {
        activity: ActivityId,
        question: String,
        correct: u32,
    },
}

/// A lesson pack as stored in `content/lessons/*.json`, before it is checked.
#[derive(Deserialize)]
struct LessonPack {
    id: String,
    title: Localized,
    outcomes: Vec<Outcome>,
    activities: Vec<Activity>,
}

/// A checked lesson: every reference resolves.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "LessonPack")]
pub struct Lesson {
    pub id: String,
    pub title: Localized,
    pub graph: OutcomeGraph,
    activities: Vec<Activity>,
}

impl TryFrom<LessonPack> for Lesson {
    type Error = LessonError;
    fn try_from(pack: LessonPack) -> Result<Self, Self::Error> {
        Lesson::new(pack.id, pack.title, pack.outcomes, pack.activities)
    }
}

impl Lesson {
    pub fn new(
        id: impl Into<String>,
        title: Localized,
        outcomes: Vec<Outcome>,
        activities: Vec<Activity>,
    ) -> Result<Self, LessonError> {
        let graph = OutcomeGraph::new(outcomes)?;
        let mut seen = BTreeSet::new();
        for activity in &activities {
            if !seen.insert(activity.id.clone()) {
                return Err(LessonError::DuplicateActivity(activity.id.clone()));
            }
            if let Some(missing) = referenced_outcomes(activity)
                .into_iter()
                .find(|o| graph.get(o).is_none())
            {
                return Err(LessonError::UnknownOutcome {
                    activity: activity.id.clone(),
                    outcome: missing.clone(),
                });
            }
            if let ActivityKind::Quiz { questions, .. } = &activity.kind {
                for question in questions {
                    if let Expected::Choice { options, correct } = &question.expected
                        && *correct as usize >= options.len()
                    {
                        return Err(LessonError::BadChoice {
                            activity: activity.id.clone(),
                            question: question.id.clone(),
                            correct: *correct,
                        });
                    }
                }
            }
        }
        Ok(Self {
            id: id.into(),
            title,
            graph,
            activities,
        })
    }

    pub fn activity(&self, id: &ActivityId) -> Option<&Activity> {
        self.activities.iter().find(|a| &a.id == id)
    }

    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// The first remediation activity covering `outcome`, if the lesson has one.
    pub fn remediation_for(&self, outcome: &OutcomeId) -> Option<&Activity> {
        self.activities
            .iter()
            .find(|a| a.role == ActivityRole::Remediation && a.outcomes.contains(outcome))
    }
}

fn referenced_outcomes(activity: &Activity) -> Vec<&OutcomeId> {
    let mut refs: Vec<&OutcomeId> = activity.outcomes.iter().collect();
    match &activity.kind {
        ActivityKind::Quiz { questions, .. } => refs.extend(questions.iter().map(|q| &q.outcome)),
        ActivityKind::OpenResponse { outcome, .. } => refs.push(outcome),
        ActivityKind::Video { .. } | ActivityKind::Reading { .. } => {}
    }
    refs
}
