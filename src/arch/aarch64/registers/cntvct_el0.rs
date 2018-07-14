pub use register::cpu::RegisterReadOnly;

pub struct Reg;

impl RegisterReadOnly<u64, ()> for Reg {
    #[inline]
    fn get(&self) -> u64 {
        let reg: u64;
        unsafe {
            asm!("mrs $0, CNTVCT_EL0" : "=r"(reg) ::: "volatile");
        }
        reg
    }
}

pub static CNTVCT_EL0: Reg = Reg {};
