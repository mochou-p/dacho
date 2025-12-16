<!-- dacho/README.md -->

# dacho
🦀 rust  
🌋 vulkan  
🎮 game  
⚙️ engine  

📦 read about dacho crates [here](crates)  
💡 read about dacho examples [here](examples)  

## features
> [!IMPORTANT]
> all per-crate features are disabled by default,  
> but the `dacho` meta crate, enables **ALL** features of **ALL** re-exported `dacho_*` crates  
> (shown with checkboxes)  

### dacho
- [x] app_gilrs - enables the `gilrs` feature in `dacho_app`
- [x] window_winit_wayland - enables the `winit_wayland` feature in `dacho_window`
- [x] window_winit_x11 - enables the `winit_x11` feature in `dacho_window`

### dacho_app
- [ ] gilrs - adds gamepad input support for the `GameTrait`

### dacho_window
- [ ] winit_wayland - enables `winit`'s `wayland` feature
- [ ] winit_x11 - enables `winit`'s `x11` feature

## profiles
> [!NOTE]
> modified built-in profiles, and extra custom ones for specific use cases  

### built-in profiles
name | settings | command
:-|:-|:-
dev | optimised for compile times | `cargo build`
release | optimised for runtime performance | `cargo build --release`

### custom profiles
name | settings | command
:-|:-|:-
debugging | inherits dev + debug info | `cargo build --profile debugging`
profiling | inherits release + debug info | `cargo build --profile profiling`

