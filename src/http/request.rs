use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Display},
    str::{self, Utf8Error},
};
use super::Params;

#[derive(Debug)]
pub struct Request<'bfr> {
    path: &'bfr str,
    params: Option<Params<'bfr>>,
    method: &'bfr str,
}

impl<'bfr> TryFrom<&'bfr [u8]> for Request<'bfr> {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'bfr [u8]) -> Result<Request<'bfr>, Self::Error> {
        let req = str::from_utf8(buf)?;
        let (method, req) = next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = next_word(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let mut params = None;
        if let Some(i) = path.find('?') {
            params = Some(Params::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            params,
            method,
        })
    }
}

fn next_word(sentence: &str) -> Option<(&str, &str)> {
    for (i, c) in sentence.char_indices() {
        if c == ' ' || c == '\r' {
            return Some((&sentence[..i], &sentence[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ParseError::InvalidRequest => "InvalidRequest",
            ParseError::InvalidEncoding => "InvalidEncoding",
            ParseError::InvalidProtocol => "InvalidProtocol",
            ParseError::InvalidMethod => "InvalidMethod",
        };
        write!(f, "{}", msg)
    }
}

impl Error for ParseError {}
