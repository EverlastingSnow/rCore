//#![deny(missing_docs)]
//#![deny(warnings)]



#![no_std]
#![no_main]
use core::arch::global_asm;
use core::arch::asm;
use log::*;
#[macro_use]
mod console;
mod config;
mod lang_items;
mod logging;
mod loader;
mod sbi;
mod sync;
mod timer;
pub mod task;
pub mod syscall;
pub mod trap;


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

//init fpu to support fs
fn clear_fpu(){
    unsafe {
        for i in 0..32 {
            asm!("fcvt.d.w f{i}, x0", i = in(reg) i);
            asm!("fmv.d.x f{i}, x0", i = in(reg) i);
            asm!("fmv.w.x f{i}, x0", i = in(reg) i);
        }
    }
}
fn init_fpu() {
    unsafe {
        asm!(
            r#"
        li t0, 0x4000 # bit 14 is FS most significant bit
        li t2, 0x2000 # bit 13 is FS least significant bit
        csrrc x0, sstatus, t0
        csrrs x0, sstatus, t2
        "#
        );
    }
    clear_fpu();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}


#[no_mangle]
pub fn rust_main() -> !{
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end add r of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();    
    init_fpu();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trap::init();
    loader::load_apps();
    trap::enable_time_interrupt();
    timer::set_next_trigger();

    use riscv::register::sstatus;
    unsafe {sstatus::set_sie()};
    loop {
        if trap::check_kernel_interrupt() {
            info!("kernel interrupt returned");
            break;
        }
    }
    unsafe {sstatus::clear_sie()};
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}