//! This is the driver of moic(Multiple-object-interaction interrupt controller)
//!

#![no_std]
#![deny(missing_docs)]

use core::sync::atomic::Ordering;
pub use cap_queue::Capability;
use task::TASK_META_ALIGN;
pub use task::{TaskMeta, TaskId};
extern crate alloc;

mod cap_queue;
mod ready_queue;
mod task;
mod error;

pub use error::MoicErr;


/// moic
#[derive(Debug, Clone, Copy)]
pub struct Moic(usize);

impl Moic {
    ///
    pub const fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    /// the mmio registers
    fn regs(&self) -> &'static pac::moic::Hart {
        unsafe { &*(self.0 as *const _) }
    }

    /// Add a task
    pub fn add(&self, task_id: TaskId) {
        self.regs().add().write(|w| unsafe { w.bits(task_id.value() as _) });
    }

    /// 
    pub fn fetch(&self) -> Result<TaskId, MoicErr> {
        let raw_task_id = self.regs().fetch().read().bits() as i64;
        if raw_task_id > 0 {
            Ok(TaskId(raw_task_id as _))
        } else if raw_task_id == 0 {
            Err(MoicErr::NoTask)
        } else {
            Err(MoicErr::FetchErr)
        }
    }

    /// 
    pub fn switch_hypervisor(&self, hypervisor_id: TaskId) {
        self.regs().switch_hypervisor().write(|w| unsafe { w.bits(hypervisor_id.value() as _) })
    }

    /// 
    pub fn switch_os(&self, os_id: Option<TaskId>) {
        if os_id.is_none() {
            let current = unsafe { &mut *((self.regs().current().read().tcb().bits() << TASK_META_ALIGN) as *mut TaskMeta) };
            let lock = current.lock.lock();
            let ocnt = self.regs().status().read().ocnt().bits();
            if ocnt <= 1 {
                let rq_count = current.ready_queue.count;
                current.ready_queue.inner.reserve(rq_count);
                let recv_cap_count = current.recv_cap_queue.count;
                current.recv_cap_queue.inner.reserve(recv_cap_count);
                let send_cap_count = current.send_cap_queue.count;
                current.send_cap_queue.inner.reserve(send_cap_count);
            }
            self.regs().switch_os().write(|w| unsafe { 
                w.bits(0) 
            });
            drop(lock);
        } else {
            self.regs().switch_os().write(|w| unsafe { 
                w.bits(os_id.unwrap().value() as _) 
            });
        }
    }

    /// This interface is used for `os -> process` or `process -> os`.
    pub fn switch_process(&self, process_id: Option<TaskId>) {
        let current = unsafe { &mut *((self.regs().current().read().tcb().bits() << TASK_META_ALIGN) as *mut TaskMeta) };
        let lock = current.lock.lock();
        let ocnt = self.regs().status().read().ocnt().bits();
        if ocnt <= 1 {
            let rq_count = current.ready_queue.count;
            current.ready_queue.inner.reserve(rq_count);
            let recv_cap_count = current.recv_cap_queue.count;
            current.recv_cap_queue.inner.reserve(recv_cap_count);
            let send_cap_count = current.send_cap_queue.count;
            current.send_cap_queue.inner.reserve(send_cap_count);
        }
        self.regs().switch_process().write(|w| unsafe { 
            w.bits(process_id.map_or(0, |tid| tid.value()) as _) 
        });
        drop(lock);
    }

    /// 
    pub fn register_sender(&self, send_task_id: TaskId, recv_os_id: TaskId, recv_proc_id: TaskId, recv_task_id: TaskId) {
        self.regs().register_send_task().write(|w| unsafe {
            w.bits(send_task_id.value() as _)
        });
        self.regs().register_send_target_os().write(|w| unsafe {
            w.bits(recv_os_id.value() as _)
        });
        self.regs().register_send_target_proc().write(|w| unsafe {
            w.bits(recv_proc_id.value() as _)
        });
        self.regs().register_send_target_task().write(|w| unsafe {
            w.bits(recv_task_id.value() as _)
        });
    }

    /// 
    pub fn register_receiver(&self, recv_task_id: TaskId, send_os_id: TaskId, send_proc_id: TaskId, send_task_id: TaskId) {
        self.regs().register_recv_task().write(|w| unsafe {
            w.bits(recv_task_id.value() as _)
        });
        self.regs().register_recv_target_os().write(|w| unsafe {
            w.bits(send_os_id.value() as _)
        });
        self.regs().register_recv_target_proc().write(|w| unsafe {
            w.bits(send_proc_id.value() as _)
        });
        self.regs().register_recv_target_task().write(|w| unsafe {
            w.bits(send_task_id.value() as _)
        });
    }

    /// 
    pub fn send_intr(&self, recv_os_id: TaskId, recv_proc_id: TaskId, recv_task_id: TaskId) {
        self.regs().send_intr_os().write(|w| unsafe {
            w.bits(recv_os_id.value() as _)
        });
        self.regs().send_intr_proc().write(|w| unsafe {
            w.bits(recv_proc_id.value() as _)
        });
        self.regs().send_intr_task().write(|w| unsafe {
            w.bits(recv_task_id.value() as _)
        });
    }

    /// 
    pub fn remove_task(&self, task: TaskId) {
        self.regs().remove().write(|w| unsafe {
            w.bits(task.value() as _)
        })
    }

}
