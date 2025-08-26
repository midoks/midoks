use tonic::{Request, Response, Status};

use crate::db::pool;
use crate::rpc::auth::AuthMiddleware;
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
        // 验证请求头认证
        AuthMiddleware::verify_request(&request)?;
        
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
        // 验证请求头认证
        AuthMiddleware::verify_request(&request)?;
        
        let login_req = request.into_inner();
        println!("admin login username: {:?}", login_req.username);
        println!("admin login password: {:?}", login_req.password);

        let reply = AdminLoginResponse {
            id: -1,       // 用户ID
            is_ok: false, // 登录是否成功
            message: "登陆失败".to_string(),
        };

        match pool::Manager::instance().await {
            Ok(db) => {
                println!("数据库管理器实例: {:?}", db);
                println!("addr: {:p}", &db);
            }
            Err(e) => {
                println!("获取数据库管理器失败: {:?}", e);
            }
        }

        Ok(Response::new(reply))
    }
}
