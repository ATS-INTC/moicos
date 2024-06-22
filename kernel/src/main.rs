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
mod moictests;
mod panic;

#[macro_use]
extern crate log;
extern crate alloc;

pub use boot::hart_id;

pub fn rust_main(hartid: usize) {
    // moictests::moic_test_rq_main(hartid);
    // moictests::moic_test_device_cap_main(hartid);
    // moictests::moic_test_recv_cap_main(hartid);
    // moictests::moic_test_send_cap_main(hartid);
    // moictests::os_send_intr_os_main(hartid);
    // moictests::os_send_intr_process_main(hartid);
    // moictests::process_send_intr_os_main(hartid);
    // moictests::process_send_intr_process_main(hartid);
    moictests::process_send_intr_process_not_online_main(hartid);
}

pub fn rust_main_secondary(hartid: usize) {
    // moictests::moic_test_rq_secondary(hartid);
    // moictests::moic_test_device_cap_secondary(hartid);
    // moictests::moic_test_recv_cap_secondary(hartid);
    // moictests::moic_test_send_cap_secondary(hartid);
    // moictests::os_send_intr_os_secondary(hartid);
    // moictests::os_send_intr_process_secondary(hartid);
    // moictests::process_send_intr_os_secondary(hartid);
    // moictests::process_send_intr_process_secondary(hartid);
    moictests::process_send_intr_process_not_online_secondary(hartid);
}
