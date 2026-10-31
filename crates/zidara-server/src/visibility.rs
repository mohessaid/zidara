//! Which learner an event is about. The database shows learners only their own events and
//! cohort-wide ones, so every stored event carries this answer in `learner_id`. The rule lives
//! in the kernel, so shells without a database apply the same one.

pub use zidara_kernel::visibility::learner_of;
