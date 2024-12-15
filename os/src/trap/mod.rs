//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

mod context;

use crate::{syscall::syscall, task::{suspend_current_and_run_next, user_time_end, kernel_time_end}, timer::set_next_trigger};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode, scause::{self, Exception, Trap}, sie, stval, stvec
};
use log::*;

//use rv64g to support calc fs
global_asm!(".attribute arch, \"rv64g\"", include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

static mut KERNEL_INTERRUPT_TRIGGERED: bool = false;

pub fn check_kernel_interrupt() -> bool {
    unsafe { (&raw mut KERNEL_INTERRUPT_TRIGGERED as *mut bool ).read_volatile()}
}

pub fn mark_kernel_interrupt() {
    unsafe {
        (&raw mut KERNEL_INTERRUPT_TRIGGERED as *mut bool).write_volatile(true);
    }
}

use riscv::register::sstatus;
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    match sstatus::read().spp() {
        sstatus::SPP::Supervisor => kernel_trap_handler(cx),
        sstatus::SPP::User => user_trap_handler(cx),
    }
}

pub fn user_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    user_time_end();//calc user time cost
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;//save next pc
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            panic!("[kernel] Cannot continue!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            panic!("[kernel] Cannot continue!");
        }
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            //set next timer & suspend current task & run next
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    kernel_time_end();//calc kernel time cost
    cx
}

pub fn kernel_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            info!("kernel interrupt: timer slice out");
            mark_kernel_interrupt();
            set_next_trigger();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            panic!("[kernel] PageFault in kernel, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
        }
        _ => {
            panic!("Unknown kernel exception or interrupt!");
        }
    }
    cx
}

pub use context::TrapContext;

pub fn enable_time_interrupt() {
    unsafe {sie::set_stimer();}
}