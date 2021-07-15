use serde_derive::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GiftId(pub u32);

impl GiftId {}
