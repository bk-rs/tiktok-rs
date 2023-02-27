use std::{
    io::{Error as IoError, ErrorKind as IoErrorKind},
    path::PathBuf,
};

use tokio::fs::metadata;

#[derive(Debug)]
pub struct Info {
    pub file_size: u64,
    pub file_name: Option<String>,
}

//
pub async fn info(path: &PathBuf) -> Result<Info, IoError> {
    let file_metadata = metadata(&path).await?;

    if !file_metadata.is_file() {
        return Err(IoError::new(IoErrorKind::Other, "is_file required"));
    }

    let file_size = file_metadata.len();
    let file_name = path
        .file_name()
        .and_then(|x| x.to_str())
        .map(|x| x.to_owned());

    Ok(Info {
        file_size,
        file_name,
    })
}
