//! HTTP handlers. Data only: the server sends JSON, never pages.

use axum::extract::{FromRequestParts, Path, Query, State};
use axum::http::{header, request::Parts};
use axum::response::IntoResponse;
use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;
use zidara_kernel::decide::{Env, Envelope};
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::ids::{CohortId, FrameworkId, TenantId, UserId};
use zidara_kernel::roles::{Actor, Role};
use zidara_kernel::view::{learner_view, studio_view};

use crate::AppState;
use crate::agent::mark_pending;
use crate::auth::{issue, verify};
use crate::commands::{ApiError, Outcome, apply_batch};
use crate::store::{self, CohortRow, Tx};
use crate::xapi::statement;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login))
        .route("/api/guest", post(guest))
        .route("/api/me", get(me))
        .route("/api/packs/{id}", get(pack))
        .route("/api/cohorts/{id}/commands", post(commands))
        .route("/api/cohorts/{id}/events", get(events))
        .route("/api/cohorts/{id}/learner-view", get(learner))
        .route("/api/cohorts/{id}/studio", get(studio))
        .route("/api/cohorts/{id}/xapi", get(xapi))
        .route("/api/cohorts/{id}/roster", get(roster))
        .with_state(state)
}

/// The signed-in user, from `Authorization: Bearer <token>`.
#[derive(Clone, Copy, Debug)]
pub struct Auth(pub Uuid);

impl FromRequestParts<AppState> for Auth {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(ApiError::Unauthorized)?;
        verify(&state.keys, token, state.clock.now())
            .map(Auth)
            .map_err(|_| ApiError::Unauthorized)
    }
}

#[derive(Deserialize)]
struct Login {
    user_id: Uuid,
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<Login>,
) -> Result<Json<Value>, ApiError> {
    if !state.demo_login {
        return Err(ApiError::NotFound);
    }
    let mut tx = store::begin_as(&state.pool, body.user_id).await?;
    let known: i64 = sqlx::query_scalar("select count(*) from users where id = $1")
        .bind(body.user_id)
        .fetch_one(&mut *tx)
        .await?;
    if known == 0 {
        return Err(ApiError::Unauthorized);
    }
    Ok(Json(
        json!({ "token": issue(&state.keys, body.user_id, state.clock.now()) }),
    ))
}

/// Starts a guest: a new learner in every open class, with no name and no password (ADR 0012).
async fn guest(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    if !state.demo_login {
        return Err(ApiError::NotFound);
    }
    let mut tx = state.pool.begin().await?;
    let guest: Option<Uuid> = sqlx::query_scalar("select private.join_as_guest($1::integer)")
        .bind(state.guest_limit)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    let guest = guest.ok_or(ApiError::Full)?;
    Ok(Json(
        json!({ "token": issue(&state.keys, guest, state.clock.now()) }),
    ))
}

async fn me(State(state): State<AppState>, Auth(user): Auth) -> Result<Json<Value>, ApiError> {
    let mut tx = store::begin_as(&state.pool, user).await?;
    let name: String = sqlx::query_scalar("select display_name from users where id = $1")
        .bind(user)
        .fetch_one(&mut *tx)
        .await?;
    let cohorts: Vec<(Uuid, Uuid, String, String, String, String)> = sqlx::query_as(
        "select c.id, c.tenant_id, c.name, c.lesson_id, c.framework, m.role
         from memberships m join cohorts c on c.id = m.cohort_id
         where m.user_id = $1 order by c.name",
    )
    .bind(user)
    .fetch_all(&mut *tx)
    .await?;
    let cohorts: Vec<Value> = cohorts
        .into_iter()
        .map(|(id, tenant_id, name, lesson_id, framework, role)| {
            json!({ "id": id, "tenant_id": tenant_id, "name": name, "lesson_id": lesson_id, "framework": framework, "role": role })
        })
        .collect();
    Ok(Json(
        json!({ "id": user, "display_name": name, "cohorts": cohorts }),
    ))
}

async fn pack(
    State(state): State<AppState>,
    Auth(_): Auth,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let pack = state.school.pack(&id).ok_or(ApiError::NotFound)?;
    Ok(([(header::CONTENT_TYPE, "application/json")], pack))
}

async fn commands(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
    Json(envelopes): Json<Vec<Envelope>>,
) -> Result<Json<Value>, ApiError> {
    let ids: Vec<Uuid> = envelopes.iter().map(|e| e.id).collect();
    let outcomes = apply_batch(
        &state.pool,
        &state.school,
        state.clock.as_ref(),
        user,
        cohort,
        envelopes,
    )
    .await?;
    let answered = outcomes.iter().any(|o| {
        matches!(o, Outcome::Applied { events } if events.iter().any(|e| matches!(e.body, EventBody::AnswerSubmitted { .. })))
    });
    let results: Vec<Value> = ids
        .iter()
        .zip(&outcomes)
        .map(|(id, outcome)| json!({ "id": id, "outcome": outcome }))
        .collect();
    if answered && state.passes.request(cohort) {
        // The agent marks outside this request (the learner does not wait for the model), one
        // pass per class at a time; answers that arrive meanwhile are marked by one more pass.
        // Marking folds a whole class's history: it runs on the blocking pool, one pass at a
        // time, so request workers never wait behind it.
        let state = state.clone();
        let runtime = tokio::runtime::Handle::current();
        tokio::task::spawn_blocking(move || {
            runtime.block_on(async {
                loop {
                    let permit = state.passes.gate.acquire().await;
                    let _ = mark_pending(
                        &state.pool,
                        &state.school,
                        state.clock.as_ref(),
                        &state.assessor,
                        state.agent,
                        cohort,
                    )
                    .await;
                    drop(permit);
                    if !state.passes.finished(cohort) {
                        break;
                    }
                }
            })
        });
    }
    Ok(Json(Value::Array(results)))
}

#[derive(Deserialize)]
struct Cursor {
    after_ms: Option<i64>,
    after_seq: Option<i64>,
    limit: Option<i64>,
}

async fn events(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
    Query(cursor): Query<Cursor>,
) -> Result<Json<Value>, ApiError> {
    let (mut tx, _) = member(&state, user, cohort).await?;
    let after = (
        cursor.after_ms.unwrap_or(i64::MIN),
        cursor.after_seq.unwrap_or(i64::MIN),
    );
    let page = store::page(
        &mut tx,
        cohort,
        after,
        cursor.limit.unwrap_or(500).clamp(1, 1_000),
    )
    .await?;
    Ok(Json(
        page.into_iter()
            .map(|(seq, event)| json!({ "seq": seq, "event": event }))
            .collect(),
    ))
}

#[derive(Deserialize)]
struct Locale {
    locale: Option<String>,
}

async fn learner(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
    Query(locale): Query<Locale>,
) -> Result<Json<Value>, ApiError> {
    let (mut tx, _) = member(&state, user, cohort).await?;
    let row = store::cohort(&mut tx, cohort).await?;
    let history = store::history(&mut tx, cohort).await?;
    let locale = locale.locale.unwrap_or_else(|| "ar".into());
    let view = with_env(&state, &row, cohort, |env| {
        learner_view(UserId(user), &locale, &history, env)
    })??;
    Ok(Json(serde_json::to_value(view)?))
}

async fn studio(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
    Query(locale): Query<Locale>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = teacher(&state, user, cohort).await?;
    let row = store::cohort(&mut tx, cohort).await?;
    let history = store::history(&mut tx, cohort).await?;
    let roster = store::roster(&mut tx, cohort).await?;
    let locale = locale.locale.unwrap_or_else(|| "ar".into());
    let view = with_env(&state, &row, cohort, |env| {
        studio_view(&roster, &locale, &history, env)
    })??;
    Ok(Json(serde_json::to_value(view)?))
}

async fn xapi(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = teacher(&state, user, cohort).await?;
    let row = store::cohort(&mut tx, cohort).await?;
    let history: Vec<Event> = store::history(&mut tx, cohort).await?;
    Ok(Json(
        history
            .iter()
            .filter_map(|e| statement(e, &row.lesson_id))
            .collect(),
    ))
}

/// The cohort's learners with their names, for the teacher's studio.
async fn roster(
    State(state): State<AppState>,
    Auth(user): Auth,
    Path(cohort): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = teacher(&state, user, cohort).await?;
    let people: Vec<(Uuid, String)> = sqlx::query_as(
        "select u.id, u.display_name from memberships m join users u on u.id = m.user_id
         where m.cohort_id = $1 and m.role = 'learner' order by u.display_name",
    )
    .bind(cohort)
    .fetch_all(&mut *tx)
    .await?;
    Ok(Json(
        people
            .into_iter()
            .map(|(id, name)| json!({ "id": id, "display_name": name }))
            .collect(),
    ))
}

/// A transaction as `user`, refused unless they belong to `cohort`.
async fn member(state: &AppState, user: Uuid, cohort: Uuid) -> Result<(Tx, Actor), ApiError> {
    let mut tx = store::begin_as(&state.pool, user).await?;
    let actor = store::actor(&mut tx, user, cohort)
        .await?
        .ok_or(ApiError::NotMember)?;
    Ok((tx, actor))
}

async fn teacher(state: &AppState, user: Uuid, cohort: Uuid) -> Result<Tx, ApiError> {
    let (tx, actor) = member(state, user, cohort).await?;
    if actor.role() != Role::Teacher {
        return Err(ApiError::Forbidden);
    }
    Ok(tx)
}

fn with_env<T>(
    state: &AppState,
    row: &CohortRow,
    cohort: Uuid,
    view: impl FnOnce(&Env<'_>) -> T,
) -> Result<T, ApiError> {
    let lesson = state
        .school
        .lesson(&row.lesson_id)
        .ok_or_else(|| ApiError::UnknownLesson(row.lesson_id.clone()))?;
    let framework = FrameworkId::new(row.framework.clone());
    let env = Env {
        now: state.clock.now(),
        tenant: TenantId(row.tenant_id),
        cohort: CohortId(cohort),
        lesson,
        frameworks: state.school.registry(),
        default_framework: &framework,
        policy: state.school.policy,
    };
    Ok(view(&env))
}
