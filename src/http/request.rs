use super::Params;
use std::{convert::TryFrom, str};
use super::ParseError;

#[derive(Debug)]
pub struct Request<'bfr> {
    path: &'bfr str,
    params: Option<Params<'bfr>>,
    method: &'bfr str,
}

impl<'bfr> Request<'bfr>{
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn params(&self) -> Option<&Params> {
        self.params.as_ref()
    }

    pub fn method(&self) -> &str {
        &self.method
    }
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
