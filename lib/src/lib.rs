#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(never_type)]

mod console;
mod env;
mod fork;
mod ipc;
mod layout;
mod page;
mod panic;
mod syscall;

use core::{arch::global_asm, unreachable};
pub use console::print;
pub use env::*;
pub use fork::*;
pub use ipc::*;
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

pub fn user_halt() -> ! {
    syscall_panic()
}

pub fn pageref(v: VA) -> i32 {
    if !unsafe { &*VPD.add(v.pdx()) }.flags().contains(PteFlags::V) {
        return 0;
    }

    let pte = unsafe { &*VPT.add(v.0 >> PGSHIFT) };

    if !pte.flags().contains(PteFlags::V) {
        return 0;
    }

    unsafe { &*PAGES.add(pte.0 >> PGSHIFT) }.ref_count as i32
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