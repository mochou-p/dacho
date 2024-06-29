<!-- dacho/README.md -->

# dacho

## Disclaimer
This project is in an early stage of development

## Usage
In dacho, you can make/load a `World`, and run/save it  
```rust
// dacho hello world

use dacho::prelude::*;

fn main() {
    World::new()
        .run();
}
```
See the [dacho example](https://github.com/mochou-p/dacho-example) for more

## Notes
- the `dev` profile requires [Vulkan Validation Layers](https://github.com/KhronosGroup/Vulkan-ValidationLayers)
(use `--release` otherwise)
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

