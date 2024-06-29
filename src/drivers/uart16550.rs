use core::fmt::{Error, Write};

use crate::util::io::{inb, outb};

#[derive(Debug)]
pub struct SerialDriver {
    port: u16,
}

impl SerialDriver {
    pub const unsafe fn new(port: u16) -> SerialDriver {
        SerialDriver { port }
    }

    fn rx_tx_port(&self) -> u16 {
        self.port
    }

    fn int_enable_reg(&self) -> u16 {
        self.port + 1
    }

    fn fifo_ctrl_reg(&self) -> u16 {
        self.port + 2
    }

    fn line_ctrl_reg(&self) -> u16 {
        self.port + 3
    }

    #[allow(dead_code)]
    fn modem_ctl_reg(&self) -> u16 {
        self.port + 4
    }

    fn line_status_reg(&self) -> u16 {
        self.port + 5
    }

    #[allow(dead_code)]
    fn modem_status_reg(&self) -> u16 {
        self.port + 6
    }

    #[allow(dead_code)]
    fn scratch_reg(&self) -> u16 {
        self.port + 6
    }

    /// Inialize the device by specifing the baud rate and other settings.
    /// The following is done to properly initialize the device:
    /// 1. Disable all interrupts
    /// 2. Enable the DLAB (Divisor Latch Access Bit) to set the baud rate
    /// 3. Set the baud rate to 38400 bps
    /// 4. Set the data format to 8 bits, no parity, one stop bit
    /// 5. Enable the FIFO buffer, clear it, and set a 14-byte threshold
    pub fn init(&mut self) -> Result<(), ()> {
        // Disable all interrupts
        outb(self.int_enable_reg(), 0x00);
        // Enable DLAB (set baud rate divisor)
        outb(self.line_ctrl_reg(), 0x80);
        // Set baud rate
        outb(self.rx_tx_port(), 0x03);
        outb(self.int_enable_reg(), 0x00);
        // 8 bits, no parity, one stop bit
        outb(self.line_ctrl_reg(), 0x03);
        // Enable FIFO, clear, with 14-byte threshold
        outb(self.fifo_ctrl_reg(), 0xC7);

        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) {
        while inb(self.line_status_reg()) & 0x20 == 0 {}
        outb(self.rx_tx_port(), byte);
    }

    pub fn write(&mut self, data: &[u8]) {
        for byte in data {
            self.write_byte(*byte);
        }
    }
}

impl Write for SerialDriver {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write(s.as_bytes());
        Ok(())
    }
}
