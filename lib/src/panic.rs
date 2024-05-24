use core::panic::PanicInfo;

use crate::{exit, println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    exit()
}