//! The assessment agent: an actor with a membership, not a back door. It reads what its role
//! lets it read, marks outside any transaction, and proposes through the same command path as
//! everyone else.

use sqlx::PgPool;
use uuid::Uuid;
use zidara_kernel::activity::ActivityKind;
use zidara_kernel::decide::{Command, Envelope};
use zidara_kernel::state::fold;

use crate::assessor::{Assessor, OpenItem};
use crate::commands::{ApiError, apply};
use crate::school::{Clock, School};
use crate::store;

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
    let mut marked = 0;
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
        // No transaction is open while the model thinks.
        let assessment = assessor.assess(&item, &answer.text).await;
        let envelope = Envelope {
            id: Uuid::now_v7(),
            at: clock.now(),
            command: Command::ProposeAssessment {
                answer: *id,
                assessment,
            },
        };
        apply(pool, school, clock, agent, cohort, envelope).await?;
        marked += 1;
    }
    Ok(marked)
}
