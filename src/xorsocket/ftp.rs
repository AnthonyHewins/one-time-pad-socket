use std::path::PathBuf;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};

pub fn serve(file: PathBuf, ip: Ipv4Addr, port: u16) {
    let server = bind_server(ip, port);
    let bin = get_file_binary(file);
 
    loop {
        match server.accept() {
            Ok((socket, addr)) => send_file(socket, addr, bin.to_vec()),
            Err(e) => println!("Unable to accept client: {:?}", e)
        }
    }
}

pub fn retrieve(ip: Ipv4Addr, port: u16) {
    let buffer = &mut Vec::<u8>::new();
    match connect_server(ip, port).read_to_end(buffer) {
        Ok(_) => stdout().write(&buffer).expect("Error writing binary to STDOUT"),
        Err(e) => panic!("Error trying to read: {:?}", e)
    };
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

fn bind_server(ip: Ipv4Addr, port: u16) -> TcpListener {
    match TcpListener::bind(SocketAddrV4::new(ip, port)) {
        Ok(listener) => {
            println!("Listening on {}", listener.local_addr().unwrap().to_string());
            listener
        },
        Err(e) => panic!(
            format!("Couldn't bind {}:{} because of {:?}", ip.to_string(), port, e)
        )
    }
}

fn connect_server(ip: Ipv4Addr, port: u16) -> TcpStream {
    match TcpStream::connect(SocketAddrV4::new(ip, port)) {
        Ok(stream) => stream,
        Err(e) => panic!(
            format!("Can't connect to {}:{} because of {:?}", ip.to_string(), port, e)
        )
    }
}

fn send_file(mut socket: TcpStream, addr: SocketAddr, bin: Vec<u8>) {
    println!("Received connection from {:?}", addr);

    match socket.write_all(&bin) {
        Ok(()) => println!("Successfully sent to the client."),
        Err(_) => println!("Experienced an interrupt writing to client.")
    }
}
