// dacho/src/renderer/setup/debug.rs

// crates
use {
    anyhow::Result,
    ash::{extensions::ext, vk}
};

// super
use super::{Entry, Instance};

// crate
use crate::{
    game::logger::Logger,
    renderer::VulkanObject,
    log
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
        log!(info, "Creating Debug messenger for Validation Layers");

        let loader = ext::DebugUtils::new(entry.raw(), instance.raw());

        let messenger = {
            let create_info = messenger_create_info();

            unsafe { loader.create_debug_utils_messenger(&create_info, None) }?
        };

        Ok(Self { loader, messenger })
    }

    pub fn destroy(&self) {
        log!(info, "Destroying Debug messenger");

        unsafe { self.loader.destroy_debug_utils_messenger(self.messenger, None); }
    }
}

unsafe extern "system" fn validation_layers_callback(
    message_severity:        MessageSeverity,
    message_type:            MessageType,
    p_callback_data:  *const CallbackData,
    _p_user_data:     *mut   core::ffi::c_void
) -> vk::Bool32 {
    static mut NUMBER: usize = 0;
    NUMBER += 1;

    let severity = match message_severity {
        MessageSeverity::VERBOSE => "\x1b[36;1m[Verbose]",
        MessageSeverity::INFO    => "\x1b[36;1m[Info]",
        MessageSeverity::WARNING => "\x1b[33;1m[Warning]",
        MessageSeverity::ERROR   => "\x1b[31;1m[Error]",
        _                        => "\x1b[35;1m[Unknown]"
    };

    let kind = match message_type {
        MessageType::GENERAL     => "\x1b[36;1m[General]",
        MessageType::PERFORMANCE => "\x1b[33;1m[Performance]",
        MessageType::VALIDATION  => "\x1b[31;1m[Validation]",
        _                        => "\x1b[35;1m[Unknown]"
    };

    let message = core::ffi::CStr::from_ptr((*p_callback_data).p_message);

    #[allow(clippy::uninlined_format_args)] // false positive
    let mut msg = format!(
        "\n\x1b[33m({NUMBER}) \x1b[1m{kind} \x1b[1m{severity}\x1b[0m\n{:?}\n",
        message
    );

    if let Some(index) = msg.find("The Vulkan spec states:") {
        msg.insert_str(index, "\n\x1b[35;1m->\x1b[0m ");
    }

    println!("{msg}");

    vk::FALSE
}

pub fn messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: core::ptr::null(),
        flags:  vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity:
            MessageSeverity::WARNING |
            MessageSeverity::ERROR,
        message_type:
            MessageType::GENERAL     |
            MessageType::PERFORMANCE |
            MessageType::VALIDATION,
        pfn_user_callback: Some(validation_layers_callback),
        p_user_data:       core::ptr::null_mut()
    }
}

