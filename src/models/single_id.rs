use serde_derive::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SingleId(pub u32);

impl SingleId {}
