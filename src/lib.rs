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
#![feature(alloc, allocator_api)]

#![no_std]

extern crate volatile;
extern crate spin;
#[macro_use]
extern crate once;
#[macro_use]
extern crate lazy_static;

extern crate alloc;

#[macro_use]
extern crate register;

#[macro_use]
pub mod io;
pub mod memory;
pub mod arch;

use memory::allocator::Allocator;
#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

