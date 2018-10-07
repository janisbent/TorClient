extern crate tor_crypto;
use tor_crypto::Crypt;
use std::net::TcpStream;
use std::io::{Read, Write};

pub struct PFServer {
    stream: TcpStream,
    crypt: Crypt,
}


impl PFServer {
    pub fn new(stream: TcpStream, pfpub: [u8; 32]) -> PFServer {
        let mut c = Crypt::new();
        c.ecdh(&pfpub);
        PFServer { stream: stream, crypt: c }
    }

    pub fn establish_connection(&mut self) {
        let _ = self.stream.write(&self.crypt.pk);
        let mut resp = [0; 32];
        let res = match self.stream.read(&mut resp) {
            Ok(32) => Ok(true),
            Ok(_)  => Ok(false),
            Err(r) => Err(r),
        };
        // need to unpack response
    }
}
