// dacho/examples/usage/src/main.rs

#![expect(clippy::absolute_paths, reason = "example style")]


#[derive(Default)]
struct Game {
    counter: usize
}

impl dacho::app::Game for Game {
    fn update(&mut self) {
        self.counter += 1;
        print!("{}\r", self.counter);
        {
            use std::io::Write as _;
            _ = std::io::stdout().flush();
        }
    }
}


fn main() {
    dacho::app::App::<Game>::default()
        .run();

    println!();
}

