//! The Claude client against a fake Messages API (wiremock). No database, so mutation
//! testing runs these on every mutant.

use std::time::Duration;

use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use zidara_kernel::governance::{Assessment, Criterion, MIXED, Rubric};
use zidara_kernel::score::Score;
use zidara_server::assessor::{Assessor, Claude, OpenItem};

fn rubric() -> Rubric {
    Rubric {
        criteria: vec![
            Criterion {
                any_of: vec!["multiply".into()],
                weight: 1,
            },
            Criterion {
                any_of: vec!["same".into()],
                weight: 1,
            },
        ],
        min_words: 3,
    }
}

fn item(rubric: &Rubric) -> OpenItem<'_> {
    OpenItem {
        prompt: "How do you make a fraction equal to 1/2?",
        rubric,
    }
}

fn reply(text: &str) -> Value {
    json!({
        "id": "msg_1", "type": "message", "role": "assistant", "model": "claude-opus-5-5",
        "content": [ { "type": "thinking", "thinking": "" }, { "type": "text", "text": text } ],
        "stop_reason": "end_turn"
    })
}

async fn claude(server: &MockServer, timeout_ms: u64) -> Assessor {
    Assessor::Claude(Claude::new(
        server.uri(),
        "test-key",
        "claude-opus-5-5",
        Duration::from_millis(timeout_ms),
    ))
}

#[tokio::test]
async fn claude_scores_are_parsed() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .and(header("anthropic-beta", "server-side-fallback-2026-07-01"))
        .respond_with(ResponseTemplate::new(200).set_body_json(reply(
            r#"{"score_bp":7500,"confidence_bp":9100,"rationale":"Mentions multiplying."}"#,
        )))
        .expect(1)
        .mount(&server)
        .await;
    let rubric = rubric();
    let got = claude(&server, 2_000)
        .await
        .assess(&item(&rubric), "you multiply both")
        .await;
    assert_eq!(
        got,
        Assessment {
            score: Score::of(7_500),
            confidence: Score::of(9_100),
            rationale: "Mentions multiplying.".into(),
            met: None,
        }
    );
    let sent: Value =
        serde_json::from_slice(&server.received_requests().await.unwrap()[0].body).unwrap();
    assert_eq!(sent["model"], "claude-opus-5-5");
    assert_eq!(sent["fallbacks"], "default");
    assert_eq!(sent["output_config"]["format"]["type"], "json_schema");
    assert!(
        sent["messages"][0]["content"]
            .as_str()
            .unwrap()
            .contains("you multiply both")
    );
}

async fn assert_falls_back(response: ResponseTemplate, timeout_ms: u64) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(response)
        .mount(&server)
        .await;
    let rubric = rubric();
    let answer = "you multiply the top and bottom by the same number";
    let got = claude(&server, timeout_ms)
        .await
        .assess(&item(&rubric), answer)
        .await;
    // The rubric's score, but never confident enough to be applied without a teacher.
    assert_eq!(got.score, Score::FULL);
    assert!(got.confidence <= MIXED, "{got:?}");
    assert!(
        got.rationale.starts_with("rubric fallback"),
        "{}",
        got.rationale
    );
    assert_eq!(
        got.met.map(|m| (m.criteria, m.of)),
        Some((2, 2)),
        "the fallback keeps the rubric's findings"
    );
}

#[tokio::test]
async fn a_failing_model_falls_back_and_queues() {
    assert_falls_back(ResponseTemplate::new(500), 2_000).await;
    assert_falls_back(
        ResponseTemplate::new(200).set_body_json(reply("not json")),
        2_000,
    )
    .await;
    assert_falls_back(
        ResponseTemplate::new(200).set_body_json(reply(
            r#"{"score_bp":12000,"confidence_bp":9000,"rationale":"x"}"#,
        )),
        2_000,
    )
    .await;
    let mut refusal = reply(r#"{"score_bp":1,"confidence_bp":1,"rationale":"x"}"#);
    refusal["stop_reason"] = json!("refusal");
    assert_falls_back(ResponseTemplate::new(200).set_body_json(refusal), 2_000).await;
    assert_falls_back(
        ResponseTemplate::new(200)
            .set_body_json(reply("{}"))
            .set_delay(Duration::from_millis(800)),
        100,
    )
    .await;
}

#[tokio::test]
async fn models_without_server_fallbacks_send_no_beta_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(reply(r#"{"score_bp":1,"confidence_bp":2,"rationale":"x"}"#)),
        )
        .mount(&server)
        .await;
    let haiku = Assessor::Claude(Claude::new(
        server.uri(),
        "k",
        "claude-haiku-5-5",
        Duration::from_secs(2),
    ));
    let rubric = rubric();
    haiku.assess(&item(&rubric), "a b c").await;
    let sent = &server.received_requests().await.unwrap()[0];
    assert!(sent.headers.get("anthropic-beta").is_none());
}
