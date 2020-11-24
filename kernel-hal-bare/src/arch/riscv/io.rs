use core::fmt::{Write, Arguments, Result};
use crate::sbi;

pub fn putchar(c: u8) {
    sbi::console_putchar(c as usize);
}

pub fn putfmt(fmt: Arguments) {
    unimplemented!()
    // SerialWriter.write_fmt(fmt).unwrap();
    // loop {}
}

// struct SerialWriter;

// impl Write for SerialWriter {
//     fn write_str(&mut self, s: &str) -> Result {
//         for c in s.as_bytes() {
//             putchar(*c);
//         }
//         Ok(())
//     }
// }
