use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::Unique;
use memory::virtual_mem::PhysicalAddr;
//use ALLOCATOR;
use HEAP_ALLOCATOR;
pub struct Stack {
    ptr: Unique<[u8; Stack::SIZE]>,
}

impl Stack {
    pub const SIZE: usize = 1 << 20;
    pub const ALIGN: usize = 16;

    fn layout() -> Layout {
        unsafe { Layout::from_size_align_unchecked(Self::SIZE, Self::ALIGN) }
    }

    pub fn new() -> Option<Stack> {
        let raw_ptr = unsafe {
            let raw_ptr: *mut u8 = HEAP_ALLOCATOR.alloc(Stack::layout());
            if raw_ptr == (0 as *mut u8) {
                return None;
            }
            raw_ptr.write_bytes(0, Self::SIZE);
            raw_ptr
        };
        let ptr = Unique::new(raw_ptr as *mut _).expect("non-null");
        Some(Stack { ptr: ptr })
    }

    unsafe fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr() as *mut u8
    }

    pub fn top(&self) -> PhysicalAddr {
        unsafe { self.as_mut_ptr().add(Self::SIZE).into() }
    }

    pub fn bottom(&self) -> PhysicalAddr {
        unsafe { self.as_mut_ptr().into() }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { HEAP_ALLOCATOR.dealloc(self.as_mut_ptr(), Self::layout()) }
    }
}

use core::fmt;

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Stack")
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .field("size", &Self::SIZE)
            .finish()
    }
}
