pub mod gpio;

pub mod mini_uart;

#[macro_use]
pub mod console;

/// Base address of IO devices
const IO_BASE: usize = 0x3F000000;

