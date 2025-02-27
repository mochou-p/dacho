// dacho/core/renderer/vulkan/backend/src/descriptors/mod.rs

mod pool;
mod set;
mod set_layout;
mod uniform;

pub(super) use {pool::*, set::*, set_layout::*, uniform::*};

