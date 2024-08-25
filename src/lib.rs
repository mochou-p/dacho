// dacho/src/lib.rs

// -clippy::pedantic
#![allow(clippy::module_name_repetitions)]

// -clippy::cargo
#![allow(clippy::multiple_crate_versions)]

// modules
    mod app;
    mod ecs;
pub mod prelude;
    mod renderer;
    mod shader;

// debug
#[cfg(debug_assertions)]
use core::any::type_name;

#[cfg(debug_assertions)]
fn type_name_tail<T>() -> String {
    let type_name = type_name::<T>();
    let tail      = &type_name[type_name.rfind(':').expect("name error") + 1..];

    tail.to_owned()
}

fn path_to_log_source(path: &str) -> String {
    let pattern    = "src/";
    let src_path   = &path[path.rfind(pattern).expect("path error") + pattern.len()..];
    let src_crate  = &src_path[..src_path.find('/').expect("path error")];

    ["dacho", src_crate].join("::")
}

