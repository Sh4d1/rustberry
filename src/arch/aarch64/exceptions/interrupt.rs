use io::IO_BASE;

use volatile::*;

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    AuxInt = 29,
    I2cSpiSlv = 43,
    Pwa0 = 45,
    Pwa1 = 46,
    Smi = 48,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    I2c = 53,
    Spi = 54,
    Pcm = 55,
    Uart = 56,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    IRQ_BASIC_PENDING: Volatile<u32>,
    IRQ_PENDING_1: Volatile<u32>,
    IRQ_PENDING_2: Volatile<u32>,
    FIQ_CONTROL: Volatile<u32>,
    ENABLE_IRQ_1: Volatile<u32>,
    ENABLE_IRQ_2: Volatile<u32>,
    ENABLE_BASIC_IRQ: Volatile<u32>,
    DISABLE_IRQ_1: Volatile<u32>,
    DISABLE_IRQ_2: Volatile<u32>,
    DISABLE_BASIC_IRQ: Volatile<u32>,
}

pub struct Controller {
    registers: &'static mut Registers,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    pub fn enable(&mut self, int: Interrupt) {
        if (int as u32) < 32 {
            self.registers
                .ENABLE_IRQ_1
                .update(|v| *v ^= 1 << (int as u32))
        } else {
            self.registers
                .ENABLE_IRQ_2
                .update(|v| *v ^= 1 << (int as u32 - 32))
        }
    }

    pub fn disable(&mut self, int: Interrupt) {
        if (int as u32) < 32 {
            self.registers
                .DISABLE_IRQ_1
                .update(|v| *v ^= 1 << (int as u32))
        } else {
            self.registers
                .DISABLE_IRQ_2
                .update(|v| *v ^= 1 << (int as u32 - 32))
        }
    }

    pub fn is_pending(&self, int: Interrupt) -> bool {
        if (int as u32) < 32 {
            let ret: u32 = self.registers.IRQ_PENDING_1.read() & (1 << (int as u32));
            if ret == 0 {
                return true;
            }
            ret & (ret - 1) == 0
        } else {
            let ret: u32 = self.registers.IRQ_PENDING_2.read() & (1 << (int as u32 - 32));
            ret & (ret - 1) == 0
        }
    }
}
