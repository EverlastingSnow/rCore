//! Constants used in rCore

pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 10;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
pub const CLOCK_FREQ: usize = 0x989680;
//CLOCK_FREQ can be acquied from following steps
//qemu-system-riscv64 -machine virt,dumpdtb=dump.dtb
//dtc -o dump.dts dump.dtb
//check dump.dtb
