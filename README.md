<!-- dacho/README.md -->

# dacho

## Usage

In dacho, you can make/load a `World`, and run/save it.
This is the minimal [hello_world example](examples/hello_world.rs):
```rust
use dacho::prelude::*;

fn main() {
    World::new()
        .run();
}
```
See other [examples](examples/) for more

## Notes

Run with `--release` if your system is missing [Vulkan Validation Layers](https://github.com/KhronosGroup/Vulkan-ValidationLayers)

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
dual licensed as above, without any additional terms or conditions.

