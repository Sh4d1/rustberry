pub use register::cpu::RegisterReadWrite;

register_bitfields! {u32,
CNTV_CTL_EL0 [
    ENABLE        OFFSET(0)  NUMBITS(1) [],
    IMASK         OFFSET(1)  NUMBITS(1) [],
    ISTATUS       OFFSET(2)  NUMBITS(1) []
]
}

pub struct Reg;

impl RegisterReadWrite<u32, CNTV_CTL_EL0::Register> for Reg {
    #[inline]
    fn get(&self) -> u32 {
        let reg: u32;
        unsafe {
            asm!("mrs $0, CNTV_CTL_EL0" : "=r"(reg) ::: "volatile");
        }
        reg
    }

    #[inline]
    fn set(&self, value: u32) {
        unsafe {
            asm!("msr CNTV_CTL_EL0, $0" :: "r"(value) :: "volatile");
        }
    }
}

pub static CNTV_CTL_EL0: Reg = Reg {};
