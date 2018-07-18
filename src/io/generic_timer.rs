use arch::registers::*;
use volatile::*;

const GEN_TIMER_REG_BASE: usize = 0x40000000;

#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    CONTROL: Volatile<u32>,
    _unused1: [Volatile<u32>; 8],
    LOCAL_IRQ: Volatile<u32>,
    _unused2: [Volatile<u32>; 3],
    LOCAL_TIMER_CTL: Volatile<u32>,
    LOCAL_TIMER_FLAGS: Volatile<u32>,
    _unused3: Volatile<u32>,
    CORE_TIMER_IRQCNTL: [Volatile<u32>; 4],
    CORE_MAILBOX_IRQCNTL: [Volatile<u32>; 4],
    CORE_IRQ_SRC: [Volatile<u32>; 4],
}

pub struct GenericTimer {
    registers: &'static mut Registers,
}

impl GenericTimer {
    pub fn new() -> GenericTimer {
        GenericTimer {
            registers: unsafe { &mut *(GEN_TIMER_REG_BASE as *mut Registers) },
        }
    }

    pub fn ack(&mut self) {
        if self.registers.CORE_IRQ_SRC[0].read() & 0x08 != 0 {
            CNTV_TVAL_EL0.set(62500000);
        }
    }

    pub fn tick_in(&mut self, us: u32) {
        let cntfrq = CNTFRQ_EL0.get();
        CNTV_TVAL_EL0.set(((cntfrq as f64) * (us as f64) / 1000000.0) as u32);
    }

    pub fn init(&mut self) {
        let cntfrq = CNTFRQ_EL0.get();

        CNTV_TVAL_EL0.set(cntfrq);

        self.registers.CORE_TIMER_IRQCNTL[0].write(0x08);
        CNTV_CTL_EL0.set(1);
    }
}

pub fn ack() {
    GenericTimer::new().ack();
}

pub fn tick_in(us: u32) {
    GenericTimer::new().tick_in(us);
}
