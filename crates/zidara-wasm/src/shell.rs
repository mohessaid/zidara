//! The JavaScript face of `Core`. Every method forwards; errors become thrown strings.
//! Times cross as `f64` milliseconds (exact up to 2^53, past the year 285,000).

use wasm_bindgen::prelude::*;

use crate::core::Core;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Shell {
    core: Core,
}

#[wasm_bindgen]
impl Shell {
    /// `setup`: `{ lesson, framework, tenant, cohort }` as JSON.
    #[wasm_bindgen(constructor)]
    pub fn new(setup: &str) -> Result<Shell, JsError> {
        Ok(Shell {
            core: Core::new(setup).map_err(|e| JsError::new(&e))?,
        })
    }

    pub fn load(&mut self, events: &str) -> Result<(), JsError> {
        self.core.load(events).map_err(|e| JsError::new(&e))
    }

    pub fn events(&self) -> String {
        self.core.events()
    }

    /// Returns `{ status: "applied", events } | { status: "duplicate" } | { status: "refused", error }`.
    pub fn run(&mut self, actor: &str, envelope: &str) -> Result<String, JsError> {
        self.core.run(actor, envelope).map_err(|e| JsError::new(&e))
    }

    /// The log as `user` in `role` may read it.
    #[wasm_bindgen(js_name = visibleEvents)]
    pub fn visible_events(&self, user: &str, role: &str) -> Result<String, JsError> {
        self.core
            .visible_events(user, role)
            .map_err(|e| JsError::new(&e))
    }

    /// `[{ answer, assessment }]`: the rubric's mark for each open answer awaiting one.
    #[wasm_bindgen(js_name = rubricMarks)]
    pub fn rubric_marks(&self) -> String {
        self.core.rubric_marks()
    }

    /// The version of a quiz due for `user` (show that one; the kernel refuses others).
    #[wasm_bindgen(js_name = nextVariant)]
    pub fn next_variant(&self, user: &str, activity: &str) -> Result<u32, JsError> {
        self.core
            .next_variant(user, activity)
            .map_err(|e| JsError::new(&e))
    }

    /// Whether `response` answers one question: feedback at once, nothing recorded.
    pub fn check(
        &self,
        activity: &str,
        variant: u32,
        index: u32,
        response: &str,
    ) -> Result<bool, JsError> {
        self.core
            .check(activity, variant, index, response)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = learnerView)]
    pub fn learner_view(&self, user: &str, locale: &str, now: f64) -> Result<String, JsError> {
        self.core
            .learner_view(user, locale, now as i64)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = studioView)]
    pub fn studio_view(&self, roster: &str, locale: &str, now: f64) -> Result<String, JsError> {
        self.core
            .studio_view(roster, locale, now as i64)
            .map_err(|e| JsError::new(&e))
    }
}
