// dacho/core/shader/src/lib.rs

#[cfg(not(feature = "wgsl"))]
compile_error!("shader language missing, turn on the `wgsl` feature");

#[cfg(feature = "wgsl")]
pub use dacho_wgsl::*;

