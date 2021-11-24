use crate::api::{model::public::Ticker, requests::util::*};

pub struct PublicAPI {}

impl PublicAPI {
    pub async fn ticker(currency: Currency) -> Result<Ticker, Error> {
        let res = HTTP_CLIENT
            .get(format!("https://tradeogre.com/api/v1/ticker/{}", currency))
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}
