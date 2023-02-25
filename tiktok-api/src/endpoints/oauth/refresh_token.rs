use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::common::{endpoint_parse_response, EndpointError, EndpointRet};
use crate::objects::oauth::{AccessToken, Message};

//
pub const URL: &str = "https://open-api.tiktok.com/oauth/refresh_token/";
pub const GRANT_TYPE: &str = "refresh_token";

//
#[derive(Debug, Clone)]
pub struct RefreshTokenEndpoint {
    pub client_key: String,
    pub refresh_token: String,
}
impl RefreshTokenEndpoint {
    pub fn new(client_key: impl AsRef<str>, refresh_token: impl AsRef<str>) -> Self {
        Self {
            client_key: client_key.as_ref().into(),
            refresh_token: refresh_token.as_ref().into(),
        }
    }
}

impl Endpoint for RefreshTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<RefreshTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;
        url.query_pairs_mut()
            .append_pair("client_key", &self.client_key)
            .append_pair("grant_type", GRANT_TYPE)
            .append_pair("refresh_token", &self.refresh_token);

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header(USER_AGENT, "tiktok-api")
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        endpoint_parse_response(response)
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshTokenResponseBody {
    pub data: AccessToken,
    pub message: Message,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let req = RefreshTokenEndpoint::new("KEY", "TOKEN")
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.uri(), "https://open-api.tiktok.com/oauth/refresh_token/?client_key=KEY&grant_type=refresh_token&refresh_token=TOKEN");
    }

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<RefreshTokenResponseBody>(include_str!(
            "../../../tests/response_body_files/oauth/refresh_token.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.open_id, "_000fwZ23Mw4RY9cB4lDQyKCgQg4Ft6SyTuE");
                assert_eq!(ok_json.message, Message::Success);
            }
            x => panic!("{x:?}"),
        }
    }
}
