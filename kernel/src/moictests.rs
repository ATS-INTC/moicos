use moic_driver::{Moic, MoicErr, TaskId, TaskMeta};
use spin::Lazy;
use crate::boot::SMP;
use array_init::array_init;

pub static MOICS: Lazy<[Moic; SMP]> = Lazy::new(|| 
    array_init(|i| Moic::new(0x1000000 + i * 0x1000))
);

static OS_TCB: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(0, false));
static PROC_TCB: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(0, false));

pub fn moic_test_rq_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].switch_os(None);
    for _ in 0..9 {
        let mut item = MOICS[hartid].fetch();
        while item.is_err() {
            if item.clone().unwrap_err() == MoicErr::FetchErr {
                break;
            }
            item = MOICS[hartid].fetch();
        }
        info!("{:X?}", item);
    }
    log::info!("{}", *OS_TCB);
    MOICS[hartid].switch_os(Some(*OS_TCB));
    log::info!("{}", *OS_TCB);
    for _ in 0..7 {
        let mut item = MOICS[hartid].fetch();
        while item.is_err() {
            if item.clone().unwrap_err() == MoicErr::FetchErr {
                break;
            }
            item = MOICS[hartid].fetch();
        }
        info!("{:X?}", item);
        item.unwrap().manual_drop();
    }
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    for _ in 0..12 {
        let mut item = MOICS[hartid].fetch();
        while item.is_err() {
            if item.clone().unwrap_err() == MoicErr::FetchErr {
                break;
            }
            item = MOICS[hartid].fetch();
        }
        info!("{:X?}", item);
        item.unwrap().manual_drop();
    }
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
    log::info!("{}", *OS_TCB);
    MOICS[hartid].switch_os(Some(*OS_TCB));
    for _ in 0..2 {
        let mut item = MOICS[hartid].fetch();
        while item.is_err() {
            if item.clone().unwrap_err() == MoicErr::FetchErr {
                break;
            }
            item = MOICS[hartid].fetch();
        }
        info!("{:X?}", item);
        item.unwrap().manual_drop();
    }
}

pub fn moic_test_rq_secondary(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    let item = MOICS[hartid].fetch();
    item.unwrap().manual_drop();
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].add(TaskMeta::new(1, false));
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

pub fn moic_test_device_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x19990109) }, 
            TaskId::EMPTY, 
            TaskId::EMPTY, 
            unsafe { TaskId::virt(i) }
        );
    }
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x1999) }, 
            TaskId::EMPTY, 
            TaskId::EMPTY, 
            unsafe { TaskId::virt(i) }
        );
    }
    MOICS[hartid].switch_process(None);
    info!("{}", *PROC_TCB);
    MOICS[hartid].switch_os(None);
    info!("{}", *OS_TCB);
}

pub fn moic_test_device_cap_secondary(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x19990109) }, 
            TaskId::EMPTY, 
            TaskId::EMPTY, 
            unsafe { TaskId::virt(i) }
        );
    }
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x1999) }, 
            TaskId::EMPTY, 
            TaskId::EMPTY, 
            unsafe { TaskId::virt(i) }
        );
    }
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

pub fn moic_test_recv_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x19990109) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x2899) }
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    info!("{}", *OS_TCB);
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x1999) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x8979) }
    );
    MOICS[hartid].switch_process(None);
    info!("{}", *PROC_TCB);
    info!("{}", *OS_TCB);
    MOICS[hartid].switch_os(None);
    info!("{}", *PROC_TCB);
    info!("{}", *OS_TCB);
}

pub fn moic_test_recv_cap_secondary(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x19990109) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x2899) }
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x8976) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x98545) }
    );
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

pub fn moic_test_send_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x19990109) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x2899) }
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    info!("{}", *OS_TCB);
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x1999) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x8979) }
    );
    MOICS[hartid].switch_process(None);
    info!("{}", *PROC_TCB);
    info!("{}", *OS_TCB);
    MOICS[hartid].switch_os(None);
    info!("{}", *PROC_TCB);
    info!("{}", *OS_TCB);
}

pub fn moic_test_send_cap_secondary(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x19990109) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x2899) }
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x8976) }, 
        *OS_TCB, 
        TaskId::EMPTY, 
        unsafe { TaskId::virt(0x98545) }
    );
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

