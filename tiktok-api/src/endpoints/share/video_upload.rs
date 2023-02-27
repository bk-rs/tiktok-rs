use chrono::Utc;
use reqwest::{
    multipart::{Form, Part},
    Body, Client, Error as ReqwestError,
};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use url::{ParseError as UrlParseError, Url};

//
pub const URL: &str = "https://open-api.tiktok.com/share/video/upload/";

//
pub async fn video_upload<T>(
    client: &Client,
    open_id: impl AsRef<str>,
    access_token: impl AsRef<str>,
    stream: T,
    stream_length: Option<u64>,
    file_name: Option<String>,
) -> Result<VideoUploadResponseBody, VideoUploadError>
where
    T: Into<Body>,
{
    let open_id = open_id.as_ref();
    let access_token = access_token.as_ref();

    //
    let mut req_url = Url::parse(URL).map_err(VideoUploadError::MakeRequestUrlFailed)?;

    req_url
        .query_pairs_mut()
        .append_pair("open_id", open_id)
        .append_pair("access_token", access_token);

    //
    let part = if let Some(stream_length) = stream_length {
        Part::stream_with_length(stream, stream_length)
    } else {
        Part::stream(stream)
    };

    let part = if let Some(file_name) = file_name {
        part.file_name(file_name)
    } else {
        part.file_name(format!("{}.mp4", Utc::now().timestamp_millis()))
    };

    let form = Form::new().part("video", part).percent_encode_noop();

    //
    let resp = client
        .post(req_url)
        .multipart(form)
        .send()
        .await
        .map_err(VideoUploadError::RespondFailed)?;

    //
    let resp_body = resp
        .bytes()
        .await
        .map_err(VideoUploadError::ReadResponseBodyFailed)?;
    let resp_body = resp_body.as_ref();

    serde_json::from_slice::<VideoUploadResponseBody>(resp_body)
        .map_err(VideoUploadError::DeResponseBodyFailed)
}

#[cfg(feature = "with_tokio")]
pub async fn video_upload_from_reader_stream<S>(
    client: &Client,
    open_id: impl AsRef<str>,
    access_token: impl AsRef<str>,
    stream: S,
    stream_length: Option<u64>,
    file_name: Option<String>,
) -> Result<VideoUploadResponseBody, VideoUploadError>
where
    S: tokio::io::AsyncRead + Send + Sync + 'static,
{
    use tokio_util::io::ReaderStream;

    video_upload(
        client,
        open_id,
        access_token,
        Body::wrap_stream(ReaderStream::new(stream)),
        stream_length,
        file_name,
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn video_upload_from_file(
    client: &Client,
    open_id: impl AsRef<str>,
    access_token: impl AsRef<str>,
    file_path: &std::path::PathBuf,
) -> Result<VideoUploadResponseBody, VideoUploadError> {
    use tokio::fs::File;

    let crate::tokio_fs_util::Info {
        file_size,
        file_name,
    } = crate::tokio_fs_util::info(file_path)
        .await
        .map_err(VideoUploadError::GetFileInfoFailed)?;

    let file = File::open(&file_path)
        .await
        .map_err(VideoUploadError::OpenFileFailed)?;

    video_upload_from_reader_stream(
        client,
        open_id,
        access_token,
        file,
        Some(file_size),
        file_name,
    )
    .await
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoUploadResponseBody {
    pub data: VideoUploadResponseBodyData,
    pub extra: VideoUploadResponseBodyExtra,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoUploadResponseBodyData {
    pub err_code: i64,
    pub error_code: i64,
    pub share_id: Option<String>,
    pub error_msg: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoUploadResponseBodyExtra {
    pub error_detail: String,
    pub logid: String,
}

//
//
//
#[derive(Debug)]
pub enum VideoUploadError {
    MakeRequestUrlFailed(UrlParseError),
    RespondFailed(ReqwestError),
    ReadResponseBodyFailed(ReqwestError),
    DeResponseBodyFailed(SerdeJsonError),
    #[cfg(feature = "with_tokio_fs")]
    GetFileInfoFailed(std::io::Error),
    #[cfg(feature = "with_tokio_fs")]
    OpenFileFailed(std::io::Error),
}
impl core::fmt::Display for VideoUploadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for VideoUploadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<VideoUploadResponseBody>(include_str!(
            "../../../tests/response_body_files/share/video_upload.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.err_code, 0);
                assert_eq!(ok_json.extra.error_detail, "");
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<VideoUploadResponseBody>(include_str!(
            "../../../tests/response_body_files/share/video_upload__err.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.data.err_code, 20000);
                assert_eq!(
                    ok_json.extra.error_detail,
                    "access_token not found in the request query param"
                );
            }
            x => panic!("{x:?}"),
        }
    }
}
