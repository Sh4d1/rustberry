use super::IO_BASE;
use core::marker::PhantomData;
use volatile::*;

/// Base address of `GPIO` registers
const GPIO_BASE: usize = IO_BASE + 0x200000;

/// Alternative GPIO functions
#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    FSEL: [Volatile<u32>; 6],
    __reserved0: u32,
    SET: [WriteOnly<u32>; 2],
    __reserved1: u32,
    CLR: [WriteOnly<u32>; 2],
    __reserved2: u32,
    LEV: [ReadOnly<u32>; 2],
    __reserved3: u32,
    EDS: [Volatile<u32>; 2],
    __reserved4: u32,
    REN: [Volatile<u32>; 2],
    __reserved5: u32,
    FEN: [Volatile<u32>; 2],
    __reserved6: u32,
    HEN: [Volatile<u32>; 2],
    __reserved7: u32,
    LEN: [Volatile<u32>; 2],
    __reserved8: u32,
    AREN: [Volatile<u32>; 2],
    __reserved9: u32,
    AFEN: [Volatile<u32>; 2],
    __reserved10: u32,
    PUD: Volatile<u32>,
    PUDCLK: [Volatile<u32>; 2],
}

/// States of a pin
pub enum Uninitialized {}
pub enum Input {}
pub enum Output {}
pub enum Alt {}

/// A Gpio pin in the state `State`
pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut Registers,
    _state: PhantomData<State>,
}

impl<T> Gpio<T> {
    /// Transition `self` to a new state `S`
    #[inline(always)]
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData,
        }
    }

    /// Set the Gpio pull-up/pull-down state for values in `pin_value`
    pub fn set_gpio(&mut self, gpio_value: u8, pin_value: u32, pudclk_index: u8) {
        unsafe {
            self.registers.PUD.write(gpio_value as u32);
            for _ in 0..150 {
                asm!("nop" :::: "volatile");
            }

            self.registers.PUDCLK[pudclk_index as usize].write(pin_value as u32);
            for _ in 0..150 {
                asm!("nop" :::: "volatile");
            }
            self.registers.PUDCLK[pudclk_index as usize].write(0);
        }
    }
}

impl Gpio<Uninitialized> {
    /// Returns a new `Uninitialized` `GPIO` struct for pin `pin`
    pub fn new(pin: u8) -> Gpio<Uninitialized> {
        if pin > 53 {
            panic!("Gpio::new(): pin {} exceeds maximum of 53", pin);
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            pin: pin,
            _state: PhantomData,
        }
    }

    /// Enables the alt function `function`
    pub fn into_alt(self, function: Function) -> Gpio<Alt> {
        let select = self.pin / 10;
        let offset = self.pin % 10;

        self.registers.FSEL[select as usize].write((function as u32) << (3 * offset));
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData,
        }
    }

    /// Sets the pin to an `Output` pin
    pub fn into_output(self) -> Gpio<Output> {
        self.into_alt(Function::Output).transition()
    }

    /// Sets the pin to an `Input` pin
    pub fn into_input(self) -> Gpio<Input> {
        self.into_alt(Function::Input).transition()
    }
}

impl Gpio<Output> {
    /// Sets the pin
    pub fn set(&mut self) {
        let index = if self.pin >= 32 { 1 } else { 0 };
        self.registers.SET[index as usize].write(1 << (self.pin - index * 32));
    }

    /// Clear the pin
    pub fn clear(&mut self) {
        let index = if self.pin >= 32 { 1 } else { 0 };
        self.registers.CLR[index as usize].write(1 << (self.pin - index * 32));
    }
}

impl Gpio<Input> {
    /// Gets the pin value, returns `true` if the level is high, `false` if it is low
    pub fn level(&mut self) -> bool {
        let index = if self.pin >= 32 { 1 } else { 0 };
        let high = 1 << (self.pin - index * 32);
        (self.registers.LEV[index as usize].read() & high) == high
    }
}
