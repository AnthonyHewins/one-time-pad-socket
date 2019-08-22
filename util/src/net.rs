use std::thread;
use std::net::{TcpStream, TcpListener, SocketAddr};

pub struct ClientServer {
    pub client: TcpStream,
    pub server: TcpStream
}

impl ClientServer {
    pub fn new() -> ClientServer {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let addr = listener.local_addr().unwrap();
        let server = thread::spawn(move || listener.accept().unwrap().0);
        let client = TcpStream::connect(addr).unwrap();

        ClientServer {server: server.join().unwrap(), client: client}
    }
}

pub fn mock_server() -> SocketAddr {
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = server.local_addr().unwrap();
    thread::spawn( move || server.accept().unwrap() );
    addr
}
