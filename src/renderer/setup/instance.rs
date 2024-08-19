// dacho/src/renderer/setup/instance.rs

// crates
use {
    anyhow::Result,
    ash::vk,
    raw_window_handle::HasRawDisplayHandle,
    winit::event_loop::ActiveEventLoop
};

// super
use super::Entry;

// crate
use crate::{
    renderer::{VulkanObject, LOG_SRC},
    debug
};

// debug
#[cfg(debug_assertions)]
use {
    core::{
        ffi::c_void,
        ptr::from_ref
    },
    std::ffi::CString,
    super::debug::messenger_create_info
};

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub struct Instance {
    raw: ash::Instance
}

impl Instance {
    pub fn new(
        event_loop: &ActiveEventLoop,
        entry:      &Entry
    ) -> Result<Self> {
        debug!(LOG_SRC, "Creating Instance");

        let raw = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            #[cfg(debug_assertions)] {
                debug!(LOG_SRC, "Adding Vulkan Validation Layers to Instance");

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

                unsafe { entry.raw().create_instance(&create_info, None) }?
            }

            #[cfg(not(debug_assertions))] {
                let create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_extension_names(required_extensions);

                unsafe { entry.raw().create_instance(&create_info, None) }?
            }
        };

        Ok(Self { raw })
    }
}

impl VulkanObject for Instance {
    type RawType = ash::Instance;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self) {
        debug!(LOG_SRC, "Destroying Instance");

        unsafe { self.raw.destroy_instance(None); }
    }
}

