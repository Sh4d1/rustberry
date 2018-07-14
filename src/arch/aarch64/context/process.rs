use super::stack::Stack;
use alloc::boxed::Box;
use arch::exceptions::TrapFrame;

pub type Id = u64;

#[derive(PartialEq)]
pub enum State {
    Ready,
    Waiting,
    Running,
}

use core::fmt;

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Ready => write!(f, "State::Ready"),
            State::Running => write!(f, "State::Running"),
            State::Waiting => write!(f, "State::Waiting"),
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
        self.state == State::Ready
    }
}
