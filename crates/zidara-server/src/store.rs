//! Database access for the app role. Every transaction starts by saying who is acting; the
//! database then decides what that person may see and write.

use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use zidara_kernel::event::Event;
use zidara_kernel::ids::{AgentId, CohortId, EventId, TenantId, Timestamp, UserId};
use zidara_kernel::roles::{Actor, Role};

use crate::error::ApiError;

pub type Tx = Transaction<'static, Postgres>;
/// A connection inside a transaction or savepoint.
pub type Conn = sqlx::PgConnection;

/// Begins a transaction acting as `user`. The setting is `SET LOCAL`, so it ends with the
/// transaction and a pooled connection cannot carry it into the next request.
pub async fn begin_as(pool: &PgPool, user: Uuid) -> Result<Tx, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("select set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}

/// Serialises commands per cohort until the transaction ends.
pub async fn lock_cohort(tx: &mut Tx, cohort: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("select pg_advisory_xact_lock(hashtextextended($1::text, 0))")
        .bind(cohort)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Who `user` is in `cohort`, or `None` when they are not a member.
pub async fn actor(tx: &mut Tx, user: Uuid, cohort: Uuid) -> Result<Option<Actor>, ApiError> {
    let row: Option<(String, String, String)> = sqlx::query_as(
        "select m.role, u.kind, u.display_name from memberships m join users u on u.id = m.user_id
         where m.user_id = $1 and m.cohort_id = $2",
    )
    .bind(user)
    .bind(cohort)
    .fetch_optional(&mut **tx)
    .await?;
    let Some((role, kind, name)) = row else {
        return Ok(None);
    };
    let role: Role = serde_json::from_value(Value::String(role))?;
    Ok(Some(if kind == "agent" {
        Actor::Agent {
            agent: AgentId::new(name),
            role,
        }
    } else {
        Actor::Human {
            user: UserId(user),
            role,
        }
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct CohortRow {
    pub tenant_id: Uuid,
    pub lesson_id: String,
    pub framework: String,
}

pub async fn cohort(tx: &mut Tx, cohort: Uuid) -> Result<CohortRow, sqlx::Error> {
    sqlx::query_as("select tenant_id, lesson_id, framework from cohorts where id = $1")
        .bind(cohort)
        .fetch_one(&mut **tx)
        .await
}

/// Records an envelope id; `false` when it was already applied.
pub async fn record_envelope(
    tx: &mut Conn,
    id: Uuid,
    tenant: Uuid,
    cohort: Uuid,
    user: Uuid,
) -> Result<bool, sqlx::Error> {
    let inserted: Option<Uuid> = sqlx::query_scalar(
        "insert into applied_envelopes (id, tenant_id, cohort_id, user_id) values ($1, $2, $3, $4)
         on conflict (id) do nothing returning id",
    )
    .bind(id)
    .bind(tenant)
    .bind(cohort)
    .bind(user)
    .fetch_optional(&mut *tx)
    .await?;
    Ok(inserted.is_some())
}

/// The cohort's events this user may see, oldest first, with the learner each is about.
pub async fn history_with_learners(
    tx: &mut Tx,
    cohort: Uuid,
) -> Result<Vec<(Event, Option<UserId>)>, ApiError> {
    let rows: Vec<(Uuid, Uuid, Option<Uuid>, i64, Value, Value)> = sqlx::query_as(
        "select id, tenant_id, learner_id, at_ms, actor, body from events where cohort_id = $1 order by at_ms, seq",
    )
    .bind(cohort)
    .fetch_all(&mut **tx)
    .await?;
    rows.into_iter()
        .map(|(id, tenant, learner, at, actor, body)| {
            let event = Event {
                id: EventId(id),
                at: Timestamp(at),
                tenant: TenantId(tenant),
                cohort: CohortId(cohort),
                actor: serde_json::from_value(actor)?,
                body: serde_json::from_value(body)?,
            };
            Ok((event, learner.map(UserId)))
        })
        .collect()
}

/// A page of visible events after a `(at_ms, seq)` cursor, with each event's `seq`.
pub async fn page(
    tx: &mut Tx,
    cohort: Uuid,
    after: (i64, i64),
    limit: i64,
) -> Result<Vec<(i64, Event)>, ApiError> {
    let rows: Vec<(i64, Uuid, Uuid, i64, Value, Value)> = sqlx::query_as(
        "select seq, id, tenant_id, at_ms, actor, body from events
         where cohort_id = $1 and (at_ms, seq) > ($2, $3) order by at_ms, seq limit $4",
    )
    .bind(cohort)
    .bind(after.0)
    .bind(after.1)
    .bind(limit)
    .fetch_all(&mut **tx)
    .await?;
    rows.into_iter()
        .map(|(seq, id, tenant, at, actor, body)| {
            let event = Event {
                id: EventId(id),
                at: Timestamp(at),
                tenant: TenantId(tenant),
                cohort: CohortId(cohort),
                actor: serde_json::from_value(actor)?,
                body: serde_json::from_value(body)?,
            };
            Ok((seq, event))
        })
        .collect()
}

/// The learners of a cohort, for teachers (RLS shows a learner only their own row).
pub async fn roster(tx: &mut Tx, cohort: Uuid) -> Result<Vec<UserId>, sqlx::Error> {
    let ids: Vec<Uuid> = sqlx::query_scalar("select user_id from memberships where cohort_id = $1 and role = 'learner' order by user_id")
        .bind(cohort)
        .fetch_all(&mut **tx)
        .await?;
    Ok(ids.into_iter().map(UserId).collect())
}

pub async fn history(tx: &mut Tx, cohort: Uuid) -> Result<Vec<Event>, ApiError> {
    Ok(history_with_learners(tx, cohort)
        .await?
        .into_iter()
        .map(|(e, _)| e)
        .collect())
}

/// One array per column, for a single `unnest` insert.
#[derive(Default)]
struct Columns {
    ids: Vec<Uuid>,
    tenants: Vec<Uuid>,
    cohorts: Vec<Uuid>,
    learners: Vec<Option<Uuid>>,
    times: Vec<i64>,
    actors: Vec<Value>,
    bodies: Vec<Value>,
}

/// Appends events in one statement.
pub async fn insert(tx: &mut Conn, events: &[(Event, Option<UserId>)]) -> Result<(), ApiError> {
    let mut c = Columns::default();
    for (event, learner) in events {
        c.ids.push(event.id.0);
        c.tenants.push(event.tenant.0);
        c.cohorts.push(event.cohort.0);
        c.learners.push(learner.map(|l| l.0));
        c.times.push(event.at.0);
        c.actors.push(serde_json::to_value(&event.actor)?);
        c.bodies.push(serde_json::to_value(&event.body)?);
    }
    sqlx::query(
        "insert into events (id, tenant_id, cohort_id, learner_id, at_ms, actor, body)
         select * from unnest($1::uuid[], $2::uuid[], $3::uuid[], $4::uuid[], $5::int8[], $6::jsonb[], $7::jsonb[])",
    )
    .bind(c.ids)
    .bind(c.tenants)
    .bind(c.cohorts)
    .bind(c.learners)
    .bind(c.times)
    .bind(c.actors)
    .bind(c.bodies)
    .execute(&mut *tx)
    .await?;
    Ok(())
}
