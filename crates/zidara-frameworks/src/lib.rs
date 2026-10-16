//! Frameworks: pedagogies made explicit, each behind the kernel's `Framework` trait.
//! A new framework is a new file here and a new set of acceptance stories.

pub mod cambridge;
pub mod mastery;

use zidara_kernel::framework::{Framework, FrameworkRegistry};
use zidara_kernel::ids::FrameworkId;

#[derive(Debug)]
pub struct Registry {
    frameworks: Vec<Box<dyn Framework>>,
}

impl Registry {
    pub fn new(frameworks: Vec<Box<dyn Framework>>) -> Self {
        Self { frameworks }
    }
}

impl FrameworkRegistry for Registry {
    fn get(&self, id: &FrameworkId) -> Option<&dyn Framework> {
        self.frameworks
            .iter()
            .find(|f| f.id() == *id)
            .map(|f| f.as_ref())
    }
}

/// Monday 5 October 2026, 00:00 UTC: the demo term starts.
pub const TERM_START: i64 = 1_791_158_400_000;
/// One week in milliseconds.
pub const WEEK: i64 = 7 * 24 * 60 * 60 * 1000;

/// The demo school's frameworks, identical on server and phone: mastery, and Cambridge-style
/// with the scheme of work opening one outcome a week from the start of term.
pub fn demo_registry() -> Registry {
    let opens_at = [("F1", 0), ("F2", 1), ("F3", 2)]
        .into_iter()
        .map(|(o, week)| {
            (
                zidara_kernel::ids::OutcomeId::new(o),
                zidara_kernel::ids::Timestamp(TERM_START + week * WEEK),
            )
        })
        .collect();
    Registry::new(vec![
        Box::new(mastery::Mastery::standard()),
        Box::new(cambridge::CambridgeStyle::primary_checkpoint(opens_at)),
    ])
}
