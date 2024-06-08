// dacho/src/renderer/rendering/pipeline.rs

use {
    anyhow::Result,
    ash::vk,
    futures::executor::block_on
};

use {
    super::render_pass::*,
    crate::{
        application::logger::Logger,
        renderer::{
            descriptors::set_layout::*,
            devices::logical::*,
            presentation::swapchain::*,
            VulkanObject
        },
        shader::{compilation::*, input::*},
        log
    }
};

#[cfg(debug_assertions)]
use crate::log_indent;

pub struct Pipeline {
        raw:    vk::Pipeline,
    pub name:   String,
    pub layout: vk::PipelineLayout
}

impl Pipeline {
    pub fn new(
        device:                &Device,
        descriptor_set_layout: &DescriptorSetLayout,
        swapchain:             &Swapchain,
        render_pass:           &RenderPass,
        shader_info:           &ShaderInfo
    ) -> Result<Self> {
        #[cfg(debug_assertions)] {
            log!(info, "Creating Pipeline `{}`", shader_info.name);
            log_indent!(1);
        }

        let layout = {
            let set_layouts = [*descriptor_set_layout.raw()];

            let create_info = {
                vk::PipelineLayoutCreateInfo::builder()
                    .set_layouts(&set_layouts)
            };

            unsafe { device.raw().create_pipeline_layout(&create_info, None) }?
        };

        let module = {
            let code = read_spirv(&shader_info.name)?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.raw().create_shader_module(&create_info, None) }?
        };

        let raw = {
            let vert_entry = std::ffi::CString::new("vertex")?;
            let frag_entry = std::ffi::CString::new("fragment")?;

            let stages = [
                vk::PipelineShaderStageCreateInfo::builder()
                    .stage(vk::ShaderStageFlags::VERTEX)
                    .module(module)
                    .name(&vert_entry)
                    .build(),
                vk::PipelineShaderStageCreateInfo::builder()
                    .stage(vk::ShaderStageFlags::FRAGMENT)
                    .module(module)
                    .name(&frag_entry)
                    .build()
            ];

            let (vertex_binding, mut vertex_attributes, last_location) =
                vertex_descriptions(&shader_info.vertex_info);

            let (instance_binding, mut instance_attributes) =
                instance_descriptions(&shader_info.instance_info, last_location);

            let binding_descriptions = [vertex_binding, instance_binding];

            let mut attribute_descriptions = Vec::with_capacity(vertex_attributes.len() + instance_attributes.len());
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
                .polygon_mode(shader_info.polygon_mode)
                .cull_mode(shader_info.cull_mode);

            let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(vk::SampleCountFlags::TYPE_8)
                .sample_shading_enable(true)
                .min_sample_shading(0.2);

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

            let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
                .stages(&stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .color_blend_state(&color_blend_state)
                .depth_stencil_state(&depth_stencil_state)
                .layout(layout)
                .render_pass(*render_pass.raw())
                .subpass(0);

            unsafe {
                device.raw().create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[*pipeline_info],
                    None
                )
            }
                .expect("Error creating pipelines")[0]
        };

        unsafe { device.raw().destroy_shader_module(module, None); }

        #[cfg(debug_assertions)]
        log_indent!(-1);

        let name = shader_info.name.clone();

        Ok(Self { name, layout, raw })
    }
}

impl VulkanObject for Pipeline {
    type RawType = vk::Pipeline;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, device: Option<&Device>) {
        if let Some(device) = device {
            unsafe {
                device.raw().destroy_pipeline(self.raw, None);
                device.raw().destroy_pipeline_layout(self.layout, None);
            }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

fn read_spirv(filename: &str) -> Result<Vec<u32>> {
    #[cfg(debug_assertions)]
    log!(info, "Reading `{filename}` SPIR-V");

    let spv = &format!("assets/.cache/shaders.{filename}.wgsl.spv");

    let read = std::fs::read(spv);

    let bytes = match read {
        Ok(_) => { read? },
        Err(_) => {
            if !std::path::Path::new(&format!("assets/shaders/{filename}.wgsl")).exists() {
                log!(panic, "Shader `{filename}.wgsl` not found");
            }

            block_on(compile_shaders())?;

            std::fs::read(spv)?
        }
    };

    let words = unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };

    Ok(words.to_vec())
}

#[derive(Default, PartialEq)]
enum ParseState {
    #[default]
    Searching,
    Vertex,
    Instance,
    Finished
}

pub fn shader_input_types(
    filename: &String
) -> Result<(Vec<Type>, Vec<Type>)> {
    #[cfg(debug_assertions)]
    log!(info, "Parsing `{filename}` for input types");

    let bytes = &std::fs::read(format!("assets/shaders/{filename}.wgsl"))?;
    let code  = std::str::from_utf8(bytes)?;

    let (mut vertex_types, mut instance_types) = (vec![], vec![]);

    let mut parse_state = ParseState::default();

    for line in code.lines() {
        match parse_state {
            ParseState::Searching => {
                if line == "struct VertexInput {" {
                    parse_state = ParseState::Vertex;
                }
            },
            ParseState::Vertex => {
                if line.is_empty() {
                    parse_state = ParseState::Instance;
                } else {
                    vertex_types.push(wgsl_field_to_type(line)?);
                }
            },
            ParseState::Instance => {
                if line == "}" {
                    parse_state = ParseState::Finished;
                } else {
                    instance_types.push(wgsl_field_to_type(line)?);
                }
            },
            ParseState::Finished => {
                break;
            }
        }
    }

    if parse_state != ParseState::Finished {
        log!(panic, "Failed to parse `{filename}` for input types");
    }

    Ok((vertex_types, instance_types))
}

