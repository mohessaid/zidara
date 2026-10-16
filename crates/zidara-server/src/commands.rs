//! The one write path. Every change, online or synced from a phone, comes through here and
//! through the kernel's `decide`, inside one short transaction per batch.

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
    let mut outcomes = apply_batch(pool, school, clock, user, cohort, vec![envelope]).await?;
    Ok(outcomes.remove(0))
}

/// Applies envelopes in order, in one transaction, with one cohort lock and one history read:
/// each command is decided against the history plus everything accepted earlier in the batch.
/// A refused command rolls back only its own savepoint, leaving no ledger row, so it may be
/// sent again. (Per-envelope transactions reread the whole history each time; see JOURNAL.)
pub async fn apply_batch(
    pool: &PgPool,
    school: &School,
    clock: &dyn Clock,
    user: Uuid,
    cohort: Uuid,
    envelopes: Vec<Envelope>,
) -> Result<Vec<Outcome>, ApiError> {
    let mut tx = store::begin_as(pool, user).await?;
    store::lock_cohort(&mut tx, cohort).await?;
    let actor = store::actor(&mut tx, user, cohort)
        .await?
        .ok_or(ApiError::NotMember)?;
    let row = store::cohort(&mut tx, cohort).await?;
    let lesson = school
        .lesson(&row.lesson_id)
        .ok_or_else(|| ApiError::UnknownLesson(row.lesson_id.clone()))?;
    let history = store::history_with_learners(&mut tx, cohort).await?;
    let mut known: BTreeMap<EventId, Option<UserId>> =
        history.iter().map(|(e, l)| (e.id, *l)).collect();
    let mut events: Vec<Event> = history.into_iter().map(|(e, _)| e).collect();
    let mut latest = events
        .iter()
        .filter(|e| e.actor == actor)
        .map(|e| e.at.0)
        .max();
    let framework = FrameworkId::new(row.framework);
    let mut outcomes = Vec::with_capacity(envelopes.len());
    for envelope in envelopes {
        // A savepoint inside the batch's transaction.
        let mut step = sqlx::Connection::begin(&mut *tx).await?;
        if !store::record_envelope(&mut step, envelope.id, row.tenant_id, cohort, user).await? {
            step.commit().await?;
            outcomes.push(Outcome::Duplicate);
            continue;
        }
        // When the person acted, no later than now and no earlier than their latest event.
        let at = latest
            .map_or(envelope.at.0, |l| envelope.at.0.max(l))
            .min(clock.now().0);
        let env = Env {
            now: Timestamp(at),
            tenant: TenantId(row.tenant_id),
            cohort: CohortId(cohort),
            lesson,
            frameworks: school.registry(),
            default_framework: &framework,
            policy: school.policy,
        };
        match decide(&actor, envelope.command, &events, &env, &mut UuidV7) {
            Err(error) => {
                step.rollback().await?;
                outcomes.push(Outcome::Refused { error });
            }
            Ok(decided) => {
                let rows: Vec<(Event, Option<UserId>)> = decided
                    .iter()
                    .map(|e| {
                        let learner = learner_of(&e.body, &known);
                        known.insert(e.id, learner);
                        (e.clone(), learner)
                    })
                    .collect();
                store::insert(&mut step, &rows).await?;
                step.commit().await?;
                latest = Some(at);
                events.extend(decided.iter().cloned());
                outcomes.push(Outcome::Applied { events: decided });
            }
        }
    }
    tx.commit().await?;
    Ok(outcomes)
}
