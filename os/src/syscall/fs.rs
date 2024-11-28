//! File and filesystem-related syscalls

//use crate::loader::get_user_stack_range;
//use log::debug;
use log::error;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize { 
    //TODO
    // let (app_range, app_id)= get_current_app_range_and_id();
    // println!("{}, {}, {}", app_range.0, app_range.1, app_id);
    // let stack_range = get_user_stack_range(app_id);
    // debug!("app_range: [{:#x}, {:#x})", app_range.0,app_range.1);
    // debug!("stack_range: [{:#x}, {:#x})", stack_range.0,stack_range.1);
    // let buf_begin_pointer = buf as usize;
    // let buf_end_pointer = unsafe{buf.offset(len as isize)} as usize;
    // debug!("buf_begin_pointer: {:#x}", buf_begin_pointer);
    // debug!("buf_end_pointer: {:#x}", buf_end_pointer);
    // if !(
    //         (buf_begin_pointer >= app_range.0 && buf_begin_pointer < app_range.1) && 
    //         (buf_end_pointer >= app_range.0 && buf_end_pointer < app_range.1)
    //     )&&
    //     !(
    //         (buf_begin_pointer >= stack_range.0 && buf_begin_pointer < stack_range.1) && 
    //         (buf_end_pointer >= stack_range.0 && buf_end_pointer < stack_range.1)
    //     )
    // {
    //     error!("out of range!");
    //     return -1 as isize;
    // }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            error!("Unsupported fd in sys_write!");
            -1 as isize
        }
    }
}