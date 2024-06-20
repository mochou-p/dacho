// dacho/src/lib.rs

#![allow(clippy::cast_precision_loss)]     // mantissa
#![allow(clippy::module_name_repetitions)] // struct names similar/matching mod name

#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::struct_field_names)]

// modules
    mod application;
pub mod prelude;
    mod renderer;
    mod shader;

