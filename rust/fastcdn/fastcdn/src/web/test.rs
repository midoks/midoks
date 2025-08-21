// use sqlx::MySqlPool;
// use std::sync::Arc;

use std::env;

// 引入共享的RPC客户端
use fastcdn_common::{HelloClient, PingClient};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // println!("开始测试...");

    // test_rpc().await;
    // test_conf().await;
    // test_db().await;

    // println!("✓ 所有测试完成");
    Ok(())
}

#[allow(dead_code)]
pub async fn test_rpc() {
    println!("正在测试gRPC连接...");
    // 测试Ping服务
    match PingClient::connect("http://127.0.0.1:10001").await {
        Ok(mut client) => match client.ping().await {
            Ok(response) => println!("✓ Ping服务连接成功: {}", response),
            Err(e) => println!("✗ Ping服务调用失败: {}", e),
        },
        Err(e) => println!("✗ Ping服务连接失败: {}", e),
    }

    // 测试Hello服务
    match HelloClient::connect("http://127.0.0.1:10001").await {
        Ok(mut client) => match client.say_hello("FastCDN Web").await {
            Ok(response) => println!("✓ Hello服务响应: {}", response),
            Err(e) => println!("✗ Hello服务调用失败: {}", e),
        },
        Err(e) => println!("✗ Hello服务连接失败: {}", e),
    }
}

#[allow(dead_code)]
pub async fn test_conf() {
    match env::current_dir() {
        Ok(path) => {
            println!("当前运行目录: {}", path.display());

            match fastcdn_common::config::server::Manager::new() {
                Ok(config_manager) => {
                    let server_config = config_manager.server();
                    println!("✓ 配置文件加载成功: {:#?}", server_config);

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
    match fastcdn_common::db::pool::Manager::new().await {
        Ok(db_manager) => {
            println!("✓ 数据库管理器创建成功");

            // 测试数据库连接
            match db_manager.test_connection().await {
                Ok(_) => println!("✓ 数据库连接测试成功"),
                Err(e) => println!("✗ 数据库连接测试失败: {}", e),
            }

            // 测试数据库迁移
            match db_manager.migrate().await {
                Ok(_) => println!("✓ 数据库迁移成功"),
                Err(e) => println!("✗ 数据库迁移失败: {}", e),
            }
        }
        Err(e) => println!("✗ 数据库管理器创建失败: {}", e),
    }
}
