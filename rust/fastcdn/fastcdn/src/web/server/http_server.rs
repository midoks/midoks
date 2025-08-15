use super::static_handler::StaticHandler;
use crate::app::api;
use actix_web::{App, HttpServer, web};

/// HTTP服务器配置和启动
pub struct HttpServerManager;

impl HttpServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        let config_intance = fastcdn_common::config::ConfigServer::Server::instance()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        match config_intance.lock() {
            Ok(config) => {
                let http_listen_raw = config.get_http_addresses()[0];
                let http_listen = http_listen_raw.replace("\"", "");
                println!("web start: {:?}", http_listen);

                let service = HttpServer::new(|| {
                    App::new()
                        .service(
                            web::resource("/static/{_:.*}")
                                .route(web::get().to(StaticHandler::handle_static)),
                        )
                        .service(web::scope("/api").service(api::hello))
                        .route("/", web::get().to(StaticHandler::index))
                })
                .bind(&http_listen)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                match service.run().await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("服务器启动失败: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(_e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to acquire config lock",
                ));
            }
        }
    }
}
