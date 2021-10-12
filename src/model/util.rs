use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

impl FromStr for OrderType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("This can't fail unless the API is having an aneurysm"),
        }
    }
}

impl Display for OrderType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!("Doesn't need to be implemented properly for our use case.")
    }
}
