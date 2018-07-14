pub use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u32, ()> for Reg {
    #[inline]
    fn get(&self) -> u32 {
        let reg: u32;
        unsafe {
            asm!("mrs $0, CNTV_TVAL_EL0" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    #[inline]
    fn set(&self, value: u32) {
        unsafe {
            asm!("msr CNTV_TVAL_EL0, $0" :: "r"(value) :: "volatile");
        }
    }
}

pub static CNTV_TVAL_EL0: Reg = Reg {};
