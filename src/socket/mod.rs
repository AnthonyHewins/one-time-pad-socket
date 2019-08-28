extern crate file_lock;

use std::io::{Read, Error};
use std::net::{ToSocketAddrs, TcpStream};
use std::path::PathBuf;

use file_lock::FileLock;

pub mod ftp;
mod io;
mod internals;

pub struct Socket {
    filelock: FileLock,
    socket: TcpStream,
    key: Vec::<u8>
}

impl Socket {
    /// BUFSIZE is 8KiB
    const BUFSIZE: usize = 8 * 1_024 * 1_024;
}

impl Socket {
    pub fn new<P: Into<PathBuf>>(socket: TcpStream, key: P) -> Result<Socket, Error> {
        let path = key.into();

        let mut filelock = FileLock::lock(path.to_str().unwrap(), true, true)?;
        let mut buf = Vec::<u8>::new();
        filelock.file.read_to_end(&mut buf)?;

        Ok( Socket { filelock: filelock, socket: socket, key: buf } )
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
