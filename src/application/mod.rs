// dacho/src/application/mod.rs

    mod camera;
#[cfg(debug_assertions)]
pub mod logger;
    mod scene;
    mod timer;
    mod window;

use {
    anyhow::Result,
    futures::executor::block_on,
    glam::f32 as glam,
    winit::{
        event::{DeviceEvent, Event, WindowEvent},
        event_loop::{EventLoop, EventLoopWindowTarget},
        keyboard::{KeyCode::*, PhysicalKey::Code}
    }
};

use {
    camera::Camera,
    scene::Scene,
    timer::Timer,
    window::Window,
    super::renderer::Renderer
};

#[cfg(debug_assertions)]
use {
    anyhow::Context,
    futures::future::join_all,
    logger::Logger,
    tokio::spawn
};

pub struct Application {
    window:   Window,
    renderer: Renderer,
    camera:   Camera,
    timer:    Timer
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        #[cfg(debug_assertions)] {
            Logger::info("Creating Application");
            Logger::indent(1);

            block_on(compile_shaders())?;
        }

        let window = Window::new("dacho", 1600, 900, event_loop)?;

        let (scene, skybox_texture, gltf_textures) = block_on(Scene::demo())?;

        let renderer = Renderer::new(
            event_loop, &window.window, window.width, window.height, &scene, &skybox_texture, &gltf_textures
        )?;

        let camera = Camera::new(glam::Vec3::new(0.0, 0.0, 8.0));
        let timer  = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        #[cfg(debug_assertions)]
        Logger::indent(-1);

        Ok(Self { window, renderer, camera, timer })
    }

    pub fn handle_event<T>(&mut self, event: &Event<T>, elwt: &EventLoopWindowTarget<T>) {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, is_synthetic: false, .. }, .. } => {
                if event.physical_key == Code(Escape) {
                    elwt.exit();
                }

                self.camera.keyboard_input(event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                self.camera.mouse_motion(delta);
            },
            Event::AboutToWait => {
                self.renderer.wait_for_device();
                self.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                self.renderer.redraw(self.camera.transform(), self.timer.elapsed());

                #[cfg(debug_assertions)]
                self.timer.fps();
            },
            _ => ()
        }
    }
}

use naga::{
    back::spv::{Options as SpvOptions, write_vec},
    front::wgsl::Frontend,
    valid::{Capabilities, ValidationFlags, Validator}
};

#[cfg(debug_assertions)]
async fn compile_shader(filepath: std::path::PathBuf) -> Result<()> {
    let wgsl_in  = &format!("{}", filepath.display());
    let filename = &wgsl_in[wgsl_in.rfind('/').context("Error parsing shader path")?+1..];
    let spv_out  = &format!("assets/.cache/shaders.{filename}.spv");

    let options = SpvOptions {
        lang_version: (1, 5),
        ..Default::default()
    };

    let bytes_in  = &std::fs::read(wgsl_in)?;
    let code      = std::str::from_utf8(bytes_in)?;
    let module    = Frontend::new().parse(code);
    let _         = module.clone().map_err(|error| Logger::error(format!("`{filename}`: {error}")));
    let info      = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module.clone()?);
    let _         = info.clone().map_err(|error| Logger::error(format!("`{filename}`: {error}")));
    let words     = write_vec(&module?, &info?, &options, None)?;

    let bytes_out: Vec<u8> = words.iter().flat_map(|word| word.to_ne_bytes().to_vec()).collect();

    std::fs::write(spv_out, bytes_out)?;

    Logger::info(format!("Compiled `{spv_out}`"));

    Ok(())
}

#[cfg(debug_assertions)]
async fn compile_shaders() -> Result<()> {
    Logger::info("Compilining shaders");
    Logger::indent(1);

    let mut filenames = vec![];
    let mut futures   = vec![];

    for shader in std::fs::read_dir("assets/shaders")? {
        let path = shader?.path();

        filenames.push(path.clone());
        futures.push(spawn(compile_shader(path)));
    }

    let results = join_all(futures).await;

    let mut i = 0_usize;

    for result in results {
        match result? {
            Err(r) => { Logger::error(format!("`{}`: {r}", filenames[i].display())); }
            _ => () 
        }

        i += 1;
    }

    panic!();

    Logger::indent(-1);

    Ok(())
}

