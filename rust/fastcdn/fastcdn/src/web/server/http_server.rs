use super::static_handler::StaticHandler;
use crate::web::app;
use crate::web::middleware::JwtMiddleware;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

                let open_swagger = config.open_swagger_doc;

                let service = HttpServer::new(move || {
                    let cors = Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600);

                    let mut app = App::new().wrap(cors);

                    if open_swagger {
                        app = app.service(
                            SwaggerUi::new("/swagger-ui/{_:.*}")
                                .url("/api-docs/openapi.json", crate::web::app::ApiDoc::openapi()),
                        );
                    }

                    app
                        // 静态资源和公共路由不需要JWT验证
                        .service(
                            web::resource("/static/{_:.*}")
                                .route(web::get().to(StaticHandler::handle_static)),
                        )
                        .service(
                            // 登录接口不需要JWT验证
                            web::scope("/api/auth").service(app::auth::login::login),
                        )
                        .service(
                            // 需要JWT验证的API路由
                            web::scope("/api")
                                .wrap(JwtMiddleware)
                                .service(app::api::hello)
                                .service(
                                    web::scope("/user").route(
                                        "/info",
                                        web::get().to(app::auth::user_info::user_info),
                                    ),
                                )
                                .service(
                                    web::scope("/auth")
                                        .route("/codes", web::get().to(app::auth::codes::codes)),
                                ),
                        )
                        .service(
                            // 安装不需要JWT验证的设置路由
                            web::scope("/setup")
                                .service(app::setup::test::db_test_post)
                                .service(app::setup::test::db_test_get),
                        )
                        .route("/", web::get().to(StaticHandler::index))
                })
                .bind(&http_listen)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                match service.run().await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("server startup failed: {}", e);
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
