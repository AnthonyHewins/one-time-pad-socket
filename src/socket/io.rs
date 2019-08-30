use std::io::{Read, Result, Write};

use super::*;

// TODO: use write_all so it supports Vec<u8>

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

/*
#[cfg(test)]
mod tests {
    extern crate util;
    use super::*;

    use std::thread;

    fn copy_1timepad(from: &PathBuf) -> PathBuf {
        let copy_file = PathBuf::from("/tmp/rust_key_copy");
        std::fs::copy(from, &copy_file);
        copy_file
    }
 
    fn socket_client_server() -> (Socket, Socket) {
        let tmp = util::fs::TmpFile::new();
        let cs = util::net::ClientServer::new();
        let copy = copy_1timepad(&tmp.path);

        (
            Socket::new(cs.server, tmp.path).unwrap(),
            Socket::new(cs.client, copy).unwrap()
        )
    }
    
    #[test]
    fn test_conversation() {
        let (mut server, mut client) = socket_client_server();

        let send = thread::spawn(move || server.write(b"str"));
        send.join();
    }

    #[test]
    fn test_write() {

    }
}
*/
