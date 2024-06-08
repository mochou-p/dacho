// dacho/src/renderer/commands/buffers.rs

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// super
use super::{pool::*, *};

// crate
use crate::renderer::{
    devices::logical::*,
    presentation::swapchain::*,
    rendering::pipeline::*,
    VulkanObject
};

// debug
#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log, log_indent
};

pub struct CommandBuffers {
    raw: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub fn new(
       command_pool: &CommandPool,
       swapchain:    &Swapchain,
       device:       &Device
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating CommandBuffers");

        let raw = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(*command_pool.raw())
                .command_buffer_count(swapchain.image_count as u32);

            unsafe { device.raw().allocate_command_buffers(&allocate_info) }?
        };

        Ok(Self { raw })
    }

    pub fn record(
        &self,
        device:   &Device,
        commands: &[Command]
    ) -> Result<()> {
        #[cfg(debug_assertions)] {
            log!(info, "Recording commands ({} command buffers)", self.raw.len());
            log_indent!(1);
        }

        #[cfg(debug_assertions)]
        let mut draw_calls = 0;
        #[cfg(debug_assertions)]
        let mut binds      = 0;
        #[cfg(debug_assertions)]
        let mut first_command_buffer;
        #[cfg(debug_assertions)]
        let mut just_drew  = false;

        for (i, &command_buffer) in self.raw.iter().enumerate() {
            #[cfg(debug_assertions)] {
                first_command_buffer = i == 0;
            }

            {
                let begin_info = vk::CommandBufferBeginInfo::builder();

                unsafe { device.raw().begin_command_buffer(command_buffer, &begin_info) }?;
            }

            let mut last_pipeline: Option<&Pipeline> = None;

            for command in commands.iter() {
                match command {
                    Command::BeginRenderPass(render_pass, swapchain) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            log!(info, "Beginning RenderPass");
                            log_indent!(1);
                        }

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
                            .render_pass(*render_pass.raw())
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
                            device.raw().cmd_begin_render_pass(
                                command_buffer,
                                &begin_info,
                                vk::SubpassContents::INLINE
                            );
                        }
                    },
                    Command::BindPipeline(pipeline) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            log!(info, "Binding Pipeline `{}`", pipeline.name);
                            log_indent!(1);

                            just_drew  = false;
                            binds     += 1
                        }

                        last_pipeline = Some(pipeline);

                        unsafe {
                            device.raw().cmd_bind_pipeline(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                *pipeline.raw()
                            );
                        }
                    },
                    Command::BindVertexBuffers(vertex_buffer, instance_buffer) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            if just_drew {
                                log_indent!(1);
                            }

                            log!(info, "Binding VertexBuffers");

                            binds += 1
                        }

                        unsafe {
                            device.raw().cmd_bind_vertex_buffers(
                                command_buffer,
                                0,
                                &[*vertex_buffer.raw(), *instance_buffer.raw()],
                                &[0, 0]
                            );
                        }
                    },
                    Command::BindIndexBuffer(index_buffer) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            log!(info, "Binding IndexBuffer");

                            binds += 1
                        }

                        unsafe {
                            device.raw().cmd_bind_index_buffer(
                                command_buffer,
                                *index_buffer.raw(),
                                0,
                                vk::IndexType::UINT32
                            );
                        }
                    },
                    Command::BindDescriptorSets(descriptor_set) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            log!(info, "Binding DescriptoSet");

                            binds += 1
                        }

                        unsafe {
                            device.raw().cmd_bind_descriptor_sets(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                last_pipeline.context("No last pipeline")?.layout,
                                0,
                                &[*descriptor_set.raw()],
                                &[]
                            );
                        }
                    },
                    Command::DrawIndexed(index_count, instance_count) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            log_indent!(-1);
                            log!(info, "Drawing");

                            just_drew   = true;
                            draw_calls += 1;
                        }

                        unsafe {
                            device.raw().cmd_draw_indexed(
                                command_buffer,
                                *index_count,
                                *instance_count,
                                0,
                                0,
                                0
                            );
                        }
                    }
                }
            }

            #[cfg(debug_assertions)]
            if first_command_buffer {
                log_indent!(-1);
                log!(info, "Ending RenderPass");
            }

            unsafe {
                device.raw().cmd_end_render_pass(command_buffer);
                device.raw().end_command_buffer(command_buffer)?;
            }
        }

        #[cfg(debug_assertions)] {
            log_indent!(-1);
            log!(info, "Recorded {draw_calls} draw calls and {binds} binds (per command buffer)");
        }

        Ok(())
    }
}

impl VulkanObject for CommandBuffers {
    type RawType = Vec<vk::CommandBuffer>;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }
}

