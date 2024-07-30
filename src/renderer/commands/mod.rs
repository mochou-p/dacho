// dacho/src/renderer/commands/mod.rs

// modules
mod buffers;
mod pool;

#[allow(clippy::wildcard_imports)]
pub(super) use {buffers::*, pool::*};

// crates
use ash::vk;

// super
use super::{
    descriptors::DescriptorSet,
    presentation::Swapchain,
    rendering::{Pipeline, RenderPass}
};

pub enum Command {
    BeginRenderPass,
    BindPipeline(String),
    BindVertexBuffers(vk::Buffer, vk::Buffer),
    BindIndexBuffer(vk::Buffer),
    BindDescriptorSets,
    DrawIndexed(u32, u32)
}

