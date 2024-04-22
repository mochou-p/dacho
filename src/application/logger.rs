// dacho/src/application/logger.rs

use std::io::Write;

struct Color;

static mut INDENTATION: isize = 0;

impl Color {
    const BLUE:  &'static str = "\x1b[36;1m";
    const RESET: &'static str = "\x1b[0m";
}

pub struct Logger;

impl Logger {
    pub fn info<T: Into<String> + std::fmt::Display>(message: T) {
        println!(
            "{}{}Info{} {}",
            " ".repeat((unsafe { INDENTATION } * 5) as usize),
            Color::BLUE,
            Color::RESET,
            message
        );
    }

    pub fn info_r<T: Into<String> + std::fmt::Display>(message: T) {
        print!(
            "{}{}Info{} {}\r",
            " ".repeat((unsafe { INDENTATION } * 5) as usize),
            Color::BLUE,
            Color::RESET,
            message
        );

        std::io::stdout()
            .flush()
            .expect("Failed to flush stdout");
    }

    pub fn indent(delta: i8) {
        if delta > 0 || unsafe { INDENTATION } > 0 {
            unsafe { INDENTATION += delta as isize; }
        }
    }
}

