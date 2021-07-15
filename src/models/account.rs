use crate::{client::Client, models::tokens::Tokens};
use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct Account {
    pub id: u32,
    pub token: String,
    pub device_id: String,
}

impl Account {
    pub async fn login(client: &Client, tokens: &mut Tokens) -> Result<Self> {
        let device_id: [u8; 16] = rand::random();
        let device_id = hex::encode(device_id);

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            s_token: String,
            user_uid: u32,
        }
        let resp: Response = client
            .post(
                "/auth/v3/app/login",
                &[
                    ("os_type", "OS02"),
                    ("access_token", &tokens.access_token().await?),
                    ("device_id", &device_id),
                    ("app_user_id", &tokens.id.to_string()),
                ],
            )
            .await?
            .result()?;
        Ok(Self {
            id: resp.user_uid,
            token: resp.s_token,
            device_id,
        })
    }
}
