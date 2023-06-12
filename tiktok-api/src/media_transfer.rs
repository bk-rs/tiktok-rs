use reqwest::{Body, Client, StatusCode};
use url::Url;

//
pub const CHUNK_SIZE_MIN: usize = 1024 * 1024 * 5;
pub const CHUNK_SIZE_MAX: usize = 1024 * 1024 * 64;
pub const CHUNK_COUNT_MIN: usize = 1;
pub const CHUNK_COUNT_MAX: usize = 1000;

//
fn get_chunk_size(chunk_size: usize) -> usize {
    core::cmp::min(core::cmp::max(chunk_size, CHUNK_SIZE_MIN), CHUNK_SIZE_MAX)
}

pub fn get_chunk_size_and_total_chunk_count(
    video_size: usize,
    chunk_size: usize,
) -> (usize, usize) {
    let chunk_size = get_chunk_size(chunk_size);

    if video_size <= chunk_size {
        (video_size, 1)
    } else {
        (
            chunk_size,
            (video_size as f64 / chunk_size as f64).floor() as usize,
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
    video_size: usize,
    stream: T,
) -> Result<StatusCode, UploadError>
where
    T: Into<Body>,
{
    let content_length = byte_range.end - byte_range.start;
    let content_range = format!(
        "bytes {}-{}/{}",
        byte_range.start,
        byte_range.end - 1,
        video_size,
    );

    // println!("content_length:{content_length} content_range:{content_range}");

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
        StatusCode::PARTIAL_CONTENT | StatusCode::CREATED => Ok(response_status),
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
    video_size: usize,
    stream: S,
) -> Result<StatusCode, UploadError>
where
    S: tokio::io::AsyncRead + Send + Sync + 'static,
{
    use tokio_util::io::ReaderStream;

    upload_part(
        client,
        upload_url,
        content_type,
        byte_range,
        video_size,
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
    file_size: usize,
) -> Result<StatusCode, UploadError> {
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
        file_size,
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
    chunk_size: Option<usize>,
) -> Result<Vec<Result<StatusCode, UploadError>>, UploadError> {
    let crate::tokio_fs_util::Info {
        file_size,
        file_name: _,
    } = crate::tokio_fs_util::info(file_path)
        .await
        .map_err(UploadError::GetFileInfoFailed)?;

    let video_size = file_size as usize;
    let (chunk_size, total_chunk_count) =
        get_chunk_size_and_total_chunk_count(video_size, chunk_size.unwrap_or(CHUNK_SIZE_MAX));

    if total_chunk_count > CHUNK_COUNT_MAX {
        return Err(UploadError::ChunkSizeTooSmaillOrFileTooLarge);
    }

    let mut ret_list = vec![];
    for chunk_count in CHUNK_COUNT_MIN..=CHUNK_COUNT_MAX {
        let chunk_index = chunk_count - 1;

        let file_index_start = chunk_index * chunk_size;
        let file_index_end = if total_chunk_count == chunk_count {
            video_size
        } else {
            file_index_start + chunk_size
        };

        match upload_part_from_file(
            client.to_owned(),
            upload_url.to_owned(),
            content_type,
            file_path,
            file_index_start..file_index_end,
            file_size as usize,
        )
        .await
        {
            Ok(x) => ret_list.push(Ok(x)),
            Err(err) => {
                ret_list.push(Err(err));
                break;
            }
        }

        if file_index_end >= video_size {
            break;
        }
    }

    Ok(ret_list)
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
    ChunkSizeTooSmaillOrFileTooLarge,
}
impl core::fmt::Display for UploadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for UploadError {}
