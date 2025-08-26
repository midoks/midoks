use tonic::{Request, Status, metadata::MetadataValue};
use crate::config::api_node::ApiNode;

/// RPC认证中间件
pub struct AuthMiddleware;

impl AuthMiddleware {
    /// 验证请求头中的nodeId和secret
    pub fn verify_request<T>(request: &Request<T>) -> Result<(), Status> {
        let metadata = request.metadata();
        
        // 从请求头获取nodeId和secret
        let node_id = metadata
            .get("node-id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("缺少node-id请求头"))?;
            
        let secret = metadata
            .get("secret")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Status::unauthenticated("缺少secret请求头"))?;
        
        // 获取配置并验证凭据
        let api_node_config = ApiNode::instance()
            .map_err(|e| Status::internal(format!("配置加载失败: {}", e)))?;
            
        let config = api_node_config.lock().unwrap();
        
        if !config.verify_credentials(node_id, secret) {
            return Err(Status::unauthenticated("无效的nodeId或secret"));
        }
        
        Ok(())
    }
    
    /// 为客户端请求添加认证头
    pub fn add_auth_headers<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
        let api_node_config = ApiNode::instance()
            .map_err(|e| Status::internal(format!("配置加载失败: {}", e)))?;
            
        let config = api_node_config.lock().unwrap();
        
        // 添加nodeId和secret到请求头
        let node_id_value = MetadataValue::try_from(&config.node_id)
            .map_err(|e| Status::internal(format!("nodeId格式错误: {}", e)))?;
        let secret_value = MetadataValue::try_from(&config.secret)
            .map_err(|e| Status::internal(format!("secret格式错误: {}", e)))?;
            
        request.metadata_mut().insert("node-id", node_id_value);
        request.metadata_mut().insert("secret", secret_value);
        
        Ok(request)
    }
}