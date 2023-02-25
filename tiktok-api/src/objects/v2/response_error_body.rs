use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::objects::v2::Error;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseErrorBody {
    pub data: Map<String, Value>,
    pub error: Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v2::ErrorCode;

    #[test]
    fn test_de() {
        match serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../../tests/response_body_files/v2/user_info__err__access_token_invalid.json"
        )) {
            Ok(err_json) => {
                assert_eq!(err_json.error.code, ErrorCode::AccessTokenInvalid);
            }
            x => panic!("{x:?}"),
        }
    }
}
