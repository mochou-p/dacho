// dacho/src/renderer/instance.rs

use {
    anyhow::Result,
    ash::vk,
    raw_window_handle::HasRawDisplayHandle,
    winit::event_loop::EventLoop
};

#[cfg(debug_assertions)]
use {
    super::debug::messenger_create_info,
    crate::application::logger::Logger
};

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: [&str; 1] = [
    "VK_LAYER_KHRONOS_validation"
];

pub struct Instance {
    pub raw: ash::Instance
}

impl Instance {
    pub fn new(
        event_loop: &EventLoop<()>,
        entry:      &ash::Entry
    ) -> Result<Self> {
        #[cfg(debug_assertions)] {
            Logger::info("Creating Instance");
            Logger::indent(1);
        }

        let raw = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            #[cfg(debug_assertions)] {
                Logger::info("Enabling Validation Layers");

                let mut extension_names = required_extensions.to_vec();

                let debug     = std::ffi::CString::new("VK_EXT_debug_utils")?;
                let debug_ptr = debug.as_ptr();

                extension_names.push(debug_ptr);

                let debug_utils_create_info = messenger_create_info();

                let layers_raw: Vec<std::ffi::CString> = VALIDATION_LAYERS
                    .iter()
                    .map(|layer| std::ffi::CString::new(*layer).expect("CString error"))
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

                create_info.p_next = &debug_utils_create_info
                    as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const std::ffi::c_void;

                unsafe { entry.create_instance(&create_info, None) }?
            }

            #[cfg(not(debug_assertions))] {
                let create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_extension_names(required_extensions);

                unsafe { entry.create_instance(&create_info, None) }?
            }
        };

        #[cfg(debug_assertions)]
        Logger::indent(-1);

        Ok(Self { raw })
    }

    pub fn destroy(&self) {
        #[cfg(debug_assertions)]
        Logger::info("Destroying Instance");

        unsafe { self.raw.destroy_instance(None); }
    }
}

