//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

pub const MAX_SYSCALL_NUM: usize = 5;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

const SYSCALL_INFO_WRITE: usize = 0;
const SYSCALL_INFO_EXIT: usize = 1;
const SYSCALL_INFO_YIELD: usize = 2;
const SYSCALL_INFO_GET_TIME: usize = 3;
const SYSCALL_INFO_TASK_INFO: usize = 4;
const SYSCALL_INFO_ERROR_ID: usize = 1023;

mod fs;
mod process;

use fs::*;
use crate::task::{record_syscall_times, task::TaskInfo};
use process::*;

//syscall_id -> syscal_info_id
fn get_syscall_info_id(syscall_id: usize) -> usize{
    match syscall_id {
        SYSCALL_WRITE         => SYSCALL_INFO_WRITE,
        SYSCALL_EXIT          => SYSCALL_INFO_EXIT,
        SYSCALL_YIELD         => SYSCALL_INFO_YIELD,
        SYSCALL_GET_TIME      => SYSCALL_INFO_GET_TIME,
        SYSCALL_TASK_INFO     => SYSCALL_INFO_TASK_INFO,
        _ => {println!("Unsupported syscall id!"); SYSCALL_INFO_ERROR_ID}
    }
}
/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    let syscall_info_id = get_syscall_info_id(syscall_id);
    if syscall_info_id != SYSCALL_INFO_ERROR_ID {
        record_syscall_times(syscall_info_id);
    }
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_TASK_INFO => {
            let task_info_ptr = args[0] as *mut TaskInfo;
            if !task_info_ptr.is_null() {
                sys_task_info(task_info_ptr)
            } else {
                //bad ptr
                panic!("Invalid pointer passed to sys_task_info");
            }
        },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
