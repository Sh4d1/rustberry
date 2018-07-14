#[macro_use]
pub mod console;

pub mod gpio;

pub mod mini_uart;

//pub mod timer;

pub mod generic_timer;

/// Base address of IO devices
pub const IO_BASE: usize = 0x3F000000;

