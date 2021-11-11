use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::api::model::util::OrderType;
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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderHistory {
    uuid: String,
    pub(crate) date: usize,
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) quantity: f64,
    pub(crate) market: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Balance {
    success: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub balance: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub available: f64,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Balances {
    success: bool,
    #[serde_as(as = "HashMap<_, DisplayFromStr>")]
    balances: HashMap<String, f64>,
}

impl Balances {
    pub fn get(&self, key: &str) -> Option<&f64> {
        self.balances.get(key)
    }

    pub fn get_all(&self) -> &HashMap<String, f64> {
        &self.balances
    }
}
