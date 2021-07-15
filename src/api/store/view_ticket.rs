use crate::{
    client::Client,
    deserializer::yn_bool::yn_bool,
    models::{account::Account, series_id::SeriesId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Debug, Serialize)]
struct Request {
    seriesid: SeriesId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub welcome_rent_ticket: u64,
    pub rent_ticket: u64,
    pub welcome_buy_ticket: u64,
    pub buy_ticket: u64,
    pub event_ticket: u64,
    pub charged_at: String,
    #[serde(deserialize_with = "yn_bool")]
    pub waitfree: bool,
    #[serde(deserialize_with = "yn_bool")]
    pub user_activation: bool,
}

pub async fn view_ticket(
    client: &Client,
    account: &Account,
    series_id: SeriesId,
) -> Result<Response> {
    Ok(client
        .post_with_account(
            "/api/v6/store/view/ticket",
            account,
            &Request {
                seriesid: series_id,
            },
        )
        .await?
        .result()?)
}
