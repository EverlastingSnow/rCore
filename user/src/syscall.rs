use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME:usize = 169;
const SYSCALL_TASK_INFO: usize = 410;


//call env call
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe{
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; 5],
    pub task_call_times: usize
}


pub fn sys_task_info(ptr:*mut TaskInfo) -> isize {
    syscall(SYSCALL_TASK_INFO, [ptr as usize, 0, 0])
}