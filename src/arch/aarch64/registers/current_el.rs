pub use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u32, ()> for Reg {
    #[inline]
    fn get(&self) -> u32 {
        let reg: u32;
        unsafe {
            asm!("mrs $0, CurrentEL" : "=r"(reg) ::: "volatile");
        }
        reg >> 2
    }
}

pub static CURRENT_EL: Reg = Reg {};
