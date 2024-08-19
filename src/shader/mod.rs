// dacho/src/shader/mod.rs

// modules
mod compilation;
mod input;

#[allow(clippy::wildcard_imports)]
pub use {compilation::*, input::*};

const LOG_SRC: &str = "dacho::shader";

