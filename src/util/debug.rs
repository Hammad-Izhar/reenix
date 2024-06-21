use crate::drivers::uart16550::SerialDriver;

// TODO: Make Synchronization Safe

/// QEMU COM1 serial port is at 0x3F8 which we'll use as a debug port.
pub static mut COM1: SerialDriver = unsafe { SerialDriver::new(0x3F8) };

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
    ($fmt:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        unsafe {
            $crate::util::debug::COM1.write_fmt(format_args!($($arg)*)).unwrap()
        }
    }};
}

#[macro_export]
macro_rules! dbg_println {
    ($fmt:expr) => {
        $crate::dbg_print!(fmt, "\n")
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::dbg_print!(fmt, "{}\n", format_args!($($arg)*))
    };
}
