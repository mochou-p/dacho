// dacho/core/renderer/src/lib.rs

#[cfg(not(feature = "vulkan"))]
compile_error!("graphics backend missing, turn on the `vulkan` feature");

#[cfg(all(feature = "vulkan-validation", not(feature = "vulkan")))]
compile_error!("the `vulkan` feature is required for `vulkan-validation`");

#[cfg(feature = "vulkan")]
pub use dacho_vulkan::*;

