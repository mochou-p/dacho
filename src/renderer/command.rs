// dacho/src/renderer/command.rs

use anyhow::Result;

use ash::vk;

use super::{
    render_pass::RenderPass,
    swapchain::Swapchain,
    pipeline::Pipeline,
    buffer::Buffer
};

pub enum Command<'a> {
    BeginRenderPass(&'a RenderPass, &'a Swapchain),
    BindPipeline(&'a Pipeline),
    BindVertexBuffers(&'a Buffer, &'a Buffer),
    BindIndexBuffer(&'a Buffer),
    BindDescriptorSets(&'a vk::DescriptorSet),
    DrawIndexed(u32, u32)
}

pub struct CommandPool {
    pub command_pool: vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &ash::Device) -> Result<Self> {
        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.create_command_pool(&create_info, None) }?
        };

        Ok(
            Self {
                command_pool
            }
        )
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}

pub struct CommandBuffers {
    pub command_buffers: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub fn new(
       command_pool: &vk::CommandPool,
       swapchain:    &Swapchain,
       device:       &ash::Device
    ) -> Result<Self> {
        let command_buffers = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(*command_pool)
                .command_buffer_count(swapchain.image_count as u32);

            unsafe { device.allocate_command_buffers(&allocate_info) }?
        };

        Ok(
            Self {
                command_buffers
            }
        )
    }

    pub fn record(
        &self,
        device:   &ash::Device,
        commands: &Vec<Command>
    ) -> Result<()> {
        for (i, &command_buffer) in self.command_buffers.iter().enumerate() {
            {
                let begin_info = vk::CommandBufferBeginInfo::builder();

                unsafe { device.begin_command_buffer(command_buffer, &begin_info) }?;
            }

            let mut last_pipeline: Option<&Pipeline> = None;

            for command in commands.iter() {
                match command {
                    Command::BeginRenderPass(render_pass, swapchain) => {
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
                            .render_pass(render_pass.render_pass)
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
                            device.cmd_begin_render_pass(
                                command_buffer,
                                &begin_info,
                                vk::SubpassContents::INLINE
                            );
                        }
                    },
                    Command::BindPipeline(pipeline) => {
                        last_pipeline = Some(&pipeline);

                        unsafe {
                            device.cmd_bind_pipeline(
                                command_buffer,
                                vk::PipelineBindPoint::GRAPHICS,
                                pipeline.pipeline
                            );
                        }
                    },
                    Command::BindVertexBuffers(vertex_buffer, instance_buffer) => {
                        let vertex_buffers = [
                            vertex_buffer.buffer,
                            instance_buffer.buffer
                        ];

                        let offsets = [0, 0];

                        unsafe {
                            device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
                        }
                    },
                    Command::BindIndexBuffer(index_buffer) => {
                        unsafe {
                            device.cmd_bind_index_buffer(command_buffer, index_buffer.buffer, 0, vk::IndexType::UINT16);
                        }
                    },
                    Command::BindDescriptorSets(descriptor_set) => {
                        let descriptor_sets = [**descriptor_set];

                        unsafe {
                            device.cmd_bind_descriptor_sets(
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
                        unsafe {
                            device.cmd_draw_indexed(
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

            unsafe {
                device.cmd_end_render_pass(command_buffer);
                device.end_command_buffer(command_buffer)?;
            }
        }

        Ok(())
    }
}

