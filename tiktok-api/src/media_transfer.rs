use reqwest::{Body, Client, StatusCode};
use url::Url;

//
pub const CHUNK_SIZE_MIN: usize = 1024 * 1024 * 5;
pub const CHUNK_SIZE_MAX: usize = 1024 * 1024 * 64;
pub const CHUNK_COUNT_MIN: usize = 1;
pub const CHUNK_COUNT_MAX: usize = 1000;

//
pub fn get_chunk_size_and_total_chunk_count(file_size: u64) -> (u64, u64) {
    if file_size <= CHUNK_SIZE_MAX as u64 {
        (file_size, 1)
    } else {
        (
            CHUNK_SIZE_MAX as u64,
            (file_size as f64 / CHUNK_SIZE_MAX as f64).ceil() as u64,
        )
    }
}

//
//
//
pub async fn upload_part<T>(
    client: Client,
    upload_url: Url,
    content_type: &str,
    byte_range: core::ops::Range<usize>,
    stream: T,
) -> Result<(), UploadError>
where
    T: Into<Body>,
{
    let content_length = byte_range.end - byte_range.start;
    let content_range = format!(
        "bytes {}-{}/{}",
        byte_range.start,
        byte_range.end - 1,
        byte_range.end - byte_range.start
    );

    //
    let response = client
        .put(upload_url)
        .header("Content-Type", content_type)
        .header("Content-Length", content_length)
        .header("Content-Range", content_range)
        .body(stream)
        .send()
        .await
        .map_err(UploadError::RespondFailed)?;

    //
    let response_status = response.status();

    match response_status {
        StatusCode::PARTIAL_CONTENT | StatusCode::CREATED => Ok(()),
        status => {
            let response_body = response
                .bytes()
                .await
                .map_err(UploadError::ReadResponseBodyFailed)?;
            let response_body = response_body.as_ref();

            Err(UploadError::ResponseMismatch(
                status,
                response_body.to_vec(),
            ))
        }
    }
}

#[cfg(feature = "with_tokio")]
pub async fn upload_part_from_reader_stream<S>(
    client: Client,
    upload_url: Url,
    content_type: &str,
    byte_range: core::ops::Range<usize>,
    stream: S,
) -> Result<(), UploadError>
where
    S: tokio::io::AsyncRead + Send + Sync + 'static,
{
    use tokio_util::io::ReaderStream;

    upload_part(
        client,
        upload_url,
        content_type,
        byte_range,
        Body::wrap_stream(ReaderStream::new(stream)),
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn upload_part_from_file(
    client: Client,
    upload_url: Url,
    content_type: &str,
    file_path: &std::path::PathBuf,
    file_index: core::ops::Range<usize>,
) -> Result<(), UploadError> {
    use tokio::{
        fs::File,
        io::{AsyncReadExt as _, AsyncSeekExt as _, SeekFrom},
    };

    let file_index_start = file_index.start;
    let file_index_end = file_index.end;

    let file_take_size = file_index_end - file_index_start;

    let mut file = File::open(&file_path)
        .await
        .map_err(UploadError::OpenFileFailed)?;
    file.seek(SeekFrom::Start(file_index_start as u64))
        .await
        .map_err(UploadError::OpenFileFailed)?;
    let file = file.take(file_take_size as u64);

    upload_part_from_reader_stream(
        client.to_owned(),
        upload_url,
        content_type,
        file_index,
        file,
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn upload_from_file(
    client: Client,
    upload_url: Url,
    content_type: &str,
    file_path: &std::path::PathBuf,
) -> Result<(), UploadError> {
    let crate::tokio_fs_util::Info {
        file_size,
        file_name: _,
    } = crate::tokio_fs_util::info(file_path)
        .await
        .map_err(UploadError::GetFileInfoFailed)?;

    for chunk_count in CHUNK_COUNT_MIN..=CHUNK_COUNT_MAX {
        let chunk_index = chunk_count - 1;

        let file_index_start = chunk_index * CHUNK_SIZE_MAX;
        let file_index_end = core::cmp::min(file_index_start + CHUNK_SIZE_MAX, file_size as usize);

        upload_part_from_file(
            client.to_owned(),
            upload_url.to_owned(),
            content_type,
            file_path,
            file_index_start..file_index_end,
        )
        .await?;

        if file_index_start + CHUNK_SIZE_MAX > file_size as usize {
            break;
        }
    }

    Ok(())
}

//
//
//
#[derive(Debug)]
pub enum UploadError {
    RespondFailed(reqwest::Error),
    ReadResponseBodyFailed(reqwest::Error),
    ResponseMismatch(StatusCode, Vec<u8>),
    #[cfg(feature = "with_tokio_fs")]
    GetFileInfoFailed(std::io::Error),
    #[cfg(feature = "with_tokio_fs")]
    OpenFileFailed(std::io::Error),
}
impl core::fmt::Display for UploadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for UploadError {}
