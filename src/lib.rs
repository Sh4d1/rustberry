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

pub mod io;
