use crate::{
    client::Client,
    models::{account::Account, file_id::FileId, single_id::SingleId},
};
use anyhow::Result;
use serde_derive::*;

#[derive(Serialize)]
struct Request {
    #[serde(rename = "productId")]
    single_id: SingleId,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    aid: String,
    #[serde(rename = "downloadData.members")]
    download_data: DonwloadData,
}

#[derive(Debug, Deserialize)]
pub struct DonwloadData {
    #[serde(rename = "camelCase")]
    total_size: u64,
    slide: Vec<File>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    id: FileId,
    size: u64,
}

pub async fn download_data(
    client: &Client,
    account: &Account,
    single_id: SingleId,
) -> Result<Response> {
    Ok(client
        .post_with_account(
            "/api/v5/inven/get_download_data",
            account,
            &Request { single_id },
        )
        .await?
        .result()?)
}
