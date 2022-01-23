use crate::{handler::Handler, http::Request};
use std::{convert::TryFrom, io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self: Self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, sock_addr)) => {
                    let mut buffer = [0; 1024];
                    println!("Received a req from {}", sock_addr.to_string());
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    println!("Req : {:?}", req);
                                    handler.handle_request(&req).send(&mut stream);
                                }
                                Err(err) => {
                                    handler.handle_bad_request(&err).send(&mut stream);
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
