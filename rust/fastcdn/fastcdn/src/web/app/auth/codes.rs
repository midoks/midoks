use crate::web::middleware::UserId;
use actix_web::{HttpMessage, HttpRequest, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CodesResponse {
    pub message: String,
    pub code: i16,
    pub data: Vec<String>,
}

// 使用路由配置而不是宏
#[utoipa::path(
    get,
    path = "/api/auth/codes",
    responses(
        (status = 200, description = "Codes list", body = CodesResponse)
    ),
    tag = "auth"
)]
pub async fn codes(req: HttpRequest) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = req.extensions().get::<UserId>().map(|id| id.0).unwrap_or(0);

    println!("codes user_id:{:?}", user_id);
    let response = CodesResponse {
        message: "ok".to_string(),
        code: 0,
        data: vec![
            "AC_100100".to_string(),
            "AC_100110".to_string(),
            "AC_100120".to_string(),
            "AC_100010".to_string(),
        ],
    };

    web::Json(response)
}
