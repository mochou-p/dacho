// dacho/src/game/logger.rs

#[macro_export]
macro_rules! log {
    ($fn:ident, $($args:expr),*) => {
        #[cfg(debug_assertions)] {
            $crate::app::logger::Logger::$fn(&$crate::path_to_log_source(file!()), &format!($($args),*))
        }
    };
}

#[macro_export]
macro_rules! log_from {
    ($fn:ident, $source:expr, $($args:expr),*) => {
        #[cfg(debug_assertions)] {
            $crate::app::logger::Logger::$fn($source, &format!($($args),*))
        }
    };
}

#[macro_export]
macro_rules! self_log {
    ($fn:ident, $prefix:expr) => {
        #[cfg(debug_assertions)]
        $crate::log!($fn, "{} {}", $prefix, $crate::type_name_tail::<Self>())
    };
}

#[macro_export]
macro_rules! create_log {
    ($severity:ident) => {
        #[cfg(debug_assertions)]
        $crate::self_log!($severity, "Creating")
    };
}

#[macro_export]
macro_rules! destroy_log {
    ($severity:ident) => {
        #[cfg(debug_assertions)]
        $crate::self_log!($severity, "Destroying")
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
    #[allow(clippy::print_stdout)]
    fn stdout(source: &str, message: &str, color: &str, is_everything_colored: bool) {
        if is_everything_colored {
            println!("{color}[{source}] {message}{}", Self::RESET);
        } else {
            println!("{color}[{source}]{} {message}", Self::RESET);
        }
    }

    #[inline]
    #[allow(clippy::print_stderr)]
    fn stderr(source: &str, message: &str, color: &str) {
        eprintln!("{color}[{source}]{} {message}", Self::RESET);
    }

    #[inline]
    pub fn debug(source: &str, message: &str) {
        Self::stdout(source, message, Self::BLACK, true);
    }

    #[inline]
    pub fn info(source: &str, message: &str) {
        Self::stdout(source, message, Self::CYAN, false);
    }

    #[inline]
    pub fn warning(source: &str, message: &str) {
        Self::stderr(source, message, Self::YELLOW);
    }

    #[inline]
    pub fn error(source: &str, message: &str) {
        Self::stderr(source, message, Self::RED);
    }
}

