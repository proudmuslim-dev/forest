use lazy_static::lazy_static;
use std::{fmt, fmt::Formatter};

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    APIError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}

// TODO: Complete list
#[derive(Debug, Copy, Clone)]
pub enum Currency {
    BTC,
    XMR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Currency::BTC => write!(f, "BTC"),
            Currency::XMR => write!(f, "XMR"),
        }
    }
}
