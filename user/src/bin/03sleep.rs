#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_, task_info};
use user_lib::syscall::{TaskInfo, TaskStatus};

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        yield_();
    }
    let mut task = TaskInfo {
        status: TaskStatus::UnInit,
        syscall_times: [0; 5],
        task_call_times: 0
    };
    let ptr: *mut TaskInfo = &mut task;
    task_info(ptr);
    println!("Test sleep OK!");
    0
}