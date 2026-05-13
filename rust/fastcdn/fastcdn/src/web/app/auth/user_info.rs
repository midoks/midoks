use crate::web::middleware::UserId;
use actix_web::{HttpMessage, HttpRequest, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserInfoData {
    pub username: String,
    pub id: i64,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserInfoResponse {
    pub message: String,
    pub code: i16,
    pub data: UserInfoData,
}

// 使用路由配置而不是宏
#[utoipa::path(
    get,
    path = "/api/user/info",
    responses(
        (status = 200, description = "User info response", body = UserInfoResponse)
    ),
    tag = "user"
)]
pub async fn user_info(req: HttpRequest) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = req.extensions().get::<UserId>().map(|id| id.0).unwrap_or(0);

    println!("user_id:{:?}", user_id);
    let response = UserInfoResponse {
        message: "ok".to_string(),
        code: 0,
        data: UserInfoData {
            id: 1,
            username: "测试".to_string(),
            roles: vec!["super".to_string()],
        },
    };

    web::Json(response)
}
