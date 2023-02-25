use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::common::{endpoint_parse_response, EndpointError, EndpointRet};
use crate::objects::v2::{Error, User};

//
pub const URL: &str = "https://open.tiktokapis.com/v2/user/info/";
pub const FIELDS_DEFAULT: &str = "open_id,union_id,avatar_url,avatar_url_100,avatar_large_url,display_name,bio_description,profile_deep_link,is_verified,follower_count,following_count,likes_count";

//
#[derive(Debug, Clone)]
pub struct UserInfoEndpoint {
    pub access_token: String,
    pub fields: String,
}
impl UserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().into(),
            fields: FIELDS_DEFAULT.into(),
        }
    }

    pub fn with_fields(mut self, fields: impl AsRef<str>) -> Self {
        self.fields = fields.as_ref().into();
        self
    }
}

impl Endpoint for UserInfoEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UserInfoResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;
        url.query_pairs_mut().append_pair("fields", &self.fields);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
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
pub struct UserInfoResponseBody {
    pub data: UserInfoResponseBodyData,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfoResponseBodyData {
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v2::ErrorCode;

    #[test]
    fn test_render_request() {
        let req = UserInfoEndpoint::new("TOKEN").render_request().unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://open.tiktokapis.com/v2/user/info/?fields=open_id%2Cunion_id%2Cavatar_url%2Cavatar_url_100%2Cavatar_large_url%2Cdisplay_name%2Cbio_description%2Cprofile_deep_link%2Cis_verified%2Cfollower_count%2Cfollowing_count%2Clikes_count");

        let req = UserInfoEndpoint::new("TOKEN")
            .with_fields("open_id,union_id,avatar_url")
            .render_request()
            .unwrap();
        assert_eq!(
            req.uri(),
            "https://open.tiktokapis.com/v2/user/info/?fields=open_id%2Cunion_id%2Cavatar_url"
        );
    }

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<UserInfoResponseBody>(include_str!(
            "../../../tests/response_body_files/v2/user_info__full.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(
                    ok_json.data.user.open_id,
                    Some("_000fwZ23Mw4RY9cB4lDQyKCgQg4Ft6SyTuE".into())
                );
                assert_eq!(ok_json.error.code, ErrorCode::Ok);
            }
            x => panic!("{x:?}"),
        }

        //
        match serde_json::from_str::<UserInfoResponseBody>(include_str!(
            "../../../tests/response_body_files/v2/user_info.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(
                    ok_json.data.user.open_id,
                    Some("723f24d7-e717-40f8-a2b6-cb8464cd23b4".into())
                );
                assert_eq!(ok_json.error.code, ErrorCode::Ok);
            }
            x => panic!("{x:?}"),
        }
    }
}
