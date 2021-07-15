use crate::{
    client::Client,
    enums::category::Category,
    models::{file_id::FileId, series_id::SeriesId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request<'a> {
    word: &'a str,
    page: u64,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub current_category: Category,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub total_count: u64,
    pub is_end: bool,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: SeriesId,
    pub title: String,
    pub author: String,
    pub publisher_name: String,
    pub read_count: u64,
    pub category_uid: Category,
    pub category: String,
    pub sub_category: String,
    pub image_url: FileId,
}

pub async fn search(client: &Client, keyword: &str, page: u64) -> Result<Response> {
    Ok(client
        .post(
            "/api/v5/store/search",
            &Request {
                word: keyword,
                page,
            },
        )
        .await?
        .result()?)
}
