pub mod model;
pub mod ui;

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod public {
        use crate::model::public::*;

        #[test]
        fn ticker() {
            let j = r#"
                {
                    "success": true,
                    "initialprice": "0.02502002",
                    "price": "0.02500000",
                    "high": "0.03102001",
                    "low": "0.02500000",
                    "volume": "0.15549958",
                    "bid": "0.02420000",
                    "ask": "0.02625000"
                }
            "#;

            let _ticker: Ticker = serde_json::from_str(j).unwrap();
        }

        #[test]
        fn trade_history() {
            let j = r#"
            [{"date":1515128233,"type":"sell","price":"0.02454320","quantity":"0.17614230"},{"date":1515128233,"type":"sell","price":"0.02454181","quantity":"0.11651065"},{"date":1515128233,"type":"sell","price":"0.02453774","quantity":"11.37791774"},{"date":1515128235,"type":"sell","price":"0.02453774","quantity":"52.62616027"},{"date":1515128235,"type":"sell","price":"0.02453774","quantity":"0.39786743"},{"date":1515128253,"type":"sell","price":"0.02453774","quantity":"0.12844529"},{"date":1515128253,"type":"sell","price":"0.02453774","quantity":"7.89600000"},{"date":1515128253,"type":"sell","price":"0.02453774","quantity":"24.21560927"},{"date":1515128253,"type":"sell","price":"0.02453759","quantity":"0.25618000"},{"date":1515128253,"type":"sell","price":"0.02453660","quantity":"3.07034916"},{"date":1515128253,"type":"sell","price":"0.02453660","quantity":"5.66611628"},{"date":1515128254,"type":"sell","price":"0.02453660","quantity":"0.28166838"},{"date":1515128255,"type":"sell","price":"0.02453660","quantity":"1.00000000"},{"date":1515128271,"type":"sell","price":"0.02453660","quantity":"2.98866618"},{"date":1515128271,"type":"sell","price":"0.02453660","quantity":"38.26919550"},{"date":1515128271,"type":"sell","price":"0.02453660","quantity":"3.85000000"}]
            "#;

            let history: Vec<Trade> = serde_json::from_str(j).unwrap();
            println!("{:#?}", history)
        }
    }

    #[cfg(test)]
    mod private {
        #[allow(unused_imports)]
        use crate::model::private::*;

        #[test]
        fn it_works() {
            assert_eq!(1 + 1, 2)
        }
    }
}
