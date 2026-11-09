use core::fmt;
use core::fmt::Write;

use crate::vga_buffer::{Color, WRITER};

// LOGGING FUNCTION

pub fn _log(color: Color, prefix: &str, args: fmt::Arguments) {

    let mut writer = WRITER.lock();

    writer.set_color(color, Color::Black);

    writer.write_string(prefix);
    writer.write_fmt(args).unwrap();
    writer.write_byte(b'\n');

    writer.set_color(Color::LightGray, Color::Black);
}

// INFO MACRO - GREEN

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::vga_buffer::Color::Green,
            "[ INFO ] ",
            format_args!($($arg)*)
        )
    };
}

// WARN MACRO - YELLOW

#[macro_export]
macro_rules! kwarn {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::vga_buffer::Color::Yellow,
            "[ WARN ] ",
            format_args!($($arg)*)
        )
    };
}

// ERROR MACRO - RED

#[macro_export]
macro_rules! kerror {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::vga_buffer::Color::Red,
            "[ ERROR ] ",
            format_args!($($arg)*)
        )
    };
}

// DEBUG MACRO - CYAN

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::vga_buffer::Color::Cyan,
            "[ DEBUG ] ",
            format_args!($($arg)*)
        )
    };
}
