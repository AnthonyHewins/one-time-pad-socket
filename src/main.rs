use std::path::PathBuf;
use std::net::{SocketAddr, Ipv4Addr, ToSocketAddrs};
use structopt::StructOpt;

mod socket;
mod ftp;
mod listener;
mod keygen;

#[derive(StructOpt)]
#[structopt(name = "1timepad socket", about = "Send data securely after XOR'ing it")]
struct Cli {
    /// Where the 1 time pad is stored
    #[structopt(default_value = "~/.onetimepad/key", short, long, help = "Override key location")]
    key: PathBuf,

    /// Send flag
    #[structopt(short, long, help = "Send a file")]
    send: Option<PathBuf>,

    /// Receive flag
    #[structopt(short, long, help = "Receive a file")]
    receive: bool,

    /// IP
    #[structopt(short, long, help = "The IP you want to interact with")]
    ip: Option<String>,

    /// Port
    #[structopt(short, long, help = "The port to communicate over")]
    port: Option<u16>
}

fn parse_socket(ip: Option<String>, port: Option<u16>) -> SocketAddr {
    let ip_str = ip.unwrap_or(String::from("0.0.0.0"));
    SocketAddr::new(
        ip_str.parse().expect("Invalid IP address"),
        port.unwrap_or(0)
    )
}

fn main() {
    let args = Cli::from_args();

    if args.send.is_some() {

        ftp::serve(
            parse_socket(args.ip, args.port),
            args.send.unwrap(),
            args.key
        );

    } else if args.receive {

        ftp::get(
            parse_socket(args.ip, args.port),
            args.key
        );

    } else {

        println!("No command specified. Specify the -s or -r flag. Exiting.");
        std::process::exit(1);

    }
}
