use std::path::PathBuf;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::net::{ToSocketAddrs, SocketAddr, SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};

use super::socket::Socket;
use super::listener::Listener;

pub fn serve<S: ToSocketAddrs>(ip: S, file: PathBuf, key: PathBuf) {
    let server = Listener::bind(ip).unwrap();
    let mut buf: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
    let file = get_file_binary(file);

    match &mut server.accept(key) {
        Ok(socket) => {
            println!("Got a client, sending file...");
            println!("Wrote {}B", socket.write(&buf).expect("Failed writing buf to client"));
        },
        Err(e) => println!("Unable to accept client: {:?}", e)
    }
}

pub fn get<S: ToSocketAddrs>(ip: S, key: PathBuf) {
    let socket = Socket::connect(ip, key).expect("Failed connecting to server: network problem");
    let mut buf: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
    socket.read(&mut buf);
    stdout().write(&buffer).expect("Error writing binary to STDOUT");
}

fn get_file_binary(path: PathBuf) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {:?}, is it being used/do you have permission?", e)
    };

    match file.read_to_end(&mut buffer) {
        Ok(bytes) => println!("Read {} bytes, preparing to serve", bytes),
        Err(e) => panic!("Error reading the file: {:?}", e)
    };

    return buffer;
}
