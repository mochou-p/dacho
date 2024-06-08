// dacho/src/shader/compilation.rs

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
    application::logger::Logger,
    log
};

// debug
#[cfg(debug_assertions)]
use crate::log_indent;

async fn compile_shader(filepath: std::path::PathBuf) -> Result<()> {
    let wgsl_in  = &format!("{}", filepath.display());
    let filename = &wgsl_in[wgsl_in.rfind('/').context("Error parsing shader path")?+1..];
    let spv_out  = &format!("assets/.cache/shaders.{filename}.spv");

    let options = SpvOptions {
        lang_version: (1, 5),
        ..Default::default()
    };

    let bytes_in = &std::fs::read(wgsl_in)?;
    let code     = std::str::from_utf8(bytes_in)?;
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

    {
        let cache_dir = "assets/.cache";

        if !std::path::Path::new(cache_dir).exists() {
            std::fs::create_dir(cache_dir)?;
        }
    }

    std::fs::write(spv_out, bytes_out)?;

    #[cfg(debug_assertions)]
    log!(info, "Compiled `{wgsl_in}`");

    Ok(())
}

pub async fn compile_shaders() -> Result<()> {
    #[cfg(debug_assertions)] {
        log!(info, "Compiling shaders");
        log_indent!(1);
    }

    let mut filenames = vec![];
    let mut futures   = vec![];

    for shader in std::fs::read_dir("assets/shaders")? {
        let path = shader?.path();

        filenames.push(path.display().to_string());
        futures.push(spawn(compile_shader(path)));
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
    log_indent!(-1);

    if j != 0 {
        log!(panic, "Failed to compile all shaders");
    }

    Ok(())
}

