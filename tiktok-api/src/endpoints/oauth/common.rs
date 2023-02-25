use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body, Response,
};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

use crate::objects::oauth::ResponseErrorBody;

//
//
//
#[derive(Debug, Clone)]
pub enum EndpointRet<T>
where
    T: core::fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<ResponseErrorBody, Body>)),
}

//
//
//
#[derive(Debug)]
pub enum EndpointError {
    MakeRequestUrlFailed(UrlParseError),
    MakeRequestFailed(HttpError),
    DeResponseBodyFailed(SerdeJsonError),
}
impl core::fmt::Display for EndpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for EndpointError {}

//
//
//
pub fn endpoint_parse_response<T>(response: Response<Body>) -> Result<EndpointRet<T>, EndpointError>
where
    T: core::fmt::Debug + Clone + DeserializeOwned,
{
    let status = response.status();
    #[allow(clippy::single_match)]
    match status {
        StatusCode::OK => {
            use crate::objects::oauth::Message;

            #[allow(clippy::single_match)]
            match serde_json::from_slice::<Message>(response.body()) {
                Ok(Message::Success) => {
                    let ok_json = serde_json::from_slice::<T>(response.body())
                        .map_err(EndpointError::DeResponseBodyFailed)?;

                    return Ok(EndpointRet::Ok(ok_json));
                }
                _ => {}
            }
        }
        _ => {}
    }

    match serde_json::from_slice::<ResponseErrorBody>(response.body()) {
        Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
        Err(_) => Ok(EndpointRet::Other((
            status,
            Err(response.body().to_owned()),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_parse_response() -> Result<(), Box<dyn std::error::Error>> {
        let resp_body = include_str!("../../../tests/response_body_files/oauth/refresh_token__err_with_expired_refresh_token.json");
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(resp_body.as_bytes().to_vec())?;

        match endpoint_parse_response::<()>(resp) {
            Ok(EndpointRet::Other((status_code, Ok(err_body)))) => {
                assert_eq!(status_code, StatusCode::OK);
                assert_eq!(err_body.data.error_code, 10010);
            }
            x => panic!("{x:?}"),
        }

        Ok(())
    }
}
