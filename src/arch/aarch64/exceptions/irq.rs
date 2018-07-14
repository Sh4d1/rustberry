use super::interrupt::Interrupt;
use super::trap_frame::TrapFrame;
use arch::context::State;
use io::generic_timer;
use SCHEDULER;
pub fn handle_irq(interrupt: Interrupt, tf: &mut TrapFrame) {
    kprintln!("{:?}", interrupt);
    generic_timer::ack();
    SCHEDULER.switch(State::Ready, &mut *tf);
}
