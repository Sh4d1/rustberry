pub use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u32, ()> for Reg {
    #[inline]
    fn get(&self) -> u32 {
        let reg: u32;
        unsafe {
            asm!("mrs $0, CNTFRQ_EL0" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    #[inline]
    fn set(&self, value: u32) {
        unsafe {
            asm!("msr CNTFRQ_EL0, $0" :: "r"(value) :: "volatile");
        }
    }
}

pub static CNTFRQ_EL0: Reg = Reg {};
