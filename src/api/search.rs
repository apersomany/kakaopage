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
    current_category: Category,
    results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    total_count: u64,
    is_end: bool,
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    id: SeriesId,
    title: String,
    author: String,
    publisher_name: String,
    read_count: u64,
    category_uid: Category,
    category: String,
    sub_category: String,
    image_url: FileId,
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
