// dacho/core/renderer/vulkan/backend/src/descriptors/mod.rs

// modules
mod pool;
mod set;
mod set_layout;
mod uniform;

#[allow(clippy::wildcard_imports)]
pub(super) use {pool::*, set::*, set_layout::*, uniform::*};

