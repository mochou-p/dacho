// dacho/core/renderer/vulkan/backend/src/images/sampler.rs

use {
    anyhow::Result,
    ash::vk
};

use crate::{
    app::logger::Logger,
    renderer::{
        devices::Device,
        VulkanObject
    },
    log
};


pub struct Sampler {
    raw: vk::Sampler
}

impl Sampler {
    pub fn new(device: &Device) -> Result<Self> {
        let create_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(4.0)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        let raw = unsafe { device.raw().create_sampler(&create_info, None) }?;

        Ok(Self { raw })
    }
}

impl VulkanObject for Sampler {
    type RawType = vk::Sampler;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, device: Option<&Device>) {
        if let Some(device) = device {
            unsafe { device.raw().destroy_sampler(self.raw, None); }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

