//! Process management syscalls
use core::mem::{self, size_of, size_of_val};

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{copy_into_translated_byte_buffer, translated_byte_buffer, VirtAddr}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};

/// time in sec and usec
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// time in sec
    pub sec: usize,
    /// time in usec
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}



/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();

    let result = TimeVal{
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let reuslt_in_bytes: [u8; mem::size_of::<TimeVal>()] = unsafe {
        mem::transmute::<TimeVal,[u8; mem::size_of::<TimeVal>()] >(result)
    };

    let len = mem::size_of::<TimeVal>();

    copy_into_translated_byte_buffer(current_user_token(), ts as *const u8, len, reuslt_in_bytes);

    0
}


use crate::task::TASK_MANAGER;

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let mut result = TASK_MANAGER.get_current_task_control_block();

    let start_time = result.time;
    let now = get_time_us() / 1000;
    result.time = now - start_time;
    let reuslt_in_bytes: [u8; mem::size_of::<TaskInfo>()] = unsafe {
        mem::transmute::<TaskInfo,[u8; mem::size_of::<TaskInfo>()] >(result)
    };

    copy_into_translated_byte_buffer(current_user_token(), ti as *const u8, mem::size_of::<TaskInfo>(), reuslt_in_bytes);

    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if start % PAGE_SIZE > 0{
        return -1;
    }
    if port & !0x7 != 0 {
        return -1;
    }

    if port & 0x7 == 0 {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    let pages = (len + PAGE_SIZE -1 ) / PAGE_SIZE;

    TASK_MANAGER.mmap(start, pages, port)
}

/// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let pages = (len + PAGE_SIZE -1 ) / PAGE_SIZE;
    TASK_MANAGER.munmap(start, pages)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
