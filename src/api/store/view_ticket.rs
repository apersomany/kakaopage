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
    welcome_rent_ticket: u64,
    rent_ticket: u64,
    welcome_buy_ticket: u64,
    buy_ticket: u64,
    event_ticket: u64,
    charged_at: String,
    #[serde(deserialize_with = "yn_bool")]
    waitfree: bool,
    #[serde(deserialize_with = "yn_bool")]
    user_activation: bool,
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
