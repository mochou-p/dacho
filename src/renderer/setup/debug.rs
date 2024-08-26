// dacho/src/renderer/setup/debug.rs

// core
use core::{
    ffi::{c_void, CStr},
    ptr::{null, null_mut}
};

// crates
use {
    anyhow::Result,
    ash::{extensions::ext, vk}
};

// super
use super::{Entry, Instance};

// crate
use crate::{
    renderer::VulkanObject,
    log_from, create_log, destroy_log
};

type MessageSeverity = vk::DebugUtilsMessageSeverityFlagsEXT;
type MessageType     = vk::DebugUtilsMessageTypeFlagsEXT;
type CallbackData    = vk::DebugUtilsMessengerCallbackDataEXT;

pub struct Debug {
    loader:    ext::DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT
}

impl Debug {
    pub fn new(
        entry:    &Entry,
        instance: &Instance
    ) -> Result<Self> {
        create_log!(debug);

        let loader = ext::DebugUtils::new(entry.raw(), instance.raw());

        let messenger = {
            let create_info = messenger_create_info();

            unsafe { loader.create_debug_utils_messenger(&create_info, None) }?
        };

        Ok(Self { loader, messenger })
    }

    pub fn destroy(&self) {
        destroy_log!(debug);

        unsafe { self.loader.destroy_debug_utils_messenger(self.messenger, None); }
    }
}

unsafe extern "system" fn validation_layers_callback(
    message_severity:        MessageSeverity,
    message_type:            MessageType,
    p_callback_data:  *const CallbackData,
    _p_user_data:     *mut   c_void
) -> vk::Bool32 {
    let source = match message_type {
        MessageType::GENERAL                => "vulkan::general",
        MessageType::PERFORMANCE            => "vulkan::performance",
        MessageType::VALIDATION             => "vulkan::validation",
        MessageType::DEVICE_ADDRESS_BINDING => "vulkan::DAB",
        _                                   => "vulkan::unknown"
    };

    let message = CStr::from_ptr((*p_callback_data).p_message);

    match message_severity {
        MessageSeverity::VERBOSE => { log_from!(debug,   source, "{:?}", message); },
        MessageSeverity::INFO    => { log_from!(info,    source, "{:?}", message); },
        MessageSeverity::WARNING => { log_from!(warning, source, "{:?}", message); },
        _                        => { log_from!(error,   source, "{:?}", message); }
    };

    vk::FALSE
}

pub fn messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: null(),
        flags:  vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity:
            MessageSeverity::WARNING |
            MessageSeverity::ERROR,
        message_type:
            MessageType::GENERAL     |
            MessageType::PERFORMANCE |
            MessageType::VALIDATION,
        pfn_user_callback: Some(validation_layers_callback),
        p_user_data:       null_mut()
    }
}

