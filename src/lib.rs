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
    }

    #[cfg(test)]
    mod private {
        #[allow(unused_imports)]
        use crate::model::private::*;

        #[test]
        fn it_works() {
            assert_eq!(1+1, 2)
        }
    }
}
