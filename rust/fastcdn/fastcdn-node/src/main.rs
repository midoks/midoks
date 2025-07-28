use clap::{Parser, Subcommand};
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tonic::Request;

// 引入自动生成的proto代码
use fastcdn_node::hello::hello_service_client::HelloServiceClient;
use fastcdn_node::hello::HelloRequest;
use fastcdn_node::ping::ping_service_client::PingServiceClient;
use fastcdn_node::ping::PingRequest;

/// 命令行信息
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn-node",
    long_about = "fastcdn node service"
)]

struct Cli {
    /// display version information
    #[arg(short, long, global = true)]
    verbose: bool,

    /// subcommand operation mode
    #[command(subcommand)]
    command: Commands,
}
/// subcommand operation mode
#[derive(Subcommand, Debug)]
enum Commands {
    /// start the fastcdn node server
    Start {
        /// run server in daemon mode (background)
        #[arg(short, long)]
        daemon: bool,
    },
    /// stop the fastcdn node server
    Stop {},
    /// reload the fastcdn node server
    Reload {},

    /// fastcdn node server Status
    Status {},

    /// test function
    Test {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let result = match &args.command {
        Commands::Start { daemon } => {
            if *daemon {
                start_daemon().await
            } else {
                start_node_server().await
            }
        }
        Commands::Stop {} => stop_daemon().await,
        Commands::Reload {} => {
            println!("reload node server!!!");
            Ok(())
        }
        Commands::Status {} => check_status().await,
        Commands::Test {} => test_grpc_connection().await,
    };

    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            eprintln!("错误: {}", error);
            Err(error)
        }
    }
}

// 启动节点服务器（前台模式）
async fn start_node_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在启动 fastcdn node 服务器...");

    // 测试与API服务器的连接
    test_api_connection().await?;

    println!("✓ fastcdn node 服务器启动成功");
    println!("✓ 已连接到 fastcdn-api RPC 服务");

    // 这里可以添加实际的服务器逻辑
    // 为了演示，我们让它保持运行
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        println!("time...");
        // 定期ping API服务器
        if let Err(e) = ping_api_server().await {
            println!("警告: 无法连接到API服务器: {}", e);
        }
    }
}

// 启动daemon模式
async fn start_daemon() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在启动后台node服务...");

    let current_exe =
        std::env::current_exe().map_err(|e| format!("无法获取当前执行文件路径: {}", e))?;

    let child = Command::new(&current_exe)
        .arg("start")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("无法启动后台进程: {}", e))?;

    let pid = child.id();

    // 保存PID到文件
    let mut file =
        File::create("fastcdn-node.pid").map_err(|e| format!("无法创建PID文件: {}", e))?;
    file.write_all(pid.to_string().as_bytes())
        .map_err(|e| format!("无法写入PID文件: {}", e))?;

    println!("✓ Node服务已在后台启动，PID: {}", pid);
    println!("✓ PID已保存到文件: fastcdn-node.pid");
    println!("✓ 后台Node服务启动成功");

    Ok(())
}

// 停止daemon
async fn stop_daemon() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在停止 fastcdn node 服务器...");

    if !std::path::Path::new("fastcdn-node.pid").exists() {
        println!("PID文件不存在，服务可能未在后台运行");
        return Ok(());
    }

    let mut file = File::open("fastcdn-node.pid").map_err(|e| format!("无法打开PID文件: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("无法读取PID文件: {}", e))?;

    let pid: u32 = contents
        .trim()
        .parse()
        .map_err(|e| format!("PID文件格式错误: {}", e))?;

    // 检查进程是否存在
    let check_output = Command::new("kill").args(["-0", &pid.to_string()]).output();

    match check_output {
        Ok(output) if output.status.success() => {
            println!("找到后台Node进程 PID: {}", pid);

            // 终止进程
            let kill_output = Command::new("kill")
                .args(["-TERM", &pid.to_string()])
                .output()
                .map_err(|e| format!("无法执行kill命令: {}", e))?;

            if kill_output.status.success() {
                println!("✓ 成功终止Node进程 PID: {}", pid);
            } else {
                return Err(format!(
                    "终止进程失败: {}",
                    String::from_utf8_lossy(&kill_output.stderr)
                )
                .into());
            }
        }
        _ => {
            println!("警告: PID {} 对应的进程不存在，可能已经退出", pid);
        }
    }

    // 删除PID文件
    remove_file("fastcdn-node.pid").map_err(|e| format!("无法删除PID文件: {}", e))?;

    println!("✓ 已删除PID文件");
    println!("✓ Node服务器停止成功");

    Ok(())
}

// 检查状态
async fn check_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("检查 fastcdn node 服务状态...");

    if std::path::Path::new("fastcdn-node.pid").exists() {
        let mut file = File::open("fastcdn-node.pid")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let pid: u32 = contents.trim().parse()?;

        let check_output = Command::new("kill").args(["-0", &pid.to_string()]).output();

        println!("{:#?}", check_output);

        match check_output {
            Ok(output) if output.status.success() => {
                println!("✓ Node服务正在运行，PID: {}", pid);
            }
            _ => {
                println!("✗ PID文件存在但进程不在运行");
            }
        }
    } else {
        println!("✗ Node服务未在后台运行");
    }

    // 测试API连接
    match test_api_connection().await {
        Ok(_) => println!("✓ API服务器连接正常"),
        Err(e) => println!("✗ API服务器连接失败: {}", e),
    }

    Ok(())
}

// 测试gRPC连接
async fn test_grpc_connection() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试gRPC连接...");

    test_api_connection().await?;

    println!("✓ 所有gRPC连接测试通过");
    Ok(())
}

// 测试API连接
async fn test_api_connection() -> Result<(), Box<dyn std::error::Error>> {
    // 测试Ping服务
    let mut ping_client = PingServiceClient::connect("http://127.0.0.1:50051").await?;
    let ping_request = Request::new(PingRequest {});
    let _ping_response = ping_client.ping(ping_request).await?;
    println!("✓ Ping服务连接成功");

    // 测试Hello服务
    let mut hello_client = HelloServiceClient::connect("http://127.0.0.1:50051").await?;
    let hello_request = Request::new(HelloRequest {
        name: "fastcdn-node".to_string(),
    });
    let hello_response = hello_client.say_hello(hello_request).await?;
    println!("✓ Hello服务响应: {}", hello_response.get_ref().message);

    Ok(())
}

// 定期ping API服务器
async fn ping_api_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut ping_client = PingServiceClient::connect("http://127.0.0.1:50051").await?;
    let ping_request = Request::new(PingRequest {});
    let _ping_response = ping_client.ping(ping_request).await?;
    Ok(())
}
