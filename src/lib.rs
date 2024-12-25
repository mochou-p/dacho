// dacho/src/lib.rs

#![allow(clippy::multiple_crate_versions, reason = "outside dacho's power")]

pub use {
    dacho_app        as app,
    dacho_components as components,
    dacho_ecs        as ecs,
    dacho_types      as types,
    dacho_proc_macro as proc_macro
};

