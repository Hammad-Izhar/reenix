use crate::drivers::uart16550::SerialDriver;

// TODO: Make Synchronization Safe

/// QEMU COM1 serial port is at 0x3F8 which we'll use as a debug port.
pub static mut COM1: SerialDriver = unsafe { SerialDriver::new(0x3F8) };

#[derive(Debug)]
#[allow(dead_code)]
pub enum DebugMode {
    Panic,
}

pub fn dbg_init() {
    unsafe {
        COM1.init().unwrap();
    }
}

#[macro_export]
macro_rules! dbg_print {
    ($typ:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        unsafe {
            write!(COM1, $($arg)*).unwrap();
        }
    }};
}

#[macro_export]
macro_rules! dbg_println {
    ($typ:expr) => {
        $crate::dbg_print!($typ, "\n")
    };
    ($typ:expr, $fmt: expr) => {
        $crate::dbg_print!($typ, concat!($fmt, "\n"))
    };

    ($typ:expr, $fmt: expr, $($arg:tt)*) => {
        $crate::dbg_print!($typ, concat!($fmt, "\n"), $($arg)*)
    };
}
