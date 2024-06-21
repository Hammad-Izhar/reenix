#![no_std]
#![no_main]

mod boot;
mod drivers;
mod util;

use core::{arch::asm, panic::PanicInfo};

use reenix::util::debug::dbg_init;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "mov dword ptr [0xb8000], 0x4f4b4f4f
             mov dword ptr [0xb8004], 0x4f4b4f4f"
        );
    }

    dbg_println!(DebugMode::Panic, "Panic!");

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov dword ptr [0xb8000], 0x2f4b2f4f
             mov dword ptr [0xb8004], 0x2f4b2f4f"
        )
    }

    dbg_init();

    unsafe {
        asm!(
            "mov dword ptr [0xb8008], 0x2f4b2f4f
             mov dword ptr [0xb800c], 0x2f4b2f4f"
        )
    }

    dbg_println!(DebugMode::Panic, "Hello, World!");

    unsafe { asm!("mov dword ptr [0xb8010], 0x2f4b2f4f") }

    loop {}
}
