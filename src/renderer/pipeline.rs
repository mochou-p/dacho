// dacho/src/renderer/pipeline.rs

use std::io::Write;

use anyhow::{Context, Result, anyhow};

use ash::vk;

use super::{
    descriptor::DescriptorSetLayout,
    swapchain::Swapchain,
    vertex_input::{ShaderInfo, Type, instance_descriptions, str_to_type, vertex_descriptions}
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
        render_pass:           &vk::RenderPass,
        shader_info:           &ShaderInfo
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

        let modules = shader_modules(&shader_info.name, device)?;

        let pipeline = {
            let entry_point = std::ffi::CString::new("main")?;

            let mut stages = vec![];

            for (module, stage) in modules.iter() {
                let stage_ = vk::PipelineShaderStageCreateInfo::builder()
                    .stage(*stage)
                    .module(*module)
                    .name(&entry_point)
                    .build();

                stages.push(stage_);
            }

            let (vertex_binding, mut vertex_attributes, last_location) =
                vertex_descriptions(&shader_info.vertex_info);

            let (instance_binding, mut instance_attributes) =
                instance_descriptions(&shader_info.instance_info, last_location);

            let binding_descriptions = [vertex_binding, instance_binding];

            let mut attribute_descriptions = vec![];
            attribute_descriptions.append(&mut vertex_attributes);
            attribute_descriptions.append(&mut instance_attributes);

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
                .cull_mode(shader_info.cull_mode);

            let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(vk::SampleCountFlags::TYPE_8);

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
            }
                .expect("Error creating pipelines")[0]
        };

        for (module, _) in modules.iter() {
            unsafe { device.destroy_shader_module(*module, None); }
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

fn read_spirv(filename: String) -> Result<Vec<u32>> {
    let bytes = &std::fs::read(format!("assets/shaders/bin/{filename}"))?;
    let words = bytemuck::cast_slice::<u8, u32>(bytes);

    Ok(words.to_vec())
}

pub fn shader_input_types(
    filename: &String
) -> Result<(Vec<Type>, Vec<Type>)> {
    let bytes = &std::fs::read(format!("assets/shaders/{filename}/{filename}.vert"))?;
    let code  = std::str::from_utf8(bytes)?;

    let in_    = " in ";
    let in_len = in_.len();

    let mut   vertex_info = vec![];
    let mut instance_info = vec![];

    let (mut found_in, mut found_nl) = (false, false);

    for (i, line) in code.lines().enumerate() {
        if found_in && line == "" {
            if found_nl {
                break;
            }

            found_nl = true;
        }

        if let Some(left) = line.find(in_) {
            let var = line[left + in_len..].trim_start();

            if let Some(right) = var.find(" ") {
                let kind = str_to_type(&var[..right]);
                found_in = true;

                if found_nl {
                    instance_info.push(kind);
                } else {
                    vertex_info.push(kind);
                }
            } else {
                print!("      ");
                std::io::stdout().flush()?;

                return Err(
                    anyhow!(
                        format!(
                            "\x1b[31;1mFailed\x1b[0m to parse `{}.vert` at line {}",
                            filename,
                            i + 1
                        )
                    )
                );
            }
        }
    }

    Ok((vertex_info, instance_info))
}

fn str_to_stage(string: &str) -> vk::ShaderStageFlags {
    match string {
        "vert" => vk::ShaderStageFlags::VERTEX,
        "frag" => vk::ShaderStageFlags::FRAGMENT,
        _      => { panic!("Unknown shader stage '{string}'"); }
    }
}

fn shader_modules(
    name:   &String,
    device: &ash::Device
) -> Result<Vec<(vk::ShaderModule, vk::ShaderStageFlags)>> {
    let mut modules = vec![];

    let directory = std::fs::read_dir(format!("assets/shaders/{name}"))?;

    for entry in directory {
        let path = entry?.path();

        if path.is_file() {
            let mut stage_str = path
                .to_str()
                .context("Failed to convert PathBuf to &str")?;

            stage_str = &stage_str[
                stage_str
                    .rfind(".")
                    .context("Failed to parse shader filename")? + 1
                ..
            ];

            let stage = str_to_stage(stage_str);

            let module = {
                let code = read_spirv(format!("{name}.{stage_str}.spv"))?;

                let create_info = vk::ShaderModuleCreateInfo::builder()
                    .code(&code);

                unsafe { device.create_shader_module(&create_info, None) }?
            };

            modules.push((module, stage));
        }
    }

    Ok(modules)
}

