#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use std::collections::BTreeMap;

use zidara_frameworks::{Registry, cambridge::CambridgeStyle, mastery::Mastery};
use zidara_kernel::framework::FrameworkRegistry;
use zidara_kernel::ids::FrameworkId;

#[test]
fn finds_frameworks_by_id() {
    let registry = Registry::new(vec![
        Box::new(Mastery::standard()),
        Box::new(CambridgeStyle::primary_checkpoint(BTreeMap::new())),
    ]);
    assert_eq!(
        registry.get(&FrameworkId::new("mastery")).unwrap().id(),
        FrameworkId::new("mastery")
    );
    assert_eq!(
        registry
            .get(&FrameworkId::new("cambridge-style"))
            .unwrap()
            .id(),
        FrameworkId::new("cambridge-style")
    );
    assert!(registry.get(&FrameworkId::new("ib")).is_none());
}

#[test]
fn the_demo_registry_has_both_frameworks_and_a_weekly_calendar() {
    use zidara_frameworks::{TERM_START, WEEK, demo_registry};
    assert_eq!(TERM_START, 1_791_158_400_000);
    assert_eq!(WEEK, 604_800_000);
    let registry = demo_registry();
    assert!(registry.get(&FrameworkId::new("mastery")).is_some());
    let cambridge = registry.get(&FrameworkId::new("cambridge-style")).unwrap();
    let o = |id: &str| zidara_kernel::outcome::Outcome {
        id: zidara_kernel::ids::OutcomeId::new(id),
        title: zidara_kernel::locale::Localized::default(),
        prerequisites: vec![],
    };
    let graph = zidara_kernel::outcome::OutcomeGraph::new(vec![o("F1"), o("F2"), o("F3")]).unwrap();
    let judgements = BTreeMap::new();
    let locks = |now| {
        let ctx = zidara_kernel::framework::ProgressContext {
            graph: &graph,
            judgements: &judgements,
            now: zidara_kernel::ids::Timestamp(now),
        };
        cambridge
            .availability(&ctx)
            .into_iter()
            .map(|a| a.lock.is_some())
            .collect::<Vec<_>>()
    };
    assert_eq!(locks(TERM_START), [false, true, true]);
    assert_eq!(locks(TERM_START + WEEK), [false, false, true]);
    assert_eq!(locks(TERM_START + 2 * WEEK), [false, false, false]);
}
