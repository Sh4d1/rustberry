mod esr;
mod trap_frame;
mod interrupt;
mod irq;

pub use self::trap_frame::TrapFrame;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}


#[no_mangle]
pub extern "C" fn handle_exception(info: Info, esr: u32, tf: &mut TrapFrame) {
    use self::esr::Syndrome;

    match info.kind {
        Kind::Synchronous => {
            match Syndrome::from(esr) {
                Syndrome::Brk(_) => tf.elr += 4,
                _ => {},
            }
            kprintln!("{:?}", info);
            kprintln!("{:?}", esr::Syndrome::from(esr));
        },
        Kind::Irq => {
            use self::irq;
            use self::interrupt;
            let ctl = interrupt::Controller::new();
            if ctl.is_pending(interrupt::Interrupt::Timer1) {
                irq::handle_irq(interrupt::Interrupt::Timer1, &mut *tf);
            } else {
                kprintln!("IRQ != 1 WIP");
            }
        }
        _ => {
            kprintln!("Other WIP");
        }
    } 
    
}
