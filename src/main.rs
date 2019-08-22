use std::path::PathBuf;
use std::net::Ipv4Addr;
use structopt::StructOpt;

mod socket;

#[derive(StructOpt)]
#[structopt(name = "1timepad socket", about = "Send data securely after XOR'ing it")]
struct Cli {
    /// Where the 1 time pad is stored
    #[structopt(default_value = "", short, long, help = "Override key location")]
    key: PathBuf,

    /// Send flag
    #[structopt(short, long, help = "Send a file")]
    send: Option<PathBuf>,

    /// Receive flag
    #[structopt(short, long, help = "Receive a file")]
    receive: bool,

    ///IP
    #[structopt(short, long, help = "The IP you want to interact with")]
    ip: Option<Ipv4Addr>,

    ///Port
    #[structopt(short, long, help = "The port to communicate over")]
    port: Option<u16>
}

fn main() {
    let args = Cli::from_args();

    if args.send.is_some() {

        socket::ftp::serve(
            args.send.unwrap(),
            args.ip.unwrap_or_else(|| Ipv4Addr::new(0,0,0,0)),
            args.port.unwrap_or(0)
        );

    } else if args.receive {

        socket::ftp::retrieve(
            args.ip.expect("Must specify IP to get a file"),
            args.port.unwrap_or(80)
        );

    } else {

        println!("No command specified. Specify the -s or -r flag. Exiting.");
        std::process::exit(1);

    }
}
