#![allow(dead_code)]
#![allow(unused_imports)]
use crate::api_request_handler;
use reqwest;
use reqwest::Client;
use serde_json;
pub struct APIClient<'a> {
    pub token: &'a str,
    pub url: &'a str,
}

impl APIClient<'_> {
    pub async fn get_json<T: api_request_handler::request_type::RequestType>(
        &self,
        request: &mut T,
    ) -> Result<(), reqwest::Error> {
        let request_str = &request.generate_request()[..];
        let response = self.send_request(request_str).await?;
        request.set_json(response);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn generate_request_url_test() {
        let api_client = APIClient {
            token: "YOUR_API_KEY",
            url: "https://financialmodelingprep.com/api/",
        };
        let url = &api_client
            .generate_request_url(r#"v3/example_request"#)
            .await;
        assert_eq!(
            url,
            r#"https://financialmodelingprep.com/api/v3/example_request?apikey=YOUR_API_KEY"#
        );
    }
    #[tokio::test]
    async fn send_request_test() {
        let api_client = APIClient {
            token: "a995695cf5f49207748df3ee74fa7e71",
            url: "https://financialmodelingprep.com/api/",
        };
        let responce = api_client
            .send_request("v3/financial-statement-symbol-lists")
            .await
            .unwrap();
        println!("{}", responce);
    }
}
