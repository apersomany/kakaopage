use serde_derive::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SeriesId(pub u32);

impl SeriesId {}
