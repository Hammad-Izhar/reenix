#![no_std]
#![no_main]

mod boot;
// mod drivers;
// mod util;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

global_asm!(include_str!("boot/boot.S"));

#[no_mangle]
pub extern "C" fn kmain() {}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "mov dword ptr [0xb8000], 0x4f4b4f4f
             mov dword ptr [0xb8004], 0x4f4b4f4f"
        );
    }

    loop {}
}
