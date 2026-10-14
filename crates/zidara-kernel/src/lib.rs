//! The Zidara kernel: the six building blocks of a pedagogy, as pure functions.
//!
//! No I/O, no clock, no randomness. Shells pass time and ids in, and store what comes out.

pub mod ids;
pub mod locale;
pub mod outcome;
pub mod score;
