// dacho/examples/usage/src/main.rs

#![expect(clippy::absolute_paths, reason = "example style")]


fn main() {
    dacho::app::App::default()
        .run();
}

