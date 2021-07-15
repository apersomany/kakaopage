use crate::{
    client::Client,
    models::{account::Account, series_id::SeriesId, single_id::SingleId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Debug, Serialize)]
struct Request {
    seriesid: SeriesId,
    singleid: SingleId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub welcome_rent_ticket: Option<u64>,
    pub rent_ticket: Option<u64>,
    pub welcome_buy_ticket: Option<u64>,
    pub buy_ticket: Option<u64>,
    pub event_ticket: Option<u64>,
    pub waitfree_charged_at: Option<String>,
}

pub async fn use_ticket(
    client: &Client,
    account: &Account,
    series_id: SeriesId,
    single_id: SingleId,
) -> Result<Response> {
    let resp = client
        .post_with_account(
            "/api/v7/store/use/ticket",
            account,
            &Request {
                seriesid: series_id,
                singleid: single_id,
            },
        )
        .await?;
    if resp.result_code == -351 {
        Ok(resp.data)
    } else {
        Ok(resp.result()?)
    }
}
