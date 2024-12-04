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

const SYSCALL_INFO: [&'static str; 5] = [
    "SYSCALL_WRITE",
    "SYSCALL_EXIT",
    "SYSCALL_YIELD",
    "SYSCALL_GET_TIME",
    "SYSCALL_TASK_INFO",
];
pub fn print_sys_task_info(ti:&mut TaskInfo) {
    for i in 0..5{
        info!("syscall times {} {:?}", SYSCALL_INFO[i], ti.syscall_times[i]);
    }
    info!("task times now: {:?}ms", ti.task_call_times);
}
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    if ti.is_null() {
        return -1;
    }
    info!("Sys task info:");
    let tcb = get_current_task_info();
    unsafe {
        let task_info = &mut *ti;
        task_info.status = tcb.task_status;
        task_info.syscall_times = tcb.syscall_times;
        task_info.task_call_times = get_time_ms() - tcb.start_time as usize;
        print_sys_task_info(task_info);
        0
    }
}