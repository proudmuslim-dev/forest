use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    success: bool,
    #[serde(rename = "initialprice")]
    #[serde_as(as = "DisplayFromStr")]
    initial_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    price: f64,
    #[serde_as(as = "DisplayFromStr")]
    high: f64,
    #[serde_as(as = "DisplayFromStr")]
    low: f64,
    #[serde_as(as = "DisplayFromStr")]
    volume: f64,
    #[serde_as(as = "DisplayFromStr")]
    bid: f64,
    #[serde_as(as = "DisplayFromStr")]
    ask: f64,
}
