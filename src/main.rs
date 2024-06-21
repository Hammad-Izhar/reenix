#![no_std]
#![no_main]

mod boot;

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov dword ptr [0xb8000], 0x2f4b2f4f
             mov dword ptr [0xb8004], 0x2f4b2f4f
             mov dword ptr [0xb8008], 0x2f4b2f4f
             mov dword ptr [0xb800c], 0x2f4b2f4f
             mov dword ptr [0xb8010], 0x2f4b2f4f"
        )
    }

    loop {}
}
