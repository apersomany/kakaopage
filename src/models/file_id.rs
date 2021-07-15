use crate::IMG_URL;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileId(String);

impl FileId {
    pub fn get_url(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(format!("{}/download/resource?kid={}", IMG_URL, self.0))
        }
    }
}
