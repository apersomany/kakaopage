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
    id: SeriesId,
    title: String,
    author_name: String,
    publisher_name: String,
    description: String,
    read_count: u64,
    category_uid: Category,
    category: String,
    sub_category: String,
    image_url: FileId,
    pub_period: String,
    page_rating_count: u64,
    page_rating_summary: f64,
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
