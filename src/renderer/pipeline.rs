// dacho/src/renderer/pipeline.rs

use anyhow::Result;

use ash::vk;

use super::{
    descriptor::DescriptorSetLayout,
    instance::Instance,
    swapchain::Swapchain,
    vertex::Vertex
};

pub struct Pipeline {
    pub layout:   vk::PipelineLayout,
    pub pipeline: vk::Pipeline
}

impl Pipeline {
    pub fn new(
        device:                &ash::Device,
        descriptor_set_layout: &DescriptorSetLayout,
        swapchain:             &Swapchain,
        render_pass:           &vk::RenderPass
    ) -> Result<Self> {
        let layout = {
            let set_layouts = [
                descriptor_set_layout.descriptor_set_layout
            ];

            let create_info = {
                vk::PipelineLayoutCreateInfo::builder()
                    .set_layouts(&set_layouts)
            };

            unsafe { device.create_pipeline_layout(&create_info, None) }?
        };

        #[cfg(debug_assertions)]
        compile_shaders()?;

        let vert_module = {
            let code = read_spirv("assets/shaders/tile/bin/vert.spv")?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.create_shader_module(&create_info, None) }?
        };

        let frag_module = {
            let code = read_spirv("assets/shaders/tile/bin/frag.spv")?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.create_shader_module(&create_info, None) }?
        };

        let pipeline = {
            let entry_point = std::ffi::CString::new("main")?;

            let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_module)
                .name(&entry_point);

            let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_module)
                .name(&entry_point);

            let stages = vec![
                vert_stage.build(),
                frag_stage.build()
            ];

            let mut binding_descriptions = Vertex::binding_descriptions();
            binding_descriptions.extend(Instance::binding_descriptions());

            let mut attribute_descriptions = Vertex::attribute_descriptions();
            attribute_descriptions.extend(Instance::attribute_descriptions());

            let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
                .vertex_binding_descriptions(&binding_descriptions)
                .vertex_attribute_descriptions(&attribute_descriptions);

            let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

            let viewports = [
                vk::Viewport::builder()
                    .x(0.0)
                    .y(0.0)
                    .width(swapchain.extent.width as f32)
                    .height(swapchain.extent.height as f32)
                    .min_depth(0.0)
                    .max_depth(1.0)
                    .build()
            ];

            let scissors = [
                vk::Rect2D::builder()
                    .offset(
                        vk::Offset2D::builder()
                            .x(0)
                            .y(0)
                            .build()
                    )
                    .extent(swapchain.extent)
                    .build()
            ];

            let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
                .viewports(&viewports)
                .scissors(&scissors);

            let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
                .line_width(1.0)
                .front_face(vk::FrontFace::CLOCKWISE)
                .cull_mode(vk::CullModeFlags::BACK);

            let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1);

            let color_blend_attachments = [
                vk::PipelineColorBlendAttachmentState::builder()
                    .blend_enable(true)
                    .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                    .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                    .color_blend_op(vk::BlendOp::ADD)
                    .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
                    .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                    .alpha_blend_op(vk::BlendOp::ADD)
                    .color_write_mask(
                        vk::ColorComponentFlags::R |
                        vk::ColorComponentFlags::G |
                        vk::ColorComponentFlags::B |
                        vk::ColorComponentFlags::A
                    )
                    .build()
            ];

            let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
                .attachments(&color_blend_attachments); 

            let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
                .depth_test_enable(true)
                .depth_write_enable(true)
                .depth_compare_op(vk::CompareOp::LESS)
                .depth_bounds_test_enable(false)
                .min_depth_bounds(0.0)
                .max_depth_bounds(1.0)
                .stencil_test_enable(false);

            let pipeline_infos = [
                vk::GraphicsPipelineCreateInfo::builder()
                    .stages(&stages)
                    .vertex_input_state(&vertex_input_state)
                    .input_assembly_state(&input_assembly_state)
                    .viewport_state(&viewport_state)
                    .rasterization_state(&rasterization_state)
                    .multisample_state(&multisample_state)
                    .color_blend_state(&color_blend_state)
                    .depth_stencil_state(&depth_stencil_state)
                    .layout(layout)
                    .render_pass(*render_pass)
                    .subpass(0)
                    .build()
            ];

            unsafe {
                device.create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &pipeline_infos,
                    None
                )
            }.expect("Error creating pipelines")[0]
        };

        unsafe {
            device.destroy_shader_module(frag_module, None);
            device.destroy_shader_module(vert_module, None);
        }

        Ok(
            Self {
                layout,
                pipeline
            }
        )
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

#[cfg(debug_assertions)]
fn compile_shaders() -> Result<()> {
    let mut filepath = std::env::current_dir()?;
    filepath.push("compile_shaders.py");

    std::process::Command::new("python")
        .arg(
            filepath
                .display()
                .to_string()
        )
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

fn read_spirv(path: &str) -> Result<Vec<u32>> {
    let bytes = &std::fs::read(path)?;
    let words = bytemuck::cast_slice::<u8, u32>(bytes);

    Ok(words.to_vec())
}
