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

extern crate volatile;
extern crate spin;
#[macro_use]
extern crate once;
#[macro_use]
extern crate lazy_static;

pub mod io;
pub mod memory;
