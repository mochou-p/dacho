// dacho/src/renderer/setup/mod.rs

// modules
#[cfg(debug_assertions)]
mod debug;
mod entry;
mod instance;

#[cfg(debug_assertions)]
#[allow(clippy::wildcard_imports)]
pub(super) use debug::*;
#[allow(clippy::wildcard_imports)]
pub(super) use {entry::*, instance::*};

