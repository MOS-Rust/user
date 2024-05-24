use core::fmt::{self, Write};

use crate::syscall_print_console;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        syscall_print_console(s);
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
