// dacho/src/application/logger.rs

#![allow(dead_code)]

// core
use core::fmt::Display;

// std
use std::io::Write;

struct Color;

#[macro_export]
macro_rules! log_indent {
    ($i:expr) => {
        Logger::indent($i)
    };
}

#[macro_export]
macro_rules! log {
    ($f:ident, $($arg:expr),*) => {
        Logger::$f(&format!($($arg),*))
    };
}

static mut INDENTATION:      usize = 0;
static     INDENTATION_SIZE: usize = 5;

impl Color {
    const RED:    &'static str = "\x1b[31;1m";
    const YELLOW: &'static str = "\x1b[33;1m";
    const CYAN:   &'static str = "\x1b[36;1m";
    const RESET:  &'static str = "\x1b[0m";
}

pub struct Logger;

impl Logger {
    fn info_str<T: Into<String> + Display>(s: &T) -> String {
        format!(
            "{}{}Info{} {}",
            " ".repeat(unsafe { INDENTATION } * INDENTATION_SIZE),
            Color::CYAN, Color::RESET,
            s
        )
    }

    fn warning_str<T: Into<String> + Display>(s: &T) -> String {
        format!(
            "{}{}Warning{} {}",
            " ".repeat(unsafe { INDENTATION } * INDENTATION_SIZE),
            Color::YELLOW, Color::RESET,
            s
        )
    }

    fn error_str<T: Into<String> + Display>(s: &T) -> String {
        format!(
            "{}{}Error{} {}",
            " ".repeat(unsafe { INDENTATION } * INDENTATION_SIZE),
            Color::RED, Color::RESET,
            s
        )
    }

    pub fn indent(delta: bool) {
        if delta {
            unsafe { INDENTATION += 1; }
        } else if unsafe { INDENTATION } > 0 {
            unsafe { INDENTATION -= 1; }
        }
    }

    pub fn info<T: Into<String> + Display>(message: &T) {
        println!("{}", Self::info_str(message));
    }

    pub fn info_r<T: Into<String> + Display>(message: &T) {
        print!("{}\r", Self::info_str(message));

        std::io::stdout()
            .flush()
            .expect("Failed to flush stdout");
    }

    pub fn warning<T: Into<String> + Display>(message: &T) {
        println!("{}", Self::warning_str(message));
    }

    pub fn error<T: Into<String> + Display>(message: &T) {
        println!("{}", Self::error_str(message));
    }

    pub fn panic<T: Into<String> + Display>(message: &T) {
        panic!("{}", Self::error_str(message));
    }
}

