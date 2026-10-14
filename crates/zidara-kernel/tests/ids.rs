#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::ids::*;

#[test]
fn text_ids_keep_their_text() {
    assert_eq!(OutcomeId::new("F2").as_str(), "F2");
    assert_eq!(ActivityId::new("quiz-f1").as_str(), "quiz-f1");
    assert_eq!(AgentId::new("assessor").as_str(), "assessor");
    assert_eq!(FrameworkId::new("mastery").as_str(), "mastery");
}

#[test]
fn sequential_ids_count_from_one() {
    let mut ids = SequentialIds::default();
    assert_eq!(ids.next_id(), EventId(Uuid::from_u128(1)));
    assert_eq!(ids.next_id(), EventId(Uuid::from_u128(2)));
}

#[test]
fn ids_and_time_serialize_transparently() {
    assert_eq!(
        serde_json::to_string(&OutcomeId::new("F2")).unwrap(),
        "\"F2\""
    );
    assert_eq!(serde_json::to_string(&Timestamp(5)).unwrap(), "5");
    let user = UserId(Uuid::from_u128(1));
    assert_eq!(
        serde_json::to_string(&user).unwrap(),
        "\"00000000-0000-0000-0000-000000000001\""
    );
}
