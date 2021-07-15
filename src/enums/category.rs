use serde_repr::*;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Category {
    All = 0,
    Comic = 10,
    Novel = 11,
    Book = 16,
    TalkDrip = 18,
    Movie = 21,
    Broadcast = 22,
}
