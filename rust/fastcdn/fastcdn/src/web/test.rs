use std::env;

use fastcdn_common::rpc::fastcdn::AdminLoginRequest;
use fastcdn_common::rpc::fastcdn::PingRequest;

use fastcdn_common::rpc::client::rpc::CommonRpc;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    test_rpc().await;
    Ok(())
}

#[allow(dead_code)]
pub async fn test_rpc() {
    println!("正在测试gRPC连接...");
    // 测试Admin服务 - 使用login方法
    match CommonRpc::connect("http://127.0.0.1:10001").await {
        Ok(mut client) => {
            let request = AdminLoginRequest {
                username: "admin".to_string(),
                password: "password".to_string(),
            };
            match client.login(request).await {
                Ok(response) => println!("✓ Admin登录服务响应: {:?}", response),
                Err(e) => println!("✗ Admin登录服务调用失败: {}", e),
            }
        }
        Err(e) => println!("✗ Admin服务连接失败: {}", e),
    }
}

#[allow(dead_code)]
pub async fn test_rpc_all() {
    println!("正在测试gRPC连接...");
    // 测试Admin服务 - 使用login方法
    match CommonRpc::connect("http://127.0.0.1:10001").await {
        Ok(mut client) => {
            let request = AdminLoginRequest {
                username: "admin".to_string(),
                password: "password".to_string(),
            };
            match client.login(request).await {
                Ok(response) => println!("✓ Admin登录服务响应: {:?}", response),
                Err(e) => println!("✗ Admin登录服务调用失败: {}", e),
            }
        }
        Err(e) => println!("✗ Admin服务连接失败: {}", e),
    }

    // 测试Ping服务
    match CommonRpc::connect("http://127.0.0.1:10001").await {
        Ok(mut client) => {
            let ping_request = PingRequest {};
            match client.ping(ping_request).await {
                Ok(response) => println!("✓ Ping服务连接成功: {:?}", response),
                Err(e) => println!("✗ Ping服务调用失败: {}", e),
            }
        }
        Err(e) => println!("✗ Ping服务连接失败: {}", e),
    }
}

#[allow(dead_code)]
pub async fn test_conf() {
    match env::current_dir() {
        Ok(path) => {
            println!("当前运行目录: {}", path.display());

            match fastcdn_common::config::server::Server::instance() {
                Ok(server_config_arc) => {
                    let server_config = server_config_arc.lock().unwrap();
                    println!("✓ 配置文件加载成功: {:#?}", *server_config);

                    // 显示配置信息
                    println!("环境: {}", server_config.env);
                    println!(
                        "HTTP服务: {}",
                        if server_config.http.on {
                            "启用"
                        } else {
                            "禁用"
                        }
                    );
                }
                Err(e) => println!("✗ 配置文件加载失败: {}", e),
            }
        }
        Err(e) => println!("✗ 获取当前目录失败: {}", e),
    }
}

#[allow(dead_code)]
pub async fn test_db() {
    // 测试数据库连接
    match fastcdn_common::db::pool::Manager::instance().await {
        Ok(db_manager) => {
            println!("✓ 数据库管理器创建成功");

            // 测试数据库迁移
            match db_manager.migrate().await {
                Ok(_) => println!("✓ 数据库迁移成功"),
                Err(e) => println!("✗ 数据库迁移失败: {}", e),
            }
        }
        Err(e) => println!("✗ 数据库管理器创建失败: {}", e),
    }
}
