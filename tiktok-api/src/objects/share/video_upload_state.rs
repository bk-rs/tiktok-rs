// https://developers.tiktok.com/doc/webhooks-events/

use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VideoUploadState {
    Failed,
    Completed,
}
