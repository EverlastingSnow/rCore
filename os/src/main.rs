// // os/src/main.rs

//#![feature(panic_info_message)]

#![no_std]
#![no_main]
#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod log;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

extern "C" {
    static skernel: u8;    // .text 段的起始地址
    static stext: u8;       // .text 段的起始地址
    static etext: u8;       // .text 段的结束地址
    static srodata: u8;     // .rodata 段的起始地址
    static erodata: u8;     // .rodata 段的结束地址
    static sdata: u8;       // .data 段的起始地址
    static edata: u8;       // .data 段的结束地址
    static sbss: u8;        // .bss 段的起始地址
    static ebss: u8;        // .bss 段的结束地址
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum LogLevel {
    ERROR = 0,
    INFO = 1,
    DEBUG = 2,
}
pub static LOG_LEVEL: LogLevel = LogLevel::INFO;

pub fn print_segment_addresses() {
    unsafe {
        // 获取 .text 段地址
        let s_text = &stext as *const u8;
        let e_text = &etext as *const u8;
        info!(".text [{:#x}, {:#x})", s_text as usize, e_text as usize);

        // 获取 .rodata 段地址
        let s_rodata = &srodata as *const u8;
        let e_rodata = &erodata as *const u8;
        debug!(".rodata [{:#x}, {:#x})", s_rodata as usize, e_rodata as usize);

        // 获取 .data 段地址
        let s_data = &sdata as *const u8;
        let e_data = &edata as *const u8;
        error!(".data [{:#x}, {:#x})", s_data as usize, e_data as usize);
    }
}

#[no_mangle]
pub fn rust_main() -> !{
    clear_bss();    

    if LOG_LEVEL as u8 <= LogLevel::INFO as u8 {
        print_segment_addresses();
    }
    println!("Hello World!");
    panic!("Shutdown machine!");
}

//清零bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe {(a as * mut u8).write_volatile(0) }
    });
}