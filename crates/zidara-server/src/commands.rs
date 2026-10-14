//! The one write path. Every change, online or synced from a phone, comes through here and
//! through the kernel's `decide`, inside one short transaction.

use std::collections::BTreeMap;

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;
use zidara_kernel::decide::{Env, Envelope, KernelError, decide};
use zidara_kernel::event::Event;
use zidara_kernel::ids::{CohortId, EventId, FrameworkId, TenantId, Timestamp, UserId};

pub use crate::error::ApiError;
use crate::ids::UuidV7;
use crate::school::{Clock, School};
use crate::store;
use crate::visibility::learner_of;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Applied { events: Vec<Event> },
    Duplicate,
    Refused { error: KernelError },
}

/// Applies one envelope from `user` to `cohort`.
pub async fn apply(
    pool: &PgPool,
    school: &School,
    clock: &dyn Clock,
    user: Uuid,
    cohort: Uuid,
    envelope: Envelope,
) -> Result<Outcome, ApiError> {
    let mut tx = store::begin_as(pool, user).await?;
    store::lock_cohort(&mut tx, cohort).await?;
    let actor = store::actor(&mut tx, user, cohort)
        .await?
        .ok_or(ApiError::NotMember)?;
    let row = store::cohort(&mut tx, cohort).await?;
    if !store::record_envelope(&mut tx, envelope.id, row.tenant_id, cohort, user).await? {
        tx.commit().await?;
        return Ok(Outcome::Duplicate);
    }
    let lesson = school
        .lesson(&row.lesson_id)
        .ok_or_else(|| ApiError::UnknownLesson(row.lesson_id.clone()))?;
    let history = store::history_with_learners(&mut tx, cohort).await?;
    // When the person acted, no later than now and no earlier than their latest event.
    let latest = history
        .iter()
        .filter(|(e, _)| e.actor == actor)
        .map(|(e, _)| e.at.0)
        .max();
    let at = latest
        .map_or(envelope.at.0, |l| envelope.at.0.max(l))
        .min(clock.now().0);
    let framework = FrameworkId::new(row.framework);
    let env = Env {
        now: Timestamp(at),
        tenant: TenantId(row.tenant_id),
        cohort: CohortId(cohort),
        lesson,
        frameworks: school.registry(),
        default_framework: &framework,
        policy: school.policy,
    };
    let events: Vec<Event> = history.iter().map(|(e, _)| e.clone()).collect();
    let decided = match decide(&actor, envelope.command, &events, &env, &mut UuidV7) {
        Ok(decided) => decided,
        Err(error) => {
            tx.rollback().await?;
            return Ok(Outcome::Refused { error });
        }
    };
    let mut known: BTreeMap<EventId, Option<UserId>> =
        history.iter().map(|(e, l)| (e.id, *l)).collect();
    let rows: Vec<(Event, Option<UserId>)> = decided
        .iter()
        .map(|e| {
            let learner = learner_of(&e.body, &known);
            known.insert(e.id, learner);
            (e.clone(), learner)
        })
        .collect();
    store::insert(&mut tx, &rows).await?;
    tx.commit().await?;
    Ok(Outcome::Applied { events: decided })
}
