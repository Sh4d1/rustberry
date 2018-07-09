use super::mini_uart;
use core::fmt;
use spin::Mutex;

/// Struct to get a global Console interface
pub struct Console {
    mu: Option<mini_uart::MiniUart>,
}

impl Console {
    /// Creates a new `Console`
    pub fn new() -> Console {
        Console { mu: None }
    }

    /// Init a newly created console, can only be called once.
    pub fn init(&mut self) {
        assert_has_not_been_called!("Can only init the Console once");
        self.mu = Some(mini_uart::MiniUart::new());
    }

    // Writes the byte `byte` to the MiniUart
    pub fn write_byte(&mut self, byte: u8) {
        match &mut self.mu {
            Some(mu) => mu.write_byte(byte),
            None => panic!("Console is not initialized"),
        }
    }

    // Writes the string `string` to the MiniUart
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // if it is printable
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // if not printable
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Global `Console` struct
lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console::new());
}
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::io::console::kprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn kprint(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}
