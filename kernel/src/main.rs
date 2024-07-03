#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod boot;
#[macro_use]
mod console;
mod heap;
#[allow(unused)]
mod moictests;
mod panic;

#[macro_use]
extern crate log;
extern crate alloc;

pub use boot::hart_id;

pub fn rust_main(hartid: usize) {
    unsafe { unified_task::trap_entry() };
}

pub fn rust_main_secondary(hartid: usize) {
}
