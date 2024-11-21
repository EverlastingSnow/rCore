//! batch subsystem

use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
//use crate::trap::TrapContext;
use crate::loader;
use crate::loader::get_base_i;
use crate::config::*;
use lazy_static::*;



struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_current_app_range_and_id(&self) -> ((usize, usize), usize) {
        let app_id = self.get_current_app() as usize - 1;
        let current_app_base = get_base_i(app_id);
        ((current_app_base, current_app_base+self.app_start[self.current_app]-self.app_start[self.current_app-1]), app_id)
    }
}

pub fn get_current_app_range_and_id() -> ((usize, usize), usize) {
    APP_MANAGER.exclusive_access().get_current_app_range_and_id()
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

/// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    
    if current_app >= app_manager.num_app {
        println!("All applications completed!");
        shutdown(false);
    }


    app_manager.move_to_next_app();
    drop(app_manager);

    println!("[kernel] Loading app_{}", current_app);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(loader::init_app_cx(current_app));
    }
    panic!("Unreachable in batch::run_current_app!");
}
