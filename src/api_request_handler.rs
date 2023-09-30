#![allow(unused_imports)]
use crate::api_fetch;
use crate::api_request_handler::request_type::RequestType;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;

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
