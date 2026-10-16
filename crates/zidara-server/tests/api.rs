//! The HTTP API, end to end against the real database (no network: tower's `oneshot`).
mod support;

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use support::*;
use tower::ServiceExt;
use uuid::Uuid;
use zidara_server::assessor::Assessor;
use zidara_server::auth::{Keys, issue};
use zidara_server::school::{FixedClock, School};
use zidara_server::{AppState, app};

const NOW: i64 = 1_791_158_400_000 + 3_600_000;

struct Api {
    router: Router,
    keys: Keys,
    db: TestDb,
}

async fn api(demo_login: bool) -> Api {
    let db = db().await;
    let keys = Keys::from_secret(b"api test secret, long enough to sign");
    let state = AppState {
        pool: db.app.clone(),
        school: Arc::new(School::demo()),
        clock: Arc::new(FixedClock::at(NOW)),
        keys: keys.clone(),
        assessor: Arc::new(Assessor::Rubric),
        agent: db.seed.agent,
        demo_login,
    };
    Api {
        router: app(state),
        keys,
        db,
    }
}

impl Api {
    fn token(&self, user: Uuid) -> String {
        issue(&self.keys, user, zidara_kernel::ids::Timestamp(NOW))
    }

    async fn call(
        &self,
        method: &str,
        uri: &str,
        user: Option<Uuid>,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let mut request = Request::builder().method(method).uri(uri);
        if let Some(user) = user {
            request = request.header("authorization", format!("Bearer {}", self.token(user)));
        }
        let request = match body {
            Some(body) => request
                .header("content-type", "application/json")
                .body(Body::from(body.to_string())),
            None => request.body(Body::empty()),
        }
        .unwrap();
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(Value::Null),
        )
    }
}

/// An envelope for version `variant` of quiz-f2 (attempts rotate 0, 1, 2, …).
fn quiz_f2(variant: u32, right: usize) -> Value {
    json!({ "id": Uuid::now_v7(), "at": NOW, "command": support::quiz_f2(variant, right) })
}

#[tokio::test]
async fn a_lesson_round_trip() {
    let api = api(true).await;
    let s = api.db.seed;
    let (status, login) = api
        .call(
            "POST",
            "/api/login",
            None,
            Some(json!({ "user_id": s.amina })),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert!(login["token"].as_str().unwrap().len() > 20);
    let (status, me) = api.call("GET", "/api/me", Some(s.amina), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["display_name"], "Amina");
    assert_eq!(
        me["cohorts"],
        json!([{ "id": s.a, "tenant_id": s.tenant, "name": "Class 4 A", "lesson_id": "equivalent-fractions", "framework": "mastery", "role": "learner" }])
    );
    let (status, pack) = api
        .call(
            "GET",
            "/api/packs/equivalent-fractions",
            Some(s.amina),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(pack["title"]["ar"], "الكسور المتكافئة");
    let sent = quiz_f2(0, 3);
    let (status, results) = api
        .call(
            "POST",
            &format!("/api/cohorts/{}/commands", s.a),
            Some(s.amina),
            Some(json!([sent.clone(), sent])),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(results[0]["outcome"]["status"], "applied");
    assert_eq!(results[1]["outcome"]["status"], "duplicate");
    let (status, view) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/learner-view?locale=fr", s.a),
            Some(s.amina),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        view["outcomes"][1]["title"],
        "Trouver des fractions équivalentes"
    );
    assert_eq!(view["outcomes"][1]["judgement"]["state"], "achieved");
}

#[tokio::test]
async fn sign_in_needs_a_known_user_a_valid_token_and_demo_mode() {
    let api = api(true).await;
    let (status, _) = api
        .call(
            "POST",
            "/api/login",
            None,
            Some(json!({ "user_id": Uuid::now_v7() })),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = api.call("GET", "/api/me", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let request = Request::get("/api/me")
        .header("authorization", "Bearer nonsense")
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        api.router.clone().oneshot(request).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
    let closed = api_without_demo().await;
    let (status, _) = closed
        .call(
            "POST",
            "/api/login",
            None,
            Some(json!({ "user_id": closed.db.seed.amina })),
        )
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

async fn api_without_demo() -> Api {
    api(false).await
}

#[tokio::test]
async fn learners_cannot_open_the_studio_or_other_cohorts() {
    let api = api(true).await;
    let s = api.db.seed;
    for uri in [
        format!("/api/cohorts/{}/studio", s.a),
        format!("/api/cohorts/{}/xapi", s.a),
        format!("/api/cohorts/{}/learner-view", s.b),
        format!("/api/cohorts/{}/events", s.b),
    ] {
        let (status, _) = api.call("GET", &uri, Some(s.amina), None).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{uri}");
    }
    let (status, _) = api
        .call(
            "POST",
            &format!("/api/cohorts/{}/commands", s.b),
            Some(s.amina),
            Some(json!([quiz_f2(0, 3)])),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = api
        .call("GET", "/api/packs/nope", Some(s.amina), None)
        .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn events_page_by_keyset() {
    let api = api(true).await;
    let s = api.db.seed;
    for (variant, right) in [(0, 0), (1, 0), (2, 3)] {
        api.call(
            "POST",
            &format!("/api/cohorts/{}/commands", s.a),
            Some(s.amina),
            Some(json!([quiz_f2(variant, right)])),
        )
        .await;
    }
    let (_, all) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/events", s.a),
            Some(s.amina),
            None,
        )
        .await;
    let all = all.as_array().unwrap().clone();
    assert!(all.len() >= 5, "{}", all.len());
    let mut paged = Vec::new();
    let (mut after_ms, mut after_seq) = (i64::MIN, i64::MIN);
    loop {
        let uri = format!(
            "/api/cohorts/{}/events?after_ms={after_ms}&after_seq={after_seq}&limit=2",
            s.a
        );
        let (_, page) = api.call("GET", &uri, Some(s.amina), None).await;
        let page = page.as_array().unwrap().clone();
        if page.is_empty() {
            break;
        }
        assert!(page.len() <= 2);
        let last = page.last().unwrap();
        after_ms = last["event"]["at"].as_i64().unwrap();
        after_seq = last["seq"].as_i64().unwrap();
        paged.extend(page);
    }
    assert_eq!(paged, all);
}

#[tokio::test]
async fn teachers_get_the_studio_and_xapi() {
    let api = api(true).await;
    let s = api.db.seed;
    api.call(
        "POST",
        &format!("/api/cohorts/{}/commands", s.a),
        Some(s.amina),
        Some(json!([quiz_f2(0, 3)])),
    )
    .await;
    let (status, studio) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/studio?locale=en", s.a),
            Some(s.karim),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(studio["outcomes"][1]["achieved"], 1);
    assert_eq!(studio["outcomes"][1]["of"], 2);
    let (status, xapi) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/xapi", s.a),
            Some(s.karim),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(xapi.as_array().unwrap().len(), 1);
    assert_eq!(
        xapi[0]["object"]["id"],
        "urn:zidara:activity:equivalent-fractions/quiz-f2"
    );
}

#[tokio::test]
async fn the_agent_runs_after_an_answer() {
    let api = api(true).await;
    let s = api.db.seed;
    let answer = json!({ "id": Uuid::now_v7(), "at": NOW, "command": { "type": "submit_answer", "activity": "explain-f2", "answer": "it is the same" } });
    api.call(
        "POST",
        &format!("/api/cohorts/{}/commands", s.a),
        Some(s.amina),
        Some(json!([answer])),
    )
    .await;
    let studio = format!("/api/cohorts/{}/studio", s.a);
    for _ in 0..40 {
        let (_, view) = api.call("GET", &studio, Some(s.karim), None).await;
        if view["queue"].as_array().is_some_and(|q| !q.is_empty()) {
            assert_eq!(view["queue"][0]["answer"], "it is the same");
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("the agent never proposed a mark");
}

#[tokio::test]
async fn a_misconfigured_cohort_is_a_server_error_not_a_crash() {
    let api = api(true).await;
    let s = api.db.seed;
    sqlx::query("update cohorts set lesson_id = 'gone' where id = $1")
        .bind(s.a)
        .execute(&api.db.owner)
        .await
        .unwrap();
    let (status, body) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/learner-view", s.a),
            Some(s.amina),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["error"], "the cohort's lesson \"gone\" is not loaded");
}

#[tokio::test]
async fn teachers_see_their_roster_by_name_and_learners_cannot() {
    let api = api(true).await;
    let s = api.db.seed;
    let (status, roster) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/roster", s.a),
            Some(s.karim),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    let mut names: Vec<&str> = roster
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["display_name"].as_str().unwrap())
        .collect();
    names.sort();
    assert_eq!(names, ["Amina", "Yacine"]);
    let (status, _) = api
        .call(
            "GET",
            &format!("/api/cohorts/{}/roster", s.a),
            Some(s.amina),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}
