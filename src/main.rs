extern crate clap;
extern crate regex;
extern crate tor_crypto;
mod pfserver;
mod router;
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use clap::{Arg, App};
use pfserver::PFServer;
use std::fs::File;
use std::io::Read;


fn parse_args() -> (TcpListener, TcpStream, [u8; 32]) {
	let matches = App::new("Tor Client")
                          .version("1.0")
                          .author("Ben J. <janisbent@gmail.com>")
                          .about("Tor network local client")
                          .arg(Arg::with_name("PORT")
                               .help("Sets the port to run the client on")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("PFIP")
                               .help("IP of the Tor pathfinding server in form of ip:port")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("PFPUB")
                               .help("The path to the public key of the Tor pathfinding server")
                               .required(true)
                               .index(3))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    let port  = matches.value_of("PORT").unwrap();
    let pfip  = matches.value_of("PFIP").unwrap();
    let pfpubfn = matches.value_of("PFPUB").unwrap();
    let listener = match TcpListener::bind(format!("{}{}", "127.0.0.1:", port)) {
        Ok(l)  => l,
        Err(r) => {
            println!("Error on PORT: {} - \"{}\"", r, port);
            exit(1);
        },
    };
    let pfstream = match TcpStream::connect(pfip) {
        Ok(l)  => l,
        Err(r) => {
            println!("Error on PFIP: {} - \"{}\"", r, pfip);
            exit(1);
        },
    };
    let mut f = match File::open(pfpubfn) {
        Ok(f)  => f,
        Err(r) => {
            println!("Error on PFPUB: {} - \"{}\"", r, pfpubfn);
            exit(1);
        },
    };
    let mut pfpub = [0; 32];
    match f.read(&mut pfpub) {
        Ok(_) => (),
        Err(r) => {
            println!("Error on reading PFPUB: {} - \"{}\"", r, pfpubfn);
            exit(1);
        },
    };

    (listener, pfstream, pfpub)
}


fn main() {
    let (_listener, pfstream, pfpub) = parse_args();
    let _pf = PFServer::new(pfstream, pfpub);
    println!("Hello, world!");
}
