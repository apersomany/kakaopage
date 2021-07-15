use crate::{client::Client, models::account::Account};
use anyhow::Result;
use serde_derive::*;

#[derive(Debug, Deserialize)]
pub struct Response {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {}

pub async fn my_serieses(client: &Client, account: &Account) -> Result<Response> {
    Ok(client
        .post_with_account("/api/v5/inven/get_my_series", account, &())
        .await?
        .result()?)
}
