//! App management syscalls

use crate::task::{exit_current_and_run_next, get_current_task_info, suspend_current_and_run_next};
use crate::timer::{get_time_ms, get_time_us};
use log::*;
use crate::task::task::TaskInfo;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_us() as isize
}

pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    info!("[kernel] sys_task_info");
    let tcb = get_current_task_info();
    unsafe {
        *_ti = TaskInfo {
            status: tcb.task_status,
            syscall_times: tcb.syscall_times,
            task_call_times: get_time_ms() - tcb.start_time as usize,
        }
    };
    0
}