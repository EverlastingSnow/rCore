use core::fmt;

pub const BLUE: &str = "\x1b[34m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const RESET: &str = "\x1b[0m";

//日志输出
pub fn log_print(level: &str, color: &str, args: fmt::Arguments) {
    print!("{}", color);
    print!("[{}]: ", level);
    print!("{}", args);
    println!("{}", RESET);
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log::log_print("INFO", $crate::log::BLUE, format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::log::log_print("DEBUG", $crate::log::YELLOW, format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log::log_print("ERROR", $crate::log::RED, format_args!($($arg)*));
    }}
}