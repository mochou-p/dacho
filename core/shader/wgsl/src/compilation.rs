// dacho/core/shader/wgsl/src/compilation.rs

use {
    core::str::from_utf8,
    std::{fs::{create_dir_all, read, read_dir, write}, path::Path}
};

use {
    anyhow::{Context as _, Result},
    futures::future::join_all,
    naga::{back::spv::{Options as SpvOptions, write_vec}, front::wgsl::Frontend, valid::{Capabilities, ValidationFlags, Validator}},
    tokio::spawn
};

use dacho_log::{log, log_from, fatal};


#[expect(clippy::allow_attributes, reason = "same error logging code for all builds")]
fn compile_shader(filepath: &Path) -> Result<()> {
    let wgsl_in  = &format!("{}", filepath.display());
    let filename = &wgsl_in[wgsl_in.rfind('/').context("Error parsing shader path")?+1..];
    let spv_out  = &format!("target/dacho/shaders/{filename}.spv");

    let options = SpvOptions {
        lang_version: (1, 5),
        ..Default::default()
    };

    let bytes_in = &read(wgsl_in)?;
    let code     = from_utf8(bytes_in)?;
    let module   = Frontend::new().parse(code);

    #[allow(clippy::blocks_in_conditions, clippy::used_underscore_binding, reason = "logger cfg")]
    if module.clone().map_err(|_error| { log_from!(error, "naga", "`{wgsl_in}`: {_error}"); }).is_err() {
        fatal!("Some shaders failed naga parsing");
    }

    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module.clone()?);

    #[allow(clippy::blocks_in_conditions, clippy::used_underscore_binding, reason = "logger cfg")]
    if info.clone().map_err(|_error| { log_from!(error, "naga", "`{wgsl_in}`: {_error}"); }).is_err() {
        fatal!("Some shaders failed naga validation");
    }

    let words = write_vec(&module?, &info?, &options, None)?;

    let bytes_out: Vec<u8> = words.iter().flat_map(|word| word.to_ne_bytes().to_vec()).collect();

    create_dir_all("target/dacho/shaders")?;

    write(spv_out, bytes_out)?;

    log!(debug, "Compiled `{filename}`");

    Ok(())
}

pub async fn compile_shaders() -> Result<()> {
    log!(info, "Compiling shaders");

    let mut futures = vec![];

    for shader in read_dir("assets/shaders").expect("please move your shaders to `assets/shaders/*.wgsl") {
        let path = shader?.path();

        futures.push(spawn(async move { compile_shader(&path) }));
    }

    let results = join_all(futures).await;

    let mut error_count = 0_usize;

    for result in results {
        if result.is_err() {
            error_count += 1;
        }
    }

    if error_count != 0 {
        fatal!("Failed to compile all shaders");
    }

    Ok(())
}

