use crate::api::model::util::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
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
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    bid: Option<f64>,
    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    ask: Option<f64>,
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Ticker {
    success: bool,
    #[serde(rename = "initialprice")]
    #[serde_as(as = "DisplayFromStr")]
    initial_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) price: f64,
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

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    date: usize,
    // I could just name it r#type, but that's not a great approach
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    price: f64,
    #[serde_as(as = "DisplayFromStr")]
    quantity: f64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    success: bool,
    #[serde_as(as = "HashMap<_, DisplayFromStr>")]
    pub(crate) buy: HashMap<String, f64>,
    #[serde_as(as = "HashMap<_, DisplayFromStr>")]
    pub(crate) sell: HashMap<String, f64>,
}
