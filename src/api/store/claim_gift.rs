use crate::{
    client::Client,
    models::{account::Account, gift_id::GiftId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    pub today_gift_uid: GiftId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub ticket_num: u64,
    pub ticket_name: String,
}

pub async fn claim_gift(client: &Client, account: &Account, gift_id: GiftId) -> Result<Response> {
    Ok(client
        .post_with_account(
            "/api/v5/store/get_today_gift",
            account,
            &Request {
                today_gift_uid: gift_id,
            },
        )
        .await?
        .result()?)
}
