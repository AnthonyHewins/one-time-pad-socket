use std::io::{BufReader, Read};

impl Read for super::Socket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.socket.read(&buf);

        for i in 0..bytes_read {
            buf[i] = buf[i] ^ self.key[i];
        };

        unshift_bytes(bytes_read);
        bytes_read
    }

    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
