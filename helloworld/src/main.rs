#![no_std]
#![no_main]

use user_lib::println;

#[no_mangle]
pub extern "Rust" fn main() {
    println!("Hello, world!");
}
