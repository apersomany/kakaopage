use crate::{
    client::Client,
    enums::direction::Direction,
    models::{file_id::FileId, series_id::SeriesId, single_id::SingleId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    seriesid: SeriesId,
    direction: Direction,
    page: u64,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub total_count: u64,
    pub is_end: bool,
    pub singles: Vec<Single>,
}

#[derive(Debug, Deserialize)]
pub struct Single {
    pub id: SingleId,
    pub title: String,
    pub image_url: FileId,
    pub order_value: u64,
}

pub async fn singles(
    client: &Client,
    series_id: SeriesId,
    direction: Direction,
    page: u64,
) -> Result<Response> {
    Ok(client
        .post(
            "/api/v5/store/singles",
            &Request {
                seriesid: series_id,
                direction,
                page,
            },
        )
        .await?
        .result()?)
}
