use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorUnauthorized,
    http::header,
};
use futures::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;

// 用户ID的扩展类型
pub struct UserId(pub i64);

// JWT中间件
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // 检查是否有Authorization头
            if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
                // 提取token
                let auth_str = auth_header.to_str().unwrap_or_default();
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();

                    println!("token:{:?}", token);
                    // 验证token
                    match fastcdn::utils::jwt::validate(token) {
                        Ok(claims) => {
                            // 从token中提取用户ID
                            let user_id = claims.sub.parse::<i64>().unwrap_or(0);

                            // 将用户ID添加到请求扩展中，以便后续处理程序使用
                            req.extensions_mut().insert(UserId(user_id));

                            // 继续处理请求
                            let res = service.call(req).await?;
                            Ok(res)
                        }
                        Err(_) => {
                            // Token验证失败
                            Err(ErrorUnauthorized("无效的token"))
                        }
                    }
                } else {
                    // 不是Bearer token
                    Err(ErrorUnauthorized("无效的认证头"))
                }
            } else {
                // 没有Authorization头
                // 这里可以选择拒绝请求或者继续处理
                // 如果某些路由不需要认证，可以在这里放行
                let res = service.call(req).await?;
                Ok(res)
            }
        })
    }
}

// 辅助函数，用于从请求中获取用户ID
pub fn get_user_id(req: &ServiceRequest) -> Option<i64> {
    req.extensions().get::<UserId>().map(|user_id| user_id.0)
}