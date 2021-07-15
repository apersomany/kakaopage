use anyhow::{anyhow, Result};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct APIResponse<Res> {
    pub(crate) result_code: i32,
    pub(crate) message: String,
    #[serde(flatten)]
    pub(crate) data: Res,
}

impl<Res> APIResponse<Res> {
    pub(crate) fn result(self) -> Result<Res> {
        if self.result_code < 0 {
            Err(anyhow!(format!(
                "API Error {}: {}",
                self.result_code, self.message
            )))
        } else {
            Ok(self.data)
        }
    }
}
