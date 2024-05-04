// dacho/src/application/logger.rs

use std::io::Write;

struct Color;

static mut INDENTATION: isize = 0;

impl Color {
    const RED:   &'static str = "\x1b[31;1m";
    const CYAN:  &'static str = "\x1b[36;1m";
    const RESET: &'static str = "\x1b[0m";
}

pub struct Logger;

impl Logger {
    fn info_str<T: Into<String> + std::fmt::Display>(s: T) -> String {
        format!(
            "{}{}Info{} {}",
            " ".repeat((unsafe { INDENTATION } * 5) as usize),
            Color::CYAN,
            Color::RESET,
            s
        )
    }

    fn error_str<T: Into<String> + std::fmt::Display>(s: T) -> String {
        format!(
            "{}{}Error{} {}",
            " ".repeat((unsafe { INDENTATION } * 5) as usize),
            Color::RED,
            Color::RESET,
            s
        )
    }

    pub fn indent(delta: i8) {
        if delta > 0 || unsafe { INDENTATION } > 0 {
            unsafe { INDENTATION += delta as isize; }
        }
    }

    pub fn info<T: Into<String> + std::fmt::Display>(message: T) {
        println!("{}", Self::info_str(message));
    }

    pub fn info_r<T: Into<String> + std::fmt::Display>(message: T) {
        print!("{}\r", Self::info_str(message));

        std::io::stdout()
            .flush()
            .expect("Failed to flush stdout");
    }

    pub fn error<T: Into<String> + std::fmt::Display>(message: T) {
        println!("{}", Self::error_str(message));
    }

    pub fn panic<T: Into<String> + std::fmt::Display>(message: T) {
        panic!("{}", Self::error_str(message));
    }
}

