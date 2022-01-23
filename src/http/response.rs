use std::{io::Result as IoResult, io::Write, net::TcpStream};

use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, f: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(s) => s,
            None => "",
        };
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.msg(),
            body
        )
    }
}
