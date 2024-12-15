mod context;
mod switch;

#[allow(clippy::module_inception)]
pub mod task;

use log::info;
use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::timer::{get_time_ms, get_time_us};
use crate::syscall::MAX_SYSCALL_NUM;
use lazy_static::*;
//use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

static mut SWITCH_TIME_START: usize = 0;
static mut SWITCH_TIME: usize = 0;


//implement switch and calc cost time
unsafe fn __switch(current_task_cx_ptr: *mut TaskContext,next_task_cx_ptr: *const TaskContext) {
    SWITCH_TIME_START = get_time_us();
    switch::__switch(current_task_cx_ptr, next_task_cx_ptr);
    SWITCH_TIME += get_time_us() - SWITCH_TIME_START;
}

fn get_switch_time() ->usize {
    unsafe { SWITCH_TIME}
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
    stop_watch: usize,
}

impl TaskManagerInner {
    fn refresh_stop_watch(&mut self) -> usize {
        let start_time = self.stop_watch;
        self.stop_watch = get_time_ms();
        self.stop_watch - start_time
    }
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

//initialize TaskManager
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            user_time: 0,
            kernel_time: 0,
            start_time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                    stop_watch: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        task0.start_time = get_time_ms();
        inner.refresh_stop_watch();
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as * mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task");
    }

    //running -> ready
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        inner.tasks[current].task_status = TaskStatus::Ready;           
    }

    //running -> exit
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
        info!("task_{} exited. user_time: {} ms, kernel_time: {} ms.", current, inner.tasks[current].user_time, inner.tasks[current].kernel_time);
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
    //find ready app
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            if inner.tasks[next].start_time == 0 {
                inner.tasks[next].start_time = get_time_ms();
            }
            inner.current_task = next;
            info!("Task_{} Running", next);
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All applications completed!");
            println!("Task_switch_time {} us", get_switch_time());
            shutdown(false);
        }
    }
    //calc kernel time: now - st
    fn kernel_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].kernel_time += inner.refresh_stop_watch();
    }
    //calc user time: now - st
    fn user_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].user_time += inner.refresh_stop_watch();
    }
    //get task info
    fn get_current_task_info(&self) -> TaskControlBlock {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current]
    }

    fn record_syscall_times(&self, syscall_id: usize){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[syscall_id] += 1;
    }
}
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    info!("Suspend task_{}", TASK_MANAGER.inner.exclusive_access().current_task);
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    info!("Exited task_{}", TASK_MANAGER.inner.exclusive_access().current_task);
    run_next_task();
}

pub fn kernel_time_end() {
    TASK_MANAGER.kernel_time_end();
}

pub fn user_time_end() {
    TASK_MANAGER.user_time_end();
}

pub fn get_current_task_info() -> TaskControlBlock{
    TASK_MANAGER.get_current_task_info()
}

pub fn record_syscall_times(syscall_id: usize) {
    TASK_MANAGER.record_syscall_times(syscall_id);
}