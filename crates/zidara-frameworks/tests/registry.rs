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
