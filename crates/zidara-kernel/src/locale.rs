//! Text in several languages. Right-to-left is the renderer's job; the kernel keeps the words.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Text keyed by BCP 47 language tag (`ar`, `fr`, `en`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(transparent)]
pub struct Localized(pub BTreeMap<String, String>);

impl Localized {
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        Self(
            pairs
                .iter()
                .map(|(tag, text)| ((*tag).to_owned(), (*text).to_owned()))
                .collect(),
        )
    }

    /// The text in `locale`, else English, else any language, else empty.
    pub fn get(&self, locale: &str) -> &str {
        self.0
            .get(locale)
            .or_else(|| self.0.get("en"))
            .or_else(|| self.0.values().next())
            .map_or("", String::as_str)
    }
}
