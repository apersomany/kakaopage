use crate::{
    client::Client,
    enums::category::Category,
    models::{file_id::FileId, series_id::SeriesId, single_id::SingleId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    pub seriesid: SeriesId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub home: Home,
    pub free_desc: String,
    pub first_single_id: SingleId,
}

#[derive(Debug, Deserialize)]
pub struct Home {
    pub id: SeriesId,
    pub title: String,
    pub author_name: String,
    pub publisher_name: String,
    pub description: String,
    pub read_count: u64,
    pub category_uid: Category,
    pub category: String,
    pub sub_category: String,
    pub image_url: FileId,
    pub pub_period: String,
    pub page_rating_count: u64,
    pub page_rating_summary: f64,
}

pub async fn home(client: &Client, series_id: SeriesId) -> Result<Response> {
    Ok(client
        .post(
            "/api/v6/store/home",
            &Request {
                seriesid: series_id,
            },
        )
        .await?
        .result()?)
}
