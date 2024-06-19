#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]


mod boot;
#[macro_use]
mod console;
mod panic;
mod heap;

#[macro_use]
extern crate log;
extern crate alloc;

use alloc::vec::Vec;
pub use boot::hart_id;

pub fn rust_main(hartid: usize) {
    info!("into rust main");
    let mut vec = Vec::new();
    vec.push(45);
    println!("{:?}", vec.pop());
}

pub fn rust_main_secondary(hartid: usize) {
    info!("into rust main secondary");
}

