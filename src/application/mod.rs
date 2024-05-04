// dacho/src/application/mod.rs

    mod camera;
#[cfg(debug_assertions)]
pub mod logger;
    mod scene;
    mod timer;
    mod window;

use {
    anyhow::Result,
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
    futures::{
        executor::block_on,
        future::join_all
    },
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

        let (scene, skybox_texture, gltf_textures) = Scene::demo()?;

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

#[cfg(debug_assertions)]
async fn compile_shader(filepath: std::path::PathBuf) -> Result<Option<(String, String)>> {
    let in_      = &format!("{}", filepath.display());
    let filename = &in_[in_.rfind('/').context("Error parsing shader path")?+1..];
    let out      = &format!("assets/.cache/shaders.{filename}.spv");

    let output = std::process::Command::new("glslc")
        .args([in_, "-o", out])
        .output()?;

    match output.status.code().context("Error receiving status code")? {
        0 => {
            Logger::info(format!("Compiled `{filename}`"));

            Ok(None)
        },
        _ => {
            Ok(Some((filename.to_string(), String::from_utf8(output.stderr)?)))
        }
    }
}

#[cfg(debug_assertions)]
async fn compile_shaders() -> Result<()> {
    Logger::info("Compilining shaders");
    Logger::indent(1);

    let mut futures = vec![];

    for shader in std::fs::read_dir("assets/shaders")? {
        for stage in std::fs::read_dir(shader?.path())? {
            futures.push(spawn(compile_shader(stage?.path())));
        }
    }

    let     results = join_all(futures).await;
    let mut errors  = false;

    for result in results {
        if let Some((filename, error)) = result?? {
            Logger::error(format!("in `{filename}`:\n{error}"));
            errors = true;
        }
    }

    Logger::indent(-1);

    if errors {
        Logger::panic("Failed to compile all shaders");
    }

    Ok(())
}

