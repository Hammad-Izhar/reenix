use core::arch::asm;

#[inline(always)]
pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al",
            in("dx") port,
            in("al") val,
            options(nostack, nomem, preserves_flags)
        );
    }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let x: u8;
    unsafe {
        asm!("in al, dx",
            in("dx") port,
            out("al") x,
            options(nostack, nomem, preserves_flags)
        );
    }
    x
}

#[allow(dead_code)]
#[inline(always)]
pub fn outw(port: u16, val: u16) {
    unsafe {
        asm!("out dx, ax",
            in("dx") port,
            in("ax") val,
            options(nostack, nomem, preserves_flags)
        );
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn inw(port: u16) -> u16 {
    let x: u16;
    unsafe {
        asm!("in ax, dx",
            in("dx") port,
            out("ax") x,
            options(nostack, nomem, preserves_flags)
        );
    }
    x
}

#[allow(dead_code)]
#[inline(always)]
pub fn outl(port: u16, val: u32) {
    unsafe {
        asm!("out dx, eax",
            in("dx") port,
            in("eax") val,
            options(nostack, nomem, preserves_flags)
        );
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn inl(port: u16) -> u32 {
    let x: u32;
    unsafe {
        asm!("in eax, dx",
            in("dx") port,
            out("eax") x,
            options(nostack, nomem, preserves_flags)
        );
    }
    x
}
