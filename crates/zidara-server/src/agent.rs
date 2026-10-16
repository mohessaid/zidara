//! The assessment agent: an actor with a membership, not a back door. It reads what its role
//! lets it read, marks outside any transaction, and proposes through the same command path as
//! everyone else.

use sqlx::PgPool;
use uuid::Uuid;
use zidara_kernel::activity::ActivityKind;
use zidara_kernel::decide::{Command, Envelope};
use zidara_kernel::state::fold;

use crate::assessor::{Assessor, OpenItem};
use crate::commands::{ApiError, apply_batch};
use crate::school::{Clock, School};
use crate::store;

/// How many proposals the agent applies per transaction (bounds how long it holds the lock).
pub const PROPOSALS_PER_BATCH: usize = 8;

/// Marks every answer in `cohort` that has no proposal yet; returns how many it marked.
pub async fn mark_pending(
    pool: &PgPool,
    school: &School,
    clock: &dyn Clock,
    assessor: &Assessor,
    agent: Uuid,
    cohort: Uuid,
) -> Result<usize, ApiError> {
    let mut tx = store::begin_as(pool, agent).await?;
    if store::actor(&mut tx, agent, cohort).await?.is_none() {
        return Err(ApiError::NotMember);
    }
    let row = store::cohort(&mut tx, cohort).await?;
    let state = fold(&store::history(&mut tx, cohort).await?);
    tx.commit().await?;
    let lesson = school
        .lesson(&row.lesson_id)
        .ok_or_else(|| ApiError::UnknownLesson(row.lesson_id.clone()))?;
    // Mark every pending answer first, with no transaction open while the model thinks, then
    // propose them all in one batch: one lock, one history read.
    let mut proposals = Vec::new();
    for (id, answer) in state
        .answers
        .iter()
        .filter(|(id, _)| !state.has_proposal_for(**id))
    {
        let Some(ActivityKind::OpenResponse { prompt, rubric, .. }) =
            lesson.activity(&answer.activity).map(|a| &a.kind)
        else {
            continue;
        };
        let item = OpenItem {
            prompt: prompt.get("en"),
            rubric,
        };
        let assessment = assessor.assess(&item, &answer.text).await;
        proposals.push(Envelope {
            id: Uuid::now_v7(),
            at: clock.now(),
            command: Command::ProposeAssessment {
                answer: *id,
                assessment,
            },
        });
    }
    // Proposed in small batches: one batch for everything held the class lock long enough to
    // queue every learner behind the agent (p95 doubled under load); 8 at a time does not.
    let marked = proposals.len();
    let mut proposals = proposals.into_iter().peekable();
    while proposals.peek().is_some() {
        let chunk: Vec<Envelope> = proposals.by_ref().take(PROPOSALS_PER_BATCH).collect();
        apply_batch(pool, school, clock, agent, cohort, chunk).await?;
    }
    Ok(marked)
}

/// At most one marking pass per class at a time. Answers that arrive during a pass ask for one
/// more pass, not one each: per-answer passes each rescanned every answer and fought over the
/// class lock (39 requests/s under load; see docs/JOURNAL.md).
#[derive(Debug)]
pub struct Passes {
    // cohort -> another pass wanted after the current one
    running: std::sync::Mutex<std::collections::HashMap<Uuid, bool>>,
    /// One marking pass at a time across all classes, so marking never takes more than one
    /// core from requests (agent passes caused the request tail under load; see JOURNAL).
    pub gate: tokio::sync::Semaphore,
}

impl Default for Passes {
    fn default() -> Self {
        Self {
            running: Default::default(),
            gate: tokio::sync::Semaphore::new(1),
        }
    }
}

impl Passes {
    /// Asks for a pass; `true` means the caller should run it now.
    pub fn request(&self, cohort: Uuid) -> bool {
        let mut running = self
            .running
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match running.get_mut(&cohort) {
            Some(again) => {
                *again = true;
                false
            }
            None => {
                running.insert(cohort, false);
                true
            }
        }
    }

    /// Ends a pass; `true` means another was asked for meanwhile and the caller should run it.
    pub fn finished(&self, cohort: Uuid) -> bool {
        let mut running = self
            .running
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match running.get_mut(&cohort) {
            Some(again) if *again => {
                *again = false;
                true
            }
            _ => {
                running.remove(&cohort);
                false
            }
        }
    }
}
