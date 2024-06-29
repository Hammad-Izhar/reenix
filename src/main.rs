#![no_std]
#![no_main]

mod boot;
mod drivers;
mod util;

use core::{arch::global_asm, panic::PanicInfo};

use util::debug::DebugMode;

global_asm!(include_str!("boot/boot.S"));

#[no_mangle]
pub extern "C" fn kmain() {
    util::debug::dbg_init();

    dbg_println!(DebugMode::Error);
    dbg_println!(DebugMode::Error, "Hello, World!");
    dbg_println!(DebugMode::Error, "{}, World!", "Hello");

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    dbg_println!(DebugMode::Error, "{}", info);

    loop {}
}
