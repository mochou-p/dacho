// dacho/core/renderer/vulkan/backend/src/setup/instance.rs

use {
    anyhow::Result,
    ash::vk,
    raw_window_handle::HasRawDisplayHandle as _,
    winit::event_loop::ActiveEventLoop
};

use {
    super::Entry,
    crate::{create_log, destroy_log}
};

#[cfg(feature = "validation")]
use {
    core::{ffi::c_void, ptr::from_ref},
    alloc::ffi::CString,

    crate::log,

    dacho_vulkan_validation::messenger_create_info
};


#[cfg(feature = "validation")]
const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct Instance {
    pub raw: ash::Instance
}

impl Instance {
    pub fn new(
        event_loop: &ActiveEventLoop,
        entry:      &Entry
    ) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            #[cfg(feature = "validation")] {
                log!(debug, "Enabling Vulkan Validation Layers");

                let mut extension_names = required_extensions.to_vec();

                let debug     = CString::new("VK_EXT_debug_utils")?;
                let debug_ptr = debug.as_ptr();

                extension_names.push(debug_ptr);

                let debug_utils_create_info = messenger_create_info();

                let layers_raw: Vec<CString> = VALIDATION_LAYERS
                    .iter()
                    .map(|layer| CString::new(*layer).expect("CString error"))
                    .collect();

                let layer_names: Vec<*const i8> = layers_raw
                    .iter()
                    .map(|layer| layer.as_ptr())
                    .collect();

                let mut create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_layer_names(&layer_names)
                    .enabled_extension_names(&extension_names)
                    .build();

                create_info.p_next = from_ref::<vk::DebugUtilsMessengerCreateInfoEXT>(
                    &debug_utils_create_info
                ).cast::<c_void>();

                unsafe { entry.raw.create_instance(&create_info, None) }?
            }

            #[cfg(not(feature = "validation"))] {
                let create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_extension_names(required_extensions);

                unsafe { entry.raw.create_instance(&create_info, None) }?
            }
        };

        Ok(Self { raw })
    }

    pub fn drop(&self) {
        destroy_log!(debug);

        unsafe { self.raw.destroy_instance(None); }
    }
}

