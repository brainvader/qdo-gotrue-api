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
    pub fn new<T>(url: T, headers: HeaderMap) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            headers: headers,
        }
    }
}

impl GoTrueApi {
    pub async fn singup<T>(&self, email: T, password: T) -> Result<Session>
    where
        T: Into<String>,
    {
        let base_url =
            Url::parse(&self.url).context(format!("filed to parse url: {}", self.url))?;
        let signup_path = "/auth/v1/singup";
        let api_url = base_url
            .join(signup_path)
            .context(format!("failed to join {}", signup_path))?;
        let mut body: HashMap<&str, String> = HashMap::new();
        body.insert("email", email.into());
        body.insert("password", password.into());
        let fetcher = reqwest::Client::new();
        let request_builder = fetcher
            .post(api_url)
            .headers(self.headers.clone())
            .json(&body);

        println!("{:#?}", request_builder);
        let request = request_builder.build().unwrap();
        let res_body = request.body().unwrap();
        let body = std::str::from_utf8(res_body.as_bytes().unwrap());
        println!("{:#?}", body);

        let response = fetcher.execute(request).await?;
        // let response = request_builder.send().await?;
        // println!("text: {:#?}", response);
        let response_text = &response.text().await.expect("can't get text");
        // println!("text: {:#?}", response_text);
        let session = serde_json::from_str::<Session>(response_text)?;
        // let session = response.json::<Session>().await?;
        Ok(session)
        // match response.error_for_status() {
        //     Ok(res) => Ok(res.json::<Session>().await?),
        //     Err(err) => Err(ApiError {
        //         message: "Error happend".to_string(),
        //         status: err.status(),
        //     })?,
        // }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub provider_token: Option<String>,
    pub access_token: String,
    pub expires_in: Option<i64>,
    pub expires_at: Option<i64>,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
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
