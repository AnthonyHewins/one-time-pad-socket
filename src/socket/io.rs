use std::io::{BufReader, Read, Result, Write};

use super::*;

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut encrypted: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
        self.xor(buf, &mut encrypted)?;
        self.socket.write(&mut encrypted)
    }

    fn flush(&mut self) -> Result<()> { Ok(()) }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut encrypted: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
        self.xor(buf, &mut encrypted)?;
        self.socket.read(&mut encrypted)
    }
}

#[cfg(test)]
mod tests {
    
}
