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

            let _history: Vec<Trade> = serde_json::from_str(j).unwrap();
        }
    }

    #[cfg(test)]
    mod private {
        use crate::model::private::*;

        #[test]
        fn submit_order() {
            let j = r#"
            {"success":true,"uuid":"235f770b-aa3f-4a31-8194-73d9612c2df1","bnewbalavail":"0.10000000","snewbalavail":"0.50000000"}
            "#;

            let _order: SubmitOrder = serde_json::from_str(j).unwrap();
        }

        #[test]
        fn order_history() {
            let j = r#"
            [{"uuid":"a40ac710-8dc5-b5a8-aa69-389715197b14","date":1514876938,"type":"sell","price":"0.02621960","quantity":"1.55772526","market":"BTC-XMR"},{"uuid":"7cbbdbf9-a3a8-d106-c53a-2b17e535580d","date":1514856437,"type":"sell","price":"0.02590469","quantity":"1.54412193","market":"BTC-XMR"},{"uuid":"f2a156c6-b085-c272-3132-657585ab19cf","date":1514847818,"type":"buy","price":"0.02226940","quantity":"4.47924057","market":"BTC-XMR"},{"uuid":"1c137c7e-2653-639a-531e-10a227cda052","date":1514845801,"type":"sell","price":"0.02514424","quantity":"0.04565047","market":"BTC-XMR"},{"uuid":"0f62c05e-7293-fa1c-f13f-7ca54272db00","date":1514831165,"type":"sell","price":"0.02614656","quantity":"0.11642460","market":"BTC-XMR"},{"uuid":"82a254c4-2408-8962-13cb-c4fa6eff0536","date":1514503221,"type":"sell","price":"0.02489833","quantity":"0.40163143","market":"BTC-XMR"},{"uuid":"0e2211e3-4f59-cd7c-2825-a56ddb49288e","date":1514428646,"type":"buy","price":"0.02348630","quantity":"0.24549267","market":"BTC-XMR"},{"uuid":"d9fb4ecd-b565-89d6-4313-7d265d131dce","date":1514426549,"type":"buy","price":"0.02293571","quantity":"271.49500000","market":"BTC-XMR"},{"uuid":"57a9b092-38cc-e1da-64fe-9978d097226b","date":1514426168,"type":"buy","price":"0.02028922","quantity":"0.09857452","market":"BTC-XMR"},{"uuid":"301f7892-5da5-9338-1a6d-bb4523e84acc","date":1514419953,"type":"sell","price":"0.02585061","quantity":"0.07579787","market":"BTC-XMR"},{"uuid":"f04e2526-2dfc-f551-2fe1-f494f0b05a6d","date":1514419230,"type":"sell","price":"0.02466752","quantity":"18.33794037","market":"BTC-XMR"},{"uuid":"664c3b32-8373-ff6f-de21-c39cf69836d3","date":1514417522,"type":"sell","price":"0.02585998","quantity":"3.69720197","market":"BTC-XMR"},{"uuid":"4e29ae46-ee04-1842-2731-9d1ceaf2617a","date":1514415373,"type":"sell","price":"0.02568255","quantity":"0.37899005","market":"BTC-XMR"},{"uuid":"18ca887c-1275-fd0f-f0ff-c740f2372c08","date":1514408908,"type":"sell","price":"0.02570058","quantity":"0.12440815","market":"BTC-XMR"}]
            "#;

            let _order_history: Vec<OrderHistory<'_>> = serde_json::from_str(j).unwrap();
        }
    }
}
