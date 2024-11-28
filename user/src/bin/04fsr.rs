#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let pi = 3.14f64;
    let r = 5.0;
    println!("pi * r * r = {}", pi * r * r);
    0
}
