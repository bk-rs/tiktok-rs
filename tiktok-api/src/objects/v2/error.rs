use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    pub log_id: String,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Ok,
    AccessTokenInvalid,
    InternalError,
    InvalidFileUpload,
    InvalidParams,
    RateLimitExceeded,
    ScopeNotAuthorized,
    ScopePermissionMissed,
    #[serde(other)]
    Other(Box<str>),
}
