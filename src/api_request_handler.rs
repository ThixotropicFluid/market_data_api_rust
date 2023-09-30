#![allow(unused_imports)]
use crate::api_fetch;
use crate::api_request_handler::request_type::RequestType;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
pub mod request_type {
    pub trait RequestType {
        type Data;
        fn set_json(&mut self, json: serde_json::Value);
        fn get_json(&self) -> Option<serde_json::Value>;
        fn generate_request(&self) -> String;
        fn from_json(&self) -> Option<Self::Data>;
    }
    pub trait RequestData {}

    #[derive(Debug)]
    pub struct StockQuote<'a> {
        pub ticker: &'a str,
        pub json: Option<serde_json::Value>,
    }
    impl RequestType for StockQuote<'_> {
        type Data = StockQuoteData;
        fn set_json(&mut self, json: serde_json::Value) {
            self.json = Some(json);
        }
        fn get_json(&self) -> Option<serde_json::Value> {
            None
        }
        fn generate_request(&self) -> String {
            let mut res = String::from("v3/quote-short/");
            res.push_str(self.ticker);
            res
        }
        fn from_json(&self) -> Option<Self::Data> {
            self.json.clone().unwrap();
            let res: Vec<Self::Data> = serde_json::from_value(self.json.clone()?).unwrap();
            res.get(0).cloned()
        }
    }
    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct StockQuoteData {
        pub symbol: String,
        pub price: f32,
        pub volume: i32,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn stock_quote_test() {
        let api_client = api_fetch::APIClient {
            token: "a995695cf5f49207748df3ee74fa7e71",
            url: "https://financialmodelingprep.com/api/",
        };
        let mut request = request_type::StockQuote {
            ticker: "AAPL",
            json: None,
        };
        api_client.get_json(&mut request).await.unwrap();
        let result = request.from_json().unwrap();
        assert_eq!(result.symbol, "AAPL")
    }
}
