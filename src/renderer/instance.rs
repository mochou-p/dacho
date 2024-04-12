// dacho/src/renderer/instance.rs

use anyhow::Result;

use ash::vk;

use raw_window_handle::HasRawDisplayHandle;

use winit::event_loop::EventLoop;

#[cfg(debug_assertions)]
use super::debug::messenger_create_info;

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: [&'static str; 1] = [
    "VK_LAYER_KHRONOS_validation"
];

pub struct Instance {
    pub instance: ash::Instance
}

impl Instance {
    pub fn new(
        event_loop: &EventLoop<()>,
        entry:      &ash::Entry
    ) -> Result<Self> {
        let instance = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            #[cfg(debug_assertions)]
            {
                let mut extension_names = required_extensions.to_vec();

                let debug     = std::ffi::CString::new("VK_EXT_debug_utils")?;
                let debug_ptr = debug.as_ptr();

                extension_names.push(debug_ptr);

                let debug_utils_create_info = messenger_create_info();

                let layers_raw: Vec<std::ffi::CString> = VALIDATION_LAYERS
                    .iter()
                    .map(|layer| std::ffi::CString::new(*layer).unwrap())
                    .collect();

                let layer_names: Vec<*const i8> = layers_raw
                    .iter()
                    .map(|layer| layer.as_ptr())
                    .collect();

                let create_info = vk::InstanceCreateInfo {
                    s_type:  vk::StructureType::INSTANCE_CREATE_INFO,
                    p_next: &debug_utils_create_info
                        as *const vk::DebugUtilsMessengerCreateInfoEXT
                        as *const std::ffi::c_void,
                    flags:                       vk::InstanceCreateFlags::empty(),
                    p_application_info:         &application_info.build(),
                    pp_enabled_layer_names:      layer_names.as_ptr(),
                    enabled_layer_count:         layer_names.len() as u32,
                    pp_enabled_extension_names:  extension_names.as_ptr(),
                    enabled_extension_count:     extension_names.len() as u32
                };

                unsafe { entry.create_instance(&create_info, None) }?
            }

            #[cfg(not(debug_assertions))]
            {
                let create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_extension_names(required_extensions);

                unsafe { entry.create_instance(&create_info, None) }?
            }
        };

        Ok(
            Self {
                instance
            }
        )
    }

    pub fn destroy(&self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}

