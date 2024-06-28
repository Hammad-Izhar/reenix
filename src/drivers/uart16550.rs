use core::{
    arch::asm,
    fmt::{Error, Write},
};

use x86_64::instructions::port::Port;

#[derive(Debug)]
pub struct SerialDriver {
    port: u16,
    rx_tx_port: Port<u8>,
    int_enable_reg: Port<u8>,
    fifo_ctrl_reg: Port<u8>,
    line_ctrl_reg: Port<u8>,
    modem_ctrl_reg: Port<u8>,
    line_status_reg: Port<u8>,
    modem_status_reg: Port<u8>,
    scratch_reg: Port<u8>,
}

impl SerialDriver {
    pub const unsafe fn new(port: u16) -> SerialDriver {
        SerialDriver {
            port,
            rx_tx_port: Port::new(port),
            int_enable_reg: Port::new(port + 1),
            fifo_ctrl_reg: Port::new(port + 2),
            line_ctrl_reg: Port::new(port + 3),
            modem_ctrl_reg: Port::new(port + 4),
            line_status_reg: Port::new(port + 5),
            modem_status_reg: Port::new(port + 6),
            scratch_reg: Port::new(port + 7),
        }
    }

    /// Inialize the device by specifing the baud rate and other settings.
    /// The following is done to properly initialize the device:
    /// 1. Disable all interrupts
    /// 2. Enable the DLAB (Divisor Latch Access Bit) to set the baud rate
    /// 3. Set the baud rate to 38400 bps
    /// 4. Set the data format to 8 bits, no parity, one stop bit
    /// 5. Enable the FIFO buffer, clear it, and set a 14-byte threshold
    pub fn init(&mut self) -> Result<(), ()> {
        unsafe {
            // Disable all interrupts
            // self.int_enable_reg.write(0x00);
            asm!("out dx, al", in("dx") self.port, in("al") 0x00 as u8);
            // Enable DLAB (set baud rate divisor)
            // self.line_ctrl_reg.write(0x80);
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x80 as u8);
            // Set baud rate
            // self.rx_tx_port.write(0x03);
            asm!("out dx, al", in("dx") self.port, in("al") 0x03 as u8);
            // self.int_enable_reg.write(0x00);
            asm!("out dx, al", in("dx") self.port + 1, in("al") 0x00 as u8);
            // 8 bits, no parity, one stop bit
            // self.line_ctrl_reg.write(0x03);
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x03 as u8);
            // Enable FIFO, clear, with 14-byte threshold
            // self.fifo_ctrl_reg.write(0xC7);
            asm!("out dx, al", in("dx") self.port + 2, in("al") 0xC7 as u8);
        }

        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while self.line_status_reg.read() & 0x20 == 0 {}
            self.rx_tx_port.write(byte);
        }
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
