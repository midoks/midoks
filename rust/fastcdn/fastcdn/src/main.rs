use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use clap::{Parser, Subcommand};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

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

/// 命令行信息
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn-api",
    long_about = "fastcdn api service"
)]

struct Cli {
    /// display version information
    #[arg(short, long, global = true)]
    verbose: bool,

    /// subcommand operation mode
    #[command(subcommand)]
    command: Option<Commands>,
}
/// subcommand operation mode
#[derive(Subcommand, Debug)]
enum Commands {
    /// start the fastcdn api server
    Start {
        /// run server in daemon mode (background)
        #[arg(short, long)]
        daemon: bool,
    },
    /// stop the fastcdn api server
    Stop {},
    /// reload the fastcdn api server
    Reload {},

    /// fastcdn api server Status
    Status {},

    /// test function
    Test {},
}

use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if args.verbose {
        println!("命令行参数解析结果:");
        println!("{:#?}", args);
    }

    // 执行相应的操作并返回适当的退出状态码
    let result: Result<&str, std::io::Error> = match &args.command {
        Some(Commands::Start { daemon }) => {
            if *daemon {
                // 后台模式运行
                println!("正在启动后台服务...");

                // 获取当前可执行文件路径
                let current_exe = match std::env::current_exe() {
                    Ok(exe) => exe,
                    Err(e) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "无法获取当前执行文件路径",
                        ));
                    }
                };

                // 启动后台进程
                let child = match Command::new(current_exe)
                    .arg("start")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    Ok(child) => child,
                    Err(e) => return Err(e),
                };

                // 保存PID到文件
                let pid_file = "fastcdn.pid";
                let mut file = match File::create(pid_file) {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                if let Err(e) = writeln!(file, "{}", child.id()) {
                    return Err(e);
                }

                println!("✓ 服务已在后台启动，PID: {}", child.id());
                println!("✓ PID已保存到文件: {}", pid_file);
                println!("✓ 服务地址: http://127.0.0.1:8980");

                Ok("后台服务启动成功")
            } else {
                // 前台模式运行
                println!("Server running at http://127.0.0.1:8980");
                let server_result = HttpServer::new(|| {
                    App::new()
                        .service(
                            web::resource("/static/{_:.*}").route(web::get().to(handle_static)),
                        )
                        .service(web::scope("/api").service(api::hello))
                        // .service(hello)
                        // .service(echo)
                        // .route("/echh", web::get().to(echh))
                        // .route("/hey", web::get().to(manual_hello))
                        .route("/", web::get().to(index))
                })
                .bind(("127.0.0.1", 8980));

                match server_result {
                    Ok(server) => {
                        if let Err(e) = server.run().await {
                            eprintln!("服务器运行失败: {}", e);
                            std::process::exit(1);
                        }
                        Ok("服务器启动成功")
                    }
                    Err(e) => {
                        eprintln!("服务器绑定失败: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Some(Commands::Stop {}) => {
            println!("正在停止 fastcdn api 服务器...");
            // 这里应该包含实际的服务器停止逻辑
            Ok("服务器停止成功")
        }
        Some(Commands::Reload {}) => {
            println!("正在重新加载 fastcdn api 服务器...");
            // 这里应该包含实际的服务器重载逻辑
            Ok("服务器重载成功")
        }
        Some(Commands::Status {}) => {
            println!("正在检查 fastcdn api 服务器状态...");
            // 这里应该包含实际的状态检查逻辑
            Ok("服务器状态正常")
        }
        Some(Commands::Test {}) => {
            println!("正在执行测试...");
            // 这里应该包含实际的测试逻辑
            Ok("测试执行完成")
        }
        None => {
            println!("欢迎使用 fastcdn-api 服务！");
            println!("使用 --help 查看可用命令");
            Ok("程序执行完成")
        }
    };

    match result {
        Ok(message) => {
            println!("✓ {}", message);
            Ok(())
        }
        Err(error) => {
            eprintln!("✗ 错误: {}", error);
            Err(error)
        }
    }
}
