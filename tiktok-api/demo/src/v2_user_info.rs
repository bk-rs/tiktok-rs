/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p tiktok-api-demo --bin tiktok_api_demo_v2_user_info -- 'YOUR_ACCESS_TOKEN'
*/

use std::env;

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use tiktok_api::endpoints::v2::{EndpointRet, UserInfoEndpoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::args().nth(1).ok_or("access_token missing")?;

    let client = IsahcClient::new()?;

    //
    let user_info = UserInfoEndpoint::new(&access_token);
    let ret = client.respond_endpoint(&user_info).await?;
    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{ok_json:?}");
        }
        EndpointRet::Other(_) => {
            panic!("{ret:?}");
        }
    }

    Ok(())
}
