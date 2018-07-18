use alloc::alloc::Layout;
use HEAP_ALLOCATOR;
pub fn init_heap() {
    let (start, end) = memory_map().expect("failed to find memory map");
    unsafe {
        HEAP_ALLOCATOR.lock().init(start, end - start);
    }
}

//#[lang = "oom"]
#[no_mangle]
#[alloc_error_handler]
pub fn rust_oom(_layout: Layout) -> ! {
    panic!("out of memory");
}

extern "C" {
    static _end: u8;
}

use memory::atags::{Atag, Atags};

fn memory_map() -> Option<(usize, usize)> {
    let binary_end = unsafe { (&_end as *const u8) as u32 };
    let mut size = 0;
    for v in Atags::get() {
        match v {
            Atag::Mem(mem) => {
                size = mem.size;
            }
            _ => {} // not used for now at least
        }
    }

    Some((binary_end as usize, (size - binary_end) as usize))
}
