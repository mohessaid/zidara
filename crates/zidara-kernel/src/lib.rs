//! The Zidara kernel: the six building blocks of a pedagogy, as pure functions.
//!
//! No I/O, no clock, no randomness. Shells pass time and ids in, and store what comes out.
//! Outcomes, activities, evidence, judgement, progression and roles live here; frameworks
//! implement [`framework::Framework`]; [`decide::decide`] changes things and
//! [`state::fold`] and [`view`] read them.

pub mod activity;
pub mod decide;
pub mod event;
pub mod evidence;
pub mod framework;
pub mod governance;
pub mod ids;
pub mod lesson;
pub mod locale;
pub mod outcome;
pub mod roles;
pub mod rules;
pub mod score;
pub mod state;
pub mod view;
pub mod visibility;
