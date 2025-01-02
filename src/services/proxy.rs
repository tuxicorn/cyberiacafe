use reqwest::Client;
use std::time::Duration;

pub async fn check_url_via_socks_proxy(url: &str, proxy_address: &str) -> Result<String, reqwest::Error> {
    let proxy = reqwest::Proxy::all(proxy_address).expect("Failed to configure proxy");

    let client = Client::builder()
        .proxy(proxy)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    match client.get(url).send().await {
        Ok(response) if response.status().is_success() => Ok("online".to_string()),
        Ok(_) => Ok("offline".to_string()),
        Err(e) => Err(e),
    }
}
