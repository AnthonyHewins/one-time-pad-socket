use std::io;
use std::net::{ToSocketAddrs, TcpListener, SocketAddr};
use std::path::PathBuf;

use super::socket::Socket;

pub struct Listener {
    server: TcpListener        
}

impl Listener {
    pub fn bind<S: ToSocketAddrs>(sockaddr: S) -> io::Result<Listener> {
        Ok( Listener { server: TcpListener::bind(sockaddr)? } )
    }

    pub fn accept<P: Into<PathBuf>>(&self, key: P) -> io::Result<Socket> {
        Socket::new(self.server.accept()?.0, key)
    }

    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.server.local_addr()
    }
}

#[cfg(test)]
mod tests {
    extern crate util;

    use util::fs::TmpFile;
    use std::thread;
    use std::net::{TcpListener, TcpStream};
    use super::*;

    #[test]
    fn bind() {
        Listener::bind("127.0.0.1:0").unwrap();
    }

    #[test]
    fn local_addr() {
        let sock = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = sock.local_addr().unwrap();
        let listener = Listener { server: sock };

        assert_eq!(addr, listener.addr().unwrap());
    }
    
    #[test]
    fn accept() {
        let server = Listener::bind("127.0.0.1:0").unwrap();

        let addr = server.addr().unwrap();

        let tmp = TmpFile::new();        
        let server_thread = thread::spawn(move || server.accept(tmp.path).unwrap());

        let client = TcpStream::connect(addr).unwrap();
        let server = server_thread.join();
    }
}
