use super::interrupt::Interrupt;
use super::trap_frame::TrapFrame;
use arch::context::State;
use io::generic_timer;
use SCHEDULER;
pub fn handle_irq(interrupt: Interrupt, tf: &mut TrapFrame) {
    //    kprintln!("{:?}", interrupt);
    //generic_timer::tick_in(100);
    generic_timer::ack();
    unsafe {
        let reg: u32;
        asm!("mrs $0, CNTPCT_EL0" : "=r"(reg) ::: "volatile");
        kprintln!("{}", reg);
    }
    SCHEDULER.switch(State::Ready, &mut *tf);
}
