//! View intents: what is true for a learner or a teacher, never how to draw it.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::activity::{Expected, Response};
use crate::decide::{Env, KernelError, current_framework};
use crate::event::Event;
use crate::evidence::Evidence;
use crate::framework::{Judgement, LockReason, OutcomeState, ProgressContext};
use crate::governance::Proposal;
use crate::ids::{ActivityId, AgentId, EventId, FrameworkId, OutcomeId, UserId};
use crate::roles::Actor;
use crate::rules::NotifyReason;
use crate::state::{ProposalStatus, fold};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct OutcomeView {
    pub outcome: OutcomeId,
    pub title: String,
    pub judgement: Judgement,
    pub lock: Option<LockReason>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct LearnerView {
    pub framework: FrameworkId,
    pub outcomes: Vec<OutcomeView>,
    /// The first open outcome not yet achieved: where the learner goes next.
    pub next: Option<OutcomeId>,
    pub assigned: Vec<ActivityId>,
    /// The live round's current question, when the teacher is running one.
    pub challenge: Option<LiveQuestion>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct LiveQuestion {
    pub activity: ActivityId,
    pub question: u32,
    /// Whether this learner got it right, once they have answered.
    pub answered: Option<bool>,
}

/// The live round as the teacher's board shows it: who has answered the current question, how
/// many are right, and how many picked each option. Nobody is ranked.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct LiveTally {
    pub activity: ActivityId,
    pub question: u32,
    pub questions: u32,
    pub answered: u32,
    pub correct: u32,
    /// For a choice question, how many picked each option; empty otherwise.
    pub picks: Vec<u32>,
}

pub fn learner_view(
    learner: UserId,
    locale: &str,
    history: &[Event],
    env: &Env<'_>,
) -> Result<LearnerView, KernelError> {
    let state = fold(history);
    let framework = current_framework(&state, env)?;
    let judgements: BTreeMap<OutcomeId, Judgement> = env
        .lesson
        .graph
        .iter()
        .map(|o| {
            (
                o.id.clone(),
                framework.judge(&state.evidence_for(learner, &o.id)),
            )
        })
        .collect();
    let ctx = ProgressContext {
        graph: &env.lesson.graph,
        judgements: &judgements,
        now: env.now,
    };
    let locks: BTreeMap<OutcomeId, Option<LockReason>> = framework
        .availability(&ctx)
        .into_iter()
        .map(|a| (a.outcome, a.lock))
        .collect();
    let outcomes: Vec<OutcomeView> = env
        .lesson
        .graph
        .iter()
        .map(|o| OutcomeView {
            outcome: o.id.clone(),
            title: o.title.get(locale).to_owned(),
            judgement: judgements[&o.id].clone(),
            lock: locks.get(&o.id).cloned().flatten(),
        })
        .collect();
    let next = outcomes
        .iter()
        .find(|o| o.lock.is_none() && o.judgement.state != OutcomeState::Achieved)
        .map(|o| o.outcome.clone());
    let assigned: BTreeSet<ActivityId> = state
        .assignments
        .values()
        .filter(|(l, _, _)| *l == learner)
        .map(|(_, _, a)| a.clone())
        .collect();
    let challenge = state.challenge.map(|c| LiveQuestion {
        answered: c
            .answers
            .get(&(learner, c.question))
            .map(|(_, right)| *right),
        activity: c.activity,
        question: c.question,
    });
    Ok(LearnerView {
        framework: framework.id(),
        outcomes,
        next,
        assigned: assigned.into_iter().collect(),
        challenge,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct OutcomeSummary {
    pub outcome: OutcomeId,
    pub title: String,
    pub achieved: u32,
    pub needs_support: u32,
    pub of: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct QueuedProposal {
    pub id: EventId,
    pub proposal: Proposal,
    pub answer: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct Flag {
    pub learner: UserId,
    pub outcome: OutcomeId,
    pub reason: NotifyReason,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct AgentAction {
    pub id: EventId,
    pub agent: AgentId,
    pub evidence: Evidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
pub struct StudioView {
    pub framework: FrameworkId,
    pub outcomes: Vec<OutcomeSummary>,
    pub queue: Vec<QueuedProposal>,
    pub flags: Vec<Flag>,
    /// Evidence an agent recorded on its own; each can be undone.
    pub agent_actions: Vec<AgentAction>,
    pub challenge: Option<LiveTally>,
}

pub fn studio_view(
    roster: &[UserId],
    locale: &str,
    history: &[Event],
    env: &Env<'_>,
) -> Result<StudioView, KernelError> {
    let state = fold(history);
    let framework = current_framework(&state, env)?;
    let outcomes = env
        .lesson
        .graph
        .iter()
        .map(|o| {
            let states: Vec<OutcomeState> = roster
                .iter()
                .map(|l| framework.judge(&state.evidence_for(*l, &o.id)).state)
                .collect();
            OutcomeSummary {
                outcome: o.id.clone(),
                title: o.title.get(locale).to_owned(),
                achieved: count(&states, OutcomeState::Achieved),
                needs_support: count(&states, OutcomeState::NeedsSupport),
                of: roster.len() as u32,
            }
        })
        .collect();
    let queue = state
        .proposals
        .iter()
        .filter(|(_, (_, status))| *status == ProposalStatus::Queued)
        .map(|(id, (proposal, _))| QueuedProposal {
            id: *id,
            proposal: proposal.clone(),
            answer: state
                .answers
                .get(&proposal.answer)
                .map(|a| a.text.clone())
                .unwrap_or_default(),
        })
        .collect();
    let flags = state
        .notifications
        .iter()
        .map(|(learner, outcome, reason)| Flag {
            learner: *learner,
            outcome: outcome.clone(),
            reason: *reason,
        })
        .collect();
    let agent_actions = state
        .evidence
        .iter()
        .filter_map(|(id, actor, evidence)| match actor {
            Actor::Agent { agent, .. } => Some(AgentAction {
                id: *id,
                agent: agent.clone(),
                evidence: evidence.clone(),
            }),
            _ => None,
        })
        .collect();
    let challenge = state.challenge.map(|c| {
        let asked = env.lesson.questions(&c.activity);
        let now: Vec<&(Response, bool)> = c
            .answers
            .iter()
            .filter(|((_, q), _)| *q == c.question)
            .map(|(_, answer)| answer)
            .collect();
        let options = match asked.get(c.question as usize).map(|q| &q.expected) {
            Some(Expected::Choice { options, .. }) => options.len(),
            _ => 0,
        };
        let picks = (0..options as u32)
            .map(|k| {
                now.iter()
                    .filter(|(r, _)| *r == Response::Choice { index: k })
                    .count() as u32
            })
            .collect();
        LiveTally {
            activity: c.activity,
            question: c.question,
            questions: asked.len() as u32,
            answered: now.len() as u32,
            correct: now.iter().filter(|(_, right)| *right).count() as u32,
            picks,
        }
    });
    Ok(StudioView {
        framework: framework.id(),
        outcomes,
        queue,
        flags,
        agent_actions,
        challenge,
    })
}

fn count(states: &[OutcomeState], wanted: OutcomeState) -> u32 {
    states.iter().filter(|s| **s == wanted).count() as u32
}
