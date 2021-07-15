use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

pub(crate) fn yn_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "Y" => Ok(true),
        "N" => Ok(false),
        other => Err(Error::invalid_value(Unexpected::Str(other), &"Y or N")),
    }
}
