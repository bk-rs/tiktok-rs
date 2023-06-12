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
use crate::{
    media_transfer::{get_chunk_size_and_total_chunk_count, CHUNK_SIZE_MAX},
    objects::v2::Error,
};

//
pub const URL: &str = "https://open.tiktokapis.com/v2/post/publish/inbox/video/init/";

//
#[derive(Debug, Clone)]
pub struct VideoUploadInitEndpoint {
    pub access_token: String,
    pub source_info: VideoUploadInitRequestBodySourceInfo,
}
impl VideoUploadInitEndpoint {
    pub fn new(
        access_token: impl AsRef<str>,
        source_info: VideoUploadInitRequestBodySourceInfo,
    ) -> Self {
        Self {
            access_token: access_token.as_ref().into(),
            source_info,
        }
    }

    #[cfg(feature = "with_tokio_fs")]
    pub async fn with_file(
        access_token: impl AsRef<str>,
        file_path: &std::path::PathBuf,
        chunk_size: Option<usize>,
    ) -> Result<Self, EndpointError> {
        let crate::tokio_fs_util::Info {
            file_size,
            file_name: _,
        } = crate::tokio_fs_util::info(file_path)
            .await
            .map_err(EndpointError::GetFileInfoFailed)?;

        let video_size = file_size as usize;
        let (chunk_size, total_chunk_count) =
            get_chunk_size_and_total_chunk_count(video_size, chunk_size.unwrap_or(CHUNK_SIZE_MAX));

        let source_info = VideoUploadInitRequestBodySourceInfo::FileUpload {
            video_size,
            chunk_size,
            total_chunk_count,
        };

        Ok(Self {
            access_token: access_token.as_ref().into(),
            source_info,
        })
    }
}

impl Endpoint for VideoUploadInitEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<VideoUploadInitResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request_body = VideoUploadInitRequestBody {
            source_info: self.source_info.to_owned(),
        };
        let request_body =
            serde_json::to_vec(&request_body).map_err(EndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(USER_AGENT, "tiktok-api")
            .header(ACCEPT, "application/json; charset=UTF-8")
            .body(request_body)
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
pub struct VideoUploadInitRequestBody {
    pub source_info: VideoUploadInitRequestBodySourceInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "source")]
pub enum VideoUploadInitRequestBodySourceInfo {
    #[serde(rename = "FILE_UPLOAD")]
    FileUpload {
        video_size: usize,
        chunk_size: usize,
        total_chunk_count: usize,
    },
    #[serde(rename = "PULL_FROM_URL")]
    PullFromUrl { video_url: Url },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoUploadInitResponseBody {
    pub data: VideoUploadInitResponseBodyData,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoUploadInitResponseBodyData {
    pub publish_id: String,
    pub upload_url: Option<Url>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v2::ErrorCode;

    #[test]
    fn test_render_request() {
        let req = VideoUploadInitEndpoint::new(
            "TOKEN",
            VideoUploadInitRequestBodySourceInfo::FileUpload {
                video_size: 30567100,
                chunk_size: 30567100,
                total_chunk_count: 1,
            },
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.uri(), URL);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&req.body()).unwrap(),
            serde_json::json!({
                "source_info": {
                    "source": "FILE_UPLOAD",
                    "video_size": 30567100,
                    "chunk_size" : 30567100,
                    "total_chunk_count": 1
                }
            })
        );

        let req = VideoUploadInitEndpoint::new(
            "TOKEN",
            VideoUploadInitRequestBodySourceInfo::PullFromUrl {
                video_url: "https://example.verified.domain.com/example_video.mp4"
                    .parse()
                    .unwrap(),
            },
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.uri(), URL);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&req.body()).unwrap(),
            serde_json::json!({
                "source_info": {
                    "source": "PULL_FROM_URL",
                    "video_url": "https://example.verified.domain.com/example_video.mp4",
                }
            })
        );
    }

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<VideoUploadInitResponseBody>(include_str!(
            "../../../tests/response_body_files/v2/video_upload_init.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.publish_id, "v_inbox_file~v2.123456789");
                assert_eq!(
                    ok_json.data.upload_url,
                    Some("https://open-upload.tiktokapis.com/video/?upload_id=67890&upload_token=Xza123".parse().unwrap())
                );
                assert_eq!(ok_json.error.code, ErrorCode::Ok);
            }
            x => panic!("{x:?}"),
        }

        //
        match serde_json::from_str::<VideoUploadInitResponseBody>(include_str!(
            "../../../tests/response_body_files/v2/video_upload_init__without_upload_url.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.publish_id, "v_inbox_file~v2.123456789");
                assert!(ok_json.data.upload_url.is_none());
                assert_eq!(ok_json.error.code, ErrorCode::Ok);
            }
            x => panic!("{x:?}"),
        }
    }
}
