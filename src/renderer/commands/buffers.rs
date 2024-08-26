// dacho/src/renderer/commands/buffers.rs

// std
use std::collections::HashMap;

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// super
use super::{Command, CommandPool};

// crate
use crate::{
    renderer::{
        descriptors::DescriptorSet,
        devices::Device,
        presentation::Swapchain,
        rendering::{Pipeline, RenderPass}
    },
    create_log, fatal
};

pub struct CommandBuffers {
    pub raw: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub fn new(
       command_pool: &CommandPool,
       swapchain:    &Swapchain,
       device:       &Device
    ) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool.raw)
                .command_buffer_count(u32::try_from(swapchain.image_count)?);

            unsafe { device.raw.allocate_command_buffers(&allocate_info) }?
        };

        Ok(Self { raw })
    }

    #[allow(clippy::too_many_lines)]
    pub fn record(
        &self,
        device:         &Device,
        commands:       &[Command],
        render_pass:    &RenderPass,
        swapchain:      &Swapchain,
        pipelines:      &HashMap<String, Pipeline>,
        descriptor_set: &DescriptorSet
    ) -> Result<()> {
        for (i, &command_buffer) in self.raw.iter().enumerate() {
            {
                let begin_info = vk::CommandBufferBeginInfo::builder();

                unsafe { device.raw.begin_command_buffer(command_buffer, &begin_info) }?;
            }

            let mut last_pipeline: Option<&Pipeline> = None;

            for command in commands {
                match command {
                    Command::BeginRenderPass => {
                        let clear_values = [
                            vk::ClearValue {
                                color: vk::ClearColorValue {
                                    float32: [0.0, 0.0, 0.0, 1.0]
                                }
                            },
                            vk::ClearValue {
                                depth_stencil: vk::ClearDepthStencilValue {
                                    depth:   1.0,
                                    stencil: 0
                                }
                            }
                        ];

                        let begin_info = vk::RenderPassBeginInfo::builder()
                            .render_pass(render_pass.raw)
                            .framebuffer(swapchain.framebuffers[i])
                            .render_area(
                                vk::Rect2D::builder()
                                    .offset(
                                        vk::Offset2D::builder()
                                            .x(0)
                                            .y(0)
                                            .build()
                                    )
                                    .extent(swapchain.extent)
                                    .build()
                            )
                            .clear_values(&clear_values);

                        unsafe {
                            device.raw.cmd_begin_render_pass(
                                command_buffer,
                                &begin_info,
                                vk::SubpassContents::INLINE
                            );
                        }
                    },
                    Command::BindPipeline(name) => {
                        let Some(pipeline) = pipelines.get(name) else {
                            fatal!("Failed to get Pipeline {name}");
                        };

                        last_pipeline = Some(pipeline);

                        unsafe {
                            device.raw.cmd_bind_pipeline(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                pipeline.raw
                            );
                        }
                    },
                    Command::BindVertexBuffers(vertex_buffer, instance_buffer) => {
                        unsafe {
                            device.raw.cmd_bind_vertex_buffers(
                                command_buffer,
                                0, &[*vertex_buffer, *instance_buffer], &[0, 0]
                            );
                        }
                    },
                    Command::BindIndexBuffer(index_buffer) => {
                        unsafe {
                            device.raw.cmd_bind_index_buffer(
                                command_buffer,
                                *index_buffer, 0, vk::IndexType::UINT32
                            );
                        }
                    },
                    Command::BindDescriptorSets => {
                        unsafe {
                            device.raw.cmd_bind_descriptor_sets(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                last_pipeline.context("No last pipeline")?.layout,
                                0, &[descriptor_set.raw], &[]
                            );
                        }
                    },
                    Command::DrawIndexed(index_count, instance_count) => {
                        unsafe {
                            device.raw.cmd_draw_indexed(
                                command_buffer,
                                *index_count, *instance_count,
                                0, 0, 0
                            );
                        }
                    }
                }
            }

            unsafe {
                device.raw.cmd_end_render_pass(command_buffer);
                device.raw.end_command_buffer(command_buffer)?;
            }
        }

        Ok(())
    }
}

