use alloc::allocator::{Alloc, AllocErr, GlobalAlloc, Layout};

use super::*;
use core::ptr::NonNull;

#[derive(Debug)]
pub struct Allocator {
    current: usize,
    end: usize,
}

impl Allocator {
    pub fn new(start: usize, end: usize) -> Allocator {
        Allocator {
            current: start,
            end: end,
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.current, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());
        if alloc_end <= self.end {
            self.current = alloc_end;
            return alloc_start as *mut u8;
        } else {
            return 0 as *mut u8;
        }
    }

    pub fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {}
}
