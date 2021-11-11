use crate::api::model::private::OrderHistory;
use crate::{
    api::model::private::{Balance, Balances},
    api::requests::util::*,
};

pub struct PrivateAPI {}

impl PrivateAPI {
    pub async fn balance(key: String, currency: Currency) -> Result<Balance, Error> {
        let (key, secret) = get_key(key);

        let currency = currency.to_string();

        let res: Balance = HTTP_CLIENT
            .post("https://tradeogre.com/api/v1/account/balance")
            .basic_auth(key, Some(secret))
            .form(&[("currency", currency.as_str())])
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    pub async fn balances(key: String) -> Result<Balances, Error> {
        let (key, secret) = get_key(key);

        let res: Balances = HTTP_CLIENT
            .get("https://tradeogre.com/api/v1/account/balances")
            .basic_auth(key, Some(secret))
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    pub async fn orders(key: String) -> Result<Vec<OrderHistory>, Error> {
        let (key, secret) = get_key(key);

        let res: Vec<OrderHistory> = HTTP_CLIENT
            .post("https://tradeogre.com/api/v1/account/orders")
            .basic_auth(key, Some(secret))
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

fn get_key(key: String) -> (String, String) {
    let key: Vec<&str> = key.split(':').collect();
    (key[0].to_owned(), key[1].to_owned())
}
