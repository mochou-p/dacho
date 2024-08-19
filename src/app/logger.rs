// dacho/src/game/logger.rs

#[macro_export]
macro_rules! debug {
    ($source:expr, $($args:expr),*) => {
        $crate::app::logger::Logger::debug($source, &format!($($args),*))
    };
}

#[macro_export]
macro_rules! info {
    ($source:expr, $($args:expr),*) => {
        $crate::app::logger::Logger::info($source, &format!($($args),*))
    };
}

#[macro_export]
macro_rules! warning {
    ($source:expr, $($args:expr),*) => {
        $crate::app::logger::Logger::warning($source, &format!($($args),*))
    };
}

#[macro_export]
macro_rules! error {
    ($source:expr, $($args:expr),*) => {
        $crate::app::logger::Logger::error($source, &format!($($args),*))
    };
}

#[macro_export]
macro_rules! fatal {
    ($source:expr, $($arg:expr),*) => {
        use $crate::app::logger::Logger;

        panic!("{}[{}]{} {}", Logger::RED, $source, Logger::RESET, format!($($arg),*))
    };
}

#[allow(clippy::exhaustive_structs)]
pub struct Logger;

impl Logger {
    pub const RED:    &'static str = "\x1b[31;1m";
    pub const YELLOW: &'static str = "\x1b[33;1m";
    pub const CYAN:   &'static str = "\x1b[36;1m";
    pub const WHITE:  &'static str = "\x1b[0;1m";
    pub const BLACK:  &'static str = "\x1b[90m";
    pub const RESET:  &'static str = "\x1b[0m";

    #[inline]
    #[allow(dead_code, clippy::print_stdout)]
    fn stdout(source: &str, message: &str, color: &str, is_everything_colored: bool) {
        if is_everything_colored {
            println!("{color}[{source}] {message}{}", Self::RESET);
        } else {
            println!("{color}[{source}]{} {message}", Self::RESET);
        }
    }

    #[inline]
    #[allow(dead_code, clippy::print_stderr)]
    fn stderr(source: &str, message: &str, color: &str) {
        eprintln!("{color}[{source}]{} {message}", Self::RESET);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn debug(source: &str, message: &str) {
        #[cfg(debug_assertions)]
        Self::stdout(source, message, Self::BLACK, true);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn info(source: &str, message: &str) {
        #[cfg(debug_assertions)]
        Self::stdout(source, message, Self::CYAN, false);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn warning(source: &str, message: &str) {
        #[cfg(debug_assertions)]
        Self::stderr(source, message, Self::YELLOW);
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn error(source: &str, message: &str) {
        #[cfg(debug_assertions)]
        Self::stderr(source, message, Self::RED);
    }
}

