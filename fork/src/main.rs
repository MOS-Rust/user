#![no_std]
#![no_main]

use user_lib::{fork, println, syscall_yield};

#[no_mangle]
pub extern "Rust" fn main() {
    println!("Hello, world!");
    match fork() {
        0 => {
            println!("I am child");
            syscall_yield();
            println!("I am child after yield");
        }
        _ => {
            println!("I am parent");
            syscall_yield();
            println!("I am parent after yield");
        }
    }
}
