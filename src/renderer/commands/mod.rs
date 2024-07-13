// dacho/src/renderer/commands/mod.rs

// modules
mod buffers;
mod pool;

#[allow(clippy::wildcard_imports)]
#[allow(unused_imports)]
pub(super) use {buffers::*, pool::*};

// super
use super::{
    buffers::Buffer,
    descriptors::DescriptorSet,
    presentation::Swapchain,
    rendering::{Pipeline, RenderPass}
};

pub enum Command<'a> {
    BeginRenderPass(&'a RenderPass, &'a Swapchain),
    BindPipeline(&'a Pipeline),
    BindVertexBuffers(&'a Buffer, &'a Buffer),
    BindIndexBuffer(&'a Buffer),
    BindDescriptorSets(&'a DescriptorSet),
    DrawIndexed(u32, u32)
}

