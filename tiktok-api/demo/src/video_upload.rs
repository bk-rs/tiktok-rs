/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p tiktok-api-demo --bin tiktok_api_demo_video_upload -- 'YOUR_ACCESS_TOKEN' '/path/x.mp4'
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p tiktok-api-demo --bin tiktok_api_demo_video_upload -- 'YOUR_ACCESS_TOKEN' 'https://example.com/x.mp4'
*/

use std::env;

use http_api_isahc_client::{Client as _, IsahcClient};
use tiktok_api::endpoints::v2::{
    video_upload_init::VideoUploadInitRequestBodySourceInfo, EndpointRet, VideoUploadInitEndpoint,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::args().nth(1).ok_or("access_token missing")?;
    let path_or_url = env::args()
        .nth(2)
        .ok_or_else(|| "arg path_or_url missing".to_string())?;

    let client = IsahcClient::new()?;

    //
    if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
        let video_upload_init = VideoUploadInitEndpoint::new(
            &access_token,
            VideoUploadInitRequestBodySourceInfo::PullFromUrl {
                video_url: path_or_url.parse()?,
            },
        );
        let ret = client.respond_endpoint(&video_upload_init).await?;
        match &ret {
            EndpointRet::Ok(ok_json) => {
                println!("{ok_json:?}");
            }
            EndpointRet::Other(_) => {
                panic!("{ret:?}");
            }
        }
    } else {
        let video_upload_init =
            VideoUploadInitEndpoint::with_file(&access_token, &path_or_url.parse()?).await?;
        let ret = client.respond_endpoint(&video_upload_init).await?;
        let _upload_url = match &ret {
            EndpointRet::Ok(ok_json) => {
                println!("{ok_json:?}");
                ok_json
                    .data
                    .upload_url
                    .to_owned()
                    .ok_or("upload_url missing")?
            }
            EndpointRet::Other(_) => {
                panic!("{ret:?}");
            }
        };
    }

    Ok(())
}
