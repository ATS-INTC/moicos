use crate::{rust_main, rust_main_secondary};
use core::sync::atomic::{Ordering, AtomicUsize};

/// Boot CPU_NUM
pub const SMP: usize = 4;

/// Boot kernel size allocated in `__entry` for single CPU.
const BOOT_STACK_SIZE: usize = 0x4_0000;

/// Total boot kernel size.
const TOTAL_BOOT_STACK_SIZE: usize = BOOT_STACK_SIZE * SMP;

/// Initialize kernel stack in .bss section.
#[link_section = ".bss.stack"]
static mut STACK: [u8; TOTAL_BOOT_STACK_SIZE] = [0u8; TOTAL_BOOT_STACK_SIZE];

/// Entry for the first kernel.
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn __entry(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_init,
        options(noreturn),
    )
}

/// Entry for other kernels.
#[naked]
#[no_mangle]
unsafe extern "C" fn __entry_others(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",
        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_secondary_init,
        options(noreturn),
    )
}

#[inline]
pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

extern "C" {
    fn sbss();
    fn ebss();
}

fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

static BOOT_HART: AtomicUsize = AtomicUsize::new(0);


pub fn rust_main_init(hartid: usize) {
    clear_bss();
    crate::console::init(option_env!("LOG"));
    crate::heap::init_heap();
    BOOT_HART.fetch_add(1, Ordering::Relaxed);
    if SMP > 1 {
        for i in 0..SMP {
            let boot_hart_cnt = BOOT_HART.load(Ordering::SeqCst);
            if i != hartid {
                // Starts other harts.
                let ret = sbi_rt::hart_start(i, __entry_others as _, 0);
                assert!(ret.is_ok(), "Failed to shart hart {}", i);
                while BOOT_HART.load(Ordering::SeqCst) == boot_hart_cnt {}
            }
        }
    }
    rust_main(hartid);
    unreachable!();
}

pub fn rust_main_secondary_init(hartid: usize) {
    BOOT_HART.fetch_add(1, Ordering::SeqCst);
    rust_main_secondary(hartid);
    unreachable!();
}