//! The golden session (`crates/zidara-wasm/tests/golden/session.json`), recorded by the
//! phone's kernel, replayed through the server's HTTP API on PostgreSQL: after every step the
//! learner's view must be the one the phone computed. The assessment agent is off here; its
//! marks arrive after sync and are not part of what the phone computes offline.
mod support;

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use support::*;
use tower::ServiceExt;
use uuid::Uuid;
use zidara_kernel::ids::Timestamp;
use zidara_server::assessor::Assessor;
use zidara_server::auth::{Keys, issue};
use zidara_server::school::{FixedClock, School};
use zidara_server::{AppState, app};

const SESSION: &str = include_str!("../../zidara-wasm/tests/golden/session.json");

#[tokio::test]
async fn the_server_reaches_the_views_the_phone_computed() {
    let session: Value = serde_json::from_str(SESSION).unwrap();
    let db = db().await;
    let keys = Keys::from_secret(b"golden test secret, long enough to sign");
    let clock = Arc::new(FixedClock::at(0));
    let router = app(AppState {
        pool: db.app.clone(),
        school: Arc::new(School::demo()),
        clock: clock.clone(),
        keys: keys.clone(),
        assessor: Arc::new(Assessor::Rubric),
        agent: Uuid::nil(),
        passes: Default::default(),
        demo_login: false,
    });
    let cohort = session["setup"]["cohort"].as_str().unwrap();
    let learner: Uuid = session["learner"].as_str().unwrap().parse().unwrap();
    for (i, step) in session["steps"].as_array().unwrap().iter().enumerate() {
        let at = step["envelope"]["at"].as_i64().unwrap();
        clock.set(at);
        let actor: Uuid = step["actor"]["user"].as_str().unwrap().parse().unwrap();
        let token = issue(&keys, actor, Timestamp(at));
        let post = Request::post(format!("/api/cohorts/{cohort}/commands"))
            .header("authorization", format!("Bearer {token}"))
            .header("content-type", "application/json")
            .body(Body::from(
                Value::Array(vec![step["envelope"].clone()]).to_string(),
            ))
            .unwrap();
        let response = router.clone().oneshot(post).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "step {i}");
        let token = issue(&keys, learner, Timestamp(at));
        let get = Request::get(format!("/api/cohorts/{cohort}/learner-view?locale=ar"))
            .header("authorization", format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();
        let bytes = router
            .clone()
            .oneshot(get)
            .await
            .unwrap()
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        let view: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(view, session["views_ar"][i], "step {i}");
    }
}
