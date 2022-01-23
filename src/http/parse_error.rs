use std::{
    error::Error,
    fmt::{self, Display},
    str::Utf8Error,
};

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
