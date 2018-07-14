//pub mod bump;
#[path = "bump.rs"]
mod imp;

use alloc::allocator::{Layout, GlobalAlloc};
use spin::Mutex;

#[derive(Debug)]
pub struct Allocator(Mutex<Option<imp::Allocator>>);

impl Allocator {
    pub const fn uninitialized() -> Self {
        Allocator(Mutex::new(None))
    }

    pub fn initialize(&self) {
        let (start, end) = memory_map().expect("failed to find memory map");
        *self.0.lock() = Some(imp::Allocator::new(start, end));
    }
}

unsafe impl<'a> GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().as_mut().expect("allocator uninitialized").alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().as_mut().expect("allocator uninitialized").dealloc(ptr, layout);
    }
}

#[lang = "oom"]
#[no_mangle]
pub extern "C" fn oom() -> ! {
    panic!("Memory Allocation failed")
}

extern "C" {
    static _end: u8;
}

use memory::atags::{Atags,Atag};

fn memory_map() -> Option<(usize, usize)> {
    let binary_end = unsafe { (&_end as *const u8) as u32  };
    let mut size = 0;
    for v in Atags::get() {
        match v {
            Atag::Mem(mem) => {
                size = mem.size;
            },
            _ => {},
        }
    }
    
    Some((binary_end as usize, (size-binary_end) as usize))
}

pub fn align_down(addr: usize, align: usize) -> usize {
    if (align == 0) || ((align & (align - 1)) != 0) {
        panic!("align is not a power of 2")
    }
    addr & (!align + 1)
}

pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}


#[cfg(test)]
mod test_align {
    use super::*;

    #[test]
    fn test_align_down() {
        assert_eq!(align_down(0, 2), 0);
        assert_eq!(align_down(0, 8), 0);
        assert_eq!(align_down(0, 1 << 5), 0);

        assert_eq!(align_down(1 << 10, 1 << 10), 1 << 10);
        assert_eq!(align_down(1 << 20, 1 << 10), 1 << 20);
        assert_eq!(align_down(1 << 23, 1 << 4), 1 << 23);

        assert_eq!(align_down(1, 1 << 4), 0);
        assert_eq!(align_down(10, 1 << 4), 0);

        assert_eq!(align_down(0xFFFF, 1 << 2), 0xFFFC);
        assert_eq!(align_down(0xFFFF, 1 << 3), 0xFFF8);
        assert_eq!(align_down(0xFFFF, 1 << 4), 0xFFF0);
        assert_eq!(align_down(0xFFFF, 1 << 5), 0xFFE0);
        assert_eq!(align_down(0xAFFFF, 1 << 8), 0xAFF00);
        assert_eq!(align_down(0xAFFFF, 1 << 12), 0xAF000);
        assert_eq!(align_down(0xAFFFF, 1 << 16), 0xA0000);
    }

    #[test]
    #[should_panic]
    fn test_panics_align_down_1() {
        align_down(0xFFFF0000, 7);
    }

    #[test]
    #[should_panic]
    fn test_panics_align_down_2() {
        align_down(0xFFFF0000, 123);
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 2), 0);
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(0, 1 << 5), 0);

        assert_eq!(align_up(1 << 10, 1 << 10), 1 << 10);
        assert_eq!(align_up(1 << 20, 1 << 10), 1 << 20);
        assert_eq!(align_up(1 << 23, 1 << 4), 1 << 23);

        assert_eq!(align_up(1, 1 << 4), 1 << 4);
        assert_eq!(align_up(10, 1 << 4), 1 << 4);

        assert_eq!(align_up(0xFFFF, 1 << 2), 0x10000);
        assert_eq!(align_up(0xFFFF, 1 << 3), 0x10000);
        assert_eq!(align_up(0xFFFF, 1 << 4), 0x10000);
        assert_eq!(align_up(0xAFFFF, 1 << 12), 0xB0000);

        assert_eq!(align_up(0xABCDAB, 1 << 2), 0xABCDAC);
        assert_eq!(align_up(0xABCDAB, 1 << 4), 0xABCDB0);
        assert_eq!(align_up(0xABCDAB, 1 << 8), 0xABCE00);
        assert_eq!(align_up(0xABCDAB, 1 << 12), 0xABD000);
        assert_eq!(align_up(0xABCDAB, 1 << 16), 0xAC0000);
    }

    #[test]
    #[should_panic]
    fn test_panics_align_up_1() {
        align_up(0xFFFF0000, 7);
    }
    #[test]
    #[should_panic]
    fn test_panics_align_up_2() {
        align_up(0xFFFF0000, 456);
    }

}
