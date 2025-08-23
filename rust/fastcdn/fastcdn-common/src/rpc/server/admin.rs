use tonic::{Request, Response, Status};

use crate::db::pool;

use crate::rpc::fastcdn::admin_server::Admin;
use crate::rpc::fastcdn::{
    AdminCreateRequest, AdminCreateResponse, AdminLoginRequest, AdminLoginResponse,
};

/// Admin 实现
#[derive(Debug, Default)]
pub struct FcAdmin {}

#[tonic::async_trait]
impl Admin for FcAdmin {
    async fn create(
        &self,
        request: Request<AdminCreateRequest>,
    ) -> Result<Response<AdminCreateResponse>, Status> {
        println!("收到 admin create 请求: {:?}", request);

        let reply = AdminCreateResponse {
            id: 1, // 示例ID，实际应该从数据库生成
        };

        match pool::Manager::instance().await {
            Ok(manager) => println!("数据库管理器实例: {:?}", manager),
            Err(e) => println!("获取数据库管理器失败: {:?}", e),
        }

        Ok(Response::new(reply))
    }

    async fn login(
        &self,
        request: Request<AdminLoginRequest>,
    ) -> Result<Response<AdminLoginResponse>, Status> {
        println!("收到 admin login 请求: {:?}", request);

        let reply = AdminLoginResponse {
            id: 1,                           // 用户ID
            is_ok: true,                     // 登录是否成功
            message: "登录成功".to_string(), // 响应消息
        };

        match pool::Manager::instance().await {
            Ok(manager) => {
                println!("数据库管理器实例: {:?}", manager);
                println!("addr: {}", &manager);
            }
            Err(e) => println!("获取数据库管理器失败: {:?}", e),
        }

        Ok(Response::new(reply))
    }
}
