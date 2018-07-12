#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(global_asm)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(alloc, allocator_api)]

#[cfg(not(test))]
global_asm!(include_str!("asm/init.S"));

#[cfg(test)]
extern crate std;

#[macro_use]
extern crate rustberry;
extern crate volatile;
#[macro_use]
extern crate alloc;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() {
    rustberry::ALLOCATOR.initialize();
    rustberry::io::console::CONSOLE.lock().init();
    for v in rustberry::memory::atags::Atags::get() {
        kprintln!("{:?}", v);
    }
    use alloc::boxed::Box;

    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test2 = Box::new("hello");
    kprintln!("{:?} {:?}", heap_test, heap_test2);
    use rustberry::arch::registers::current_el::*;;
    unsafe {
        asm!("brk 3" :::: "volatile");
    }
    unsafe {
        asm!("svc 3" :::: "volatile");
    }
    loop {
        let c = rustberry::io::console::CONSOLE.lock().read_byte();
        kprint!("{}", c as char);
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub extern "C" fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}
