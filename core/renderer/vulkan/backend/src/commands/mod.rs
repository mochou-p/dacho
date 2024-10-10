// dacho/core/renderer/vulkan/backend/src/commands/mod.rs

mod buffers;
mod pool;

use ash::vk;

pub(super) use {buffers::*, pool::*};


pub enum Command {
    BeginRenderPass,
    BindPipeline(String),
    BindVertexBuffers(vk::Buffer, vk::Buffer),
    BindIndexBuffer(vk::Buffer),
    BindDescriptorSets,
    DrawIndexed(u32, u32)
}

