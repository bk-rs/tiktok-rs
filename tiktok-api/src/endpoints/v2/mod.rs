//
pub mod common;
pub use common::EndpointRet;

//
pub mod user_info;
pub use user_info::UserInfoEndpoint;

//
#[cfg(feature = "with_video_upload")]
pub mod video_upload_init;
#[cfg(feature = "with_video_upload")]
pub use video_upload_init::VideoUploadInitEndpoint;
