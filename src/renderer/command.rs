// dacho/src/renderer/command.rs

use anyhow::Result;

use ash::vk;

use super::{
    device::Device,
    descriptor::DescriptorSet,
    render_pass::RenderPass,
    swapchain::Swapchain,
    pipeline::Pipeline,
    buffer::Buffer
};

#[cfg(debug_assertions)]
use crate::application::logger::Logger;

pub enum Command<'a> {
    BeginRenderPass(&'a RenderPass, &'a Swapchain),
    BindPipeline(&'a Pipeline),
    BindVertexBuffers(&'a Buffer, &'a Buffer),
    BindIndexBuffer(&'a Buffer),
    BindDescriptorSets(&'a DescriptorSet),
    DrawIndexed(u32, u32)
}

pub struct CommandPool {
    pub raw: vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &Device) -> Result<Self> {
        #[cfg(debug_assertions)]
        Logger::info("Creating CommandPool");

        let raw = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.raw.create_command_pool(&create_info, None) }?
        };

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        #[cfg(debug_assertions)]
        Logger::info("Destroying CommandPool");

        unsafe { device.raw.destroy_command_pool(self.raw, None); }
    }
}

pub struct CommandBuffers {
    pub raw: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub fn new(
       command_pool: &CommandPool,
       swapchain:    &Swapchain,
       device:       &Device
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        Logger::info("Creating CommandBuffers");

        let raw = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool.raw)
                .command_buffer_count(swapchain.image_count as u32);

            unsafe { device.raw.allocate_command_buffers(&allocate_info) }?
        };

        Ok(Self { raw })
    }

    pub fn record(
        &self,
        device:   &Device,
        commands: &[Command]
    ) -> Result<()> {
        #[cfg(debug_assertions)] {
            Logger::info(format!("Recording commands ({} command buffers)", self.raw.len()));
            Logger::indent(1);
        }

        #[cfg(debug_assertions)]
        let mut draw_calls = 0;
        #[cfg(debug_assertions)]
        let mut binds      = 0;
        #[cfg(debug_assertions)]
        let mut first_command_buffer;
        #[cfg(debug_assertions)]
        let mut just_drew = false;

        for (i, &command_buffer) in self.raw.iter().enumerate() {
            #[cfg(debug_assertions)] {
                first_command_buffer = i == 0;
            }

            {
                let begin_info = vk::CommandBufferBeginInfo::builder();

                unsafe { device.raw.begin_command_buffer(command_buffer, &begin_info) }?;
            }

            let mut last_pipeline: Option<&Pipeline> = None;

            for command in commands.iter() {
                match command {
                    Command::BeginRenderPass(render_pass, swapchain) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            Logger::info("Beginning RenderPass");
                            Logger::indent(1);
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
                    Command::BindPipeline(pipeline) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            Logger::info(format!("Binding Pipeline `{}`", pipeline.name));
                            Logger::indent(1);

                            just_drew  = false;
                            binds     += 1
                        }

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
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            if just_drew {
                                Logger::indent(1);
                            }

                            Logger::info("Binding VertexBuffers");

                            binds += 1
                        }

                        let vertex_buffers = [
                            vertex_buffer.raw,
                            instance_buffer.raw
                        ];

                        let offsets = [0, 0];

                        unsafe {
                            device.raw.cmd_bind_vertex_buffers(
                                command_buffer,
                                0,
                                &vertex_buffers,
                                &offsets
                            );
                        }
                    },
                    Command::BindIndexBuffer(index_buffer) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            Logger::info("Binding IndexBuffer");

                            binds += 1
                        }

                        unsafe {
                            device.raw.cmd_bind_index_buffer(
                                command_buffer,
                                index_buffer.raw,
                                0,
                                vk::IndexType::UINT32
                            );
                        }
                    },
                    Command::BindDescriptorSets(descriptor_set) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            Logger::info("Binding DescriptoSet");

                            binds += 1
                        }

                        let descriptor_sets = [descriptor_set.raw];

                        unsafe {
                            device.raw.cmd_bind_descriptor_sets(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                last_pipeline.unwrap().layout,
                                0,
                                &descriptor_sets,
                                &[]
                            );
                        }
                    },
                    Command::DrawIndexed(index_count, instance_count) => {
                        #[cfg(debug_assertions)]
                        if first_command_buffer {
                            Logger::indent(-1);
                            Logger::info("Drawing");

                            just_drew   = true;
                            draw_calls += 1;
                        }

                        unsafe {
                            device.raw.cmd_draw_indexed(
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
                Logger::indent(-1);
                Logger::info("Ending RenderPass");
            }

            unsafe {
                device.raw.cmd_end_render_pass(command_buffer);
                device.raw.end_command_buffer(command_buffer)?;
            }
        }

        #[cfg(debug_assertions)] {
            Logger::indent(-1);
            Logger::info(format!("Recorded {draw_calls} draw calls and {binds} binds (per command buffer)"));
        }

        Ok(())
    }
}

