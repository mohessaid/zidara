//! The Zidara kernel for JavaScript: `Core` is the phone's kernel over a local log, with a
//! JSON contract; `Shell` exposes it through wasm-bindgen.

mod core;
mod shell;

pub use crate::core::Core;
pub use crate::shell::Shell;
