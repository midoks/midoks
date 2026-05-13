use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};

// 必须为所有需要序列化/反序列化的结构体添加derive
#[derive(Debug, Serialize, Deserialize)] // 添加Debug方便日志记录
pub struct DbTestResponse {
    pub message: String,
    pub status: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // 接收JSON请求的结构体
pub struct DbTestRequest {
    pub hostname: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,
}

#[post("/db_test")]
pub async fn db_test_post(req: web::Json<DbTestRequest>) -> impl Responder {
    let config = fastcdn_common::db::pool::DbConfig {
        hostname: req.hostname.clone(),
        port: req.port,
        dbname: req.dbname.clone(),
        username: req.username.clone(),
        password: req.password.clone(),
    };

    match fastcdn_common::db::pool::Manager::new().await {
        Ok(db) => match db.test_connection(&config).await {
            Ok(_) => web::Json(DbTestResponse {
                message: "ok".to_string(),
                status: 0,
            }),
            Err(e) => web::Json(DbTestResponse {
                message: format!("error: {}", e),
                status: -1,
            }),
        },
        Err(e) => web::Json(DbTestResponse {
            message: format!("db error: {}", e),
            status: -1,
        }),
    }
}

#[get("/db_test")]
pub async fn db_test_get() -> impl Responder {
    web::Json(DbTestResponse {
        message: "ok".to_string(),
        status: 0,
    })
}
