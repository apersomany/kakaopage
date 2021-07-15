use crate::{
    client::Client,
    deserializer::yn_bool::yn_bool,
    models::{account::Account, file_id::FileId, single_id::SingleId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    singleid: SingleId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(deserialize_with = "yn_bool")]
    allow_waitfree_ticket: bool,
    single: Single,
}

#[derive(Debug, Deserialize)]
pub struct Single {
    epub_viewer_id: FileId,
}

pub async fn single(client: &Client, account: &Account, single_id: SingleId) -> Result<Response> {
    Ok(client
        .post_with_account(
            "/api/v5/store/get/single",
            account,
            &Request {
                singleid: single_id,
            },
        )
        .await?
        .result()?)
}
