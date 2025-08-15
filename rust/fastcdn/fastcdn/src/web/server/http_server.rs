use super::static_handler::StaticHandler;
use crate::app::api;
use actix_web::{App, HttpServer, web};

/// HTTP服务器配置和启动
pub struct HttpServerManager;

impl HttpServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        let server = HttpServer::new(|| {
            App::new()
                .service(
                    web::resource("/static/{_:.*}")
                        .route(web::get().to(StaticHandler::handle_static)),
                )
                .service(web::scope("/api").service(api::hello))
                .route("/", web::get().to(StaticHandler::index))
        });

        let config_intance = fastcdn_common::config::ConfigServer::Server::instance()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        match config_intance.lock() {
            Ok(config) => {
                let http_bind = config.get_http_addresses()[0];
                // println!("{:?}", http_bind);
                server
                    .bind(http_bind)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
                    .run()
                    .await
            }
            Err(e) => {
                // 修复返回类型 - 直接返回错误而不是在 match 分支中
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to acquire config lock",
                ));
            }
        }
    }
}
