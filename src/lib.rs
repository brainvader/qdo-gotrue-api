use std::collections::HashMap;
use std::fmt::Display;

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use reqwest::Url;
use thiserror::Error;

use serde::{Deserialize, Serialize};

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
    async fn singup<T>(&self, email: T, password: T) -> Result<Session>
    where
        T: Into<String>,
    {
        let base_url =
            Url::parse(&self.url).context(format!("filed to parse url: {}", self.url))?;
        let resource_name = "singup";
        let api_url = base_url
            .join(resource_name)
            .context(format!("failed to join {}", resource_name))?;
        let mut body: HashMap<&str, String> = HashMap::new();
        body.insert("email", email.into());
        body.insert("password", password.into());
        let fetcher = reqwest::Client::new();
        let response = fetcher
            .post(api_url)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?;

        match response.error_for_status() {
            Ok(res) => Ok(res.json::<Session>().await?),
            Err(err) => Err(ApiError {
                message: "Error happend".to_string(),
                status: err.status(),
            })?,
        }
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
#[error("ApiError: {} {:?}", message, status)]
pub struct ApiError {
    message: String,
    status: Option<reqwest::StatusCode>,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    provider_token: Option<String>,
    access_token: String,
    expires_in: Option<i64>,
    expires_at: Option<i64>,
    refresh_token: Option<String>,
    token_type: String,
    user: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    app_metadata: HashMap<String, serde_json::Value>,
    user_metadata: HashMap<String, serde_json::Value>,
    aud: String,
    confirmation_sent_at: Option<String>,
    recovery_sent_at: Option<String>,
    action_link: String,
    email: String,
    phone: String,
    created_at: String,
    confirmed_at: Option<String>,
    email_confirmed_at: Option<String>,
    phone_confirmed_at: Option<String>,
    last_sign_in_at: Option<String>,
    role: Option<String>,
    updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
