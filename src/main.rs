#![no_std]
#![no_main]

mod boot;
mod drivers;
mod util;

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("boot/boot.S"));

#[no_mangle]
pub extern "C" fn kmain() {
    util::debug::dbg_init();

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
