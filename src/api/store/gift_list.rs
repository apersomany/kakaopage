use crate::{
    client::Client,
    models::{account::Account, gift_id::GiftId, series_id::SeriesId},
};
use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    list: Vec<Gift>,
}

#[derive(Debug, Deserialize)]
pub struct Gift {
    id: SeriesId,
    #[serde(rename = "today_gift_uid")]
    gift_id: GiftId,
}

pub async fn gift_list(client: &Client, account: &Account) -> Result<Response> {
    Ok(client
        .post_with_account("/api/v6/store/today_gift_list", account, &())
        .await?
        .result()?)
}
