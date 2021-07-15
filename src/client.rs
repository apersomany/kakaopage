use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    models::{account::Account, request::APIRequest, response::APIResponse},
    API_URL,
};
pub struct Client(reqwest::Client);

impl Client {
    pub fn new() -> Result<Self> {
        Ok(Self(
            reqwest::Client::builder().user_agent("reqwest").build()?,
        ))
    }

    pub(crate) async fn get<Res>(&self, endpoint: &str) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.0
            .get([API_URL, endpoint].join(""))
            .send()
            .await?
            .json::<APIResponse<Res>>()
            .await?
            .result()
    }

    pub(crate) async fn post<Req, Res>(&self, endpoint: &str, data: Req) -> Result<APIResponse<Res>>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        Ok(self
            .0
            .post([API_URL, endpoint].join(""))
            .form(&data)
            .send()
            .await?
            .json::<APIResponse<Res>>()
            .await?)
    }

    pub(crate) async fn post_with_dummy<Req, Res>(
        &self,
        endpoint: &str,
        data: Req,
    ) -> Result<APIResponse<Res>>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        Ok(self
            .0
            .post([API_URL, endpoint].join(""))
            .form(&APIRequest {
                s_token: "".into(),
                user_uid: 0,
                data,
            })
            .send()
            .await?
            .json::<APIResponse<Res>>()
            .await?)
    }

    pub(crate) async fn post_with_account<Req, Res>(
        &self,
        endpoint: &str,
        account: &Account,
        data: Req,
    ) -> Result<APIResponse<Res>>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        Ok(self
            .0
            .post([API_URL, endpoint].join(""))
            .form(&APIRequest {
                s_token: account.token.clone(),
                user_uid: account.id,
                data,
            })
            .send()
            .await?
            .json::<APIResponse<Res>>()
            .await?)
    }
}
