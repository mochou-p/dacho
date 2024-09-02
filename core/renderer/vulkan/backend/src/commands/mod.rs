// dacho/core/renderer/vulkan/backend/src/commands/mod.rs

// modules
mod buffers;
mod pool;

#[allow(clippy::wildcard_imports)]
pub(super) use {buffers::*, pool::*};

// crates
use ash::vk;

pub enum Command {
    BeginRenderPass,
    BindPipeline(String),
    BindVertexBuffers(vk::Buffer, vk::Buffer),
    BindIndexBuffer(vk::Buffer),
    BindDescriptorSets,
    DrawIndexed(u32, u32)
}

