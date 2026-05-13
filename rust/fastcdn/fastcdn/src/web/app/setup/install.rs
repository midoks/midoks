use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;

// 必须为所有需要序列化/反序列化的结构体添加derive
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)] // 添加Debug方便日志记录
pub struct InstallResponse {
    pub message: String,
    pub status: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone, utoipa::ToSchema)] // 接收JSON请求的结构体
pub struct InstallRequest {
    pub api_host: String,
    pub api_port: u32,
    pub api_protocol: String,
    pub api_type: String,

    pub node_id: String,
    pub secret: String,

    pub hostname: String,
    pub port: u16,
    pub dbname: String,
    pub username: String,
    pub password: String,

    pub admin_username: String,
    pub admin_password: String,
}

#[utoipa::path(
    post,
    path = "/setup/install",
    request_body = InstallRequest,
    responses(
        (status = 200, description = "Install result", body = InstallResponse)
    ),
    tag = "setup"
)]
#[post("/install")]
pub async fn install_post(
    req: web::Json<InstallRequest>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    // println!("{:?}", req);

    let config = fastcdn_common::db::pool::DbConfig {
        hostname: req.hostname.clone(),
        port: req.port,
        dbname: req.dbname.clone(),
        username: req.username.clone(),
        password: req.password.clone(),
    };

    let db_yaml = fastcdn_common::config::db::Db {
        user: req.username.clone(),
        password: req.password.clone(),
        database: req.dbname.clone(),
        host: format!("{}:{}", req.hostname, req.port),
    };
    let _ = db_yaml.write();
    let _ = db_yaml.write_api();

    // 测试db连接测试
    if let Err(e) = fastcdn_common::db::pool::Manager::new().await {
        return Ok(web::Json(InstallResponse {
            message: format!("db error: {}", e),
            status: -1,
        }));
    }

    let db = fastcdn_common::db::pool::Manager::new().await?;
    if let Err(e) = db.test_connection(&config).await {
        return Ok(web::Json(InstallResponse {
            message: format!("error: {}", e),
            status: -1,
        }));
    }

    let mut result_map: serde_json::Value = serde_json::json!({});
    if req.api_type == "new" {
        // 安装API节点
        let output = Command::new("bin/fastcdn-api")
            .current_dir("fastcdn-api")
            .arg("setup")
            .arg("--protocol=http")
            .arg(format!("--host={}", req.api_host))
            .arg(format!("--port={}", req.api_port))
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            println!("output error: {}", String::from_utf8_lossy(&output.stderr));
            return Ok(web::Json(InstallResponse {
                message: format!(
                    "install api error: {:?}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                status: -1,
            }));
        }

        // println!("output: {}", String::from_utf8_lossy(&output.stdout));
        result_map = serde_json::from_slice(&output.stdout).unwrap_or_default();
        // println!("result_map:{:?}", result_map);

        // 关闭正在运行的API节点，防止冲突
        let _ = Command::new("bin/fastcdn-api")
            .current_dir("fastcdn-api")
            .arg("stop")
            .output()
            .expect("Failed to execute command");

        // 启动API节点
        let _ = Command::new("bin/fastcdn-api")
            .current_dir("fastcdn-api")
            .arg("start")
            .arg("-d")
            .output()
            .expect("Failed to execute command");
    } else if req.api_type == "old" {
    }
    println!("req.api_type: {:?}", req.api_type);

    let api_admin_yaml = fastcdn_common::config::api_admin::ApiAdmin {
        rpc_endpoints: vec![format!("http://{}:{}", req.api_host, req.api_port)],
        rpc_disable_update: false,
        node_id: result_map
            .get("node_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        secret: result_map
            .get("secret")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
    };
    let _ = api_admin_yaml.write();

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let mut admin_rpc = fastcdn_common::rpc::client::CommonRpc::admin_rpc().await?;

    let req_admin = fastcdn_common::rpc::fastcdn::CreateOrUpdateAdminRequest {
        username: req.admin_username.clone(),
        password: req.admin_password.clone(),
    };

    let resp = Arc::get_mut(&mut admin_rpc)
        .ok_or("failed to get mutable reference to admin_rpc")?
        .create_or_update_admin(req_admin.clone())
        .await?;

    println!("resp:{:?}", resp);

    Ok(web::Json(InstallResponse {
        message: "ok".to_string(),
        status: 0,
    }))
}

#[get("/install")]
pub async fn install_get() -> impl Responder {
    web::Json(InstallResponse {
        message: "ok".to_string(),
        status: 0,
    })
}
