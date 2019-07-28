use std::net::{TcpStream};
use std::io::{Write, IoSlice};

pub struct XorSocket {
    socket: TcpStream
}

impl Write for XorSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.socket.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self.socket.write_vectored(bufs)
    }

    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
