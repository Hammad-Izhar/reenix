use yansi::{Color, Style};

use crate::drivers::uart16550::SerialDriver;

// TODO: Make Synchronization Safe

/// QEMU COM1 serial port is at 0x3F8 which we'll use as a debug port.
pub static mut COM1: SerialDriver = unsafe { SerialDriver::new(0x3F8) };

pub static LOG_LEVEL: DebugMode = DebugMode::Trace;
pub static ERROR: Style = Color::Red.on_primary();
pub static WARN: Style = Color::Yellow.on_primary();
pub static INFO: Style = Color::Green.on_primary();
pub static DEBUG: Style = Color::Cyan.on_primary();
pub static TRACE: Style = Color::Magenta.on_primary();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DebugMode {
    Trace = 0,
    Info,
    Debug,
    Warn,
    Error,
}

pub fn dbg_init() {
    unsafe {
        COM1.init().unwrap();
    }
}

#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        unsafe {
            write!($crate::util::debug::COM1, $($arg)*).unwrap();
        }
    }};
}

#[macro_export]
macro_rules! dbg_println {
    ($typ:expr) => {{
        use core::panic::Location;
        use yansi::Paint;

        let location = Location::caller();

        let (dbg_type, dbg_style) = match $typ {
            $crate::util::debug::DebugMode::Error => ("ERROR", $crate::util::debug::ERROR),
            $crate::util::debug::DebugMode::Warn => ("WARN", $crate::util::debug::WARN),
            $crate::util::debug::DebugMode::Info => ("INFO", $crate::util::debug::INFO),
            $crate::util::debug::DebugMode::Debug => ("DEBUG", $crate::util::debug::DEBUG),
            $crate::util::debug::DebugMode::Trace => ("TRACE", $crate::util::debug::TRACE),
        };

        if $typ >= $crate::util::debug::LOG_LEVEL {

        $crate::dbg_print!(
            "[{} {}:{}]\n",
            dbg_type.paint(dbg_style),
            location.file(),
            location.line()
        );
    }
    }};

    ($typ:expr, $fmt: expr) => {{
        use core::panic::Location;
        use yansi::Paint;

        let location = Location::caller();

        let (dbg_type, dbg_style) = match $typ {
            $crate::util::debug::DebugMode::Error => ("ERROR", $crate::util::debug::ERROR),
            $crate::util::debug::DebugMode::Warn => ("WARN", $crate::util::debug::WARN),
            $crate::util::debug::DebugMode::Info => ("INFO", $crate::util::debug::INFO),
            $crate::util::debug::DebugMode::Debug => ("DEBUG", $crate::util::debug::DEBUG),
            $crate::util::debug::DebugMode::Trace => ("TRACE", $crate::util::debug::TRACE),
        };

        $crate::dbg_print!(
            concat!("[{} {}:{}] ", $fmt, "\n"),
            dbg_type.paint(dbg_style),
            location.file(),
            location.line(),
        );
    }};

    ($typ:expr, $fmt: expr, $($arg:tt)*) => {{
        use core::panic::Location;
        use yansi::Paint;

        let location = Location::caller();

        let (dbg_type, dbg_style) = match $typ {
            $crate::util::debug::DebugMode::Error => ("ERROR", $crate::util::debug::ERROR),
            $crate::util::debug::DebugMode::Warn => ("WARN", $crate::util::debug::WARN),
            $crate::util::debug::DebugMode::Info => ("INFO", $crate::util::debug::INFO),
            $crate::util::debug::DebugMode::Debug => ("DEBUG", $crate::util::debug::DEBUG),
            $crate::util::debug::DebugMode::Trace => ("TRACE", $crate::util::debug::TRACE),
        };

        $crate::dbg_print!(
            concat!("[{} {}:{}] ", $fmt, "\n"),
            dbg_type.paint(dbg_style),
            location.file(),
            location.line(),
            $($arg)*
        );
    }};
}
