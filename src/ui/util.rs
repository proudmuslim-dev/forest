use crate::api::{
    model::private::{Balances, OrderHistory},
    requests::{util::Error, PrivateAPI},
};

pub async fn load_dashboard(
    key: String,
) -> (Result<Balances, Error>, Result<Vec<OrderHistory>, Error>) {
    let balances = PrivateAPI::balances(key.clone()).await;
    let orders = PrivateAPI::orders(key).await;

    (balances, orders)
}
