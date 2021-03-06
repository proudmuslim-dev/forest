use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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
#[derive(Serialize, Deserialize, Display, Debug, Copy, Clone)]
pub enum Currency {
    #[strum(to_string = "USDT-BTC")]
    #[serde(rename = "USDT-BTC")]
    BTC,
    #[strum(to_string = "BTC-ETH")]
    #[serde(rename = "BTC-ETH")]
    ETH,
    #[strum(to_string = "BTC-XMR")]
    #[serde(rename = "BTC-XMR")]
    XMR,
}
