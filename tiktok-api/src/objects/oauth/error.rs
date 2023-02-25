use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub captcha: Option<String>,
    pub desc_url: Option<String>,
    pub description: Option<String>,
    pub error_code: isize,
}
