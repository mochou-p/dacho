<!-- dacho/README.md -->

# dacho

## What is it
An ECS game engine, written in pure Rust

## Disclaimer
This project is in it's early stage  
A lot of features are still WIP or TODO  
Expect breaking changes at each release  
There is no documentation yet  
The current ECS implementation is temporary

## Usage
In dacho, you insert Systems into schedules  
```rust
// A simple usage example

use dacho::prelude::*;

fn main() {
    let mut app = App::new("My game");

    app.start(|_| println!("Hello, World!"));

    app.keyboard(|_, key, is_pressed| {
        if is_pressed && key == Key::Space {
            println!("jump!");
        }
    });

    app.run();
}
```
See the [dacho example](https://github.com/mochou-p/dacho-example) for more

## Cargo features
- [ ] [vulkan-validation-layers](https://github.com/KhronosGroup/Vulkan-ValidationLayers) (requires the [Vulkan SDK](https://vulkan.lunarg.com/sdk/home) installed on your system)
###### A checked box means the feature is on by default

## Notes
- If you are using a tiling WM, it is currently recommended that you make a rule to open dacho as floating

## License
Licensed under either of
 * Apache License, Version 2.0  
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license  
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions

