use std::fmt::Display;

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use reqwest::Url;
use thiserror::Error;

pub struct GoTrueApi {
    url: String,
    headers: HeaderMap,
}

impl GoTrueApi {
    fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            headers: HeaderMap::new(),
        }
    }
}

impl GoTrueApi {
    async fn singup<T>(&self, email: T, password: T)
    where
        T: Into<String>,
    {
    }
    async fn signin<T>(&self, email: T, password: T)
    where
        T: Into<String>,
    {
    }
    async fn signout<Token, E>(&self, token: Token) -> Result<()>
    where
        Token: Into<String> + Display,
    {
        let base = Url::parse(&self.url).context(format!("filed to parse url: {}", self.url))?;
        let api_url = base.join("logout")?;
        let fetcher = reqwest::Client::new();
        let _ = fetcher
            .post(api_url)
            .headers(self.headers.clone())
            .bearer_auth(token)
            .send()
            .await?;
        Ok(())
    }
    async fn refreshToken<Token>(&self, jwt: Token)
    where
        Token: Into<String>,
    {
    }
}

#[derive(Debug, Error)]
#[error("ApiError: {} {}", message, status)]
pub struct ApiError {
    message: String,
    status: i64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
