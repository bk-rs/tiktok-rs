use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Message {
    Success,
    #[serde(rename = "error")]
    ConstantError,
    #[serde(other)]
    Other(Box<str>),
}
