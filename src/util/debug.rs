use yansi::{Color, Style};

use crate::{drivers::uart16550::SerialDriver, sync::spinlock::Spinlock};

/// QEMU COM1 serial port is at 0x3F8 which we'll use as a debug port.
pub static COM1: Spinlock<SerialDriver> = Spinlock::new(unsafe { SerialDriver::new(0x3F8) });

pub static ERROR: Style = Color::Red.on_primary();
pub static WARN: Style = Color::Yellow.on_primary();
pub static INFO: Style = Color::Green.on_primary();
pub static DEBUG: Style = Color::Cyan.on_primary();
pub static TRACE: Style = Color::Magenta.on_primary();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DebugMode {
    Trace,
    Info,
    Debug,
    Warn,
    Error,
}

impl core::fmt::Display for DebugMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use yansi::Paint;
        let (dbg_type, dbg_style) = match self {
            DebugMode::Error => ("ERROR", ERROR),
            DebugMode::Warn => ("WARN", WARN),
            DebugMode::Info => ("INFO", INFO),
            DebugMode::Debug => ("DEBUG", DEBUG),
            DebugMode::Trace => ("TRACE", TRACE),
        };

        write!(f, "{}", dbg_type.paint(dbg_style))
    }
}

pub fn dbg_init() {
    COM1.lock().init().unwrap();
}

#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => {{
            use core::fmt::Write;
            write!($crate::util::debug::COM1.lock(), $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! dbg_println {
    ($typ:expr) => {{
        use core::panic::Location;
        let location = Location::caller();

        $crate::dbg_print!(
            "[{} {}:{}]\n",
            $typ,
            location.file(),
            location.line()
        );
    }};

    ($typ:expr, $fmt: expr) => {{
        use core::panic::Location;
        let location = Location::caller();

        $crate::dbg_print!(
            concat!("[{} {}:{}] ", $fmt, "\n"),
            $typ,
            location.file(),
            location.line(),
        );
    }};

    ($typ:expr, $fmt: expr, $($arg:tt)*) => {{
        use core::panic::Location;
        let location = Location::caller();

        $crate::dbg_print!(
            concat!("[{} {}:{}] ", $fmt, "\n"),
            $typ,
            location.file(),
            location.line(),
            $($arg)*
        );
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::dbg_println!($crate::util::debug::DebugMode::Trace, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::dbg_println!($crate::util::debug::DebugMode::Debug, $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::dbg_println!($crate::util::debug::DebugMode::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::dbg_println!($crate::util::debug::DebugMode::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::dbg_println!($crate::util::debug::DebugMode::Error, $($arg)*);
    };
}
