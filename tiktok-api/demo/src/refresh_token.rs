/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p tiktok-api-demo --bin tiktok_api_demo_refresh_token -- 'YOUR_CLIENT_KEY' 'YOUR_REFRESH_TOKEN'
*/

use std::env;

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use tiktok_api::endpoints::oauth::{EndpointRet, RefreshTokenEndpoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client_key = env::args().nth(1).ok_or("client_key missing")?;
    let refresh_token = env::args().nth(2).ok_or("refresh_token missing")?;

    let client = IsahcClient::new()?;

    //
    let refresh_token = RefreshTokenEndpoint::new(&client_key, refresh_token);
    let ret = client.respond_endpoint(&refresh_token).await?;
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
