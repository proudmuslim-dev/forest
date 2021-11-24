use crate::api::{
    model::private::{Balances, OrderHistory},
    model::public::Ticker,
    requests::{
        util::{Currency, Error},
        PrivateAPI, PublicAPI,
    },
};

pub async fn load_dashboard(
    key: String,
) -> (Result<Balances, Error>, Result<Vec<OrderHistory>, Error>) {
    let balances = PrivateAPI::balances(key.clone()).await;
    let orders = PrivateAPI::orders(key).await;

    (balances, orders)
}

pub async fn tickers(markets: Vec<Currency>) -> Result<Vec<Ticker>, Error> {
    let mut responses = vec![];
    for currency in markets {
        responses.push(PublicAPI::ticker(currency).await.unwrap());
    }

    Ok(responses)
}
