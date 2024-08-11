// dacho/src/shader/compilation.rs

// core
use core::str::from_utf8;

// std
use std::{
    fs::{create_dir_all, read, read_dir, write},
    path::Path
};

// crates
use {
    anyhow::{Context, Result, bail},
    futures::future::join_all,
    naga::{
        back::spv::{Options as SpvOptions, write_vec},
        front::wgsl::Frontend,
        valid::{Capabilities, ValidationFlags, Validator}
    },
    tokio::spawn
};

// crate
use crate::{
    app::logger::Logger,
    log
};

// debug
#[cfg(debug_assertions)]
use crate::log_indent;

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

    if module.clone().map_err(|error| log!(error, "`{wgsl_in}`: {error}")).is_err() {
        bail!("");
    }

    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module.clone()?);
    
    if info.clone().map_err(|error| log!(error, "`{wgsl_in}`: {error}")).is_err() {
        bail!("");
    }

    let words = write_vec(&module?, &info?, &options, None)?;

    let bytes_out: Vec<u8> = words.iter().flat_map(|word| word.to_ne_bytes().to_vec()).collect();

    create_dir_all("target/dacho/shaders")?;

    write(spv_out, bytes_out)?;

    #[cfg(debug_assertions)]
    log!(info, "Compiled `{wgsl_in}`");

    Ok(())
}

pub async fn compile_shaders() -> Result<()> {
    #[cfg(debug_assertions)] {
        log!(info, "Compiling shaders");
        log_indent!(true);
    }

    let mut filenames = vec![];
    let mut futures   = vec![];

    for shader in read_dir("assets/shaders").expect("please move your shaders to `assets/shaders/*.wgsl") {
        let path = shader?.path();

        filenames.push(path.display().to_string());
        futures.push(spawn(async move { compile_shader(&path) }));
    }

    let results = join_all(futures).await;

    let (mut i, mut j) = (0_usize, 0_usize);

    for result in results {
        if let Err(error) = result? {
            if error.to_string() != "" {
                let filename = &filenames[i][filenames[i].rfind('/').context("Error parsing shader path")?+1..];

                log!(error, "`{filename}`: {error}");
            }

            j += 1;
        }

        i += 1;
    }

    #[cfg(debug_assertions)]
    log_indent!(false);

    if j != 0 {
        log!(panic, "Failed to compile all shaders");
    }

    Ok(())
}

