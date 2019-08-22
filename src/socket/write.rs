use std::io::{Result, Write};

use super::*;

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        self.check_len(len)?;

        let mut key = self.unshift_bytes(len);
        for i in 0..len {
            key[i] = key[i] ^ buf[i];
        }

        self.socket.write(&mut key)
    }

    fn flush(&mut self) -> Result<()> { Ok(()) }
}
