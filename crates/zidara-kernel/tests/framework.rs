#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use std::collections::BTreeMap;

use zidara_kernel::framework::{Judgement, OutcomeState, ProgressContext};
use zidara_kernel::ids::{OutcomeId, Timestamp};
use zidara_kernel::outcome::OutcomeGraph;

#[test]
fn progression_reads_judged_states_and_treats_the_rest_as_not_started() {
    let graph = OutcomeGraph::new(vec![]).unwrap();
    let mut judgements = BTreeMap::new();
    let achieved = Judgement {
        state: OutcomeState::Achieved,
        ..Judgement::NOT_STARTED
    };
    judgements.insert(OutcomeId::new("F1"), achieved);
    let ctx = ProgressContext {
        graph: &graph,
        judgements: &judgements,
        now: Timestamp(0),
    };
    assert_eq!(ctx.state_of(&OutcomeId::new("F1")), OutcomeState::Achieved);
    assert_eq!(
        ctx.state_of(&OutcomeId::new("F2")),
        OutcomeState::NotStarted
    );
}
