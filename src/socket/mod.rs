extern crate file_lock;

use std::io::Error;
use std::net::{ToSocketAddrs, TcpStream};
use std::path::PathBuf;

use file_lock::FileLock;

pub mod ftp;
mod write;
mod internals;

pub struct Socket {
    filelock: FileLock,
    socket: TcpStream
}

impl Socket {
    /// BUFSIZE is 8KiB
    const BUFSIZE: usize = 8 * 1_024 * 1_024;
}

impl Socket {
    pub fn new<P: Into<PathBuf>>(socket: TcpStream, key: P) -> Result<Socket, Error> {
        let path = key.into();
        let filelock = FileLock::lock(path.to_str().unwrap(), true, true)?;
        Ok( Socket { filelock: filelock, socket: socket } )
    }

    pub fn connect<S: ToSocketAddrs, P: Into<PathBuf>>(addr: S, key: P) -> Result<Socket, Error> {
        Socket::new(TcpStream::connect(addr)?, key)
    }
}

#[cfg(test)]
mod tests {
    extern crate util;

    use super::*;

    #[test]
    fn new_works() {
        let cs = util::net::ClientServer::new();
        let tmp = util::fs::TmpFile::new();
        Socket::new(cs.server, tmp.path).unwrap();
    }

    #[test]
    fn connect_works() {
        let tmp = util::fs::TmpFile::new();

        let addr = util::net::mock_server();
        Socket::connect(addr, &tmp.path).unwrap();

        let addr = util::net::mock_server();
        Socket::connect(addr.to_string(), &tmp.path).unwrap();
    }
}
