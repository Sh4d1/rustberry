use super::stack::Stack;
use alloc::boxed::Box;
use arch::exceptions::TrapFrame;
use core::mem::replace;

pub type Id = u64;

pub type EventPollFn = Box<FnMut(&mut Process) -> bool + Send>;

pub enum State {
    Ready,
    Waiting(EventPollFn),
    Running,
}

use core::fmt;

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Ready => write!(f, "State::Ready"),
            State::Running => write!(f, "State::Running"),
            State::Waiting(_) => write!(f, "State::Waiting"),
        }
    }
}

#[derive(Debug)]
pub struct Process {
    pub trap_frame: Box<TrapFrame>,
    pub stack: Stack,
    pub state: State,
}

impl Process {
    pub fn new() -> Option<Process> {
        let stack = Stack::new();
        match stack {
            None => return None,
            _ => {}
        }
        Some(Process {
            trap_frame: Box::new(TrapFrame::default()),
            stack: stack.unwrap(),
            state: State::Ready,
        })
    }

    pub fn is_ready(&mut self) -> bool {
        true
        //        let mut mut_state = replace(&mut self.state, State::Ready);
        //        match &mut mut_state {
        //            State::Ready => true,
        //            State::Waiting(ref mut func) => {
        //                if !func(self) {
        //                    self.state = mut_state;
        //                    false
        //                } else {
        //                    true
        //                }
        //            }
        //            _ => false,
        //        }
    }
}
