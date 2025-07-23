use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

mod app;
use app::api;

// 嵌入静态文件
#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

// 处理静态文件请求
async fn handle_static(req: HttpRequest) -> impl Responder {
    let path = req.path().trim_start_matches('/');
    // println!("{}", path);
    match Asset::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            let body: Cow<[u8]> = content.data.into();
            HttpResponse::Ok().content_type(mime.as_ref()).body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

// 首页重定向
async fn index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/static/index.html"))
        .finish()
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ec")]
async fn ec(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/echos")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn echh() -> impl Responder {
    HttpResponse::Ok().body("ffdd")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there2!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/static/{_:.*}").route(web::get().to(handle_static)))
            .service(web::scope("/api").service(api::hello))
            // .service(hello)
            // .service(echo)
            // .route("/echh", web::get().to(echh))
            // .route("/hey", web::get().to(manual_hello))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8980))?
    .run()
    .await
}
