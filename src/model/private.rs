use crate::model::util::OrderType;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitOrder<'v> {
    success: bool,
    uuid: &'v str,
    #[serde(rename = "bnewbalavail")]
    #[serde_as(as = "DisplayFromStr")]
    b_new_bal_avail: f64,
    #[serde(rename = "snewbalavail")]
    #[serde_as(as = "DisplayFromStr")]
    s_new_bal_avail: f64,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Order<'v> {
    success: bool,
    #[serde_as(as = "DisplayFromStr")]
    date: usize,
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    order_type: OrderType,
    market: &'v str,
    #[serde_as(as = "DisplayFromStr")]
    price: f64,
    #[serde_as(as = "DisplayFromStr")]
    quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    fulfilled: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cancel {
    success: bool,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderHistory<'v> {
    uuid: &'v str,
    date: usize,
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    price: f64,
    #[serde_as(as = "DisplayFromStr")]
    quantity: f64,
    market: &'v str,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    success: bool,
    #[serde_as(as = "DisplayFromStr")]
    balance: f64,
    #[serde_as(as = "DisplayFromStr")]
    available: f64,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Balances {
    success: bool,
    #[serde_as(as = "HashMap<_, DisplayFromStr>")]
    pub(crate) balances: HashMap<String, f64>,
}
