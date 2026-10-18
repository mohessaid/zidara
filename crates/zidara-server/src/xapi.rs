//! Evidence as xAPI statements, so any learning record store can read Zidara's log.

use serde_json::{Value, json};
use zidara_kernel::event::{Event, EventBody};
use zidara_kernel::score::Score;

/// One statement per piece of evidence; `None` for every other event.
pub fn statement(event: &Event, lesson: &str) -> Option<Value> {
    let EventBody::EvidenceRecorded { evidence, .. } = &event.body else {
        return None;
    };
    let raw = evidence.score.bp();
    Some(json!({
        "id": event.id.0.to_string(),
        "actor": { "objectType": "Agent", "account": { "homePage": "urn:zidara", "name": evidence.learner.0.to_string() } },
        "verb": { "id": "http://adlnet.gov/expapi/verbs/scored", "display": { "en-US": "scored" } },
        "object": { "objectType": "Activity", "id": format!("urn:zidara:activity:{lesson}/{}", evidence.activity.as_str()) },
        "result": { "score": { "scaled": f64::from(raw) / f64::from(Score::MAX_BP), "raw": raw, "min": 0, "max": Score::MAX_BP } },
        "context": { "extensions": {
            "urn:zidara:outcome": evidence.outcome.as_str(),
            "urn:zidara:kind": evidence.kind,
            "urn:zidara:recorded-by": event.actor,
        } },
        "timestamp": rfc3339(event.at.0),
    }))
}

/// Milliseconds since the Unix epoch as `YYYY-MM-DDTHH:MM:SS.mmmZ`.
/// Days to a civil date follows Howard Hinnant's `civil_from_days`.
pub fn rfc3339(ms: i64) -> String {
    let (days, ms_of_day) = (ms.div_euclid(86_400_000), ms.rem_euclid(86_400_000));
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    let (h, m, s, milli) = (
        ms_of_day / 3_600_000,
        ms_of_day / 60_000 % 60,
        ms_of_day / 1_000 % 60,
        ms_of_day % 1_000,
    );
    format!("{year:04}-{month:02}-{day:02}T{h:02}:{m:02}:{s:02}.{milli:03}Z")
}
