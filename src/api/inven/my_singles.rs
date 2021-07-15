use crate::{
    client::Client,
    models::{account::Account, series_id::SeriesId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    #[serde(rename = "seriesPid")]
    pub series_id: SeriesId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {}

pub async fn my_singles(
    client: &Client,
    account: &Account,
    series_id: SeriesId,
) -> Result<Response> {
    Ok(client
        .post_with_account(
            "/api/v5/inven/get_my_items_inseries",
            account,
            &Request { series_id },
        )
        .await?
        .result()?)
}
