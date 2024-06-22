use crate::boot::SMP;
use array_init::array_init;
use moic_driver::{Moic, MoicErr, TaskId, TaskMeta};
use riscv::register::{sideleg, sie, sip, sstatus, uie};
use spin::Lazy;

pub static MOICS: Lazy<[Moic; SMP]> =
    Lazy::new(|| array_init(|i| Moic::new(0x1000000 + i * 0x1000)));

static OS_TCB: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(0, false));
static PROC_TCB: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(0, false));

/// 这个测试用于测试 OS 与 process 之间切换时，控制器内的任务队列与内存之间的交互是否正常工作
/// 这个测试必须在 SMP=4 的情况下进行。
/// 1. 主核和副核都将先进入 OS，先添加 3 个任务到 os 中，副核将会取出 1 个任务。
/// 2. 主核切换出 OS，并尝试取出任务，将会提示出错
/// 3. 主核切换回 OS，从中取出任务
/// 4. 主核与副核切换到 process，各自添加 3 个任务到 process 中，主核运行的 process 取出全部的任务
/// 5. 主核与副核都切换回 OS，副核退出，主核运行的 OS 取出剩下的任务
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

/// 这个测试用于测试 OS 与 process 之间切换时，控制器内的外设中断处理任务表与内存之间的交互是否正常工作
/// 主核与副核先进入到 OS 中，注册外设的中断处理任务，再切换到 process，注册外设的中断处理任务
/// 主核与副核全部退出 process 后，查看 process 的外设的中断处理函数表是否正常
/// 主核与副核全部退出 OS 后，查看 OS 的外设的中断处理函数表是否正常
pub fn moic_test_device_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x19990109) },
            TaskId::EMPTY,
            TaskId::EMPTY,
            unsafe { TaskId::virt(i) },
        );
    }
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x1999) },
            TaskId::EMPTY,
            TaskId::EMPTY,
            unsafe { TaskId::virt(i) },
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
            unsafe { TaskId::virt(i) },
        );
    }
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    for i in 0..6 {
        MOICS[hartid].register_receiver(
            unsafe { TaskId::virt(0x1999) },
            TaskId::EMPTY,
            TaskId::EMPTY,
            unsafe { TaskId::virt(i) },
        );
    }
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

/// 这个测试用于测试 OS 与 process 之间切换时，控制器内的 IPC 接收能力表与内存之间的交互是否正常工作
/// 主核与副核先进入到 OS 中，注册 IPC 接收方，再切换到 process，注册 IPC 接收方
/// 主核与副核全部退出 process 后，查看 process 的 IPC 接收方表是否正常
/// 主核与副核全部退出 OS 后，查看 OS 的 IPC 接收方表是否正常
pub fn moic_test_recv_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x19990109) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x2899) },
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    info!("{}", *OS_TCB);
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x1999) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x8979) },
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
        unsafe { TaskId::virt(0x2899) },
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_receiver(
        unsafe { TaskId::virt(0x8976) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x98545) },
    );
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

/// 这个测试用于测试 OS 与 process 之间切换时，控制器内的 IPC 发送能力表与内存之间的交互是否正常工作
/// 主核与副核先进入到 OS 中，注册 IPC 发送方，再切换到 process，注册 IPC 发送方
/// 主核与副核全部退出 process 后，查看 process 的 IPC 发送方表是否正常
/// 主核与副核全部退出 OS 后，查看 OS 的 IPC 发送方表是否正常
pub fn moic_test_send_cap_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x19990109) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x2899) },
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    info!("{}", *OS_TCB);
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x1999) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x8979) },
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
        unsafe { TaskId::virt(0x2899) },
    );
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_sender(
        unsafe { TaskId::virt(0x8976) },
        *OS_TCB,
        TaskId::EMPTY,
        unsafe { TaskId::virt(0x98545) },
    );
    MOICS[hartid].switch_process(None);
    MOICS[hartid].switch_os(None);
}

static SENDER_TASK: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(4, false));
static RECEIVER_TASK: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(4, true));

/***************** 以下的测试必须要在 SMP=2 的条件下测试，并且测试可能会出现 qemu 不能退出的情况 *****************/

/// 这个测试用于测试 OS 向 OS 发送中断
/// 主核注册发送方，副核注册接收方
/// 主核发送中断后，副核从控制器中取出任务
pub fn os_send_intr_os_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_sender(*SENDER_TASK, *OS_TCB, TaskId::EMPTY, *RECEIVER_TASK);
    MOICS[hartid].add(*SENDER_TASK);
    let send_task = MOICS[hartid].fetch().unwrap();
    MOICS[hartid].send_intr(*OS_TCB, TaskId::EMPTY, *RECEIVER_TASK);
}

pub fn os_send_intr_os_secondary(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(*RECEIVER_TASK, *OS_TCB, TaskId::EMPTY, *SENDER_TASK);
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

/// 这个测试用于测试 OS 向 process 发送中断
/// 主核运行 OS 注册发送方，副核运行 process 注册接收方
/// 主核发送中断后，副核 process 从控制器中取出任务
pub fn os_send_intr_process_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_sender(*SENDER_TASK, *OS_TCB, *PROC_TCB, *RECEIVER_TASK);
    MOICS[hartid].add(*SENDER_TASK);
    let send_task = MOICS[hartid].fetch().unwrap();
    MOICS[hartid].send_intr(*OS_TCB, *PROC_TCB, *RECEIVER_TASK);
}

pub fn os_send_intr_process_secondary(hartid: usize) {
    unsafe {
        sstatus::set_spp(sstatus::SPP::User);
        sideleg::set_usoft();
        uie::set_usoft();
    };
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_receiver(*RECEIVER_TASK, *OS_TCB, TaskId::EMPTY, *SENDER_TASK);
    info!("{:X?}", *RECEIVER_TASK);
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

/// 这个测试用于测试 OS 向 process 发送中断
/// 主核运行 process 注册发送方，副核运行 OS 注册接收方
/// 主核发送中断后，副核 OS 从控制器中取出任务
pub fn process_send_intr_os_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_sender(*SENDER_TASK, *OS_TCB, TaskId::EMPTY, *RECEIVER_TASK);
    MOICS[hartid].add(*SENDER_TASK);
    let send_task = MOICS[hartid].fetch().unwrap();
    MOICS[hartid].send_intr(*OS_TCB, TaskId::EMPTY, *RECEIVER_TASK);
}

pub fn process_send_intr_os_secondary(hartid: usize) {
    unsafe {
        sstatus::set_sie();
        sie::set_ssoft();
    };
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(*RECEIVER_TASK, *OS_TCB, *PROC_TCB, *SENDER_TASK);
    info!("{:X?}", *RECEIVER_TASK);
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

static PROC2_TCB: Lazy<TaskId> = Lazy::new(|| TaskMeta::new(4, false));
/// 这个测试用于测试 process 向 process 发送中断
/// 主核运行 process 注册发送方，副核运行 process 注册接收方
/// 主核发送中断后，副核 process 从控制器中取出任务
pub fn process_send_intr_process_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_sender(*SENDER_TASK, *OS_TCB, *PROC2_TCB, *RECEIVER_TASK);
    MOICS[hartid].add(*SENDER_TASK);
    let send_task = MOICS[hartid].fetch().unwrap();
    MOICS[hartid].send_intr(*OS_TCB, *PROC2_TCB, *RECEIVER_TASK);
}

pub fn process_send_intr_process_secondary(hartid: usize) {
    unsafe {
        sstatus::set_spp(sstatus::SPP::User);
        sideleg::set_usoft();
        uie::set_usoft();
    };
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].switch_process(Some(*PROC2_TCB));
    MOICS[hartid].register_receiver(*RECEIVER_TASK, *OS_TCB, *PROC_TCB, *SENDER_TASK);
    info!("{:X?}", *RECEIVER_TASK);
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

/// 这个测试用于测试 process 向不在线的 process 发送中断
/// 主核运行 process 注册发送方，副核切换到 process 注册接收方后切换回 OS
/// 主核发送中断后，副核 OS 从控制器中取出 receive process，在切换到 receive process 后，从控制器中取出被唤醒的任务
pub fn process_send_intr_process_not_online_main(hartid: usize) {
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].switch_process(Some(*PROC_TCB));
    MOICS[hartid].register_sender(*SENDER_TASK, *OS_TCB, *PROC2_TCB, *RECEIVER_TASK);
    MOICS[hartid].add(*SENDER_TASK);
    let send_task = MOICS[hartid].fetch().unwrap();
    MOICS[hartid].send_intr(*OS_TCB, *PROC2_TCB, *RECEIVER_TASK);
}

pub fn process_send_intr_process_not_online_secondary(hartid: usize) {
    unsafe {
        sstatus::set_spp(sstatus::SPP::User);
        sideleg::set_usoft();
        uie::set_usoft();
    };
    MOICS[hartid].switch_os(Some(*OS_TCB));
    MOICS[hartid].register_receiver(*PROC2_TCB, *OS_TCB, *PROC_TCB, *SENDER_TASK);
    MOICS[hartid].switch_process(Some(*PROC2_TCB));
    MOICS[hartid].register_receiver(*RECEIVER_TASK, *OS_TCB, *PROC_TCB, *SENDER_TASK);
    MOICS[hartid].switch_process(None);
    // info!("{:X?}", *PROC2_TCB);
    let mut item = MOICS[hartid].fetch();
    while item.is_err() {
        if item.clone().unwrap_err() == MoicErr::FetchErr {
            break;
        }
        item = MOICS[hartid].fetch();
    }
    info!("{:X?}", item);
    // item.unwrap().manual_drop();
    MOICS[hartid].switch_process(Some(item.unwrap()));
    info!("{:X?}", *RECEIVER_TASK);
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
