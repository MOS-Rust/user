#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(never_type)]

mod console;
mod env;
mod fork;
mod layout;
mod page;
mod panic;
mod syscall;

use core::{arch::global_asm, unreachable};
pub use console::print;
pub use env::*;
pub use fork::*;
pub use layout::*;
pub use syscall::*;

global_asm!(include_str!("entry.S"));

extern "Rust" {
    fn main(argc: isize, argv: *const *const u8);
}

#[no_mangle]
/// Rust entry for user space applications
///
/// # Safety
///
/// This function is unsafe.
unsafe extern "C" fn libmain(argc: isize, argv: *const *const u8) -> ! {
    main(argc, argv);
    exit()
}

pub fn curenv() -> &'static Env {
    unsafe { &*ENVS.add(envx(syscall_getenvid())) }
}

pub fn exit() -> ! {
    syscall_env_destroy(curenv().id);
    unreachable!()
}

#[macro_export]
macro_rules! try_run {
    ($e:expr) => {
        match $e {
            0 => (),
            r => {
                return r;
            }
        }
    };
}