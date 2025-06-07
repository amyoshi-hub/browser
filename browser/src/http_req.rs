// src/http_req.rs
use reqwest;
// use tokio; // unused import だったので削除

pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}
