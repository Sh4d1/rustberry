use super::process::{Id, Process, State};
use alloc::collections::vec_deque::VecDeque;
use arch::exceptions::TrapFrame;
use spin::Mutex;

pub const TICK: u32 = 2 * 1000 * 1000;

#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

impl GlobalScheduler {
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    pub fn add(&self, process: Process) -> Option<Id> {
        self.0
            .lock()
            .as_mut()
            .expect("scheduler uninitialized")
            .add(process)
    }

    #[must_use]
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        self.0
            .lock()
            .as_mut()
            .expect("scheduler uninitialized")
            .switch(new_state, tf)
    }

    pub fn start(&self) {
        *self.0.lock() = Some(Scheduler::new());
        let mut p1 = Process::new().expect("failed to create process p1");
        p1.trap_frame.elr = proc1 as *mut u64 as u64;
        p1.trap_frame.sp = p1.stack.top().as_u64();
        p1.trap_frame.spsr = 1;
        let tr = &*p1.trap_frame as *const _;
        self.add(p1);
        let mut p2 = Process::new().expect("failed to create process p2");
        p2.trap_frame.elr = proc2 as *mut u64 as u64;
        p2.trap_frame.sp = p2.stack.top().as_u64();
        self.add(p2);
        let mut p3 = Process::new().expect("failed to create process p3");
        p3.trap_frame.elr = proc3 as *mut u64 as u64;
        p3.trap_frame.sp = p3.stack.top().as_u64();
        self.add(p3);

        use io;
        io::generic_timer::GenericTimer::new().init();
        unsafe {
            asm!("  mov x0, $0
                    mov sp, x0
                    bl context_restore
                    adr x0, _start
                    mov sp, x0
                    eret"
                :: "r"(tr)
                :: "volatile");
        }
    }
}
extern "C" fn proc1() {
    kprintln!("Hey proc1");
    unsafe {
        asm!("brk 1" :::: "volatile");
    }
    loop {
        unsafe {
            kprintln!("Hey proc1");
            asm!("nop" :::: "volatile");
            for _ in 1..99999 {
                asm!("nop" :::: "volatile");
            }
        }
    }
}

extern "C" fn proc3() {
    kprintln!("Hey proc3");
    unsafe {
        asm!("brk 3" :::: "volatile");
    }
    loop {
        unsafe {
            kprintln!("Hey proc3");
            asm!("nop" :::: "volatile");
            for _ in 1..99999 {
                asm!("nop" :::: "volatile");
            }
        }
    }
}
extern "C" fn proc2() {
    kprintln!("Hey proc2");
    unsafe {
        asm!("brk 2" :::: "volatile");
    }
    loop {
        unsafe {
            kprintln!("Hey proc2");
            asm!("nop" :::: "volatile");
            for _ in 1..99999 {
                asm!("nop" :::: "volatile");
            }
        }
    }
}

#[derive(Debug)]
struct Scheduler {
    processes: VecDeque<Process>,
    current: Option<Id>,
    last_id: Option<Id>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            processes: VecDeque::new(),
            current: None,
            last_id: None,
        }
    }

    fn add(&mut self, mut process: Process) -> Option<Id> {
        if self.last_id.is_none() {
            self.last_id = Some(0);
            self.current = Some(0);
        } else {
            self.last_id = Some(self.last_id.unwrap() + 1);
        }
        process.trap_frame.tpidr = self.last_id.unwrap();
        self.processes.push_back(process);
        self.last_id
    }

    fn switch(&mut self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        if self.processes.is_empty() {
            unsafe {
                asm!("wfi" :::: "volatile");
            }
        }
        let mut proc = self.processes.pop_front().expect("1");
        proc.state = new_state;
        *proc.trap_frame = *tf;

        self.processes.push_back(proc);
        loop {
            let mut new_proc = self.processes.pop_front().expect("2");
            if new_proc.is_ready() {
                new_proc.state = State::Running;
                self.current = Some(new_proc.trap_frame.tpidr);
                *tf = *new_proc.trap_frame;
                self.processes.push_front(new_proc);
                break;
            } else {
                self.processes.push_back(new_proc);
            }
        }
        Some(self.current.expect("3"))
    }
}
