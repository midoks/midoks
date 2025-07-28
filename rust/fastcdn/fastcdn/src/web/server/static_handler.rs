use actix_web::{HttpRequest, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

// 嵌入静态文件
#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

/// 静态文件处理器
pub struct StaticHandler;

impl StaticHandler {
    /// 处理静态文件请求
    pub async fn handle_static(req: HttpRequest) -> impl Responder {
        let path = req.path().trim_start_matches('/');
        match Asset::get(path) {
            Some(content) => {
                let mime = from_path(path).first_or_octet_stream();
                let body: Cow<[u8]> = content.data.into();
                HttpResponse::Ok().content_type(mime.as_ref()).body(body)
            }
            None => HttpResponse::NotFound().body("404 Not Found"),
        }
    }

    /// 首页重定向
    pub async fn index() -> impl Responder {
        HttpResponse::Found()
            .append_header(("Location", "/static/index.html"))
            .finish()
    }
}