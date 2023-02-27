/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p tiktok-api-demo --bin tiktok_api_demo_video_upload -- 'YOUR_OPEN_ID' 'YOUR_ACCESS_TOKEN' '/path/x.mp4'
*/

use std::{env, path::PathBuf};

use tiktok_api::endpoints::share::video_upload::video_upload_from_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let open_id = env::args().nth(1).ok_or("open_id missing")?;
    let access_token = env::args().nth(2).ok_or("access_token missing")?;
    let file_path: PathBuf = env::args()
        .nth(3)
        .ok_or_else(|| "arg file_path missing".to_string())?
        .parse()?;

    let client = reqwest::Client::builder()
        .connection_verbose(false)
        .build()?;

    //
    let json = video_upload_from_file(&client, open_id, access_token, &file_path).await?;
    println!("{json:?}");

    Ok(())
}
