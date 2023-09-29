use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde;
use serde_json;

struct APIClient<'a> {
    token: &'a str,
    url: &'a str,
    headers: Option<HeaderMap>,
}
async fn genrate_headers(api_client: &APIClient<'_>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Token {}", api_client.token)).unwrap(),
    );
    headers
}
async fn ping(api_client: &APIClient<'_>) -> Result<(), reqwest::Error> {
    // Define the API endpoint and token
    let url = "https://api.marketdata.app/v1/stocks/quotes/SPY/";

    // Create a reqwest Client
    let client = Client::new();

    // Create headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Token {}", api_client.token)).unwrap(),
    );

    // Send the GET request with headers
    //let response = client.get(url).send().await?.json().await?;
    let response = client
        .get(api_client.url)
        .headers(headers.clone())
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_header_generation() {
        let mut api_client = APIClient {
            token: "not_a_token",
            url: "not_a_url",
            headers: None,
        };
        let headers = genrate_headers(&api_client).await;
        api_client.headers = Some(headers);
        assert_eq!(
            api_client
                .headers
                .unwrap()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap(),
            format!("Token {}", api_client.token)
        );
    }
    async fn test_api_client_construction() {}
    // Define a test function for the ping function.
    #[tokio::test]
    async fn test_ping() {
        let api_client = APIClient {
            token: "ekxVZ2l4RUdnTkxVNnJ3dlkxUlFUMmtLdnpVSDFOUGttdE5FWGItR1hHYz0",
            url: "https://www.rust-lang.org/",
            headers: None,
        };
        match ping(&api_client).await {
            Ok(()) => {}
            Err(err) => {
                dbg!(err);
            }
        }
    }
}
