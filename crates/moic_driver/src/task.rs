//! Coroutine Control Block structures for more control.
//!

use alloc::{boxed::Box, vec::Vec};
use crate::{
    cap_queue::{CapQueue, Capability, DeviceCapTable},
    ready_queue::ReadyQueue,
};
use core::fmt::Display;
use spin::Mutex;
pub(crate) const TASK_META_ALIGN: usize = 6;
pub(crate) const MAX_PRIORITY: usize = 32;

/// The Identity of `Task`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub(crate)usize);

unsafe impl Send for TaskId {}
unsafe impl Sync for TaskId {}

impl TaskId {
    /// 
    pub const EMPTY: Self = Self(0);
    
    /// Assume that val is a valid `TaskId`.
    pub unsafe fn virt(val: usize) -> Self {
        Self(val)
    }

    /// 
    pub(crate) fn value(&self) -> usize {
        self.0
    }

    /// 
    pub fn manual_drop(self) {
        let raw_tid = self.0;
        let raw_meta = (raw_tid & (!0x3f)) as *mut TaskMeta;
        let boxed_meta = unsafe { Box::from_raw(raw_meta) };
        drop(boxed_meta);
    }
}

impl From<Box<TaskMeta>> for TaskId {
    fn from(value: Box<TaskMeta>) -> Self {
        let priority = value.priority;
        let is_preempt = value.is_preempt;
        let mut raw_meta_ptr = Box::into_raw(value) as usize;
        raw_meta_ptr |= (priority % MAX_PRIORITY) << 1;
        if is_preempt {
            raw_meta_ptr |= 1;
        }
        Self(raw_meta_ptr)
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw_meta: *const TaskMeta = self.into();
        write!(f, "{}", unsafe {&*raw_meta})
    }
}

#[repr(usize)]
#[derive(Debug)]
pub enum Status {
    Inited = 0,
    Ready = 1,
}

/// The `TaskMeta`
#[repr(C)]
pub struct TaskMeta {
    /// 
    pub ready_queue: ReadyQueue,
    /// 
    pub device_cap_table: DeviceCapTable,
    /// 
    pub send_cap_queue: CapQueue,
    /// 
    pub recv_cap_queue: CapQueue,
    /// 
    pub status: Status,
    /// 
    pub priority: usize,
    /// 
    pub is_preempt: bool,
    /// 
    pub lock: Mutex<()>,
}

impl TaskMeta {

    ///
    pub const fn init() -> Self {
        Self {
            ready_queue: ReadyQueue::EMPTY,
            device_cap_table: DeviceCapTable::EMPTY,
            send_cap_queue: CapQueue::EMPTY,
            recv_cap_queue: CapQueue::EMPTY,
            status: Status::Inited,
            priority: 0,
            is_preempt: false,
            lock: Mutex::new(()),
        }
    }

    /// 
    pub fn new(priority: usize, is_preempt: bool) -> TaskId {
        let task_meta = Box::new(TaskMeta {
            ready_queue: ReadyQueue::EMPTY,
            device_cap_table: DeviceCapTable::EMPTY,
            send_cap_queue: CapQueue::EMPTY,
            recv_cap_queue: CapQueue::EMPTY,
            status: Status::Inited,
            priority,
            is_preempt,
            lock: Mutex::new(()),
        });
        TaskId::from(task_meta)
    }

    /// 
    pub fn device_cap(&self) -> &DeviceCapTable {
        &self.device_cap_table
    }

    /// 
    pub fn send_cap(&self) -> Vec<Capability> {
        self.send_cap_queue.inner.iter().map(|c| c.clone()).collect()
    }

    /// 
    pub fn recv_cap(&self) -> Vec<Capability> {
        self.recv_cap_queue.inner.iter().map(|c| c.clone()).collect()
    }
}

impl From<&TaskId> for *const TaskMeta {
    fn from(value: &TaskId) -> Self {
        let tid = value.0;
        let raw_meta_ptr = tid & (!0x3f);
        raw_meta_ptr as _
    }
}

impl From<TaskId> for &mut TaskMeta {
    fn from(value: TaskId) -> Self {
        let tid = value.0;
        let raw_meta_ptr = tid & (!0x3f);
        unsafe { &mut *(raw_meta_ptr as *mut TaskMeta) }
    }
}

impl Drop for TaskMeta {
    fn drop(&mut self) {
    }
}

impl Display for TaskMeta {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TaskMeta(
{:X?},
SendCap: {:X?},
RecvCap: {:X?},
Status: {:?},
Priority: {},
)", 
            self.ready_queue,
            self.send_cap_queue,
            self.recv_cap_queue,
            self.status,
            self.priority
        )
    }
}