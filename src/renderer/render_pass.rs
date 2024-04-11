// dacho/src/renderer/render_pass.rs

use anyhow::Result;

use ash::vk;

pub struct RenderPass {
    pub render_pass: vk::RenderPass
}

impl RenderPass {
    pub fn new(
        device: &ash::Device
    ) -> Result<Self> {
        let render_pass = {
            let attachments = [
                vk::AttachmentDescription::builder()
                    .format(vk::Format::B8G8R8A8_SRGB)
                    .load_op(vk::AttachmentLoadOp::CLEAR)
                    .store_op(vk::AttachmentStoreOp::STORE)
                    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .initial_layout(vk::ImageLayout::UNDEFINED)
                    .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .build(),
                vk::AttachmentDescription::builder()
                    .format(vk::Format::D32_SFLOAT_S8_UINT)
                    .load_op(vk::AttachmentLoadOp::CLEAR)
                    .store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .initial_layout(vk::ImageLayout::UNDEFINED)
                    .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .build()
            ];

            let color_attachments = [
                vk::AttachmentReference::builder()
                    .attachment(0)
                    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .build()
            ];

            let depth_attachment = vk::AttachmentReference::builder()
                .attachment(1)
                .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

            let subpasses = [
                vk::SubpassDescription::builder()
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                    .color_attachments(&color_attachments)
                    .depth_stencil_attachment(&depth_attachment)
                    .build()
            ];

            let subpass_dependencies = [
                vk::SubpassDependency::builder()
                    .src_subpass(vk::SUBPASS_EXTERNAL)
                    .src_stage_mask(
                        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT |
                            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    )
                    .dst_subpass(0)
                    .dst_stage_mask(
                        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT |
                            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
                    .dst_access_mask(
                        vk::AccessFlags::COLOR_ATTACHMENT_WRITE |
                            vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
                    )
                    .build()
            ];

            let create_info = vk::RenderPassCreateInfo::builder()
                    .attachments(&attachments)
                    .subpasses(&subpasses)
                    .dependencies(&subpass_dependencies);

            unsafe { device.create_render_pass(&create_info, None) }?
        };

        Ok(
            Self {
                render_pass
            }
        )
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe { device.destroy_render_pass(self.render_pass, None); }
    }
}

