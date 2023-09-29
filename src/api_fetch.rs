use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, UPGRADE_INSECURE_REQUESTS};
use reqwest::Client;
use serde_json;
use std::fmt;
pub struct APIClient<'a> {
    token: &'a str,
    url: &'a str,
}

impl APIClient<'_> {
    pub async fn get_api_request(&self, api_request: APIRequest) -> Result<APIResponce, APIError> {
        Err(APIError::BadToken)
    }
    async fn generate_request_url(&self, request: &str) -> String {
        let mut url = String::new();

        url.push_str(self.url);
        url.push_str(request);
        url.push_str("?apikey=");
        url.push_str(self.token);

        url
    }
    async fn send_request(&self, request: &str) -> Result<serde_json::Value, reqwest::Error> {
        let client = Client::new();

        let url = &self.generate_request_url(request).await;

        let response = client.get(url).send().await?;
        // Parse the JSON response and return it as serde_json::Value
        let json_response: serde_json::Value = response.json().await?;
        return Ok(json_response);
    }
}
#[derive(Debug)]
pub enum APIError {
    BadToken,
}
impl std::error::Error for APIError {}
impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

pub enum APIRequest {}
pub enum APIResponce {}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn generate_request_url_test() {
        let api_client = APIClient {
            token: "YOUR_API_KEY",
            url: "https://financialmodelingprep.com/api/v3/",
        };
        let url = &api_client.generate_request_url(r#"example_request"#).await;
        assert_eq!(
            url,
            r#"https://financialmodelingprep.com/api/v3/example_request?apikey=YOUR_API_KEY"#
        );
    }
    #[tokio::test]
    async fn send_request_test() {
        let api_client = APIClient {
            token: "a995695cf5f49207748df3ee74fa7e71",
            url: "https://financialmodelingprep.com/api/v3/",
        };
        let responce = api_client
            .send_request("financial-statement-symbol-lists")
            .await
            .unwrap();
        println!("{}", responce);
    }
}
