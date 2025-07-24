use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};

// 必须为所有需要序列化/反序列化的结构体添加derive
#[derive(Debug, Serialize, Deserialize)] // 添加Debug方便日志记录
pub struct ApiResponse {
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Deserialize)] // 接收JSON请求的结构体
pub struct DataRequest {
    pub input: String,
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    web::Json(ApiResponse {
        message: "Hello from xxx server!".to_string(),
        status: 200,
    })
}

#[post("/data")]
pub async fn get_data(body: web::Json<DataRequest>) -> impl Responder {
    web::Json(ApiResponse {
        message: format!("Received: {}", body.input),
        status: 200,
    })
}
