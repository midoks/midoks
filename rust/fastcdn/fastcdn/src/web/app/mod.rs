use serde::{Deserialize, Serialize};
use utoipa::OpenApi;

pub mod api;
pub mod auth;
pub mod setup;

// 必须为所有需要序列化/反序列化的结构体添加derive
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)] // 添加Debug方便日志记录
pub struct DataResponse {
    pub message: String,
    pub status: i16,
}

// 使用 utoipa 定义 OpenAPI 文档
#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login::login,
        auth::codes::codes,
        auth::user_info::user_info,
        setup::install::install_post
    ),
    components(
        schemas(
            auth::login::LoginRequest,
            auth::login::LoginResponse,
            auth::login::LoginResponseData,
            auth::codes::CodesResponse,
            auth::user_info::UserInfoResponse,
            auth::user_info::UserInfoData,
            setup::install::InstallRequest,
            setup::install::InstallResponse,
            DataResponse
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "user", description = "User endpoints"),
        (name = "setup", description = "Setup endpoints")
    )
)]
pub struct ApiDoc;
