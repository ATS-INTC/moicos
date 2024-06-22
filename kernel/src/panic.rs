use crate::boot::SMP;
use core::sync::atomic::{AtomicUsize, Ordering};
use sbi_rt::*;
static SHUTDOWN_HART: AtomicUsize = AtomicUsize::new(SMP);

/// not_kernel panic
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{}: {}", info.location().unwrap(), info.message().unwrap());
    let online = SHUTDOWN_HART.fetch_sub(1, Ordering::SeqCst);
    if online <= 1 {
        system_reset(Shutdown, SystemFailure);
    }
    loop {}
}
