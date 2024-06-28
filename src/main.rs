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
pub extern "C" fn kmain() {
    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i * 2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop {}
}

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
