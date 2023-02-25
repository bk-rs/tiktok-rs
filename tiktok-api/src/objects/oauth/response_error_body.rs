use serde::{Deserialize, Serialize};

use crate::objects::oauth::{Error, Message};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseErrorBody {
    pub data: Error,
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        match serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../../tests/response_body_files/oauth/refresh_token__err.json"
        )) {
            Ok(err_json) => {
                assert_eq!(err_json.data.error_code, 10002);
                assert_eq!(err_json.message, Message::ConstantError);
            }
            x => panic!("{x:?}"),
        }
    }
}
