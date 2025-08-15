use super::static_handler::StaticHandler;
use crate::app::api;
use actix_web::{App, HttpServer, web};

/// HTTP服务器配置和启动
pub struct HttpServerManager;

impl HttpServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        let config = fastcdn_common::config::ConfigServer::Manager::new();
        println!("{:?}", config);

        println!("Server running at http://127.0.0.1:8980");

        let server_result = HttpServer::new(|| {
            App::new()
                .service(
                    web::resource("/static/{_:.*}")
                        .route(web::get().to(StaticHandler::handle_static)),
                )
                .service(web::scope("/api").service(api::hello))
                .route("/", web::get().to(StaticHandler::index))
        })
        .bind("127.0.0.1:8990");

        match server_result {
            Ok(server) => {
                if let Err(e) = server.run().await {
                    eprintln!("服务器运行失败: {}", e);
                    std::process::exit(1);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("服务器绑定失败: {}", e);
                std::process::exit(1);
            }
        }
    }
}
