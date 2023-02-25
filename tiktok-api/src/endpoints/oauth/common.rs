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
    match status {
        StatusCode::OK => {
            let ok_json = serde_json::from_slice::<T>(response.body())
                .map_err(EndpointError::DeResponseBodyFailed)?;

            Ok(EndpointRet::Ok(ok_json))
        }
        status => match serde_json::from_slice::<ResponseErrorBody>(response.body()) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((
                status,
                Err(response.body().to_owned()),
            ))),
        },
    }
}
