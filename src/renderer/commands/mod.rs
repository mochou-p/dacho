// dacho/src/renderer/commands/mod.rs

pub mod buffers;
pub mod pool;

use super::{
    buffers::*,
    descriptors::set::*,
    presentation::swapchain::*,
    rendering::{pipeline::*, render_pass::*}
};

pub enum Command<'a> {
    BeginRenderPass(&'a RenderPass, &'a Swapchain),
    BindPipeline(&'a Pipeline),
    BindVertexBuffers(&'a Buffer, &'a Buffer),
    BindIndexBuffer(&'a Buffer),
    BindDescriptorSets(&'a DescriptorSet),
    DrawIndexed(u32, u32)
}

