use anyhow::Context;
use qdo_gotrue_api::GoTrueApi;

use dotenv::dotenv;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let url = env::var("URL").expect("URL is not found");
    let anon_key = env::var("ANON_KEY").expect("ANON_KEY is not found");
    let email = env::var("EMAIL").expect("EMAIL is not found");
    let password = env::var("PASSWORD").expect("PASSWORD is not found");

    let mut headers = HeaderMap::new();
    let apikey_value = HeaderValue::from_str(&anon_key).unwrap();
    headers.insert("apiKey", apikey_value);
    let api = GoTrueApi::new(url, headers);
    let session = api
        .singup(email, password)
        .await
        .context(format! {"Failed to signup"})?;

    println!("{}", session.access_token);

    Ok(())
}
