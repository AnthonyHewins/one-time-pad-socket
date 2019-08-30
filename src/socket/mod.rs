use std::io::{Read, Error};
use std::fs::File;
use std::net::{ToSocketAddrs, TcpStream};
use std::path::PathBuf;

mod io;
mod internals;

// TODO: make it lock the file

pub struct Socket {
    file: PathBuf,
    socket: TcpStream,
    key: Vec::<u8>
}

impl Socket {
    /// BUFSIZE is 8KiB
    pub const BUFSIZE: usize = 8 * 1_024 * 1_024;

    pub fn new<P: Into<PathBuf>>(socket: TcpStream, key: P) -> Result<Socket, Error> {
        let path = key.into();

        let mut buf = Vec::<u8>::new();
        File::open(&path)?.read_to_end(&mut buf)?;

        Ok( Socket { file: path, socket: socket, key: buf } )
    }

    pub fn connect<S: ToSocketAddrs, P: Into<PathBuf>>(addr: S, key: P) -> Result<Socket, Error> {
        Socket::new(TcpStream::connect(addr)?, key)
    }
}

#[cfg(test)]
mod tests {
    extern crate util;

    use super::*;
    use util::{net::ClientServer, fs::TmpFile};

    fn match_fields(sock: Socket, tmp: &TmpFile) {
        assert_eq!(sock.file, tmp.path);
        assert_eq!(sock.key, tmp.bytes);
    }

    #[test]
    fn new_works() {
        let cs = ClientServer::new();
        let tmp = TmpFile::new();

        let socket = Socket::new(cs.server, &tmp.path).unwrap();
        match_fields(socket, &tmp);
    }

    #[test]
    fn connect_works() {
        let tmp = util::fs::TmpFile::new();

        let addr = util::net::mock_server();
        let socket = Socket::connect(addr, &tmp.path).unwrap();
        match_fields(socket, &tmp);
        
        let addr = util::net::mock_server();
        let socket = Socket::connect(addr, &tmp.path).unwrap();
        match_fields(socket, &tmp);
    }
}
