#![allow(dead_code)]

use reqwest as req;
use reqwest::Error;
use serde::Deserialize;
use serde_json;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
struct JsonResponse {
  url: String,
  args: serde_json::Value,
}

async fn fetch_data(seconds: u64) -> Result<JsonResponse, Error> {
  let request_url = format!("https://httpbin.org/delay/{}", seconds);
  let response = req::get(request_url).await?;
  let delayed_response = response.json().await?;
  Ok(delayed_response)
}

async fn calculate_last_login() {
  sleep(Duration::from_secs(1)).await;
  println!("Logged in 2 days ago.");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let start_time = Instant::now();
  let data = fetch_data(3);
  let time_since = calculate_last_login();
  let (posts, _) = tokio::join!(data, time_since);
  let duration = start_time.elapsed();
  println!("Fetched {:?}.", posts);
  println!("Time taken: {:?}.", duration);
  Ok(())
}
