use super::gpio;
use super::IO_BASE;
use volatile::*;

/// Base address for the MU regsiters
const MU_REG_BASE: usize = IO_BASE + 0x215040;

/// Address of `AUXENB` register
const AUX_ENABLES: *mut Volatile<u8> = (IO_BASE + 0x215004) as *mut Volatile<u8>;

#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    IO: Volatile<u32>,
    IER: Volatile<u32>,
    IIR: Volatile<u32>,
    LCR: Volatile<u32>,
    MCR: Volatile<u32>,
    LSR: Volatile<u32>,
    MSR: Volatile<u32>,
    SCRATCH: Volatile<u32>,
    CNTL: Volatile<u32>,
    STAT: Volatile<u32>,
    BAUD: Volatile<u32>,
}

/// MiniUart strucutre for the RPi3
pub struct MiniUart {
    registers: &'static mut Registers,
    timeout: Option<u32>,
}

impl MiniUart {
    /// Initialize the MiniUart by enabling pin 14 and 15 to alt5
    pub fn new() -> MiniUart {
        let _gpio14 = gpio::Gpio::new(14).into_alt(gpio::Function::Alt5);
        let mut gpio15 = gpio::Gpio::new(15).into_alt(gpio::Function::Alt5);
        gpio15.set_gpio(0, (1 << 14) | (1 << 15), 0);

        let registers = unsafe {
            (*AUX_ENABLES).update(|v| *v |= 1);
            &mut *(MU_REG_BASE as *mut Registers)
        };

        registers.CNTL.write(0);
        registers.IER.write(0);
        registers.LCR.write(3);
        registers.MCR.write(0);
        registers.BAUD.write(270);
        registers.CNTL.write(3);

        MiniUart {
            registers: registers,
            timeout: None,
        }
    }

    /// Writes the byte `byte` to the serial port
    pub fn write_byte(&mut self, byte: u8) {
        loop {
            if self.registers.LSR.read() & 0x20 != 0 {
                break;
            }
        }
        self.registers.IO.write(byte as u32);
    }
}
