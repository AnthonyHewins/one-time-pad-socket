use std::path::PathBuf;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::net::ToSocketAddrs;

use super::socket::Socket;
use super::listener::Listener;

pub fn serve<S: ToSocketAddrs>(ip: S, file: PathBuf, key: PathBuf) {
    println!("Binding server...");
    let server = Listener::bind(ip).expect("Unable to bind server");
    println!("Server bound to {}. Invite traffic.", server.addr().unwrap());

    let mut buf: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
    get_file_binary(file, &mut buf);

    match &mut server.accept(key) {
        Ok(socket) => {
            println!("Got a client, sending file...");
            println!("Wrote {}B", socket.write(&buf).expect("Failed writing buf to client"));
        },
        Err(e) => println!("Unable to accept client: {:?}", e)
    }
}

pub fn get<S: ToSocketAddrs>(ip: S, key: PathBuf) {
    let mut socket = Socket::connect(ip, key).expect("Failed connecting to server: network problem");
    let mut buf: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
    socket.read(&mut buf);
    stdout().write(&buf).expect("Error writing binary to STDOUT");
}

fn get_file_binary(path: PathBuf, buf: &mut [u8]) {
    let mut file = File::open(path).expect("Unable to open file to serve");
    file.read(buf);
}
