use serde_derive::Serialize;

#[derive(Serialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Ascend,
    #[serde(rename = "desc")]
    Descend,
}
