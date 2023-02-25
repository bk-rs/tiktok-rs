use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub open_id: Option<String>,
    pub union_id: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_url_100: Option<String>,
    pub avatar_large_url: Option<String>,
    pub display_name: Option<String>,
    pub bio_description: Option<String>,
    pub profile_deep_link: Option<String>,
    pub is_verified: Option<bool>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub likes_count: Option<i64>,
}
