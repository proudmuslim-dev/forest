pub mod model;
pub mod ui;

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod public {
        use crate::model::public::*;
        use linked_hash_map::LinkedHashMap;

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

        #[test]
        fn markets() {
            let j = r#"
            [{"BTC-AEON":{"initialprice":"0.00022004","price":"0.00025992","high":"0.00025992","low":"0.00022003","volume":"0.00359066","bid":"0.00022456","ask":"0.00025993"}},{"BTC-BTCP":{"initialprice":"0.00300573","price":"0.00325000","high":"0.00379000","low":"0.00300010","volume":"0.04753022","bid":"0.00300099","ask":"0.00325000"}},{"BTC-BTN":{"initialprice":"0.00000032","price":"0.00000033","high":"0.00000033","low":"0.00000028","volume":"0.01306734","bid":"0.00000027","ask":"0.00000033"}},{"BTC-CIV":{"initialprice":"0.00032127","price":"0.00026700","high":"0.00032127","low":"0.00026700","volume":"0.73182101","bid":"0.00026700","ask":"0.00029000"}},{"BTC-COAL":{"initialprice":"0.00000289","price":"0.00000330","high":"0.00000330","low":"0.00000288","volume":"0.00297381","bid":"0.00000289","ask":"0.00000345"}},{"BTC-DASH":{"initialprice":"0.04699999","price":"0.05757790","high":"0.05757790","low":"0.04699999","volume":"0.00322117","bid":"0.04880001","ask":"0.05757750"}},{"BTC-DNR":{"initialprice":"0.00027742","price":"0.00027743","high":"0.00027743","low":"0.00027742","volume":"0.00078309"}},{"BTC-DOGE":{"initialprice":"0.00000041","price":"0.00000041","high":"0.00000041","low":"0.00000039","volume":"0.23236572","bid":"0.00000040","ask":"0.00000041"}},{"BTC-ETN":{"initialprice":"0.00000352","price":"0.00000338","high":"0.00000352","low":"0.00000319","volume":"2.50156282","bid":"0.00000328","ask":"0.00000337"}},{"BTC-FBF":{"initialprice":"0.00000002","price":"0.00000002","high":"0.00000002","low":"0.00000002","volume":"0.00020160"}},{"BTC-GRFT":{"initialprice":"0.00000307","price":"0.00000317","high":"0.00000336","low":"0.00000296","volume":"5.66677757","bid":"0.00000317","ask":"0.00000318"}},{"BTC-IPBC":{"initialprice":"0.00001874","price":"0.00001995","high":"0.00001995","low":"0.00001711","volume":"0.13150579","bid":"0.00001875","ask":"0.00001996"}},{"BTC-IRD":{"initialprice":"0.00000380","price":"0.00000310","high":"0.00000396","low":"0.00000310","volume":"0.07091748","bid":"0.00000310","ask":"0.00000337"}},{"BTC-ITNS":{"initialprice":"0.00000057","price":"0.00000053","high":"0.00000057","low":"0.00000049","volume":"0.01109704","bid":"0.00000053","ask":"0.00000055"}},{"BTC-KRB":{"initialprice":"0.00006215","price":"0.00006900","high":"0.00006900","low":"0.00005001","volume":"0.00205379","bid":"0.00006900","ask":"0.00007195"}},{"BTC-LTC":{"initialprice":"0.01905000","price":"0.01922345","high":"0.01922994","low":"0.01832040","volume":"0.38355349","bid":"0.01878022","ask":"0.01922343"}},{"BTC-LUX":{"initialprice":"0.00065505","price":"0.00065505","high":"0.00065505","low":"0.00065505","volume":"0.00069824","bid":"0.00071401","ask":"0.00075971"}},{"BTC-NAH":{"initialprice":"0.00000204","price":"0.00000202","high":"0.00000204","low":"0.00000202","volume":"0.05168677","bid":"0.00000202","ask":"0.00000380"}},{"BTC-NBR":{"initialprice":"0.00000066","price":"0.00000066","high":"0.00000070","low":"0.00000052","volume":"0.02534533","bid":"0.00000058","ask":"0.00000066"}},{"BTC-PCN":{"initialprice":"0.00000001","price":"0.00000001","high":"0.00000001","low":"0.00000001","volume":"0.00088627","bid":"0.00000000","ask":"0.00000001"}},{"BTC-PLURA":{"initialprice":"0.00000025","price":"0.00000030","high":"0.00000041","low":"0.00000023","volume":"0.39319767","bid":"0.00000030","ask":"0.00000033"}},{"BTC-SUMO":{"initialprice":"0.00017004","price":"0.00017007","high":"0.00017007","low":"0.00017004","volume":"0.00245623","bid":"0.00017008","ask":"0.00019994"}},{"BTC-TRTL":{"initialprice":"0.00000001","price":"0.00000001","high":"0.00000002","low":"0.00000001","volume":"0.42322449","bid":"0.00000001","ask":"0.00000002"}},{"BTC-WAE":{"initialprice":"0.00000017","price":"0.00000013","high":"0.00000018","low":"0.00000012","volume":"0.01046213","bid":"0.00000013","ask":"0.00000017"}},{"BTC-XAO":{"initialprice":"0.00000090","price":"0.00000095","high":"0.00000100","low":"0.00000090","volume":"0.00177852","bid":"0.00000095","ask":"0.00000100"}},{"BTC-XMR":{"initialprice":"0.02502002","price":"0.02500000","high":"0.03102001","low":"0.02500000","volume":"0.15549958","bid":"0.02420000","ask":"0.02625000"}},{"BTC-XTL":{"initialprice":"0.00000004","price":"0.00000004","high":"0.00000004","low":"0.00000003","volume":"0.40128073","bid":"0.00000003","ask":"0.00000004"}},{"BTC-XUN":{"initialprice":"0.00000024","price":"0.00000024","high":"0.00000030","low":"0.00000021","volume":"0.01266742","bid":"0.00000024","ask":"0.00000028"}},{"BTC-XVG":{"initialprice":"0.00000449","price":"0.00000498","high":"0.00000510","low":"0.00000385","volume":"0.07170363","bid":"0.00000426","ask":"0.00000497"}}]
            "#;

            // Dear future me: I'm so sorry
            let _markets: Vec<LinkedHashMap<&'static str, Market>> =
                serde_json::from_str(j).unwrap();
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
