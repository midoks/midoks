use tonic::{Request, Response, Status};

use crate::db::pool;
use crate::orm;
use crate::rpc::auth::AuthMiddleware;
use crate::rpc::fastcdn::admin_server::Admin;
use crate::rpc::fastcdn::{
    AdminCreateRequest, AdminCreateResponse, AdminLoginRequest, AdminLoginResponse,
    CreateOrUpdateAdminRequest, CreateOrUpdateAdminResponse,
};

/// Admin 实现
#[derive(Debug, Default)]
pub struct FcAdmin {}

#[tonic::async_trait]
impl Admin for FcAdmin {
    async fn login(
        &self,
        request: Request<AdminLoginRequest>,
    ) -> Result<Response<AdminLoginResponse>, Status> {
        // 验证请求头认证
        AuthMiddleware::verify_admin_request(&request).await?;

        let req = request.into_inner();
        let mut reply = AdminLoginResponse {
            id: -1,
            status: false,
            message: "please enter the correct username and password!".to_string(),
        };

        let result = orm::admin::check_admin_password(&req.username, &req.password).await;

        println!("req:{:?}", req);
        println!("result:{:?}", result);
        if let Ok(id) = result {
            if id > 0 {
                reply.id = id as i64;
                reply.status = true;
                reply.message = "ok".to_string();
            }
        }
        Ok(Response::new(reply))
    }

    // 创建或修改管理员
    async fn create_or_update_admin(
        &self,
        request: Request<CreateOrUpdateAdminRequest>,
    ) -> Result<Response<CreateOrUpdateAdminResponse>, Status> {
        AuthMiddleware::verify_admin_request(&request).await?;

        let req = request.get_ref();

        let adminids = orm::admin::find_admin_id_with_username(&req.username)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut resp = CreateOrUpdateAdminResponse { id: 0 };
        if adminids.len() > 0 {
            let admin_id = adminids[0]["id"].as_u64().unwrap_or(0);
            orm::admin::update_admin_password(admin_id, &req.password)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            resp.id = admin_id as i64;
        } else {
            let admin_id = orm::admin::add(
                &req.username,
                &req.password,
                &req.username,
                true,
                true,
                true,
                "zh",
                "zh",
                true,
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
            resp.id = admin_id as i64;
        }

        // println!("resp:{:?}", resp);
        Ok(Response::new(resp))
    }

    async fn create(
        &self,
        request: Request<AdminCreateRequest>,
    ) -> Result<Response<AdminCreateResponse>, Status> {
        // 验证请求头认证
        AuthMiddleware::verify_request(&request)?;

        let reply = AdminCreateResponse { id: 1 };

        match pool::Manager::instance().await {
            Ok(manager) => println!("数据库管理器实例: {:?}", manager),
            Err(e) => println!("获取数据库管理器失败: {:?}", e),
        }

        Ok(Response::new(reply))
    }
}
