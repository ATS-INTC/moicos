use buddy_system_allocator::LockedHeap;
const KERNEL_HEAP_SIZE: usize = 0x400_0000;

#[no_mangle]
#[link_section = ".bss.memory"]
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0u8; KERNEL_HEAP_SIZE];

#[global_allocator]
static ALLOCATOR: LockedHeap<32> = LockedHeap::new();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout)
}

pub fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
