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
#![no_main]

global_asm!(include_str!("asm/init.S"));

extern crate rustberry;
extern crate volatile;

#[no_mangle]
pub extern "C" fn main() {
    let mut mu = rustberry::io::mini_uart::MiniUart::new();
    mu.write_byte(65);
    loop {}
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
