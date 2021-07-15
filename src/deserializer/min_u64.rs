use serde::{Deserialize, Deserializer};

pub(crate) fn min_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(u64::deserialize(deserializer)? * 60)
}
