use serde::{Deserialize, Serialize};

//
pub const EXPIRES_IN_DEFAULT: usize = 86400;
pub const REFRESH_EXPIRES_IN_DEFAULT: usize = 31536000;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessToken {
    pub open_id: String,
    pub scope: String,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_expires_in: i64,
}
