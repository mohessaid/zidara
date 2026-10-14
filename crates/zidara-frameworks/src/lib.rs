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
