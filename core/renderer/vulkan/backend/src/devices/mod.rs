// dacho/core/renderer/vulkan/backend/src/devices/mod.rs

// modules
mod logical;
mod physical;

#[allow(clippy::wildcard_imports)]
pub(super) use {logical::*, physical::*};

