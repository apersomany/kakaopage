use anyhow::{anyhow, Result};
use reqwest::{redirect::Policy, Client};
use serde_derive::*;

use crate::timestamp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub id: u32,
    pub access_token: String,
    pub access_expiry: u64,
    pub refresh_token: String,
    pub refresh_expiry: u64,
}

#[derive(Deserialize)]
struct TokensResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: Option<String>,
    refresh_token_expires_in: Option<u64>,
}

impl Tokens {
    pub async fn from_credentials(email: &str, password: &str) -> Result<Self> {
        let client = Client::builder().redirect(Policy::none()).build()?;

        let resp = client
            .post("https://auth.kakao.com/kakao_accounts/login.json")
            .form(&[
                ("client_id", "0a7f850343ab4b0261674dded5d55cf7"),
                ("app_key", "0a7f850343ab4b0261674dded5d55cf7"),
                ("email", email),
                ("password", password),
            ])
            .send()
            .await?;

        let resp = client
            .head("https://kauth.kakao.com/oauth/authorize")
            .query(&[
                ("client_id", "0a7f850343ab4b0261674dded5d55cf7"),
                ("approval_type", "invdividual"),
                ("response_type", "code"),
                (
                    "redirect_uri",
                    "kakao0a7f850343ab4b0261674dded5d55cf7://oauth",
                ),
            ])
            .header(
                "cookie",
                resp.cookies()
                    .map(|c| format!("{}={}", c.name(), c.value()))
                    .collect::<Vec<_>>()
                    .join(";"),
            )
            .send()
            .await?;

        let resp = client
            .post("https://kauth.kakao.com/oauth/token")
            .form(&[
                (
                    "code",
                    resp.headers()
                        .get("location")
                        .unwrap()
                        .to_str()?
                        .split("=")
                        .nth(1)
                        .unwrap(),
                ),
                ("grant_type", "authorization_code"),
                ("approval_type", "individual"),
                ("client_id", "0a7f850343ab4b0261674dded5d55cf7"),
            ])
            .send()
            .await?;

        let tokens: TokensResponse = resp.json().await?;

        let resp = Client::new()
            .get("https://kapi.kakao.com/v1/user/access_token_info")
            .header("authorization", format!("bearer {}", tokens.access_token))
            .send()
            .await?;

        #[derive(Debug, Deserialize)]
        struct TokenInfoResponse {
            id: u32,
        }

        let token_info: TokenInfoResponse = resp.json().await?;

        Ok(Self {
            id: token_info.id,
            access_token: tokens.access_token,
            access_expiry: timestamp() + tokens.expires_in,
            refresh_token: tokens.refresh_token.unwrap(),
            refresh_expiry: timestamp() + tokens.refresh_token_expires_in.unwrap(),
        })
    }

    pub async fn access_token(&mut self) -> Result<String> {
        if timestamp() < self.access_expiry {
            Ok(self.access_token.clone())
        } else {
            if timestamp() < self.refresh_expiry {
                let client = Client::builder()
                    .user_agent("libkakaopage")
                    .redirect(Policy::none())
                    .build()?;

                let resp = client
                    .post("https://kauth.kakao.com/oauth/token")
                    .form(&[
                        ("grant_type", "refresh_token"),
                        ("approval_type", "individual"),
                        ("client_id", "0a7f850343ab4b0261674dded5d55cf7"),
                        ("refresh_token", &self.refresh_token),
                    ])
                    .send()
                    .await?;

                let tokens: TokensResponse = resp.json().await?;
                self.access_token = tokens.access_token.clone();
                self.access_expiry = timestamp() + tokens.expires_in;
                if tokens.refresh_token.is_some() {
                    self.refresh_token = tokens.refresh_token.unwrap();
                    self.refresh_expiry = timestamp() + tokens.refresh_token_expires_in.unwrap();
                }
                Ok(tokens.access_token)
            } else {
                Err(anyhow!("Refresh token expired"))
            }
        }
    }
}
